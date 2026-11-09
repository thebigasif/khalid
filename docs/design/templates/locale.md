# Locale: design brief

Locale is an original Khalid template for a city guide: the places worth a visit in one city, listed by area, each with its opening hours and price level. It should feel like a well-used pocket guide: calm paper, clear listings and the facts a visitor needs before they walk over.

## Who it is for

- A local writer or a small team who keeps a guide to one city or town.
- A neighbourhood site, a visitor board or a hotel that lists the places it recommends.
- A food, coffee or shopping guide to one place, where every listing needs its hours and price.

## The content model

Locale adds no front matter. It reads what Khalid already has:

- **A place is a post with facts.** The facts are read by position: 1 the kind of place, such as `Café` or `Bookshop`, 2 the price level, 3 the opening hours, 4 the address. The labels are the writer's own.
- **The price level** is written as `1` to `4`, or as one to four of the same sign, such as `££` or `€€€`. Either shows as a price meter of four marks. Any other value, such as `Free`, shows as written.
- **An area is a category,** such as Old Harbour or Hillgate. Tags are themes across areas, such as coffee, rainy day or open late.
- **A place's name** is the part of its title before a colon, so a title can carry a short description for search and still list as a plain name.
- **A note is a post without facts:** a walk, a season's tips, how the guide chooses its places. Notes show apart from the listings.

## The front page

1. **Masthead.** The guide's name large with its intro, then the **area chips**: one chip per area with how many places it holds, each a jump link to that area further down the page.
2. **Places by area.** One block per area, in the order `categories.toml` lists them. Each block has the area's name, its place count and a link to the whole area, then its places as listing rows, two across on wide screens.
3. **Notes from the city.** The posts without facts as picture cards, three across on wide screens.
4. The numbered page links, the call to action band and the newsletter.

An area that holds only notes gets no block.

## The listing row (the signature device)

Each place is one row, set like the entry in a printed guide: a small square picture, then the place's name, a dotted leader, and the **price meter** at the end of the same line. Under it, in small capitals, the kind of place and the opening hours. The price meter is four marks: the sign the writer used (such as `£`) or a round coin when the level is a number, with the marks up to the level in ink and the rest pale. Screen readers hear the level, not the marks.

The same row shows on the front page, on every list, on a writer's page and under a place as related places.

## The post page

1. The area as a small label with a pin mark, the place's name with the rest of its title under it, the dek and the byline with the date.
2. **At a glance:** a ruled card beside the lead picture on wide screens and under it on a phone. It shows the price meter large, then the kind, the opening hours and the address, each with the writer's label.
3. The text in one column, 18 px or larger, 60 to 75 characters a line.
4. Tags as chips, the share links, the author box, then related places as listing rows.

A note has no At a glance card.

## Other pages

- **Lists** (an area, a tag, the archive): the list's name and intro, then its places as listing rows, two across, then its notes as a dated feed.
- **Writers** (`/authors/`): the shared people grid.
- **A writer:** the photo or mark, role and bio, then their places as listing rows and their notes as a feed.

## Type pair

- Headings: **Young Serif** 400, a sturdy, friendly serif with the look of a printed guidebook.
- Text, labels and the listing rows: **Karla** 400 and 700, a plain grotesque that stays clear at small sizes and at 18 px.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, cut to Latin WOFF2 files and self-hosted in the sample brand. The pack's own default uses system fonts.

## Palette

The pack's own default is a deep harbour teal on warm paper:

| Token | Value | Check |
|---|---|---|
| background | `#fbf8f2` | |
| surface | `#efe9dc` | |
| text | `#1d211f` | 15.4:1 on background, 13.5:1 on surface |
| muted | `#555b55` | 6.6:1 on background, 5.8:1 on surface |
| link | `#1d5c57` | 7.3:1 on background, 6.4:1 on surface |
| accent | `#1d5c57` | white text on it 7.7:1 |
| border | `#dcd3c2` | |

The demo's sample brand, a city guide called The Varrow List, uses a brick red (`#a8361b`) on white with a warm stone surface (`#f4efe6`): 6.6:1 for links on white, 5.7:1 on the surface.

## How it differs from the existing templates

- **Atlas** is a magazine for many topics, with a lettered map-sheet index of topics and featured stories. Locale is a directory of places, not stories: its front page is the places themselves, grouped by area, each with hours and a price.
- **Compass** teaches step by step, with a trail and key points on every guide. Locale has no steps: a place is a listing with practical facts.
- **Itinerary** (built at the same time) is a set of trip diaries across countries, day by day, with a packing list. Locale stays in one city and is ordered by area, not by day or trip. It has no dates on its listings and no route.
- **Harbor** shows facts as tags on recipe cards. Locale draws the price as a meter and lines up hours and kind on a ruled listing row.

Locale adds no shared section part. The listing row and the price meter stay in the pack (`parts-locale.html`), since no other template on the list lists places. It reuses the hero, the feed list and the people grid.
