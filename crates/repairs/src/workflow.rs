//! The life of a repair, as pure rules.
//!
//! ```text
//! announced → received → routed ⇄ in_progress ⇄ waiting_customer / waiting_parts
//!                          ↑            ↓
//!                  (any department)  completed → shipped
//! ```
//! A customer may announce a repair before the parcel arrives (`announced`). Once received it is
//! *routed* to a department (electronics, electric, mechanical, ...) by someone who triages; the
//! department works it and may hand it on to another department by routing it again.

pub const STATUSES: [&str; 9] = [
    "announced",
    "received",
    "routed",
    "in_progress",
    "waiting_customer",
    "waiting_parts",
    "completed",
    "shipped",
    "cancelled",
];

/// States in which the repair sits in a department (and so has one).
pub const IN_DEPARTMENT: [&str; 4] = ["routed", "in_progress", "waiting_customer", "waiting_parts"];

/// States from which a repair may be (re)routed: it has arrived and is not finished.
pub fn can_route_from(status: &str) -> bool {
    status == "received" || IN_DEPARTMENT.contains(&status)
}

/// Allowed changes of status other than routing (which has its own action, because it needs a
/// department).
pub fn can_change(from: &str, to: &str) -> bool {
    matches!(
        (from, to),
        ("announced", "received")
            | ("announced", "cancelled")
            | ("received", "cancelled")
            // sent back to triage, e.g. routed to the wrong department
            | ("routed", "received")
            | ("routed", "in_progress")
            | ("routed", "cancelled")
            | ("in_progress", "waiting_customer")
            | ("in_progress", "waiting_parts")
            | ("in_progress", "completed")
            | ("in_progress", "cancelled")
            | ("waiting_customer", "in_progress")
            | ("waiting_customer", "cancelled")
            | ("waiting_parts", "in_progress")
            | ("waiting_parts", "cancelled")
            | ("completed", "shipped")
            // rework: the customer or the test says it is not fixed
            | ("completed", "in_progress")
    )
}

/// What the customer sees in the portal. Internal steps are not shown.
pub fn portal_status(status: &str) -> &'static str {
    match status {
        "announced" => "announced",
        "received" | "routed" | "in_progress" | "waiting_parts" => "in_repair",
        "waiting_customer" => "waiting_customer",
        "completed" => "ready",
        "shipped" => "shipped",
        _ => "cancelled",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_normal_path_is_allowed() {
        for (a, b) in [
            ("announced", "received"),
            ("routed", "in_progress"),
            ("in_progress", "completed"),
            ("completed", "shipped"),
        ] {
            assert!(can_change(a, b), "{a} → {b}");
        }
    }

    #[test]
    fn routing_is_not_a_status_change_and_work_cannot_start_unrouted() {
        assert!(
            !can_change("received", "routed"),
            "routing is its own action"
        );
        assert!(!can_change("received", "in_progress"));
        assert!(!can_change("announced", "in_progress"));
        assert!(!can_change("announced", "routed"));
    }

    #[test]
    fn finished_repairs_stay_finished() {
        for to in STATUSES {
            assert!(!can_change("shipped", to), "shipped → {to}");
            assert!(!can_change("cancelled", to), "cancelled → {to}");
        }
        assert!(
            !can_change("completed", "cancelled"),
            "ship or rework, not cancel"
        );
        assert!(can_change("completed", "in_progress"));
    }

    #[test]
    fn nothing_changes_to_itself_or_unknown_states() {
        for s in STATUSES {
            assert!(!can_change(s, s), "{s}");
        }
        assert!(!can_change("in_progress", "nonsense"));
    }

    #[test]
    fn routing_needs_an_arrived_unfinished_repair() {
        assert!(!can_route_from("announced"));
        for s in [
            "received",
            "routed",
            "in_progress",
            "waiting_customer",
            "waiting_parts",
        ] {
            assert!(can_route_from(s), "{s}");
        }
        for s in ["completed", "shipped", "cancelled"] {
            assert!(!can_route_from(s), "{s}");
        }
    }

    #[test]
    fn the_customer_sees_a_simpler_story() {
        assert_eq!(portal_status("routed"), "in_repair");
        assert_eq!(portal_status("waiting_parts"), "in_repair");
        assert_eq!(portal_status("completed"), "ready");
        assert_eq!(portal_status("waiting_customer"), "waiting_customer");
        // Every state maps to something.
        for s in STATUSES {
            assert!(!portal_status(s).is_empty());
        }
    }
}
