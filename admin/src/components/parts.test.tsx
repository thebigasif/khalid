import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { axeViolations } from "../../test/render";
import { en } from "../strings/en";
import { EmptyState } from "./EmptyState";
import { PageHeader } from "./PageHeader";
import { StatusPill } from "./StatusPill";
import { Toast } from "./Toast";

describe("the shared parts of every screen", () => {
  it("heads a screen with one heading, its lede and its actions", async () => {
    render(
      <main>
        <PageHeader title="Posts" lede="Every post on the blog.">
          <button type="button">Write</button>
        </PageHeader>
      </main>,
    );
    expect(screen.getByRole("heading", { level: 1, name: "Posts" })).toBeVisible();
    expect(screen.getByText("Every post on the blog.")).toHaveClass("lede");
    expect(screen.getByRole("button", { name: "Write" }).parentElement).toHaveClass("page-actions");
    expect(await axeViolations()).toEqual([]);
  });

  it("says what is empty and gives one next step", () => {
    render(
      <EmptyState icon="posts" title="No posts yet.">
        <a href="/admin/new">Write your first post</a>
      </EmptyState>,
    );
    expect(screen.getByText("No posts yet.")).toBeVisible();
    expect(screen.getByRole("link", { name: "Write your first post" })).toBeVisible();
  });

  it("names each post status in words, coloured by what it means", () => {
    render(
      <>
        <StatusPill status="published" />
        <StatusPill status="scheduled" />
        <StatusPill status="draft" />
      </>,
    );
    expect(screen.getByText(en.status.published)).toHaveClass("pill", "ok");
    expect(screen.getByText(en.status.scheduled)).toHaveClass("pill", "info");
    expect(screen.getByText(en.status.draft)).toHaveClass("pill", "quiet");
  });

  it("keeps the toast's live region in place while it has nothing to say", () => {
    const { rerender } = render(<Toast message={null} />);
    expect(screen.getByRole("status")).toBeEmptyDOMElement();
    rerender(<Toast message="Saved." />);
    expect(screen.getByRole("status")).toHaveTextContent("Saved.");
  });
});
