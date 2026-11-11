//! CSV building blocks shared by the exports and imports: RFC 4180 writing and reading, delimiter
//! detection, typed cell parsing (decimal comma) and the common import report.
//!
//! An import is always all-or-nothing: every row is checked first and *all* row errors are
//! reported together ([`ImportReport`]); nothing is written unless there are none.

use std::collections::{HashMap, HashSet};

use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::Json;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::error::{AppError, AppResult};

/// Largest accepted import body.
pub const MAX_BODY_BYTES: usize = 5 * 1024 * 1024;
/// Largest accepted number of data rows in an import.
pub const MAX_ROWS: usize = 20_000;
/// Largest accepted number of cells in one record (the widest known file has 19 columns).
pub const MAX_COLUMNS: usize = 100;
/// How often an import is planned again when a concurrent writer wins a unique key between the
/// planning and the write (see [`with_replan`]).
pub const MAX_PLAN_ATTEMPTS: usize = 3;

// ---------------------------------------------------------------------------------------------
// Writing
// ---------------------------------------------------------------------------------------------

/// One cell, quoted when needed. A value that would run as a formula in a spreadsheet gets a
/// leading apostrophe; [`unquote_formula`] undoes that on import. A literal value that starts with
/// apostrophes followed by a formula character (`'=x`) is protected the same way (`''=x`), so the
/// two cases stay distinguishable and every value round-trips exactly.
pub fn quote_cell(value: &str) -> String {
    let value = if needs_protection(value) {
        format!("'{value}")
    } else {
        value.to_string()
    };
    if value.contains([',', '"', '\n', '\r', ';']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value
    }
}

const FORMULA_START: [char; 6] = ['=', '+', '-', '@', '\t', '\r'];

/// True when the value, after any leading apostrophes, starts like a formula. Such a value is
/// exported with one more apostrophe in front.
fn needs_protection(value: &str) -> bool {
    value.trim_start_matches('\'').starts_with(FORMULA_START)
}

/// Removes the one apostrophe [`quote_cell`] added; any other value is returned as it is.
fn unquote_formula(value: &str) -> &str {
    match value.strip_prefix('\'') {
        Some(rest) if needs_protection(value) => rest,
        _ => value,
    }
}

/// A CSV document: comma separated, CRLF lines, header row first, no byte order mark.
pub fn write_csv(header: &[&str], rows: impl IntoIterator<Item = Vec<String>>) -> String {
    let mut out = header.join(",");
    out.push_str("\r\n");
    for r in rows {
        out.push_str(
            &r.iter()
                .map(|c| quote_cell(c))
                .collect::<Vec<_>>()
                .join(","),
        );
        out.push_str("\r\n");
    }
    out
}

/// Headers and body of a CSV download.
pub fn download(filename: &str, body: String) -> (HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/csv; charset=utf-8"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{filename}\""))
            .unwrap_or_else(|_| HeaderValue::from_static("attachment")),
    );
    (headers, body)
}

// ---------------------------------------------------------------------------------------------
// Report
// ---------------------------------------------------------------------------------------------

/// One problem in an import file.
#[derive(Debug, Clone, Serialize, ToSchema, PartialEq, Eq)]
pub struct RowError {
    /// Line in the file where the row starts (the header is line 1).
    pub row: usize,
    /// Column name; null when the problem concerns the whole row.
    pub column: Option<String>,
    pub message: String,
}

/// Outcome of an import (or of its dry run). `errors` is empty on success; with errors nothing was
/// written and the counts are 0.
#[derive(Debug, Default, Serialize, ToSchema)]
pub struct ImportReport {
    pub created: usize,
    pub updated: usize,
    pub unchanged: usize,
    pub errors: Vec<RowError>,
}

impl ImportReport {
    /// The report of a planned import: the counts are filled in by the caller. `width_errors` (rows
    /// the parser could not use) and `plan_errors` (found while checking the other rows) are
    /// combined in file order.
    pub fn planned(
        unchanged: usize,
        width_errors: &[RowError],
        plan_errors: Vec<RowError>,
    ) -> Self {
        let mut errors = width_errors.to_vec();
        errors.extend(plan_errors);
        errors.sort_by_key(|e| e.row);
        ImportReport {
            unchanged,
            errors,
            ..Default::default()
        }
    }

    /// 200 with the counts, or 422 with the errors.
    pub fn into_response(mut self) -> (StatusCode, Json<ImportReport>) {
        if self.errors.is_empty() {
            (StatusCode::OK, Json(self))
        } else {
            self.created = 0;
            self.updated = 0;
            self.unchanged = 0;
            (StatusCode::UNPROCESSABLE_ENTITY, Json(self))
        }
    }

