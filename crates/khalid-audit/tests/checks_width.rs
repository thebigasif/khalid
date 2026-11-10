//! The fixed width check: a template override or a raw HTML block that
//! sets a pixel width wider than a phone screen.

pub mod support;

use khalid_audit::{Check, Thresholds};
use support::{HOME, crawled, page};

/// A head with nothing for the other checks to find.
const HEAD: &str = r#"<title>Why internal links matter</title>
<meta name="description" content="Internal links help readers find the rest of your site.">
<meta property="og:title" content="Why internal links matter">
<meta property="og:type" content="article">
<meta property="og:image" content="https://www.example.com/blog/share.png">
<meta property="og:url" content="https://www.example.com/blog/">
<meta name="twitter:card" content="summary">
<script type="application/ld+json">{"@type":"BlogPosting"}</script>"#;

/// A heading and `count` words, enough text for the other checks.
#[cfg(test)]
fn words(count: usize) -> String {
    format!("<h1>Links</h1><p>{}</p>", "word ".repeat(count))
}

/// The fixed width issues on one page with `head` and `body`.
#[cfg(test)]
async fn found(head: &str, body: &str) -> Vec<String> {
    let crawl = crawled(&[(HOME, page(head, body))]).await;
    khalid_audit::audit(&crawl, Thresholds::default())
        .iter()
        .filter(|issue| issue.check() == Check::FixedWidth)
        .map(|issue| issue.detail().to_owned())
        .collect()
}

#[tokio::test]
async fn a_pixel_width_wider_than_a_phone_is_named() {
    let cases = [
        (
            r#"<div style="width: 800px">Wide</div>"#,
            "a div with \"width: 800px\"",
        ),
        (
            r#"<section style="color: red; MIN-WIDTH:640PX !important">Wide</section>"#,
            "a section with \"min-width: 640px\"",
        ),
        (
            r#"<table width="900"><tr><td>Wide</td></tr></table>"#,
            "a table with width=\"900\"",
        ),
        (
            "<style>.wrapper { margin: 0 auto; width: 960px }</style>",
            "\".wrapper\" with \"width: 960px\" in a style block",
        ),
    ];
    for (markup, detail) in cases {
        let body = format!("{}{markup}", words(400));
        assert_eq!(found(HEAD, &body).await, [detail.to_owned()], "{markup}");
    }
}

#[tokio::test]
async fn a_style_block_in_the_head_is_read_too() {
    let head = format!("{HEAD}<style>/* the old site */ body {{ min-width: 1024px; }}</style>");
    assert_eq!(
        found(&head, &words(400)).await,
        ["\"body\" with \"min-width: 1024px\" in a style block".to_owned()]
    );
}

#[tokio::test]
async fn several_wide_parts_on_a_page_are_one_issue_naming_the_first() {
    let body = format!(
        r#"{}<div style="width:700px">One</div><table width="700"></table><p style="width: 400px">Two</p>"#,
        words(400)
    );
    assert_eq!(
        found(HEAD, &body).await,
        ["a div with \"width: 700px\", and 2 more".to_owned()]
    );
}

#[tokio::test]
async fn widths_that_fit_a_phone_are_fine() {
    let body = format!(
        r#"{}<div style="width: 320px">Narrow</div>
<div style="max-width: 1200px; width: 100%">Fluid</div>
<div style="width: 40rem">In rem</div>
<img src="/blog/a.png" alt="A photo" width="1600" height="900">
<iframe src="/blog/map/" title="A map" width="560" height="315"></iframe>
<table width="100%"></table>
<pre><code>.wrapper {{ width: 960px }}</code></pre>
<style>
@media (min-width: 900px) {{ .wrapper {{ width: 860px }} }}
@media print {{ body {{ width: 1000px }} }}
.card {{ max-width: 720px; width: 300px }}
</style>"#,
        words(400)
    );
    assert_eq!(found(HEAD, &body).await, Vec::<String>::new());
}
