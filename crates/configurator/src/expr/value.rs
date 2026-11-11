//! Runtime values of the formula language and their JSON form.

use std::collections::BTreeMap;
use std::str::FromStr;

use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Dec(Decimal),
    Str(String),
    List(Vec<Value>),
    Record(BTreeMap<String, Value>),
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Int(_) => "integer",
            Value::Dec(_) => "decimal",
            Value::Str(_) => "text",
            Value::List(_) => "list",
            Value::Record(_) => "record",
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn as_decimal(&self) -> Option<Decimal> {
        match self {
            Value::Int(i) => Some(Decimal::from(*i)),
            Value::Dec(d) => Some(*d),
            _ => None,
        }
    }

    /// Integral numbers as i64 (a decimal like `4.0` counts).
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            Value::Dec(d) if d.fract().is_zero() => i64::try_from(*d).ok(),
            _ => None,
        }
    }

    /// Text shown in templates and comments: decimals without trailing zeros, null as empty.
    pub fn to_text(&self) -> String {
        match self {
            Value::Null => String::new(),
            Value::Bool(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Dec(d) => d.normalize().to_string(),
            Value::Str(s) => s.clone(),
            Value::List(items) => items
                .iter()
                .map(Value::to_text)
                .collect::<Vec<_>>()
                .join(", "),
            Value::Record(m) => m
                .iter()
                .map(|(k, v)| format!("{k}: {}", v.to_text()))
                .collect::<Vec<_>>()
                .join(", "),
        }
    }

    /// Equality as the `==` operator sees it: numbers compare by value.
    pub fn loose_eq(&self, other: &Value) -> bool {
        match (self.as_decimal(), other.as_decimal()) {
            (Some(a), Some(b)) => a == b,
            _ => match (self, other) {
                (Value::List(a), Value::List(b)) => {
                    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.loose_eq(y))
                }
                (Value::Record(a), Value::Record(b)) => {
                    a.len() == b.len()
                        && a.iter()
                            .zip(b)
                            .all(|((ka, va), (kb, vb))| ka == kb && va.loose_eq(vb))
                }
                _ => self == other,
            },
        }
    }

    /// JSON form: decimals are strings (like money elsewhere in the API), integers numbers.
    pub fn to_json(&self) -> serde_json::Value {
        use serde_json::Value as J;
        match self {
            Value::Null => J::Null,
            Value::Bool(b) => J::Bool(*b),
            Value::Int(i) => J::from(*i),
            Value::Dec(d) => J::String(d.normalize().to_string()),
            Value::Str(s) => J::String(s.clone()),
            Value::List(items) => J::Array(items.iter().map(Value::to_json).collect()),
            Value::Record(m) => {
                J::Object(m.iter().map(|(k, v)| (k.clone(), v.to_json())).collect())
            }
        }
    }

    /// Reads JSON without a type hint: whole numbers become integers, other numbers decimals.
    pub fn from_json(v: &serde_json::Value) -> Value {
        use serde_json::Value as J;
        match v {
            J::Null => Value::Null,
            J::Bool(b) => Value::Bool(*b),
            J::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Int(i)
                } else {
                    parse_decimal(&n.to_string())
                        .map(Value::Dec)
                        .unwrap_or(Value::Null)
                }
            }
            J::String(s) => Value::Str(s.clone()),
            J::Array(a) => Value::List(a.iter().map(Value::from_json).collect()),
            J::Object(o) => Value::Record(
                o.iter()
                    .map(|(k, v)| (k.clone(), Value::from_json(v)))
                    .collect(),
            ),
        }
    }
}

/// Parses a decimal written plainly or in scientific notation.
pub fn parse_decimal(s: &str) -> Option<Decimal> {
    let s = s.trim();
    Decimal::from_str(s)
        .ok()
        .or_else(|| Decimal::from_scientific(s).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_round_trip_and_text() {
        let v = Value::from_json(&serde_json::json!({"a": 1, "b": 1.25, "c": [true, null, "x"]}));
        assert_eq!(
            v.to_json(),
            serde_json::json!({"a": 1, "b": "1.25", "c": [true, null, "x"]})
        );
        assert_eq!(Value::Dec("2.500".parse().unwrap()).to_text(), "2.5");
        assert!(Value::Int(2).loose_eq(&Value::Dec("2.0".parse().unwrap())));
        assert_eq!(Value::Dec("4.0".parse().unwrap()).as_int(), Some(4));
        assert_eq!(parse_decimal("1e3"), Some(Decimal::from(1000)));
    }
}