    /// A report of an import that was rejected before planning (nothing is counted).
    pub fn failed(errors: Vec<RowError>) -> Self {
        ImportReport {
            errors,
            ..Default::default()
        }
    }
}

/// Runs an import attempt and runs it again (a fresh plan in a fresh transaction) when it fails
/// with a conflict: a concurrent writer took a unique key (a barcode, a new code) or changed the
/// stock after the plan was made. The new plan sees the committed data and reports the conflict as
/// a row error or turns the create into an update. After [`MAX_PLAN_ATTEMPTS`] the conflict is
/// returned as it is.
pub async fn with_replan<T, F, Fut>(mut attempt: F) -> AppResult<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = AppResult<T>>,
{
    let mut tries = 1;
    loop {
        match attempt().await {
            Err(AppError::Conflict(_)) if tries < MAX_PLAN_ATTEMPTS => tries += 1,
            result => return result,
        }
    }
}

/// Query parameters of the import endpoints.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ImportParams {
    /// Validate and count only; nothing is written.
    #[serde(default)]
    pub dry_run: bool,
}

// ---------------------------------------------------------------------------------------------
// Reading
// ---------------------------------------------------------------------------------------------

/// The delimiter of a file: `;` when the header line has more semicolons than commas (Excel in
/// many locales), else `,`.
pub fn detect_delimiter(text: &str) -> char {
    let (mut commas, mut semis, mut quoted) = (0, 0, false);
    for c in text.chars() {
        match c {
            '"' => quoted = !quoted,
            '\n' if !quoted => break,
            ',' if !quoted => commas += 1,
            ';' if !quoted => semis += 1,
            _ => {}
        }
    }
    if semis > commas {
        ';'
    } else {
        ','
    }
}

/// A format error of the file as a whole at `row`.
fn format_error(row: usize, message: impl Into<String>) -> RowError {
    RowError {
        row,
        column: None,
        message: message.into(),
    }
}

/// Ends the current record: pushes the last field and keeps the record unless the line is blank.
/// Fails when this would be more records than the header plus [`MAX_ROWS`] data rows.
fn finish_record(
    records: &mut Vec<(usize, Vec<String>)>,
    fields: &mut Vec<String>,
    field: &mut String,
    start: usize,
) -> Result<(), RowError> {
    fields.push(std::mem::take(field));
    let blank = fields.len() == 1 && fields[0].is_empty();
    let record = std::mem::take(fields);
    if blank {
        return Ok(());
    }
    if records.len() > MAX_ROWS {
        return Err(format_error(
            1,
            format!("too many rows: at most {MAX_ROWS} are accepted"),
        ));
    }
    records.push((start, record));
    Ok(())
}

/// Splits RFC 4180 text into records, each with the line number where it starts. Blank lines are
/// skipped. The shape is checked while reading, before anything large is built: a quote inside an
/// unquoted value, text after a closing quote, an unclosed quote, more than [`MAX_ROWS`] data rows
/// or more than [`MAX_COLUMNS`] cells in a record end the parse with one error.
fn parse_records(text: &str, delim: char) -> Result<Vec<(usize, Vec<String>)>, RowError> {
    let mut records = Vec::new();
    let mut fields: Vec<String> = Vec::new();
    let mut field = String::new();
    let (mut line, mut start, mut quoted, mut closed) = (1usize, 1usize, false, false);
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if quoted {
            match c {
                '"' if chars.peek() == Some(&'"') => {
                    chars.next();
                    field.push('"');
                }
                '"' => {
                    quoted = false;
                    closed = true;
                }
                '\n' => {
                    line += 1;
                    field.push(c);
                }
                _ => field.push(c),
            }
            continue;
        }
        match c {
            '"' if field.is_empty() && !closed => quoted = true,
            '"' if closed => {
                return Err(format_error(start, "unexpected text after a closing quote"))
            }
            '"' => {
                return Err(format_error(
                    start,
                    "a quote inside an unquoted value; quote the whole value and double the quotes in it",
                ))
            }
            c if c == delim => {
                fields.push(std::mem::take(&mut field));
                closed = false;
                if fields.len() >= MAX_COLUMNS {
                    return Err(format_error(
                        start,
                        format!("too many columns: at most {MAX_COLUMNS} are accepted"),
                    ));
                }
            }
            '\r' => {}
            '\n' => {
                finish_record(&mut records, &mut fields, &mut field, start)?;
                closed = false;
                line += 1;
                start = line;
            }
            _ if closed => return Err(format_error(start, "unexpected text after a closing quote")),
            _ => field.push(c),
        }
    }
    if quoted {
        return Err(format_error(start, "a quoted value is not closed"));
    }
    if !field.is_empty() || !fields.is_empty() {
        finish_record(&mut records, &mut fields, &mut field, start)?;
    }
    Ok(records)
}

