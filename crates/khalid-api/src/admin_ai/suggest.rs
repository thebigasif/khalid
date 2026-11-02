//! `POST /api/admin/v1/ai/suggestions/`: options for one field, checked
//! against the length rules, from the words sent or a saved post's.

use std::error::Error;
use std::sync::Arc;

use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::{Extension, Json};
use khalid_core::{AiProvider, UserMessage};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::AiRoute;
use crate::admin_api::invalid_body;
use crate::admin_key::require;
use crate::ai::{AiAsk, AiTask, AiWords, AiWriter};
use crate::keys::{Principal, Scope};
use crate::problem::{Problem, ProblemType};
use crate::publish_door::PublishDoor;

/// What to write and the words to write it from. With `post`, the saved
/// post's title, text, language and focus keyword are used, and any of
/// them given here win, so unsaved changes count.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct SuggestionRequest {
    /// What the suggestion is for.
    task: AiTask,
    /// A post's identity, such as `en.choosing-anchor-text`.
    post: Option<String>,
    /// The post's title.
    title: Option<String>,
    /// The post's text, in markdown.
    text: Option<String>,
    /// The post's focus keyword.
    focus_keyword: Option<String>,
    /// The post's language, such as `en`.
    language: Option<String>,
}

/// One option, with its length checked.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct Suggestion {
    /// The words.
    text: String,
    /// How many characters it has.
    characters: usize,
    /// Whether it fits the length rule for its task.
    fits: bool,
    /// Why it does not fit, in plain words, or `null` when it does.
    note: Option<String>,
}

/// The options to pick from, fitting ones first. Nothing in a post changes
/// until a writer picks one.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct Offered {
    /// What they are for.
    task: AiTask,
    /// The label every suggestion is shown with.
    #[schema(example = "AI suggestion from Anthropic")]
    label: String,
    /// The options.
    suggestions: Vec<Suggestion>,
}

