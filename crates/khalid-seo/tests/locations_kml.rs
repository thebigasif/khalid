//! The KML geo sitemap of a local business's locations, as Yoast Local
//! writes it (Yoast comparison, 2 November 2026), and the map point in
//! each location's schema node.

#![cfg(test)]

mod common;

use common::{demo_site, internal_links_post, internal_links_post_de, locale};
use khalid_core::GeoPoint;
use khalid_seo::{
    LocalBusiness, PageKind, PostalAddress, SeoPage, UrlPath, locations_kml, locations_kml_url,
    page_graph,
};

fn located(name: &str, geo: Option<GeoPoint>) -> LocalBusiness {
    let mut business = LocalBusiness::new(
        "Dentist",
        name,
        PostalAddress::new("12 Mill Lane", "Leeds", "LS1 4AB", "GB"),
    );
    business.telephone = Some(String::from("+44 113 496 0000"));
    business.geo = geo;
    business
}

#[test]
fn each_location_with_a_map_point_is_a_placemark() {
    let mut site = demo_site();
    let leeds = GeoPoint::new(53.7997, -1.5492).unwrap();
    site.locations = vec![
        located("Mill Lane & Sons", Some(leeds)),
        located("No pin", None),
    ];
    let kml = locations_kml(&site).unwrap();
    assert!(kml.starts_with(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<kml xmlns=\"http://www.opengis.net/kml/2.2\">"
    ));
    assert!(kml.contains("<name>Mill Lane &amp; Sons</name>"), "{kml}");
    assert!(
        kml.contains("<address>12 Mill Lane, Leeds, LS1 4AB, GB</address>"),
        "{kml}"
    );
    assert!(kml.contains("<phoneNumber>+44 113 496 0000</phoneNumber>"));
    // KML writes the longitude first.
    assert!(
        kml.contains("<Point><coordinates>-1.5492,53.7997</coordinates></Point>"),
        "{kml}"
    );
    assert!(!kml.contains("No pin"), "{kml}");
    assert_eq!(kml.matches("<Placemark>").count(), 1);
}

#[test]
fn there_is_no_file_without_a_map_point_or_when_it_is_off() {
    let mut site = demo_site();
    site.locations = vec![located("No pin", None)];
    assert_eq!(locations_kml(&site), None);
    site.locations = vec![located("Pin", GeoPoint::new(1.0, 2.0).ok())];
    site.local_kml = false;
    assert_eq!(locations_kml(&site), None);
}

#[test]
fn the_file_sits_in_the_mount_folder() {
    let url = locations_kml_url(&demo_site()).unwrap();
    assert!(url.as_str().ends_with("/locations.kml"), "{url}");
}

#[test]
fn a_location_node_names_its_map_point_and_a_post_does_not() {
    let mut site = demo_site();
    site.locations = vec![located("Pin", GeoPoint::new(53.7997, -1.5492).ok())];
    let home = SeoPage::new(PageKind::Home, locale("en"), UrlPath::root(), "");
    let json: serde_json::Value =
        serde_json::from_str(&page_graph(&site, &home).unwrap().to_json()).unwrap();
    let nodes = json["@graph"].as_array().unwrap();
    let business = nodes
        .iter()
        .find(|node| node["@type"] == "Dentist")
        .unwrap();
    assert_eq!(
        business["geo"],
        serde_json::json!({"@type": "GeoCoordinates", "latitude": 53.7997, "longitude": -1.5492})
    );
    for post in [internal_links_post(), internal_links_post_de()] {
        let json = page_graph(&site, &post).unwrap().to_json();
        assert!(!json.contains("GeoCoordinates"), "{json}");
    }
}
