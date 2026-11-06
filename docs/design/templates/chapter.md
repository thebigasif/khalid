# Chapter: design brief

Chapter is an original Khalid template for an author's own site. It shows the author's books, excerpts from them and the events where readers can meet the author. It should feel like the opening pages of a well-made book, not like a blog.

## Who it is for

- A novelist, poet or non-fiction writer with one or more books, who wants one home for them.
- An author who shares excerpts and short essays between books, and lists readings, launches and festival talks.
- A small press or literary agent who keeps a site for one author.

## The content model

Chapter adds no front matter. It reads what Khalid already has:

- **Books, events and writing are categories.** Three options name them by slug: `books_category` (default `books`), `events_category` (default `events`) and `excerpts_category` (default `excerpts`). Every other post is writing, such as an essay or news.
- **A book is a post in the books category.** Its lead picture is the cover art. Its facts are read by position: 1 the publisher, 2 the year, 3 the length or format, 4 a link to order it. The labels are the author's own.
- **An event is a post in the events category.** Its facts are read by position: 1 when, written as the author wants it shown, 2 the venue, 3 the town or city, 4 a link for tickets, or any other short fact such as "Entry: free".
- **An excerpt is a post in the excerpts category.** It is set like a page of the book.
- **Authors are authors.** One site usually has one author and a second person, such as a publicist, who posts the events.

## The front page

1. **The opening:** the author's name large, the tagline under it, then the newest book beside its cover: "The new book", its title, its description, its first three facts and two buttons, "About the book" and the order link.
2. **I. Books:** every book as a shelf of covers, each with its title and year under it.
3. **II. Excerpts and essays:** every other post as a contents page: a number, the title, a dotted leader and the date, with the category and description under it.
4. **III. Events:** the newest events as dated rows: when, the title, the venue and city, and a tickets link.
5. The call to action band, then the newsletter form.

A section with no posts is left out, and the numerals count only the sections shown.

## The post page

- **A book:** the cover on the start side, beside the title, the description, the author and a ruled list of the facts, with the order link as a button. Then the text.
- **An event:** the title, then an event panel with when, where and the tickets link, then the lead picture and the text.
- **An excerpt:** a small label with the category, the title, the description in italics and an ornament. The text is set in the heading serif, larger, with a drop cap and the first line in small capitals, like the first page of a chapter.
- Every post ends with its tags, the share links, the author box and related posts as a short contents page.

## Other pages

- **Lists** (category, tag, archive): the list's name and intro, then its posts in the same three forms as the front page: a shelf for books, a contents page for writing and dated rows for events.
- **People** (`/authors/`): a grid of people with their role.
- **A person:** their mark or photo, name, role and bio, then their books, writing and events.

## Type pair

- Headings, book titles and excerpts: **EB Garamond** 500 and its italic 400, a book face based on Claude Garamond's type.
- Text and labels: **Alegreya Sans** 400 and 700, a humanist sans made for long reading, which sits well beside a Garamond.

Both are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand.

## Palette

The pack's own default is a warm paper with a rust accent, like an old cloth binding:

| Token | Value | Check |
|---|---|---|
| background | `#fbf8f2` | |
| surface | `#f3ede2` | |
| text | `#24201b` | 15.3:1 on the background, 13.9:1 on the surface |
| muted | `#5c544a` | 7.0:1 on the background, 6.4:1 on the surface |
| link | `#8a3a1c` | 7.3:1 on the background, 6.7:1 on the surface |
| accent | `#8a3a1c` | white text on it 7.8:1 |
| border | `#ddd3c3` | |

The demo's sample brand is a pine green (`#2d5a4b`) on a pale paper (`#f9f6ef`), with every text pair above 6:1.

## Signature device

**The contents page.** Each part of the front page opens like a chapter, with a Roman numeral over its name. Writing is listed as a book's table of contents: a number in old-style figures, the title in the serif, and a dotted leader running to the date. The same contents page lists writing on every list, on each person's page and under each post as related posts. An excerpt opens with a drop cap and small capitals, so it reads like the first page of a chapter.

## How it differs from the existing templates

- **Ledger** has a dotted leader in its contents list, but it is a text-led knowledge blog with no pictures. Chapter leads with book covers and sets excerpts as book pages.
- **Solo** is one writer's personal blog with a sign-up headline and a feed. Chapter has no feed: it splits books, writing and events into their own forms.
- **Research** lists papers as a numbered reference register with citations. Chapter's books are covers on a shelf, and its facts are for buying a book or going to an event.
- **Casper** is a picture-led blog with a card grid. Chapter's pictures are portrait book covers, and its writing has no cards at all.

No other template has a cover shelf, event rows or a chapter opener. The cover shelf is a new shared section part (`sections/cover_shelf.html`), since a musician's releases, film reviews and games can use it too. The contents page and the event list are new shared parts too: a course, a podcast or a newsletter can list its lessons, episodes or issues as a contents page, and a musician or a charity can list dates as an event list.
