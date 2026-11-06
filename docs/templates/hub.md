---
title: "Hub"
order: 55
---

# Hub

Hub is a template for a creator's link-in-bio home. It is the one link you put in every profile: your photo, your name and a stack of large link buttons, then your latest posts, all in one narrow column.

```toml
[template]
name = "hub"
```

## Who it suits

- A video maker, podcaster, illustrator or maker who sends people from social profiles to one address.
- A creator who sells something, such as a shop, a course or a workshop, and wants that link first.
- A creator who also writes: notes for each video, studio news or how-to posts.

It suits a magazine, a news site or a team blog less well. For one writer with long essays, Solo reads better.

## Your links

Write your links under `[template.options]`, one line each, up to eight. Each line is the words on the button, a `|`, and the address. Add a second `|` and a short note to show a line under the words.

```toml
[template.options]
creator = "tamsin-hale"
tagline = ["Potter in Bristol. I make mugs and bowls, and a pottery video every other Wednesday."]
links = [
  "Watch my pottery videos | https://www.example.com/videos | A new video every other Wednesday",
  "Shop mugs and bowls | https://www.example.com/shop",
  "My glaze recipes | /blog/mixing-my-own-glazes/",
]
hero_stats = ["48k | subscribers", "120 | videos"]
```

Without `links`, your main menu fills the stack. Your call to action from `[template.cta]` leads the stack, filled with your accent colour, and its `note` shows under its words.

`creator` is the slug of the author whose photo leads the front page, such as the name of their folder in `content/en/authors/`. Without it, the newest post's author leads.

## What the front page shows

1. Your round photo in an ink ring, your site's name large, and your `tagline` under it. When the front page has an intro of its own, the intro shows instead.
2. Up to four numbers in a row from `hero_stats`, such as subscribers and videos.
3. **The link stack:** your call to action first, then each link as a large button with a hard edge and a shadow like a stack of cards.
4. The newsletter form, when your site takes sign-ups.
5. **Latest posts:** each post as a row with a square picture, the date and category, the title and the summary.
6. Numbered page links when there are more posts.

When `[[reader.home_sections]]` lists sections, they take the place of the latest posts, as on every front page.

## What a post shows

1. An "All posts" link back to the front page, then the category as a small label.
2. The title, the post's description as a summary, your photo, name and role, the date and the reading time.
3. The lead picture with the same edge and corners as the link buttons.
4. The post in the same narrow column, with a folded contents list for long posts.
5. The tags, the share links and the author box.
6. **More from you:** your links again, under a heading such as "More from Tamsin Hale". Turn this off with `post_links = false`.
7. Related posts as rows with a square picture.

## Other pages

Lists (a category, a tag, the archive) and author pages keep the same narrow column, with posts as the same rows.

Everything reads with scripts off.

## Your brand

Hub brings its own colours: a raspberry accent (`#a3304e`) on a warm off-white page, with near-black ink for the edges of the buttons and pictures. Your colours, fonts and logo in `theme/tokens.toml` replace them. A friendly sans for headings suits it best, since your name and the button words are set in the heading font.

Hub's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Hub's default |
|---|---|
| `[fonts] heading` | The system's rounded sans, then its plain sans |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.0625rem` on a phone to `1.125rem` on a wide screen |
| `[type] h1` | `1.875rem` to `2.5rem` |
| `[type] h2` | `1.375rem` to `1.625rem` |
| `[type] h3` | `1.125rem` to `1.25rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.65` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0.875rem`, softly rounded |
| `[shape] measure` | `36rem`, the one column every page uses |
| `[space] section` | `3.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `links` | none | Up to eight link buttons, each written `label \| address` or `label \| address \| note`. Without them, the main menu fills the stack. |
| `creator` | empty | The slug of the author whose photo leads the front page. Empty shows the newest post's author. |
| `tagline` | none | One line under your name on the front page. |
| `cta_tile` | `true` | Your call to action leads the link stack, filled with the accent colour. |
| `post_links` | `true` | A post ends with your link buttons again. |
| `hero_stats` | none | Up to four numbers under your name, each written `number \| what it counts`. |
| `logo` | `false` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `back_link` | `true` | A post shows an "All posts" link back to the front page above its title. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"row"` | The shape of the posts in lists: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "bluesky", "whatsapp", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"none"` | Where a band with your call to action shows as well: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
