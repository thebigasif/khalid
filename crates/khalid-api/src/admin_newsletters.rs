//! The Admin API's newsletter routes: each newsletter with its counts and
//! sign-up form, the emails that went out, and sending a post (G39, plan
//! 4.6).

use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{OriginalUri, Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json, middleware};
use khalid_core::{Slug, UserMessage};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::admin_key::{AdminDoor, require, require_admin_key};
use crate::admin_subscribers::Subscribers;
use crate::api_version::negotiate_version;
use crate::keys::{Principal, Scope};
use crate::problem::{Problem, ProblemType};
use crate::resources::{AdminNewsletter, AdminSend, NewsletterSend, SendReadiness};
use crate::subscriber_desk::SubscriberError;

/// The newsletter routes, under the Admin API's base. Each needs
/// `subscribers:write`, as the subscriber routes do.
pub(crate) fn routes(subscribers: Subscribers, door: AdminDoor) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_newsletters))
        .routes(routes!(send_post))
        .routes(routes!(count_readers))
        .route_layer(middleware::from_fn_with_state(door, require_admin_key))
        .route_layer(middleware::from_fn(negotiate_version))
        .with_state(subscribers)
}

/// The site's newsletters, whether it can send, and what went out.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AdminNewsletters {
    /// Each newsletter, in alphabetical order.
    newsletters: Vec<AdminNewsletter>,
    /// Whether a post can be sent now, and if not, what the site lacks.
    sending: SendReadiness,
    /// Every email that went out, newest first.
    sends: Vec<AdminSend>,
}

/// A send that started.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct SendStarted {
    /// The key it goes out under, such as `post:en/hello`. Its totals show
    /// in `sends` once the copies are out.
    #[schema(example = "post:en/hello")]
    key: String,
}

/// Lists the newsletters with their readers and sign-up forms, and the
/// emails that went out. Needs `subscribers:write`.
#[utoipa::path(
    get, path = "/newsletters/", tag = "admin subscribers",
    security(("admin_key" = [])),
    responses(
        (status = 200, description = "The newsletters and the emails that went out", body = AdminNewsletters),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks subscribers:write", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn list_newsletters(
    State(subscribers): State<Subscribers>,
    Extension(principal): Extension<Principal>,
) -> Result<Json<AdminNewsletters>, Problem> {
    require(&principal, Scope::SubscribersWrite)?;
    let Some(desk) = subscribers else {
        return Ok(Json(AdminNewsletters {
            newsletters: Vec::new(),
            sending: SendReadiness::NoMail,
            sends: Vec::new(),
        }));
    };
    let newsletters = desk
        .newsletters()
        .await
        .map_err(|e| Problem::internal(&e))?;
    let sends = desk.sends().await.map_err(|e| Problem::internal(&e))?;
    let sending = desk.readiness();
    Ok(Json(AdminNewsletters {
        newsletters,
        sending,
        sends,
    }))
}

/// Emails a published post to a newsletter's confirmed subscribers, once.
/// The answer comes as the copies start to leave. Needs
/// `subscribers:write`.
#[utoipa::path(
    post, path = "/newsletters/{name}/sends/", tag = "admin subscribers",
    security(("admin_key" = [])),
    params(("name" = String, Path, description = "The newsletter, such as main")),
    request_body = NewsletterSend,
    responses(
        (status = 202, description = "The post is going out", body = SendStarted),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks subscribers:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "The site has no newsletter or post by that name", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "The post went to this newsletter before, or the site cannot send email yet", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The body could not be read, or the post is not published", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn send_post(
    State(subscribers): State<Subscribers>,
    Extension(principal): Extension<Principal>,
    Path(name): Path<String>,
    OriginalUri(uri): OriginalUri,
    body: Result<Json<NewsletterSend>, JsonRejection>,
) -> Result<(StatusCode, Json<SendStarted>), Problem> {
    require(&principal, Scope::SubscribersWrite)?;
    let Json(send) = body.map_err(|rejection| unreadable(&rejection.body_text()))?;
    let Some(desk) = subscribers else {
        return Err(not_ready(SendReadiness::NoMail));
    };
    let list = Slug::try_new(&name).map_err(|_| no_list(&name, uri.path()))?;
    let started = desk.send(&list, &send).await;
    let key = started.map_err(|error| match error {
        SubscriberError::NoList => no_list(&name, uri.path()),
        SubscriberError::NoPost => no_post(&send.post),
        SubscriberError::NotPublished => not_published(&send.post),
        SubscriberError::SentBefore => sent_before(&send.post, &name),
        SubscriberError::NotReady(readiness) => not_ready(readiness),
        error => Problem::internal(&error),
    })?;
    Ok((StatusCode::ACCEPTED, Json(SendStarted { key })))
}

/// Which readers to count: labels, separated by commas.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReadersQuery {
    /// The labels every counted reader carries, such as `paid,vip`. Leave
    /// it out to count every confirmed reader.
    labels: Option<String>,
}

