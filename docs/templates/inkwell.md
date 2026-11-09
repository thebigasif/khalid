---
title: "Inkwell"
order: 87
---

# Inkwell

Inkwell is a template for the archive of a newsletter that has run for years. Every issue has a number. The front page puts the sign-up beside the newsletter's name, shows the latest issue with its number set large, then lists every issue grouped by year, each with its parts at a glance.

```toml
[template]
name = "inkwell"
```

## Who it suits

- A writer or a small group who has sent a newsletter for years and keeps every issue on the web.
- A society, club or library whose letter goes out a few times a year.
- A weekly or monthly letter with regular sections, such as a lead story, a reading list and letters from readers.

It suits a news site or a blog with long guides less well. For those, Wire or Almanac read better. For a newsletter about markets, with figures and charts, use Ticker.

## Issues, numbers and parts

Inkwell needs nothing new in your posts. Each issue is a post, and its facts are read by their place, not their label:

```yaml
---
title: "Runner bean seed in a wet summer: what to save"
category: issues
facts:
  - label: Issue
    value: "91"
  - label: Growing
    value: "Drying beans indoors in a wet spell"
  - label: From the tin
    value: "Scarlet Emperor, our first bean"
  - label: Dates
    value: "Autumn swap, 10 October"
---
```

- **The first fact is the issue's number,** such as `91`. Write the number only: Inkwell adds "No." and "Issue" itself. A first fact longer than eight characters is not read as a number.
- **The second to fourth facts are the issue's parts.** The label is the section, such as Growing or Letters, and the value is what it holds.

A post without a number, such as a supplement or a note between issues, is an extra. It shows in the archive without a number, with its category over its title and its description under it. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

## What the front page shows

1. **The masthead:** the `tagline` as a small label, your newsletter's name large, and your intro. Beside it, the sign-up form with your `signup_note`. On a site that takes no sign-ups, such as a static site, the same box shows your call to action as a button and a link to your feed.
2. **The latest issue:** the newest post with a number. Its number is set very large, beside the date, the title, the description, "In this issue" with its parts, a "Read this issue" button and its lead picture.
3. **Every issue:** links to each year on the page, then the issues grouped by year. Each year opens like the spine of a bound volume: the year in large figures over a heavy rule, with how many numbered issues it holds. Each issue is a row with its number, its title and date, and its parts side by side.
4. The page links, your call to action as a band, and the newsletter form on later pages.

The rows have no pictures, so the front page loads one photograph, the latest issue's.

## What an issue shows

- A stamp with the issue's number, then the date and the category.
- The title, the description and the author.
- **In this issue:** the issue's parts in a box ruled down its start edge.
- The lead picture, then the text at a reading width.
- The tags, the share links and the author box, then the sign-up form, then more issues as rows like the archive's.

## Other pages

- **Lists** (a category, a tag, the archive): the list's name and intro, then its issues grouped by year.
- **People** (`/authors/`): the editor and writers in a grid with their mark or photo and role.
- **A person's page:** their photo or mark, role, experience and bio, then the issues they wrote, grouped by year.

## Your brand

Inkwell brings its own colours: a blue-black ink and an ink-blue accent (`#1d4c9c`) on a pale paper, with heavy ink rules under the masthead, over each year and over the footer. Your colours, fonts and logo in `theme/tokens.toml` replace them. A book face with old-style figures suits the heading font best, since the issue numbers and the years are set in it.

Inkwell's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Inkwell's default |
|---|---|
| `[fonts] heading` | Iowan Old Style, then other book faces and Georgia |
| `[fonts] body` | the system's sans |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.25rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.25rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.65` |
| `[type] heading_weight` | `600` |
| `[shape] radius` | `0px`, square |
| `[shape] measure` | `36rem`, about 67 characters a line |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One short line over your newsletter's name, such as how often it goes out and since when. |
| `signup_note` | none | One line in the sign-up form, such as how many letters a reader gets. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
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
| `newsletter` | `"both"` | Where the sign-up form shows: `none`, `post`, `home` or `both`. On the front page it sits in the masthead. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
