---
title: "Quill"
order: 85
---

# Quill

Quill is a template for a minimal text blog. It shows your name, a list of your posts and each post in large, calm type, and nothing else. It has the fewest parts of any template and the smallest stylesheet, so its pages load fastest.

```toml
[template]
name = "quill"
```

## Who it suits

- A writer, essayist or diarist who publishes words, not pictures.
- Someone who keeps a public notebook of short notes, reading notes or letters.
- Anyone who wants a blog that loads at once on a slow phone.

It suits a magazine, a shop's blog or a site led by pictures less well.

## What the front page shows

1. The header: your site's name in italic after a small mark like a pen's nib, and your menu in quiet small type.
2. Your site's name large, in regular weight.
3. **Your posts:** each one a single line between thin rules, the title and then the date at the end of the line. On a phone the date drops under the title. Lists show no pictures, summaries or authors.
4. Links to newer and older pages when there are more posts.

When `[[reader.home_sections]]` lists sections, they take the place of the list, as on every front page.

## What a post shows

1. The title large, with the post's description under it in italic.
2. One quiet line with the author, the date, the reading time and the category.
3. The lead picture, when the post has one.
4. The text in one column of about 66 characters a line, at 18 px on a phone and 21 px on a wide screen, with a folded contents list for long posts.
5. **The nib** in the middle under the last line, to show the post has ended.
6. The tags, the author box and related posts as plain links.

## Other pages

Lists (a category, a tag, the archive) and author pages use the same one-line list under their own title.

Everything reads with scripts off.

## Your brand

Quill brings its own colours: blue-black ink (`#234a85`) for links and the nib on a paper-white page. It uses the system's book serif, such as Charter or Georgia, so it loads no font at all. Your colours, fonts and logo in `theme/tokens.toml` replace them. One reading serif for both headings and text suits it best.

Quill's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Quill's default |
|---|---|
| `[fonts] heading` | The system's book serif |
| `[fonts] body` | The system's book serif |
| `[type] body` | `1.125rem` on a phone to `1.3125rem` on a wide screen |
| `[type] h1` | `2.125rem` to `3.25rem` |
| `[type] h2` | `1.375rem` to `1.75rem` |
| `[type] h3` | `1.1875rem` to `1.4375rem` |
| `[type] small` | `0.9375rem` |
| `[type] line_height` | `1.7` |
| `[type] heading_weight` | `500` |
| `[shape] radius` | `0px`, square corners |
| `[shape] measure` | `40rem`, the one column every page uses |
| `[space] section` | `4rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `logo` | `false` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"compact"` | The shape of the posts in lists: `compact` (one line), `image`, `text`, `row` or `ranked`. |
| `pagination` | `"pages"` | `pages` shows only newer and older; `numbered` links to each page near this one. |
| `related` | `"links"` | How the related posts under a post are laid out: `links`, `list`, `grid`, `row` or `scroll`. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `reading_progress` | `"bar"` | A thin line at the top of a post that fills as you read: `bar` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `share` | `"none"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"post"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"none"` | Where a band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `hero_stats` | none | Up to four numbers under your name, each written `number \| what it counts`. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
