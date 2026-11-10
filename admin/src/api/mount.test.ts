import { afterEach, describe, expect, it } from "vitest";
import { adminPath, blogAddress, mountPath } from "./mount";

/** Puts a meta named `name` in the page, as the server does. */
function named(name: string, content: string) {
  const meta = document.createElement("meta");
  meta.name = name;
  meta.content = content;
  document.head.append(meta);
}

describe("the blog's folder", () => {
  afterEach(() => {
    document.head.querySelector("meta[name=khalid-mount]")?.remove();
    document.head.querySelector("meta[name=khalid-admin-folder]")?.remove();
  });

  it("keeps the admin at admin when the page names no folder", () => {
    expect(adminPath()).toBe("/admin");
  });

  it("puts the admin in the folder site.toml names, under the mount", () => {
    named("khalid-mount", "/blog/");
    named("khalid-admin-folder", "studio");
    expect(adminPath()).toBe("/blog/studio");
  });

  it("ignores a folder name that is not one plain folder", () => {
    named("khalid-admin-folder", "../elsewhere");
    expect(adminPath()).toBe("/admin");
  });

  it("is the host's root when the page names none", () => {
    expect(mountPath()).toBe("/");
    expect(blogAddress()).toBe(globalThis.location.origin);
  });

  it("is the folder the server names, which the APIs sit under", () => {
    const meta = document.createElement("meta");
    meta.name = "khalid-mount";
    meta.content = "/blog/";
    document.head.append(meta);
    expect(mountPath()).toBe("/blog/");
    expect(blogAddress()).toBe(`${globalThis.location.origin}/blog`);
  });
});
