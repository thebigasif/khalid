# Pages that help readers find their way

Khalid makes these pages from your files. You write no template and set no switch: add the files, and the pages appear in every template, with scripts on or off.

## The glossary from A to Z

Each language with at least one glossary term gets a page at `/glossary/` (or `/de/glossary/` in German). It shows a row of letters to jump to, then every term under its first letter with the first sentence of its definition. Accented letters sort with their base letter, so `Ähnlichkeit` sits under A.

To give the page its own title, description and a short intro, add `content/en/glossary/_index.md`:

```markdown
---
title: SEO glossary
description: Every SEO term we use, explained in plain words.
---

Start with the terms you meet most often.
```

Search engines read the page as a `DefinedTermSet` (a schema.org list of defined words) that names every term. The page is in the XML sitemap and on the HTML sitemap at `/sitemap/`.

## Case studies and the changelog

Put case studies in `content/en/case-studies/` and changelog entries in `content/en/changelog/`. Khalid makes:

- `/case-studies/`, a card for each study with its client, its result and its summary, newest first, and a page for each study.
- `/changelog/`, every entry under its month, newest first, with its version and its whole text, and a page for each entry.

```markdown
---
title: How a small bakery doubled its search traffic
date: 2026-10-20
client: Northfield Bakery
outcome: Search visits doubled in four months.
description: One topic cluster on sourdough, every post linked to it.
---
```

A changelog entry needs a `date` and a `title` or a `version`. `khalid lint` holds both collections to this: a case study with no `client`, `outcome` or `date` blocks under `case-study-incomplete`, which names the field to add, and a changelog entry with no `date` blocks under `changelog-entry-undated`, since the date also gives the month the entry is listed under. `khalid build` and `khalid serve` leave a blocked entry off the site until you add what it lacks, and say which rule held it back. Lower either in `lint.toml` if your site does without. Each folder may hold an `_index.md` with a title, a description and an intro, as the glossary does. Search engines read each list as an `ItemList` (a schema.org list of pages in order).

## Series

A series is a set of posts written to be read in order, such as a course in five parts. Give each part the same series name and its place:

```yaml
series:
  name: Topic clusters from scratch
  order: 2
```

Each part then shows a box at the top with "Part 2 of 5", the series' name and every part, and links to the part before and the part after at the end. The series gets its own page at `/series/topic-clusters-from-scratch/` that lists the parts in order. Parts with no `order` follow the numbered ones, oldest first.

The Content API gives each part a `series` object with its `name`, the `slug` of the series page and its `order`, so a site with its own front end can show the same.

## Hubs

A hub is the front door to one topic. Any page can be one: add a `hub` table that names the topic's category and, if you like, the posts to read first, in order.

```yaml
---
title: Technical SEO, from the ground up
hub:
  category: technical-seo
  chapters:
    - what-a-sitemap-does
    - fixing-slow-pages
---

Technical SEO is how search engines find, read and store your pages.
```

The page shows its own text as the topic's introduction, then the chapters as a numbered list, then the six newest posts in the category that are not chapters. The category's own list shows "Start here" with a link to the hub above its posts.
