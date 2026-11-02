//! The blog the admin's page names, so its sign-in screen shows whose
//! admin it is: the name from `site.toml`, and the main site's logo from
//! theme sync when the admin may show it.

use std::path::Path;

use axum::Router;
use khalid_admin::Blog;
use khalid_core::{AbsoluteUrl, Config};

use crate::look;

/// The admin's routes for the `site` folder whose settings are `config`.
pub(crate) fn router(site: &Path, config: &Config) -> Router {
    let logo = logo(site, config);
    let blog = Blog {
        mount: config.mount(),
        name: config.name(),
        logo: logo.as_ref().map(AbsoluteUrl::as_str),
    };
    khalid_admin::router(khalid_admin::built(), blog)
}

/// The main site's logo when it sits on the blog's own host, the only
/// place the admin's policy takes pictures from before sign-in. A logo
/// elsewhere, or none, leaves the blog's name alone at the top.
fn logo(site: &Path, config: &Config) -> Option<AbsoluteUrl> {
    let look = look::of(site, config).ok()?;
    let logo = look::tokens(site, &look).ok()?.brand.logo?;
    (logo.host() == config.address().host()).then_some(logo)
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

    #[test]
    fn a_logo_on_the_blogs_own_host_is_named_and_one_elsewhere_is_not() {
        let site = tempfile::tempdir().unwrap();
        let mut theme = site.path().to_owned();
        theme.push("theme");
        std::fs::create_dir(&theme).unwrap();
        theme.push("tokens.toml");
        let tokens = "[brand]\nlogo = \"https://example.com/logo.svg\"\n";
        std::fs::write(&theme, tokens).unwrap();
        let shown = logo(site.path(), &config("https://example.com/"));
        let shown = shown.as_ref().map(AbsoluteUrl::as_str);
        assert_eq!(shown, Some("https://example.com/logo.svg"));
        assert_eq!(logo(site.path(), &config("https://other.example/")), None);
    }

    #[test]
    fn a_site_with_no_logo_names_none() {
        let site = tempfile::tempdir().unwrap();
        assert_eq!(logo(site.path(), &config("https://example.com/")), None);
    }
}
