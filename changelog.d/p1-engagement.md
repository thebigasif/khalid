Added
- Charts in posts: `:::chart` draws bar, column and line charts from numbers in the post or a CSV file beside it, as a picture with no script, in your template's colours, with a table for screen readers (`docs/charts-embeds-tracking.md`).
- Embed interactive tools: `::embed` shows a page from any host you list in `[media] embed_domains` in a lazy, sandboxed frame of fixed height, and only those hosts join the Content Security Policy.
- Tracking tags: `[seo.links.tracking]` adds `utm_source`, `utm_medium` and `utm_campaign` to links that point to your own product's domains, in posts, pages and the call to action, and to no other link.
- A "preferred source on Google" button after each post and in the footer of every template, turned on with `[template.preferred_source]`.
- A newsletter page at `/newsletter/` with what readers get, the sign-up form on a server, a recent issue and an archive of past issues, each with its own page, from `content/newsletters/main/` (`docs/newsletters.md`).
