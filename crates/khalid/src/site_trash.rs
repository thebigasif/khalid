//! The trash of a running site (KH-EDT-6): a deleted post's or page's
//! file, or its folder, moves to `trash/<key>/` under the path it had,
//! such as `trash/20261031T093000Z-en.hello/content/en/posts/hello.md`,
//! so restoring moves it back. `trash/` sits beside `content/`, which is
//! all a build reads, so nothing in it is served.

use std::fs;
use std::path::{Component, Path, PathBuf};
use std::sync::PoisonError;

use chrono::{NaiveDateTime, TimeDelta};
use khalid_api::content_store::BoxFuture;
use khalid_api::post_store::PostStoreError;
use khalid_api::post_trash::{self, PostTrash, Restored, TrashError, Trashed, TrashedKind};
use khalid_api::resources::{AdminPage, PostStatus};
use khalid_content::{PageFront, parse_post};
use khalid_core::SitePath;

use super::{Site, SitePosts, blocking, failure};
use crate::entry_files::front;
use crate::post_text::PostText;
use crate::site_entries::SiteEntries;
use crate::site_pages::reload::LiveConfig;

/// The trash folder at the top of a site.
const TRASH: &str = "trash";

impl Site {
    /// Moves `moved`, post `id`'s file or folder inside the site, to the
    /// trash, keyed by now.
    pub(super) fn keep(&self, moved: &Path, id: &str) -> Result<(), PostStoreError> {
        let rel = moved
            .strip_prefix(&self.folder)
            .map_err(|_| failure("a post lies outside its site"))?;
        let mut to = self.trashed_at(&post_trash::key(self.clock.now(), id));
        to.push(rel);
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).map_err(PostStoreError::new)?;
        }
        fs::rename(moved, &to).map_err(PostStoreError::new)
    }

    /// Whether `key` is one folder name inside `trash/`, which cannot climb.
    pub(super) fn names_one_copy(&self, key: &str) -> bool {
        let inside = SitePath::within(&self.folder, &format!("{TRASH}/{key}")).is_ok();
        inside && !key.contains('/')
    }

    fn trashed_at(&self, key: &str) -> PathBuf {
        let mut path = self.folder.clone();
        path.extend([TRASH, key]);
        path
    }

    /// The path a copy in the trash had in the site, and its status: a
    /// draft's file is three folders down, a post's file or folder four.
    fn trashed_from(&self, key: &str) -> Result<(PathBuf, PostStatus), TrashError> {
        let top = self.trashed_at(key);
        let mut rel = PathBuf::new();
        let mut drafts = top.clone();
        drafts.push(super::DRAFTS);
        let (depth, status) = if drafts.is_dir() {
            (3, PostStatus::Draft)
        } else {
            (4, PostStatus::Published)
        };
        for _ in 0..depth {
            let mut here = top.clone();
            here.push(&rel);
            let entry = fs::read_dir(&here)
                .ok()
                .and_then(|mut entries| entries.next())
                .and_then(Result::ok)
                .ok_or(TrashError::NotFound)?;
            rel.push(entry.file_name());
        }
        Ok((rel, status))
    }

    /// The copy at `key`, or `None` when nothing is there. A key comes from
    /// a request, so one that is not a single folder name inside `trash/`
    /// names nothing (RULES.md BL-SEC-4).
    fn listed(&self, key: &str) -> Option<Trashed> {
        if !self.names_one_copy(key) {
            return None;
        }
        let (stamp, id) = key.split_once('-')?;
        let at = NaiveDateTime::parse_from_str(stamp, "%Y%m%dT%H%M%SZ").ok()?;
        let (rel, status) = self.trashed_from(key).ok()?;
        let mut file = self.trashed_at(key);
        file.push(&rel);
        if file.is_dir() {
            file.push("index.md");
        }
        let text = fs::read_to_string(&file).ok()?;
        let kind = kind_of(&rel);
        let title = match kind {
            TrashedKind::Page => front::<PageFront>(&PostText::parse(&text)?).ok()?.title,
            _ => parse_post(&rel, &text).ok()?.front.title,
        };
        let title = title.unwrap_or_default();
        let listed = Trashed::listed(key, id, &title, status, at.and_utc());
        Some(listed.of_kind(kind))
    }

    fn trashed_now(&self) -> Result<Vec<Trashed>, PostStoreError> {
        let mut folder = self.folder.clone();
        folder.push(TRASH);
        let Ok(entries) = fs::read_dir(&folder) else {
            return Ok(Vec::new());
        };
        let mut keys = Vec::new();
        for entry in entries {
            let entry = entry.map_err(PostStoreError::new)?;
            keys.extend(entry.file_name().to_str().map(str::to_owned));
        }
        keys.sort_unstable_by(|a, b| b.cmp(a));
        Ok(keys.iter().filter_map(|key| self.listed(key)).collect())
    }

    /// Whether a post, or a page when `kind` says so, has identity `id`.
    fn taken(&self, kind: TrashedKind, id: &str) -> Result<bool, PostStoreError> {
        if kind != TrashedKind::Page {
            return Ok(self.find(id)?.is_some());
        }
        let pages = SiteEntries::<AdminPage>::new(&self.folder, &self.config);
        pages.has(id).map_err(PostStoreError::new)
    }

    fn restore_now(&self, key: &str) -> Result<Restored, TrashError> {
        let _writing = self.seen.lock().unwrap_or_else(PoisonError::into_inner);
        let trashed = self.listed(key).ok_or(TrashError::NotFound)?;
        let (rel, _) = self.trashed_from(key)?;
        let mut to = self.folder.clone();
        to.push(&rel);
        if to.exists() || self.taken(trashed.kind, &trashed.id)? {
            return Err(TrashError::Taken { id: trashed.id });
        }
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).map_err(PostStoreError::new)?;
        }
        let mut from = self.trashed_at(key);
        from.push(&rel);
        fs::rename(&from, &to).map_err(PostStoreError::new)?;
        fs::remove_dir_all(self.trashed_at(key)).map_err(PostStoreError::new)?;
        if trashed.kind == TrashedKind::Page {
            return Ok(Restored::Page);
        }
        let restored = self.get_now(&trashed.id)?;
        let restored = restored.ok_or_else(|| failure("a restored post cannot be read back"))?;
        Ok(Restored::Post(Box::new(restored)))
    }

    fn purge_now(&self, key: &str) -> Result<(), TrashError> {
        let _writing = self.seen.lock().unwrap_or_else(PoisonError::into_inner);
        self.listed(key).ok_or(TrashError::NotFound)?;
        fs::remove_dir_all(self.trashed_at(key)).map_err(|e| PostStoreError::new(e).into())
    }

    /// Deletes for good each copy deleted more than the site's
    /// `trash_days` ago and answers their keys; none when it is 0. A
    /// folder in `trash/` that is not a readable copy is left alone.
    fn empty_old_now(&self) -> Result<Vec<String>, PostStoreError> {
        let config = (self.live.get()).map_or_else(|| self.config.clone(), LiveConfig::now);
        let Some(days) = config.trash_days() else {
            return Ok(Vec::new());
        };
        let before = self.clock.now() - TimeDelta::days(i64::from(days));
        let _writing = self.seen.lock().unwrap_or_else(PoisonError::into_inner);
        let mut emptied = Vec::new();
        for old in self.trashed_now()? {
            if old.trashed_at < before {
                fs::remove_dir_all(self.trashed_at(&old.key)).map_err(PostStoreError::new)?;
                emptied.push(old.key);
            }
        }
        Ok(emptied)
    }
}

