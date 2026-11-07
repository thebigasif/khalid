# Fixture: design brief

Fixture is an original Khalid template for a sports club. It shows the next match, the latest results, the club's form and the squad. It should read like a club's own match-day site, not a blog.

## Who it is for

- An amateur or semi-professional club in a league: football, rugby, hockey, netball, cricket or any sport with matches and scores.
- A club volunteer, such as the secretary or a coach, who writes a short report after each match and a preview before the next one.
- A club with juniors, a clubhouse and fundraising news beside its matches.

## The content model

Fixture adds no front matter. It reads what Khalid already has:

- **A match is a post with facts.** The facts are read by position: 1 the opponent, 2 the result or the kick-off, 3 the ground (home or away), 4 the competition. The labels are the writer's own.
- **A result** is a match whose second fact starts with `W`, `D` or `L` and a space, then the score with the club's goals first, such as `W 3-1`. Anything else in the second fact, such as `Sat 7 Nov, 14:30`, is a kick-off, and the post is a preview.
- **Upcoming fixtures** are the previews published after the newest result. The newest of them is the next match.
- **News is a post without facts.** It shows in the club news feed.
- **The squad and staff are authors.** Their role says where they play or what they do, such as "Goalkeeper" or "Club secretary". A player need not have written a post.
- **Teams and topics are categories,** such as First team, Juniors and Club news.

## The front page

1. **Hero.** The club's name large and its tagline on the left. On the right, the **next match** card: the opponent large, the kick-off, the ground and the competition, linked to the preview.
2. **The form guide** under the name: the last five results as five square chips, W, D or L, oldest first. Then a figure strip of up to four numbers from `hero_stats`, such as played, won and goals.
3. **Results:** the newest results as a ruled scoreboard, each row with the date, the result chip, the opponent, home or away, the score in large figures and the competition.
4. **Fixtures** beside it on wide screens: the other upcoming previews, then **Club news** as a short dated feed.
5. The call to action band, such as "Join the club", and the numbered page links.

## The post page

1. The category as a small label, the title, and the dek.
2. **The scoreboard:** the two sides with the home side first, the score between them in large figures, or "v" and the kick-off for a preview. Under it a strip with the result word (Win, Draw or Defeat), the competition, the ground and the date.
3. The author with their role, then the lead picture and the report in one column.
4. Tags, share links, the author box, then related posts as scoreboard rows.

A news post has no scoreboard.

## Other pages

- **Lists** (category, tag, archive): the list's name and intro, then the posts as scoreboard rows under a heading for each month, like a fixture list.
- **Squad** (`/authors/`): the squad and staff in a grid of cards, each with their mark, name and role.
- **A person:** their mark, role and bio, then the matches and news they wrote.

## Type pair

- Headings, scores and chips: **Oswald** 600, a condensed sans that reads like a scoreboard and fits long club names in a narrow column.
- Text: **Archivo** 400 and 600, a sturdy grotesque with clear figures.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand. The pack's own default stack is a condensed system sans for headings and the system sans for text.

## Palette

The pack's own default is pitch green on white with a pale chalk surface:

| Token | Value | Check |
|---|---|---|
| background | `#ffffff` | |
| surface | `#f1f4ef` | |
| text | `#111a14` | 17.8:1 on white |
| muted | `#4b574e` | 7.6:1 on white, 6.8:1 on surface |
| link | `#11603a` | 7.6:1 on white, 6.9:1 on surface |
| accent | `#11603a` | white text on it 7.6:1 |
| border | `#d5ddd3` | |

A win chip is the accent with white text. A draw chip is the surface with text and a border. A defeat chip is the text colour with white text. The letter inside each chip carries the meaning, so colour is never the only signal.

The demo's sample brand is a club in deep navy (`#1c2d5a`) with a warm chalk surface (`#f5f1e6`).

## Signature device

**The form guide and the scoreline.** Five square result chips sit under the club's name, and every result on the site is a ruled scoreboard row: the chip, the opponent, and the score in large condensed figures. The post page opens with a full scoreboard, the two sides either side of the score. No other template shows a score.

## How it differs from the existing templates

- **Wire** is a news site with a lead story and a feed. Fixture leads with the next match and the club's form, and its lists are scoreboard rows grouped by month, with no picture cards.
- **Research** lists papers as a numbered register. Fixture's facts are a match: the opponent, the score, the ground and the competition, shown as a scoreboard.
- **Workbench** shows tested figures in a score panel. Fixture's panel is a match scoreboard with the home side first.

It adds one shared section part, the results list (`sections/results_list.html`), for fixtures and results. It reuses the hero, figure strip, feed list and people grid.
