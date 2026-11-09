//! `[privacy]` in `site.toml`: how long a reader's choice lasts, how Global
//! Privacy Control is read, whether choices are counted, an outside
//! consent tool, text overrides and the owner's contact details, and the
//! two hashes later parts key on (privacy plan U2, KTD3, KTD8).

use std::path::Path;

use khalid_core::{Config, ConsentCategory, Environment, GpcMode, Locale, Mode};

const SITE: &str = r#"name = "Field Notes on SEO"
address = "https://www.example.com"
default_language = "en"
languages = ["en", "de"]
"#;

/// The fixed list of embed providers, as `khalid-content` names them.
const EMBEDS: &[&str] = &[
    "YouTube",
    "Vimeo",
    "Spotify",
    "SoundCloud",
    "CodePen",
    "Loom",
];

/// The built-in banner text of this release, as the renderer would pass it.
const BUILT_IN: &str = "We would like to use optional cookies.";

const ANALYTICS: &str = r#"
[[code]]
head = "<script src='https://stats.example.net/a.js'></script>"
category = "analytics"
vendor = "Example Stats"
policy = "https://stats.example.net/privacy"
"#;

#[cfg(test)]
fn load(text: &str) -> Result<Config, String> {
    Config::from_toml(Path::new("site.toml"), text, &Environment::empty())
        .map_err(|error| error.to_string())
}

#[cfg(test)]
fn rev(text: &str) -> String {
    let config = load(text).unwrap();
    config.privacy().rev(config.code(), EMBEDS).to_string()
}

#[cfg(test)]
fn version(text: &str) -> String {
    let config = load(text).unwrap();
    config
        .privacy()
        .version(config.code(), EMBEDS, BUILT_IN)
        .to_string()
}

#[test]
fn an_empty_site_gets_the_defaults() {
    let config = load(SITE).unwrap();
    let privacy = config.privacy();
    assert_eq!(privacy.lifetime_months, 6);
    assert_eq!(privacy.gpc, GpcMode::Marketing);
    assert!(privacy.counts(Mode::Server));
    assert_eq!(privacy.external_tool, None);
    assert!(privacy.text.is_empty());
    assert_eq!(privacy.contact.email, None);
}

#[test]
fn reads_every_key() {
    let text = format!(
        "{SITE}\n[privacy]\nlifetime_months = 13\ngpc = \"reject-all\"\ncounts = false\n\
         external_tool = \"Cookiebot\"\n\n[privacy.text.de]\nbanner = \"Darf es Statistik sein?\"\n\
         marketing = \"Werbung\"\n\n[privacy.contact]\nname = \"Ines Weber\"\n\
         email = \"privacy@example.com\"\naddress = \"1 Main Street, Springfield\"\n"
    );
    let config = load(&text).unwrap();
    let privacy = config.privacy();
    assert_eq!(privacy.lifetime_months, 13);
    assert_eq!(privacy.gpc, GpcMode::RejectAll);
    assert!(!privacy.counts(Mode::Server));
    assert_eq!(privacy.external_tool.as_deref(), Some("Cookiebot"));
    let german = &privacy.text[&Locale::try_new("de").unwrap()];
    assert_eq!(german.banner.as_deref(), Some("Darf es Statistik sein?"));
    assert_eq!(german.marketing.as_deref(), Some("Werbung"));
    assert_eq!(german.analytics, None);
    assert_eq!(privacy.contact.name.as_deref(), Some("Ines Weber"));
    assert_eq!(
        privacy.contact.email.as_deref(),
        Some("privacy@example.com")
    );
}

#[test]
fn refuses_a_lifetime_outside_one_to_thirteen_months() {
    for months in [0, 14, 24] {
        let text = format!("{SITE}\n[privacy]\nlifetime_months = {months}\n");
        assert_eq!(
            load(&text).unwrap_err(),
            "site.toml, line 7, field \"lifetime_months\": a reader's choice lasts 1 to 13 months, so use a number from 1 to 13",
            "{months} months"
        );
    }
}

#[test]
fn refuses_counts_on_a_static_site() {
    let text = format!("mode = \"static\"\n{SITE}\n[privacy]\ncounts = true\n");
    assert_eq!(
        load(&text).unwrap_err(),
        "site.toml, line 8, field \"counts\": a static host sends the pages without Khalid, so there are no choices to count; set counts = false, or mode = \"server\""
    );
}

