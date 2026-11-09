# Brief: design brief

Brief is an original Khalid template for a short digest newsletter, sent daily or weekly. Each issue is a handful of tight items: a headline, one line that says what happened and a link to the source. A reader should finish an issue in about two minutes. The template is built for that: one narrow sheet of paper, numbered items and nothing to scroll past.

## Who it is for

- A curator who sends a daily or weekly round-up of links on one subject.
- A team or a club that sends members a short digest of news.
- A writer who wants a quick list of what they read, with their own one-line take.

It suits long essays less well. A long post still reads well, but the front page is made for short issues.

## The content model

Brief adds no front matter. It reads what Khalid already has:

- **An issue is a post.** Each item is a `##` heading, one paragraph that says what happened in a line or two, and a link to the source at the end of that paragraph. A short opening line and a sign-off may come before and after the items.
- **Facts, by position:** 1 the issue number, such as `128`, and 2 the number of items, such as `6`. Both are optional. The template reads only the digits of each value.
- **Categories are the editions,** such as Morning brief and Week in review. Tags are topics.
- **Writers are authors.**
- The reading time Khalid works out is shown on every issue, since the promise of the format is that it is short.

## The front page

1. **The masthead.** The newsletter's name large, the tagline and the sign-up form under it, inside the sheet. On a wide screen the form sits beside the name.
2. **The latest issue.** The newest post with facts, or the newest post when none has any. A stamp line with the edition, the issue number and the date, then the title, the summary and the item ticks: one short mark per item with "6 items" and the reading time beside it. A button opens the issue. Its lead picture, when it has one, sits above the stamp line, short and wide.
3. **Earlier issues.** A ruled list, one row per issue: the date as a small stamp at the start, the title, the summary on one line, then the issue number, items and reading time in small type.
4. **Editions.** Every category as a row of chips.
5. Page links and the call to action band.

Later pages show the list only.

## The post page

1. The stamp line: the edition, the issue number, the date and the reading time.
2. The title, the summary as a dek, the writer, then the item ticks.
3. The lead picture, short and wide, if the issue has one.
4. The items. Each `##` heading gets its number in the margin, 01, 02 and on, in large figures, with a hairline over it. The paragraph under it reads as the item's one line, and its last link carries an arrow, so the source is easy to spot.
5. The sign-off line: "That's the brief." with the reading time, so the reader knows they are done.
6. Tags, share links, the author box, the sign-up form, then the three issues before it as rows.

## Other pages

- **Lists** (edition, tag, archive): the name and intro, then the issue rows.
- **Writers** (`/authors/`): the people grid.
- **A writer:** their mark, role and bio, then their issues as rows.

## The sheet

Every page is one sheet of paper about 44rem wide, centred on a tinted desk. The header, the page and the footer sit on the sheet, so the site reads like the email does. On a phone the desk goes and the sheet fills the screen. A long post's side contents sit on the desk beside the sheet.

## Type pair

- Headings, labels, stamps and numbers: **Schibsted Grotesk**, a sturdy news grotesque, at 400 to 700 from one variable file.
- Text: **Lora** at 400 to 700, a calm serif that holds up at the short lengths of a digest.

Both are SIL OFL, downloaded from the Google Fonts repository with their OFL files and self-hosted in the sample brand. The pack itself uses system stacks until a brand sets fonts.

## Palette

The pack's own default: white paper on a pale grey-green desk, near-black ink, one brick red for links and the item numbers.

| Token | Value | Check |
|---|---|---|
| background | `#ffffff` | the sheet |
| surface | `#eceee9` | the desk and the sign-up box |
| text | `#17191c` | 17.6:1 on the sheet |
| muted | `#565c61` | 6.8:1 on the sheet, 5.8:1 on the desk |
| link | `#9c3418` | 7.2:1 on the sheet, 6.2:1 on the desk |
| accent | `#c4461d` | 4.9:1 on the sheet, 4.2:1 on the desk |
| border | `#d8dbd4` | hairlines |

The demo's sample brand, Tideline, uses deep sea teal links and a buoy orange accent on a sea mist desk.

## Signature device

**The item ticks and the margin numbers.** Every issue shows how many items it holds as a row of short upright ticks, like the marks on a stopwatch, beside "6 items" and the reading time. Inside the issue, each item's number hangs in the margin in large figures. A reader sees at once how short the issue is and where they are in it.

## How it differs from the existing templates

- **Ticker** is a markets newsletter that leads with figures, charts and tables in a wide frame. Brief has no charts and no tables: it is one narrow sheet of short items.
- **Hub** is a narrow column of large link buttons to other profiles. Brief's links sit inside short written items, numbered, and its lists are issues, not links.
- **Chapter** lists excerpts as a book's contents page. Brief lists issues as dated rows with their counts.

Brief reuses the shared hero, progress strip (as the item ticks) and people grid. It adds no new shared part.

## What the demo shows

Tideline, a made-up weekday digest about the Selwick coast, a made-up stretch of coast, and the sea beyond it. Every place, group and story is invented, and every link goes to a reserved example address.
