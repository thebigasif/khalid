//! `khalid search-console email`: the weekly report in plain words, sent
//! to the address in `report_to` the way `[mail]` says (plan 5.11).
//! `khalid serve` sends it once a week on its own; the day it last went
//! out is kept in `data/search-console-email.txt`, so a restart, or this
//! command run by hand, does not send it twice.

use std::fmt::Write as _;
use std::path::Path;
use std::sync::Arc;
use std::{fs, io};

use chrono::{Days, NaiveDate};
use khalid_audit::history;
use khalid_content::SiteContent;
use khalid_core::{Config, Environment, UserMessage};
use khalid_newsletter::{EmailAddress, Mail, Mailer};
use khalid_outbound::Outbound;
use khalid_search_console::{Question, SearchConsole, go_deeper};

use crate::bing::{Failed, block_on};
use crate::cli::Output;
use crate::exit::Exit;
use crate::search_console_report::{ReportBody, gather};
use crate::{atomic, newsletter, report, serve_message};

/// Days between two weekly emails.
const WEEK: u64 = 7;

/// Gathers the week's numbers for the checked `site` and mails them to
/// `report_to`, with the mail secret from `env`.
pub(crate) fn run(
    site: (&Path, &Config, &SiteContent),
    env: &Environment,
    asked: (&SearchConsole, &Outbound, NaiveDate),
    output: Output,
) -> Result<Exit, Failed> {
    let (to, mailer) = mail_of(site.1, env, asked.1)?;
    send(site, (&to, mailer.as_ref()), asked)?;
    let sent = format!("The weekly Search Console email went to {to}.\n");
    Ok(match output {
        Output::Json => report::json(
            &serde_json::json!({ "sent_to": to.to_string() }),
            Exit::Success,
        ),
        Output::Human => report::text(&sent, Exit::Success),
    })
}

/// Who gets the weekly email of the site with `config`, and the way it
/// leaves, signed in with the mail secret from `env`.
pub(crate) fn mail_of(
    config: &Config,
    env: &Environment,
    outbound: &Outbound,
) -> Result<(EmailAddress, Arc<dyn Mailer>), Failed> {
    let report_to = config
        .search_console()
        .and_then(|settings| settings.report_to.as_deref());
    let (Some(to), Some(mail)) = (report_to, config.mail()) else {
        return Err((no_one(), Exit::Failure));
    };
    let to = EmailAddress::parse(to).map_err(|_| (no_one(), Exit::Failure))?;
    let secret = env.mail_secret(&mail.transport);
    let mailer = newsletter::mailer(config, secret, outbound)
        .ok()
        .flatten()
        .ok_or_else(|| (serve_message::internal(), Exit::Internal))?;
    Ok((to, mailer))
}

/// Gathers the week's numbers for the checked `site` and mails them `to`
/// through `mailer`, then keeps the day as the last one it went out.
pub(crate) fn send(
    site: (&Path, &Config, &SiteContent),
    (to, mailer): (&EmailAddress, &dyn Mailer),
    asked: (&SearchConsole, &Outbound, NaiveDate),
) -> Result<(), Failed> {
    let (folder, config, _) = site;
    let body = gather(site, asked)?;
    let subject = format!("{}: your week in Google search", config.name());
    let text = weekly(config, &body, &score(folder));
    let letter = Mail::new(to.clone(), subject, text);
    block_on(mailer.send(&letter))?.map_err(|_| (not_sent(), Exit::Failure))?;
    if let Err(error) = keep_sent(folder, asked.2) {
        tracing::warn!(%error, "the day the weekly email went out could not be kept");
    }
    Ok(())
}

/// Whether the weekly email of the site in `folder` is due on `today`: it
/// never went out, or last went out a week or more before.
pub(crate) fn due(folder: &Path, today: NaiveDate) -> bool {
    let last = fs::read_to_string(sent_file(folder))
        .ok()
        .and_then(|text| text.trim().parse::<NaiveDate>().ok());
    last.and_then(|day| day.checked_add_days(Days::new(WEEK)))
        .is_none_or(|next| next <= today)
}

/// Keeps `day` as the last day the weekly email of the site in `folder`
/// went out.
fn keep_sent(folder: &Path, day: NaiveDate) -> io::Result<()> {
    let file = sent_file(folder);
    if let Some(data) = file.parent() {
        fs::create_dir_all(data)?;
    }
    atomic::replace(&file, &format!("{day}\n"))
}

