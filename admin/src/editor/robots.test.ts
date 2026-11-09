import { describe, expect, it } from "vitest";
import { parseRobots, robotsText } from "./robots";

describe("robots directives", () => {
  it("reads each choice and keeps words it has no field for", () => {
    expect(parseRobots("noindex, follow, noarchive, max-snippet:50, notranslate")).toEqual({
      noindex: true,
      nofollow: false,
      restrictions: ["noarchive"],
      maxSnippet: "50",
      maxImagePreview: "",
      others: ["notranslate"],
    });
  });

  it("writes the words back in Khalid's order, and nothing for the defaults", () => {
    const robots = parseRobots("max-image-preview:standard, noimageindex, nofollow");
    expect(robotsText(robots)).toBe("nofollow, noimageindex, max-image-preview:standard");
    expect(robotsText(parseRobots("index, follow, max-image-preview:large"))).toBe("");
    expect(robotsText(parseRobots("noindex, follow"))).toBe("noindex");
  });
});