/// Asks the owner's provider for options: five for titles, descriptions
/// and alt text, three fixes, or up to five takeaways. Needs
/// `posts:write`. The bulk SEO editor calls it once per post with `post`.
#[utoipa::path(
    post, path = "/ai/suggestions/", tag = "admin ai",
    security(("admin_key" = [])),
    request_body = SuggestionRequest,
    responses(
        (status = 200, description = "The options", body = Offered),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks posts:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "No post has the identity in post", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "Suggestions are off, the key is not set, or SerpDad is not connected", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The request lacks the words its task needs", body = Problem, content_type = "application/problem+json"),
        (status = 502, description = "The AI service did not answer with suggestions", body = Problem, content_type = "application/problem+json"),
    )
)]
pub(super) async fn suggest(
    State(route): State<AiRoute>,
    Extension(principal): Extension<Principal>,
    request: Result<Json<SuggestionRequest>, JsonRejection>,
) -> Result<Json<Offered>, Problem> {
    require(&principal, Scope::PostsWrite)?;
    let Json(request) = request.map_err(|rejection| invalid_body(&rejection.body_text()))?;
    let writer = ready(&route).await?;
    let task = request.task;
    let words = words(&route.posts, request).await?;
    let lengths = route.desk.lengths;
    let ask = AiAsk {
        task,
        system: task.system(&lengths),
        prompt: task.prompt(&words),
        count: task.count(),
    };
    let written = writer.write(&ask).await.map_err(|failed| {
        let error: &(dyn Error + 'static) = &failed;
        tracing::warn!(error, "the AI service wrote no suggestions");
        Problem::new(ProblemType::Upstream, &failed.message())
    })?;
    let mut suggestions: Vec<Suggestion> = written
        .into_iter()
        .map(|text| {
            let note = lengths.misfit(task, &text);
            let characters = text.chars().count();
            let fits = note.is_none();
            Suggestion {
                text,
                characters,
                fits,
                note,
            }
        })
        .collect();
    if suggestions.is_empty() {
        return Err(Problem::new(ProblemType::Upstream, &nothing_written()));
    }
    suggestions.sort_by_key(|suggestion| !suggestion.fits);
    let label = format!("AI suggestion from {}", route.desk.provider.label());
    Ok(Json(Offered {
        task,
        label,
        suggestions,
    }))
}

/// The writer, once suggestions are on, the key is set and, for
/// `SerpDad`, the connection is on.
async fn ready(route: &AiRoute) -> Result<Arc<dyn AiWriter>, Problem> {
    let provider = route.desk.provider;
    if provider == AiProvider::Off {
        let message = UserMessage::new(
            "Writing suggestions are off.",
            "The site's owner has not chosen a service to write them, so nothing is sent anywhere.",
            "Ask the owner to add an [ai] table to site.toml, as docs/deploy.md shows.",
        );
        return Err(Problem::new(ProblemType::NotSetUp, &message));
    }
    let Some(writer) = route.desk.writer.clone() else {
        let message = UserMessage::new(
            format!("The server has no key for {}.", provider.label()),
            "Suggestions are on in site.toml, but the server needs KHALID_AI_KEY to ask.",
            "Ask the owner to set KHALID_AI_KEY on the server and restart it.",
        );
        return Err(Problem::new(ProblemType::NotSetUp, &message));
    };
    if provider == AiProvider::Serpdad {
        let link = match &route.serpdad {
            Some(store) => store.link().await.map_err(|e| Problem::internal(&e))?,
            None => None,
        };
        if link.is_none() {
            let message = UserMessage::new(
                "SerpDad is not connected.",
                "Suggestions come from SerpDad, and only while it is connected.",
                "Connect it on the SerpDad page, or choose another provider in site.toml.",
            );
            return Err(Problem::new(ProblemType::NotSetUp, &message));
        }
    }
    Ok(writer)
}

/// The words `request` gives, over the saved post's when it names one.
async fn words(posts: &PublishDoor, request: SuggestionRequest) -> Result<AiWords, Problem> {
    let mut words = AiWords::default();
    if let Some(id) = &request.post {
        let post = posts.get(id).await.map_err(|e| Problem::internal(&e))?;
        let post = post.ok_or_else(|| no_post(id))?;
        words.language = id.split_once('.').map(|(language, _)| language.to_owned());
        words.title = Some(post.title().to_owned());
        words.text = Some(post.markdown().to_owned());
        let keyword = post.settings().focus_keyword.as_ref();
        words.focus_keyword = keyword.map(|keyword| keyword.as_str().to_owned());
    }
    words.language = request.language.or(words.language);
    words.title = request.title.or(words.title);
    words.text = request.text.or(words.text);
    words.focus_keyword = request.focus_keyword.or(words.focus_keyword);
    let needs = match request.task {
        AiTask::LintFix if words.finding.is_none() => Some("the failing check, in finding"),
        AiTask::AltText if words.picture.is_none() => Some("the picture, in picture"),
        AiTask::AltText | AiTask::LintFix => None,
        _ if blank(words.title.as_deref()) && blank(words.text.as_deref()) => {
            Some("the post's title or text, or a post's identity in post")
        }
        _ => None,
    };
    match needs {
        Some(needs) => Err(missing_words(needs)),
        None => Ok(words),
    }
}

fn blank(text: Option<&str>) -> bool {
    text.is_none_or(|text| text.trim().is_empty())
}

/// The 422 for a request without the words its task needs.
fn missing_words(needs: &str) -> Problem {
    let message = UserMessage::new(
        format!("This suggestion needs {needs}."),
        "A suggestion is written from the words it is sent, and these were left out.",
        "Send them in the request, as /api/openapi.json shows.",
    );
    Problem::new(ProblemType::InvalidBody, &message)
}

/// The 404 for a post that is not there.
fn no_post(id: &str) -> Problem {
    let message = UserMessage::new(
        format!("There is no post {id}."),
        "A suggestion can only read a post that exists.",
        "Check the post's identity, or send its title and text instead.",
    );
    Problem::new(ProblemType::NotFound, &message)
}

/// The 502 for a service that answered with no options.
fn nothing_written() -> UserMessage {
    UserMessage::new(
        "The AI service answered with no suggestions.",
        "It may not have understood the post, or it is busy.",
        "Try again in a minute. Nothing in the post changed.",
    )
}
