---
title: "Frequency"
order: 86
---

# Frequency

Frequency is a template for a podcast's own website. It plays your newest episode on the front page, lists every episode with its number and running time, and tells listeners where to find the show. It reads like a show's home on the web, not a blog with sound files in it.

```toml
[template]
name = "frequency"
```

## Who it suits

- A podcast with one or two regular hosts that publishes numbered episodes, such as a weekly science, hobby or business show.
- A show that already lives in podcast apps and wants show notes, transcripts and links on a site it owns.
- A team or company that runs a podcast and wants each episode to be a page search engines can find.

It suits a written blog or a video channel less well. For those, Classic or Channel read better.

## Episodes are posts with a player

Frequency needs nothing new in your posts. Put the player at the very top of the show notes, so it joins the episode plate: the `::audio` block with a sound file, or an `::embed` from a podcast site. Then give the post its facts. Frequency reads them by their place, not their label:

```yaml
---
title: "Episode 45: how the space station stays up"
facts:
  - label: Episode
    value: "45"
  - label: Length
    value: "61 min"
  - label: Guest
    value: "Dr Tomás Ibarra, orbital engineer"
  - label: Season
    value: "4"
---

::audio{src="episode-45.mp3" title="Episode 45: how the space station stays up"}
```

1. **The episode number,** shown large on every row.
2. **The running time in minutes,** such as "61 min". It draws the running-time dial. Write minutes only: "1 h 12 min" reads as 112.
3. **The guest,** or anything short, such as the hosts.
4. **Any short fact,** such as the season.

A post without facts is a note, such as a live show or a break. It shows in the episode list without a number and without a plate. See [Facts on a post](../templates.md#facts-on-a-post) for the limits. Categories are your topics, tags are each episode's subjects, and authors are the hosts.

## Where to listen

List your podcast apps and feed in `listen_on`, one line each, written `label | address`:

```toml
[template.options]
listen_on = [
  "Apple Podcasts | https://podcasts.apple.com/podcast/id123",
  "Spotify | https://open.spotify.com/show/abc",
  "RSS feed | /blog/feed.xml",
]
```

They show as tiles under the show's name on the front page, and again after every episode while `post_listen` is on.

## What the front page shows

1. Your show's name, large, the `tagline` under it, the listen tiles and up to four figures from `hero_stats`, such as the number of episodes.
2. **Latest episode** beside it on wide screens: a dark console with the newest episode that has a player. It shows the picture, the number and date, the title and summary, the running-time dial, the player itself and a link to the show notes.
3. **Episodes:** every other post as a numbered row, with its topic, date, title, summary, running-time dial and guest, and a square picture on screens wider than a phone.
4. **Topics:** every category as a chip.
5. Numbered page links, the call to action band and, when you take sign-ups, the newsletter form.

On page 2 and later, only the episode rows show.

## What an episode shows

1. The episode number and topic as a small label, the title and the post's description.
2. The hosts and the date, with the episode's picture beside them on wide screens and under them on phones.
3. A folded contents list for long show notes.
4. **The episode plate:** a dark band with every fact and the running-time dial. The player at the top of the show notes joins it, so the two read as one console.
5. The show notes in one column about 36rem wide.
6. **Listen on:** the listen tiles again.
7. The tags, the share links and the host box, then the call to action band and related episodes as rows.

## Other pages

- **Lists** (a topic, a tag, the archive): the list's name and intro, then its episodes as rows.
- **Hosts** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they made.
- **A host's page:** their photo or mark, role, experience and bio, then their episodes as rows.

## Your brand

Frequency brings its own colours: warm paper (`#fbfaf7`), ink (`#17161c`) for the console and the plate, and a signal orange (`#b23a0e`) for links, buttons and the dial. Your colours, fonts and logo in `theme/tokens.toml` replace them. Episode numbers, running times and dates use the `mono` font, so a mono with clear figures suits it best.

Frequency's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Frequency's default |
|---|---|
| `[fonts] heading` | Avenir Next, then other clean sans fonts |
| `[fonts] body` | The system's own sans |
| `[fonts] mono` | The system's own monospace |
| `[type] body` | `1.0625rem` on a phone to `1.125rem` on a wide screen |
| `[type] h1` | `2.125rem` to `3.25rem` |
| `[type] h2` | `1.5rem` to `2rem` |
| `[type] h3` | `1.1875rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0.375rem` |
| `[shape] measure` | `40rem` |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the show's name on the front page. |
| `listen_on` | none | Up to six listen links, each written `label \| address`, on the front page and after every episode. |
| `post_listen` | `true` | Every episode ends with the listen links. |
| `hero_stats` | none | Up to four figures under the show's name, each written `number \| what it counts`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"row"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many related episodes show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where long show notes' contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "whatsapp", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"both"` | Where the call to action band shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
