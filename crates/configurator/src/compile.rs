//! Turns a [`Definition`] into a checked, ready-to-run form: every formula parsed and type
//! checked, tables typed, and the fields and computed values ordered by their dependencies.
//! All problems are collected with their JSON path.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::definition::*;
use crate::expr::{self, parse, Expr, ExprError, TableInfo, Template, Type, TypeEnv, Value};
use crate::tables::CompiledTable;

/// Deepest output rule nesting (as the quotation import accepts).
pub const MAX_RULE_DEPTH: usize = 8;

/// Names with a meaning of their own in formulas.
pub const RESERVED: &[&str] = &[
    "items",
    "groups",
    "group_name",
    "index",
    "label",
    "each",
    "each_index",
    "parent",
    "pi",
    "true",
    "false",
    "null",
    "and",
    "or",
    "not",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ScopeKey {
    Project,
    Group,
    /// An item table, by index.
    Item(usize),
}

/// A few dozen nodes per definition: the size difference does not matter.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum NodeKind {
    Field(Field),
    Computed(Computed),
}

/// A field or computed value of one scope.
#[derive(Debug, Clone)]
pub struct Node {
    pub scope: ScopeKey,
    pub name: String,
    pub path: String,
    pub kind: NodeKind,
    pub default_expr: Option<Expr>,
    pub visible_if: Option<Expr>,
    pub enabled_if: Option<Expr>,
    pub expr: Option<Expr>,
    pub options_from: Option<Expr>,
    pub product_filter: Option<Expr>,
    pub ty: Type,
    /// A group field with the name of a project field: defaults to the project value.
    pub overrides_project: bool,
}

impl Node {
    pub fn label(&self) -> &str {
        let l = match &self.kind {
            NodeKind::Field(f) => &f.label,
            NodeKind::Computed(c) => &c.label,
        };
        if l.is_empty() {
            &self.name
        } else {
            l
        }
    }

    fn exprs(&self) -> impl Iterator<Item = &Expr> {
        [
            &self.default_expr,
            &self.visible_if,
            &self.enabled_if,
            &self.expr,
            &self.options_from,
            &self.product_filter,
        ]
        .into_iter()
        .flatten()
    }
}

#[derive(Debug, Clone)]
pub enum CodeSpec {
    Expr(Expr),
    Template(Template),
}

#[derive(Debug, Clone)]
pub struct CRule {
    pub id: String,
    pub path: String,
    pub scope: ScopeKey,
    pub when: Option<Expr>,
    pub for_each: Option<Expr>,
    pub code: CodeSpec,
    pub quantity: Option<Expr>,
    pub description: Option<Template>,
    pub comment: Option<Template>,
    pub section: Option<Template>,
    pub merge_identical: bool,
    pub percent_of_parent: Option<Expr>,
    pub percent_base: Option<PercentBase>,
    pub children: Vec<CRule>,
}

impl CRule {
    /// Free names read by the rule's own formulas (for the trace).
    pub fn names(&self) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        for e in [
            &self.when,
            &self.for_each,
            &self.quantity,
            &self.percent_of_parent,
        ]
        .into_iter()
        .flatten()
        {
            out.extend(e.free_names());
        }
        match &self.code {
            CodeSpec::Expr(e) => out.extend(e.free_names()),
            CodeSpec::Template(t) => t.exprs().for_each(|e| out.extend(e.free_names())),
        }
        out
    }
}

#[derive(Debug, Clone)]
pub struct CValidation {
    pub id: String,
    pub path: String,
    pub scope: ScopeKey,
    pub when: Expr,
    pub severity: Severity,
    pub message: Template,
    pub field: Option<String>,
}

/// A checked definition, ready for [`crate::engine`].
#[derive(Debug, Clone)]
pub struct Compiled {
    pub def: Definition,
    pub tables: BTreeMap<String, CompiledTable>,
    pub nodes: Vec<Node>,
    /// Node indexes in evaluation order.
    pub order: Vec<usize>,
    pub rules: Vec<CRule>,
    pub validations: Vec<CValidation>,
    /// Per item table: the template naming an item.
    pub item_labels: Vec<Option<Template>>,
}

impl Compiled {
    pub fn node(&self, scope: ScopeKey, name: &str) -> Option<&Node> {
        self.nodes
            .iter()
            .find(|n| n.scope == scope && n.name == name)
    }

    pub fn names_in(&self, scope: ScopeKey) -> impl Iterator<Item = &Node> {
        self.nodes.iter().filter(move |n| n.scope == scope)
    }
}

pub fn valid_name(n: &str) -> bool {
    let mut chars = n.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn field_type(f: &Field, tables: &BTreeMap<String, CompiledTable>) -> Type {
    match f.field_type {
        FieldType::Number => Type::Dec,
        FieldType::Integer => Type::Int,
        FieldType::Boolean => Type::Bool,
        FieldType::Text | FieldType::Product => Type::Str,
        FieldType::Choice => {
            if let Some(of) = &f.options_from {
                if let (Some(t), Some(c)) = (&of.table, &of.column) {
                    return tables
                        .get(t)
                        .and_then(|t| t.column_type(c))
                        .unwrap_or(Type::Any);
                }
                return Type::Str;
            }
            f.options
                .iter()
                .map(|o| Type::of_value(&Value::from_json(&o.value)))
                .reduce(|a, b| expr::types::join(&a, &b))
                .unwrap_or(Type::Any)
        }
    }
}

struct State<'a> {
    def: &'a Definition,
    tables: BTreeMap<String, CompiledTable>,
    nodes: Vec<Node>,
    errors: Vec<DefinitionError>,
}

