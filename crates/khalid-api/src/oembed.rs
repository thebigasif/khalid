//! `/api/oembed`: the oEmbed provider, so another site that is given the
//! address of a post shows it as a card (plan 5.5, `api.md` Surfaces).
//! It needs no key: it says only what the post's page already shows.

use std::sync::Arc;

use axum::Json;
use axum::extract::rejection::QueryRejection;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use khalid_core::{Slug, UserMessage};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::content_store::{ContentStore, PostKey, SiteResources};
use crate::problem::{Problem, ProblemType};
use crate::resources::{Author, Collection, HeadElement, Listable as _, Post};

/// Where the provider answers, the address a post page's discovery link
/// names.
pub const PATH: &str = "/api/oembed";

/// The card's width when the consumer sets no `maxwidth`, in pixels: the
/// width X and Mastodon give theirs.
const WIDTH: u32 = 550;

/// The narrowest card that still reads, in pixels.
const NARROWEST: u32 = 200;

/// The store the route reads from.
type Store = Arc<dyn ContentStore>;

/// The provider's route, declared with `utoipa::path` like every other.
pub(crate) fn routes(store: Store) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(embed))
        .with_state(store)
}

/// What a consumer asks: which post, in which format, how wide at most.
/// A card is as tall as its text, so `maxheight` is ignored like any other
/// parameter.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub(crate) struct EmbedQuery {
    /// The full address of a published post.
    url: String,
    /// `json`, the only format served. `xml` answers 501, as oEmbed asks.
    format: Option<String>,
    /// The widest the card may be, in pixels, from 200; 550 when left out.
    maxwidth: Option<u32>,
}

/// A post as an oEmbed `rich` answer: a card of its title, summary and
/// site that the consumer puts in its page as it is.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct Embed {
    /// Always `rich`.
    #[schema(example = "rich")]
    r#type: &'static str,
    /// Always `1.0`.
    #[schema(example = "1.0")]
    version: &'static str,
    /// The post's title.
    title: String,
    /// The first author's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,
    /// The first author's page on the blog.
    #[serde(skip_serializing_if = "Option::is_none")]
    author_url: Option<String>,
    /// The site's name.
    provider_name: String,
    /// The blog's front page.
    provider_url: String,
    /// The post's share image.
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_url: Option<String>,
    /// Its width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_width: Option<u32>,
    /// Its height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_height: Option<u32>,
    /// The card: a quote of the title, the summary and the site, each a
    /// plain link, with no script and no frame.
    html: String,
    /// The card's width in pixels.
    width: u32,
    /// Unset: the card is as tall as its text.
    #[schema(value_type = Option<u32>)]
    height: Option<u32>,
}

/// Answers a consumer with the card of the post at `url`.
#[utoipa::path(
    get, path = PATH, tag = "oembed", params(EmbedQuery),
    responses(
        (status = 200, description = "The post as a card", body = Embed),
        (status = 400, description = "No url was given", body = Problem, content_type = "application/problem+json"),
        (status = 404, description = "No published post is at the url", body = Problem, content_type = "application/problem+json"),
        (status = 501, description = "A format other than json was asked for"),
    )
)]
async fn embed(
    State(store): State<Store>,
    query: Result<Query<EmbedQuery>, QueryRejection>,
) -> Result<Response, Problem> {
    let Query(query) = query.map_err(|_| no_url())?;
    if query
        .format
        .as_deref()
        .is_some_and(|format| format != "json")
    {
        return Ok(StatusCode::NOT_IMPLEMENTED.into_response());
    }
    let wanted = page_of(&query.url);
    let slug = wanted
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or_default();
    let found = match Slug::try_new(slug) {
        Ok(slug) => (store.find(Collection::Posts, &PostKey::Slug(slug)).await)
            .map_err(|error| Problem::internal(&error))?,
        Err(_) => None,
    };
    let post = found
        .filter(|post| page_of(post.url().as_str()) == wanted)
        .ok_or_else(|| not_found(&query.url))?;
    let site = store
        .site()
        .await
        .map_err(|error| Problem::internal(&error))?;
    let width = query
        .maxwidth
        .map_or(WIDTH, |most| most.clamp(NARROWEST, WIDTH));
    Ok(Json(card(&post, &site, width)).into_response())
}

