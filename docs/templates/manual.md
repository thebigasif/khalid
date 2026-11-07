---
title: "Manual"
order: 86
---

# Manual

Manual is a template for a help centre or product documentation. A reader arrives with a question, finds the answer by search or by topic, reads it in numbered sections, and knows where to go next. It reads like a help centre, not a blog.

```toml
[template]
name = "manual"
```

## Who it suits

- A company that answers its customers' questions in a help centre, such as how to set up, charge or return a product.
- A software team that writes its product documentation as short how-to articles.
- A club, school or council that keeps a set of "how do I" pages for its members.

It suits long essays less well. For long guides that teach a subject, Almanac or Compass reads better.

## Articles, topics and guides

Manual needs nothing new in your posts. It reads what Khalid already has.

**An article is a post.** Give it a title that says the task or the question, such as "Charge the battery", and a description that answers it in one line.

**A topic is a category.** Topics follow the order of your `categories.toml`, and the front page numbers them 01, 02, 03 in that order. A category's intro shows at the top of its topic page.

**A guide is a series.** When a reader should follow some articles in order, such as the three steps of setting up, give them the same series name and their place as `order`:

```yaml
---
title: "Set your saddle and handlebar height"
description: "Get a comfortable riding position in ten minutes with one hex key."
category: getting-started
series:
  name: Set up your new bike
  order: 2
facts:
  - label: Time
    value: "10 minutes"
  - label: You need
    value: "5 mm hex key"
  - label: Applies to
    value: "Every model"
---
```

The article then shows "Step 2 of 3" and lists every step of the guide beside the text. See [Series](../discovery.md#series).

**Facts are "Before you start".** Up to four facts show in a box under the title, such as the time a task takes, the tools and the models it applies to. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

**Popular articles are featured posts.** Mark a post `featured: true` and it shows as a popular article on the front page.

## What the front page shows

1. "Help centre" as a small label, your site's title large and the `tagline` under it, on a pale panel.
2. **A wide search field,** when your site answers searches (`khalid serve`). A static site has no search page, so the field does not show.
3. Up to four popular articles as quick links under it.
4. **Browse by topic:** a tile for each topic with its number, its name, its four newest articles and a link to every article in it. The front page shows up to six topics.
5. **Popular articles** in two columns, each with its topic and its description. `popular_count` sets how many.
6. **Recently updated:** the newest articles with their dates. `updates_count` sets how many.
7. Your call to action as a band, such as "Contact us".

## What an article shows

1. The trail from the home page to the article's topic.
2. The topic as a small label, and "Step 2 of 3" when the article is part of a guide.
3. The title, the description as a summary, the date it was last updated, the reading time and who wrote it.
4. **Before you start:** the article's facts in a box.
5. The lead picture, then the text at 18px in a column of about 36rem, near 70 characters a line.
6. **Numbered sections.** Each main heading is numbered 1, 2, 3 and each heading under it 2.1, 2.2. On a wide screen the numbers stand in the margin before each heading. The contents carry the same numbers, so "see 2.1" means the same in both places.
7. **The next steps panel:** "Did this answer your question?" with two halves. "Yes, it did" links to the next step of the guide, or the next article to read. "Not quite" links to the search page, every article in the topic and your call to action. It is plain links, with no script and no form.
8. The tags, the author box, and related articles.

On a wide screen the article stands in three columns: the guide's steps (or related articles) at the start, the text in the middle, and "On this page" at the end, which stays in view as the reader scrolls. On a phone the columns stack, and the contents fold into one box under the title.

## Other pages

- **A topic** (a category page): "Topic" over its name, its intro and how many articles it holds, then its articles as rows with their descriptions.
- **A guide** (the series page): "Guide" over its name, then its articles as numbered steps.
- **Other lists** (a tag, the archive): the same rows, each with its topic.
- **Authors, an author and search:** the shared pages, in Manual's type and colours.

## Your brand

Manual brings its own colours: a calm cobalt (`#1a56b8`) on white, with a pale blue surface for the search panel and the "Before you start" box. Section numbers are set in a monospace in your accent colour. Your colours, fonts and logo in `theme/tokens.toml` replace them. A clear sans suits it best, since readers scan for the step they need.

Manual's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Manual's default |
|---|---|
| `[fonts] heading` | The system's own sans |
| `[fonts] body` | The system's own sans |
| `[fonts] mono` | The system's own monospace, for section numbers |
| `[type] body` | `1.0625rem` on a phone to `1.125rem` on a wide screen |
| `[type] h1` | `2rem` to `2.75rem` |
| `[type] h2` | `1.375rem` to `1.625rem` |
| `[type] h3` | `1.1875rem` to `1.3125rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0.375rem` |
| `[shape] measure` | `38rem` |
| `[space] section` | `4rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the help centre's name on the front page, such as what it helps with. |
| `popular_count` | `6` | How many popular articles (featured posts) show on the front page, from `0` to `8`. The first four also show under the search field. |
| `updates_count` | `6` | How many recently updated articles show on the front page, from `1` to `20`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | An article shows its description under its title. |
| `updated_note` | `false` | An updated article says why it was updated, from its `updated_note`. |
| `cards` | `"text"` | The shape of the cards on shared pages such as an author's page: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `5` | How many related articles show, from `3` to `10`. |
| `contents` | `"side-end"` | Where an article's contents go: `side-end`, `side-start` (the guide's steps then move to the end), `inline`, `inline-folded` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"nested"` | How the contents and the headings are numbered: `nested` for 1 and 1.1, `numbered` for the main headings only, or `none`. |
| `share` | `"none"` | Where an article shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["email", "linkedin"]` | The networks the share links go to, in order. |
| `newsletter` | `"none"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |

Everything reads with scripts off. Manual adds no script.
