---
title: "Folio"
order: 40
---

# Folio

Folio is a product team's notebook: essays, engineering notes with code, and announcements under the blog's own masthead. It suits a software or design team whose posts are written by the people who built the thing.

Pick it when you want the front page to open with a confident name, one featured story beside a numbered list of the newest posts (or a dated log of your releases), a row of picture cards and an index of the rest, and want long notes to read well: a contents rail beside the text, framed code and tables, an author box and related posts. If your posts are short news items with no pictures, Wire or Classic will suit you better.

## How to switch it on

```toml
# site.toml
[template]
name = "folio"
```

Build or restart the blog. Folio brings its own neutral palette and type scale; a value in your `theme/tokens.toml` wins, so a synced theme still gives Folio your main site's colours and fonts.

## What readers see

**The front page**

- A slim announcement line, then your logo, a quiet menu and your call to action as a small rounded button. On a phone the menu takes its own line, scrolls sideways and fades at the end to show there is more, and the button stays in the band at the end of the page.
- The masthead: the blog's name set large and tight on faint ruled paper (a fine grid that fades out), an optional tagline, and the categories as tabs on a hairline: "All", then each category your posts use. On a phone the tabs become small rounded chips.
- The lead: the newest post you marked `featured: true`, its picture large with the title set big under it, and beside it a numbered rail of the five newest posts, 01 to 05, each with its category and date. On a phone the rail follows the featured post.
- The next six posts as cards without an outline: the picture in a thin frame, the category in the accent and the date, the title, up to three lines of summary, then the author's round photo and name. Three cards to a row on a wide screen, two on a tablet. On a phone each card is a ruled row instead: the title, the date under it and a small square picture at the end.
- Every older post on the page as a text-only index: one ruled row each, the date in small mono capitals beside the title, two columns on a wide screen. On a phone the date sits under the title. The front page lists every post, so readers and search engines can reach each one.
- Without a featured post, or with `topic_tabs` on, the posts show as cards in the shape `cards` sets.
- When some posts list `facts` (see "Release notes" below), Folio treats those posts as releases and the lead changes. The essays lead: the featured essay large, then four picture cards in two columns. Beside them runs a narrow release log on one hairline: the newest eight releases, each with its version as a small tag, its date in small mono capitals and its linked title. The newest release has an accent square on the line and a tinted tag. On a wide screen the log stays in view while the essays scroll; on a phone it follows the essays. The log is headed by the name of the newest release's category, with a link to all of that category's posts. Every other post shows in the text index below, so the front page still lists every post. The numbered rail and the row of six cards do not show in this layout.
- An optional quote, centred, in large type under a short accent rule.
- Numbered page links under long lists, and the band with your call to action.

**A post**

- The breadcrumbs as a small line above the title: the blog, then the post's category.
- A release (a post with `facts`) shows its facts above the title: the first as a tag, such as "Version 2.5", then the others as small mono pairs, such as "Changes 6" and "Fixes 4". A post without facts shows nothing extra.
- A large left-aligned title, the post's summary (`dek:`) under it, then a hairline, the authors with their photos and roles, and one line with the reviewer, the dates and the reading time.
- The lead picture, wider than the text. On a wide screen it spans most of the page, centred.
- The text in one column of about 70 characters. From a laptop up the text is larger (1.25rem) and the column widens with it, so lines keep their length. On a wide screen the contents sit in a quiet rail beside it, with the section being read marked in the accent; on a phone they fold into a box above the text.
- Code in a framed sheet, tables with hairline rows inside one frame, callouts with a thin accent edge, and quotes set large under a short accent rule.
- After the text: the share links, the author box, the band with your call to action, the posts that link here, and three related posts as cards.

**A list page** (a category, a tag or the archive) shows the name in large type, then the cards under a hairline.

**An author page** shows the author's photo beside their name and role, the bio, then their posts as the same cards and the other authors.

## Release notes

Folio shows a post's `facts` as a release. Write the version first, because Folio uses the first fact as the release's tag. Add up to three more facts, such as how many changes and fixes it has:

