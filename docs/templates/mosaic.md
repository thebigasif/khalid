---
title: "Mosaic"
order: 13
---

# Mosaic

Mosaic is a template for a photo blog. The front page lays your posts out as tiles in a rhythm of sizes: a large picture beside a tall one, a row of three, two halves. Each picture is cut to its tile, and its place, date and title sit under it, never on it.

```toml
[template]
name = "mosaic"
```

## Who it suits

- A photo blog that posts often, where each post is one or two pictures with short notes.
- A walking, travel or nature photographer who wants the front page to feel like a wall of recent work.
- A small group of photographers who share one site.

It shows many pictures at once. For one picture at a time, the full width of the page, Aperture does.

## Notes are facts

Mosaic needs nothing new in your posts. Give each post a lead picture, a category for its place or kind, and up to four `facts`:

```yaml
---
title: "First snow on the rim"
category: canyons
image:
  src: lead.jpg
  alt: Junipers on the edge of the Grand Canyon covered in fresh snow
facts:
  - label: Park
    value: "Grand Canyon"
  - label: Where
    value: "South Rim, near Mather Point"
  - label: Light
    value: "Cloud, clearing"
  - label: Taken
    value: "December"
---
```

The facts show beside the post's title as small cells, each label over its value. Mosaic does not read them by position, so write whatever short notes fit your pictures, such as the camera, the film or the weather. A post without facts shows none. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

## What the front page shows

1. Your site's name and the `tagline` on one line, and up to four numbers from `hero_stats` when you write them.
2. **The mosaic:** your posts as tiles. On a wide screen the tiles repeat a pattern of nine: a large tile beside a tall one, three equal tiles, two halves, then a tall tile beside a large one. A last row that would leave a gap is closed: one tile alone runs the full width, and two share it in halves. On a tablet the pattern is a full tile, two halves and three thirds; on a phone, a full tile and then two halves.
3. Numbered page links, your call to action as a band, and the newsletter form.

Each tile shows a small square in your accent colour, the category and date, and the title. The larger tiles also show the summary, unless you turn `tile_summaries` off. The whole tile is one link.

When `[[reader.home_sections]]` lists sections, each section is a mosaic of its own.

## What a post shows

1. The picture first, the full width of the frame and never cropped. A tall picture keeps its shape, at most nearly the height of the screen. Its caption, such as a credit, sits under it.
2. **The title row,** under the picture: the category, the title and the summary on one side, and the author, the date and the facts on the other.
3. The story in one column, with a folded contents list for a long post.
4. The tags, the share links and the author box.
5. **More pictures:** the related posts as tiles in the same rhythm.

## Other pages

- **Lists** (a category, a tag, the archive): the list's name and intro, then its posts as the same mosaic.
- **People** (`/authors/`): everyone who writes for the site, with their mark or photo, role and how many posts they wrote.
- **A person's page:** their photo or mark, role, experience and bio, then their posts as tiles.

## Your brand

Mosaic brings its own look: a pale stone ground like the grout between tiles, near-black ink and one deep teal accent, with a plain grotesque for titles and words. Your colours, fonts and logo in `theme/tokens.toml` replace them. The accent colours the small squares, so pick one that reads well beside your pictures.

Mosaic's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Mosaic's default |
|---|---|
| Colours | background `#f6f4ef`, surface `#e9e6dd`, text `#1b1a17`, muted `#5a574f`, link and accent `#0b5d61`, border `#d8d4c9` |
| `[fonts] heading` and `body` | Helvetica Neue, then Arial and other plain grotesques |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.125rem` to `3.5rem` |
| `[type] h2` | `1.5rem` to `2rem` |
| `[type] h3` | `1.1875rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `600` |
| `[shape] radius` | `3px` |
| `[shape] measure` | `36rem` |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line beside your site's name on the front page. |
| `tile_summaries` | `true` | The larger tiles show the post's summary under the title. |
| `photo_facts` | `true` | A post shows its facts beside the title. |
| `hero_stats` | none | Up to four numbers under your name, each written `number \| what it counts`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `5` | How many related posts show as tiles under a post, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "whatsapp", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |

## Pictures

Every tile crops its picture to the tile's shape from the middle, so keep the subject in the middle third. The large and tall tiles share one row height, so a landscape on a tall tile shows its middle. Give each lead picture at least 1600 pixels across: the post shows it the full width of the frame, about 81rem.

Pictures are never covered by text. The category, date, title and summary always sit under the picture.
