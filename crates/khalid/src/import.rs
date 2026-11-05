//! `khalid import`: brings a blog from `WordPress`, Ghost, Medium, Substack or a markdown folder into the
//! site through `khalid-import`, keeping every slug, and stores the
//! redirect map in `data/site.db` through the redirect desk (plan section
//! 5.7, KH-API-7, KH-SEO-8).

mod images;

use std::collections::BTreeMap;
use std::fmt::{Display, Write as _};
use std::fs;
use std::path::{Path, PathBuf};

use khalid_api::redirect_store::Origin;
use khalid_core::{AbsoluteUrl, Config, UserMessage};
use khalid_import::{PlannedFile, WriteError, folder, ghost, medium, substack, wordpress};
use khalid_routes::Routes;
use khalid_seo::{Matching, RedirectEntry, read_redirects_csv};
use serde::Serialize;

use self::images::Images;
use crate::cli::{
    Cli, Images as ImagesChoice, ImportArgs, ImportSource, Output, Run, command_for, path_for,
};
use crate::doctor_files::counted;
use crate::exit::Exit;
use crate::import_held::{HELD_CSV, Held};
use crate::{import_held, import_rest, redirects, report};

/// The JSON shape of `khalid import --json`.
#[derive(Serialize)]
struct ImportedBody<'a> {
    dry_run: bool,
    report: &'a str,
    written: Vec<String>,
    stored_redirects: usize,
    /// The imported posts the linter blocks, from the site folder.
    held_posts: Vec<String>,
    /// The old addresses that wait for those posts instead of being stored.
    held_redirects: usize,
}

/// One source's plan, which says what an import would write.
enum Plan {
    Wordpress(wordpress::Plan),
    Ghost(ghost::Plan),
    Medium(medium::Plan),
    Substack(substack::Plan),
    Folder(folder::Plan),
}

impl Plan {
    /// The dry-run report, in plain sentences.
    fn report(&self) -> String {
        match self {
            Self::Wordpress(plan) => plan.report().to_string(),
            Self::Ghost(plan) => plan.report().to_string(),
            Self::Medium(plan) => plan.report().to_string(),
            Self::Substack(plan) => plan.report().to_string(),
            Self::Folder(plan) => plan.report().to_string(),
        }
    }

    fn files(&self) -> &[PlannedFile] {
        match self {
            Self::Wordpress(plan) => &plan.files,
            Self::Ghost(plan) => &plan.files,
            Self::Medium(plan) => &plan.files,
            Self::Substack(plan) => &plan.files,
            Self::Folder(plan) => &plan.files,
        }
    }

    fn files_mut(&mut self) -> &mut [PlannedFile] {
        match self {
            Self::Wordpress(plan) => &mut plan.files,
            Self::Ghost(plan) => &mut plan.files,
            Self::Medium(plan) => &mut plan.files,
            Self::Substack(plan) => &mut plan.files,
            Self::Folder(plan) => &mut plan.files,
        }
    }

    /// The feature image of each post that has one, by the post's file.
    /// Only `WordPress` and Ghost exports name one.
    fn leads(&self) -> BTreeMap<PathBuf, String> {
        let mut leads = BTreeMap::new();
        match self {
            Self::Wordpress(plan) => {
                for item in &plan.items {
                    for held in &item.held {
                        if let wordpress::Held::FeatureImage(image) = held {
                            leads.insert(item.path.clone(), image.as_str().to_owned());
                        }
                    }
                }
            }
            Self::Ghost(plan) => {
                for post in &plan.posts {
                    for held in &post.held {
                        if let ghost::Held::FeatureImage(image) = held {
                            leads.insert(post.path.clone(), image.as_str().to_owned());
                        }
                    }
                }
            }
            Self::Medium(_) | Self::Substack(_) | Self::Folder(_) => {}
        }
        leads
    }

    /// Downloads the images the plan's posts show into their folders, so
    /// the report no longer lists a feature image that became a lead image.
    fn download_images(&mut self, site: &Path, cli: &Cli) -> Result<Images, (UserMessage, Exit)> {
        let leads = self.leads();
        let images = images::bring_in(self.files_mut(), &leads, site, cli)?;
        let taken = |path: &PathBuf| images.leads.contains(path);
        match self {
            Self::Wordpress(plan) => {
                for item in plan.items.iter_mut().filter(|item| taken(&item.path)) {
                    item.held
                        .retain(|held| !matches!(held, wordpress::Held::FeatureImage(_)));
                }
            }
            Self::Ghost(plan) => {
                for post in plan.posts.iter_mut().filter(|post| taken(&post.path)) {
                    post.held
                        .retain(|held| !matches!(held, ghost::Held::FeatureImage(_)));
                }
            }
            Self::Medium(_) | Self::Substack(_) | Self::Folder(_) => {}
        }
        Ok(images)
    }

