# Port City Wire: art direction

Port City Wire is the made-up local news site of a made-up harbour city, Port Arden. Its photographs should look like a working harbour town photographed by a staff photographer on an ordinary week: ferries, quays, cranes, weather on the seafront, a library, a bakery counter, a muddy pitch. Plain, true and a little weathered, never a tourist brochure.

## Tone

- **Daylight and weather, as they were.** Overcast skies, low sun, spray and rain are welcome, because a local paper reports weather. No dramatic filters, no black and white, no night shots so dark they lose their subject. The one dusk photograph keeps its lamps and quays readable.
- **Places and things, not people.** People appear at a distance, as part of a scene: runners spread across a course, walkers on a promenade, cyclists up the street. No face is the subject of a photograph. One hand appears, sliding loaves out of an oven.
- **Working detail.** Rust on a bollard, a worn football on a chalk spot, the steel ramp of a car ferry, a card catalogue's brass handles. These close details give the feed its texture between the wider news scenes.
- **Nothing to read.** No brand, ship name, shop sign or place name readable in the picture. Candidates were turned down for a ferry operator's name on the hull, a ship's number painted on a deck, a museum sign about a real fish market, a bank's sign in a library, a brand on a running shoe and on a pencil case, and book spines with readable titles.
- **Nowhere real by name.** Every photograph was taken somewhere other than Port Arden, which does not exist. Alt text describes what is visible and never says the photo shows Port Arden.

## Palette

Cool harbour greys and blues, with warm accents:

- sea greys, slate blues and overcast whites from the harbour, ferry and storm photographs;
- red brick, sandstone and timber from the town hall, library and quay photographs;
- warm accents of orange (excavators, a rusty bollard, bread crusts) and green (pitches, parkland, harbour grass).

This sits well beside the template's white ground and vermilion accent, and the Wire sample brand's signal red: the photographs carry no strong reds of their own, so the red stays the page's signal colour for kickers and the breaking strip. Avoid photographs dominated by saturated red or by a single bright brand colour.

## Subject mix

Of the 31 photographs, 10 are close details and 21 are wider scenes, so about a third are close details (the brief asks for at least a third). A news site leads with places and events more than a recipe blog does, so the wider share is larger than Harbor Kitchen's.

- **Close details (10):** a ferry's stern ramp and wake, a football on a penalty spot, a rope on a rusty bollard, fish on ice, a sourdough loaf on a rack, loaves on a baker's peel, card catalogue drawers, rain on a window, a bicycle symbol on asphalt, a row of painted bicycle stands.
- **Wider scenes (21):** a small car ferry, a waiting area full of ferry traffic, a storm wave over a sea wall, a windy harbour with a breakwater, two promenade views after a storm, a brick town hall, a sandstone town hall, an empty council chamber, a press and public gallery, an amphibious excavator, an excavator on a harbour bank, a mobile crane by a harbour basin, a harbour at dusk, fishing boats at a quay, a football pitch, a cross country course, a school yard, two library interiors, library stacks.

## One ratio per slot

| Slot | Ratio | Source size | Notes |
|---|---|---|---|
| Lead picture, post and cards | 3:2 | 1600 to 2000 px wide | The template crops cards and the lead story to its own ratio from the same file. |
| In-body figure | 3:2 | 2000 px wide | Shown at the text width, about 40rem, with a caption. |
| Author mark | 1:1 | 160 px SVG | Drawn, one near-black ink on a light tint of the same ink. No photographs of people. |

The source files are not all 3:2. Of the 19 leads, 7 are 3:2, 7 are within a little of it (4:3 to 1.57:1), two are 16:9, one is 1.69:1, one is 2:1 and one is 1.23:1.

- The 4:3 leads lose about a tenth at the top and bottom, which none of their subjects needs.
- The 16:9 leads (the school yard and the cross country course) and the cycle lane lead lose a little from the sides. Their subjects are spread across the frame, so nothing important goes.
- The bakery lead is 2:1. The loaves sit in the middle of the frame, so a 3:2 crop from the sides keeps all of them and loses only the oven door frame on the left.
- The parking story's town hall is 1.23:1, so a 3:2 crop loses about a sixth at the top and bottom. The clock tower's dome sits close to the top edge and may be clipped. It is the one lead to watch in the picture review; if it is clipped, swap it with the brick town hall used for the budget story.

## Phone crop

On a phone, the lead picture fills the width and is cropped from the centre. Each lead was checked for a centred subject:

- Single subjects (the ferry, the excavator, the crane, the town halls, the bicycle symbol) sit in the middle third and survive a tight crop.
- Repeating textures (fish on ice, card drawers, waves, library stacks, the waiting cars) look the same at any crop.
- Scenes with a subject off centre were turned down for lead slots or used in the body. The storm wave breaks left of centre, but its spray fills the frame, so any crop shows it.

No photograph carries text, so no overlay contrast check is needed. If a later design sets the breaking strip or a headline over a lead picture, the bright leads (the storm spray, the library walkway and the overcast harbour) need a scrim to pass AA.

## Sources and licences

Every photograph comes from Wikimedia Commons through `scripts/demo-photo.py`, under CC0, public domain, CC BY or CC BY-SA. `photos.lock` records each one, and `ATTRIBUTION.md` and the photo credits page credit them.
