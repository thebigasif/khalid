//! The Design screen's changes to `theme/tokens.toml`: written in place,
//! a bad value refused, and a locked value kept by the next sync.

use khalid_theme::{DesignFile, Rgb, SiteTheme, ThemeTokens, TokenName, TokensError};

/// A synced file, with a comment, a locked link and a type scale the
/// screen does not edit.
const SYNCED: &str = "# Picked by hand for the launch.\nlocked = [\"colors.link\"]\n\n\
    [colors]\nbackground = \"#ffffff\"\nsurface = \"#f6f7f9\"\ntext = \"#1f2328\"\n\
    muted = \"#57606a\"\nlink = \"#b3261e\" # brand red\naccent = \"#0b57d0\"\n\
    border = \"#d0d7de\"\n\n[fonts]\nbody = \"Inter, sans-serif\"\nheading = \"Inter, sans-serif\"\n\
    mono = \"ui-monospace, monospace\"\n\n[[fonts.files]]\nfamily = \"Inter\"\nstyle = \"normal\"\n\
    weight = \"400\"\npath = \"inter-latin.woff2\"\n\n[shape]\nradius = \"0.375rem\"\n\
    measure = \"40rem\"\n\n[type]\nbody = \"1.0625rem\"\n";

#[cfg(test)]
fn hex(text: &str) -> Rgb {
    Rgb::from_css(text).unwrap()
}

#[test]
fn a_change_is_written_in_place_and_keeps_the_rest_of_the_file() {
    let mut file = DesignFile::from_toml(SYNCED).unwrap();
    file.set(TokenName::Text, "#111111").unwrap();
    file.set(TokenName::Heading, "Georgia, serif").unwrap();
    file.set_locked(&[TokenName::Link, TokenName::Text]);
    let text = file.to_toml();
    let kept = [
        "# Picked by hand for the launch.\nlocked = [",
        "link = \"#b3261e\" # brand red",
        "text = \"#111111\"",
        "[type]\nbody = \"1.0625rem\"",
        "path = \"inter-latin.woff2\"",
    ];
    for part in kept {
        assert!(text.contains(part), "{part} in {text}");
    }
    let tokens = ThemeTokens::from_toml(&text).unwrap();
    let locked = [TokenName::Text, TokenName::Link];
    assert_eq!(tokens.locked, locked, "in the order of the file");
    assert_eq!(tokens.fonts.heading.as_str(), "Georgia, serif");
}

#[test]
fn a_change_to_a_missing_file_writes_only_that_value() {
    let site = tempfile::tempdir().unwrap();
    let mut file = DesignFile::read(site.path()).unwrap();
    file.set(TokenName::Link, "#b3261e").unwrap();
    let logo = "https://example.com/logo.svg";
    file.set(TokenName::Logo, logo).unwrap();
    file.write(site.path()).unwrap();
    let read = DesignFile::read(site.path()).unwrap();
    let text = read.to_toml();
    assert!(text.starts_with("# The blog's colours"), "{text}");
    assert!(
        !text.contains("background"),
        "the pack keeps the rest: {text}"
    );
    let tokens = read.effective("").unwrap();
    assert_eq!(tokens.colors.link, hex("#b3261e"));
    assert_eq!(tokens.brand.logo.unwrap().as_str(), logo);
}

#[test]
fn an_empty_logo_takes_it_out_of_the_file() {
    let brand = "[brand]\nlogo = \"https://example.com/logo.svg\"\n\
        icon = \"https://example.com/i.png\"\n";
    let mut file = DesignFile::from_toml(brand).unwrap();
    file.set(TokenName::Logo, "").unwrap();
    let tokens = file.effective("").unwrap();
    assert!(tokens.brand.logo.is_none());
    assert!(tokens.brand.icon.is_some(), "the icon stays");
}

#[test]
fn a_bad_value_is_refused_and_names_its_token() {
    let mut file = DesignFile::from_toml(SYNCED).unwrap();
    let cases = [
        (TokenName::Link, "red"),
        (TokenName::Link, "#12345"),
        (TokenName::Body, "Inter; color: red"),
        (TokenName::Radius, "4 px"),
        (TokenName::Logo, "javascript:alert(1)"),
        (TokenName::Text, ""),
    ];
    for (token, value) in cases {
        match file.set(token, value) {
            Err(TokensError::Value { token: named, .. }) => assert_eq!(named, token, "{value}"),
            other => panic!("{value} for {token}: {other:?}"),
        }
    }
    assert_eq!(file.to_toml(), SYNCED, "nothing changed");
}

#[test]
fn a_locked_value_set_on_the_screen_survives_a_sync() {
    let mut file = DesignFile::from_toml(SYNCED).unwrap();
    file.set(TokenName::Accent, "#6b21a8").unwrap();
    file.set_locked(&[TokenName::Link, TokenName::Accent]);
    let mut tokens = ThemeTokens::from_toml(&file.to_toml()).unwrap();
    let fresh = ThemeTokens::from_theme(&SiteTheme::default());
    let diff = tokens.resync(&fresh).to_string();
    assert!(diff.contains("colors.accent stays #6b21a8"), "{diff}");
    assert_eq!(tokens.colors.accent, hex("#6b21a8"), "locked on the screen");
    assert_eq!(tokens.colors.link, hex("#b3261e"), "locked by hand");
    assert_eq!(tokens.colors.text, fresh.colors.text, "not locked");
}

#[test]
fn an_empty_locked_list_leaves_the_file() {
    let mut file = DesignFile::from_toml(SYNCED).unwrap();
    file.set_locked(&[]);
    assert!(!file.to_toml().contains("locked"));
}

#[test]
fn every_write_moves_the_time_past_the_copy_it_was_read_with() {
    let site = tempfile::tempdir().unwrap();
    DesignFile::from_toml(SYNCED)
        .unwrap()
        .write(site.path())
        .unwrap();
    for _ in 0..20 {
        let file = DesignFile::read(site.path()).unwrap();
        file.write(site.path()).unwrap();
        let after = DesignFile::read(site.path()).unwrap().changed();
        assert!(after > file.changed(), "{after:?} after {:?}", file.changed());
    }
}
