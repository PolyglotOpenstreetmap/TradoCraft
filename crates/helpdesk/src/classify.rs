//! Recognises references and intent in incoming e-mails. Only suggestions: the agent decides.

/// Ticket number tag used in outgoing subjects, e.g. `[Ticket#T2026-00012]`.
pub fn ticket_tag(number: &str) -> String {
    format!("[Ticket#{number}]")
}

/// Finds a ticket number tag in a subject.
pub fn find_ticket_number(subject: &str) -> Option<String> {
    let start = subject.find("[Ticket#")? + "[Ticket#".len();
    let rest = &subject[start..];
    let end = rest.find(']')?;
    let n = rest[..end].trim();
    is_document_number(n, 'T').then(|| n.to_string())
}

/// `T2026-00012`, `Q2026-00012`, ...: a letter, 4 digits, a dash and 5 digits.
fn is_document_number(s: &str, prefix: char) -> bool {
    let b = s.as_bytes();
    b.len() == 11
        && b[0] == prefix as u8
        && b[1..5].iter().all(u8::is_ascii_digit)
        && b[5] == b'-'
        && b[6..].iter().all(u8::is_ascii_digit)
}

/// Quotation numbers (`Q2026-00012`) mentioned in a text, in order of appearance, without duplicates.
pub fn find_quotation_numbers(text: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    for i in 0..chars.len() {
        if chars[i] != 'Q' || (i > 0 && chars[i - 1].is_alphanumeric()) {
            continue;
        }
        let candidate: String = chars[i..].iter().take(11).collect();
        let after = chars.get(i + 11);
        if is_document_number(&candidate, 'Q')
            && !after.is_some_and(|c| c.is_ascii_digit())
            && !found.contains(&candidate)
        {
            found.push(candidate);
        }
    }
    found
}

const ORDER_WORDS: [&str; 18] = [
    // nl
    "akkoord",
    "bestellen",
    "bestelling",
    "goedgekeurd",
    // fr
    "commande",
    "commander",
    "accord",
    "bon pour accord",
    // en
    "purchase order",
    "we accept",
    "approved",
    "go ahead",
    // de
    "auftrag",
    "bestellen",
    "einverstanden",
    // es
    "pedido",
    "aceptamos",
    "de acuerdo",
];

const CHANGE_WORDS: [&str; 10] = [
    "aanpassen",
    "wijzigen",
    "modifier",
    "changer",
    "change the",
    "adjust",
    "ändern",
    "anpassen",
    "modificar",
    "cambiar",
];

/// Suggests a ticket category for an incoming e-mail.
pub fn suggest_category(subject: &str, body: &str, mentions_quotation: bool) -> &'static str {
    let text = format!("{subject}\n{body}").to_lowercase();
    let has = |words: &[&str]| words.iter().any(|w| text.contains(w));
    match (mentions_quotation, has(&CHANGE_WORDS), has(&ORDER_WORDS)) {
        (true, true, _) => "quote_change",
        (true, false, true) => "order_confirmation",
        (true, false, false) => "question",
        (false, _, _)
            if has(&[
                "offerte",
                "devis",
                "quote",
                "quotation",
                "angebot",
                "presupuesto",
                "prijs",
                "prix",
                "price",
                "preis",
                "precio",
            ]) =>
        {
            "lead"
        }
        _ => "question",
    }
}

/// Removes the quoted history below a reply ("> ..." lines and "On ... wrote:" blocks), so the
/// ticket shows only what is new. The full original stays available in the raw message.
pub fn strip_quoted(body: &str) -> String {
    let markers = [
        "-----original message-----",
        "-----oorspronkelijk bericht-----",
        "-----message d'origine-----",
        "-----ursprüngliche nachricht-----",
        "-----mensaje original-----",
    ];
    let mut out = Vec::new();
    let lines: Vec<&str> = body.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let l = line.trim().to_lowercase();
        if markers.iter().any(|m| l.starts_with(m)) {
            break;
        }
        // "On Tue, 3 Nov 2026 Alice wrote:" / "Op di 3 nov. schreef Alice:" / "Le mar. 3 nov., Alice a écrit :" /
        // "Am Di., 3. Nov. schrieb Alice:" / "El mar, 3 nov, Alice escribió:" — followed by quoted lines.
        let verbs = [" wrote", " schreef", " a écrit", " schrieb", " escribió"];
        if l.ends_with(':')
            && verbs.iter().any(|v| l.contains(v))
            && lines[i + 1..].iter().any(|n| n.starts_with('>'))
        {
            break;
        }
        if line.starts_with('>') {
            continue;
        }
        out.push(*line);
    }
    out.join("\n").trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticket_tags_round_trip() {
        let subject = format!("Re: Your question {}", ticket_tag("T2026-00042"));
        assert_eq!(find_ticket_number(&subject), Some("T2026-00042".into()));
        assert_eq!(find_ticket_number("[Ticket#hello]"), None);
        assert_eq!(find_ticket_number("no tag"), None);
    }

    #[test]
    fn finds_quotation_numbers() {
        let text = "About Q2026-00012 rev 2 (and again Q2026-00012), not XQ2026-00099 or Q2026-000123, but Q2027-00001.";
        assert_eq!(
            find_quotation_numbers(text),
            vec!["Q2026-00012", "Q2027-00001"]
        );
    }

    #[test]
    fn suggests_categories_in_several_languages() {
        assert_eq!(
            suggest_category(
                "Re: offerte Q2026-00012",
                "Akkoord, u mag de bestelling plaatsen.",
                true
            ),
            "order_confirmation"
        );
        assert_eq!(
            suggest_category("Devis", "Bon pour accord, merci.", true),
            "order_confirmation"
        );
        assert_eq!(
            suggest_category("Angebot", "Können Sie die Menge ändern?", true),
            "quote_change"
        );
        assert_eq!(
            suggest_category(
                "Solicitud",
                "Necesitamos un presupuesto para 3 bombas",
                false
            ),
            "lead"
        );
        assert_eq!(
            suggest_category("Hello", "Where is my delivery?", false),
            "question"
        );
    }

    #[test]
    fn strips_quoted_history() {
        let body = "Thanks, we agree.\n\nOn Tue, 3 Nov 2026 Sales wrote:\n> Here is the quote\n> Kind regards";
        assert_eq!(strip_quoted(body), "Thanks, we agree.");
        let outlook = "Akkoord.\n\n-----Oorspronkelijk bericht-----\nVan: sales";
        assert_eq!(strip_quoted(outlook), "Akkoord.");
        assert_eq!(strip_quoted("Line 1\n> quoted\nLine 2"), "Line 1\nLine 2");
        let dutch =
            "Dank u!\n\nOp di 3 nov. 2026 schreef Support <sales@example.com>:\n> 6 tot 8 weken";
        assert_eq!(strip_quoted(dutch), "Dank u!");
        let german = "Danke.\n\nAm Di., 3. Nov. 2026 schrieb Sales:\n> Angebot";
        assert_eq!(strip_quoted(german), "Danke.");
        // A sentence ending in "wrote:" without quoted lines below is kept.
        assert_eq!(strip_quoted("As you wrote:\nyes"), "As you wrote:\nyes");
    }
}
