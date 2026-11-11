//! The login e-mail, in the five languages.

/// Subject and body of the e-mail with the one-time login link.
pub fn login_mail(
    language: &str,
    greeting: &str,
    company: &str,
    link: &str,
    minutes: i64,
) -> (String, String) {
    let (subject, intro, valid, ignore, regards) = match language {
        "nl" => (
            format!("Aanmelden bij het klantenportaal van {company}"),
            "Via deze link meldt u zich aan bij ons klantenportaal:",
            format!("De link is {minutes} minuten geldig en kan één keer gebruikt worden."),
            "Hebt u dit niet aangevraagd? Dan kunt u deze e-mail negeren.",
            "Met vriendelijke groeten",
        ),
        "fr" => (
            format!("Connexion au portail client de {company}"),
            "Utilisez ce lien pour vous connecter à notre portail client :",
            format!(
                "Ce lien est valable {minutes} minutes et ne peut être utilisé qu'une seule fois."
            ),
            "Vous n'avez pas fait cette demande ? Vous pouvez ignorer cet e-mail.",
            "Cordialement",
        ),
        "de" => (
            format!("Anmeldung im Kundenportal von {company}"),
            "Mit diesem Link melden Sie sich in unserem Kundenportal an:",
            format!("Der Link ist {minutes} Minuten gültig und kann nur einmal verwendet werden."),
            "Sie haben das nicht angefordert? Dann können Sie diese E-Mail ignorieren.",
            "Mit freundlichen Grüßen",
        ),
        "es" => (
            format!("Acceso al portal de clientes de {company}"),
            "Utilice este enlace para acceder a nuestro portal de clientes:",
            format!("El enlace es válido durante {minutes} minutos y solo puede usarse una vez."),
            "¿No lo ha solicitado? Puede ignorar este correo.",
            "Atentamente",
        ),
        _ => (
            format!("Sign in to the {company} customer portal"),
            "Use this link to sign in to our customer portal:",
            format!("The link is valid for {minutes} minutes and can be used once."),
            "If you did not request this, you can ignore this e-mail.",
            "Kind regards",
        ),
    };
    let body =
        format!("{greeting}\n\n{intro}\n{link}\n\n{valid}\n{ignore}\n\n{regards},\n{company}\n");
    (subject, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_language_has_the_link_and_greeting() {
        for l in ["nl", "fr", "en", "de", "es"] {
            let (subject, body) =
                login_mail(l, "Hello Sam,", "Demo", "https://p/login#token=x", 15);
            assert!(subject.contains("Demo"), "{l}");
            assert!(body.starts_with("Hello Sam,"), "{l}");
            assert!(body.contains("https://p/login#token=x"), "{l}");
            assert!(body.contains("15"), "{l}");
        }
    }
}
