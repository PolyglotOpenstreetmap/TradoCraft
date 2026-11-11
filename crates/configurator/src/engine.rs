//! The evaluator: a pure function from a compiled definition, the user's inputs and a catalogue
//! snapshot to every value (with where it came from), the generated line tree, the issues and a
//! trace per line. No database access; the API builds the [`Catalog`].

use std::collections::{BTreeMap, HashMap};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::compile::{compile, CRule, CodeSpec, Compiled, Node, NodeKind, ScopeKey};
use crate::definition::{Definition, DefinitionError, Field, FieldType, PercentBase, Severity};
use crate::expr::value::parse_decimal;
use crate::expr::{self, Budget, Env, Expr, ExprError, Host, Value};

/// Steps one evaluation may take over all its formulas.
pub const DEFAULT_BUDGET: u64 = 2_000_000;

/// A user's own value for an overridable computed value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Pin {
    /// The computed value's name.
    pub name: String,
    /// For an item value: the item table ...
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    /// ... and the row (0-based).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row: Option<usize>,
    /// For a group value: the group name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub value: serde_json::Value,
    /// The calculated value when the pin was made; when the calculation now differs the pin is
    /// reported as stale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calculated: Option<serde_json::Value>,
}

/// What the user entered. Unknown names are ignored.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Inputs {
    /// Project field values by name.
    #[serde(default)]
    #[schema(value_type = Object)]
    pub project: BTreeMap<String, serde_json::Value>,
    /// Group field values by group name, then field name.
    #[serde(default)]
    #[schema(value_type = Object)]
    pub groups: BTreeMap<String, BTreeMap<String, serde_json::Value>>,
    /// Rows per item table: field values by name.
    #[serde(default)]
    #[schema(value_type = Object)]
    pub items: BTreeMap<String, Vec<BTreeMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub pins: Vec<Pin>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatalogProduct {
    /// The product's id (absent in unit tests).
    pub id: Option<uuid::Uuid>,
    pub code: String,
    pub name: String,
    pub family: Option<String>,
    pub unit: String,
    pub active: bool,
    pub attributes: BTreeMap<String, Value>,
}

/// A snapshot of the product catalogue: codes, families, active flags and attributes.
#[derive(Debug, Clone, Default)]
pub struct Catalog {
    pub products: HashMap<String, CatalogProduct>,
}

impl Catalog {
    pub fn add(&mut self, p: CatalogProduct) {
        self.products.insert(p.code.clone(), p);
    }

    pub fn record(&self, code: &str) -> Value {
        match self.products.get(code) {
            None => Value::Null,
            Some(p) => {
                let mut m = BTreeMap::new();
                m.insert("code".into(), Value::Str(p.code.clone()));
                m.insert("name".into(), Value::Str(p.name.clone()));
                m.insert(
                    "family".into(),
                    p.family.clone().map(Value::Str).unwrap_or(Value::Null),
                );
                m.insert("unit".into(), Value::Str(p.unit.clone()));
                m.insert("active".into(), Value::Bool(p.active));
                m.insert("attributes".into(), Value::Record(p.attributes.clone()));
                Value::Record(m)
            }
        }
    }

