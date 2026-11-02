//! More of each SEO plugin's fields read (Yoast package WP4): the Open
//! Graph and X cards, cornerstone or pillar posts, related keyphrases and
//! synonyms, and the breadcrumb title and schema types Khalid has no field
//! for, listed as not carried.

use std::fs;

use khalid_import::wordpress::{Export, SeoMeta, SeoPlugin, read_wxr};

#[cfg(test)]
fn export() -> Export {
    let dir = env!("CARGO_MANIFEST_DIR");
    let text = fs::read_to_string(format!("{dir}/fixtures/wordpress/seo/social.xml")).unwrap();
    read_wxr(&text).unwrap()
}

#[cfg(test)]
fn seo(id: u64) -> SeoMeta {
    let export = export();
    SeoMeta::read(export.item(id).unwrap(), &export.site)
}

#[cfg(test)]
fn set_aside(seo: &SeoMeta) -> Vec<(SeoPlugin, &str)> {
    let unread = seo.unread.iter();
    unread.map(|f| (f.plugin, f.key.as_str())).collect()
}

#[test]
fn yoast_cards_cornerstone_and_related_keyphrases_are_read() {
    let seo = seo(30);
    assert_eq!(
        seo.facebook.title.as_deref(),
        Some("Why rye bread needs time: the slow way")
    );
    assert_eq!(
        seo.facebook.description.as_deref(),
        Some("Rye wants a day, not an hour.")
    );
    assert_eq!(
        seo.facebook.image.as_deref(),
        Some("https://northfield.example.com/wp-content/uploads/rye-card.jpg")
    );
    assert_eq!(
        seo.x.title.as_deref(),
        Some("Rye takes a day &amp; a night")
    );
    assert!(seo.pillar);
    assert_eq!(
        seo.related_keywords,
        ["rye starter", "rye flour", "dark rye", "sourdough rye"]
    );
    assert_eq!(
        set_aside(&seo),
        [
            (SeoPlugin::Yoast, "_yoast_wpseo_bctitle"),
            (SeoPlugin::Yoast, "_yoast_wpseo_schema_page_type"),
            (SeoPlugin::Yoast, "_yoast_wpseo_schema_article_type"),
        ]
    );
}

#[test]
fn rank_math_cards_pillar_and_set_aside_fields_are_read() {
    let seo = seo(31);
    assert_eq!(
        seo.facebook.title.as_deref(),
        Some("Which flour makes the best loaf?")
    );
    assert_eq!(
        seo.x.description.as_deref(),
        Some("Protein is what counts.")
    );
    assert!(seo.x.title.is_none() && seo.x.image.is_none());
    assert!(seo.pillar);
    assert_eq!(seo.focus_keywords.len(), 6, "{:?}", seo.focus_keywords);
    assert_eq!(seo.related_keywords, Vec::<String>::new());
    assert_eq!(
        set_aside(&seo),
        [
            (SeoPlugin::RankMath, "rank_math_breadcrumb_title"),
            (SeoPlugin::RankMath, "rank_math_rich_snippet"),
        ]
    );
}

#[test]
fn all_in_one_seo_cards_and_additional_keyphrases_are_read() {
    let seo = seo(32);
    assert_eq!(
        seo.facebook.title.as_deref(),
        Some("Proofing baskets for round loaves")
    );
    assert_eq!(seo.x.description.as_deref(), Some("Cane or wood pulp?"));
    assert!(!seo.pillar);
    assert_eq!(seo.related_keywords, ["banneton", "brotform"]);
    assert!(seo.unread.is_empty(), "{:?}", seo.unread);
}

#[test]
fn a_card_part_the_plugin_left_out_stays_empty() {
    let seo = seo(32);
    assert!(seo.facebook.description.is_none() && seo.facebook.image.is_none());
    assert!(!seo.facebook.is_empty() && !seo.x.is_empty());
}
