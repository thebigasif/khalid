import { render, screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { en } from "../strings/en";
import { LintPanel } from "./LintPanel";
import type { Finding } from "./lint";

const GUIDE = "https://serpdad.com/khalid/docs/lint-rules/#internal-links";

const NO_LINK: Finding = {
  rule: "no-internal-link",
  severity: "block",
  subject: null,
  what: "This post links to none of your other posts.",
  why: "Internal links are how readers and Google find the rest of your site.",
  fix: "Add one link to a related post.",
  guide_url: GUIDE,
};
const TIP: Finding = { ...NO_LINK, rule: "long-paragraph", severity: "info", what: "A tip." };

describe("the checks beside the post", () => {
  it("links each finding to its guide, in a new tab", () => {
    render(<LintPanel findings={[NO_LINK]} score={null} />);
    const check = screen.getByRole("region", { name: en.editor.blockTag });
    const guide = within(check).getByRole("link", { name: en.editor.guide });
    expect(guide).toHaveAttribute("href", GUIDE);
    expect(guide).toHaveAttribute("target", "_blank");
  });

  it("shows a tip as a tip, and no link when the blog sends no guide", () => {
    const { guide_url: _, ...older } = TIP;
    render(<LintPanel findings={[older]} score={null} />);
    const tip = screen.getByRole("region", { name: en.editor.tipTag });
    expect(within(tip).getByText("A tip.")).toBeVisible();
    expect(within(tip).queryByRole("link")).toBeNull();
  });
});
