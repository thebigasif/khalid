//! The Admin API's core settings on a running site: `site.toml`, edited in
//! place so its comments and tables stay as written, checked as `khalid
//! serve` checks it at start before anything is written, and written
//! atomically (RULES.md BL-RUN-9). `khalid serve` puts most saved settings
//! in use at once (see `site_reload.rs`); the ones it reads only at start
//! (see [`settings_events::start_only`]) wait for a restart.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use chrono::{DateTime, Utc};
use khalid_api::content_store::BoxFuture;
use khalid_api::entry_store::EntryStoreError;
use khalid_api::resources::{AdminSettings, SettingsChange, TemplateSetting, TemplateValue};
use khalid_api::settings_store::{SettingsError, SettingsStore};
use khalid_core::{Config, ConfigError, Environment, Mode, OptionValue};
use khalid_theme::Look;
use toml_edit::{Array, DocumentMut, Item, Table, Value};

use crate::atomic;
use crate::settings_events;
use crate::site_config;

mod cta;
mod home;
mod nav;
mod parts;
mod privacy;

/// The settings of one site folder. Cloning shares it.
#[derive(Debug, Clone)]
pub(crate) struct SiteSettings {
    site: PathBuf,
    /// The settings the server started with.
    start: Arc<Config>,
    /// The environment the server started with, which holds the keys a
    /// bucket or mail needs and may set the address. Never answered.
    env: Arc<Environment>,
    /// Held while a change is checked and written, so no two interleave.
    writing: Arc<Mutex<()>>,
}

/// The core settings, as a change may set them.
#[derive(Debug, Clone)]
struct Core {
    name: String,
    address: String,
    mount: String,
    default_language: String,
    languages: Vec<String>,
    template: TemplateSetting,
    mode: Mode,
}

impl Core {
    fn of(config: &Config) -> Self {
        let choice = config.template();
        // A table never reaches here: the look refuses it in `checked`.
        let value = |value: &OptionValue| match value {
            OptionValue::Switch(on) => Some(TemplateValue::Switch(*on)),
            OptionValue::Number(number) => Some(TemplateValue::Number(*number)),
            OptionValue::Word(word) => Some(TemplateValue::Word(word.clone())),
            OptionValue::Words(words) => Some(TemplateValue::Words(words.clone())),
            _ => None,
        };
        let options = (choice.options().iter())
            .filter_map(|(key, written)| Some((key.clone(), value(written)?)))
            .collect();
        Self {
            template: TemplateSetting {
                name: choice.name().to_string(),
                options,
            },
            name: config.name().to_owned(),
            address: config.address().as_str().to_owned(),
            mount: config.mount().to_owned(),
            default_language: config.default_language().as_str().to_owned(),
            languages: config
                .languages()
                .iter()
                .map(|l| l.as_str().to_owned())
                .collect(),
            mode: config.mode(),
        }
    }
}

impl SiteSettings {
    /// The settings in `site`'s `site.toml`, for a server that started with
    /// `start` read with `env` on top, as `site.toml` is read again here.
    /// Every step runs on a blocking thread (RULES.md BL-RUN-7).
    pub(crate) fn new(site: &Path, start: &Config, env: &Environment) -> Self {
        Self {
            site: site.to_path_buf(),
            start: Arc::new(start.clone()),
            env: Arc::new(env.clone()),
            writing: Arc::new(Mutex::new(())),
        }
    }

    fn path(&self) -> PathBuf {
        let mut path = self.site.clone();
        path.push(Config::FILE_NAME);
        path
    }

    /// The file's text and when it last changed.
    fn text(&self) -> Result<(String, DateTime<Utc>), EntryStoreError> {
        let path = self.path();
        let text = fs::read_to_string(&path).map_err(EntryStoreError::new)?;
        let changed = fs::metadata(&path)
            .and_then(|meta| meta.modified())
            .map_err(EntryStoreError::new)?;
        Ok((text, changed.into()))
    }

    /// The settings `text` holds, checked, as last changed at `changed`.
    fn settings(&self, text: &str, changed: DateTime<Utc>) -> Result<AdminSettings, SettingsError> {
        let config = self.checked(text)?;
        let restart_needed = !settings_events::start_only(&self.start, &config).is_empty();
        let core = Core::of(&config);
        Ok(AdminSettings {
            name: core.name,
            address: core.address,
            mount: core.mount,
            default_language: core.default_language,
            languages: core.languages,
            mode: mode_word(core.mode).to_owned(),
            template: core.template,
            cta: cta::cta(&config),
            seo: parts::table(text, "seo")?,
            privacy: privacy::privacy_table(text)?,
            code: privacy::code_entries(text)?,
            home_sections: home::sections(&config),
            nav: nav::menus(&config),
            updated_at: changed,
            restart_needed,
        })
    }

