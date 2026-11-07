# Atelier: design brief

Atelier is an original Khalid template for a fashion label, a stylist or a fashion writer. Its front page is a lookbook: each outfit is a numbered look with a tall picture and the list of pieces worn. It should feel like a printed lookbook, not a shop and not a news blog.

## Who it is for

- A small label that shows each season's collection look by look.
- A stylist who posts outfits for clients and readers, with the pieces named.
- A fashion writer who mixes outfit posts with short notes on cloth, care and making.

## The content model

Atelier adds no front matter. It reads what Khalid already has:

- **A look is a post with facts and a picture.** Each fact is one piece of the outfit: the label says what it is, such as "Coat", and the value names it, such as "Double-faced wool, slate". Up to four pieces.
- **A note is a post without facts.** Notes on cloth, care or the studio show in the journal, not in the lookbook.
- **Collections are categories,** such as "Autumn Winter 2026". Each category page is that collection's lookbook, with its intro as the collection note.
- **Tags** are colours and cloths. **Authors** are the designer and the stylists. `featured` picks the cover look.

## The front page

1. **Hero.** The label's name large in the display serif, with the `tagline` beside it on wide screens and under it on phones.
2. **The cover spread.** The featured look, or the newest look: its picture on one side, and on the other its look number, collection, title, description and the list of pieces. Nothing is written on the picture.
3. **The lookbook.** The other looks in a grid of tall pictures, each numbered "Look 02" and on, with its title and pieces under the picture.
4. **Collections** beside **the journal.** Every category as a list of names, and the newest notes as a dated feed.
5. The page links, the call to action band and the newsletter.

## The post page

1. **The spread.** On wide screens, the lead picture fills the start side at its own shape, with its caption under it. The end side holds the collection, the title, the description, the byline and date, and **the pieces** as a ledger: each label, a dotted leader, then the piece. On a phone the picture comes first and the text follows.
2. The post's text in one column of about 66 characters, at 18 px or more.
3. Tags as colours and cloths, the share links and the author box ("styled by").
4. **More looks:** related posts as a small numbered grid of pictures.

A note, a post without facts, has the same spread without the ledger.

## Other pages

- **Lists** (collection, tag, archive): the list's kind, name and intro, then its posts in the lookbook grid, numbered from Look 01.
- **People** (`/authors/`): the designer and stylists in a grid.
- **A person:** their mark, role and bio, then their looks in the grid.

## Type pair

- Headings: **Bodoni Moda** 500, a high-contrast didone at a display size. It gives the names and titles the voice of a fashion magazine.
- Text: **Jost** 400 and 500, a geometric sans in the tradition of the 1920s grotesques. It stays calm under the serif and reads well at 18 px.
- Labels, look numbers and the ledger's labels: Jost in small capitals with wide spacing.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand.

## Palette

The pack's own default is ink on a warm white page, with a rust link colour:

| Token | Value | Check |
|---|---|---|
| background | `#fcfbf9` | |
| surface | `#f2efea` | |
| text | `#191817` | 17.2:1 on the background |
| muted | `#5b5751` | 6.9:1 on the background, 6.3:1 on the surface |
| link | `#7a3324` | 8.7:1 on the background, 7.9:1 on the surface |
| accent | `#191817` | white text on it 17.7:1 |
| border | `#ddd8d0` | |

The demo's sample brand, Studio Ilvane, is ecru (`#f7f4ee`) with a terracotta accent (`#8c3a22`, 7.0:1 on the page, 7.7:1 under white text).

## Signature device

**The look plate.** Every outfit is a numbered plate: a tall picture, and beside or under it "Look 03" in spaced small capitals, the title in the didone and the pieces as a ledger with dotted leaders between each label and its piece. The same plate shows as the cover spread, in every grid and at the top of each post, so the whole site reads like a lookbook. Captions and numbers always sit beside or under a picture, never on it.

## How it differs from the existing templates

- **Atlas** and **Keystone** are magazines with landscape cards and many sections. Atelier has one subject per post, tall pictures and a numbered lookbook grid.
- **Harbor** lists facts as a recipe card on food posts. Atelier's facts are the pieces of an outfit, set as a ledger with dotted leaders beside the picture.
- **Research** numbers papers in a text register. Atelier numbers looks in a picture grid, and its post opens as a two-page spread.

Atelier adds two shared section parts that other picture-led templates on the list can use: a **spread** (a picture beside a text column with a facts ledger, for architecture project spreads and single-image photo stories) and a **picture grid** (numbered tall pictures with the caption under each, for project and story grids). See `crates/khalid-render/templates/sections/README.md`.
