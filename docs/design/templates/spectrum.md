# Spectrum: design brief

Spectrum is an original Khalid template for a magazine arranged by topic. Each topic gets its own colour, and the front page gives every topic a row of its own on a band of that colour. A reader sees at a glance what the magazine covers and finds a topic by its colour on every page.

## Who it is for

- A general-interest magazine with four to six regular topics, such as science, food, travel, culture, nature and design.
- A club, school or city magazine whose readers come for one topic and stay for the next.
- Any site whose topics matter more than its newest post.

## The content model

Spectrum adds no front matter. It reads what Khalid already has:

- **Topics are categories.** Each post's category decides its row and its colour.
- **Each topic's colour comes from its place in the main menu.** The first topic in `[[nav.primary]]` takes the first colour, the second the next, and so on round six colours. A topic that is not in the menu takes a colour from its name. Owners order the colours by ordering the menu.
- **The lead story** is the newest featured post, or the newest post when none is featured.
- **The topic rows** are the home page's topic groups Khalid already builds (up to six topics, four newest posts each), or the sections in `[[reader.home_sections]]` when the site sets them.
- Authors, tags, pages and the newsletter work as in every template.

## The front page

1. **Masthead:** the magazine's name large, its tagline, and the **spectrum bar** under it: one coloured segment per topic, each with its name, linking to that topic's row lower down the page.
2. **Lead story:** a large picture beside the topic label, the title, the description and the byline.
3. **Topic rows:** one per topic, each on a full-width band in the topic's light tint, with a thick rule in the topic's strong colour on top. The row's head holds the topic name and an "All in" link. Its first post is a large card with a picture and description; the others are smaller cards with a picture, title and date. A post shown as the lead story is not repeated in its row, unless it is the topic's only post.
4. The newsletter form and the call to action band.

Later pages of the front page list the posts as cards, each with its topic label and colour.

## The post page

1. A band across the window in the post's topic tint, with the topic's strong rule on top. On it: the topic label, the title, the description, the author and the date.
2. The lead picture under the band, as wide as the frame.
3. The text in one column of about 62 characters a line at 19 to 20 px, so a line holds 60 to 75 characters.
4. Tags, share links and the author box.
5. Related posts as cards, each with its own topic colour.

## Other pages

- **A topic** (category page): the same tinted band as a post, with the topic's name and intro, then its posts as cards.
- **Tags and the archive:** a plain header, then cards with their own topic colours.
- **Writers** (`/authors/`): the shared people grid.
- **A writer:** their mark or photo, role and bio, then their posts as cards.

## Type pair

- Headings, labels and the spectrum bar: **Schibsted Grotesk** 700, a sturdy news grotesque that stays clear in small capitals and in large titles.
- Text: **Petrona** 400 to 700, a warm text serif with a large x-height that reads well at 19 px.

Both are SIL OFL without reserved font names, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand. The pack's own defaults are system fonts.

## Palette

The pack's own colours are ink on white with an ink blue accent:

| Token | Value | Check |
|---|---|---|
| background | `#ffffff` | |
| surface | `#f3f1ec` | |
| text | `#17171a` | 17.9:1 on white, 15.9:1 on surface |
| muted | `#55555c` | 7.4:1 on white, 6.6:1 on surface |
| link | `#1f4fbf` | 7.2:1 on white, 6.4:1 on surface |
| accent | `#1f4fbf` | white on it 7.2:1 |
| border | `#dedad2` | |

**The topic colours** are made from the accent with `color-mix()`, so a site's own accent sets the whole spectrum. Each topic mixes the accent with itself the long way round the OKLCH hue circle, which turns its hue by 60 degrees more than the last and keeps its lightness and chroma. That strong shade is used only for rules and small squares, never for text. The band's tint is 14 percent of it mixed into the background, so text on it is the site's own text colour.

| Colours | Text on the tints | Muted text | Links | Rules on the tints |
|---|---|---|---|---|
| Pack defaults (the showcase) | 14.2:1 or more | 5.9:1 or more | 5.7:1 or more | 5.2:1 or more |
| Kite, the demo brand | 14.4:1 or more | 6.0:1 or more | 5.4:1 or more | 4.8:1 or more |

Browsers that cannot mix hues this way show the surface colour for every topic, so nothing breaks.

The demo's sample brand is Kite, a made-up magazine, with a brick red accent (`#a8321c`, 6.7:1 on white) on white and a warm paper surface.

## Signature device

**The spectrum.** Every topic keeps one colour through the whole site: its segment in the bar on the front page, the band of its row, the small square before its label on every card, and the band at the top of each of its posts and its topic page. Read down the front page, the rows make a spectrum of the magazine's subjects.

## How it differs from the existing templates

- **Atlas** indexes many topics in a lettered map index under one tinted masthead. Spectrum gives each topic its own colour and a full row of picture cards, and carries that colour to every post.
- **Meridian** and **Keystone** are calm company journals with one accent. Spectrum is a magazine with many hues, one per topic.
- **Wire** leads with the newest headlines. Spectrum leads with one story, then arranges everything else by topic, not by date.
- The topic tabs part that several templates offer shows one topic at a time behind tabs. Spectrum shows every topic at once, each in its own band.

Spectrum reuses the shared hero, feed list and people grid parts. It adds no new shared part: the colour of a topic is a pack rule, and its row is a pack template.
