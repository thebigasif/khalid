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
outside a link, or else on a line "Related: [title](path)." at the end.
Open the post to move it where it reads best.
