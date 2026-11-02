//! The locations page: every one of the site's `[[seo.locations]]` as a
//! card, in the order of `site.toml`, as Yoast Local's store locator
//! lists them. With scripts off it is a plain list; with them on, a
//! button sorts it nearest first from where the reader is.

use khalid_core::{Locale, LocationSettings};

use crate::escape::attribute;
use crate::{LocationWords, RenderError, builtin_strings, location_card};

/// The locations page of one language: its title, its description and
/// the HTML of its body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct LocationsPage {
    /// The page's title and heading, such as "Our locations".
    pub title: String,
    /// One sentence on what the page holds, also its meta description.
    pub description: String,
    /// The intro, the sort button and the list of cards.
    pub body: String,
}

impl LocationsPage {
    /// The page of `locations` in `locale`'s language, or English.
    ///
    /// # Errors
    ///
    /// [`RenderError::Strings`] when a built-in strings file cannot be read.
    pub fn new(locations: &[LocationSettings], locale: &Locale) -> Result<Self, RenderError> {
        let table = builtin_strings(locale)?;
        let word = |key: &str| table.get(key).cloned().unwrap_or_default();
        let card_words = LocationWords::for_locale(locale)?;
        let description = word("locations_intro");
        let mut body = ["<p>", &escape(&description), "</p>\n"].concat();
        let pinned = locations.iter().filter(|location| location.geo.is_some());
        if pinned.count() > 1 {
            body.extend([
                "<p><button type=\"button\" class=\"locations-nearest\" hidden data-sorted=\"",
                &attribute(&word("locations_sorted")),
                "\" data-refused=\"",
                &attribute(&word("locations_refused")),
                "\">",
                &escape(&word("locations_nearest")),
                "</button></p>\n",
            ]);
        }
        body.push_str("<ol class=\"locations\">\n");
        for location in locations {
            body.extend(["<li>\n", &location_card(location, &card_words), "</li>\n"]);
        }
        body.push_str("</ol>\n");
        Ok(Self {
            title: word("locations_title"),
            description,
            body,
        })
    }
}

/// `text` as element content.
fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, LocationSettings};

    use super::LocationsPage;

    fn locations(pins: usize) -> Vec<LocationSettings> {
        #[derive(serde::Deserialize)]
        struct File {
            locations: Vec<LocationSettings>,
        }
        let mut text = String::new();
        for number in 0..3 {
            text.extend([
                "[[locations]]\nname = \"Shop ",
                &number.to_string(),
                "\"\nstreet = \"1 High St\"\nlocality = \"York\"\n\
                 postal_code = \"YO1 1AA\"\ncountry = \"GB\"\n",
            ]);
            if number < pins {
                text.push_str("geo = { latitude = 53.96, longitude = -1.08 }\n");
            }
        }
        toml::from_str::<File>(&text).unwrap().locations
    }

    #[test]
    fn every_location_is_listed_in_order_with_a_sort_button_for_two_pins() {
        let english = Locale::try_new("en").unwrap();
        let page = LocationsPage::new(&locations(2), &english).unwrap();
        assert_eq!(page.title, "Our locations");
        assert_eq!(page.body.matches("<li>").count(), 3);
        let first = page.body.find("Shop 0").unwrap();
        assert!(first < page.body.find("Shop 2").unwrap());
        assert!(
            page.body.contains(">Show the nearest first</button>"),
            "{}",
            page.body
        );
        let one = LocationsPage::new(&locations(1), &english).unwrap();
        assert!(!one.body.contains("<button"), "{}", one.body);
        let german = LocationsPage::new(&locations(0), &Locale::try_new("de").unwrap()).unwrap();
        assert_eq!(german.title, "Unsere Standorte");
    }
}
