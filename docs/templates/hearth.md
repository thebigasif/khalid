---
title: "Hearth"
order: 8
---

# Hearth

Hearth is a warm, classic blog for a small team or a family business. The front page welcomes readers, leads with one story and its picture, then shows the other posts as a grid of picture cards. Posts read in a friendly column. Every picture sits in an arch, like the opening of a hearth.

```toml
[template]
name = "hearth"
```

## Who it suits

- A family business that writes about its work, its seasons and its people, such as a nursery, a bakery, a bike shop or a farm.
- A small team blog where two or three people take turns to write, each with a name and a face.
- A local shop or maker who wants readers to come back for the next story, and then come in.

For a blog where the words matter more than pictures, pick Ledger. For recipes with times and servings, pick Harbor.

## What it needs from your posts

Hearth needs nothing new in your posts. It reads what Khalid already has:

- **A lead picture** on each post. Cards and the lead story show it, cropped to fit the arch.
- **A category** on each post. It shows over the title on cards and posts, and the front page lists every category as a link.
- **Authors with a photo or a drawn mark.** Each card shows its author's face and name, and a post shows its author under the title.
- **`featured: true`** on the post you want to lead the front page. Without one, the newest post leads.

## What the front page shows

1. **The welcome:** your blog's name large and centred, under a small arch, with the front page's intro or your `tagline` under it, and up to four numbers from `hero_stats` when you write them.
2. **The lead story:** on a warm panel, its picture in a tall arch on one side, and its category, title, summary, author and date on the other. With `featured` on, the newest featured post leads; otherwise the newest post does. It shows on the first page only.
3. **Latest posts:** the other posts on the page as cards, three across on a wide screen and two on a tablet. Each card has its picture in a low arch, the category, the title, the summary, and the author's face, name and date. On a phone each card is a row: its picture in a small tall arch beside the category, the title and the author, so the page stays light.
4. **Topics:** every category as a rounded link.
5. Numbered page links, your call to action as a band, and the newsletter form.

When `[[reader.home_sections]]` lists sections, each section is a grid of up to six cards in place of Latest posts.

## What a post shows

1. A centred head: the category, the title, the summary, the author's face and name, the date and the reading time.
2. The lead picture, wider than the text, in a low arch, with its caption under it.
3. The text in a friendly reading column of about 66 characters a line, with a folded contents list for a long post.
4. The tags, the share links and the author box, set as a signed note.
5. **Keep reading:** the related posts as cards.

## Other pages

- **Lists** (a category, a tag, the archive): the list's name and intro centred over a hairline, then the card grid.
- **People** (`/authors/`): everyone who writes, with their face or mark, role and how many posts they wrote.
- **A person's page:** their face or mark, role, experience and bio, their posts as cards, then the rest of the team.

## Your brand

Hearth brings its own look: cream paper, dark brown ink and one terracotta accent, like brick round a fire. Titles are set in a warm serif and the text in a plain sans. Your colours, fonts and logo in `theme/tokens.toml` replace them. A serif with soft curves for headings and a round, open sans for text suit it best.

Hearth's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Hearth's default |
|---|---|
| Colours | background `#fdf8f1`, surface `#f6ecdf`, text `#2a201a`, muted `#6a5848`, link and accent `#a3412a`, border `#eadccb` |
| `[fonts] heading` | Constantia, then Georgia and other serifs |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.0625rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.5rem` |
| `[type] h2` | `1.5rem` to `2.125rem` |
| `[type] h3` | `1.25rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.7` |
| `[type] heading_weight` | `600` |
| `[shape] radius` | `0.5rem`, the small corners at the foot of each arch |
| `[shape] measure` | `42rem` |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under your blog's name on the front page. The front page's intro, when you write one, shows in its place. |
| `featured` | `true` | The newest post marked featured leads the front page. With `false`, the newest post leads. |
| `hero_stats` | none | Up to four numbers under the welcome, each written `number \| what it counts`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `false` | A trail of links over each page, from the home page down. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many related posts Keep reading shows under a post, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["facebook", "whatsapp", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |

## Pictures

The lead story's picture is cut to 4:5 under a full round arch on a wide screen, and to 4:3 on a phone. Card pictures are cut to 4:3 and a post's lead picture to 16:10, each under a low arch. Give each lead picture at least 1600 pixels across, with its subject in the middle, so every crop keeps it.

Pictures are never covered by text. Titles, summaries and bylines always sit beside or under the picture.
