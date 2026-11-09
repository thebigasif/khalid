---
title: "Ghost's Content API"
order: 100
---

# Ghost's Content API

A front end built on Ghost's Content API can read a Khalid blog instead. It keeps its code and Ghost's own client, `@tryghost/content-api`, and changes two things: the blog's address and the key.

This API only reads. It answers at `/ghost/api/content/` on the blog's address, the same path Ghost uses. It is off until you switch it on.

## Switch it on

Add this line to `site.toml`, above the first table:

```toml
ghost_content_api = true
```

Restart `khalid serve`. While the line is missing or `false`, every address under `/ghost/api/content/` answers 404, as any unknown address does.

Make a content key for the front end:

```sh
khalid keys create --kind content --name "Ghost front end"
```

Khalid shows the key once. It is safe in a web page, as a Ghost content key is.

## Point the front end at Khalid

```js
import GhostContentAPI from "@tryghost/content-api";

const api = new GhostContentAPI({
  url: "https://www.example.com/blog",
  key: "the key Khalid showed",
  version: "v5.0",
});

const posts = await api.posts.browse({ limit: 5, include: "tags,authors" });
```

`url` is the blog's address with its mount path and no slash at the end. Ghost's client adds `/ghost/api/content/` itself. The API takes Ghost's `Accept-Version` header for v5, which the client sends. Khalid's own Content API, at `/api/content/v1/`, does not change.

## What it answers

| Resource | Browse | Read |
|---|---|---|
| Posts | `posts/` | `posts/<id>/`, `posts/slug/<slug>/` |
| Pages | `pages/` | `pages/<id>/`, `pages/slug/<slug>/` |
| Tags | `tags/` | `tags/<id>/`, `tags/slug/<slug>/` |
| Authors | `authors/` | `authors/<id>/`, `authors/slug/<slug>/` |
| Settings | `settings/` | |

A list takes `limit` (a number, or `all`), `page`, `order`, `filter`, `include=tags,authors` on posts and pages, `fields`, and `formats`. Posts come with `html`; ask `formats=plaintext` for the text as well.

Answers use Ghost's wrapper, such as `{"posts": [...], "meta": {"pagination": {...}}}`, and Ghost's field names. Each carries an `ETag`, so a front end or a cache can ask again cheaply and get 304 when nothing changed.

**One language.** A Ghost site has one language, so this API answers in the site's default language only. Posts, tags and authors in the other languages stay out, and so does a translation that shares its slug with the original.

### Filters

The common part of Ghost's filter language works:

- the fields `tag`, `author`, `featured`, `slug`, `id`, `status`, `visibility`, `published_at`, `primary_tag.slug` and `primary_author.slug`, with `tags.slug` and `authors.slug` as other names for `tag` and `author`;
- `+` for and, `,` for or, and brackets for groups;
- `-` for not, as in `primary_tag.slug:-news`;
- lists, as in `tag:[news,guides]`;
- `>`, `<`, `>=` and `<=` on `published_at`.

A filter that uses anything else answers Ghost's 400 error, with a message that names the part this API does not answer.

### How Khalid's posts look in Ghost's fields

| Ghost field | From Khalid |
|---|---|
| `id`, `slug`, `title`, `url` | The post's own |
| `html`, `plaintext` | The post's body as its page shows it |
| `feature_image`, `meta_title`, `meta_description` | The post's page head: its share image, title and description |
| `custom_excerpt` | The post's `description` |
| `excerpt` | The `description`, or the start of the text |
| `published_at`, `created_at`, `updated_at` | The post's date and its `updated` date |
| `reading_time` | Words over 275, rounded, as Ghost counts |
| `featured` | `featured: true` in the front matter |
| `tags`, `primary_tag` | The post's tags; the first is the primary tag |
| `authors`, `primary_author` | The post's authors, with `profile_image`; the first is the primary author |

A members-only post comes with `visibility: "members"`, `access: false` and an empty `html`, as Ghost answers a key with no signed-in member.

Fields Khalid has no meaning for carry Ghost's neutral values: `comments` is `false`, and code injection, `og_*`, `twitter_*`, `canonical_url`, `custom_template`, `email_subject` and image captions are `null`.

### Settings

`settings/` gives the blog's name as `title`, the `[seo.titles]` tagline as `description` and `meta_description`, the publisher's logo as `logo`, the share image as `cover_image`, the X account as `twitter`, the default language as `lang` and `locale`, the header menu as `navigation`, the footer menu as `secondary_navigation`, and the blog's address as `url`. The time zone is `Etc/UTC`, since Khalid keeps times in UTC.

## Keys and errors

The key goes in `?key=`, as Ghost's client sends it. A missing or wrong key answers Ghost's 401, `UnauthorizedError`. An address with nothing at it answers Ghost's 404, `NotFoundError`. A request this API cannot read answers Ghost's 400, `BadRequestError`. Each error is in Ghost's shape: `{"errors": [{"message": "...", "type": "..."}]}`. A content key has the same request limits here as on Khalid's Content API (see `docs/standards/api.md`), counted apart.

## Not covered

- Ghost's Handlebars themes. Khalid's templates draw the pages.
- Ghost's Admin API. Khalid's Admin API keeps its own shape.
- Members, tiers, offers and newsletters through this path.
- The look of Ghost's editor cards (`kg-*` classes) inside `html`.

Each is a separate piece of work. Khalid's own CI runs Ghost's client, at a pinned version, against the showcase site on every change to this API, so a break shows at once. `just ghost-client` runs the same check on your machine.
