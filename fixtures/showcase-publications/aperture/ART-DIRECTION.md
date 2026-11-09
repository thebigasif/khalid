# Northlight: art direction

Northlight is a made-up photo journal of the north. Its photographs are the subject of the site, not decoration, so they must read as one person's work: one eye, one camera, one way of seeing.

## One photographer

Every photograph is by Simo Räsänen (Ximonic) from Wikimedia Commons, under CC BY-SA 4.0. All thirteen were made with the same camera, a Canon EOS 5D Mark IV, with five lenses: 14mm, 16-35mm, 24-70mm, 50mm and 70-200mm. The camera details on each post come from the data stored in the original file on Commons, so they are true for the picture shown. Each lead picture carries his credit as its caption.

## Tone

- **Place and light are the subject.** Fells, fjords, forests, rivers and mountain villages. Animals may appear; no person is the subject of a picture.
- **Natural light, true colour.** Midday clarity, low winter sun, long northern evenings. No heavy filters.
- **Nothing to read.** No signs or text in the picture. The bridge in Helsinki shows no lettering at this size.

## Palette

Greens of birch and fell, the blues of fjord and lake, the white and grey of snow, and the warm gold of larch and low sun. They sit well on the template's warm white and the sample brand's fjord blue. No picture is dominated by red or purple.

## Subject mix

Of the 13 photographs, 4 are close or middle-distance subjects and 9 are wide views.

- **Close (4):** frozen spray on a rapid, a band of light on snow between trunks, reindeer on a ridge, duckboards through cotton grass.
- **Wide (9):** a waterfall, two fjords, a village in morning mist, the Marmolada, the Sexten Dolomites, a bridge in evening light, the lakes below Saana, wooden stairs up a cliff, a hanging bridge.
- **Shapes:** 2 panoramas wider than 2:1, 1 upright picture, 10 close to 3:2. Aperture keeps every shape, so the front page has a rhythm.

## One ratio per slot

| Slot | Ratio | Source size | Notes |
|---|---|---|---|
| Story and post picture | The picture's own | 1600 px wide | Drawn at the frame's width, about 81rem, and never cropped. A tall picture is at most 88% of the screen's height, in the middle. |
| Row on a list page | The picture's own | 1600 px wide | Two thirds of the frame, at most 80% of the screen's height. |
| Contact sheet | 3:2 | 1600 px wide | Cropped to 3:2 from the middle. The upright winter picture keeps its band of light in the middle third. |
| Author mark | 1:1 | 160 px SVG | Drawn, one ink colour on a light tint of the same ink. No photographs of people. |

## Phone crop

On a phone every picture fills the width and keeps its shape, so nothing is cropped. The panoramas are short at that width, which reads as a panorama should. In the contact sheet each subject sits in the middle third: the reindeer, the bridge pylon, the stairs and the band of light all survive a 3:2 crop.

## Sources and licences

Every photograph was fetched from Wikimedia Commons through `scripts/demo-photo.py`, which read its CC BY-SA 4.0 licence on the file's own page. `photos.lock` records each one, and `ATTRIBUTION.md` and the photo credits page credit them.
