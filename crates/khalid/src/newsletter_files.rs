//! The newsletters a site has, one folder each in `content/newsletters/`
//! (task 5.7). `main` needs no folder. A folder's name is the `list=` a
//! `::newsletter` block gives, and its `welcome.md` and `welcome-day-<n>.md`
//! files are the welcome sequence a reader gets on joining (5.9, G39).

use std::fs;
use std::path::{Path, PathBuf};

use khalid_core::{SitePath, Slug};
use khalid_newsletter::{BoxFuture, Lists, Welcome, WelcomeFiles};

pub(crate) mod archive_pages;

/// Where the newsletters' folders are, inside the site folder.
const FOLDER: &str = "content/newsletters";

/// `main` and every newsletter with a folder in the site in `folder`. A
/// folder whose name is not a slug is skipped, as no block can name it,
/// and a site without the folder has `main` alone.
pub(crate) fn lists(folder: &Path) -> Lists {
    let Ok(newsletters) = SitePath::within(folder, FOLDER) else {
        return Lists::main_only();
    };
    let Ok(entries) = fs::read_dir(newsletters.as_path()) else {
        return Lists::main_only();
    };
    let names = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            Slug::try_new(&name)
                .ok()
                .filter(|slug| slug.as_str() == name)
        });
    Lists::new(names)
}

/// The welcome sequences in a site's newsletter folders, read again each
/// time a reader is due one, so an edited `welcome.md` needs no restart
/// (task 5.9, G39).
#[derive(Debug)]
pub(crate) struct WelcomeFolder {
    site: PathBuf,
}

impl WelcomeFolder {
    /// The welcome sequences of the site in `folder`.
    pub(crate) fn new(folder: &Path) -> Self {
        Self {
            site: folder.to_path_buf(),
        }
    }
}

impl WelcomeFiles for WelcomeFolder {
    fn steps<'a>(&'a self, list: &'a Slug) -> BoxFuture<'a, Vec<Welcome>> {
        let (site, list) = (self.site.clone(), list.clone());
        Box::pin(async move {
            let read = tokio::task::spawn_blocking(move || welcome_steps(&site, &list));
            read.await.unwrap_or_default()
        })
    }
}

/// The steps of the welcome sequence of `list` in the site in `folder`:
/// `welcome.md` at once, and each `welcome-day-<n>.md` on day `n`, by day.
/// A file that is unreadable or without a `# ` subject line sends nothing.
fn welcome_steps(folder: &Path, list: &Slug) -> Vec<Welcome> {
    let Ok(newsletter) = SitePath::within(folder, &format!("{FOLDER}/{list}")) else {
        return Vec::new();
    };
    let Ok(entries) = fs::read_dir(newsletter.as_path()) else {
        return Vec::new();
    };
    let mut steps: Vec<Welcome> = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            let day = Welcome::day_of_file(&name)?;
            let file = SitePath::within(folder, &format!("{FOLDER}/{list}/{name}")).ok()?;
            let text = fs::read_to_string(file.as_path()).ok()?;
            Some(Welcome::read(&text)?.on_day(day))
        })
        .collect();
    steps.sort_by_key(Welcome::day);
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_folder_named_as_a_slug_is_a_newsletter_beside_main() {
        let site = tempfile::tempdir().unwrap();
        let root = site.path().to_str().unwrap();
        for name in ["weekly", "Not A Slug", "launch-notes"] {
            fs::create_dir_all(format!("{root}/{FOLDER}/{name}")).unwrap();
        }
        fs::write(format!("{root}/{FOLDER}/notes.md"), "a file").unwrap();
        let names: Vec<_> = lists(site.path()).iter().map(Slug::to_string).collect();
        assert_eq!(names, ["launch-notes", "main", "weekly"]);
    }

    #[test]
    fn each_readable_welcome_file_is_a_step_on_its_day() {
        let site = tempfile::tempdir().unwrap();
        let root = site.path().to_str().unwrap();
        fs::create_dir_all(format!("{root}/{FOLDER}/main")).unwrap();
        for (name, text) in [
            ("welcome.md", "# Hello\n\nThanks.\n"),
            ("welcome-day-10.md", "# Ten days in\n\nA tip.\n"),
            ("welcome-day-3.md", "# Three days in\n\nA tip.\n"),
            ("welcome-day-4.md", "No heading.\n"),
            ("welcome-day-04.md", "# Padded\n\nA tip.\n"),
            ("page.md", "# The page\n\nText.\n"),
        ] {
            fs::write(format!("{root}/{FOLDER}/main/{name}"), text).unwrap();
        }
        let main = Slug::try_new("main").unwrap();
        let found: Vec<_> = welcome_steps(site.path(), &main)
            .iter()
            .map(|step| (step.day(), step.subject().to_owned()))
            .collect();
        let days: Vec<_> = found.iter().map(|(day, _)| *day).collect();
        assert_eq!(days, [0, 3, 10], "{found:?}");
        let weekly = Slug::try_new("weekly").unwrap();
        assert_eq!(welcome_steps(site.path(), &weekly), []);
    }

    #[test]
    fn a_site_without_the_folder_has_main_alone() {
        let site = tempfile::tempdir().unwrap();
        assert_eq!(lists(site.path()), Lists::main_only());
    }
}