impl State<'_> {
    fn err(&mut self, path: impl Into<String>, msg: impl Into<String>) {
        self.errors.push(DefinitionError::new(path, msg));
    }

    fn expr_err(&mut self, path: &str, e: ExprError) {
        self.errors.push(DefinitionError {
            path: path.to_string(),
            message: e.message,
            start: Some(e.start),
            end: Some(e.end),
        });
    }

    fn parse(&mut self, path: String, src: &Option<String>) -> Option<Expr> {
        let src = src.as_ref()?;
        match parse(src) {
            Ok(e) => Some(e),
            Err(e) => {
                self.expr_err(&path, e);
                None
            }
        }
    }

    fn parse_template(&mut self, path: String, src: &Option<String>) -> Option<Template> {
        let src = src.as_ref()?;
        match Template::parse(src) {
            Ok(t) => Some(t),
            Err(e) => {
                self.expr_err(&path, e);
                None
            }
        }
    }
}

/// Type environment of a scope during compilation.
struct DefEnv<'a> {
    nodes: &'a [Node],
    def: &'a Definition,
    tables: &'a BTreeMap<String, CompiledTable>,
    scope: ScopeKey,
    /// `label` is only known once all values are computed (outputs, validations).
    with_label: bool,
}

impl DefEnv<'_> {
    fn scope_name(&self, scope: ScopeKey, n: &str) -> Option<Type> {
        self.nodes
            .iter()
            .find(|x| x.scope == scope && x.name == n)
            .map(|x| x.ty.clone())
    }

    fn record_of(&self, scope: ScopeKey) -> BTreeMap<String, Type> {
        let mut m: BTreeMap<String, Type> = self
            .nodes
            .iter()
            .filter(|x| x.scope == scope)
            .map(|x| (x.name.clone(), x.ty.clone()))
            .collect();
        m.insert("group_name".into(), Type::Str);
        if let ScopeKey::Item(_) = scope {
            m.insert("index".into(), Type::Int);
            m.insert("label".into(), Type::Str);
        }
        m
    }

    fn table_list(&self, n: &str) -> Option<Type> {
        let t = self.def.item_tables.iter().position(|t| t.name == n)?;
        Some(Type::List(Box::new(Type::Record(Some(
            self.record_of(ScopeKey::Item(t)),
        )))))
    }
}

impl TypeEnv for DefEnv<'_> {
    fn name(&self, n: &str) -> Option<Type> {
        match self.scope {
            ScopeKey::Item(t) => self
                .scope_name(ScopeKey::Item(t), n)
                .or_else(|| self.scope_name(ScopeKey::Group, n))
                .or_else(|| self.scope_name(ScopeKey::Project, n))
                .or(match n {
                    "index" => Some(Type::Int),
                    "group_name" => Some(Type::Str),
                    "label" if self.with_label => Some(Type::Str),
                    _ => None,
                }),
            ScopeKey::Group => self
                .scope_name(ScopeKey::Group, n)
                .or_else(|| self.scope_name(ScopeKey::Project, n))
                .or_else(|| self.table_list(n))
                .or_else(|| match n {
                    "items" if self.def.item_tables.len() == 1 => {
                        self.table_list(&self.def.item_tables[0].name)
                    }
                    "group_name" => Some(Type::Str),
                    _ => None,
                }),
            ScopeKey::Project => self
                .scope_name(ScopeKey::Project, n)
                .or_else(|| self.table_list(n))
                .or_else(|| match n {
                    "groups" => {
                        let mut r = self.record_of(ScopeKey::Group);
                        if self.def.item_tables.len() == 1 {
                            if let Some(t) = self.table_list(&self.def.item_tables[0].name) {
                                r.insert("items".into(), t);
                            }
                        }
                        Some(Type::List(Box::new(Type::Record(Some(r)))))
                    }
                    _ => None,
                }),
        }
    }

    fn table(&self, n: &str) -> Option<TableInfo> {
        self.tables.get(n).map(CompiledTable::info)
    }
}