/// Whether the copy at `rel` in the trash was a page: its path is
/// `content/<language>/pages/...`.
fn kind_of(rel: &Path) -> TrashedKind {
    let mut parts = rel.components().map(Component::as_os_str);
    let content = parts.next().is_some_and(|part| part == "content");
    let pages = parts.nth(1).is_some_and(|part| part == "pages");
    if content && pages {
        TrashedKind::Page
    } else {
        TrashedKind::Post
    }
}

impl SitePosts {
    /// The site folder these posts are in.
    pub(crate) fn folder(&self) -> &Path {
        &self.0.folder
    }

    /// Moves `moved`, entry `id`'s file or folder inside the site, to the
    /// trash these posts keep, keyed by now.
    ///
    /// # Errors
    ///
    /// Returns [`PostStoreError`] when it lies outside the site or cannot
    /// be moved.
    pub(crate) fn keep(&self, moved: &Path, id: &str) -> Result<(), PostStoreError> {
        self.0.keep(moved, id)
    }

    /// Deletes for good each post that has waited in the trash longer
    /// than the site's `trash_days`, and answers their keys.
    ///
    /// # Errors
    ///
    /// Returns [`PostStoreError`] when the trash cannot be read or a copy
    /// cannot be removed.
    pub(crate) async fn empty_old_trash(&self) -> Result<Vec<String>, PostStoreError> {
        blocking(&self.0, Site::empty_old_now).await
    }
}

impl PostTrash for SitePosts {
    fn trashed(&self) -> BoxFuture<'_, Result<Vec<Trashed>, PostStoreError>> {
        Box::pin(blocking(&self.0, Site::trashed_now))
    }

    fn restore<'a>(&'a self, key: &'a str) -> BoxFuture<'a, Result<Restored, TrashError>> {
        let key = key.to_owned();
        Box::pin(blocking(&self.0, move |site| site.restore_now(&key)))
    }

    fn purge<'a>(&'a self, key: &'a str) -> BoxFuture<'a, Result<(), TrashError>> {
        let key = key.to_owned();
        Box::pin(blocking(&self.0, move |site| site.purge_now(&key)))
    }
}
