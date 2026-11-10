//! The doctor check for mail (G38): `[mail]` loaded with the settings, the
//! secret the way of sending needs is set, and with `--send-test-mail` one
//! email goes out through the mailer `khalid serve` uses. It only looks at
//! whether the secret is set; it never prints it.

use khalid_core::{Config, Environment, MailSettings, MailTransport, Secret, UserMessage};
use khalid_newsletter::{EmailAddress, Mail};

use crate::cli::Cli;
use crate::doctor_check::{Check, Finding, MailTest};
use crate::{newsletter, outbound, server_log};

/// What the summary says of a site that sends no mail.
const NO_MAIL: &str = "The site sends no email, so newsletter sign-ups stay closed and form answers stay in the database only. Add [mail] to site.toml to send them.";

/// Checks the mail settings of `config` against the secrets in `env`, and
/// sends one test email when `test` asks. Skipped when the settings did
/// not load.
pub(crate) fn check(
    config: Option<&Config>,
    env: &Environment,
    (test, cli): (MailTest, &Cli),
) -> Finding {
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
    if test == MailTest::Skip {
        return Finding::ok(Check::Mail, said);
    }
    match send_test(config, mail, secret, cli) {
        Ok(to) => Finding::ok(Check::Mail, format!("{said} A test email went to {to}.")),
        Err(message) => Finding::problem(Check::Mail, message),
    }
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

/// Sends one email to the address form answers go to, through the mailer
/// `mail` makes with `secret`, and returns that address.
fn send_test(
    config: &Config,
    mail: &MailSettings,
    secret: Option<Secret<String>>,
    cli: &Cli,
) -> Result<String, UserMessage> {
    let to = EmailAddress::parse(mail.form_answers_to()).map_err(|_| internal())?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| internal())?;
    let client = outbound::client(cli).map_err(|_| internal())?;
    let mailer = newsletter::mailer(config, secret, &client)?.ok_or_else(internal)?;
    let text = format!(
        "khalid doctor sent this email to check that {} can send mail. Sign-up links, form answers and alerts leave the same way.\n\nThere is nothing to do.\n",
        config.name()
    );
    let email = Mail::new(
        to.clone(),
        "A test email from khalid doctor".to_owned(),
        text,
    );
    match runtime.block_on(mailer.send(&email)) {
        Ok(()) => Ok(to.as_str().to_owned()),
        Err(error) => Err(UserMessage::new(
            format!(
                "The test email to {} could not be sent: {}.",
                to.as_str(),
                server_log::causes(&error)
            ),
            "Sign-up links, form answers and alerts leave the same way, so none of them arrive.",
            failed_fix(mail),
        )),
    }
}

/// What to check when the test email through `mail` failed.
fn failed_fix(mail: &MailSettings) -> String {
    let again = "then run `khalid doctor --send-test-mail` again.";
    match (&mail.transport, variable(&mail.transport)) {
        (MailTransport::Smtp(_), _) => format!(
            "Check host, port, security and username under [mail.smtp] in site.toml, and {} if it signs in, {again}",
            Environment::SMTP_PASSWORD
        ),
        (transport, Some(variable)) => format!(
            "Check that {variable} is a working key and that {} accepts {} as a sender, {again}",
            way(transport),
            mail.from
        ),
        _ => format!("Check [mail] in site.toml, {again}"),
    }
}

/// The message when the test cannot start for a reason inside Khalid or
/// the computer.
fn internal() -> UserMessage {
    UserMessage::new(
        "Khalid could not start sending the test email.",
        "The cause is inside Khalid or the computer, not in your site.",
        "Run `khalid doctor --send-test-mail` again, and report it with the output of `khalid --version` if it fails again.",
    )
}
