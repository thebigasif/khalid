# Tern Notes: art direction

Tern Notes is the blog of the made-up team that makes Tern, a note-taking app. Its photographs should look like the desk and the building of a small, careful product team: paper, pencils, plain tools, a plant by the window and the quiet details of the rooms they work in. Nothing in them should look like a software advert.

## Tone

- **Quiet studio light.** Soft daylight from a window, or even light on a pale ground. No hard flash, no neon, no night scenes. Two candidates of lit walls at night were turned down as too dark.
- **Objects, not people.** Paper, pencils, a ruler, thread, a plane and a plant fill the frame. Hands may appear when they show work being done, such as sketching or ruling a line, but no face is the subject of a picture.
- **Made by hand.** Worn wood, pencil shavings, folded paper and riveted metal: things that show they were made or used with care, which is what the essays are about.
- **Nothing to read.** No screens with text, no book spines, no labels, no brand names and no logos. This rule turned down more candidates than any other: book stacks and library shelves with readable titles, a typewriter and a computer with their makers' names, a paper bag with a printed slogan, packaged index cards, a saw in its retail packet and a carpenter's pencil printed with a company name.

## Palette

Warm neutrals with a few soft colours, so the photographs sit calmly beside Folio's graphite ink, warm grey panels and single ochre accent:

- paper whites, pale concrete, soft greys and graphite;
- warm wood, kraft paper and the ochre of a skylight or a lit stair;
- small notes of colour: plant greens, thread pinks and blues, a yellow pencil.

Avoid photographs dominated by saturated blue, purple or black. The concrete corner's blue sky is the one strong colour and fills only a narrow edge of its frame.

## Subject mix

Of the 26 photographs, 19 are close details and 7 are wider scenes, so close details are well over the third the brief asks for.

- **Close details of objects (13):** wound thread on bobbins, a pen on a cloth-bound notebook, a hand writing with a pencil, a cube of note paper, an open planner page, two pencils on yellow card, a sharpener and shavings, hands ruling a line, a paper aeroplane, a block plane with shavings, an orchid, a seedling in moss, a potted plant seen from above.
- **Architectural details (6):** a curved white wall, a concrete corner, riveted metal panels, a skylight at the top of a spiral stair, a wooden spiral stair from above, a window's shadow on a curtain.
- **Wider scenes (7):** a hand sketching screen layouts at a table, notebooks and coffee on floorboards, a plant on a window sill, a library card catalogue, a bookshelf, paper cranes on a table, a succulent on a long table.

Release notes use details and architecture, because a release is about one precise change. Essays use objects in use and wider scenes, because they are about people working.

## One ratio per slot

| Slot | Ratio | Source size | Notes |
|---|---|---|---|
| Lead picture, post and cards | 3:2 | 2000 px wide | Every lead was chosen in landscape at 3:2 or 4:3, apart from the paper cranes at 16:9. The template crops cards to its own ratio from the same file. |
| In-body figure | 3:2 | 2000 px wide | Shown at the text width, about 40rem, with a caption. |
| Author mark | 1:1 | 160 px SVG | Drawn, one graphite ink on a light tint of the same ink. No photographs of people. |

Of the 18 leads, 13 are 3:2, two are about 4:3 (the sticky-note cube and the card catalogue), two are close to 5:3 (the pen on a notebook and the block plane) and one is 16:9 (the paper cranes). A 4:3 source loses about a tenth at the top and bottom in a 3:2 crop, which none of these subjects needs. The 16:9 cranes lose a little at the sides, where there is only more table.

## Phone crop

On a phone, the lead picture fills the width and is cropped from the centre. Each lead was checked for a centred subject:

- Single subjects (the orchid, the plane, the paper cube, the potted plant, the pen) sit in the middle third and survive a tight crop.
- Repeating textures (thread, the card catalogue, riveted panels, the skylight) look the same at any crop.
- Off-centre subjects were placed in the body, where they show at full width: the pencils on yellow card and the window shadow. The paper aeroplane lead keeps the whole plane inside the middle 70 percent, and the concrete corner keeps its edge near the centre.

No photograph carries text, so no overlay contrast check is needed. If a later design sets text over a lead picture, the darker leads (the plane, the card catalogue, the riveted panels) need a scrim to pass AA.

## Sources and licences

Every photograph comes from Wikimedia Commons through `scripts/demo-photo.py commons`, under CC0, CC BY-SA 2.0, CC BY-SA 3.0 or CC BY-SA 4.0. Most are photographs their authors released under CC0 on another site and that were copied to Commons with that licence. `photos.lock` records each one, and `ATTRIBUTION.md` and the photo credits page credit them.
