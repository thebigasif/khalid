//! The consent cookie read and written, and a page rendered for a reader
//! who has not answered, filled for each kind of reader (privacy plan U3,
//! KTD3, KTD5, KTD9).

use axum::http::{HeaderMap, HeaderValue, header};
use khalid_server::{
    Answer, Categories, ConsentSite, HeldCode, Reader, SavedNotice, filled, under_gpc,
};

const REV: &str = "1a2b3c4d";

const PAGE: &str = "<head><!--khalid-held--><script nonce=\"s\">post()</script><!--/khalid-held--><!--khalid-site-head--></head><body><!--khalid-saved--><template><p role=\"status\">Saved.</p></template><!--/khalid-saved--><p>Text.</p><!--khalid-media-card--><div class=\"embed-card\">Card</div><!--/khalid-media-card--><!--khalid-media--><template><iframe src=\"https://www.youtube-nocookie.com/embed/x\"></iframe></template><!--/khalid-media--><!--khalid-site-body--><!--khalid-ask--><aside>Allow?<!--khalid-no-gpc--><label>Marketing</label><!--/khalid-no-gpc--><!--khalid-gpc--><template><p>GPC honoured.</p></template><!--/khalid-gpc--></aside><!--/khalid-ask--><!--khalid-chosen--><template><aside>Change</aside></template><!--/khalid-chosen--></body>";

#[cfg(test)]
fn site() -> ConsentSite {
    let held = HeldCode::new()
        .with(Categories::ANALYTICS, "<script>stats()</script>", "")
        .with(
            Categories::MARKETING,
            "<script>ads()</script>",
            "<script>chat()</script>",
        );
    ConsentSite::new(REV, held, Categories::MARKETING)
}

#[cfg(test)]
fn headers(cookie: Option<&str>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    if let Some(cookie) = cookie {
        headers.insert(header::COOKIE, HeaderValue::from_str(cookie).unwrap());
    }
    headers
}

/// A reader with `cookie` whose browser sends no GPC.
#[cfg(test)]
fn reader(cookie: Option<&str>) -> Reader {
    Reader::of(&headers(cookie), REV)
}

/// A reader with `cookie` whose browser sends GPC.
#[cfg(test)]
fn with_gpc(cookie: Option<&str>) -> Reader {
    let mut headers = headers(cookie);
    headers.insert("sec-gpc", HeaderValue::from_static("1"));
    Reader::of(&headers, REV)
}

#[cfg(test)]
fn answer(cookie: &str) -> Option<Answer> {
    reader(Some(cookie)).answer()
}

#[test]
fn the_cookie_keeps_categories_day_and_flags() {
    let mut given = Answer::new(Categories::ANALYTICS.with(Categories::MARKETING), 20_400);
    given.over_gpc = true;
    let value = given.value(REV);
    assert_eq!(value, "2.1a2b3c4d.6.20400.1");
    assert_eq!(answer(&format!("a=1; khalid-consent={value}")), Some(given));
    let media = Answer {
        media_only: true,
        ..Answer::new(Categories::EXTERNAL_MEDIA, 20_400)
    };
    assert_eq!(media.value(REV), "2.1a2b3c4d.8.20400.2");
}

#[test]
fn a_cookie_for_other_settings_or_out_of_shape_is_no_answer() {
    for cookie in [
        "khalid-consent=2.ffffffff.6.20400.0",
        "khalid-consent=2.1a2b3c4d.16.20400.0",
        "khalid-consent=2.1a2b3c4d.6.20400.4",
        "khalid-consent=2.1a2b3c4d.6.20400",
        "khalid-consent=3.1a2b3c4d.6.20400.0",
        "khalid-consent=maybe",
        "not-khalid-consent=no",
    ] {
        assert_eq!(answer(cookie), None, "{cookie}");
    }
    assert_eq!(reader(None).answer(), None);
}

#[test]
fn an_old_no_is_reject_all_and_an_old_yes_asks_again() {
    assert_eq!(
        answer("khalid-consent=no"),
        Some(Answer::new(Categories::NONE, 0))
    );
    assert_eq!(answer("khalid-consent=yes"), None);
}

#[test]
fn gpc_keeps_locked_categories_off_unless_the_reader_turned_them_on() {
    let all = Answer::new(Categories::ALL, 1).value(REV);
    let cookie = format!("khalid-consent={all}");
    let locked = Categories::MARKETING;
    assert_eq!(reader(Some(&cookie)).released(locked), Categories::ALL);
    let gpc_reader = with_gpc(Some(&cookie));
    assert!(gpc_reader.gpc());
    assert_eq!(
        gpc_reader.released(locked),
        Categories::ALL.without(Categories::MARKETING)
    );
    let mut chosen = Answer::new(Categories::MARKETING, 1);
    chosen.over_gpc = true;
    let cookie = format!("khalid-consent={}", chosen.value(REV));
    assert_eq!(
        with_gpc(Some(&cookie)).released(locked),
        Categories::MARKETING
    );
    let mut headers = HeaderMap::new();
    headers.insert("sec-gpc", HeaderValue::from_static("0"));
    assert!(!Reader::of(&headers, REV).gpc());
}

