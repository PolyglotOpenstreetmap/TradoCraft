//! Conflict detection for a job on a technician's calendar. Pure functions on local (naive) times:
//! the caller converts instants to the company's time zone first, so nothing here needs a clock,
//! a database or a time zone table. `ResourceCalendar` is deliberately generic: a work centre can
//! use the same shape later.

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

/// A block of working time on a weekday (1 = Monday ... 7 = Sunday).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoursSlot {
    pub weekday: u32,
    pub start: NaiveTime,
    pub end: NaiveTime,
}

/// A job already on the calendar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookedJob {
    pub id: Uuid,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

/// Everything known about one resource (a technician) that matters for placing a job.
#[derive(Debug, Clone, Default)]
pub struct ResourceCalendar {
    pub id: Uuid,
    /// Effective working hours (own rows, else the company default). Empty = not defined.
    pub hours: Vec<HoursSlot>,
    /// Inclusive date ranges.
    pub absences: Vec<(NaiveDate, NaiveDate)>,
    pub skills: Vec<Uuid>,
    pub booked: Vec<BookedJob>,
}

/// The job to place.
#[derive(Debug, Clone)]
pub struct Candidate<'a> {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub required_skills: &'a [Uuid],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConflictKind {
    /// Overlaps another job of the technician, travel buffer included.
    DoubleBooked,
    /// Partly or wholly outside the technician's working hours.
    OutsideHours,
    /// The technician is absent on one of the days.
    OnAbsence,
    /// The technician lacks a skill the job requires.
    MissingSkill,
    /// The job runs over midnight.
    CrossesDays,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct Conflict {
    pub kind: ConflictKind,
    pub severity: Severity,
    /// The technician concerned; empty for conflicts of the job itself.
    pub technician_id: Option<Uuid>,
    /// Translation key for the message, e.g. `planning.conflict.double_booked`.
    pub message_key: String,
    /// The other job, for a double booking.
    pub other_job_id: Option<Uuid>,
    /// The missing skill, for a missing-skill conflict.
    pub skill_id: Option<Uuid>,
}

impl Conflict {
    fn new(kind: ConflictKind, severity: Severity, tech: Option<Uuid>) -> Self {
        let key = match kind {
            ConflictKind::DoubleBooked => "double_booked",
            ConflictKind::OutsideHours => "outside_hours",
            ConflictKind::OnAbsence => "on_absence",
            ConflictKind::MissingSkill => "missing_skill",
            ConflictKind::CrossesDays => "crosses_days",
        };
        Conflict {
            kind,
            severity,
            technician_id: tech,
            message_key: format!("planning.conflict.{key}"),
            other_job_id: None,
            skill_id: None,
        }
    }
}

/// The last calendar day a job touches (a job ending exactly at midnight stays on the day before).
fn last_day(start: NaiveDateTime, end: NaiveDateTime) -> NaiveDate {
    if end > start && end.time() == NaiveTime::MIN {
        end.date() - Duration::days(1)
    } else {
        end.date()
    }
}

fn minutes_of(t: NaiveTime) -> i64 {
    i64::from(t.hour() * 60 + t.minute())
}

/// True when the working hours of every day the job touches cover the part of the job on that day.
/// Resources without any working hours are never "outside".
fn within_hours(hours: &[HoursSlot], start: NaiveDateTime, end: NaiveDateTime) -> bool {
    if hours.is_empty() {
        return true;
    }
    let mut day = start.date();
    let last = last_day(start, end);
    while day <= last {
        let seg_start = if day == start.date() {
            minutes_of(start.time())
        } else {
            0
        };
        let seg_end = if day == end.date() {
            minutes_of(end.time())
        } else {
            24 * 60
        };
        let mut slots: Vec<(i64, i64)> = hours
            .iter()
            .filter(|h| h.weekday == day.weekday().number_from_monday())
            .map(|h| (minutes_of(h.start), minutes_of(h.end)))
            .collect();
        slots.sort();
        let mut cursor = seg_start;
        for (s, e) in slots {
            if s <= cursor {
                cursor = cursor.max(e);
            }
        }
        if cursor < seg_end {
            return false;
        }
        day += Duration::days(1);
    }
    true
}

/// The travel buffer only applies between jobs that share a calendar day.
fn buffer_between(
    a: (NaiveDateTime, NaiveDateTime),
    b: (NaiveDateTime, NaiveDateTime),
    buffer: i64,
) -> Duration {
    let days = |r: (NaiveDateTime, NaiveDateTime)| (r.0.date(), last_day(r.0, r.1));
    let (a0, a1) = days(a);
    let (b0, b1) = days(b);
    if a0 <= b1 && b0 <= a1 {
        Duration::minutes(buffer)
    } else {
        Duration::zero()
    }
}

/// All conflicts of placing `job` on each of `resources`, in a stable order: the job's own
/// conflicts first, then per resource (in the given order) double booking, absence, hours, skills.
pub fn detect(
    job: &Candidate,
    buffer_minutes: i64,
    resources: &[ResourceCalendar],
) -> Vec<Conflict> {
    let mut out = Vec::new();
    let first = job.start.date();
    let last = last_day(job.start, job.end);
    if last > first {
        out.push(Conflict::new(
            ConflictKind::CrossesDays,
            Severity::Warning,
            None,
        ));
    }
    for r in resources {
        let mut booked: Vec<&BookedJob> = r.booked.iter().collect();
        booked.sort_by_key(|b| (b.start, b.id));
        for b in booked {
            let gap = buffer_between((job.start, job.end), (b.start, b.end), buffer_minutes);
            if b.start < job.end + gap && job.start < b.end + gap {
                let mut c = Conflict::new(ConflictKind::DoubleBooked, Severity::Error, Some(r.id));
                c.other_job_id = Some(b.id);
                out.push(c);
            }
        }
        if r.absences.iter().any(|(f, t)| *f <= last && *t >= first) {
            out.push(Conflict::new(
                ConflictKind::OnAbsence,
                Severity::Error,
                Some(r.id),
            ));
        }
        if !within_hours(&r.hours, job.start, job.end) {
            out.push(Conflict::new(
                ConflictKind::OutsideHours,
                Severity::Warning,
                Some(r.id),
            ));
        }
        for s in job.required_skills {
            if !r.skills.contains(s) {
                let mut c =
                    Conflict::new(ConflictKind::MissingSkill, Severity::Warning, Some(r.id));
                c.skill_id = Some(*s);
                out.push(c);
            }
        }
    }
    out
}

pub fn has_errors(conflicts: &[Conflict]) -> bool {
    conflicts.iter().any(|c| c.severity == Severity::Error)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub fn dt(d: u32, h: u32, m: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2026, 11, d)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap()
    }
    pub fn t(h: u32, m: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(h, m, 0).unwrap()
    }
    pub fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }
    /// Monday to Friday 08:00-12:00 and 13:00-17:00.
    pub fn office_hours() -> Vec<HoursSlot> {
        (1..=5)
            .flat_map(|w| {
                [
                    HoursSlot {
                        weekday: w,
                        start: t(8, 0),
                        end: t(12, 0),
                    },
                    HoursSlot {
                        weekday: w,
                        start: t(13, 0),
                        end: t(17, 0),
                    },
                ]
            })
            .collect()
    }
    pub fn tech() -> ResourceCalendar {
        ResourceCalendar {
            id: id(1),
            hours: office_hours(),
            ..Default::default()
        }
    }
    fn job(start: NaiveDateTime, end: NaiveDateTime) -> Candidate<'static> {
        Candidate {
            start,
            end,
            required_skills: &[],
        }
    }
    fn kinds(c: &[Conflict]) -> Vec<ConflictKind> {
        c.iter().map(|c| c.kind).collect()
    }

    // 2026-11-09 is a Monday.
    #[test]
    fn a_free_job_inside_hours_has_no_conflicts() {
        assert!(detect(&job(dt(9, 9, 0), dt(9, 11, 0)), 15, &[tech()]).is_empty());
    }

    #[test]
    fn overlap_is_a_double_booking_with_the_other_job() {
        let mut r = tech();
        r.booked.push(BookedJob {
            id: id(9),
            start: dt(9, 10, 0),
            end: dt(9, 11, 30),
        });
        let c = detect(&job(dt(9, 9, 0), dt(9, 10, 30)), 0, &[r]);
        assert_eq!(kinds(&c), vec![ConflictKind::DoubleBooked]);
        assert_eq!(c[0].severity, Severity::Error);
        assert_eq!(c[0].other_job_id, Some(id(9)));
        assert_eq!(c[0].technician_id, Some(id(1)));
        assert_eq!(c[0].message_key, "planning.conflict.double_booked");
    }

    #[test]
    fn the_travel_buffer_counts_between_jobs_of_the_same_day() {
        let mut r = tech();
        r.booked.push(BookedJob {
            id: id(9),
            start: dt(9, 9, 0),
            end: dt(9, 10, 0),
        });
        // Starts 10 minutes after the other ends: fine without a buffer, a conflict with 15.
        let j = job(dt(9, 10, 10), dt(9, 11, 0));
        assert!(detect(&j, 0, std::slice::from_ref(&r)).is_empty());
        assert_eq!(
            kinds(&detect(&j, 15, &[r.clone()])),
            vec![ConflictKind::DoubleBooked]
        );
        // Exactly the buffer apart is fine, and so is the other side.
        assert!(detect(
            &job(dt(9, 10, 15), dt(9, 11, 0)),
            15,
            std::slice::from_ref(&r)
        )
        .is_empty());
        assert!(kinds(&detect(&job(dt(9, 8, 0), dt(9, 8, 50)), 15, &[r]))
            .contains(&ConflictKind::DoubleBooked));
    }

    #[test]
    fn the_buffer_does_not_reach_across_days() {
        let mut r = tech();
        r.booked.push(BookedJob {
            id: id(9),
            start: dt(9, 16, 0),
            end: dt(9, 17, 0),
        });
        let j = job(dt(10, 8, 0), dt(10, 9, 0));
        assert!(detect(&j, 120, &[r]).is_empty());
    }

    #[test]
    fn touching_jobs_do_not_overlap_without_a_buffer() {
        let mut r = tech();
        r.booked.push(BookedJob {
            id: id(9),
            start: dt(9, 9, 0),
            end: dt(9, 10, 0),
        });
        assert!(detect(&job(dt(9, 10, 0), dt(9, 11, 0)), 0, &[r]).is_empty());
    }

    #[test]
    fn outside_working_hours_is_a_warning() {
        let c = detect(&job(dt(9, 16, 0), dt(9, 18, 0)), 15, &[tech()]);
        assert_eq!(kinds(&c), vec![ConflictKind::OutsideHours]);
        assert_eq!(c[0].severity, Severity::Warning);
        // The lunch break is outside hours as well.
        assert_eq!(
            kinds(&detect(&job(dt(9, 11, 0), dt(9, 14, 0)), 15, &[tech()])),
            vec![ConflictKind::OutsideHours]
        );
        // A weekend day has no hours.
        assert_eq!(
            kinds(&detect(&job(dt(14, 9, 0), dt(14, 10, 0)), 15, &[tech()])),
            vec![ConflictKind::OutsideHours]
        );
    }

    #[test]
    fn adjacent_blocks_cover_a_job_and_no_hours_means_no_check() {
        let mut r = tech();
        r.hours = vec![
            HoursSlot {
                weekday: 1,
                start: t(8, 0),
                end: t(12, 0),
            },
            HoursSlot {
                weekday: 1,
                start: t(12, 0),
                end: t(16, 0),
            },
        ];
        assert!(detect(&job(dt(9, 10, 0), dt(9, 15, 0)), 0, &[r]).is_empty());
        let none = ResourceCalendar {
            id: id(1),
            ..Default::default()
        };
        assert!(detect(&job(dt(14, 3, 0), dt(14, 4, 0)), 0, &[none]).is_empty());
    }

    #[test]
    fn an_absence_is_an_error_on_any_day_it_touches() {
        let mut r = tech();
        r.absences.push((dt(10, 0, 0).date(), dt(12, 0, 0).date()));
        let c = detect(&job(dt(10, 9, 0), dt(10, 10, 0)), 15, &[r.clone()]);
        assert_eq!(kinds(&c), vec![ConflictKind::OnAbsence]);
        assert_eq!(c[0].severity, Severity::Error);
        assert!(detect(&job(dt(9, 9, 0), dt(9, 10, 0)), 15, &[r.clone()]).is_empty());
        assert!(detect(&job(dt(13, 9, 0), dt(13, 10, 0)), 15, &[r]).is_empty());
    }

    #[test]
    fn a_missing_skill_is_a_warning_per_skill() {
        let mut r = tech();
        r.skills = vec![id(100)];
        let need = [id(100), id(200)];
        let j = Candidate {
            start: dt(9, 9, 0),
            end: dt(9, 10, 0),
            required_skills: &need,
        };
        let c = detect(&j, 15, &[r]);
        assert_eq!(kinds(&c), vec![ConflictKind::MissingSkill]);
        assert_eq!(c[0].skill_id, Some(id(200)));
        assert_eq!(c[0].severity, Severity::Warning);
    }

    #[test]
    fn crossing_midnight_is_flagged_once_for_the_job() {
        let two = [
            tech(),
            ResourceCalendar {
                id: id(2),
                hours: office_hours(),
                ..Default::default()
            },
        ];
        let c: Vec<_> = detect(&job(dt(9, 16, 0), dt(10, 9, 0)), 15, &two)
            .into_iter()
            .filter(|c| c.kind == ConflictKind::CrossesDays)
            .collect();
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].technician_id, None);
        // Ending exactly at midnight stays on one day.
        assert!(
            !kinds(&detect(&job(dt(9, 20, 0), dt(10, 0, 0)), 15, &[tech()]))
                .contains(&ConflictKind::CrossesDays)
        );
    }

    #[test]
    fn each_technician_gets_their_own_conflicts() {
        let mut busy = tech();
        busy.booked.push(BookedJob {
            id: id(9),
            start: dt(9, 9, 0),
            end: dt(9, 10, 0),
        });
        let free = ResourceCalendar {
            id: id(2),
            hours: office_hours(),
            ..Default::default()
        };
        let c = detect(&job(dt(9, 9, 30), dt(9, 10, 30)), 15, &[busy, free]);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].technician_id, Some(id(1)));
        assert!(has_errors(&c));
        assert!(!has_errors(&[]));
    }
}
