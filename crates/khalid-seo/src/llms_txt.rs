//! `llms.txt`: a markdown guide to the blog for language models, in the
//! format proposed at llmstxt.org.

use std::fmt;

use khalid_core::AbsoluteUrl;

use crate::sitemap::is_listed;
use crate::{PageKind, SeoError, SeoPage, SeoSite, SitemapSection, UrlPath};

/// Where the file lives, inside the mount folder.
const LLMS_PATH: &str = "/llms.txt";

/// The address of `llms.txt`: in the mount folder.
///
/// # Errors
///
/// [`SeoError::Url`] when the address cannot be built.
pub fn llms_txt_url(site: &SeoSite) -> Result<AbsoluteUrl, SeoError> {
    site.url_for(&site.default_language, &UrlPath::try_new(LLMS_PATH)?)
}

/// The site's choices for `llms.txt`. By default the file is written and
/// lists every page search engines may list.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct LlmsTxtOptions {
    /// Whether the file is written at all.
    pub enabled: bool,
    /// Text after the summary, such as who writes the site and how to
    /// cite it.
    pub intro: Option<String>,
    /// Pages listed first, under "Key pages", in this order, each a path
    /// inside the mount folder such as `/about/`.
    pub key_pages: Vec<UrlPath>,
    /// Paths inside the mount folder whose pages are left out, such as
    /// `/tag/`. Every page at or below each path is left out.
    pub exclude: Vec<UrlPath>,
}

impl Default for LlmsTxtOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            intro: None,
            key_pages: Vec::new(),
            exclude: Vec::new(),
        }
    }
}

/// One link line: `- [title](url): description`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Link {
    title: String,
    url: AbsoluteUrl,
    description: String,
}

/// One `##` section.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Section {
    heading: &'static str,
    links: Vec<Link>,
}

/// The blog's `llms.txt`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmsTxt {
    title: String,
    summary: String,
    intro: Option<String>,
    sections: Vec<Section>,
}

impl LlmsTxt {
    /// The `llms.txt` for `site`: the site name, its tagline as the summary,
    /// the intro from [`LlmsTxtOptions`], then the key pages, then every
    /// other listed post and page with its excerpt, in the order given.
    /// Archives go under `Optional`, which the format marks as safe to skip.
    /// The home page is the file itself, so it is not listed, and pages
    /// under an excluded path are left out.
    ///
    /// # Errors
    ///
    /// [`SeoError::Url`] when a page's address cannot be built.
    pub fn for_site(site: &SeoSite, pages: &[SeoPage]) -> Result<Self, SeoError> {
        let options = &site.llms_txt;
        let at = |path: &UrlPath| site.url_for(&site.default_language, path);
        let key_urls: Vec<AbsoluteUrl> =
            options.key_pages.iter().map(at).collect::<Result<_, _>>()?;
        let excluded: Vec<AbsoluteUrl> =
            options.exclude.iter().map(at).collect::<Result<_, _>>()?;
        let section = |heading| Section {
            heading,
            links: Vec::new(),
        };
        let mut sections = vec![
            section("Key pages"),
            section("Posts"),
            section("Pages"),
            section("Optional"),
        ];
        let mut keys: Vec<(usize, Link)> = Vec::new();
        for page in pages
            .iter()
            .filter(|page| page.kind != PageKind::Home && is_listed(site, page))
        {
            let url = site.url_for(&page.language, &page.path)?;
            if excluded
                .iter()
                .any(|path| url.as_str().starts_with(path.as_str()))
            {
                continue;
            }
            let link = Link {
                title: page.title.clone(),
                description: page
                    .excerpt
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" "),
                url,
            };
            if let Some(order) = key_urls.iter().position(|key| *key == link.url) {
                keys.push((order, link));
                continue;
            }
            let slot = match SitemapSection::of(page.kind) {
                SitemapSection::Posts => 1,
                SitemapSection::Pages => 2,
                SitemapSection::Archives => 3,
            };
            if let Some(section) = sections.get_mut(slot) {
                section.links.push(link);
            }
        }
        keys.sort_by_key(|(order, _)| *order);
        if let Some(first) = sections.first_mut() {
            first.links = keys.into_iter().map(|(_, link)| link).collect();
        }
        sections.retain(|section| !section.links.is_empty());
        Ok(Self {
            title: site.name.clone(),
            summary: site.tagline.clone(),
            intro: options
                .intro
                .clone()
                .filter(|intro| !intro.trim().is_empty()),
            sections,
        })
    }
}

/// Writes text inside a markdown link label, with its brackets escaped.
fn write_label(f: &mut fmt::Formatter<'_>, text: &str) -> fmt::Result {
    for c in text.chars() {
        if matches!(c, '[' | ']' | '\\') {
            f.write_str("\\")?;
        }
        write!(f, "{c}")?;
    }
    Ok(())
}

impl fmt::Display for LlmsTxt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# {}", self.title)?;
        if !self.summary.is_empty() {
            writeln!(f, "\n> {}", self.summary)?;
        }
        if let Some(intro) = &self.intro {
            writeln!(f, "\n{}", intro.trim())?;
        }
        for section in &self.sections {
            writeln!(f, "\n## {}\n", section.heading)?;
            for link in &section.links {
                f.write_str("- [")?;
                write_label(f, &link.title)?;
                write!(f, "]({})", link.url)?;
                if !link.description.is_empty() {
                    write!(f, ": {}", link.description)?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