#[test]
fn a_reader_who_has_not_answered_gets_the_banner_and_no_code() {
    let html = filled(PAGE, &reader(None), &site(), SavedNotice::Shown);
    assert!(html.contains("<!--khalid-ask--><aside>Allow?"), "{html}");
    assert!(html.contains("<label>Marketing</label>"), "{html}");
    assert!(!html.contains("()"), "{html}");
    assert!(html.contains("<template><iframe"), "the player stays inert");
    assert!(html.contains("<template><p role"), "no saved notice");
}

#[test]
fn the_gpc_copy_drops_the_marketing_row_and_says_the_signal_was_honoured() {
    let html = under_gpc(PAGE);
    assert!(!html.contains("Marketing"), "{html}");
    assert!(
        html.contains("<aside>Allow?<p>GPC honoured.</p></aside>"),
        "{html}"
    );
    let without_held = PAGE.replace(
        "<!--khalid-held--><script nonce=\"s\">post()</script><!--/khalid-held-->",
        "",
    );
    assert_eq!(
        filled(PAGE, &with_gpc(None), &site(), SavedNotice::Hidden),
        under_gpc(&without_held)
    );
}

#[test]
fn a_reader_who_chose_analytics_gets_only_analytics_code() {
    let value = Answer::new(Categories::ANALYTICS, 1).value(REV);
    let cookie = format!("khalid-consent={value}");
    let html = filled(PAGE, &reader(Some(&cookie)), &site(), SavedNotice::Shown);
    assert_eq!(
        html,
        "<head><script>stats()</script></head><body><p role=\"status\">Saved.</p><p>Text.</p><!--khalid-media-card--><div class=\"embed-card\">Card</div><!--/khalid-media-card--><!--khalid-media--><template><iframe src=\"https://www.youtube-nocookie.com/embed/x\"></iframe></template><!--/khalid-media--><aside>Change</aside></body>"
    );
}

#[test]
fn a_reader_who_allowed_everything_gets_every_piece_and_the_players() {
    let value = Answer::new(Categories::ALL, 1).value(REV);
    let cookie = format!("khalid-consent={value}");
    let html = filled(PAGE, &reader(Some(&cookie)), &site(), SavedNotice::Hidden);
    assert_eq!(
        html,
        "<head><!--khalid-held--><script nonce=\"s\">post()</script><!--/khalid-held--><script>stats()</script>\n<script>ads()</script></head><body><!--khalid-saved--><template><p role=\"status\">Saved.</p></template><!--/khalid-saved--><p>Text.</p><iframe src=\"https://www.youtube-nocookie.com/embed/x\"></iframe><script>chat()</script><aside>Change</aside></body>"
    );
}

#[test]
fn allowing_external_media_from_a_card_loads_players_and_keeps_the_banner() {
    let media = Answer {
        media_only: true,
        ..Answer::new(Categories::EXTERNAL_MEDIA, 1)
    };
    let cookie = format!("khalid-consent={}", media.value(REV));
    let html = filled(PAGE, &reader(Some(&cookie)), &site(), SavedNotice::Hidden);
    assert!(html.contains("<p>Text.</p><iframe"), "{html}");
    assert!(
        !html.contains("embed-card"),
        "the card goes with its player: {html}"
    );
    assert!(html.contains("<!--khalid-ask--><aside>Allow?"), "{html}");
    assert!(!html.contains("()"), "{html}");
}

#[test]
fn an_old_no_gets_the_change_form_and_no_code() {
    let html = filled(
        PAGE,
        &reader(Some("khalid-consent=no")),
        &site(),
        SavedNotice::Hidden,
    );
    assert!(!html.contains("Allow?") && !html.contains("()"), "{html}");
    assert!(html.contains("<aside>Change</aside>"), "{html}");
}

#[test]
fn held_code_never_closed_is_cut_to_the_end() {
    let html = filled(
        "<p>a</p><!--khalid-held--><script>x()",
        &reader(None),
        &site(),
        SavedNotice::Hidden,
    );
    assert_eq!(html, "<p>a</p>");
    let html = filled(
        "<p>a</p><!--khalid-media-card--><div class=\"embed-card\">Card</div><!--/khalid-media-card--><!--khalid-media--><template><iframe>",
        &reader(Some("khalid-consent=2.1a2b3c4d.8.1.0")),
        &site(),
        SavedNotice::Hidden,
    );
    assert_eq!(html, "<p>a</p>");
}
