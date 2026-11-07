# Margin: design brief

Margin is an original Khalid template for one writer's personal site: a short introduction, a sign-up and essays. It should read like a well-set book of essays with notes pencilled in the margin, not like a magazine, a newsletter archive or a link page.

## Who it is for

- An essayist, columnist or academic who writes for the public under their own name.
- A writer whose essays carry asides and sources: footnotes they want close to the text, not at the bottom.
- A writer whose main aim is a reader who signs up for the letter.

## The content model

Margin adds no front matter. It reads what Khalid already has:

- **An essay is a post.** Its dek is its description. Its lead picture opens it.
- **Margin notes are markdown footnotes** (`[^1]`), which Khalid already renders as a list at the end of the body. Margin moves that list into the margin column with CSS alone.
- **The writer is an author.** The `writer` option names the author whose photo and role sit beside the introduction.
- **The sign-up** is Khalid's newsletter form when the site takes sign-ups, or else `[template.cta]` as a link, with up to three `letter_notes` beside it.
- Categories are topics. Tags work as in every template.

## The front page

On a wide screen every band is the text column (35rem) and the margin (14rem), with a 3rem gutter and one ruled line in the accent down its middle.

1. **Introduction.** The site's name large and the tagline, with the writer's photo, name and role in the margin.
2. **Sign-up band** between hairlines: a heading, one line, the button or form, and the notes in the margin.
3. **Essays** under a small capitals label on an ink rule: the newest with its picture across both columns, then each essay as a row with its title and summary in the text and its date, reading time and topic in the margin. Only the first essay has a picture, so the page stays light.
4. Numbered page links.

## The post page

1. The topic, the title and the dek in the text column; the byline, date and reading time in the margin, aligned to the foot of the title.
2. The lead picture across both columns, cut to 3:2.
3. The text at 20 px, about 70 characters a line. The footnotes sit in the margin, sticky, so they stay beside the text as it scrolls. The note a reader jumps to is tinted.
4. Tags, share links, the author box, the sign-up, then related essays as rows.

With the contents list beside the post (`side-end` or `side-start`), the list takes the margin and the notes follow the text. On a phone the margin's notes become a dateline over each row, and the footnotes follow the essay under an accent rule.

## Type pair

The pack's own default is the system's book serif: Iowan Old Style, Palatino, Georgia. The sample brand sets everything in **Crimson Pro** 400, 600 and italic 400, an SIL OFL book serif with no reserved font name, downloaded from the Google Fonts repository on GitHub with its OFL file, cut to Latin and self-hosted. The margin notes use its italic, so one family carries the whole page.

## Palette

| Token | Pack default | Sample brand |
|---|---|---|
| background | `#fbf9f4` warm paper | `#f8f6f0` |
| surface | `#f2eee5` | `#eeeae0` |
| text | `#1f1c18` | `#1a1d24` |
| muted | `#5d574e` | `#565b66` |
| link | `#8f3424` | `#234a7a` |
| accent | `#a33d2c` brick red | `#2d5a8c` ink blue |
| border | `#e0d9cc` | `#dcd6c9` |

Text, muted text and links pass 4.5:1 on the background and the surface, and the accent passes 3:1.

## The signature device

**The ruled margin.** One thin line in the accent runs between the text and the margin on every wide page, and everything small lives on its far side, set in italic: the writer, the sign-up notes, each essay's date and topic, the byline and the essay's footnotes, which stay beside the text as the reader scrolls.

## How it differs from the other templates

- **Ledger** and **Classic** are calm text blogs with a single column; neither has a margin column or sidenotes.
- **Lattice** sets footnotes as numbered tabs and a panel at the end of the text; Margin keeps them beside the text, sticky in the margin.
- **Chapter** is an author's site built around books, covers and events; Margin has no books and puts the sign-up first.
- **Hub** is a narrow link-in-bio page; Margin is a reading site.

## Shared sections

Margin uses no shared section part. Its rows need the margin column on every row, which no part has, so they are the pack's own `parts-margin.html` (`essay_rows`, `margin_note`).
