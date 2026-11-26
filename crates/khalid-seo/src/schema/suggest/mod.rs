//! [`suggest_schema`]: schema types a post's content suggests, beyond what
//! its front matter and blocks already give it, each with the reason and a
//! confidence.

mod draft;
mod facts;

use serde::Serialize;

pub use draft::draft_schema;
pub use facts::{FoundAddress, FoundHours, FoundPrice, PostFacts, read_facts};

use crate::schema::vocabulary::Vocabulary;

/// One schema type the content suggests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchemaSuggestion {
    /// The schema.org type, such as `Recipe`.
    pub kind: &'static str,
    /// Why, naming what the post says, such as "The post lists 6
    /// ingredients and 4 steps."
    pub reason: String,
    /// How sure the suggestion is, from 1 to 100.
    pub confidence: u8,
}

/// The types `facts` suggest, most likely first, leaving out any type that
/// `present`, the types the page's graph already carries, holds or extends.
pub fn suggest_schema(facts: &PostFacts, present: &[&str]) -> Vec<SchemaSuggestion> {
    let mut found: Vec<SchemaSuggestion> = [
        recipe(facts),
        article(facts),
        product(facts),
        business(facts),
        questions(facts),
    ]
    .into_iter()
    .flatten()
    .filter(|suggestion| !carried(suggestion.kind, present))
    .collect();
    found.sort_by_key(|suggestion| std::cmp::Reverse(suggestion.confidence));
    found
}

/// Whether a type in `present` is `kind` or a kind of it.
fn carried(kind: &str, present: &[&str]) -> bool {
    let vocabulary = Vocabulary::get();
    present.iter().any(|have| vocabulary.is_a(have, kind))
}

fn suggestion(kind: &'static str, reason: String, confidence: u8) -> SchemaSuggestion {
    SchemaSuggestion {
        kind,
        reason,
        confidence,
    }
}

fn count(n: usize, one: &str, many: &str) -> String {
    if n == 1 {
        format!("1 {one}")
    } else {
        format!("{n} {many}")
    }
}

fn recipe(facts: &PostFacts) -> Option<SchemaSuggestion> {
    let (ingredients, steps) = (facts.ingredients.len(), facts.steps.len());
    let confidence = match (ingredients, steps) {
        (2.., 2..) => 90,
        (3.., _) | (_, 3..) if ingredients > 0 => 60,
        _ => return None,
    };
    let reason = format!(
        "The post lists {} and {}.",
        count(ingredients, "ingredient", "ingredients"),
        count(steps, "step", "steps")
    );
    Some(suggestion("Recipe", reason, confidence))
}

fn article(facts: &PostFacts) -> Option<SchemaSuggestion> {
    let name = facts.byline.as_ref()?;
    let date = facts.dates.first()?;
    let reason = format!(
        "The post names its writer, {name}, and a date, {}.",
        date.format("%-d %B %Y")
    );
    Some(suggestion("Article", reason, 75))
}

fn product(facts: &PostFacts) -> Option<SchemaSuggestion> {
    let first = facts.prices.first()?;
    let confidence = if facts.prices.len() > 1 { 70 } else { 55 };
    let reason = format!(
        "The post gives {}, such as {} {}.",
        count(facts.prices.len(), "price", "prices"),
        first.amount,
        first.currency
    );
    Some(suggestion("Product", reason, confidence))
}

fn business(facts: &PostFacts) -> Option<SchemaSuggestion> {
    let address = facts.addresses.first()?;
    let more = usize::from(!facts.phones.is_empty()) + usize::from(!facts.hours.is_empty());
    let confidence = match more {
        0 => 55,
        1 => 75,
        _ => 90,
    };
    let mut said = vec![format!("an address, {}", address.street)];
    if let Some(phone) = facts.phones.first() {
        said.push(format!("a phone number, {phone}"));
    }
    if !facts.hours.is_empty() {
        said.push("opening hours".to_owned());
    }
    let reason = format!("The post gives {}.", either(&said));
    Some(suggestion("LocalBusiness", reason, confidence))
}

fn questions(facts: &PostFacts) -> Option<SchemaSuggestion> {
    let asked = facts.questions.len();
    let confidence = match asked {
        0 => return None,
        1 => 45,
        2 => 65,
        _ => 80,
    };
    let reason = format!(
        "The post asks {} in its headings and answers each below it.",
        count(asked, "question", "questions")
    );
    Some(suggestion("FAQPage", reason, confidence))
}

/// `a`, `a and b`, `a, b and c`.
fn either(parts: &[String]) -> String {
    match parts.split_last() {
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::{read_facts, suggest_schema};

    #[test]
    fn a_recipe_post_suggests_a_recipe_first() {
        let facts = read_facts(
            "Our kit costs $12.\n\n## Ingredients\n\n- oats\n- butter\n- honey\n\n\
             ## Method\n\n1. Melt.\n2. Stir.\n3. Bake.\n",
        );
        let found = suggest_schema(&facts, &["BlogPosting"]);
        let kinds: Vec<&str> = found.iter().map(|s| s.kind).collect();
        assert_eq!(kinds, ["Recipe", "Product"]);
        assert_eq!(found[0].reason, "The post lists 3 ingredients and 3 steps.");
        assert_eq!(found[0].confidence, 90);
    }

    #[test]
    fn a_type_the_page_already_carries_is_not_suggested_again() {
        let facts = read_facts("*By Jane Doe*\n\nUpdated 2026-11-12.\n");
        assert_eq!(suggest_schema(&facts, &[])[0].kind, "Article");
        // A blog post is a kind of article.
        assert_eq!(suggest_schema(&facts, &["BlogPosting"]), []);
    }

    #[test]
    fn an_address_with_a_phone_and_hours_is_a_sure_business() {
        let facts = read_facts(
            "Find us at 12 High Street, Leeds. Call 0113 496 0000. Open Mon-Sat 9am-6pm.",
        );
        let found = suggest_schema(&facts, &[]);
        assert_eq!(found[0].kind, "LocalBusiness");
        assert_eq!(found[0].confidence, 90);
        assert_eq!(
            found[0].reason,
            "The post gives an address, 12 High Street, a phone number, 0113 496 0000 and opening hours."
        );
    }

    #[test]
    fn questions_in_headings_suggest_a_faq_page() {
        let facts = read_facts("## Is it safe?\n\nYes.\n\n## Does it keep?\n\nFor a year.\n");
        let found = suggest_schema(&facts, &[]);
        assert_eq!((found[0].kind, found[0].confidence), ("FAQPage", 65));
        assert_eq!(suggest_schema(&read_facts("Plain words."), &[]), []);
    }
}