    /// `text` as a config, when it passes every check made at start.
    fn checked(&self, text: &str) -> Result<Config, SettingsError> {
        let config = Config::from_toml(&self.path(), text, &self.env);
        let config = config.map_err(refused)?;
        site_config::check(&self.site, &config).map_err(|message| SettingsError::Refused {
            field: "address".to_owned(),
            problem: message.what().to_owned(),
        })?;
        Look::choose(config.template()).map_err(|error| SettingsError::Refused {
            field: format!("{}.{}", error.table().trim_matches(['[', ']']), error.key()),
            problem: error.to_string(),
        })?;
        Ok(config)
    }

    fn read_now(&self) -> Result<AdminSettings, EntryStoreError> {
        let (text, changed) = self.text()?;
        self.settings(&text, changed).map_err(EntryStoreError::new)
    }

    fn change_now(&self, change: SettingsChange) -> Result<AdminSettings, SettingsError> {
        let _writing = self.lock();
        let (text, changed) = self.text()?;
        if changed != change.updated_at {
            return Err(SettingsError::Stale {
                updated_at: changed,
            });
        }
        let mut document = text.parse::<DocumentMut>().map_err(EntryStoreError::new)?;
        let texts = [
            ("name", change.name),
            ("address", change.address),
            ("mount", change.mount),
            ("default_language", change.default_language),
        ];
        for (key, sent) in texts {
            if let Some(sent) = sent {
                set(&mut document, key, sent.into());
            }
        }
        if let Some(languages) = change.languages {
            let languages: Array = languages.into_iter().collect();
            set(&mut document, "languages", languages.into());
        }
        if let Some(template) = change.template {
            set_template(&mut document, template);
        }
        if let Some(sent) = change.cta {
            cta::set_cta(&mut document, sent.into_setting());
        }
        if let Some(parts) = change.seo {
            parts::set_parts(&mut document, "seo", parts)?;
        }
        if let Some(parts) = change.privacy {
            privacy::set_privacy(&mut document, parts)?;
        }
        if let Some(sections) = change.home_sections {
            home::set_sections(&mut document, sections);
        }
        if let Some(menus) = change.nav {
            nav::set_menus(&mut document, menus);
        }
        if let Some(entries) = change.code {
            privacy::set_code(&mut document, entries)?;
        }
        let text = document.to_string();
        self.checked(&text)?;
        atomic::replace(&self.path(), &text).map_err(EntryStoreError::new)?;
        let (text, changed) = self.text()?;
        self.settings(&text, changed)
    }

    fn lock(&self) -> MutexGuard<'_, ()> {
        self.writing.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Runs `step` on a blocking thread, since it reads or writes files.
    fn blocking<R, E>(
        &self,
        step: impl FnOnce(&Self) -> Result<R, E> + Send + 'static,
    ) -> BoxFuture<'static, Result<R, E>>
    where
        R: Send + 'static,
        E: From<EntryStoreError> + Send + 'static,
    {
        let settings = self.clone();
        Box::pin(async move {
            tokio::task::spawn_blocking(move || step(&settings))
                .await
                .map_err(|error| E::from(EntryStoreError::new(error)))?
        })
    }
}

impl SettingsStore for SiteSettings {
    fn read(&self) -> BoxFuture<'_, Result<AdminSettings, EntryStoreError>> {
        self.blocking(Self::read_now)
    }

    fn change(
        &self,
        change: SettingsChange,
    ) -> BoxFuture<'_, Result<AdminSettings, SettingsError>> {
        self.blocking(move |settings| settings.change_now(change))
    }
}

/// The refusal a config error answers with, naming its field.
fn refused(error: ConfigError) -> SettingsError {
    match error {
        ConfigError::Field { field, problem, .. } => SettingsError::Refused { field, problem },
        ConfigError::Parse { field, source, .. } => SettingsError::Refused {
            field,
            problem: source.message().to_owned(),
        },
        error => SettingsError::Store(EntryStoreError::new(error)),
    }
}

/// The word `site.toml` writes for `mode`.
fn mode_word(mode: Mode) -> &'static str {
    match mode {
        Mode::Server => "server",
        Mode::Static => "static",
        Mode::Headless => "headless",
    }
}

/// Sets `[template]` to `template`: its name, and every option in place of
/// the old ones. `[template.cta]` and the table's place in the file stay.
fn set_template(document: &mut DocumentMut, template: TemplateSetting) {
    let mut table = match document.remove("template") {
        Some(Item::Table(table)) => table,
        Some(Item::Value(Value::InlineTable(inline))) => inline.into_table(),
        _ => Table::new(),
    };
    table.set_implicit(false);
    table.insert("name", Item::Value(template.name.into()));
    // The old options' table, emptied, keeps its place and comments.
    let old = table.remove("options");
    if !template.options.is_empty() {
        let mut options = match old {
            Some(Item::Table(mut old)) => {
                old.clear();
                old
            }
            _ => Table::new(),
        };
        for (key, value) in template.options {
            let value = match value {
                TemplateValue::Switch(on) => on.into(),
                TemplateValue::Number(number) => number.into(),
                TemplateValue::Word(word) => word.into(),
                TemplateValue::Words(words) => Value::Array(words.into_iter().collect()),
            };
            options.insert(&key, Item::Value(value));
        }
        table.insert("options", Item::Table(options));
    }
    document.insert("template", Item::Table(table));
}

