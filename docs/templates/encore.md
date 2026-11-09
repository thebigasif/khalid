---
title: "Encore"
order: 85
---

# Encore

Encore is a template for a musician or a band. It puts the music first: your newest release, your tour dates and every record you have made, with your notes from the studio and the road beside them. It looks like a gig poster on a record shelf.

```toml
[template]
name = "encore"
```

## Who it suits

- A band or a solo artist who wants one home for records, shows and news.
- A small label or a composer with a short catalogue.
- An artist who writes studio diaries and tour stories between releases.

It suits a magazine or a company blog less well. For those, Atlas or Keystone read better.

## Releases are posts with facts

Encore needs nothing new in your posts. A post with `facts` is a release. A post without facts is a note, such as a tour diary or news. Put the facts in this order, since Encore reads them by their place, not their label:

```yaml
---
title: "Glass Weather, our second album, is out now"
description: "Eleven songs recorded live in a Glasgow church hall."
facts:
  - label: Format
    value: "Album"
  - label: Released
    value: "2 October 2026"
  - label: Label
    value: "Fieldnote Records"
  - label: Listen on Bandcamp
    value: "https://www.example.com/glass-weather/"
image:
  src: cover.jpg
  alt: Raindrops on a window pane against a grey sky
---
```

1. **The format,** such as Album, EP, Single or Live album. It shows under the cover on every shelf.
2. **The release date.**
3. **The label,** or any other short fact.
4. **Where to listen.** A fact whose value starts with `https://` becomes a button, and its label is the button's text, such as "Listen on Bandcamp" or "Buy the vinyl".

Your labels show on the page, so write them in your own words. See [Facts on a post](../templates.md#facts-on-a-post) for the limits: a value has at most 40 characters, so use a short link. The post's picture is the cover. Encore crops it square, so a picture with its subject in the middle works best. Write the track list in the post as a numbered list.

Categories sort your posts, such as Releases, Studio notes and Tour diary. Band members are authors, and their `role` says what they play.

## Tour dates

The tour list comes from the `tour_dates` option, one show a line, so you add or take out a show in one line:

```toml
[template.options]
tour_dates = [
  "2026-11-14 | Glasgow | The Old Granary | https://tickets.example.com/glasgow",
  "2026-11-18 | Inverness | Riverside Hall | Sold out",
]
```

Each line is `date | city | venue | tickets`. Write the date as year, month and day, such as `2026-11-14`; the list shows the day large with the month beside it. The last part is a ticket link, which becomes a Tickets button, or a short note such as "Sold out" or "Few tickets left". Shows appear in the order you write them, so take a show out once it has passed. Nothing shows until you add a date. The list has the address `#tour` on your front page, so a menu link can go straight to it.

## What the front page shows

1. Your band's name, very large, the `tagline` under it, and the places to listen from `listen_on` as a row of links.
2. **Out now:** your newest release, its cover on the sleeve with the record half out, its facts as liner notes and the Listen button.
3. **On tour:** the tour dates, each with its day, city, venue and Tickets button or note.
4. **Releases:** every other release on the page as a shelf of sleeves, with the title, the format and the year.
5. **Notes:** your newest posts without facts as picture cards with the date and category. `notes_count` sets how many.
6. Numbered page links, your call to action as a band, and the newsletter form.

On the next pages of the front page, the newest release and the tour list step aside, and every release and note on that page shows.

## What a post shows

1. The category as a small label, the title in large capitals and the post's description under it.
2. The author with their mark and role, and the date.
3. **A release** shows its cover on the sleeve, then the liner notes: the facts in ruled rows and a button for each link.
4. **A note** shows its lead picture.
5. The post in one column about 36rem wide, with a folded contents list for long posts.
6. The tags, the share links and the author box.
7. Related posts: releases as sleeves and notes as picture cards.

## Other pages

- **Lists** (a category, a tag, the archive): the list's name and intro, then its releases as a shelf and its notes as picture cards.
- **The band** (`/authors/`): every member with their mark or photo, what they play and how many posts they wrote.
- **A member's page:** their mark, role and bio, then their releases and notes, and the rest of the band.

## Your brand

Encore brings its own colours: black ink and a vermilion accent (`#c42e1c`) on poster paper (`#f4f1ea`). The record labels, the ticket buttons on hover and the month names use the accent and link colours. Your colours, fonts and logo in `theme/tokens.toml` replace them. A tall condensed heading font suits it best, since the band's name, the tour cities and the release titles are set in the heading font in capitals.

Encore's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Encore's default |
|---|---|
| `[fonts] heading` | Impact, then other condensed sans fonts |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.5rem` to `4rem` |
| `[type] h2` | `1.75rem` to `2.25rem` |
| `[type] h3` | `1.25rem` to `1.5rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `400`, since condensed poster fonts have one heavy weight |
| `[shape] radius` | `0rem`, square |
| `[shape] measure` | `40rem` |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the band's name on the front page. |
| `listen_on` | none | Up to four places to listen under the band's name, each written `name \| address`. |
| `tour_dates` | none | The shows on the front page's tour list, up to 24, each written `date \| city \| venue \| tickets`. |
| `notes_count` | `3` | How many notes show on the front page, from `0` to `9`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `4` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "facebook", "whatsapp", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`, such as a link to your tickets. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video, such as your music videos. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
