---
title: "Summit"
order: 86
---

# Summit

Summit is a template for a conference, a festival of talks or any event with a programme. It shows the schedule, the speakers and the venues, and it reads like a printed programme, not a blog.

```toml
[template]
name = "summit"
```

## Who it suits

- A yearly conference or summit that gives each session its own page.
- A festival of talks spread over several venues in one town.
- A meetup or event series that posts its programme and its news in one place.

It suits a magazine or a company blog less well. For those, Atlas or Keystone read better.

## Sessions are posts with facts

Summit needs nothing new in your posts. A post with `facts` is a session. A post without facts is an update, such as "Tickets on sale". Put the facts in this order, since Summit reads them by their place, not their label:

```yaml
---
title: "Opening keynote: streets are for staying"
description: "What eleven high streets taught one urban designer about streets where people stay."
facts:
  - label: Day
    value: "2027-05-13"
  - label: Time
    value: "09:30"
  - label: Room
    value: "Main hall"
  - label: Format
    value: "Keynote"
---
```

1. **The day,** written `2027-05-13`. Summit shows it as "Thursday 13 May 2027" in your site's language. Other text, such as "Day 1", shows as you write it.
2. **The start time,** written with two digits for the hour, such as `09:30`, so the times sort in order.
3. **The room** or any place inside the venue.
4. **The format,** such as Keynote, Talk, Workshop, Panel or Walk. It shows as a small tag.

Your labels show on the session page, so write "Stage" or "Length" when that fits better. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

The author is the speaker. A panel names its other speakers in `co_authors`, and they all show on the session page. Categories are your venues: give each one an intro in `categories.toml` with its address and how to get there, and it shows at the top of that venue's page. Tags are tracks, such as "Transport" or "Housing".

## What the front page shows

1. A small line from `event_line`, such as the dates and the town, then your event's name, large, and the `tagline` under it.
2. A button with your call to action from `[template.cta]`, such as "Book a ticket".
3. A row of up to four large numbers from `hero_stats`, such as days, sessions, speakers and venues.
4. **Programme:** a link to each day, then every session on the timetable rail, grouped by day and sorted by time. Each row shows the time, the title, the speaker, the room and venue, and the format.
5. **Speakers:** the speakers of the sessions on the page, with their marks and roles. `speakers_count` sets how many.
6. **Venues** beside **Updates** on wide screens: each venue that holds sessions as a numbered tile with its number of sessions, and the newest updates with their dates. `news_count` sets how many updates.
7. Numbered page links, and your call to action as a band.

## What a session shows

1. The trail from the home page to the session's venue.
2. The venue as a small label, the title, and the post's description as a one-line summary.
3. The speakers with their marks and roles, and the date the post went up.
4. **The pass:** a ticket stub with the weekday and date large, a dashed tear line, then the time, room and format.
5. The lead picture, then the post in one column about 36rem wide, with a folded contents list for long posts.
6. The tracks (tags), the share links and the speaker box.
7. Related sessions on the timetable rail, each with its day.

An update has no pass.

## Other pages

- **A venue or a track** (a category, a tag, the archive): the name and intro, links to each day, then its sessions on the timetable rail, then its updates.
- **Speakers** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A speaker's page:** their mark or photo, role, experience and bio, then their sessions on the timetable rail, their updates, and the other speakers.

## Your brand

Summit brings its own colours: a signal orange accent (`#c2410c`) on white, with a warm paper surface for the ticket stub and the venue tiles. Section labels are small capitals after an accent square, and the header sits under a heavy ink rule. Your colours, fonts and logo in `theme/tokens.toml` replace them. A heavy sans for headings suits it best, since the event's name and the dates are set in the heading font.

Summit's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Summit's default |
|---|---|
| `[fonts] heading` | Avenir Next, then the system's own sans |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.0625rem` on a phone to `1.125rem` on a wide screen |
| `[type] h1` | `2.125rem` to `3.25rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.1875rem` to `1.3125rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `800` |
| `[shape] radius` | `0.25rem` |
| `[shape] measure` | `42rem` |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `event_line` | none | One short line above the event's name, such as its dates and town. |
| `tagline` | none | One line under the event's name on the front page. |
| `hero_stats` | none | Up to four numbers under the event's name, each written `number \| what it counts`. |
| `speakers_count` | `8` | How many speakers show on the front page, from `0` to `24`. |
| `news_count` | `4` | How many updates show beside the venues on the front page, from `0` to `10`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"text"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `4` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"numbered"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "linkedin", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |

## Design

The demo is Easthaven City Summit, a made-up three-day summit on streets, transport and public space. Its sample brand is deep street green (`#0d6b4f`) on white with a pale green surface, Archivo 800 for headings and Figtree for the text, both SIL OFL from the Google Fonts repository. The design brief is in `docs/design/templates/summit.md`.
