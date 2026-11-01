# Example: a local business blog

This is the blog of Larch Lane Cycles, a bike repair workshop in Norwich. The business is made up. The repair advice is real.

## Who it is for

A shop, a workshop, a clinic or any business with a door people walk through. The blog answers the questions customers ask at the counter. That helps people nearby find the business when they search, and it shows them the business knows its trade.

## What it shows

| Feature | Where to look |
|---|---|
| Opening hours and address in search results | `site.toml`, the `[[seo.locations]]` table and its hours |
| Address and phone on a page | `content/en/pages/about.md` and `visit.md`, the `::business` block |
| Step-by-step guides that search engines can read as steps | every post with a `::::howto` block, such as `fix-flat-bike-tyre` |
| Questions and answers that search engines can read as an FAQ | `content/en/pages/questions.md` and the bike service post, the `::::faq` block |
| Key points at the top of each guide | the `summary` callout under each post's first paragraph |
| Warnings and tips | the `warning` and `tip` callouts |
| An opening hours table that reads well on a phone | `content/en/pages/visit.md` |
| Two authors with their roles | `content/en/authors/` |

It uses the **Compass** template, made for teaching blogs: each guide opens with its key points, then the steps, then the questions.

## Try it

```
khalid --site fixtures/examples/local-business lint
khalid --site fixtures/examples/local-business build --out public
```

The site needs the Compass template. Until Compass is built into your copy of Khalid, change `name = "compass"` to `name = "classic"` in `site.toml` to try it.

## Make it yours

Change the name, address, phone and opening hours in `site.toml` first. Search engines show those details, so they must be right. Then replace the posts with the questions your own customers ask.
