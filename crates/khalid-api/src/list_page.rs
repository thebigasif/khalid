//! `page`, `limit` and `filter` for the Admin lists that once came whole
//! (RULES.md BL-API-5): redirects, tags, pages, authors, snippets, trash,
//! versions, lint issues, keys and webhooks.
//!
//! A caller that sends none of the three still gets the whole list, in the
//! list's own order, so older callers keep working. The filter reads the
//! fields each item shows in its answer, so it needs no code per list.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::IntoParams;

use crate::content_api::check_fields;
use crate::pagination::Meta;
use crate::problem::Problem;
use crate::query::{FieldValues, Limit, ListQuery, Literal, RawListQuery};

/// The list parameters an Admin list that once came whole accepts.
#[derive(Debug, Clone, Default, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub(crate) struct PageQuery {
    /// `filter=status:301`, over the fields each item shows.
    filter: Option<String>,
    /// Items per page, 1 to 100, or `all`. Without it the whole list comes
    /// on one page.
    limit: Option<String>,
    /// `page=2`, counted from 1.
    page: Option<String>,
}

/// One page of a list and where it sits.
#[derive(Debug)]
pub(crate) struct Paged<T> {
    /// The items on this page, in the list's own order.
    pub(crate) items: Vec<T>,
    /// `meta.pagination`, as every paged list carries.
    pub(crate) meta: Meta,
}

impl PageQuery {
    /// The page of `items` this query asks for, keeping their order. Only
    /// `fields` may be filtered on.
    ///
    /// # Errors
    ///
    /// A 400 [`Problem`] when a parameter cannot be read or the filter
    /// names a field not in `fields`.
    pub(crate) fn page<T: Serialize>(
        &self,
        items: Vec<T>,
        fields: &[&str],
    ) -> Result<Paged<T>, Problem> {
        let raw = RawListQuery {
            filter: self.filter.clone(),
            limit: Some(self.limit.clone().unwrap_or_else(|| "all".to_owned())),
            page: self.page.clone(),
            ..RawListQuery::default()
        };
        let query = ListQuery::try_from(&raw)?;
        check_fields(&query, fields)?;
        let found: Vec<T> = match query.filter() {
            Some(filter) => items
                .into_iter()
                .filter(|item| filter.matches(&Shown::of(item)))
                .collect(),
            None => items,
        };
        let total = u64::try_from(found.len()).unwrap_or(u64::MAX);
        let meta = Meta::new(query.page(), query.limit(), total);
        let skip = usize::try_from(meta.pagination().offset()).unwrap_or(usize::MAX);
        let take = match query.limit() {
            Limit::Count(count) => usize::try_from(count).unwrap_or(usize::MAX),
            Limit::All => usize::MAX,
        };
        let items = found.into_iter().skip(skip).take(take).collect();
        Ok(Paged { items, meta })
    }
}

/// An item as its answer shows it, for the filter to read.
struct Shown(Value);

impl Shown {
    fn of(item: &impl Serialize) -> Self {
        Self(serde_json::to_value(item).unwrap_or(Value::Null))
    }
}

impl FieldValues for Shown {
    fn values(&self, field: &str) -> Vec<Literal> {
        match self.0.get(field) {
            Some(Value::Array(values)) => values.iter().filter_map(literal).collect(),
            Some(value) => literal(value).into_iter().collect(),
            None => Vec::new(),
        }
    }
}

/// A plain value as a filter compares it. `null`, objects and numbers with
/// a fraction hold nothing.
fn literal(value: &Value) -> Option<Literal> {
    match value {
        Value::String(text) => Some(Literal::Text(text.clone())),
        Value::Bool(flag) => Some(Literal::Bool(*flag)),
        Value::Number(number) => number.as_i64().map(Literal::Integer),
        Value::Null | Value::Array(_) | Value::Object(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn query(filter: Option<&str>, limit: Option<&str>, page: Option<&str>) -> PageQuery {
        PageQuery {
            filter: filter.map(str::to_owned),
            limit: limit.map(str::to_owned),
            page: page.map(str::to_owned),
        }
    }

    fn items() -> Vec<Value> {
        (1..=5)
            .map(|n| json!({"id": n, "status": if n % 2 == 0 { 301 } else { 410 }, "tags": ["a", "b"]}))
            .collect()
    }

    #[test]
    fn no_parameters_answer_the_whole_list_in_order() {
        let paged = query(None, None, None).page(items(), &["status"]).unwrap();
        assert_eq!(paged.items, items());
        assert_eq!(paged.meta, Meta::new(1, Limit::All, 5));
    }

    #[test]
    fn limit_and_page_cut_the_list() {
        let paged = query(None, Some("2"), Some("3"));
        let paged = paged.page(items(), &["status"]).unwrap();
        let ids: Vec<&Value> = paged.items.iter().map(|item| &item["id"]).collect();
        assert_eq!(ids, [&json!(5)]);
        assert_eq!(paged.meta, Meta::new(3, Limit::Count(2), 5));
    }

    #[test]
    fn a_filter_reads_numbers_and_lists() {
        let paged = query(Some("status:301"), None, None);
        assert_eq!(paged.page(items(), &["status"]).unwrap().items.len(), 2);
        let paged = query(Some("tags:b"), None, None);
        assert_eq!(paged.page(items(), &["tags"]).unwrap().items.len(), 5);
    }

    #[test]
    fn a_field_the_list_does_not_name_is_refused() {
        let refused = query(Some("secret:x"), None, None).page(items(), &["status"]);
        assert!(refused.is_err());
        let refused = query(None, Some("0"), None).page(items(), &["status"]);
        assert!(refused.is_err());
    }
}
