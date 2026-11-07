# Hub: design brief

Hub is an original Khalid template for a creator's link-in-bio home. It is the one link a creator puts in every profile: a short page with their photo, their name and a stack of large link buttons, and under them the latest posts. It reads like a link page that grew a blog, not like a magazine.

## Who it is for

- A video maker, podcaster, illustrator or maker who sends people from social profiles to one address.
- A creator who sells something, such as a shop, a course or a workshop, and wants that link first.
- A creator who also writes: notes for each video, studio news, how-to posts.

## The content model

Hub adds no front matter. It reads what Khalid already has:

- **The creator is an author.** The `creator` option names the author's slug, and their photo leads the front page. Without it, the newest post's author leads.
- **The links are an option.** `links` holds up to eight lines, each `label | address`, or `label | address | note`. Without links, the main menu fills the stack.
- **The call to action is `[template.cta]`.** It leads the stack in the accent colour, with its `note` as the line under the label.
- **The numbers are `hero_stats`,** such as subscribers and videos.
- Posts, categories and tags work as in every template.

## The front page

One column, 36rem wide on every screen.

1. **Profile.** The creator's round photo in an ink ring over an accent disc, the site's name large, one line from `tagline` (or the front page's intro), and up to four numbers in a row.
2. **Link stack.** The call to action first, filled with the accent, then each link as a large tile with its note under the label.
3. The newsletter form, when the site takes sign-ups.
4. **Latest posts.** Each post as a row between hairlines: a square picture in the ink edge, the date and category, the title and the summary.
5. Numbered page links.

## The post page

1. "All posts" back to the front page, then the category as a small label.
2. The title, the dek, the author's photo, name and role, the date and reading time.
3. The lead picture in the same ink edge and corners as the tiles.
4. The text in the same 36rem column, at 18 px on a wide screen, about 72 characters a line.
5. Tags as pills, share links, the author box.
6. **More from the creator:** the link stack again, without the call to action (`post_links`).
7. Related posts as rows with a square picture.

## Other pages

Lists, tags, the archive and author pages keep the shared layout in the same column, with posts as the same rows.

## Type pair

- Headings, the name and the tile labels: **Sora** 700, a geometric sans with a friendly, wide shape that reads like a profile name.
- Text: **Hanken Grotesk** 400 and 600, a clear grotesque that stays even at 18 px.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand. The pack's own default uses the system's rounded sans for headings and the system sans for text.

## Palette

The pack's own default is a raspberry accent on a warm off-white page:

| Token | Value | Check |
|---|---|---|
| background | `#fbf8f3` | |
| surface | `#f1ebe1` | |
| text | `#1f1b17` | 16.2:1 on the page |
| muted | `#5a524a` | 7.2:1 on the page, 6.5:1 on the surface |
| link | `#a3304e` | 6.4:1 on the page, 5.7:1 on the surface |
| accent | `#a3304e` | white text on it 6.8:1 |
| border | `#ddd3c4` | |

The demo's sample brand is a potter's studio: terracotta (`#9a3e1b`) on clay white (`#f8f3ec`), with a sand surface (`#efe4d4`). Text 14.2:1, muted 6.9:1, link 6.2:1 on the page.

## Signature device

**The link stack.** Every link is a full-width tile with a 2 px ink edge, rounded corners and a hard offset shadow in the same ink, so the stack looks like a pile of cards. A tile presses down into its shadow on hover. The call to action leads, filled with the accent. The same ink edge frames the profile photo, the row pictures, the lead picture and the buttons, so every page carries it.

## How it differs from the existing templates

- Every other template has a wide frame. Hub's frame is one narrow column, about 576 px, on every page.

The link stack is a new shared section part (`crates/khalid-render/templates/sections/link_stack.html`). A musician's streaming links, a podcast's "listen on" links or a course's enrol links can use it too.
