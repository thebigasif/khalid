//! `khalid redirects explain`: why an address answers what it answers on
//! the running site, from post aliases and stored redirects, in the order
//! `khalid serve` tries them (KH-SEO-8).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use khalid_api::redirect_desk::RedirectDesk;
use khalid_content::{ContentTree, read_content};
use khalid_core::{Environment, SystemClock, UserMessage};
use khalid_routes::Routes;
use khalid_seo::{RedirectEntry, RedirectRule, RedirectTable, UrlPath};
use serde::Serialize;

use crate::build_redirects::{self, Hop};
use crate::cli::Output;
use crate::exit::Exit;
use crate::redirects::{self, Done};
use crate::{build_gate, build_message, content_message, report, routes_message, site_config};

/// The JSON shape of `khalid redirects explain --json`.
#[derive(Serialize)]
struct ExplainedBody {
    path: String,
    status: Option<u16>,
    to: Option<String>,
    text: String,
}

/// The aliases in the `site` folder's post headers: those `khalid serve`
/// answers, and those it holds back while the linter blocks their post.
#[derive(Debug, Default)]
pub(crate) struct Aliases {
    live: Vec<Hop>,
    held: Vec<(String, PathBuf)>,
}

impl Aliases {
    /// Reads the settings, routes and posts of `site`, and judges each
    /// dated post with the publish gate, as `khalid serve` does. A site
    /// with no `content` folder has no aliases.
    ///
    /// # Errors
    ///
    /// What, why and fix when the settings, routes, posts or lint
    /// settings cannot be read, or two posts claim one alias.
    pub(crate) fn read(site: &Path) -> Result<Self, UserMessage> {
        let mut folder = site.to_path_buf();
        folder.push("content");
        if !folder.is_dir() {
            return Ok(Self::default());
        }
        let config = site_config::load(site, &Environment::from_process())?;
        let routes = Routes::load(site).map_err(|error| routes_message::of(&error))?;
        let content = ContentTree::walk(site)
            .and_then(|tree| read_content(&tree))
            .map_err(|error| content_message::of(site, &error))?;
        let checked = (&config, &routes, &content);
        let refused = build_gate::screen(site, checked, &SystemClock)?;
        let held = content
            .posts
            .iter()
            .filter(|doc| refused.iter().any(|post| post.path == doc.path))
            .flat_map(|doc| {
                doc.front
                    .aliases
                    .iter()
                    .map(|a| (a.clone(), doc.path.clone()))
            })
            .collect();
        let published = build_gate::published(&content, &refused);
        let live = build_redirects::hops(&config, &routes, &published)?;
        Ok(Self { live, held })
    }

    fn live(&self, path: &str) -> Option<&Hop> {
        self.live.iter().find(|hop| hop.from == path)
    }

    fn held(&self, path: &str) -> Option<&Path> {
        let found = self.held.iter().find(|(alias, _)| alias == path);
        found.map(|(_, post)| post.as_path())
    }
}

/// Explains `path` from the stored redirects on `desk` and the `aliases`.
pub(crate) async fn explain(
    desk: &RedirectDesk,
    path: &UrlPath,
    aliases: &Aliases,
    output: Output,
) -> Done {
    let stored = desk
        .table()
        .await
        .map_err(|refusal| (redirects::refused(&refusal), Exit::Failure))?;
    let table = joined(&stored, &aliases.live).map_err(|message| (message, Exit::Failure))?;
    let explained = table.explain(path);
    let mut lines: Vec<String> = explained
        .hops
        .iter()
        .filter_map(|hop| {
            let alias = aliases.live(hop.from.as_str())?;
            let post = alias.post.display();
            Some(format!(
                "{} is an alias in the header of {post}.\n",
                hop.from
            ))
        })
        .collect();
    lines.push(explained.to_string());
    let held = explained
        .answer
        .is_none()
        .then(|| aliases.held(path.as_str()));
    if let Some(post) = held.flatten() {
        lines.push(format!(
            "It is an alias of {}, which the linter blocks, so it leads to the not-found page \
             until that post passes. Run khalid lint to see why.\n",
            post.display()
        ));
    }
    let text = lines.concat();
    Ok(match output {
        Output::Json => {
            let answer = explained.answer.as_ref();
            let body = ExplainedBody {
                path: path.to_string(),
                status: answer.map(khalid_seo::Redirect::status),
                to: answer.and_then(|a| a.location()).map(ToString::to_string),
                text,
            };
            report::json(&body, Exit::Success)
        }
        Output::Human => report::text(&text, Exit::Success),
    })
}

/// The table `khalid serve` answers from: the `stored` exact rules an
/// alias does not shadow, then the `live` aliases, then the stored
/// patterns. Stored rules keep their numbers when no alias shadows one.
fn joined(stored: &RedirectTable, live: &[Hop]) -> Result<RedirectTable, UserMessage> {
    let aliased: BTreeSet<&str> = live.iter().map(|hop| hop.from.as_str()).collect();
    let mut rules: Vec<RedirectRule> = stored
        .rules()
        .iter()
        .filter(|rule| !aliased.contains(rule.from.as_str()))
        .cloned()
        .collect();
    for hop in live {
        rules.push(hop.rule()?);
    }
    let patterns = stored
        .entries()
        .into_iter()
        .filter_map(|entry| match entry {
            RedirectEntry::Pattern(pattern) => Some(pattern),
            _ => None,
        });
    let table = RedirectTable::new(rules).map_err(|error| build_message::seo(&error))?;
    Ok(table.with_patterns(patterns.collect()))
}
