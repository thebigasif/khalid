//! A sync over the file the Design screen writes: only the values the
//! file sets, a pack under it, and every change made in place (backlog
//! row 13, plan 4.9).

use khalid_theme::{
    DesignFile, FontFileEntry, Rgb, SiteTheme, ThemeTokens, TokenChange, TokenName,
};

/// What the Design screen writes when a person changes two colours and
/// locks one, with a comment of their own.
const PARTIAL: &str = "locked = [\"colors.accent\"]\n\n[colors]\n\
    link = \"#b3261e\" # brand red\naccent = \"#6b21a8\"\n";

/// A template's tokens under the file.
const PACK: &str = "[colors]\nbackground = \"#fbf8f3\"\ntext = \"#2b2118\"\n";

#[cfg(test)]
fn hex(text: &str) -> Rgb {
    Rgb::from_css(text).unwrap()
}

/// The main site as a fresh sync reads it: a blue link and accent on the
/// default white page.
#[cfg(test)]
fn site() -> ThemeTokens {
    let mut site = ThemeTokens::from_theme(&SiteTheme::default());
    site.colors.link = hex("#2255aa");
    site.colors.accent = hex("#2255aa");
    site
}

#[test]
fn a_partial_file_takes_the_site_values_in_place_and_keeps_its_lock() {
    let mut file = DesignFile::from_toml(PARTIAL).unwrap();

    let (tokens, diff) = file.resync(PACK, &site()).unwrap();

    assert_eq!(tokens.colors.link, hex("#2255aa"), "not locked");
    assert_eq!(tokens.colors.accent, hex("#6b21a8"), "locked on the screen");
    assert_eq!(tokens.colors.background, site().colors.background);
    let text = file.to_toml();
    assert!(text.contains("link = \"#2255aa\" # brand red"), "{text}");
    assert!(text.contains("accent = \"#6b21a8\""), "{text}");
    assert!(text.starts_with("locked = [\"colors.accent\"]"), "{text}");
    assert!(
        !text.contains("[fonts]"),
        "fonts that match stay out: {text}"
    );
    let said = diff.to_string();
    assert!(
        said.contains("colors.link changed from #b3261e to #2255aa."),
        "{said}"
    );
    assert!(
        said.contains("colors.background changed from #fbf8f3 to #ffffff."),
        "{said}"
    );
    assert!(
        said.contains("colors.accent stays #6b21a8, because it is locked."),
        "{said}"
    );
}

#[test]
fn a_value_the_pack_sets_and_the_site_shares_is_left_to_the_pack() {
    let mut file = DesignFile::from_toml("").unwrap();
    let mut from = site();
    from.colors.background = hex("#fbf8f3");

    let (_, diff) = file.resync(PACK, &from).unwrap();

    let named: Vec<TokenName> = diff
        .changes()
        .iter()
        .filter_map(|change| match change {
            TokenChange::Updated { name, .. } => Some(*name),
            _ => None,
        })
        .collect();
    assert!(!named.contains(&TokenName::Background), "{named:?}");
    assert!(!file.to_toml().contains("background"), "{}", file.to_toml());
}

#[test]
fn a_logo_the_site_dropped_leaves_the_file_and_new_font_files_are_listed() {
    let brand = "[brand]\nlogo = \"https://example.com/old.svg\"\n";
    let mut file = DesignFile::from_toml(brand).unwrap();
    let mut from = site();
    from.fonts.files = vec![FontFileEntry {
        family: "Inter".to_owned(),
        style: "normal".to_owned(),
        weight: "400".to_owned(),
        path: "inter-latin.woff2".to_owned(),
        range: None,
        metrics: None,
    }];

    let (tokens, _) = file.resync("", &from).unwrap();

    assert!(tokens.brand.logo.is_none());
    assert_eq!(tokens.fonts.files, from.fonts.files);
    let again = DesignFile::from_toml(&file.to_toml()).unwrap();
    assert_eq!(again.effective("").unwrap().fonts.files, from.fonts.files);
}

#[test]
fn a_file_with_a_bad_value_is_refused_before_anything_changes() {
    let bad = "[colors]\nlink = \"red\"\n";
    let mut file = DesignFile::from_toml(bad).unwrap();

    assert!(file.resync("", &site()).is_err());
    assert_eq!(file.to_toml(), bad);
}
