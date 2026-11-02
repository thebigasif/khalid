import { describe, expect, it } from "vitest";
import type { MediaItem } from "../api/media";
import { isPhoto, squareCrop } from "./photo";

/** A library picture `width` by `height` with its focal point at `x`, `y`. */
function picture(width: number, height: number, x = 500, y = 500): MediaItem {
  return {
    id: "3f2a9c1e0b57d0aa",
    name: "lena.jpg",
    folder: "authors",
    media_type: "image/jpeg",
    kind: "image",
    bytes: 52_000,
    width,
    height,
    alt: "",
    caption: "",
    focal_point: { x, y },
    url: "/blog/media/library/3f2a9c1e0b57d0aa/lena.jpg",
    uploaded_at: "2026-11-02T09:00:00Z",
    uploaded_by: "owner",
    updated_at: "2026-11-02T09:00:00Z",
    updated_by: "owner",
  };
}

describe("an author's photo", () => {
  it("keeps the middle of a wide picture as a square", () => {
    expect(squareCrop(picture(1600, 800))).toEqual({
      turn: 0,
      crop: { left: 250, top: 0, width: 500, height: 1000 },
    });
  });

  it("moves the square toward the focal point, but never past the edge", () => {
    expect(squareCrop(picture(800, 1600, 500, 100))?.crop).toEqual({
      left: 0,
      top: 0,
      width: 1000,
      height: 500,
    });
    expect(squareCrop(picture(1600, 800, 900, 500))?.crop?.left).toBe(500);
  });

  it("leaves a square picture as it is", () => {
    expect(squareCrop(picture(600, 600))).toBeNull();
  });

  it("takes only pictures the blog can draw in sizes", () => {
    expect(isPhoto(picture(600, 600))).toBe(true);
    expect(isPhoto({ ...picture(600, 600), media_type: "application/pdf" })).toBe(false);
  });
});
