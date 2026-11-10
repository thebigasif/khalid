//! Where the Admin API copies the JSON-LD of another page, as a starting
//! point for a post's custom JSON-LD field ("copy schema from a URL",
//! KH-SEO-4). `khalid serve` fetches the page through the shared outbound
//! client, so the private-network guard covers it (BL-RUN-8).

use khalid_core::AbsoluteUrl;
use khalid_seo::{CopiedSchema, CopySchemaError};

use crate::content_store::BoxFuture;

/// Fetches a page and copies its JSON-LD.
pub trait SchemaCopier: Send + Sync {
    /// The JSON-LD of the page at `url`.
    ///
    /// # Errors
    ///
    /// The errors [`CopiedSchema::fetch`] returns: the request failed or was
    /// refused, the page answered with an error status, or it holds no
    /// JSON-LD.
    fn copy<'a>(
        &'a self,
        url: &'a AbsoluteUrl,
    ) -> BoxFuture<'a, Result<CopiedSchema, CopySchemaError>>;
}
