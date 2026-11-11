//! Typed lookup tables: `exact` (all key columns match) and `range` (key columns match and a value
//! lies between a min and a max column). A lookup returns the whole row as a record; `offset`
//! moves to a later (positive) or earlier (negative) row; [`OnMissing`] decides what happens when
//! no row fits or the offset runs past the table.

use std::collections::BTreeMap;

use crate::definition::{ColumnType, DefinitionError, OnMissing, Table, TableKind};
use crate::expr::value::parse_decimal;
use crate::expr::{TableInfo, Type, Value};

#[derive(Debug, Clone)]
pub struct CompiledTable {
    pub def: Table,
    pub rows: Vec<BTreeMap<String, Value>>,
}

fn column_ty(c: ColumnType) -> Type {
    match c {
        ColumnType::Number => Type::Dec,
        ColumnType::Integer => Type::Int,
        ColumnType::Text => Type::Str,
        ColumnType::Boolean => Type::Bool,
    }
}

fn cell(c: ColumnType, v: &serde_json::Value) -> Option<Value> {
    use serde_json::Value as J;
    Some(match (c, v) {
        (_, J::Null) => Value::Null,
        (ColumnType::Number, J::Number(n)) => Value::Dec(parse_decimal(&n.to_string())?),
        (ColumnType::Number, J::String(s)) => Value::Dec(parse_decimal(s)?),
        (ColumnType::Integer, J::Number(n)) => Value::Int(n.as_i64()?),
        (ColumnType::Text, J::String(s)) => Value::Str(s.clone()),
        (ColumnType::Boolean, J::Bool(b)) => Value::Bool(*b),
        _ => return None,
    })
}

