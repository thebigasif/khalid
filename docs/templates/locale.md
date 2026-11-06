---
title: "Locale"
order: 85
---

# Locale

Locale is a template for a city guide: the places worth a visit in one city or town, listed by area. Every place shows its kind, its opening hours and its price level, set like an entry in a printed guide, so a reader knows before they walk over whether it is open and what it costs.

```toml
[template]
name = "locale"
```

## Who it suits

- A local writer or a small team who keeps a guide to one city or town.
- A neighbourhood site, a visitor board or a hotel that lists the places it recommends.
- A food, coffee or shopping guide to one place, where every listing needs its hours and price.

It suits trip diaries across many countries less well, and step-by-step guides not at all. For those, Atlas or Compass reads better.

## Places are posts with facts

Locale needs nothing new in your posts. A post with `facts` is a place. A post without facts is a note, such as a walk, a season's tips or how you choose your places. Put the facts in this order, since Locale reads them by their place, not their label:

```yaml
---
title: "Saggar Bakehouse: sourdough from the last pottery kiln"
category: kiln-quarter
facts:
  - label: Kind
    value: Bakery
  - label: Price
    value: "£"
  - label: Hours
    value: "Tue to Sun, 7:30 to 15:00"
  - label: Address
    value: 14 Kiln Yard, Kiln Quarter
---
```

1. **The kind of place,** such as `Café`, `Bookshop` or `Park`.
2. **The price level.** Write `1` to `4`, or one to four of the same sign, such as `££` or `€€€`. Either shows as a price meter of four marks, with the marks up to the level in ink: round coins for a number, the sign itself for signs. Any other value, such as `Free`, shows as written. Screen readers hear "Price level 2 of 4", not the marks.
3. **The opening hours,** such as `Daily, 7:00 to 17:00`.
4. **The address.**

Your labels show on the place's page, so write them in your own words. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

**Areas are categories,** such as Old Harbour or Hillgate. List them in `categories.toml` in the order you want them on the front page. A place without a category does not show on the front page. Tags are themes across areas, such as coffee, rainy day or open late.

**A place's name** is the part of its title before a colon. "Saggar Bakehouse: sourdough from the last pottery kiln" lists as "Saggar Bakehouse" on every listing row. On the place's own page the name is large and the rest of the title sits under it. The full title stays in search results.

## What the front page shows

1. Your site's name, its intro or `tagline`, and a chip per area with how many places it holds. Each chip jumps to that area further down the page.
2. **Places by area:** a block per area, in the order `categories.toml` lists them, with the area's name, its place count and a link to the area's page. Its places follow as listing rows, two across on a laptop. An area with only notes gets no block.
3. **The listing row:** a small square picture, the place's name, a dotted leader and the price meter on one line, then the kind and the opening hours in small capitals.
4. **Notes from the city:** the posts without facts as picture cards, three across on a laptop.
5. Numbered page links, your call to action as a band and the newsletter form.

With `[[reader.home_sections]]` in `site.toml`, each section shows as an area block of its own instead.

## What a post shows

1. The area as a small label with a pin, the place's name large, and the post's description under it.
2. The author with their mark and role, and the date.
3. **The picture beside the At a glance card** on wider screens, and above it on a phone. The card shows the price meter large, then the kind, the hours and the address, each with your label.
4. The post in one column, at 18 px or larger.
5. The tags, the share links and the author box.
6. **More places:** related places as listing rows, and related notes as cards.

A note has no At a glance card. Its picture sits under the byline.

## Other pages

- **Lists** (an area, a tag, the archive): the list's name and intro, then its places as listing rows, two across on a laptop, then its notes as a dated feed.
- **Writers** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A writer's page:** their photo or mark, role, experience and bio, then the places they wrote up as listing rows, their notes as a feed, and the other writers.

## Your brand

Locale brings its own colours: a deep harbour teal (`#1d5c57`) on warm paper, with near-black for the double rules and the price marks. Your colours, fonts and logo in `theme/tokens.toml` replace them. A sturdy serif for headings suits it best, since every place name is set in the heading font.

Locale's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Locale's default |
|---|---|
| `[fonts] heading` | Rockwell, then other slab and serif fonts |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.125rem` to `3.375rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.1875rem` to `1.3125rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `400` |
| `[shape] radius` | `0.25rem` |
| `[shape] measure` | `37.5rem` |
| `[space] section` | `3.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the site's name on the front page, when the home page has no intro. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `4` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "facebook", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |

## Design

The demo is The Varrow List, a made-up guide to Varrow, a made-up harbour city. Every place, street and person in it is made up, and none of its photographs shows a real business. Its sample brand is a brick red (`#a8361b`) on white with a warm stone surface, Young Serif for headings and place names, and Karla for the text and the listing rows. Both fonts are under the SIL Open Font License.
