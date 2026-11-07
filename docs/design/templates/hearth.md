# Hearth: design brief

Hearth is an original Khalid template for a warm, classic blog run by a small team or a family business: a nursery, a bakery's back room, a bike shop, a guest house or a family farm. It should feel like walking into a shop where the owners know your name. It is friendly and calm, with pictures first and nothing that shouts.

## Who it is for

- A family business that writes about its work, its seasons and its people, a few posts a month.
- A small team blog where two or three people take turns to write, each with a name and a face.
- A local shop or maker who wants readers to come back for the next story, then come in.

## The content model

Hearth adds no front matter. It reads what Khalid already has:

- **Posts** with a lead picture, a category, tags and one or more authors.
- **The lead story** is the newest featured post, or the newest post when none is featured.
- **Categories** are the blog's topics. The front page lists them as links.
- **Authors** are the team. Their marks or photos show on cards, posts and the team page.

## The front page

1. **Welcome.** The blog's name large and centred, under a small arch mark, with its intro or `tagline` under it and optional `hero_stats`. A hairline closes it.
2. **The lead story.** On a warm surface: the lead story's picture in a tall arch on the start side; the category, the title, the summary and the author's name, face and date beside it.
3. **The card grid.** The other posts on the page, three across on a wide screen, two on a tablet, one on a phone. Each card: the picture in a low arch, the category, the title, the summary, the author and the date.
4. **Topics.** Every category as a rounded link.
5. Numbered page links, home sections as card grids of their own, the call to action band and the newsletter.

## The post page

1. A centred head: the category, the title, the summary, then the authors' faces and names, the date and the reading time.
2. The lead picture wider than the text, in a low arch, with its caption.
3. **The reading column:** about 66 characters a line, 19 px text, generous line spacing.
4. Tags, share links and the author box as a signed note.
5. **Keep reading:** the related posts as cards.

## Other pages

- **Lists** (category, tag, archive): a centred head with the name and intro over a hairline, then the card grid.
- **The team** (`/authors/`): the shared people grid.
- **A person:** their face, name, role and bio, then their posts as cards and the rest of the team.

## Type pair

- Titles: a warm old-style serif. The pack's own default is Constantia, then Georgia. The demo brand uses **Brygada 1918** 600.
- Text: a round, open sans. The pack's own default is the system's sans. The demo brand uses **Mulish** 400 and 700.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand. Neither has a reserved font name.

## Palette

The pack's own default: cream paper, dark brown ink and one terracotta accent, like brick round a fire.

| Token | Value | Check |
|---|---|---|
| background | `#fdf8f1` | |
| surface | `#f6ecdf` | |
| text | `#2a201a` | 15.1:1 on background, 13.6:1 on surface |
| muted | `#6a5848` | 6.4:1 on background, 5.8:1 on surface |
| link | `#a3412a` | 5.9:1 on background, 5.4:1 on surface |
| accent | `#a3412a` | white text on it 6.3:1 |
| border | `#eadccb` | |

The demo's sample brand is Larkfield Nursery: warm white (`#fbf9f4`), a pale sage surface (`#eef0e3`), deep green ink (`#1e2a1f`) and a moss accent (`#3d6b35`, 6.0:1 on the background).

## Signature device

**The arch.** Every picture sits in an arched frame, like the opening of a hearth or an old doorway. The lead story's picture is a full round arch; card and post pictures have a low arch. A small outline arch also sits over the blog's name. Pictures are never covered by text.

## How it differs from the existing templates

- **Classic** is the plain default with square cards and no welcome. Hearth has a centred welcome, a lead story on a warm panel and arched pictures throughout.
- **Harbor** is a recipe blog with tag pills and an offer card. Hearth has no recipe strip and no offer, and its cards carry the author's face.
- **Ledger** is text-led with no pictures in its lists. Hearth leads with pictures.

Hearth reuses the shared people grid. Its card lives in the pack, in `parts-hearth.html`, since every Hearth page uses it and no other template draws the arch.