/// Which nodes a name read in `scope` depends on.
fn resolve_deps(nodes: &[Node], def: &Definition, scope: ScopeKey, name: &str) -> Vec<usize> {
    let find = |s: ScopeKey| nodes.iter().position(|x| x.scope == s && x.name == name);
    let all_of = |s: ScopeKey| -> Vec<usize> {
        nodes
            .iter()
            .enumerate()
            .filter(|(_, x)| x.scope == s)
            .map(|(i, _)| i)
            .collect()
    };
    let table = def.item_tables.iter().position(|t| t.name == name);
    match scope {
        ScopeKey::Item(t) => find(ScopeKey::Item(t))
            .or_else(|| find(ScopeKey::Group))
            .or_else(|| find(ScopeKey::Project))
            .into_iter()
            .collect(),
        ScopeKey::Group => {
            if let Some(i) = find(ScopeKey::Group).or_else(|| find(ScopeKey::Project)) {
                return vec![i];
            }
            if let Some(t) = table {
                return all_of(ScopeKey::Item(t));
            }
            if name == "items" && def.item_tables.len() == 1 {
                return all_of(ScopeKey::Item(0));
            }
            vec![]
        }
        ScopeKey::Project => {
            if let Some(i) = find(ScopeKey::Project) {
                return vec![i];
            }
            if let Some(t) = table {
                return all_of(ScopeKey::Item(t));
            }
            if name == "groups" {
                let mut v = all_of(ScopeKey::Group);
                if def.item_tables.len() == 1 {
                    v.extend(all_of(ScopeKey::Item(0)));
                }
                return v;
            }
            vec![]
        }
    }
}

fn expect(st: &mut State, path: &str, e: &Expr, t: &Type, want: &str) {
    let ok = match want {
        "bool" => matches!(t, Type::Bool | Type::Any | Type::Null),
        "number" => matches!(t, Type::Int | Type::Dec | Type::Any | Type::Null),
        "list" => matches!(t, Type::List(_) | Type::Any | Type::Null),
        "text" => matches!(t, Type::Str | Type::Int | Type::Any | Type::Null),
        _ => true,
    };
    if !ok {
        let what = match want {
            "bool" => "a yes/no value",
            "number" => "a number",
            "list" => "a list",
            _ => "text",
        };
        st.expr_err(
            path,
            ExprError::new(format!("expected {what}, found {t}"), e.span),
        );
    }
}

fn check_literal(f: &Field, v: &serde_json::Value) -> Result<(), String> {
    use serde_json::Value as J;
    let ok = match f.field_type {
        FieldType::Number => match v {
            J::Number(_) => true,
            J::String(s) => expr::value::parse_decimal(s).is_some(),
            _ => false,
        },
        FieldType::Integer => v.as_i64().is_some(),
        FieldType::Boolean => v.is_boolean(),
        FieldType::Text | FieldType::Product => v.is_string(),
        FieldType::Choice => {
            if f.options.is_empty() {
                v.is_string() || v.is_number() || v.is_boolean()
            } else {
                let x = Value::from_json(v);
                f.options
                    .iter()
                    .any(|o| Value::from_json(&o.value).loose_eq(&x))
            }
        }
    };
    if v.is_null() || ok {
        Ok(())
    } else if f.field_type == FieldType::Choice && !f.options.is_empty() {
        Err(format!("{v} is not one of the options"))
    } else {
        Err(format!(
            "{v} is not a valid {} value",
            f.field_type.as_str()
        ))
    }
}

