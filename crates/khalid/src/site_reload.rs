//! Reading a site's settings again while `khalid serve` runs: `site.toml`,
//! `routes.toml` and every file under `theme/`, through the same steps as
//! at start, before the full render that follows a change to one of them.
//! A file that no longer reads keeps the last good settings, and `mount`
//! and `mode`, which shape the server's own routes, keep their start value.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::SystemTime;

use khalid_core::{Config, ConfigError, Environment, Mode, UserMessage};
use khalid_routes::Routes;

use super::{Settings, SitePages, Theme};
use crate::{config_message, look, report, routes_message, site_config};

/// The settings files at the top of the site folder. Every file under
/// [`THEME`] counts too.
const FILES: [&str; 2] = [Config::FILE_NAME, "routes.toml"];

/// The folder of the theme's tokens, stylesheet, fonts and templates.
const THEME: &str = "theme";

/// Each settings file's size and when it last changed.
type Stamp = BTreeMap<PathBuf, (u64, Option<SystemTime>)>;

/// What reading the settings again needs.
#[derive(Debug)]
pub(super) struct Reload {
    /// The environment the settings were read with at start.
    env: Environment,
    /// The `mount` and `mode` the server started with, which it keeps.
    start: (String, Mode),
    /// The settings files as the settings were last read.
    seen: Mutex<Stamp>,
}

impl SitePages {
    /// Reads the settings again, with `env` on top, before each full render
    /// that follows a change to one of their files. `khalid serve` calls it
    /// once, before the watch starts.
    pub(crate) fn reload_with(&self, env: Environment) {
        let config = &self.settings().config;
        let start = (config.mount().to_owned(), config.mode());
        let seen = Mutex::new(stamp(&self.0.folder));
        let _ = self.0.reload.set(Reload { env, start, seen });
    }

    /// Makes the pages with the settings in the files when one changed
    /// since the last look. A file that no longer reads keeps the last good
    /// settings, and one line says why, once for each change.
    pub(super) fn reload_if_changed(&self) {
        let (Some(reload), folder) = (self.0.reload.get(), &self.0.folder) else {
            return;
        };
        let now = stamp(folder);
        let mut seen = reload.seen.lock().unwrap_or_else(PoisonError::into_inner);
        if *seen == now {
            return;
        }
        *seen = now;
        drop(seen);
        let (settings, kept) = match reload.read(folder) {
            Ok(read) => read,
            Err(message) => return keeps_last_good(&message),
        };
        for name in kept {
            report::notice(&format!(
                "site.toml changed {name}, which khalid serve reads only when it starts, \
                 so it keeps the old value. Restart khalid serve to use the new one."
            ));
        }
        for name in start_only(&self.settings().config, &settings.config) {
            report::notice(&format!(
                "site.toml changed {name}, which khalid serve also reads when it starts. \
                 The pages use it now; restart khalid serve to apply it everywhere."
            ));
        }
        let mut running = (self.0.settings.write()).unwrap_or_else(PoisonError::into_inner);
        *running = Arc::new(settings);
    }
}

impl Reload {
    /// The settings in the files of the site `folder`, read and checked as
    /// at start, with the start's `mount` and `mode`, and the names of
    /// those two that the files change.
    fn read(&self, folder: &Path) -> Result<(Settings, Vec<&'static str>), UserMessage> {
        let mut config = site_config::load(folder, &self.env)?;
        let (mount, mode) = &self.start;
        let kept: Vec<&'static str> = [("mount", config.mount() != mount)]
            .into_iter()
            .chain([("mode", config.mode() != *mode)])
            .filter_map(|(name, changed)| changed.then_some(name))
            .collect();
        if !kept.is_empty() {
            config = self.keeping_start(folder)?;
        }
        let routes = Routes::load(folder).map_err(|error| routes_message::of(&error))?;
        let theme = Theme::read(folder, &config)?;
        let settings = Settings {
            config,
            routes,
            theme,
        };
        Ok((settings, kept))
    }

