# Itinerary: design brief

Itinerary is an original Khalid template for a travel diary: trips filed by country, each written up day by day, with a few notes such as a packing list. It should feel like the papers in a traveller's pocket: tickets, a boarding pass and stamps in a passport.

## Who it is for

- A couple, a family or one traveller who writes up each trip as a plan others can copy.
- A slow travel or walking blog where the days, the route and the cost matter.
- A diary with notes beside the trips: packing lists, planning tips, favourite night trains.

## The content model

Itinerary adds no front matter. It reads what Khalid already has:

- **A trip is a post with facts,** read by position: 1 the length (a number of days), 2 the route, 3 when, 4 the cost. The labels are the writer's own.
- **A note is a post without facts.** It shows in a dated list under the trips.
- **Countries are categories.** A category whose posts on the page are all notes is not a country, by the same rule as Research's news column.
- **Days are the post's main headings,** such as "Days 1 and 2: Tbilisi". The contents list them.
- **A packing list is a task list** in the post (`- [ ]`), styled as a checklist.

## The front page

1. **Hero.** The diary's name large, its tagline, and a figure strip from `hero_stats`, such as countries, trips and days on the road (shared parts `hero` and `figure_strip`).
2. **Latest trip.** The newest trip as a wide ticket: picture, country, title, route, season, cost, and a stub with the days.
3. **Countries.** Each country as a passport stamp with its trip count.
4. **Trip diaries.** The other trips as a grid of tickets.
5. **Notes from the road.** The newest notes as a dated feed (shared part `feed_list`).
6. Page links, the call to action band and the newsletter.

## The post page

1. Breadcrumbs, the country as a small stamp, the title, the description, the byline and the date.
2. **The boarding pass:** route, when and cost in cells, the days large on a stub behind a perforation.
3. The lead picture, then the text in one column, with the contents ("Day by day") beside it on wide screens.
4. **The route:** a dashed line down the start edge of a trip's text, with a stop at each main heading.
5. Checklists for task lists, then tags, share links, the author box, and related posts as small tickets.

A note has no pass and no route.

## Other pages

- **Lists** (a country, a tag, the archive): the name on a stamp, the intro, then tickets.
- **Writers:** the shared `people_grid`. **A writer:** bio, then their trips as tickets.

## Type pair

- Headings: **Zilla Slab** 700, a slab serif with the feel of a luggage label and a printed ticket.
- Text: **Karla** 400 to 700, a friendly grotesque that stays clear at 18 px and up.
- Codes on tickets (labels, dates): the system's monospace, like a printed fare.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand. The pack's own stack falls back to Rockwell and Georgia.

## Palette

The pack's own default is a rust accent, like stamp ink, on white with a kraft surface:

| Token | Value | Check |
|---|---|---|
| background | `#ffffff` | |
| surface | `#f5efe4` | |
| text | `#1e1c1a` | 17.0:1 on white, 14.8:1 on surface |
| muted | `#5c564e` | 7.3:1 on white, 6.3:1 on surface |
| link | `#a33a12` | 6.6:1 on white, 5.8:1 on surface |
| accent | `#b8461a` | 5.3:1 on white, 4.7:1 on surface, white text on it 5.3:1 |
| border | `#e2d8c8` | |

The demo's sample brand, Slow Miles, is forest green (`#1f6b4f`) on white with a pale leaf surface (`#eef4ef`).

## Signature device

**The ticket.** Every trip is a ticket: a kraft card with the picture on top and a stub at the bottom behind a dashed perforation, with a notch cut at each end. The stub carries the number of days, large, and the date. On the front page the newest trip is a wide ticket with the stub standing at the end; on its own page the facts become a boarding pass. Countries are passport stamps, a little askew, and the days of a trip are stops on a dashed route.

## How it differs from the existing templates

- **Atlas** is a city magazine: a map-sheet masthead with a lettered index of topics, featured stories and picture cards. Itinerary has no masthead index and no magazine cards. It is organised by trip and country, and its unit is the ticket with days, route, season and cost.
- **Compass** is a teaching blog: numbered stops on one dashed trail of topics and a numbered contents route. Itinerary numbers nothing itself: the days come from the writer's own headings, the route runs down a single trip, and the contents are titled Day by day.
- **Harbor** and **Workbench** show facts as recipe or score panels. Itinerary's panel is a boarding pass with a perforated stub, and its lists are tickets.
- No other template styles task lists as a checklist or shows categories as stamps.

No new shared section part is added. The ticket and the stamp are this template's own; the hero, figure strip, feed list and people grid come from the shared library.