```yaml
---
title: "Tern 2.4: offline search"
category: changelog
facts:
  - label: Version
    value: "2.4"
  - label: Changes
    value: "6"
  - label: Fixes
    value: "4"
---
```

Folio picks facts by their place in the list, not by their label, so the labels can be in any language. On the front page the log shows only the first fact's value; the label is read out to screen readers. Put all your releases in one category, since the log links to the category of the newest one. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

## Options

Set any of these under `[template.options]` in `site.toml`. A line you leave out keeps Folio's default.

| Option | Folio's default | What it changes |
|---|---|---|
| `hero_stats` | none | Up to four numbers under the front page's title, each written `number \| what it counts`. Nothing shows until you write them. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. They work with scripts off. |
| `video_section` | `true` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `true` | A row of the newest posts with a podcast or other sound player, when there are any. |
| `masthead_name` | the blog's name | One line: the name the masthead shows, such as `["The Workshop"]`. |
| `tagline` | none | One line under the name, such as `["Notes from the team that builds Acme."]`. Without it, the blog's intro shows. |
| `quote` | none | A quote centred under the cards: the words, then who said them, such as `["Ship small, ship often.", "Lena Vogt"]`. |
| `logo` | `true` | The header shows your logo from `[brand] logo`. Without a logo, the blog's name shows. |
| `back_link` | `false` | A post shows an "All posts" link back to the front page above its title. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the front page to it. |
| `dek` | `true` | A post shows its `dek:` summary under the title. |
| `category_tabs` | `true` | The masthead shows the row of category tabs. |
| `cards` | `image` | The shape of post cards in lists: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_author_role` | `false` | Each card shows its author's role beside their name. |
| `featured` | `true` | The newest post marked `featured: true` shows large above the cards, and not again below. |
| `pagination` | `numbered` | The links under a long list: `pages` for newer and older, or `numbered` for a link to each page. |
| `related` | `grid` | How related posts are laid out: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `3` | How many related posts show, from 3 to 10. |
| `contents` | `side-end` | Where a post's contents go: `side-end`, `side-start`, `inline`, `inline-folded` or `none`. |
| `reading_progress` | `bar` | A thin line at the top of a post that fills as the reader scrolls: `bar` or `none`. |
| `header_search` | `button` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `share` | `end` | Where a post shows share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "linkedin", "bluesky", "email"]` | Which networks the share links go to, in order. |
| `newsletter` | `none` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `post` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | Whether the header shows your call to action as a button. |

For example, a masthead with its own name, a tagline and the newsletter after each post:

```toml
[template.options]
masthead_name = ["The Workshop"]
tagline = ["Notes from the team that builds Acme."]
newsletter = "post"
```

## Reader features

Folio keeps every `[reader]` feature on, as Classic does: the reading time, the contents, the author box and related posts all show. Write a feature in `[reader]` to turn it off, such as `related_posts = false`. Your line always wins.

## Your call to action and newsletter

The header button and the band after each post show the call to action you write once:

```toml
[template.cta]
label = "Try Acme free"
href = "https://example.com/signup/"
note = "Set up in five minutes."
```

Without `[template.cta]`, neither shows. The band is a sheet of the ruled paper with your note set large and the button beside it. The newsletter form is off in Folio. When you turn it on, it shows as a white sheet with the field and the button on one line, and only on a blog that can take sign-ups: `mode = "server"` with a `[mail]` table.

## Colours, type and spacing

Folio sets these in place of the shared defaults. A value in your own `theme/tokens.toml` wins.

| Token | Folio's value |
|---|---|
| `[colors]` | a white page, a warm grey surface (`#f5f4f1`), graphite text (`#111113`), grey quiet text (`#5c5c66`), an ochre accent (`#9a5900`) and links (`#8a4f00`), light hairlines (`#e3e1db`) |
| `[fonts] heading` and `body` | the reader's own interface face: San Francisco, Segoe UI Variable or Roboto |
| `[fonts] mono` | the system's code face: SF Mono, Menlo or Consolas |
| `[type] body` | 1.0625rem on a phone to 1.1875rem on a wide screen |
| `[type] h1` | 2.5rem to 4.5rem. A post's title runs up to 4.5rem and the masthead's name up to 5.5rem |
| `[type] h2` | 1.625rem to 2.125rem |
| `[type] line_height` | 1.7 |
| `[space] section` | 6rem between sections |
| `[shape] radius` | 0.5rem corners on every frame: pictures, code, tables, callouts and bands |
| `[shape] measure` | 40rem, about 70 characters, the width of a post's text |

