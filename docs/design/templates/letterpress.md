# Letterpress: design brief

Letterpress is an original Khalid template for a newsletter that treats each issue as a printed edition. The front page is the cover of the current issue. Older issues wait under it as a pile of small printed covers. Each issue reads like a page set in metal type: a folio line, a large title, a drop cap and an end mark.

## Who it is for

- A writer or a small team that sends one letter a week or a month and keeps every issue on the web.
- A newsletter with a strong voice, where each issue is a single piece with a few parts, not a stream of short posts.
- A publication that wants its archive to feel collected, like back issues on a shelf.

It is not a sign-up page with one form, a plain archive table, or a markets letter with charts (Ticker).

## The content model

Letterpress adds no front matter. Every post is an issue. Its facts are read by position:

1. **The issue number,** written as it should show, such as `No. 50`.
2. to 4. **In this issue:** up to three lines, each the name of a part (the label, such as "Workshop visit") and its headline (the value). They show as the issue's contents, on the cover and at the top of the issue.

A post without facts is still an issue. It shows its date where the number would be, and no contents box. Categories (such as a weekly and a special edition), tags, authors and pages work as everywhere else.

## The front page

1. **The nameplate:** the site's name very large and centred, the tagline under it in italics, then a folio line between a thick and a thin rule: the newest issue's number, its date and its reading time.
2. **The cover:** the newest issue. The lead picture sits in a plate, a hairline frame with a margin of paper, beside the text. The text has a "Latest issue" label, the title, the description, the byline, the "In this issue" contents with dotted leaders and a "Read this issue" button. No text is ever set over the picture.
3. **The sign-up form,** right under the cover, when the site takes sign-ups, then the call to action band.
4. **Past issues:** every other issue as a small printed cover, three to a row on a wide screen: the number and date over a double rule, the title and the description. Each card has a second sheet behind it, so the archive reads as a pile of back issues. These covers carry no pictures, so the page stays light.
5. Numbered page links, then the shared home sections when they are switched on.

## The issue page

- A folio line between a thick and a thin rule: the issue number, the date and the reading time.
- The category as a small label, the title large and centred, the description in italics, the byline.
- The lead picture in a plate.
- **In this issue:** the contents box with dotted leaders, when the issue has them.
- The text in a book face at a reading width, with a drop cap and the first line in small capitals.
- **The end mark:** a printer's ornament and "End of No. 50", so the reader knows the issue is over.
- Tags, share links, the author box, the sign-up form, then "More issues" as small printed covers.

## Other pages

- **Lists** (category, tag, archive): the name and intro over a thick and a thin rule, then the issues as small printed covers.
- **People** (`/authors/`): a grid of people with their role.
- **A person:** their mark, name, role and bio, then their issues as small printed covers.

## Type pair

- Headings, the nameplate and issue numbers: **Old Standard TT** 700, a revival of the Modern faces of late 19th-century printing.
- Text: **Libre Caslon Text** 400, its italic and 700, a Caslon made for reading on screens, the face of many printed books.

Both are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, subset to Latin and self-hosted in the sample brand. The pack's own default uses Didot and Baskerville stacks, with Georgia last.

## Palette

The pack's own default is a printer's blue ink on an uncoated paper:

| Token | Value | Check |
|---|---|---|
| background | `#f6f2e9` | |
| surface | `#ece5d6` | |
| text | `#1a1a1c` | 15.6:1 on the background, 13.9:1 on the surface |
| muted | `#56524a` | 7.0:1 on the background, 6.2:1 on the surface |
| link | `#1f3f8a` | 8.8:1 on the background, 7.8:1 on the surface |
| accent | `#1f3f8a` | white text on it 9.8:1 |
| border | `#d6ccb8` | |

The demo's sample brand is a letterpress red (`#a2301c`) on a cream paper (`#f7f3ea`): links 6.4:1 on the background and 5.7:1 on the surface, white on the accent 7.1:1.

## Signature device

**The folio line.** A thick rule over a thin rule, with the issue number, the date and the reading time set in small capitals between them, as on the top of a printed page. It sits under the nameplate, at the top of every issue and, in small, at the top of every past issue's cover. The pile of past issues, each a small cover with a sheet behind it, and the end mark under each issue complete the printed feel.

## How it differs from the existing templates

- **Ticker** is a markets newsletter: a dark tape of figures, bar charts and data tables. Letterpress has no numbers to chart; its facts are an issue number and its contents.
- **Chapter** sets an author's site like a book, with a cover shelf of pictures. Letterpress sets a newsletter like a periodical: one cover issue, and back issues as typographic covers with no pictures.
- **Ledger** and **Folio** are text-led blogs with ruled lists. Letterpress is built around issues: a numbered cover, contents in each issue, and an end mark.

The small printed covers are Letterpress's own part (`parts-letterpress.html`). The front page reuses the shared hero part.
