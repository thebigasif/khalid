//! The command examples in the docs, run against the real binary with
//! trycmd: every `console` block in `docs/cli.md`, and the getting-started
//! steps in `tests/cmd/getting-started.md`, which make a site in a fresh
//! folder. A renamed flag or a changed message fails here before it
//! misleads a reader. Blocks marked `text` are shown, not run.

use std::path::Path;

#[test]
fn every_console_example_runs_as_written() {
    trycmd::TestCases::new()
        .register_bin("khalid", Path::new(env!("CARGO_BIN_EXE_khalid")))
        .case("../../docs/cli.md")
        .case("tests/cmd/*.md");
}