The page itself is up to 88rem wide. The accent marks the category on each card, links, buttons, the current tab and the section being read; everything else is neutral. All colours pass the WCAG AA contrast check.

## What Folio does not do yet

- Share links are the networks' names, not their logos.
- The footer shows your footer links in one row; it has no columns of its own unless you add `[[nav.footer_groups]]`.

## Design

This is Folio's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A product or engineering team whose posts are written by the people who built the thing: essays, release notes and engineering notes with code.

**Reference blogs**, by kind. Their pictures are kept with the design notes, outside the repository.

- A CSS framework company's blog. The page is drawn on faint ruled lines like graph paper, the title set very large and tight, the date in small mono capitals above it, the author in a column beside the text, and plenty of room around few elements.
- A web hosting platform's blog. A grid of posts on a precise, ruled page, small mono labels for dates and categories, and posts in a clean column with framed code.

| Part | Folio |
|---|---|
| Type pairing | A tight display sans for headings, set large with slight negative tracking, over a neutral text sans. With no theme, the pack uses the system sans (San Francisco, Segoe UI Variable, Roboto). |
| Colour | Quiet monochrome: white ground, near-black ink, warm grey panels and hairlines. One accent only, used where a reader can act and on the section being read. |
| The one idea | A notebook's index. The front page is a masthead that owns the top of the screen, then a lead story beside a numbered rail, a short row of cards, and a text-only index of ruled rows with mono dates. You can name it from its thumbnail: a large name, one big picture, a column of numbers or a dated release log, and ruled lines. |
| Display type | The masthead's name from 2.5rem on a phone to 5.5rem on a wide screen, calm enough to sit beside the essays, tracked tight (-0.05em, loose enough for a serif brand face too). Post and list titles up to 4.5rem and 8rem. The featured essay's title beside the release log stops at 3.75rem, below the masthead's name. A post's text is 1.25rem from 64rem up, in a column 1.1 times the measure. |
| Spacing rhythm | A base step of 0.5rem on a strict grid. Large gaps (6rem) between front page sections, tight gaps (0.75rem) inside a card. |
| Frame at 1440 px and wider | At least **80rem**. |
| Cards and lists | Cards without an outline: a picture in a thin frame, a small category and date line, the title, up to three lines of summary, the author's photo and name. Three across on a wide screen. |
| Phone header and menu | The logo on one row; the menu on its own line under it as one row that scrolls sideways, its last 2rem faded so no word looks cut. No script. The call to action waits for the band at the end. |
| Phone lists | Picture cards become ruled rows (title, date under it, a small square picture at the end); the index puts the date under the title. The front page still lists every post, at about 6,000 px for 40 posts. |
| Tabs and indexes at 390 px | The category tabs become small rounded chips that wrap. The contents rail folds into one box above the text. |
| Link, card and focus states | Links underlined with a thin line that turns the accent on hover. A card's title turns the accent on hover. Focus is a 2 px ring in the text colour, so it shows on any accent. |
| Signature details | The masthead: the blog's name large and tight on faint ruled paper that fades out, and the same paper behind each post and list title. Numbers 01 to 05 in mono down the lead rail, or, on a blog with releases, the dated release log on one hairline with version tags in mono. Hairlines everywhere a box would be. Code in a framed sheet with a small label. |

**Sample brand.** The demo uses `fixtures/showcase-brands/folio/theme/`: graphite ink (`#111113`) on white with warm grey panels (`#f5f4f0`) and one gold accent (`#755600` links, `#876400` accent), Inter Tight for headings and Inter for text, and a ruled-sheet mark.
