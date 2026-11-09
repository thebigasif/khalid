---
title: "Margin"
order: 65
---

# Margin

Margin is a template for one writer's personal site. It has a short introduction, a sign-up and your essays. On a wide screen every page is a column of text with a margin beside it, split by one ruled line. The small things go in the margin: who you are, the notes beside the sign-up, each essay's date and topic, and the essay's own footnotes.

```toml
[template]
name = "margin"
```

## Who it suits

- An essayist, columnist or academic who writes for the public under their own name.
- A writer whose essays carry asides, sources and small notes they do not want to cut.
- A writer who wants readers to sign up for a letter more than anything else.

It suits a magazine with many writers, a news site or a shop less well.

## Notes in the margin

Write a note as a markdown footnote: put `[^1]` where the note belongs, and the note's text on its own line, anywhere in the post.

```markdown
The ferns are brown stubs in March and over my head by July.[^1]

[^1]: Bracken, strictly. My neighbour corrects me every summer.
```

On a wide screen the notes sit in the margin beside the text and stay in view as the reader scrolls. The note a reader jumps to from its number is marked. On a phone the notes follow the essay under a ruled line. Nothing needs a script.

## Your introduction and sign-up

```toml
[template.options]
writer = "nell-ashby"
tagline = ["I write essays about walking, weather and the small habits of an ordinary week."]
letter_notes = ["Once a month, never more", "Free, with no adverts"]

[template.cta]
label = "Get the monthly letter"
href = "https://www.example.com/letter"
note = "One letter a month with the new essay."
```

`writer` is the slug of the author whose photo, name and role sit in the margin of the introduction, such as the name of their folder in `content/en/authors/`. Without it, the newest essay's author shows.

The sign-up band shows Khalid's newsletter form when your site takes sign-ups. A static site takes none, so the band shows your call to action from `[template.cta]` instead: its `note` as the text and its `label` on the button. Up to three `letter_notes` sit in the margin beside it.

## What the front page shows

1. **Introduction:** your site's name large and your `tagline`, or the front page's own intro when it has one. Your photo, name and role sit in the margin.
2. **Sign-up band** between two hairlines, with its notes in the margin.
3. **Essays:** the newest leads with its picture across the column and the margin. Each essay is a row with its title and summary in the text, and its date, reading time and topic in the margin.
4. Numbered page links when there are more essays.

When `[[reader.home_sections]]` lists sections, they take the place of the essays, each as the same rows.

## What a post shows

1. The topic as a small label, the title and the description as a dek. The byline, the date and the reading time sit in the margin beside them.
2. The lead picture across the column and the margin.
3. The essay in a column of about 70 characters a line, with its footnotes in the margin.
4. The tags, the share links and the author box, then the newsletter form when your site takes sign-ups.
5. Related essays as rows with their own margin notes.

With `contents = "side-end"` or `"side-start"`, the contents list takes the margin and the notes follow the essay.

## Other pages

Lists (a topic, a tag, the archive) show their name and intro, then the essays as the same rows. An author's page has their name, role and bio in the text, and their photo, post count and profiles in the margin. On a phone or a tablet every page is one column, and each row's margin note becomes a dateline over its title.

## Your brand

Margin brings its own colours: warm paper (`#fbf9f4`), dark ink and a brick red accent (`#a33d2c`) for the ruled margin line, the topic labels and the note numbers. Your colours, fonts and logo in `theme/tokens.toml` replace them. A book serif with an italic suits it best, since the margin notes are set in the italic.

Margin's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Margin's default |
|---|---|
| `[fonts] heading` | Iowan Old Style, then Palatino, then Georgia |
| `[fonts] body` | The same book serif |
| `[type] body` | `1.125rem` on a phone to `1.25rem` on a wide screen |
| `[type] h1` | `2.125rem` to `2.875rem` |
| `[type] h2` | `1.5rem` to `1.75rem` |
| `[type] h3` | `1.25rem` to `1.375rem` |
| `[type] small` | `0.9375rem`, the margin notes |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `600` |
| `[shape] radius` | `0.125rem`, nearly square |
| `[shape] measure` | `52rem`, the text column and the margin together: 35rem of text, a 3rem gutter and a 14rem margin |
| `[space] section` | `3.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `writer` | empty | The slug of the author in the margin of the introduction. Empty shows the newest essay's author. |
| `tagline` | none | The short introduction under your name on the front page. |
| `letter_notes` | none | Up to three short notes in the margin beside the sign-up. |
| `logo` | `false` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"text"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many related essays show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the newsletter form shows on a site that takes sign-ups: `none`, `post`, `home` or `both`. |
| `cta_band` | `"none"` | Where a band with your call to action shows as well: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |

## The demo

The demo shows Nell Ashby, a made-up essayist on the coast, with nine essays on walking, weather, writing and home, one of them a guest essay by Owen Rees. Every essay has footnotes, so the margin shows its notes. Its photographs come from the WordPress photo directory under CC0, and its photo credits page lists each one. The sample brand sets everything in Crimson Pro, an OFL book serif, in ink blue on warm paper.
