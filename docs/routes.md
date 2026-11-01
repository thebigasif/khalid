# The blog's addresses: `routes.toml`

`routes.toml` sits beside `site.toml` and sets how the blog's web addresses look. `khalid init` writes it with only a comment, and that is enough: a post sits at `/<slug>/`, tags at `/tag/<tag>/` and categories at `/category/<category>/`, all under the blog's mount path, such as `/blog/`.

Change it only when you want other addresses. Khalid reads it when it starts, so restart `khalid serve` after an edit. A mistake stops the start with the line and the setting at fault.

## Collections: where posts live and what their address is

A collection is a folder in `content/<language>/`. Its posts share one address pattern, called a permalink. `posts` is the one every site has.

```toml
[collections.posts]
permalink = "/articles/{slug}/"

[collections.news]
permalink = "/news/{year}/{slug}/"
news = true
```

- A permalink starts and ends with `/` and holds `{slug}` once, so every post has its own address.
- It may also hold `{year}`, `{month}`, `{day}` and `{collection}`, each once. They come from the post's `date:`.
- `news = true` marks a collection's posts as news: search engines get the `NewsArticle` type and a news sitemap.
- The language prefix, such as `/de/`, and the mount path are added for you.

Change the permalink of a live blog with care: the old addresses stop working. Add each old address to the post's `aliases:` so readers and search engines are sent on.

## Channels: extra lists that leave addresses alone

A channel is a page with its own feed that lists some posts. It never changes a post's own address.

```toml
[channels.featured]
path = "/featured/"
filter = { featured = true }

[channels.from-the-founder]
path = "/from-the-founder/"
filter = { author = "sam-rivera" }
```

A filter needs at least one of `author`, `tag`, `category`, `collection` or `featured`. A post must match every one that is set.

## Tag and category folders

```toml
[taxonomies]
tag = "topic"
category = "section"
```

Each is one lowercase word, or words joined by hyphens. With the lines above, tag pages move from `/tag/<tag>/` to `/topic/<tag>/`.
