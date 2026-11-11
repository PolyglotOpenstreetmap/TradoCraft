//! Lenient deserialisers for values sent by generated clients.

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};

fn parse_date(s: &str) -> Option<NaiveDate> {
    // Accept `2026-11-05` and datetime strings like `2026-11-05T00:00:00.000` (Dart DateTime).
    NaiveDate::parse_from_str(s.get(..10)?, "%Y-%m-%d").ok()
}

pub fn date<'de, D: Deserializer<'de>>(d: D) -> Result<NaiveDate, D::Error> {
    let s = String::deserialize(d)?;
    parse_date(&s)
        .ok_or_else(|| serde::de::Error::custom(format!("invalid date {s:?}, expected YYYY-MM-DD")))
}

pub fn opt_date<'de, D: Deserializer<'de>>(d: D) -> Result<Option<NaiveDate>, D::Error> {
    match Option::<String>::deserialize(d)? {
        None => Ok(None),
        Some(s) => parse_date(&s).map(Some).ok_or_else(|| {
            serde::de::Error::custom(format!("invalid date {s:?}, expected YYYY-MM-DD"))
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct T {
        #[serde(default, deserialize_with = "opt_date")]
        d: Option<NaiveDate>,
    }

    #[test]
    fn accepts_dates_and_datetimes() {
        let expected = NaiveDate::from_ymd_opt(2026, 11, 5);
        for json in [
            r#"{"d":"2026-11-05"}"#,
            r#"{"d":"2026-11-05T00:00:00.000"}"#,
        ] {
            assert_eq!(serde_json::from_str::<T>(json).unwrap().d, expected);
        }
        assert_eq!(serde_json::from_str::<T>(r#"{"d":null}"#).unwrap().d, None);
        assert_eq!(serde_json::from_str::<T>("{}").unwrap().d, None);
        assert!(serde_json::from_str::<T>(r#"{"d":"05/11/2026"}"#).is_err());
    }
}
