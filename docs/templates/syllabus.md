---
title: "Syllabus"
order: 86
---

# Syllabus

Syllabus is a template for an online course. It shows your course as modules and lessons, and every lesson shows how far through the course it is. It reads like a course page, not a blog.

```toml
[template]
name = "syllabus"
```

## Who it suits

- A teacher or a small school that publishes a course as a set of lessons, free or paid.
- A company that teaches its product in a fixed order, such as an onboarding course.
- A tutor who also posts course news, such as a live session or a new module.

It suits a blog of separate guides less well. For guides that each stand alone, Compass reads better.

## Lessons, modules and the course

Syllabus needs nothing new in your posts. It reads three things Khalid already has.

**The course is a series.** Give every lesson the same series name, and its place as `order`:

```yaml
---
title: "The graded wash"
description: "Paint a wash that fades from dark to light, the first step to a sky."
category: washes
series:
  name: First Washes
  order: 5
facts:
  - label: Length
    value: "16 min"
  - label: Format
    value: "Video and exercise"
  - label: You need
    value: "Paper, a big round brush"
---
```

Khalid then knows each lesson's number and how many lessons there are, so the lesson shows "Lesson 5 of 13" and a progress ruler. It needs no script and no reader account. See [Series](../discovery.md#series).

**A module is a category.** Modules follow the order of your `categories.toml`, and a category's intro shows at the top of its module page.

**A lesson is a post with facts.** Syllabus reads the first fact as the lesson's length and shows it in the course outline. The other facts, such as the format or what you need, show in the lesson's "at a glance" row. See [Facts on a post](../templates.md#facts-on-a-post) for the limits. A post without facts is course news.

Tutors are authors. Write what each one teaches as their role.

## What the front page shows

1. Your course's name, large, and the `tagline` under it.
2. A "Start with lesson 1" button that links to the first lesson.
3. A row of up to four large numbers from `hero_stats`, such as modules, lessons and hours of video. Nothing shows until you write them.
4. **The course map** on a pale band: every lesson as one small square, grouped by module, with each module's number and name over its squares.
5. **The course outline:** each module as a block with its number, its name and which lessons it holds, such as "Lessons 4 to 7", then one row per lesson with a small square, its number, its title and its length. Lessons are listed oldest first, the order a course is published in, and numbered on across the modules. On a wide screen the module's name sits in a column beside its lessons.
6. **Course news** beside the outline on wide screens and under it on phones: the newest posts without facts, each with its date. `news_count` sets how many.
7. Your call to action as a band, such as "Enrol", and the newsletter.

The front page lists the posts of its first page, 20 at most, so a course of up to 20 lessons and news posts shows whole.

## What a lesson shows

1. The trail from the home page to the lesson's module.
2. The module's name as a small label with a filled square, the title, and the post's description as the lesson's aim.
3. The tutor, who reviewed it, and the date.
4. **The progress ruler** on a pale panel: the course's name, "Lesson 5 of 13", how much of the course that is, such as "38% of the course", and one square per lesson. The lessons before this one are filled in ink, this one in your accent colour, and the rest are open. Under the squares, "Every lesson" opens a list of every lesson, linked.
5. **At a glance:** the lesson's facts in a ruled row of cells.
6. The lead picture, then the lesson in one column about 40rem wide, with a folded contents list for long lessons.
7. **Previous lesson** and **Next lesson** as two large blocks. The next one is filled in your accent colour.
8. The tags, the share links and the tutor box, then related posts as outline rows.

A post outside the course series, such as course news, has no ruler and no lesson blocks.

## Other pages

- **A module** (a category page): "Module" over its name, the module's intro and how many lessons it holds, then its lessons as outline rows, oldest first.
- **Other lists** (a tag, the archive, the series page): the same rows. A list that fits on one page shows its posts oldest first; a longer list shows the newest first, page by page. The series page lists the lessons in course order and numbers them.
- **Tutors** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A tutor's page:** their photo or mark, role and bio, then their lessons as outline rows, and the other tutors.

## Your brand

Syllabus brings its own colours: a deep viridian green (`#1d5d4b`) on white, with a warm paper surface for the course map and the progress panel. Section labels are small capitals over a heavy ink rule. Your colours, fonts and logo in `theme/tokens.toml` replace them. A clear sans for headings suits it best, since lesson titles and numbers are read at a glance.

Syllabus's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Syllabus's default |
|---|---|
| `[fonts] heading` | The system's own sans |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.0625rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2rem` to `2.875rem` |
| `[type] h2` | `1.375rem` to `1.625rem` |
| `[type] h3` | `1.1875rem` to `1.3125rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.65` |
| `[type] heading_weight` | `600` |
| `[shape] radius` | `0.25rem` |
| `[shape] measure` | `40rem` |
| `[space] section` | `4rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the course's name on the front page. |
| `hero_stats` | none | Up to four numbers under the course's name, each written `number \| what it counts`. |
| `start_button` | `true` | The front page shows a button to the first lesson. |
| `course_map` | `true` | The front page shows the course map, one square per lesson. |
| `news_count` | `4` | How many course news posts show beside the outline on the front page, from `0` to `10`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"text"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `5` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"numbered"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "linkedin", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"post"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |

Everything reads with scripts off. Syllabus adds no script.
