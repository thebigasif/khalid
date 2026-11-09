---
title: "Notebook"
order: 82
---

# Notebook

Notebook is a template for one person's public notebook: short notes, links with a line or two about them, and things noticed along the way. Every page is a stream, newest first, like the pages of a notebook kept every few days.

```toml
[template]
name = "notebook"
```

## Who it suits

- One person who writes often and briefly: a gardener, a developer, a researcher, a designer or a reader.
- Someone who shares links with a comment as well as notes of their own.
- A notebook with a guest author now and then.

It suits a magazine, a news site or a team blog less well: it has no lead story and no grid of cards.

## What you write

Notebook adds nothing to your front matter. A note is a post, and its `description` is what the stream shows under the title, so a short note reads in full without a click. Your categories are the notebook's sections, such as Garden or Reading, and your tags are the small subjects on each note.

### Link notes

A note whose first fact is a web address is a link note. The fact's label names the place, and its value is the address:

```yaml
facts:
  - label: Northern Growers
    value: "https://www.example.com/ng/leaf-mould"
```

The stream shows "Read it at Northern Growers" under the note, and the post ends with the same link as a button. A fact's value holds at most 40 characters, so use a short link for a long address. An address on your own site, starting with `/`, works too. Any other facts show as a small list under the note's title.

## What the front page shows

1. Your site's name, with your `tagline` beside it.
2. **The stream:** every note, newest first, under a month heading such as "November 2026". Each note has its date in the margin, the day large and the month small, beside a double margin rule in your accent colour. Then its section, its title, its description, the "Read it at" link of a link note and a small picture when it has one.
3. **The side column** on a wide screen, under the stream on a phone: "About this notebook" with the front page's intro or your tagline, your sections, and the newsletter form when your site takes sign-ups.
4. Numbered page links when there are more notes.

When `[[reader.home_sections]]` lists sections, they take the place of the stream, each a stream of its own.

## What a note shows

1. The date in the margin with its year, beside the same margin rule.
2. The section, the title, the description as a summary, the author and the date, and any facts that are not a link.
3. The lead picture, then the note in one column of about 64 characters a line.
4. A link note's "Read it at" button, the tags as `#tag` links and the share links.
5. The author box, then **More notes:** related notes as a dated list.

## Other pages

Lists (a section, a tag, the archive) use the same stream, with the margin and the month headings, under the list's name and intro. The people and author pages are Khalid's shared pages.

Everything reads with scripts off.

## Your brand

Notebook brings its own colours: a cream page like notebook paper, brown-black ink and a red margin rule (`#9e2f1d`) like an exercise book. Your colours, fonts and logo in `theme/tokens.toml` replace them. The dates, labels and tags are set in your monospace font, so a typewriter face suits it well.

Notebook's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Notebook's default |
|---|---|
| `[fonts] heading` | Palatino, then Book Antiqua and Georgia |
| `[fonts] body` | Palatino, then Book Antiqua and Georgia |
| `[fonts] mono` | The system's monospace |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `1.875rem` to `2.625rem` |
| `[type] h2` | `1.375rem` to `1.625rem` |
| `[type] h3` | `1.1875rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `2px` |
| `[shape] measure` | `32rem`, the column a note is read in |
| `[space] section` | `4rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line beside your site's name on the front page. It also fills "About this notebook" when the front page has no intro. |
| `logo` | `false` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `dek` | `true` | A note shows its description under its title. |
| `updated_note` | `false` | An updated note says why it was updated, from its `updated_note`. |
| `cards` | `"text"` | The shape of the posts on author pages: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `5` | How many related notes show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long note's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a note shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "hacker-news", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"none"` | Where a band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `hero_stats` | none | Up to four numbers under your name, each written `number \| what it counts`. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
