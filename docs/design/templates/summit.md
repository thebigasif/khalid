# Summit: design brief

Summit is an original Khalid template for a conference, a festival of talks or any event with a programme. It shows the schedule, the speakers and the venues. It should read like a printed programme you pick up at the door, not like a blog.

## Who it is for

- A yearly conference or summit that puts each session on its own page.
- A festival of talks spread over several venues in one town.
- A meetup or event series that posts its programme and its news in one place.

## The content model

Summit adds no front matter. It reads what Khalid already has:

- **A session is a post with facts.** The facts are read by position: 1 the day, written `2027-05-13`, 2 the start time, written `09:30`, 3 the room, 4 the format, such as Keynote, Talk or Workshop. The labels are the writer's own.
- **An update is a post without facts,** such as "Tickets on sale". It shows in the updates feed, not in the programme.
- **Speakers are authors.** The author's role is their job and where they work. A panel names its other speakers in `co_authors`.
- **Venues are categories.** Each category page is that venue's programme, with its intro as the address and how to get there. Tags are tracks, such as "Transport" or "Housing".

## The front page

1. **Hero.** A line with the dates and the town (`event_line`) as a small label, the event's name large, its tagline, a tickets button from `[template.cta]` and a figure strip of up to four numbers, such as days, sessions, speakers and venues.
2. **Programme.** Links to each day, then every session grouped by day, on the timetable rail.
3. **Speakers.** The speakers of the sessions on the page, in a grid with their marks and roles.
4. **Venues** beside **Updates** on wide screens. Venues are the categories as numbered tiles with how many sessions each holds. Updates is a short dated feed.
5. The call to action band, such as "Book a ticket", and the numbered page links.

## The session page

1. The venue above the title, as a small label.
2. The title, the dek as a one-line summary, and the speakers with their roles.
3. **The pass:** the session's day, time, room and format set as a ticket stub, with the date large on the stub and a dashed tear line beside it.
4. The lead picture, then the text in one column.
5. Tracks (tags), the share links and the speaker box.
6. Related sessions as a short timetable.

An update has no pass.

## Other pages

- **A venue or a track** (category, tag): the intro, then that place's sessions on the timetable rail, grouped by day, then its updates.
- **Speakers** (`/authors/`): a grid of speakers with their roles and how many posts each wrote.
- **A speaker:** their bio, then their sessions on the timetable rail, then the other speakers.

## Type pair

- Headings: **Archivo** 800, a sturdy grotesque that reads like a poster.
- Text: **Figtree** 400 to 600, a clear geometric sans with good figures for times.
- Times use tabular figures, so the timetable lines up.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand.

## Palette

The pack's own default is a signal orange accent on white with a warm paper surface:

| Token | Value | Check |
|---|---|---|
| background | `#ffffff` | |
| surface | `#f5f1ea` | |
| text | `#16181d` | 17.8:1 on white, 15.8:1 on surface |
| muted | `#575c66` | 6.7:1 on white, 6.0:1 on surface |
| link | `#b23a0a` | 6.0:1 on white, 5.3:1 on surface |
| accent | `#c2410c` | 5.2:1 on white, white text on it 5.2:1 |
| border | `#ded7cc` | |

The demo's sample brand is a city summit in deep street green (`#0d6b4f`) on white with a pale green surface (`#eef5f1`).

## Signature device

**The timetable rail.** Every session hangs off one vertical rail: the start time in large tabular figures on the left, a dot on the rail, then the title, the speaker, the room and venue, and the format as a small tag. Each day opens with its date set large, the weekday above it. The same rail shows on the front page, on every venue and track page and on each speaker's page, and the session page repeats the time on its ticket stub, so the whole site reads like one printed programme.

## How it differs from the existing templates

- **Research** lists papers as a numbered reference register. Summit's lists are a timetable sorted by day and time, and its facts are a day, a time, a room and a format.
- **Episode** leads with the newest episode and its player. Summit leads with the whole programme, grouped by day, not with the newest post.
- **Wire** is a dated news feed. Summit's order comes from the session's own day and time, not from when the post was written.
- **Keystone** and **Casper** are picture-led blogs. Summit shows no pictures in its lists: pictures appear only on the session page.

The timetable is a new shared section part, `schedule` (see `crates/khalid-render/templates/sections/README.md`), for sports fixtures, tour dates and course weeks as well.
