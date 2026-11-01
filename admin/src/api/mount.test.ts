import { afterEach, describe, expect, it } from "vitest";
import { blogAddress, mountPath } from "./mount";

describe("the blog's folder", () => {
  afterEach(() => {
    document.head.querySelector("meta[name=khalid-mount]")?.remove();
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
