# Channel: design brief

Channel is an original Khalid template for a video creator. Each post is one video with its notes, and runs of videos form series and playlists. It should read like the home of a video channel, not a magazine.

## Who it is for

- A creator who publishes videos on a video site and wants a home of their own, with notes, links and a transcript under each video.
- A small team that teaches a craft or a skill in videos, often in series.
- A creator who mixes videos with the odd written post, such as gear notes or a studio update.

## The content model

Channel adds no front matter. It reads what Khalid already has:

- **A video post is a post with a video in it**: a `::video` block, or a video embedded from YouTube, Vimeo or Loom. Khalid already finds these for the `video_section` row (`home_rows.rs`). The post page checks its own body for the same marks.
- **The running time is the first fact.** Facts are read by position: 1 the length, such as `16:42`, then any short facts, such as the level or the clay. The labels are the writer's own.
- **Playlists are categories.** Each category page is a playlist, with its intro as the description.
- **Series are Khalid series.** A run of parts shows "Part 2 of 3" on each part and on the series page.
- **The thumbnail is the lead picture.** The video's poster can be the same picture. The demo's thumbnails are free photos, never frames from real videos.

## The front page

1. **Channel header.** The channel's name, its tagline, and a figure strip of up to four numbers from `hero_stats`, such as videos, series and subscribers.
2. **Now showing.** The newest post with a video, large: its thumbnail on a wide screen, the play bar under it, and beside it the playlist, title, excerpt, date and a Watch button.
3. **Latest videos.** Every other post on the page as a grid of thumbnail cards, three across on a laptop and four on a wide screen.
4. **Playlists.** One shelf per category with its newest videos in a row that scrolls sideways on a phone, and a link to the whole playlist. `shelves` sets how many.
5. Numbered page links, the call to action band, such as "Subscribe on the video site", and the newsletter form.

## The post page

1. The trail, then the playlist above the title as a small label.
2. The title, the dek, the author with their mark, the date, and every fact as a small label, the running time first.
3. **The screen.** The post's first video, wider than the text column on a wide screen, on the surface colour. A post without a video shows its lead picture there instead.
4. The notes in one column. No contents list by default, so nothing comes between the title and the video.
5. The series box and the links to the parts on either side, when the post is part of a series.
6. Tags, share links and the author box.
7. **Up next:** related videos as thumbnail cards with their play bars.

## Other pages

- **Lists** (playlist, tag, series, archive): the list's name and intro, then the posts as thumbnail cards. A series page numbers them.
- **Hosts** (`/authors/`): the shared people grid.
- **A host:** their mark, name, role and bio, then their videos as thumbnail cards.

## Type pair

- Headings: **Sora** 600, a wide geometric sans made for screens. It reads well small on cards and large as a channel name.
- Text: **Onest** 400 and 600, a plain grotesque with clear figures for running times.

Both are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand.

## Palette

The pack's own default is a red record mark on white:

| Token | Value | Check |
|---|---|---|
| background | `#ffffff` | |
| surface | `#f3f1ec` | |
| text | `#17171c` | 17.9:1 on white |
| muted | `#54555d` | 7.4:1 on white, 6.6:1 on surface |
| link | `#b3261e` | 6.5:1 on white, 5.8:1 on surface |
| accent | `#b3261e` | white text on it 6.5:1 |
| border | `#dddad3` | |

The demo's sample brand is a pottery channel: fired clay (`#9a3f1f`) on a cream page (`#fbf8f3`) with a warm surface (`#f1e9dd`). Every pair passes AA.

## Signature device

**The play bar.** Under every thumbnail runs a thin track, like the seek bar of a paused player: a small play mark in the accent at its start and the running time at its end, in tabular figures. It sits under the picture, never on it, so no text covers a photo. The lead video on the front page has a thicker bar. The same bar shows on the front page, on every list, on a host's page and under "Up next", so the whole site reads as a channel.

## How it differs from the existing templates

- **Episode** leads with a podcast's newest episode and its sound player in boxed rows. Channel leads with a picture, a play bar and a grid of thumbnails, and each post opens on a wide screen.
- **Solo** is one writer's feed of large titles with no pictures in the list. Channel is picture-first.
- **Casper** and **Keystone** are picture-led blogs with a lead story. Channel's cards carry running times, its front page has playlist shelves, and its posts end with "Up next".
- **Hub**, the creator hub on the same list, is a link-in-bio home. Channel is a video library.
