//! `khalid import substack` and `khalid import ghost` bring the reader list
//! that sits beside the export into `data/site.db` (backlog row 9), the
//! way the Admin API's import does: each reader keeps the status the old
//! system gave them, an address the site has stays as it is, and no email
//! goes out.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use khalid_api::resources::SubscriberImport;
use khalid_api::subscriber_desk::ImportMode;
use khalid_core::{Clock, SystemClock, UserMessage};
use khalid_db::SubscriberTable;
use khalid_import::subscribers::read_readers;
use khalid_newsletter::EmailAddress;

use crate::cli::{ImportArgs, ImportSource};
use crate::doctor_files::counted;
use crate::exit::Exit;
use crate::serve_apis::subscriber_files;
use crate::{newsletter_files, redirects};

/// A reader list found beside an export.
pub(crate) struct ReaderList {
    /// The file's name, as the person sees it in the folder.
    name: String,
    /// The file's text.
    text: String,
    /// The system that wrote it, such as Substack.
    from: &'static str,
}

/// The reader list beside the export `args` name: Substack's `email_list`
/// file in the unpacked folder, or Ghost's members export in the folder of
/// its JSON file. `None` for other sources, or when there is none.
pub(crate) fn find(args: &ImportArgs) -> Result<Option<ReaderList>, (UserMessage, Exit)> {
    let export = args.export.as_path();
    // Each test sees the file's name in lowercase, without `.csv`.
    let (folder, from, wanted): (_, _, fn(&str) -> bool) = match args.source {
        ImportSource::Substack => (export, "Substack", |stem| stem.starts_with("email_list")),
        ImportSource::Ghost => (
            export
                .parent()
                .filter(|up| !up.as_os_str().is_empty())
                .unwrap_or(Path::new(".")),
            "Ghost",
            |stem| stem.contains("members"),
        ),
        ImportSource::Wordpress | ImportSource::Medium | ImportSource::Markdown => return Ok(None),
    };
    // A folder that cannot be read holds no list; reading the posts says why.
    let Ok(entries) = fs::read_dir(folder) else {
        return Ok(None);
    };
    let mut found: Vec<(String, PathBuf)> = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
        .map(|entry| entry.path())
        .filter(|path| {
            let csv = path
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("csv"));
            let stem = path.file_stem().and_then(|stem| stem.to_str());
            csv && stem.is_some_and(|stem| wanted(&stem.to_lowercase()))
        })
        .filter_map(|path| Some((path.file_name()?.to_str()?.to_owned(), path)))
        .collect();
    found.sort();
    let Some((name, path)) = found.into_iter().next() else {
        return Ok(None);
    };
    let text = fs::read_to_string(&path).map_err(|error| {
        let message = UserMessage::new(
            format!(
                "Khalid could not read the reader list {}: {error}.",
                path.display()
            ),
            "Nothing was imported.",
            "Check that the file can be read, then run the command again.",
        );
        (message, Exit::Usage)
    })?;
    Ok(Some(ReaderList { name, text, from }))
}

/// What a dry run says about `list`: how many readers would come in.
pub(crate) fn dry_text(list: &ReaderList) -> String {
    let read = match read_readers(&list.text) {
        Ok(read) => read,
        Err(error) => {
            return format!(
                "The reader list {} cannot be read: {error}. A real run stops before it writes anything.\n",
                list.name
            );
        }
    };
    let good = read
        .readers
        .iter()
        .filter(|(_, reader)| EmailAddress::parse(&reader.email).is_ok())
        .count();
    let left = read.readers.len().saturating_sub(good) + read.unread.len();
    let readers = counted(good, "reader");
    let rows = match left {
        0 => String::new(),
        _ => format!(", and {} Khalid leaves out", counted(left, "row")),
    };
    format!(
        "The reader list {} holds {readers}{rows}. A real run brings them in as they stood on {}, and sends them no email.\n",
        list.name, list.from
    )
}

/// Refuses `list` when it is no reader list, before the import writes
/// anything.
pub(crate) fn check(list: &ReaderList) -> Result<(), (UserMessage, Exit)> {
    let Err(error) = read_readers(&list.text) else {
        return Ok(());
    };
    let message = UserMessage::new(
        format!("Khalid cannot read the reader list {}: {error}.", list.name),
        "Nothing was imported, so the site holds no half blog.",
        format!(
            "Download the list from {} again, or move {} out of the folder to bring the posts alone.",
            list.from, list.name
        ),
    );
    Err((message, Exit::Failure))
}

/// Brings the readers of `list` into the site at `site`, and says what
/// came in.
pub(crate) fn bring_in(site: &Path, list: &ReaderList) -> Result<String, (UserMessage, Exit)> {
    let lists = newsletter_files::lists(site);
    let at = SystemClock.now();
    let done = redirects::on_main_site(site, async |database, id| {
        let table = SubscriberTable::new(database.clone(), id);
        let file = (list.text.as_str(), None, ImportMode::Save);
        subscriber_files::import(&table, &lists, file, at).await
    })?;
    let done = done.map_err(|error| {
        let message = UserMessage::new(
            format!(
                "The posts came in, but the readers in {} did not: {error}.",
                list.name
            ),
            "Your readers are not on the site's newsletters yet.",
            "Bring the file in on the admin's Subscribers screen, under Move readers in or out.",
        );
        (message, Exit::Failure)
    })?;
    Ok(brought_text(list, &done))
}

/// What an import of `list` did, in plain sentences.
fn brought_text(list: &ReaderList, done: &SubscriberImport) -> String {
    let count = |n: u64| usize::try_from(n).unwrap_or(usize::MAX);
    let mut text = format!(
        "Brought in {} from {}, each as they stood on {}, and sent them no email.\n",
        counted(count(done.added), "reader"),
        list.name,
        list.from
    );
    // Writing to a String cannot fail.
    if done.already > 0 {
        let _ = writeln!(
            text,
            "{} on the list already stayed as they were.",
            counted(count(done.already), "reader")
        );
    }
    if !done.skipped.is_empty() {
        let rows: Vec<String> = done
            .skipped
            .iter()
            .map(|row| format!("line {}, {}", row.line, row.why))
            .collect();
        let _ = writeln!(
            text,
            "Left out {}: {}.",
            counted(done.skipped.len(), "row"),
            rows.join("; ")
        );
    }
    text
}
