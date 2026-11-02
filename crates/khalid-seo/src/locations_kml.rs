//! `locations.kml`: the geo sitemap of a local business, one pin per
//! location, as Yoast Local writes it. Map services read it from the
//! sitemap index.

use khalid_core::AbsoluteUrl;

use crate::escape::escape_markup;
use crate::{SeoError, SeoSite, UrlPath};

/// Where the file lives, inside the mount folder.
const KML_PATH: &str = "/locations.kml";

/// The address of `locations.kml` in the mount folder.
///
/// # Errors
///
/// [`SeoError::Url`] when the address cannot be built.
pub fn locations_kml_url(site: &SeoSite) -> Result<AbsoluteUrl, SeoError> {
    site.url_for(&site.default_language, &UrlPath::try_new(KML_PATH)?)
}

/// The KML document of every location of `site` that has a map point, or
/// `None` when none has one or the file is switched off.
pub fn locations_kml(site: &SeoSite) -> Option<String> {
    if !site.local_kml {
        return None;
    }
    let pinned: Vec<_> = (site.locations.iter())
        .filter_map(|business| business.geo.map(|geo| (business, geo)))
        .collect();
    if pinned.is_empty() {
        return None;
    }
    let mut out = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <kml xmlns=\"http://www.opengis.net/kml/2.2\">\n<Document>\n<name>",
    );
    escape_markup(&site.name, &mut out);
    out.push_str("</name>\n");
    for (business, geo) in pinned {
        let address = &business.address;
        let mut lines = vec![address.street.as_str(), address.locality.as_str()];
        lines.extend(address.region.as_deref());
        lines.extend([address.postal_code.as_str(), address.country.as_str()]);
        out.push_str("<Placemark>\n<name>");
        escape_markup(&business.name, &mut out);
        out.push_str("</name>\n<address>");
        escape_markup(&lines.join(", "), &mut out);
        out.push_str("</address>\n");
        if let Some(phone) = &business.telephone {
            out.push_str("<phoneNumber>");
            escape_markup(phone, &mut out);
            out.push_str("</phoneNumber>\n");
        }
        // KML writes the longitude first.
        let point = format!("{},{}", geo.longitude(), geo.latitude());
        out.extend(["<Point><coordinates>", &point, "</coordinates></Point>\n"]);
        out.push_str("</Placemark>\n");
    }
    out.push_str("</Document>\n</kml>\n");
    Some(out)
}
