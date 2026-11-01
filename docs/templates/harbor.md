# Harbor

Harbor is a template for a busy marketing blog: a team that publishes often, wants readers to sign up, and has one offer to put in front of every reader, such as a free checklist or a trial.

```toml
[template]
name = "harbor"
```

## Who it suits

- A company blog that publishes several posts a week and wants the newest ones easy to scan.
- A blog that grows a mailing list: the sign-up form sits at the top of the front page and after each post.
- A blog with one call to action: Harbor shows it in the header, in each post under the author's name, and on the front page.
- Long how-to posts with tables, tips and downloads.

It suits a personal essay blog less well. For that, a calmer template such as classic reads better.

## What the front page shows

1. The blog's name, large, and its intro on a tint of your accent colour across the window, with the newsletter form beside them on wide screens and under them on phones.
2. **Featured:** the newest post with `featured: true`, large on a soft panel, with its picture beside the words. Under it, a row of up to three more featured posts as picture cards.
3. **Latest:** the newest posts as rows, each with a small picture beside its title, author and date. On wide screens the rows sit in two columns. A post with no picture gets a drawn cover in tints of your accent colour, here and on every card, so they line up.
4. Numbered page links under the list.
5. Your call to action as a band.

A post shows once in the featured slot and not again under Latest. A featured post can show both among the three featured cards and under Latest, so a reader who scans by date still finds it.

## What a post shows

1. The trail from the home page to the post (breadcrumbs).
2. The title, the author with a square photo, and one date: the date the post was last updated, or the date it was published if it was never updated.
3. An offer card with your call to action, under the author's name.
4. The lead picture, wider than the text on large screens.
5. An open contents box, without numbers.
6. The post. A small wave in your accent colour sits above each section heading. Tips, notes, long tables that fold and gated downloads have Harbor's look.
7. The post's first tag and the share links on one line.
8. The newsletter form.
9. Up to ten related posts in one row of small picture cards that scrolls sideways. It needs no script: swipe on a phone, or scroll with a trackpad or the arrow keys.

Harbor has no author box after the post and no reading time; turn either on in `[reader]` if you want them.

## Your brand

Harbor brings its own colours: a berry accent (`#b0225a`) on white with a pink-tinted surface. Your colours, fonts and logo in `theme/tokens.toml`, such as the ones theme sync writes, replace them, so it looks like your main site. Buttons use the accent colour with black or white text, whichever reads better on it. Cards, the offer and the gated downloads use the surface and border colours.

Harbor's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Harbor's default |
|---|---|
| `[type] body` | `1.0625rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.25rem` |
| `[type] h2` | `1.625rem` to `2.125rem` |
| `[type] h3` | `1.3125rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `800` |
| `[shape] radius` | `0.5rem` |
| `[shape] measure` | `70ch`, the width of a post's text |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"row"` | The shape of the cards under Latest and on every list: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `false` | Each card shows how long the post takes to read. |
| `featured` | `true` | The newest featured post shows large at the top of the front page and each list. |
| `picks` | `3` | How many more featured posts show in the row under it, from `0` to `8`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related` | `"scroll"` | How related posts are laid out: `scroll`, `grid`, `list`, `row` or `links`. |
| `related_count` | `10` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered`, or `nested` for 1, 1.1. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["linkedin", "x", "facebook", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. On the front page it sits beside the blog's name. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a button. |
| `offer_card` | `true` | Each post shows your call to action as an offer card under the author's name. |
| `section_marks` | `true` | A small wave in your accent colour sits above each section heading of a post. |

The call to action comes from `[template.cta]`; see [the templates guide](../templates.md#your-call-to-action). Without it, the header button, the offer card and the band do not show.

For example, a blog with no offer and fewer related posts:

```toml
[template]
name = "harbor"
[template.options]
offer_card = false
related_count = 5
```

## Good to know

- Everything reads with scripts off. Harbor adds no script.
- The newsletter form shows only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table. It signs readers up to your default list.
- The front page shows the form once, at the top. One form per page keeps the page simple and every field label unique.
- A post marks no category above its title yet, because posts do not carry their category to the page. The breadcrumbs name it.
