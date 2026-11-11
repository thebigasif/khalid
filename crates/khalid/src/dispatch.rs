//! Where `main` sends each command: one arm per command and no catch-all,
//! so a new command does not compile until it has a place to run. Each
//! command gets what it needs and no more: nothing, the site's folder, or
//! the checked site. Kept apart from `main` to keep each file short
//! (BL-SIZE-1).

use std::path::Path;

use khalid_content::SiteContent;
use khalid_core::{Config, Environment, SystemClock, UserMessage};
use khalid_routes::Routes;

use crate::cli::{AnalyticsCommand, Cli, Command, ThemeCommand};
use crate::exit::Exit;
use crate::lint::Mode;
use crate::new::NewPost;
use crate::{
    Bodies, agents, ai_visitors, backup, bing, build, check, completions, crawl_report, db, doctor,
    export, import, init, keys, lint, look, mcp, new, newsletter_cli, plan, publish, pull,
    redirects, report, restore, search_console, serve, serve_site, site_audit, sites, staff_owner,
    theme_check, theme_sync, uncited_report, update, watch_report, webhooks,
};

/// What one check of a site gave a command: its settings, its routes and
/// its content.
type Site<'a> = (&'a Config, &'a Routes, &'a SiteContent);

/// Runs `command`, after reading the folder or checking the site it needs.
#[expect(
    clippy::too_many_lines,
    reason = "one arm per command and no catch-all, so a command with nowhere to run does not compile"
)]
pub(crate) fn run(cli: &Cli, env: &Environment, command: &Command) -> Exit {
    let output = cli.output;
    match command {
        Command::Init(args) => {
            let folder = args
                .folder
                .as_deref()
                .or(cli.site.as_deref())
                .unwrap_or(Path::new("."));
            init::run(folder, args, output)
        }
        Command::Agents => agents::run(output),
        Command::Completions(args) => completions::run(args, output),
        #[cfg(feature = "man")]
        Command::Man(args) => crate::man::run(args, output),
        Command::Restore { archive, folder } => restore::run(archive, folder, output),
        Command::Sites { command } => sites::run(
            cli.site.as_deref().unwrap_or(Path::new(".")),
            command,
            output,
        ),
        Command::Update { target } => update::run(*target, cli),
        Command::Serve(args) => serve_site(cli, env, command, args),
        Command::Doctor { network, test_mail } => in_folder(cli, command, |folder| {
            doctor::run(folder, (*network, *test_mail), cli)
        }),
        Command::Redirects { command: asked } => {
            in_folder(cli, command, |folder| redirects::run(folder, asked, output))
        }
        Command::Webhooks { command: asked } => {
            in_folder(cli, command, |folder| webhooks::run(folder, asked, output))
        }
        Command::Crawl { command: asked } => in_folder(cli, command, |folder| {
            crawl_report::run(folder, *asked, &SystemClock, output)
        }),
        Command::Analytics {
            command: AnalyticsCommand::Ai { days },
        } => in_folder(cli, command, |folder| {
            ai_visitors::run(folder, *days, output)
        }),
        Command::Analytics {
            command: AnalyticsCommand::Uncited { days },
        } => in_folder(cli, command, |folder| {
            uncited_report::run(folder, *days, &SystemClock, output)
        }),
        Command::Backup { out } => in_folder(cli, command, |folder| {
            backup::run(folder, out.as_deref(), &SystemClock, output)
        }),
        Command::Pull => in_folder(cli, command, |folder| {
            pull::run(folder, &SystemClock, output)
        }),
        Command::Db { command: asked } => on_site(cli, env, command, |folder, (config, ..)| {
            db::run((folder, config), asked.clone(), output)
        }),
        Command::Watch => on_site(cli, env, command, |folder, _| {
            watch_report::run(folder, &SystemClock, output)
        }),
        Command::Lint { path, fix } => on_site(cli, env, command, |folder, _| {
            let mode = if *fix { Mode::Fix } else { Mode::Check };
            lint::run(folder, path.as_deref(), mode, output)
        }),
        Command::Mcp => on_site(cli, env, command, |folder, (config, ..)| {
            mcp::run(folder, config, (env, cli))
        }),
        Command::Import(args) => on_site(cli, env, command, |folder, (config, routes, _)| {
            import::run(folder, (config, routes), args, cli)
        }),
        Command::Build { out, no_indexnow } => on_site(cli, env, command, |folder, site| {
            build::run(folder, site, (out.as_deref(), *no_indexnow), cli)
        }),
        Command::Export => on_site(cli, env, command, |folder, site| {
            export::run(folder, site, &SystemClock, output)
        }),
        Command::Audit { server, network } => on_site(cli, env, command, |folder, site| {
            site_audit::run(
                folder,
                site,
                (server.as_ref(), *network),
                &SystemClock,
                (cli, serve::mail_secret(env, site.0)),
            )
        }),
        Command::Keys { command: asked } => on_site(cli, env, command, |folder, _| {
            keys::run(folder, asked, output)
        }),
        Command::Staff { command: asked } => on_site(cli, env, command, |folder, (config, ..)| {
            staff_owner::run(folder, config, asked, output)
        }),
        Command::Bing { command: asked } => on_site(cli, env, command, |_, (config, ..)| {
            bing::run(config, env, *asked, cli)
        }),
        Command::SearchConsole { command: asked } => {
            on_site(cli, env, command, |folder, (config, _, content)| {
                search_console::run((folder, config, content), env, asked.clone(), cli)
            })
        }
        Command::Theme { command: asked } => on_site(cli, env, command, |folder, (config, ..)| {
            theme(folder, config, asked, cli)
        }),
        Command::Publish { slug, language } => {
            on_site(cli, env, command, |folder, (config, ..)| {
                publish::run(cli, (folder, config), (slug, language.as_ref()))
            })
        }
        Command::Plan { per_week } => on_site(cli, env, command, |folder, (config, ..)| {
            plan::run(cli, (folder, config), *per_week)
        }),
        Command::Newsletter { command: asked } => {
            on_site(cli, env, command, |folder, (config, ..)| {
                newsletter_cli::run(cli, (folder, config), asked)
            })
        }
        Command::New(args) => on_site(cli, env, command, |folder, (config, ..)| {
            new::run(folder, config, NewPost::from(args), &SystemClock, output)
        }),
    }
}

