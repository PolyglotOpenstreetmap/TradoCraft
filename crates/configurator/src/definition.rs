//! The configurator definition: one typed JSON document, exactly what is stored in a model
//! revision, exported and imported (ADR 0034).
//!
//! Scopes: `project` (one per configuration) → `group` (a section, e.g. a hall; one per distinct
//! value of an item table's `group_field`) → `item` (one row of a repeating item table).
//! [`validate`] reports every problem with a JSON path; [`crate::compile`] does the work.

use std::collections::BTreeMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The definition format this code reads and writes.
pub const SCHEMA_VERSION: u32 = 1;

fn schema_version() -> u32 {
    SCHEMA_VERSION
}

fn yes() -> bool {
    true
}

/// Label and help text in another language.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Text {
    #[serde(default)]
    pub label: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub help: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Definition {
    #[serde(default = "schema_version")]
    pub schema_version: u32,
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// Names per language (`label`, `help`).
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub translations: BTreeMap<String, Text>,
    /// Layout sections of the form, referenced by `layout.section` of fields.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<LayoutSection>,
    #[serde(default)]
    pub project: Scope,
    #[serde(default)]
    pub group: GroupScope,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item_tables: Vec<ItemTable>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<Table>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<OutputRule>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validations: Vec<ValidationRule>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutSection {
    pub name: String,
    #[serde(default)]
    pub label: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub translations: BTreeMap<String, Text>,
}

/// Project-level inputs and computed values.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scope {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<Field>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub computed: Vec<Computed>,
}

/// Per-group inputs and computed values. A group field with the name of a project field
/// overrides it within the group and defaults to the project value.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupScope {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub label: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub translations: BTreeMap<String, Text>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<Field>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub computed: Vec<Computed>,
}

/// A repeating table of items (rows), grouped by one of its fields.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemTable {
    pub name: String,
    #[serde(default)]
    pub label: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub translations: BTreeMap<String, Text>,
    /// The field (text or choice) whose value names the item's group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_field: Option<String>,
    /// Template naming one item in comments and messages; default `{label} {index}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_rows: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_rows: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<Field>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub computed: Vec<Computed>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    Number,
    Integer,
    Boolean,
    Choice,
    Text,
    /// A product code, picked from the catalogue (filtered by family and a condition).
    Product,
}

impl FieldType {
    pub fn as_str(self) -> &'static str {
        match self {
            FieldType::Number => "number",
            FieldType::Integer => "integer",
            FieldType::Boolean => "boolean",
            FieldType::Choice => "choice",
            FieldType::Text => "text",
            FieldType::Product => "product",
        }
    }
}

/// Physical dimension of a number field; values are stored in the field's `unit`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dimension {
    Mass,
    Length,
    Power,
    Voltage,
    Current,
    Speed,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChoiceOption {
    pub value: serde_json::Value,
    #[serde(default)]
    pub label: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub translations: BTreeMap<String, Text>,
}

/// Options of a choice taken from elsewhere.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsFrom {
    /// The distinct values of a column of a lookup table ...
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    /// ... or the codes of the active products of a family.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Layout {
    /// Name of a layout section of the definition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// Relative width in a grid (item tables) or form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}

/// An input. Attributes that do not belong to the field's type are refused by [`validate`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: FieldType,
    #[serde(default)]
    pub label: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub help: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub translations: BTreeMap<String, Text>,
    /// A literal default value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    /// A formula for the default (instead of `default`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_expr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_if: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled_if: Option<String>,
    #[serde(default)]
    pub required: bool,
    // number / integer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<Dimension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub display_units: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<Decimal>,
    // choice
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<ChoiceOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options_from: Option<OptionsFrom>,
    // text
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    // product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// Condition on `p` (the product record), e.g. `p.attr('kw') >= 1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_filter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<Layout>,
}

/// A named formula. When `overridable`, the user may pin another value.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Computed {
    pub name: String,
    #[serde(default)]
    pub label: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub help: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub translations: BTreeMap<String, Text>,
    pub expr: String,
    #[serde(default)]
    pub overridable: bool,
    /// A formula giving the allowed alternatives (a list) of an overridable value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options_from: Option<String>,
    /// Shown to the user (false for helper values).
    #[serde(default = "yes")]
    pub show: bool,
    /// Display unit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<Layout>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TableKind {
    /// Rows match on all key columns.
    #[default]
    Exact,
    /// Rows match on the key columns plus a value between a min and a max column.
    Range,
}

