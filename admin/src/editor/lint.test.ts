import { describe, expect, it } from "vitest";
import type { Problem } from "../api/posts";
import { findingsOf, lintFindings, scoreOf } from "./lint";

const NO_LINK = {
  rule: "no-internal-link",
  severity: "block",
  locale: "en",
  slug: "planning-a-topic-cluster",
  subject: null,
  what: "This post links to none of your other posts.",
  why: "Internal links are how readers and Google find the rest of your site.",
  fix: "Add one link to a related post.",
  guide: "internal-links",
  guide_url: "https://serpdad.com/khalid/docs/lint-rules/#internal-links",
  suggestions: [],
};
const FILLER = { ...NO_LINK, rule: "slug-filler-words", severity: "warn", subject: "a" };
const TIP = { ...NO_LINK, rule: "long-paragraph", severity: "info" };

function refused(findings: unknown[]): Problem {
  return {
    type: "https://khalid.dev/problems/lint-blocked",
    title: "The linter stopped this post.",
    status: 422,
    detail: "The linter found something that stops this post from going live.",
    fix: "Fix each finding marked block, then publish again.",
    lint: { blocks: true, findings },
  };
}

describe("lint findings from a refused publish", () => {
  it("reads every finding, blocks before warnings", () => {
    const findings = lintFindings(refused([FILLER, NO_LINK]));
    expect(findings.map((finding) => finding.rule)).toEqual([
      "no-internal-link",
      "slug-filler-words",
    ]);
    expect(findings[1]?.subject).toBe("a");
  });

  it("keeps tips after the warnings, each with its guide", () => {
    const findings = lintFindings(refused([TIP, FILLER, NO_LINK]));
    expect(findings.map((finding) => finding.severity)).toEqual(["block", "warn", "info"]);
    expect(findings[2]?.guide_url).toBe(NO_LINK.guide_url);
  });

  it("skips anything that is not a finding", () => {
    const odd = [{ rule: "x" }, "text", null, { ...NO_LINK, guide_url: 3 }, NO_LINK];
    expect(lintFindings(refused(odd))).toHaveLength(1);
  });

  it("finds nothing in a problem without lint, or no problem", () => {
    expect(lintFindings({ ...refused([]), lint: null })).toEqual([]);
    expect(lintFindings(null)).toEqual([]);
  });
});

describe("the score in a lint report", () => {
  it("reads the value and the points each rule cost", () => {
    const lint = {
      blocks: false,
      findings: [FILLER],
      score: { value: 95, lost: [{ rule: "slug-filler-words", points: 5 }, { rule: 3 }] },
    };
    expect(scoreOf(lint)).toEqual({ value: 95, lost: [{ rule: "slug-filler-words", points: 5 }] });
    expect(findingsOf(lint).map((finding) => finding.rule)).toEqual(["slug-filler-words"]);
  });

  it("is null for a report without one", () => {
    expect(scoreOf({ blocks: false, findings: [] })).toBeNull();
    expect(scoreOf({ score: { value: "high" } })).toBeNull();
    expect(scoreOf(null)).toBeNull();
  });
});
