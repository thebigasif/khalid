//! `[seo.ai_page]` in `site.toml`: the AI info page's settings, with the
//! owner's own words per language (GEO P3, backlog row 25).

use std::path::Path;

use khalid_core::{Config, Environment, Locale};

const SITE: &str = r#"name = "Field Notes on SEO"
address = "https://www.example.com"
default_language = "en"
languages = ["en", "de"]
"#;

#[cfg(test)]
fn load(text: &str) -> Result<Config, String> {
    Config::from_toml(Path::new("site.toml"), text, &Environment::empty())
        .map_err(|error| error.to_string())
}

#[test]
fn a_site_without_the_table_has_no_words_per_language() {
    let config = load(SITE).unwrap();
    assert!(!config.ai_page().off);
    assert!(config.ai_page().text.is_empty());
}

#[test]
fn each_language_may_have_its_own_covers_and_cite() {
    let text = format!(
        "{SITE}\n[seo.ai_page]\ncovers = \"Plain guides to SEO.\"\n\n\
         [seo.ai_page.text.de]\ncovers = \"Verständliche Anleitungen zu SEO.\"\n\
         cite = \"Nennen Sie Field Notes und verlinken Sie den Beitrag.\"\n"
    );
    let config = load(&text).unwrap();
    let settings = config.ai_page();
    assert_eq!(settings.covers.as_deref(), Some("Plain guides to SEO."));
    let german = &settings.text[&Locale::try_new("de").unwrap()];
    assert_eq!(
        german.covers.as_deref(),
        Some("Verständliche Anleitungen zu SEO.")
    );
    assert_eq!(
        german.cite.as_deref(),
        Some("Nennen Sie Field Notes und verlinken Sie den Beitrag.")
    );
}

#[test]
fn an_unknown_key_in_a_language_is_refused() {
    let text = format!("{SITE}\n[seo.ai_page.text.de]\nheading = \"Über uns\"\n");
    let error = load(&text).unwrap_err();
    assert!(error.contains("heading"), "{error}");
}
