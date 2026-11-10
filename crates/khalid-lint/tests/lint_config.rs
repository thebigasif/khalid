//! Severities from `lint.toml`, and mistakes in it named by line and column.

use khalid_lint::{LintConfig, LintError, RuleId, Severity, Thresholds};

#[test]
fn lint_toml_sets_a_severity_per_rule() {
    let config = LintConfig::parse("[rules]\nno-date = \"warn\"\nno-author = \"off\"\n").unwrap();
    assert_eq!(
        config.severity(RuleId::new("no-date")),
        Some(Severity::Warn)
    );
    assert_eq!(
        config.severity(RuleId::new("no-author")),
        Some(Severity::Off)
    );
    let tip = LintConfig::parse("[rules]\nno-date = \"info\"\n").unwrap();
    assert_eq!(tip.severity(RuleId::new("no-date")), Some(Severity::Info));
    assert_eq!(
        config.severity(RuleId::new("no-title")),
        None,
        "a rule the file does not name keeps its default"
    );
}

#[test]
fn an_empty_lint_toml_keeps_every_default() {
    assert_eq!(LintConfig::parse("").unwrap(), LintConfig::default());
}

#[test]
fn an_unknown_rule_names_its_line_and_column() {
    let error =
        LintConfig::parse("[rules]\nno-date = \"warn\"\n  no-dates = \"off\"\n").unwrap_err();
    assert!(
        matches!(&error, LintError::UnknownRule { rule, line: 3, column: 3 } if rule == "no-dates"),
        "{error:?}"
    );
    assert_eq!(
        error.to_string(),
        "lint.toml line 3, column 3: there is no rule called \"no-dates\""
    );
}

#[test]
fn an_unknown_severity_names_its_line() {
    let error = LintConfig::parse("[rules]\nno-date = \"error\"\n").unwrap_err();
    assert!(
        matches!(error, LintError::Syntax { line: 2, .. }),
        "{error:?}"
    );
}

#[test]
fn an_unknown_section_names_its_line() {
    let error = LintConfig::parse("[rules]\n\n[rulez]\n").unwrap_err();
    assert!(
        matches!(error, LintError::Syntax { line: 3, .. }),
        "{error:?}"
    );
}

#[test]
fn text_that_is_not_toml_names_its_line() {
    let error = LintConfig::parse("[rules]\nno-date = warn\n").unwrap_err();
    assert!(
        matches!(error, LintError::Syntax { line: 2, .. }),
        "{error:?}"
    );
}

#[test]
fn lint_toml_tunes_a_threshold() {
    let config = LintConfig::parse("[thresholds]\nsame-link-text-max = 5\n").unwrap();
    assert_eq!(config.thresholds().same_link_text_max, 5);
    assert_eq!(LintConfig::default().thresholds(), Thresholds::default());
    let config = LintConfig::parse("[thresholds]\nthin-post-words = 500\n").unwrap();
    assert_eq!(config.thresholds().thin_post_words, 500);
}

#[test]
fn an_unknown_threshold_names_its_line() {
    let error = LintConfig::parse("[thresholds]\n\nsame-link-max = 5\n").unwrap_err();
    assert!(
        matches!(error, LintError::Syntax { line: 3, .. }),
        "{error:?}"
    );
}

#[test]
fn a_negative_threshold_names_its_line() {
    let error = LintConfig::parse("[thresholds]\nsame-link-text-max = -1\n").unwrap_err();
    assert!(
        matches!(error, LintError::Syntax { line: 2, .. }),
        "{error:?}"
    );
}

#[test]
fn lint_toml_names_the_categories_that_need_a_reviewer() {
    let text = "[reviewers]\nrequired-in = [\"health\", \"money\"]\n";
    let config = LintConfig::parse(text).unwrap();
    let slugs: Vec<&str> = config
        .reviewers_required_in()
        .iter()
        .map(khalid_core::Slug::as_str)
        .collect();
    assert_eq!(slugs, ["health", "money"]);
    assert!(
        LintConfig::default().reviewers_required_in().is_empty(),
        "a site that lists no category needs no reviewer"
    );
}

#[test]
fn a_reviewer_category_that_is_not_a_slug_names_its_line() {
    let error = LintConfig::parse("[reviewers]\nrequired-in = [\"Health Tips\"]\n").unwrap_err();
    assert!(
        matches!(error, LintError::Syntax { line: 2, .. }),
        "{error:?}"
    );
    let error = LintConfig::parse("[reviewers]\nrequired = [\"health\"]\n").unwrap_err();
    assert!(
        matches!(error, LintError::Syntax { line: 2, .. }),
        "{error:?}"
    );
}