fn check_field(st: &mut State, f: &Field, path: &str) {
    let t = f.field_type;
    let mut not_for = |attr: &str, set: bool, allowed: &[FieldType]| {
        if set && !allowed.contains(&t) {
            let list: Vec<&str> = allowed.iter().map(|a| a.as_str()).collect();
            st.err(
                format!("{path}.{attr}"),
                format!("{attr} is only for {} fields", list.join(" and ")),
            );
        }
    };
    use FieldType::*;
    not_for("dimension", f.dimension.is_some(), &[Number]);
    not_for("unit", f.unit.is_some(), &[Number, Integer]);
    not_for("display_units", !f.display_units.is_empty(), &[Number]);
    not_for("min", f.min.is_some(), &[Number, Integer]);
    not_for("max", f.max.is_some(), &[Number, Integer]);
    not_for("step", f.step.is_some(), &[Number, Integer]);
    not_for("options", !f.options.is_empty(), &[Choice]);
    not_for("options_from", f.options_from.is_some(), &[Choice]);
    not_for("max_length", f.max_length.is_some(), &[Text]);
    not_for("pattern", f.pattern.is_some(), &[Text]);
    not_for("family", f.family.is_some(), &[Product]);
    not_for("product_filter", f.product_filter.is_some(), &[Product]);

    if let (Some(a), Some(b)) = (f.min, f.max) {
        if a > b {
            st.err(format!("{path}.min"), "min is larger than max");
        }
    }
    if let Some(dim) = f.dimension {
        let known: Vec<&str> = units(dim).iter().map(|(u, _)| *u).collect();
        match &f.unit {
            None => st.err(
                format!("{path}.unit"),
                "a number with a dimension needs a unit",
            ),
            Some(u) if !known.contains(&u.as_str()) => st.err(
                format!("{path}.unit"),
                format!(
                    "unit {u} is not a unit of this dimension ({})",
                    known.join(", ")
                ),
            ),
            _ => {}
        }
        for (i, u) in f.display_units.iter().enumerate() {
            if !known.contains(&u.as_str()) {
                st.err(
                    format!("{path}.display_units[{i}]"),
                    format!(
                        "unit {u} is not a unit of this dimension ({})",
                        known.join(", ")
                    ),
                );
            }
        }
    } else if !f.display_units.is_empty() {
        st.err(
            format!("{path}.display_units"),
            "display units need a dimension",
        );
    }
    if t == Choice {
        if f.options.is_empty() && f.options_from.is_none() {
            st.err(
                format!("{path}.options"),
                "a choice needs options or options_from",
            );
        }
        if !f.options.is_empty() && f.options_from.is_some() {
            st.err(
                format!("{path}.options_from"),
                "give options or options_from, not both",
            );
        }
        let mut seen: Vec<Value> = Vec::new();
        for (i, o) in f.options.iter().enumerate() {
            if !(o.value.is_string() || o.value.is_number() || o.value.is_boolean()) {
                st.err(
                    format!("{path}.options[{i}].value"),
                    "an option value must be text, a number or a boolean",
                );
            }
            let v = Value::from_json(&o.value);
            if seen.iter().any(|s| s.loose_eq(&v)) {
                st.err(
                    format!("{path}.options[{i}].value"),
                    format!("option {} is listed twice", o.value),
                );
            }
            seen.push(v);
        }
        if let Some(of) = &f.options_from {
            match (&of.table, &of.column, &of.family) {
                (Some(tn), Some(c), None) => match st.tables.get(tn) {
                    None => st.err(
                        format!("{path}.options_from.table"),
                        format!("unknown table {tn}"),
                    ),
                    Some(tab) if tab.column_type(c).is_none() => st.err(
                        format!("{path}.options_from.column"),
                        format!("table {tn} has no column {c}"),
                    ),
                    _ => {}
                },
                (None, None, Some(_)) => {}
                _ => st.err(
                    format!("{path}.options_from"),
                    "options_from takes either table and column, or family",
                ),
            }
        }
    }
    if let Some(p) = &f.pattern {
        if let Err(e) = regex::RegexBuilder::new(p).size_limit(1 << 20).build() {
            st.err(format!("{path}.pattern"), format!("invalid pattern: {e}"));
        }
    }
    if f.default.is_some() && f.default_expr.is_some() {
        st.err(
            format!("{path}.default_expr"),
            "give default or default_expr, not both",
        );
    }
    if let Some(d) = &f.default {
        if let Err(m) = check_literal(f, d) {
            st.err(format!("{path}.default"), m);
        } else if let (Some(n), true) = (
            Value::from_json(d).as_decimal(),
            matches!(t, Number | Integer),
        ) {
            if f.min.is_some_and(|m| n < m) || f.max.is_some_and(|m| n > m) {
                st.err(format!("{path}.default"), "the default is outside min..max");
            }
        }
    }
    if let Some(sec) = f.layout.as_ref().and_then(|l| l.section.as_ref()) {
        if !st.def.sections.iter().any(|s| &s.name == sec) {
            st.err(
                format!("{path}.layout.section"),
                format!("unknown layout section {sec}"),
            );
        }
    }
}

fn add_scope_nodes(
    st: &mut State,
    scope: ScopeKey,
    fields: &[Field],
    computed: &[Computed],
    base: &str,
    taken: &mut HashMap<String, String>,
) {
    let mut local: HashMap<String, String> = HashMap::new();
    for (i, f) in fields.iter().enumerate() {
        let path = format!("{base}.fields[{i}]");
        check_field(st, f, &path);
        add_node(
            st,
            scope,
            &f.name,
            &path,
            NodeKind::Field(f.clone()),
            &mut local,
            taken,
        );
    }
    for (i, c) in computed.iter().enumerate() {
        let path = format!("{base}.computed[{i}]");
        if c.options_from.is_some() && !c.overridable {
            st.err(
                format!("{path}.options_from"),
                "options_from is only for overridable values",
            );
        }
        add_node(
            st,
            scope,
            &c.name,
            &path,
            NodeKind::Computed(c.clone()),
            &mut local,
            taken,
        );
    }
}

fn add_node(
    st: &mut State,
    scope: ScopeKey,
    name: &str,
    path: &str,
    kind: NodeKind,
    local: &mut HashMap<String, String>,
    taken: &mut HashMap<String, String>,
) {
    if !valid_name(name) {
        st.err(
            format!("{path}.name"),
            format!("'{name}' is not a valid name (letters, digits, _; not starting with a digit)"),
        );
    } else if RESERVED.contains(&name) {
        st.err(
            format!("{path}.name"),
            format!("'{name}' is a reserved name"),
        );
    }
    if let Some(other) = local.get(name) {
        st.err(
            format!("{path}.name"),
            format!("{name} is already defined at {other}"),
        );
        return;
    }
    let overrides_project = matches!((&kind, scope), (NodeKind::Field(_), ScopeKey::Group))
        && st.nodes.iter().any(|n| {
            n.scope == ScopeKey::Project && n.name == name && matches!(n.kind, NodeKind::Field(_))
        });
    if let Some(other) = taken.get(name) {
        if !overrides_project {
            st.err(
                format!("{path}.name"),
                format!("{name} is already defined at {other}"),
            );
            return;
        }
    }
    local.insert(name.to_string(), path.to_string());
    let (default_expr, visible_if, enabled_if, expr_, options_from, product_filter) = match &kind {
        NodeKind::Field(f) => (
            st.parse(format!("{path}.default_expr"), &f.default_expr),
            st.parse(format!("{path}.visible_if"), &f.visible_if),
            st.parse(format!("{path}.enabled_if"), &f.enabled_if),
            None,
            None,
            st.parse(format!("{path}.product_filter"), &f.product_filter),
        ),
        NodeKind::Computed(c) => (
            None,
            None,
            None,
            st.parse(format!("{path}.expr"), &Some(c.expr.clone())),
            st.parse(format!("{path}.options_from"), &c.options_from),
            None,
        ),
    };
    let ty = match &kind {
        NodeKind::Field(f) => field_type(f, &st.tables),
        NodeKind::Computed(_) => Type::Any,
    };
    st.nodes.push(Node {
        scope,
        name: name.to_string(),
        path: path.to_string(),
        kind,
        default_expr,
        visible_if,
        enabled_if,
        expr: expr_,
        options_from,
        product_filter,
        ty,
        overrides_project,
    });
}

