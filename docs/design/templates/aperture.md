# Aperture: design brief

Aperture is an original Khalid template for one photographer. Each post is one picture and the story behind it. It should feel like turning the pages of a photo book, one picture at a time, not like scanning a wall of thumbnails.

## Who it is for

- A photographer who publishes one picture at a time, with a few paragraphs on the place, the light and the settings.
- A photo journal or a walking blog where every post leads with a single strong picture.
- A camera club or a teacher who wants every picture to show its settings, so readers can learn from them.

## The content model

Aperture adds no front matter. It reads what Khalid already has:

- **A story is a post with a lead picture.** The picture's caption carries the credit.
- **The camera details are facts,** read by position: 1 the camera, 2 the lens, 3 the exposure, 4 the date taken. The labels are the writer's own, so "Film" or "Phone" work too.
- **Places or series are categories.** Each category page is that place's pictures. Tags are subjects, such as winter or wildlife.

## The front page

1. **Masthead line.** The photographer's name and the tagline on one line over a hairline. Small, so the first picture is near the top.
2. **Stories.** The newest posts (option `stories`, five by default), one under the other. Each picture fills the frame at its own shape; under it, the caption row: place and date in a monospace, the title and summary on the start side, the exposure plate on the end side.
3. **Contact sheet.** The other posts on the page as small 3:2 pictures in a light strip that scrolls sideways, each with its title, place and date.
4. Numbered page links, the call to action band and the newsletter.

## The post page

1. The picture first, the full width of the frame, never cropped. A tall picture keeps its shape in the middle. The credit sits under it on the end side.
2. **The exposure plate:** four ruled cells in a row.
3. One calm column of about 36rem: the place, the title, the summary in italics, the author and date, then the story.
4. Tags, share links, the author box.
5. **More pictures:** related posts as a contact sheet.

## Other pages

- **Lists** (category, tag, archive): one picture a row, two thirds of the frame, with the words and plate beside it. Never a grid.
- **People** (`/authors/`): the shared people grid.
- **A person:** their bio, then their stories as rows.

## Type pair

- Words: a book serif for titles and text. The pack's own default is Iowan Old Style, then Charter and Georgia. The demo brand uses **Newsreader** 400 and 500, a serif with an optical size made for reading on screens.
- Camera details, the menu and labels: a monospace. The demo brand uses **JetBrains Mono** 400.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand. Neither has a reserved font name.

## Palette

The pack's own default: a warm white like a print's border, near-black ink and one rust accent like a darkroom's safe light.

| Token | Value | Check |
|---|---|---|
| background | `#faf9f6` | |
| surface | `#f0eee8` | |
| text | `#1c1b19` | 16.4:1 on background |
| muted | `#5c5850` | 6.7:1 on background, 6.1:1 on surface |
| link | `#9a3412` | 6.9:1 on background, 6.3:1 on surface |
| accent | `#9a3412` | white text on it 7.3:1 |
| border | `#dedad0` | |

The demo's sample brand is Northlight: a cool white (`#fbfaf8`), near-black ink and a deep fjord blue (`#1f5a73`).

## Signature device

**The exposure plate.** Under every picture, the camera, lens, exposure and date sit in a strip of ruled cells set in a monospace, with tiny spaced labels, like the printed edge of a roll of film. It shows on the front page, on each post and on every list, so the whole site reads as one photographer's notebook of how each picture was made.

## How it differs from the existing templates

- **Edge** (a Ghost port) makes the front page a masonry wall of pictures with no words. Aperture shows one picture at a time, the full width of the frame, each with its words and plate under it.
- **London** (a Ghost port) shows tall tiles two side by side, every third across the width. Aperture never puts two pictures side by side in its stories or lists; only the contact sheet shows small pictures, in one sideways strip.
- **Keystone** and **Workbench** show facts as a credits line or a score panel. Aperture's facts are camera details in a film-edge plate under the picture.
- **Atlas** and **Harbor** are picture-led magazines with cards. Aperture has no card grid at all.

No section part fits the plate or the contact sheet closely enough to share yet; both live in the pack. Aperture reuses the shared people grid.
