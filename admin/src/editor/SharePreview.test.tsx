import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import type { AdminPost } from "../api/posts";
import { seoEditorWords } from "../strings/seoEditorWords";
import { SharePreview } from "./SharePreview";
import { settingsDraft } from "./settings";

const POST: AdminPost = {
  id: "en.bike-service",
  slug: "bike-service",
  title: "What a bike service includes",
  status: "published",
  markdown: "Text.\n",
  updated_at: "2026-11-02T09:00:00Z",
  settings: {
    co_authors: [],
    featured: false,
    review_state: "ready",
    access: "public",
    password_protected: false,
    tags: [],
    share_cards: {},
    aliases: [],
    feature_image: { src: "cover.svg", alt: "A spanner" },
  },
};

const URL = "https://www.example.com/blog/bike-service/";

describe("the share previews", () => {
  it("shows a picture beside the post from the post's page on this server", () => {
    const { container } = render(
      <SharePreview title={POST.title} url={URL} settings={settingsDraft(POST)} />,
    );
    const shown = [...container.querySelectorAll("img")].map((img) => img.getAttribute("src"));
    expect(shown).toEqual([
      `${window.location.origin}/blog/bike-service/cover.svg`,
      `${window.location.origin}/blog/bike-service/cover.svg`,
    ]);
  });

  it("says there is no picture yet while the post's address is not known", () => {
    render(<SharePreview title={POST.title} url="" settings={settingsDraft(POST)} />);
    expect(screen.getAllByText(seoEditorWords.noImage)).toHaveLength(2);
  });
});
