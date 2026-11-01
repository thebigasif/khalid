# Your Khalid blog on Cloudflare

This repository is a blog built with [Khalid](https://github.com/thebigasif/khalid), an open-source blog engine with good search defaults built in. Cloudflare builds it on every change you push, and serves it for free.

| What | Where |
|---|---|
| The blog's name and address | `site.toml` |
| Posts | `content/en/posts/` |
| Pages, such as About | `content/en/pages/` |
| Authors | `content/en/authors/` |
| Where each kind of page lives | `routes.toml` |
| The Khalid release the build installs | `build.sh` |
| What Cloudflare serves | `wrangler.jsonc`: the `public/` folder the build writes |

First, put your blog's address in `site.toml`, after `address`. Then open the welcome post in `content/en/posts/` and follow it.

The full guide, with your own domain and updates: https://github.com/thebigasif/khalid/blob/main/docs/deploy/cloudflare.md

To build the blog on your own computer, run `sh build.sh` on Linux or a Mac. The pages land in `public/`.
