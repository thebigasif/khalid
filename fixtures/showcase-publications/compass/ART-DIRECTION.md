# Northbound: art direction

Northbound is the blog of a made-up evening coding school for adults. Its photographs should feel like a calm desk at the start of an evening's practice, and like a walk on a marked trail: plain tools, good daylight, nothing that shouts. The trail pictures carry the template's signature device, the dashed trail of numbered stops, into the photographs as a metaphor for the learning path.

## Tone

- **Quiet and bright.** Daylight or soft window light, pale wood, white desks, paper. No neon keyboards, no dark "hacker" rooms, no glowing screens.
- **Tools, not people.** Hands may appear when they do something, such as typing or holding a compass, but no face is the subject of a picture.
- **Screens stay dark or out of view.** No readable code, text or interface on any screen, so no picture contradicts the code in a guide.
- **Nothing to read.** No brand logos, labels or watermarks. Candidates were turned down for a supermarket logo on a trail marker, a fibre-optic maker's badge, a network switch maker's logo, a printed shipping label, a ruler's brand name, a laptop model name and a coffee cup print. Numbers on a compass dial, a ruler's scale and a map's control circles are kept, because they read as texture, not as words.

## Palette

Neutral grounds with one warm or green accent per picture:

- pale wood, white desks, paper and grey steel;
- the greens and browns of woodland paths and fields;
- single colour accents: the red jigsaw piece, the coloured pencils, the copper wires of a cable.

These sit well with the template's teal and the Compass sample brand's sky blue, because the grounds are neutral and the accents are small. Avoid photographs dominated by black, purple or saturated blue.

## Subject mix

Of the 27 photographs, 15 are close details and 12 are wider scenes, so more than half are close details (the brief asks for at least a third).

- **Close details (15):** a compass in a hand, a magnifying glass on a ruler, a red jigsaw piece, the cut end of a network cable, a network plug, hands typing, coloured pencils, a notebook's spiral binding, a printer's type case, keyboard keys, a compass on an orienteering map, glasses on a notebook, a blank exercise book, a phone in a case, a paper boat in a puddle.
- **Wider scenes (12):** a laptop and mug on a desk, an alarm clock on a wooden table, a laptop and coffee from above, a desk with a laptop and books, a white desk from above, a planner with pencils, a woodland path with a marker post, two forks in a footpath, mossy stone steps, a hillside trail with a mountain, hiking boots on sand.

The desk pictures go with the guides you do at a computer (the first program, HTML, JavaScript, the editor, the developer tools). The path and trail pictures go with the guides about direction and choice: starting out, Git's branches and commits, and what to learn next.

## One ratio per slot

| Slot | Ratio | Source size | Notes |
|---|---|---|---|
| Lead picture, post and cards | 3:2 | 2000 px wide | Every lead was chosen in landscape. The template crops cards to its own ratio from the same file. |
| In-body figure | 3:2 | 1540 to 2000 px wide | Shown at the text width, about 39rem, on Compass's pale mat with a caption. |
| Author mark | 1:1 | 160 px SVG | Drawn in one ink colour on a light tint of the same ink. No photographs of people. |

The source files are not all 3:2. Of the 18 leads, 10 are 3:2, 6 are 4:3, one, the magnifying glass, is 1.61:1 and one, the keyboard keys on the terminal guide, is 1.87:1. A 4:3 source loses about a tenth at the top and bottom in a 3:2 crop, which none of these subjects needs. The keyboard is a repeating texture, so a crop from the sides costs nothing. The magnifying glass loses a little from the sides; its lens sits in the middle.

## Phone crop

On a phone, the lead picture fills the width and is cropped from the centre. Each lead was checked for a centred subject:

- Single subjects (the compass, the magnifying glass, the cable end, the type case, the phone, the paper boat) sit in the middle third and survive a tight crop.
- Repeating textures (the keyboard, the coloured pencils) look the same at any crop.
- Desk scenes from above keep their main object, the laptop or the notebook, inside the middle 70 percent. The white desk on the developer tools guide has a laptop corner at the right edge; losing it on a phone costs nothing.
- The fork in the path keeps the split in the centre, so both branches stay in frame.

No photograph carries text, so no overlay contrast check is needed. If a later design sets text over a lead picture, the darker leads (the coloured pencils on black cloth, the paper boat on red earth) need a scrim to pass AA.

## Sources and licences

Every photograph comes from Wikimedia Commons or the WordPress photo directory through `scripts/demo-photo.py`, under CC0, CC BY or CC BY-SA. `photos.lock` records each one, and `ATTRIBUTION.md` and the photo credits page credit them.
