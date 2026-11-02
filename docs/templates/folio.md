# Folio

Folio is a product team's notebook: essays, engineering notes with code, and announcements under the blog's own masthead. It suits a software or design team whose posts are written by the people who built the thing.

Pick it when you want the front page to open with a confident name, one featured story and a grid of picture cards, and want long notes to read well: a contents rail beside the text, framed code and tables, an author box and related posts. If your posts are short news items with no pictures, Wire or Classic will suit you better.

## How to switch it on

```toml
# site.toml
[template]
name = "folio"
```

Build or restart the blog. Folio brings its own neutral palette and type scale; a value in your `theme/tokens.toml` wins, so a synced theme still gives Folio your main site's colours and fonts.

## What readers see

**The front page**

- A slim announcement line, then your logo, a quiet menu and your call to action as a small rounded button. On a phone the menu takes its own line and the button stays in the band at the end of the page.
- The masthead: the blog's name set very large and tight on faint ruled paper (a fine grid that fades out), an optional tagline, and the categories as tabs on a hairline: "All", then each category your posts use. On a phone the tabs become small rounded chips.
- The newest post you marked `featured: true`: its picture large on the left, the category and date, the title, the summary and the author on the right.
- Every other post as a card without an outline: the picture in a thin frame, the category in the accent and the date, the title, up to three lines of summary, then the author's round photo and name. Three cards to a row on a wide screen, two on a tablet, one on a phone.
- An optional quote, centred, in large type under a short accent rule.
- Numbered page links under long lists, and the band with your call to action.

**A post**

- The breadcrumbs as a small line above the title: the blog, then the post's category.
- A large left-aligned title, the post's summary (`dek:`) under it, then a hairline, the authors with their photos and roles, and one line with the reviewer, the dates and the reading time.
- The lead picture, wider than the text.
- The text in one column of about 70 characters. On a wide screen the contents sit in a quiet rail beside it, with the section being read marked in the accent; on a phone they fold into a box above the text.
- Code in a framed sheet, tables with hairline rows inside one frame, callouts with a thin accent edge, and quotes set large under a short accent rule.
- After the text: the share links, the author box, the band with your call to action, the posts that link here, and three related posts as cards.

**A list page** (a category, a tag or the archive) shows the name in large type, then the cards under a hairline.

**An author page** shows the author's photo beside their name and role, the bio, then their posts as the same cards and the other authors.

## Options

Set any of these under `[template.options]` in `site.toml`. A line you leave out keeps Folio's default.

| Option | Folio's default | What it changes |
|---|---|---|
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
| `[colors]` | a white page, a soft grey surface (`#f4f4f2`), ink text (`#141418`), grey quiet text (`#5b5b66`), a violet accent and links (`#4b36d1`), light hairlines (`#e4e4df`) |
| `[fonts] heading` and `body` | the reader's own interface face: Inter where it is installed, else San Francisco, Segoe UI or Roboto |
| `[fonts] mono` | JetBrains Mono where installed, else the system's code face |
| `[type] body` | 1rem on a phone to 1.0625rem on a wide screen; a post's text is set a step larger |
| `[type] h1` | 2.25rem to 3.5rem, a post's title. The masthead's name runs larger, up to 5.75rem |
| `[type] h2` | 1.5rem to 1.875rem |
| `[type] line_height` | 1.65 |
| `[space] section` | 5rem between sections |
| `[shape] radius` | 0.75rem corners on every frame: pictures, code, tables, callouts and bands |
| `[shape] measure` | 42rem, about 70 characters, the width of a post's text |

The page itself is up to 76rem wide. The accent marks the category on each card, links, buttons, the current tab and the section being read; everything else is neutral. All colours pass the WCAG AA contrast check.

## What Folio does not do yet

- Share links are the networks' names, not their logos.
- The footer shows your footer links in one row; it has no columns of its own unless you add `[[nav.footer_groups]]`.
