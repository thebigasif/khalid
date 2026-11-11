//! The `khalid` command.
//!
//! This crate owns the edge: it parses the command line, runs a command, and turns every outcome
//! into an exit code and a message a person or an agent can read. It must not know business
//! rules. Those live in the library crates, which this binary calls the same way the API does.

mod account_author;
mod admin_blog;
mod agents;
mod ai_visitors;
mod alert_watch;
mod alerts;
mod announce;
mod assets;
mod atomic;
mod audit_alert;
mod audit_events;
mod audit_message;
mod backup;
mod backup_message;
mod bing;
mod bing_report;
mod build;
mod build_ai_page;
mod build_files;
mod build_gate;
mod build_home;
mod build_lists;
mod build_message;
mod build_named_pages;
mod build_pages;
mod build_reader;
mod build_redirects;
mod build_schema;
mod build_seo;
mod build_sitemap;
mod categories;
mod cli;
mod cli_access;
mod cli_data;
mod cli_import;
mod cli_name;
mod cli_parse;
mod cli_reports;
mod comment_pages;
mod completions;
mod config_message;
mod consent_route;
mod content_feed;
mod content_message;
mod crawl_alerts;
mod crawl_first;
mod crawl_report;
mod crawl_words;
mod crawler_lists;
mod db;
mod db_export;
mod db_message;
mod definition_text;
mod dispatch;
mod doctor;
mod doctor_address;
mod doctor_check;
mod doctor_files;
mod doctor_folder;
mod doctor_pictures;
mod doctor_robots;
mod draft_comment_mail;
mod entry_events;
mod entry_files;
mod exit;
mod export;
mod form_delivery;
mod form_pages;
mod home_images;
mod https_message;
mod import;
mod import_held;
mod import_rest;
mod imprint_page;
mod index;
mod init;
mod init_message;
mod init_prompt;
mod keys;
mod keys_message;
mod late_api;
mod link_graph;
mod lint;
mod lint_fix;
mod lint_input;
mod lint_message;
mod lint_text;
mod listed_post;
mod live_redirects;
mod look;
#[cfg(feature = "man")]
mod man;
mod mcp;
mod mcp_content;
mod mcp_message;
mod mcp_tools;
mod media_library;
mod memory;
mod miss_log;
mod mounted;
mod new;
mod new_author;
mod new_branch;
mod new_message;
mod new_template;
mod newsletter;
mod newsletter_cli;
mod newsletter_digest;
mod newsletter_files;
mod newsletter_send;
mod outbound;
mod outside_writes;
mod own_descriptions;
mod page_code;
mod page_counts;
mod page_styles;
mod plan;
mod plan_message;
mod post_categories;
mod post_files;
mod post_gate;
mod post_heads;
mod post_images;
mod post_lock;
mod post_photos;
mod post_text;
mod preview;
mod privacy_page;
mod prune_rows;
mod publish;
mod pull;
mod pull_message;
mod push_hook;
mod redirect_files;
mod redirect_route;
mod redirects;
mod redirects_explain;
mod repair;
mod report;
mod restore;
mod restore_message;
mod routes_message;
mod scheduler;
mod search_console;
mod search_console_email;
mod search_console_finder;
mod search_console_follow_ups;
mod search_console_lost;
mod search_console_post;
mod search_console_report;
mod search_index;
mod search_page;
mod search_route;
mod search_static;
mod seo_crawl_settings;
mod seo_extras;
mod seo_graph_settings;
mod seo_head_settings;
mod seo_settings;
mod serve;
mod serve_address;
mod serve_apis;
mod serve_args;
mod serve_check;
mod serve_https;
mod serve_message;
mod serve_sites;
mod server_log;
mod settings_events;
mod settings_text;
mod share_images;
mod site_addresses;
mod site_audit;
mod site_briefs;
mod site_categories;
mod site_chrome;
mod site_config;
mod site_content;
mod site_entries;
mod site_facts;
mod site_files;
mod site_folders;
mod site_hosts;
mod site_html;
mod site_list;
mod site_lists;
mod site_outline;
mod site_page;
mod site_pages;
mod site_posts;
mod site_schema_templates;
mod site_settings;
mod site_tags;
mod site_watch;
mod sites;
mod sites_https;
mod sites_message;
mod slug_moves;
mod slug_repair;
mod staff;
mod staff_message;
mod staff_owner;
mod staff_toolbar;
mod static_mode;
mod subscriber_events;
mod tag_events;
mod theme_check;
mod theme_message;
mod theme_preview;
mod theme_sync;
mod trails;
mod uncited_report;
mod unlock;
mod update;
mod update_message;
mod update_release;
mod vector_size;
mod visitor_log;
mod watch_report;
mod webhooks;

