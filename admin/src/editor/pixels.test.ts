import { describe, expect, it } from "vitest";
import { fitted, widthOf } from "./pixels";

describe("search result widths", () => {
  it("measures wide letters wider than narrow ones", () => {
    expect(widthOf("WWW", 20)).toBeGreaterThan(widthOf("iii", 20) * 3);
    expect(widthOf("abc", 10)).toBe(16.12);
  });

  it("keeps a line that fits, and cuts a long one after a whole word", () => {
    expect(fitted("Choosing anchor text", 600, 20)).toEqual({
      shown: "Choosing anchor text",
      cut: false,
    });
    const long = "Choosing anchor text that tells readers and search engines where a link goes";
    const line = fitted(long, 600, 20);
    expect(line.cut).toBe(true);
    expect(line.shown.endsWith(" …")).toBe(true);
    expect(long.startsWith(line.shown.replace(" …", ""))).toBe(true);
    expect(widthOf(line.shown, 20)).toBeLessThanOrEqual(600);
  });
});
