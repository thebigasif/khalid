# Gazette: design brief

Gazette is an original Khalid template for a local news site with many sections: council, schools, sport, business, what's on, community and so on. It should feel like the front of a town's paper: every section on the first screen or two, short headlines packed close, and the newest stories running down the side.

## Who it is for

- A local news site or a town paper that publishes every day across many sections.
- A community newsroom with a few reporters, each covering one or two beats.
- A parish, district or neighbourhood site that posts notices, results and events as well as news.

## The content model

Gazette adds no front matter. It reads what Khalid already has:

- **A section is a category,** such as Council, Schools or Sport.
- **The place is the first fact.** A story can give the street, village or ward it happened in as its first fact, such as `Where: Mill Lane`. The place shows as a small label before the headline everywhere, like the dateline of a printed story, and it starts the dateline on the story itself. A story without facts shows no place.
- **The top story** is the newest post marked `featured`, or the newest post when none is featured.
- Tags are running topics across sections, such as roadworks or the flood defences.

## The front page

1. **The edition bar.** The paper's name, then the date of the newest story and the tagline on one ruled line.
2. **The section index.** A row of links, one per section, each a jump link to its block further down. On a phone it scrolls sideways.
3. **The top story.** Its picture beside the place or section, the headline large, the summary and the byline.
4. **Section blocks.** One block per section, two across on wide screens. Each opens with the **section flag**, then its newest story with a small picture and summary, then up to three more as dense headlines with their places.
5. **The latest news rail.** Beside the top story and the blocks on wide screens, under the top story on a phone: the newest stories, grouped under the day they were published, each a place label and a headline. The newsletter form closes the rail.
6. The numbered page links, the call to action band.

Later pages of the front page list their stories as dense headlines grouped by day, with a summary under each.

## The section flag (the signature device)

Each section block, each section page and each story opens with the section's name set as a flag: bold capitals in reverse, paper on ink, sitting on a heavy ink rule that runs the width of the block. On the front page the flag links to the section, and the rule carries a quiet "All Sport stories" link at its end. The flag makes the page scan like a printed paper, where each section has its own heading bar.

## The post page

1. The section flag, the headline, the dek and the byline with the date.
2. The lead picture with its caption under it as the credit line.
3. **The dateline**: the place in small capitals, set before the text.
4. The text in one column, 18 px or larger, 60 to 75 characters a line.
5. Tags as chips, the share links, the author box, then **More news**: related stories as dense headlines with places and summaries.

## Other pages

- **A section, a tag or the archive:** the section flag with the name and intro, the newest story as a lead with its picture, then the rest as dense headlines grouped by day.
- **Reporters** (`/authors/`): the shared people grid.
- **A reporter:** the photo or mark, role and bio, then their stories as dense headlines grouped by day.

## Type pair

- Headlines: **Domine** 700, a sturdy text serif made for headlines and short text. It stays dark and tight at small sizes, which suits dense lists.
- Text, labels and flags: **Mulish** 400 and 700, a plain, open sans that reads well at 18 px and in small capitals.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, cut to Latin WOFF2 files and self-hosted in the sample brand. The pack's own default uses system fonts.

## Palette

The pack's own default is a civic blue on newsprint:

| Token | Value | Check |
|---|---|---|
| background | `#f8f6f1` | |
| surface | `#ece7dc` | |
| text | `#16181c` | 16.5:1 on background, 14.4:1 on surface |
| muted | `#55534d` | 7.1:1 on background, 6.2:1 on surface |
| link | `#1c4f86` | 7.7:1 on background, 6.8:1 on surface |
| accent | `#1c4f86` | white text on it 8.4:1 |
| border | `#d9d2c3` | |

The flags are background-coloured text on the text colour, 16.5:1. The demo's sample brand, a town paper called The Marlbeck Courier, uses a deep green (`#17603e`) on white: 7.6:1 for links on white.

## How it differs from the existing templates

- **Wire** is a news and columns site led by its writers: a lead story beside the latest headlines, four picture cards, then one long dated feed beside the editor's picks. Gazette is led by its sections: the front page is a grid of section blocks, each under its flag, and the only feed is the narrow latest news rail grouped by day. Wire has no places and no section index; Gazette has no editor's picks, no edition breaking strip and no writer's card.
- **Atlas** is a magazine with a lettered map-sheet index of topics and picture cards. Gazette is dense text: one small picture per section and headlines with places.
- **Locale** lists places by area with hours and a price. Gazette reports news; its place is only a label on a story.

## Shared parts

Gazette reuses the hero, the people grid and the numbered page links. It adds one shared part, the **headline list** (`sections/headline_list.html`): dense headline rows, each with an optional place label from a fact, a summary and a date, or grouped under day headings. Newsletter digests and magazines on the template list can use it too.
