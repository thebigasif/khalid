//! `khalid man <FOLDER>`: writes a manual page for khalid and one for each
//! command, such as `khalid-init.1`, made from the same command line `clap`
//! parses. Only builds with the `man` feature have it: `just man` and the
//! release write the pages once, and the release archive and the Docker
//! image ship them, so the binary itself carries no page writer.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use clap::{Args, CommandFactory};
use khalid_core::UserMessage;
use serde::Serialize;

use crate::cli::{Cli, Output};
use crate::exit::Exit;
use crate::report;

/// What `khalid man` takes.
#[derive(Debug, Args)]
pub(crate) struct ManArgs {
    /// The folder to write the pages to, made when missing
    #[arg(value_name = "FOLDER")]
    pub(crate) out: PathBuf,
}

/// The JSON shape of `khalid man --json`.
#[derive(Serialize)]
struct PagesBody<'a> {
    folder: &'a Path,
    pages: Vec<String>,
}

/// Writes the pages into the folder `args` names and says how many, or
/// lists them as JSON.
pub(crate) fn run(args: &ManArgs, output: Output) -> Exit {
    let pages = match write(&args.out) {
        Ok(pages) => pages,
        Err(error) => return report::failure(output, &failed(&args.out, &error), Exit::Failure),
    };
    match output {
        Output::Human => report::text(
            &format!(
                "Khalid wrote {} manual pages to {}. Read one with `man {}`.\n",
                pages.len(),
                args.out.display(),
                page_path(&args.out, "khalid.1").display()
            ),
            Exit::Success,
        ),
        Output::Json => report::json(
            &PagesBody {
                folder: &args.out,
                pages,
            },
            Exit::Success,
        ),
    }
}

/// Writes every page into `out` and returns their file names in order.
fn write(out: &Path) -> io::Result<Vec<String>> {
    fs::create_dir_all(out)?;
    let mut command = Cli::command().disable_help_subcommand(true);
    command.build();
    let mut pages = Vec::new();
    write_pages(command, out, &mut pages)?;
    pages.sort();
    Ok(pages)
}

/// Writes the page for `command` and for each of its commands that is not
/// hidden, each naming this version of khalid as its source.
fn write_pages(command: clap::Command, out: &Path, pages: &mut Vec<String>) -> io::Result<()> {
    let shown = command.get_subcommands().filter(|sub| !sub.is_hide_set());
    for sub in shown.cloned().collect::<Vec<_>>() {
        write_pages(sub, out, pages)?;
    }
    let page = clap_mangen::Man::new(command)
        .source(concat!("khalid ", env!("CARGO_PKG_VERSION")))
        .manual("Khalid Manual");
    page.generate_to(out)?;
    pages.push(page.get_filename());
    Ok(())
}

/// The page `name` inside `out`.
fn page_path(out: &Path, name: &str) -> PathBuf {
    let mut path = out.to_path_buf();
    path.push(name);
    path
}

/// The message for pages that could not be written to `out`.
fn failed(out: &Path, error: &io::Error) -> UserMessage {
    UserMessage::new(
        format!(
            "Khalid could not write the manual pages to {}: {error}.",
            out.display()
        ),
        "Without every page, `man` shows an old page for some commands, or none.",
        "Name a folder you can write to, or one that does not exist yet, and run the command again.",
    )
}
