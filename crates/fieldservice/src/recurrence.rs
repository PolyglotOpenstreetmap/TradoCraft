//! When a contract visit is due. Pure date arithmetic (no time zones, so no daylight saving
//! trouble): a visit is due on a calendar day.

use chrono::{Datelike, Months, NaiveDate};

/// How a visit template repeats.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Recurrence {
    /// Every `n` weeks from the anchor date.
    Weeks(u32),
    /// Every `n` months from the anchor date. A day that does not exist in a month moves to the
    /// last day of that month (31 January + 1 month = 28 February), and the following dates
    /// are counted from the anchor, not from the clamped date (31 March, not 28 March).
    Months(u32),
    /// In the listed months (1-12) of every year, on the anchor's day of the month (clamped).
    MonthList(Vec<u32>),
}

/// At most this many dates are returned for one call; a safety net for absurd ranges.
const MAX_DATES: usize = 1000;

fn last_day_of_month(year: i32, month: u32) -> u32 {
    let first = NaiveDate::from_ymd_opt(year, month, 1).expect("valid month");
    let next = first + Months::new(1);
    (next - chrono::Duration::days(1)).day()
}

/// All due dates `d` with `max(from, anchor) <= d <= to`, in order.
pub fn due_dates(
    rec: &Recurrence,
    anchor: NaiveDate,
    from: NaiveDate,
    to: NaiveDate,
) -> Vec<NaiveDate> {
    let mut out = Vec::new();
    if to < anchor || to < from {
        return out;
    }
    let from = from.max(anchor);
    match rec {
        Recurrence::Weeks(n) => {
            let step = 7 * i64::from((*n).max(1));
            let mut d = anchor;
            // Skip ahead to the first date in range.
            if from > anchor {
                let skip = (from - anchor).num_days() / step;
                d = anchor + chrono::Duration::days(skip * step);
            }
            while d <= to && out.len() < MAX_DATES {
                if d >= from {
                    out.push(d);
                }
                d += chrono::Duration::days(step);
            }
        }
        Recurrence::Months(n) => {
            let n = (*n).max(1);
            let mut k = 0u32;
            while out.len() < MAX_DATES {
                let Some(d) = anchor.checked_add_months(Months::new(k * n)) else {
                    break;
                };
                if d > to {
                    break;
                }
                if d >= from {
                    out.push(d);
                }
                k += 1;
            }
        }
        Recurrence::MonthList(months) => {
            let mut months: Vec<u32> = months
                .iter()
                .copied()
                .filter(|m| (1..=12).contains(m))
                .collect();
            months.sort_unstable();
            months.dedup();
            if months.is_empty() {
                return out;
            }
            for year in from.year()..=to.year() {
                for &m in &months {
                    let day = anchor.day().min(last_day_of_month(year, m));
                    let Some(d) = NaiveDate::from_ymd_opt(year, m, day) else {
                        continue;
                    };
                    if d >= from && d <= to && d >= anchor && out.len() < MAX_DATES {
                        out.push(d);
                    }
                }
            }
        }
    }
    out
}

/// The first due date after `after` (exclusive), looking at most ten years ahead.
pub fn next_due(rec: &Recurrence, anchor: NaiveDate, after: NaiveDate) -> Option<NaiveDate> {
    let from = after.succ_opt()?;
    let to = from.checked_add_months(Months::new(120))?;
    due_dates(rec, anchor, from, to).into_iter().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    #[test]
    fn every_n_weeks_counts_from_the_anchor() {
        let r = Recurrence::Weeks(2);
        let v = due_dates(&r, d(2026, 11, 2), d(2026, 11, 20), d(2027, 1, 10));
        assert_eq!(v, vec![d(2026, 11, 30), d(2026, 12, 14), d(2026, 12, 28)]);
        // Nothing before the anchor.
        let v = due_dates(&r, d(2026, 11, 2), d(2026, 1, 1), d(2026, 11, 3));
        assert_eq!(v, vec![d(2026, 11, 2)]);
        assert!(due_dates(&r, d(2026, 11, 2), d(2026, 1, 1), d(2026, 10, 31)).is_empty());
    }

    #[test]
    fn month_steps_clamp_to_the_end_of_the_month_but_do_not_drift() {
        let r = Recurrence::Months(1);
        let v = due_dates(&r, d(2027, 1, 31), d(2027, 1, 1), d(2027, 5, 31));
        assert_eq!(
            v,
            vec![
                d(2027, 1, 31),
                d(2027, 2, 28),
                d(2027, 3, 31),
                d(2027, 4, 30),
                d(2027, 5, 31)
            ]
        );
        // A leap year keeps 29 February.
        let v = due_dates(&r, d(2028, 1, 31), d(2028, 2, 1), d(2028, 2, 29));
        assert_eq!(v, vec![d(2028, 2, 29)]);
        // Quarterly and yearly.
        let q = due_dates(
            &Recurrence::Months(3),
            d(2026, 11, 30),
            d(2026, 11, 1),
            d(2027, 12, 31),
        );
        assert_eq!(
            q,
            vec![
                d(2026, 11, 30),
                d(2027, 2, 28),
                d(2027, 5, 30),
                d(2027, 8, 30),
                d(2027, 11, 30)
            ]
        );
        let y = due_dates(
            &Recurrence::Months(12),
            d(2028, 2, 29),
            d(2028, 1, 1),
            d(2030, 12, 31),
        );
        assert_eq!(y, vec![d(2028, 2, 29), d(2029, 2, 28), d(2030, 2, 28)]);
    }

    #[test]
    fn a_month_list_repeats_every_year_on_the_anchor_day() {
        let r = Recurrence::MonthList(vec![9, 3, 3, 13, 0]);
        let v = due_dates(&r, d(2026, 3, 31), d(2026, 1, 1), d(2027, 12, 31));
        assert_eq!(
            v,
            vec![
                d(2026, 3, 31),
                d(2026, 9, 30),
                d(2027, 3, 31),
                d(2027, 9, 30)
            ]
        );
        // Not before the anchor, and February clamps.
        let v = due_dates(
            &Recurrence::MonthList(vec![2]),
            d(2026, 3, 30),
            d(2026, 1, 1),
            d(2028, 12, 31),
        );
        assert_eq!(v, vec![d(2027, 2, 28), d(2028, 2, 29)]);
        assert!(due_dates(
            &Recurrence::MonthList(vec![]),
            d(2026, 3, 1),
            d(2026, 1, 1),
            d(2028, 1, 1)
        )
        .is_empty());
    }

    #[test]
    fn the_next_due_date_is_strictly_after() {
        let r = Recurrence::Months(1);
        assert_eq!(
            next_due(&r, d(2026, 11, 15), d(2026, 11, 15)),
            Some(d(2026, 12, 15))
        );
        assert_eq!(
            next_due(&r, d(2026, 11, 15), d(2026, 1, 1)),
            Some(d(2026, 11, 15))
        );
    }
}
