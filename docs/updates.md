# A post's Updates list

A post can keep a list of dated notes that say what changed. Readers see
it under the post as "Updates", newest first. Search engines see the
newest date as the day the post last changed.

```yaml
---
title: What a sitemap does, and what it does not
date: 2026-09-28
updates:
  - date: 2026-11-02
    note: Refreshed the crawl numbers.
  - date: 2026-10-02
    note: Added the part on image sitemaps.
---
```

Each entry needs:

- `date`: the day the change happened, written as `YYYY-MM-DD`. Use the
  day of the real change, never the day you saved a typo fix.
- `note`: one plain sentence on what changed.

Khalid sorts the list newest first, whatever order you write it in. Two
entries on the same day are refused: join them into one note.

## What the newest date does

The post's last real update is the later of `updated:` and the newest date
in `updates:`. That one date feeds:

- the "Updated" date at the top of the post,
- `dateModified` in the page's JSON-LD (structured data that tells search
  engines about the page),
- `lastmod` in the sitemap,
- IndexNow, which tells Bing and others the page changed,
- the linter's stale-post and review rules.

`updated:` and `updated_note:` still work as before. A post can use either
or both. The Updates list is the better choice once a post has changed
more than once, because readers see the whole history.

## Templates

The default post template shows the list after the body. The theme packs
keep their own layouts and do not show it yet; their dates, JSON-LD and
sitemap still use the newest entry.