#[test]
fn a_static_site_counts_nothing_by_default() {
    let config = load(&format!("mode = \"static\"\n{SITE}")).unwrap();
    assert!(!config.privacy().counts(Mode::Static));
}

#[test]
fn refuses_a_blank_tool_and_an_email_without_an_at() {
    let text = format!("{SITE}\n[privacy]\nexternal_tool = \" \"\n");
    let error = load(&text).unwrap_err();
    assert!(
        error.contains("field \"external_tool\"") && error.contains("Cookiebot"),
        "{error}"
    );
    let text = format!("{SITE}\n[privacy.contact]\nemail = \"privacy.example.com\"\n");
    let error = load(&text).unwrap_err();
    assert!(
        error.contains("line 7, field \"email\"") && error.contains('@'),
        "{error}"
    );
}

#[test]
fn refuses_an_unknown_key_and_gpc_mode() {
    let error = load(&format!("{SITE}\n[privacy]\nkeep_addresses = true\n")).unwrap_err();
    assert!(error.contains("keep_addresses"), "{error}");
    let error = load(&format!("{SITE}\n[privacy]\ngpc = \"ignore\"\n")).unwrap_err();
    assert!(error.contains("field \"gpc\""), "{error}");
}

#[test]
fn there_are_five_fixed_categories_and_only_necessary_is_always_on() {
    let keys: Vec<&str> = ConsentCategory::ALL.iter().map(|c| c.key()).collect();
    assert_eq!(
        keys,
        [
            "necessary",
            "preferences",
            "analytics",
            "marketing",
            "external-media"
        ]
    );
    let optional: Vec<bool> = ConsentCategory::ALL
        .iter()
        .map(|c| c.is_optional())
        .collect();
    assert_eq!(optional, [false, true, true, true, true]);
}

#[test]
fn wording_changes_the_version_but_not_the_rev() {
    let plain = format!("{SITE}{ANALYTICS}");
    let worded = format!("{plain}\n[privacy.text.en]\nbanner = \"May we count visits?\"\n");
    assert_eq!(rev(&plain), rev(&worded));
    assert_ne!(version(&plain), version(&worded));
    let config = load(&plain).unwrap();
    let release = config
        .privacy()
        .version(config.code(), EMBEDS, "New built-in words.");
    assert_ne!(version(&plain), release.to_string());
}

#[test]
fn a_new_vendor_or_lifetime_changes_the_rev() {
    let plain = format!("{SITE}{ANALYTICS}");
    let vendor = format!(
        "{plain}\n[[code]]\nbody = \"<script src='https://ads.example.org/t.js'></script>\"\n\
         category = \"marketing\"\nvendor = \"Example Ads\"\n"
    );
    let lifetime = format!("{plain}\n[privacy]\nlifetime_months = 12\n");
    assert_ne!(rev(&plain), rev(&vendor));
    assert_ne!(rev(&plain), rev(&lifetime));
    assert_ne!(rev(&vendor), rev(&lifetime));
    assert_ne!(version(&plain), version(&lifetime));
}

#[test]
fn a_new_embed_in_a_post_leaves_the_rev_alone() {
    // External media always lists every provider Khalid can embed, so the
    // rev reads only settings: a site with no embed yet and one whose new
    // post shows a Loom recording share it.
    let before = rev(SITE);
    let after = rev(SITE);
    assert_eq!(before, after);
    let config = load(SITE).unwrap();
    let fewer = config.privacy().rev(config.code(), &EMBEDS[..5]);
    assert_ne!(before, fewer.to_string());
}

#[test]
fn no_tracking_code_leaves_the_rev_alone() {
    let quiet = format!(
        "{SITE}\n[[code]]\nhead = \"<script src='/counter.js'></script>\"\ncategory = \"no-tracking\"\nvendor = \"Own counter\"\n"
    );
    assert_eq!(rev(SITE), rev(&quiet));
}

#[test]
fn the_rev_is_short_and_the_version_longer() {
    let rev = rev(SITE);
    assert_eq!(rev.len(), 8, "{rev}");
    assert!(rev.chars().all(|c| c.is_ascii_hexdigit()), "{rev}");
    let version = version(SITE);
    assert_eq!(version.len(), 16, "{version}");
    // Kept in cookies and in the proof tables, so it must never move with
    // a Rust release: these are the values for the defaults today.
    assert_eq!(
        (rev.as_str(), version.as_str()),
        ("903f4ca1", "93071fba9b4f59b3")
    );
}
