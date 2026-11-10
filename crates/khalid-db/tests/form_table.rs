//! Form answers on SQLite come back as they went in, in order, and each
//! site sees only its own (KH-GRO-1). The admin reads them back a form
//! and a page at a time, newest first.

use std::path::PathBuf;

use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use khalid_api::form_answers::{AnswerField, AnswerPage, FormAnswers, FormSummary};
use khalid_core::Slug;
use khalid_db::{Database, FormTable};
use khalid_forms::{FormId, FormStore, Submission};

#[cfg(test)]
fn at(minutes: i64) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, 31, 9, 0, 0).unwrap() + TimeDelta::minutes(minutes)
}

/// A migrated database with the form answers of the sites `demo` and
/// `other`.
#[cfg(test)]
async fn opened() -> (tempfile::TempDir, FormTable, FormTable) {
    let dir = tempfile::tempdir().unwrap();
    let mut path = PathBuf::from(dir.path());
    path.push("site.db");
    let db = Database::open(&path).await.unwrap();
    db.migrate().await.unwrap();
    let demo = db.site(&Slug::try_new("demo").unwrap()).await.unwrap();
    let other = db.site(&Slug::try_new("other").unwrap()).await.unwrap();
    let tables = (FormTable::new(db.clone(), demo), FormTable::new(db, other));
    (dir, tables.0, tables.1)
}

#[cfg(test)]
fn sent(form: &str, fields: &[(&str, &str)]) -> Submission {
    Submission {
        form: FormId::try_new(form).unwrap(),
        page: "/blog/write-to-us/".to_owned(),
        fields: fields
            .iter()
            .map(|&(name, value)| (name.to_owned(), value.to_owned()))
            .collect(),
    }
}

#[tokio::test]
async fn answers_come_back_in_the_order_they_were_sent() {
    let (_dir, forms, _) = opened().await;
    let first = sent(
        "contact",
        &[
            ("name", "Lena Vogt"),
            ("message", "Quotes \" and 'ticks', a {brace} and ü."),
            ("reply_to", "lena@example.com"),
        ],
    );
    let second = sent("contact", &[("message", "A second note.")]);
    let survey = sent("survey", &[("answer", "Yes")]);
    forms.insert(&second, at(5)).await.unwrap();
    forms.insert(&first, at(0)).await.unwrap();
    forms.insert(&survey, at(1)).await.unwrap();

    let contact = FormId::try_new("contact").unwrap();
    let kept = forms.of_form(&contact).await.unwrap();
    assert_eq!(kept, [(first, at(0)), (second, at(5))]);
}

#[tokio::test]
async fn a_site_never_sees_another_site_s_answers() {
    let (_dir, demo, other) = opened().await;
    demo.insert(&sent("contact", &[("name", "Ada")]), at(0))
        .await
        .unwrap();
    let contact = FormId::try_new("contact").unwrap();
    assert_eq!(other.of_form(&contact).await.unwrap(), []);
    assert_eq!(demo.of_form(&contact).await.unwrap().len(), 1);
}

#[tokio::test]
async fn the_admin_lists_each_form_the_one_answered_last_first() {
    let (_dir, demo, other) = opened().await;
    demo.insert(&sent("contact", &[("name", "Ada")]), at(0))
        .await
        .unwrap();
    demo.insert(&sent("survey", &[("answer", "Yes")]), at(3))
        .await
        .unwrap();
    demo.insert(&sent("contact", &[("name", "Lena")]), at(9))
        .await
        .unwrap();
    other
        .insert(&sent("quote", &[("name", "Mo")]), at(20))
        .await
        .unwrap();

    let forms = FormAnswers::forms(&demo).await.unwrap();
    let summary = |id: &str, answers, minutes| FormSummary {
        id: id.to_owned(),
        answers,
        last_at: at(minutes),
    };
    assert_eq!(forms, [summary("contact", 2, 9), summary("survey", 1, 3)]);
}

#[tokio::test]
async fn the_admin_reads_answers_a_page_at_a_time_newest_first() {
    let (_dir, demo, other) = opened().await;
    for minute in 0..5 {
        let message = format!("Note {minute}");
        let fields = [("email", "lena@example.com"), ("message", message.as_str())];
        demo.insert(&sent("contact", &fields), at(minute))
            .await
            .unwrap();
    }

    let page = FormAnswers::answers(&demo, "contact", 1, 2).await.unwrap();
    assert_eq!(page.total, 5);
    let times: Vec<_> = page.answers.iter().map(|answer| answer.sent_at).collect();
    assert_eq!(times, [at(3), at(2)]);
    let first = &page.answers[0];
    assert_eq!(first.page, "/blog/write-to-us/");
    let field = |name: &str, value: &str| AnswerField {
        name: name.to_owned(),
        value: value.to_owned(),
    };
    assert_eq!(
        first.fields,
        [
            field("email", "lena@example.com"),
            field("message", "Note 3")
        ]
    );

    let past_the_end = FormAnswers::answers(&demo, "contact", 10, 2).await.unwrap();
    assert_eq!((past_the_end.answers.len(), past_the_end.total), (0, 5));
    let elsewhere = FormAnswers::answers(&other, "contact", 0, 25)
        .await
        .unwrap();
    assert_eq!(elsewhere, AnswerPage::default());
    let no_form = FormAnswers::answers(&demo, "../Contact", 0, 25)
        .await
        .unwrap();
    assert_eq!(no_form, AnswerPage::default());
}

#[tokio::test]
async fn an_answer_deleted_on_request_is_gone_and_only_that_one() {
    let (_dir, demo, other) = opened().await;
    demo.insert(&sent("contact", &[("email", "lena@example.com")]), at(0))
        .await
        .unwrap();
    demo.insert(&sent("contact", &[("email", "ada@example.com")]), at(1))
        .await
        .unwrap();
    other
        .insert(&sent("contact", &[("email", "mo@example.com")]), at(2))
        .await
        .unwrap();
    let page = FormAnswers::answers(&demo, "contact", 0, 25).await.unwrap();
    let ada = page.answers[0].id;
    let lena = page.answers[1].id;
    let mo = FormAnswers::answers(&other, "contact", 0, 25)
        .await
        .unwrap()
        .answers[0]
        .id;

    // Another site's answer, the wrong form, or a name no form block can
    // have deletes nothing.
    assert!(!FormAnswers::delete(&demo, "contact", mo).await.unwrap());
    assert!(!FormAnswers::delete(&demo, "survey", lena).await.unwrap());
    assert!(
        !FormAnswers::delete(&demo, "../Contact", lena)
            .await
            .unwrap()
    );

    assert!(FormAnswers::delete(&demo, "contact", lena).await.unwrap());
    assert!(!FormAnswers::delete(&demo, "contact", lena).await.unwrap());
    let left = FormAnswers::answers(&demo, "contact", 0, 25).await.unwrap();
    let ids: Vec<i64> = left.answers.iter().map(|answer| answer.id).collect();
    assert_eq!((ids, left.total), (vec![ada], 1));
    let contact = FormId::try_new("contact").unwrap();
    assert_eq!(other.of_form(&contact).await.unwrap().len(), 1);
}
