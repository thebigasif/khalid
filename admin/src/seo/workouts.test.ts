import { describe, expect, it } from "vitest";
import type { LinkSuggestion } from "../api/seo";
import { pillarOf, relatedTo } from "./workouts";

/** A suggestion of the post `id`, sharing only a category unless `pillar`. */
function suggestion(id: string, pillar = false): LinkSuggestion {
  const shares = { tags: [], pillar, cluster: pillar ? "internal-links" : null };
  return { id, title: id, path: `/blog/${id}/`, score: 3, shares: { ...shares, category: "seo" } };
}

describe("the workouts' steps", () => {
  it("finds the pillar a member does not link to yet", () => {
    const pillar = suggestion("internal-links-guide", true);
    expect(pillarOf([suggestion("sitemaps"), pillar])).toEqual(pillar);
    expect(pillarOf([suggestion("sitemaps")])).toBeNull();
  });

  it("keeps an orphan's closest posts, best first", () => {
    const all = ["a", "b", "c", "d"].map((id) => suggestion(id));
    expect(relatedTo(all).map((each) => each.id)).toEqual(["a", "b", "c"]);
    expect(relatedTo([])).toEqual([]);
  });
});
