import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { type Answers, adminBlog, noContent } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import type { AdminAuthor, AuthorEdit } from "../api/authors";
import type { MediaItem } from "../api/media";
import { en } from "../strings/en";

const LENA: AdminAuthor = {
  id: "de.lena-vogt",
  slug: "lena-vogt",
  language: "de",
  name: "Lena Vogt",
  role: "Technical SEO editor",
  bio: "Lena looks after the technical side.\n",
  same_as: [],
  updated_at: "2026-10-30T09:00:00Z",
};

/** A blog with Lena that records every edit, then answers with `more`. */
function blog(more: Answers = {}) {
  const edits: AuthorEdit[] = [];
  const sent = adminBlog({
    "GET /authors/": () => json(200, { authors: [LENA] }),
    "GET /authors/de.lena-vogt/": () => json(200, { authors: [LENA] }),
    "PUT /authors/de.lena-vogt/": async (request) => {
      const body: { authors: AuthorEdit[] } = await request.json();
      edits.push(...body.authors);
      return json(200, { authors: [{ ...LENA, updated_at: "2026-10-31T10:00:00Z" }] });
    },
    ...more,
  });
  return { sent, edits };
}

const FIELDS = en.authors.edit.fields;
const PHOTO = en.authors.edit.photo;
const PICK = PHOTO.pick("Lena at her desk");

/** A square photo of Lena in the media library. */
const PICTURE: MediaItem = {
  id: "3f2a9c1e0b57d0aa",
  name: "lena.jpg",
  folder: "authors",
  media_type: "image/jpeg",
  kind: "image",
  bytes: 52_000,
  width: 800,
  height: 800,
  alt: "Lena at her desk",
  caption: "",
  focal_point: { x: 500, y: 500 },
  url: "/blog/media/library/3f2a9c1e0b57d0aa/lena.jpg",
  uploaded_at: "2026-11-02T09:00:00Z",
  uploaded_by: "owner",
  updated_at: "2026-11-02T09:00:00Z",
  updated_by: "owner",
};

describe("an author's page", () => {
  it("opens from the list and saves only the changed fields", async () => {
    signedIn();
    const { edits } = blog();
    renderAt("/authors");
    const edit = en.authors.list.editLabel("Lena Vogt", "German");
    await userEvent.click(await screen.findByRole("link", { name: edit }));
    expect(await screen.findByRole("heading", { level: 1, name: "Lena Vogt" })).toBeVisible();
    await userEvent.type(screen.getByLabelText(FIELDS.credentials.label), "Eight years in SEO");
    await userEvent.type(screen.getByLabelText(FIELDS.sameAs.label), "https://example.com/lena");
    await userEvent.click(screen.getByRole("button", { name: en.authors.edit.save }));
    expect(await screen.findByText(en.authors.edit.saved)).toBeVisible();
    expect(edits).toEqual([
      {
        updated_at: LENA.updated_at,
        credentials: "Eight years in SEO",
        same_as: ["https://example.com/lena"],
      },
    ]);
    expect(await axeViolations()).toEqual([]);
  });

  it("shows the API's words when someone changed the author first", async () => {
    signedIn();
    const stale = {
      type: "https://khalid.dev/problems/conflict",
      title: "Someone else changed this author first.",
      status: 409,
      detail: "Saving now would undo their change.",
      fix: "Reload the page and make your change again.",
    };
    blog({ "PUT /authors/de.lena-vogt/": () => json(409, stale) });
    renderAt("/authors/de.lena-vogt");
    await userEvent.clear(await screen.findByLabelText(FIELDS.role.label));
    await userEvent.click(screen.getByRole("button", { name: en.authors.edit.save }));
    expect(await screen.findByRole("alert")).toHaveTextContent(stale.title);
  });

  it("uploads a photo, crops it square, and saves where the square copy lives", async () => {
    signedIn();
    const wide: MediaItem = { ...PICTURE, width: 1600, height: 800 };
    const square = "/blog/media/library/9c1e0b57d0aa3f2a/lena.jpg";
    const crops: unknown[] = [];
    const { edits } = blog({
      "POST /media/": () => json(201, { media: [wide] }),
      "POST /media/3f2a9c1e0b57d0aa/copies/": async (request) => {
        crops.push(await request.json());
        const copy = { ...wide, id: "9c1e0b57d0aa3f2a", url: square, width: 800 };
        return json(201, { media: [copy] });
      },
    });
    renderAt("/authors/de.lena-vogt");
    const file = new File(["photo"], "lena.jpg", { type: "image/jpeg" });
    await userEvent.upload(await screen.findByLabelText(PHOTO.upload), file);
    expect(await screen.findByText(PHOTO.ready)).toBeVisible();
    expect(crops).toEqual([{ turn: 0, crop: { left: 250, top: 0, width: 500, height: 1000 } }]);
    expect(screen.getByRole("img", { name: PHOTO.preview })).toHaveAttribute("src", square);
    await userEvent.click(screen.getByRole("button", { name: en.authors.edit.save }));
    expect(await screen.findByText(en.authors.edit.saved)).toBeVisible();
    expect(edits).toEqual([{ updated_at: LENA.updated_at, photo: square }]);
    expect(await axeViolations()).toEqual([]);
  });

  it("picks a square picture from Media as it is, and removes it again", async () => {
    signedIn();
    const { edits } = blog({
      "GET /media/": () =>
        json(200, { media: [PICTURE], meta: { pagination: { page: 1, pages: 1, total: 1 } } }),
    });
    renderAt("/authors/de.lena-vogt");
    await userEvent.click(await screen.findByRole("button", { name: PHOTO.choose }));
    await userEvent.click(await screen.findByRole("button", { name: PICK }));
    expect(await screen.findByText(PHOTO.ready)).toBeVisible();
    await userEvent.click(screen.getByRole("button", { name: PHOTO.remove }));
    expect(screen.queryByRole("img", { name: PHOTO.preview })).toBeNull();
    await userEvent.click(screen.getByRole("button", { name: PHOTO.choose }));
    await userEvent.click(await screen.findByRole("button", { name: PICK }));
    await userEvent.click(screen.getByRole("button", { name: en.authors.edit.save }));
    expect(await screen.findByText(en.authors.edit.saved)).toBeVisible();
    expect(edits).toEqual([{ updated_at: LENA.updated_at, photo: PICTURE.url }]);
  });

  it("deletes the author with the copy's time, and goes back to the list", async () => {
    signedIn();
    const { sent } = blog({ "DELETE /authors/de.lena-vogt/": noContent });
    renderAt("/authors/de.lena-vogt");
    const remove = en.authors.edit.remove.submit("Lena Vogt");
    await userEvent.click(await screen.findByRole("button", { name: remove }));
    expect(await screen.findByRole("heading", { level: 1, name: en.authors.title })).toBeVisible();
    const deleted = sent.find((request) => request.method === "DELETE");
    expect(deleted?.url).toContain("updated_at=2026-10-30T09%3A00%3A00Z");
  });
});
