---
title: "Channel"
order: 85
---

# Channel

Channel is a template for a video creator. Each post is one video with its notes, and the front page reads like the home of a video channel: the newest video large, then a grid of thumbnails, then a shelf for each playlist.

```toml
[template]
name = "channel"
```

The name has nothing to do with the channels in `routes.toml`, the filtered lists such as `/featured/`. Those work in Channel as in every template.

## Who it suits

- A creator who publishes videos on a video site and wants a home of their own, with notes, links and a transcript under each video.
- A small team that teaches a craft or a skill in videos, often in series.
- A creator who mixes videos with the odd written post, such as gear notes or a studio update.

It suits a podcast less well; Episode leads with a sound player. For a writer with no videos, Solo reads better.

## Videos are posts with a video in them

Channel needs nothing new in your posts. A post is a video post when its body holds a video: a `::video` block, or a YouTube, Vimeo or Loom link in an `::embed`. Put the video first in the post, so the post opens on it.

```markdown
---
title: "Your first bowl, part 2: opening and pulling the walls"
description: "How to open centred clay, set the floor and pull even walls."
category: throwing
series:
  name: Your first bowl
  order: 2
facts:
  - label: Length
    value: "19:05"
  - label: Level
    value: "Beginner"
image:
  src: lead.jpg
  alt: A potter's hands shape a ridged clay pot on a turning wheel
---
::video{src="opening.mp4" title="Opening and pulling the walls" poster="lead.jpg" duration="19:05"}

The notes start here.
```

- **The first fact is the running time.** Channel shows it at the end of the play bar under every thumbnail. Write it as the video site does, such as `19:05`. A post with no facts, such as a written post, gets a bare bar, so its card still lines up with the rest.
- **The other facts** show as small labels under the post's title, such as the level or the kit. Channel reads facts by their place, not their label. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.
- **The lead picture is the thumbnail.** It shows on every card, cut to 16:9. Use the same picture as the video's `poster`. A post without a video shows its lead picture where the video would be.
- **Playlists are categories.** Give each one an intro in `categories.toml`, and it shows at the top of that playlist's page.
- **Series** are Khalid's own series: give each part the same `series` name and its `order`. Each part shows "Part 2 of 3" and links to the parts on either side, and the series page numbers its videos.

## What the front page shows

1. Your channel's name, large, and the `tagline` under it.
2. A row of up to four large numbers from `hero_stats`, such as videos, series and subscribers. Nothing shows until you write them.
3. **Now showing:** your newest post with a video, large, on a panel of your surface colour: the thumbnail with a thick play bar under it, and beside it the playlist, title, excerpt, date and a Watch now button. Only the first page shows it.
4. **Latest videos:** every other post on the page as thumbnail cards, three across on a laptop. Under each picture runs the play bar: a play mark, a track and the running time. No words ever sit on a picture.
5. Numbered page links.
6. **Playlists:** a shelf for each category, with its four newest videos in a row and a link to the whole playlist. On a phone the row scrolls sideways. `shelves` sets how many show.
7. Your call to action as a band, such as "Subscribe to the channel", and the newsletter form.

With `[[reader.home_sections]]` in `site.toml`, each section is a shelf in place of Now showing and the latest videos.

## What a post shows

1. The trail from the home page to the post's playlist.
2. The playlist as a small label, the title, and the post's description under it.
3. The host with their mark and role, the date, and the facts as small labels.
4. **The screen:** the video, on your surface colour, wider than the text on a laptop. A post without a video shows its lead picture there.
5. The notes in one column about 40rem wide.
6. The series box and the links to the parts before and after, when the post is part of a series.
7. The tags, the share links and the author box.
8. **Up next:** related posts as thumbnail cards with their play bars.

## Other pages

- **Lists** (a playlist, a tag, a series, the archive): the list's name and intro, then the posts as thumbnail cards. A series page numbers them, "Part 1" and on.
- **Hosts** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they made.
- **A host's page:** their photo or mark, role, experience and bio, then their videos as thumbnail cards, and the other hosts.

## Your brand

Channel brings its own colours: a red record mark (`#b3261e`) on white, with a warm surface behind each video and on the Now showing panel. Buttons and labels are rounded, like a video site's. Your colours, fonts and logo in `theme/tokens.toml` replace them. A wide sans for headings suits it best.

Channel's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Channel's default |
|---|---|
| `[fonts] heading` | The system's own sans |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.0625rem` on a phone to `1.125rem` on a wide screen |
| `[type] h1` | `2rem` to `3rem` |
| `[type] h2` | `1.375rem` to `1.75rem` |
| `[type] h3` | `1.125rem` to `1.25rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.65` |
| `[type] heading_weight` | `600` |
| `[shape] radius` | `0.5rem` |
| `[shape] measure` | `40rem` |
| `[space] section` | `4rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the channel's name on the front page. |
| `hero_stats` | none | Up to four numbers under the channel's name, each written `number \| what it counts`. |
| `shelves` | `4` | How many playlists show as a shelf on the front page, from `0` to `6`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `6` | How many videos Up next shows, from `3` to `10`. |
| `contents` | `"none"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. Channel shows none, so a post opens on its video. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "facebook", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
