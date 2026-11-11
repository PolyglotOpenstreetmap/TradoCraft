//! Ranked slot suggestions for a job. A simple, deterministic heuristic on local (naive) times:
//! for every technician and day, take the free gaps inside the working hours (other jobs padded
//! with the travel buffer, absences skipped) and offer the earliest 15-minute-aligned start in each
//! gap that is long enough. Ranking: fewest missing skills, earliest day, tightest fit (least
//! unused time in the gap, so the least disruption of the day), earliest start, technician id.

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::conflicts::{HoursSlot, ResourceCalendar};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    pub technician_id: Uuid,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub missing_skills: Vec<Uuid>,
    /// Minutes of the gap left over after the job.
    pub slack_minutes: i64,
}

/// A technician without any working hours is assumed to work Monday to Friday 08:00-17:00.
fn assumed_hours() -> Vec<HoursSlot> {
    (1..=5)
        .map(|weekday| HoursSlot {
            weekday,
            start: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            end: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
        })
        .collect()
}

/// Rounds up to the next quarter hour.
fn snap_up(t: NaiveDateTime) -> NaiveDateTime {
    let t = t.with_second(0).unwrap().with_nanosecond(0).unwrap();
    let rest = i64::from(t.minute() % 15);
    if rest == 0 && t.second() == 0 {
        t
    } else {
        t + Duration::minutes(15 - rest)
    }
}

/// Free gaps of one resource on one day.
fn gaps_on(
    r: &ResourceCalendar,
    hours: &[HoursSlot],
    day: NaiveDate,
    buffer: i64,
) -> Vec<(NaiveDateTime, NaiveDateTime)> {
    if r.absences.iter().any(|(f, t)| *f <= day && *t >= day) {
        return vec![];
    }
    let mut blocks: Vec<(NaiveDateTime, NaiveDateTime)> = hours
        .iter()
        .filter(|h| h.weekday == day.weekday().number_from_monday())
        .map(|h| (day.and_time(h.start), day.and_time(h.end)))
        .collect();
    blocks.sort();
    let pad = Duration::minutes(buffer);
    let mut busy: Vec<(NaiveDateTime, NaiveDateTime)> = r
        .booked
        .iter()
        .map(|b| (b.start - pad, b.end + pad))
        .collect();
    busy.sort();
    let mut gaps = Vec::new();
    for (a, b) in blocks {
        let mut cursor = a;
        for (s, e) in &busy {
            if *e <= cursor || *s >= b {
                continue;
            }
            if *s > cursor {
                gaps.push((cursor, *s));
            }
            cursor = cursor.max(*e);
        }
        if cursor < b {
            gaps.push((cursor, b));
        }
    }
    gaps
}

/// Up to `limit` suggestions for a job of `duration_minutes` on the days `from..=to`.
pub fn suggest(
    duration_minutes: i64,
    buffer_minutes: i64,
    required_skills: &[Uuid],
    from: NaiveDate,
    to: NaiveDate,
    resources: &[ResourceCalendar],
    limit: usize,
) -> Vec<Suggestion> {
    let need = Duration::minutes(duration_minutes.max(1));
    let mut out = Vec::new();
    for r in resources {
        let hours = if r.hours.is_empty() {
            assumed_hours()
        } else {
            r.hours.clone()
        };
        let missing: Vec<Uuid> = required_skills
            .iter()
            .filter(|s| !r.skills.contains(s))
            .copied()
            .collect();
        let mut day = from;
        while day <= to {
            for (a, b) in gaps_on(r, &hours, day, buffer_minutes) {
                let start = snap_up(a);
                if start + need <= b {
                    out.push(Suggestion {
                        technician_id: r.id,
                        start,
                        end: start + need,
                        missing_skills: missing.clone(),
                        slack_minutes: (b - a).num_minutes() - duration_minutes,
                    });
                }
            }
            day += Duration::days(1);
        }
    }
    out.sort_by_key(|s| {
        (
            s.missing_skills.len(),
            s.start.date(),
            s.slack_minutes,
            s.start,
            s.technician_id,
        )
    });
    out.truncate(limit);
    out
}

