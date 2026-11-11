# 0034 — Admin-defined configurators and their formula language

**Status:** Accepted (phase C1: engine and API; the runner and builder screens follow)

**Context.** Configurable products are sold as a tree of lines that depends on many inputs:
repeating items (segments, units) grouped by location, project-wide options, lookup tables that
turn measurements into product codes, codes built from parts, accessories added under
conditions, tallies packed into boards, options priced as a share of their parent, warnings, and
calculated values a salesperson may override. Writing this per product in code (or in a separate
configurator application) is slow and puts selling rules out of reach of the people who own them.
ADR 0033 already turns a line tree into a quotation; what is missing is a way to *build* the
configurator inside the ERP.

**Decision.** New crate `crates/configurator` (module `configurator`, ADR 0028), migration
`20270115000001_configurator.sql`. An administrator describes a configurator as one typed JSON
document (the *definition*) using a small, safe formula language. A pure evaluator turns the
definition, the user's inputs and a catalogue snapshot into values, lines, issues and a trace.
Saved configurations become quotations through `tradocraft_sales::configured::import`.

### 1. Storage, revisions and import/export

* `configurator.models` (per company: `code`, `name`, `description`, `translations`, optional
  `product_id`), `configurator.model_revisions` (`revision`, `status` draft → active → obsolete,
  `definition jsonb`) and `configurator.configurations` (revision, customer, `inputs`, `result`
  snapshot, `quotation_id`).
* Revisions follow the BOM pattern (ADR 0011): partial unique indexes allow one active and one
  draft revision per model; only drafts can be edited; activating marks the previous active
  revision obsolete. Every change is audited.
* A draft may be saved with problems (its `errors` are returned, so work is never lost) but must
  be a well-formed document; activation refuses a definition with problems.
* Export/import format: `{format: "tradocraft.configurator", format_version: 1, model: {code,
  name, description, translations}, definition}`. Import creates a draft of the model with the
  same code (409 if it already has one) or a new model. The public demo
  `seed/configurators/conveyor_line.json` is in this format and is loaded by the development seed
  as an active revision.

### 2. The definition

```json
{
  "schema_version": 1,
  "name": "Conveyor line",
  "project": {
    "fields": [{ "name": "phases", "type": "choice", "options": [{"value": 1}, {"value": 3}], "default": 3 }],
    "computed": [{ "name": "total_length", "expr": "sum(segments, s => s.length_m)" }]
  },
  "group": { "fields": [{ "name": "voltage", "type": "number", "dimension": "voltage", "unit": "V" }] },
  "item_tables": [{
    "name": "segments", "group_field": "hall", "item_label": "{group_name} / segment {index}",
    "fields": [{ "name": "hall", "type": "text" }, { "name": "length_m", "type": "number", "unit": "m", "dimension": "length", "min": 0.5 }],
    "computed": [{ "name": "motor_code", "expr": "lookup('motors', {phases: phases, kw: power_kw}).code",
                   "overridable": true, "options_from": "map(rows('motors'), r => r.code)" }]
  }],
  "tables": [{ "name": "motors", "kind": "range", "keys": ["phases"],
               "range": {"key": "kw", "min": "kw_min", "max": "kw_max"}, "on_missing": "clamp",
               "columns": [...], "rows": [{"phases": 3, "kw_min": 0, "kw_max": 0.55, "code": "MOT-055"}] }],
  "outputs": [{ "id": "motor", "scope": "item", "code": "motor_code", "merge_identical": true,
                "children": [{ "id": "coat", "when": "coating", "code": "'COAT-PROT'", "percent_of_parent": "20" }] }],
  "validations": [{ "id": "low_voltage", "scope": "group", "when": "voltage < 380", "severity": "warning",
                    "message": "{voltage} V is too low" }]
}
```

* **Scopes.** `project` (one), `group` (one per distinct value of an item table's `group_field`,
  e.g. a hall; its name is the section on the quotation) and `item` (one per row of an item table).
  A group field named like a project field overrides it within the group and defaults to the
  project value. The group field itself is known before anything is calculated, so it cannot have
  `default_expr`, `visible_if` or `enabled_if`.