    fn write(&self, site: &Path) -> Result<Vec<PathBuf>, WriteError> {
        match self {
            Self::Wordpress(plan) => plan.write(site),
            Self::Ghost(plan) => plan.write(site),
            Self::Medium(plan) => plan.write(site),
            Self::Substack(plan) => plan.write(site),
            Self::Folder(plan) => plan.write(site),
        }
    }

    /// The redirect map's file, from the site folder.
    fn map_file(&self) -> &'static str {
        match self {
            Self::Wordpress(_) => wordpress::REDIRECTS_CSV,
            Self::Ghost(_) => ghost::REDIRECTS_CSV,
            Self::Medium(_) => medium::REDIRECTS_CSV,
            Self::Substack(_) => substack::REDIRECTS_CSV,
            Self::Folder(_) => folder::REDIRECTS_CSV,
        }
    }
}

type Done = Result<Exit, (UserMessage, Exit)>;

/// Runs `khalid import` on the checked site in `site`: a dry run prints the
/// report, and a real run writes the files and stores the redirects.
pub(crate) fn run(site: &Path, checked: (&Config, &Routes), args: &ImportArgs, cli: &Cli) -> Exit {
    let (config, routes, output) = (checked.0, checked.1, cli.output);
    planned(args, (config, routes), cli)
        .and_then(|mut plan| match (args.run, args.images) {
            (Run::Dry, choice) => {
                let mut said = plan.report();
                if choice == ImagesChoice::Download {
                    let count = images::count(plan.files(), &plan.leads());
                    let images = counted(count, "image");
                    let _ = writeln!(
                        said,
                        "With --download-images, the import would download {images}."
                    );
                }
                Ok(answer(&said, None, output))
            }
            (Run::Write, ImagesChoice::Download) => {
                let images = plan.download_images(site, cli)?;
                import(site, (config, routes), (&plan, Some(&images)), output)
            }
            (Run::Write, ImagesChoice::Link) => {
                import(site, (config, routes), (&plan, None), output)
            }
        })
        .unwrap_or_else(|(message, exit)| report::failure(output, &message, exit))
}

/// Writes the plan's files and the images it downloaded, then stores the
/// redirects that start on this site's host, but those that lead to an
/// imported post the publish gate blocks, which wait in its aliases.
fn import(
    site: &Path,
    (config, routes): (&Config, &Routes),
    (plan, images): (&Plan, Option<&Images>),
    output: Output,
) -> Done {
    let mut written = plan
        .write(site)
        .map_err(|error| (not_written(&error), Exit::Failure))?;
    if let Some(images) = images {
        written.extend(images::write(site, images)?);
    }
    let map = plan.map_file();
    let not_stored = |why: String, exit| {
        let message = UserMessage::new(
            format!("The posts came in, but the redirects in {map} were not stored."),
            why,
            format!(
                "Fix that, then run `{}`.",
                command_for(site, &format!("redirects import {}", path_for(site, map)))
            ),
        );
        (message, exit)
    };
    let csv = plan
        .files()
        .iter()
        .find(|file| file.path == Path::new(map))
        .map(|file| ours(&file.text))
        .unwrap_or_default();
    let mut entries = read_redirects_csv(&csv)
        .map_err(|error| not_stored(format!("The map cannot be read: {error}."), Exit::Internal))?;
    let origin = config.address().as_str().trim_end_matches('/');
    entries.retain(|entry| !to_itself(origin, entry));
    let held = import_held::hold(site, (config, routes), &written, &mut entries)
        .map_err(|message| (message, Exit::Failure))?;
    let stored = store(site, entries).map_err(|(why, exit)| not_stored(why, exit))?;
    // The report's first sentence is for a dry run only.
    let said = plan.report();
    let mut said = said
        .split_once('\n')
        .map_or("", |(_, rest)| rest)
        .to_owned();
    if let Some(images) = images {
        said.push_str(&images_text(images));
    }
    Ok(answer(&said, Some((site, &written, stored, &held)), output))
}

/// What became of the images the posts show, in plain sentences.
fn images_text(images: &Images) -> String {
    let fetched = counted(images.fetched, "image");
    let mut text = format!(
        "Downloaded {fetched} from the old site, each into the folder of the post that shows it.\n"
    );
    // Writing to a String cannot fail.
    for (address, why) in &images.kept {
        let _ = writeln!(
            text,
            "Kept at its old address, so it goes when the old site goes: {address} ({why})."
        );
    }
    text
}

/// Stores `entries` through the redirect desk, and returns how many, or
/// says why not in one sentence.
fn store(site: &Path, entries: Vec<RedirectEntry>) -> Result<usize, (String, Exit)> {
    if entries.is_empty() {
        return Ok(0);
    }
    redirects::on_desk(site, async |desk| {
        desk.store_checked(entries, Origin::Import).await
    })
    .map_err(|(message, exit)| (message.what().to_owned(), exit))?
    .map(|rows| rows.len())
    .map_err(|refusal| {
        (
            format!("They clash with the stored ones: {refusal}."),
            Exit::Failure,
        )
    })
}

