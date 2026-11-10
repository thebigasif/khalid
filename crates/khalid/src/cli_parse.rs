//! What Khalid answers when `clap` stops before a command runs: the help,
//! the version, or a command line it could not read. Kept apart from
//! `main.rs` so that file stays under 600 lines (BL-SIZE-1).

use std::ffi::OsString;

use clap::error::ErrorKind;
use khalid_core::UserMessage;

use crate::cli::Output;
use crate::exit::Exit;
use crate::report;

/// Answers `--help`, `--version` and command lines `clap` could not read.
pub(crate) fn parse_failure(error: &clap::Error, output: Output) -> Exit {
    match error.kind() {
        ErrorKind::DisplayHelp => report::text(&error.render().to_string(), Exit::Success),
        ErrorKind::DisplayVersion => report::version(output),
        _ => report::failure(output, &usage_message(error), Exit::Usage),
    }
}

/// Turns a `clap` error into what, why and fix.
fn usage_message(error: &clap::Error) -> UserMessage {
    let rendered = error.render().to_string();
    let detail: Vec<&str> = rendered
        .lines()
        .map(str::trim)
        .take_while(|line| !line.starts_with("Usage:") && !line.starts_with("For more information"))
        .filter(|line| !line.is_empty())
        .collect();
    let detail = detail.join(" ");
    let detail = detail.strip_prefix("error: ").unwrap_or(&detail);
    UserMessage::new(
        format!(
            "Khalid could not read the command line: {}.",
            detail.trim_end_matches('.')
        ),
        "Khalid stopped before doing anything, so nothing has changed.",
        "Run `khalid --help` to see the commands and their options.",
    )
}

impl Output {
    /// Whether `--json` appears in arguments that `clap` could not parse.
    pub(crate) fn requested_in(args: &[OsString]) -> Self {
        if args.iter().any(|arg| arg == "--json") {
            Self::Json
        } else {
            Self::Human
        }
    }
}
