# 0000. The build writes nginx and Apache redirect files beside `_redirects`

**Number:** next free. **Status:** Proposed, 2026-11-10. Gap audit 4, row 28 (plan section 7, KH-SEO-8).

## Context

ADR 0031 writes a static build's redirects as `_redirects`, the file Netlify and Cloudflare Pages read. Plan section 7 promises static mode "redirects exported as a `_redirects` or Nginx file". A blog built to a folder that nginx or Apache serves lost every redirect: its readers and search engines met a 404 at each old address.

## Decision

`khalid build` writes the same moves, in the same order, into three files in the output folder, whenever it has at least one move:

- `_redirects`, unchanged.
- `nginx-redirects.conf`: one `location = "<from>" { return <status> "<to>"; }` per move, for an `include` inside the blog's `server` block. An exact location matches only that address.
- `.htaccess`: one `RedirectMatch <status> "^<from>$" "<to>"` per move, inside `<IfModule mod_alias.c>`. `Redirect` would also move every address under the old one, so the build writes an anchored pattern with each special character escaped. The `IfModule` wrapper keeps a server without mod_alias serving pages instead of failing with a 500.
- In both files, each character of the target nginx or Apache reads as code, `$`, quotes, braces and the like, and every character outside ASCII, is percent-encoded, so a path can never end its line or name a variable. Apache reads `&` in a target as the whole match, so it is written `\&`.

The three files come from one list of `RedirectLine` values, so they cannot disagree. The build always writes all three rather than asking for the host: they are small, and a host ignores the files that are not its own.

## Consequences

- A static blog on nginx or Apache keeps its redirects with one `include` line, or none.
- `nginx-redirects.conf` sits in the public folder, so it can be downloaded. It holds only the redirects `_redirects` already shows.
- Still missing from row 28: a `_headers` file and the Content API as static JSON files. Both wait for their own pull requests.