/// Whether `entry` sends a path to the same path on the site at `origin`,
/// as a map row does for an address that only changes host. The page
/// answers that address itself, so the row is not stored.
fn to_itself(origin: &str, entry: &RedirectEntry) -> bool {
    let columns = entry.columns();
    let to = columns.to.as_deref().unwrap_or_default();
    columns.matching == Matching::Exact && to.strip_prefix(origin).unwrap_or(to) == columns.from
}

/// The rows of a redirect map that this site can answer: those whose old
/// address is a path or a pattern. A row holding a whole address on another
/// host stays in the file for that host.
fn ours(csv: &str) -> String {
    let mut kept = String::new();
    for (index, line) in csv.lines().enumerate() {
        let path = line.trim_start_matches('"').starts_with('/');
        if index == 0 || path || line.ends_with(",regex") {
            kept.push_str(line);
            kept.push('\n');
        }
    }
    kept
}

/// Prints what would come in, or with `done`, the files written, the
/// redirects stored and what was held back.
fn answer(said: &str, done: Option<(&Path, &[PathBuf], usize, &Held)>, output: Output) -> Exit {
    match (output, done) {
        (Output::Json, _) => {
            let none = Held::default();
            let (_, written, stored, held) = done.unwrap_or((Path::new("."), &[], 0, &none));
            let body = ImportedBody {
                dry_run: done.is_none(),
                report: said,
                written: written.iter().map(|p| p.display().to_string()).collect(),
                stored_redirects: stored,
                held_posts: held.posts.iter().map(|p| p.display().to_string()).collect(),
                held_redirects: held.aliased + held.outside,
            };
            report::json(&body, Exit::Success)
        }
        (Output::Human, None) => report::text(said, Exit::Success),
        (Output::Human, Some((site, written, stored, held))) => {
            let files = counted(written.len(), "file");
            let stored = counted(stored, "redirect");
            let held = held_text(site, held);
            let lint = command_for(site, "lint");
            let text = format!(
                "Wrote {files} and stored {stored}.\n{held}{said}\
                 Run `{lint}` to see how each post scores.\n"
            );
            report::text(&text, Exit::Success)
        }
    }
}

/// The imported posts the linter blocks and where their old addresses
/// wait, in plain sentences, or nothing when every post went live. The
/// commands it names run from the folder the import ran in.
fn held_text(site: &Path, held: &Held) -> String {
    let paths: Vec<String> = held.posts.iter().map(|p| p.display().to_string()).collect();
    let mut text = match paths.len() {
        0 => return String::new(),
        1 => format!(
            "1 imported post is blocked by the linter, so readers do not see it yet: {}.\n",
            paths.join(", ")
        ),
        posts => format!(
            "{posts} imported posts are blocked by the linter, so readers do not see them yet: {}.\n",
            paths.join(", ")
        ),
    };
    // Writing to a String cannot fail.
    if held.aliased > 0 {
        let _ = writeln!(
            text,
            "Old addresses held in their aliases, the list in each post's header: {}. Each leads to its post once the post passes, and to the not-found page until then.",
            held.aliased
        );
    }
    if held.outside > 0 {
        let file = path_for(site, HELD_CSV);
        let store = command_for(site, &format!("redirects import {file}"));
        let _ = writeln!(
            text,
            "Old addresses held in {file}, because they lie outside the blog's folder: {}. Store them with `{store}` once their posts are live.",
            held.outside
        );
    }
    text
}

