---
title: "Atelier"
order: 86
---

# Atelier

Atelier is a template for a fashion label, a stylist or a fashion writer. Its front page is a lookbook: each outfit is a numbered look with a tall picture and the pieces worn beside or under it. It reads like a printed lookbook, not a shop.

```toml
[template]
name = "atelier"
```

## Who it suits

- A small label that shows each season's collection one look at a time.
- A stylist who posts outfits and names every piece.
- A fashion writer who mixes outfit posts with short notes on cloth, care and making.

It suits a news site or a company blog less well. For those, Wire or Keystone read better.

## Looks are posts with facts

Atelier needs nothing new in your posts. A post with `facts` and a picture is a look. A post without facts is a note, and it shows in the journal. Each fact is one piece of the outfit: the label says what it is, and the value names it.

```yaml
---
title: "The field coat in waxed cotton"
description: "Look 01 of our Autumn Winter 2026 collection: the field coat over a cream knit."
category: autumn-winter-2026
featured: true
facts:
  - label: Coat
    value: "Waxed cotton field coat, slate"
  - label: Knit
    value: "Lambswool crew, cream"
  - label: Hat
    value: "Wool felt fedora, sand"
  - label: Bag
    value: "Soft leather pouch, ink"
image:
  src: lead.jpg
  alt: A woman in a slate field coat and a sand felt hat against a pale wall
  caption: "Look 01. The field coat, worn open over the cream crew."
---
```

A look shows up to four pieces. See [Facts on a post](../templates.md#facts-on-a-post) for the limits. A tall picture, about 4 by 5, suits a look best: the lookbook grid crops every picture to that shape from the middle, and the post shows it whole.

Categories are your collections, such as "Autumn Winter 2026". Give each one an intro in `categories.toml`, and it shows at the top of that collection's page. Tags suit colours and cloths. `featured: true` picks the look that opens the front page.

## What the front page shows

1. Your label's name, large and centred, and the `tagline` under it.
2. **The lookbook.** The featured look, or else the newest, as a spread: its picture on one side, and on the other "Look 01", its title, its description and its pieces as a ledger, each label joined to its piece by a dotted line.
3. The other looks in a grid of tall pictures, numbered "Look 02" and on, each with its title and pieces under the picture.
4. **Collections:** every category that holds a look. **Journal** beside it on wide screens: the newest notes with their date and category. `journal_count` sets how many.
5. Numbered page links, the call to action band, such as "Book a fitting", and the newsletter form.

No caption or number is ever written on a picture. They always sit beside it or under it.

## What a post shows

1. The trail from the home page to the post's collection.
2. **The spread.** The lead picture on one side at its own shape, with its caption under it. On the facing side: the collection, the title, the description, the pieces and the author with the date. On a phone the picture comes first.
3. The post in one column of about 66 characters, with a folded contents list for long posts.
4. The tags, the share links and the author box.
5. **More looks:** related posts as a small picture grid.

A note has the same spread without the pieces.

## Other pages

- **Lists** (a collection, a tag, the archive): the list's name and intro, centred, then its looks as a numbered picture grid and its notes as a dated feed.
- **People** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A person's page:** their photo or mark, role, experience and bio, then their looks and notes, and the rest of the studio.

## Your brand

Atelier brings its own colours: ink on a warm white page, with a rust colour for links. Labels are spaced capitals, and titles use the heading font. A high-contrast serif for headings suits it best, such as Bodoni Moda or Playfair Display, with a calm sans for the text. Your colours, fonts and logo in `theme/tokens.toml` replace Atelier's.

The demo's sample brand, Studio Ilvane, is a made-up label: ecru paper, a terracotta accent, Bodoni Moda for titles and Jost for the text.

Atelier's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Atelier's default |
|---|---|
| `[fonts] heading` | Didot, then Bodoni and other serifs |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.5rem` |
| `[type] h2` | `1.5rem` to `2rem` |
| `[type] h3` | `1.25rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.7` |
| `[type] heading_weight` | `500` |
| `[shape] radius` | `0px`, square like a printed page |
| `[shape] measure` | `38rem` |
| `[space] section` | `5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the label's name on the front page. |
| `journal_count` | `4` | How many notes show in the journal on the front page, from `0` to `10`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `4` | How many related looks show, from `3` to `10`. |
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
