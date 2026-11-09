---
title: "Itinerary"
order: 55
---

# Itinerary

Itinerary is a template for a travel diary. It files your trips by country, shows each trip as a ticket with its days, route, season and cost, and lays out every trip day by day. Notes such as a packing list sit under the trips.

```toml
[template]
name = "itinerary"
```

## Who it suits

- A couple, a family or one traveller who writes up each trip as a day-by-day plan others can copy.
- A slow travel or walking blog where the route, the days and the cost matter as much as the photos.
- A diary with a few notes beside the trips, such as packing lists and planning tips.

It suits a city magazine or a guide to one place less well. For a magazine with many topics, Atlas reads better. For step-by-step guides, Compass does.

## Trips are posts with facts

Itinerary needs nothing new in your posts. A post with `facts` is a trip. A post without facts is a note. Put the facts in this order, since Itinerary reads them by their place, not their label:

```yaml
---
title: "Georgia itinerary: ten days from Tbilisi to the Kazbegi mountains"
category: georgia
facts:
  - label: Days
    value: "10"
  - label: Route
    value: "Tbilisi to Stepantsminda and back"
  - label: When
    value: "May 2026"
  - label: Cost for two
    value: "£1,180"
---
```

1. **How long the trip was,** as a number. It shows large on the ticket's stub, with its label under it.
2. **The route.**
3. **When you went.**
4. **What it cost,** or any other short fact.

Your labels show on the page, so write "Nights" or "Budget" when that fits better. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

Categories are your countries. Give each one an intro in `categories.toml`, and it shows at the top of that country's page. Keep notes in a category of their own, such as "Packing and planning".

Write a trip's main headings as its days, such as `## Days 1 and 2: Tbilisi old town`. Each one becomes a stop on the route down the side of the post, and the contents list them under "Day by day".

A task list in a post shows as a checklist, which suits a packing list:

```markdown
- [ ] Passport
- [x] Rain jacket
```

## What the front page shows

1. Your diary's name, large, and the `tagline` under it.
2. A row of up to four large numbers from `hero_stats`, such as countries, trips and days on the road. Nothing shows until you write them.
3. **Latest trip:** your newest trip as a wide ticket: its picture, country, title, route, season and cost, and a stub with the number of days behind a perforated line.
4. **Countries:** each country with trips on the page as a passport stamp, a little askew, with how many trips it holds. A category whose posts on the page are all notes gets no stamp.
5. **Trip diaries:** the other trips as tickets, three across on a laptop and one on a phone.
6. **Notes from the road:** the newest posts without facts as a dated list. `notes_count` sets how many.
7. Numbered page links, your call to action as a band and the newsletter form.

A site with no trips at all shows every post as a ticket, and each category as a stamp.

## What a post shows

1. The trail from the home page to the post's country.
2. The country as a small stamp, the title, the description under it, the author with their mark and role, who reviewed it, and the date.
3. **The boarding pass:** the route, season and cost in a row of cells, and the days large on a stub behind a perforated line.
4. The lead picture, then the post in one column, about 70 characters a line.
5. **Day by day:** on wide screens the contents stand beside the post and stay in view. On a phone they fold at the top.
6. The days on a dashed route: each main heading has a stop on a line down the start side of the text.
7. The tags, the share links and the author box.
8. Related posts as small tickets.

A note has no boarding pass and no route.

## Other pages

- **Lists** (a country, a tag, the archive): the list's name on a stamp, its intro, then the posts as tickets.
- **Writers** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A writer's page:** their photo or mark, role and bio, then their trips as tickets, and the other writers.

## Your brand

Itinerary brings its own colours: a rust accent (`#b8461a`), like the ink of a passport stamp, on white, with a kraft-paper surface for the tickets and the boarding pass. Your colours, fonts and logo in `theme/tokens.toml` replace them. A slab serif or a sturdy serif for headings suits it best, since the stamps and the day numbers are set in the heading font.

Itinerary's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Itinerary's default |
|---|---|
| `[fonts] heading` | Rockwell, then Roboto Slab, Zilla Slab and Georgia |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.125rem` to `3.25rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.1875rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.65` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0.375rem` |
| `[shape] measure` | `38rem` |
| `[space] section` | `4rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the diary's name on the front page. |
| `hero_stats` | none | Up to four numbers under the diary's name, each written `number \| what it counts`. |
| `notes_count` | `5` | How many notes show under the trips on the front page, from `0` to `10`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `contents` | `"side-end"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. Your day headings carry their own numbers. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["whatsapp", "facebook", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
