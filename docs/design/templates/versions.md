# Versions: design brief

Versions is an original Khalid template for a software product's changelog. It answers three questions fast: what changed, in which version, and is it new, better or fixed. It has no essays and no picture cards, so it does not repeat Folio.

## Who it is for

- A software company or a small team that ships versions of an app and wants a public changelog.
- An open source project that writes a short note on each change.
- A product team that wants readers to follow only the changes they care about, such as fixes.

## The content model

Versions adds no front matter. It reads what Khalid already has:

- **A change is a post.** Each post is one change, short, with a title that says what changed.
- **The category is the type of change:** New, Improved or Fixed, and optionally a fourth, such as Removed. The `change_types` option lists their slugs in order, so the marks never depend on a name or a language.
- **The first fact is the version,** such as `3.4.0`. Later facts, such as the platforms and the plans, show on the change's page.
- **Tags are product areas,** such as Boards or Mobile apps. Each tag page is that area's changelog.
- **Authors** are the people who wrote the note, with their role.

## The front page

1. **Hero:** "Changelog" as a small label, the product's name large and the tagline.
2. **Filter row:** "All changes", then each type as a chip. Each chip is a plain link to that type's list, so the filter works with scripts off.
3. **The version rail:** the changes grouped by version. Each version sits on the rail with its date, a node on the line, and its changes beside it: type chip, title, one-line summary.
4. **Beside it, on wide screens:** "Get updates", with the sign-up form when the site takes sign-ups and the feed of every change and of each type, then "Versions on this page" as anchor links into the rail.
5. Numbered page links and the call to action band.

## The post page

1. The type chip and the version above the title.
2. The title, the summary (dek), the date and the author with their role.
3. **The facts row:** the version in the monospace, then the other facts, in one ruled panel.
4. The text in one column of about 37rem, at 18px.
5. The areas (tags) as chips, the author box, the newsletter form.
6. More changes as a short version rail.

## Other pages

- **A type of change** (category): its name, intro and its own feed link, the filter row with this type marked, the rail and the versions on the page.
- **An area** (tag), **the archive:** the same, with the filter row taken from the menus: every menu link in the same folder as the list, so areas filter to areas.
- **Authors and an author:** the people grid, and each person's changes on the rail.

## Type pair

- One sans for titles and text: **Instrument Sans** 400 and 600. It is plain and compact, like a product's own interface.
- Version numbers: **Red Hat Mono** 500, so versions line up and read as code.

Both are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand. The pack's own default uses the system sans and monospace.

## Palette

The pack's own default is a blue accent on white with a pale grey surface:

| Token | Value | Check |
|---|---|---|
| background | `#ffffff` | |
| surface | `#f4f5f2` | |
| text | `#16191d` | 17.6:1 on white, 16.1:1 on surface |
| muted | `#525a62` | 7.0:1 on white, 6.4:1 on surface |
| link | `#1d4fb8` | 7.3:1 on white, 6.7:1 on surface |
| accent | `#1d4fb8` | 7.3:1 on white, white text on it 7.3:1 |
| border | `#dde1e4` | |

The demo's sample brand, Mossboard, is moss green (`#2b6a36`, 6.4:1 on its warm white `#fcfcfa`) with a pale leaf surface (`#eff3ec`).

## Signature device

**The version rail.** Every list is a timeline: the version in large monospace on a rail at the start, a node on a thin line, and the version's changes beside it. The newest node is filled. Each change carries a type chip that tells the types apart by shape as well as colour: New is a filled chip with a dot, Improved a tinted chip with a ring, Fixed a plain chip with a square. The same rail shows on the front page, on every list, on each author's page and under each change.

## How it differs from the existing templates

- **Folio** is a product team's essays, with releases as a narrow log beside them. Versions has no essays: the whole site is the log, one post per change, and it filters by type of change.
- **Research** lists papers as a numbered reference register. Versions groups posts by version on a rail and marks each by its type.
- **Wire** is a dated news feed. Versions groups by version, not by day, and its filter row and per-type feeds let a reader follow only one kind of change.

Two new shared section parts come with it: the release log and the filter row (see `crates/khalid-render/templates/sections/README.md`).
