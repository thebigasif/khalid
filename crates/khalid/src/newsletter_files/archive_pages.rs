//! The newsletter's page at `newsletter/` and a page for each past issue
//! at `newsletter/<slug>/`, from `content/newsletters/main/`: what readers
//! get, how often, a recent issue, the archive, and on a running server the
//! sign-up form. A site of plain files shows the same pages without the
//! form, since nothing could take the sign-up.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use khalid_content::{Gated, Wikilinks, render_framed};
use khalid_core::{Config, Mode, SitePath, UserMessage};
use khalid_newsletter::{ArchivedIssue, NewsletterPage, archive};
use khalid_render::{BodyHtml, PageBody, PageKind, RenderPage, Renderer};
use khalid_seo::PageKind as SeoKind;

use crate::build_pages::{LanguagePages, Page, meta, page, path_of};
use crate::post_lock::escaped;
use crate::{build_message, build_seo, definition_text, serve_message};

/// Where the newsletter's page sits under the default language's address.
pub(crate) const REST: &str = "newsletter/";

/// The main newsletter's folder in the site.
const MAIN: &str = "content/newsletters/main";

/// Adds the newsletter's page and one page per issue sent by `today` to
/// the default language of `languages`, when the site in `folder` has
/// `content/newsletters/main/page.md`. The words around them come from
/// the strings of `renderer`.
///
/// # Errors
///
/// What, why and fix when a file's markdown has a wrong block or an
/// address cannot be made from the site's settings.
pub(crate) fn add(
    folder: &Path,
    config: &Config,
    (renderer, today): (&Renderer, NaiveDate),
    languages: &mut [LanguagePages],
) -> Result<(), UserMessage> {
    let page_file = format!("{MAIN}/page.md");
    let Some(newsletter) = read(folder, &page_file).and_then(|text| NewsletterPage::read(&text))
    else {
        return Ok(());
    };
    let locale = config.default_language();
    let Some(language) = (languages.iter_mut()).find(|language| &language.site.locale == locale)
    else {
        return Ok(());
    };
    let words = Words::of(renderer, config)?;
    let form = sign_up(config, &words);
    let mut listed = Vec::new();
    for issue in archive(issues(folder), today) {
        let (page, shown) = issue_page(config, issue, &words, &form)?;
        language.pages.push(page);
        listed.push(shown);
    }
    let source = PathBuf::from(page_file);
    let text = markdown(config, &newsletter.body, &source)?;
    let description = definition_text::summary(&text);
    let html = [text, form, archive_html(&listed, &words)].concat();
    let title = &newsletter.title;
    language
        .pages
        .push(made(config, (REST, title, &description), html, &source)?);
    Ok(())
}

/// An issue as the newsletter's page lists it.
struct Shown {
    issue: ArchivedIssue,
    href: String,
    summary: String,
}

/// The page of `issue`, with the day it went out, a link back to every
/// issue and `form`, and the issue as the archive lists it.
fn issue_page(
    config: &Config,
    issue: ArchivedIssue,
    words: &Words,
    form: &str,
) -> Result<(Page, Shown), UserMessage> {
    let rest = format!("{REST}{}/", issue.slug);
    let source = PathBuf::from(format!("{MAIN}/issues/{}-{}.md", issue.date, issue.slug));
    let text = markdown(config, &issue.body, &source)?;
    let summary = definition_text::summary(&text);
    let locale = config.default_language();
    let mut body = format!(
        "<p class=\"issue-date\">{} <time datetime=\"{}\">{}</time></p>\n{text}",
        words.sent,
        issue.date,
        issue.date.format("%-d %B %Y"),
    );
    let _ = write!(
        body,
        "<p class=\"issue-back\"><a href=\"{}\">{}</a></p>\n{form}",
        path_of(config, locale, REST),
        words.all,
    );
    let page = made(config, (&rest, &issue.title, &summary), body, &source)?;
    let href = path_of(config, locale, &rest);
    Ok((
        page,
        Shown {
            issue,
            href,
            summary,
        },
    ))
}

