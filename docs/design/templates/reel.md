# Reel: design brief

Reel is an original Khalid template for a site that reviews films and TV series. Each review gives a star rating, the cast and where to watch. It should feel like a printed cinema programme: warm paper, condensed titles, wide pictures and a ticket for every verdict.

## Who it is for

- A film critic, or a small team of critics, who reviews new releases in cinemas and on streaming services.
- A TV blog that reviews each new season and wants the reader to see the verdict and the service at once.
- A site that mixes reviews with features, such as watch lists, festival reports and interviews.

## The content model

Reel adds no front matter. It reads what Khalid already has:

- **A review is a post with facts.** The facts are read by position: 1 the rating, 2 the release, such as "2026, 118 min" or "Season 2, 8 episodes", 3 the cast, with names split by commas, 4 where to watch, with services split by commas. The labels are the critic's own.
- **The rating** is a number out of 5, such as `4` or `3.5`, or a number with its own scale, such as `8/10`. Reel draws it as five stars. A rating that is not a number, such as "Must see", shows as words.
- **The kind is the category,** such as Films, Series or Documentaries. Features, such as lists and interviews, are posts without facts.
- **Critics are authors.**

## The front page

1. **Hero.** The site's name in condensed capitals and its tagline.
2. **Now showing.** The featured review, or the newest review: a wide letterbox picture beside the title, the excerpt and the ticket.
3. **Latest reviews.** A grid of poster cards: a tall picture, then the stars, the title, the release and where to watch. Nothing is written over a picture.
4. **The score sheet.** Every review on the page in a ruled table, highest rated first: title, kind, release, rating and where to watch.
5. **Features.** Posts without facts, as a dated feed.
6. The call to action, the newsletter and the page links.

A site without facts shows its posts as poster cards, with no score sheet.

## The post page

1. The kind (category) above the title, the title, the dek, the critic and the date.
2. The lead picture in a letterbox frame, wider than the text.
3. **The ticket:** the stars and the score on its stub, then the release, the cast and where to watch, each service as a chip.
4. The review in one column.
5. Share links, the critic's box, then more reviews as poster cards.

A feature has no ticket.

## Other pages

- **Lists** (category, tag, archive): the poster grid, with the category's intro under its name.
- **Critics** (`/authors/`): a grid of the critics with their role and number of reviews.
- **A critic:** their bio, then their reviews as poster cards.

## Type pair

- Headings: **Oswald** 600, a condensed sans in the spirit of cinema posters and marquee boards. The site name and section labels are set in capitals.
- Text: **Spectral** 400 and its italic, a calm screen serif for long reviews. Film titles in the text are set in italic, as critics do.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand. The pack's own default uses the system's condensed sans and serif.

## Palette

The pack's own default is warm programme paper with one cinema red:

| Token | Value | Check |
|---|---|---|
| background | `#fbf8f3` | |
| surface | `#f2ebe0` | |
| text | `#1c1715` | 16.8:1 on background |
| muted | `#5c524b` | 7.2:1 on background, 6.4:1 on surface |
| link | `#9b1c2e` | 7.6:1 on background, 6.8:1 on surface |
| accent | `#9b1c2e` | white text on it 8.1:1 |
| border | `#e0d5c5` | |

The demo's sample brand, Tenth Row, uses a deeper oxblood (`#8a1a2b`, 8.9:1 on its background) on the same paper.

## Signature device

**The ticket.** Every verdict sits on a cinema ticket: a surface panel with a round notch cut from each side and a perforated line between the stub and the rest. The stub holds the stars and the score; the rest holds the release, the cast and where to watch. A small stub, stars and score, repeats on every poster card and in the score sheet, so the rating is always the first thing a reader sees.

## How it differs from the existing templates

- **Workbench** reviews tools and products: squared paper, a green score out of ten with a meter, a spec grid and comparison tables. Reel reviews films and series: a star rating out of five on a ticket, the cast, where to watch, tall poster cards and a ranked score sheet.
- **Atlas** and **Keystone** lead with pictures too, but in landscape cards with no rating. Reel's cards are tall posters with a stub under the picture.
- **Research** and **Ledger** are text registers. Reel is picture-led and its table is ranked by score.

The star rating is a new shared section part, `rating`, which the gaming template can use too (see `crates/khalid-render/templates/sections/README.md`).
