//! `khalid serve`: the site answered over HTTP from rendered pages with its
//! sitemaps, feeds and moved addresses, the Content API from the site's own
//! index, and the Admin API over its files, each showing every saved edit,
//! published post or API write again after a moment, until Ctrl+C or a stop
//! signal. A headless site has no pages, and every page address answers 404.

use std::path::Path;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use axum::Router;
use khalid_api::publish_door::PublishDoor;
use khalid_api::unlock::{Transport, UnlockSigner};
use khalid_content::SiteContent;
use khalid_core::{Config, Environment, S3Keys, Secret, Slug, SystemClock, UserMessage};
use khalid_db::{Database, ScheduleTable, SiteId, VersionTable};
use khalid_outbound::{Outbound, OutboundError};
use khalid_routes::Routes;
use khalid_server::{FolderLook, LivePages, Pages, SecurityHeaders, watch};
use serde::Serialize;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;

use crate::alert_watch::Every;
use crate::announce::{Announcing, SiteAnnounce};
use crate::cli::{Cli, Output};
use crate::content_feed::{self, Destination};
use crate::entry_events::Entries;
use crate::exit::Exit;
use crate::media_library::PictureKeys;
use crate::memory::HandBack;
use crate::outside_writes::Reloads;
use crate::post_files::Assets;
use crate::scheduler::WeeklyEmail;
use crate::search_console_finder::SiteFollowUps;
use crate::serve_check::BodyCheck;
use crate::serve_https::{Listen, ServeClock};
use crate::site_content::LiveContent;
use crate::site_hosts::{self, Among, Door};
use crate::site_html::NotFound;
use crate::site_pages::SitePages;
use crate::site_posts::SitePosts;
use crate::tag_events::Tags;
use crate::{
    admin_blog, announce, assets, comment_pages, consent_route, db, db_message, media_library,
    newsletter, page_counts, preview, push_hook, redirect_route, report, scheduler, search_route,
    serve_apis, serve_message, share_images, site_config, staff_toolbar, unlock,
};

#[path = "serve_mail.rs"]
mod mail;
use mail::mail_and_alerts;

/// How often the site folder is looked at for a saved edit where the system
/// has no file events for it (ADR "the server watches files with events").
const WATCH_EVERY: Duration = Duration::from_millis(500);

/// How long requests in flight may take to finish after a stop signal
/// before their connections are closed (RULES.md BL-RUN-10).
pub(crate) const GRACE: Duration = Duration::from_secs(10);

/// The JSON shape of the line `khalid serve --json` prints once it listens.
#[derive(Serialize)]
struct ServeBody<'a> {
    address: &'a str,
}

/// The secrets `khalid serve` reads from the environment at start.
#[derive(Debug)]
pub(crate) struct Keys {
    /// The media bucket's two keys, when the environment has them.
    pub(crate) bucket: Option<S3Keys>,
    /// The password, key or token the `[mail]` transport signs in with.
    pub(crate) mail: Option<Secret<String>>,
    /// The secret git mode's push hook checks, which opens the hook.
    pub(crate) git_hook: Option<Secret<String>>,
    /// The secret the mail hook checks, which opens the hook.
    pub(crate) mail_hook: Option<Secret<String>>,
    /// The Search Console follow-ups, signed in with the owner's Google
    /// secrets when the site opts in.
    pub(crate) follow_ups: SiteFollowUps,
    /// The weekly Search Console email, when the site names `report_to`
    /// and has `[mail]` and the Google sign-in.
    pub(crate) weekly: Option<Arc<WeeklyEmail>>,
    /// The keys photo and GIF search ask Unsplash and GIPHY with.
    pub(crate) pictures: PictureKeys,
    /// The writer of the editor's suggestions, asked with `KHALID_AI_KEY`.
    pub(crate) ai: khalid_api::ai::AiDesk,
    /// The whole environment, which `site.toml` is read with again when it
    /// changes, and when the Admin API reads or saves its settings.
    pub(crate) settings: Environment,
}

impl Keys {
    /// The keys `env` holds for the site in `folder` with `config`, as
    /// `cli` reaches the outside.
    pub(crate) fn read(env: &Environment, (folder, config): (&Path, &Config), cli: &Cli) -> Self {
        Self {
            bucket: env.s3_keys(),
            mail: mail_secret(env, config),
            git_hook: env.git_hook_secret(),
            mail_hook: env.mail_hook_secret(),
            follow_ups: SiteFollowUps::new(folder, config, env, cli),
            weekly: WeeklyEmail::new(folder, config, env, cli),
            ai: crate::serve_apis::ai_writer::desk(config, env, cli),
            settings: env.clone(),
            pictures: PictureKeys {
                unsplash: env.unsplash_key(),
                giphy: env.giphy_key(),
            },
        }
    }
}

