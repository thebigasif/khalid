# Notebook: design brief

Notebook is an original Khalid template for one person's public notebook: short notes, links with a line or two about them, and things noticed along the way. It reads as a stream, newest first, like the pages of a notebook kept every few days. It is not a magazine: no lead story, no card grid, no picture first.

## Who it is for

- One person who writes often and briefly: a gardener, a developer, a researcher, a designer, a reader.
- Someone who shares links with a comment, as well as notes of their own.
- A notebook with a guest now and then. A second author is fine, but the site is one voice.

## The content model

Notebook adds no front matter. It reads what Khalid already has:

- **A note is a post.** Its description is the note's first lines in the stream, so a short note reads in full without a click.
- **A link note** is a post whose first fact is a web address. The fact's label names the place, such as "Northern Growers' Journal", and its value is the address. The stream and the post both show a "Read it at" link to it. Any other facts show as a small list on the post.
- **Sections are categories,** such as Garden, Reading or Birds. **Tags** are the small subjects on each note.

## The front page

1. **Masthead.** The site's name and the tagline (option `tagline`), or the front page's intro, on a short line.
2. **The stream.** Every note, newest first, under a month heading, such as "November 2026". Each entry has the date in the margin, then its section, title, first lines, the "Read it at" link of a link note and a small picture when it has one.
3. **The side column** on a wide screen, under the stream on a phone: "About this notebook" (the intro or tagline), the sections with links, and the newsletter form.
4. Numbered page links, the home sections, the call to action band.

## The post page

1. The date stamp in the margin beside the title, with the year.
2. The section, the title, the summary, the author and date, and any facts.
3. The note in one column of about 64 characters a line, at least 18 px.
4. A link note's "Read it at" button, the tags as `#tag` links and the share links.
5. The author box, then **More notes:** related notes as the shared feed list.

## Other pages

- **Lists** (a section, a tag, the archive) use the same stream with its margin and month headings.
- **People** and **a person** use the shared pages, with the posts as text cards.

## Type pair

- Words: a calligraphic book serif. The pack's own default is Palatino, then Book Antiqua and Georgia. The demo brand uses **Alegreya** 400, 400 italic and 700.
- The margin dates, section labels and tags: a typewriter face. The pack's own default is the system monospace. The demo brand uses **Courier Prime** 400.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand. Neither has a reserved font name.

## Palette

The pack's own default: a cream page like notebook paper, brown-black ink and a red margin rule like an exercise book.

| Token | Value | Check |
|---|---|---|
| background | `#fbf8f1` | |
| surface | `#f3eee2` | |
| text | `#24211c` | 15.1:1 on background |
| muted | `#5e584d` | 6.6:1 on background, 6.1:1 on surface |
| link | `#9e2f1d` | 6.9:1 on background, 6.3:1 on surface |
| accent | `#9e2f1d` | white text on it 7.3:1 |
| border | `#e2dac8` | |

The demo's sample brand is Maren Holt's garden notebook: a pale sage page (`#f8f8f3`), green-black ink and a deep leaf green (`#2b6a3e`, 6.1:1 on the page).

## Signature device

**The margin rule.** A double line in the accent colour runs down the stream and down every note, like the margin of an exercise book. The date of each entry sits in the margin to its start side in the typewriter face: the day large, the month small. Month headings cross the rule. The same margin and stamp open every post, so each page reads as a page of the same notebook.

## How it differs from the existing templates

- **Classic** and **Ledger** list posts as cards or ruled rows with the date beside the title. Notebook is a dated stream with a margin, month headings and link notes.
- **Folio** is a team's notebook of long essays under a masthead. Notebook is one person's short notes and links.
- **Versions** puts changes on a version rail. Notebook's rail is the calendar.

Notebook reuses the shared feed list for related notes. The stream, the margin and the link notes live in the pack: no other template on the list needs a dated margin yet.