/// A parsed import file: known columns in any order, one [`CsvRow`] per usable data row.
#[derive(Debug)]
pub struct CsvTable {
    pub columns: Vec<String>,
    pub rows: Vec<CsvRow>,
    /// Rows left out of `rows` because their width does not match the header. The importers
    /// check the other rows anyway and report these together with their own errors.
    pub width_errors: Vec<RowError>,
}

/// One data row; cell values are exactly as written (see [`Cells`] for typed access).
#[derive(Debug)]
pub struct CsvRow {
    pub line: usize,
    cells: HashMap<String, String>,
}

impl CsvRow {
    /// The cell as written (surrounding spaces kept), if the file has the column.
    pub fn get(&self, column: &str) -> Option<&str> {
        self.cells.get(column).map(String::as_str)
    }

    /// The cell without surrounding spaces; None when the file lacks the column or the cell is
    /// empty or only spaces.
    pub fn get_trimmed(&self, column: &str) -> Option<&str> {
        self.get(column).map(str::trim).filter(|v| !v.is_empty())
    }
}

impl CsvTable {
    /// Whether the file has this column.
    pub fn has(&self, column: &str) -> bool {
        self.columns.iter().any(|c| c == column)
    }
}

/// Parses an import body. Header problems (unknown, duplicate or missing columns, an empty file,
/// too many rows or columns, malformed quoting, bad encoding) are returned as errors on line 1 /
/// the offending line. A data row with a different number of cells than the header is a row error
/// (extra cells are tolerated when they are all empty) and is left out of the table; the other
/// rows are kept. Cell values are not trimmed (see [`Cells`]); header names are.
pub fn parse_table(
    body: &[u8],
    known: &[&str],
    required: &[&str],
) -> Result<CsvTable, Vec<RowError>> {
    let text = std::str::from_utf8(body)
        .map_err(|_| vec![format_error(1, "the file is not valid UTF-8")])?;
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let delim = detect_delimiter(text);
    let mut records = parse_records(text, delim).map_err(|e| vec![e])?;
    if records.is_empty() {
        return Err(vec![format_error(1, "the file is empty")]);
    }
    let (header_line, header) = records.remove(0);
    let columns: Vec<String> = header.iter().map(|h| h.trim().to_lowercase()).collect();

    let mut errors = Vec::new();
    let mut seen = HashSet::new();
    for c in &columns {
        if !known.contains(&c.as_str()) {
            errors.push(RowError {
                row: header_line,
                column: Some(c.clone()),
                message: format!("unknown column; known columns are {}", known.join(", ")),
            });
        } else if !seen.insert(c.as_str()) {
            errors.push(RowError {
                row: header_line,
                column: Some(c.clone()),
                message: "column appears twice".into(),
            });
        }
    }
    for r in required {
        if !columns.iter().any(|c| c == r) {
            errors.push(RowError {
                row: header_line,
                column: Some((*r).to_string()),
                message: "required column is missing".into(),
            });
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }

    let mut rows = Vec::with_capacity(records.len());
    let mut width_errors = Vec::new();
    for (line, mut cells) in records {
        let wrong_width = cells.len() < columns.len()
            || cells[columns.len()..].iter().any(|c| !c.trim().is_empty());
        if wrong_width {
            width_errors.push(format_error(
                line,
                format!(
                    "{} values but the header has {} columns",
                    cells.len(),
                    columns.len()
                ),
            ));
            continue;
        }
        cells.truncate(columns.len());
        let map = columns
            .iter()
            .zip(cells.iter())
            .map(|(c, v)| (c.clone(), unquote_formula(v).to_string()))
            .collect();
        rows.push(CsvRow { line, cells: map });
    }
    Ok(CsvTable {
        columns,
        rows,
        width_errors,
    })
}

/// What a cell holds: the column is not in the file, the cell is empty, or it has a value.
#[derive(Debug, Clone, PartialEq)]
pub enum Cell<T> {
    Absent,
    Blank,
    Value(T),
}

impl<T> Cell<T> {
    /// Sets a required field: only a value changes it (blank and absent keep the current one).
    pub fn apply(self, target: &mut T) {
        if let Cell::Value(v) = self {
            *target = v;
        }
    }

    /// Sets an optional field: a value sets it, a blank cell clears it, an absent column keeps it.
    pub fn apply_opt(self, target: &mut Option<T>) {
        match self {
            Cell::Value(v) => *target = Some(v),
            Cell::Blank => *target = None,
            Cell::Absent => {}
        }
    }

    /// The value, if the cell has one.
    pub fn value(self) -> Option<T> {
        match self {
            Cell::Value(v) => Some(v),
            _ => None,
        }
    }
}

/// Typed access to the cells of one row; parse problems are recorded as row errors.
///
/// Text cells ([`Cells::text`]) keep their surrounding spaces, so an exported value comes back
/// exactly as it was; an empty cell is blank, a cell of only spaces is a value (it must not clear
/// or change a stored value of that kind). Numbers, booleans and codes/keywords
/// ([`Cells::trimmed`], [`Cells::decimal`], ...) are read without surrounding spaces, and there a
/// cell of only spaces is blank.
pub struct Cells<'a> {
    row: &'a CsvRow,
    errors: &'a mut Vec<RowError>,
}

