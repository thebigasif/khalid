---
title: "Aperture"
order: 12
---

# Aperture

Aperture is a template for one photographer. Each post is one picture and the story behind it. The picture fills the width of the page, and under it a plate gives the camera, the lens and the exposure.

```toml
[template]
name = "aperture"
```

## Who it suits

- A photographer who publishes one picture at a time, with a few paragraphs on where and how it was made.
- A photo journal or a walking blog where every post leads with a single strong picture.
- A camera club or a teacher who wants each picture to show its settings, so readers can learn from them.

It is not a portfolio grid. For a blog where the words come first, Ledger does.

## Camera details are facts

Aperture needs nothing new in your posts. Give each post a lead picture and up to four `facts`. Aperture reads the facts by their place, not their label:

```yaml
---
title: "The lakes below Saana"
image:
  src: lead.jpg
  alt: A wide view from a fell top over a dark blue lake in a bowl of tundra
  caption: "Photograph by Simo Räsänen (Ximonic), CC BY-SA 4.0"
facts:
  - label: Camera
    value: "Canon EOS 5D Mark IV"
  - label: Lens
    value: "EF 24-70mm f/4L at 24mm"
  - label: Exposure
    value: "f/8 · 1/320 s · ISO 100"
  - label: Taken
    value: "4 August 2026"
---
```

1. **The camera.**
2. **The lens,** and the focal length when it is a zoom.
3. **The exposure:** aperture, shutter speed and ISO.
4. **The date the picture was taken,** which is often not the date of the post. Or any other short fact, such as the place or the film.

Your labels show on the page, so write "Film" or "Phone" when that fits better. A post without facts shows no plate. See [Facts on a post](../templates.md#facts-on-a-post) for the limits. Categories are your places or series, and tags are subjects, such as winter or wildlife.

## What the front page shows

1. Your name and the `tagline` on one line, over a hairline, and up to four numbers from `hero_stats` when you write them.
2. **Stories:** the newest posts, one under the other. Each is its picture the full width of the frame, then a caption row: the place and date, the title and the summary on one side, and the plate with the camera details on the other. `stories` sets how many, five by default.
3. **Contact sheet:** the other posts on the page as small pictures in a light strip that scrolls sideways, each with its title, place and date.
4. Numbered page links, your call to action as a band, and the newsletter form.

When `[[reader.home_sections]]` lists sections, each section is a contact sheet of its own in place of the stories.

## What a post shows

1. The picture first, the full width of the frame, with its caption, such as the credit, under it on the end side. A tall picture keeps its shape and sits in the middle.
2. **The plate:** the four facts in a ruled row, in a monospace, like the edge of a strip of film. On a phone it becomes two rows of two.
3. In one calm column about 36rem wide: the place, the title, the summary in italics, the author and date.
4. The story, with a folded contents list for a long post.
5. The tags, the share links and the author box.
6. **More pictures:** the related posts as a contact sheet.

## Other pages

- **Lists** (a place, a tag, the archive): the list's name and intro, then one picture a row, large on the start side, with its place, date, title, summary and plate beside it. Never a grid.
- **People** (`/authors/`): everyone who writes for the site, with their mark or photo, role and how many posts they wrote.
- **A person's page:** their photo or mark, role, experience and bio, then their stories as rows.

## Your brand

Aperture brings its own look: a warm white like the border of a print, near-black ink and one rust accent like a darkroom's safe light. Words are set in a book serif, and the camera details, the menu and the small labels in a monospace. Your colours, fonts and logo in `theme/tokens.toml` replace them. A calm serif or a plain sans suits it; keep `[fonts] mono` set, since the plate uses it.

Aperture's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Aperture's default |
|---|---|
| Colours | background `#faf9f6`, surface `#f0eee8`, text `#1c1b19`, muted `#5c5850`, link and accent `#9a3412`, border `#dedad0` |
| `[fonts] heading` and `body` | Iowan Old Style, then Charter, Georgia and other serifs |
| `[fonts] mono` | The system's own monospace |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2rem` to `3.25rem` |
| `[type] h2` | `1.5rem` to `2rem` |
| `[type] h3` | `1.1875rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `500` |
| `[shape] radius` | `0px`, square |
| `[shape] measure` | `36rem` |
| `[space] section` | `5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line beside your name on the front page. |
| `stories` | `5` | How many of the newest posts show as full-width stories on the front page, from `1` to `12`. The rest of the page goes into the contact sheet. |
| `camera_plate` | `true` | Each picture shows its plate of camera details, read from the post's facts. |
| `hero_stats` | none | Up to four numbers under your name, each written `number \| what it counts`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `6` | How many related posts the contact sheet under a post shows, from `3` to `10`. |
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

The stories and the post's picture are drawn as wide as the frame, about 81rem, so give each lead picture at least 1600 pixels across. A tall picture is never cropped: it keeps its shape, at most nearly the height of the screen. The contact sheet crops each picture to 3:2.

Pictures are never covered by text. The title, summary and plate always sit under or beside the picture.
