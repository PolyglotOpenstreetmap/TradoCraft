//! The fixed function catalogue: arity and a one-line description (also served to the editor).

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
pub struct Signature {
    pub name: &'static str,
    /// How it is written, e.g. `round(x, digits?)`.
    pub usage: &'static str,
    pub group: &'static str,
    pub description: &'static str,
    #[serde(skip)]
    pub min: usize,
    #[serde(skip)]
    pub max: usize,
}

const fn f(
    name: &'static str,
    usage: &'static str,
    group: &'static str,
    min: usize,
    max: usize,
    description: &'static str,
) -> Signature {
    Signature {
        name,
        usage,
        group,
        description,
        min,
        max,
    }
}

const MANY: usize = 64;

pub const FUNCTIONS: &[Signature] = &[
    f("ceil", "ceil(x, digits?)", "math", 1, 2, "Rounds up (towards +infinity)."),
    f("floor", "floor(x, digits?)", "math", 1, 2, "Rounds down (towards -infinity)."),
    f("round", "round(x, digits?)", "math", 1, 2, "Rounds half away from zero."),
    f("abs", "abs(x)", "math", 1, 1, "Absolute value."),
    f("sqrt", "sqrt(x)", "math", 1, 1, "Square root (12 decimals)."),
    f("sin", "sin(x)", "math", 1, 1, "Sine of an angle in radians (12 decimals)."),
    f("cos", "cos(x)", "math", 1, 1, "Cosine of an angle in radians (12 decimals)."),
    f("pow", "pow(x, n)", "math", 2, 2, "x to the whole power n (0 to 64)."),
    f("pi", "pi()", "math", 0, 0, "The number pi (also written as the name pi)."),
    f("min", "min(a, b, ...) | min(list, x => ...)", "math", 1, MANY, "Smallest value; nulls are skipped."),
    f("max", "max(a, b, ...) | max(list, x => ...)", "math", 1, MANY, "Largest value; nulls are skipped."),
    f("concat", "concat(a, b, ...)", "text", 1, MANY, "Joins values as text."),
    f("str", "str(x)", "text", 1, 1, "A value as text (decimals without trailing zeros)."),
    f("number", "number(text)", "text", 1, 1, "Reads a number from text; null if it is not one."),
    f("pad", "pad(x, width, fill?)", "text", 2, 3, "Pads on the left to a width, with '0' by default."),
    f("upper", "upper(text)", "text", 1, 1, "Upper case."),
    f("lower", "lower(text)", "text", 1, 1, "Lower case."),
    f("len", "len(text | list)", "text", 1, 1, "Number of characters or elements."),
    f("contains", "contains(text | list, x)", "text", 2, 2, "Whether the text or list contains x."),
    f("matches", "matches(text, pattern)", "text", 2, 2, "Whether the regular expression matches."),
    f("capture", "capture(text, pattern, group?)", "text", 2, 3, "The text of a group (1 by default) of the first match, or null."),
    f("sum", "sum(list, x => ...)", "aggregate", 1, 2, "Sum of the values; nulls are skipped."),
    f("avg", "avg(list, x => ...)", "aggregate", 1, 2, "Average of the values, or null for none."),
    f("count", "count(list, x => condition?)", "aggregate", 1, 2, "Number of elements (that meet the condition)."),
    f("any", "any(list, x => condition)", "aggregate", 1, 2, "Whether any element meets the condition."),
    f("all", "all(list, x => condition)", "aggregate", 1, 2, "Whether every element meets the condition."),
    f("distinct", "distinct(list, x => ...)", "aggregate", 1, 2, "The different values, in order of first appearance."),
    f("filter", "filter(list, x => condition)", "list", 2, 2, "The elements that meet the condition."),
    f("map", "map(list, x => ...)", "list", 2, 2, "Each element transformed."),
    f("first", "first(list, x => condition?)", "list", 1, 2, "The first element (that meets the condition), or null."),
    f("last", "last(list)", "list", 1, 1, "The last element, or null."),
    f("sort", "sort(list, x => key?)", "list", 1, 2, "Sorted ascending (by key)."),
    f("range", "range(from, to)", "list", 2, 2, "Whole numbers from..to, both included."),
    f("chunks", "chunks(n, size)", "packing", 2, 2, "Splits n into groups of size: chunks(9, 4) = [4, 4, 1]."),
    f("bins", "bins(values, capacity)", "packing", 2, 2, "Packs values first-fit into bins of a capacity; a list of bins (lists)."),
    f("lookup", "lookup('table', {key: value, ...}, offset?)", "table", 2, 3, "The matching row of a table as a record (offset moves to a later or earlier row)."),
    f("rows", "rows('table')", "table", 1, 1, "All rows of a table."),
    f("product", "product(code)", "product", 1, 1, "The product with this code (code, name, family, active, unit, attributes), or null. Use .attr('key') for an attribute."),
    f("exists", "exists(code)", "product", 1, 1, "Whether an active product with this code exists."),
];

pub fn signature(name: &str) -> Option<&'static Signature> {
    FUNCTIONS.iter().find(|s| s.name == name)
}

/// Positions of arguments that must be a function `x => ...` (when present).
pub fn lambda_positions(name: &str) -> &'static [usize] {
    match name {
        "sum" | "avg" | "count" | "any" | "all" | "distinct" | "filter" | "map" | "first"
        | "sort" => &[1],
        _ => &[],
    }
}
