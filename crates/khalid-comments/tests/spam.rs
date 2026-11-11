//! Staff move a comment to spam, which hides it, and back to the queue
//! when it was marked by mistake.

use std::sync::{Arc, Mutex, PoisonError};

use chrono::{TimeZone, Utc};
use khalid_api::comment_moderation::{CommentModeration, ModerationError};
use khalid_api::resources;
use khalid_comments::{
    ApiModeration, AuthorName, Comment, CommentAuthor, CommentEvents, CommentId, CommentStatus,
    CommentText, Comments, CommentsError, MemoryCommentStore, NewComment, PostFile, Thread,
};
use khalid_core::FixedClock;

/// Writes down each post whose shown comments changed, in order.
#[derive(Debug, Default)]
struct Shown(Mutex<Vec<String>>);

impl CommentEvents for Shown {
    fn shown_changed(&self, post: &PostFile) {
        let line = post.as_str().to_owned();
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(line);
    }
}

#[cfg(test)]
fn comments(shown: &Arc<Shown>) -> Comments<MemoryCommentStore> {
    let at = Utc.with_ymd_and_hms(2026, 11, 10, 9, 0, 0).unwrap();
    let listener: Arc<dyn CommentEvents> = Arc::<Shown>::clone(shown);
    Comments::new(MemoryCommentStore::default(), FixedClock::new(at)).with_events(listener)
}

#[cfg(test)]
fn by_reader(text: &str) -> NewComment {
    NewComment {
        post: PostFile::new("p.md"),
        parent: None,
        author: CommentAuthor::Reader {
            name: AuthorName::parse("Lena").unwrap(),
            email: None,
        },
        text: CommentText::parse(text).unwrap(),
    }
}

#[cfg(test)]
fn heard(shown: &Shown) -> Vec<String> {
    shown.0.lock().unwrap().clone()
}

#[tokio::test]
async fn spam_hides_a_shown_comment_and_moves_its_page() {
    let shown = Arc::new(Shown::default());
    let comments = comments(&shown);
    let sent = comments.submit(by_reader("Buy now")).await.unwrap();
    comments.approve(sent.id).await.unwrap();
    let spam = comments.mark_spam(sent.id).await.unwrap();
    assert_eq!(spam.status, CommentStatus::Spam);
    let page = comments.shown_on(&PostFile::new("p.md")).await.unwrap();
    assert_eq!(page, Vec::<Thread>::new());
    assert_eq!(heard(&shown), ["p.md", "p.md"], "approve, then spam");
}

#[tokio::test]
async fn spam_on_a_waiting_comment_moves_no_page() {
    let shown = Arc::new(Shown::default());
    let comments = comments(&shown);
    let sent = comments.submit(by_reader("Buy now")).await.unwrap();
    comments.mark_spam(sent.id).await.unwrap();
    let spam = comments
        .listed(Some(CommentStatus::Spam), 10)
        .await
        .unwrap();
    let ids: Vec<CommentId> = spam.iter().map(|comment| comment.id).collect();
    assert_eq!(ids, [sent.id]);
    assert_eq!(heard(&shown), Vec::<String>::new());
}

#[tokio::test]
async fn not_spam_puts_the_comment_back_in_the_queue() {
    let shown = Arc::new(Shown::default());
    let comments = comments(&shown);
    let sent = comments.submit(by_reader("A real question")).await.unwrap();
    comments.mark_spam(sent.id).await.unwrap();
    let back = comments.not_spam(sent.id).await.unwrap();
    assert_eq!(back.status, CommentStatus::Pending);
    assert_eq!(heard(&shown), Vec::<String>::new(), "nothing was shown");
}

#[tokio::test]
async fn not_spam_leaves_a_comment_that_is_not_spam_as_it_is() {
    let shown = Arc::new(Shown::default());
    let comments = comments(&shown);
    let sent = comments.submit(by_reader("Hello")).await.unwrap();
    comments.approve(sent.id).await.unwrap();
    let same: Comment = comments.not_spam(sent.id).await.unwrap();
    assert_eq!(same.status, CommentStatus::Approved);
    assert_eq!(heard(&shown), ["p.md"], "only the approval");
}

#[tokio::test]
async fn moving_a_missing_comment_says_so() {
    let comments = comments(&Arc::default());
    let missing = CommentId::new(7);
    let not_found = |result| matches!(result, Err(CommentsError::NotFound { id }) if id == missing);
    assert!(not_found(comments.mark_spam(missing).await));
    assert!(not_found(comments.not_spam(missing).await));
}

#[tokio::test]
async fn the_admin_api_moves_a_comment_to_spam_and_back() {
    let comments = comments(&Arc::default());
    comments.submit(by_reader("Buy now")).await.unwrap();
    let api = ApiModeration::new(Arc::new(comments));
    let spam = api.mark_spam(1).await.unwrap();
    assert_eq!(spam.status, resources::CommentStatus::Spam);
    let back = api.not_spam(1).await.unwrap();
    assert_eq!(back.status, resources::CommentStatus::Pending);
    let missing = api.mark_spam(9).await;
    assert!(matches!(missing, Err(ModerationError::NotFound)));
    let missing = api.not_spam(9).await;
    assert!(matches!(missing, Err(ModerationError::NotFound)));
}
