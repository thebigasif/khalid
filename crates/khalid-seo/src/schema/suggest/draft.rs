//! [`draft_schema`]: JSON-LD for a suggested type, filled in from what the
//! post says, ready for the writer to check and keep in the post's schema
//! field.

use super::facts::{FoundAddress, FoundHours, FoundPrice, PostFacts};
use crate::{SchemaGraph, SchemaNode, SchemaValue};

/// JSON-LD for `kind`, a type [`suggest_schema`](super::suggest_schema)
/// suggests, filled in from `facts` and the post's `title`: prices with
/// their currency, addresses, phone numbers, opening hours, the author,
/// dates and social profiles. A breadcrumb trail the post writes joins the
/// graph. `None` for a type it does not draft.
pub fn draft_schema(kind: &str, facts: &PostFacts, title: &str) -> Option<String> {
    let mut nodes = vec![match kind {
        "Recipe" => recipe(facts, title),
        "Article" => article(facts, title),
        "Product" => product(facts, title),
        "LocalBusiness" => business(facts, title),
        "FAQPage" => questions(facts),
        _ => return None,
    }];
    if let Some(trail) = breadcrumbs(&facts.breadcrumbs) {
        nodes.push(trail);
    }
    Some(SchemaGraph::new(nodes).to_json())
}

fn texts(items: &[String]) -> SchemaValue {
    SchemaValue::List(items.iter().map(|item| SchemaValue::text(item)).collect())
}

fn recipe(facts: &PostFacts, title: &str) -> SchemaNode {
    let steps = facts.steps.iter().map(|step| {
        SchemaValue::Node(Box::new(
            SchemaNode::new("HowToStep").with("text", SchemaValue::text(step)),
        ))
    });
    let mut node = SchemaNode::new("Recipe").with("name", SchemaValue::text(title));
    if !facts.ingredients.is_empty() {
        node.set("recipeIngredient", texts(&facts.ingredients));
    }
    if !facts.steps.is_empty() {
        node.set("recipeInstructions", SchemaValue::List(steps.collect()));
    }
    with_author(node, facts)
}

fn article(facts: &PostFacts, title: &str) -> SchemaNode {
    let mut node = SchemaNode::new("Article").with("headline", SchemaValue::text(title));
    if let Some(day) = facts.dates.first() {
        node.set("datePublished", SchemaValue::Date(*day));
    }
    with_author(node, facts)
}

/// `node` with the byline's writer as its `author`, with the social
/// profiles the post links as their `sameAs`.
fn with_author(mut node: SchemaNode, facts: &PostFacts) -> SchemaNode {
    if let Some(name) = &facts.byline {
        let mut person = SchemaNode::new("Person").with("name", SchemaValue::text(name));
        if !facts.profiles.is_empty() {
            person.set("sameAs", texts(&facts.profiles));
        }
        node.set("author", SchemaValue::Node(Box::new(person)));
    }
    node
}

fn offer(price: &FoundPrice) -> SchemaValue {
    SchemaValue::Node(Box::new(
        SchemaNode::new("Offer")
            .with("price", SchemaValue::text(&price.amount))
            .with("priceCurrency", SchemaValue::text(&price.currency)),
    ))
}

fn product(facts: &PostFacts, title: &str) -> SchemaNode {
    let mut node = SchemaNode::new("Product").with("name", SchemaValue::text(title));
    match facts.prices.as_slice() {
        [] => {}
        [one] => node.set("offers", offer(one)),
        many => node.set(
            "offers",
            SchemaValue::List(many.iter().map(offer).collect()),
        ),
    }
    node
}

fn address(found: &FoundAddress) -> SchemaValue {
    let mut node =
        SchemaNode::new("PostalAddress").with("streetAddress", SchemaValue::text(&found.street));
    if let Some(town) = &found.locality {
        node.set("addressLocality", SchemaValue::text(town));
    }
    if let Some(code) = &found.postal_code {
        node.set("postalCode", SchemaValue::text(code));
    }
    SchemaValue::Node(Box::new(node))
}

