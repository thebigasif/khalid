//! The blog the admin's page names, so its sign-in screen shows whose
//! admin it is: the name from `site.toml`, and the main site's logo and
//! accent from theme sync when the admin may show them.

use std::path::Path;

use axum::Router;
use khalid_admin::Blog;
use khalid_core::{AbsoluteUrl, Config};
use khalid_render::{Color, Tokens};

use crate::look;

/// WCAG 2.2 AA asks text and buttons for this contrast against white,
/// the admin's own background.
const MINIMUM_CONTRAST: f64 = 4.5;

/// The admin's routes for the `site` folder whose settings are `config`.
pub(crate) fn router(site: &Path, config: &Config) -> Router {
    let tokens = tokens(site, config);
    let logo = tokens.as_ref().and_then(|tokens| logo(tokens, config));
    let accent = tokens.as_ref().and_then(accent);
    let blog = Blog {
        mount: config.mount(),
        name: config.name(),
        logo: logo.as_ref().map(AbsoluteUrl::as_str),
        accent: accent.as_ref().map(Color::as_str),
    };
    khalid_admin::router(khalid_admin::built(), blog)
}

/// The tokens of the site's look, or `None` when they cannot be read; the
/// admin then keeps its own name badge and colour.
fn tokens(site: &Path, config: &Config) -> Option<Tokens> {
    let look = look::of(site, config).ok()?;
    look::tokens(site, &look).ok()
}

/// The main site's logo when it sits on the blog's own host, the only
/// place the admin's policy takes pictures from before sign-in. A logo
/// elsewhere, or none, leaves the blog's name alone at the top.
fn logo(tokens: &Tokens, config: &Config) -> Option<AbsoluteUrl> {
    let logo = tokens.brand.logo.clone()?;
    (logo.host() == config.address().host()).then_some(logo)
}

/// The theme's accent when white text on it, and it on white, are easy to
/// read; a paler one leaves the admin's own blue.
fn accent(tokens: &Tokens) -> Option<Color> {
    let white = Color::try_from("#ffffff".to_owned()).ok()?;
    let accent = &tokens.colors.accent;
    (accent.contrast(&white) >= MINIMUM_CONTRAST).then(|| accent.clone())
}

#[cfg(test)]
mod tests {
    use khalid_core::Environment;

    use super::*;

    fn config(address: &str) -> Config {
        let text = format!(
            "name = \"Field notes\"\naddress = \"{address}\"\ndefault_language = \"en\"\nlanguages = [\"en\"]\n"
        );
        Config::from_toml(Path::new("site.toml"), &text, &Environment::empty()).unwrap()
    }

    /// A site folder whose `theme/tokens.toml` holds `text`.
    fn site_with(text: &str) -> tempfile::TempDir {
        let site = tempfile::tempdir().unwrap();
        let mut theme = site.path().to_owned();
        theme.push("theme");
        std::fs::create_dir(&theme).unwrap();
        theme.push("tokens.toml");
        std::fs::write(&theme, text).unwrap();
        site
    }

    #[test]
    fn a_logo_on_the_blogs_own_host_is_named_and_one_elsewhere_is_not() {
        let site = site_with("[brand]\nlogo = \"https://example.com/logo.svg\"\n");
        let here = config("https://example.com/");
        let tokens = tokens(site.path(), &here).unwrap();
        let shown = logo(&tokens, &here);
        let shown = shown.as_ref().map(AbsoluteUrl::as_str);
        assert_eq!(shown, Some("https://example.com/logo.svg"));
        assert_eq!(logo(&tokens, &config("https://other.example/")), None);
    }

    #[test]
    fn a_site_with_no_logo_names_none() {
        let site = tempfile::tempdir().unwrap();
        let here = config("https://example.com/");
        let tokens = tokens(site.path(), &here).unwrap();
        assert_eq!(logo(&tokens, &here), None);
    }

    #[test]
    fn a_dark_accent_is_named_and_a_pale_one_is_not() {
        let here = config("https://example.com/");
        let site = site_with("[colors]\naccent = \"#7c3aed\"\n");
        let dark = accent(&tokens(site.path(), &here).unwrap());
        assert_eq!(dark.as_ref().map(Color::as_str), Some("#7c3aed"));
        let site = site_with("[colors]\naccent = \"#fde047\"\n");
        assert_eq!(accent(&tokens(site.path(), &here).unwrap()), None);
    }
}