impl<'a> Cells<'a> {
    /// Typed access to `row`; problems are pushed onto `errors`.
    pub fn new(row: &'a CsvRow, errors: &'a mut Vec<RowError>) -> Self {
        Cells { row, errors }
    }

    /// How many errors have been recorded so far (in the whole import).
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Records a problem with the row (`column` None = the row as a whole).
    pub fn error(&mut self, column: Option<&str>, message: impl Into<String>) {
        self.errors.push(RowError {
            row: self.row.line,
            column: column.map(str::to_string),
            message: message.into(),
        });
    }

    /// Whether an error for `column` was recorded since the count was `since`
    /// (see [`Cells::error_count`]).
    pub fn failed_since(&self, since: usize, column: &str) -> bool {
        self.errors[since..]
            .iter()
            .any(|e| e.column.as_deref() == Some(column))
    }

    /// Records one error per validation message, on the first of `columns` the message starts
    /// with (the row as a whole when none). The rules of the API name their field first.
    pub fn problems(&mut self, messages: impl IntoIterator<Item = String>, columns: &[&str]) {
        for m in messages {
            let column = columns.iter().find(|c| m.starts_with(**c)).copied();
            self.error(column, m);
        }
    }

    fn raw(&self, column: &str) -> Cell<&str> {
        match self.row.cells.get(column).map(String::as_str) {
            None => Cell::Absent,
            Some("") => Cell::Blank,
            Some(v) => Cell::Value(v),
        }
    }

    fn trimmed_raw(&self, column: &str) -> Cell<&str> {
        match self.raw(column) {
            Cell::Value(v) if v.trim().is_empty() => Cell::Blank,
            Cell::Value(v) => Cell::Value(v.trim()),
            other => other,
        }
    }

    /// A free text cell, exactly as written.
    pub fn text(&self, column: &str) -> Cell<String> {
        match self.raw(column) {
            Cell::Absent => Cell::Absent,
            Cell::Blank => Cell::Blank,
            Cell::Value(v) => Cell::Value(v.to_string()),
        }
    }

    /// A code or keyword cell: surrounding spaces removed, only spaces = blank.
    pub fn trimmed(&self, column: &str) -> Cell<String> {
        match self.trimmed_raw(column) {
            Cell::Absent => Cell::Absent,
            Cell::Blank => Cell::Blank,
            Cell::Value(v) => Cell::Value(v.to_string()),
        }
    }

    fn parsed<T>(&mut self, column: &str, parse: impl Fn(&str) -> Result<T, String>) -> Cell<T> {
        match self.trimmed_raw(column) {
            Cell::Absent => Cell::Absent,
            Cell::Blank => Cell::Blank,
            Cell::Value(v) => match parse(v) {
                Ok(t) => Cell::Value(t),
                Err(m) => {
                    self.error(Some(column), m);
                    Cell::Absent
                }
            },
        }
    }

    /// A decimal cell, see [`parse_decimal`].
    pub fn decimal(&mut self, column: &str) -> Cell<Decimal> {
        self.parsed(column, parse_decimal)
    }

    /// A whole number cell.
    pub fn int(&mut self, column: &str) -> Cell<i32> {
        self.parsed(column, |v| {
            v.parse()
                .map_err(|_| format!("{v:?} is not a whole number"))
        })
    }

