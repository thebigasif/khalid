---
title: "Reel"
order: 85
---

# Reel

Reel is a template for a site that reviews films and TV series. Every review shows its star rating, the cast and where to watch, and the site reads like a printed cinema programme.

```toml
[template]
name = "reel"
```

## Who it suits

- A film critic, or a small team of critics, who reviews new releases in cinemas and on streaming services.
- A TV blog that reviews each new season.
- A site that mixes reviews with features, such as watch lists, festival reports and interviews.

For reviews of tools and products, with scores out of ten and comparison tables, Workbench fits better.

## Reviews are posts with facts

Reel needs nothing new in your posts. A post with `facts` is a review. A post without facts is a feature. Put the facts in this order, since Reel reads them by their place, not their label:

```yaml
---
title: "Saltglass review: a quiet storm of a film"
description: "A slow lighthouse drama, and the best film in cinemas this autumn."
category: films
facts:
  - label: Rating
    value: "4.5"
  - label: Year and length
    value: "2026, 118 min"
  - label: Starring
    value: "Ilse Marrow, Teodor Vance, Hanne Ostrova"
  - label: Where to watch
    value: "In cinemas, Lanternbox"
---
```

1. **The rating.** A number out of 5, such as `4` or `3.5`, shows as five stars. A number with its own scale, such as `8/10`, is drawn on five stars too. Words, such as "Must see", show as words.
2. **The release,** such as "2026, 118 min" or "Season 2, 8 episodes".
3. **The cast.** Split names with commas, and each name shows on its own. A documentary can put its director here.
4. **Where to watch.** Split services with commas, and each one shows as its own chip.

Your labels show on the page, so write "Starring" or "Voices" as fits. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

Categories are the kinds of review, such as Films, Series and Documentaries. Give each category an intro in `categories.toml`, and it shows at the top of that category's page.

## What the front page shows

1. Your site's name in large condensed capitals, with the `tagline` under it.
2. **Now showing:** the newest post marked `featured: true`, or else the newest review. Its picture sits in a letterbox frame, between two dark bars, beside the title, the summary and its ticket.
3. **Latest reviews:** the other reviews as tall poster cards. Under each picture are the title, the stars, the release and where to watch. Nothing is written over a picture.
4. **The score sheet:** the reviews on the page, highest rated first, as numbered rows with the kind, the release, where to watch and the stars. `score_sheet` sets how many. Ratings are ranked as written, so use one scale across the site.
5. **Features:** the newest posts without facts, each with its date and category. `features_count` sets how many.
6. Numbered page links, your call to action as a band, and the newsletter form.

A site with no reviews yet shows its posts as poster cards, with no score sheet.

## What a review shows

1. The trail from the home page to the review's category.
2. The category as a small label, the title, and the post's description under it.
3. The critic with their mark and role, and the date.
4. The lead picture in a letterbox frame, wider than the text on a wide screen.
5. **The ticket:** a panel with a notch cut from each side. Its stub holds the stars and the score. Past a dashed line are the release, the cast and where to watch.
6. The review in one column, with a folded contents list for long posts.
7. The share links and the critic's box.
8. More reviews as poster cards with their stars.

A feature has no ticket.

## Other pages

- **Lists** (a category, a tag, the archive): the list's name and intro, then its posts as poster cards with their stars and release.
- **Critics** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A critic's page:** their photo or mark, role, experience and bio, then their reviews as poster cards, and the other critics.

## Your brand

Reel brings its own colours: warm programme paper (`#fbf8f3`), near-black ink and one cinema red (`#9b1c2e`) for the stars, links and buttons, with a deeper paper surface for the ticket. Your colours, fonts and logo in `theme/tokens.toml` replace them. A condensed sans for headings suits it best, since the site's name and section labels are set in capitals.

Reel's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Reel's default |
|---|---|
| `[fonts] heading` | Avenir Next Condensed, then other condensed sans fonts |
| `[fonts] body` | Charter, then Georgia and other serifs |
| `[type] body` | `1.0625rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.5rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.1875rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.65` |
| `[type] heading_weight` | `600` |
| `[shape] radius` | `0.25rem` |
| `[shape] measure` | `40rem` |
| `[space] section` | `4rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the site's name on the front page. |
| `score_sheet` | `8` | How many reviews the score sheet ranks, from `0` to `20`. `0` leaves it out. |
| `features_count` | `4` | How many features show on the front page, from `0` to `10`. |
| `featured` | `true` | The newest post marked featured leads the front page as Now showing. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `4` | How many more reviews show under a review, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"numbered"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "whatsapp", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
