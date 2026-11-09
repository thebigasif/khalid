---
title: "Respawn"
order: 85
---

# Respawn

Respawn is a template for a games site: news, reviews and a release calendar. Every review shows its score as a number in a dark square and a health bar of ten cells, so a reader can scan a page of reviews at a glance.

```toml
[template]
name = "respawn"
```

## Who it suits

- A small games site or a solo writer who reviews games and posts news.
- A gaming newsletter or club that keeps a list of the games coming out soon.
- A site about one kind of game, such as indie games or racing games.

It suits a reviews site for tools or gear less well. For that, Workbench reads better.

## Reviews are posts with facts

Respawn needs nothing new in your posts. A post with `facts` is a review. A post without facts is news, a guide or a feature. Put the facts in this order, since Respawn reads them by their place, not their label:

```yaml
---
title: "Cinder Valley review: a farming game that rewards patience"
description: "A slow, warm farming game with the best soil system we have seen in years."
facts:
  - label: Score
    value: "8.5"
  - label: Platforms
    value: "PC, consoles"
  - label: Developer
    value: "Ashfield Games"
  - label: Released
    value: "21 October 2026"
---
```

1. **The score out of 10,** such as `8`, `8.5` or `10`. A half point shows as a half cell in the health bar. Any other value, such as `Recommended`, shows in the dark square without a bar.
2. **The platforms.** They show under the score on cards and lists.
3. **The developer,** or any short fact.
4. **The release date,** or any short fact.

Your labels show on the review page, so write them in your own words. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

Categories are your sections, such as Reviews, News and Guides. Tags are genres and series.

## The release calendar

The front page shows the games coming out soon from the `release_calendar` option, up to 12 lines, each written `date | game | platforms`:

```toml
[template.options]
release_calendar = [
  "2026-11-20 | Hollowmere | PC, consoles | /hollowmere-release-date/",
  "2026-12-04 | Tidewright | PC, consoles",
  "Spring 2027 | Kitebound: Updraft | PC, consoles",
]
```

- A date written `YYYY-MM-DD` shows as a calendar tile with the month and the day large.
- Any other date, such as `Spring 2027`, shows as written.
- An optional fourth part links the game's name, such as to your news story about it.

List the games in the order you want them shown, and take a line out once the game is out.

## What the front page shows

1. Your site's name and the `tagline` beside it.
2. **The lead story:** the newest featured post, or the newest post, with its picture, category, title and summary, and its score when it is a review.
3. **Latest news** beside it on wide screens and under it on phones: the newest posts without facts, each with its date and category. `news_count` sets how many.
4. **Release calendar:** a tile per game from `release_calendar`. On a phone the tiles scroll sideways.
5. **Latest reviews:** picture cards for the newest reviews, each with its score, health bar and platforms.
6. **More stories:** the other posts without facts as picture cards.
7. Numbered page links, your call to action as a band and the newsletter form.

## What a post shows

1. The category as a small label, the title and the post's description under it.
2. The author with their mark and role, and the date.
3. **The score strip** on a review: the score and health bar, then the other facts in a row between two heavy rules.
4. The lead picture, then the post in one column, at 18 px or larger.
5. **The verdict** at the end of a review: the score large with the health bar and the post's description as the one-line verdict. `verdict` turns it off.
6. The tags, the share links and the author box.
7. Related posts as rows, each with its picture and, on a review, its score.

A post without facts has no score strip and no verdict.

## Other pages

- **Lists** (a category, a tag, the archive): the list's name and intro, then each post as a row with its picture, category, date and title, and the score on each review.
- **Writers** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A writer's page:** their photo or mark, role, experience and bio, then their posts as rows, and the other writers.

## Your brand

Respawn brings its own colours: an electric violet accent (`#5a22d4`) on a cool white page, with near-black for the score squares and the heavy rules. Your colours, fonts and logo in `theme/tokens.toml` replace them. A squared or condensed sans for headings suits it best, since scores, labels and the calendar's dates are set in the heading font.

Respawn's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Respawn's default |
|---|---|
| `[fonts] heading` | Bahnschrift, then other condensed sans fonts |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.125rem` to `3.25rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.1875rem` to `1.3125rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0px`, square |
| `[shape] measure` | `36rem` |
| `[space] section` | `3.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line beside the site's name on the front page. |
| `release_calendar` | none | Up to 12 games coming out soon, each written `date \| game \| platforms`, with an optional link as a fourth part. |
| `news_count` | `4` | How many news posts show in the latest news beside the lead story, from `0` to `10`. The rest show as cards further down. |
| `verdict` | `true` | A review ends with the verdict: its score large and its description. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `featured` | `true` | The newest featured post leads the front page. Without it, the newest post leads. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `4` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "reddit", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |

## Design

The demo is Lowpoly Post, a made-up games site. Every game, studio and event in it is made up, and none of its photographs shows a real game. Its sample brand is a deep magenta (`#a3124f`) on white with a pale lilac surface, Chakra Petch for titles, scores and labels, and Barlow for the text. Both fonts are under the SIL Open Font License.
