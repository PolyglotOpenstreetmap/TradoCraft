//! Guesses the language of a message from common words. Good enough to pre-select the ticket
//! language for an unknown sender; the agent can always change it.

const WORDS: [(&str, &[&str]); 5] = [
    (
        "nl",
        &[
            "de",
            "het",
            "een",
            "en",
            "van",
            "ik",
            "je",
            "wij",
            "we",
            "niet",
            "met",
            "voor",
            "is",
            "dat",
            "graag",
            "bedankt",
            "groeten",
            "offerte",
            "bestelling",
            "kunnen",
            "zijn",
            "op",
            "ook",
            "u",
            "uw",
        ],
    ),
    (
        "fr",
        &[
            "le",
            "la",
            "les",
            "et",
            "des",
            "est",
            "je",
            "nous",
            "vous",
            "pas",
            "pour",
            "avec",
            "une",
            "un",
            "que",
            "merci",
            "cordialement",
            "devis",
            "commande",
            "bonjour",
            "sur",
            "dans",
            "votre",
            "pouvez",
        ],
    ),
    (
        "en",
        &[
            "the", "and", "is", "are", "we", "you", "your", "not", "for", "with", "this", "that",
            "of", "to", "please", "thanks", "regards", "quote", "order", "hello", "can", "would",
            "our", "have",
        ],
    ),
    (
        "de",
        &[
            "der",
            "die",
            "das",
            "und",
            "ist",
            "ich",
            "wir",
            "sie",
            "nicht",
            "mit",
            "für",
            "ein",
            "eine",
            "zu",
            "bitte",
            "danke",
            "grüße",
            "angebot",
            "bestellung",
            "können",
            "auf",
            "auch",
            "ihr",
            "ihre",
        ],
    ),
    (
        "es",
        &[
            "el",
            "la",
            "los",
            "las",
            "y",
            "es",
            "yo",
            "nosotros",
            "usted",
            "no",
            "para",
            "con",
            "una",
            "un",
            "que",
            "gracias",
            "saludos",
            "presupuesto",
            "pedido",
            "hola",
            "por",
            "en",
            "su",
            "podemos",
        ],
    ),
];

/// Returns the most likely language code, or `None` when the text gives too little evidence.
pub fn detect_language(text: &str) -> Option<&'static str> {
    let words: Vec<String> = text
        .split(|c: char| !c.is_alphabetic())
        .filter(|w| !w.is_empty())
        .take(400)
        .map(|w| w.to_lowercase())
        .collect();
    let mut best: Option<(&'static str, usize)> = None;
    let mut second = 0;
    for (code, list) in WORDS {
        let score = words.iter().filter(|w| list.contains(&w.as_str())).count();
        match best {
            Some((_, s)) if score > s => {
                second = s;
                best = Some((code, score));
            }
            Some((_, s)) if score > second && score <= s => second = score,
            None => best = Some((code, score)),
            _ => {}
        }
    }
    let (code, score) = best?;
    // Require a few hits and a clear lead over the runner-up.
    (score >= 3 && score > second).then_some(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_the_five_languages() {
        let samples = [
            ("nl", "Goedemiddag, wij hebben de offerte goed ontvangen en willen graag een bestelling plaatsen. Met vriendelijke groeten"),
            ("fr", "Bonjour, nous avons bien reçu le devis et nous voudrions passer la commande. Merci et cordialement"),
            ("en", "Hello, we have received the quote and would like to place the order. Thanks and kind regards"),
            ("de", "Guten Tag, wir haben das Angebot erhalten und möchten die Bestellung aufgeben. Danke und viele Grüße"),
            ("es", "Hola, hemos recibido el presupuesto y queremos hacer el pedido. Gracias y saludos"),
        ];
        for (expected, text) in samples {
            assert_eq!(detect_language(text), Some(expected), "{text}");
        }
    }

    #[test]
    fn too_little_text_gives_no_guess() {
        assert_eq!(detect_language("OK"), None);
        assert_eq!(detect_language("Q2026-00012 PUMP-05"), None);
    }
}