    fn active_in_family(&self, family: Option<&str>) -> Vec<&CatalogProduct> {
        let mut v: Vec<&CatalogProduct> = self
            .products
            .values()
            .filter(|p| p.active && family.is_none_or(|f| p.family.as_deref() == Some(f)))
            .collect();
        v.sort_by(|a, b| a.code.cmp(&b.code));
        v
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ValueSource {
    /// Entered by the user.
    Input,
    /// The field's default.
    Default,
    /// No value.
    Empty,
    /// Hidden by `visible_if`: the default (or nothing) is used.
    Hidden,
    /// Calculated by its formula.
    Calculated,
    /// Pinned by the user; the calculation still gives the same value it gave when pinned.
    Pinned,
    /// Pinned, but the calculation changed since (or the pin is no longer an allowed option).
    Stale,
    /// The formula failed (see the issues).
    Error,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ValueResult {
    pub name: String,
    /// field | computed
    pub kind: String,
    pub value: serde_json::Value,
    pub source: ValueSource,
    pub visible: bool,
    pub enabled: bool,
    /// What the formula gives, when the value is pinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated: Option<serde_json::Value>,
    /// Allowed values (choices, product pickers, alternatives of an overridable value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct GroupResult {
    pub name: String,
    pub values: Vec<ValueResult>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ItemResult {
    /// 1-based row number.
    pub index: usize,
    pub group: String,
    pub label: String,
    pub values: Vec<ValueResult>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ItemTableResult {
    pub table: String,
    pub rows: Vec<ItemResult>,
}

/// Where a line comes from: the rule and, per contributing scope, the values it read.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TraceSource {
    /// `project`, `groups[Hall 1]` or `segments[0]`.
    pub scope: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The `for_each` element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub each: Option<serde_json::Value>,
    /// The names the rule's formulas read, with their values.
    #[schema(value_type = Object)]
    pub values: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Trace {
    pub rule: String,
    /// JSON path of the rule in the definition.
    pub rule_path: String,
    pub sources: Vec<TraceSource>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Line {
    pub code: String,
    /// The catalogue product with this code, if there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<uuid::Uuid>,
    /// Its name: translated when the evaluation was asked in a language it has a translation
    /// for, else the base name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[schema(value_type = String)]
    pub quantity: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<String>)]
    pub percent_of_parent: Option<Decimal>,
    /// parent_list_unit_price | parent_net_unit_price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_base: Option<String>,
    pub rule: String,
    #[schema(no_recursion)]
    pub children: Vec<Line>,
    pub trace: Trace,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Issue {
    pub severity: Severity,
    /// The validation id, or a built-in code: required, out_of_range, invalid_value,
    /// evaluation_error, pin_stale, pin_ignored, rows, no_code, unknown_product.
    pub code: String,
    pub message: String,
    pub scope: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Evaluation {
    pub project: Vec<ValueResult>,
    pub groups: Vec<GroupResult>,
    pub items: Vec<ItemTableResult>,
    pub lines: Vec<Line>,
    pub issues: Vec<Issue>,
    /// Formula steps used.
    pub steps: u64,
}

impl Evaluation {
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Error)
    }
}

impl Evaluation {
    /// The product ids of all lines (children included), without duplicates.
    pub fn product_ids(&self) -> Vec<uuid::Uuid> {
        fn walk(lines: &[Line], out: &mut Vec<uuid::Uuid>) {
            for l in lines {
                if let Some(id) = l.product_id {
                    if !out.contains(&id) {
                        out.push(id);
                    }
                }
                walk(&l.children, out);
            }
        }
        let mut out = Vec::new();
        walk(&self.lines, &mut out);
        out
    }

    /// Replaces the product names of the lines by the given (translated) names.
    pub fn localize(&mut self, names: &HashMap<uuid::Uuid, String>) {
        fn walk(lines: &mut [Line], names: &HashMap<uuid::Uuid, String>) {
            for l in lines {
                if let Some(n) = l.product_id.and_then(|id| names.get(&id)) {
                    l.product_name = Some(n.clone());
                }
                walk(&mut l.children, names);
            }
        }
        walk(&mut self.lines, names);
    }
}

/// Compiles and evaluates; definition problems are returned instead.
pub fn evaluate_definition(
    def: &Definition,
    inputs: &Inputs,
    catalog: &Catalog,
) -> Result<Evaluation, Vec<DefinitionError>> {
    Ok(evaluate(&compile(def)?, inputs, catalog))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Inst {
    Project,
    Group(usize),
    Item(usize, usize),
}

#[derive(Default)]
struct ItemState {
    group: usize,
    values: BTreeMap<String, Value>,
    label: Option<String>,
    results: BTreeMap<usize, ValueResult>,
}

struct GroupState {
    name: String,
    values: BTreeMap<String, Value>,
    results: BTreeMap<usize, ValueResult>,
}

struct EngineHost<'a> {
    c: &'a Compiled,
    catalog: &'a Catalog,
}

impl Host for EngineHost<'_> {
    fn lookup(
        &self,
        table: &str,
        keys: &BTreeMap<String, Value>,
        offset: i64,
    ) -> Result<Value, String> {
        self.c
            .tables
            .get(table)
            .ok_or_else(|| format!("unknown table {table}"))?
            .lookup(keys, offset)
    }
    fn rows(&self, table: &str) -> Result<Value, String> {
        Ok(self
            .c
            .tables
            .get(table)
            .ok_or_else(|| format!("unknown table {table}"))?
            .rows_value())
    }
    fn product(&self, code: &str) -> Value {
        self.catalog.record(code)
    }
}

struct Engine<'a> {
    c: &'a Compiled,
    catalog: &'a Catalog,
    inputs: &'a Inputs,
    budget: Budget,
    project: BTreeMap<String, Value>,
    project_results: BTreeMap<usize, ValueResult>,
    groups: Vec<GroupState>,
    items: Vec<Vec<ItemState>>,
    issues: Vec<Issue>,
}

struct ScopeEnv<'a, 'b> {
    e: &'b Engine<'a>,
    inst: Inst,
}

impl ScopeEnv<'_, '_> {
    fn item_record(&self, t: usize, r: usize) -> Value {
        let it = &self.e.items[t][r];
        let mut m = it.values.clone();
        m.insert("index".into(), Value::Int(r as i64 + 1));
        m.insert(
            "group_name".into(),
            Value::Str(
                self.e
                    .groups
                    .get(it.group)
                    .map(|g| g.name.clone())
                    .unwrap_or_default(),
            ),
        );
        m.insert(
            "label".into(),
            it.label.clone().map(Value::Str).unwrap_or(Value::Null),
        );
        Value::Record(m)
    }

    fn table_list(&self, n: &str, group: Option<usize>) -> Option<Value> {
        let t = self.e.c.def.item_tables.iter().position(|x| x.name == n)?;
        Some(Value::List(
            (0..self.e.items[t].len())
                .filter(|r| group.is_none_or(|g| self.e.items[t][*r].group == g))
                .map(|r| self.item_record(t, r))
                .collect(),
        ))
    }
}

impl Env for ScopeEnv<'_, '_> {
    fn get(&self, n: &str) -> Option<Value> {
        let e = self.e;
        let single = (e.c.def.item_tables.len() == 1).then(|| e.c.def.item_tables[0].name.as_str());
        match self.inst {
            Inst::Item(t, r) => {
                let it = &e.items[t][r];
                it.values
                    .get(n)
                    .or_else(|| e.groups.get(it.group).and_then(|g| g.values.get(n)))
                    .or_else(|| e.project.get(n))
                    .cloned()
                    .or_else(|| match n {
                        "index" => Some(Value::Int(r as i64 + 1)),
                        "group_name" => Some(Value::Str(
                            e.groups
                                .get(it.group)
                                .map(|g| g.name.clone())
                                .unwrap_or_default(),
                        )),
                        "label" => Some(it.label.clone().map(Value::Str).unwrap_or(Value::Null)),
                        _ => None,
                    })
            }
            Inst::Group(g) => e.groups[g]
                .values
                .get(n)
                .or_else(|| e.project.get(n))
                .cloned()
                .or_else(|| self.table_list(n, Some(g)))
                .or_else(|| match n {
                    "items" => single.and_then(|s| self.table_list(s, Some(g))),
                    "group_name" => Some(Value::Str(e.groups[g].name.clone())),
                    _ => None,
                }),
            Inst::Project => e
                .project
                .get(n)
                .cloned()
                .or_else(|| self.table_list(n, None))
                .or_else(|| match n {
                    "groups" => Some(Value::List(
                        (0..e.groups.len())
                            .map(|g| {
                                let mut m = e.groups[g].values.clone();
                                m.insert("group_name".into(), Value::Str(e.groups[g].name.clone()));
                                if let Some(s) = single {
                                    if let Some(l) = self.table_list(s, Some(g)) {
                                        m.insert("items".into(), l);
                                    }
                                }
                                Value::Record(m)
                            })
                            .collect(),
                    )),
                    _ => None,
                }),
        }
    }
}

fn truthy(v: &Value) -> bool {
    matches!(v, Value::Bool(true))
}

fn coerce(f: &Field, v: &serde_json::Value) -> Result<Value, String> {
    use serde_json::Value as J;
    if v.is_null() {
        return Ok(Value::Null);
    }
    let bad = || format!("{v} is not a valid {} value", f.field_type.as_str());
    Ok(match f.field_type {
        FieldType::Number => match v {
            J::Number(n) => Value::Dec(parse_decimal(&n.to_string()).ok_or_else(bad)?),
            J::String(s) if s.trim().is_empty() => Value::Null,
            J::String(s) => Value::Dec(parse_decimal(s).ok_or_else(bad)?),
            _ => return Err(bad()),
        },
        FieldType::Integer => match v {
            J::String(s) if s.trim().is_empty() => Value::Null,
            J::String(s) => Value::Int(s.trim().parse().map_err(|_| bad())?),
            _ => Value::Int(Value::from_json(v).as_int().ok_or_else(bad)?),
        },
        FieldType::Boolean => match v {
            J::Bool(b) => Value::Bool(*b),
            J::String(s) if s == "true" => Value::Bool(true),
            J::String(s) if s == "false" => Value::Bool(false),
            _ => return Err(bad()),
        },
        FieldType::Text | FieldType::Product => match v {
            J::String(s) => Value::Str(s.clone()),
            J::Number(n) => Value::Str(n.to_string()),
            _ => return Err(bad()),
        },
        FieldType::Choice => match v {
            J::String(_) | J::Number(_) | J::Bool(_) => Value::from_json(v),
            _ => return Err(bad()),
        },
    })
}

fn fmt_list(v: &[Value]) -> String {
    v.iter().map(Value::to_text).collect::<Vec<_>>().join(", ")
}

/// Values in a trace: long lists are summarised.
fn trace_value(v: &Value) -> serde_json::Value {
    match v {
        Value::List(l) if l.len() > 20 => {
            serde_json::Value::String(format!("[{} elements]", l.len()))
        }
        other => other.to_json(),
    }
}

pub fn evaluate(c: &Compiled, inputs: &Inputs, catalog: &Catalog) -> Evaluation {
    let mut e = Engine {
        c,
        catalog,
        inputs,
        budget: Budget::new(DEFAULT_BUDGET),
        project: BTreeMap::new(),
        project_results: BTreeMap::new(),
        groups: Vec::new(),
        items: Vec::new(),
        issues: Vec::new(),
    };
    e.run()
}

impl<'a> Engine<'a> {
    fn scope_name(&self, inst: Inst) -> String {
        match inst {
            Inst::Project => "project".into(),
            Inst::Group(g) => format!("groups[{}]", self.groups[g].name),
            Inst::Item(t, r) => format!("{}[{r}]", self.c.def.item_tables[t].name),
        }
    }

    /// A human prefix for messages: the item label or the group name.
    fn where_(&self, inst: Inst) -> String {
        match inst {
            Inst::Project => String::new(),
            Inst::Group(g) if self.groups[g].name.is_empty() => String::new(),
            Inst::Group(g) => format!("{}: ", self.groups[g].name),
            Inst::Item(t, r) => format!("{}: ", self.item_label(t, r)),
        }
    }

    fn item_label(&self, t: usize, r: usize) -> String {
        self.items[t][r].label.clone().unwrap_or_else(|| {
            let tab = &self.c.def.item_tables[t];
            let l = if tab.label.is_empty() {
                &tab.name
            } else {
                &tab.label
            };
            format!("{l} {}", r + 1)
        })
    }

    fn issue(
        &mut self,
        severity: Severity,
        code: &str,
        message: String,
        inst: Inst,
        field: Option<&str>,
        rule: Option<&str>,
    ) {
        let scope = self.scope_name(inst);
        self.issues.push(Issue {
            severity,
            code: code.into(),
            message,
            scope,
            field: field.map(str::to_string),
            rule: rule.map(str::to_string),
        });
    }

    fn eval(&self, ex: &Expr, inst: Inst, locals: &[(String, Value)]) -> Result<Value, ExprError> {
        let env = ScopeEnv { e: self, inst };
        let host = EngineHost {
            c: self.c,
            catalog: self.catalog,
        };
        expr::eval_with(ex, locals, &env, &host, &self.budget)
    }

    fn render(
        &self,
        t: &expr::Template,
        inst: Inst,
        locals: &[(String, Value)],
    ) -> Result<String, ExprError> {
        let env = ScopeEnv { e: self, inst };
        let host = EngineHost {
            c: self.c,
            catalog: self.catalog,
        };
        t.render(locals, &env, &host, &self.budget)
    }

    /// Evaluates; a failure becomes an `evaluation_error` issue and null.
    fn eval_or_issue(
        &mut self,
        ex: &Expr,
        inst: Inst,
        locals: &[(String, Value)],
        what: &str,
        field: Option<&str>,
        rule: Option<&str>,
    ) -> (Value, bool) {
        match self.eval(ex, inst, locals) {
            Ok(v) => (v, true),
            Err(err) => {
                let msg = format!("{}{what}: {}", self.where_(inst), err.message);
                self.issue(Severity::Error, "evaluation_error", msg, inst, field, rule);
                (Value::Null, false)
            }
        }
    }

    fn setup_groups(&mut self) {
        let mut names: Vec<String> = Vec::new();
        let mut items: Vec<Vec<ItemState>> = Vec::new();
        for t in &self.c.def.item_tables {
            let rows = self
                .inputs
                .items
                .get(&t.name)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            let gf = t
                .group_field
                .as_ref()
                .and_then(|g| t.fields.iter().find(|f| &f.name == g));
            let mut states = Vec::new();
            for row in rows {
                let name = match gf {
                    None => String::new(),
                    Some(f) => row
                        .get(&f.name)
                        .filter(|v| !v.is_null())
                        .or(f.default.as_ref())
                        .map(|v| Value::from_json(v).to_text().trim().to_string())
                        .unwrap_or_default(),
                };
                let g = match names.iter().position(|n| n == &name) {
                    Some(g) => g,
                    None => {
                        names.push(name);
                        names.len() - 1
                    }
                };
                states.push(ItemState {
                    group: g,
                    ..Default::default()
                });
            }
            items.push(states);
        }
        // Only the groups the items name: inputs of a group whose last item was removed or
        // renamed are kept in the inputs but not evaluated.
        self.groups = names
            .into_iter()
            .map(|name| GroupState {
                name,
                values: BTreeMap::new(),
                results: BTreeMap::new(),
            })
            .collect();
        self.items = items;
    }

    fn run(&mut self) -> Evaluation {
        self.setup_groups();
        for (t, tab) in self.c.def.item_tables.iter().enumerate() {
            let n = self.items[t].len() as u32;
            let label = if tab.label.is_empty() {
                &tab.name
            } else {
                &tab.label
            };
            if tab.min_rows.is_some_and(|m| n < m) || tab.max_rows.is_some_and(|m| n > m) {
                let msg = format!(
                    "{label}: {n} row(s), allowed {}..{}",
                    tab.min_rows.unwrap_or(0),
                    tab.max_rows.map(|m| m.to_string()).unwrap_or_default()
                );
                self.issue(Severity::Error, "rows", msg, Inst::Project, None, None);
            }
        }
        let c = self.c;
        for &i in &c.order {
            let node = &c.nodes[i];
            let insts: Vec<Inst> = match node.scope {
                ScopeKey::Project => vec![Inst::Project],
                ScopeKey::Group => (0..self.groups.len()).map(Inst::Group).collect(),
                ScopeKey::Item(t) => (0..self.items[t].len()).map(|r| Inst::Item(t, r)).collect(),
            };
            for inst in insts {
                let (value, result) = match &node.kind {
                    NodeKind::Field(f) => self.eval_field(node, f, inst),
                    NodeKind::Computed(_) => self.eval_computed(node, inst),
                };
                self.store(inst, i, &node.name, value, result);
            }
        }
        // Item labels, once everything is known.
        for t in 0..self.items.len() {
            for r in 0..self.items[t].len() {
                if let Some(tpl) = &c.item_labels[t] {
                    let label = match self.render(tpl, Inst::Item(t, r), &[]) {
                        Ok(s) if !s.trim().is_empty() => Some(s),
                        Ok(_) => None,
                        Err(err) => {
                            let msg = format!("item label: {}", err.message);
                            self.issue(
                                Severity::Error,
                                "evaluation_error",
                                msg,
                                Inst::Item(t, r),
                                None,
                                None,
                            );
                            None
                        }
                    };
                    self.items[t][r].label = label;
                }
            }
        }
        self.run_validations();
        let mut lines = Vec::new();
        for rule in &c.rules {
            let mut generated = Vec::new();
            for inst in self.instances(rule.scope) {
                self.gen_rule(rule, inst, &[], &mut generated);
            }
            if rule.merge_identical {
                generated = merge(generated);
            }
            lines.extend(generated.into_iter().map(|g| g.line));
        }
        self.check_codes(&lines);
        Evaluation {
            project: std::mem::take(&mut self.project_results)
                .into_values()
                .collect(),
            groups: self
                .groups
                .iter_mut()
                .map(|g| GroupResult {
                    name: g.name.clone(),
                    values: std::mem::take(&mut g.results).into_values().collect(),
                })
                .collect(),
            items: (0..self.items.len())
                .map(|t| ItemTableResult {
                    table: self.c.def.item_tables[t].name.clone(),
                    rows: (0..self.items[t].len())
                        .map(|r| ItemResult {
                            index: r + 1,
                            group: self
                                .groups
                                .get(self.items[t][r].group)
                                .map(|g| g.name.clone())
                                .unwrap_or_default(),
                            label: self.item_label(t, r),
                            values: std::mem::take(&mut self.items[t][r].results)
                                .into_values()
                                .collect(),
                        })
                        .collect(),
                })
                .collect(),
            lines,
            issues: std::mem::take(&mut self.issues),
            steps: self.budget.used(),
        }
    }

    fn instances(&self, scope: ScopeKey) -> Vec<Inst> {
        match scope {
            ScopeKey::Project => vec![Inst::Project],
            ScopeKey::Group => (0..self.groups.len()).map(Inst::Group).collect(),
            ScopeKey::Item(t) => (0..self.items[t].len()).map(|r| Inst::Item(t, r)).collect(),
        }
    }

    fn store(&mut self, inst: Inst, i: usize, name: &str, value: Value, result: ValueResult) {
        match inst {
            Inst::Project => {
                self.project.insert(name.into(), value);
                self.project_results.insert(i, result);
            }
            Inst::Group(g) => {
                self.groups[g].values.insert(name.into(), value);
                self.groups[g].results.insert(i, result);
            }
            Inst::Item(t, r) => {
                self.items[t][r].values.insert(name.into(), value);
                self.items[t][r].results.insert(i, result);
            }
        }
    }

    fn raw_input(&self, inst: Inst, name: &str) -> Option<&'a serde_json::Value> {
        let inputs = self.inputs;
        match inst {
            Inst::Project => inputs.project.get(name),
            Inst::Group(g) => inputs
                .groups
                .get(&self.groups[g].name)
                .and_then(|m| m.get(name)),
            Inst::Item(t, r) => inputs
                .items
                .get(&self.c.def.item_tables[t].name)
                .and_then(|rows| rows.get(r))
                .and_then(|m| m.get(name)),
        }
        .filter(|v| !v.is_null())
    }

    fn condition(&mut self, ex: &Option<Expr>, inst: Inst, what: &str, field: &str) -> bool {
        match ex {
            None => true,
            Some(ex) => {
                let (v, ok) = self.eval_or_issue(ex, inst, &[], what, Some(field), None);
                !ok || truthy(&v)
            }
        }
    }

    fn choice_options(&mut self, node: &Node, f: &Field, inst: Inst) -> Option<Vec<Value>> {
        match f.field_type {
            FieldType::Choice => {
                if let Some(of) = &f.options_from {
                    if let (Some(t), Some(col)) = (&of.table, &of.column) {
                        let mut out: Vec<Value> = Vec::new();
                        if let Some(tab) = self.c.tables.get(t) {
                            for row in &tab.rows {
                                if let Some(v) = row.get(col) {
                                    if !v.is_null() && !out.iter().any(|x| x.loose_eq(v)) {
                                        out.push(v.clone());
                                    }
                                }
                            }
                        }
                        return Some(out);
                    }
                    return Some(
                        self.catalog
                            .active_in_family(of.family.as_deref())
                            .into_iter()
                            .map(|p| Value::Str(p.code.clone()))
                            .collect(),
                    );
                }
                Some(
                    f.options
                        .iter()
                        .map(|o| Value::from_json(&o.value))
                        .collect(),
                )
            }
            FieldType::Product => {
                let candidates: Vec<String> = self
                    .catalog
                    .active_in_family(f.family.as_deref())
                    .into_iter()
                    .map(|p| p.code.clone())
                    .collect();
                let mut out = Vec::new();
                for code in candidates {
                    let keep = match &node.product_filter {
                        None => true,
                        Some(ex) => {
                            let locals = [("p".to_string(), self.catalog.record(&code))];
                            let (v, ok) = self.eval_or_issue(
                                ex,
                                inst,
                                &locals,
                                &format!("{} filter", node.label()),
                                Some(&node.name),
                                None,
                            );
                            if !ok {
                                return Some(out);
                            }
                            truthy(&v)
                        }
                    };
                    if keep {
                        out.push(Value::Str(code));
                    }
                }
                Some(out)
            }
            _ => None,
        }
    }

    fn eval_field(&mut self, node: &Node, f: &Field, inst: Inst) -> (Value, ValueResult) {
        let label = node.label().to_string();
        let visible = self.condition(
            &node.visible_if,
            inst,
            &format!("{label} (visible_if)"),
            &node.name,
        );
        let enabled = self.condition(
            &node.enabled_if,
            inst,
            &format!("{label} (enabled_if)"),
            &node.name,
        );
        let options = self.choice_options(node, f, inst);

        let default = |e: &mut Self| -> (Value, bool) {
            if let Some(d) = &f.default {
                return (coerce(f, d).unwrap_or(Value::Null), true);
            }
            if let Some(ex) = &node.default_expr {
                let (v, _) = e.eval_or_issue(
                    ex,
                    inst,
                    &[],
                    &format!("{label} (default)"),
                    Some(&node.name),
                    None,
                );
                let v = match (&v, f.field_type) {
                    (Value::Int(i), FieldType::Number) => Value::Dec(Decimal::from(*i)),
                    _ => v,
                };
                return (v, true);
            }
            if node.overrides_project {
                if let Some(v) = e.project.get(&node.name) {
                    return (v.clone(), true);
                }
            }
            (Value::Null, false)
        };

        let (mut value, mut source) = if !visible {
            (default(self).0, ValueSource::Hidden)
        } else {
            match self.raw_input(inst, &node.name).filter(|_| enabled) {
                Some(raw) => match coerce(f, raw) {
                    Ok(v) if !v.is_null() => (v, ValueSource::Input),
                    Ok(_) => {
                        let (v, _) = default(self);
                        let s = if v.is_null() {
                            ValueSource::Empty
                        } else {
                            ValueSource::Default
                        };
                        (v, s)
                    }
                    Err(m) => {
                        let msg = format!("{}{label}: {m}", self.where_(inst));
                        self.issue(
                            Severity::Error,
                            "invalid_value",
                            msg,
                            inst,
                            Some(&node.name),
                            None,
                        );
                        (Value::Null, ValueSource::Error)
                    }
                },
                None => {
                    let (v, _) = default(self);
                    let s = if v.is_null() {
                        ValueSource::Empty
                    } else {
                        ValueSource::Default
                    };
                    (v, s)
                }
            }
        };

        if visible && source != ValueSource::Error {
            let w = self.where_(inst);
            if value.is_null() {
                if f.required {
                    self.issue(
                        Severity::Error,
                        "required",
                        format!("{w}{label} is required"),
                        inst,
                        Some(&node.name),
                        None,
                    );
                }
            } else {
                if let Some(n) = value.as_decimal() {
                    let below = f.min.is_some_and(|m| n < m);
                    let above = f.max.is_some_and(|m| n > m);
                    if below || above {
                        let unit = f
                            .unit
                            .as_deref()
                            .map(|u| format!(" {u}"))
                            .unwrap_or_default();
                        let range = format!(
                            "{}..{}",
                            f.min.map(|m| m.normalize().to_string()).unwrap_or_default(),
                            f.max.map(|m| m.normalize().to_string()).unwrap_or_default()
                        );
                        self.issue(
                            Severity::Error,
                            "out_of_range",
                            format!("{w}{label} must be within {range}{unit}"),
                            inst,
                            Some(&node.name),
                            None,
                        );
                    }
                }
                if let Value::Str(s) = &value {
                    if f.max_length.is_some_and(|m| s.chars().count() > m as usize) {
                        self.issue(
                            Severity::Error,
                            "invalid_value",
                            format!(
                                "{w}{label} is longer than {} characters",
                                f.max_length.unwrap_or(0)
                            ),
                            inst,
                            Some(&node.name),
                            None,
                        );
                    }
                    if let Some(p) = &f.pattern {
                        if let Ok(re) = regex::Regex::new(p) {
                            if !re.is_match(s) {
                                self.issue(
                                    Severity::Error,
                                    "invalid_value",
                                    format!("{w}{label} does not have the expected form"),
                                    inst,
                                    Some(&node.name),
                                    None,
                                );
                            }
                        }
                    }
                }
                if let Some(opts) = &options {
                    if !opts.iter().any(|o| o.loose_eq(&value)) {
                        let msg = if f.field_type == FieldType::Product {
                            format!("{w}{label}: {} is not an allowed product", value.to_text())
                        } else {
                            format!(
                                "{w}{label}: {} is not one of {}",
                                value.to_text(),
                                fmt_list(opts)
                            )
                        };
                        self.issue(
                            Severity::Error,
                            "invalid_value",
                            msg,
                            inst,
                            Some(&node.name),
                            None,
                        );
                        source = ValueSource::Error;
                        value = Value::Null;
                    }
                }
            }
        }
        let result = ValueResult {
            name: node.name.clone(),
            kind: "field".into(),
            value: value.to_json(),
            source,
            visible,
            enabled,
            calculated: None,
            options: options.map(|o| o.iter().map(Value::to_json).collect()),
        };
        (value, result)
    }

    fn pin_for(&self, node: &Node, inst: Inst) -> Option<&'a Pin> {
        let inputs = self.inputs;
        inputs.pins.iter().find(|p| {
            p.name == node.name
                && match inst {
                    Inst::Project => p.table.is_none() && p.group.is_none(),
                    Inst::Group(g) => {
                        p.table.is_none()
                            && p.group.as_deref() == Some(self.groups[g].name.as_str())
                    }
                    Inst::Item(t, r) => {
                        p.table.as_deref() == Some(self.c.def.item_tables[t].name.as_str())
                            && p.row == Some(r)
                    }
                }
        })
    }

