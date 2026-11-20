//! The comments under `WordPress` posts come in with their post, their
//! time and their place in the thread (decision D-4).

use std::fs;
use std::path::Path;

use chrono::NaiveDate;
use khalid_core::{Config, Environment};
use khalid_import::wordpress::{CommentLeftReason, Plan, plan, read_wxr};
use khalid_routes::Routes;

/// One `wp:comment` with the fields `WordPress` writes.
#[cfg(test)]
fn comment(id: u64, parent: u64, approved: &str, kind: &str, text: &str) -> String {
    format!(
        "<wp:comment><wp:comment_id>{id}</wp:comment_id><wp:comment_author><![CDATA[Reader {id}]]></wp:comment_author><wp:comment_author_email><![CDATA[r{id}@example.com]]></wp:comment_author_email><wp:comment_date><![CDATA[2024-03-13 11:0{id}:00]]></wp:comment_date><wp:comment_date_gmt><![CDATA[2024-03-13 10:0{id}:00]]></wp:comment_date_gmt><wp:comment_content><![CDATA[{text}]]></wp:comment_content><wp:comment_approved><![CDATA[{approved}]]></wp:comment_approved><wp:comment_type><![CDATA[{kind}]]></wp:comment_type><wp:comment_parent>{parent}</wp:comment_parent></wp:comment>"
    )
}

/// The sample export with `comments` in place of its post's one comment.
#[cfg(test)]
fn planned(comments: &[String]) -> Plan {
    let dir = env!("CARGO_MANIFEST_DIR");
    let text = fs::read_to_string(format!("{dir}/fixtures/wordpress/basic.xml")).unwrap();
    let (before, rest) = text.split_once("<wp:comment>").unwrap();
    let (_, after) = rest.split_once("</wp:comment>").unwrap();
    let text = format!("{before}{}{after}", comments.concat());
    let toml = "name = \"Northfield Bakery\"\naddress = \"https://northfield.example.com\"\nmount = \"/blog/\"\nmode = \"server\"\ndefault_language = \"en\"\nlanguages = [\"en\"]\n";
    let site = Config::from_toml(Path::new("site.toml"), toml, &Environment::empty()).unwrap();
    plan(&read_wxr(&text).unwrap(), None, &site, &Routes::default())
}

#[test]
fn a_comment_comes_in_under_its_post_with_its_time_in_utc() {
    let plan = planned(&[comment(
        1,
        0,
        "1",
        "comment",
        "<p>Rye &amp; <em>time</em>.</p>",
    )]);
    let [came] = plan.comments.as_slice() else {
        panic!("{:?}", plan.comments)
    };
    assert_eq!(
        came.post,
        Path::new("content/en/posts/2024-03-12-keep-a-starter-alive.md")
    );
    assert_eq!(
        (came.author.as_str(), came.email.as_deref()),
        ("Reader 1", Some("r1@example.com"))
    );
    let at = NaiveDate::from_ymd_opt(2024, 3, 13).and_then(|day| day.and_hms_opt(10, 1, 0));
    assert_eq!(Some(came.at), at);
    assert_eq!(came.text, "Rye & time.");
    assert!(came.approved && came.parent.is_none());
}

#[test]
fn a_reply_to_a_reply_joins_the_first_comment_of_its_thread() {
    let plan = planned(&[
        comment(3, 2, "1", "comment", "Third"),
        comment(1, 0, "1", "comment", "First"),
        comment(2, 1, "0", "", "Second"),
    ]);
    let threads: Vec<(u64, Option<u64>, bool)> = plan
        .comments
        .iter()
        .map(|c| (c.id, c.parent, c.approved))
        .collect();
    assert_eq!(
        threads,
        [(1, None, true), (2, Some(1), false), (3, Some(1), true)]
    );
}

#[test]
fn spam_pings_and_binned_comments_stay_behind_and_the_report_counts_them() {
    let plan = planned(&[
        comment(1, 0, "spam", "comment", "Buy now"),
        comment(2, 1, "1", "comment", "A reply to spam"),
        comment(3, 0, "1", "pingback", "Another site linked here"),
        comment(4, 0, "trash", "comment", "Gone"),
        comment(5, 0, "0", "comment", "<p> </p>"),
    ]);
    let left: Vec<(u64, CommentLeftReason)> = plan
        .left_comments
        .iter()
        .map(|c| (c.id, c.reason.clone()))
        .collect();
    assert_eq!(
        left,
        [
            (1, CommentLeftReason::Spam),
            (3, CommentLeftReason::Ping),
            (4, CommentLeftReason::Removed),
            (5, CommentLeftReason::Empty),
        ]
    );
    assert_eq!(plan.comments.len(), 1);
    assert_eq!(plan.comments[0].parent, None, "its parent stayed behind");
    let report = plan.report().to_string();
    assert!(
        report.contains(
            "Comments coming in: 1, of which 0 wait for approval.\n\
             Comments left behind: spam (1); pingbacks and trackbacks from other sites (1); in the bin (1); with no text or no date (1).\n"
        ),
        "{report}"
    );
}