/// The password, key or token `env` holds for the `[mail]` transport of
/// the site with `config`, for the commands that send mail without serving.
pub(crate) fn mail_secret(env: &Environment, config: &Config) -> Option<Secret<String>> {
    let mail = config.mail().map(|mail| &mail.transport);
    mail.and_then(|transport| env.mail_secret(transport))
}

/// Serves the `site` folder, whose `config`, `routes` and content were
/// checked, all but the content's bodies, on `listen`. The database opens
/// first and closes last, after every request, the watch, the scheduler and
/// the announcing have stopped. Webhooks and `IndexNow` are sent through
/// `outbound`, the one shared client.
///
/// `start` holds the content, the files beside its folder posts, and a
/// look at the folder taken before it was read. Its bodies are checked
/// beside the start, before the server listens. The first pages are
/// rendered from that content, which is then dropped, so the folder is read
/// once at start and a file saved from the look on is always seen as an
/// edit.
pub(crate) fn run(
    site: &Path,
    (config, routes): (&Config, &Routes),
    (content, assets, look): (SiteContent, Assets, FolderLook),
    (listen, outbound, keys): (&Listen, Result<Outbound, OutboundError>, Keys),
    (output, alert_every): (Output, Every),
) -> Exit {
    let content = Arc::new(content);
    let bodies = BodyCheck::start(site, &content, (config, routes));
    let start = (content, assets, look, bodies);
    let built = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_keep_alive(Duration::from_secs(1))
        .build();
    let (Ok(runtime), Ok(outbound)) = (built, outbound) else {
        return report::failure(output, &serve_message::internal(), Exit::Internal);
    };
    let to = (listen, &outbound, (output, alert_every), keys);
    match serve_site(&runtime, (site, config, routes), start, to, None) {
        Ok(()) if output == Output::Human => report::text("Stopped.\n", Exit::Success),
        Ok(()) => Exit::Success,
        Err((message, exit)) => report::failure(output, &message, exit),
    }
}

/// Serves one site on `runtime` as [`run`] does, or, `among` many, in its
/// place behind the sites folder's listener. Its database opens first and
/// closes last, after every request, the watch, the scheduler and the
/// announcing have stopped.
pub(crate) fn serve_site(
    runtime: &Runtime,
    (site, config, routes): (&Path, &Config, &Routes),
    start: (Arc<SiteContent>, Assets, FolderLook, BodyCheck),
    to: (&Listen, &Outbound, (Output, Every), Keys),
    among: Option<&Among>,
) -> Result<(), (UserMessage, Exit)> {
    let (database, id) = open(site, runtime).map_err(|message| (message, Exit::Failure))?;
    let served = serve_with(
        (site, config, routes),
        start,
        to,
        runtime,
        (&database, id),
        among,
    );
    runtime.block_on(database.close());
    served
}

