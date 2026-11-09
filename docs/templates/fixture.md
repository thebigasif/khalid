---
title: "Fixture"
order: 87
---

# Fixture

Fixture is a template for a sports club. It shows the next match, the latest results, the club's form and the squad, and it reads like a club's match-day site, not a blog.

```toml
[template]
name = "fixture"
```

## Who it suits

- An amateur or semi-professional club in a league: football, rugby, hockey, netball, cricket or any sport with matches and scores.
- A club volunteer, such as the secretary or a coach, who writes a short report after each match and a preview before the next one.
- A club with juniors, a clubhouse and fundraising news beside its matches.

It suits a magazine or a company blog less well. For those, Wire or Keystone read better.

## Matches are posts with facts

Fixture needs nothing new in your posts. A post with `facts` is a match. A post without facts is club news. Put the facts in this order, since Fixture reads them by their place, not their label:

```yaml
---
title: "Hart's late winner gives Rovers a 1-0 win at Ospring Vale"
description: "Lewis Hart scored eight minutes from time to give Rovers a 1-0 win."
facts:
  - label: Opponent
    value: "Ospring Vale"
  - label: Result
    value: "W 1-0"
  - label: Ground
    value: "Away"
  - label: Competition
    value: "League"
---
```

1. **The opponent.**
2. **The result or the kick-off.** A result starts with `W`, `D` or `L` and a space, then the score with your club's goals first: `W 3-1`, `D 2-2` or `L 0-1`. Anything else, such as `Sat 7 Nov, 2:30pm`, is a kick-off, and the post is a preview of a match still to come.
3. **The ground.** A value that starts with A, such as `Away`, puts the opponent first on the scoreboard, as the home side.
4. **The competition,** such as League or County Cup.

Your labels show nowhere on the scoreboard, so write them in your own language. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

The previews published after your newest result are your upcoming fixtures, and the newest of them is the next match. So write a preview before each match and a report after it, and the front page keeps itself up to date.

Categories are your teams or topics, such as First team, Juniors and Club news. Give each category an intro in `categories.toml`, and it shows at the top of that team's page.

## The squad is your authors

Your players and staff are authors, each with a page in `content/<language>/authors/`. Their `role` says where they play or what they do, such as "Goalkeeper, number 1" or "Club secretary". A player does not need to write a post to be in the squad.

## What the front page shows

1. Your club's name, large, and the `tagline` under it.
2. **The form guide:** your last five results as five square chips, W, D or L, oldest first. A win is in your accent colour, a draw on the surface and a defeat in the text colour, and the letter says which, so colour is never the only signal.
3. A row of up to four large numbers from `hero_stats`, such as played, won and goals scored. Nothing shows until you write them.
4. **Next match** beside the name on wide screens: the opponent large, the kick-off, the ground and the competition, linked to the preview.
5. **Results:** your newest results as scoreboard rows. Each row shows the date, the result chip, the opponent, the ground and competition, the score in large figures and the report's title. `results_count` sets how many.
6. **Fixtures** beside it: the other upcoming matches. Then **Club news:** the newest posts without facts. `news_count` sets how many.
7. Numbered page links, and your call to action as a band, such as "Join the club".

## What a post shows

1. The trail from the home page to the post's team.
2. The team as a small label, the title, and the post's description as a one-line summary.
3. **The scoreboard:** the home side first, each side's goals in large figures, or "v" for a preview. A strip under it gives the result (Win, Draw or Defeat) or the kick-off, the competition and the ground.
4. The author with their mark and role, and the date.
5. The lead picture, then the post in one column about 37rem wide, with a folded contents list for long posts.
6. The tags, the share links and the author box.
7. Related posts as scoreboard rows.

A news post has no scoreboard.

## Other pages

- **Lists** (a team, a tag, the archive): the list's name and intro, then the matches and news as scoreboard rows under a heading for each month, like a fixture list.
- **Squad and staff** (`/authors/`): everyone in a grid of cards with their mark or photo, name and role.
- **A person's page:** their mark, role, experience and bio, then the matches and news they wrote, by month, and the rest of the squad.

## Your brand

Fixture brings its own colours: pitch green (`#11603a`) on white, with a pale chalk surface for the next match card and the scoreboard. Names, scores and section labels are set in condensed capitals. Your colours, fonts and logo in `theme/tokens.toml` replace them. A condensed sans for headings suits it best, since scores and club names are set in the heading font.

Fixture's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Fixture's default |
|---|---|
| `[fonts] heading` | Arial Narrow, then Roboto Condensed and other sans |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.125rem` to `3.25rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.1875rem` to `1.3125rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `600` |
| `[shape] radius` | `0.25rem` |
| `[shape] measure` | `42rem` |
| `[space] section` | `3.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the club's name on the front page. |
| `hero_stats` | none | Up to four numbers under the club's name, each written `number \| what it counts`. |
| `results_count` | `6` | How many results show on the front page, from `1` to `10`. |
| `news_count` | `4` | How many news posts show beside the results on the front page, from `0` to `10`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"text"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `4` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["whatsapp", "facebook", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"post"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a button, such as "Join the club". |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
