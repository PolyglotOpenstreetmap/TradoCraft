//! The life of an incoming package, as pure rules (ADR 0030).
//!
//! ```text
//! registered ──► opened ──► linked   (kind = repair; at least one repair linked)
//!      │            │
//!      │            └──► closed     (nothing to repair: the note says why)
//!      └──────────────► closed      (refused at the dock, sent back unopened)
//! ```
//! Opening a parcel as `other` or `return_to_sender` closes it at once. `opened → linked` happens
//! when the first repair is linked; removing the last link moves it back to `opened`.

/// The parcel statuses.
pub const STATUSES: [&str; 4] = ["registered", "opened", "linked", "closed"];
/// What an opened parcel can be.
pub const KINDS: [&str; 3] = ["repair", "other", "return_to_sender"];
/// The condition the box arrived in.
pub const CONDITIONS: [&str; 4] = ["intact", "damaged", "opened", "wet"];

/// Allowed changes of status. `opened ⇄ linked` is driven by the links, see [`after_link_change`].
pub fn can_change(from: &str, to: &str) -> bool {
    matches!(
        (from, to),
        ("registered", "opened")
            | ("registered", "closed")
            | ("opened", "linked")
            | ("opened", "closed")
            // the last repair was unlinked again
            | ("linked", "opened")
    )
}

/// The status a parcel gets when it is opened and decided to be `kind`; `None` for an unknown kind.
pub fn status_after_open(kind: &str) -> Option<&'static str> {
    match kind {
        "repair" => Some("opened"),
        "other" | "return_to_sender" => Some("closed"),
        _ => None,
    }
}

/// A parcel takes repairs only once it is opened as a repair parcel.
pub fn can_take_repairs(status: &str, kind: Option<&str>) -> bool {
    matches!(status, "opened" | "linked") && kind == Some("repair")
}

/// The status a parcel should have after its links changed (`links` is the number after the
/// change), or `None` when it stays as it is.
pub fn after_link_change(status: &str, links: i64) -> Option<&'static str> {
    match (status, links > 0) {
        ("opened", true) => Some("linked"),
        ("linked", false) => Some("opened"),
        _ => None,
    }
}

/// The details of a parcel can be corrected until it is finished.
pub fn is_editable(status: &str) -> bool {
    matches!(status, "registered" | "opened")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_normal_paths_are_allowed() {
        for (a, b) in [
            ("registered", "opened"),
            ("registered", "closed"),
            ("opened", "linked"),
            ("opened", "closed"),
            ("linked", "opened"),
        ] {
            assert!(can_change(a, b), "{a} -> {b}");
        }
    }

    #[test]
    fn a_closed_or_skipped_parcel_cannot_move() {
        for s in STATUSES {
            assert!(!can_change("closed", s), "closed -> {s}");
            assert!(!can_change(s, s), "{s} -> {s}");
        }
        assert!(!can_change("registered", "linked"), "must be opened first");
        assert!(!can_change("linked", "closed"));
        assert!(!can_change("linked", "registered"));
        assert!(!can_change("opened", "registered"));
    }

    #[test]
    fn every_transition_uses_known_statuses() {
        for a in STATUSES {
            for b in STATUSES {
                if can_change(a, b) {
                    assert!(STATUSES.contains(&a) && STATUSES.contains(&b));
                }
            }
        }
    }

    #[test]
    fn opening_decides_the_next_status() {
        assert_eq!(status_after_open("repair"), Some("opened"));
        assert_eq!(status_after_open("other"), Some("closed"));
        assert_eq!(status_after_open("return_to_sender"), Some("closed"));
        assert_eq!(status_after_open("bogus"), None);
        for k in KINDS {
            assert!(status_after_open(k).is_some(), "{k}");
        }
    }

    #[test]
    fn only_an_opened_repair_parcel_takes_repairs() {
        assert!(can_take_repairs("opened", Some("repair")));
        assert!(can_take_repairs("linked", Some("repair")));
        assert!(!can_take_repairs("registered", None));
        assert!(!can_take_repairs("closed", Some("repair")));
        assert!(!can_take_repairs("opened", Some("other")));
    }

    #[test]
    fn the_first_link_links_and_the_last_unlink_reopens() {
        assert_eq!(after_link_change("opened", 1), Some("linked"));
        assert_eq!(after_link_change("linked", 2), None);
        assert_eq!(after_link_change("linked", 1), None);
        assert_eq!(after_link_change("linked", 0), Some("opened"));
        assert_eq!(after_link_change("opened", 0), None);
        assert_eq!(after_link_change("closed", 0), None);
        assert_eq!(after_link_change("registered", 1), None);
    }

    #[test]
    fn automatic_changes_are_allowed_changes() {
        for s in STATUSES {
            for n in [0, 1, 3] {
                if let Some(to) = after_link_change(s, n) {
                    assert!(can_change(s, to), "{s} -> {to}");
                }
            }
        }
    }

    #[test]
    fn details_can_be_corrected_until_the_parcel_is_done() {
        assert!(is_editable("registered") && is_editable("opened"));
        assert!(!is_editable("linked") && !is_editable("closed"));
    }
}