/// Renders the site with its checked settings and routes, then answers on
/// `listen`, or in its place `among` many, until told to stop.
fn serve_with(
    (site, config, routes): (&Path, &Config, &Routes),
    (checked, assets, look, bodies): (Arc<SiteContent>, Assets, FolderLook, BodyCheck),
    (listen, outbound, (output, every), keys): (&Listen, &Outbound, (Output, Every), Keys),
    runtime: &Runtime,
    (database, id): (&Database, SiteId),
    among: Option<&Among>,
) -> Result<(), (UserMessage, Exit)> {
    let (content, hand_back) = (LiveContent::default(), HandBack::default());
    let (redirects, addresses) = redirect_route::stores((database, id));
    let (to, telling, announcing) = destination(
        (site, config, &checked),
        (database, id, outbound),
        (&content, &hand_back),
        (&redirects, &addresses),
    )?;
    let (feed, feeding) = content_feed::start(runtime.handle(), to);
    let reload = (feed, &keys.settings, &telling);
    let (source, first) = first_pages((site, config, routes), reload, &checked)
        .and_then(|first| bodies.finish().map(|()| first))
        .map_err(|message| (message, Exit::Failure))?;
    let security = security_headers(config)?;
    let search = search_route::Search::of(&source, (database, id), &security);
    let ((posted, forms), alerting) = mail_and_alerts(
        (site, config, listen, every),
        (&search, runtime, &telling),
        &content,
        ((keys.mail, keys.mail_hook), outbound),
    )?;
    let (mailer, panel) = (alerting.mailer.clone(), site_hosts::panel(among));
    let served = runtime.block_on(async {
        let front = Door::open((site, listen, among), (output, config)).await?;
        let live = LivePages::new(first);
        hand_back.start(live.replacements());
        let rest = (source.clone(), live.clone());
        let watching = watch(site, &source, &live, look, WATCH_EVERY);
        let announcing = announcing.retrying(&telling);
        // Each part reads the database or the folder alone, so all open at once.
        let (alerting, alert_watching) = alerting.start((config, database, id, telling.clone()));
        let replies = (security.clone(), telling.clone());
        let commenting = comment_pages::start(config, (database, id), (&source, &live), replies);
        let shown_pages = consent_route::pages(config, live.clone(), security.clone());
        let ((comments, moderation, rereading), (door, posts), (media, files, library), unlocked) =
            tokio::try_join!(
                async { Ok(commenting.await) },
                publish_door((site, config), (database, id, every.clock), telling),
                media_library::open(site, config, (keys.bucket, keys.pictures), outbound),
                unlocking((site, config), (database, id), &source, shown_pages),
            )?;
        posts.follow(source.live_config());
        let reloads = Reloads::new(site, (redirects.clone(), library, rereading));
        let timed = (alerting, keys.weekly, forms);
        let scheduling = scheduler::start((&door, &posts, every.clock), timed, reloads);
        let previews = preview::router(Arc::clone(&door), source.clone(), config, security.clone());
        let pages = (unlocked, live, config.mount());
        let (desk, pages) =
            redirect_route::start((database, id), pages, (redirects, addresses)).await;
        let (counter, pages) = page_counts::start((database, id), pages, (site, config, outbound));
        let editing = (content.clone(), posts.clone());
        let pages = staff_toolbar::start((database, id, config), editing, pages);
        let app = assembled(
            (site, config, &security),
            (files, previews, search_route::router(config, search)),
            (comments, posted, pages),
        );
        let apis = (
            (content, door, posts),
            (moderation, media, desk, keys.follow_ups, keys.ai),
            (database, id, (site, config, &keys.settings)),
            (panel, mailer),
        );
        let app = serve_apis::with_apis(app, apis, push_hook::router(site, keys.git_hook, &rest));
        // The rest renders once the routes are ready, so it does not slow
        // setting them up (ADR "the APIs are built beside the first answer").
        let (failed, failure) = oneshot::channel();
        tokio::spawn(render_the_rest(rest, (checked, assets), failed));
        let why = Arc::new(Mutex::new(None));
        let stopping = stop_or_fail(failure, Arc::clone(&why));
        let stopped = front.answer(app, stopping).await;
        watching.stop().await;
        tokio::join!(scheduling.stop(), alert_watching.stop());
        tokio::join!(announcing.stop(), counter.stop());
        ended(&why, stopped)
    });
    // The pages hold the last feed. Once they are gone the Content API's
    // task finishes its last write, before the database closes.
    drop(source);
    if let Err(error) = runtime.block_on(feeding) {
        tracing::warn!(?error, "the Content API's task stopped early");
    }
    served
}

/// The site's pages, fed by `feed`, which read their settings again with
/// `env` when they change and tell `telling` once they do, and the first of
/// them to serve: only the home pages are rendered before the server
/// listens, and the rest follows behind the first answer (ADR "khalid serve
/// answers before the full render").
fn first_pages(
    (site, config, routes): (&Path, &Config, &Routes),
    (feed, env, telling): (content_feed::ContentFeed, &Environment, &SiteAnnounce),
    checked: &SiteContent,
) -> Result<(SitePages, Pages), UserMessage> {
    let clock = Arc::new(SystemClock);
    let source = SitePages::new(site, config.clone(), routes.clone(), Some(feed), clock)?;
    source.reload_with(env.clone(), Some(telling.clone()));
    let first = source.render_first(checked)?;
    let first = Pages::first(first).map_err(|_| serve_message::internal())?;
    let first = with_hint(first, &source);
    Ok((source, first))
}

/// The pages, with the stylesheet named as a preload on every HTML answer
/// when the site has one (G20).
fn with_hint(pages: Pages, source: &SitePages) -> Pages {
    match source.stylesheet() {
        Some(href) => pages.with_preloads(vec![href]),
        None => pages,
    }
}

