//! The report definition a user builds, and [`plan`], which checks it against the catalog and
//! turns it into something the query builder can run without further validation.
//!
//! Output column keys (used by `sort` and `chart`):
//! - a plain column or a grouping: `quotation.total`, `quotation.quote_date:month` (with bucket)
//! - a summary: `quotation.total:sum`

use std::str::FromStr;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tradocraft_core::Permission;
use utoipa::ToSchema;

use crate::catalog::{self, Object, ReportFieldType};
use crate::relative::RelativeDate;

/// Most columns in a report.
pub const MAX_COLUMNS: usize = 30;
/// Most filters in a report.
pub const MAX_FILTERS: usize = 20;
/// Most groupings in a report.
pub const MAX_GROUPS: usize = 3;
/// Most sort keys in a report.
pub const MAX_SORTS: usize = 5;
/// Most joined objects in a report.
pub const MAX_JOINS: usize = 2;
/// Most values in an `in` filter.
pub const MAX_IN_VALUES: usize = 100;
/// Longest free text (labels) kept in a definition.
pub const MAX_TEXT: usize = 200;

/// How a column is summarised in a grouped or single-row report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportSummary {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}

impl ReportSummary {
    fn code(self) -> &'static str {
        match self {
            ReportSummary::Count => "count",
            ReportSummary::Sum => "sum",
            ReportSummary::Avg => "avg",
            ReportSummary::Min => "min",
            ReportSummary::Max => "max",
        }
    }
    fn title(self) -> &'static str {
        match self {
            ReportSummary::Count => "Count of",
            ReportSummary::Sum => "Sum of",
            ReportSummary::Avg => "Average of",
            ReportSummary::Min => "Minimum of",
            ReportSummary::Max => "Maximum of",
        }
    }
}

/// Comparison operator of a filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportOp {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Contains,
    StartsWith,
    In,
    IsNull,
    NotNull,
}

/// How the filters of a report combine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum ReportLogic {
    #[default]
    And,
    Or,
}

/// Date bucket for grouping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportBucket {
    Day,
    Month,
    Year,
}

/// Sort direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum ReportDir {
    #[default]
    Asc,
    Desc,
}

/// Kind of chart drawn for a report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportChartKind {
    Bar,
    Line,
    Pie,
}

/// A column of a report: a field, optionally summarised, with an optional label.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReportColumn {
    /// `object.field`, e.g. `quotation.total`.
    pub field: String,
    /// Turns the column into a summary; needs a grouping or makes the report a single row.
    #[serde(default)]
    pub summary: Option<ReportSummary>,
    /// Heading; default is derived from the field.
    #[serde(default)]
    pub label: Option<String>,
}

/// A filter condition on one field.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReportFilter {
    pub field: String,
    pub op: ReportOp,
    /// A string, number or boolean; an array for `in`; ignored for `is_null` and `not_null`.
    /// Dates are `YYYY-MM-DD`, also for date-time fields (compared as UTC days).
    ///
    /// On a date or date-time field the value may instead be a relative date, an object that is
    /// resolved against today each time the report runs (so a saved report stays live):
    /// `{"relative": "<token>"}` with the token `today`, `yesterday`, `this_week`, `last_week`
    /// (weeks start on Monday), `this_month`, `last_month`, `this_quarter`, `last_quarter`,
    /// `this_year` or `last_year`; or `{"relative": "last_n_days", "n": 30}` (the 30 days up to and
    /// including today) and `{"relative": "next_n_days", "n": 7}` (today and the 6 days after),
    /// with `n` from 1 to 3660. Each token is a range of days. `eq` matches the whole range, `ne`
    /// everything outside it; `gte` and `lt` compare with the first day, `gt` and `lte` with the
    /// last day (`gte last_month` is "from the start of last month", `gt last_month` is "after
    /// last month", `lte` and `lt` the other way round). For a single day (`today`,
    /// `yesterday`) all operators therefore read naturally. Not allowed with `in`, `is_null` and
    /// `not_null`, or on other field types.
    #[serde(default)]
    pub value: Option<serde_json::Value>,
}

/// A grouping, optionally bucketed by date.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReportGroupBy {
    pub field: String,
    /// For date fields: group by day, month or year.
    #[serde(default)]
    pub bucket: Option<ReportBucket>,
}

/// A sort key, naming an output column.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReportSort {
    /// Output column key, see the module documentation.
    pub column: String,
    #[serde(default)]
    pub dir: ReportDir,
}

