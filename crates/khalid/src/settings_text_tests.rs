//! Settings written into a post's front matter read back the same.

use std::path::Path;

use chrono::NaiveDate;
use khalid_api::resources::{
    CustomJsonLd, FeatureImage, MAX_SECONDARY_KEYWORDS, NonBlank, PostSettings, PostStatus,
    ReviewState, SearchTitle, ShareCard,
};
use khalid_content::parse_post;
use khalid_core::Slug;

use super::{settings_of, write_settings};
use crate::post_text::PostText;

const FILE: &str = "---\ntitle: Choosing anchor text\nauthor: asif\n---\n\nBody.\n";

/// Writes `after` over the settings of [`FILE`] and reads the file back.
fn round_trip(after: &PostSettings) -> (String, PostSettings) {
    let before = settings_of(&parse_post(Path::new("a.md"), FILE).unwrap().front);
    let mut text = PostText::parse(FILE).unwrap();
    write_settings(&mut text, &before, after, PostStatus::Draft).unwrap();
    let written = text.to_text();
    let read = settings_of(&parse_post(Path::new("a.md"), &written).unwrap().front);
    (written, read)
}

/// Text that is not blank.
fn text(words: &str) -> NonBlank {
    NonBlank::try_new(words).unwrap()
}

#[test]
fn co_authors_image_review_state_and_share_cards_read_back_the_same() {
    let mut after = PostSettings::default();
    after.co_authors = vec![Slug::try_new("lena-vogt").unwrap()];
    let mut image = FeatureImage::new(text("cover.jpg"), text("Two \"linked\" pages"));
    image.caption = Some(text("Links: the short version."));
    after.feature_image = Some(image);
    after.review_state = ReviewState::InReview;
    let mut card = ShareCard::default();
    card.title = Some(text("Anchor text in five minutes"));
    card.image = Some(text("card.png"));
    after.share_cards.x = Some(card);
    let (written, read) = round_trip(&after);
    assert_eq!(read, after, "{written}");
    assert!(written.contains("co_authors: [lena-vogt]\n"), "{written}");
    assert!(written.contains("review_state: in_review\n"), "{written}");
}

#[test]
fn a_recorded_review_leaves_the_dates_alone() {
    // A review sets its day and nothing else: `date` and the updated
    // marker keep exactly the values they had (F3).
    let before = settings_of(&parse_post(Path::new("a.md"), FILE).unwrap().front);
    let mut after = before.clone();
    after.reviewed = Some(NaiveDate::from_ymd_opt(2026, 10, 29).unwrap());
    let mut text = PostText::parse(FILE).unwrap();
    write_settings(&mut text, &before, &after, PostStatus::Draft).unwrap();
    let written = text.to_text();
    // The review day is written...
    assert!(written.contains("reviewed: 2026-10-29\n"), "{written}");
    // ...and the dated line, present before, is unchanged.
    let dated = parse_post(Path::new("a.md"), &written).unwrap().front;
    assert_eq!(
        dated.date,
        parse_post(Path::new("a.md"), FILE).unwrap().front.date
    );
    assert_eq!(
        dated.updated,
        parse_post(Path::new("a.md"), FILE).unwrap().front.updated
    );
    // What is written reads back as the settings that were asked for.
    assert_eq!(settings_of(&dated), after, "{written}");
}

#[test]
fn settings_put_back_to_their_defaults_leave_no_line() {
    let mut set = PostSettings::default();
    set.co_authors = vec![Slug::try_new("lena-vogt").unwrap()];
    set.review_state = ReviewState::Ready;
    set.share_cards.facebook = Some(ShareCard::default());
    let (written, _) = round_trip(&set);
    let mut text = PostText::parse(&written).unwrap();
    let defaults = PostSettings::default();
    write_settings(&mut text, &set, &defaults, PostStatus::Draft).unwrap();
    assert_eq!(text.to_text(), FILE);
}

#[test]
fn secondary_keywords_read_back_the_same_as_a_quoted_list() {
    let mut after = PostSettings::default();
    after.focus_keyword = Some(text("anchor text"));
    after.secondary_keywords = vec![text("link text"), text("words: that link")];
    let (written, read) = round_trip(&after);
    assert_eq!(read, after, "{written}");
    let line = "secondary_keywords: [\"link text\", \"words: that link\"]\n";
    assert!(written.contains(line), "{written}");
    assert_eq!(
        MAX_SECONDARY_KEYWORDS,
        khalid_content::MAX_SECONDARY_KEYWORDS
    );
}

#[test]
fn search_title_focus_keyword_and_schema_read_back_the_same() {
    let mut after = PostSettings::default();
    let title = "Anchor text: \"say where\" {sep} {site}";
    after.seo_title = Some(SearchTitle::try_new(title).unwrap());
    after.focus_keyword = Some(text("anchor text"));
    let schema = "{\n  \"@type\": \"Event\",\n  \"name\": \"Anchor text clinic\"\n}";
    after.schema = Some(CustomJsonLd::try_new(schema).unwrap());
    let (written, read) = round_trip(&after);
    assert_eq!(read, after, "{written}");
    assert!(
        written.contains("focus_keyword: \"anchor text\"\n"),
        "{written}"
    );
}

#[test]
fn a_settings_edit_keeps_the_facts_as_written() {
    let file = "---\ntitle: A\nfacts:\n  - label: Client\n    value: Calder Street Library\nfeatured: true\n---\n\nBody.\n";
    let before = settings_of(&parse_post(Path::new("a.md"), file).unwrap().front);
    let mut after = before.clone();
    after.featured = false;
    after.excerpt = Some(text("A library's new reading room."));
    let mut text = PostText::parse(file).unwrap();
    write_settings(&mut text, &before, &after, PostStatus::Draft).unwrap();
    let written = text.to_text();
    assert!(
        written.contains("facts:\n  - label: Client\n    value: Calder Street Library\n"),
        "{written}"
    );
    let front = parse_post(Path::new("a.md"), &written).unwrap().front;
    assert_eq!(front.facts.len(), 1, "{written}");
}
