//! The one way mail leaves `khalid serve`, and what it serves: the routes
//! readers post to, the retry of form emails and the alerts.

use std::path::Path;
use std::sync::Arc;

use axum::Router;
use khalid_analytics::VisitorLogStore;
use khalid_core::{Config, Secret, UserMessage};
use khalid_db::{NotFoundTable, VisitorLogTable};
use khalid_forms::FormDelivery;
use khalid_outbound::Outbound;
use tokio::runtime::Runtime;

use crate::alert_watch::Every;
use crate::announce::SiteAnnounce;
use crate::exit::Exit;
use crate::form_delivery::{FormRetry, SiteDelivery};
use crate::serve_https::{self, Listen};
use crate::site_content::LiveContent;
use crate::site_watch::SiteWatch;
use crate::{alert_watch, form_pages, newsletter, search_route};

/// The routes readers post to, and the retry of form emails that failed.
pub(super) type Posted = (Router, Option<Arc<FormRetry>>);

/// The routes a reader posts to from a page besides comments, and the
/// alerts ready to start, sharing the one way mail leaves, made from
/// `[mail]` with its `secret` and `outbound`. The routes are the
/// newsletter's and the form block's (G4), whose answers are emailed and
/// told to `telling` (G40), as is each confirmed subscriber; both keep
/// what they receive in the database `site` reads, and the newsletter
/// reads the clock `every` carries. The alert checks and
/// the speed and uptime watch run `every` so often. The form emails that
/// fail are tried again by the retry it returns beside the routes.
pub(super) fn mail_and_alerts(
    (folder, config, listen, every): (&Path, &Config, &Listen, Every),
    (site, runtime, telling): (&search_route::Search, &Runtime, &SiteAnnounce),
    content: &LiveContent,
    (secret, outbound): (Option<Secret<String>>, &Outbound),
) -> Result<(Posted, alert_watch::Ready), (UserMessage, Exit)> {
    let (database, id) = (&site.db, site.site);
    let internal = |message| (message, Exit::Internal);
    let mailer = newsletter::mailer(config, secret, outbound).map_err(internal)?;
    let (newsletters, welcomes) = newsletter::router(
        (folder, config),
        (database, id, every.clock),
        (runtime, telling),
        mailer.clone(),
    )
    .map_err(internal)?;
    let retry = FormRetry::for_site(mailer.clone(), (database, id));
    let delivery = SiteDelivery::new(config, retry.clone(), telling.clone()).map_err(internal)?;
    let delivery: Arc<dyn FormDelivery> = Arc::new(delivery);
    let forms = (&site.security, delivery);
    let forms = form_pages::router(config, (database, id), &site.pages, forms);
    let sources = alert_watch::Sources {
        misses: Arc::new(NotFoundTable::new(database.clone(), id)),
        content: content.clone(),
        https: serve_https::certificate(folder, listen),
        settings: config.alerts().clone(),
        visitors: config.visitor_log().on.then(|| {
            let log: Arc<dyn VisitorLogStore> =
                Arc::new(VisitorLogTable::new(database.clone(), id));
            log
        }),
        watch: every.watch.and_then(|looks| {
            let watch = SiteWatch::new(config, outbound, (database, id))?;
            Some((watch, looks))
        }),
        welcomes,
    };
    let ready = alert_watch::Ready {
        mailer,
        sources,
        every: every.checks,
    };
    Ok(((newsletters.merge(forms), retry), ready))
}