    fn eval_computed(&mut self, node: &Node, inst: Inst) -> (Value, ValueResult) {
        let NodeKind::Computed(c) = &node.kind else {
            unreachable!()
        };
        let label = node.label().to_string();
        let (calculated, ok) = match &node.expr {
            Some(ex) => self.eval_or_issue(ex, inst, &[], &label, Some(&node.name), None),
            None => (Value::Null, false),
        };
        let options = match (&node.options_from, c.overridable) {
            (Some(ex), true) => {
                let (v, _) = self.eval_or_issue(
                    ex,
                    inst,
                    &[],
                    &format!("{label} (options)"),
                    Some(&node.name),
                    None,
                );
                match v {
                    Value::List(l) => Some(l),
                    _ => Some(vec![]),
                }
            }
            _ => None,
        };
        let mut value = calculated.clone();
        let mut source = if ok {
            ValueSource::Calculated
        } else {
            ValueSource::Error
        };
        let mut shown_calc = None;
        if let Some(pin) = self.pin_for(node, inst) {
            if c.overridable {
                let mut pv = Value::from_json(&pin.value);
                if let (Value::Dec(_), Value::Int(i)) = (&calculated, &pv) {
                    pv = Value::Dec(Decimal::from(*i));
                }
                if let (Value::Dec(_), Value::Str(s)) = (&calculated, &pv) {
                    if let Some(d) = parse_decimal(s) {
                        pv = Value::Dec(d);
                    }
                }
                let changed = pin
                    .calculated
                    .as_ref()
                    .is_some_and(|was| !Value::from_json(was).loose_eq(&calculated) && !matches!((Value::from_json(was), &calculated), (Value::Str(s), Value::Dec(d)) if parse_decimal(&s) == Some(*d)));
                let not_allowed = options
                    .as_ref()
                    .is_some_and(|o| !o.iter().any(|x| x.loose_eq(&pv)));
                let w = self.where_(inst);
                source = if changed || not_allowed {
                    let why = if not_allowed {
                        format!("{} is no longer an allowed value", pv.to_text())
                    } else {
                        format!("it now calculates to {}", calculated.to_text())
                    };
                    self.issue(
                        Severity::Warning,
                        "pin_stale",
                        format!("{w}{label} is set to {} by hand, but {why}", pv.to_text()),
                        inst,
                        Some(&node.name),
                        None,
                    );
                    ValueSource::Stale
                } else {
                    ValueSource::Pinned
                };
                shown_calc = Some(calculated.to_json());
                value = pv;
            } else {
                let w = self.where_(inst);
                self.issue(
                    Severity::Info,
                    "pin_ignored",
                    format!("{w}{label} cannot be set by hand; the pinned value is ignored"),
                    inst,
                    Some(&node.name),
                    None,
                );
            }
        }
        let result = ValueResult {
            name: node.name.clone(),
            kind: "computed".into(),
            value: value.to_json(),
            source,
            visible: c.show,
            enabled: c.overridable,
            calculated: shown_calc,
            options: options.map(|o| o.iter().map(Value::to_json).collect()),
        };
        (value, result)
    }