impl CompiledTable {
    pub fn compile(t: &Table, path: &str) -> Result<CompiledTable, Vec<DefinitionError>> {
        let mut errors = Vec::new();
        let mut err = |p: String, m: String| errors.push(DefinitionError::new(p, m));
        let mut cols: BTreeMap<&str, ColumnType> = BTreeMap::new();
        if t.columns.is_empty() {
            err(format!("{path}.columns"), "a table needs columns".into());
        }
        for (i, c) in t.columns.iter().enumerate() {
            if !crate::compile::valid_name(&c.name) {
                err(
                    format!("{path}.columns[{i}].name"),
                    format!("'{}' is not a valid column name", c.name),
                );
            }
            if cols.insert(&c.name, c.column_type).is_some() {
                err(
                    format!("{path}.columns[{i}].name"),
                    format!("column {} is defined twice", c.name),
                );
            }
        }
        for (i, k) in t.keys.iter().enumerate() {
            if !cols.contains_key(k.as_str()) {
                err(format!("{path}.keys[{i}]"), format!("{k} is not a column"));
            }
        }
        match (t.kind, &t.range) {
            (TableKind::Exact, Some(_)) => err(
                format!("{path}.range"),
                "range is only for range tables".into(),
            ),
            (TableKind::Exact, None) if t.keys.is_empty() => err(
                format!("{path}.keys"),
                "an exact table needs at least one key".into(),
            ),
            (TableKind::Range, None) => err(
                format!("{path}.range"),
                "a range table needs range (key, min, max)".into(),
            ),
            (TableKind::Range, Some(r)) => {
                for (attr, c) in [("min", &r.min), ("max", &r.max)] {
                    match cols.get(c.as_str()) {
                        None => err(
                            format!("{path}.range.{attr}"),
                            format!("{c} is not a column"),
                        ),
                        Some(ColumnType::Number | ColumnType::Integer) => {}
                        Some(_) => err(
                            format!("{path}.range.{attr}"),
                            format!("{c} must be a number column"),
                        ),
                    }
                }
                if !crate::compile::valid_name(&r.key) || t.keys.contains(&r.key) {
                    err(
                        format!("{path}.range.key"),
                        format!("'{}' must be a name different from the keys", r.key),
                    );
                }
            }
            _ => {}
        }
        let mut rows = Vec::with_capacity(t.rows.len());
        for (ri, row) in t.rows.iter().enumerate() {
            let mut out = BTreeMap::new();
            for (k, v) in row {
                match cols.get(k.as_str()) {
                    None => err(
                        format!("{path}.rows[{ri}].{k}"),
                        format!("{k} is not a column"),
                    ),
                    Some(ct) => match cell(*ct, v) {
                        Some(x) => {
                            out.insert(k.clone(), x);
                        }
                        None => err(
                            format!("{path}.rows[{ri}].{k}"),
                            format!("{v} is not a valid {ct:?} value").to_lowercase(),
                        ),
                    },
                }
            }
            for c in &t.columns {
                out.entry(c.name.clone()).or_insert(Value::Null);
            }
            for k in &t.keys {
                if out.get(k).is_some_and(Value::is_null) {
                    err(
                        format!("{path}.rows[{ri}].{k}"),
                        format!("key {k} is empty"),
                    );
                }
            }
            if let Some(r) = &t.range {
                let lo = out.get(&r.min).and_then(Value::as_decimal);
                let hi = out.get(&r.max).and_then(Value::as_decimal);
                if let (Some(a), Some(b)) = (lo, hi) {
                    if a > b {
                        err(
                            format!("{path}.rows[{ri}]"),
                            format!("{} is larger than {}", r.min, r.max),
                        );
                    }
                }
            }
            rows.push(out);
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(CompiledTable {
            def: t.clone(),
            rows,
        })
    }

    pub fn column_type(&self, c: &str) -> Option<Type> {
        self.def
            .columns
            .iter()
            .find(|x| x.name == c)
            .map(|x| column_ty(x.column_type))
    }

    pub fn info(&self) -> TableInfo {
        let mut keys = self.def.keys.clone();
        if let Some(r) = &self.def.range {
            keys.push(r.key.clone());
        }
        TableInfo {
            columns: self
                .def
                .columns
                .iter()
                .map(|c| (c.name.clone(), column_ty(c.column_type)))
                .collect(),
            keys,
        }
    }

    pub fn rows_value(&self) -> Value {
        Value::List(self.rows.iter().cloned().map(Value::Record).collect())
    }

    fn row(&self, i: usize) -> Value {
        Value::Record(self.rows[i].clone())
    }

    /// Applies the offset to a matched position among `candidates` (row indexes).
    fn offset(&self, candidates: &[usize], pos: usize, offset: i64) -> Result<Value, String> {
        let target = (pos as i64).saturating_add(offset);
        if target >= 0 && (target as usize) < candidates.len() {
            return Ok(self.row(candidates[target as usize]));
        }
        match self.def.on_missing {
            OnMissing::Clamp => {
                let k = target.clamp(0, candidates.len() as i64 - 1) as usize;
                Ok(self.row(candidates[k]))
            }
            OnMissing::Null => Ok(Value::Null),
            OnMissing::Error => Err(format!(
                "offset {offset} goes past the end of table {}",
                self.def.name
            )),
        }
    }

    pub fn lookup(&self, keys: &BTreeMap<String, Value>, offset: i64) -> Result<Value, String> {
        let name = &self.def.name;
        let range_key = self.def.range.as_ref().map(|r| r.key.as_str());
        for k in keys.keys() {
            if !self.def.keys.contains(k) && Some(k.as_str()) != range_key {
                return Err(format!("{k} is not a key of table {name}"));
            }
        }
        let mut given = Vec::new();
        for k in &self.def.keys {
            match keys.get(k) {
                None => return Err(format!("the lookup in {name} needs key {k}")),
                // A missing input gives no row rather than an error.
                Some(Value::Null) => return Ok(Value::Null),
                Some(v) => given.push((k, v)),
            }
        }
        let matching: Vec<usize> = (0..self.rows.len())
            .filter(|i| {
                given
                    .iter()
                    .all(|(k, v)| self.rows[*i].get(*k).is_some_and(|x| x.loose_eq(v)))
            })
            .collect();
        let describe = || {
            given
                .iter()
                .map(|(k, v)| format!("{k} = {}", v.to_text()))
                .collect::<Vec<_>>()
                .join(", ")
        };
        match &self.def.range {
            None => {
                let Some(&first) = matching.first() else {
                    return match self.def.on_missing {
                        OnMissing::Error => Err(format!("no row in {name} for {}", describe())),
                        _ => Ok(Value::Null),
                    };
                };
                // Offsets move through the whole table in row order.
                let all: Vec<usize> = (0..self.rows.len()).collect();
                self.offset(&all, first, offset)
            }
            Some(r) => {
                let v = match keys.get(&r.key) {
                    None => return Err(format!("the lookup in {name} needs key {}", r.key)),
                    Some(Value::Null) => return Ok(Value::Null),
                    Some(v) => v
                        .as_decimal()
                        .ok_or_else(|| format!("{} must be a number", r.key))?,
                };
                let mut cands = matching;
                let lo = |i: &usize| self.rows[*i].get(&r.min).and_then(Value::as_decimal);
                let hi = |i: &usize| self.rows[*i].get(&r.max).and_then(Value::as_decimal);
                cands.sort_by(|a, b| match (lo(a), lo(b)) {
                    (None, None) => std::cmp::Ordering::Equal,
                    (None, _) => std::cmp::Ordering::Less,
                    (_, None) => std::cmp::Ordering::Greater,
                    (Some(x), Some(y)) => x.cmp(&y),
                });
                if let Some(pos) = cands
                    .iter()
                    .position(|i| lo(i).is_none_or(|m| m <= v) && hi(i).is_none_or(|m| v <= m))
                {
                    return self.offset(&cands, pos, offset);
                }
                let outside = || {
                    let what = if describe().is_empty() {
                        String::new()
                    } else {
                        format!(" for {}", describe())
                    };
                    format!(
                        "{} = {} is outside the ranges of table {name}{what}",
                        r.key,
                        v.normalize()
                    )
                };
                match self.def.on_missing {
                    OnMissing::Error => Err(outside()),
                    OnMissing::Null => Ok(Value::Null),
                    OnMissing::Clamp => {
                        if cands.is_empty() {
                            return Ok(Value::Null);
                        }
                        // The next row up, or the last row when the value is above them all.
                        let pos = cands
                            .iter()
                            .position(|i| lo(i).is_some_and(|m| m > v))
                            .unwrap_or(cands.len() - 1);
                        self.offset(&cands, pos, offset)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::{Column, RangeSpec};
    use serde_json::json;

    fn motors(on_missing: OnMissing) -> CompiledTable {
        let t = Table {
            name: "motors".into(),
            kind: TableKind::Range,
            keys: vec!["phases".into()],
            range: Some(RangeSpec {
                key: "kw".into(),
                min: "kw_min".into(),
                max: "kw_max".into(),
            }),
            on_missing,
            columns: vec![
                Column {
                    name: "phases".into(),
                    column_type: ColumnType::Integer,
                },
                Column {
                    name: "kw_min".into(),
                    column_type: ColumnType::Number,
                },
                Column {
                    name: "kw_max".into(),
                    column_type: ColumnType::Number,
                },
                Column {
                    name: "size".into(),
                    column_type: ColumnType::Text,
                },
            ],
            rows: [
                json!({"phases": 3, "kw_min": 0, "kw_max": 0.55, "size": "05"}),
                json!({"phases": 3, "kw_min": 1.1, "kw_max": 2.2, "size": "22"}),
                json!({"phases": 3, "kw_min": 0.55, "kw_max": 1.1, "size": "11"}),
                json!({"phases": 1, "kw_min": 0, "kw_max": 0.55, "size": "05"}),
            ]
            .into_iter()
            .map(|v| serde_json::from_value(v).unwrap())
            .collect(),
            ..Default::default()
        };
        CompiledTable::compile(&t, "$.tables[0]").unwrap()
    }

    fn keys(phases: i64, kw: &str) -> BTreeMap<String, Value> {
        let mut m = BTreeMap::new();
        m.insert("phases".into(), Value::Int(phases));
        m.insert("kw".into(), Value::Dec(kw.parse().unwrap()));
        m
    }

    fn size(v: Result<Value, String>) -> String {
        match v.unwrap() {
            Value::Record(r) => r["size"].to_text(),
            Value::Null => "null".into(),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn range_lookup_with_offset_and_policies() {
        let t = motors(OnMissing::Error);
        assert_eq!(size(t.lookup(&keys(3, "0.3"), 0)), "05");
        // Bounds are inclusive; the first fitting row (by min) wins. Rows need not be sorted.
        assert_eq!(size(t.lookup(&keys(3, "0.55"), 0)), "05");
        assert_eq!(size(t.lookup(&keys(3, "0.7"), 0)), "11");
        assert_eq!(size(t.lookup(&keys(3, "0.7"), 1)), "22");
        assert_eq!(size(t.lookup(&keys(3, "1.5"), -1)), "11");
        let e = t.lookup(&keys(3, "0.7"), 2).unwrap_err();
        assert!(e.contains("past the end"), "{e}");
        let e = t.lookup(&keys(3, "5"), 0).unwrap_err();
        assert!(e.contains("outside the ranges"), "{e}");
        let e = t.lookup(&keys(1, "1"), 0).unwrap_err();
        assert!(e.contains("phases = 1"), "{e}");

        let c = motors(OnMissing::Clamp);
        assert_eq!(size(c.lookup(&keys(3, "5"), 0)), "22");
        assert_eq!(size(c.lookup(&keys(3, "0.7"), 5)), "22");
        assert_eq!(size(c.lookup(&keys(3, "0.7"), i64::MAX)), "22");
        assert_eq!(size(c.lookup(&keys(3, "0.7"), i64::MIN)), "05");
        assert_eq!(size(c.lookup(&keys(1, "0.9"), 0)), "05");
        assert_eq!(size(c.lookup(&keys(2, "0.3"), 0)), "null");

        let n = motors(OnMissing::Null);
        assert_eq!(size(n.lookup(&keys(3, "5"), 0)), "null");
        assert_eq!(size(n.lookup(&keys(3, "0.7"), 9)), "null");
    }

    #[test]
    fn exact_lookup_and_key_errors() {
        let t = Table {
            name: "codes".into(),
            keys: vec!["a".into(), "b".into()],
            columns: vec![
                Column {
                    name: "a".into(),
                    column_type: ColumnType::Text,
                },
                Column {
                    name: "b".into(),
                    column_type: ColumnType::Integer,
                },
                Column {
                    name: "code".into(),
                    column_type: ColumnType::Text,
                },
            ],
            rows: [
                json!({"a": "x", "b": 1, "code": "X1"}),
                json!({"a": "x", "b": 2, "code": "X2"}),
            ]
            .into_iter()
            .map(|v| serde_json::from_value(v).unwrap())
            .collect(),
            ..Default::default()
        };
        let t = CompiledTable::compile(&t, "$").unwrap();
        let k = |a: &str, b: i64| {
            let mut m = BTreeMap::new();
            m.insert("a".to_string(), Value::Str(a.into()));
            m.insert("b".to_string(), Value::Int(b));
            m
        };
        let code = |v: Value| match v {
            Value::Record(r) => r["code"].to_text(),
            _ => "null".into(),
        };
        assert_eq!(code(t.lookup(&k("x", 2), 0).unwrap()), "X2");
        assert_eq!(code(t.lookup(&k("x", 1), 1).unwrap()), "X2");
        assert!(t
            .lookup(&k("y", 1), 0)
            .unwrap_err()
            .contains("no row in codes"));
        let mut only_a = BTreeMap::new();
        only_a.insert("a".to_string(), Value::Str("x".into()));
        assert!(t.lookup(&only_a, 0).unwrap_err().contains("needs key b"));
        let mut bad = k("x", 1);
        bad.insert("zz".into(), Value::Int(1));
        assert!(t.lookup(&bad, 0).unwrap_err().contains("zz is not a key"));
    }

    #[test]
    fn compile_reports_bad_tables() {
        let t: Table = serde_json::from_value(json!({
            "name": "t", "kind": "range", "keys": ["p", "nope"],
            "range": {"key": "p", "min": "lo", "max": "txt"},
            "columns": [{"name": "p", "type": "integer"}, {"name": "lo", "type": "number"}, {"name": "txt", "type": "text"}],
            "rows": [{"p": "three", "lo": 1}, {"p": 1, "other": 2}]
        }))
        .unwrap();
        let errs = CompiledTable::compile(&t, "$.tables[2]").unwrap_err();
        let paths: Vec<&str> = errs.iter().map(|e| e.path.as_str()).collect();
        assert!(paths.contains(&"$.tables[2].keys[1]"), "{errs:?}");
        assert!(paths.contains(&"$.tables[2].range.max"), "{errs:?}");
        assert!(paths.contains(&"$.tables[2].range.key"), "{errs:?}");
        assert!(paths.contains(&"$.tables[2].rows[0].p"), "{errs:?}");
        assert!(paths.contains(&"$.tables[2].rows[1].other"), "{errs:?}");
    }
}
