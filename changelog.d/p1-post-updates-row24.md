Added
- A post can list dated notes of what changed under `updates:` in its front matter, each with a `date` and a one-sentence `note`. The default post template shows them after the post as "Updates", newest first.
- The newest date in that list counts as the post's last real update, like `updated:`. It moves the visible updated date, `dateModified` in the structured data, the sitemap's `lastmod`, IndexNow and the linter's stale-post rule.
