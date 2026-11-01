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
- Readers who scan: the front page puts the newest posts in one fast column.

It suits a blog with a few long guides less well. For that, Harbor or classic reads better.

## What the front page shows

1. The blog's name and its intro, small, so the stories come first.
2. **The lead story:** the newest post with `featured: true`, large, with its picture.
3. **Two tabs beside it:** "Latest", the five newest posts as a short list, and "Editor's picks", your featured posts as a numbered list with their reading time and date. Choosing a tab needs no script: each tab is a link, and the page shows the list it names. On phones the tabs sit under the lead story.
4. The topic links as pills, when your blog has them.
5. **All posts:** one column of rows, each with its picture, the writer's photo and name, the date, the reading time, the title and one line of its summary. The newsletter form sits once inside this list, after the eighth post (option `newsletter_after`), or after the list when it is shorter.
6. Numbered page links.

There is no view count anywhere. Wire's numbered list is the editor's picks, not the most read posts.

## What a post shows

1. The trail from the home page to the post (breadcrumbs).
2. The title and its subtitle, from the post's `description`.
3. The writer's row: a round photo, their role as a small badge, their name, the date and the reading time.
4. The lead picture, with its caption as the credit line under it. Write the credit in the picture's `caption`.
5. The writer's card: photo, name, role, a short bio that folds open, and their profile links.
6. The post. Pull quotes stand out in your heading font with a bar in your accent colour; a paragraph that starts in bold reads as a lead-in; "see also" lines (`:::callout{kind=read}`) sit quietly between two rules.
7. The post's tags as pills.
8. The author box with the full bio.
9. The newsletter form.
10. Three related posts as rows.

Wire has no contents box and no share links by default; turn the contents on with the `contents` option.

## Your brand

Wire takes your colours, fonts and logo from `theme/tokens.toml`, so it looks like your main site. A thick rule in your accent colour runs under the header, the current tab is underlined in it, and buttons use it with black or white text, whichever reads better.

Wire's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Wire's default |
|---|---|
| `[type] body` | `1.0625rem` on a phone to `1.125rem` on a wide screen |
| `[type] h1` | `2rem` to `3rem` |
| `[type] h2` | `1.5rem` to `1.75rem` |
| `[type] h4` | `1.25rem` to `1.375rem`, the size of the subtitle |
| `[type] line_height` | `1.6` |
| `[space] section` | `2.5rem` |
| `[shape] radius` | `0.25rem` |
| `[shape] measure` | `68ch`, the width of a post's text |

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
| `picks` | `5` | How many featured posts the "Editor's picks" tab lists, from `0` to `8`. At `0` the tabs go and only "Latest" shows. |
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

- Everything reads with scripts off. Wire adds no script; the tabs work with plain links and CSS.
- The newsletter form shows only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table. It shows once per page, so every field label stays unique.
- A writer's role comes from `role` in their file under `content/<language>/authors/`.
