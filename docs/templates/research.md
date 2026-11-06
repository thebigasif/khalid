---
title: "Research"
order: 85
---

# Research

Research is a template for a research lab or an academic group. It shows your papers, the people who wrote them and the topics they belong to, and it reads like a lab's website, not a blog.

```toml
[template]
name = "research"
```

## Who it suits

- A university lab, research group or institute that writes a short summary of each paper it publishes.
- A group that also posts news, such as a new member, a grant, a field season or an open position.
- A single researcher who keeps a publication list with a note on each paper.

It suits a magazine or a company blog less well. For those, Atlas or Keystone read better.

## Papers are posts with facts

Research needs nothing new in your posts. A post with `facts` is a paper. A post without facts is news. Put the facts in this order, since Research reads them by their place, not their label:

```yaml
---
title: "Seagrass meadows recover faster when grazing turtles return"
description: "Meadows grazed lightly by turtles regrew after storms about a third faster."
facts:
  - label: Journal
    value: "Marine Habitat Review"
  - label: Year
    value: "2026"
  - label: DOI
    value: "10.5555/tidemark.2026.014"
  - label: Citations
    value: "3"
---
```

1. **The venue:** the journal, conference or book.
2. **The year.** Without it, Research uses the year of the post's date.
3. **A DOI or a link.** A DOI, such as `10.5555/tidemark.2026.014`, links to doi.org. A link starting with `https://` links to itself.
4. **The citations,** or any other short fact.

Your labels show on the page, so write "Conference" or "Preprint" when that fits better. See [Facts on a post](../templates.md#facts-on-a-post) for the limits. Each post has one author, so the author is the paper's lead author or the member who wrote the summary.

Categories are your projects or research areas, and tags are a paper's keywords. Give each category an intro in `categories.toml`, and it shows at the top of that project's page.

## What the front page shows

1. Your lab's name, large, and the `tagline` under it.
2. A row of up to four large numbers from `hero_stats`, such as papers, people and grants. Nothing shows until you write them.
3. **Recent publications:** your newest papers as a numbered reference list. Each entry hangs its number, such as [3], in the margin, then shows the title, the author, the venue in small capitals with the year, and the DOI and citations under it.
4. **News** beside it on wide screens and under it on phones: the newest posts without facts, each with its date and category. `news_count` sets how many.
5. **Research areas:** every category as a numbered index. A category whose posts on the page are all news, such as a lab news category, is left out, so news never shows as a research area.
6. Numbered page links, and your call to action as a band, such as "Open positions".

## What a post shows

1. The trail from the home page to the post's research area.
2. The research area as a small label, the title, and the post's description as a one-line summary.
3. The author with their mark and role, who reviewed it, and the date.
4. **The paper panel:** the facts in a ruled row of cells, with the DOI linked.
5. The lead picture, then the post in one column about 37.5rem wide, about 72 characters a line, with a folded contents list for long posts.
6. **Cite this:** the reference in one line, built from the author, year, title, venue and DOI, and a BibTeX entry folded under it. A reader selects it and copies it. It needs no script.
7. The keywords (tags), the share links and the author box.
8. Related posts as a short reference list.

A news post has no paper panel and no cite box.

## Other pages

- **Lists** (a category, a tag, the archive): the list's name and intro, then the posts as a reference list under a heading for each year.
- **People** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A person's page:** their photo or mark, role, experience and bio, then their papers by year, and the rest of the group.

## Your brand

Research brings its own colours: a claret accent (`#8a1c3f`) on white, with a warm paper surface for the paper panel and the cite box. Section labels are small capitals over a dark rule, as in a journal. Your colours, fonts and logo in `theme/tokens.toml` replace them. A serif for headings suits it best, since paper titles are set in the heading font.

Research's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Research's default |
|---|---|
| `[fonts] heading` | Charter, then Georgia and other serifs |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.0625rem` on a phone to `1.125rem` on a wide screen |
| `[type] h1` | `2rem` to `3rem` |
| `[type] h2` | `1.375rem` to `1.75rem` |
| `[type] h3` | `1.125rem` to `1.25rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.65` |
| `[type] heading_weight` | `600` |
| `[shape] radius` | `0.125rem`, nearly square |
| `[shape] measure` | `42rem` |
| `[space] section` | `4rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the lab's name on the front page. |
| `hero_stats` | none | Up to four numbers under the lab's name, each written `number \| what it counts`. |
| `news_count` | `5` | How many news posts show beside the publications on the front page, from `0` to `10`. |
| `cite_box` | `true` | A paper ends with the Cite this box. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"text"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `5` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"numbered"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "linkedin", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"post"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
