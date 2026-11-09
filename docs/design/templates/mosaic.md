# Mosaic: design brief

Mosaic is an original Khalid template for a photo blog. The posts are laid out as tiles in a rhythm of sizes, like a wall of tiles set by hand: a large one, a tall one, a row of small ones, two halves. Every picture is cut to its tile, and its words always sit under it, never on it.

## Who it is for

- A photo blog that posts often, where each post is one or two pictures with short notes.
- A walking, travel or nature photographer who wants the front page to feel like a wall of recent work, not a list.
- A small group of photographers who share one site, each with their own page of tiles.

## The content model

Mosaic adds no front matter. It reads what Khalid already has:

- **A tile is a post with a lead picture.** A post without one shows a plain tile of the surface colour.
- **Places or kinds of picture are categories.** The category shows over each title, and each category page is a mosaic of its own.
- **Facts are free short notes about the picture,** such as the park, the place, the light and the season. Mosaic shows them as label and value pairs beside the title. It does not read them by position, so any four facts work.

## The front page

1. **Masthead line.** The site's name and the tagline on one line. Small, so the first row of tiles starts near the top.
2. **The mosaic.** A small label, "Latest pictures", over the tiles. On a wide screen the tiles repeat a pattern of nine on twelve columns: a large tile beside a tall one at one row height, three equal tiles, two halves, then a tall tile beside a large one. A last row that would leave a gap closes the wall: a lone tile runs the full width, and a pair splits it in halves.
3. Numbered page links, the call to action band and the newsletter.

From 40rem to 64rem the pattern is six: a full tile, two halves, three thirds. On a phone it is three: a full tile, then two halves.

## Each tile

The picture cut to the tile's shape, then a small square in the accent (the tessera), the category and the date, then the title. The large, half and full tiles also show the summary. The whole tile is one link.

## The post page

1. The picture first, the full width of the frame, never cropped. A tall picture keeps its shape, at most nearly the height of the screen. The caption, such as a credit, sits under it on the end side.
2. **The title row,** under the picture: the category with its tessera, the title and the summary on the start side; the author, the date and the facts as small cells on the end side.
3. The story in one column of about 36rem, 60 to 75 characters a line.
4. Tags, share links and the author box.
5. **More pictures:** the related posts as tiles in the same rhythm.

## Other pages

- **Lists** (category, tag, archive): the list's name and intro over a hairline, then its posts as the same mosaic.
- **People** (`/authors/`): the shared people grid.
- **A person:** their mark, role and bio, then their posts as tiles.

## Type pair

- Pack default: Helvetica Neue, then Arial and other plain grotesques, for titles and words alike, so the pictures carry the colour.
- Demo brand: **Familjen Grotesk** 600 for titles, a grotesque with a little warmth, and **Albert Sans** 400 and 600 for the words.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand. Neither declares a reserved font name.

## Palette

The pack's own default: a pale stone ground like the grout between tiles, near-black ink and one deep teal accent.

| Token | Value | Check |
|---|---|---|
| background | `#f6f4ef` | |
| surface | `#e9e6dd` | |
| text | `#1b1a17` | 15.8:1 on background, 13.9:1 on surface |
| muted | `#5a574f` | 6.6:1 on background, 5.8:1 on surface |
| link | `#0b5d61` | 6.9:1 on background, 6.1:1 on surface |
| accent | `#0b5d61` | white text on it 7.6:1 |
| border | `#d8d4c9` | |

The demo's sample brand is Dry Light: a sand white page (`#fbf8f3`), warm near-black ink (`#231e19`) and a dusk indigo (`#3c3f8c`, 8.7:1 on the page).

## Signature device

**The rhythm and the tessera.** The tiles never sit in an even grid: they follow a set pattern of large, tall, small and half tiles, so the front page reads as a mosaic laid in a beat. Every caption, label and the site's name begins with a small square, a single tile in the accent colour; the name's mark is four of them.

## How it differs from the existing templates

- **Aperture** shows one picture at a time, the full width of the page, with a camera plate, and lists one picture a row. Mosaic shows many pictures at once in a set rhythm of sizes, and never a camera plate.
- **Gallery** is a portfolio wall of pictures. Mosaic is a blog: every tile carries its place, date and title under the picture, and posts carry notes.
- **Atelier** and **Plinth** use even grids of tall pictures and two-page spreads. Mosaic's tiles change size within each row.

No shared section part fits the rhythm closely enough to share yet, so the mosaic lives in the pack. Mosaic reuses the shared people grid and the shared picture macro, so every picture carries its srcset.
