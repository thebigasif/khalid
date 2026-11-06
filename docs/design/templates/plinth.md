# Plinth: design brief

Plinth is an original Khalid template for an architecture or interiors practice. It shows built work as spreads, each project with its photograph, its plan and its data. It should read like a practice's printed monograph, not a blog and not a portfolio grid.

## Who it is for

- A small architecture practice that writes up each finished project with photographs and a drawn plan.
- An interior designer or a landscape studio that wants the same: a project, its place, its size and its year.
- A practice that also posts short studio notes, such as a talk, a material it is testing or a job opening.

## The content model

Plinth adds no front matter. It reads what Khalid already has:

- **A project is a post with facts.** The facts are the project's data, such as location, year, floor area and stage. The labels are the writer's own. On cards the first two facts show, so put the place and the year first.
- **A studio note is a post without facts.** It shows in the notes column, not among the projects.
- **Types of work are categories**, such as houses, interiors, workplaces and public buildings. Tags are materials and themes.
- **People are authors.** Their role says what they do in the practice.
- **Plans are figures in the post**, drawn as SVG files in the post's folder and placed with `:::figure{src="plan.svg" alt="..."}`. Plinth frames each one as a drawing sheet with a number.

## The front page

1. **Hero.** The practice's name large, its tagline, and an optional figure strip from `hero_stats`, such as projects built and years in practice.
2. **Projects as spreads.** Each project is one spread: the photograph on one page, and on the facing page the type of work, the title, the summary and the title block. The photograph sits on a heavy ink bar, the plinth. Spreads alternate sides down the page. On a phone each spread stacks.
3. **Studio notes**: posts without facts, as a short dated feed.
4. **Types of work**: every category that holds projects, as a ruled index.
5. The call to action band, such as "Start a project", and the numbered page links.

## The post page

1. The type of work above the title, the title large, and the dek as a one-line summary.
2. **The project spread:** the lead photograph wide, and beside it on a wide screen the title block with the project's facts, the author and the date.
3. The text in one reading column. Each figure in it, such as a plan, section or elevation, is a numbered drawing sheet: a ruled frame on the drawing paper colour with its number in the corner. Sheets are wider than the text on a wide screen.
4. Materials (tags), the share links, the author box, then related projects as a row of small sheets.

A studio note has no title block. Its photograph sits alone.

## Other pages

- **Lists** (category, tag, archive): the list's name and intro, then the projects as a grid of numbered sheets: photograph, number, title and the first two facts.
- **People** (`/authors/`): the shared people grid.
- **A person:** their role and bio, then their projects as sheets.

## Type pair

- Headings: **Jost** 500, a geometric sans in the spirit of the lettering on drawings and in modernist monographs.
- Text: **Hanken Grotesk** 400 and 600, a plain grotesque that stays calm in long project texts.
- Title block labels and sheet numbers: the system's monospace, small and in capitals, as on a drawing.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand.

## Palette

The pack's own default is brick on warm drawing paper:

| Token | Value | Check |
|---|---|---|
| background | `#f7f5f0` | |
| surface | `#ebe7de` | |
| text | `#1c1b19` | 15.8:1 on background |
| muted | `#5c5951` | 6.4:1 on background, 5.7:1 on surface |
| link | `#9b3d23` | 6.3:1 on background, 5.5:1 on surface |
| accent | `#9b3d23` | white text on it 6.8:1 |
| border | `#d6d0c4` | |

The demo's sample brand, Tessel Rowe Architects, uses a deep green (`#2f5a46`, 7.1:1) with ink `#20241f` on a pale stone (`#f4f3ee`).

## Signature device

**The title block.** On an architect's drawing, a ruled box in the corner names the project, the place, the scale and the sheet. Plinth gives every project one: a grid of ruled cells, each fact's label in small monospace capitals over its value, with the project's sheet number in its own cell. It sits beside the photograph on the front page spreads and on the post page, and the same numbering runs through the drawing sheets in a post. The heavy ink bar under each photograph, the plinth, gives the template its name.

## How it differs from the existing templates

- **Keystone** is a company blog: one lead story, a grid of framed picture cards, and its case studies as an index of ruled text rows. Plinth has no lead story and no card grid on its front page. Each project is a full spread with its photograph, and its facts sit in a boxed title block, not in table columns.
- **Folio** is a studio's log of releases beside essays. Plinth leads with built work and its drawings.
- **Atlas** and **Meridian** are magazines with many pictures of equal weight. Plinth gives one project at a time the whole width.
- **Research** shares the idea of facts read as data, but shows them as a reference list. Plinth shows them as a title block and frames figures as drawing sheets.

The title block is a new shared section part (`sections/title_block.html`), since a fashion lookbook's credits or a trip's dates and distances can use the same box.
