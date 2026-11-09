# Demo publications for the template demos

Each template's demo can show a publication of its own, written for that template, in place of the showcase site's posts: a travel magazine for Atlas, a local newspaper for Wire, a gear review site for Workbench. The showcase site (`fixtures/showcase-site/`) stays the feature showcase at `/khalid/demo/`, the floor the layout check holds every template to, and what the Rust tests build from.

`scripts/demo-publication.sh` puts a publication over a copy of the showcase site. `scripts/pages.sh` and the picture review (`scripts/demo-pictures.sh`) call it before `scripts/demo-brand.sh`, so the template's sample brand dresses the publication and its logo lands on the publication's about page. A template with no folder here keeps the showcase.

## The photographs are not part of Khalid

The photographs the demo publications show are not part of Khalid and are not under the AGPL. Each keeps its own licence, from its photographer and the site it came from, and each publication credits its photos in its `ATTRIBUTION.md`. They are not stored in this repository: they come from the `demo-photos.tar.gz` file on the `demos` release when a demo is built, and no Khalid package ships them.

## What a publication holds

`<template>/`, named as the template is, such as `atlas/`, holds:

- `site.toml`: the settings this publication owns, which replace the showcase's. It holds `name`, `languages`, `[[nav.primary]]`, `[[nav.footer]]`, `[announcement]` and `[template.cta]`. It may also hold `[template.options]`, such as a tagline, which the template's demo keeps. The demo adds `cta_band = "both"` unless the publication sets `cta_band` itself. The showcase's own name, languages, menus, announcement and call to action are taken out even where this file leaves one out. Set `[seo.titles]`, `[seo.publisher]` and `[seo.ai_page]` too, so the publication's own name shows in search results and its page for AI tools. Every other table of the showcase stays: a table set here replaces the showcase's table of that name and every table inside it.
- `categories.toml` and `routes.toml`, which replace the showcase's. Without one, the showcase's is taken out.
- `[mail]` and `[mail.smtp]` in `site.toml` too: the showcase's own sign newsletters with the showcase's name.
- `content/`: the whole content folder, every language, with the authors in `content/<language>/authors/`. It replaces the showcase's. It needs an about page at `content/en/pages/about/` with the slug `about`, where the sample brand puts its logo. Link to the publication's own pages by their path on the showcase's mount, such as `/blog/about/`, as the showcase does. The build moves every such path to the demo's own address.
- `photos.lock`: each photograph the publication shows, as a `[[photos]]` entry. The photographs themselves are never committed.
- `ATTRIBUTION.md`: each photograph's credit and licence, in plain words, as its licence asks.
- `check-pages.txt`: the pages the layout check and the picture review open for this publication.
- `ART-DIRECTION.md`: the publication's photo tone and palette, its subject mix, one image ratio per slot and how each crops on a phone (plan KTD4).
- `ready`: a file, empty or with a note, that marks the publication approved. Until it is there, the publication shows in the picture review only. The Pages demo and the layout check keep the showcase until then.

## photos.lock

```toml
[[photos]]
file = "en/posts/2026-03-02-lisbon-on-foot/lead.jpg"
sha256 = "<the 64 letters and digits sha256sum prints for the file>"
source = "https://unsplash.com/photos/<id>"
licence = "Unsplash License, read 2026-11-05: https://unsplash.com/license"
credit = "Photo by <name> on Unsplash"
```

`file` is the photo's place inside `content/`, with no `..`. In `demo-photos.tar.gz` the photo sits at `<template>/<file>`. Before a publication is put in place, every photo is checked against its `sha256`. A photo that is missing, or whose checksum differs, stops the build and names the photo. A publication with no `photos.lock` fetches nothing.

The script keeps the photos in a cache outside the repository, `~/.cache/khalid/demo-photos` unless `DEMO_PHOTOS_CACHE` names another, and fetches `demo-photos.tar.gz` again only when the cache lacks a photo or holds another copy of one. `DEMO_PHOTOS_URL` sets where the file comes from: by default the `demos` release, or a file on this computer.

## check-pages.txt

One page per line: its kind, then its path after the site's mount. A line starting with `#` is a note.

```text
home /
post /lisbon-on-foot/
table /cost-of-a-week-in-porto/
category /category/city-guides/
tag /tag/walking/
author /authors/ana-ribeiro/
```

Name a post with its lead picture (`post`), a post with a long table (`table`), a category, a tag and an author. The layout check opens each of these pages and holds a `post` or `table` page to the reading check. The home page is always checked, even when this file leaves it out. The picture review takes the home page and the first `post` and `category`.

## Trying a publication

`DEMO_PUBLICATION_ALL=1` puts every publication in place, ready or not, as the picture review does. `DEMO_PUBLICATIONS` names another folder of publications. The tests use the sample in `fixtures/test-publications/`, which never moves here.

## Writing a publication

- Name each post's folder `<date>-<slug>/`, with the slug its front matter gives.
- Put a front matter `description` that holds a colon in quotes. An unquoted second colon stops the whole build.
- The showcase's `lint.toml` still applies, so the linter's scores are measured under it. Every post must lint with no block and score at least 90.
- Never use an em dash. Author pictures are drawn marks, never photographs of people.
- `scripts/demo-photo.py` finds, fetches and credits the photographs: `search` looks in Openverse for photos under CC0, the Public Domain Mark, CC BY or CC BY-SA; `fetch` checks the licence on the photo's own page, keeps the photo at most 2000 px wide in the cache and prints its `photos.lock` entry; `credits` writes `ATTRIBUTION.md` and the visible photo credits page, `content/en/pages/photo-credits/`; `pack` builds `demo-photos.tar.gz`. It needs Pillow for `fetch`. Openverse refuses some data centre addresses, so run it from a home or office connection.