/// Kahn's algorithm, picking the earliest declared node first; the rest are in cycles.
fn order(nodes: &[Node], deps: &[BTreeSet<usize>]) -> (Vec<usize>, Vec<usize>) {
    let n = nodes.len();
    let mut indeg: Vec<usize> = deps.iter().map(BTreeSet::len).collect();
    let mut users: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, d) in deps.iter().enumerate() {
        for &j in d {
            users[j].push(i);
        }
    }
    let mut ready: BTreeSet<usize> = (0..n).filter(|i| indeg[*i] == 0).collect();
    let mut out = Vec::with_capacity(n);
    while let Some(i) = ready.pop_first() {
        out.push(i);
        for &u in &users[i] {
            indeg[u] -= 1;
            if indeg[u] == 0 {
                ready.insert(u);
            }
        }
    }
    let rest: Vec<usize> = (0..n).filter(|i| !out.contains(i)).collect();
    (out, rest)
}

/// One cycle among `rest`, as node indexes (first repeated at the end).
fn find_cycle(start: usize, deps: &[BTreeSet<usize>], rest: &BTreeSet<usize>) -> Vec<usize> {
    let mut path = vec![start];
    let mut cur = start;
    loop {
        let Some(&next) = deps[cur].iter().find(|d| rest.contains(d)) else {
            return path;
        };
        if let Some(pos) = path.iter().position(|p| *p == next) {
            let mut cycle = path[pos..].to_vec();
            cycle.push(next);
            return cycle;
        }
        path.push(next);
        cur = next;
    }
}

fn scope_label(def: &Definition, s: ScopeKey) -> String {
    match s {
        ScopeKey::Project => "project".into(),
        ScopeKey::Group => "group".into(),
        ScopeKey::Item(t) => def.item_tables[t].name.clone(),
    }
}

/// Resolves the scope of a rule or validation.
fn rule_scope(
    st: &mut State,
    scope: Option<RuleScope>,
    table: &Option<String>,
    path: &str,
) -> ScopeKey {
    match scope.unwrap_or(RuleScope::Project) {
        RuleScope::Project | RuleScope::Group => {
            if table.is_some() {
                st.err(format!("{path}.table"), "table is only for item scope");
            }
            if scope == Some(RuleScope::Group) {
                ScopeKey::Group
            } else {
                ScopeKey::Project
            }
        }
        RuleScope::Item => match table {
            Some(t) => match st.def.item_tables.iter().position(|x| &x.name == t) {
                Some(i) => ScopeKey::Item(i),
                None => {
                    st.err(format!("{path}.table"), format!("unknown item table {t}"));
                    ScopeKey::Project
                }
            },
            None if st.def.item_tables.len() == 1 => ScopeKey::Item(0),
            None => {
                st.err(format!("{path}.table"), "item scope needs the item table");
                ScopeKey::Project
            }
        },
    }
}

fn env<'a>(st: &'a State, scope: ScopeKey, with_label: bool) -> DefEnv<'a> {
    DefEnv {
        nodes: &st.nodes,
        def: st.def,
        tables: &st.tables,
        scope,
        with_label,
    }
}

fn check_expr(
    st: &mut State,
    path: &str,
    e: &Expr,
    scope: ScopeKey,
    locals: &[(String, Type)],
    want: &str,
) -> Type {
    let (t, errs) = expr::check_with(e, locals, &env(st, scope, true));
    let failed = !errs.is_empty();
    for x in errs {
        st.expr_err(path, x);
    }
    if !failed {
        expect(st, path, e, &t, want);
    }
    t
}

fn check_template(
    st: &mut State,
    path: &str,
    t: &Template,
    scope: ScopeKey,
    locals: &[(String, Type)],
) {
    let errs = t.check(locals, &env(st, scope, true));
    for x in errs {
        st.expr_err(path, x);
    }
}

