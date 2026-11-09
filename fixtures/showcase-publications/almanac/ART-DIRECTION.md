# The Field Almanac: art direction

The Field Almanac is a made-up library of long guides to growing food at home and on an allotment. Its photographs should look like a working plot and a potting shed through the year: real soil, real seedlings and real harvests, close enough to see what the guide is talking about.

## Tone

- **Daylight on the plot.** Natural light, from soft spring overcast to low autumn sun and a frosty morning. No studio flash, no dark moody shots and no blur.
- **The subject is the crop or the soil.** Seedlings, leaves, seeds, roots and fruit fill the frame. No person is the subject of a picture; candidates of people harvesting potatoes and of hands planting celery were turned down for this reason.
- **Worn and used.** Plastic module trays dusty with compost, wicker baskets, galvanised cans, old canes and a fork with a weathered handle. Nothing bought for the photograph.
- **Nothing to read.** No labels, brand names, signs, watermarks or text. Candidates with handwritten plant labels, a moulded tray number and a compost site's sign were turned down for this reason.

## Palette

Earth and leaf colours on natural grounds:

- the browns of soil, compost, wicker and weathered wood;
- fresh greens of seedlings, bean pods, kale and lettuce;
- the warm reds and oranges of tomatoes, apples and beetroot.

They sit beside the Almanac template's moss green and warm paper because the photographs share its natural range. Two photographs lean away from it: the runner bean seeds (purple and black) and the phacelia field (lilac). Both are used where their colour is the point, and one is an in-body figure.

## Subject mix

Of the 30 photographs, 22 are close details and 8 are wider scenes, so about three quarters are close details (the brief asks for at least a third).

- **Close details (22):** chitted seed potatoes, three module trays of seedlings, cress, a squash seedling, a mix of vegetable seeds, white bean seeds, runner bean seeds, loam clods, an earthworm, the inside of a compost bin, a slug, runner bean pods, a tomato flower, a basket of tomatoes, a basket of apples, bunches of curing garlic, garlic bulbs and cloves, crimson clover, frosted kale and a basket of beetroot.
- **Wider scenes (8):** raised beds with a bean frame, a raised bed of salad, a row of compost bays, a group of watering cans, onions drying in a field, an apple tree with windfalls, a field of phacelia, and a spade and fork against a fence.

## One ratio per slot

| Slot | Ratio | Source size | Notes |
|---|---|---|---|
| Lead picture, post and cards | 3:2 | 2000 px wide | Every lead is landscape, from 3:2 to about 6:5. The template crops cards to its own ratio from the same file. |
| In-body figure | 3:2 | 2000 px wide | Shown at the text width, about 42rem, with a caption. |
| Author mark | 1:1 | 160 px SVG | Drawn in one moss-green ink on a light tint of it: a spade, soil layers with a worm, a sprouting seed and a crate. No photographs of people. |

The source files are not all 3:2. Of the 18 leads, 8 are 3:2 (one, the garlic bunches, is 1900 px wide), 6 are about 4:3, two (the squash seedling and the frosted kale) are 7:5, one (the white beans) is 5:4 and one (the apple basket) is 8:7. A 4:3 source loses about a tenth at the top and bottom in a 3:2 crop, which none of these subjects needs. The apple basket loses about a fifth top and bottom; the handle may be clipped, so it is the lead to watch in the picture review. In the body, the apple tree figure is square and shows in full.

## Phone crop

On a phone, the lead picture fills the width and is cropped from the centre. Each lead was checked for a centred subject:

- Single subjects (the slug, the squash seedling, the frosted kale, the garlic bulbs, the apple and tomato baskets) sit in the middle third and survive a tight crop.
- Repeating textures (seedling trays, potatoes in an egg tray, bean pods, white beans, soil clods, the inside of the compost bin, the phacelia field) look the same at any crop.
- The plot lead (raised beds behind a bean frame) is busy at full width but keeps a bed and the frame in the centre.

No photograph carries text, so no overlay contrast check is needed. If a later design sets text over a lead picture, the darker leads (the squash seedling, the compost bin, the garlic bulbs on grey) need a scrim to pass AA.

## Sources and licences

Every photograph comes from Wikimedia Commons through `scripts/demo-photo.py commons` and `fetch`, under CC0, the public domain, CC BY or CC BY-SA. `photos.lock` records each one, and `ATTRIBUTION.md` and the photo credits page credit them. Three Commons entries by the same photographer carried a contact note in the author field, which the fetch printed into `photos.lock` as a multi-line string; those three `creator` and `credit` values were trimmed by hand to the photographer's name.