    /// A boolean cell, see [`parse_bool`].
    pub fn boolean(&mut self, column: &str) -> Cell<bool> {
        self.parsed(column, parse_bool)
    }
}

/// A decimal with `.` or `,` as the decimal separator. Thousands separators are not accepted, so
/// `1,234` is 1.234 and `1.234,5` is rejected rather than guessed. A value that does not fit a
/// [`Decimal`] exactly (more than 28 decimals or 28 digits) is rejected, never rounded.
pub fn parse_decimal(v: &str) -> Result<Decimal, String> {
    let bad = || format!("{v:?} is not a number");
    if v.contains(',') && v.contains('.') {
        return Err(format!(
            "{v:?} is ambiguous: use a single decimal separator and no thousands separator"
        ));
    }
    let norm = v.replace(',', ".");
    Decimal::from_str_exact(&norm).map_err(|_| match norm.parse::<Decimal>() {
        Ok(_) => format!("{v:?} has more digits than can be stored without rounding"),
        Err(_) => bad(),
    })
}

/// A boolean: `true/false/1/0/yes/no/y/n`, any case.
pub fn parse_bool(v: &str) -> Result<bool, String> {
    match v.to_lowercase().as_str() {
        "true" | "1" | "yes" | "y" => Ok(true),
        "false" | "0" | "no" | "n" => Ok(false),
        _ => Err(format!("{v:?} is not true or false")),
    }
}

