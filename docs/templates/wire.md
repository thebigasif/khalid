# Wire

Wire is a template for a news and columns site: a blog that publishes often, has several writers, and wants readers to see at a glance what is new and who wrote it.

```toml
[template]
name = "wire"
```

## Who it suits

- A newsroom-style blog with daily or weekly posts on many topics.
- A team blog with several regular writers, each with a role such as "Editor" or "Columnist".
- Opinion and analysis posts, where the subtitle and the writer matter as much as the title.
- Readers who scan: the front page shows many stories at once, each with its date in plain sight.

It suits a blog with a few long guides less well. For that, Harbor or classic reads better.

## What the front page shows

1. **The lead story:** the newest post with `featured: true`, large, with its picture, topic, subtitle and writer.
2. **Latest posts beside it:** the next four posts, each led by its date and reading time, with its title, one line of its summary and its writer. On phones this list sits under the lead story.
3. **More posts:** the next four posts in a row, each with its picture, topic, title, summary, writer and date.
4. **The feed:** every other post as a row. With room, each row's date stands in its own column on the start side and its picture on the end side. The newsletter form sits once inside the feed, after the eighth post on the page counting the lists above it (option `newsletter_after`), or after the feed when it is shorter.
5. **A rail beside the feed:** "Editor's picks", your other featured posts as a numbered list, then the topic links.
6. Numbered page links, then each home section as a row of four.

The blog's name stays in the page for search engines and screen readers, and the logo says it on screen, so the stories come first. A blog with an `intro` on its home page shows the name and the intro above the lead story.

There is no view count anywhere. Wire's numbered list is the editor's picks, not the most read posts.

## What a post shows

1. The trail from the home page to the post (breadcrumbs), its topic in your link colour.
2. The title, large, and its subtitle, from the post's `description`.
3. The writer's row: a round photo, their name and role, the date, when it was updated, the reading time and who reviewed it.
4. The lead picture across the page, with its caption as the credit line under it. Write the credit in the picture's `caption`.
5. The post in a reading column, with the writer's card beside it on a wide screen: photo, name, role, a short bio that folds open, and their profile links. On a phone the card sits above the post.
6. Inside the post: subheads in the heading font, links underlined in your accent colour, the contents box under a heavy rule in two columns, a summary under an accent rule, pull quotes between two heavy rules, numbers in a stat callout set large, and "see also" lines (`:::callout{kind=read}`) quietly between two hairlines.
7. The post's tags.
8. The author box with the full bio.
9. The newsletter form and the call to action band, when they are on.
10. Three related posts in a row of three, each with its picture.

Wire has no share links by default.

## List pages

A topic, tag or archive page shows its name large, its featured post as a lead with the picture beside it, then the feed as on the front page.

## Your brand

Wire takes your colours, fonts and logo from `theme/tokens.toml`, so it looks like your main site. Section titles sit under a heavy rule in your accent colour, the current menu item is underlined in it, and buttons use it with black or white text, whichever reads better. Headlines, labels and the menu use your heading font; posts and every subtitle use your body font.

Wire's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Wire's default |
|---|---|
| `[colors]` | white page, near-black text, a vermilion accent `#d92e17` and link colour `#c22a16` |
| `[fonts] heading` | the reader's own sans: San Francisco, Segoe UI or Roboto |
| `[fonts] body` | the reader's own serif: Charter, Sitka, Cambria, Noto Serif or Georgia |
| `[type] body` | `1.0625rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.5rem` |
| `[type] h2` | `1.5rem` to `2rem` |
| `[type] line_height` | `1.65` |
| `[space] section` | `4rem` |
| `[shape] radius` | `0.125rem`; pictures keep square corners |
| `[shape] measure` | `40rem`, the width of a post's text |

Nothing is downloaded for the fonts: each reader sees the sans and serif their own device has.

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its `description` as a subtitle under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"row"` | The shape of the cards in the feed and on every list: `row`, `image`, `text`, `compact` or `ranked`. |
| `card_reading_time` | `true` | Each card shows how long the post takes to read. |
| `card_author_role` | `false` | Each card shows its writer's role after their name. |
| `featured` | `true` | The newest featured post is the lead story on the front page and tops each list. |
| `picks` | `5` | How many featured posts the "Editor's picks" list in the rail shows, from `0` to `8`. At `0` the list goes. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related` | `"row"` | How related posts are laid out: `row`, `grid`, `list`, `scroll` or `links`. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `contents` | `"none"` | Where a long post's contents go: `none`, `inline`, `inline-folded`, `side-end` or `side-start`. |
| `contents_numbers` | `"numbered"` | How the contents are numbered: `numbered`, `nested` for 1, 1.1, or `none`. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `newsletter_after` | `8` | After how many posts in the front page's list the newsletter form shows, from `2` to `20`. |
| `cta_band` | `"none"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `author_card` | `true` | A post shows the writer's card, with a bio that folds open, right after the picture. |

The call to action comes from `[template.cta]`; see [the templates guide](../templates.md#your-call-to-action).

For example, a site that wants its call to action in the header and the newsletter form higher up:

```toml
[template]
name = "wire"
[template.options]
cta_button = true
newsletter_after = 4
```

## Good to know

- Everything reads with scripts off. Wire adds no script.
- The newsletter form shows only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table. It shows once per page, so every field label stays unique.
- A writer's role comes from `role` in their file under `content/<language>/authors/`.
