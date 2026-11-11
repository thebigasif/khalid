//! The page fetcher behind the Admin API's "copy schema from a URL" route
//! (KH-SEO-4): the shared outbound client, with its private-network guard.

use khalid_api::content_store::BoxFuture;
use khalid_api::schema_copy::SchemaCopier;
use khalid_core::AbsoluteUrl;
use khalid_outbound::{Limits, Outbound};
use khalid_seo::{CopiedSchema, CopySchemaError};

use crate::cli::Cli;
use crate::outbound;

/// The largest page read. A page with a lot of script inline is rarely
/// over 2 MiB.
const MOST_BYTES: usize = 4 * 1024 * 1024;

/// The copier for `khalid serve`, or `None` when the client cannot be set
/// up, so the route answers that this server fetches no pages.
pub(crate) fn copier(cli: &Cli) -> Option<PageCopier> {
    let limits = Limits::default().with_max_body_bytes(MOST_BYTES);
    outbound::client_within(cli, limits).ok().map(PageCopier)
}

/// Fetches pages through one outbound client.
#[derive(Debug)]
pub(crate) struct PageCopier(Outbound);

impl SchemaCopier for PageCopier {
    fn copy<'a>(
        &'a self,
        url: &'a AbsoluteUrl,
    ) -> BoxFuture<'a, Result<CopiedSchema, CopySchemaError>> {
        Box::pin(CopiedSchema::fetch(&self.0, url))
    }
}
