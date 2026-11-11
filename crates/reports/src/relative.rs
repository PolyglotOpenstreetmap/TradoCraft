//! Relative dates for date filters: `{"relative": "last_n_days", "n": 30}` instead of
//! `YYYY-MM-DD`. A saved report keeps the token, so it is resolved against "today" each time the
//! report runs (`plan` takes `today`, it never reads the clock).
//!
//! Every token stands for an inclusive range of days `[start, end]`; a single day has
//! `start == end`. Weeks start on Monday, quarters are Jan-Mar, Apr-Jun, Jul-Sep, Oct-Dec.

use chrono::{Datelike, Days, NaiveDate};
use serde::{Deserialize, Serialize};

/// Largest `n` of `last_n_days` and `next_n_days`.
pub const MAX_RELATIVE_DAYS: u32 = 3660;

/// The accepted tokens, for error messages.
pub const TOKENS: &str = "today, yesterday, last_n_days, next_n_days, this_week, last_week, \
this_month, last_month, this_quarter, last_quarter, this_year, last_year";

/// A date that is relative to today, such as "last month", resolved on every run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "relative", rename_all = "snake_case", deny_unknown_fields)]
pub enum RelativeDate {
    Today,
    Yesterday,
    /// The `n` days up to and including today.
    LastNDays {
        n: u32,
    },
    /// Today and the `n - 1` days after it.
    NextNDays {
        n: u32,
    },
    ThisWeek,
    LastWeek,
    ThisMonth,
    LastMonth,
    ThisQuarter,
    LastQuarter,
    ThisYear,
    LastYear,
}

fn ymd(y: i32, m: u32, d: u32) -> Result<NaiveDate, String> {
    NaiveDate::from_ymd_opt(y, m, d).ok_or_else(|| "the date is out of range".to_string())
}

/// First day of the month `months` away from the month of `d` (negative is earlier).
fn month_start(d: NaiveDate, months: i32) -> Result<NaiveDate, String> {
    let idx = d.year() * 12 + d.month0() as i32 + months;
    ymd(idx.div_euclid(12), idx.rem_euclid(12) as u32 + 1, 1)
}

/// The `len` months starting `offset` months from the start of `d`'s month.
fn months(d: NaiveDate, offset: i32, len: i32) -> Result<(NaiveDate, NaiveDate), String> {
    let start = month_start(d, offset)?;
    let end = month_start(d, offset + len)?
        .pred_opt()
        .ok_or_else(|| "the date is out of range".to_string())?;
    Ok((start, end))
}

impl RelativeDate {
    /// Parses the JSON object of a filter value, with a message that names the problem.
    pub fn parse(v: &serde_json::Value) -> Result<Self, String> {
        let token = v.get("relative").and_then(|t| t.as_str());
        let r: RelativeDate = serde_json::from_value(v.clone()).map_err(|_| match token {
            Some(t) => format!(
                "'{}' is not a relative date (use {TOKENS}; last_n_days and next_n_days need \
                 \"n\", the others must not have other keys)",
                t.chars().take(40).collect::<String>()
            ),
            None => {
                format!("a relative date looks like {{\"relative\": \"today\"}} (tokens: {TOKENS})")
            }
        })?;
        let keys = v.as_object().map_or(0, |o| o.len());
        match r {
            Self::LastNDays { n } | Self::NextNDays { n } => {
                if !(1..=MAX_RELATIVE_DAYS).contains(&n) {
                    return Err(format!("n must be between 1 and {MAX_RELATIVE_DAYS}"));
                }
                if keys != 2 {
                    return Err("a relative date takes only \"relative\" and \"n\"".into());
                }
            }
            _ if keys != 1 => {
                return Err(format!(
                    "'{}' takes no other keys than \"relative\"",
                    token.unwrap_or_default()
                ))
            }
            _ => {}
        }
        Ok(r)
    }

    /// The inclusive range of days this token covers when today is `today`.
    pub fn range(self, today: NaiveDate) -> Result<(NaiveDate, NaiveDate), String> {
        let oor = || "the date is out of range".to_string();
        let day = |d: Option<NaiveDate>| d.map(|d| (d, d)).ok_or_else(oor);
        match self {
            Self::Today => Ok((today, today)),
            Self::Yesterday => day(today.pred_opt()),
            Self::LastNDays { n } => Ok((
                today
                    .checked_sub_days(Days::new(u64::from(n) - 1))
                    .ok_or_else(oor)?,
                today,
            )),
            Self::NextNDays { n } => Ok((
                today,
                today
                    .checked_add_days(Days::new(u64::from(n) - 1))
                    .ok_or_else(oor)?,
            )),
            Self::ThisWeek | Self::LastWeek => {
                let monday = today
                    .checked_sub_days(Days::new(
                        u64::from(today.weekday().num_days_from_monday())
                            + if self == Self::LastWeek { 7 } else { 0 },
                    ))
                    .ok_or_else(oor)?;
                Ok((
                    monday,
                    monday.checked_add_days(Days::new(6)).ok_or_else(oor)?,
                ))
            }
            Self::ThisMonth => months(today, 0, 1),
            Self::LastMonth => months(today, -1, 1),
            Self::ThisQuarter | Self::LastQuarter => {
                let into = today.month0() as i32 % 3;
                months(
                    today,
                    -into - if self == Self::LastQuarter { 3 } else { 0 },
                    3,
                )
            }
            Self::ThisYear | Self::LastYear => {
                let y = today.year() - i32::from(self == Self::LastYear);
                Ok((ymd(y, 1, 1)?, ymd(y, 12, 31)?))
            }
        }
    }