/// Where the Content API's feed writes, as `khalid serve` sets it up, and
/// the site's announcements, sent through `outbound`. The feed tells them
/// of each entry saved since the run began with the `checked` content.
///
/// # Errors
///
/// Returns a message when the `IndexNow` key file's address cannot be made
/// from the site's address.
fn destination(
    (site, config, checked): (&Path, &Config, &SiteContent),
    (database, id, outbound): (&Database, SiteId, &Outbound),
    (live, hand_back): (&LiveContent, &HandBack),
    (redirects, addresses): (
        &crate::live_redirects::LiveRedirects,
        &crate::site_addresses::LiveAddresses,
    ),
) -> Result<(Destination, SiteAnnounce, Announcing), (UserMessage, Exit)> {
    let (telling, announcing) = announce::once(config, (database, id), outbound)
        .map_err(|message| (message, Exit::Failure))?;
    let to = Destination {
        folder: site.to_path_buf(),
        config: config.clone(),
        db: database.clone(),
        site: id,
        live: live.clone(),
        hand_back: hand_back.clone(),
        addresses: addresses.clone(),
        redirects: redirects.clone(),
        announce: Some(telling.clone()),
        started: (
            Entries::of(checked).with_drafts(site),
            Tags::of(config, checked),
        ),
        pings: content_feed::UpdatePings::default(),
    };
    to.pings.set(telling.clone());
    Ok((to, telling, announcing))
}

/// The router the server answers outside the APIs: the site's own routes,
/// the drawn share cards, the admin, the comments, the newsletters and the
/// forms.
fn assembled(
    (site, config, security): (&Path, &Config, &SecurityHeaders),
    (files, previews, searches): (Router, Router, Router),
    (comments, newsletters, pages): (Router, Router, Router),
) -> Router {
    files
        .merge(previews)
        .merge(searches)
        .merge(share_images::router(site, config, security.clone()))
        .merge(admin_blog::router(site, config))
        .merge(comments)
        .merge(newsletters)
        .merge(consent_route::router(config, security.clone()))
        .merge(pages)
}

/// The publish door the Admin API, the scheduler and preview links share, on `clock`. It keeps
/// schedules and earlier versions in the database, and tells `telling` of each post that goes live.
async fn publish_door(
    (site, config): (&Path, &Config),
    (database, id, clock): (&Database, SiteId, ServeClock),
    telling: SiteAnnounce,
) -> Result<(Arc<PublishDoor>, SitePosts), (UserMessage, Exit)> {
    let file = db::data_file(site).map_err(|message| (message, Exit::Failure))?;
    let posts = SitePosts::new(site, config.clone(), SystemClock);
    let schedules = Arc::new(ScheduleTable::new(database.clone(), id));
    let versions = Arc::new(VersionTable::new(database.clone(), id));
    let signer = preview::signer(config, (database, id, &file), clock).await;
    let door = PublishDoor::new(Arc::new(posts.clone()), Arc::new(telling))
        .with_schedules(schedules, clock)
        .with_versions(versions)
        .with_planner(Arc::new(posts.clone()))
        .with_previews(signer.map_err(|message| (message, Exit::Failure))?);
    Ok((Arc::new(door), posts))
}

/// `pages`, the site's page router, with the password form and the cookie
/// checks of `source`'s password posts and posts with gated content. The
/// cookies are signed with the site's preview key, each under its own
/// label, and marked `Secure` when the
/// site's address is `https`.
async fn unlocking(
    (site, config): (&Path, &Config),
    (database, id): (&Database, SiteId),
    source: &SitePages,
    pages: axum::Router,
) -> Result<axum::Router, (UserMessage, Exit)> {
    let file = db::data_file(site).map_err(|message| (message, Exit::Failure))?;
    let key = database.preview_key(id, &SystemClock).await;
    let key = key.map_err(|error| (db_message::of(&file, &error), Exit::Failure))?;
    let transport = if config.address().as_str().starts_with("https://") {
        Transport::Https
    } else {
        Transport::Http
    };
    let signer = UnlockSigner::new(&key, transport, SystemClock);
    let reader = newsletter::reader_pass(config, &key);
    Ok(unlock::router(
        config,
        source.gate(),
        (signer, reader),
        pages,
        SystemClock,
    ))
}

