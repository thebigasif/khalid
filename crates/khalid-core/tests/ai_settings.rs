//! `[ai]` in `site.toml`: off unless the owner names a provider, the key
//! only ever from the environment (Yoast package WP4).

use std::path::Path;

use khalid_core::{AiProvider, AiSettings, Config, Environment};

const SITE: &str = r#"name = "Field Notes on SEO"
address = "https://www.example.com"
default_language = "en"
languages = ["en"]
"#;

#[cfg(test)]
fn load(text: &str) -> Result<Config, String> {
    Config::from_toml(Path::new("site.toml"), text, &Environment::default())
        .map_err(|error| error.to_string())
}

#[test]
fn a_site_without_the_table_sends_nothing() {
    let ai = load(SITE).unwrap().ai().clone();
    assert_eq!(ai, AiSettings::default());
    assert_eq!(ai.provider, AiProvider::Off);
    assert!(!ai.is_on());
}

#[test]
fn reads_every_key() {
    let text = format!(
        "{SITE}\n[ai]\nprovider = \"openai\"\nmodel = \"small-model\"\nendpoint = \"https://ai.example.com/v1/chat/completions\"\n"
    );
    let config = load(&text).unwrap();
    let ai = config.ai();
    assert!(ai.is_on());
    assert_eq!(ai.provider, AiProvider::Openai);
    assert_eq!(ai.model.as_deref(), Some("small-model"));
    assert_eq!(
        ai.endpoint.as_ref().map(khalid_core::AbsoluteUrl::as_str),
        Some("https://ai.example.com/v1/chat/completions")
    );
}

#[test]
fn anthropic_and_serpdad_need_no_model() {
    for provider in ["anthropic", "serpdad"] {
        let text = format!("{SITE}\n[ai]\nprovider = \"{provider}\"\n");
        assert!(load(&text).unwrap().ai().is_on(), "{provider}");
    }
}

#[test]
fn an_openai_compatible_service_needs_a_model() {
    let text = format!("{SITE}\n[ai]\nprovider = \"openai\"\n");
    assert_eq!(
        load(&text).unwrap_err(),
        "site.toml, line 7, field \"provider\": an OpenAI-compatible service needs model, the model's name as the service spells it"
    );
}

#[test]
fn refuses_a_blank_model() {
    let text = format!("{SITE}\n[ai]\nprovider = \"anthropic\"\nmodel = \" \"\n");
    assert_eq!(
        load(&text).unwrap_err(),
        "site.toml, line 8, field \"model\": a model's name cannot be blank; remove the line to use the provider's usual model"
    );
}

#[test]
fn refuses_a_key_in_the_file() {
    let text = format!("{SITE}\n[ai]\nprovider = \"anthropic\"\nkey = \"made-up\"\n");
    let error = load(&text).unwrap_err();
    assert!(error.contains("unknown field `key`"), "{error}");
}

#[test]
fn the_key_comes_from_the_environment_and_is_hidden() {
    let env = Environment::from_pairs(&[(Environment::AI_KEY, " generated-for-tests ")]);
    let key = env.ai_key().unwrap();
    assert_eq!(key.expose(), "generated-for-tests");
    assert!(!format!("{env:?}").contains("generated-for-tests"));
    assert!(Environment::default().ai_key().is_none());
}
