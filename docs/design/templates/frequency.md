# Frequency: design brief

Frequency is an original Khalid template for a podcast's own website. It puts the newest episode and its player first, lists every episode with its number and running time, and tells listeners where to find the show. It should read like a show's home on the web, not a blog with sound files in it.

## Who it is for

- A podcast with one or two regular hosts that publishes numbered episodes, such as a weekly science, hobby or business show.
- A radio-style show that already lives in podcast apps and wants show notes, transcripts and links on a site it owns.
- A team or company that runs a podcast beside its blog and wants each episode to be a page search engines can find.

Frequency hosts no audio of its own beyond what a post carries. The player is Khalid's `::audio` block, or an embedded player from a podcast site, and the listen links point to the podcast apps.

## The content model

Frequency adds no front matter. It reads what Khalid already has:

- **An episode is a post with a player.** The writer puts `::audio` (or an `::embed` from a podcast site) at the top of the show notes. Khalid already finds the newest post with a player and gives the front page its player, so the newest episode plays there.
- **Facts, read by position:** 1 the episode number, 2 the running time in minutes (such as "48 min"), 3 the guest, 4 any short fact, such as the season. The labels are the writer's own.
- **A post without facts** is a note, such as a live show or a break between seasons. It shows in the episode list without a number.
- **Topics are categories.** Tags are a post's subjects. Authors are the hosts.
- **Where to listen** is the `listen_on` option: one line per app, written `label | address`.
- **The call to action** (`[template.cta]`) is the show's own ask, such as "Follow the show" or "Support the show".

## The front page

1. **The show:** its name large, the tagline, the listen links as a row of tiles, and up to four figures from `hero_stats`, such as the number of episodes.
2. **On air,** beside the show on wide screens: a dark console with the newest episode's picture, its number, the running-time dial, its title and summary, the player and a link to the show notes.
3. **Episodes:** every other episode as a numbered row: the number large in mono figures, the topic and date, the title, the summary, the running-time dial with the guest, and a square picture at the end.
4. **Topics:** every category as a chip.
5. The call to action band and the newsletter.

On page 2 and later, the show and On air step aside and the episode rows fill the page.

## The post page

1. The episode number and topic as a small label, the title, the dek, the hosts and the date, with the episode's picture beside them on wide screens.
2. **The episode plate:** a dark band that holds the facts (episode, length, guest, the fourth fact) and the running-time dial. The player that opens the show notes joins it, so the plate and player read as one console.
3. The show notes in one column, about 66 characters a line.
4. **Listen on:** the listen links again, so a reader can follow the show in their own app.
5. Tags, share links, the host box, the call to action band, and related episodes as numbered rows.

A note, without facts, has no plate.

## Other pages

- **Lists** (category, tag, archive): the list's name and intro, then the episode rows.
- **Hosts** (`/authors/`): the shared people grid.
- **A host:** their mark or photo, role and bio, then their episodes as rows.

## Type pair

- Headings: **Bricolage Grotesque** 700, a lively grotesque with the warmth of a show poster.
- Text: **Rethink Sans** 400 and 600, open and calm at 18 px for long show notes and transcripts.
- Figures: **DM Mono** 500 for episode numbers, running times and dates, like the readout on a radio.

All three are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand. The pack's own default falls back to system fonts.

## Palette

The pack's own default is warm paper with ink and a signal orange:

| Token | Value | Check |
|---|---|---|
| background | `#fbfaf7` | |
| surface | `#f1eee6` | |
| text | `#17161c` | 17.2:1 on background; the console's ground |
| muted | `#56535e` | 7.2:1 on background, 6.5:1 on surface |
| link | `#b23a0e` | 5.7:1 on background, 5.2:1 on surface |
| accent | `#b23a0e` | white text on it 6.0:1 |
| border | `#e2ddd2` | |

The console reverses the page: background-coloured text on the text colour, 17.2:1. Its quieter text is a mix of the two at about 8:1.

The demo's sample brand is a night-sky show in deep indigo (`#3b3fae`, 8.1:1 on its background) with a lavender surface.

## Signature device

**The running-time dial.** Every episode carries a short tuning scale, a ruler of ticks for every five minutes and a taller tick every quarter hour, with the accent line running along it to the episode's length. It sits under each row, in the plate of each episode and large on the On air console, so a listener sees at a glance which episodes are a quick listen and which are long. The dial is a plain `<meter>` built from the second fact, with no script.

## How it differs from the existing templates

- **Channel** is video first, with thumbnail cards and a play bar. Frequency is sound first: rows with episode numbers, a running-time dial and a real player on the front page.
- **Encore** shows a musician's releases on a shelf of sleeves and tour dates. Frequency has no shelf: its episodes are numbered rows read in order.
- **Hub** is one narrow column of link buttons. Frequency uses the shared link stack only for its listen links.
- **Chapter** lists excerpts as a book's contents page. Frequency's rows carry a number, a picture and a dial, and its front page plays the newest episode.

The dial lives in Frequency's own `parts-frequency.html`, since only a show with running times needs it.
