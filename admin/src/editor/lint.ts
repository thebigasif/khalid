import type { Problem } from "../api/posts";

/**
 * One lint finding, as `khalid lint --json` and the Admin API print it. The
 * OpenAPI document gives a refused publish's `lint` as a plain object, so
 * this narrows it field by field rather than trusting it.
 */
export type Finding = {
  rule: string;
  severity: Severity;
  subject: string | null;
  what: string;
  why: string;
  fix: string;
  /** The address of the guide that explains the rule; an older blog sends none. */
  guide_url?: string;
};

/** How much a finding matters, heaviest first; `info` is a tip that costs no points. */
const SEVERITIES = ["block", "warn", "info"] as const;
type Severity = (typeof SEVERITIES)[number];

function isFinding(value: unknown): value is Finding {
  if (typeof value !== "object" || value === null) {
    return false;
  }
  const texts = ["rule", "what", "why", "fix"].every(
    (name) => name in value && typeof Reflect.get(value, name) === "string",
  );
  const severity = "severity" in value ? value.severity : undefined;
  const subject = "subject" in value ? value.subject : null;
  const guide = "guide_url" in value ? value.guide_url : undefined;
  return (
    texts &&
    SEVERITIES.some((known) => known === severity) &&
    (subject === null || typeof subject === "string") &&
    (guide === undefined || typeof guide === "string")
  );
}

/** A lint report as the API sends it: a plain object, read field by field. */
export type Lint = Record<string, unknown> | null | undefined;

/** The findings in a lint report: blocks first, then warnings, then tips. */
export function findingsOf(lint: Lint): Finding[] {
  if (lint === null || lint === undefined || !Array.isArray(lint.findings)) {
    return [];
  }
  const all: unknown[] = lint.findings;
  const findings = all.filter(isFinding);
  return SEVERITIES.flatMap((severity) => findings.filter((found) => found.severity === severity));
}

/** The findings behind a refused publish, blocks first; none for any other problem. */
export function lintFindings(problem: Problem | null): Finding[] {
  return findingsOf(problem?.lint);
}

/** A post's score out of 100, and how many points each rule cost it. */
export type Score = { value: number; lost: { rule: string; points: number }[] };

function isLoss(value: unknown): value is Score["lost"][number] {
  return (
    typeof value === "object" &&
    value !== null &&
    "rule" in value &&
    typeof value.rule === "string" &&
    "points" in value &&
    typeof value.points === "number"
  );
}

/** The score in a lint report about one post, or null when it has none. */
export function scoreOf(lint: Lint): Score | null {
  const score = lint?.score;
  if (typeof score !== "object" || score === null || !("value" in score)) {
    return null;
  }
  const value = score.value;
  if (typeof value !== "number") {
    return null;
  }
  const lost = "lost" in score && Array.isArray(score.lost) ? score.lost : [];
  return { value, lost: lost.filter(isLoss) };
}