    fn run_validations(&mut self) {
        let c = self.c;
        for v in &c.validations {
            for inst in self.instances(v.scope) {
                let (r, ok) = self.eval_or_issue(
                    &v.when,
                    inst,
                    &[],
                    &format!("validation {}", v.id),
                    v.field.as_deref(),
                    None,
                );
                if !ok || !truthy(&r) {
                    continue;
                }
                let msg = match self.render(&v.message, inst, &[]) {
                    Ok(m) => m,
                    Err(err) => format!("{} ({})", v.id, err.message),
                };
                let msg = format!("{}{msg}", self.where_(inst));
                self.issue(v.severity, &v.id, msg, inst, v.field.as_deref(), None);
            }
        }
    }

    fn opt_template(
        &mut self,
        t: &Option<expr::Template>,
        inst: Inst,
        locals: &[(String, Value)],
        rule: &CRule,
        what: &str,
    ) -> Option<String> {
        let t = t.as_ref()?;
        match self.render(t, inst, locals) {
            Ok(s) if s.trim().is_empty() => None,
            Ok(s) => Some(s.trim().to_string()),
            Err(err) => {
                let msg = format!(
                    "{}rule {} ({what}): {}",
                    self.where_(inst),
                    rule.id,
                    err.message
                );
                self.issue(
                    Severity::Error,
                    "evaluation_error",
                    msg,
                    inst,
                    None,
                    Some(&rule.id),
                );
                None
            }
        }
    }

