import { describe, expect, it } from "vitest";
import type { Design } from "../api/design";
import { designWords } from "../strings/designWords";
import { contrast } from "./contrast";
import { designChange, faintCount, readable, scoreWords, valuesOf } from "./designChange";

const TOKENS = {
  "colors.background": "#ffffff",
  "colors.surface": "#f6f7f9",
  "colors.text": "#1f2328",
  "colors.muted": "#57606a",
  "colors.link": "#0b57d0",
  "colors.accent": "#0b57d0",
  "colors.border": "#d0d7de",
};

const DESIGN: Design = {
  tokens: TOKENS,
  locked: ["colors.link"],
  fonts: [],
  contrast: [],
  updated_at: null,
};

describe("the contrast beside each colour", () => {
  it("rounds down as the server does, and reads #rgb", () => {
    expect(contrast("#1f2328", "#ffffff")).toBe(15.7);
    expect(contrast("#000", "#fff")).toBe(21);
    expect(contrast("#777777", "#ffffff")).toBe(4.4);
    expect(contrast("blue", "#ffffff")).toBeNull();
  });

  it("scores text on the page, panels by the text on them, and lines not at all", () => {
    expect(scoreWords(readable("colors.text", TOKENS))).toBe(designWords.ratio(15.7));
    expect(readable("colors.surface", TOKENS)).toMatchObject({ textOn: true, passes: true });
    expect(scoreWords(readable("colors.border", TOKENS))).toBe(designWords.notText);
    expect(faintCount(TOKENS)).toBe(0);
  });

  it("calls a colour faint on panels when only there it fails", () => {
    const panels = { ...TOKENS, "colors.surface": "#57606a" };
    const muted = readable("colors.muted", panels);
    expect(muted).toMatchObject({ kind: "score", passes: false, onPanels: true });
    expect(faintCount(panels)).toBeGreaterThan(0);
  });
});

describe("what a save sends", () => {
  it("sends nothing when nothing changed", () => {
    expect(designChange(DESIGN, valuesOf(DESIGN))).toBeNull();
  });

  it("sends only the changed tokens, trimmed, and the locked list when it changed", () => {
    const values = valuesOf(DESIGN);
    values.tokens["colors.link"] = " #b3261e ";
    expect(designChange(DESIGN, values)).toEqual({
      updated_at: null,
      tokens: { "colors.link": "#b3261e" },
    });
    values.locked = ["colors.text", "colors.link"];
    expect(designChange(DESIGN, values)?.locked).toEqual(["colors.text", "colors.link"]);
  });
});