use std::ffi::OsString;
use std::path::Path;
use std::process::ExitCode;

use clap::{CommandFactory, Parser};
use khalid_content::{ContentTree, SiteContent, check_bodies, read_content};
use khalid_core::{Config, Environment, UserMessage};
use khalid_routes::Routes;
use khalid_server::FolderLook;

use crate::cli::{Cli, Command, Output, ServeArgs};
use crate::dispatch::needs_site;
use crate::exit::Exit;
use crate::post_files::Assets;

#[global_allocator]
static ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> ExitCode {
    let args: Vec<OsString> = std::env::args_os().collect();
    let exit = match Cli::try_parse_from(&args) {
        Ok(cli) => run(&cli.here()),
        Err(error) => cli_parse::parse_failure(&error, Output::requested_in(&args)),
    };
    exit.into()
}

/// Runs the command the person asked for, or prints the help when they
/// asked for none.
fn run(cli: &Cli) -> Exit {
    match &cli.command {
        None => report::text(&Cli::command().render_help().to_string(), Exit::Success),
        Some(command) => dispatch::run(cli, &Environment::from_process(), command),
    }
}

/// Runs `khalid serve`, which is `command`: every site in a sites folder,
/// or the one site `--site` names. With `--init`, makes that site first
/// when the folder holds none.
fn serve_site(cli: &Cli, env: &Environment, command: &Command, args: &ServeArgs) -> Exit {
    let root = cli.site.as_deref().unwrap_or(Path::new("."));
    if site_list::file(root).is_file() {
        return serve_sites::run(root, cli, args);
    }
    let Some(folder) = cli.site.as_deref() else {
        return report::failure(cli.output, &needs_site(command), Exit::Usage);
    };
    if args.init
        && let Err(message) = init::first_start(folder, env)
    {
        return report::failure(cli.output, &message, Exit::Failure);
    }
    // Small memory pages only, and the server's watch starts from a look
    // taken before the content is read, so a post saved while the server
    // starts still shows.
    memory::small_pages_only();
    let look = FolderLook::take(folder);
    match check(folder, env, Bodies::ByTheCommand) {
        // The server renders its first pages from the checked content, then
        // drops it, so it takes the content rather than borrowing it.
        Ok((config, routes, content, assets)) => {
            let keys = serve::Keys::read(env, (folder, &config), cli);
            let start = (content, assets, look);
            serve_https::serve(cli, folder, (&config, &routes), start, (args, keys))
        }
        Err(message) => report::failure(cli.output, &message, Exit::Failure),
    }
}

/// What one check of a site folder read: its settings, routes and content,
/// and the files beside its folder posts.
type Checked = (Config, Routes, SiteContent, Assets);

/// When the bodies of a site's content are checked for broken wikilinks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bodies {
    /// With the rest of the content, before the command runs.
    Now,
    /// By the command itself: `khalid serve` checks them beside its start
    /// and still refuses to listen when one is broken.
    ByTheCommand,
}

/// Checks the settings, the routes and the content of the `site` folder, and
/// returns them with the files beside folder posts. The folder is walked
/// once, and no file's text is kept beyond the content.
fn check(site: &Path, environment: &Environment, bodies: Bodies) -> Result<Checked, UserMessage> {
    let config = site_config::load(site, environment)?;
    let routes = Routes::load(site).map_err(|error| routes_message::of(&error))?;
    let (content, assets) = ContentTree::walk(site)
        .and_then(|tree| Ok((read_content(&tree)?, Assets::of(&tree))))
        .and_then(|read| match bodies {
            Bodies::Now => check_bodies(&read.0, &config, &routes).map(|()| read),
            Bodies::ByTheCommand => Ok(read),
        })
        .map_err(|error| content_message::of(site, &error))?;
    Ok((config, routes, content, assets))
}