/// Sets `key` to `new`, keeping the comments and spacing around the old
/// value, such as the comment block above the first key.
fn set(document: &mut DocumentMut, key: &str, new: Value) {
    match document.get_mut(key).and_then(Item::as_value_mut) {
        Some(old) => {
            let decor = old.decor().clone();
            *old = new;
            *old.decor_mut() = decor;
        }
        None => {
            document.insert(key, Item::Value(new));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    /// A bucket and mail through an SMTP server that signs in, whose keys
    /// and password live in the environment.
    const BUCKET_AND_MAIL: &str = "\n[media.s3]\nendpoint = \"https://s3.example.com\"\n\
        bucket = \"notes\"\nregion = \"auto\"\npublic_address = \"https://media.example.com/\"\n\n\
        [mail]\nfrom = \"news@example.com\"\n\n[mail.smtp]\nhost = \"smtp.example.com\"\n\
        username = \"news@example.com\"\n";

    /// The keys, password and address of a server started with them, all
    /// made for these tests.
    fn started_with() -> Environment {
        Environment::from_pairs(&[
            (Environment::S3_ACCESS_KEY_ID, "made-for-this-test"),
            (Environment::S3_SECRET_ACCESS_KEY, "made-for-this-test"),
            (Environment::SMTP_PASSWORD, "made-for-this-test"),
            (Environment::ADDRESS, "https://notes.example.org/"),
        ])
    }

    /// A folder whose `site.toml` is the demo site's and then `more`, and
    /// its settings for a server started on it with `env`.
    fn site_with(more: &str, env: &Environment) -> (tempfile::TempDir, SiteSettings) {
        let folder = tempfile::tempdir().unwrap();
        let mut demo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        demo.push("../../fixtures/demo-site/site.toml");
        let text = format!("{}{more}", fs::read_to_string(demo).unwrap());
        let mut file = folder.path().to_path_buf();
        file.push(Config::FILE_NAME);
        fs::write(&file, &text).unwrap();
        let start = Config::from_toml(&file, &text, env).unwrap();
        let settings = SiteSettings::new(folder.path(), &start, env);
        (folder, settings)
    }

    /// A change of the name alone, made on `read`.
    fn renamed(read: &AdminSettings) -> SettingsChange {
        let change = serde_json::json!({"name": "Notes", "updated_at": read.updated_at});
        serde_json::from_value(change).unwrap()
    }

    #[test]
    fn a_site_with_a_bucket_and_signed_in_mail_reads_and_saves_with_the_start_keys() {
        let (_folder, settings) = site_with(BUCKET_AND_MAIL, &started_with());
        let read = settings.read_now().unwrap();
        assert!(!read.restart_needed);
        let saved = settings.change_now(renamed(&read)).unwrap();
        assert_eq!(saved.name, "Notes");
        assert!(!saved.restart_needed);
    }

    #[test]
    fn the_address_from_the_environment_is_the_one_in_use_and_waits_for_nothing() {
        let (_folder, settings) = site_with("", &started_with());
        let read = settings.read_now().unwrap();
        assert_eq!(read.address, "https://notes.example.org/");
        assert!(!read.restart_needed);
        let change = serde_json::json!({"address": "https://www.example.net",
            "updated_at": read.updated_at});
        let saved = settings.change_now(serde_json::from_value(change).unwrap());
        assert!(!saved.unwrap().restart_needed, "the environment still wins");
    }

    #[test]
    fn a_start_only_setting_saved_still_waits_for_a_restart() {
        let (_folder, settings) = site_with("", &Environment::empty());
        let read = settings.read_now().unwrap();
        let change = serde_json::json!({"address": "https://www.example.net",
            "updated_at": read.updated_at});
        let saved = settings.change_now(serde_json::from_value(change).unwrap());
        assert!(saved.unwrap().restart_needed);
    }

    #[test]
    fn a_new_template_replaces_every_option_and_keeps_the_call_to_action() {
        let before = "name = \"Blog\"\n\n# The look.\n[template]\nname = \"trial\"\n\n\
                      [template.options]\nbanner = true\n\n\
                      [template.cta]\nlabel = \"Try it\"\nhref = \"/start/\"\n\n[nav]\nmain = []\n";
        let mut document: DocumentMut = before.parse().unwrap();
        let options = BTreeMap::from([("columns".to_owned(), TemplateValue::Number(3))]);
        let template = TemplateSetting {
            name: "sampler".to_owned(),
            options,
        };
        set_template(&mut document, template);
        let after = "name = \"Blog\"\n\n# The look.\n[template]\nname = \"sampler\"\n\n\
                     [template.options]\ncolumns = 3\n\n\
                     [template.cta]\nlabel = \"Try it\"\nhref = \"/start/\"\n\n[nav]\nmain = []\n";
        assert_eq!(document.to_string(), after);
    }
}
