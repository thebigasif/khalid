---
title: "Chapter"
order: 85
---

# Chapter

Chapter is a template for an author's own site. It shows your books, pages from them and the events where readers can meet you. It is set like the first pages of a book, not like a blog.

```toml
[template]
name = "chapter"
```

## Who it suits

- A novelist, poet or non-fiction writer with one or more books.
- An author who shares excerpts and short essays between books, and lists readings, launches and festival talks.
- A small press or an agent who keeps a site for one author.

It suits a news site or a company blog less well. For those, Wire or Keystone read better.

## Books, events and writing

Chapter needs nothing new in your posts. It sorts them by category:

- **Books** are the posts in the category whose slug is `books_category`, `books` unless you set it. The lead picture is the cover art.
- **Events** are the posts in the category whose slug is `events_category`, `events` unless you set it.
- **Excerpts** are the posts in the category whose slug is `excerpts_category`, `excerpts` unless you set it. They are set as a page of the book.
- **Writing** is every other post, such as an excerpt, an essay or news.

A book's facts are read by their place, not their label:

```yaml
---
title: "The Orchard Year, a novel"
category: books
facts:
  - label: Publisher
    value: "Fernhill Press"
  - label: Published
    value: "2026"
  - label: Length
    value: "336 pages"
  - label: Order a copy
    value: "https://www.example.com/buy/orchard-year"
---
```

1. **The publisher.**
2. **The year.** It shows under the cover on the shelf.
3. **The length or format,** such as "Essays, 212 pages".
4. **A link to order it.** It shows as a button named by its label.

An event's facts are read the same way: 1 when, written as you want it shown, such as "Thu 19 November 2026, 7 pm"; 2 the venue; 3 the town or city; 4 a link for tickets, or another short fact such as `Entry: Free, please book`. On any post, a last fact that is a link shows as a button, not in the list. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

## What the front page shows

1. Your name, large, and the `tagline` under it in italics.
2. **The new book:** your newest book with its description, its first three facts, an "About the book" button and the order link, beside its cover.
3. **I. Books:** every book as a shelf of covers, each with its title and year.
4. **II. Excerpts and essays:** every other post as a book's contents page. Each has a number, its title, a dotted leader running to its date, and its category and description under it.
5. **III. Events:** the newest events, `events_count` of them, as dated rows with when, the title, the venue and city, and the tickets link.
6. Your call to action as a band, and the newsletter form.

Each part opens like a chapter, with a Roman numeral over its name. A part with no posts is left out, and the numerals count only the parts shown.

## What a post shows

- **A book:** its cover beside the title, the description, the author, the facts in a ruled list and the order button. Then the text at a reading width.
- **An event:** the title, then the facts in a panel and the tickets button, then the lead picture and the text.
- **An excerpt:** the category, the title, the description in italics and an ornament. The text is set in the heading font and larger, with a drop cap and the first line in small capitals.
- **Other writing:** the category, the title, the description, the lead picture and the text.

Every post ends with its tags, the share links, the author box and related posts as a short contents page.

## Other pages

- **Lists** (a category, a tag, the archive): the list's name and intro, then its books as a shelf, its writing as a contents page and its events as dated rows. A list with only one of these shows it without a heading.
- **People** (`/authors/`): everyone in a grid with their mark or photo and role.
- **A person's page:** their photo or mark, role, experience and bio, then their books, writing and events.

## Your brand

Chapter brings its own colours: a rust accent (`#8a3a1c`) on a warm paper, with a double rule under the header and over the footer. Your colours, fonts and logo in `theme/tokens.toml` replace them. A book face for headings suits it best, since book titles, the event dates and excerpts are set in the heading font.

Chapter's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Chapter's default |
|---|---|
| `[fonts] heading` | Garamond, then other book faces and Georgia |
| `[fonts] body` | Gill Sans, then other humanist sans |
| `[type] body` | `1.1875rem` on a phone to `1.25rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.25rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.25rem` to `1.375rem` |
| `[type] small` | `0.9375rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `500` |
| `[shape] radius` | `0px`, square |
| `[shape] measure` | `32rem`, about 70 characters a line |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under your name on the front page. |
| `books_category` | `books` | The slug of the category that holds your books. |
| `events_category` | `events` | The slug of the category that holds your events. |
| `excerpts_category` | `excerpts` | The slug of the category whose posts are set as a page of the book. |
| `events_count` | `4` | How many events show on the front page, from `0` to `10`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"text"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `4` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
