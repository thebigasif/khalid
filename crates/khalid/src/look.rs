//! The template pack a site chose in `[template]` of `site.toml`: checked
//! against the packs in `khalid-theme` when the settings are read, and
//! handed to the renderer in its own shape (ADR "templates are built-in
//! packs").

use std::fs;
use std::path::Path;

use khalid_api::resources::{AdminTemplate, OptionTakes, ReaderDefault, TemplateOptionSpec};
use khalid_core::{Config, ConfigError, UserMessage, toml_location};
use khalid_render::{RenderCta, RenderPack, TemplateOption, Tokens};
use khalid_theme::{CheckedOption, Look, LookError, OptionKind};

use crate::{build_message, config_message};

/// The look of the `site` folder whose settings are `config`.
///
/// # Errors
///
/// The message for a `[template]` the packs refuse, with its line in
/// `site.toml`.
pub(crate) fn of(site: &Path, config: &Config) -> Result<Look, UserMessage> {
    Look::choose(config.template()).map_err(|error| message(site, &error))
}

/// `config` with its look's reader switches for the keys `[reader]`
/// leaves out, once the look is checked.
///
/// # Errors
///
/// The message for a `[template]` the packs refuse, with its line.
pub(crate) fn apply(site: &Path, config: Config) -> Result<Config, UserMessage> {
    let look = of(site, &config)?;
    Ok(config.with_reader_defaults(look.pack().reader))
}

/// The look's pack as the renderer takes it.
pub(crate) fn render_pack(look: &Look) -> RenderPack<'static> {
    let pack = look.pack();
    let option = |value: &CheckedOption| match value {
        CheckedOption::Switch(on) => TemplateOption::Switch(*on),
        CheckedOption::Number(number) => TemplateOption::Number(*number),
        CheckedOption::Word(word) => TemplateOption::Word(word.clone()),
        CheckedOption::Lines(lines) => TemplateOption::Lines(lines.clone()),
    };
    RenderPack {
        name: pack.name,
        templates: (pack.templates.iter())
            .map(|file| (file.name, file.source))
            .collect(),
        options: (look.options().iter())
            .map(|(key, value)| ((*key).to_owned(), option(value)))
            .collect(),
        cta: look.cta().map(|cta| RenderCta {
            label: cta.label.to_string(),
            href: cta.href.to_string(),
            note: cta.note.as_ref().map(ToString::to_string),
        }),
    }
}

/// The tokens of the `site` folder over its look's defaults.
///
/// # Errors
///
/// The message for a mistake in `theme/tokens.toml`.
pub(crate) fn tokens(site: &Path, look: &Look) -> Result<Tokens, UserMessage> {
    Tokens::load_over(site, look.pack().tokens).map_err(|error| build_message::tokens(&error))
}

/// Every built-in template as the Admin API lists it, read from the
/// engine's packs, so a template another builder adds is listed as well.
pub(crate) fn admin_templates() -> Vec<AdminTemplate> {
    let words = |words: &[&str]| words.iter().map(|word| (*word).to_owned()).collect();
    let takes = |kind| match kind {
        OptionKind::Switch { default } => OptionTakes::Switch { default },
        OptionKind::Choice { allowed, default } => OptionTakes::Choice {
            allowed: words(allowed),
            default: default.to_owned(),
        },
        OptionKind::Count {
            least,
            most,
            default,
        } => OptionTakes::Count {
            least,
            most,
            default,
        },
        OptionKind::Line { most } => OptionTakes::Line { most },
        OptionKind::Lines { most } => OptionTakes::Lines { most },
        OptionKind::Picks { allowed, default } => OptionTakes::Picks {
            allowed: words(allowed),
            default: words(default),
        },
    };
    let template = |pack: &khalid_theme::Pack| AdminTemplate {
        name: pack.name.to_owned(),
        summary: pack.summary.to_owned(),
        reader: (pack.reader.iter())
            .map(|(switch, on)| ReaderDefault {
                key: switch.key().to_owned(),
                on: *on,
            })
            .collect(),
        options: (pack.options.iter())
            .map(|spec| TemplateOptionSpec {
                key: spec.key.to_owned(),
                help: spec.help.to_owned(),
                takes: takes(spec.kind),
            })
            .collect(),
    };
    khalid_theme::packs()
        .iter()
        .map(|pack| template(pack))
        .collect()
}

