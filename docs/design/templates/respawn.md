# Respawn: design brief

Respawn is an original Khalid template for a games site: news, reviews and a release calendar. It should feel quick and lively, like a games desk, and still read calmly for long reviews.

## Who it is for

- A small games site or a solo writer who reviews games and posts news.
- A gaming newsletter or club that keeps a list of the games coming out soon.
- A site about one genre, such as indie games or racing games, with reviews and short news.

## The content model

Respawn adds no front matter. It reads what Khalid already has:

- **A review is a post with facts.** The facts are read by position: 1 the score out of 10, such as `8.5`, 2 the platforms, 3 the developer, 4 the release date or any other short fact. The labels are the writer's own.
- **News is a post without facts.** It shows in the news feed and the news cards.
- **The release calendar** is the `release_calendar` option: up to 12 lines, each `date | game | platforms`, such as `2026-11-20 | Hollowmere | PC, consoles`. A date written as `YYYY-MM-DD` shows as a calendar tile; any other text, such as `Spring 2027`, shows as written.
- **Sections are categories,** such as Reviews, News and Guides. Tags are genres and series.

## The front page

1. **Lead row.** The featured post, or the newest post, large: its picture, its category, title and summary, with its score when it is a review. Beside it on wide screens, **Latest news**: a dated feed of the newest posts without facts.
2. **Release calendar.** A row of date tiles, one per game: the month and day large in the heading font, the game's name, and its platforms. On a phone the row scrolls sideways with snap points.
3. **Latest reviews.** Picture cards for the newest reviews. Under each picture: the score tile and the health bar, then the title and the platforms.
4. **More news.** The rest of the posts as picture cards.
5. The numbered page links, the call to action band and the newsletter.

## The post page

1. The category as a small label, the title, the dek and the byline with the date.
2. **The score strip** on a review: the score tile, the health bar and the other facts (platforms, developer, release) in a ruled row.
3. The lead picture, then the text in one column, 18 px or larger, 60 to 75 characters a line.
4. **The verdict** after the text on a review: the score tile large, the health bar and the dek as the one-line verdict.
5. Tags as chips, the share links, the author box, then related posts as compact rows with their scores.

A news post has no score strip and no verdict.

## Other pages

- **Lists** (category, tag, archive): the list's name and intro, then each post as a row: a small picture, the category and date, the title, and the score tile on reviews.
- **Writers** (`/authors/`): the shared people grid.
- **A writer:** the photo or mark, role and bio, then their posts as rows.

## Type pair

- Headings, scores and labels: **Chakra Petch** 700, a squared sans with a game-screen feel. Labels use it in capitals with wide spacing.
- Text: **Barlow** 400 and 600, a plain, slightly narrow sans that reads well at 18 px.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, cut to Latin WOFF2 files and self-hosted in the sample brand. The pack's own default uses system fonts.

## Palette

The pack's own default is an electric violet on cool white:

| Token | Value | Check |
|---|---|---|
| background | `#f8f8fb` | |
| surface | `#eceaf4` | |
| text | `#16141f` | 17.2:1 on background, 15.3:1 on surface |
| muted | `#524f63` | 7.5:1 on background, 6.6:1 on surface |
| link | `#5a22d4` | 7.6:1 on background, 6.8:1 on surface |
| accent | `#5a22d4` | white text on it 8.0:1 |
| border | `#d6d3e2` | |

The demo's sample brand, a games site called Lowpoly Post, uses a deep magenta (`#a3124f`) on white with a pale lilac surface (`#f4eff6`): 7.6:1 for links on white, 6.7:1 on the surface.

## Signature device

**The health bar.** Every score shows as a square ink tile with the number in Chakra Petch, followed by a bar of ten square segments, filled in the accent up to the score, with a half segment for a half point. It shows on review cards, in the score strip, in the verdict and on list rows, so a reader can scan a page of reviews by their bars alone. The release calendar's date tiles, set in the same squared font, are the second mark of the look.

## How it differs from the existing templates

- **Workbench** reviews tools with a ruled spec panel and a thin meter. Respawn's score is a game-style health bar of ten segments in an ink tile, its front page has a release calendar, and its look is bold and squared, not exact and ruled.
- **Wire** is a news site with a ranked list and a feed. Respawn splits reviews from news by their facts and puts the score on every review.
- **Harbor** shows facts as tags on recipe cards. Respawn reads the first fact as a score and draws it.

The score meter is a new shared section part, `sections/score_meter.html`, which a film or product review template can reuse. The release calendar stays in this pack.
