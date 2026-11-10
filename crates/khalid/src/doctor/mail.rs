//! The doctor check for mail (G38): `[mail]` loaded with the settings and
//! the secret its way of sending needs is set. It only looks at whether
//! the secret is set; it never prints it.

use khalid_core::{Config, Environment, MailTransport, UserMessage};

use crate::doctor_check::{Check, Finding};

/// What the summary says of a site that sends no mail.
const NO_MAIL: &str = "The site sends no email, so newsletter sign-ups stay closed and form answers stay in the database only. Add [mail] to site.toml to send them.";

/// Checks the mail settings of `config` against the secrets in `env`.
/// Skipped when the settings did not load.
pub(crate) fn check(config: Option<&Config>, env: &Environment) -> Finding {
    let Some(config) = config else {
        return Finding::skipped(
            Check::Mail,
            "Khalid reads how mail is sent from the settings, so fix those first.",
        );
    };
    let Some(mail) = config.mail() else {
        return Finding::ok(Check::Mail, NO_MAIL);
    };
    let way = way(&mail.transport);
    let secret = env.mail_secret(&mail.transport);
    let said = match variable(&mail.transport) {
        Some(variable) if secret.is_none() => {
            return Finding::problem(Check::Mail, missing(&way, variable));
        }
        Some(variable) => format!(
            "Mail goes out through {way} from {}, with {variable} set.",
            mail.from
        ),
        None => format!("Mail goes out through {way} from {}.", mail.from),
    };
    Finding::ok(Check::Mail, said)
}

/// The way `transport` sends, as a person names it.
fn way(transport: &MailTransport) -> String {
    match transport {
        MailTransport::Smtp(smtp) => format!("the SMTP server {}", smtp.host),
        MailTransport::Resend => "Resend".to_owned(),
        MailTransport::Postmark(_) => "Postmark".to_owned(),
        _ => "the way [mail] names".to_owned(),
    }
}

/// The variable that holds the secret `transport` needs, or `None` for an
/// SMTP server that takes mail without signing in.
fn variable(transport: &MailTransport) -> Option<&'static str> {
    match transport {
        MailTransport::Smtp(smtp) if smtp.username.is_none() => None,
        MailTransport::Smtp(_) => Some(Environment::SMTP_PASSWORD),
        MailTransport::Resend => Some(Environment::RESEND_API_KEY),
        MailTransport::Postmark(_) => Some(Environment::POSTMARK_TOKEN),
        _ => None,
    }
}

/// The problem when `variable`, the secret for `way`, is not set.
fn missing(way: &str, variable: &str) -> UserMessage {
    UserMessage::new(
        format!("Mail is set to go through {way}, but {variable} is not set."),
        "khalid serve refuses to start without it, so no sign-up link, form answer or alert could be sent.",
        format!(
            "Set {variable} in the environment khalid serve runs in, as docs/deploy.md shows under Sending email, then run `khalid doctor` again."
        ),
    )
}