/// The settings message for `error`, at its line of the `site` folder's
/// `site.toml`.
fn message(site: &Path, error: &LookError) -> UserMessage {
    let mut path = site.to_path_buf();
    path.push(Config::FILE_NAME);
    let text = fs::read_to_string(&path).unwrap_or_default();
    config_message::of(&ConfigError::Field {
        line: toml_location::line_of_key_in(&text, error.table(), error.key()),
        path,
        field: error.key().to_owned(),
        problem: error.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use khalid_render::{Renderer, stylesheet_with};

    use super::*;

    /// RULES.md BL-WEB-7: the public CSS stays under 20 KB compressed.
    const BUDGET_BYTES: usize = 20 * 1024;

    #[test]
    fn the_theme_offers_every_network_the_renderer_links_to() {
        assert_eq!(
            khalid_theme::slot::SHARE_NETWORKS,
            khalid_render::SHARE_NETWORKS
        );
    }

    #[test]
    fn every_pack_loads_compiles_and_ships_under_the_css_budget() {
        let site = tempfile::tempdir().unwrap();
        for pack in khalid_theme::packs() {
            let tokens = Tokens::load_over(site.path(), pack.tokens).unwrap();
            let css = stylesheet_with(&tokens, pack.stylesheet).unwrap();
            let bytes = css.brotli().len();
            assert!(bytes < BUDGET_BYTES, "{}: {bytes} bytes", pack.name);
            println!("{}: {bytes} bytes of CSS, compressed", pack.name);
            let chosen = RenderPack {
                name: pack.name,
                templates: (pack.templates.iter())
                    .map(|file| (file.name, file.source))
                    .collect(),
                options: BTreeMap::new(),
                cta: None,
            };
            let renderer = Renderer::with_pack(site.path(), &chosen);
            assert!(renderer.is_ok(), "{}", pack.name);
        }
    }

    #[test]
    fn every_class_a_pack_template_uses_has_a_rule() {
        let site = tempfile::tempdir().unwrap();
        for pack in khalid_theme::packs() {
            let tokens = Tokens::load_over(site.path(), pack.tokens).unwrap();
            let css = stylesheet_with(&tokens, pack.stylesheet).unwrap();
            let css = String::from_utf8(css.body().to_vec()).unwrap();
            for file in pack.templates {
                let classes = (file.source.split("class=\"").skip(1))
                    .filter_map(|attribute| attribute.split('"').next())
                    .flat_map(str::split_whitespace);
                for class in classes {
                    let needle = format!(".{class}");
                    let styled = css.match_indices(&needle).any(|(at, _)| {
                        let next = css.get(at + needle.len()..).and_then(|r| r.chars().next());
                        !next.is_some_and(|c| c.is_alphanumeric() || c == '-' || c == '_')
                    });
                    assert!(styled, "{}: .{class} in {}", pack.name, file.name);
                }
            }
        }
    }

    #[test]
    fn the_theme_offers_an_option_for_every_part_the_templates_read() {
        let offered: Vec<&str> = khalid_theme::parts::all()
            .iter()
            .map(|spec| spec.key)
            .collect();
        assert_eq!(offered, khalid_render::part_keys());
    }

    #[test]
    fn a_site_cannot_ask_for_more_picks_or_related_posts_than_the_pages_carry() {
        let khalid_theme::OptionKind::Count { most, .. } = khalid_theme::parts::picks(0).kind
        else {
            panic!("picks is not a count");
        };
        assert_eq!(usize::try_from(most).unwrap(), khalid_render::MOST_PICKS);
        let khalid_theme::OptionKind::Count { most, .. } =
            khalid_theme::parts::related_count(5).kind
        else {
            panic!("related_count is not a count");
        };
        assert_eq!(usize::try_from(most).unwrap(), khalid_render::MOST_RELATED);
    }
}
