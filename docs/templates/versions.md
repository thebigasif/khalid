---
title: "Versions"
order: 95
---

# Versions

Versions is a template for a software product's changelog. Every change sits on a version rail, grouped by the version it shipped in. Readers filter the changes by type, such as new, improved or fixed, and get updates by email or in a feed reader.

```toml
[template]
name = "versions"
```

## Who it suits

- A software company or a small team that ships versions of an app and wants a public changelog.
- An open source project that writes a short note on each change.
- A product team whose readers want to follow only some changes, such as the fixes.

It has no layout for essays. For a product team that also writes long notes on how it builds, Folio reads better.

## Changes are posts

Versions needs nothing new in your posts. Each post is one change:

- **Its category is its type of change,** such as New, Improved or Fixed.
- **Its first fact is the version** it shipped in. Add up to three more facts, such as the platforms and the plans it reaches.
- **Its tags are the product areas** it touches, such as Boards or Mobile apps.

```yaml
---
title: "Recurring cards repeat a task on the days you choose"
category: new
description: "Recurring cards come back on a schedule you set, so routine work never drops off the board."
tags: [boards, calendar]
facts:
  - label: Version
    value: "3.4.0"
  - label: Platforms
    value: "Web, iOS, Android"
  - label: Plans
    value: "All plans"
---
```

Versions reads the facts by their place, not their label, so the labels can be in any language. Changes that come one after another with the same first fact form one release. Give the changes of one version the same date, so they sit together. A post without facts forms a release of its own date. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

Each type of change is a category. Write them in `categories.toml` with an intro each, and list their slugs in order in `change_types`. The first type is marked as new, with a filled chip, the second as improved, with a tinted chip and a ring, and the third as fixed, with a plain chip and a square. A fourth, such as `removed`, gets a dashed chip. The marks differ in shape as well as colour, so they read without colour too.

```toml
[template.options]
change_types = ["new", "improved", "fixed"]
```

## What the front page shows

1. "Changelog" as a small label, your product's name large, and the `tagline` under it.
2. **The filter row:** "All changes", then each type of change as a chip. Each chip is a plain link to that type's page, so the filter works with scripts off.
3. **The version rail:** each version in large monospace with its date on a rail, a node on a thin line (filled for the newest), and its changes beside it. Each change shows its type chip, its title and its summary.
4. **Get updates,** beside the rail on wide screens and after it on phones: the sign-up form when your site takes sign-ups, then a link to the feed of every change and one per type, such as only the fixes.
5. **Versions on this page:** each version as a link to its place on the rail.
6. Numbered page links, and your call to action as a band.

## What a change shows

1. The trail from the home page to the change's type.
2. The type chip, which opens every change of that type, and the version.
3. The title, the post's description as a one-line summary, the date and the author with their role.
4. **The facts row:** the version in monospace, then the other facts, in one ruled panel.
5. The lead picture, such as a screenshot, framed by a hairline. Then the text in one column about 37rem wide, at 18px, with a folded contents list for long posts.
6. The areas (tags) as chips, the author box and the newsletter form.
7. More changes as a short version rail.

## Other pages

- **A type of change** (a category page): its name and intro, a link to its own feed, the filter row with this type marked, then its changes on the rail and its versions beside them.
- **An area** (a tag page), **the archive** and other lists: the same layout. Their filter row holds "All changes", then every menu link in the same folder as the list, from the header, the footer and the footer groups. So put your areas in a footer group, and an area's page shows every area as a chip.
- **Authors** (`/authors/`): everyone in a grid with their mark or photo, role and how many changes they wrote.
- **An author's page:** their photo or mark, role, experience and bio, then their changes on the rail.

## Your brand

Versions brings its own colours: a blue accent (`#1d4fb8`) on white with a pale grey surface for the update box and the facts row. Your colours, fonts and logo in `theme/tokens.toml` replace them. Version numbers are set in your `[fonts] mono`, so a monospace with clear figures suits it best.

Versions' own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Versions' default |
|---|---|
| `[fonts] heading` | The system's own sans |
| `[fonts] body` | The system's own sans |
| `[fonts] mono` | The system's own monospace |
| `[type] body` | `1rem` on a phone to `1.0625rem` on a wide screen; a change's text is `1.125rem` |
| `[type] h1` | `2rem` to `2.75rem` |
| `[type] h2` | `1.375rem` to `1.625rem` |
| `[type] h3` | `1.125rem` to `1.25rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `600` |
| `[shape] radius` | `0.375rem` |
| `[shape] measure` | `40rem` |
| `[space] section` | `3.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the product's name on the front page. |
| `change_types` | `["new", "improved", "fixed"]` | The slugs of the categories that are types of change, in order, which sets each type's mark. Up to four. |
| `feed_links` | `true` | The update box and each type's page link to their feeds. |
| `version_index` | `true` | The front page and lists show the versions on the page beside the rail, as anchor links. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A change shows its description under its title. |
| `updated_note` | `false` | An updated change says why it was updated, from its `updated_note`. |
| `cards` | `"row"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `4` | How many more changes show under a change, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long change's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"none"` | Where a change shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["linkedin", "bluesky", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"post"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. The front page always has its own update box. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a button, such as "Try it free". |
