//! The commands `main` hands on once it knows what they need: a checked
//! site, its settings, its folder or a search engine. Kept apart from
//! `main` to keep each file short (BL-SIZE-1).

use std::path::Path;

use khalid_content::SiteContent;
use khalid_core::{Config, Environment, SystemClock, UserMessage};
use khalid_routes::Routes;

use crate::cli::{AnalyticsCommand, Cli, Command, Output, ThemeCommand};
use crate::exit::Exit;
use crate::new::NewPost;
use crate::{
    ai_visitors, backup, bing, build, crawl_report, export, keys, new, newsletter_cli, plan,
    publish, pull, redirects, report, search_console, serve, site_audit, staff_owner, theme_check,
    theme_sync, uncited_report, webhooks,
};

/// Runs the commands that ask a search engine about the checked site, with
/// the keys in `env`: `bing` and `search-console`.
pub(crate) fn outside(
    (folder, config, content): (&Path, &Config, &SiteContent),
    env: &Environment,
    command: &Command,
    cli: &Cli,
) -> Exit {
    match command {
        Command::Bing { command } => bing::run(config, env, *command, cli),
        Command::SearchConsole { command } => {
            search_console::run((folder, config, content), env, command.clone(), cli)
        }
        _ => report::failure(cli.output, &not_built(command), Exit::Internal),
    }
}

/// Runs the commands that need only a checked site's settings: `theme`,
/// `publish`, `plan`, `newsletter` and `new`.
pub(crate) fn on_settings(folder: &Path, config: &Config, command: &Command, cli: &Cli) -> Exit {
    match command {
        Command::New(args) => new::run(
            folder,
            config,
            NewPost::from(args),
            &SystemClock,
            cli.output,
        ),
        Command::Newsletter { command } => newsletter_cli::run(cli, (folder, config), command),
        Command::Theme { command } => theme(folder, config, command, cli),
        Command::Publish { slug, language } => {
            publish::run(cli, (folder, config), (slug, language.as_ref()))
        }
        Command::Plan { per_week } => plan::run(cli, (folder, config), *per_week),
        _ => report::failure(cli.output, &not_built(command), Exit::Internal),
    }
}

/// Runs the commands that read a checked site's content: `build`,
/// `export`, and `audit` from a build or the server it names.
pub(crate) fn on_content(
    folder: &Path,
    checked: (&Config, &Routes, &SiteContent),
    asked: &Command,
    (cli, environment): (&Cli, &Environment),
) -> Exit {
    match asked {
        Command::Build { out, no_indexnow } => {
            build::run(folder, checked, (out.as_deref(), *no_indexnow), cli)
        }
        Command::Export => export::run(folder, checked, &SystemClock, cli.output),
        Command::Audit { server, network } => site_audit::run(
            folder,
            checked,
            (server.as_ref(), *network),
            &SystemClock,
            (cli, serve::mail_secret(environment, checked.0)),
        ),
        _ => report::failure(cli.output, &not_built(asked), Exit::Internal),
    }
}

/// Runs `theme sync` and `theme check` on a checked site.
fn theme(folder: &Path, config: &Config, command: &ThemeCommand, cli: &Cli) -> Exit {
    match command {
        ThemeCommand::Sync { url, from_css } => theme_sync::run(
            folder,
            url,
            from_css.as_deref(),
            config.default_language(),
            cli,
        ),
        ThemeCommand::Check { url } => {
            theme_check::run(folder, url.as_ref().unwrap_or(config.address()), cli)
        }
    }
}

/// Runs the commands that decide who may reach the site: `keys`, `staff`.
pub(crate) fn access(folder: &Path, config: &Config, command: &Command, output: Output) -> Exit {
    match command {
        Command::Keys { command } => keys::run(folder, command, output),
        Command::Staff { command } => staff_owner::run(folder, config, command, output),
        _ => report::failure(output, &not_built(command), Exit::Internal),
    }
}

/// Runs the commands that need the site folder but not its checked content:
/// `redirects`, `webhooks`, `crawl` and `analytics`, which read the
/// settings and the database themselves, `backup`, which keeps a broken
/// site as well as a whole one, and `pull`, which checks the site after it
/// brings in the new files.
pub(crate) fn on_folder(folder: &Path, asked: &Command, output: Output) -> Exit {
    match asked {
        Command::Crawl { command } => crawl_report::run(folder, *command, &SystemClock, output),
        Command::Redirects { command } => redirects::run(folder, command, output),
        Command::Webhooks { command } => webhooks::run(folder, command, output),
        Command::Analytics {
            command: AnalyticsCommand::Ai { days },
        } => ai_visitors::run(folder, *days, output),
        Command::Backup { out } => backup::run(folder, out.as_deref(), &SystemClock, output),
        Command::Analytics {
            command: AnalyticsCommand::Uncited { days },
        } => uncited_report::run(folder, *days, &SystemClock, output),
        Command::Pull => pull::run(folder, &SystemClock, output),
        _ => report::failure(output, &not_built(asked), Exit::Internal),
    }
}

/// The message for a command that works on one site folder when none was given.
pub(crate) fn needs_site(command: &Command) -> UserMessage {
    UserMessage::new(
        format!(
            "The command `khalid {}` needs a site folder.",
            command.name()
        ),
        "It works on the files inside one site, so Khalid has to know which one.",
        format!(
            "Run it inside the folder that holds site.toml, or run `khalid --site <folder> {}`.",
            command.name()
        ),
    )
}

/// The message for a command that a later phase builds.
pub(crate) fn not_built(command: &Command) -> UserMessage {
    UserMessage::new(
        format!("The command `khalid {}` is not built yet.", command.name()),
        "Khalid is in its first phase, and this command arrives in a later one.",
        "Run `khalid --help` to see every command, and check the next release.",
    )
}
