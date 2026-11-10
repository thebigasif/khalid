//! `khalid completions <shell>`: prints the script that lets a shell
//! complete Khalid's commands and options on Tab. It is made from the same
//! command line `clap` parses, so it always matches the binary that runs.

use clap::{Args, CommandFactory};
use clap_complete::Shell;
use serde::Serialize;

use crate::cli::{Cli, Output};
use crate::exit::Exit;
use crate::report;

/// What `khalid completions` takes.
#[derive(Debug, Args)]
pub(crate) struct CompletionsArgs {
    /// The shell to print the script for
    #[arg(value_enum, value_name = "SHELL")]
    pub(crate) shell: Shell,
}

/// The JSON shape of `khalid completions --json`.
#[derive(Serialize)]
struct ScriptBody {
    shell: String,
    script: String,
}

/// Prints the completion script for the shell `args` names, as text or as
/// the `script` field of one JSON value.
pub(crate) fn run(args: &CompletionsArgs, output: Output) -> Exit {
    let script = script(args.shell);
    match output {
        Output::Human => report::text(&script, Exit::Success),
        Output::Json => report::json(
            &ScriptBody {
                shell: args.shell.to_string(),
                script,
            },
            Exit::Success,
        ),
    }
}

/// The completion script for `shell`.
fn script(shell: Shell) -> String {
    let mut written = Vec::new();
    clap_complete::generate(shell, &mut Cli::command(), "khalid", &mut written);
    String::from_utf8_lossy(&written).into_owned()
}
