import { describe, expect, it } from "vitest";
import source from "./router.tsx?raw";

// The screens an owner opens now and then, with tables, charts or many
// settings parts. Each loads when opened, so the first download stays
// inside its budget in docs/standards/performance.md.
const LATER = [
  "Analytics",
  "Calendar",
  "Editor",
  "Forms",
  "Navigation",
  "Privacy",
  "SearchConsole",
  "SeoSettings",
  "Snippets",
  "Subscribers",
  "Tags",
];

describe("the admin's router", () => {
  it.each(LATER)("loads the %s screen only when it is opened", (screen) => {
    expect(source).not.toMatch(new RegExp(`^import .* from "\\./${screen}";$`, "m"));
    expect(source).toContain(`lazyRouteComponent(() => import("./${screen}"), "${screen}")`);
  });
});
