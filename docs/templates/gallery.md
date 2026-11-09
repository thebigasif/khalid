---
title: "Gallery"
order: 45
---

# Gallery

Gallery is a template for a studio or an illustrator. The front page is a wall of pictures: each work hangs on a pale mat at its own shape, with a numbered label under it. Each post is a project page. Nothing on the site needs a script.

```toml
[template]
name = "gallery"
```

## Who it suits

- An illustrator, printmaker or designer who shows finished work with a short note on each piece.
- A small studio that shows client projects, such as posters, book covers, prints and patterns.
- A maker whose work is flat or framed, such as textiles, paper cuts or tiles shot from above.

It is not a single-picture story site: for one photograph at a time with camera details, Aperture does that. It never crops a picture into an even grid, as Atelier's lookbook does.

## Project details are facts

Gallery needs nothing new in your posts. Give each post a lead picture and up to four `facts`. Gallery reads the facts by their place, not their label:

```yaml
---
title: "Haeckel's jellyfish, restored for six prints"
category: natural-history
image:
  src: lead.jpg
  alt: A tall plate of jellyfish with long red and pink tentacles on a pale ground
facts:
  - label: Client
    value: "Harbour Science Museum"
  - label: Year
    value: "2026"
  - label: Medium
    value: "Giclée on cotton rag"
  - label: Edition
    value: "6 prints, 50 of each"
---
```

1. **The client** or publisher. The list of works shows it.
2. **The year** the work was made. The wall label and the list of works show it. Without it, they show the year of the post.
3. **The medium,** such as ink and watercolour, or risograph in three colours. The wall label shows it. Without it, the label shows the category.
4. **The size or edition,** or any other short fact.

Your labels show on the project page, so write "Publisher" or "Size" when that fits better. A post without facts shows no project sheet. See [Facts on a post](../templates.md#facts-on-a-post) for the limits. Categories are your disciplines, such as illustration, print or pattern, and tags are subjects.

More pictures of a project go in the post's text as figures. Each shows on a mat in the reading column.

## What the front page shows

1. Your studio's name large, with your statement beside it: the home page's intro, or the `statement` option. Up to four numbers from `hero_stats` follow when you write them.
2. **Discipline chips:** "All work", then each category, as plain links.
3. **The wall:** every post on the page in columns, two on a phone, three on a tablet and four on a wide screen. Each picture sits on a mat at its own shape, never cropped. Under it, never on it, the wall label: the number, the title, then the medium and year. The whole work is one link.
4. **List of works:** the same works under the same numbers, each with its title, client and year. The `checklist` option turns it off.
5. Numbered page links, your call to action as a band, and the newsletter form.

When `[[reader.home_sections]]` lists sections, each section is a wall of its own.

## What a project page shows

1. The category, the title and the summary on one side, and the project sheet on the other: the four facts in ruled rows.
2. **The plate:** the lead picture on its mat, as wide as the frame allows and never taller than most of the screen, with its caption under it.
3. In one reading column: the author and date, a folded contents list for a long post, and the text with the project's other pictures.
4. The tags, the share links and the author box.
5. **More work:** the related posts as a small wall.

## Other pages

- **Lists** (a category, a tag, the archive): the list's name and intro over a rule, the discipline chips with this one marked, then the wall.
- **People** (`/authors/`): everyone in the studio, with their mark or photo, role and how many posts they wrote.
- **A person's page:** their photo or mark, role, experience and bio, then their work as a wall.

## Your brand

Gallery brings its own look: a plaster wall, pale mats, near-black ink and one vermilion accent. Titles and the studio's name are set in a wide sans, and the text and labels in a plain sans. Your colours, fonts and logo in `theme/tokens.toml` replace them. A wide grotesque for the headings suits it; the `surface` colour is the colour of the mats.

Gallery's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Gallery's default |
|---|---|
| Colours | background `#efebe4`, surface `#fbfaf7`, text `#1b1a17`, muted `#575249`, link and accent `#a3301f`, border `#d5cfc4` |
| `[fonts] heading` | Avenir Next, then Helvetica Neue, Arial and the system's sans |
| `[fonts] body` | Helvetica Neue, then Arial and the system's sans |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.5rem` |
| `[type] h2` | `1.5rem` to `2rem` |
| `[type] h3` | `1.1875rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0px`, square |
| `[space] section` | `5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `statement` | none | One or two lines beside your studio's name on the front page, when the home page has no intro. |
| `checklist` | `true` | The front page closes with the list of works under the wall's numbers. |
| `hero_stats` | none | Up to four numbers under your name, each written `number \| what it counts`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `4` | How many related posts More work shows under a post, from `3` to `10`. |
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

## Pictures

Pictures are never cropped and never covered by text. The wall draws each one about a quarter of the frame wide on a wide screen and half the screen on a phone. The project plate draws the lead picture up to the frame's width, about 84rem, so give each lead picture at least 1600 pixels across. A tall picture keeps its shape and stands in the middle of its mat.
