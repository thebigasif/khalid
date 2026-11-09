---
title: "Lattice"
order: 85
---

# Lattice

Lattice is a template for a science explainer site. Each post explains one idea, such as why the sky is blue or how tides work. Every picture is a numbered figure, every source a footnote and every hard word a link to your glossary.

```toml
[template]
name = "lattice"
```

## Who it suits

- A science writer, a museum, a school or a science centre that explains how the world works in plain words.
- A team that wants every claim to carry a source, and every term to have a plain definition.
- Readers who arrive with one question and want the answer, the numbers behind it and the words to go further.

It suits a research lab less well, since a lab lists its papers: Research reads better there. For step-by-step guides, Compass reads better.

## What you write

Lattice needs nothing new in your posts. It reads what Khalid already has:

- **The description** shows under the title as the answer in one line.
- **Key numbers** are the post's `facts`, up to four. Write the number as the value and say what it is in the label:

```yaml
facts:
  - label: Wavelength of blue light
    value: "450 nm"
  - label: Wavelength of red light
    value: "700 nm"
```

- **Figures** are the figure block, a picture with its caption. Lattice numbers each one. The lead picture is figure 01, and the figures in the text follow, 02, 03 and on.

```markdown
:::figure{src=sunset.jpg alt="An orange sky over a dark headland at dusk"}
At sunset, light crosses far more air, so most of the blue is scattered away before it reaches you.
:::
```

- **Footnotes** are markdown footnotes. Write `[^1]` after a claim and `[^1]: The source.` at the end of the post. Lattice shows each mark as a small numbered tab and the notes as a ruled panel under the text.
- **Glossary terms** are your glossary (see [Pages that help readers find their way](../discovery.md)). Link a term with `[[glossary:wavelength]]`, and Lattice marks it with a dotted underline in the accent colour. Put a link to `/glossary/` in your menu so readers can find the whole list.
- **Topics** are categories, and tags are a post's subjects.

## What the front page shows

1. Your site's name, large, and the `tagline` under it. Up to four numbers from `hero_stats` show under it when you write them.
2. **The lead explainer:** your newest featured post, or else your newest post, as a large figure with its topic, title, summary and key numbers.
3. **Topics:** every category as a numbered tile.
4. **Latest explainers:** the other posts as a grid of numbered figures, each with its topic, title and summary.
5. Numbered page links, your call to action as a band, and the sign-up form when you set `newsletter` to `home` or `both`.

## What a post shows

1. The trail from the home page, then the topic as a small label.
2. The title, the description as the answer, the author, the date and the reading time.
3. The lead picture as figure 01, on its grid.
4. **Key numbers:** the facts as a row of large numbers.
5. The text in one column of about 65 characters a line, with numbered figures, footnote tabs and marked glossary terms. Long posts get a folded contents list.
6. The notes, the subjects (tags), the share links and the author box.
7. Three related explainers as figures.

## Other pages

- **Lists** (a topic, a tag, the archive): the list's name and intro, then the posts as numbered figures.
- **The glossary:** the letters as square tabs, then each term as a dictionary entry on a rule.
- **A glossary term:** its definition in the reading column, the related terms and the explainers that use it.

## Your brand

Lattice brings its own colours: ink on white, a cool grey grid behind each figure, a blue link and a burnt orange accent (`#b5400f`) for the numbers. Your colours, fonts and logo in `theme/tokens.toml` replace them. A clean sans for headings and a serif made for screens for the text suit it best, with a mono font for the numbers.

Lattice's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Lattice's default |
|---|---|
| `[fonts] heading` | Helvetica Neue, then Arial and the system's sans |
| `[fonts] body` | Georgia, then other serifs |
| `[fonts] mono` | The system's own mono |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.5rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.25rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.65` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0px`, square |
| `[shape] measure` | `36rem` |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the site's name on the front page. |
| `hero_stats` | none | Up to four numbers under the site's name, each written `number \| what it counts`. |
| `key_numbers` | `true` | A post's facts show as key numbers under its lead picture and under the lead explainer. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"numbered"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "facebook", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"post"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
