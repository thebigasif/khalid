//! Where the Admin API reads what visitors sent through a post's form
//! block (KH-GRO-1, plan section 4.6).

use std::error::Error;

use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

use crate::content_store::BoxFuture;

/// The answers kept for the site's forms. `khalid-db` implements it on the
/// table the form route writes to.
pub trait FormAnswers: Send + Sync {
    /// Every form with at least one answer, the one answered last first.
    ///
    /// # Errors
    ///
    /// [`FormAnswersError`] when the answers cannot be read.
    fn forms(&self) -> BoxFuture<'_, Result<Vec<FormSummary>, FormAnswersError>>;

    /// At most `limit` answers to `form`, newest first, after skipping
    /// `offset`, and how many it has in all. A form nobody answered, or a
    /// name no form block can have, has none.
    ///
    /// # Errors
    ///
    /// [`FormAnswersError`] when the answers cannot be read.
    fn answers<'a>(
        &'a self,
        form: &'a str,
        offset: u64,
        limit: u32,
    ) -> BoxFuture<'a, Result<AnswerPage, FormAnswersError>>;

    /// Deletes the answer `answer` to `form` for good, as a visitor may ask
    /// under the GDPR. `false` when the form has no such answer.
    ///
    /// # Errors
    ///
    /// [`FormAnswersError`] when the answer cannot be deleted.
    fn delete<'a>(
        &'a self,
        form: &'a str,
        answer: i64,
    ) -> BoxFuture<'a, Result<bool, FormAnswersError>>;
}

/// One form and how many answers it has.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct FormSummary {
    /// The form's id, as its block names it.
    #[schema(example = "contact")]
    pub id: String,
    /// How many answers are kept, spam left out.
    #[schema(example = 12)]
    pub answers: u64,
    /// When the newest answer was sent.
    pub last_at: DateTime<Utc>,
}

/// One answer a visitor sent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct FormAnswer {
    /// Its identity: a later answer has a larger number.
    pub id: i64,
    /// The address of the page the form was on.
    #[schema(example = "/blog/write-to-us/")]
    pub page: String,
    /// When it was sent.
    pub sent_at: DateTime<Utc>,
    /// Each field in the order the form sent them.
    pub fields: Vec<AnswerField>,
}

/// One field of an answer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct AnswerField {
    /// The field's name in the form block.
    #[schema(example = "email")]
    pub name: String,
    /// What the visitor typed.
    #[schema(example = "lena@example.com")]
    pub value: String,
}

/// One page of a form's answers.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AnswerPage {
    /// The answers on this page, newest first.
    pub answers: Vec<FormAnswer>,
    /// How many answers the form has in all.
    pub total: u64,
}

/// Why the answers could not be read or deleted.
#[derive(Debug, thiserror::Error)]
#[error("the form answers could not be read or deleted")]
pub struct FormAnswersError(#[source] pub Box<dyn Error + Send + Sync>);
