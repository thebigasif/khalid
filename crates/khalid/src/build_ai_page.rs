//! The AI info page (phase GEO, P3): who runs the site, what it covers,
//! how to cite it and how to reach the team, from the site's own settings,
//! in each language Khalid has the page's sentences for (backlog row 25).

use std::path::Path;

use chrono::NaiveDate;
use khalid_core::{Config, Locale, UserMessage};
use khalid_render::{BodyHtml, PageBody, PageKind, RenderPage};
use khalid_seo::PageKind as SeoKind;

use crate::build_pages::{Page, meta, page, path_of};
use crate::post_lock::escaped;
use crate::{build_lists, build_seo};

/// Where the page sits under each language's address.
pub(crate) const REST: &str = "ai/";

/// The page's built-in sentences in one language. `{who}` stands for the
/// site's publisher and `{contact}` for the contact line.
#[derive(Debug)]
struct Sentences {
    language: &'static str,
    /// The footer link's label.
    label: &'static str,
    title: &'static str,
    description: &'static str,
    runs: &'static str,
    covers: &'static str,
    cite: &'static str,
    reach: &'static str,
    contact: &'static str,
}

const ENGLISH: Sentences = Sentences {
    language: "en",
    label: "AI info",
    title: "About this site for readers and AI",
    description: "Who runs {who}, what it covers, how to cite it and how to reach the team.",
    runs: "{who} runs this site.",
    covers: "{who} publishes notes and guides.",
    cite: "Name {who} and link the post you quote.",
    reach: "To reach the team: {contact}.",
    contact: "the address in the site's contact page",
};

const GERMAN: Sentences = Sentences {
    language: "de",
    label: "KI-Info",
    title: "Über diese Website für Leser und KI",
    description: "Wer {who} betreibt, worüber die Website schreibt, wie man sie zitiert und wie man das Team erreicht.",
    runs: "{who} betreibt diese Website.",
    covers: "{who} veröffentlicht Notizen und Anleitungen.",
    cite: "Nennen Sie {who} und verlinken Sie den Beitrag, den Sie zitieren.",
    reach: "So erreichen Sie das Team: {contact}.",
    contact: "die Adresse auf der Kontaktseite der Website",
};

/// The sentences of `locale`'s copy, or `None` when the page has no copy
/// in that language. The main language always has one, in English when
/// Khalid has no sentences in it. Another language has one when Khalid
/// has its sentences and they differ from the main copy's, so no two
/// copies repeat the same title and description.
fn sentences(config: &Config, locale: &Locale) -> Option<&'static Sentences> {
    let own = |locale: &Locale| {
        [&ENGLISH, &GERMAN]
            .into_iter()
            .find(|words| words.language == locale.language())
    };
    let main = own(config.default_language()).unwrap_or(&ENGLISH);
    if locale == config.default_language() {
        return Some(main);
    }
    own(locale).filter(|words| words.language != main.language)
}

/// The footer link to the AI info page for readers of `locale`: the label
/// and the address of their language's copy, or of the main language's
/// when theirs has none. `None` when `[seo.ai_page] off = true`.
pub(crate) fn link(config: &Config, locale: &Locale) -> Option<(String, String)> {
    if config.ai_page().off {
        return None;
    }
    let main = config.default_language();
    let (at, words) = sentences(config, locale).map_or_else(
        || (main, sentences(config, main).unwrap_or(&ENGLISH)),
        |words| (locale, words),
    );
    Some((words.label.to_owned(), path_of(config, at, REST)))
}

/// The AI info page of `locale`, at `ai/`, in plain HTML that needs no
/// scripts, with its own `AboutPage` schema, updated `today`. `None` when
/// `[seo.ai_page] off = true`, or the page has no copy in `locale`. The
/// main copy reads `covers` and `cite` from `[seo.ai_page]`; every copy
/// reads them from `[seo.ai_page.text.<language>]`, and falls back to
/// built-in sentences in its own language.
pub(crate) fn ai_page(
    config: &Config,
    locale: &Locale,
    today: NaiveDate,
) -> Result<Option<Page>, UserMessage> {
    let settings = config.ai_page();
    let Some(words) = sentences(config, locale).filter(|_| !settings.off) else {
        return Ok(None);
    };
    let main = locale == config.default_language();
    let own = settings.text.get(locale);
    let publisher = config.seo().publisher.name.clone().unwrap_or_default();
    let who = if publisher.is_empty() {
        config.name().to_owned()
    } else {
        publisher
    };
    let fill = |sentence: &str| sentence.replace("{who}", &who);
    let tagline = &config.seo().titles.tagline;
    let covers = (main.then(|| settings.covers.clone()).flatten())
        .or_else(|| own.and_then(|text| text.covers.clone()))
        .or_else(|| (main && !tagline.is_empty()).then(|| tagline.clone()))
        .unwrap_or_else(|| fill(words.covers));
    let cite = (main.then(|| settings.cite.clone()).flatten())
        .or_else(|| own.and_then(|text| text.cite.clone()))
        .unwrap_or_else(|| fill(words.cite));
    let contact = (settings.contact.clone())
        .or_else(|| config.seo().publisher.email.clone())
        .unwrap_or_else(|| words.contact.to_owned());
    // The settings are plain text, so each is escaped into the page.
    let reach = words.reach.replace("{contact}", &escaped(&contact));
    let body = [
        format!("<p>{}</p>", escaped(&fill(words.runs))),
        format!("<p>{}</p>", escaped(&covers)),
        format!("<p>{}</p>", escaped(&cite)),
        format!("<p>{reach}</p>"),
    ]
    .join("\n");
    let title = words.title.to_owned();
    let description = fill(words.description);
    let path = path_of(config, locale, REST);
    let mut seo = build_seo::page(SeoKind::AiPage, locale, REST, &title)?;
    seo.excerpt.clone_from(&description);
    seo.translation = build_lists::list_key("ai")?;
    let meta = meta(config, title.clone(), description, path, PageKind::Website)?;
    let body = RenderPage {
        title,
        updated: Some(today),
        body: BodyHtml::from_sanitised(body),
        term: None,
    };
    Ok(Some(
        page(locale, config, REST, meta, PageBody::Page(body), seo)
            .made_from(Path::new("site.toml")),
    ))
}
