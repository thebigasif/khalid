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
