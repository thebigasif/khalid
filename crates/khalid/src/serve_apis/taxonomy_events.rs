//! The tag and category webhooks (backlog row 35): `tag.renamed` and
//! `tag.removed` when the Admin API renames or removes a tag in every post,
//! and `category.added`, `category.renamed` and `category.deleted` when it
//! changes `categories.toml`. Each fires after the files are written, with
//! the tag or category as the Admin API shows it. A rename also carries the
//! copy it was made on as `previous`, and a removal the tag or category as
//! it last was.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use khalid_api::content_store::BoxFuture;
use khalid_api::entry_store::{EntryError, EntryStore, EntryStoreError};
use khalid_api::resources::{AdminCategory, AdminTag, CategoryEdit, NewCategory, TagEdit};
use khalid_api::tag_store::TagStore;
use khalid_webhooks::Event;
use serde::Serialize;

use crate::announce::SiteAnnounce;

/// The body of a tag event: the tag, and on a rename the tag it was.
#[derive(Debug, Serialize)]
struct TagBody {
    tag: AdminTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous: Option<AdminTag>,
}

/// The body of a category event: the category, and on a rename the
/// category it was.
#[derive(Debug, Serialize)]
struct CategoryBody {
    category: AdminCategory,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous: Option<AdminCategory>,
}

/// A tag store that tells `announce` of each tag renamed or removed, and
/// otherwise does what the store it wraps does.
pub(crate) struct AnnouncedTags {
    store: Arc<dyn TagStore>,
    announce: SiteAnnounce,
}

impl AnnouncedTags {
    /// Wraps `store`, telling `announce`.
    pub(crate) fn new(store: Arc<dyn TagStore>, announce: SiteAnnounce) -> Self {
        Self { store, announce }
    }

    /// Tag `id` as it is now. A tag no post has is not found, so nothing
    /// changes and nothing fires.
    async fn before(&self, id: &str) -> Result<AdminTag, EntryError> {
        let tags = self.store.list().await?;
        let tag = tags.into_iter().find(|tag| tag.id() == id);
        tag.ok_or(EntryError::NotFound)
    }
}

impl TagStore for AnnouncedTags {
    fn list(&self) -> BoxFuture<'_, Result<Vec<AdminTag>, EntryStoreError>> {
        self.store.list()
    }

    fn rename<'a>(
        &'a self,
        id: &'a str,
        edit: TagEdit,
    ) -> BoxFuture<'a, Result<AdminTag, EntryError>> {
        Box::pin(async move {
            let before = self.before(id).await?;
            let tag = self.store.rename(id, edit).await?;
            if tag.id() != before.id() || tag.name() != before.name() {
                let (tag, previous) = (tag.clone(), Some(before));
                let body = TagBody { tag, previous };
                self.announce.fire(Event::TagRenamed, body);
            }
            Ok(tag)
        })
    }

    fn remove<'a>(
        &'a self,
        id: &'a str,
        updated_at: DateTime<Utc>,
    ) -> BoxFuture<'a, Result<(), EntryError>> {
        Box::pin(async move {
            let tag = self.before(id).await?;
            self.store.remove(id, updated_at).await?;
            let body = TagBody {
                tag,
                previous: None,
            };
            self.announce.fire(Event::TagRemoved, body);
            Ok(())
        })
    }
}

/// A category store that tells `announce` of each category added, renamed
/// or deleted, and otherwise does what the store it wraps does.
pub(crate) struct AnnouncedCategories {
    store: Arc<dyn EntryStore<AdminCategory>>,
    announce: SiteAnnounce,
}

impl AnnouncedCategories {
    /// Wraps `store`, telling `announce`.
    pub(crate) fn new(store: Arc<dyn EntryStore<AdminCategory>>, announce: SiteAnnounce) -> Self {
        Self { store, announce }
    }

    /// Category `id` as it is now. One that does not exist is not found,
    /// so nothing changes and nothing fires.
    async fn before(&self, id: &str) -> Result<AdminCategory, EntryError> {
        self.store.get(id).await?.ok_or(EntryError::NotFound)
    }

    fn fire(&self, event: Event, category: AdminCategory, previous: Option<AdminCategory>) {
        let body = CategoryBody { category, previous };
        self.announce.fire(event, body);
    }
}

impl EntryStore<AdminCategory> for AnnouncedCategories {
    fn list(&self) -> BoxFuture<'_, Result<Vec<AdminCategory>, EntryStoreError>> {
        self.store.list()
    }

    fn get<'a>(
        &'a self,
        id: &'a str,
    ) -> BoxFuture<'a, Result<Option<AdminCategory>, EntryStoreError>> {
        self.store.get(id)
    }

    fn create(&self, new: NewCategory) -> BoxFuture<'_, Result<AdminCategory, EntryError>> {
        Box::pin(async move {
            let category = self.store.create(new).await?;
            self.fire(Event::CategoryAdded, category.clone(), None);
            Ok(category)
        })
    }

    fn update<'a>(
        &'a self,
        id: &'a str,
        edit: CategoryEdit,
    ) -> BoxFuture<'a, Result<AdminCategory, EntryError>> {
        Box::pin(async move {
            let before = self.before(id).await?;
            let category = self.store.update(id, edit).await?;
            if category.names() != before.names() {
                self.fire(Event::CategoryRenamed, category.clone(), Some(before));
            }
            Ok(category)
        })
    }

    fn delete<'a>(
        &'a self,
        id: &'a str,
        updated_at: DateTime<Utc>,
    ) -> BoxFuture<'a, Result<(), EntryError>> {
        Box::pin(async move {
            let category = self.before(id).await?;
            self.store.delete(id, updated_at).await?;
            self.fire(Event::CategoryDeleted, category, None);
            Ok(())
        })
    }
}
