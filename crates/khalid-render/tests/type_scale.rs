//! `[type]` and `[space]` in `theme/tokens.toml`: today's sizes as the
//! defaults, fluid sizes from a smallest and a largest, and the text
//! colour on the accent picked for contrast (templates spec 4.3).

use khalid_render::{RenderError, Tokens};

/// The custom properties of `tokens`, one `name: value` per line.
fn variables(tokens: &Tokens) -> String {
    tokens.to_css()
}

#[test]
fn the_defaults_are_todays_sizes() {
    let css = variables(&Tokens::default());
    for line in [
        "--text-body: clamp(1.0625rem, 1rem + 0.25vw, 1.1875rem);",
        "--text-h1: clamp(1.875rem, 1.4rem + 2vw, 2.75rem);",
        "--text-h2: clamp(1.4rem, 1.2rem + 1vw, 1.875rem);",
        "--text-h3: clamp(1.15rem, 1.05rem + 0.5vw, 1.375rem);",
        "--text-h4: 1em;",
        "--text-small: 0.9375rem;",
        "--leading: 1.6;",
        "--heading-weight: 700;",
        "--space: 0.5rem;",
        "--space-section: 2.5rem;",
        "--color-on-accent: #ffffff;",
    ] {
        assert!(css.contains(line), "{line} in {css}");
    }
}

#[test]
fn a_min_and_max_become_a_fluid_size_between_20_and_80rem() {
    let tokens = Tokens::from_toml(
        "[type]\nbody = \"1rem 1.25rem\"\nh1 = \"2.25rem 3.25rem\"\nh3 = \"1.25rem\"\n\
         h4 = \"1.125rem 1.125rem\"\nline_height = 1.65\nheading_weight = 800\n\
         [space]\nunit = \"0.25rem\"\nsection = \"3rem\"\n",
    )
    .unwrap();
    let css = variables(&tokens);
    for line in [
        "--text-body: clamp(1rem, 0.9167rem + 0.4167vw, 1.25rem);",
        "--text-h1: clamp(2.25rem, 1.9167rem + 1.6667vw, 3.25rem);",
        "--text-h3: 1.25rem;",
        "--text-h4: 1.125rem;",
        "--leading: 1.65;",
        "--heading-weight: 800;",
        "--space: 0.25rem;",
        "--space-section: 3rem;",
    ] {
        assert!(css.contains(line), "{line} in {css}");
    }
}

#[test]
fn each_wrong_value_is_refused_with_its_line() {
    for (text, wanted) in [
        ("[type]\nbody = \"16px\"\n", "a type size is one or two rem"),
        ("[type]\nh1 = \"3rem 2rem\"\n", "the smaller size first"),
        ("[type]\nh1 = \"1rem 2rem 3rem\"\n", "one or two rem sizes"),
        ("[type]\nline_height = 4.0\n", "a line height is a number"),
        ("[type]\nheading_weight = 750\n", "a weight is 100 to 900"),
        ("[space]\nsection = \"lots\"\n", "a length must be a number"),
        ("[type]\nsize = \"1rem\"\n", "unknown field `size`"),
    ] {
        let Err(RenderError::Tokens { line, source }) = Tokens::from_toml(text) else {
            panic!("{text} should be refused");
        };
        assert_eq!(line, Some(2), "{text}");
        let message = source.message();
        assert!(message.contains(wanted), "{text}: {message}");
    }
}

#[test]
fn text_on_a_light_accent_is_dark() {
    let tokens = Tokens::from_toml("[colors]\naccent = \"#ffd166\"\n").unwrap();
    assert!(variables(&tokens).contains("--color-on-accent: #000000;"));
}
