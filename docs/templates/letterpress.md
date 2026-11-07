---
title: "Letterpress"
order: 88
---

# Letterpress

Letterpress is a template for a newsletter that treats each issue as a printed edition. The front page is the cover of your newest issue. Older issues wait under it as a pile of small printed covers. Each issue opens with a folio line, like the top of a printed page, and closes with an end mark.

```toml
[template]
name = "letterpress"
```

## Who it suits

- A writer or a small team that sends one letter a week or a month and keeps every issue on the web.
- A newsletter with a strong voice, where each issue is one piece with a few parts.
- A publication that wants its archive to feel collected, like back issues on a shelf.

It suits a news site or a markets letter less well. For those, Wire or Ticker read better.

## Issues are posts

Every post is an issue. Letterpress needs nothing new in your posts. It reads an issue's facts by their place, not their label:

```yaml
---
title: "Fifty issues, set by hand"
facts:
  - label: Issue
    value: "No. 50"
  - label: Workshop visit
    value: "A print shop in a railway arch"
  - label: Tools
    value: "The composing stick"
  - label: Letters
    value: "Your first press"
---
```

1. **The issue number,** written as you want it shown, such as `No. 50`. It shows in the folio line, on the issue's small cover and in the end mark.
2. to 4. **In this issue:** up to three parts of the issue. The label is the part's name and the value its headline. They show as the issue's contents, with dotted leaders, on the cover and at the top of the issue.

An issue without facts still works. It shows its date where the number would be, and no contents. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

## What the front page shows

1. **The nameplate:** your newsletter's name very large and centred, the `tagline` under it in italics, then the folio line: the newest issue's number, its date and its reading time between a thick and a thin rule.
2. **The cover:** your newest issue, its lead picture in a plate beside "Latest issue", the title, the description, the byline, "In this issue" and a "Read this issue" button. Without a lead picture, the text takes the whole width.
3. The newsletter form and your call to action as a band.
4. **Past issues:** every other issue as a small printed cover, three to a row on a wide screen: the number and date over a double rule, the title, the description and the category. These covers carry no pictures, so the page loads fast.

The second and later pages show the past issues only.

## What an issue shows

- The folio line with the issue number, the date and the reading time.
- The category, the title, the description in italics and the byline, centred.
- The lead picture in a plate.
- "In this issue", from the second to the fourth fact.
- The text at a reading width, with a drop cap and the first line in small capitals.
- The end mark: an ornament and "End of the issue" with its number.
- The tags, the share links, the author box, the newsletter form and more issues as small printed covers.

## Other pages

- **Lists** (a category, a tag, the archive): the list's name and intro between a thick and a thin rule, then its issues as small printed covers.
- **People** (`/authors/`): everyone in a grid with their mark or photo and role.
- **A person's page:** their photo or mark, role, experience and bio, then their issues as small printed covers.

## Your brand

Letterpress brings its own colours: a printer's blue (`#1f3f8a`) and a near-black ink on an uncoated paper. Your colours, fonts and logo in `theme/tokens.toml` replace them. A Modern or Didone face for headings suits it best, since the nameplate, the titles and the issue numbers are set in the heading font, with a book face for the text.

Letterpress's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Letterpress's default |
|---|---|
| `[fonts] heading` | Didot, then Bodoni and Times New Roman |
| `[fonts] body` | Baskerville, then Book Antiqua and Georgia |
| `[type] body` | `1.1875rem` on a phone to `1.25rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.5rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.25rem` to `1.375rem` |
| `[type] small` | `0.9375rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0px`, square |
| `[shape] measure` | `38rem`, about 68 characters a line |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the newsletter's name on the front page. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | An issue shows its description under its title. |
| `updated_note` | `false` | An updated issue says why it was updated, from its `updated_note`. |
| `cards` | `"text"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `4` | How many more issues show under an issue, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long issue's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where an issue shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |

## Design

The demo is The Proof Sheet, a made-up weekly letter about things made by hand. Its sample brand is a letterpress red (`#a2301c`) on a cream paper, Old Standard TT 700 for headings and Libre Caslon Text for the text, both SIL OFL from the Google Fonts repository. The design brief is in `docs/design/templates/letterpress.md`.
