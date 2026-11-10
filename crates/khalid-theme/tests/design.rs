//! The Design screen's view of `theme/tokens.toml`: the values pages use
//! over the pack, and the file written back as it was read.

use khalid_theme::{DesignFile, Rgb, ThemeTokens, TokensError};

/// A synced file, with a comment, a locked link and a type scale the
/// screen does not edit.
const SYNCED: &str = "# Picked by hand for the launch.\nlocked = [\"colors.link\"]\n\n\
    [colors]\nbackground = \"#ffffff\"\nsurface = \"#f6f7f9\"\ntext = \"#1f2328\"\n\
    muted = \"#57606a\"\nlink = \"#b3261e\" # brand red\naccent = \"#0b57d0\"\n\
    border = \"#d0d7de\"\n\n[fonts]\nbody = \"Inter, sans-serif\"\nheading = \"Inter, sans-serif\"\n\
    mono = \"ui-monospace, monospace\"\n\n[[fonts.files]]\nfamily = \"Inter\"\nstyle = \"normal\"\n\
    weight = \"400\"\npath = \"inter-latin.woff2\"\n\n[shape]\nradius = \"0.375rem\"\n\
    measure = \"40rem\"\n\n[type]\nbody = \"1.0625rem\"\n";

/// A pack that sets its own link colour and corners.
const PACK: &str =
    "[colors]\nlink = \"#7a3e00\"\naccent = \"#7a3e00\"\n\n[shape]\nradius = \"2px\"\n";

#[cfg(test)]
fn hex(text: &str) -> Rgb {
    Rgb::from_css(text).unwrap()
}

#[test]
fn a_site_without_a_file_shows_the_pack_over_the_defaults() {
    let site = tempfile::tempdir().unwrap();
    let file = DesignFile::read(site.path()).unwrap();
    assert!(file.changed().is_none(), "no file yet");
    let tokens = file.effective(PACK).unwrap();
    assert_eq!(tokens.colors.link, hex("#7a3e00"), "the pack's link");
    assert_eq!(tokens.colors.text, ThemeTokens::default().colors.text);
}

#[test]
fn every_pack_gives_tokens_the_screen_can_show() {
    let file = DesignFile::from_toml("[colors]\nlink = \"#b3261e\"\n").unwrap();
    for pack in khalid_theme::packs() {
        let tokens = file.effective(pack.tokens());
        let tokens = tokens.unwrap_or_else(|error| panic!("{}: {error:?}", pack.name));
        assert_eq!(tokens.colors.link, hex("#b3261e"), "{}", pack.name);
    }
}

#[test]
fn a_partial_file_wins_over_the_pack_value_by_value() {
    let file = DesignFile::from_toml("[colors]\nlink = \"#b3261e\"\n").unwrap();
    let tokens = file.effective(PACK).unwrap();
    assert_eq!(tokens.colors.link, hex("#b3261e"), "the file's link");
    assert_eq!(tokens.colors.accent, hex("#7a3e00"), "the pack's accent");
    assert_eq!(tokens.shape.radius.as_str(), "2px", "the pack's corners");
}

#[test]
fn a_file_is_written_back_as_it_was_read() {
    let site = tempfile::tempdir().unwrap();
    let path = DesignFile::from_toml(SYNCED)
        .unwrap()
        .write(site.path())
        .unwrap();
    assert!(path.ends_with("theme/tokens.toml"), "{}", path.display());
    let read = DesignFile::read(site.path()).unwrap();
    assert!(read.changed().is_some());
    assert_eq!(read.to_toml(), SYNCED, "comments, order and [type] kept");
    let tokens = read.effective("").unwrap();
    assert_eq!(tokens.colors.link, hex("#b3261e"));
    assert_eq!(tokens.fonts.files.len(), 1);
}

#[test]
fn a_file_the_renderer_refuses_is_refused_with_its_line() {
    let error = DesignFile::from_toml("[colors]\nlink = \"#b3261e\"\nglow = \"#fff\"\n")
        .unwrap()
        .effective("")
        .unwrap_err();
    assert!(matches!(error, TokensError::Parse { .. }), "{error:?}");
    let error = DesignFile::from_toml("[colors\n").unwrap_err();
    assert!(
        matches!(error, TokensError::Parse { line: Some(1), .. }),
        "{error:?}"
    );
}