/// Runs `run` on the folder `--site` names, without checking the site
/// first: these commands read the settings and the database themselves,
/// `backup` keeps a broken site as well as a whole one, and `pull` checks
/// the site after it brings in the new files.
fn in_folder(cli: &Cli, command: &Command, run: impl FnOnce(&Path) -> Exit) -> Exit {
    match cli.site.as_deref() {
        Some(folder) => run(folder),
        None => report::failure(cli.output, &needs_site(command), Exit::Usage),
    }
}

/// Checks the settings, routes and content of the folder `--site` names,
/// then runs `run` on the checked site. Says what is wrong instead when
/// there is no folder or the check fails.
fn on_site(
    cli: &Cli,
    env: &Environment,
    command: &Command,
    run: impl FnOnce(&Path, Site<'_>) -> Exit,
) -> Exit {
    let Some(folder) = cli.site.as_deref() else {
        return report::failure(cli.output, &needs_site(command), Exit::Usage);
    };
    match check(folder, env, Bodies::Now) {
        Ok((config, routes, content, _)) => run(folder, (&config, &routes, &content)),
        Err(message) => report::failure(cli.output, &message, Exit::Failure),
    }
}

/// Runs `theme sync` and `theme check` on a checked site.
fn theme(folder: &Path, config: &Config, command: &ThemeCommand, cli: &Cli) -> Exit {
    match command {
        ThemeCommand::Sync { url, from_css } => {
            theme_sync::run(folder, url, from_css.as_deref(), config, cli)
        }
        ThemeCommand::Check { url } => {
            let url = url.as_ref().unwrap_or(config.address());
            theme_check::run(folder, url, look::pack_text(config), cli)
        }
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
