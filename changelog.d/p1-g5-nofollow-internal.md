Fixed
- `khalid lint` now warns when a blog on a subdomain, such as `blog.example.com`, tells Google not to follow its links to the main site, `example.com`. The site's own hosts are its address, the `www.` and main domain pair, and the hosts listed in `internal_domains` under `[seo.links]`; listing the main site there stops the `nofollow` and the warning.
