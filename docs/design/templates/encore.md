# Encore: design brief

Encore is an original Khalid template for a musician or a band. It shows the music first: the newest release, the tour dates and every record, with the band's notes from the studio and the road beside them. It should feel like a gig poster and a record shelf, not a blog.

## Who it is for

- A band or a solo musician who wants one home for releases, shows and news.
- A small label or a composer with a short catalogue of records.
- An artist who writes studio diaries and tour notes between releases.

## The content model

Encore adds no front matter. It reads what Khalid already has:

- **A release is a post with facts.** The facts are read by position: 1 the format (album, EP, single), 2 the release date, 3 the label, 4 where to listen. A fourth fact that starts with `https://` becomes a Listen button. The labels are the writer's own.
- **A note is a post without facts:** a studio diary, a tour story, news.
- **Band members are authors.** Their role says what they play.
- **Tour dates are a template option,** `tour_dates`, one show a line: `date | city | venue | ticket link or a word such as Sold out`. They are site settings, not posts, so a date can be added or taken out in one line.
- **Where to listen** is the `listen_on` option, `name | address`, as in Episode.
- Categories sort the posts, such as Releases, Studio and Tour diary.

## The front page

1. **Hero.** The band's name very large in the poster face, its tagline, and the places to listen as a row of plain links.
2. **Out now.** The newest release large: its cover on a sleeve with the record half out, beside the title, the facts and the Listen button.
3. **On tour.** The tour dates as a poster list: the day number large, the month under it, the city and venue, and a ticket link or a note such as Sold out. Nothing shows without dates.
4. **Releases.** Every other release on the page as a shelf of sleeves, each with its title, format and year.
5. **Notes.** The newest notes as picture cards: date, category, title.
6. The numbered page links, the call to action band and the newsletter form.

## The post page

1. The category as a small label, the title in the poster face, the dek, the author with their role and the date.
2. **A release** opens with its cover on the sleeve beside the liner panel: the facts in a ruled list and the Listen button. The text follows, such as the track list and the story of the record.
3. **A note** opens with its lead picture at the text width.
4. Tags, share links and the author box, then related posts as sleeves and note cards.

## Other pages

- **Lists** (category, tag, archive): the list's name and intro, then releases as sleeves and notes as cards.
- **The band** (`/authors/`): the people grid, each with their mark, name and what they play.
- **A member:** their mark, role and bio, then their posts.

## Type pair

- Headings: **Anton** 400, a tall condensed grotesque like a gig poster. Used in capitals for the band's name, the tour days and section labels.
- Text: **Archivo** 400 to 700, a sturdy grotesque that reads well at 18 px and has clear figures for dates.

Both are SIL OFL, from the Google Fonts repository on GitHub, each with its OFL file, and self-hosted in the sample brand.

## Palette

The pack's own default is poster paper and ink with one vermilion accent:

| Token | Value | Check |
|---|---|---|
| background | `#f4f1ea` | |
| surface | `#e8e2d4` | |
| text | `#151413` | 16.3:1 on background, 14.3:1 on surface |
| muted | `#524d45` | 7.4:1, 6.5:1 on surface |
| link | `#a8261a` | 6.3:1, 5.5:1 on surface |
| accent | `#c42e1c` | 5.0:1, white text on it 5.6:1 |
| border | `#cdc5b4` | |

The demo's sample brand, a made-up Glasgow band called Northlight Parade, is deep violet (`#5532d0`) on warm white with a pale lilac surface.

## Signature device

**The sleeve.** Every release cover sits on its sleeve with a black record pulled halfway out of the side, drawn in CSS from the tokens, with grooves and a label in the accent colour. On hover the record slides out a little further. The same sleeve shows on the front page, in every list and at the top of a release, so the site reads as a record shelf. The tour list's huge day numbers in the poster face are the second mark.

## How it differs from the existing templates

- **Episode** is a podcast's feed of boxed episodes with a player. Encore has no player: it is a catalogue of records with sleeves, a tour list and a liner panel.
- **Folio** has a dated release log for software. Encore's releases are records with covers, formats and labels.
- **Research** lists posts with facts as a text register. Encore shows them as pictures on sleeves and adds a dated tour list from an option.
- **Keystone** and **Casper** are picture blogs. Encore splits releases from notes and leads with music, not the newest post.

## Shared parts

Encore uses the shared hero and people grid. Its tour list is its own for now: the shared schedule part on the planned list (conference, sports fixtures) can take it over when it lands.