/// The card of `post` on `site`, `width` pixels wide.
fn card(post: &Post, site: &SiteResources, width: u32) -> Embed {
    let (provider_name, provider_url) = site.settings().map_or_else(
        || (String::new(), String::new()),
        |settings| {
            (
                settings.title().to_owned(),
                settings.url().as_str().to_owned(),
            )
        },
    );
    let author = (post.authors().first()).and_then(|slug| {
        let same = |author: &&Author| {
            author.slug().as_str() == slug && author.language() == post.language()
        };
        site.authors().iter().find(same)
    });
    let image = |property: &str| image_meta(post, property);
    let pixels = |property: &str| image(property).and_then(|value| value.parse().ok());
    let url = escape(post.url().as_str());
    let title = escape(post.title());
    let excerpt =
        (post.excerpt()).map_or_else(String::new, |text| ["<p>", &escape(text), "</p>"].concat());
    let home = if provider_url.is_empty() {
        String::new()
    } else {
        let (home, name) = (escape(&provider_url), escape(&provider_name));
        format!("<p><a href=\"{home}\">{name}</a></p>")
    };
    let html = format!(
        "<blockquote class=\"khalid-embed\" cite=\"{url}\"><p><a href=\"{url}\">{title}</a></p>{excerpt}{home}</blockquote>"
    );
    Embed {
        r#type: "rich",
        version: "1.0",
        title: post.title().to_owned(),
        author_name: author.map(|author| author.name().to_owned()),
        author_url: author
            .and_then(|author| author.url())
            .map(|url| url.as_str().to_owned()),
        provider_name,
        provider_url,
        thumbnail_url: image("og:image"),
        thumbnail_width: pixels("og:image:width"),
        thumbnail_height: pixels("og:image:height"),
        html,
        width,
        height: None,
    }
}

/// The `content` of the Open Graph tag `property` in the head of the
/// post's page, when its pages have been made.
fn image_meta(post: &Post, property: &str) -> Option<String> {
    let parts = post.head()?.parts()?;
    parts.tags().iter().find_map(|tag| match tag {
        HeadElement::Meta {
            property: Some(name),
            content,
            ..
        } if name == property => Some(content.clone()),
        _ => None,
    })
}

/// `url` without its query or fragment, ending in `/`, so the address a
/// consumer copied with tracking tags still names its post.
fn page_of(url: &str) -> String {
    let page = url.split(['?', '#']).next().unwrap_or_default();
    if page.ends_with('/') {
        page.to_owned()
    } else {
        format!("{page}/")
    }
}

/// `text` safe inside an HTML attribute or element.
fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn no_url() -> Problem {
    let message = UserMessage::new(
        "The request named no post.",
        "oEmbed needs the address of the post to show as a card.",
        "Add ?url= with the post's full address, such as /api/oembed?url=https://example.com/blog/a-post/.",
    );
    Problem::new(ProblemType::InvalidQuery, &message).with_rule("url")
}

fn not_found(url: &str) -> Problem {
    let message = UserMessage::new(
        "No published post is at this address.",
        "Only a published post of this blog can be shown as a card.",
        "Check the address, which must be the post's full address on this blog.",
    );
    Problem::new(ProblemType::NotFound, &message).with_instance(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_address_with_tracking_tags_names_the_same_page() {
        let page = "https://example.com/blog/a/";
        for copied in [
            "https://example.com/blog/a",
            "https://example.com/blog/a/?utm_source=x",
            "https://example.com/blog/a#top",
        ] {
            assert_eq!(page_of(copied), page, "{copied}");
        }
    }

    #[test]
    fn markup_in_a_title_is_written_as_text() {
        assert_eq!(
            escape("<b>\"Tom's\" & co</b>"),
            "&lt;b&gt;&quot;Tom&#39;s&quot; &amp; co&lt;/b&gt;"
        );
    }
}