/// How many readers a send would reach.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct SegmentReaders {
    /// The newsletter.
    #[schema(example = "main")]
    list: String,
    /// The labels each counted reader carries, each once.
    #[schema(example = json!(["paid"]))]
    labels: Vec<String>,
    /// How many confirmed readers carry every label.
    #[schema(example = 42)]
    readers: u64,
}

/// Counts the confirmed readers of a newsletter who carry every label
/// given, so a send to them can say how many it reaches before it starts.
/// Needs `subscribers:write`.
#[utoipa::path(
    get, path = "/newsletters/{name}/readers/", tag = "admin subscribers",
    security(("admin_key" = [])),
    params(("name" = String, Path, description = "The newsletter, such as main"), ReadersQuery),
    responses(
        (status = 200, description = "How many readers a send would reach", body = SegmentReaders),
        (status = 400, description = "A label is not lowercase letters, digits and hyphens", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "No valid integration key or staff token", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The key lacks subscribers:write", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "The site has no newsletter by that name", body = Problem, content_type = "application/problem+json"),
    )
)]
async fn count_readers(
    State(subscribers): State<Subscribers>,
    Extension(principal): Extension<Principal>,
    Path(name): Path<String>,
    OriginalUri(uri): OriginalUri,
    query: Result<Query<ReadersQuery>, QueryRejection>,
) -> Result<Json<SegmentReaders>, Problem> {
    require(&principal, Scope::SubscribersWrite)?;
    let Query(query) = query.map_err(|_| bad_labels())?;
    let list = Slug::try_new(&name).map_err(|_| no_list(&name, uri.path()))?;
    let mut labels: Vec<Slug> = Vec::new();
    for typed in query.labels.as_deref().unwrap_or_default().split(',') {
        let typed = typed.trim();
        if typed.is_empty() {
            continue;
        }
        let label = Slug::try_new(typed).map_err(|_| bad_labels())?;
        if !labels.contains(&label) {
            labels.push(label);
        }
    }
    let readers = match subscribers {
        Some(desk) => desk.readers(&list, &labels).await,
        None => Ok(0),
    };
    let readers = readers.map_err(|error| match error {
        SubscriberError::NoList => no_list(&name, uri.path()),
        error => Problem::internal(&error),
    })?;
    Ok(Json(SegmentReaders {
        list: list.as_str().to_owned(),
        labels: labels
            .iter()
            .map(|label| label.as_str().to_owned())
            .collect(),
        readers,
    }))
}

fn bad_labels() -> Problem {
    let message = UserMessage::new(
        "A label in the count is not one Khalid can read.",
        "Labels are lowercase letters, digits and hyphens, such as paid.",
        "Separate the labels with commas, such as labels=paid,vip.",
    );
    Problem::new(ProblemType::InvalidQuery, &message)
}

fn unreadable(reason: &str) -> Problem {
    let message = UserMessage::new(
        format!("The send could not be read: {reason}."),
        "Nothing was sent.",
        "Send the post's identity, such as en.hello, and any labels as a list.",
    );
    Problem::new(ProblemType::InvalidBody, &message)
}

fn no_list(name: &str, path: &str) -> Problem {
    let message = UserMessage::new(
        format!("The site has no newsletter called {name}."),
        "Each newsletter but main is a folder in content/newsletters/.",
        "Pick a newsletter from the list, or make its folder first.",
    );
    Problem::new(ProblemType::NotFound, &message).with_instance(path)
}

fn no_post(post: &str) -> Problem {
    let message = UserMessage::new(
        format!("There is no post {post}."),
        "Only a post in the site's content can be sent.",
        "Pick the post from the list of published posts.",
    );
    Problem::new(ProblemType::NotFound, &message).with_rule("post")
}

fn not_published(post: &str) -> Problem {
    let message = UserMessage::new(
        format!("The post {post} is not published."),
        "Subscribers get only posts that readers can open on the site.",
        "Publish it first, then send it.",
    );
    Problem::new(ProblemType::InvalidBody, &message).with_rule("post")
}

fn sent_before(post: &str, list: &str) -> Problem {
    let message = UserMessage::new(
        format!("The post {post} went to {list} before, so nothing was sent."),
        "Each post is emailed once; an edit never sends it again.",
        "Write a new post to tell subscribers about the change.",
    );
    Problem::new(ProblemType::Conflict, &message)
}

/// The 409 for a site that cannot send email yet, saying what it lacks.
fn not_ready(readiness: SendReadiness) -> Problem {
    let message = match readiness {
        SendReadiness::NoPostalAddress => UserMessage::new(
            "Khalid sends no newsletter until the site has a postal address.",
            "Anti-spam laws ask every newsletter email to show where the sender can be reached by post.",
            "Add postal_address = \"...\" under [mail] in site.toml, then send again.",
        ),
        _ => UserMessage::new(
            "This site cannot send email yet.",
            "A newsletter goes out the way [mail] in site.toml names, and there is none.",
            "Add a [mail] table to site.toml, as docs/deploy.md shows under \"Sending email\".",
        ),
    };
    Problem::new(ProblemType::NotSetUp, &message).with_rule("sending")
}