    fn inst_label(&self, inst: Inst) -> Option<String> {
        match inst {
            Inst::Project => None,
            Inst::Group(g) => Some(self.groups[g].name.clone()).filter(|n| !n.is_empty()),
            Inst::Item(t, r) => Some(self.item_label(t, r)),
        }
    }

    fn gen_rule(
        &mut self,
        rule: &CRule,
        inst: Inst,
        outer: &[(String, Value)],
        out: &mut Vec<Generated>,
    ) {
        let rid = Some(rule.id.as_str());
        if let Some(w) = &rule.when {
            let (v, ok) = self.eval_or_issue(
                w,
                inst,
                outer,
                &format!("rule {} (when)", rule.id),
                None,
                rid,
            );
            if !ok || !truthy(&v) {
                return;
            }
        }
        let elements: Vec<Option<Value>> = match &rule.for_each {
            None => vec![None],
            Some(fe) => {
                let (v, _) = self.eval_or_issue(
                    fe,
                    inst,
                    outer,
                    &format!("rule {} (for_each)", rule.id),
                    None,
                    rid,
                );
                match v {
                    Value::List(l) => l.into_iter().map(Some).collect(),
                    Value::Null => vec![],
                    other => vec![Some(other)],
                }
            }
        };
        for (k, each) in elements.into_iter().enumerate() {
            let mut locals = outer.to_vec();
            if let Some(each) = &each {
                locals.push(("each".into(), each.clone()));
                locals.push(("each_index".into(), Value::Int(k as i64 + 1)));
            }
            let code = match &rule.code {
                CodeSpec::Expr(ex) => self
                    .eval_or_issue(
                        ex,
                        inst,
                        &locals,
                        &format!("rule {} (code)", rule.id),
                        None,
                        rid,
                    )
                    .0
                    .to_text(),
                CodeSpec::Template(t) => match self.render(t, inst, &locals) {
                    Ok(s) => s,
                    Err(err) => {
                        let msg = format!(
                            "{}rule {} (code): {}",
                            self.where_(inst),
                            rule.id,
                            err.message
                        );
                        self.issue(Severity::Error, "evaluation_error", msg, inst, None, rid);
                        continue;
                    }
                },
            };
            let code = code.trim().to_string();
            if code.is_empty() {
                let msg = format!("{}rule {} gave no product code", self.where_(inst), rule.id);
                self.issue(Severity::Warning, "no_code", msg, inst, None, rid);
                continue;
            }
            let quantity = match &rule.quantity {
                None => Decimal::ONE,
                Some(q) => {
                    let (v, _) = self.eval_or_issue(
                        q,
                        inst,
                        &locals,
                        &format!("rule {} (quantity)", rule.id),
                        None,
                        rid,
                    );
                    match v.as_decimal() {
                        Some(d) => d,
                        None => continue,
                    }
                }
            };
            if quantity <= Decimal::ZERO {
                continue;
            }
            // Without a percentage the line would be quoted at its own list price: an error.
            let percent = match &rule.percent_of_parent {
                None => None,
                Some(p) => {
                    let (v, ok) = self.eval_or_issue(
                        p,
                        inst,
                        &locals,
                        &format!("rule {} (percent_of_parent)", rule.id),
                        None,
                        rid,
                    );
                    match v.as_decimal() {
                        Some(d) => Some(d),
                        None => {
                            if ok {
                                let msg = format!(
                                    "{}rule {} (percent_of_parent) gave no number",
                                    self.where_(inst),
                                    rule.id
                                );
                                self.issue(
                                    Severity::Error,
                                    "evaluation_error",
                                    msg,
                                    inst,
                                    None,
                                    rid,
                                );
                            }
                            continue;
                        }
                    }
                }
            };
            let section = self
                .opt_template(&rule.section, inst, &locals, rule, "section")
                .or_else(|| match inst {
                    Inst::Group(_) | Inst::Item(..) if outer.iter().all(|(n, _)| n != "parent") => {
                        self.inst_label_group(inst)
                    }
                    _ => None,
                });
            let description =
                self.opt_template(&rule.description, inst, &locals, rule, "description");
            let comment = self.opt_template(&rule.comment, inst, &locals, rule, "comment");

            // Trace: the values the rule read.
            let env_values: BTreeMap<String, serde_json::Value> = {
                let env = ScopeEnv { e: self, inst };
                rule.names()
                    .into_iter()
                    .filter_map(|n| {
                        locals
                            .iter()
                            .rev()
                            .find(|(x, _)| *x == n)
                            .map(|(_, v)| v.clone())
                            .or_else(|| env.get(&n))
                            .map(|v| (n, trace_value(&v)))
                    })
                    .collect()
            };
            let mut child_locals = locals.clone();
            child_locals.retain(|(n, _)| n != "parent");
            let mut parent = BTreeMap::new();
            parent.insert("code".to_string(), Value::Str(code.clone()));
            parent.insert("quantity".to_string(), Value::Dec(quantity));
            child_locals.push(("parent".into(), Value::Record(parent)));
            let mut children = Vec::new();
            for c in &rule.children {
                self.gen_rule(c, inst, &child_locals, &mut children);
            }
            out.push(Generated {
                label: self.inst_label(inst),
                line: Line {
                    product_id: self.catalog.products.get(&code).and_then(|p| p.id),
                    product_name: self.catalog.products.get(&code).map(|p| p.name.clone()),
                    code,
                    quantity,
                    section,
                    description,
                    comment,
                    percent_of_parent: percent,
                    percent_base: rule.percent_base.map(|b| match b {
                        PercentBase::ParentListUnitPrice => "parent_list_unit_price".to_string(),
                        PercentBase::ParentNetUnitPrice => "parent_net_unit_price".to_string(),
                    }),
                    rule: rule.id.clone(),
                    children: children.into_iter().map(|g| g.line).collect(),
                    trace: Trace {
                        rule: rule.id.clone(),
                        rule_path: rule.path.clone(),
                        sources: vec![TraceSource {
                            scope: self.scope_name(inst),
                            label: self.inst_label(inst),
                            each: each.as_ref().map(Value::to_json),
                            values: env_values,
                        }],
                    },
                },
            });
        }
    }

