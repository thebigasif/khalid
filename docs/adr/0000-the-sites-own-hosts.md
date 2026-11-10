# 0000. The site's own hosts are its address, the www and apex pair, and `internal_domains`

**Number:** next free. **Status:** Proposed, 2026-11-09. Gap audit 2, G5 (KH-LNT-2).

## Context

The linter rule `nofollow-internal-link` warns when a link to the site's own pages carries `rel="nofollow"`. It could never fire on a real site, for three reasons:

- A post's markdown escapes raw HTML, so a writer cannot put `rel` on a link. The only `rel` words a link gets come from the link settings in `[seo.links]`.
- The link settings never add anything to a path, to the site's own host with or without `www.`, or to a host in `internal_domains`.
- The rule looked only at links the renderer marks internal, which are paths. The renderer marks every full address as external, even one on the site's own host.

So the rule could only fire where the linter and the link settings disagree about which hosts are the site's. They agreed everywhere, and `sibling_hosts` was empty by design, because one process serves one site.

The case that matters is a blog on a subdomain. A blog at `blog.example.com` with `rel = ["nofollow"]` tells Google not to follow every link to its own main site, `example.com`, and nothing told the writer.

## Decision

- **The site's own hosts** are, for the linter:
  - the host of the site's address, with or without `www.`;
  - the `www.` and apex pair of its domain: `example.com` and `www.example.com` for `blog.example.com`;
  - every host in `[seo.links] internal_domains`, with its subdomains, as the link settings read them.
- **No new setting.** `internal_domains` already means "other domains that count as this site". A second list for the linter alone would ask the owner to say the same thing twice.
- **The apex** is the last two labels of the host, or the last three under a two-letter country ending whose second label is a common second level (`co`, `com`, `org`, `net`, `ac`, `gov`, `edu`, `or`, `ne`), as in `example.co.uk`. Hosts on a short list of shared hosting domains (`github.io`, `netlify.app`, `pages.dev` and others) have no apex, and neither do numeric addresses or single names such as `localhost`. Khalid carries no public suffix list: the list is large, changes often, and a wrong apex only costs a warning the owner can silence by listing the host or turning the rule off in `lint.toml`.
- **The rule** now fires on an internal link with `nofollow`, and on a full address with `nofollow` whose host is one of the site's own. Its fix says to remove `nofollow` or add the host to `internal_domains`.
- **`sibling_hosts` keeps its meaning**: sister sites served by the same process, for `cross-site-link`. It stays empty while a process serves one site. The site's own hosts are a separate list, `own_hosts`, so a link to the main site is never called a link to a sister site.
- **The link settings do not change.** They still add `rel` to the apex of a subdomain blog. The warning lets the owner choose, and the fix makes the page and the linter agree.

## Consequences

- A subdomain blog that adds `nofollow` to outside links gets one warning per link to its main site, until it lists the main site in `internal_domains`.
- A blog on its own domain, or mounted on the main site's host, gets no new warning: the link settings already leave those links alone.
- A blog on a shared host, or on an unusual country domain, may get a wrong apex. The warning names the link, and the fix is one line in `site.toml`.
