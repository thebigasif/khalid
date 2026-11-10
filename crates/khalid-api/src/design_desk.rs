//! Where the Admin API's Design route reads and changes the site's
//! `theme/tokens.toml`, through `khalid-theme`'s one model of the file.

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use khalid_theme::{ContrastCheck, DesignFile, Rgb, ThemeTokens, TokenName, TokensError};

use crate::resources::{AdminDesign, ContrastPair, DesignChange, FontChoice};

/// The chosen template's tokens, in the form of `theme/tokens.toml`, read
/// each time so a template chosen on the Look screen counts at once.
pub type PackTokens = Arc<dyn Fn() -> &'static str + Send + Sync>;

/// A serif list every reader has, offered beside the system font.
const SERIF: &str = r#"Georgia, "Times New Roman", serif"#;

/// The site's `theme/tokens.toml` for the Design route. Changes wait for
/// each other, so two saves never mix.
#[derive(Clone)]
pub struct DesignDesk {
    site: PathBuf,
    pack: PackTokens,
    writing: Arc<Mutex<()>>,
}

impl fmt::Debug for DesignDesk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignDesk")
            .field("site", &self.site)
            .finish_non_exhaustive()
    }
}

/// Why the design could not be read or changed.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DesignError {
    /// The file changed after the copy the change was made on.
    #[error("theme/tokens.toml changed after the copy the change was made on")]
    Stale {
        /// When it last changed, or `None` when it is gone.
        updated_at: Option<DateTime<Utc>>,
    },
    /// The change names a token the file does not have.
    #[error("{name} is not a token in theme/tokens.toml")]
    UnknownToken {
        /// The name as it was sent.
        name: String,
    },
    /// The file could not be read or written, or holds a mistake.
    #[error(transparent)]
    Tokens(#[from] TokensError),
    /// The blocking task that does the work stopped.
    #[error("the design task stopped before it finished")]
    Stopped,
}

impl DesignDesk {
    /// The design of the site in `site`, over the tokens `pack` gives.
    pub fn new(site: &Path, pack: PackTokens) -> Self {
        Self {
            site: site.to_path_buf(),
            pack,
            writing: Arc::new(Mutex::new(())),
        }
    }

    /// The design as the site's pages use it now.
    ///
    /// # Errors
    ///
    /// Returns [`DesignError::Tokens`] when the file cannot be read or
    /// holds a mistake.
    pub async fn read(&self) -> Result<AdminDesign, DesignError> {
        self.blocking(|desk| desk.design(&DesignFile::read(&desk.site)?))
            .await
    }

    /// Applies `change` and writes the file, but only when it has not
    /// changed since the copy the change was made on (BL-API-7) and every
    /// value is one the renderer takes. Nothing is written otherwise.
    ///
    /// # Errors
    ///
    /// Returns [`DesignError::Stale`], [`DesignError::UnknownToken`], and
    /// [`DesignError::Tokens`] for a refused value or a failed write.
    pub async fn change(&self, change: DesignChange) -> Result<AdminDesign, DesignError> {
        self.blocking(move |desk| {
            let _writing = desk.writing.lock().unwrap_or_else(PoisonError::into_inner);
            let mut file = DesignFile::read(&desk.site)?;
            let updated_at = file.changed().map(moment);
            if updated_at != change.updated_at {
                return Err(DesignError::Stale { updated_at });
            }
            for (name, value) in &change.tokens {
                file.set(token(name)?, value)?;
            }
            if let Some(locked) = &change.locked {
                let names = locked.iter().map(|name| token(name));
                file.set_locked(&names.collect::<Result<Vec<_>, _>>()?);
            }
            file.effective((desk.pack)())?;
            file.write(&desk.site)?;
            desk.design(&DesignFile::read(&desk.site)?)
        })
        .await
    }

    /// `file` as the Admin API shows it.
    fn design(&self, file: &DesignFile) -> Result<AdminDesign, DesignError> {
        let tokens = file.effective((self.pack)())?;
        Ok(AdminDesign {
            tokens: TokenName::ALL
                .into_iter()
                .map(|name| (name.as_str().to_owned(), value(&tokens, name)))
                .collect::<BTreeMap<_, _>>(),
            locked: (tokens.locked.iter())
                .map(|name| name.as_str().to_owned())
                .collect(),
            fonts: fonts(&tokens),
            contrast: tokens.contrast_checks().iter().map(pair).collect(),
            updated_at: file.changed().map(moment),
        })
    }

    /// Runs `step` on a blocking thread, since it reads and writes files
    /// (RULES.md BL-RUN-7).
    async fn blocking<R: Send + 'static>(
        &self,
        step: impl FnOnce(&Self) -> Result<R, DesignError> + Send + 'static,
    ) -> Result<R, DesignError> {
        let desk = self.clone();
        tokio::task::spawn_blocking(move || step(&desk))
            .await
            .map_err(|_| DesignError::Stopped)?
    }
}

/// The token `name` names, such as `colors.link`.
fn token(name: &str) -> Result<TokenName, DesignError> {
    TokenName::try_from(name.to_owned()).map_err(|_| DesignError::UnknownToken {
        name: name.to_owned(),
    })
}

/// The value of `name` as the file writes it; an empty text for a logo or
/// icon that is not set.
fn value(tokens: &ThemeTokens, name: TokenName) -> String {
    let address = |url: Option<&khalid_core::AbsoluteUrl>| url.map(ToString::to_string);
    match name {
        TokenName::Logo => address(tokens.brand.logo.as_ref()).unwrap_or_default(),
        TokenName::Icon => address(tokens.brand.icon.as_ref()).unwrap_or_default(),
        name => tokens.value(name),
    }
}

/// The fonts a body or heading may use: the system font, a serif, each
/// family the site serves itself, and the lists the file names now.
fn fonts(tokens: &ThemeTokens) -> Vec<FontChoice> {
    let system = ThemeTokens::default().fonts.body;
    let mut choices = vec![
        ("System font".to_owned(), system.as_str().to_owned()),
        ("Serif".to_owned(), SERIF.to_owned()),
    ];
    for file in &tokens.fonts.files {
        let stack = format!("\"{}\", system-ui, sans-serif", file.family);
        choices.push((file.family.clone(), stack));
    }
    for stack in [&tokens.fonts.body, &tokens.fonts.heading] {
        let first = stack.as_str().split(',').next().unwrap_or_default();
        choices.push((
            first.trim().trim_matches('"').to_owned(),
            stack.as_str().to_owned(),
        ));
    }
    let mut fonts: Vec<FontChoice> = Vec::new();
    for (mut label, stack) in choices {
        if fonts.iter().any(|font| font.stack == stack) {
            continue;
        }
        if fonts.iter().any(|font| font.label == label) {
            label.push_str(", as written now");
        }
        fonts.push(FontChoice { label, stack });
    }
    fonts
}

/// One check as the Admin API shows it.
fn pair(check: &ContrastCheck) -> ContrastPair {
    ContrastPair {
        token: check.token.as_str().to_owned(),
        ground: check.ground.as_str().to_owned(),
        ratio: check.ratio,
        target: check.target,
        passes: check.passes,
        nearest: check.nearest.map(Rgb::to_hex),
    }
}

/// `time` as the Admin API writes it, to the nanosecond, so it compares
/// equal when it comes back.
fn moment(time: SystemTime) -> DateTime<Utc> {
    let since = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let seconds = i64::try_from(since.as_secs()).unwrap_or(i64::MAX);
    DateTime::from_timestamp(seconds, since.subsec_nanos()).unwrap_or_default()
}
