# Beacon: design brief

Beacon is an original Khalid template for a charity or a cause. It shows the charity's campaigns, how far each one is towards its goal, what the charity has achieved, and one clear way to give. It should read like a charity's own website, not a blog.

## Who it is for

- A small or mid-sized charity that runs appeals with a money goal, such as a river trust, a food bank or an animal rescue.
- A cause or community group that raises money for one project at a time and posts news from its work.
- A foundation that reports its impact in a few large numbers and wants readers to give or volunteer.

Beacon never takes payments. Its donate button is the site's call to action, a link to a page the charity owns, such as a donate page with its bank details or a link to its payment provider.

## The content model

Beacon adds no front matter. It reads what Khalid already has:

- **A campaign is a post with facts.** The facts are read by position: 1 the amount raised, 2 the goal, 3 and 4 any two short facts, such as supporters and the closing date. When the first two hold a number, such as "£61,850" and "£90,000", a goal meter shows how far the campaign has come. The labels are the writer's own, so "Raised" and "Target" work as well as "Given" and "Needed".
- **News is a post without facts.** It shows in the news feed, not among the campaigns.
- **The lead appeal** is the newest featured campaign, or the newest campaign when none is featured.
- **Areas of work are categories.** Tags are a post's topics. Authors are the charity's staff and volunteers, with their role.
- **The donate call is `[template.cta]`.** It shows as a button in the header, in the hero, beside every campaign's goal and as a band at the end of the front page and of each post.

## The front page

1. **Hero:** the charity's name large, its tagline and the donate button, beside the lead appeal as a card with its picture, title and goal meter.
2. **Our impact:** a full-width band on the warm surface with up to four large numbers from `hero_stats`, such as kilometres of river reopened and trees planted.
3. **Campaigns:** the newest campaigns in a grid of cards. Each card shows its picture, area of work, title, goal meter and what it has raised.
4. **News** beside **Our work** on wide screens: a dated news feed, and every area of work as a list of links.
5. The donate band: the call to action's note in large type and its button, on the accent colour across the page.

## The post page

1. The area of work as a small label, the title and the dek.
2. On wide screens, the lead picture beside the **campaign panel**: the goal meter with the amount raised in large figures, facts 3 and 4, and the donate button. On phones the panel follows the picture.
3. The text in one column, about 66 characters a line.
4. Tags, share links, the author box, then the donate band.
5. Related posts as campaign cards.

A news post has no campaign panel.

## Other pages

- **Lists** (category, tag, archive): the list's name and intro, then its posts as campaign cards. A news post's card has no meter.
- **People** (`/authors/`): the shared people grid, titled with the site's own page name.
- **A person:** their photo or mark, role and bio, then their posts as cards.

## Type pair

- Headings and figures: **Bitter** 700, a sturdy slab serif. Its figures stay clear at large sizes, which suits the impact numbers and the amounts raised.
- Text: **Figtree** 400 and 600, a friendly, open sans that reads well at 18 px.

Both are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand.

## Palette

The pack's own default is an ember accent on white with a warm surface:

| Token | Value | Check |
|---|---|---|
| background | `#ffffff` | |
| surface | `#faf4ec` | |
| text | `#1f1d1a` | 16.8:1 on white |
| muted | `#5a544c` | 7.5:1 on white, 6.9:1 on surface |
| link | `#a13d16` | 6.6:1 on white, 6.0:1 on surface |
| accent | `#a13d16` | white text on it 6.6:1 |
| border | `#e6dccd` | |

The demo's sample brand is a river trust in deep river green (`#1b6347`, 7.2:1 on white) with a pale green surface (`#eef5f0`).

## Signature device

**The goal meter.** Every campaign carries a thick, square bar that fills with the accent colour as money comes in, with the amount raised in large slab figures above it, the goal beside it and the share reached under it, such as "69% of the goal". It shows on the lead appeal, on every campaign card and in the campaign panel of each post, so the whole site reads as a set of appeals in progress. The meter is a plain `<meter>` element built from the post's own facts, with no script.

## How it differs from the existing templates

- **Harbor** turns readers into leads with offers inside posts. Beacon has no offers: its one call is to give, and its campaigns carry goal meters.
- **Research** shares the hero, the figure strip and the feed list with Beacon, but shows papers as a reference list. Beacon's facts are money raised towards a goal, and its cards are pictures.
- **Workbench** draws a ten-step score meter on reviews. Beacon's meter is a goal, read from two facts, with the amounts in words beside it.
- **Keystone** and **Atlas** are calm company and magazine blogs with picture grids. Beacon leads with a donate call, impact numbers and campaigns in progress.

The goal meter is a new shared section part, `sections/goal_meter.html`, which an online course (lessons done) or a health programme (weeks done) can use too.
