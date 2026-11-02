import { afterEach, describe, expect, it } from "vitest";
import { blogAccent, wearAccent } from "./blog";

/** A meta the server writes in the admin's page. */
function named(name: string, content: string) {
  const meta = document.createElement("meta");
  meta.name = name;
  meta.content = content;
  document.head.append(meta);
}

afterEach(() => {
  for (const meta of document.head.querySelectorAll("meta")) {
    meta.remove();
  }
  document.documentElement.style.removeProperty("--color-link");
});

describe("the theme's accent", () => {
  it("colours the admin's buttons and links when the server names one", () => {
    named("khalid-accent", "#7c3aed");
    expect(blogAccent()).toBe("#7c3aed");
    wearAccent(document.documentElement);
    expect(document.documentElement.style.getPropertyValue("--color-link")).toBe("#7c3aed");
  });

  it("keeps the admin's own blue when the page names none", () => {
    wearAccent(document.documentElement);
    expect(blogAccent()).toBeNull();
    expect(document.documentElement.style.getPropertyValue("--color-link")).toBe("");
  });

  it("refuses anything but a plain colour", () => {
    named("khalid-accent", "red;background:url(x)");
    expect(blogAccent()).toBeNull();
  });
});
