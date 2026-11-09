//! `BreadcrumbList`: the trail from the home page to the page.

use khalid_core::AbsoluteUrl;

use crate::schema::node_id;
use crate::{SchemaNode, SchemaValue, SeoError, SeoPage, SeoSite, UrlPath};

fn list_item(position: i64, name: &str, item: AbsoluteUrl) -> SchemaValue {
    let node = SchemaNode::new("ListItem")
        .with("position", SchemaValue::Integer(position))
        .with("name", SchemaValue::text(name))
        .with("item", SchemaValue::Url(item));
    SchemaValue::Node(Box::new(node))
}

/// The page's `BreadcrumbList` at `<page>#breadcrumb`: the home page of its
/// language, then each of its crumbs, then the page itself unless it is the
/// home page.
///
/// # Errors
///
/// [`SeoError::Url`] when an address cannot be built.
pub fn breadcrumb_list(site: &SeoSite, page: &SeoPage) -> Result<SchemaNode, SeoError> {
    let page_url = site.url_for(&page.language, &page.path)?;
    let home = site.url_for(&page.language, &UrlPath::root())?;
    let home_name = site.breadcrumb_home.as_deref().unwrap_or(&site.name);
    let mut trail = vec![(home_name, home.clone())];
    for crumb in &page.breadcrumbs {
        trail.push((&crumb.name, site.url_for(&page.language, &crumb.path)?));
    }
    if page_url != home {
        let name = page.breadcrumb_title.as_ref().unwrap_or(&page.title);
        trail.push((name, page_url.clone()));
    }
    let items = (1..)
        .zip(trail)
        .map(|(position, (name, item))| list_item(position, name, item))
        .collect();
    Ok(SchemaNode::new("BreadcrumbList")
        .with_id(node_id(&page_url, "breadcrumb")?)
        .with("itemListElement", SchemaValue::List(items)))
}
