//! The schema.org types a post may choose for itself in front matter, in
//! place of the ones its kind and collection give it.

use serde::Deserialize;

/// What kind of web page a post or page is, from `page_type:`, written as
/// schema.org names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub enum PageSchemaType {
    /// A plain page, the default.
    WebPage,
    /// A page of questions and answers the site wrote.
    #[serde(rename = "FAQPage")]
    FaqPage,
    /// A page that says how to reach the site.
    ContactPage,
    /// A page about the site or its people.
    AboutPage,
    /// One question with answers from readers.
    #[serde(rename = "QAPage")]
    QaPage,
    /// A page about one thing, such as a product.
    ItemPage,
    /// A page that lists other pages.
    CollectionPage,
}

impl PageSchemaType {
    /// The schema.org name, such as `FAQPage`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WebPage => "WebPage",
            Self::FaqPage => "FAQPage",
            Self::ContactPage => "ContactPage",
            Self::AboutPage => "AboutPage",
            Self::QaPage => "QAPage",
            Self::ItemPage => "ItemPage",
            Self::CollectionPage => "CollectionPage",
        }
    }
}

/// What kind of article a post is, from `article_type:`, written as
/// schema.org names it. It wins over the collection's default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub enum ArticleSchemaType {
    /// A blog post, the default.
    BlogPosting,
    /// A general article.
    Article,
    /// A news story, listed in the news sitemap for two days.
    NewsArticle,
    /// A technical guide or reference.
    TechArticle,
    /// A report, such as a study or a yearly review.
    Report,
}
