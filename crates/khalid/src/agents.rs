//! `khalid agents`: prints the guide for AI agents that is built into the
//! binary, so it always matches the version that runs (KH-API-5).

use serde::Serialize;

use crate::assets::texts;
use crate::cli::Output;
use crate::exit::Exit;
use crate::report;

/// The guide, `agents.md` beside this file, as the binary packs it.
const GUIDE: &str = "src/agents.md";

/// The JSON shape of `khalid agents --json`.
#[derive(Serialize)]
struct GuideBody {
    guide: &'static str,
}

/// Prints the guide as markdown, or as the `guide` field of one JSON value.
pub(crate) fn run(output: Output) -> Exit {
    let guide = texts::text(GUIDE);
    match output {
        Output::Human => report::text(guide, Exit::Success),
        Output::Json => report::json(&GuideBody { guide }, Exit::Success),
    }
}
