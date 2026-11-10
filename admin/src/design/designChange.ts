// What the Design screen sends: only the tokens that differ from the copy
// it read, and the locked list only when it changed.
import type { Design, DesignChange } from "../api/design";
import { type ColorToken, designWords } from "../strings/designWords";
import { contrast } from "./contrast";

/** The screen's values: every token by name, and the tokens a sync keeps. */
export type Values = { tokens: Record<string, string>; locked: string[] };

/** The values as the server sent them. */
export function valuesOf(design: Design): Values {
  return { tokens: { ...design.tokens }, locked: [...design.locked] };
}

/** The change from `design` to `values`, or `null` when nothing differs. */
export function designChange(design: Design, values: Values): DesignChange | null {
  const tokens: Record<string, string> = {};
  for (const [name, value] of Object.entries(values.tokens)) {
    if (design.tokens[name] !== value.trim()) {
      tokens[name] = value.trim();
    }
  }
  const sorted = (names: string[]) => [...names].sort().join(" ");
  const locked = sorted(values.locked) === sorted(design.locked) ? undefined : values.locked;
  if (Object.keys(tokens).length === 0 && locked === undefined) {
    return null;
  }
  return {
    updated_at: design.updated_at,
    ...(Object.keys(tokens).length === 0 ? {} : { tokens }),
    ...(locked === undefined ? {} : { locked }),
  };
}

/** How readable one colour is, as the screen shows it beside the colour. */
export type Readable =
  | { kind: "ground" }
  | { kind: "not-text" }
  | { kind: "not-color" }
  | { kind: "score"; ratio: number; passes: boolean; textOn: boolean; onPanels: boolean };

/** The colours drawn on the background and the surface, and the contrast each needs. */
const TARGETS: Partial<Record<ColorToken, number>> = {
  "colors.text": 4.5,
  "colors.muted": 4.5,
  "colors.link": 4.5,
  "colors.accent": 3,
};

/**
 * How readable `token` is with `tokens`: a colour's contrast on the page
 * background, or on the panels when only there it is too faint, and for
 * the panels themselves, the text's contrast on them.
 */
export function readable(token: ColorToken, tokens: Record<string, string>): Readable {
  const surface = tokens["colors.surface"] ?? "";
  if (token === "colors.background") {
    return { kind: "ground" };
  }
  if (token === "colors.border") {
    return { kind: "not-text" };
  }
  const color = tokens[token] ?? "";
  const target = TARGETS[token];
  if (target === undefined) {
    const ratio = contrast(tokens["colors.text"] ?? "", surface);
    if (ratio === null) {
      return { kind: "not-color" };
    }
    return { kind: "score", ratio, passes: ratio >= 4.5, textOn: true, onPanels: false };
  }
  const onPage = contrast(color, tokens["colors.background"] ?? "");
  const onPanels = contrast(color, surface);
  if (onPage === null || onPanels === null) {
    return { kind: "not-color" };
  }
  if (onPage >= target && onPanels < target) {
    return { kind: "score", ratio: onPanels, passes: false, textOn: false, onPanels: true };
  }
  const passes = onPage >= target;
  return { kind: "score", ratio: onPage, passes, textOn: false, onPanels: false };
}

/** The colours too faint to read with `tokens`, for the line above the table. */
export function faintCount(tokens: Record<string, string>): number {
  const scores = Object.keys(TARGETS).map((token) => readable(token as ColorToken, tokens));
  return scores.filter((score) => score.kind === "score" && !score.passes).length;
}

/** The words of a score, as the pill beside the colour says them. */
export function scoreWords(score: Readable): string {
  switch (score.kind) {
    case "ground":
      return designWords.checkedAgainst;
    case "not-text":
      return designWords.notText;
    case "not-color":
      return designWords.notColor;
    case "score":
      if (score.textOn) {
        return designWords.textOn(score.ratio);
      }
      if (score.onPanels) {
        return designWords.faintOnPanels(score.ratio);
      }
      return score.passes ? designWords.ratio(score.ratio) : designWords.tooFaint(score.ratio);
  }
}