* **Fields:** `number` (with `dimension` mass/length/power/voltage/current/speed, a `unit` and
  `display_units`; values are stored in `unit`), `integer`, `boolean`, `choice` (`options`, or
  `options_from` a table column or a product family), `text` (`max_length`, `pattern`) and
  `product` (a code, filtered by `family` and a `product_filter` on `p`). Each has `label`, `help`,
  `translations`, `default` (literal) or `default_expr`, `visible_if`, `enabled_if`, `required`,
  `min`/`max`/`step` and a `layout` hint (`section`, `column`, `order`, `width`). Attributes that do
  not belong to the type are refused. A hidden field takes its default (or nothing) and is not
  checked; a disabled field ignores input.
* **Computed values** (any scope) are named formulas. `overridable` ones may be pinned by the
  user; `options_from` lists the allowed alternatives.
* **Tables** are typed (`columns` number/integer/text/boolean). `exact` tables match all `keys`;
  `range` tables match the keys plus a value between the `min` and `max` columns (inclusive; an
  empty bound is open; rows are ordered by `min`). `lookup(table, {keys}, offset)` returns the whole
  row. `offset` moves to a later row (range tables: within the rows of the same keys; exact
  tables: in table order). `on_missing`: `error` (an evaluation error), `null`, or `clamp` (the
  next row up, the last row above all ranges, and the first/last row for offsets).
* **Output rules** generate the line tree: `scope` (+ `table`), `when`, `for_each` (a list; one
  line per element with `each` and `each_index`), `code` (formula) or `code_template`, `quantity`
  (default 1; zero or less drops the line), `description`, `comment`, `section` (templates;
  section defaults to the group name), `children` (which see `parent.code` and
  `parent.quantity`, may have `percent_of_parent` and `percent_base`) and `merge_identical`:
  lines of the rule with the same code, texts and children collapse into one, quantities add up,
  and the comment lists the items or groups covered.
* **Validations**: `when` true raises an issue with `severity` (error/warning/info) and a
  `message` template at a `field`. Errors block quoting.
* `validate()` (via `compile`) reports **every** problem with a JSON path such as
  `$.item_tables[0].computed[1].expr` and, inside formulas, the character range.

### 3. Evaluation

`engine::evaluate(compiled, inputs, catalog) -> Evaluation` is pure. Inputs are
`{project: {name: value}, groups: {group: {name: value}}, items: {table: [{name: value}]}, pins:
[{name, table?, row?, group?, value, calculated?}]}`.

* Fields and computed values of all scopes form one dependency graph (a name read in a formula
  resolves item → group → project; a project formula reading an item table or `groups` depends on
  all its values). It is ordered topologically; cycles are definition errors
  (`circular dependency: total → segments.v → total`).
* Each value reports `source`: `input`, `default`, `empty`, `hidden`, `calculated`, `pinned`
  (pinned and the calculation still gives what it gave when pinned), `stale` (the calculation
  changed since, or the pin is no longer an allowed option; also a `pin_stale` warning) or
  `error`; plus `visible`, `enabled`, `options` and, when pinned, `calculated`.
* Built-in issues: `required`, `out_of_range`, `invalid_value`, `rows`, `evaluation_error`,
  `pin_stale`, `pin_ignored`, `no_code`, `unknown_product`; then the definition's validations.
  A failing formula never aborts the evaluation: its value is null and an issue explains why.
* Every line has a `trace`: the rule id and JSON path and, per contributing scope (several after a
  merge), the label, the `for_each` element and the values of the names the rule read.
* Every line names its catalogue product (`product_id`, `product_name`; absent for unknown codes),
  so clients price the tree without looking codes up. `evaluate?language=xx` gives the product
  names in that language where a translation exists, else the base name.
* The catalogue snapshot (codes, names, families, active flags, typed attributes) comes from
  `tradocraft_products::attributes::catalog_snapshot`.

### 4. Formula language

Values: decimal (exact, `rust_decimal`), integer, boolean, text, list, record (a table row or
`{k: v}`), null. Integer arithmetic stays integer until it overflows; `/` always gives a decimal.

| Syntax | Meaning |
|---|---|
| `12`, `0.55`, `'text'`, `"text"`, `true`, `null`, `[1, 2]`, `{phases: 3}` | literals |
| `a.b`, `xs[0]`, `xs[-1]` | field, index (out of range → null) |
| `-x`, `not x` / `!x` | unary |
| `* / %`, `+ -` | arithmetic; `+` also joins two texts |
| `< <= > >=`, `== !=` | comparison (numbers by value, texts alphabetically) |
| `and` / `&&`, `or` / `\|\|` | logic, short-circuit |
| `a ?? b` | `a` unless it is null |
| `c ? a : b` | condition (right associative) |
| `x => expr`, `(a, b) => expr` | functions, only as arguments |

