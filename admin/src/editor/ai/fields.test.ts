import { describe, expect, it, vi } from "vitest";
import { fillField, helps, paragraph, textField, withBlock } from "./fields";

describe("filling a field from beside the form", () => {
  it("sets the value and sends input then focusout, as typing and leaving would", async () => {
    document.body.innerHTML = '<div><input id="setting-seoTitle" value="Old" /></div>';
    const field = textField(document.body, "setting-seoTitle");
    if (field === null) {
      throw new Error("the SEO title field was not found");
    }
    const seen: string[] = [];
    field.addEventListener("input", () => seen.push(`input ${field.value}`));
    field.addEventListener("focusout", () => seen.push("focusout"));
    fillField(field, "Choosing anchor text that helps readers");
    expect(seen).toEqual(["input Choosing anchor text that helps readers"]);
    // Leaving waits a turn, so the form draws the new value before it saves.
    await new Promise((done) => setTimeout(done, 0));
    expect(seen).toEqual(["input Choosing anchor text that helps readers", "focusout"]);
  });

  it("reaches a textarea too, and nothing that is not a field", () => {
    document.body.innerHTML = '<textarea id="media-alt"></textarea><p id="note">Hi</p>';
    const alt = textField(document.body, "media-alt");
    if (alt === null) {
      throw new Error("the alt text field was not found");
    }
    const changed = vi.fn();
    alt.addEventListener("input", changed);
    fillField(alt, "A chart of links between posts");
    expect(alt.value).toBe("A chart of links between posts");
    expect(changed).toHaveBeenCalledOnce();
    expect(textField(document.body, "note")).toBeNull();
  });
});

describe("adding a block to the post", () => {
  it("leaves one blank line before it and ends in one line break", () => {
    const block = ":::takeaways\n- One.\n:::";
    expect(withBlock("Some text.\n\n", block)).toBe("Some text.\n\n:::takeaways\n- One.\n:::\n");
    expect(withBlock("", block)).toBe(":::takeaways\n- One.\n:::\n");
  });
});

describe("where a suggested fix can help", () => {
  it("offers it for checks about words", () => {
    expect(helps("The title has 72 characters.")).toBe(true);
    expect(helps("The meta description is missing.")).toBe(true);
    expect(helps("An image has no alt text.")).toBe(true);
  });

  it("leaves it out where a person must act", () => {
    expect(helps("The post has no author.")).toBe(false);
    expect(helps("A link leads to a page that is gone.")).toBe(false);
  });

  it("reads a finding's lines from its section", () => {
    document.body.innerHTML =
      "<section><p>Warning</p><p><strong>Title too long</strong></p></section>";
    const section = document.body.querySelector("section");
    expect(section === null ? "" : paragraph(section, 1)).toBe("Title too long");
    expect(section === null ? "" : paragraph(section, 5)).toBe("");
  });
});
