# Vital: design brief

Vital is an original Khalid template for a health and wellness site that publishes programmes and routines: a movement coach, a small studio, a walking group or a wellbeing writer. It should feel calm and practical, like a good class handout, not a magazine or a clinic.

## Who it is for

- A coach or a small studio that shares routines people can follow at home, such as a ten-minute stretch or a wind-down before sleep.
- A group that runs programmes over several weeks, such as a beginner walking plan.
- A writer who mixes routines with short notes, such as a new timetable or a walking group.

Vital is for general wellbeing. Its demo makes no medical claims, and its `care_note` option puts a plain safety line on every routine.

## The content model

Vital adds no front matter. It reads what Khalid already has:

- **A routine is a post with facts,** read by position: 1 how long a session takes, 2 the level, 3 how often or for how long, 4 what you need. The labels are the writer's own.
- **A note is a post without facts.** Notes show in a dated feed, not among the routines.
- **Programmes are categories,** such as Move well, Walk more or Breathe and rest. A category of notes only is left out of the programme list, by the same rule as Research's research areas.
- **Coaches are authors,** with their role.
- **Steps are the post's numbered list.** A routine writes its moves as an ordered list in the body.
- A series, when a programme has one post per week, shows Khalid's own series box.

## The front page

1. **Hero:** the site's name, its `tagline`, and a figure strip from `hero_stats`, such as routines, programmes and minutes a day.
2. **Programmes:** each category as a large rounded chip with a small ring mark.
3. **Routines:** the newest routines as session cards in a grid. Each card has its picture, its programme, its title, and the session ring with its time beside the level and how often.
4. **Notes** beside the care note on wide screens: the newest notes as a dated feed. `notes_count` sets how many.
5. The page links, the call to action band and the newsletter form.

## The post page

1. The programme as a small label, the title, the dek and the coach with their role and the date.
2. **The session panel:** a large ring with the time, then the level, how often and what you need, on a pale rounded panel. Under it, the `care_note` when it is set.
3. The lead picture, rounded.
4. The text in one column. Its numbered list is drawn as **the routine track**: each step's number in a ring, joined by a soft line down the margin.
5. Tags, share links, the author box, then related routines as session cards.

A note has no session panel and no track.

## Other pages

- **Lists** (a programme, a tag, the archive): the name and intro, then the posts as session cards.
- **Coaches** (`/authors/`): the shared people grid.
- **A coach:** their mark, role and bio, then their routines as session cards.

## Type pair

- Headings: **Young Serif** 400, a soft, round serif that feels warm without being cute.
- Text: **Figtree** 400 to 700, a friendly, very legible sans that reads well at 18 px.

Both are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files and self-hosted in the sample brand. The pack's own default uses the system's serif and sans.

## Palette

The pack's default is a sage green on a warm off-white, with a pale sage surface for the session panel:

| Token | Value | Check |
|---|---|---|
| background | `#fbfaf6` | |
| surface | `#edf1e8` | |
| text | `#1c2420` | 15.2:1 on background, 13.9:1 on surface |
| muted | `#4d5a52` | 6.9:1 on background, 6.3:1 on surface |
| link | `#2d6649` | 6.5:1 on background, 5.9:1 on surface |
| accent | `#2d6649` | white text on it 6.8:1 |
| border | `#d6ddd0` | |

The demo's sample brand, Fernhill Wellbeing, is a warm clay (`#9a3f1f`, 6.5:1 on its background) on a cream page (`#fffaf5`) with a peach surface (`#f8ebe0`).

## Signature device

**The session ring.** Every routine carries its time inside a round ring in the accent colour: small on each card, large on the post's session panel. The same ring shape numbers the steps of the routine track down the post, so a reader sees at a glance how long a session takes and how many moves it has. Shapes are soft throughout: pill chips, rounded cards and pictures, no hard rules.

## How it differs from the existing templates

- **Harbor** shows recipe facts as small tags on a card strip. Vital's facts are a session: the time sits in a ring, and the post opens with a session panel and a safety note.
- **Compass** opens each guide with its key points and a dashed trail. Vital has no key points box; its track is the post's own numbered list, drawn as rings on a soft line.
- **Research** is a text register with no pictures. Vital leads with pictures and rounded cards.
- **Syllabus** (an online course) orders lessons into modules with progress. Vital has no order or progress: routines stand alone, grouped by programme.

The session cards are a new shared section part (`sections/session_cards.html`), since a course's lessons, a recipe's times and an event's sessions can use the same card.
