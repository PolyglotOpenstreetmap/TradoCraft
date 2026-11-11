# 0033 — Configured quotations: line structure, percent-of-parent pricing and import

**Status:** Accepted (phase 1a)

**Context.** Configurable products (product type `configurable`, e.g. the demo `UNIT-200`) are
sold as a main item with options below it: a frame, a controller, a protective coating priced as
a share of the unit. Until now a quotation was a flat list of lines, so options could not be shown
under their unit, documents could not be organised in sections, and an option priced "20 % of the
unit" had to be calculated by hand. Configurations are often made in a separate configurator tool;
retyping them in the ERP is slow and error-prone, and the configurator must be able to send an
updated configuration without creating duplicates.

**Decision.** Migration `20270114000001_configured_quotations.sql`; code in
`crates/sales/src/{configured,service,routes,pdf}.rs`, `crates/pricing` and
`crates/products/src/translations.rs`.

### 1. Line structure

* `sales.quotation_lines` gains `parent_line_id` (a composite foreign key on
  `(quotation_id, parent_line_id)` so a parent is always in the same quotation; `ON DELETE
  CASCADE`), `section` (heading), `comment` (printed below the line), `percent_of_parent` and
  `percent_base`.
* The API speaks positions, not ids: `QuotationLineInput` has optional `parent_position` (the
  1-based position of an **earlier** line in the same list), `section`, `comment`,
  `percent_of_parent` and `percent_base`. `QuotationLine` returns the same fields
  (`parent_position` is derived from the stored `parent_line_id`). All are optional, so existing
  clients are unaffected.
* Revisions copy the structure (`keep` copies rows; `current` and the repricing comparison rebuild
  inputs with the parent positions remapped after the product-less `SHIPPING` line is dropped).
  Orders do **not** copy the structure yet (see Later).

### 2. Percent-of-parent pricing (extends ADR 0002/0012)

* `tradocraft_pricing::percent_of_parent_price(parent, percent, base)` returns the child's unit
  price, rounded to 4 decimals, from the already priced parent line. Lines are priced in order, so
  the parent (always earlier) is resolved first.
* `base = parent_list_unit_price` (default): the child gets `percent` % of the parent's list unit
  price and then its own rebate (product → family → general) and line discount, like a list line.
* `base = parent_net_unit_price`: the child gets `percent` % of the parent's net unit price and is
  already net: no rebate (`rebate_source = net`) and no line discount (422).
* The percentage is per unit; the child's quantity applies as usual (send the same quantity as the
  parent for "x % of the line"). Accepted range 0 to 1000, stored with 3 decimals.
* A percent line is only as confirmed as its parent: under an unconfirmed indicative parent it is
  unconfirmed too, and the quotation cannot be sent until the parent price is confirmed.
* A product used only on percent lines needs no zone price (`get_products_for_quoting_with`); used
  on its own it still does (422).

### 3. Import: `POST /api/v1/quotations/configured`

* Payload: `source`, `external_ref`, `customer_id`, optional `price_zone_code`, `purpose`,
  `delivery_country_code`, `power_grid_id`, `valid_days`, `notes`; `configuration` (opaque JSON,
  stored verbatim in `sales.quotations.configuration` for traceability, never interpreted);
  `strict`; and `lines`, a tree of `{code, quantity, section?, comment?, description?,
  unit_price?, percent_of_parent?, percent_base?, children[]}`.
* Codes are resolved through `tradocraft_products::active_ids_by_code`. An unknown or inactive
  code is skipped **with its sub-tree** and produces a `SalesWarning` `unknown_product`
  (`details.path` like `1.4`, `details.skipped_children`). `price_zone_code` is only an
  expectation: prices always come from the customer's own zone (ADR 0002); a different value adds
  `price_zone_mismatch`. With `strict: true` either refuses the whole import with 422. Customer and
  zone come from `crm::get_customer_for_quoting`, as for every quotation; there is no zone or
  language override.
* **Idempotency** per `(company, source, external_ref)`, stored on every revision
  (`import_source`, `import_external_ref`, indexed) and serialised with a transaction-scoped
  advisory lock: no quotation yet → a new draft (201, `action: created`); latest revision is a
  draft → it is replaced and repriced today (200, `updated`); latest revision is sent or rejected
  → the existing revise path (`keep`) creates the next revision, which is then updated (201,
  `revised`); accepted → 409. An opportunity link made in the ERP survives a re-import. A manual
  revision of an imported quotation keeps the source reference, so a later re-import finds it.
* The import reuses the draft code paths of `POST/PUT /quotations` and `/revise` (refactored into
  `create_draft`, `update_draft`, `revise` that run in the caller's transaction), so pricing,
  shipping sync, power grid and product relation warnings (ADR 0025, 0027) and audit events are
  identical. The import itself is audited as `imported` with source, reference, action and the
  unknown codes. The response is `{action, quotation}`; the import's own warnings come first in
  `quotation.warnings`.
* Limits: at most 500 lines, 8 levels, `source`/`external_ref` up to 200 characters.

### 4. Permission and role

* New permission `quotations.import`, owned by the `sales` module (ADR 0028), granted to `admin`
  and `sales_manager`.
* New role `integration` for a service account (seed user `integration@example.com`):
  `quotations.import`, `quotations.read`, `customers.read`, `products.read`. It can import and read
  back (including the PDF) but cannot create, edit, send or accept quotations by hand. Sending
  stays with sales.

### 5. Documents

* `pdf.rs` passes the lines both flat (`lines`) and grouped by section (`groups`, each with a
  subtotal). A sub-line without its own section is shown in its parent's; sections appear in the
  order of their first line. Each line has its `level` (depth), `comment`, `percent_of_parent` and
  `indicative`.
* `templates/quotation.typ` prints a heading and subtotal per named section, indents options under
  their parent, prints comments in small italics and the percentage next to the description.
  Unconfirmed indicative prices (only possible on drafts) are marked with `*` and a footnote
  "indicative price, to be confirmed" in the document language. Lines priced from the parent's net
  price show "net".
* The template directory is configurable with the existing `TEMPLATE_DIR`; `EXTRA_SEED_DIR` runs
  extra `*.sql` files after the development seed, so demo data and layouts can live outside this
  repository.

### 6. Product names per language

* `products.product_translations (product_id, language, name, description)`, shared master data
  like products. `GET/PUT /products/{id}/translations` (`products.read` / `products.write`; PUT
  replaces the list, audited).
* The name is chosen **when the line is priced**, by the customer's language, falling back to the
  base name; it is stored in the line's `description` like before. The PDF therefore prints the
  snapshot and never changes after sending (documents are snapshots), rather than looking up
  translations at render time.

**Consequences.** Configurators can push quotations without duplicates; sales still reviews and
sends them. The pricing rules stay in one tested crate. Positions in the API keep the JSON flat and
generator-friendly (ADR 0005).

**Later.**
* Orders and the order confirmation do not carry sections, parents or comments yet; converting a
  configured quotation gives a flat order.
* The Flutter quotation screen shows the lines flat; editing the structure in the app is not built.
* A default percentage on the product (so a manually added option is priced automatically) and a
  configurator-side "price on request" flag are not part of this phase.