/// Reads the export the person named and plans the import.
fn planned(
    args: &ImportArgs,
    (config, routes): (&Config, &Routes),
    cli: &Cli,
) -> Result<Plan, (UserMessage, Exit)> {
    let export = args.export.as_path();
    match args.source {
        ImportSource::Wordpress => {
            let named = export.to_string_lossy();
            let wxr = if import_rest::is_address(&named) {
                let site = AbsoluteUrl::parse(&named).map_err(|e| unreadable(export, &e))?;
                let answers = import_rest::answers(cli, &site)?;
                wordpress::read_rest(&answers).map_err(|e| unreadable(export, &e))?
            } else {
                wordpress::read_wxr(&read(export)?).map_err(|e| unreadable(export, &e))?
            };
            let rules = match &args.redirection {
                Some(file) => Some(
                    wordpress::read_redirection(&read(file)?).map_err(|e| unreadable(file, &e))?,
                ),
                None => None,
            };
            Ok(Plan::Wordpress(wordpress::plan(
                &wxr,
                rules.as_ref(),
                config,
                routes,
            )))
        }
        ImportSource::Ghost => {
            let read = ghost::read_export(&read(export)?).map_err(|e| unreadable(export, &e))?;
            let old_site = args.old_site.as_ref().unwrap_or(config.address());
            Ok(Plan::Ghost(ghost::plan(&read, old_site, config, routes)))
        }
        ImportSource::Medium => {
            let files = posts_folder(export)?;
            let read = medium::read_export(files).map_err(|e| unreadable(export, &e))?;
            Ok(Plan::Medium(medium::plan(&read, config, routes)))
        }
        ImportSource::Substack => {
            let mut list = export.to_path_buf();
            list.push("posts.csv");
            let read = substack::read_export(&read(&list)?, posts_folder(export)?)
                .map_err(|e| unreadable(export, &e))?;
            let old_site = args.old_site.as_ref().unwrap_or(config.address());
            Ok(Plan::Substack(substack::plan(
                &read, old_site, config, routes,
            )))
        }
        ImportSource::Markdown => {
            let mut files = Vec::new();
            markdown_files(export, "", &mut files)?;
            let read = folder::read_folder(files).map_err(|e| unreadable(export, &e))?;
            let old_site = args.old_site.as_ref().unwrap_or(config.address());
            Ok(Plan::Folder(folder::plan(&read, old_site, config, routes)))
        }
    }
}

/// Folders a generator builds into or fills with packages, which hold no
/// posts of their own.
const BUILT: [&str; 4] = ["node_modules", "public", "_site", "resources"];

/// Adds every markdown file under `folder`, with its path from the top
/// starting `inside`, to `files`. Hidden and built folders are skipped.
fn markdown_files(
    folder: &Path,
    inside: &str,
    files: &mut Vec<(String, String)>,
) -> Result<(), (UserMessage, Exit)> {
    let entries = fs::read_dir(folder).map_err(|error| not_read(folder, &error))?;
    for entry in entries {
        let path = entry.map_err(|error| not_read(folder, &error))?.path();
        let name = path.file_name().map(|n| n.to_string_lossy().into_owned());
        let name = name.unwrap_or_default();
        let at = format!("{inside}{name}");
        if path.is_dir() && !name.starts_with('.') && !BUILT.contains(&name.as_str()) {
            markdown_files(&path, &format!("{at}/"), files)?;
        } else if [".md", ".markdown", ".mdx"]
            .iter()
            .any(|ext| name.ends_with(ext))
        {
            let text = fs::read(&path).map_err(|error| not_read(&path, &error))?;
            files.push((at, String::from_utf8_lossy(&text).into_owned()));
        }
    }
    Ok(())
}

/// Every file in a Medium or Substack export's `posts/` folder, by name.
/// `folder` may be the unpacked export or its `posts/` folder.
fn posts_folder(folder: &Path) -> Result<Vec<(String, String)>, (UserMessage, Exit)> {
    let mut posts = folder.to_path_buf();
    posts.push("posts");
    let posts = if posts.is_dir() {
        posts
    } else {
        folder.to_path_buf()
    };
    let entries = fs::read_dir(&posts).map_err(|error| not_read(&posts, &error))?;
    let mut files = Vec::new();
    for entry in entries {
        let path = entry.map_err(|error| not_read(&posts, &error))?.path();
        if path.is_file() {
            let name = path.file_name().map(|n| n.to_string_lossy().into_owned());
            let text = fs::read(&path).map_err(|error| not_read(&path, &error))?;
            files.push((
                name.unwrap_or_default(),
                String::from_utf8_lossy(&text).into_owned(),
            ));
        }
    }
    Ok(files)
}

fn read(file: &Path) -> Result<String, (UserMessage, Exit)> {
    fs::read_to_string(file).map_err(|error| not_read(file, &error))
}

fn not_read(file: &Path, error: &impl Display) -> (UserMessage, Exit) {
    let message = UserMessage::new(
        format!("Khalid could not read {}: {error}.", file.display()),
        "Nothing was imported.",
        "Check the path, and unpack a zip archive first, then run the command again.",
    );
    (message, Exit::Usage)
}

fn unreadable(file: &Path, error: &impl Display) -> (UserMessage, Exit) {
    let message = UserMessage::new(
        format!("Khalid cannot import {}: {error}.", file.display()),
        "Nothing was imported, so the site holds no half blog.",
        "Export the blog again from its platform, then run the command again.",
    );
    (message, Exit::Failure)
}

fn not_written(error: &WriteError) -> UserMessage {
    let fix = match error {
        WriteError::Exists { paths } => format!(
            "Move these files away, or import into a new site: {}.",
            paths
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ),
        _ => "Check that the site folder can be written to, then run the command again.".to_owned(),
    };
    UserMessage::new(
        format!("The import could not write its files: {error}."),
        "No redirect was stored, so readers still reach the old addresses.",
        fix,
    )
}
