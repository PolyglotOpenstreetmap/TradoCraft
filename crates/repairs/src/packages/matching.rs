//! Suggesting which announced repair a parcel belongs to (ADR 0030, section 4). Pure scoring: the
//! suggestions are never applied automatically, a person confirms.

use chrono::{DateTime, Duration, Utc};

/// What we know about the parcel when someone looks for its repair.
#[derive(Debug, Clone, Default)]
pub struct Wanted<'a> {
    pub customer_id: Option<uuid::Uuid>,
    pub customer_reference: &'a str,
    /// Typed or scanned from the product label.
    pub serial: &'a str,
    /// Ticket number typed by the person at the desk.
    pub ticket_number: &'a str,
    pub product: &'a str,
}

/// The announced repair under consideration.
#[derive(Debug, Clone)]
pub struct Candidate<'a> {
    pub customer_id: uuid::Uuid,
    pub customer_reference: &'a str,
    pub serial_number: &'a str,
    pub ticket_number: Option<&'a str>,
    pub product_description: &'a str,
    pub product_code: Option<&'a str>,
    pub announced_at: DateTime<Utc>,
}

/// Points for the same serial number.
pub const SERIAL: i32 = 5;
/// Points for the same customer.
pub const CUSTOMER: i32 = 3;
/// Points for the same customer reference.
pub const REFERENCE: i32 = 3;
/// Points for the same ticket number.
pub const TICKET: i32 = 3;
/// Points for a similar product description.
pub const PRODUCT: i32 = 1;
/// Points for a recent announcement.
pub const RECENT: i32 = 1;
/// An announcement older than this is no longer "recent".
pub const RECENT_DAYS: i64 = 60;
/// Most suggestions returned.
pub const MAX_SUGGESTIONS: usize = 10;

/// Equal, ignoring case and surrounding spaces; empty never matches.
fn same(a: &str, b: &str) -> bool {
    let (a, b) = (a.trim(), b.trim());
    !a.is_empty() && a.eq_ignore_ascii_case(b)
}

/// One text contains the other (both at least three characters).
fn similar(a: &str, b: &str) -> bool {
    let (a, b) = (a.trim().to_lowercase(), b.trim().to_lowercase());
    a.chars().count() >= 3 && b.chars().count() >= 3 && (a.contains(&b) || b.contains(&a))
}

/// The score and the reasons (`serial`, `customer`, `reference`, `ticket`, `product`, `recent`).
/// A candidate is only worth showing when `is_match` says so.
pub fn score(w: &Wanted, c: &Candidate, now: DateTime<Utc>) -> (i32, Vec<&'static str>) {
    let mut total = 0;
    let mut reasons = Vec::new();
    let mut add = |ok: bool, points: i32, reason: &'static str| {
        if ok {
            total += points;
            reasons.push(reason);
        }
    };
    add(same(w.serial, c.serial_number), SERIAL, "serial");
    add(w.customer_id == Some(c.customer_id), CUSTOMER, "customer");
    add(
        same(w.customer_reference, c.customer_reference),
        REFERENCE,
        "reference",
    );
    // The ticket number may be typed, or written on the slip as the customer's reference.
    let ticket = c.ticket_number.unwrap_or("");
    add(
        same(w.ticket_number, ticket) || same(w.customer_reference, ticket),
        TICKET,
        "ticket",
    );
    add(
        similar(w.product, c.product_description)
            || c.product_code.is_some_and(|code| similar(w.product, code)),
        PRODUCT,
        "product",
    );
    add(
        now - c.announced_at <= Duration::days(RECENT_DAYS),
        RECENT,
        "recent",
    );
    (total, reasons)
}

/// Product similarity and recency alone prove nothing: a suggestion needs a real link with the parcel.
pub fn is_match(reasons: &[&str]) -> bool {
    reasons
        .iter()
        .any(|r| matches!(*r, "serial" | "customer" | "reference" | "ticket"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn cand<'a>(customer: Uuid) -> Candidate<'a> {
        Candidate {
            customer_id: customer,
            customer_reference: "",
            serial_number: "",
            ticket_number: None,
            product_description: "",
            product_code: None,
            announced_at: Utc::now() - Duration::days(200),
        }
    }

    #[test]
    fn nothing_known_scores_nothing() {
        let (s, r) = score(&Wanted::default(), &cand(Uuid::now_v7()), Utc::now());
        assert_eq!(s, 0);
        assert!(r.is_empty() && !is_match(&r));
    }

    #[test]
    fn serial_matches_ignoring_case_and_spaces() {
        let w = Wanted {
            serial: " sn-1 ",
            ..Default::default()
        };
        let mut c = cand(Uuid::now_v7());
        c.serial_number = "SN-1";
        let (s, r) = score(&w, &c, Utc::now());
        assert_eq!((s, r.clone()), (SERIAL, vec!["serial"]));
        assert!(is_match(&r));
        // An empty serial never matches an empty serial.
        let (s, _) = score(&Wanted::default(), &cand(Uuid::now_v7()), Utc::now());
        assert_eq!(s, 0);
    }

    #[test]
    fn the_scores_add_up_with_their_reasons() {
        let customer = Uuid::now_v7();
        let w = Wanted {
            customer_id: Some(customer),
            customer_reference: "RMA-77",
            serial: "A1",
            ticket_number: "T2027-00004",
            product: "Pump 5 kW",
        };
        let c = Candidate {
            customer_id: customer,
            customer_reference: "rma-77",
            serial_number: "a1",
            ticket_number: Some("T2027-00004"),
            product_description: "Pump 5 kW, blue",
            product_code: None,
            announced_at: Utc::now() - Duration::days(3),
        };
        let (s, r) = score(&w, &c, Utc::now());
        assert_eq!(s, SERIAL + CUSTOMER + REFERENCE + TICKET + PRODUCT + RECENT);
        assert_eq!(
            r,
            [
                "serial",
                "customer",
                "reference",
                "ticket",
                "product",
                "recent"
            ]
        );
    }

    #[test]
    fn the_customers_reference_may_be_the_ticket_number() {
        let w = Wanted {
            customer_reference: "T2027-00009",
            ..Default::default()
        };
        let mut c = cand(Uuid::now_v7());
        c.ticket_number = Some("T2027-00009");
        let (_, r) = score(&w, &c, Utc::now());
        assert!(r.contains(&"ticket"));
    }

    #[test]
    fn recency_stops_after_sixty_days() {
        let now = Utc::now();
        let mut c = cand(Uuid::now_v7());
        c.announced_at = now - Duration::days(RECENT_DAYS);
        assert_eq!(score(&Wanted::default(), &c, now).0, RECENT);
        c.announced_at = now - Duration::days(RECENT_DAYS + 1);
        assert_eq!(score(&Wanted::default(), &c, now).0, 0);
    }

    #[test]
    fn product_and_recency_alone_are_not_a_match() {
        let w = Wanted {
            product: "pump",
            ..Default::default()
        };
        let mut c = cand(Uuid::now_v7());
        c.product_description = "Pump 5 kW";
        c.announced_at = Utc::now();
        let (s, r) = score(&w, &c, Utc::now());
        assert_eq!(s, PRODUCT + RECENT);
        assert!(!is_match(&r));
    }

    #[test]
    fn a_short_product_text_is_not_similar_to_anything() {
        assert!(!similar("ab", "abc"));
        assert!(similar("Pump", "pump 5 kW"));
    }
}