/// The newest issue as a sample to read, then every issue, newest first,
/// or nothing before the first issue goes out.
fn archive_html(listed: &[Shown], words: &Words) -> String {
    let Some(newest) = listed.first() else {
        return String::new();
    };
    let mut html = format!(
        "<section class=\"issue-sample\" aria-labelledby=\"issue-sample\">\n<h2 id=\"issue-sample\">{}</h2>\n<h3><a href=\"{}\">{}</a></h3>\n<p>{}</p>\n<p><a class=\"button\" href=\"{}\">{}</a></p>\n</section>\n",
        words.recent,
        newest.href,
        escaped(&newest.issue.title),
        escaped(&newest.summary),
        newest.href,
        words.whole,
    );
    let _ = write!(
        html,
        "<section class=\"issue-archive\" aria-labelledby=\"issue-archive\">\n<h2 id=\"issue-archive\">{}</h2>\n<ol class=\"issue-list\" role=\"list\">\n",
        words.issues,
    );
    for shown in listed {
        let _ = writeln!(
            html,
            "<li><time datetime=\"{}\">{}</time> <a href=\"{}\">{}</a><p>{}</p></li>",
            shown.issue.date,
            shown.issue.date.format("%-d %B %Y"),
            shown.href,
            escaped(&shown.issue.title),
            escaped(&shown.summary),
        );
    }
    html.push_str("</ol>\n</section>\n");
    html
}

/// The words around the pages, in the site's main language, escaped.
struct Words {
    sent: String,
    all: String,
    recent: String,
    whole: String,
    issues: String,
    label: String,
    button: String,
}

impl Words {
    fn of(renderer: &Renderer, config: &Config) -> Result<Self, UserMessage> {
        let locale = config.default_language();
        let text = |key: &str| {
            (renderer.text(locale, key))
                .map(escaped)
                .ok_or_else(serve_message::internal)
        };
        Ok(Self {
            sent: text("newsletter_sent")?,
            all: text("newsletter_all")?,
            recent: text("newsletter_recent")?,
            whole: text("newsletter_whole")?,
            issues: text("newsletter_issues")?,
            label: text("newsletter_label")?,
            button: text("newsletter_button")?,
        })
    }
}

/// The sign-up form as the `::newsletter` block writes it, on a running
/// server that can send mail; nothing on any other site.
fn sign_up(config: &Config, words: &Words) -> String {
    if config.mode() != Mode::Server || config.mail().is_none() {
        return String::new();
    }
    let block = format!(
        "::newsletter{{label=\"{}\" button=\"{}\"}}\n",
        words.label, words.button
    );
    markdown(config, &block, Path::new(MAIN)).unwrap_or_default()
}

/// `body` as HTML in the site's main language, with the site's frames and
/// link rules.
fn markdown(config: &Config, body: &str, source: &Path) -> Result<String, UserMessage> {
    let frames = &config.media().embed_domains;
    let (locale, slugs) = (config.default_language(), config.slugs());
    let locked = Gated::Locked { back: None };
    let rendered = render_framed(body, locale, slugs, &Wikilinks::new(), locked, frames)
        .map_err(|error| build_message::block(source, &error))?;
    Ok(build_seo::with_link_rules(
        config,
        rendered.html,
        "newsletter",
    ))
}

/// The page at `rest` titled `title` and described by `description`, with
/// `body`, made from the file `source`.
fn made(
    config: &Config,
    (rest, title, description): (&str, &str, &str),
    body: String,
    source: &Path,
) -> Result<Page, UserMessage> {
    let locale = config.default_language();
    let path = path_of(config, locale, rest);
    let mut seo = build_seo::page(SeoKind::Page, locale, rest, title)?;
    description.clone_into(&mut seo.excerpt);
    let (title, description) = (title.to_owned(), description.to_owned());
    let meta = meta(config, title.clone(), description, path, PageKind::Website)?;
    let body = PageBody::Page(RenderPage {
        title,
        updated: None,
        body: BodyHtml::from_sanitised(body),
    });
    Ok(page(locale, config, rest, meta, body, seo).made_from(source))
}

/// The text of the file at `relative` in the site in `folder`, if any.
fn read(folder: &Path, relative: &str) -> Option<String> {
    let path = SitePath::within(folder, relative).ok()?;
    fs::read_to_string(path.as_path()).ok()
}

/// Every issue in `issues/` of the main newsletter's folder in the site
/// in `folder`, in name order. A file that is not an issue is skipped.
fn issues(folder: &Path) -> Vec<ArchivedIssue> {
    let relative = format!("{MAIN}/issues");
    let Some(entries) = SitePath::within(folder, &relative)
        .ok()
        .and_then(|path| fs::read_dir(path.as_path()).ok())
    else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();
    names.sort();
    names
        .iter()
        .filter_map(|name| {
            let text = read(folder, &format!("{relative}/{name}"))?;
            ArchivedIssue::read(name, &text)
        })
        .collect()
}