    fn inst_label_group(&self, inst: Inst) -> Option<String> {
        let g = match inst {
            Inst::Group(g) => g,
            Inst::Item(t, r) => self.items[t][r].group,
            Inst::Project => return None,
        };
        Some(self.groups[g].name.clone()).filter(|n| !n.is_empty())
    }

    fn check_codes(&mut self, lines: &[Line]) {
        let mut unknown: Vec<(String, String)> = Vec::new();
        fn walk(lines: &[Line], c: &Catalog, out: &mut Vec<(String, String)>) {
            for l in lines {
                if !c.products.get(&l.code).is_some_and(|p| p.active)
                    && !out.iter().any(|(x, _)| x == &l.code)
                {
                    out.push((l.code.clone(), l.rule.clone()));
                }
                walk(&l.children, c, out);
            }
        }
        walk(lines, self.catalog, &mut unknown);
        for (code, rule) in unknown {
            self.issue(
                Severity::Warning,
                "unknown_product",
                format!("no active product with code {code}; the configuration cannot be quoted"),
                Inst::Project,
                None,
                Some(&rule),
            );
        }
    }
}

struct Generated {
    label: Option<String>,
    line: Line,
}

fn line_key(l: &Line, with_qty: bool) -> String {
    let children: Vec<String> = l.children.iter().map(|c| line_key(c, false)).collect();
    format!(
        "{}|{:?}|{:?}|{:?}|{:?}|{:?}|{}|[{}]",
        l.code,
        l.section,
        l.description,
        l.comment,
        l.percent_of_parent.map(|d| d.normalize()),
        l.percent_base,
        if with_qty {
            l.quantity.normalize().to_string()
        } else {
            String::new()
        },
        children.join(",")
    )
}

