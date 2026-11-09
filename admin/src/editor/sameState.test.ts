import { describe, expect, it } from "vitest";
import { circularDeepEqual } from "./sameState";

describe("telling whether an editor selector's answer changed", () => {
  it("finds equal yes-or-no answers, numbers and texts the same", () => {
    expect(circularDeepEqual(true, true)).toBe(true);
    expect(circularDeepEqual(true, false)).toBe(false);
    expect(circularDeepEqual(Number.NaN, Number.NaN)).toBe(true);
    expect(circularDeepEqual(0, -0)).toBe(true);
    expect(circularDeepEqual("a", "b")).toBe(false);
    expect(circularDeepEqual(null, undefined)).toBe(false);
  });

  it("compares an open menu by what it holds, not by which object it is", () => {
    const open = { kind: "slash", query: "ta", from: 4, to: 7 };
    expect(circularDeepEqual(open, { ...open })).toBe(true);
    expect(circularDeepEqual(open, { ...open, query: "tab" })).toBe(false);
    expect(circularDeepEqual(open, null)).toBe(false);
    expect(circularDeepEqual({ a: undefined }, { b: undefined })).toBe(false);
    expect(circularDeepEqual([1, [2]], [1, [2]])).toBe(true);
    expect(circularDeepEqual([1], { 0: 1 })).toBe(false);
  });

  it("stops on values that hold themselves", () => {
    const a: Record<string, unknown> = { name: "a" };
    a.self = a;
    const b: Record<string, unknown> = { name: "a" };
    b.self = b;
    expect(circularDeepEqual(a, b)).toBe(true);
    b.name = "b";
    expect(circularDeepEqual(a, b)).toBe(false);
  });

  it("counts other kinds of object as changed, which only draws once more", () => {
    expect(circularDeepEqual(new Date(0), new Date(0))).toBe(false);
    const day = new Date(0);
    expect(circularDeepEqual(day, day)).toBe(true);
  });
});
