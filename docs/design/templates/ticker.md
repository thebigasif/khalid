# Ticker: design brief

Ticker is an original Khalid template for a markets newsletter. Each issue reports a week of moves in numbers, so the template leads with figures, tables and charts, not pictures. Every chart is HTML and CSS: it draws with scripts off and reads as a list of numbers to a screen reader.

## Who it is for

- A weekly or daily markets letter: stocks, rates, currencies, commodities.
- A finance writer who explains how markets work, with tables in the posts.
- A trade body or research desk that publishes a regular round-up of figures.

## The content model

Ticker adds no front matter. It reads what Khalid already has:

- **An issue is a post with facts.** Each fact is one market and its move: the label names the market, such as "Stocks index", and the value is the move, such as `+1.8%`. A value that is a plain number, with an optional sign, `%`, `bp`, `pts`, currency sign or thousands commas, draws a bar. Any other value shows as text.
- **Keep the same labels in the same order each issue.** The issue table then lines up week by week, one column per market.
- **A post without facts** is an explainer or a note. It shows in the same lists, with empty figure cells.
- **Desks are categories,** such as Weekly wrap, Stocks or Rates. Tags are topics.
- **Writers are authors.**

## The front page

1. **The tape.** An ink band across the top of the page with up to four figures. It shows the `hero_stats` lines, written `+1.8% | Stocks index`, or, without them, the newest issue's facts. Each figure is marked up, down or flat by a small drawn triangle and its weight, never by colour alone.
2. **The masthead.** The letter's name large, its tagline, and the sign-up form beside them on wide screens.
3. **The latest issue.** Its desk, title, summary, date and writer on one side; its facts as a bar chart on the other, under the label "This issue in numbers".
4. **The issue table.** Every other post on the page as a data table: date, title and desk, then one column per fact. Each figure cell is shaded by how far it moved, as a heat map, up in the accent and down in ink.
5. **Desks.** Every category as a ruled row of links.
6. Page links, the call to action band and the disclaimer line.

## The post page

1. The desk as a small label, the title, the dek and the writer with the date.
2. **This issue in numbers:** the facts as a bar chart from a centre line, up bars filled in the accent and down bars hatched in ink, each with its value in figures.
3. The disclaimer line from the `disclaimer` option, if set.
4. The lead picture, then the text. Tables in the text are data tables: a heavy rule over the head, figures in tabular numbers aligned to the end, and a first column that stays put while a wide table scrolls.
5. Tags, share links, the author box, then related posts as a small issue table.

## Other pages

- **Lists** (desk, tag, archive): the name and intro, then the posts as the issue table.
- **Writers** (`/authors/`): the people grid.
- **A writer:** the bio, then their posts as the issue table.

## Type pair

- Headings and labels: **Archivo**, a grotesque made for headlines, at 400 to 700 in one variable file.
- Text: **IBM Plex Serif** 400, a sturdy serif for long reading.
- Figures: **IBM Plex Mono** 500 for the tape, the charts and every figure cell.

All three are SIL OFL, downloaded from their makers' GitHub repositories with their OFL files and self-hosted in the sample brand. The pack itself uses system stacks until a brand sets fonts.

## Palette

The pack's own default: a deep green accent for up moves, ink for down moves, on a warm off-white page.

| Token | Value | Check |
|---|---|---|
| background | `#fbfaf7` | |
| surface | `#f1efe8` | |
| text | `#15181d` | 17.0:1 on the background |
| muted | `#535861` | 6.9:1 on the background, 6.2:1 on the surface |
| link | `#0b5e44` | 7.5:1 on the background, 6.8:1 on the surface |
| accent | `#0b6b4f` | 6.2:1 on the background, white on it 6.5:1 |
| border | `#d9d5ca` | |

The demo's sample brand, Tallyline, uses a petrol blue accent (`#0f5b6e`) on the same warm paper.

## Signature device

**The tape and the moves.** A dark tape of figures runs across the front page like a market board, each figure with a drawn up or down mark. The same idea runs through the site: every issue's facts are a bar chart from a centre line, and every list is a table whose figure cells are shaded by size, so a reader sees the week's moves before reading a word.

## How it differs from the existing templates

- **Workbench** shows a score panel and comparison tables for products. Ticker's figures are signed moves, charted from a centre line, and its lists are tables of figures across issues.
- **Keystone** lists posts with facts as a project index, one column per fact. Ticker shades each figure cell by how far it moved and leads with a chart of the newest issue.
- **Research** lists papers as a reference register. Ticker shares its hero and people grid, but not its look.

Two new shared section parts come from Ticker: a bar chart and a fact table (see `crates/khalid-render/templates/sections/README.md`). A sports club's season record, a charity's impact figures and a science explainer's results can use them too.

## What the demo shows

Tallyline, a made-up weekly letter about the made-up economy of Varnholm. Every index, company, price and move is invented. The announcement bar, the about page and a line under every chart say so, and say that nothing on the site is investment advice.