    /// `site.toml` in `folder`, read as at start but with the start's
    /// `mount` and `mode` in place of its own.
    fn keeping_start(&self, folder: &Path) -> Result<Config, UserMessage> {
        let mut path = folder.to_path_buf();
        path.push(Config::FILE_NAME);
        let unread = |source| {
            config_message::of(&ConfigError::Read {
                path: path.clone(),
                source,
            })
        };
        let text = fs::read_to_string(&path).map_err(unread)?;
        // The file was just read and checked whole, so it parses.
        let mut table: toml::Table = text.parse().unwrap_or_default();
        let (mount, mode) = &self.start;
        let mode = match mode {
            Mode::Server => "server",
            Mode::Static => "static",
            Mode::Headless => "headless",
        };
        table.insert("mount".to_owned(), mount.clone().into());
        table.insert("mode".to_owned(), mode.into());
        let config = Config::from_toml(&path, &table.to_string(), &self.env);
        let config = config.map_err(|error| config_message::of(&error))?;
        site_config::check(folder, &config)?;
        look::apply(folder, config)
    }
}

/// Says on one line what `message` says, the same words `khalid --site`
/// gives, and that readers keep the last good settings and pages.
pub(super) fn keeps_last_good(message: &UserMessage) {
    let (what, why, fix) = (message.what(), message.why(), message.fix());
    let keeps = "Until then, khalid serve keeps the last good settings and pages.";
    report::notice(&format!("{what} {why} {fix} {keeps}"));
}

/// The settings besides `mount` and `mode` that `khalid serve` wires into
/// its routes and jobs at start, named as `site.toml` writes them, that
/// differ between `was` and `now`.
fn start_only(was: &Config, now: &Config) -> Vec<&'static str> {
    let languages = (was.languages(), was.default_language());
    [
        ("address", was.address() != now.address()),
        (
            "languages",
            languages != (now.languages(), now.default_language()),
        ),
        ("[media]", was.media() != now.media()),
        ("[mail]", was.mail() != now.mail()),
        ("[alerts]", was.alerts() != now.alerts()),
        ("[code]", was.code() != now.code()),
        (
            "[search_console]",
            was.search_console() != now.search_console(),
        ),
        ("[visitor_log]", was.visitor_log() != now.visitor_log()),
        ("[ai]", was.ai() != now.ai()),
        (
            "ghost_content_api",
            was.ghost_content_api() != now.ghost_content_api(),
        ),
        (
            "preview_link_days",
            was.preview_link_lifetime() != now.preview_link_lifetime(),
        ),
    ]
    .into_iter()
    .filter_map(|(name, changed)| changed.then_some(name))
    .collect()
}

/// The size and change time of `site.toml`, `routes.toml` and every file
/// under `theme/` in `folder`. A file that is missing is not listed, so
/// adding or removing one is a change.
fn stamp(folder: &Path) -> Stamp {
    let mut stamp = Stamp::new();
    let mut theme = folder.to_path_buf();
    theme.push(THEME);
    let mut look = vec![theme];
    let files = FILES.iter().map(|name| {
        let mut file = folder.to_path_buf();
        file.push(name);
        file
    });
    let mut paths: Vec<PathBuf> = files.collect();
    while let Some(dir) = look.pop() {
        for entry in fs::read_dir(&dir).into_iter().flatten().flatten() {
            match entry.file_type() {
                Ok(kind) if kind.is_dir() => look.push(entry.path()),
                _ => paths.push(entry.path()),
            }
        }
    }
    for path in paths {
        if let Ok(meta) = fs::metadata(&path) {
            stamp.insert(path, (meta.len(), meta.modified().ok()));
        }
    }
    stamp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_theme_file_added_or_changed_changes_the_stamp() {
        let site = tempfile::tempdir().unwrap();
        let before = stamp(site.path());
        let mut fonts = site.path().to_path_buf();
        fonts.push("theme/fonts");
        fs::create_dir_all(&fonts).unwrap();
        fonts.push("inter.woff2");
        fs::write(&fonts, "font").unwrap();
        let added = stamp(site.path());
        assert_ne!(before, added);
        fs::write(&fonts, "a longer font").unwrap();
        assert_ne!(added, stamp(site.path()));
    }
}
