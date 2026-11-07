# Gallery: design brief

Gallery is an original Khalid template for a studio or an illustrator. The front page is a wall of pictures, each hung on a pale mat with a numbered label under it, like a show in a small gallery. Each post is a project page. It works with no script at all.

## Who it is for

- An illustrator, printmaker or designer who shows finished work and writes a short note on each piece.
- A small studio that shows client projects: posters, book covers, prints, patterns.
- A craft maker whose work is flat or framed, such as textiles, ceramics shot from above or paper cuts.

## The content model

Gallery adds no front matter. It reads what Khalid already has:

- **A work is a post with a lead picture.** The picture is never cropped, so tall prints, wide posters and square tiles all keep their shape.
- **The project details are facts,** read by position: 1 the client or publisher, 2 the year, 3 the medium, 4 the size or edition. The labels are the writer's own.
- **Disciplines are categories,** such as illustration, print or pattern. Tags are subjects.
- **More pictures of a project** go in the post's body as figures, under the text.

## The front page

1. **Studio line.** The studio's name large, and its statement (the home intro, or the `statement` option) beside it.
2. **Discipline chips.** The shared filter row: "All work", then each category. Plain links, so they work with scripts off.
3. **The wall.** Every post on the page in CSS columns: two on a phone, three on a tablet, four on a wide screen. Each picture sits on a mat at its own shape. Under it, never on it, the wall label: the number, the title, then the medium and year.
4. **List of works.** A numbered checklist of the same works, with the client and year, like the sheet a gallery hands out at the door. The numbers match the wall labels. The `checklist` option turns it off.
5. Numbered page links, the call to action band and the newsletter.

## The post page

1. **Project head.** On the start side the discipline, the title and the summary. On the end side the project sheet: the four facts in ruled rows.
2. **The plate.** The lead picture on its mat, as wide as the frame allows and never taller than most of the screen, with its caption under it.
3. One reading column of about 60 characters: the byline, the text, and the extra pictures of the project, each on a mat.
4. Tags, share links, the author box.
5. **More work:** the related posts as a small wall of three.

## Other pages

- **Lists** (category, tag, archive): the list's name and intro, the discipline chips, then the same wall.
- **People** (`/authors/`): the shared people grid.
- **A person:** their mark, name, role and bio, then their work as a wall.

## Type pair

- Titles and the studio's name: a wide grotesque with character. The demo brand uses **Syne** 700.
- Text, labels and the menu: a calm grotesque that reads well at 18 px. The demo brand uses **Albert Sans** 400 and 600.

Both fonts are SIL OFL 1.1, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand. Neither has a reserved font name. The pack's own defaults are system sans stacks.

## Palette

The pack's own default: a plaster wall, pale mats, near-black ink and one vermilion accent.

| Token | Value | Check |
|---|---|---|
| background | `#efebe4` | the wall |
| surface | `#fbfaf7` | the mats |
| text | `#1b1a17` | 14.6:1 on background |
| muted | `#575249` | 6.5:1 on background, 7.4:1 on surface |
| link | `#a3301f` | 5.9:1 on background, 6.7:1 on surface |
| accent | `#a3301f` | light text on it 7.0:1 |
| border | `#d5cfc4` | |

The demo's sample brand is Fernbrook Press: a cooler plaster (`#ebe9e4`), near-black ink and a deep ultramarine (`#2b3f8f`).

## Signature device

**The mat and the wall label.** Every picture hangs on a pale mat with a hairline edge. Under it sits a small label: a number in tabular figures, the title, then the medium and year. The front page closes with the list of works under the same numbers. The site reads as a room you walk through, not a feed.

## How it differs from the existing templates

- **Aperture** shows one picture at a time, the full width of the page, as a story with camera details. Gallery shows many pictures at once on a wall, at their own shapes, with project details.
- **Atelier** crops every look to the same tall cell in an even grid, with spreads. Gallery never crops, hangs pictures in columns of mixed heights, and numbers them for a checklist.
- **Plinth** shows architecture projects as spreads with title blocks and plans. Gallery's project page leads with the picture on a mat and keeps the facts in a short sheet beside the title.
- **Keystone** frames pictures in a calm company grid with a lead story. Gallery has no lead story and no cards.

Gallery reuses the shared filter row and people grid. The wall and the checklist live in the pack: no other template on the list needs them yet.