#[allow(clippy::too_many_arguments)]
fn compile_rule(
    st: &mut State,
    r: &OutputRule,
    path: String,
    inherited: Option<ScopeKey>,
    depth: usize,
    ids: &mut HashMap<String, String>,
    parent_locals: &[(String, Type)],
) -> Option<CRule> {
    if r.id.trim().is_empty() {
        st.err(format!("{path}.id"), "a rule needs an id");
    } else if let Some(other) = ids.get(&r.id) {
        st.err(
            format!("{path}.id"),
            format!("id {} is already used at {other}", r.id),
        );
    } else {
        ids.insert(r.id.clone(), path.clone());
    }
    if depth > MAX_RULE_DEPTH {
        st.err(
            path.clone(),
            format!("rules are nested deeper than {MAX_RULE_DEPTH} levels"),
        );
        return None;
    }
    let scope = match inherited {
        None => rule_scope(st, r.scope, &r.table, &path),
        Some(s) => {
            if r.scope.is_some() {
                st.err(
                    format!("{path}.scope"),
                    "a child rule takes its parent's scope",
                );
            }
            if r.table.is_some() {
                st.err(
                    format!("{path}.table"),
                    "a child rule takes its parent's table",
                );
            }
            if r.merge_identical {
                st.err(
                    format!("{path}.merge_identical"),
                    "only top-level rules can merge identical lines",
                );
            }
            s
        }
    };
    if inherited.is_none() && r.percent_of_parent.is_some() {
        st.err(
            format!("{path}.percent_of_parent"),
            "only child rules can be priced as a percentage of their parent",
        );
    }
    if r.percent_base.is_some() && r.percent_of_parent.is_none() {
        st.err(
            format!("{path}.percent_base"),
            "percent_base needs percent_of_parent",
        );
    }
    let mut locals: Vec<(String, Type)> = parent_locals.to_vec();
    let when = st.parse(format!("{path}.when"), &r.when);
    if let Some(e) = &when {
        check_expr(st, &format!("{path}.when"), e, scope, &locals, "bool");
    }
    let for_each = st.parse(format!("{path}.for_each"), &r.for_each);
    if let Some(e) = &for_each {
        let t = check_expr(st, &format!("{path}.for_each"), e, scope, &locals, "list");
        let el = match t {
            Type::List(el) => *el,
            _ => Type::Any,
        };
        locals.push(("each".into(), el));
        locals.push(("each_index".into(), Type::Int));
    }
    let code = match (&r.code, &r.code_template) {
        (Some(_), Some(_)) | (None, None) => {
            st.err(format!("{path}.code"), "give either code or code_template");
            None
        }
        (Some(_), None) => st.parse(format!("{path}.code"), &r.code).map(|e| {
            check_expr(st, &format!("{path}.code"), &e, scope, &locals, "text");
            CodeSpec::Expr(e)
        }),
        (None, Some(_)) => st
            .parse_template(format!("{path}.code_template"), &r.code_template)
            .map(|t| {
                check_template(st, &format!("{path}.code_template"), &t, scope, &locals);
                CodeSpec::Template(t)
            }),
    };
    let quantity = st.parse(format!("{path}.quantity"), &r.quantity);
    if let Some(e) = &quantity {
        check_expr(st, &format!("{path}.quantity"), e, scope, &locals, "number");
    }
    let percent = st.parse(format!("{path}.percent_of_parent"), &r.percent_of_parent);
    if let Some(e) = &percent {
        check_expr(
            st,
            &format!("{path}.percent_of_parent"),
            e,
            scope,
            &locals,
            "number",
        );
    }
    let mut templates = Vec::new();
    for (attr, src) in [
        ("description", &r.description),
        ("comment", &r.comment),
        ("section", &r.section),
    ] {
        let t = st.parse_template(format!("{path}.{attr}"), src);
        if let Some(t) = &t {
            check_template(st, &format!("{path}.{attr}"), t, scope, &locals);
        }
        templates.push(t);
    }
    let mut child_locals = locals.clone();
    let mut parent = BTreeMap::new();
    parent.insert("code".to_string(), Type::Str);
    parent.insert("quantity".to_string(), Type::Dec);
    child_locals.retain(|(n, _)| n != "parent");
    child_locals.push(("parent".into(), Type::Record(Some(parent))));
    let mut children = Vec::new();
    for (i, c) in r.children.iter().enumerate() {
        if let Some(c) = compile_rule(
            st,
            c,
            format!("{path}.children[{i}]"),
            Some(scope),
            depth + 1,
            ids,
            &child_locals,
        ) {
            children.push(c);
        }
    }
    let section = templates.pop().flatten();
    let comment = templates.pop().flatten();
    let description = templates.pop().flatten();
    Some(CRule {
        id: r.id.clone(),
        path,
        scope,
        when,
        for_each,
        code: code?,
        quantity,
        description,
        comment,
        section,
        merge_identical: r.merge_identical,
        percent_of_parent: percent,
        percent_base: r.percent_base,
        children,
    })
}

