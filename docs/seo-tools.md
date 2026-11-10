---
title: "The admin's SEO tools"
order: 60
---

# The admin's SEO tools

The admin carries the tools the Yoast SEO plugin gives a WordPress site,
where they apply to Khalid. The settings themselves are in
[seo-settings.md](seo-settings.md); this page lists the screens that work
on posts.

## Dashboard: Fix these first

The dashboard lists the checks that fail on most posts, the ones that stop
a post going live first, from `GET /api/admin/v1/lint/issues/`. Each task
says what is wrong, why it matters and the fix, how many posts it names,
and opens the first of them. A blog that cannot send the list leaves the
panel out.

## Structured data that blocks a post

Structured data is the JSON-LD a page carries so search engines can show
it as a rich result, such as a recipe card. The publish check (`khalid
lint`, `khalid publish` and the Admin API's publish route) reads the same
graph `khalid build` writes for the post: its article and author, the
schema of its blocks, its collection's schema template and
the JSON-LD in its `schema` field. When a node lacks a property Google
needs, the post does not go live, and the finding names both, such as
`Event needs startDate` or `Recipe needs image`. A property Google only
recommends never stops a post.

Most blocks refuse to save without what Google needs, so the gap comes
from the `schema` field, a schema template, or a recipe block with no
`image`.

## Posts list

Each row shows the post's SEO and readability lights, how many posts link
to it and how many it links to. The SEO filter shows posts with no focus
keyword, orphans (no other post links to them) and the main posts of topic
clusters. The list API also filters by `cluster:member` and
`cluster:pillar`, and each row carries the post's `path` on the site.

## Moving live posts to the trash

Ticking live posts and choosing "Move to the trash" asks what their old
addresses should do:

- **Say the page is gone for good**, the default: a 410 answer, so search
  engines drop the address quickly.
- **Send readers to another post**: a 301 redirect to a live post picked
  from a list, which carries the old address's standing over.

The answer is stored as a redirect under Redirects once the post is in the
trash. Putting the post back from the trash keeps the redirect; remove it
under Redirects.

A post waits in the trash for 30 days, then `khalid serve` deletes it for
good. It looks at start and then once an hour. To keep posts longer or
shorter, set the number of days in `site.toml`, up to 3650:

```toml
trash_days = 90
```

`trash_days = 0` keeps every deleted post until you delete it yourself.

## Bulk SEO

One table edits the SEO title, the description and the focus keyword of
twenty posts a page. Each field shows its length against what Google
shows (about 60 characters for a title, 160 for a description). Each row
saves on its own, sending only the fields that changed. The filter shows
only posts with no SEO title, no description or no focus keyword.

## SEO workouts

Two jobs, one step at a time:

- **Orphaned posts**: each live post no other post links to, with the
  three posts closest to it by category, tags, topic cluster and keywords.
  "Link from" adds a link to the orphan in that post.
- **Main posts**: each post of a topic cluster that does not link to the
  cluster's main post yet. "Link to" adds that link.

A link goes on words of the text that name the target when there are some
outside a link, or else on a line `Related: [title](path).` at the end.
Open the post to move it where it reads best.

## Telling search engines what changed (IndexNow)

IndexNow is a shared way to tell Bing, Yandex, Seznam, Naver and Yep
which pages changed, so they read them soon. Switch it on with a key in
`site.toml`:

```toml
[seo.indexnow]
key = "3f9c0b5e2a8d4c71"
```

Khalid then sends the address of each live post that is new, or whose
`updated:` date is new or later. Any other edit, such as a typo fix, sends
nothing. Each change is sent once:

- `khalid serve` sends a change as soon as the file is saved. When it
  starts, it also sends what changed while it was stopped, such as a post
  edited by hand or arriving through `git pull`.
- `khalid build` sends what changed since the last build, and says how
  many addresses it sent. `khalid build --no-indexnow` sends nothing; the
  next build sends those changes. A build sends nothing when the site's
  `address` is not public, such as `localhost`, because search engines
  cannot read pages there.

Khalid keeps the list of what search engines heard in
`data/indexnow-sent.txt`. The first run with IndexNow on sends nothing and
starts that list, so a new site does not send every address it has.
Deleting the file is safe: the next run sends nothing and starts again.
Requests go 10,000 addresses at a time, the most IndexNow takes in one.
Google does not use IndexNow; it reads the sitemap.
