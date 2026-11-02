//! How the breadcrumb trail reads, from `[seo.breadcrumbs]` in `site.toml`.

use serde::Deserialize;

/// How the breadcrumb trail above a page reads. Every key has a default,
/// so the table may be left out:
///
/// ```toml
/// [seo.breadcrumbs]
/// separator = "»"
/// home = "Home"
/// archive_prefix = "Archives for"
/// search_prefix = "You searched for"
/// bold_last = true
/// taxonomy = "tag"
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct BreadcrumbSettings {
    /// What stands between two steps. `/` by default.
    pub separator: String,
    /// The first step's name. `None` uses the site's name.
    pub home: Option<String>,
    /// Words before the last step on a tag, category, author or date page,
    /// such as "Archives for". Empty by default.
    pub archive_prefix: String,
    /// Words before the last step on the search page, such as "You
    /// searched for". Empty by default.
    pub search_prefix: String,
    /// Whether the last step, the page itself, is shown in bold.
    pub bold_last: bool,
    /// Which of a post's terms its trail passes through.
    pub taxonomy: BreadcrumbTaxonomy,
}

impl Default for BreadcrumbSettings {
    fn default() -> Self {
        Self {
            separator: "/".to_owned(),
            home: None,
            archive_prefix: String::new(),
            search_prefix: String::new(),
            bold_last: false,
            taxonomy: BreadcrumbTaxonomy::Category,
        }
    }
}

/// Which of a post's terms its trail passes through: `"category"`, `"tag"`
/// or `"none"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BreadcrumbTaxonomy {
    /// The post's category, the default.
    #[default]
    Category,
    /// The post's first tag.
    Tag,
    /// Neither: home, then the post.
    None,
}