Precedence, low to high: `?:`, `??`, `or`, `and`, equality, comparison, `+ -`, `* / %`, unary,
postfix. **Null** propagates through arithmetic and comparison (`null + 1` is null) and counts as
false in conditions, so missing inputs do not cascade into errors (the `required` check reports
them). Field access on null is null.

Functions: math `ceil floor round (x, digits?) abs sqrt sin cos pow pi min max`; text `concat str
number pad upper lower len contains matches capture` (regular expressions, size limited);
aggregates `sum avg count any all distinct` (a list and optionally `x => ...`); lists `filter map
first last sort range`; packing `chunks(n, size)` (`chunks(9, 4) = [4, 4, 1]`) and `bins(values,
capacity)` (first fit; a list of lists); tables and products `lookup rows product exists` and the
method `product(code).attr('key')`. `GET /configurator/functions` serves the catalogue.

Names: the scope's fields and computed values (then group, then project), item table names (a
list of item records, in group and project scope), `items` (the group's items when there is one
item table), `groups`, `group_name`, `index` (1-based row), `label` (templates of outputs and
validations), and in output rules `each`, `each_index` and `parent`. These names are reserved.

Templates are text with `{formula}` parts (`DRV-{motor_size}{phases == 3 ? 'T' : 'M'}`); `{{`
and `}}` are literal braces; decimals print without trailing zeros and null as nothing.

Pipeline: lexer → Pratt parser with character positions → static type checker (unknown names,
functions, tables, keys and columns, argument counts, operand types, conditions that are not
yes/no, invalid literal patterns; unknown types are `any` and checked at run time) → interpreter.
Safety: no I/O, no recursion or loops beyond list functions, a step budget per evaluation (2
million), lists capped at 10,000 elements, nesting capped at 64. We wrote our own language rather
than embedding a scripting engine: no expression engine is in the dependency tree, and this gives
exact decimals, list/record/lambda types and editor-friendly errors under our control (only the
`regex` crate, already in the lock file, was added).

### 5. API, permissions, quoting

* Routes under `/configurator`: models (list, create, get, update, `import`), revisions (create,
  get, update, `activate`, `export`, `evaluate`), `expressions/check` and `functions` for the
  editor, configurations (CRUD) and `configurations/{id}/quote`.
* Permissions: `configurator.design` (admin, product_manager, sales_manager) changes models and revisions and
  checks formulas; `configurator.use` (admin, sales, sales_manager) evaluates active revisions and
  works with configurations; reading models needs either. Drafts can only be evaluated by
  designers. Quoting also needs `quotations.write`. The module requires `products` and `sales`.
* Quoting evaluates the configuration on **its own revision** (an error issue refuses with 422),
  maps the lines to `ConfiguredLineInput` and calls the public `configured::import` (extracted
  from the HTTP handler, which is now a thin wrapper) with `strict` (a code without an active
  product refuses with 422 rather than dropping the line), `source = "configurator"`,
  `external_ref` = the configuration id and `configuration` = model, revision and inputs. ADR
  0033's idempotency applies: a draft quotation is updated, a sent one gets its next revision.
* A configuration stays on its revision. When the model has a newer active revision,
  `newer_revision_id` says so; `PUT` with that `revision_id` upgrades it.

### 6. Product attributes

`products.product_attributes (product_id, key, kind number|text|boolean, typed value, unit)`,
`GET/PUT /products/{id}/attributes` (`products.read` / `products.write`; PUT replaces the list and
is audited with before and after).

**Consequences.** Selling rules become data that product owners maintain and version; the server
is the only engine (the app calls `evaluate` while the user types), so web, desktop and later the
portal give identical results. Definitions are portable between installations, so private
configurators can live outside this repository. The language is deliberately small; anything it
cannot express needs a new function in code, reviewed like any other change.

**Later.** The runner and builder screens (C2, C3), solution scoring cards, the upgrade flow with
differences and translated definitions (C4). CSV import/export of tables is done in the builder.
Unit conversion is stored per dimension (`definition::units`) for the screens; the engine works
in each field's unit.