    /// The token in words, for the PDF.
    pub fn words(self, lang: &str) -> String {
        let pick = |en: &str, nl: &str, fr: &str, de: &str, es: &str| {
            match lang {
                "nl" => nl,
                "fr" => fr,
                "de" => de,
                "es" => es,
                _ => en,
            }
            .to_string()
        };
        match self {
            Self::Today => pick("today", "vandaag", "aujourd'hui", "heute", "hoy"),
            Self::Yesterday => pick("yesterday", "gisteren", "hier", "gestern", "ayer"),
            Self::LastNDays { n } => pick(
                &format!("the last {n} days"),
                &format!("de laatste {n} dagen"),
                &format!("les {n} derniers jours"),
                &format!("die letzten {n} Tage"),
                &format!("los últimos {n} días"),
            ),
            Self::NextNDays { n } => pick(
                &format!("the next {n} days"),
                &format!("de volgende {n} dagen"),
                &format!("les {n} prochains jours"),
                &format!("die nächsten {n} Tage"),
                &format!("los próximos {n} días"),
            ),
            Self::ThisWeek => pick(
                "this week",
                "deze week",
                "cette semaine",
                "diese Woche",
                "esta semana",
            ),
            Self::LastWeek => pick(
                "last week",
                "vorige week",
                "la semaine dernière",
                "letzte Woche",
                "la semana pasada",
            ),
            Self::ThisMonth => pick(
                "this month",
                "deze maand",
                "ce mois",
                "diesen Monat",
                "este mes",
            ),
            Self::LastMonth => pick(
                "last month",
                "vorige maand",
                "le mois dernier",
                "letzten Monat",
                "el mes pasado",
            ),
            Self::ThisQuarter => pick(
                "this quarter",
                "dit kwartaal",
                "ce trimestre",
                "dieses Quartal",
                "este trimestre",
            ),
            Self::LastQuarter => pick(
                "last quarter",
                "vorig kwartaal",
                "le trimestre dernier",
                "letztes Quartal",
                "el trimestre pasado",
            ),
            Self::ThisYear => pick(
                "this year",
                "dit jaar",
                "cette année",
                "dieses Jahr",
                "este año",
            ),
            Self::LastYear => pick(
                "last year",
                "vorig jaar",
                "l'année dernière",
                "letztes Jahr",
                "el año pasado",
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    fn range(v: serde_json::Value, today: NaiveDate) -> (NaiveDate, NaiveDate) {
        RelativeDate::parse(&v).unwrap().range(today).unwrap()
    }

    #[test]
    fn single_days_and_day_counts() {
        let t = d(2026, 11, 11); // a Wednesday
        assert_eq!(range(json!({"relative": "today"}), t), (t, t));
        assert_eq!(
            range(json!({"relative": "yesterday"}), t),
            (d(2026, 11, 10), d(2026, 11, 10))
        );
        assert_eq!(
            range(json!({"relative": "last_n_days", "n": 30}), t),
            (d(2026, 10, 13), t)
        );
        assert_eq!(range(json!({"relative": "last_n_days", "n": 1}), t), (t, t));
        assert_eq!(
            range(json!({"relative": "next_n_days", "n": 7}), t),
            (t, d(2026, 11, 17))
        );
    }

    #[test]
    fn weeks_start_on_monday() {
        let wed = d(2026, 11, 11);
        assert_eq!(
            range(json!({"relative": "this_week"}), wed),
            (d(2026, 11, 9), d(2026, 11, 15))
        );
        // Monday and Sunday belong to the same week as the Wednesday.
        for day in [d(2026, 11, 9), d(2026, 11, 15)] {
            assert_eq!(
                range(json!({"relative": "this_week"}), day),
                (d(2026, 11, 9), d(2026, 11, 15))
            );
        }
        assert_eq!(
            range(json!({"relative": "last_week"}), wed),
            (d(2026, 11, 2), d(2026, 11, 8))
        );
        // Across a year boundary: Thursday 2026-01-01.
        assert_eq!(
            range(json!({"relative": "this_week"}), d(2026, 1, 1)),
            (d(2025, 12, 29), d(2026, 1, 4))
        );
    }

    #[test]
    fn months_quarters_years_and_rollover() {
        let t = d(2027, 1, 15);
        assert_eq!(
            range(json!({"relative": "this_month"}), t),
            (d(2027, 1, 1), d(2027, 1, 31))
        );
        assert_eq!(
            range(json!({"relative": "last_month"}), t),
            (d(2026, 12, 1), d(2026, 12, 31))
        );
        assert_eq!(
            range(json!({"relative": "this_quarter"}), t),
            (d(2027, 1, 1), d(2027, 3, 31))
        );
        assert_eq!(
            range(json!({"relative": "last_quarter"}), t),
            (d(2026, 10, 1), d(2026, 12, 31))
        );
        assert_eq!(
            range(json!({"relative": "this_year"}), t),
            (d(2027, 1, 1), d(2027, 12, 31))
        );
        assert_eq!(
            range(json!({"relative": "last_year"}), t),
            (d(2026, 1, 1), d(2026, 12, 31))
        );
    }

    #[test]
    fn boundaries_of_months_and_quarters() {
        // First and last day of a quarter.
        for day in [d(2026, 4, 1), d(2026, 6, 30)] {
            assert_eq!(
                range(json!({"relative": "this_quarter"}), day),
                (d(2026, 4, 1), d(2026, 6, 30))
            );
            assert_eq!(
                range(json!({"relative": "last_quarter"}), day),
                (d(2026, 1, 1), d(2026, 3, 31))
            );
        }
        assert_eq!(
            range(json!({"relative": "last_quarter"}), d(2026, 10, 1)),
            (d(2026, 7, 1), d(2026, 9, 30))
        );
        // Months of different lengths.
        assert_eq!(
            range(json!({"relative": "last_month"}), d(2026, 3, 31)),
            (d(2026, 2, 1), d(2026, 2, 28))
        );
        assert_eq!(
            range(json!({"relative": "this_month"}), d(2026, 12, 31)),
            (d(2026, 12, 1), d(2026, 12, 31))
        );
    }

    #[test]
    fn leap_day() {
        let t = d(2028, 2, 29);
        assert_eq!(
            range(json!({"relative": "this_month"}), t),
            (d(2028, 2, 1), d(2028, 2, 29))
        );
        assert_eq!(
            range(json!({"relative": "last_month"}), d(2028, 3, 15)),
            (d(2028, 2, 1), d(2028, 2, 29))
        );
        assert_eq!(
            range(json!({"relative": "yesterday"}), d(2028, 3, 1)),
            (t, t)
        );
        assert_eq!(
            range(json!({"relative": "last_n_days", "n": 366}), t),
            (d(2027, 3, 1), t)
        );
        assert_eq!(
            range(json!({"relative": "next_n_days", "n": 2}), d(2028, 2, 28)),
            (d(2028, 2, 28), t)
        );
        assert_eq!(
            range(json!({"relative": "last_year"}), d(2029, 2, 28)),
            (d(2028, 1, 1), d(2028, 12, 31))
        );
    }

    #[test]
    fn bad_tokens_are_rejected() {
        let bad = |v: serde_json::Value| RelativeDate::parse(&v).unwrap_err();
        assert!(bad(json!({"relative": "tomorrow"})).contains("not a relative date"));
        assert!(bad(json!({"relative": "last_n_days"})).contains("last_n_days"));
        assert!(bad(json!({"relative": "last_n_days", "n": 0})).contains("between 1 and 3660"));
        assert!(bad(json!({"relative": "last_n_days", "n": 3661})).contains("between 1 and 3660"));
        assert!(bad(json!({"relative": "last_n_days", "n": -3})).contains("last_n_days"));
        assert!(bad(json!({"relative": "last_n_days", "n": "5"})).contains("last_n_days"));
        assert!(bad(json!({"relative": "today", "n": 5})).contains("no other keys"));
        assert!(bad(json!({"relative": "last_n_days", "n": 5, "x": 1})).contains("last_n_days"));
        assert!(bad(json!({"relative": 5})).contains("looks like"));
        assert!(bad(json!({"n": 5})).contains("looks like"));
        let inj = bad(json!({"relative": "today'; DROP TABLE sales.quotations;--"}));
        assert!(inj.contains("not a relative date"));
        assert!(RelativeDate::parse(&json!({"relative": "last_n_days", "n": 3660})).is_ok());
    }

    #[test]
    fn words_follow_the_language() {
        let r = RelativeDate::LastNDays { n: 30 };
        assert_eq!(r.words("en"), "the last 30 days");
        assert_eq!(r.words("nl"), "de laatste 30 dagen");
        assert_eq!(RelativeDate::ThisQuarter.words("xx"), "this quarter");
    }
}
