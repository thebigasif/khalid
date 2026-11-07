---
title: "Glossy"
order: 86
---

# Glossy

Glossy is a template for a lifestyle magazine told in pictures: home, food, travel, style and wellbeing. Its front page opens like the cover of a printed magazine, then turns into a grid of picture cards in several shapes. It reads like a weekend supplement, not a news site.

```toml
[template]
name = "glossy"
```

## Who it suits

- A small lifestyle magazine with a few writers and a mix of subjects.
- A blogger who covers home, food and travel and has a good photograph for every post.
- A shop or brand that runs a magazine beside its products.

It suits a text-led blog or a news site less well. For those, Ledger or Wire read better.

## Stories are posts with a picture

Glossy needs nothing new in your posts. Every post is a story, and its lead picture leads its card. A post without a picture still works: its card shows the text on the surface colour. Facts are not used.

```yaml
---
title: "Cold lake swims in the mountains"
description: "Lake swimming in the mountains for beginners: how to find a safe lake and warm up after."
category: travel
featured: true
image:
  src: lead.jpg
  alt: A still blue mountain lake between tall pine trees
  caption: "A mountain lake in late October."
---
```

Categories are your sections, such as Home, Food and Travel. Give each one an intro in `categories.toml`, and it shows at the top of that section's page. `featured: true` picks the cover story.

The cards crop each picture from the middle to their own shape: wide, tall or square. Keep the subject in the middle third of the photograph, and every crop keeps it.

## What the front page shows

1. **The masthead line.** Your magazine's name large at the start and the `tagline` at the end, over a hairline.
2. **The cover.** The featured story with a picture, or else the newest one: its picture large on the start side, and beside it on a soft panel a Cover story label, its section, the title, the description and the byline.
3. **Cover lines.** Under the cover story, the next stories as a short numbered list, as on a printed cover. `cover_lines` sets how many.
4. **Latest stories.** The stories after the cover lines in a grid of cards with a repeating rhythm of six: one wide card, one tall card, three square cards, then a band card with its picture beside its text across the page. On a phone the three square cards become rows with a small square picture.
5. **Browse the sections:** every section as a large rounded link.
6. Numbered page links, the call to action band and the newsletter form.

Each story shows once: the grid starts after the cover story and the stories in its cover lines. With `cover_lines = 0`, the grid starts with the story after the cover.

Text is never written on a picture. It always sits beside it or under it.

## What a post shows

1. The trail from the home page to the post's section.
2. The section as a rounded label, the title large and centred, the description, and the author with the date and reading time.
3. The lead picture across the page, with rounded corners and its caption under it.
4. The post in one column of about 66 characters, with a folded contents list for long posts.
5. The tags as rounded labels, the share links and the author box.
6. **More stories:** related posts as square cards.

## Other pages

- **Lists** (a section, a tag, the archive): the list's name and intro, centred, then its stories in the same rhythm of cards.
- **People** (`/authors/`): every writer in a grid with their mark or photo, role and how many posts they wrote.
- **A person's page:** their photo or mark, role, experience and bio, then their stories and the other writers.

## Your brand

Glossy brings its own colours: ink on a warm cream page, a blush surface and a raspberry colour for links and buttons. Pictures, panels and labels have soft round corners. A high-contrast serif for headings suits it best, such as Playfair Display, with a friendly sans for the text. Your colours, fonts and logo in `theme/tokens.toml` replace Glossy's.

The demo's sample brand, The Weekend Edit, is a made-up magazine: cream paper, a peach surface, a deep teal accent, Playfair Display for titles and Plus Jakarta Sans for the text.

Glossy's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Glossy's default |
|---|---|
| `[fonts] heading` | Playfair Display, then Georgia and other serifs |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.75rem` |
| `[type] h2` | `1.625rem` to `2.25rem` |
| `[type] h3` | `1.25rem` to `1.5rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.7` |
| `[type] heading_weight` | `600` |
| `[shape] radius` | `1rem`, soft round corners |
| `[shape] measure` | `36rem` |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line beside the magazine's name on the front page. |
| `cover_lines` | `3` | How many of the next stories the cover lists, from `0` to `4`. The card grid starts after them. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `true` | Each card shows how long the post takes to read. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many related stories show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"numbered"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["facebook", "whatsapp", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
