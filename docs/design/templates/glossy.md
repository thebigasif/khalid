# Glossy: design brief

Glossy is an original Khalid template for a lifestyle magazine: home, food, travel, style and wellbeing, told in pictures. Its front page opens like the cover of a printed magazine, then turns into a lively grid of picture cards in several shapes. It should feel bright and generous, like a weekend supplement, not like a news site or a portfolio.

## Who it is for

- A small lifestyle magazine with a few writers and a mix of subjects.
- A blogger who covers home, food and travel and has good photographs for every post.
- A shop or brand that runs a magazine beside its products.

## The content model

Glossy adds no front matter. It reads what Khalid already has:

- **Every post is a story with a lead picture.** A post without a picture still works: its card shows the text on the surface colour.
- **Sections are categories,** such as Home, Food, Travel, Style and Wellbeing. Each category page is that section's stories.
- `featured` picks the cover story. Without one, the newest story is the cover.
- **Tags** and **authors** work as in every template. Facts are not used.

## The front page

1. **The masthead line.** The magazine's name as the page's heading at the start, its tagline at the end, over one hairline.
2. **The cover.** The cover story's picture large on the start side. On the end side, on a soft panel: a "Cover story" label and the section, the title large in the display serif, the excerpt and the byline. Under them, **cover lines**: the next stories in the issue as a short numbered list, as on a printed cover. Each story shows once on the page. Nothing is written on the picture.
3. **Latest stories.** The stories after the cover lines, so no story shows twice near the top, in a grid of cards with a repeating rhythm of six: one wide card, one tall card, three square cards, then one band card that lays its picture beside its text across the whole frame. Each card has its picture first, then the section, the title, the excerpt on the larger cards, and the date with the reading time.
4. **Browse the sections.** Every section as a large rounded link.
5. The page links, the call to action band and the newsletter.

On a phone the grid is one column, and the three square cards become compact rows with a small square picture beside the title, so the page keeps its rhythm.

## The post page

1. The section as a rounded label, the title large and centred, the description and the byline with the date and reading time.
2. The lead picture across the frame, with rounded corners and its caption under it.
3. The text in one column of about 66 characters, at 18 px or more.
4. Tags as rounded labels, the share links and the author box.
5. **More stories:** related posts as three square cards.

## Other pages

- **Lists** (section, tag, archive, dates): the kind of list, its name and intro, then its stories in the same rhythm grid.
- **People** (`/authors/`): the writers in a grid.
- **A person:** their mark or photo, role and bio, then their stories in the rhythm grid.

## Type pair

- Headings: **Playfair Display** 600, a high-contrast transitional serif with a glossy, editorial feel at large sizes.
- Text and labels: **Plus Jakarta Sans** 400 and 600, a friendly geometric sans that stays clear at 18 px and in small labels.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand.

## Palette

The pack's own default is ink on a warm cream page, a blush surface and a raspberry accent:

| Token | Value | Check |
|---|---|---|
| background | `#fffaf5` | |
| surface | `#fbe8df` | |
| text | `#221a1d` | 16.4:1 on the background, 14.4:1 on the surface |
| muted | `#5f5257` | 7.2:1 on the background, 6.3:1 on the surface |
| link | `#a3284c` | 6.8:1 on the background, 6.0:1 on the surface |
| accent | `#a3284c` | white text on it 7.1:1 |
| border | `#efddd4` | |

The demo's sample brand, The Weekend Edit, is cream (`#fbf8f3`) with a peach surface (`#f3e6dc`) and a deep teal accent (`#1d5e5a`, 7.1:1 on the page, 7.5:1 under white text).

## Signature device

**The cover with cover lines.** The front page opens like a magazine cover: one large picture and, beside it on a soft panel, the cover story's title and a numbered list of what else is in the issue. Below it, **the card rhythm** (wide, tall, three squares, band) repeats down the page, so no two neighbouring rows look the same. Rounded pictures and rounded labels give the glossy finish. Text always sits beside or under a picture, never on it.

## How it differs from the existing templates

- **Atlas** is a portal for many topics with a map-sheet masthead and equal picture cards. Glossy has a cover with cover lines and cards in four shapes that repeat in a rhythm.
- **Keystone** and **Meridian** are calm company blogs with framed pictures and wide rows. Glossy is a bright, rounded lifestyle look led by large pictures.
- **Atelier** is a numbered lookbook of tall pictures with facts as a ledger. Glossy uses no facts and no numbers on its cards, and mixes subjects across sections.
- **Harbor** is a food blog built around offers and recipe cards. Glossy is a general lifestyle magazine with no offers.

Glossy's card grid lives in its own parts file (`parts-glossy.html`), since its rhythm is the template's signature. It reuses the shared people grid part on the people and person pages.
