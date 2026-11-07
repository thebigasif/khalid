//! `khalid init --template`: the template a new site starts with, checked
//! against the same packs `[template]` in `site.toml` is checked against,
//! and the colours, fonts and shape it starts from (plan KTD10).

use khalid_core::UserMessage;
use khalid_theme::{Pack, ThemeTokens, packs};

/// The pack named `name`, or the message that lists every name.
///
/// # Errors
///
/// The message for a name no pack has, before anything is written.
pub(crate) fn find(name: &str) -> Result<&'static Pack, UserMessage> {
    packs()
        .iter()
        .copied()
        .find(|pack| pack.name == name)
        .ok_or_else(|| unknown(name))
}

/// The `[template]` table a site made with `pack` adds to `site.toml`.
pub(crate) fn table(pack: &Pack) -> String {
    format!("\n[template]\nname = {}\n", super::quoted(pack.name))
}

/// The text of `theme/tokens.toml` for a site in `pack`: the light defaults
/// with the pack's own values over them, so the new blog looks like the
/// template's demo until theme sync or a hand edit changes it.
///
/// # Errors
///
/// The message when the pack's values do not make valid tokens, which the
/// pack tests rule out for every built-in pack.
pub(crate) fn tokens(pack: &Pack) -> Result<String, UserMessage> {
    let defaults = ThemeTokens::default().to_toml();
    let parse = |text: &str| text.parse::<toml::Table>().map_err(|_| broken(pack));
    let mut merged = parse(&defaults)?;
    for (key, value) in parse(pack.tokens)? {
        match (merged.get_mut(&key), value) {
            (Some(toml::Value::Table(base)), toml::Value::Table(over)) => base.extend(over),
            (_, value) => {
                merged.insert(key, value);
            }
        }
    }
    let tokens = ThemeTokens::from_toml(&merged.to_string()).map_err(|_| broken(pack))?;
    Ok(tokens.to_toml())
}

/// The message for a template name no pack has.
fn unknown(name: &str) -> UserMessage {
    let names: Vec<&str> = packs().iter().map(|pack| pack.name).collect();
    UserMessage::new(
        format!("There is no template named {name}."),
        "A new site starts in one of the templates built into Khalid, so nothing was made.",
        format!(
            "Run `khalid init` again with --template set to one of: {}.",
            names.join(", ")
        ),
    )
}

/// The message when a built-in pack's colours cannot be read.
fn broken(pack: &Pack) -> UserMessage {
    UserMessage::new(
        format!(
            "Khalid could not read the colours of the {} template.",
            pack.name
        ),
        "This is a fault in Khalid itself, so nothing was made.",
        "Run `khalid init` without --template, and report the fault on GitHub.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_pack_gives_valid_tokens_with_its_own_values_on_top() {
        for pack in packs() {
            let text = tokens(pack).unwrap();
            let tokens = ThemeTokens::from_toml(&text).unwrap();
            let own: toml::Table = pack.tokens.parse().unwrap();
            if let Some(toml::Value::Table(colors)) = own.get("colors")
                && let Some(link) = colors.get("link").and_then(toml::Value::as_str)
            {
                assert_eq!(tokens.colors.link.to_hex(), link, "{}", pack.name);
            }
        }
    }

    #[test]
    fn classic_starts_from_the_defaults_and_names_itself() {
        let classic = khalid_theme::classic();
        assert_eq!(tokens(classic).unwrap(), ThemeTokens::default().to_toml());
        assert_eq!(table(classic), "\n[template]\nname = \"classic\"\n");
    }

    #[test]
    fn an_unknown_name_lists_every_template() {
        let message = find("velvet").unwrap_err().to_string();
        for pack in packs() {
            assert!(message.contains(pack.name), "{message}");
        }
    }
}
