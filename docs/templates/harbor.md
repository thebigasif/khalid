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

1. The blog's name, large. With sign-ups on, the newsletter form sits beside it in a raised card, on a light tint of your accent colour across the window; on phones the form goes under the name.
2. **Featured:** the newest post with `featured: true`, large, with its picture above its topic, title and summary. Beside it on wide screens, and under it on phones, a column of smaller posts as rows with a picture: **Editor's picks** when two or more other posts are featured, or else the four newest posts under **Latest posts**.
3. A grid of picture cards: the topic and date, the title, a short summary and the author at the foot of each card. Under **Latest posts**, or **More posts** when the newest four sit beside the featured post. A post with no picture gets a drawn cover in tints of your accent colour, so the cards line up.
4. Numbered page links under the grid.
5. Your call to action as a wide band, with the note large beside its button.

A post shows once in the featured slot and not again lower down, and a post in the column beside it does not show again in the grid. An editor's pick can show both beside the featured post and in the grid, so a reader who scans by date still finds it.

## What a post shows

1. The trail from the home page to the post's topic (breadcrumbs), centred.
2. A centred title block: the title, the author with a round photo and their role, who reviewed it, one date (the date the post was last updated, or the date it was published if it was never updated) and the reading time.
3. An offer card with your call to action, under the author's name: one line and one button in a light rounded box.
4. The lead picture, wider than the text on large screens.
5. An open contents box, without numbers, in two columns on wide screens.
6. The post, in one column about 42rem wide. A short bar in your accent colour sits above each section heading. Tips, notes, warnings, figures, quotes, tables, code, long tables that fold and gated downloads have Harbor's look.
7. The post's first tag and the share links on one line.
8. The author box: the author's photo, name, role and bio in a card.
9. The newsletter form, or your call to action as a band.
10. Up to ten related posts in one row of picture cards that scrolls sideways. It needs no script: swipe on a phone, or scroll with a trackpad or the arrow keys.

Turn the author box or the reading time off in `[reader]` if you do not want them.

## Your brand

Harbor brings its own colours: a deep harbour blue accent (`#383dd0`) on white with a cool mist surface. Your colours, fonts and logo in `theme/tokens.toml`, such as the ones theme sync writes, replace them, so it looks like your main site. Buttons are pills in the accent colour with black or white text, whichever reads better on it. The bands, the offer, the contents and the tips use light tints of the accent; cards use the border colour. Every section is light: Harbor has no dark band anywhere.

Harbor's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Harbor's default |
|---|---|
| `[type] body` | `1.0625rem` on a phone to `1.125rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.25rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.25rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.65` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0.75rem`; cards and boxes use one and a half times it |
| `[shape] measure` | `42rem`, the width of a post's text |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards in the front page's grid and on every list: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `false` | Each card shows how long the post takes to read. |
| `featured` | `true` | The newest featured post shows large at the top of the front page and each list. |
| `picks` | `3` | How many more featured posts show in the column beside it, from `0` to `8`. With fewer than two, the column shows the newest posts. |
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
| `section_marks` | `true` | A short bar in your accent colour sits above each section heading of a post. |

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