/// What a lookup does when no row fits (or the offset runs past the table).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnMissing {
    #[default]
    Error,
    /// The nearest row (range tables) or the first/last row (offsets).
    Clamp,
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnType {
    Number,
    Integer,
    Text,
    Boolean,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub column_type: ColumnType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RangeSpec {
    /// The name of the lookup key that carries the value, e.g. `kw`.
    pub key: String,
    /// Columns with the inclusive bounds (an empty cell is unbounded).
    pub min: String,
    pub max: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Table {
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub label: String,
    #[serde(default)]
    pub kind: TableKind,
    /// Columns matched exactly.
    #[serde(default)]
    pub keys: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<RangeSpec>,
    #[serde(default)]
    pub on_missing: OnMissing,
    pub columns: Vec<Column>,
    #[serde(default)]
    pub rows: Vec<BTreeMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RuleScope {
    Project,
    Group,
    Item,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PercentBase {
    ParentListUnitPrice,
    ParentNetUnitPrice,
}

/// Generates quotation lines.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutputRule {
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub label: String,
    /// `project` (default), `group` or `item` (top-level rules only; children inherit).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<RuleScope>,
    /// The item table of an item rule (may be left out when there is only one).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,
    /// A list; one line per element, with `each` and `each_index` (1-based).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub for_each: Option<String>,
    /// The product code as a formula ...
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// ... or as a template, e.g. `DRV-{motor.size}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_template: Option<String>,
    /// Formula; defaults to 1. Lines with a quantity of zero or less are left out.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    /// Template overriding the product name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Template printed below the line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Template; defaults to the group name for group and item rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    /// Collapse lines with the same code (and children) into one; the comment lists the items.
    #[serde(default)]
    pub merge_identical: bool,
    /// Formula (children only): price as this percentage of the parent line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent_of_parent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent_base: Option<PercentBase>,
    /// Option lines below this line; they see `parent` (`code`, `quantity`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<OutputRule>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(as = IssueSeverity)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// A check: when `when` is true, an issue with the message is reported.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationRule {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<RuleScope>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    pub when: String,
    pub severity: Severity,
    /// Template.
    pub message: String,
    /// The field the issue is shown at.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub translations: BTreeMap<String, String>,
}

/// A problem in a definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct DefinitionError {
    /// JSON path of the offending element, e.g. `$.item_tables[0].computed[1].expr`.
    pub path: String,
    pub message: String,
    /// Character range within a formula or template, when the problem is in one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<usize>,
}

impl DefinitionError {
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        DefinitionError {
            path: path.into(),
            message: message.into(),
            start: None,
            end: None,
        }
    }
}

/// Reads a definition from JSON; serde errors become one [`DefinitionError`] at `$`.
pub fn from_json(v: &serde_json::Value) -> Result<Definition, DefinitionError> {
    serde_json::from_value(v.clone())
        .map_err(|e| DefinitionError::new("$", format!("not a valid definition: {e}")))
}

/// Every problem of a definition, each with its JSON path. Empty means it can be activated.
pub fn validate(def: &Definition) -> Vec<DefinitionError> {
    match crate::compile::compile(def) {
        Ok(_) => Vec::new(),
        Err(errors) => errors,
    }
}

/// Units known per dimension, with their factor to the first (canonical) one.
pub fn units(dim: Dimension) -> &'static [(&'static str, &'static str)] {
    match dim {
        Dimension::Mass => &[("kg", "1"), ("g", "0.001"), ("t", "1000")],
        Dimension::Length => &[("m", "1"), ("mm", "0.001"), ("cm", "0.01"), ("km", "1000")],
        Dimension::Power => &[("kW", "1"), ("W", "0.001"), ("MW", "1000")],
        Dimension::Voltage => &[("V", "1"), ("mV", "0.001"), ("kV", "1000")],
        Dimension::Current => &[("A", "1"), ("mA", "0.001"), ("kA", "1000")],
        Dimension::Speed => &[
            ("m/s", "1"),
            ("m/min", "0.0166666666666666666667"),
            ("km/h", "0.2777777777777777777778"),
        ],
    }
}
