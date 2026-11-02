//! The writers behind the editor's suggestions: Anthropic's Messages API,
//! any OpenAI-compatible chat completions service, or `SerpDad`, asked
//! through the shared outbound client with the key in `KHALID_AI_KEY`
//! (`docs/adr/0000-ai-suggestions-ask-a-service-the-owner-names.md`).

use std::sync::Arc;
use std::time::Duration;

use khalid_api::ai::{AiAsk, AiDesk, AiError, AiWriter, Lengths};
use khalid_api::content_store::BoxFuture;
use khalid_core::{AbsoluteUrl, AiProvider, AiSettings, Config, Environment, Secret};
use khalid_outbound::{Limits, Outbound};
use serde_json::{Value, json};

use crate::cli::Cli;
use crate::outbound;

/// Anthropic's model when `[ai]` names none: quick and cheap.
const ANTHROPIC_MODEL: &str = "claude-haiku-5-5";

/// Where each provider is asked when `[ai]` names no `endpoint`.
const ANTHROPIC: &str = "https://api.anthropic.com/v1/messages";
const OPENAI: &str = "https://api.openai.com/v1/chat/completions";
const SERPDAD: &str = "https://serpdad.com/api/khalid/v1/suggestions";

/// A model takes longer than a web page; a minute is the most a writer
/// waits.
const WAIT: Duration = Duration::from_secs(60);

/// The most words a model may answer with.
const MOST_TOKENS: u32 = 1024;

/// The desk for the site with `config`: off unless `[ai]` names a
/// provider, and without a writer while `env` holds no key.
pub(crate) fn desk(config: &Config, env: &Environment, cli: &Cli) -> AiDesk {
    let settings = config.ai();
    if !settings.is_on() {
        return AiDesk::off();
    }
    let limits = Limits::default().with_timeout(WAIT);
    let writer = env
        .ai_key()
        .zip(outbound::client_within(cli, limits).ok())
        .and_then(|(key, outbound)| HttpWriter::new(settings, key, outbound))
        .map(|writer| -> Arc<dyn AiWriter> { Arc::new(writer) });
    AiDesk::new(settings.provider, writer, Lengths::default())
}

/// A writer that asks one provider over HTTP.
struct HttpWriter {
    provider: AiProvider,
    model: String,
    endpoint: AbsoluteUrl,
    key: Secret<String>,
    outbound: Outbound,
}

impl HttpWriter {
    /// The writer `settings` describe, or `None` for `off`.
    fn new(settings: &AiSettings, key: Secret<String>, outbound: Outbound) -> Option<Self> {
        let (usual, model) = match settings.provider {
            AiProvider::Anthropic => (ANTHROPIC, ANTHROPIC_MODEL),
            AiProvider::Openai => (OPENAI, ""),
            AiProvider::Serpdad => (SERPDAD, ""),
            _ => return None,
        };
        let endpoint = match &settings.endpoint {
            Some(endpoint) => endpoint.clone(),
            None => AbsoluteUrl::parse(usual).ok()?,
        };
        let model = settings.model.clone().unwrap_or_else(|| model.to_owned());
        Some(Self {
            provider: settings.provider,
            model,
            endpoint,
            key,
            outbound,
        })
    }

    /// The request body and headers for `ask`.
    fn request(&self, ask: &AiAsk) -> (Value, Vec<(&'static str, String)>) {
        let key = self.key.expose().clone();
        match self.provider {
            AiProvider::Anthropic => (
                json!({
                    "model": self.model,
                    "max_tokens": MOST_TOKENS,
                    "system": ask.system,
                    "messages": [{"role": "user", "content": ask.prompt}],
                }),
                vec![
                    ("x-api-key", key),
                    ("anthropic-version", "2023-06-01".to_owned()),
                ],
            ),
            AiProvider::Serpdad => (
                json!({
                    "task": ask.task,
                    "system": ask.system,
                    "prompt": ask.prompt,
                    "count": ask.count,
                }),
                vec![("authorization", format!("Bearer {key}"))],
            ),
            _ => (
                json!({
                    "model": self.model,
                    "max_tokens": MOST_TOKENS,
                    "messages": [
                        {"role": "system", "content": ask.system},
                        {"role": "user", "content": ask.prompt},
                    ],
                }),
                vec![("authorization", format!("Bearer {key}"))],
            ),
        }
    }

    /// The options in `answer`, as this provider shapes it.
    fn options(&self, answer: &Value, count: u8) -> Option<Vec<String>> {
        let text = match self.provider {
            AiProvider::Anthropic => answer
                .get("content")?
                .as_array()?
                .iter()
                .filter_map(|part| part.get("text").and_then(Value::as_str))
                .collect::<Vec<_>>()
                .join("\n"),
            AiProvider::Serpdad => answer
                .get("suggestions")?
                .as_array()?
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join("\n"),
            _ => answer
                .pointer("/choices/0/message/content")?
                .as_str()?
                .to_owned(),
        };
        Some(options_from_text(&text, count))
    }
}

impl AiWriter for HttpWriter {
    fn write<'a>(&'a self, ask: &'a AiAsk) -> BoxFuture<'a, Result<Vec<String>, AiError>> {
        Box::pin(async move {
            let (body, headers) = self.request(ask);
            let mut sent: Vec<(&str, &str)> = vec![("content-type", "application/json")];
            sent.extend(headers.iter().map(|(name, value)| (*name, value.as_str())));
            let body = serde_json::to_vec(&body).map_err(|e| AiError::Unreachable(e.into()))?;
            let answer = self.outbound.post(&self.endpoint, &sent, body).await;
            let answer = answer.map_err(|e| AiError::Unreachable(e.into()))?;
            let status = answer.status();
            if status == 401 || status == 403 {
                return Err(AiError::Refused { status });
            }
            let read: Option<Value> = serde_json::from_slice(answer.body()).ok();
            let options = read
                .filter(|_| (200..300).contains(&status))
                .and_then(|read| self.options(&read, ask.count));
            options
                .filter(|options| !options.is_empty())
                .ok_or(AiError::Answer { status })
        })
    }
}

/// The options in a model's `text`: one per line, without the numbers,
/// bullets or quotes models like to add, blank and repeated lines left
/// out, at most `count`.
fn options_from_text(text: &str, count: u8) -> Vec<String> {
    let mut options: Vec<String> = Vec::new();
    for line in text.lines() {
        let option = clean(line);
        if !option.is_empty() && !options.iter().any(|seen| seen == option) {
            options.push(option.to_owned());
        }
    }
    options.truncate(usize::from(count));
    options
}

/// `line` without a list marker or surrounding quotes.
fn clean(line: &str) -> &str {
    let line = line.trim();
    let digits = line.trim_start_matches(|c: char| c.is_ascii_digit());
    let line = match digits.strip_prefix(['.', ')']) {
        Some(rest) if digits.len() < line.len() => rest,
        _ => line,
    };
    let line = line.trim_start_matches(['-', '*', '\u{2022}']).trim();
    line.trim_matches(['"', '\u{201c}', '\u{201d}']).trim()
}

#[cfg(all(test, feature = "test-util"))]
#[path = "ai_writer_tests.rs"]
mod tests;
