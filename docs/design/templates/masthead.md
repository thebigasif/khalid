# Masthead: design brief

Masthead is an original Khalid template for a general publication: a magazine, a review or a small paper that covers several subjects, such as culture, cities, nature, food and ideas. It is the safe all-rounder. It should feel calm and edited, like a well-made monthly, and suit almost any writing.

## Who it is for

- A small magazine or review with a few writers and five or six sections.
- A local or community publication that mixes features, essays and short pieces.
- Any blog that wants to look like a publication without a busy news layout.

## The content model

Masthead adds no front matter. It reads what Khalid already has:

- **Sections are categories,** such as Culture, Cities or Food. Each section's page lists its stories.
- **Writers are authors,** with their role.
- **The lead story** is the newest post marked `featured` when the `featured` option is on, and the newest post otherwise.
- Tags, series, facts and pages work as in every template. Masthead does not read facts.

## The front page

1. **The nameplate:** the site's name, large and centred, with the menu centred under it between a heavy rule and a thin rule (the header, on every page).
2. **The strapline:** the `tagline` option, one centred line in italics under the nameplate.
3. **The lead story:** its picture, section, title, excerpt, writer and date. In the grid front the text sits beside the picture; in the list front it sits centred above it.
4. **Latest:** the next posts, `latest_count` of them. The `front` option sets their shape: `grid` puts picture cards three to a row, and `list` puts one story a row, the text beside a small picture.
5. **More stories:** the rest of the page's posts, grouped under their section's name as short lists of headlines.
6. The page links, the call to action band and the newsletter form.

With home sections in site.toml, each section shows under its own rule label in the front's shape.

## The post page

1. The trail from the home page, centred.
2. The section as a small capital label, the title centred, the dek in italics and the byline with the writer, their role and the date.
3. The lead picture, wider than the text, with its caption.
4. **The reading column:** about 66 characters a line, at least 18 px, with the first paragraph a little larger and a small square in the accent colour after the last.
5. Tags, share links, the author box, then three related stories under a rule label.

## Other pages

- **Lists** (a section, a tag, the archive): the name centred under a heavy rule, its intro, then the posts in the front's shape.
- **Writers** (`/authors/`): the shared people grid.
- **A writer:** their photo or mark, name, role and bio, centred, then their stories in the front's shape.

## Type pair

- Headings: **Playfair Display** 700, a high-contrast serif made for titles, which gives the nameplate its weight.
- Text: **Lora** 400 to 700, with its italic, a calm serif that reads well at 19 px on screen.

Both are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files and self-hosted in the sample brand. The pack's own default uses the system's serif for both.

## Palette

The pack's default is ink on warm paper with a deep red accent:

| Token | Value | Check |
|---|---|---|
| background | `#fcfbf8` | |
| surface | `#f2efe7` | |
| text | `#1b1a17` | 16.8:1 on background, 15.1:1 on surface |
| muted | `#57524a` | 7.5:1 on background, 6.7:1 on surface |
| link | `#9b1c1c` | 7.9:1 on background, 7.1:1 on surface |
| accent | `#9b1c1c` | white text on it 8.2:1 |
| border | `#d9d3c7` | |

The demo's sample brand, The Common Room, is a deep blue (`#1e4d6b`, 8.5:1 on its background) on a cream page (`#fbf8f2`) with a light sand surface (`#efe8da`).

## Signature device

**The nameplate rule.** The site's name sits centred and large over a heavy rule and a thin rule that run the full width of the page, with the menu in small capitals between them. The same pair of rules heads every section of every page, with the section's name centred between them, so the whole site reads as one publication. No rounded corners, no shadows, no tinted cards: rules and type carry the design.

## How it differs from the existing templates

- **Atlas** is a portal for many topics: a category bar, editor's picks and a topic index. Masthead is smaller and quieter: one lead story, a short latest block and headline lists by section.
- **Meridian** and **Keystone** are company journals with a lead story and picture grids. Masthead is a publication: a centred nameplate, sections and writers, and a choice of grid or list for every list.
- **Wire** is a fast news feed with a ranked list of popular posts. Masthead has no ranking and no ticker; its lists are by section.
- **Folio** puts a product team's notes under a masthead on ruled paper. Masthead has no paper texture or code framing; it is general-purpose.

Masthead adds no shared section part. It uses the shared cards, and the feed list and people grid section parts.
