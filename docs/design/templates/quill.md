# Quill: design brief

Quill is an original Khalid template for a minimal text blog. It is one person's writing and nothing else: the name, a list of posts, and each post set in large, calm type. It has the fewest parts of any template and the smallest stylesheet, so its pages are the fastest to load.

## Who it is for

- A writer, essayist or diarist who publishes words, not pictures.
- Someone who keeps a public notebook: short notes, reading notes, letters.
- Anyone who wants a blog that loads at once on a slow phone.

## The content model

Quill adds no front matter and no options of its own. Posts, categories, tags, authors and pages work as in every template. A lead picture shows on the post when a post has one, never on a list.

## The front page

One column, about 40rem wide, the same on every page.

1. The header: the site's name in italic after the nib (see below), the menu in quiet small type at the end. No rule under it.
2. The site's name large in regular weight.
3. **The list:** every post as one line between hairlines: the title, then the date at the end of the line in old-style figures. On a phone the date drops under the title. No pictures, no summaries, no authors.
4. Older and newer page links.
5. The newsletter form and the call to action only when the site switches them on.

## The post page

1. The title large in regular weight, the description under it in italic.
2. One quiet line of facts: the author, the date, the reading time and the category.
3. The lead picture, when the post has one, at the column's width.
4. The text at 18 px on a phone and 21 px on a wide screen, about 66 characters a line, with loose leading.
5. **The nib** in the middle under the last paragraph, to say the post has ended.
6. The tags as plain links, the author box and related posts as plain links.

## Other pages

Lists (a category, a tag, the archive) and author pages use the same one-line list under their own title.

## Type

One family for everything, so a site loads at most two font files.

- The pack's default is the system's book serif (Charter, Iowan Old Style, Sitka, Georgia), so it loads no font at all.
- The demo's sample brand uses **Lora**, a serif made for reading on screens, at 400 to 600 in one variable file and an italic at 400. It is SIL OFL, downloaded from the Google Fonts repository on GitHub with its OFL file, and self-hosted.

## Palette

The pack's own default is blue-black ink on paper white:

| Token | Value | Check |
|---|---|---|
| background | `#fcfbf7` | |
| surface | `#f3f0e8` | |
| text | `#1d1c1a` | 16.4:1 on the page |
| muted | `#5d5a54` | 6.6:1 on the page, 6.0:1 on the surface |
| link | `#234a85` | 8.5:1 on the page, 7.7:1 on the surface |
| accent | `#234a85` | white text on it 8.8:1 |
| border | `#e2ddd2` | |

The demo's sample brand is a personal journal, Small Hours: oxblood ink (`#7a2e1f`) on warm paper (`#fbf8f1`). Text 15.5:1, muted 6.7:1, link 8.8:1 on the page.

## Signature device

**The nib.** A small square of the accent colour turned on its point, like the tip of a pen. It stands before the site's name in the header and in the middle under the last line of every post. It is drawn in CSS, so it costs no file and no glyph.

## How it differs from the existing templates

- Classic shows a lead story and a grid of picture cards. Ledger is text-led but has a masthead, ruled study lists and summaries. Quill has no masthead, no cards and no summaries: a list is one line per post.
- Quill replaces no template file. It is tokens and a short stylesheet over the shared pages, so its own rules are the smallest of every template.
- No list page in Quill shows a picture, so a front page loads only its HTML, one stylesheet and, in a brand, its fonts.
