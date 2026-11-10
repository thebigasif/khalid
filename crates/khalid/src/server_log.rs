//! The log of `khalid serve` (G38): every `tracing` line the server and its
//! crates write, such as an email that could not be sent, goes to standard
//! error as one plain line, or one JSON object with `--json` or
//! `[log] format = "json"`. The level comes from `[log] level`, `info`
//! unless set. Nothing secret reaches it: settings hold secrets in
//! `Secret`, which never prints its value (RULES.md BL-RUN-5).

use std::error::Error;
use std::fmt::{self, Write as _};
use std::io;

use chrono::SecondsFormat;
use khalid_core::{Clock, LogFormat, LogLevel, LogSettings, SystemClock};
use serde_json::{Map, Value};
use tracing::field::{Field, Visit};
use tracing::level_filters::LevelFilter;
use tracing::{Event, Metadata, Subscriber, span};

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

/// The log writer for lines at `level` and above, in `format`, each
/// written to a fresh `writer()`.
fn subscriber<M, W>(
    level: LogLevel,
    format: LogFormat,
    writer: M,
) -> Box<dyn Subscriber + Send + Sync>
where
    M: Fn() -> W + Send + Sync + 'static,
    W: io::Write,
{
    Box::new(Lines {
        level: filter(level),
        json: format == LogFormat::Json,
        writer,
    })
}

/// Writes each event as one line, such as `2026-11-10T09:00:00.000000Z
/// WARN a form's answers could not be emailed form="contact"`, or as one
/// JSON object with `timestamp`, `level` and `fields`. Spans are not kept:
/// Khalid opens none, so a span a dependency opens adds nothing to a line.
/// Written by hand because `tracing-subscriber` and the crates under it
/// cost 66 KB of the binary's size budget.
struct Lines<M> {
    level: LevelFilter,
    json: bool,
    writer: M,
}

impl<M, W> Subscriber for Lines<M>
where
    M: Fn() -> W + Send + Sync + 'static,
    W: io::Write,
{
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        *metadata.level() <= self.level
    }

    fn max_level_hint(&self) -> Option<LevelFilter> {
        Some(self.level)
    }

    fn new_span(&self, _: &span::Attributes<'_>) -> span::Id {
        span::Id::from_u64(1)
    }

    fn record(&self, _: &span::Id, _: &span::Record<'_>) {}

    fn record_follows_from(&self, _: &span::Id, _: &span::Id) {}

    fn event(&self, event: &Event<'_>) {
        let mut fields = Fields::default();
        event.record(&mut fields);
        let at = SystemClock
            .now()
            .to_rfc3339_opts(SecondsFormat::Micros, true);
        let level = event.metadata().level().as_str();
        let mut line = if self.json {
            let mut object = Map::new();
            object.insert("timestamp".to_owned(), Value::String(at));
            object.insert("level".to_owned(), Value::String(level.to_owned()));
            object.insert("fields".to_owned(), Value::Object(fields.json));
            Value::Object(object).to_string()
        } else {
            format!("{at} {level:>5} {}{}", fields.message, fields.text)
        };
        line.push('\n');
        // A log line that cannot be written has nowhere else to go.
        let _ = (self.writer)().write_all(line.as_bytes());
    }

    fn enter(&self, _: &span::Id) {}

    fn exit(&self, _: &span::Id) {}
}

/// One event's fields: the message, the rest as ` name=value` for a plain
/// line, and all of them for a JSON line.
#[derive(Default)]
struct Fields {
    message: String,
    text: String,
    json: Map<String, Value>,
}

impl Fields {
    fn add(&mut self, field: &Field, text: String, json: Value) {
        if field.name() == "message" {
            self.message = text;
        } else {
            let _ = write!(self.text, " {}={text}", field.name());
        }
        self.json.insert(field.name().to_owned(), json);
    }
}

impl Visit for Fields {
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.add(field, value.to_string(), Value::from(value));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.add(field, value.to_string(), Value::from(value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.add(field, value.to_string(), Value::from(value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        let text = if field.name() == "message" {
            value.to_owned()
        } else {
            format!("{value:?}")
        };
        self.add(field, text, Value::from(value));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        let said = format!("{value:?}");
        self.add(field, said.clone(), Value::String(said));
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
        let next_said = next.to_string();
        // Some errors repeat their cause's words; say them once.
        if !said.ends_with(&next_said) {
            let _ = write!(said, ": {next_said}");
        }
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

        /// A writer for the log that keeps what it is given here.
        fn writer(&self) -> impl Fn() -> Self + Send + Sync + 'static {
            let kept = self.clone();
            move || kept.clone()
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

    /// Logs `error` the way a failed form email is logged, into `kept`.
    fn log_failure(kept: &Kept, format: LogFormat, error: &(dyn Error + 'static)) {
        let subscriber = subscriber(LogLevel::Info, format, kept.writer());
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

    /// An error that says `words`, with `cause` under it.
    #[derive(Debug)]
    struct Said(&'static str, Option<Box<Said>>);

    impl std::fmt::Display for Said {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.0)
        }
    }

    impl Error for Said {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.1
                .as_deref()
                .map(|cause| -> &(dyn Error + 'static) { cause })
        }
    }

    #[test]
    fn each_cause_is_said_once_in_order() {
        let root = Said("no mechanism", None);
        let middle = Said("client error: no mechanism", Some(Box::new(root)));
        let top = Said("the email could not be sent", Some(Box::new(middle)));
        assert_eq!(
            causes(&top),
            "the email could not be sent: client error: no mechanism"
        );
    }

    #[test]
    fn a_json_line_is_one_object_with_its_level_and_fields() {
        let kept = Kept::default();
        let subscriber = subscriber(LogLevel::Info, LogFormat::Json, kept.writer());
        tracing::subscriber::with_default(subscriber, || {
            tracing::warn!(form = "contact", tries = 7, "Khalid gave up on an email");
        });
        let line: Value = serde_json::from_str(kept.text().trim_end()).unwrap();
        assert_eq!(line["level"], "WARN");
        assert_eq!(line["fields"]["message"], "Khalid gave up on an email");
        assert_eq!(line["fields"]["form"], "contact");
        assert_eq!(line["fields"]["tries"], 7);
        assert!(line["timestamp"].as_str().unwrap().ends_with('Z'), "{line}");
    }

    #[test]
    fn a_plain_line_says_the_level_the_message_and_each_field() {
        let kept = Kept::default();
        let subscriber = subscriber(LogLevel::Info, LogFormat::Text, kept.writer());
        tracing::subscriber::with_default(subscriber, || {
            tracing::warn!(form = "contact", tries = 7, "Khalid gave up on an email");
        });
        let log = kept.text();
        assert!(
            log.ends_with("  WARN Khalid gave up on an email form=\"contact\" tries=7\n"),
            "{log}"
        );
    }

    #[test]
    fn a_line_below_the_level_is_left_out() {
        let kept = Kept::default();
        let subscriber = subscriber(LogLevel::Warn, LogFormat::Text, kept.writer());
        tracing::subscriber::with_default(subscriber, || {
            tracing::info!("a post went live");
            tracing::warn!("an email could not be sent");
        });
        let log = kept.text();
        assert!(!log.contains("a post went live"), "{log}");
        assert!(log.contains("an email could not be sent"), "{log}");
    }
}