/// Checks and prepares a definition. Returns every problem found.
pub fn compile(def: &Definition) -> Result<Compiled, Vec<DefinitionError>> {
    let mut st = State {
        def,
        tables: BTreeMap::new(),
        nodes: Vec::new(),
        errors: Vec::new(),
    };
    if def.schema_version != SCHEMA_VERSION {
        st.err(
            "$.schema_version",
            format!(
                "unsupported schema version {} (expected {SCHEMA_VERSION})",
                def.schema_version
            ),
        );
    }
    if def.name.trim().is_empty() {
        st.err("$.name", "the definition needs a name");
    }
    let mut section_names = BTreeSet::new();
    for (i, s) in def.sections.iter().enumerate() {
        if !section_names.insert(s.name.clone()) {
            st.err(
                format!("$.sections[{i}].name"),
                format!("section {} is listed twice", s.name),
            );
        }
    }

    // Tables first: fields and formulas refer to them.
    for (i, t) in def.tables.iter().enumerate() {
        let path = format!("$.tables[{i}]");
        if !valid_name(&t.name) {
            st.err(
                format!("{path}.name"),
                format!("'{}' is not a valid name", t.name),
            );
        }
        if st.tables.contains_key(&t.name) {
            st.err(
                format!("{path}.name"),
                format!("table {} is defined twice", t.name),
            );
            continue;
        }
        match CompiledTable::compile(t, &path) {
            Ok(c) => {
                st.tables.insert(t.name.clone(), c);
            }
            Err(errs) => st.errors.extend(errs),
        }
    }

    let mut taken: HashMap<String, String> = HashMap::new();
    add_scope_nodes(
        &mut st,
        ScopeKey::Project,
        &def.project.fields,
        &def.project.computed,
        "$.project",
        &mut taken,
    );
    for n in &st.nodes {
        taken.insert(n.name.clone(), n.path.clone());
    }
    let mut table_names: HashMap<String, String> = HashMap::new();
    for (i, t) in def.item_tables.iter().enumerate() {
        let path = format!("$.item_tables[{i}].name");
        if !valid_name(&t.name) || RESERVED.contains(&t.name.as_str()) {
            st.err(
                path.clone(),
                format!("'{}' is not a valid item table name", t.name),
            );
        }
        if let Some(other) = taken.get(&t.name).or_else(|| table_names.get(&t.name)) {
            st.err(
                path.clone(),
                format!("{} is already defined at {other}", t.name),
            );
        }
        table_names.insert(t.name.clone(), path);
    }
    taken.extend(table_names.clone());
    add_scope_nodes(
        &mut st,
        ScopeKey::Group,
        &def.group.fields,
        &def.group.computed,
        "$.group",
        &mut taken,
    );
    for n in st.nodes.iter().filter(|n| n.scope == ScopeKey::Group) {
        taken.insert(n.name.clone(), n.path.clone());
    }
    for (i, t) in def.item_tables.iter().enumerate() {
        let base = format!("$.item_tables[{i}]");
        let mut taken_t = taken.clone();
        add_scope_nodes(
            &mut st,
            ScopeKey::Item(i),
            &t.fields,
            &t.computed,
            &base,
            &mut taken_t,
        );
        if let (Some(a), Some(b)) = (t.min_rows, t.max_rows) {
            if a > b {
                st.err(
                    format!("{base}.min_rows"),
                    "min_rows is larger than max_rows",
                );
            }
        }
        if let Some(g) = &t.group_field {
            match t.fields.iter().position(|f| &f.name == g) {
                None => st.err(
                    format!("{base}.group_field"),
                    format!("{g} is not a field of this table"),
                ),
                Some(fi) => {
                    let f = &t.fields[fi];
                    if !matches!(f.field_type, FieldType::Text | FieldType::Choice) {
                        st.err(
                            format!("{base}.group_field"),
                            "the group field must be a text or choice field",
                        );
                    }
                    if f.default_expr.is_some() || f.visible_if.is_some() || f.enabled_if.is_some()
                    {
                        st.err(
                            format!("{base}.fields[{fi}]"),
                            "the group field cannot have default_expr, visible_if or enabled_if (groups are known before anything is calculated)",
                        );
                    }
                }
            }
        }
    }

    // Dependencies between nodes.
    let mut deps: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); st.nodes.len()];
    for (i, n) in st.nodes.iter().enumerate() {
        for e in n.exprs() {
            // In a product filter `p` is the product being filtered, not a value of the scope.
            let filter = n
                .product_filter
                .as_ref()
                .is_some_and(|f| std::ptr::eq(f, e));
            for name in e.free_names() {
                if filter && name == "p" {
                    continue;
                }
                for d in resolve_deps(&st.nodes, def, n.scope, &name) {
                    deps[i].insert(d);
                }
            }
        }
        if n.overrides_project {
            if let Some(p) = st
                .nodes
                .iter()
                .position(|x| x.scope == ScopeKey::Project && x.name == n.name)
            {
                deps[i].insert(p);
            }
        }
    }
    let (mut order_, rest) = order(&st.nodes, &deps);
    if !rest.is_empty() {
        let rest_set: BTreeSet<usize> = rest.iter().copied().collect();
        let mut reported = BTreeSet::new();
        for &r in &rest {
            if reported.contains(&r) {
                continue;
            }
            let cycle = find_cycle(r, &deps, &rest_set);
            if cycle.len() < 2 || cycle.first() != cycle.last() {
                continue;
            }
            let names: Vec<String> = cycle
                .iter()
                .map(|i| {
                    let n = &st.nodes[*i];
                    match n.scope {
                        ScopeKey::Project => n.name.clone(),
                        s => format!("{}.{}", scope_label(def, s), n.name),
                    }
                })
                .collect();
            let first = cycle[0];
            if cycle[..cycle.len() - 1]
                .iter()
                .any(|c| reported.contains(c))
            {
                continue;
            }
            reported.extend(cycle.iter().copied());
            let path = st.nodes[first].path.clone();
            st.err(path, format!("circular dependency: {}", names.join(" → ")));
        }
        // Keep going with a declaration order, to report type errors too.
        order_.extend(rest);
    }

    // Type check in dependency order, inferring computed types.
    for &i in &order_ {
        let n = st.nodes[i].clone();
        let field = match &n.kind {
            NodeKind::Field(f) => Some(f.clone()),
            NodeKind::Computed(_) => None,
        };
        let locals_p = [("p".to_string(), expr::types::product_type())];
        if let Some(e) = &n.default_expr {
            let t = check_expr(
                &mut st,
                &format!("{}.default_expr", n.path),
                e,
                n.scope,
                &[],
                "any",
            );
            if let Some(f) = &field {
                let want = field_type(f, &st.tables);
                let compatible = matches!(t, Type::Any | Type::Null)
                    || t == want
                    || (want == Type::Dec && t == Type::Int)
                    || matches!(want, Type::Any);
                if !compatible {
                    st.expr_err(
                        &format!("{}.default_expr", n.path),
                        ExprError::new(format!("the default must be {want}, found {t}"), e.span),
                    );
                }
            }
        }
        for (attr, e) in [("visible_if", &n.visible_if), ("enabled_if", &n.enabled_if)] {
            if let Some(e) = e {
                check_expr(
                    &mut st,
                    &format!("{}.{attr}", n.path),
                    e,
                    n.scope,
                    &[],
                    "bool",
                );
            }
        }
        if let Some(e) = &n.product_filter {
            check_expr(
                &mut st,
                &format!("{}.product_filter", n.path),
                e,
                n.scope,
                &locals_p,
                "bool",
            );
        }
        if let Some(e) = &n.expr {
            let t = check_expr(&mut st, &format!("{}.expr", n.path), e, n.scope, &[], "any");
            st.nodes[i].ty = t;
        }
        if let Some(e) = &n.options_from {
            check_expr(
                &mut st,
                &format!("{}.options_from", n.path),
                e,
                n.scope,
                &[],
                "list",
            );
        }
    }

    // Item labels.
    let mut item_labels = Vec::new();
    for (i, t) in def.item_tables.iter().enumerate() {
        let path = format!("$.item_tables[{i}].item_label");
        let tpl = st.parse_template(path.clone(), &t.item_label);
        if let Some(tpl) = &tpl {
            let errs = tpl.check(&[], &env(&st, ScopeKey::Item(i), false));
            for e in errs {
                st.expr_err(&path, e);
            }
        }
        item_labels.push(tpl);
    }

    // Output rules.
    let mut ids = HashMap::new();
    let mut rules = Vec::new();
    for (i, r) in def.outputs.iter().enumerate() {
        if let Some(r) = compile_rule(
            &mut st,
            r,
            format!("$.outputs[{i}]"),
            None,
            1,
            &mut ids,
            &[],
        ) {
            rules.push(r);
        }
    }

    // Validations.
    let mut vids: HashMap<String, String> = HashMap::new();
    let mut validations = Vec::new();
    for (i, v) in def.validations.iter().enumerate() {
        let path = format!("$.validations[{i}]");
        if v.id.trim().is_empty() {
            st.err(format!("{path}.id"), "a validation needs an id");
        } else if let Some(other) = vids.get(&v.id) {
            st.err(
                format!("{path}.id"),
                format!("id {} is already used at {other}", v.id),
            );
        } else {
            vids.insert(v.id.clone(), path.clone());
        }
        let scope = rule_scope(&mut st, v.scope, &v.table, &path);
        if let Some(f) = &v.field {
            let known = env(&st, scope, false).name(f).is_some();
            if !known {
                st.err(format!("{path}.field"), format!("unknown field {f}"));
            }
        }
        let when = st.parse(format!("{path}.when"), &Some(v.when.clone()));
        if let Some(e) = &when {
            check_expr(&mut st, &format!("{path}.when"), e, scope, &[], "bool");
        }
        let message = st.parse_template(format!("{path}.message"), &Some(v.message.clone()));
        if let Some(t) = &message {
            check_template(&mut st, &format!("{path}.message"), t, scope, &[]);
        }
        if let (Some(when), Some(message)) = (when, message) {
            validations.push(CValidation {
                id: v.id.clone(),
                path,
                scope,
                when,
                severity: v.severity,
                message,
                field: v.field.clone(),
            });
        }
    }

    if !st.errors.is_empty() {
        return Err(st.errors);
    }
    Ok(Compiled {
        def: def.clone(),
        tables: st.tables,
        nodes: st.nodes,
        order: order_,
        rules,
        validations,
        item_labels,
    })
}
