//! What a person reads when the linter cannot start (BL-ERR-4).

use khalid_core::UserMessage;
use khalid_lint::LintError;

/// The message for `error`, in what, why and fix.
pub(crate) fn of(error: &LintError) -> UserMessage {
    let why = "Khalid cannot tell which rules to run until lint.toml is right.";
    match error {
        LintError::Syntax {
            line,
            column,
            source,
        } => UserMessage::new(
            format!("lint.toml has a mistake on line {line}, column {column}."),
            why,
            format!(
                "Change line {line} of lint.toml: {}.",
                source.message().trim_end_matches('.')
            ),
        ),
        LintError::UnknownRule { rule, line, .. } => UserMessage::new(
            format!("lint.toml names a rule that does not exist, `{rule}`, on line {line}."),
            why,
            format!("Change line {line} to the name of a rule, or remove it."),
        ),
        LintError::UnknownCategory {
            category,
            line,
            known,
        } => UserMessage::new(
            format!(
                "lint.toml asks for a reviewer in a category the site does not have, `{category}`, on line {line}."
            ),
            "No post is in that category, so the reviewer rule would check nothing.",
            match known.as_slice() {
                [] => format!(
                    "Add the category to categories.toml, or remove `{category}` from line {line} of lint.toml."
                ),
                known => format!(
                    "Change `{category}` on line {line} of lint.toml to one of the site's categories: {}.",
                    known.join(", ")
                ),
            },
        ),
        _ => UserMessage::new(
            "The linter could not start because its built-in messages are broken.",
            "This is a fault inside Khalid, not in your site.",
            "Report it at https://github.com/thebigasif/khalid/issues with the version number.",
        ),
    }
}

/// The message for a `lint.toml` that exists but cannot be read.
pub(crate) fn unreadable() -> UserMessage {
    UserMessage::new(
        "Khalid could not read lint.toml in the site folder.",
        "Without it, Khalid cannot tell which rules this site changes.",
        "Check that lint.toml is a readable file, or remove it to use the defaults.",
    )
}