/// The chart of a report: kind, x column and y column.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReportChart {
    pub kind: ReportChartKind,
    /// Key of a grouping column.
    pub x: String,
    /// Key of a numeric summary column.
    pub y: String,
}

/// A stored report definition, as sent by clients.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReportDefinition {
    /// The object the report is about: `customer`, `quotation` or `order`.
    pub object: String,
    /// Further objects to bring in, each related to one already in the report.
    #[serde(default)]
    pub joins: Vec<String>,
    pub columns: Vec<ReportColumn>,
    #[serde(default)]
    pub filters: Vec<ReportFilter>,
    #[serde(default)]
    pub filter_logic: ReportLogic,
    #[serde(default)]
    pub group_by: Vec<ReportGroupBy>,
    #[serde(default)]
    pub sort: Vec<ReportSort>,
    #[serde(default)]
    pub chart: Option<ReportChart>,
}

/// Role of an output column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportColumnKind {
    Plain,
    Group,
    Summary,
}

/// An output column of a planned report.
#[derive(Debug, Clone)]
pub struct OutColumn {
    pub key: String,
    pub label: String,
    pub ty: ReportFieldType,
    pub kind: ReportColumnKind,
    /// SQL for the value; built only from catalog text.
    pub raw: String,
}

/// The shape of a planned report's result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// One row per record.
    Rows,
    /// Summaries over everything: a single row.
    Single,
    /// One row per group.
    Grouped,
}

/// A filter value, typed so it can be bound.
#[derive(Debug, Clone)]
pub enum Val {
    None,
    Text(String),
    Num(Decimal),
    Date(NaiveDate),
    /// An inclusive range of days (`eq` and `ne` on a relative date).
    Range(NaiveDate, NaiveDate),
    Bool(bool),
    Texts(Vec<String>),
    Nums(Vec<Decimal>),
    Dates(Vec<NaiveDate>),
}

/// A filter with its value already checked and ready to bind.
#[derive(Debug, Clone)]
pub struct BoundFilter {
    /// Left-hand SQL; date-times are cut to UTC days.
    pub lhs: String,
    pub op: ReportOp,
    pub value: Val,
}

/// A checked report definition, ready to be turned into SQL.
#[derive(Debug, Clone)]
pub struct Plan {
    pub base: &'static Object,
    /// Joined objects with the condition that links each.
    pub joins: Vec<(&'static Object, &'static str)>,
    pub shape: Shape,
    pub columns: Vec<OutColumn>,
    pub filters: Vec<BoundFilter>,
    pub logic: ReportLogic,
    pub sort: Vec<(usize, ReportDir)>,
    pub chart: Option<ReportChart>,
    /// The date relative dates were resolved against.
    pub as_of: NaiveDate,
}

impl Plan {
    /// What the runner must be allowed to read: one permission per object in the report, plus
    /// the right to run reports.
    pub fn permissions(&self) -> Vec<Permission> {
        let mut v = vec![Permission::ReportsRead, self.base.permission];
        for (o, _) in &self.joins {
            if !v.contains(&o.permission) {
                v.push(o.permission);
            }
        }
        v
    }
}

fn err<T>(msg: impl Into<String>) -> Result<T, String> {
    Err(msg.into())
}

fn date_expr(ty: ReportFieldType, sql: &str) -> String {
    if ty == ReportFieldType::Datetime {
        format!("({sql} AT TIME ZONE 'UTC')::date")
    } else {
        sql.to_string()
    }
}

fn op_allowed(ty: ReportFieldType, op: ReportOp) -> bool {
    use ReportOp::*;
    match ty {
        ReportFieldType::Text => {
            matches!(op, Eq | Ne | Contains | StartsWith | In | IsNull | NotNull)
        }
        ReportFieldType::Number => {
            matches!(op, Eq | Ne | Gt | Gte | Lt | Lte | In | IsNull | NotNull)
        }
        ReportFieldType::Date | ReportFieldType::Datetime => {
            matches!(op, Eq | Ne | Gt | Gte | Lt | Lte | In | IsNull | NotNull)
        }
        ReportFieldType::Bool => matches!(op, Eq | Ne | IsNull | NotNull),
    }
}

