//! [`slip_for`]: the schema.org name a hand-written type or property name is
//! most likely a slip for, such as `Event` for `Eevnt`.
//!
//! The table is compact on purpose: the binary is at its size budget, and
//! the whole vocabulary is about 2,500 names. It holds every name Khalid
//! writes itself, every name in Google's rules (`google_rules.rs`), the
//! names custom JSON-LD uses most, and the real names one letter away from
//! those, such as `Resort` beside `Report` and `weight` beside `height`.
//! A name the table does not hold, and that is not close to one it does,
//! passes, so a rare but real name is never refused.

use crate::redirect_moves::edit_distance;

/// schema.org type names, sorted, one space apart.
const TYPES: &str = "\
AboutPage AggregateOffer AggregateRating Answer Article AudioObject Blog BlogPosting Book Brand \
BreadcrumbList BusinessEvent Claim ClaimReview CollectionPage Comment ContactPage ContactPoint \
Corporation Country Course CourseInstance CreativeWork DataCatalog DataDownload Dataset \
DefinedTerm DefinedTermSet Dentist EducationEvent EducationalOrganization EntryPoint Event \
EventSeries FAQPage Festival GeoCoordinates HowTo HowToSection HowToStep HowToSupply HowToTool \
ImageObject ItemList JobPosting ListItem LocalBusiness MediaObject MobileApplication \
MonetaryAmount Movie MusicEvent NGO NewsArticle NewsMediaOrganization NutritionInformation \
Offer OpeningHoursSpecification Organization Person Place PodcastEpisode PodcastSeries \
PostalAddress PriceSpecification Product ProfessionalService ProfilePage PropertyValue QAPage \
Question Quotation Rating Recipe Report Resort Restaurant Review SaleEvent SearchAction \
SearchResultsPage Service SoftwareApplication SoftwareSourceCode SpeakableSpecification \
SportsEvent Store TechArticle Thing VideoGame VideoObject VirtualLocation WebApplication \
WebPage WebSite";

/// schema.org property names, sorted, one space apart.
const PROPERTIES: &str = "\
about acceptedAnswer actor actors additionalType address addressCountry addressLocality \
addressRegion affiliation aggregateRating alternateName alternativeHeadline alumniOf \
answerCount applicationCategory articleBody articleSection associatedMedia audience author \
availability award awards bestRating birthDate birthPlace brand breadcrumb calories caption \
category claimReviewed closes colleague colleagues comment commentCount contactPoint contactPoints contentUrl \
cookTime copyrightHolder copyrightYear creator cssSelector currency dateCreated dateModified \
datePublished dayOfWeek description director directors distribution doorTime duration email \
embedUrl employee employees encodingFormat endDate episodeNumber estimatedCost event \
eventAttendanceMode eventStatus events familyName founder founders foundingDate gender geo \
givenName gtin gtin12 gtin13 gtin14 gtin8 hasCredential hasDefinedTerm hasMap hasPart headline \
height identifier image inDefinedTermSet inLanguage isAccessibleForFree isPartOf isbn issn item \
itemListElement itemListOrder itemReviewed jobTitle keywords knowsAbout latitude license \
location logo longitude mainEntity mainEntityOfPage maximumAttendeeCapacity member memberOf \
members mentions mpn name numberOfItems nutrition offers openingHours \
openingHoursSpecification opens operatingSystem organizer parentOrganization partOfSeries \
performer performers position postalCode potentialAction prepTime price priceCurrency \
priceRange priceSpecification primaryImageOfPage provider publisher ratingCount ratingValue \
recipeCategory recipeCuisine recipeIngredient recipeInstructions recipeYield review reviewBody \
reviewCount reviewRating reviewedBy reviews sameAs seller sitemap sku speakable startDate step \
steps streetAddress subjectOf suggestedAnswer supply target telephone text thumbnailUrl tool \
totalTime uploadDate upvoteCount url urlTemplate validFrom value weight width wordCount \
worksFor worstRating";

/// Which list of names to look in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Names {
    /// Type names, such as `Event`.
    Types,
    /// Property names, such as `startDate`.
    Properties,
}

impl Names {
    fn table(self) -> &'static str {
        match self {
            Self::Types => TYPES,
            Self::Properties => PROPERTIES,
        }
    }
}

/// The known name `name` is most likely a slip for: the same name in other
/// capitals, two letters swapped, or one letter added, dropped or changed,
/// and two for a name of ten letters or more. Names under four letters are
/// only matched by their capitals. `None` when `name` is known, or is not
/// that close to a known name.
pub(crate) fn slip_for(name: &str, names: Names) -> Option<&'static str> {
    let known = || names.table().split_ascii_whitespace();
    if known().any(|word| word == name) {
        return None;
    }
    let reach = match name.len() {
        0..4 => 0,
        4..10 => 1,
        _ => 2,
    };
    let lower = name.to_ascii_lowercase();
    known()
        .filter_map(|word| {
            let other = word.to_ascii_lowercase();
            let apart = if swapped(&lower, &other) {
                1
            } else {
                edit_distance(&lower, &other)
            };
            (apart <= reach).then_some((apart, word))
        })
        .min_by_key(|(apart, _)| *apart)
        .map(|(_, word)| word)
}

/// Whether `a` is `b` with two letters side by side swapped.
fn swapped(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let differ: Vec<usize> = (0..a.len()).filter(|&i| a.get(i) != b.get(i)).collect();
    match differ.as_slice() {
        [i, j] if *j == i + 1 => a.get(*i) == b.get(*j) && a.get(*j) == b.get(*i),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{Names, PROPERTIES, TYPES, slip_for};
    use crate::schema::google_rules::TABLE;
    use crate::schema::template::is_word;

    fn sorted_names(table: &str, first: fn(&char) -> bool) {
        let words: Vec<&str> = table.split_ascii_whitespace().collect();
        assert!(words.windows(2).all(|w| w[0] < w[1]), "{words:?}");
        assert!(words.iter().all(|w| is_word(w, first)), "{words:?}");
    }

    #[test]
    fn each_table_is_sorted_once_over_and_holds_names_only() {
        sorted_names(TYPES, char::is_ascii_uppercase);
        sorted_names(PROPERTIES, char::is_ascii_lowercase);
    }

    #[test]
    fn every_name_in_google_s_rules_is_known() {
        for row in TABLE.lines() {
            let mut columns = row.split(';');
            let kinds = columns.next().unwrap_or_default();
            for kind in kinds.split_ascii_whitespace() {
                assert_eq!(slip_for(kind, Names::Types), None, "{kind}");
                assert!(TYPES.split_ascii_whitespace().any(|w| w == kind), "{kind}");
            }
            let properties = columns.skip(1).flat_map(|c| c.split([' ', '|']));
            for name in properties.filter(|n| !n.is_empty()) {
                let found = PROPERTIES.split_ascii_whitespace().any(|w| w == name);
                assert!(found, "{name}");
            }
        }
    }

    #[test]
    fn a_swap_counts_as_one_slip_and_a_far_name_as_none() {
        assert_eq!(slip_for("Eevnt", Names::Types), Some("Event"));
        assert_eq!(slip_for("Ventures", Names::Types), None);
        assert_eq!(slip_for("URL", Names::Properties), Some("url"));
        assert_eq!(slip_for("uri", Names::Properties), None);
    }
}