/// Renders every page from the `checked` content on a blocking thread
/// (RULES.md BL-RUN-7), serves them in place of the first pages, with any
/// post saved meanwhile kept on top, then adds their Brotli copies. A save
/// during the start may have served a whole set already, which is newer and
/// stays. When the pages cannot be rendered,
/// `failed` gets why, and the server stops with it, as it did when it
/// rendered every page before listening.
async fn render_the_rest(
    (source, live): (SitePages, LivePages),
    (checked, assets): (Arc<SiteContent>, Assets),
    failed: oneshot::Sender<UserMessage>,
) {
    let rendering = tokio::task::spawn_blocking(move || {
        let served = source.render_content(&checked, assets)?;
        drop(checked);
        let NotFound { default, under } = served.not_found;
        Pages::plain(served.pages, default)
            .map(|pages| pages.with_moves(served.moves).with_not_found_under(under))
            .map(|pages| with_hint(pages, &source))
            .map_err(|_| serve_message::internal())
    });
    let pages = match rendering.await {
        Ok(Ok(pages)) => pages,
        Ok(Err(message)) => {
            let _ = failed.send(message);
            return;
        }
        Err(_) => {
            let _ = failed.send(serve_message::internal());
            return;
        }
    };
    if live.serve_whole(pages)
        && let Err(error) = live.compress().await
    {
        tracing::warn!(
            ?error,
            "pages could not be compressed, so they are sent plain"
        );
    }
}

/// Completes on a stop signal, or once `failure` says why the site's pages
/// could not be rendered, which it leaves in `why`.
async fn stop_or_fail(
    failure: oneshot::Receiver<UserMessage>,
    why: Arc<Mutex<Option<UserMessage>>>,
) {
    tokio::select! {
        () = stop() => {}
        Ok(message) = failure => {
            *why.lock().unwrap_or_else(PoisonError::into_inner) = Some(message);
        }
    }
}

/// How serving ended: why the pages could not be rendered, which
/// [`stop_or_fail`] left in `why`, or else whether it `stopped` cleanly.
fn ended<T, E>(
    why: &Mutex<Option<UserMessage>>,
    stopped: Result<T, E>,
) -> Result<(), (UserMessage, Exit)> {
    match why.lock().unwrap_or_else(PoisonError::into_inner).take() {
        Some(message) => Err((message, Exit::Failure)),
        None => stopped
            .map(|_| ())
            .map_err(|_| (serve_message::stopped(), Exit::Failure)),
    }
}

/// Opens `data/site.db` in the site folder, making it when it is missing,
/// brings it up to date, and finds the site's row.
pub(crate) fn open(site: &Path, runtime: &Runtime) -> Result<(Database, SiteId), UserMessage> {
    let path = db::data_file(site)?;
    let main = Slug::try_new(db::MAIN).map_err(|_| db_message::internal())?;
    runtime.block_on(async {
        let database = Database::open(&path)
            .await
            .map_err(|error| db_message::of(&path, &error))?;
        let opened = async {
            database.migrate().await?;
            database.site(&main).await
        }
        .await;
        match opened {
            Ok(id) => Ok((database, id)),
            Err(error) => {
                database.close().await;
                Err(db_message::of(&path, &error))
            }
        }
    })
}

/// The security headers of every page, letting in the frames of the embed
/// providers and `[media] embed_domains`, and the reader script by its hash.
fn security_headers(config: &Config) -> Result<SecurityHeaders, (UserMessage, Exit)> {
    let origins = site_config::frame_origins(config);
    SecurityHeaders::new(&origins.iter().map(String::as_str).collect::<Vec<_>>())
        .and_then(|headers| headers.with_script(&assets::reader().integrity))
        .map_err(|_| (serve_message::internal(), Exit::Internal))
}

/// Says where the site answers. In JSON, this one line is the whole output.
pub(crate) fn announce(output: Output, config: &Config, address: &str) {
    match output {
        Output::Human => report::text(
            &format!(
                "Serving {} at {address}. Press Ctrl+C to stop.\n",
                config.name()
            ),
            Exit::Success,
        ),
        Output::Json => report::json(&ServeBody { address }, Exit::Success),
    };
}

/// Completes on Ctrl+C or, on Unix, the stop signal a service manager sends.
pub(crate) async fn stop() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        if let Ok(mut terminate) = signal(SignalKind::terminate()) {
            // A Ctrl+C listener that fails is left out, rather than taken
            // for a stop.
            tokio::select! {
                Ok(()) = tokio::signal::ctrl_c() => {}
                _ = terminate.recv() => {}
            }
            return;
        }
    }
    if tokio::signal::ctrl_c().await.is_err() {
        tracing::warn!("Ctrl+C cannot be heard here, so stop the server with its process id");
        std::future::pending::<()>().await;
    }
}