fn add_quantities(into: &mut Line, from: &Line) {
    into.quantity += from.quantity;
    for (a, b) in into.children.iter_mut().zip(&from.children) {
        add_quantities(a, b);
    }
}

/// Collapses lines of one rule with the same code, texts and children; the quantities add up
/// and the comment lists the items (or groups) covered.
fn merge(lines: Vec<Generated>) -> Vec<Generated> {
    let mut out: Vec<(String, Generated, Vec<String>)> = Vec::new();
    for g in lines {
        let key = line_key(&g.line, false);
        match out.iter_mut().find(|(k, _, _)| *k == key) {
            Some((_, m, labels)) => {
                add_quantities(&mut m.line, &g.line);
                m.line.trace.sources.extend(g.line.trace.sources);
                if let Some(l) = g.label {
                    if !labels.contains(&l) {
                        labels.push(l);
                    }
                }
            }
            None => {
                let labels = g.label.clone().into_iter().collect();
                out.push((key, g, labels));
            }
        }
    }
    out.into_iter()
        .map(|(_, mut g, labels)| {
            if !labels.is_empty() {
                let list = labels.join(", ");
                g.line.comment = Some(match g.line.comment.take() {
                    Some(c) => format!("{c} ({list})"),
                    None => list,
                });
            }
            g
        })
        .collect()
}
