Added
- The Admin API can change a post's category and tags. Post settings carry `category` and `tags`, an edit may send either, and the posts table rows show both, so a list can filter by `category:on-page-seo` or `tag:'internal links'`. A category the site's `categories.toml` lacks is refused, and so is a tag listed twice, so a saved post always reads back.