/// The file that holds the last day the weekly email went out.
fn sent_file(folder: &Path) -> std::path::PathBuf {
    let mut file = folder.to_path_buf();
    file.extend(["data", "search-console-email.txt"]);
    file
}

/// The site's health score from its last `khalid audit`, in a sentence.
fn score(folder: &Path) -> String {
    let mut runs = folder.to_path_buf();
    runs.push("data/audit/runs.jsonl");
    let last = fs::read_to_string(&runs)
        .ok()
        .and_then(|text| history(&text).pop());
    match last {
        Some(run) => format!(
            "Site health score: {} out of 100, from the audit on {}.",
            run.score,
            run.at.date_naive()
        ),
        None => "No site audit has run yet. Run khalid audit for the health score.".to_owned(),
    }
}

/// The weekly email's text for the site with `config`.
fn weekly(config: &Config, body: &ReportBody, score: &str) -> String {
    let mut text = format!(
        "This is {}'s week in Google search, up to {}, the last day Google has final numbers for.\n\n\
         Clicks from Google: {:.0} this week, against {:.0} the week before.\n{score}\n",
        config.name(),
        body.last,
        body.week_clicks,
        body.last_week_clicks
    );
    // Writing to a String cannot fail, so the results are dropped.
    for (heading, list) in [
        ("Gained the most clicks:", &body.winners),
        ("Lost the most clicks:", &body.losers),
    ] {
        if !list.is_empty() {
            let _ = write!(text, "\n{heading}\n");
        }
        for change in list {
            let _ = writeln!(
                text,
                "- {}: {:.0} clicks, from {:.0}.",
                change.title, change.now, change.before
            );
            if let Some(link) = &change.go_deeper {
                let _ = writeln!(text, "  Why? Go deeper in SerpDad: {link}");
            }
        }
    }
    text.push_str("\nWhat to do next:\n");
    let mut tasks = 0;
    for decay in &body.decaying {
        tasks += 1;
        let _ = writeln!(
            text,
            "- Refresh \"{}\": its clicks fell three months in a row.\n  Why? Go deeper in SerpDad: {}",
            decay.title, decay.go_deeper
        );
    }
    for page in &body.lost {
        tasks += 1;
        let _ = match page.not_found_hits {
            Some(_) => writeln!(
                text,
                "- Add a redirect for {}: it answers 404, not found, but still gets search traffic.",
                page.url
            ),
            None => writeln!(
                text,
                "- Check {}: it is marked noindex, but still gets search traffic.",
                page.url
            ),
        };
    }
    if tasks == 0 {
        text.push_str("- Nothing needs fixing this week.\n");
    }
    let next = go_deeper(Question::WhatToWriteNext, config.address().as_str());
    let _ = write!(
        text,
        "\nWhat should you write next? Go deeper in SerpDad: {next}\n\n\
         Khalid sent this because report_to under [search_console] in site.toml names you.\n"
    );
    text
}

/// The message when no one is named to get the email.
fn no_one() -> UserMessage {
    UserMessage::new(
        "No one is named to get the weekly Search Console email.",
        "Khalid sends it only to the address in report_to under [search_console], and only through [mail].",
        "Add report_to = \"you@example.com\" under [search_console], and a [mail] table, in site.toml.",
    )
}

/// The message when the mail transport did not take the email.
fn not_sent() -> UserMessage {
    UserMessage::new(
        "The weekly Search Console email could not be sent.",
        "The way [mail] in site.toml names did not take it.",
        "Check [mail] and its secret as docs/deploy.md shows under Sending email, then run the command again.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(text: &str) -> NaiveDate {
        text.parse().unwrap()
    }

    #[test]
    fn the_email_is_due_a_week_after_it_last_went_out() {
        let site = tempfile::tempdir().unwrap();
        assert!(due(site.path(), day("2026-11-10")), "never sent");

        keep_sent(site.path(), day("2026-11-10")).unwrap();
        let kept = fs::read_to_string(sent_file(site.path())).unwrap();
        assert_eq!(kept, "2026-11-10\n");
        assert!(!due(site.path(), day("2026-11-10")), "sent today");
        assert!(!due(site.path(), day("2026-11-16")), "six days on");
        assert!(due(site.path(), day("2026-11-17")), "a week on");
    }

    #[test]
    fn an_unreadable_record_counts_as_never_sent() {
        let site = tempfile::tempdir().unwrap();
        let file = sent_file(site.path());
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file, "last Monday\n").unwrap();
        assert!(due(site.path(), day("2026-11-10")));
    }
}