/// Decimal as written by exports: plain, no exponent, `.` separator, trailing zeros trimmed.
pub fn format_decimal(d: Decimal) -> String {
    d.normalize().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quoting_and_formula_guard_round_trip() {
        assert_eq!(quote_cell("plain"), "plain");
        assert_eq!(quote_cell("a,b"), "\"a,b\"");
        assert_eq!(quote_cell("say \"hi\""), "\"say \"\"hi\"\"\"");
        assert_eq!(quote_cell("+32 9"), "'+32 9");
        let t = parse_table(
            write_csv(
                &["code", "name"],
                [vec!["a".into(), "+32 9\n\"x\", y".into()]],
            )
            .as_bytes(),
            &["code", "name"],
            &["code"],
        )
        .unwrap();
        assert_eq!(t.rows[0].cells["name"], "+32 9\n\"x\", y");
    }

    #[test]
    fn delimiter_bom_order_and_line_numbers() {
        let text = "\u{feff} Name ;CODE\r\n\r\nAlpha;a1\r\n\"B;eta\";b2\r\n";
        assert_eq!(detect_delimiter(text), ';');
        let t = parse_table(text.as_bytes(), &["code", "name"], &["code"]).unwrap();
        assert_eq!(t.columns, ["name", "code"]);
        assert_eq!(t.rows.len(), 2);
        assert_eq!((t.rows[0].line, t.rows[1].line), (3, 4));
        assert_eq!(t.rows[1].cells["name"], "B;eta");
    }

    #[test]
    fn header_problems_are_all_reported() {
        let e =
            parse_table(b"code,bogus,code\n", &["code", "name"], &["code", "name"]).unwrap_err();
        let msgs: Vec<_> = e.iter().map(|e| e.message.as_str()).collect();
        assert!(msgs.iter().any(|m| m.starts_with("unknown column")));
        assert!(msgs.contains(&"column appears twice"));
        assert!(msgs.contains(&"required column is missing"));
        assert!(parse_table(b"", &["code"], &[]).is_err());
        assert!(parse_table(b"code\n\"open", &["code"], &[]).is_err());
    }

    #[test]
    fn decimals_and_bools() {
        assert_eq!(parse_decimal("12,5").unwrap(), Decimal::new(125, 1));
        assert_eq!(parse_decimal("12.5").unwrap(), Decimal::new(125, 1));
        assert!(parse_decimal("1.234,5").is_err());
        assert!(parse_decimal("abc").is_err());
        assert_eq!(format_decimal(Decimal::new(21_00, 2)), "21");
        assert!(parse_bool("Yes").unwrap());
        assert!(parse_bool("maybe").is_err());
    }

    fn one_column(values: &[&str]) -> Vec<String> {
        let csv = write_csv(&["v"], values.iter().map(|v| vec![v.to_string()]));
        let t = parse_table(csv.as_bytes(), &["v"], &["v"]).unwrap();
        assert!(t.width_errors.is_empty());
        t.rows.iter().map(|r| r.cells["v"].clone()).collect()
    }

    #[test]
    fn formulas_and_literal_apostrophes_are_distinguishable() {
        assert_eq!(quote_cell("=1+1"), "'=1+1");
        assert_eq!(quote_cell("'=Literal"), "''=Literal");
        assert_eq!(quote_cell("''-x"), "'''-x");
        assert_eq!(quote_cell("'plain"), "'plain");
        assert_eq!(quote_cell("'"), "'");
        assert_eq!(quote_cell(" =x"), " =x");
        let values = [
            "=1+1",
            "'=Literal",
            "''=Literal",
            "'+1",
            "'plain",
            "'",
            "''",
            "@a",
            "-5",
            " =x",
            "a'=b",
            "\t x",
        ];
        assert_eq!(one_column(&values), values);
    }

    #[test]
    fn cells_keep_their_surrounding_spaces() {
        let t = parse_table(b"v,w\n  a b  , \n", &["v", "w"], &["v"]).unwrap();
        assert_eq!(t.rows[0].cells["v"], "  a b  ");
        assert_eq!(t.rows[0].cells["w"], " ");
        let mut errors = Vec::new();
        let mut cells = Cells::new(&t.rows[0], &mut errors);
        assert_eq!(cells.text("w"), Cell::Value(" ".to_string()));
        assert_eq!(cells.trimmed("v"), Cell::Value("a b".to_string()));
        assert_eq!(cells.trimmed("w"), Cell::Blank);
        assert_eq!(cells.decimal("w"), Cell::Blank);
        assert!(errors.is_empty());
    }

    #[test]
    fn rows_of_the_wrong_width_are_errors_but_the_other_rows_are_kept() {
        let t = parse_table(
            b"a,b,c\n1,2,3\n4,5\n6,7,8,9\n10,11,12,\n13,14,15,  \n\n",
            &["a", "b", "c"],
            &["a"],
        )
        .unwrap();
        let lines: Vec<_> = t.rows.iter().map(|r| r.line).collect();
        assert_eq!(lines, [2, 5, 6]);
        let bad: Vec<_> = t.width_errors.iter().map(|e| e.row).collect();
        assert_eq!(bad, [3, 4]);
        assert!(t.width_errors[0].message.contains("2 values"));
    }

    #[test]
    fn quoting_must_be_well_formed() {
        for text in [
            "v\nab\"cd\"\n",
            "v\n\"ab\"cd\n",
            "v\n\"ab\" \n",
            "v\n a\"b\n",
            "v\n\"ab\"\"\n",
        ] {
            let e = parse_table(text.as_bytes(), &["v"], &[]).unwrap_err();
            assert_eq!(e.len(), 1, "{text:?}");
            assert_eq!(e[0].row, 2, "{text:?}");
        }
        // Valid: quoted delimiters, escaped quotes, empty quoted value, quote then CRLF.
        let t = parse_table(b"v,w\r\n\"a,\"\"b\"\"\",\"\"\r\n", &["v", "w"], &["v"]).unwrap();
        assert_eq!(t.rows[0].cells["v"], "a,\"b\"");
        assert_eq!(t.rows[0].cells["w"], "");
    }

    #[test]
    fn row_and_column_limits_stop_the_parse() {
        let mut ok = String::from("v\n");
        ok.push_str(&"x\n".repeat(MAX_ROWS));
        let t = parse_table(ok.as_bytes(), &["v"], &[]).unwrap();
        assert_eq!(t.rows.len(), MAX_ROWS);
        let e = parse_table(format!("{ok}x\n").as_bytes(), &["v"], &[]).unwrap_err();
        assert_eq!(e.len(), 1);
        assert!(e[0].message.contains("too many rows"));

        // Hundreds of unknown columns give one bounded error, not one error per column.
        let wide = vec!["c"; MAX_COLUMNS + 1].join(",");
        let e = parse_table(format!("{wide}\n").as_bytes(), &["v"], &[]).unwrap_err();
        assert_eq!(e.len(), 1);
        assert!(e[0].message.contains("too many columns"));
    }

    #[test]
    fn decimals_never_lose_precision() {
        let exact = "0.1234567890123456789012345678";
        assert_eq!(parse_decimal(exact).unwrap().to_string(), exact);
        assert_eq!(parse_decimal("21,50").unwrap().to_string(), "21.50");
        let e = parse_decimal("21.1234567890123456789012345678901").unwrap_err();
        assert!(e.contains("more digits"), "{e}");
        assert!(parse_decimal("1e").unwrap_err().contains("not a number"));
    }
}
