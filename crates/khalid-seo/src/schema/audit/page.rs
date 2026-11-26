//! [`PageMarkup`]: the JSON-LD and the visible text of a whole web page,
//! such as a site's old page before a move, checked with both reports.

use khalid_core::AbsoluteUrl;
use khalid_outbound::Outbound;

use super::{AuditContext, SchemaReports, schema_reports, tree};
use crate::CopySchemaError;
use crate::schema::copied::scripts;

/// The JSON-LD scripts of one page, as written, and the text a reader sees.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PageMarkup {
    scripts: Vec<String>,
    visible_text: String,
}

impl PageMarkup {
    /// Fetches `url` through `outbound` and reads its markup.
    ///
    /// # Errors
    ///
    /// [`CopySchemaError::Fetch`] when the request fails or is refused, such
    /// as an address inside the network; [`CopySchemaError::Status`] when
    /// the page answers with a status other than 2xx; and
    /// [`CopySchemaError::NoSchema`] when it holds no JSON-LD script.
    pub async fn fetch(outbound: &Outbound, url: &AbsoluteUrl) -> Result<Self, CopySchemaError> {
        let fetched = outbound
            .get(url)
            .await
            .map_err(|source| CopySchemaError::Fetch { source })?;
        let status = fetched.status();
        if !(200..300).contains(&status) {
            return Err(CopySchemaError::Status { status });
        }
        Self::from_html(&String::from_utf8_lossy(fetched.body())).ok_or(CopySchemaError::NoSchema)
    }

    /// The markup of the page `html`, or `None` when it has no JSON-LD
    /// script.
    pub fn from_html(html: &str) -> Option<Self> {
        let found: Vec<String> = scripts(html)
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .map(str::to_owned)
            .collect();
        (!found.is_empty()).then(|| Self {
            scripts: found,
            visible_text: visible_text(html),
        })
    }

    /// The JSON-LD scripts, as written, in page order.
    pub fn scripts(&self) -> &[String] {
        &self.scripts
    }

    /// The text a reader sees: the page without its tags, scripts and styles.
    pub fn visible_text(&self) -> &str {
        &self.visible_text
    }

    /// Both reports on every script of the page. Each finding's path starts
    /// with its script, such as `script 2: offers.price`, when the page has
    /// more than one. An `@id` any script defines counts as defined, and a
    /// script that is not JSON is a finding of its own.
    pub fn reports(&self) -> SchemaReports {
        let mut defined: Vec<String> = Vec::new();
        for text in &self.scripts {
            if let Ok(root) = tree::read(text) {
                let mut audit = tree::Audit::default();
                audit.root(&root);
                defined.extend(audit.defined().cloned());
            }
        }
        let context = AuditContext {
            visible_text: Some(&self.visible_text),
            known_ids: &defined,
        };
        let many = self.scripts.len() > 1;
        let mut all = SchemaReports::default();
        for (index, text) in self.scripts.iter().enumerate() {
            let label = format!("script {}", index + 1);
            let prefix = |path: String| match (many, path.is_empty()) {
                (false, _) => path,
                (true, true) => label.clone(),
                (true, false) => format!("{label}: {path}"),
            };
            match schema_reports(text, &context) {
                Ok(reports) => {
                    all.types.extend(reports.types);
                    for mut found in reports.schema_org {
                        found.path = prefix(found.path);
                        all.schema_org.push(found);
                    }
                    for mut found in reports.google {
                        found.path = prefix(found.path);
                        all.google.push(found);
                    }
                }
                Err(error) => all.schema_org.push(super::finding(
                    (super::SchemaReport::SchemaOrg, super::FindingLevel::Error),
                    &prefix(String::new()),
                    (
                        format!("A JSON-LD script on the page is not JSON: {error}."),
                        "Search engines skip a script they cannot read, so nothing in it counts.",
                        "Fix the place the message names, such as a missing comma or quote."
                            .to_owned(),
                    ),
                )),
            }
        }
        all
    }
}

/// The text of `html` a reader sees: tags, scripts, styles and comments
/// removed, the common entities decoded, and runs of space made one.
fn visible_text(html: &str) -> String {
    let lower = html.to_ascii_lowercase();
    let mut out = String::with_capacity(html.len() / 2);
    let mut at = 0;
    while let Some(open) = lower.get(at..).and_then(|rest| rest.find('<')) {
        let start = at.saturating_add(open);
        out.push_str(html.get(at..start).unwrap_or_default());
        out.push(' ');
        let rest = lower.get(start..).unwrap_or_default();
        let skip_to = ["<script", "<style", "<!--", "<noscript"]
            .iter()
            .zip(["</script>", "</style>", "-->", "</noscript>"])
            .find(|(opening, _)| rest.starts_with(**opening))
            .map(|(_, closing)| closing);
        let end = match skip_to {
            Some(closing) => rest.find(closing).map(|found| found + closing.len()),
            None => rest.find('>').map(|found| found + 1),
        };
        let Some(length) = end else {
            at = html.len();
            break;
        };
        at = start.saturating_add(length);
    }
    out.push_str(html.get(at..).unwrap_or_default());
    let decoded = out
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&pound;", "£")
        .replace("&euro;", "€");
    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::PageMarkup;

    const PAGE: &str = r#"<html><head><title>Soup kit</title>
      <script type="application/ld+json">{"@context": "https://schema.org", "@type": "Organization",
        "@id": "https://shop.example/#org", "name": "Soup Shop"}</script>
      <script type="application/ld+json">{"@type": "Product", "name": "Lentil soup kit",
        "brand": {"@id": "https://shop.example/#org"},
        "offers": {"@type": "Offer", "price": "29.99", "pricecurrency": "GBP"}}</script>
      <style>.price{color:red}</style></head>
      <body><h1>Lentil soup kit</h1><p>Only &pound;19.99 from Soup Shop.</p>
      <script>var price = "29.99";</script></body></html>"#;

    #[test]
    fn a_page_is_read_as_its_scripts_and_what_a_reader_sees() {
        let page = PageMarkup::from_html(PAGE).unwrap();
        assert_eq!(page.scripts().len(), 2);
        assert_eq!(
            page.visible_text(),
            "Soup kit Lentil soup kit Only £19.99 from Soup Shop."
        );
        assert_eq!(PageMarkup::from_html("<p>No schema</p>"), None);
    }

    #[test]
    fn each_finding_names_its_script_and_ids_join_across_scripts() {
        let reports = PageMarkup::from_html(PAGE).unwrap().reports();
        assert_eq!(reports.types, ["Organization", "Product"]);
        let paths: Vec<&str> = reports.schema_org.iter().map(|f| f.path.as_str()).collect();
        assert_eq!(paths, ["script 2: offers.pricecurrency"]);
        let unseen = reports
            .google
            .iter()
            .find(|f| f.path == "script 2: offers.price");
        assert!(unseen.is_some(), "{:?}", reports.google);
    }
}
