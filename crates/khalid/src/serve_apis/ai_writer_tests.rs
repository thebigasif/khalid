//! The writers against a stand-in server: each provider's request shape,
//! its answer read into options, and a refused key. Nothing here reaches
//! a real AI service.

use khalid_api::ai::{AiTask, Lengths};
use khalid_outbound::{Reach, Resolver};
use wiremock::matchers::{body_partial_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use super::*;

/// A key made up for these tests.
const KEY: &str = "generated-for-tests";

#[cfg(test)]
fn writer(provider: AiProvider, server: &MockServer, model: Option<&str>) -> HttpWriter {
    let resolver = Resolver::for_tests(Vec::new(), Reach::PublicAndLoopback);
    let outbound = Outbound::with_resolver(Limits::default(), resolver).unwrap();
    let mut settings = AiSettings::default();
    settings.provider = provider;
    settings.model = model.map(str::to_owned);
    settings.endpoint = Some(AbsoluteUrl::parse(&format!("{}/ask", server.uri())).unwrap());
    HttpWriter::new(&settings, Secret::new(KEY.to_owned()), outbound).unwrap()
}

#[cfg(test)]
fn ask(task: AiTask) -> AiAsk {
    let lengths = Lengths::default();
    let words = khalid_api::ai::AiWords::default();
    let mut ask = AiAsk::new(task, task.system(&lengths), task.prompt(&words));
    ask.count = 2;
    ask
}

#[tokio::test]
async fn anthropic_gets_the_key_header_and_its_text_is_split_into_options() {
    let server = MockServer::start().await;
    let answer =
        json!({"content": [{"type": "text", "text": "1. First title\n2. Second title\n3. Third"}]});
    Mock::given(method("POST"))
        .and(path("/ask"))
        .and(header("x-api-key", KEY))
        .and(header("anthropic-version", "2023-06-01"))
        .and(body_partial_json(json!({"model": "claude-haiku-5-5"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(answer))
        .expect(1)
        .mount(&server)
        .await;
    let options = writer(AiProvider::Anthropic, &server, None)
        .write(&ask(AiTask::SeoTitle))
        .await
        .unwrap();
    assert_eq!(options, ["First title", "Second title"]);
}

#[tokio::test]
async fn an_openai_compatible_service_gets_a_bearer_key_and_the_model() {
    let server = MockServer::start().await;
    let answer = json!({"choices": [{"message": {"content": "- \"A description\"\n- Another"}}]});
    Mock::given(method("POST"))
        .and(header("authorization", format!("Bearer {KEY}").as_str()))
        .and(body_partial_json(json!({"model": "small-model"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(answer))
        .mount(&server)
        .await;
    let options = writer(AiProvider::Openai, &server, Some("small-model"))
        .write(&ask(AiTask::MetaDescription))
        .await
        .unwrap();
    assert_eq!(options, ["A description", "Another"]);
}

#[tokio::test]
async fn serpdad_gets_the_task_and_answers_a_list() {
    let server = MockServer::start().await;
    let answer = json!({"suggestions": ["A chart of links", "Links between posts"]});
    Mock::given(method("POST"))
        .and(body_partial_json(json!({"task": "alt_text", "count": 2})))
        .respond_with(ResponseTemplate::new(200).set_body_json(answer))
        .mount(&server)
        .await;
    let options = writer(AiProvider::Serpdad, &server, None)
        .write(&ask(AiTask::AltText))
        .await
        .unwrap();
    assert_eq!(options, ["A chart of links", "Links between posts"]);
}

#[tokio::test]
async fn a_refused_key_and_an_empty_answer_are_told_apart() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(401))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"content": []})))
        .mount(&server)
        .await;
    let writer = writer(AiProvider::Anthropic, &server, None);
    let refused = writer.write(&ask(AiTask::SeoTitle)).await.unwrap_err();
    assert!(
        matches!(refused, AiError::Refused { status: 401 }),
        "{refused}"
    );
    let empty = writer.write(&ask(AiTask::SeoTitle)).await.unwrap_err();
    assert!(matches!(empty, AiError::Answer { status: 200 }), "{empty}");
}

#[test]
fn options_lose_their_numbers_bullets_and_quotes() {
    let text = "1. \"First\"\n2) Second\n\n- Third\n* Third\n\u{201c}Fifth\u{201d}";
    assert_eq!(
        options_from_text(text, 5),
        ["First", "Second", "Third", "Fifth"]
    );
    assert_eq!(
        options_from_text("10 ways to write anchor text", 5),
        ["10 ways to write anchor text"]
    );
}

#[test]
fn the_desk_is_off_without_the_table_and_waits_for_a_key() {
    let site = "name = \"Notes\"\naddress = \"https://www.example.com\"\ndefault_language = \"en\"\nlanguages = [\"en\"]\n";
    let load = |text: &str| {
        Config::from_toml(
            std::path::Path::new("site.toml"),
            text,
            &Environment::empty(),
        )
        .unwrap()
    };
    let cli = <Cli as clap::Parser>::parse_from(["khalid", "doctor"]);
    let off = desk(&load(site), &Environment::empty(), &cli);
    assert_eq!(format!("{off:?}"), format!("{:?}", AiDesk::off()));
    let on = load(&format!("{site}[ai]\nprovider = \"anthropic\"\n"));
    let waiting = desk(&on, &Environment::empty(), &cli);
    assert!(
        format!("{waiting:?}").contains("writer: false"),
        "{waiting:?}"
    );
    let env = Environment::from_pairs(&[(Environment::AI_KEY, KEY)]);
    let ready = desk(&on, &env, &cli);
    assert!(format!("{ready:?}").contains("writer: true"), "{ready:?}");
    assert!(!format!("{ready:?}").contains(KEY));
}
