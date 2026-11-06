# Shared section parts

These are the building blocks the original templates share: a hero, a feed, a people grid, a reference list and so on. Each part is one small template macro here, with its base rules in `crates/khalid-theme/packs/_sections/<part>.css`. A template pack builds its pages from these parts and adds only its own layout and look.

## How a pack uses a part

Import the macro in one of the pack's template files:

```jinja
{% from "sections/publication_list.html" import publication_list -%}
{{ publication_list(papers) }}
```

That import is all it takes. When the binary is built, `crates/khalid-theme/build.rs` reads each pack's `.html` files, finds every `"sections/<part>.html"` they name, and puts `_sections/<part>.css` before the pack's own stylesheet. So:

- a site ships the rules of the parts its pack uses, and no others;
- the pack's own rules come later, so it can restyle any part;
- a part without a rules file stops the build.

Every pack's text goes into one Brotli archive, so a part's rules repeated in several packs cost almost nothing in the binary. They do count toward each site's public CSS budget, so keep each part's rules short.

## Rules for a part

- One macro per file, named as the file, plus small helpers it calls or that its callers need, such as `schedule_day`. A part never imports another part, so the build always finds every part a pack uses. To put one part inside another, call it with `{% call %}`, as the hero does.
- Plain HTML that works with scripts off. Logical properties only. Colours, fonts, sizes and radius come from the tokens (`var(--color-*)`, `var(--font-*)`, `var(--text-*)`, `var(--radius)`), never fixed values.
- Every class the macro writes has a rule in its CSS file. Titles in a part are styled as `:is(h2, h3, h4)`, since the caller picks the level.
- Words come from `t` (the strings in `crates/khalid-render/strings/`), never English in the macro.
- Add a part only when two or more kinds of site on the template list need it, and list it below.

## The parts

| Part | Macro and arguments | What it shows | Kinds of site |
|---|---|---|---|
| Hero | `hero(title, intro=none, kicker=none)`; call it with `{% call %}` to add a figure strip or a button under the intro | A small label, the site's name large and its intro | Every front page: research, conference, sports club, charity, course, author, musician |
| Figure strip | `figure_strip(lines, label=none)`; `lines` as the `hero_stats` option writes them, `number \| what it counts` | A row of large numbers with what each counts | Research (papers, people), charity (impact), sports club (season record), course (students), markets newsletter |
| Feed list | `feed_list(posts, level=3)` | A dated feed: date, title, category | Research news, conference updates, sports club news, charity updates, markets newsletter |
| People grid | `people_grid(people, level=3)`; takes the authors page's entries (with post counts) or plain people | Photo or mark, name, role, post count | Research people, conference speakers, sports club squad, charity team, course tutors, musician's band |
| Publication list | `publication_list(posts, level=3, by_year=false, start=1)`; facts by position: 1 venue, 2 year, 3 DOI or link, 4 citations | A numbered reference list, grouped by year with `by_year` | Research papers, author's books, musician's releases, science explainer sources |
| Cite box | `cite_box(page, venue_fact=0, year_fact=1, link_fact=2)` | "Cite this": the reference in one line and a BibTeX entry, from the post's own data | Research papers, science explainer, author |
| Score meter | `score_meter(value, label=none, scale=10)`; `value` is a fact's value, such as `8.5` | A dark square with the score, then a bar of `scale` square cells filled up to it, a half cell for a half point. Any other value, such as "Recommended", shows in the square alone | Gaming reviews, film and TV reviews, product reviews |
| Bar chart | `bar_chart(rows, label=none, note=none)`; `rows` are label and value pairs, such as a post's facts | A bar per row, scaled to the largest value; bars grow both ways from a centre line when a value is below zero. A value that is not a number shows as text | Markets newsletter (each issue's moves), charity (impact), sports club (season record), science explainer (results) |
| Fact table | `fact_table(posts, caption=none)` | Posts as a data table: title and category, date, then one column per fact, headed by the first post's labels. Figure cells carry `data-move` and `data-heat` for a heat map, and the first column stays in view while the table scrolls | Markets newsletter (issues), sports club (results), science explainer (data), any list of posts with facts |
| Cover shelf | `cover_shelf(posts, level=3, fact=1)`; `fact` is the place of the fact shown under the title, counted from 0, such as the year | Covers in portrait (`--cover-ratio`, 2 / 3 by default), each with its title and one fact | Author's books, musician's releases (ratio 1), film and TV reviews, games |
| Contents page | `contents_page(posts, level=3)` | Numbered entries like a book's contents: the title, a dotted leader to the date, then the category and description | Author's excerpts and essays, online course lessons, podcast episodes, newsletter issues |
| Event list | `event_list(posts, level=3)`; facts by position: 1 when, 2 the venue, 3 the town or city, 4 a tickets link (its label is the link's text) or a short fact | Dated rows: when, the title, the venue and city, the tickets link | Author's readings, musician's tour dates, charity events |
| Schedule | `schedule(posts, level=3, by_day=true, sorted=true, place_fact=2, kind_fact=3)`, with `schedule_days(posts)` for links to each day and `schedule_day(day)` for a day's name; facts by position: 1 the day (`2027-05-13`), 2 the time (`09:30`), then the place and the kind | A timetable on a rail: rows by day and time, each with the time, title, author, place with the category, and a kind tag, grouped under each day's date | Conference (Summit), sports club fixtures, musician tour dates, online course weeks |
| Progress strip | `progress_strip(total, current=0, label=none, note=none)` | A row of small squares, one per step: done, the one the reader is on, and the ones ahead, under words such as "Lesson 5 of 13". With `current` 0, an open map of the steps | Online course lessons, health programme days, trip diary days |
| Results list | `results_list(posts, level=3, by_month=false)`; facts by position: 1 opponent, 2 result (`W 3-1`, `D 2-2`, `L 0-1`) or kick-off, 3 ground, 4 competition | Scoreboard rows: a W, D, L or "v" chip, the opponent, the ground and competition, the score in large figures; a post without facts is a news row. With `by_month`, grouped under month headings like a fixture list | Sports club fixtures and results, any league or match site such as gaming tournaments |
| Goal meter | `goal_meter(facts, done=0, goal=1)`; the facts at places `done` and `goal`, counted from 0, read for their digits, so "£61,850" counts as 61850 | Both facts with their labels, a meter and the share reached, such as "68% of the goal"; nothing without two numbers | Charity campaigns (raised of goal), online course (lessons done), health programme (weeks done) |

## Planned parts

Not built yet. The first template that needs one adds it here, with its CSS file.

| Part | What it will show | Kinds of site |
|---|---|---|
| Schedule or timeline | A day's timetable: rows with a time, a title and a room or place (one-off dated events use the event list) | Conference, sports club fixtures, online course weeks |
| Data table | A ruled table with a sticky first column and numbers aligned | Markets newsletter, sports club results, science explainer |
| Schedule or timeline | Dated rows with a time, a title and a place | Conference, online course weeks, musician tour dates |
| Data table | A ruled table with a sticky first column and numbers aligned | Markets newsletter, sports club league tables, science explainer |
| Rating block | A score out of a scale with a few labelled scores under it | Film reviews, gaming, product reviews |
| Call-to-action band | The site's `[template.cta]` as a band; today `cta_band` in `slots.html` does this for every template | Charity donate, course enrol, conference tickets |