fn escape_like(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if matches!(c, '\\' | '%' | '_') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

fn scalar(ty: ReportFieldType, v: &serde_json::Value) -> Result<Val, String> {
    use serde_json::Value as J;
    match (ty, v) {
        (ReportFieldType::Text, J::String(s)) => {
            if s.chars().count() > MAX_TEXT {
                return err(format!("text values are limited to {MAX_TEXT} characters"));
            }
            Ok(Val::Text(s.clone()))
        }
        (ReportFieldType::Number, J::Number(n)) => Decimal::from_str(&n.to_string())
            .map(Val::Num)
            .or_else(|_| {
                n.as_f64()
                    .and_then(|f| Decimal::try_from(f).ok())
                    .map(Val::Num)
                    .ok_or_else(|| "number out of range".to_string())
            }),
        (ReportFieldType::Number, J::String(s)) => Decimal::from_str(s.trim())
            .map(Val::Num)
            .map_err(|_| format!("'{s}' is not a number")),
        (ReportFieldType::Date | ReportFieldType::Datetime, J::Object(_)) => err(
            "a relative date works with =, \u{2260}, <, \u{2264}, > and \u{2265} only, not with 'in'",
        ),
        (ReportFieldType::Date | ReportFieldType::Datetime, J::String(s)) => {
            NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d")
                .map(Val::Date)
                .map_err(|_| format!("'{s}' is not a date (use YYYY-MM-DD)"))
        }
        (ReportFieldType::Bool, J::Bool(b)) => Ok(Val::Bool(*b)),
        _ => err("the value does not fit the field's type"),
    }
}

fn parse_value(
    ty: ReportFieldType,
    op: ReportOp,
    v: &Option<serde_json::Value>,
) -> Result<Val, String> {
    match op {
        ReportOp::IsNull | ReportOp::NotNull => Ok(Val::None),
        ReportOp::In => {
            let Some(serde_json::Value::Array(items)) = v else {
                return err("'in' needs a list of values");
            };
            if items.is_empty() || items.len() > MAX_IN_VALUES {
                return err(format!("'in' needs 1 to {MAX_IN_VALUES} values"));
            }
            let mut texts = Vec::new();
            let mut nums = Vec::new();
            let mut dates = Vec::new();
            for i in items {
                match scalar(ty, i)? {
                    Val::Text(t) => texts.push(t),
                    Val::Num(n) => nums.push(n),
                    Val::Date(d) => dates.push(d),
                    _ => return err("'in' does not work on this field"),
                }
            }
            Ok(match ty {
                ReportFieldType::Text => Val::Texts(texts),
                ReportFieldType::Number => Val::Nums(nums),
                _ => Val::Dates(dates),
            })
        }
        ReportOp::Contains | ReportOp::StartsWith => {
            let Some(v) = v else {
                return err("a value is required");
            };
            match scalar(ty, v)? {
                Val::Text(t) => Ok(Val::Text(if op == ReportOp::Contains {
                    format!("%{}%", escape_like(&t))
                } else {
                    format!("{}%", escape_like(&t))
                })),
                _ => err("this operator works on text"),
            }
        }
        _ => {
            let Some(v) = v else {
                return err("a value is required");
            };
            scalar(ty, v)
        }
    }
}

/// A filter value that is a relative date, resolved against `today` into something to bind.
fn relative_value(
    ty: ReportFieldType,
    op: ReportOp,
    v: &serde_json::Value,
    today: NaiveDate,
) -> Result<Val, String> {
    if !matches!(ty, ReportFieldType::Date | ReportFieldType::Datetime) {
        return err("relative dates only work on date fields");
    }
    if matches!(op, ReportOp::IsNull | ReportOp::NotNull | ReportOp::In) {
        return err("a relative date works with =, \u{2260}, <, \u{2264}, > and \u{2265} only");
    }
    let (start, end) = RelativeDate::parse(v)?.range(today)?;
    Ok(match op {
        ReportOp::Gte | ReportOp::Lt => Val::Date(start),
        ReportOp::Gt | ReportOp::Lte => Val::Date(end),
        _ if start == end => Val::Date(start),
        _ => Val::Range(start, end),
    })
}

/// SQL for a summary over one field. With joins every record can appear on several result
/// rows (a customer joined to two quotations and three orders appears six times), so counts,
/// sums and averages work on each record once: `count(DISTINCT id)`, and for sums a
/// `jsonb_object_agg` keyed by record id, which keeps one value per record. Min and max do not
/// care about repeats.
fn summary_sql(s: ReportSummary, o: &Object, f: &catalog::Field, dedup: bool) -> String {
    if !dedup {
        return match s {
            ReportSummary::Count => format!("count({})", f.sql),
            ReportSummary::Sum => format!("sum({})", f.sql),
            ReportSummary::Avg => format!("round(avg({}), 2)", f.sql),
            ReportSummary::Min => format!("min({})", f.sql),
            ReportSummary::Max => format!("max({})", f.sql),
        };
    }
    let id = format!("{}.id", o.alias);
    let per_record = |func: &str| {
        format!(
            "(SELECT {func}(e.v::numeric) FROM jsonb_each_text(jsonb_object_agg({id}::text, {sql}) \
             FILTER (WHERE {id} IS NOT NULL)) AS e(k, v))",
            sql = f.sql
        )
    };
    match s {
        ReportSummary::Count => {
            format!("count(DISTINCT {id}) FILTER (WHERE {} IS NOT NULL)", f.sql)
        }
        ReportSummary::Sum => per_record("sum"),
        ReportSummary::Avg => format!("round({}, 2)", per_record("avg")),
        ReportSummary::Min => format!("min({})", f.sql),
        ReportSummary::Max => format!("max({})", f.sql),
    }
}

fn bucket_expr(ty: ReportFieldType, sql: &str, b: ReportBucket) -> String {
    let d = date_expr(ty, sql);
    match b {
        ReportBucket::Day => d,
        ReportBucket::Month => format!("date_trunc('month', {d})::date"),
        ReportBucket::Year => format!("date_trunc('year', {d})::date"),
    }
}

/// Checks a definition against the catalog. The error is the first problem found. `today`
/// resolves relative dates; the stored definition is not changed.
pub fn plan(def: &ReportDefinition, today: NaiveDate) -> Result<Plan, String> {
    let base =
        catalog::object(&def.object).ok_or_else(|| format!("unknown object '{}'", def.object))?;

    // Objects in the report.
    if def.joins.len() > MAX_JOINS {
        return err(format!("at most {MAX_JOINS} joined objects"));
    }
    let mut joins: Vec<(&'static Object, &'static str)> = Vec::new();
    for j in &def.joins {
        let obj = catalog::object(j).ok_or_else(|| format!("unknown object '{j}'"))?;
        if obj.name == base.name || joins.iter().any(|(o, _)| o.name == obj.name) {
            return err(format!("object '{j}' is already in the report"));
        }
        let on = std::iter::once(base.name)
            .chain(joins.iter().map(|(o, _)| o.name))
            .find_map(|present| catalog::link(present, obj.name))
            .ok_or_else(|| format!("'{j}' is not related to the objects in the report"))?;
        joins.push((obj, on));
    }
    let included = |o: &Object| o.name == base.name || joins.iter().any(|(j, _)| j.name == o.name);
    let lookup = |q: &str| -> Result<(&'static Object, &'static catalog::Field), String> {
        let (o, f) = catalog::resolve(q).ok_or_else(|| format!("unknown field '{q}'"))?;
        if !included(o) {
            return err(format!(
                "field '{q}' belongs to an object that is not in the report"
            ));
        }
        Ok((o, f))
    };
    let multi = !joins.is_empty();
    let field_label = |o: &Object, f: &catalog::Field| {
        if multi {
            format!("{}: {}", o.label, f.label)
        } else {
            f.label.to_string()
        }
    };

    if def.columns.is_empty() || def.columns.len() > MAX_COLUMNS {
        return err(format!("a report needs 1 to {MAX_COLUMNS} columns"));
    }
    if def.group_by.len() > MAX_GROUPS {
        return err(format!("at most {MAX_GROUPS} groupings"));
    }

    // Shape.
    let summaries = def.columns.iter().filter(|c| c.summary.is_some()).count();
    let shape = if !def.group_by.is_empty() {
        if summaries != def.columns.len() {
            return err("in a grouped report every column must be a summary");
        }
        Shape::Grouped
    } else if summaries > 0 {
        if summaries != def.columns.len() {
            return err("a report cannot mix plain columns and summaries without a grouping");
        }
        Shape::Single
    } else {
        Shape::Rows
    };

    // Output columns.
    let mut columns: Vec<OutColumn> = Vec::new();
    for g in &def.group_by {
        let (o, f) = lookup(&g.field)?;
        if f.count_only || f.ty == ReportFieldType::Number {
            return err(format!("cannot group by '{}'", g.field));
        }
        let (raw, ty, key, label) = match g.bucket {
            None => (f.sql.to_string(), f.ty, g.field.clone(), field_label(o, f)),
            Some(b) => {
                if !matches!(f.ty, ReportFieldType::Date | ReportFieldType::Datetime) {
                    return err(format!(
                        "'{}' is not a date, it cannot be bucketed",
                        g.field
                    ));
                }
                let code = match b {
                    ReportBucket::Day => "day",
                    ReportBucket::Month => "month",
                    ReportBucket::Year => "year",
                };
                (
                    bucket_expr(f.ty, f.sql, b),
                    ReportFieldType::Date,
                    format!("{}:{code}", g.field),
                    format!("{} ({code})", field_label(o, f)),
                )
            }
        };
        columns.push(OutColumn {
            key,
            label,
            ty,
            kind: ReportColumnKind::Group,
            raw,
        });
    }
    for c in &def.columns {
        let (o, f) = lookup(&c.field)?;
        let custom = c
            .label
            .as_deref()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().take(MAX_TEXT).collect::<String>());
        match c.summary {
            None => {
                if f.count_only {
                    return err(format!("'{}' can only be counted", c.field));
                }
                columns.push(OutColumn {
                    key: c.field.clone(),
                    label: custom.unwrap_or_else(|| field_label(o, f)),
                    ty: f.ty,
                    kind: if shape == Shape::Rows {
                        ReportColumnKind::Plain
                    } else {
                        ReportColumnKind::Group
                    },
                    raw: f.sql.to_string(),
                });
            }
            Some(s) => {
                let ok = match s {
                    ReportSummary::Count => true,
                    _ if f.count_only => false,
                    ReportSummary::Sum | ReportSummary::Avg => f.ty == ReportFieldType::Number,
                    ReportSummary::Min | ReportSummary::Max => matches!(
                        f.ty,
                        ReportFieldType::Number | ReportFieldType::Date | ReportFieldType::Datetime
                    ),
                };
                if !ok {
                    return err(format!(
                        "'{}' cannot be summarised with {}",
                        c.field,
                        s.code()
                    ));
                }
                let raw = summary_sql(s, o, f, multi);
                let ty = match s {
                    ReportSummary::Min | ReportSummary::Max => f.ty,
                    _ => ReportFieldType::Number,
                };
                columns.push(OutColumn {
                    key: format!("{}:{}", c.field, s.code()),
                    label: custom.unwrap_or_else(|| format!("{} {}", s.title(), field_label(o, f))),
                    ty,
                    kind: ReportColumnKind::Summary,
                    raw,
                });
            }
        }
    }
    for (i, c) in columns.iter().enumerate() {
        if columns[..i].iter().any(|p| p.key == c.key) {
            return err(format!("column '{}' appears twice", c.key));
        }
    }

    // Filters.
    if def.filters.len() > MAX_FILTERS {
        return err(format!("at most {MAX_FILTERS} filters"));
    }
    let mut filters = Vec::new();
    for f in &def.filters {
        let (_, fd) = lookup(&f.field)?;
        if fd.count_only {
            return err(format!("cannot filter on '{}'", f.field));
        }
        if !op_allowed(fd.ty, f.op) {
            return err(format!(
                "operator {:?} does not work on '{}'",
                f.op, f.field
            ));
        }
        let lhs = if fd.ty == ReportFieldType::Datetime {
            date_expr(fd.ty, fd.sql)
        } else {
            fd.sql.to_string()
        };
        let value = match &f.value {
            Some(v @ serde_json::Value::Object(_)) => relative_value(fd.ty, f.op, v, today),
            v => parse_value(fd.ty, f.op, v),
        }
        .map_err(|m| format!("filter on '{}': {m}", f.field))?;
        filters.push(BoundFilter {
            lhs,
            op: f.op,
            value,
        });
    }

    // Sort.
    if def.sort.len() > MAX_SORTS {
        return err(format!("at most {MAX_SORTS} sort columns"));
    }
    let mut sort: Vec<(usize, ReportDir)> = Vec::new();
    for s in &def.sort {
        let i = columns
            .iter()
            .position(|c| c.key == s.column)
            .ok_or_else(|| format!("cannot sort on '{}': it is not a column", s.column))?;
        if sort.iter().any(|(j, _)| *j == i) {
            return err(format!("sorted twice on '{}'", s.column));
        }
        sort.push((i, s.dir));
    }

    // Chart.
    if let Some(ch) = &def.chart {
        if shape != Shape::Grouped {
            return err("a chart needs a grouped report");
        }
        let x = columns.iter().find(|c| c.key == ch.x);
        let y = columns.iter().find(|c| c.key == ch.y);
        match (x, y) {
            (Some(x), Some(y))
                if x.kind == ReportColumnKind::Group
                    && y.kind == ReportColumnKind::Summary
                    && y.ty == ReportFieldType::Number => {}
            _ => {
                return err(
                    "a chart needs a grouping column for x and a numeric summary column for y",
                )
            }
        }
    }

    Ok(Plan {
        base,
        joins,
        shape,
        columns,
        filters,
        logic: def.filter_logic,
        sort,
        chart: def.chart.clone(),
        as_of: today,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn def(v: serde_json::Value) -> ReportDefinition {
        serde_json::from_value(v).unwrap()
    }

    fn today() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 11, 11).unwrap()
    }

    fn plan(d: &ReportDefinition) -> Result<Plan, String> {
        super::plan(d, today())
    }

    fn problem(v: serde_json::Value) -> String {
        plan(&def(v)).unwrap_err()
    }

    #[test]
    fn a_plain_report_plans() {
        let p = plan(&def(json!({
            "object": "quotation",
            "columns": [{"field": "quotation.number"}, {"field": "quotation.total"}],
            "filters": [{"field": "quotation.status", "op": "eq", "value": "sent"}],
            "sort": [{"column": "quotation.total", "dir": "desc"}]
        })))
        .unwrap();
        assert_eq!(p.shape, Shape::Rows);
        assert_eq!(p.columns.len(), 2);
        assert_eq!(p.sort[0].0, 1);
        assert_eq!(
            p.permissions(),
            vec![Permission::ReportsRead, Permission::QuotationsRead]
        );
    }

    #[test]
    fn grouped_reports_list_groups_first_and_need_summaries() {
        let p = plan(&def(json!({
            "object": "quotation",
            "columns": [{"field": "quotation.total", "summary": "sum"}],
            "group_by": [{"field": "quotation.quote_date", "bucket": "month"}],
            "chart": {"kind": "bar", "x": "quotation.quote_date:month", "y": "quotation.total:sum"}
        })))
        .unwrap();
        assert_eq!(p.shape, Shape::Grouped);
        assert_eq!(p.columns[0].key, "quotation.quote_date:month");
        assert_eq!(p.columns[0].ty, ReportFieldType::Date);
        assert_eq!(p.columns[1].key, "quotation.total:sum");

        assert!(problem(json!({
            "object": "quotation",
            "columns": [{"field": "quotation.total"}],
            "group_by": [{"field": "quotation.status"}]
        }))
        .contains("every column must be a summary"));
    }

    #[test]
    fn summaries_without_grouping_make_one_row_and_cannot_mix() {
        let p = plan(&def(json!({
            "object": "order",
            "columns": [{"field": "order.records", "summary": "count"},
                        {"field": "order.total", "summary": "avg"}]
        })))
        .unwrap();
        assert_eq!(p.shape, Shape::Single);
        assert!(problem(json!({
            "object": "order",
            "columns": [{"field": "order.number"}, {"field": "order.total", "summary": "sum"}]
        }))
        .contains("cannot mix"));
    }

    #[test]
    fn joins_must_be_related_and_fields_must_belong_to_the_report() {
        let p = plan(&def(json!({
            "object": "order",
            "joins": ["customer", "quotation"],
            "columns": [{"field": "order.number"}, {"field": "customer.name"}, {"field": "quotation.number"}]
        })))
        .unwrap();
        assert_eq!(p.joins.len(), 2);
        assert_eq!(
            p.permissions(),
            vec![
                Permission::ReportsRead,
                Permission::OrdersRead,
                Permission::CustomersRead,
                Permission::QuotationsRead
            ]
        );
        assert!(p.columns[1].label.starts_with("Customers: "));

        assert!(problem(json!({
            "object": "customer",
            "columns": [{"field": "quotation.number"}]
        }))
        .contains("not in the report"));
        assert!(problem(json!({
            "object": "customer", "joins": ["customer"],
            "columns": [{"field": "customer.name"}]
        }))
        .contains("already in the report"));
        assert!(problem(json!({
            "object": "customer", "joins": ["nope"],
            "columns": [{"field": "customer.name"}]
        }))
        .contains("unknown object"));
    }

    #[test]
    fn injection_attempts_are_rejected_by_the_catalog() {
        for bad in [
            "quotation.total; DROP TABLE sales.quotations",
            "quotation.total) --",
            "q.total",
            "sales.quotations.total",
            "quotation.total'",
        ] {
            let msg = problem(json!({"object": "quotation", "columns": [{"field": bad}]}));
            assert!(msg.contains("unknown field"), "{bad}: {msg}");
        }
        assert!(problem(json!({
            "object": "quotation; select 1",
            "columns": [{"field": "quotation.total"}]
        }))
        .contains("unknown object"));
        assert!(problem(json!({
            "object": "quotation",
            "columns": [{"field": "quotation.number"}],
            "sort": [{"column": "1; select pg_sleep(10)"}]
        }))
        .contains("not a column"));
    }

    #[test]
    fn filter_operators_and_values_follow_the_field_type() {
        let ok = |f: serde_json::Value| {
            plan(&def(json!({
                "object": "quotation",
                "columns": [{"field": "quotation.number"}],
                "filters": [f]
            })))
        };
        assert!(ok(json!({"field": "quotation.total", "op": "gte", "value": 100})).is_ok());
        assert!(ok(json!({"field": "quotation.total", "op": "gte", "value": "12.50"})).is_ok());
        assert!(ok(json!({"field": "quotation.total", "op": "gte", "value": "abc"})).is_err());
        assert!(ok(json!({"field": "quotation.total", "op": "contains", "value": "1"})).is_err());
        assert!(ok(json!({"field": "quotation.number", "op": "gt", "value": "a"})).is_err());
        assert!(
            ok(json!({"field": "quotation.number", "op": "contains", "value": "50%_"})).is_ok()
        );
        assert!(
            ok(json!({"field": "quotation.status", "op": "in", "value": ["draft", "sent"]}))
                .is_ok()
        );
        assert!(ok(json!({"field": "quotation.status", "op": "in", "value": []})).is_err());
        assert!(ok(json!({"field": "quotation.status", "op": "in", "value": "draft"})).is_err());
        assert!(
            ok(json!({"field": "quotation.quote_date", "op": "gte", "value": "2027-01-31"}))
                .is_ok()
        );
        assert!(
            ok(json!({"field": "quotation.quote_date", "op": "gte", "value": "31/01/2027"}))
                .is_err()
        );
        assert!(
            ok(json!({"field": "quotation.created_at", "op": "lt", "value": "2027-01-31"})).is_ok()
        );
        assert!(ok(json!({"field": "quotation.sent_at", "op": "is_null"})).is_ok());
        assert!(ok(json!({"field": "quotation.records", "op": "eq", "value": "x"})).is_err());
        assert!(ok(json!({"field": "quotation.status", "op": "eq"})).is_err());
    }

    fn rel(field: &str, op: &str, value: serde_json::Value) -> Result<Plan, String> {
        plan(&def(json!({
            "object": "quotation",
            "columns": [{"field": "quotation.number"}],
            "filters": [{"field": field, "op": op, "value": value}]
        })))
    }

    fn bound(p: &Plan) -> (ReportOp, Val) {
        (p.filters[0].op, p.filters[0].value.clone())
    }

    #[test]
    fn relative_dates_resolve_against_the_given_today() {
        let d = |m, day| NaiveDate::from_ymd_opt(2026, m, day).unwrap();
        let last_month = json!({"relative": "last_month"});
        // Oct 1 - Oct 31 with today = 2026-11-11.
        let eq = bound(&rel("quotation.quote_date", "eq", last_month.clone()).unwrap());
        assert!(matches!(eq, (ReportOp::Eq, Val::Range(a, b)) if a == d(10, 1) && b == d(10, 31)));
        let ne = bound(&rel("quotation.quote_date", "ne", last_month.clone()).unwrap());
        assert!(matches!(ne, (ReportOp::Ne, Val::Range(..))));
        for (op, want) in [
            ("gte", d(10, 1)),
            ("lt", d(10, 1)),
            ("gt", d(10, 31)),
            ("lte", d(10, 31)),
        ] {
            let (_, v) = bound(&rel("quotation.quote_date", op, last_month.clone()).unwrap());
            assert!(matches!(v, Val::Date(x) if x == want), "{op}");
        }
        // A single day is a plain date with every operator.
        let (op, v) =
            bound(&rel("quotation.quote_date", "eq", json!({"relative": "today"})).unwrap());
        assert!(matches!((op, v), (ReportOp::Eq, Val::Date(x)) if x == d(11, 11)));
        // Another today gives another range: the definition itself is not rewritten.
        let q = def(json!({
            "object": "quotation", "columns": [{"field": "quotation.number"}],
            "filters": [{"field": "quotation.quote_date", "op": "gte",
                         "value": {"relative": "last_n_days", "n": 30}}]
        }));
        let a = super::plan(&q, d(11, 11)).unwrap();
        let b = super::plan(&q, d(12, 11)).unwrap();
        assert!(matches!(a.filters[0].value, Val::Date(x) if x == d(10, 13)));
        assert!(matches!(b.filters[0].value, Val::Date(x) if x == d(11, 12)));
        // The plan remembers the day it was resolved against.
        assert_eq!((a.as_of, b.as_of), (d(11, 11), d(12, 11)));
    }

    #[test]
    fn relative_dates_work_on_date_time_fields_as_utc_days() {
        let p = rel("quotation.sent_at", "gte", json!({"relative": "this_week"})).unwrap();
        assert!(p.filters[0].lhs.contains("AT TIME ZONE 'UTC'"));
        assert!(
            matches!(p.filters[0].value, Val::Date(x) if x == NaiveDate::from_ymd_opt(2026, 11, 9).unwrap())
        );
    }

    #[test]
    fn relative_dates_are_validated() {
        let r = |field, op, v| rel(field, op, v).unwrap_err();
        assert!(
            r("quotation.total", "gte", json!({"relative": "today"})).contains("only work on date")
        );
        assert!(
            r("quotation.status", "eq", json!({"relative": "today"})).contains("only work on date")
        );
        assert!(r(
            "quotation.quote_date",
            "is_null",
            json!({"relative": "today"})
        )
        .contains("works with"));
        assert!(
            r("quotation.quote_date", "in", json!({"relative": "today"})).contains("works with")
        );
        assert!(
            r("quotation.quote_date", "in", json!([{"relative": "today"}]))
                .contains("not with 'in'")
        );
        assert!(r(
            "quotation.quote_date",
            "eq",
            json!({"relative": "tomorrow"})
        )
        .contains("not a relative date"));
        assert!(r(
            "quotation.quote_date",
            "eq",
            json!({"relative": "last_n_days", "n": 0})
        )
        .contains("between 1 and 3660"));
        assert!(r(
            "quotation.quote_date",
            "eq",
            json!({"relative": "next_n_days", "n": 3661})
        )
        .contains("between 1 and 3660"));
        assert!(r(
            "quotation.quote_date",
            "eq",
            json!({"relative": "x'; DROP TABLE a;--"})
        )
        .contains("not a relative date"));
    }

    #[test]
    fn contains_escapes_like_wildcards() {
        let v = parse_value(
            ReportFieldType::Text,
            ReportOp::Contains,
            &Some(json!("50%_\\")),
        )
        .unwrap();
        match v {
            Val::Text(t) => assert_eq!(t, "%50\\%\\_\\\\%"),
            _ => panic!(),
        }
    }

    #[test]
    fn summary_rules_follow_the_field_type() {
        let p = |field: &str, s: &str| {
            plan(&def(json!({
                "object": "quotation",
                "columns": [{"field": field, "summary": s}]
            })))
        };
        assert!(p("quotation.total", "sum").is_ok());
        assert!(p("quotation.number", "sum").is_err());
        assert!(p("quotation.number", "count").is_ok());
        assert!(p("quotation.quote_date", "max").is_ok());
        assert!(p("quotation.status", "min").is_err());
        assert!(p("quotation.records", "count").is_ok());
        assert!(p("quotation.records", "max").is_err());
    }

    #[test]
    fn limits_and_duplicates() {
        assert!(problem(json!({"object": "quotation", "columns": []})).contains("1 to"));
        assert!(problem(json!({
            "object": "quotation",
            "columns": [{"field": "quotation.number"}, {"field": "quotation.number"}]
        }))
        .contains("twice"));
        assert!(problem(json!({
            "object": "quotation",
            "columns": [{"field": "quotation.total", "summary": "sum"}],
            "group_by": [{"field": "quotation.total"}]
        }))
        .contains("cannot group"));
        assert!(problem(json!({
            "object": "quotation",
            "columns": [{"field": "quotation.total", "summary": "sum"}],
            "group_by": [{"field": "quotation.status", "bucket": "month"}]
        }))
        .contains("not a date"));
        assert!(problem(json!({
            "object": "quotation",
            "columns": [{"field": "quotation.number"}],
            "chart": {"kind": "bar", "x": "quotation.number", "y": "quotation.number"}
        }))
        .contains("grouped"));
    }
}
