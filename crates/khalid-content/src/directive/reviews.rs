//! Reading the scored blocks: a review and a product card, with the
//! readers' scores both can carry.

use super::Attributes;
use crate::{BlockError, BlockKind, ProductBlock, REVIEW_TYPES, ReaderRatings, ReviewBlock};

/// Reads a review: the item, its type, the score and the reviewer's words.
pub(super) fn review(attributes: &Attributes, body: &[&str]) -> Result<BlockKind, BlockError> {
    attributes.only(
        "review",
        &[
            "item", "type", "rating", "best", "average", "ratings", "author",
        ],
        "item, type, rating, best, average, ratings and author",
    )?;
    let item = attributes.need("item", "a review needs item, the thing it reviews")?;
    let item_type = attributes.need(
        "type",
        "a review needs type, such as Product, Book or SoftwareApplication",
    )?;
    if !REVIEW_TYPES.contains(&item_type.as_str()) {
        return Err(BlockError::Value {
            line: attributes.line,
            problem: "a review's type is one search engines show, such as Product, Book, Course, SoftwareApplication or LocalBusiness",
        });
    }
    let rating = attributes.need("rating", "a review needs rating, its score")?;
    let best = attributes.get("best").unwrap_or_else(|| "5".to_owned());
    match (score(&rating), score(&best)) {
        (Some(value), Some(top)) if top > 0.0 && value <= top => {}
        _ => {
            return Err(BlockError::Value {
                line: attributes.line,
                problem: "a review's rating is a number from 0 to its best score, such as 4.5 out of 5",
            });
        }
    }
    Ok(BlockKind::Review(ReviewBlock {
        item,
        item_type,
        rating,
        readers: readers(attributes, &best)?,
        best,
        author: attributes.get("author"),
        body: body.join("\n"),
    }))
}

/// Reads a product card: its name, price, score and the link to buy it.
pub(super) fn product(attributes: &Attributes, body: &[&str]) -> Result<BlockKind, BlockError> {
    attributes.only(
        "product",
        &PRODUCT_ATTRIBUTES,
        "name, image, price, currency, rating, best, average, ratings, href and button",
    )?;
    let name = attributes.need("name", "a product needs name, what it is called")?;
    let (price, currency) = (attributes.get("price"), attributes.get("currency"));
    let money = match (&price, &currency) {
        (Some(price), Some(code)) => {
            let (units, cents) = price.split_once('.').unwrap_or((price, "00"));
            units.parse::<u64>().is_ok()
                && cents.len() == 2
                && cents.parse::<u8>().is_ok()
                && code.len() == 3
                && code.bytes().all(|b| b.is_ascii_uppercase())
        }
        (None, None) => true,
        _ => false,
    };
    let best = attributes.get("best").unwrap_or_else(|| "5".to_owned());
    let rating = attributes.get("rating");
    let rated = rating.as_deref().is_none_or(|rating| {
        matches!((score(rating), score(&best)), (Some(value), Some(top)) if top > 0.0 && value <= top)
    });
    let problem = if !money {
        "a product's price is a number such as 49.00 with a three-letter currency such as USD"
    } else if !rated {
        "a product's rating is a number from 0 to its best score, such as 4.5 out of 5"
    } else if attributes.get("href").is_some() != attributes.get("button").is_some() {
        "a product with href needs button, the words on its button"
    } else {
        ""
    };
    if !problem.is_empty() {
        return Err(BlockError::Value {
            line: attributes.line,
            problem,
        });
    }
    let address = |key: &'static str| {
        attributes
            .get(key)
            .map(|text| attributes.media("product", key, text))
            .transpose()
    };
    Ok(BlockKind::Product(ProductBlock {
        name,
        image: address("image")?,
        price,
        currency,
        rating,
        readers: readers(attributes, &best)?,
        best,
        href: address("href")?,
        button: attributes.get("button"),
        description: body.join("\n"),
    }))
}

const PRODUCT_ATTRIBUTES: [&str; 10] = [
    "name", "image", "price", "currency", "rating", "best", "average", "ratings", "href", "button",
];

/// Readers' scores from `average` and `ratings`, which come together: an
/// average from 0 to `best`, and a whole count of at least one.
fn readers(attributes: &Attributes, best: &str) -> Result<Option<ReaderRatings>, BlockError> {
    let (average, ratings) = (attributes.get("average"), attributes.get("ratings"));
    let count = ratings.as_deref().map(str::parse::<u32>);
    let readers = match (average, count) {
        (None, None) => return Ok(None),
        (Some(average), Some(Ok(count))) if count > 0 => ReaderRatings { average, count },
        _ => return Err(readers_problem(attributes)),
    };
    match (score(&readers.average), score(best)) {
        (Some(value), Some(top)) if value <= top => Ok(Some(readers)),
        _ => Err(readers_problem(attributes)),
    }
}

/// The mistake in a block's readers' scores.
fn readers_problem(attributes: &Attributes) -> BlockError {
    BlockError::Value {
        line: attributes.line,
        problem: "readers' scores need both average, from 0 to the best score, and ratings, how many there are, such as average=\"4.2\" ratings=\"310\"",
    }
}

/// Reads a score written with digits and at most one point, such as `4.5`.
fn score(text: &str) -> Option<f64> {
    let digits = text.bytes().all(|b| b.is_ascii_digit() || b == b'.');
    digits.then(|| text.parse().ok()).flatten()
}