/// A suggestion as the API returns it.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SuggestionOut {
    pub technician_id: Uuid,
    pub technician_name: String,
    pub scheduled_start: chrono::DateTime<chrono::Utc>,
    pub scheduled_end: chrono::DateTime<chrono::Utc>,
    pub missing_skill_ids: Vec<Uuid>,
    pub slack_minutes: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conflicts::tests::{dt, id, tech};
    use crate::conflicts::BookedJob;

    fn day(d: u32) -> NaiveDate {
        dt(d, 0, 0).date()
    }

    #[test]
    fn offers_the_start_of_the_day_for_a_free_technician() {
        let s = suggest(120, 15, &[], day(9), day(9), &[tech()], 5);
        assert_eq!(s[0].start, dt(9, 8, 0));
        assert_eq!(s[0].end, dt(9, 10, 0));
        // The afternoon block is offered too.
        assert!(s.iter().any(|x| x.start == dt(9, 13, 0)));
    }

    #[test]
    fn starts_after_an_existing_job_plus_the_buffer_on_the_quarter_hour() {
        let mut r = tech();
        r.booked.push(BookedJob {
            id: id(9),
            start: dt(9, 8, 0),
            end: dt(9, 9, 5),
        });
        let s = suggest(60, 20, &[], day(9), day(9), &[r], 1);
        // 09:05 + 20 = 09:25 -> snapped up to 09:30.
        assert_eq!(s[0].start, dt(9, 9, 30));
    }

    #[test]
    fn skips_absences_weekends_and_gaps_that_are_too_short() {
        let mut r = tech();
        r.absences.push((day(9), day(9)));
        // The morning of Tuesday is nearly full: only 30 free minutes between jobs.
        r.booked.push(BookedJob {
            id: id(8),
            start: dt(10, 8, 0),
            end: dt(10, 9, 0),
        });
        r.booked.push(BookedJob {
            id: id(9),
            start: dt(10, 9, 45),
            end: dt(10, 12, 0),
        });
        let s = suggest(60, 15, &[], day(9), day(15), &[r], 20);
        assert!(s
            .iter()
            .all(|x| x.start.date() != day(9) && x.start.date() < day(14)));
        assert!(s
            .iter()
            .all(|x| !(x.start >= dt(10, 8, 0) && x.start < dt(10, 12, 0))));
        assert!(s.iter().any(|x| x.start == dt(10, 13, 0)));
    }

    #[test]
    fn ranks_skills_first_then_day_then_tightest_fit() {
        let skill = id(100);
        let skilled = ResourceCalendar {
            id: id(2),
            skills: vec![skill],
            ..tech()
        };
        let unskilled = tech();
        // The unskilled technician is free earlier, but the skilled one ranks first anyway.
        let mut busy = skilled.clone();
        busy.booked.push(BookedJob {
            id: id(9),
            start: dt(9, 8, 0),
            end: dt(9, 12, 0),
        });
        let s = suggest(
            120,
            0,
            &[skill],
            day(9),
            day(10),
            &[unskilled.clone(), busy.clone()],
            10,
        );
        assert_eq!(s[0].technician_id, id(2));
        assert!(s[0].missing_skills.is_empty());
        assert_eq!(s[0].start, dt(9, 13, 0));
        let last = s.last().unwrap();
        assert_eq!(last.technician_id, id(1));
        assert_eq!(last.missing_skills, vec![skill]);

        // Same skills and day: the tighter gap wins over the roomier one.
        let mut a = ResourceCalendar {
            id: id(3),
            ..tech()
        };
        a.booked.push(BookedJob {
            id: id(7),
            start: dt(9, 10, 0),
            end: dt(9, 12, 0),
        });
        let b = ResourceCalendar {
            id: id(4),
            ..tech()
        };
        let s = suggest(120, 0, &[], day(9), day(9), &[b, a], 10);
        assert_eq!(s[0].technician_id, id(3));
        assert_eq!(s[0].slack_minutes, 0);
    }

    #[test]
    fn is_deterministic_and_limited() {
        let rs = [
            tech(),
            ResourceCalendar {
                id: id(2),
                ..tech()
            },
        ];
        let a = suggest(60, 15, &[], day(9), day(13), &rs, 3);
        let b = suggest(60, 15, &[], day(9), day(13), &rs, 3);
        assert_eq!(a, b);
        assert_eq!(a.len(), 3);
        // Ties between technicians are broken by id.
        assert_eq!(a[0].technician_id, id(1));
    }

    #[test]
    fn a_technician_without_hours_is_assumed_to_work_office_days() {
        let none = ResourceCalendar {
            id: id(5),
            ..Default::default()
        };
        let s = suggest(60, 0, &[], day(14), day(16), &[none], 1);
        assert_eq!(s[0].start, dt(16, 8, 0));
    }
}
