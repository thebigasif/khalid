//! The Admin API's core settings on a running site: `site.toml`, edited in
//! place so its comments and tables stay as written, checked as `khalid
//! serve` checks it at start before anything is written, and written
//! atomically (RULES.md BL-RUN-9). The running server keeps the settings
//! it started with, so a saved change applies after a restart.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use chrono::{DateTime, Utc};
use khalid_api::content_store::BoxFuture;
use khalid_api::entry_store::EntryStoreError;
use khalid_api::resources::{AdminSettings, SettingsChange, TemplateSetting, TemplateValue};
use khalid_api::settings_store::{SettingsError, SettingsStore};
use khalid_core::{Config, ConfigError, Environment, OptionValue};
use khalid_theme::Look;
use toml_edit::{Array, DocumentMut, Item, Table, Value};

use crate::atomic;
use crate::site_config;

/// The settings of one site folder. Cloning shares it.
#[derive(Debug, Clone)]
pub(crate) struct SiteSettings {
    site: PathBuf,
    /// The core settings the server started with.
    running: Core,
    /// Held while a change is checked and written, so no two interleave.
    writing: Arc<Mutex<()>>,
}

/// The core settings, as a change may set them.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Core {
    name: String,
    address: String,
    mount: String,
    default_language: String,
    languages: Vec<String>,
    template: TemplateSetting,
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
        }
    }
}

impl SiteSettings {
    /// The settings in `site`'s `site.toml`, for a server that started with
    /// `running`. Every step runs on a blocking thread (RULES.md BL-RUN-7).
    pub(crate) fn new(site: &Path, running: &Config) -> Self {
        Self {
            site: site.to_path_buf(),
            running: Core::of(running),
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
        let core = Core::of(&config);
        let restart_needed = core != self.running;
        Ok(AdminSettings {
            name: core.name,
            address: core.address,
            mount: core.mount,
            default_language: core.default_language,
            languages: core.languages,
            template: core.template,
            updated_at: changed,
            restart_needed,
        })
    }

    /// `text` as a config, when it passes every check made at start.
    fn checked(&self, text: &str) -> Result<Config, SettingsError> {
        let config = Config::from_toml(&self.path(), text, &Environment::empty());
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
