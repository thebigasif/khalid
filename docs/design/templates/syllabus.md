# Syllabus: design brief

Syllabus is an original Khalid template for an online course. It shows the course as modules and lessons, and on every lesson it shows how far through the course the reader is. It should read like a course page, not a blog.

## Who it is for

- A teacher or a small school that publishes a course as a set of lessons, free or paid.
- A company that teaches its product in a fixed order, such as an onboarding course.
- A tutor who also posts course news, such as a live session or a new module, beside the lessons.

## The content model

Syllabus adds no front matter. It reads what Khalid already has:

- **The course is a series.** Every lesson has the same `series` name, and `order` is its place. Khalid then knows each lesson's number and the total, so a lesson shows "Lesson 5 of 13" with no script and no reader account.
- **A module is a category.** Modules follow the order of `categories.toml`.
- **A lesson is a post with facts.** The facts are read by position, and only the first has a fixed role: 1 the lesson's length, shown in the outline. The others, such as the format or what you need, show in the lesson's "at a glance" row.
- **Course news is a post without facts.** It shows in a short feed beside the outline, not in it.
- **Tutors are authors.** Their role says what they teach.

## The front page

1. **Hero.** The course's name large, its intro, a "Start with lesson 1" button, then a figure strip from `hero_stats`, such as modules, lessons and hours of video.
2. **Course map.** Every lesson as one square, grouped by module, with each module's number and name over its squares. It shows the shape of the course at a glance.
3. **Course outline**, beside **Course news** on wide screens. Each module is a block: "Module 2" over a heavy rule, its name, "Lessons 4 to 7", then one row per lesson with its number, title and length. A row starts with a small square, the same square as the map.
4. The call to action band, such as "Enrol", and the newsletter.

## The lesson page

1. The module's name above the title, as a small label with a square.
2. The title, the dek as the lesson's aim, the tutor and the date.
3. **The progress ruler:** "Lesson 5 of 13" and "38% of the course", with one square per lesson. Lessons before this one are filled in ink, this one in the accent colour, the rest are open. Under it, a folded list of every lesson, linked.
4. The facts as an "at a glance" row, then the lead picture and the text.
5. **Previous lesson** and **Next lesson** as two large blocks; the next one is filled in the accent colour.
6. Tags, the share links and the tutor box.

A news post, or a post outside the course series, has no ruler and no lesson steps.

## Other pages

- **A module** (category page): "Module" over the name, the module's intro, how many lessons it has, then its lessons as outline rows in course order.
- **Lists** (tag, archive, series): the same rows. A list that fits on one page shows lessons in course order, oldest first.
- **Tutors** (`/authors/`): the shared people grid. A tutor's page shows their lessons as outline rows.

## Type pair

- Headings: **Lexend** 600, a geometric sans made to be easy to read. It gives the course a friendly, clear voice.
- Text: **Figtree** 400 and 700, a plain geometric sans with open letters and clear figures for lesson numbers.
- Lesson numbers and labels: the text font in tabular figures, in small capitals.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand.

## Palette

The pack's own default is a deep viridian green on white with a warm paper surface:

| Token | Value | Check |
|---|---|---|
| background | `#ffffff` | |
| surface | `#f3f0e8` | |
| text | `#1c1b22` | 17.1:1 on white |
| muted | `#57555f` | 7.3:1 on white, 6.4:1 on surface |
| link | `#1d5d4b` | 7.7:1 on white, 6.8:1 on surface |
| accent | `#1d5d4b` | 7.7:1 on white, white text on it 7.7:1 |
| border | `#dcd6c8` | |

The demo's sample brand is a watercolour course in ultramarine (`#2a4896`, 8.5:1 on white) with a pale blue surface (`#f1f3f9`).

## Signature device

**The progress ruler.** A row of small squares, one per lesson: filled in ink for the lessons before, in the accent colour for this one, and open for the lessons ahead, under "Lesson 5 of 13". The same squares make the course map on the front page, grouped by module, and start each row of the outline. It is a new shared section part, `progress_strip`, which a health programme ("Day 3 of 21") or a trip diary ("Day 2 of 9") can use too.

## How it differs from the existing templates

- **Compass** teaches with guides that stand alone. Its signature is a trail of numbered stops in circles, one stop per heading inside one guide. Syllabus is about the order of many lessons: square cells in a ruler, no circles, no trail and no dashed lines, and its numbers count lessons across the whole course.
- **Almanac** is a library of long guides grouped by season. Syllabus groups lessons into modules in a fixed order and shows progress through them.
- **Research** lists papers in a numbered reference register. Syllabus shares its hero, figure strip, feed and people grid parts, but its outline rows carry a lesson number and a length, and its post page carries the ruler and the lesson steps.
