Added
- A Bluesky handle on the blog's own domain: `bluesky = "did:plc:..."` under `[seo.verification]` serves the account's DID at `/.well-known/atproto-did`, and `khalid build` writes the file at the root of the host (`_root/` for a blog in a folder).

Fixed
- Verification files in `[seo.verification.files]` are now written and served for a blog at the root of its host, not only for a blog in a folder, and an HTML verification file is served as HTML.
