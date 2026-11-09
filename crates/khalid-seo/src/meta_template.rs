//! Title and description templates with variables, such as `{title} {sep} {site}`.

use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};

use crate::{PageKind, SeoError, SeoPage, SeoSite};

/// A variable a template may use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Variable {
    Title,
    Site,
    Tagline,
    Sep,
    Excerpt,
    Category,
    Author,
    Date,
    Modified,
    Page,
    Term,
    TermDescription,
    SearchPhrase,
    CurrentYear,
    CurrentMonth,
    Tag,
    FocusKeyword,
}

impl Variable {
    const ALL: [Self; 17] = [
        Self::Title,
        Self::Site,
        Self::Tagline,
        Self::Sep,
        Self::Excerpt,
        Self::Category,
        Self::Author,
        Self::Date,
        Self::Modified,
        Self::Page,
        Self::Term,
        Self::TermDescription,
        Self::SearchPhrase,
        Self::CurrentYear,
        Self::CurrentMonth,
        Self::Tag,
        Self::FocusKeyword,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Site => "site",
            Self::Tagline => "tagline",
            Self::Sep => "sep",
            Self::Excerpt => "excerpt",
            Self::Category => "category",
            Self::Author => "author",
            Self::Date => "date",
            Self::Modified => "modified",
            Self::Page => "page",
            Self::Term => "term",
            Self::TermDescription => "term_description",
            Self::SearchPhrase => "searchphrase",
            Self::CurrentYear => "currentyear",
            Self::CurrentMonth => "currentmonth",
            Self::Tag => "tag",
            Self::FocusKeyword => "focuskw",
        }
    }

    fn value<'a>(self, site: &'a SeoSite, page: &'a SeoPage) -> Cow<'a, str> {
        let text = |value: Option<&'a String>| Cow::Borrowed(value.map_or("", String::as_str));
        let words = site.words_for(&page.language);
        let day = |time: Option<DateTime<Utc>>| {
            time.map_or(Cow::Borrowed(""), |time| {
                Cow::Owned(words.date(time.date_naive()))
            })
        };
        match self {
            Self::Title => Cow::Borrowed(&page.title),
            Self::Site => Cow::Borrowed(&site.name),
            Self::Tagline => Cow::Borrowed(&site.tagline),
            Self::Sep => Cow::Borrowed(&site.separator),
            Self::Excerpt => Cow::Borrowed(&page.excerpt),
            Self::Category => text(page.category.as_ref()),
            Self::Author => text(page.author.as_ref()),
            Self::Date => day(page.published),
            Self::Modified => day(page.modified.or(page.published)),
            Self::Page => (page.page_number)
                .map_or(Cow::Borrowed(""), |number| Cow::Owned(words.page(number))),
            Self::Term if matches!(page.kind, PageKind::Tag | PageKind::Category) => {
                Cow::Borrowed(page.term.as_deref().unwrap_or(&page.title))
            }
            Self::Term => Cow::Borrowed(""),
            Self::TermDescription => text(page.term_description.as_ref()),
            Self::SearchPhrase => text(page.search_phrase.as_ref()),
            Self::CurrentYear => site.today.map_or(Cow::Borrowed(""), |today| {
                Cow::Owned(today.format("%Y").to_string())
            }),
            Self::CurrentMonth => (site.today).map_or(Cow::Borrowed(""), |today| {
                Cow::Owned(words.month(today).to_owned())
            }),
            Self::Tag => text(page.tags.first()),
            Self::FocusKeyword => text(page.focus_keyword.as_ref()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Piece {
    Text(String),
    Var(Variable),
}

/// A title or description template: text with variables in braces.
///
/// The variables are `{title}`, `{site}`, `{tagline}`, `{sep}`, `{excerpt}`,
/// `{category}`, `{author}`, `{date}` (published), `{modified}` (last real
/// update, else published), `{page}` ("Page 2 of 5", empty on page 1),
/// `{term}` (the tag or category an archive lists), `{term_description}`,
/// `{searchphrase}`, `{currentyear}`, `{currentmonth}`, `{tag}` (the first
/// tag) and `{focuskw}` (the focus keyword). Dates and the page line use
/// the page language's [`TemplateWords`](crate::TemplateWords). A variable
/// with no value renders as nothing, and the result loses doubled spaces
/// and any separator left at either end.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaTemplate {
    pieces: Vec<Piece>,
    source: String,
}

impl MetaTemplate {
    /// Parses a template.
    ///
    /// # Errors
    ///
    /// [`SeoError::TemplateVariable`] for an unknown name in braces, and
    /// [`SeoError::TemplateBrace`] for a brace without its partner.
    pub fn parse(text: &str) -> Result<Self, SeoError> {
        let brace = || SeoError::TemplateBrace {
            template: text.to_owned(),
        };
        let mut pieces = Vec::new();
        let mut rest = text;
        while let Some(open) = rest.find(['{', '}']) {
            let (before, after) = rest.split_at(open);
            let after = after.strip_prefix('{').ok_or_else(brace)?;
            let (name, tail) = after.split_once('}').ok_or_else(brace)?;
            if name.contains('{') {
                return Err(brace());
            }
            let variable = Variable::ALL
                .into_iter()
                .find(|variable| variable.name() == name)
                .ok_or_else(|| SeoError::TemplateVariable {
                    template: text.to_owned(),
                    name: name.to_owned(),
                })?;
            if !before.is_empty() {
                pieces.push(Piece::Text(before.to_owned()));
            }
            pieces.push(Piece::Var(variable));
            rest = tail;
        }
        if !rest.is_empty() {
            pieces.push(Piece::Text(rest.to_owned()));
        }
        Ok(Self {
            pieces,
            source: text.to_owned(),
        })
    }

    /// The template as written.
    pub fn as_str(&self) -> &str {
        &self.source
    }

    /// A template built from known pieces, for the defaults.
    pub(crate) fn from_pieces(pieces: Vec<Piece>) -> Self {
        let source = pieces
            .iter()
            .map(|piece| match piece {
                Piece::Text(text) => text.clone(),
                Piece::Var(variable) => format!("{{{}}}", variable.name()),
            })
            .collect();
        Self { pieces, source }
    }

    /// Fills the variables for `page` on `site` and tidies the result: one
    /// space between words, and no separator at either end.
    pub fn render(&self, site: &SeoSite, page: &SeoPage) -> String {
        let mut raw = String::new();
        for piece in &self.pieces {
            match piece {
                Piece::Text(text) => raw.push_str(text),
                Piece::Var(variable) => raw.push_str(&variable.value(site, page)),
            }
        }
        let words: Vec<&str> = raw.split_whitespace().collect();
        let separator = site.separator.trim();
        let start = words.iter().position(|word| *word != separator);
        let end = words.iter().rposition(|word| *word != separator);
        match (start, end) {
            (Some(start), Some(end)) => words.get(start..=end).unwrap_or_default().join(" "),
            _ => String::new(),
        }
    }
}

impl FromStr for MetaTemplate {
    type Err = SeoError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text)
    }
}

impl fmt::Display for MetaTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.source)
    }
}
