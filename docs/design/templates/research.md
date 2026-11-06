# Research: design brief

Research is an original Khalid template for a research lab or an academic group. It shows papers, the people who wrote them and the topics they belong to. It should read like a serious lab website, not a blog.

## Who it is for

- A university lab, a research group or an institute that publishes papers and wants a short summary of each one on its own site.
- A group that also posts news, such as a new grant, a new member or a talk, beside its papers.
- A single researcher who keeps a publication list and a short note on each paper.

## The content model

Research adds no front matter. It reads what Khalid already has:

- **A paper is a post with facts.** The facts are read by position: 1 the venue, 2 the year, 3 the DOI or a link, 4 the citation count. The labels are the writer's own, so a group can write "Journal" or "Conference".
- **News is a post without facts.** It shows in the news feed, not in the publication list.
- **People are authors.** Their role says where they sit in the group, such as "Principal investigator" or "PhD student".
- **Projects and topics are categories.** Each category page is the publication list of that project. Tags are the paper's keywords.

## The front page

1. **Hero.** The lab's name large, its intro, then a figure strip of up to four numbers from `hero_stats`, such as papers, people and grants.
2. **Recent publications**, beside **News** on wide screens. Publications are the newest papers as a numbered reference list. News is a short dated feed.
3. **Research areas.** Every category as a numbered index tile.
4. The call to action band, such as "Join the lab", and the numbered page links.

## The post page

1. The research area above the title, as a small label.
2. The title, the dek as the paper's one-line summary, and the authors with their roles.
3. **The paper panel:** the venue, year, DOI (linked to doi.org) and citations in a ruled grid of four cells.
4. The lead figure, then the text in one column.
5. **Cite this:** a box built from the post's own data, with the reference in one line and a BibTeX entry under it. No script: the reader selects and copies.
6. Keywords (tags), the author box, then related papers as a compact numbered list.

A news post has no paper panel and no cite box.

## Other pages

- **Lists** (category, tag, archive): the publication list grouped by year, with the category's intro as the project description.
- **People** (`/authors/`): a grid of people with their role and how many posts each wrote.
- **A person:** their bio, then their papers as a publication list.

## Type pair

- Headings: **Literata** 600, a serif made for long reading. It gives titles the weight of a printed paper.
- Text: **Fira Sans** 400 and 600, a clear humanist sans with good figures for numbers and DOIs.
- References and BibTeX: the system's monospace.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand.

## Palette

The pack's own default is a claret accent on white with a warm paper surface:

| Token | Value | Check |
|---|---|---|
| background | `#ffffff` | |
| surface | `#f5f3ef` | |
| text | `#1b1a1f` | 17.3:1 on white |
| muted | `#55545c` | 7.5:1 on white, 6.8:1 on surface |
| link | `#8a1c3f` | 9.1:1 on white, 8.2:1 on surface |
| accent | `#8a1c3f` | 9.1:1 on white, white text on it 9.1:1 |
| border | `#dcd7ce` | |

The demo's sample brand is a coastal lab in deep sea blue (`#0c5a78`) on white with a pale sea surface (`#eef4f6`).

## Signature device

**The reference register.** Every paper is an entry in a numbered reference list: a bracketed number such as `[12]` hangs in the margin, the title is set in the serif, the authors follow, and the venue sits in small capitals with the year. The DOI and the citation count run along a hairline under it. The same register shows on the front page, on every list and on each person's page, so the whole site reads like the reference section of a paper.

## How it differs from the existing templates

- **Ledger** is a calm, text-led blog with ruled lists. Research has no blog cards at all: its lists are a numbered register of papers grouped by year, with venue, DOI and citations.
- **Folio** shows releases as a dated log beside essays. Research's facts are bibliographic, and the post page builds a citation and a BibTeX entry from them.
- **Workbench** shows tested figures in a score panel on reviews. Research's panel holds the venue, year, DOI and citations, and its front page has a figure strip for the group's own numbers.
- **Almanac** is a field guide of long guides. Research leads with people and papers, and its category pages are project pages.

No other template has a people grid, a cite box or a year-grouped publication list. Those are new shared section parts (see `crates/khalid-render/templates/sections/README.md`).
