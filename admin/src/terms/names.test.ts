import { describe, expect, it } from "vitest";
import type { AdminCategory } from "../api/terms";
import { categoryName, filledNames, nameLanguages } from "./names";

const ON_PAGE: AdminCategory = {
  id: "on-page-seo",
  slug: "on-page-seo",
  names: { de: "Onpage-SEO", en: "On-page SEO" },
  posts: 4,
  updated_at: "2026-11-10T09:00:00Z",
};

describe("category names", () => {
  it("names categories in the site's languages, else in those they use, else English", () => {
    expect(nameLanguages(["en", "fr"], [ON_PAGE])).toEqual(["en", "fr"]);
    expect(nameLanguages(undefined, [ON_PAGE])).toEqual(["de", "en"]);
    expect(nameLanguages([], [])).toEqual(["en"]);
  });

  it("shows the name in the asked language, else the first, else the slug", () => {
    expect(categoryName(ON_PAGE, "en")).toBe("On-page SEO");
    expect(categoryName(ON_PAGE, "fr")).toBe("Onpage-SEO");
    expect(categoryName({ ...ON_PAGE, names: {} }, "en")).toBe("on-page-seo");
  });

  it("sends only the names filled in, trimmed", () => {
    expect(filledNames({ en: " Link building ", de: "  " })).toEqual({ en: "Link building" });
  });
});
