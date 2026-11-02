//! The location card a `::location` block becomes (Yoast Local comparison,
//! 2 November 2026).

use khalid_content::{LOCATION_MARK, render};
use khalid_core::{Locale, LocationSettings, SlugMode};
use khalid_render::{LocationWords, location_card, place_locations};

fn locale(tag: &str) -> Locale {
    Locale::try_new(tag).unwrap()
}

/// The locations a site lists, as `site.toml` gives them.
fn locations() -> Vec<LocationSettings> {
    #[derive(serde::Deserialize)]
    struct File {
        locations: Vec<LocationSettings>,
    }
    let text = "[[locations]]\nname = \"Joe's & Sons\"\nstreet = \"12 Mill Lane\"\n\
                locality = \"Leeds\"\npostal_code = \"LS1 4AB\"\ncountry = \"GB\"\n\
                telephone = \"+44 113 496 0000\"\ngeo = { latitude = 53.7997, longitude = -1.5492 }\n\n\
                [[locations.hours]]\ndays = [\"mon\", \"tue\"]\nopens = \"09:00\"\ncloses = \"12:30\"\n\n\
                [[locations.hours]]\ndays = [\"mon\"]\nopens = \"14:00\"\ncloses = \"17:30\"\n\n\
                [[locations]]\nname = \"Quay Street\"\nstreet = \"3 Quay Street\"\n\
                locality = \"Bristol\"\npostal_code = \"BS1 4DJ\"\ncountry = \"GB\"\n\
                map = \"https://maps.example.com/quay\"\n";
    toml::from_str::<File>(text).unwrap().locations
}

#[test]
fn a_card_shows_the_address_phone_hours_and_a_map_link() {
    let words = LocationWords::for_locale(&locale("en")).unwrap();
    let card = location_card(&locations()[0], &words);
    insta::assert_snapshot!(card);
    assert!(card.contains("<a href=\"tel:+441134960000\">+44 113 496 0000</a>"));
    assert!(card.contains("<th scope=\"row\">Monday</th><td>09:00 to 12:30, 14:00 to 17:30</td>"));
    assert!(card.contains("<th scope=\"row\">Sunday</th><td>Closed</td>"));
    assert!(card.contains("https://www.openstreetmap.org/?mlat=53.7997&amp;mlon=-1.5492"));
    let quay = location_card(&locations()[1], &words);
    assert!(
        quay.contains("<a href=\"https://maps.example.com/quay\">"),
        "{quay}"
    );
    assert!(!quay.contains("<table"), "{quay}");
}

#[test]
fn the_card_speaks_the_page_language() {
    let words = LocationWords::for_locale(&locale("de")).unwrap();
    let card = location_card(&locations()[0], &words);
    assert!(
        card.contains("<th scope=\"row\">Montag</th><td>09:00 bis 12:30, 14:00 bis 17:30</td>"),
        "{card}"
    );
    assert!(card.contains("data-open=\"Jetzt geöffnet\""), "{card}");
}

#[test]
fn each_mark_becomes_its_card_and_a_name_the_site_lacks_is_reported() {
    let words = LocationWords::for_locale(&locale("en")).unwrap();
    let body = "::location{name=\"Joe's & Sons\"}\n\n::location{name=\"Nowhere\"}\n";
    let html = render(body, &locale("en"), SlugMode::Ascii).unwrap().html;
    assert!(html.contains(LOCATION_MARK));
    let (placed, missing) = place_locations(&html, &locations(), &words).unwrap();
    assert!(
        placed.contains("<strong>Joe's &amp; Sons</strong>"),
        "{placed}"
    );
    assert!(!placed.contains(LOCATION_MARK), "{placed}");
    assert_eq!(missing, ["Nowhere"]);
    assert_eq!(
        place_locations("<p>No mark.</p>", &locations(), &words),
        None
    );
}