fn opening(hours: &FoundHours) -> SchemaValue {
    let days = hours.days.iter().map(|day| SchemaValue::text(day));
    SchemaValue::Node(Box::new(
        SchemaNode::new("OpeningHoursSpecification")
            .with("dayOfWeek", SchemaValue::List(days.collect()))
            .with("opens", SchemaValue::text(&hours.opens))
            .with("closes", SchemaValue::text(&hours.closes)),
    ))
}

fn business(facts: &PostFacts, title: &str) -> SchemaNode {
    let mut node = SchemaNode::new("LocalBusiness").with("name", SchemaValue::text(title));
    if let Some(found) = facts.addresses.first() {
        node.set("address", address(found));
    }
    if let Some(phone) = facts.phones.first() {
        node.set("telephone", SchemaValue::text(phone));
    }
    if !facts.hours.is_empty() {
        let hours = facts.hours.iter().map(opening);
        node.set(
            "openingHoursSpecification",
            SchemaValue::List(hours.collect()),
        );
    }
    if !facts.profiles.is_empty() {
        node.set("sameAs", texts(&facts.profiles));
    }
    node
}

fn questions(facts: &PostFacts) -> SchemaNode {
    let asked = facts.questions.iter().map(|(question, answer)| {
        let answer = SchemaNode::new("Answer").with("text", SchemaValue::text(answer));
        SchemaValue::Node(Box::new(
            SchemaNode::new("Question")
                .with("name", SchemaValue::text(question))
                .with("acceptedAnswer", SchemaValue::Node(Box::new(answer))),
        ))
    });
    SchemaNode::new("FAQPage").with("mainEntity", SchemaValue::List(asked.collect()))
}

fn breadcrumbs(trail: &[String]) -> Option<SchemaNode> {
    if trail.is_empty() {
        return None;
    }
    let items = trail.iter().zip(1_i64..).map(|(name, position)| {
        SchemaValue::Node(Box::new(
            SchemaNode::new("ListItem")
                .with("position", SchemaValue::Integer(position))
                .with("name", SchemaValue::text(name)),
        ))
    });
    Some(
        SchemaNode::new("BreadcrumbList")
            .with("itemListElement", SchemaValue::List(items.collect())),
    )
}

#[cfg(test)]
mod tests {
    use super::draft_schema;
    use crate::schema::suggest::read_facts;
    use crate::{AuditContext, CustomSchema, audit_json_ld};

    const BAR: &str = "Home > Places > Noodle bar\n\nFind us at 12 High Street, Leeds LS1 4DY. \
        Call us on 0113 496 0000. Open Monday to Friday, 11am to 10pm. \
        Follow https://www.instagram.com/noodlebar\n";

    #[test]
    fn a_business_draft_carries_address_phone_hours_profiles_and_trail() {
        let text = draft_schema("LocalBusiness", &read_facts(BAR), "Leeds Noodle Bar").unwrap();
        for wanted in [
            "\"streetAddress\": \"12 High Street\"",
            "\"postalCode\": \"LS1 4DY\"",
            "\"telephone\": \"0113 496 0000\"",
            "\"opens\": \"11:00\"",
            "\"closes\": \"22:00\"",
            "\"https://www.instagram.com/noodlebar\"",
            "\"@type\": \"BreadcrumbList\"",
        ] {
            assert!(text.contains(wanted), "{wanted} in {text}");
        }
    }

    #[test]
    fn every_draft_is_taken_by_the_schema_field_and_has_no_schema_org_error() {
        let facts = read_facts(&format!(
            "{BAR}\n*By Jane Doe*\n\n12 November 2026. The kit costs $19.99 or €18,50.\n\n\
             ## Ingredients\n\n- oats\n- butter\n\n## Method\n\n1. Melt.\n2. Stir.\n\n\
             ## Is it safe?\n\nYes, it is.\n"
        ));
        for kind in ["Recipe", "Article", "Product", "LocalBusiness", "FAQPage"] {
            let text = draft_schema(kind, &facts, "Oat bars").unwrap();
            assert!(CustomSchema::parse(&text).is_ok(), "{kind}: {text}");
            let found = audit_json_ld(&text, &AuditContext::default()).unwrap();
            assert_eq!(found, [], "{kind}: {text}");
        }
        assert_eq!(draft_schema("Event", &facts, "Oat bars"), None);
    }
}
