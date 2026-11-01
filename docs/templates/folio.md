# Folio

Folio is a product team's notebook: essays, engineering notes and announcements under the blog's own masthead. It suits a software or design team whose posts are written by the people who built the thing, and who want each post to read like a short magazine piece.

Pick it when your posts have good pictures and you want the front page to look like a cover: a big name, a short line about the blog, one featured story and a grid of cards. If your posts have no pictures, or you publish long how-to guides with a table of contents, Meridian or Classic will suit you better.

## How to switch it on

```toml
# site.toml
[template]
name = "folio"
```

Build or restart the blog. Your colours, fonts, logo and corner rounding still come from `theme/tokens.toml`, so Folio takes on your main site's look.

## What readers see

**The front page**

- Your logo in the header, with your call to action as a button beside the menu.
- A centred masthead: the blog's name in large type, a one-line tagline, and a row of rounded links: "All", then each category your posts use.
- The newest post you marked `featured: true`, large, with its picture beside the text on a wide screen.
- Every other post as an outlined card: the picture on top, the title, the summary, then the author's round photo, name and role. Cards sit two to a row, three on a wide screen, one on a phone. The outline turns to your accent colour when a reader points at a card or tabs to it. Cards show no date.
- An optional quote, centred, in large type.
- Numbered page links with arrows under long lists.

**An essay** (every post)

- An "All posts" link back to the front page, in place of breadcrumbs.
- A centred title, then every author with their round photo, name and role, then the dates.
- Share links under the title and again after the essay.
- One column of text. Pictures and animated pictures keep their captions. Quotes are set large in the heading font.
- After the essay: the share links, then the band with your call to action.

End an essay with its takeaways. Folio sets a summary box as a closing section under a rule:

```markdown
:::callout{kind=summary}
**Takeaways**

- What a reader should remember, one line each.
:::

*Thanks to Lena for reading drafts of this post.*
```

**An author page** shows the author's photo, name, role, bio and profile links, then their posts as the same outlined cards.

## Options

Set any of these under `[template.options]` in `site.toml`. A line you leave out keeps Folio's default.

| Option | Folio's default | What it changes |
|---|---|---|
| `masthead_name` | the blog's name | One line: the name the masthead shows, such as `["The Workshop"]`. |
| `tagline` | none | One line under the name, such as `["Notes from the team that builds Acme."]`. Without it, the blog's intro shows. |
| `quote` | none | A quote centred under the cards: the words, then who said them, such as `["Ship small, ship often.", "Lena Vogt"]`. |
| `logo` | `true` | The header shows your logo from `[brand] logo`. Without a logo, the blog's name shows. |
| `back_link` | `true` | A post shows an "All posts" link back to the front page above its title. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the front page to it. |
| `category_tabs` | `true` | The masthead shows the row of category links. |
| `cards` | `image` | The shape of post cards in lists: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_author_role` | `true` | Each card shows its author's role under their name. |
| `featured` | `true` | The newest post marked `featured: true` shows large above the cards, and not again below. |
| `pagination` | `numbered` | The links under a long list: `pages` for newer and older, or `numbered` for a link to each page. |
| `related` | `grid` | How related posts are laid out when you turn them on: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `3` | How many related posts show, from 3 to 10. |
| `contents` | `none` | Where a post's contents go: `side-end`, `side-start`, `inline`, `inline-folded` or `none`. |
| `share` | `both` | Where a post shows share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "linkedin", "bluesky", "email"]` | Which networks the share links go to, in order. |
| `newsletter` | `none` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `post` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | Whether the header shows your call to action as a button. |

For example, a masthead with its own name, a tagline and the newsletter after each essay:

```toml
[template.options]
masthead_name = ["The Workshop"]
tagline = ["Notes from the team that builds Acme."]
newsletter = "post"
```

## Reader features Folio changes

Folio turns three `[reader]` features off by default: `reading_time`, `author_box` and `related_posts`. Write any of them in `[reader]` to turn it back on, such as `related_posts = true` for three related essays as cards. Your line always wins.

## Your call to action and newsletter

The header button and the band after each essay show the call to action you write once:

```toml
[template.cta]
label = "Try Acme free"
href = "https://example.com/signup/"
note = "Set up in five minutes."
```

Without `[template.cta]`, neither shows. The newsletter form is off in Folio. When you turn it on, it shows only on a blog that can take sign-ups: `mode = "server"` with a `[mail]` table.

## Sizes and spacing

Folio sets these in place of the shared defaults. A value in your own `theme/tokens.toml` wins.

| Token | Folio's value |
|---|---|
| `[type] body` | 1.0625rem on a phone to 1.125rem on a wide screen |
| `[type] h1` | 2.5rem to 3.5rem, also the masthead's name |
| `[type] h2` | 1.5rem to 1.875rem |
| `[type] line_height` | 1.7 |
| `[space] section` | 4rem between sections |
| `[shape] radius` | 0.75rem corners |
| `[shape] measure` | 66ch, the width of a line of text |

The page itself is up to 76rem wide, with 2rem between cards; the text of an essay keeps to the 66ch measure.

## Colours

Cards and bands sit on `--color-background` with a `--color-border` outline. The accent marks links, buttons, the current category and a card's outline when a reader points at it. Buttons pick black or white text, whichever reads better on your accent.

## What Folio does not do yet

- A post's category does not show on its card or above its title yet. The masthead's category links carry it.
- A card's picture is the post's lead picture (`image:` in a folder post). It shows on category, tag and archive pages and on author pages. The front page shows its cards and featured story without pictures for now, because `khalid serve` sends the front page before pictures have their sizes.
- Share links are the networks' names, not their logos.
- A large lead picture at the top of an essay needs the post's picture in the page, which Khalid does not pass to templates yet.
