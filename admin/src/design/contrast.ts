// The WCAG 2.2 contrast of two colours, worked out as a colour is typed,
// so the score beside it moves before anything is saved. The same sums
// as `Rgb::contrast` in khalid-theme, which the server checks with.

/** One colour's red, green and blue, each from 0 to 255. */
type Channels = [number, number, number];

/** The channels of `#rrggbb` or `#rgb`, or `null` for anything else. */
export function channels(text: string): Channels | null {
  const digits = /^#([0-9a-f]{3}|[0-9a-f]{6})$/i.exec(text.trim())?.[1];
  if (digits === undefined) {
    return null;
  }
  const full = digits.length === 3 ? [...digits].map((digit) => digit + digit).join("") : digits;
  const at = (start: number) => Number.parseInt(full.slice(start, start + 2), 16);
  return [at(0), at(2), at(4)];
}

/** How bright a colour looks, from 0 for black to 1 for white. */
function luminance([red, green, blue]: Channels): number {
  const linear = (channel: number) => {
    const share = channel / 255;
    return share <= 0.04045 ? share / 12.92 : ((share + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * linear(red) + 0.7152 * linear(green) + 0.0722 * linear(blue);
}

/**
 * The contrast of two colours, such as 15.7 for 15.7 to 1, rounded down to
 * one decimal so a failing 4.47 never reads as 4.5; `null` when either is
 * not a colour yet.
 */
export function contrast(one: string, other: string): number | null {
  const first = channels(one);
  const second = channels(other);
  if (first === null || second === null) {
    return null;
  }
  const light = Math.max(luminance(first), luminance(second));
  const dark = Math.min(luminance(first), luminance(second));
  return Math.floor(((light + 0.05) / (dark + 0.05)) * 10) / 10;
}
