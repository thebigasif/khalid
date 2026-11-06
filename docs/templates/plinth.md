---
title: "Plinth"
order: 85
---

# Plinth

Plinth is a template for an architecture or interiors practice. It shows your built work one project at a time, each with its photograph, its plan and its data, and it reads like a practice's printed book of its work, not a blog.

```toml
[template]
name = "plinth"
```

## Who it suits

- An architecture practice that writes up each finished project with photographs and a plan.
- An interior designer, a landscape studio or a builder who shows finished work with its place, size and year.
- A practice that also posts short studio notes, such as a material it tested, a talk or a job opening.

It suits a news site or a blog without pictures less well. For those, Wire or Ledger read better. For a company blog with case studies among other posts, Keystone fits better.

## Projects are posts with facts

Plinth needs nothing new in your posts. A post with `facts` is a project. A post without facts is a studio note.

```yaml
---
title: "Quarry Lane Library: a reading room added to a Victorian library"
description: "How we added a new reading room without hiding the old building."
category: public-buildings
facts:
  - label: Location
    value: "Harrowfield"
  - label: Year
    value: "2024"
  - label: Floor area
    value: "640 m²"
  - label: Stage
    value: "Completed"
image:
  src: lead.jpg
  alt: A tall window set deep in a red brick wall
---
```

Every fact shows in the project's title block with your own label. Cards in lists show the first two facts, so put the place and the year first. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

Categories are your types of work, such as houses, interiors and public buildings. Give each one an intro in `categories.toml`, and it shows at the top of its page. Tags are materials and themes.

## Plans and drawings

Put a plan, section or elevation in the post's folder, such as `plan.svg`, and place it as a figure. The text inside the figure is its caption:

```markdown
:::figure{src="plan.svg" alt="Ground floor plan: the old library on the left, the new reading room on the right"}
Ground floor. The two buildings share one entrance and one desk.
:::
```

Plinth frames every figure in a post as a drawing sheet: a ruled frame on your surface colour, with its number, 01, 02 and on, in the corner. On a wide screen sheets are wider than the text. An SVG drawing stays sharp at every size. Draw it in dark lines on a light background, since it shows as you drew it.

## What the front page shows

1. Your practice's name, large, and the `tagline` under it.
2. A row of up to four large numbers from `hero_stats`, such as buildings completed and years in practice. Nothing shows until you write them.
3. **Projects:** each project as a spread. The photograph sits on one side on a heavy ink bar, the plinth. On the other side are the type of work, the title, the description and the title block: the sheet number, 01, 02 and on, then each fact in a ruled cell. The sides swap from one spread to the next. On a phone each spread stacks, photograph first.
4. **Studio notes:** the newest posts without facts, each with its date and category. `notes_count` sets how many.
5. **Types of work:** every category that holds projects, as a ruled index. A category whose posts on the page are all notes is left out.
6. Numbered page links, and your call to action as a band, such as "Start a project".

## What a post shows

1. The trail from the home page to the post's category.
2. The type of work as a small label, the title, and the post's description as a one-line summary.
3. **The project spread:** the lead photograph on its ink bar, and beside it on a wide screen the title block with the facts, the author and the date. A studio note shows its author and date under the title, and its photograph alone.
4. The post in one column about 37rem wide, with a folded contents list for long posts. Each figure is a numbered drawing sheet.
5. The materials (tags), the share links and the author box.
6. Related posts as a row of numbered sheets.

## Other pages

- **Lists** (a category, a tag, the archive): the list's name and intro, then the posts as a grid of sheets: the photograph, a number, the title and the first two facts. A note shows its date in place of facts.
- **People** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A person's page:** their photo or mark, role, experience and bio, then their projects as sheets, and the rest of the practice.

## Your brand

Plinth brings its own colours: a brick accent (`#9b3d23`) and ink on warm drawing paper, with a deeper paper surface for the drawing sheets. Section labels and the title block's labels are small monospace capitals, as on a drawing. Your colours, fonts and logo in `theme/tokens.toml` replace them. A geometric sans for headings suits it best.

Plinth's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Plinth's default |
|---|---|
| `[fonts] heading` | Futura, then Century Gothic and other geometric sans |
| `[fonts] body` | The system's own sans |
| `[fonts] mono` | The system's own monospace, for labels and sheet numbers |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.5rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.25rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.65` |
| `[type] heading_weight` | `500` |
| `[shape] radius` | `0px`, square |
| `[shape] measure` | `37rem` |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the practice's name on the front page. |
| `hero_stats` | none | Up to four numbers under the practice's name, each written `number \| what it counts`. |
| `notes_count` | `4` | How many studio notes show after the projects on the front page, from `0` to `10`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"numbered"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["linkedin", "bluesky", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"post"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |

## The demo

The demo shows Tessel Rowe Architects, a made-up practice in a made-up harbour town, with eight projects, three studio notes and three people. Its plans are simple SVG drawings made for the demo and kept beside each post. Its photographs come from the WordPress photo directory and Wikimedia Commons under open licences, and its photo credits page lists each one.
