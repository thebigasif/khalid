// Google cuts a result's title and description by their width in pixels,
// not by how many characters they have. These are Arial's widths, the font
// of Google's results, in thousandths of the font size, so the preview cuts
// where Google would without measuring in a browser.

const WIDTHS: [number, string][] = [
  [191, "'"],
  [222, "ijl"],
  [260, "|"],
  [278, " ftI.,:;!/·[]"],
  [333, "r-()"],
  [355, '"'],
  [500, "cksvxyzJ"],
  [556, "abdeghnopqu0123456789L?»$#"],
  [584, "+=<>"],
  [611, "FTZ"],
  [667, "ABEKPSVXY&"],
  [722, "CDHNRUw"],
  [778, "GOQ"],
  [833, "mM"],
  [944, "W"],
];

const WIDTH = new Map(WIDTHS.flatMap(([width, chars]) => [...chars].map((c) => [c, width])));

// A letter not listed takes an average letter's width; a wide script, such
// as Chinese, takes the whole font size.
function widthOfChar(char: string): number {
  const listed = WIDTH.get(char);
  if (listed !== undefined) {
    return listed;
  }
  return (char.codePointAt(0) ?? 0) >= 0x2e80 ? 1000 : 556;
}

/** How wide `text` is, in pixels, at `size` pixels. */
export function widthOf(text: string, size: number): number {
  let total = 0;
  for (const char of text) {
    total += widthOfChar(char);
  }
  return (total * size) / 1000;
}

/** How much of a result's line Google shows, by device. */
export const SERP_LINES = {
  desktop: { title: { width: 600, size: 20 }, description: { width: 920, size: 14 } },
  mobile: { title: { width: 680, size: 18 }, description: { width: 680, size: 14 } },
} as const;

/** A device Google shows results on. */
export type Device = keyof typeof SERP_LINES;

/** A line as Google shows it, and whether Google cut it. */
export type Fitted = { shown: string; cut: boolean };

/**
 * `text` as Google shows it in `width` pixels at `size`: whole if it fits,
 * else cut after the last whole word that fits with an ellipsis.
 */
export function fitted(text: string, width: number, size: number): Fitted {
  if (widthOf(text, size) <= width) {
    return { shown: text, cut: false };
  }
  const room = width - widthOf(" …", size);
  let shown = "";
  for (const word of text.split(" ")) {
    const next = shown === "" ? word : `${shown} ${word}`;
    if (widthOf(next, size) > room) {
      break;
    }
    shown = next;
  }
  return { shown: `${shown} …`.trimStart(), cut: true };
}
