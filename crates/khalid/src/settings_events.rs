//! The `site.changed` webhook `khalid serve` fires once it has applied saved
//! settings (audit brief G25): a changed `site.toml`, `routes.toml` or file
//! under `theme/` (see `site_reload.rs`). The body names the files and the
//! `site.toml` settings that changed, without their values, which may be
//! private, and says whether some of them wait for a restart.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use khalid_core::Config;
use khalid_webhooks::Event;
use serde::Serialize;

use crate::announce::SiteAnnounce;
use crate::index;

/// The settings in `site.toml` as one reload read them: `None` when the
/// file could not be read or is not TOML, such as while it is half saved.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct SiteToml(Option<toml::Table>);

/// The body of `site.changed`, under `settings`.
#[derive(Debug, Serialize)]
struct Body {
    /// `site.toml` when its settings changed. Kept beside `files` for
    /// receivers made when only that file fired.
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<&'static str>,
    /// Every settings file that changed, relative to the site folder with
    /// `/` between folders, in name order.
    files: Vec<String>,
    /// The top-level settings and tables of `site.toml` that changed, such
    /// as `name` or `seo`, in name order.
    changed: Vec<String>,
    /// Whether some change waits for `khalid serve` to start again.
    restart_needed: bool,
}

impl SiteToml {
    /// The settings in the site `folder`'s `site.toml`. Reads the file.
    pub(crate) fn read(folder: &Path) -> Self {
        let mut file = folder.to_path_buf();
        file.push(Config::FILE_NAME);
        let text = fs::read_to_string(&file).ok();
        Self(text.and_then(|text| text.parse().ok()))
    }

    /// These settings when they were read, else `last`, so a file caught
    /// half saved is never what the next change is compared with.
    pub(crate) fn or(self, last: &Self) -> Self {
        if self.0.is_some() { self } else { last.clone() }
    }

    /// The names of the top-level settings that were added, changed or
    /// removed in `self` since `before`, or none when either side was not
    /// read.
    fn changed_since(&self, before: &Self) -> Vec<String> {
        let (Some(now), Some(was)) = (&self.0, &before.0) else {
            return Vec::new();
        };
        let names: BTreeSet<&String> = now.keys().chain(was.keys()).collect();
        names
            .into_iter()
            .filter(|name| now.get(*name) != was.get(*name))
            .cloned()
            .collect()
    }
}

/// Fires `site.changed` on `announce` for settings just applied: `saved`
/// holds the files of the site `folder` whose stamp changed since the last
/// applied settings, `toml` the `site.toml` settings now and then, and
/// `waiting` the settings that wait for a restart. `site.toml` counts only
/// when a setting in it changed, so a saved comment fires nothing. Nothing
/// fires when no file counts.
pub(crate) fn fire(
    announce: &SiteAnnounce,
    (folder, saved): (&Path, Vec<PathBuf>),
    (now, before): (&SiteToml, &SiteToml),
    waiting: &[&str],
) {
    if let Some(body) = body((folder, saved), (now, before), waiting) {
        announce.fire(Event::SiteChanged, BTreeMap::from([("settings", body)]));
    }
}

/// The body [`fire`] sends, or `None` when nothing counts.
fn body(
    (folder, saved): (&Path, Vec<PathBuf>),
    (now, before): (&SiteToml, &SiteToml),
    waiting: &[&str],
) -> Option<Body> {
    let restart_needed = !waiting.is_empty();
    let changed = now.changed_since(before);
    let relative = saved
        .iter()
        .filter_map(|path| path.strip_prefix(folder).ok());
    let named = relative.filter_map(index::slashed);
    let files: BTreeSet<String> = named
        .filter(|name| name != Config::FILE_NAME || !changed.is_empty())
        .collect();
    let file = files
        .contains(Config::FILE_NAME)
        .then_some(Config::FILE_NAME);
    (!files.is_empty()).then(|| Body {
        file,
        files: files.into_iter().collect(),
        changed,
        restart_needed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toml(text: &str) -> SiteToml {
        SiteToml(text.parse().ok())
    }

    const SITE: &str =
        "name = \"Field Notes\"\n[seo]\nindexnow = false\n[mail]\nfrom = \"a@example.com\"\n";

    fn saved(names: &[&str]) -> (&'static Path, Vec<PathBuf>) {
        let folder = Path::new("/site");
        let paths = names.iter().map(|name| {
            let mut path = folder.to_path_buf();
            path.push(name);
            path
        });
        (folder, paths.collect())
    }

    fn json(body: Option<Body>) -> serde_json::Value {
        serde_json::to_value(body.map(|body| BTreeMap::from([("settings", body)]))).unwrap()
    }

    #[test]
    fn the_demo_settings_are_read() {
        let mut folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        folder.push("../../fixtures/demo-site");
        let read = SiteToml::read(&folder);
        assert!(read.0.is_some_and(|table| table.contains_key("name")));
        let missing = tempfile::tempdir().unwrap();
        assert_eq!(SiteToml::read(missing.path()), SiteToml::default());
    }

    #[test]
    fn changed_names_each_added_changed_or_removed_setting_only() {
        let before = toml(SITE);
        let now = toml(
            "name = \"Field Notes\"\n# A comment.\n[seo]\nindexnow = true\n[search]\non = true\n",
        );
        assert_eq!(now.changed_since(&before), ["mail", "search", "seo"]);
        assert_eq!(before.changed_since(&toml(SITE)), Vec::<String>::new());
        let broken = toml("name = \"Field");
        assert_eq!(broken.changed_since(&before), Vec::<String>::new());
    }

    #[test]
    fn the_body_names_the_files_and_the_settings_but_no_value() {
        let now = toml("name = \"Field Notes Weekly\"\n[seo]\nindexnow = false\n");
        let files = saved(&["theme/site.css", "site.toml"]);
        let body = body(files, (&now, &toml(SITE)), &["[mail]"]);
        assert_eq!(
            json(body),
            serde_json::json!({"settings": {
                "file": "site.toml",
                "files": ["site.toml", "theme/site.css"],
                "changed": ["mail", "name"],
                "restart_needed": true,
            }})
        );
    }

    #[test]
    fn a_theme_or_routes_file_fires_without_site_toml() {
        let files = saved(&["routes.toml", "theme/tokens.toml"]);
        let body = body(files, (&toml(SITE), &toml(SITE)), &[]);
        assert_eq!(
            json(body),
            serde_json::json!({"settings": {
                "files": ["routes.toml", "theme/tokens.toml"],
                "changed": [],
                "restart_needed": false,
            }})
        );
    }

    #[test]
    fn a_site_toml_saved_with_no_setting_changed_fires_nothing() {
        let commented = toml(&format!("# A comment.\n{SITE}"));
        let body = body(saved(&["site.toml"]), (&commented, &toml(SITE)), &[]);
        assert_eq!(json(body), serde_json::Value::Null);
    }
}
