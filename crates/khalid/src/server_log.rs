//! The log of `khalid serve` (G38): every `tracing` line the server and its
//! crates write, such as an email that could not be sent, goes to standard
//! error as one plain line, or one JSON object with `--json` or
//! `[log] format = "json"`. The level comes from `[log] level`, `info`
//! unless set. Nothing secret reaches it: settings hold secrets in
//! `Secret`, which never prints its value (RULES.md BL-RUN-5).

use std::error::Error;
use std::fmt::Write as _;

use khalid_core::{LogFormat, LogLevel, LogSettings};
use tracing::Subscriber;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::MakeWriter;

use crate::cli::Output;

/// Sends the server's log lines to standard error, as `settings` and the
/// command line's `output` ask. Only the first call counts, so a server of
/// several sites logs the way the first one asks.
pub(crate) fn install(settings: LogSettings, output: Output) {
    let format = match output {
        Output::Json => LogFormat::Json,
        Output::Human => settings.format,
    };
    let subscriber = subscriber(settings.level, format, std::io::stderr);
    // A log writer is already there when this is not the first call.
    let _ = tracing::subscriber::set_global_default(subscriber);
}

/// The log writer for lines at `level` and above, in `format`,
/// written to `writer`.
fn subscriber<W>(level: LogLevel, format: LogFormat, writer: W) -> Box<dyn Subscriber + Send + Sync>
where
    W: for<'w> MakeWriter<'w> + Send + Sync + 'static,
{
    let builder = tracing_subscriber::fmt()
        .with_writer(writer)
        .with_max_level(filter(level))
        .with_target(false);
    if format == LogFormat::Json {
        Box::new(builder.json().finish())
    } else {
        Box::new(builder.finish())
    }
}

/// The filter that keeps lines at `level` and above.
fn filter(level: LogLevel) -> LevelFilter {
    match level {
        LogLevel::Trace => LevelFilter::TRACE,
        LogLevel::Debug => LevelFilter::DEBUG,
        LogLevel::Warn => LevelFilter::WARN,
        LogLevel::Error => LevelFilter::ERROR,
        _ => LevelFilter::INFO,
    }
}

/// `error` and each cause under it, joined with `: `, so a log line says
/// why, such as `the email could not be sent: the SMTP server did not take
/// the email: Connection refused`. The errors of the mail transports hold
/// no email text and no secret.
pub(crate) fn causes(error: &(dyn Error + 'static)) -> String {
    let mut said = error.to_string();
    let mut cause = error.source();
    while let Some(next) = cause {
        let _ = write!(said, ": {next}");
        cause = next.source();
    }
    said
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::sync::{Arc, Mutex, PoisonError};

    use khalid_core::{Config, Environment, Secret, SmtpSettings};
    use khalid_mail::{Sender, SmtpMailer};
    use khalid_newsletter::{EmailAddress, Mail, Mailer};
    use khalid_outbound::Resolver;

    use super::*;

    /// A writer that keeps every log line, for reading back.
    #[derive(Clone, Default)]
    struct Kept(Arc<Mutex<Vec<u8>>>);

    impl Kept {
        fn text(&self) -> String {
            let bytes = self.0.lock().unwrap_or_else(PoisonError::into_inner);
            String::from_utf8_lossy(&bytes).into_owned()
        }
    }

    impl io::Write for Kept {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            let mut kept = self.0.lock().unwrap_or_else(PoisonError::into_inner);
            kept.extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl<'w> MakeWriter<'w> for Kept {
        type Writer = Self;

        fn make_writer(&'w self) -> Self::Writer {
            self.clone()
        }
    }

    /// Logs `error` the way a failed form email is logged, into `kept`.
    fn log_failure(kept: &Kept, format: LogFormat, error: &(dyn Error + 'static)) {
        let subscriber = subscriber(LogLevel::Info, format, kept.clone());
        tracing::subscriber::with_default(subscriber, || {
            let error = causes(error);
            tracing::warn!(%error, form = "contact", "a form's answers could not be emailed");
        });
    }

    fn mail() -> Mail {
        let to = EmailAddress::parse("hello@example.com").unwrap();
        Mail::new(to, "A subject".to_owned(), "A text".to_owned())
    }

    fn sender() -> Sender {
        let env = Environment::from_pairs(&[(Environment::RESEND_API_KEY, "made-for-this-test")]);
        let site = "name = \"Field Notes\"\naddress = \"https://www.example.com\"\n\
                    default_language = \"en\"\nlanguages = [\"en\"]\n\n\
                    [mail]\nfrom = \"news@example.com\"\n\n[mail.resend]\n";
        let config = Config::from_toml(std::path::Path::new("site.toml"), site, &env).unwrap();
        Sender::from(config.mail().unwrap())
    }

    #[cfg(feature = "test-util")]
    #[tokio::test]
    async fn a_refused_api_email_logs_its_status_and_never_the_key() {
        use khalid_core::AbsoluteUrl;
        use khalid_mail::{ApiMailer, Service};
        use khalid_outbound::{Limits, Outbound, Reach};
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let service = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&service)
            .await;
        let key = format!("re_made_for_this_test_{}", std::process::id());
        let resolver = Resolver::for_tests(Vec::new(), Reach::PublicAndLoopback);
        let outbound = Outbound::with_resolver(Limits::default(), resolver).unwrap();
        let endpoint = AbsoluteUrl::parse(&format!("{}/emails", service.uri())).unwrap();
        let api = ApiMailer::new(
            Service::Resend,
            Secret::new(key.clone()),
            sender(),
            outbound,
        );
        let api = api.unwrap().at(endpoint);
        let error = api.send(&mail()).await.unwrap_err();
        for format in [LogFormat::Text, LogFormat::Json] {
            let kept = Kept::default();
            log_failure(&kept, format, &error);
            let log = kept.text();
            assert!(log.contains("WARN") || log.contains("\"WARN\""), "{log}");
            assert!(log.contains("status 401"), "{log}");
            assert!(!log.contains(&key), "the key reached the log: {log}");
        }
    }

    #[tokio::test]
    async fn a_failed_smtp_email_logs_why_and_never_the_password() {
        let closed = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = closed.local_addr().unwrap().port();
        drop(closed);
        let site = format!(
            "host = \"127.0.0.1\"\nport = {port}\nsecurity = \"none\"\n\
             private_host = true\nusername = \"news@example.com\"\n"
        );
        let settings: SmtpSettings = toml::from_str(&site).unwrap();
        let password = format!("pw-made-for-this-test-{}", std::process::id());
        let secret = Some(Secret::new(password.clone()));
        let smtp = SmtpMailer::new(settings, secret, sender(), Resolver::new());
        let error = smtp.send(&mail()).await.unwrap_err();
        let kept = Kept::default();
        log_failure(&kept, LogFormat::Text, &error);
        let log = kept.text();
        assert!(
            log.contains("the SMTP server did not take the email"),
            "{log}"
        );
        assert!(
            log.contains("form=\"contact\"") || log.contains("form=contact"),
            "{log}"
        );
        assert!(
            !log.contains(&password),
            "the password reached the log: {log}"
        );
    }

    #[test]
    fn a_line_below_the_level_is_left_out() {
        let kept = Kept::default();
        let subscriber = subscriber(LogLevel::Warn, LogFormat::Text, kept.clone());
        tracing::subscriber::with_default(subscriber, || {
            tracing::info!("a post went live");
            tracing::warn!("an email could not be sent");
        });
        let log = kept.text();
        assert!(!log.contains("a post went live"), "{log}");
        assert!(log.contains("an email could not be sent"), "{log}");
    }
}
