---
title: "Dialogue"
order: 89
---

# Dialogue

Dialogue is a template for an interview show, where every episode is a conversation with one guest. The guest leads everything: their name comes first, and one line they said sits in large type under it. Each episode opens with a name plate of host and guest, so the page reads as a conversation between two people.

```toml
[template]
name = "dialogue"
```

## Who it suits

- A podcast or radio show with a new guest each time, such as makers, founders, scientists or authors.
- A team that publishes each conversation with show notes and a transcript.
- A written interview series with no sound at all.

It suits a show without guests, or one led by its episode numbers and player, less well. For a company blog or a news site, Keystone or Wire read better.

## Episodes are posts with facts

Dialogue needs nothing new in your posts. A post with `facts` is an episode. A post without facts is a note, such as news about the show. Put the facts in this order, since Dialogue reads them by their place, not their label:

```yaml
---
title: "Henrik Dahl on working with a flame"
description: "Glassblower Henrik Dahl on learning to read hot glass."
facts:
  - label: Guest
    value: "Henrik Dahl"
  - label: Work
    value: "Glassblower, Malmö"
  - label: Length
    value: "50 min"
  - label: In one line
    value: "Glass tells you when it is ready."
---
```

1. **The guest's name.** It leads the episode's row and its name plate.
2. **Who they are,** such as their work and town.
3. **A short fact,** such as the length. It shows beside the date.
4. **One line the guest said,** at most 40 characters. It is the quote line: large italic type behind a big opening mark. Without it, a row shows the post's description instead.

See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

Categories are your topics, such as In the studio or Out and about. Give each an intro in `categories.toml`, and it shows at the top of that topic's page. Tags are the topics a conversation covers: an episode lists them as "Topics in this conversation". Your hosts are the authors.

Put the player in the text with Khalid's `::audio` block. Write a pull quote as a blockquote, and Dialogue sets it like the quote line. Write a transcript as paragraphs that start with the speaker's name in bold, inside a `:::toggle`, and Dialogue sets each name as a small label.

## What the front page shows

1. Your show's name, large, the `tagline` under it, and up to four `listen_links` as outlined links, such as your feed and your podcast app.
2. **Latest guest:** the newest episode, large: its picture, "In conversation with", the guest's name, who they are, their line as the quote line, then the title, date, length and topic.
3. **Conversations:** the other episodes as guest rows. Each row has a small square picture, the guest's name and who they are in a column of their own, then the title, the quote line and the date, length and topic.
4. **Topics:** each category that holds episodes, as a ruled link.
5. **From the show:** the newest notes as a dated feed. `notes_count` sets how many.
6. Numbered page links, your call to action as a band, and the newsletter form.

While a page of your front page holds no episodes, it shows every post as a guest row instead.

## What a post shows

1. The trail from the home page to the post's topic.
2. The topic as a small label, the title, and the post's description under it.
3. **The name plate:** "Host" with the author's mark, name and role, beside "Guest" with the guest's name and who they are.
4. The date and the length.
5. The guest's line as the quote line, then the lead picture.
6. The post in one column about 37rem wide, with a folded contents list for long posts.
7. The topics of the conversation (the tags), the share links and the author box.
8. More conversations as guest rows.

A note has no name plate and no quote line. It shows its author and date as a byline.

## Other pages

- **Lists** (a topic, a tag, the archive): the list's name and intro, then the posts as guest rows.
- **Hosts** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they made.
- **A host's page:** their photo or mark, role, experience and bio, then their conversations and notes as guest rows, and the other hosts.

## Your brand

Dialogue brings its own colours: a brick red accent (`#9c2f1b`) on warm paper, for the quote marks, links and the small round light before each section's label. Shapes are square: hairlines between rows and an ink rule under the header, no rounded cards. Your colours, fonts and logo in `theme/tokens.toml` replace them. A serif with a good italic for headings and the quote line, and a plain sans for the text, suit it best.

Dialogue's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Dialogue's default |
|---|---|
| `[fonts] heading` | Charter, then Cambria and Georgia |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.5rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.25rem` to `1.375rem` |
| `[type] small` | `0.9375rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `600` |
| `[shape] radius` | `0px`, square corners |
| `[shape] measure` | `38rem` |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the show's name on the front page. |
| `listen_links` | none | Up to four places to listen, under the tagline, each written `label \| address`. |
| `notes_count` | `5` | How many notes show on the front page, from `0` to `10`. |
| `hero_stats` | none | Up to four numbers under the show's name, each written `number \| what it counts`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["whatsapp", "facebook", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
