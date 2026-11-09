//! Reading a site's settings again while `khalid serve` runs: `site.toml`,
//! `routes.toml` and every file under `theme/`, through the same steps as
//! at start, before the full render that follows a change to one of them.
//! A file that no longer reads keeps the last good settings, and `mount`
//! and `mode`, which shape the server's own routes, keep their start value.
//! Once new settings are in use, `site.changed` fires, and the admin's post
//! checks judge with them through [`LiveConfig`].

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError, RwLock};
use std::time::SystemTime;

use khalid_core::{Config, ConfigError, Environment, Mode, UserMessage};
use khalid_routes::Routes;

use super::{Settings, SitePages, Theme};
use crate::announce::SiteAnnounce;
use crate::settings_events::{self, SiteToml};
use crate::{config_message, look, report, routes_message, site_config};

/// The settings files at the top of the site folder. Every file under
/// [`THEME`] counts too.
const FILES: [&str; 2] = [Config::FILE_NAME, "routes.toml"];

/// The folder of the theme's tokens, stylesheet, fonts and templates.
const THEME: &str = "theme";

/// Each settings file's size and when it last changed.
type Stamp = BTreeMap<PathBuf, (u64, Option<SystemTime>)>;

/// The `site.toml` settings the pages are made with now, for the parts of
/// `khalid serve` outside the pages that follow a save too, such as the
/// admin's post checks. It holds the settings alone, never the pages.
#[derive(Debug, Clone)]
pub(crate) struct LiveConfig(Arc<RwLock<Arc<Settings>>>);

impl LiveConfig {
    /// A copy of the settings in use now.
    pub(crate) fn now(&self) -> Config {
        let settings = self.0.read().unwrap_or_else(PoisonError::into_inner);
        settings.config.clone()
    }
}

/// What reading the settings again needs.
#[derive(Debug)]
pub(super) struct Reload {
    /// The environment the settings were read with at start.
    env: Environment,
    /// The `mount` and `mode` the server started with, which it keeps.
    start: (String, Mode),
    /// The settings files as the settings were last read.
    seen: Mutex<Stamp>,
    /// The settings files and `site.toml` as the settings in use were read,
    /// which the next settings put in use are compared with.
    applied: Mutex<(Stamp, SiteToml)>,
    /// Told `site.changed` each time new settings are put in use.
    announce: Option<SiteAnnounce>,
}

impl SitePages {
    /// The settings the pages are made with, as they change.
    pub(crate) fn live_config(&self) -> LiveConfig {
        LiveConfig(Arc::clone(&self.0.settings))
    }

    /// Reads the settings again, with `env` on top, before each full render
    /// that follows a change to one of their files, and tells `announce`
    /// once they are in use. `khalid serve` calls it once, before the watch
    /// starts.
    pub(crate) fn reload_with(&self, env: Environment, announce: Option<SiteAnnounce>) {
        let config = &self.settings().config;
        let start = (config.mount().to_owned(), config.mode());
        let folder = &self.0.folder;
        let seen = stamp(folder);
        let applied = Mutex::new((seen.clone(), SiteToml::read(folder)));
        let seen = Mutex::new(seen);
        let reload = Reload {
            env,
            start,
            seen,
            applied,
            announce,
        };
        let _ = self.0.reload.set(reload);
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
        *seen = now.clone();
        drop(seen);
        let (settings, kept) = match reload.read(folder) {
            Ok(read) => read,
            Err(message) => return keeps_last_good(&message),
        };
        for name in &kept {
            report::notice(&format!(
                "site.toml changed {name}, which khalid serve reads only when it starts, \
                 so it keeps the old value. Restart khalid serve to use the new one."
            ));
        }
        // The settings read keep the start's `mount` and `mode`, so only
        // the settings wired in at start are named here.
        let wired = settings_events::start_only(&self.settings().config, &settings.config);
        for name in &wired {
            report::notice(&format!(
                "site.toml changed {name}, which khalid serve also reads when it starts. \
                 The pages use it now; restart khalid serve to apply it everywhere."
            ));
        }
        let mut running = (self.0.settings.write()).unwrap_or_else(PoisonError::into_inner);
        *running = Arc::new(settings);
        drop(running);
        let waiting: Vec<&str> = kept.into_iter().chain(wired).collect();
        reload.tell(folder, now, &waiting);
    }
}

impl Reload {
    /// Fires `site.changed` for the settings files of `folder` just put in
    /// use, stamped `now`, with `waiting` the settings that wait for a
    /// restart, and keeps them as the ones in use.
    fn tell(&self, folder: &Path, now: Stamp, waiting: &[&str]) {
        let toml = SiteToml::read(folder);
        let mut applied = (self.applied.lock()).unwrap_or_else(PoisonError::into_inner);
        let (was, before) = &*applied;
        let saved = (now.iter())
            .filter(|(path, stamp)| was.get(*path) != Some(*stamp))
            .chain(was.iter().filter(|(path, _)| !now.contains_key(*path)))
            .map(|(path, _)| path.clone())
            .collect();
        if let Some(announce) = &self.announce {
            settings_events::fire(announce, (folder, saved), (&toml, before), waiting);
        }
        let toml = toml.or(before);
        *applied = (now, toml);
    }

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
