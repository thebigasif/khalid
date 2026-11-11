import { describe, expect, it } from "vitest";
import type { SiteSettings } from "../api/site";
import type { Template } from "../api/templates";
import { ctaLines, ctaOf, showsCta } from "./cta";
import { lookChange, setOnly, startOf, turns, valuesOf, wordsOf } from "./lookChange";

const LEDGER: Template = {
  name: "ledger",
  summary: "A data-led blog.",
  reader: [
    { key: "reading_time", on: true },
    { key: "related_posts", on: false },
  ],
  options: [
    { key: "logo", help: "Logo.", takes: { kind: "switch", default: true } },
    { key: "dek", help: "Dek.", takes: { kind: "switch", default: false } },
    { key: "stats", help: "Stats.", takes: { kind: "lines", most: 3 } },
    { key: "tagline", help: "Tagline.", takes: { kind: "line", most: 140 } },
    { key: "count", help: "Count.", takes: { kind: "count", least: 3, most: 10, default: 5 } },
  ],
};

const SITE: SiteSettings = {
  name: "Field Notes on SEO",
  address: "https://www.example.com",
  mount: "/blog/",
  default_language: "en",
  languages: ["en"],
  template: { name: "ledger", options: { count: 6 } },
  updated_at: "2026-11-01T09:00:00Z",
  restart_needed: false,
};

describe("the Look screen's options", () => {
  it("fill every option the site leaves out with the template's default", () => {
    expect(valuesOf(LEDGER, SITE.template.options)).toEqual({
      logo: true,
      dek: false,
      stats: [],
      tagline: "",
      count: 6,
    });
  });

  it("send only what differs from the defaults, without blank lines", () => {
    const values = { logo: true, dek: true, stats: ["", " 12,000 readers ", ""], count: 5 };
    expect(setOnly(LEDGER, values)).toEqual({ dek: true, stats: ["12,000 readers"] });
    expect(setOnly(LEDGER, { tagline: "  " })).toEqual({});
    expect(setOnly(LEDGER, { tagline: " Long how-tos " })).toEqual({ tagline: "Long how-tos" });
  });

  it("make no change for the look the site has, and a whole one for another", () => {
    expect(lookChange(SITE, LEDGER, valuesOf(LEDGER, { count: 6 }))).toBeNull();
    const classic: Template = { name: "classic", summary: "Today.", reader: [], options: [] };
    expect(lookChange(SITE, classic, {})).toEqual({
      updated_at: SITE.updated_at,
      template: { name: "classic", options: {} },
    });
    expect(startOf([classic, LEDGER], SITE)).toBe(LEDGER);
    expect(startOf([classic], SITE)).toBe(classic);
  });

  it("send the call to action only when it changes, and null when emptied", () => {
    const saved = { ...SITE, cta: { label: "Try it", href: "/blog/start/" } };
    const same = valuesOf(LEDGER, { count: 6 });
    expect(lookChange(saved, LEDGER, same)).toBeNull();
    const spaced = { label: " Try it ", href: "/blog/start/", note: "" };
    expect(lookChange(saved, LEDGER, same, spaced)).toBeNull();
    expect(
      lookChange(saved, LEDGER, same, { label: "Try it", href: "/blog/start/", note: " Free. " }),
    ).toEqual({
      updated_at: SITE.updated_at,
      cta: { label: "Try it", href: "/blog/start/", note: "Free." },
    });
    expect(lookChange(saved, LEDGER, same, { label: "", href: " ", note: "" })).toEqual({
      updated_at: SITE.updated_at,
      cta: null,
    });
    expect(lookChange(SITE, LEDGER, same, ctaLines(undefined))).toBeNull();
    expect(ctaOf(ctaLines(null))).toBeNull();
  });

  it("say whether a template shows the call to action itself", () => {
    expect(showsCta(LEDGER)).toBe(false);
    const takes = { kind: "switch" as const, default: false };
    const band = { key: "cta_band", help: "Band.", takes };
    expect(showsCta({ ...LEDGER, options: [band] })).toBe(true);
  });

  it("say what a template turns on and off, in words", () => {
    expect(turns(LEDGER)).toEqual({ on: ["reading_time", "logo"], off: ["related_posts"] });
    expect(wordsOf("card_reading_time")).toBe("Card reading time");
    expect(wordsOf("hacker-news")).toBe("Hacker news");
  });
});
