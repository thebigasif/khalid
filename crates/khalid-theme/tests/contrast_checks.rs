//! The contrast of each colour pair the templates draw, as the Design
//! screen shows it beside each colour, with the nearest shade that passes.

use khalid_theme::{
    ContrastCheck, ContrastFix, MARK_CONTRAST, Rgb, TEXT_CONTRAST, ThemeTokens, TokenName,
};

#[cfg(test)]
fn hex(text: &str) -> Rgb {
    Rgb::from_css(text).unwrap()
}

#[cfg(test)]
fn check(checks: &[ContrastCheck], token: TokenName, ground: TokenName) -> ContrastCheck {
    *checks
        .iter()
        .find(|check| check.token == token && check.ground == ground)
        .unwrap_or_else(|| panic!("{token} on {ground}"))
}

#[test]
fn the_defaults_pass_every_pair_on_both_grounds() {
    let checks = ThemeTokens::default().contrast_checks();
    assert_eq!(checks.len(), 8, "four colours on two grounds");
    assert!(checks.iter().all(|check| check.passes), "{checks:?}");
    assert!(checks.iter().all(|check| check.nearest.is_none()));
    let text = check(&checks, TokenName::Text, TokenName::Background);
    assert_eq!(text.ratio, 15.7, "as the mockup shows, rounded down");
    assert_eq!(text.target, TEXT_CONTRAST);
    let accent = check(&checks, TokenName::Accent, TokenName::Surface);
    assert_eq!(
        accent.target, MARK_CONTRAST,
        "the accent marks, it is not text"
    );
}

#[test]
fn a_faint_link_fails_and_gets_the_nearest_shade_that_passes_on_both() {
    let mut tokens = ThemeTokens::default();
    tokens.colors.link = hex("#60a5fa");
    let checks = tokens.contrast_checks();
    let on_page = check(&checks, TokenName::Link, TokenName::Background);
    let on_panels = check(&checks, TokenName::Link, TokenName::Surface);
    assert!(!on_page.passes && !on_panels.passes);
    assert!(on_page.ratio < TEXT_CONTRAST, "{on_page:?}");
    let nearest = on_page.nearest.unwrap();
    assert_eq!(
        on_panels.nearest,
        Some(nearest),
        "one shade for both grounds"
    );
    // The same shade the sync's contrast pass picks.
    let fixes = tokens.clone().fix_contrast();
    assert_eq!(fixes.first().map(ContrastFix::after), Some(nearest));
    tokens.colors.link = nearest;
    let again = tokens.contrast_checks();
    let after = check(&again, TokenName::Link, TokenName::Surface);
    assert!(after.passes, "{after:?}");
}

#[test]
fn a_ratio_just_under_the_target_never_reads_as_passing() {
    let mut tokens = ThemeTokens::default();
    tokens.colors.muted = hex("#777777");
    let muted = check(
        &tokens.contrast_checks(),
        TokenName::Muted,
        TokenName::Background,
    );
    assert!(!muted.passes, "{muted:?}");
    assert!(muted.ratio < TEXT_CONTRAST, "{} rounds down", muted.ratio);
}
