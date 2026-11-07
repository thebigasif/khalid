---
title: "Masthead"
order: 102
---

# Masthead

Masthead is a template for a general publication: a magazine, a review or a small paper that covers several subjects. It is the safe all-rounder. Your site's name sits large and centred over a heavy rule and a thin one, the front page leads with one story, and every post keeps to a calm reading column.

```toml
[template]
name = "masthead"
```

## Who it suits

- A small magazine or review with a few writers and five or six sections, such as culture, cities and food.
- A local or community publication that mixes features, essays and short pieces.
- Any blog that wants to look like a publication without a busy news layout.

It suits a single-subject site with special data less well. For recipes, events or reviews, the templates made for them show more.

## Sections and writers

Masthead needs nothing new in your posts. Your categories are its sections, such as Culture or Food. Give each an intro in `categories.toml`, and it shows at the top of that section's page. Your authors are its writers: give each a `role`, such as Editor, and it shows beside their name on every post.

Put your sections in the main menu in `site.toml`, so they sit between the rules under your name.

## Grid or list

The `front` option sets the shape of every list of posts after the lead story: on the front page, on section, tag and archive pages, on a writer's page and under a post.

```toml
[template.options]
front = "list"
```

- `grid`, the default, puts picture cards three to a row on a wide screen.
- `list` puts one story a row, its section, title and excerpt beside a small picture, with a hairline between stories.

Both become one column on a phone.

## What the front page shows

1. Your site's name, large and centred, with your menu between a heavy and a thin rule under it.
2. Your `tagline`, one line in italics.
3. **The lead story:** the newest post you marked `featured: true`, or your newest post. Its picture, section, title, excerpt, writer and date. In the grid front the text sits beside the picture; in the list front it sits centred above it.
4. **Latest:** the next posts, `latest_count` of them, in the shape `front` sets.
5. **In the sections:** the rest of the page's posts as short lists of headlines, one column per section, each under its section's name.
6. Numbered page links, your call to action as a band, and the newsletter form.

If you list home sections in `site.toml`, each shows under its own rule label in the shape `front` sets, in place of steps 3 to 5.

## What a post shows

1. The trail from the home page to the post's section, centred.
2. The section as a small capital label, the title, and the post's description under it in italics.
3. The writer with their role, and the date.
4. The lead picture, wider than the text, with its caption.
5. The post in one column about 66 characters wide, the first paragraph a little larger, and a small square in your accent colour after the last.
6. The tags, the share links and the author box.
7. Three related stories under a rule label.

## Other pages

- **Lists** (a section, a tag, the archive): the list's name centred under a heavy rule, its intro, then its posts in the shape `front` sets.
- **Writers** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A writer's page:** their photo or mark, name, role and bio, centred, then their stories and the other writers.

## Your brand

Masthead brings its own colours: ink on warm paper with a deep red accent (`#9b1c1c`). Corners are square and nothing has a shadow: rules and type carry the design. Your colours, fonts and logo in `theme/tokens.toml` replace them. A strong serif for headings, which sets your name, and a calm serif for the text suit it best.

Masthead's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Masthead's default |
|---|---|
| `[fonts] heading` | Iowan Old Style, then Palatino and Georgia |
| `[fonts] body` | Charter, then Cambria and Georgia |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.125rem` to `3.5rem` |
| `[type] h2` | `1.5rem` to `2rem` |
| `[type] h3` | `1.25rem` to `1.5rem` |
| `[type] small` | `0.9375rem` |
| `[type] line_height` | `1.65` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0px`, square corners |
| `[shape] measure` | `36rem` |
| `[space] section` | `4rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `front` | `"grid"` | The shape of every list of posts after the lead story: `grid` or `list`. |
| `tagline` | none | One line under the site's name on the front page. |
| `latest_count` | `6` | How many posts show under Latest on the front page, from `3` to `12`. The rest show as headlines by section. |
| `featured` | `true` | The newest post marked `featured` is the lead story. With `false`, the newest post is. |
| `logo` | `false` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["email", "facebook", "whatsapp"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
