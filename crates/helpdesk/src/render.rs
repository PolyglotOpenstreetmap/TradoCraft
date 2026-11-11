//! `{{placeholder}}` substitution for prepared answers.

use std::collections::BTreeMap;

/// Replaces `{{key}}` with its value. Unknown or empty placeholders stay visible, so the agent
/// sees what still has to be filled in before sending.
pub fn render(text: &str, values: &BTreeMap<String, String>) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("{{") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        match after.find("}}") {
            Some(end) => {
                let key = after[..end].trim();
                match values.get(key).filter(|v| !v.is_empty()) {
                    Some(v) => out.push_str(v),
                    None => out.push_str(&rest[start..start + 2 + end + 2]),
                }
                rest = &after[end + 2..];
            }
            None => {
                out.push_str(&rest[start..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

/// Placeholders offered to authors of prepared answers.
pub const PLACEHOLDERS: [&str; 14] = [
    "customer.name",
    "contact.name",
    "contact.first_name",
    "contact.last_name",
    "contact.salutation",
    "ticket.number",
    "ticket.subject",
    "agent.name",
    "company.name",
    "quotation.number",
    "quotation.revision",
    "quotation.total",
    "order.number",
    "order.reference",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_known_and_keeps_unknown_placeholders() {
        let values = BTreeMap::from([
            ("contact.name".to_string(), "Anna".to_string()),
            ("quotation.number".to_string(), "Q2026-00012".to_string()),
            ("order.number".to_string(), String::new()),
        ]);
        assert_eq!(
            render("Dear {{ contact.name }}, about {{quotation.number}} / {{order.number}} / {{nope}} {{", &values),
            "Dear Anna, about Q2026-00012 / {{order.number}} / {{nope}} {{"
        );
    }
}
