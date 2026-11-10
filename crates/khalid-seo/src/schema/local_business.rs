//! `LocalBusiness`: a site's shops, offices or clinics, one node each.

use chrono::{NaiveTime, Weekday};
use khalid_core::{AbsoluteUrl, GeoPoint};

use crate::schema::{node_id, publisher_id, site_home};
use crate::{SchemaNode, SchemaValue, SeoError, SeoSite};

/// A street address.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct PostalAddress {
    /// The street and number, such as `12 Mill Lane`.
    pub street: String,
    /// The town or city.
    pub locality: String,
    /// The postcode.
    pub postal_code: String,
    /// The two-letter country code, such as `GB`.
    pub country: String,
    /// The county, state or region, if the address uses one.
    pub region: Option<String>,
}

impl PostalAddress {
    /// An address with no region.
    pub fn new(street: &str, locality: &str, postal_code: &str, country: &str) -> Self {
        Self {
            street: street.to_owned(),
            locality: locality.to_owned(),
            postal_code: postal_code.to_owned(),
            country: country.to_owned(),
            region: None,
        }
    }
}

/// When a location is open on some days of the week.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct OpeningHours {
    /// The days these hours apply to.
    pub days: Vec<Weekday>,
    /// When it opens.
    pub opens: NaiveTime,
    /// When it closes.
    pub closes: NaiveTime,
}

impl OpeningHours {
    /// Open from `opens` to `closes` on `days`.
    pub fn new(days: Vec<Weekday>, opens: NaiveTime, closes: NaiveTime) -> Self {
        Self {
            days,
            opens,
            closes,
        }
    }
}

/// One location of a local business.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct LocalBusiness {
    /// The schema.org type, `LocalBusiness` or a more exact one such as
    /// `Dentist` or `Bakery`. The vocabulary check confirms it exists.
    pub business_type: String,
    /// The location's name.
    pub name: String,
    /// Where it is.
    pub address: PostalAddress,
    /// Its phone number.
    pub telephone: Option<String>,
    /// When it is open.
    pub opening_hours: Vec<OpeningHours>,
    /// How expensive it is, such as `££`.
    pub price_range: Option<String>,
    /// A map of where it is.
    pub map: Option<AbsoluteUrl>,
    /// Its point on the map, for `geo` and `locations.kml`.
    pub geo: Option<GeoPoint>,
    /// Its email address.
    pub email: Option<String>,
    /// Its own website. When not set, a site's location points at the
    /// site's home and a business card at its page.
    pub url: Option<AbsoluteUrl>,
}

impl LocalBusiness {
    /// A location with a type, a name and an address.
    pub fn new(business_type: &str, name: &str, address: PostalAddress) -> Self {
        Self {
            business_type: business_type.to_owned(),
            name: name.to_owned(),
            address,
            telephone: None,
            opening_hours: Vec::new(),
            price_range: None,
            map: None,
            geo: None,
            email: None,
            url: None,
        }
    }
}

fn day_url(day: Weekday) -> String {
    let name = match day {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };
    format!("https://schema.org/{name}")
}

pub(crate) fn address_node(address: &PostalAddress) -> SchemaValue {
    let mut node = SchemaNode::new("PostalAddress")
        .with("streetAddress", SchemaValue::text(&address.street))
        .with("addressLocality", SchemaValue::text(&address.locality))
        .with("postalCode", SchemaValue::text(&address.postal_code))
        .with("addressCountry", SchemaValue::text(&address.country));
    if let Some(region) = &address.region {
        node.set("addressRegion", SchemaValue::text(region));
    }
    SchemaValue::Node(Box::new(node))
}

fn hours_node(hours: &OpeningHours) -> SchemaValue {
    let days = hours
        .days
        .iter()
        .map(|day| SchemaValue::Text(day_url(*day)))
        .collect();
    let node = SchemaNode::new("OpeningHoursSpecification")
        .with("dayOfWeek", SchemaValue::List(days))
        .with(
            "opens",
            SchemaValue::Text(hours.opens.format("%H:%M").to_string()),
        )
        .with(
            "closes",
            SchemaValue::Text(hours.closes.format("%H:%M").to_string()),
        );
    SchemaValue::Node(Box::new(node))
}

/// One node per location, at `<home>#location-<n>`, each part of the publisher.
pub(crate) fn business_nodes(site: &SeoSite) -> Result<Vec<SchemaNode>, SeoError> {
    (1..)
        .zip(&site.locations)
        .map(|(number, business)| location_node(site, business, number))
        .collect()
}

/// The node of the site's location numbered `number`, at
/// `<home>#location-<n>`: the same id on every page that shows it, so
/// engines see one place.
pub(crate) fn location_node(
    site: &SeoSite,
    business: &LocalBusiness,
    number: usize,
) -> Result<SchemaNode, SeoError> {
    let home = site_home(site)?;
    let url = business.url.clone().unwrap_or_else(|| home.clone());
    let node = SchemaNode::new(&business.business_type)
        .with_id(node_id(&home, &format!("location-{number}"))?)
        .with("name", SchemaValue::text(&business.name))
        .with("url", SchemaValue::Url(url))
        .with("address", address_node(&business.address))
        .with(
            "parentOrganization",
            SchemaValue::Reference(publisher_id(site)?),
        );
    Ok(with_details(node, business))
}

/// The node of a business card, block `number` on the page at `page`, at
/// `<page>#business-<n>`. A card may name any business, so it is not made
/// part of the site's publisher.
pub(crate) fn business_card_node(
    business: &LocalBusiness,
    page: &AbsoluteUrl,
    number: usize,
) -> Result<SchemaNode, SeoError> {
    let url = business.url.clone().unwrap_or_else(|| page.clone());
    let node = SchemaNode::new(&business.business_type)
        .with_id(node_id(page, &format!("business-{number}"))?)
        .with("name", SchemaValue::text(&business.name))
        .with("url", SchemaValue::Url(url))
        .with("address", address_node(&business.address));
    Ok(with_details(node, business))
}

/// `node` with the business's phone, email, hours, prices, map and point,
/// each where it is set.
fn with_details(mut node: SchemaNode, business: &LocalBusiness) -> SchemaNode {
    if let Some(telephone) = &business.telephone {
        node.set("telephone", SchemaValue::text(telephone));
    }
    if let Some(email) = &business.email {
        node.set("email", SchemaValue::text(email));
    }
    if !business.opening_hours.is_empty() {
        let hours = business.opening_hours.iter().map(hours_node).collect();
        node.set("openingHoursSpecification", SchemaValue::List(hours));
    }
    if let Some(price) = &business.price_range {
        node.set("priceRange", SchemaValue::text(price));
    }
    if let Some(map) = &business.map {
        node.set("hasMap", SchemaValue::Url(map.clone()));
    }
    if let Some(geo) = business.geo {
        let point = SchemaNode::new("GeoCoordinates")
            .with("latitude", SchemaValue::Number(geo.latitude()))
            .with("longitude", SchemaValue::Number(geo.longitude()));
        node.set("geo", SchemaValue::Node(Box::new(point)));
    }
    node
}
