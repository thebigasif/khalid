import type { MediaItem } from "../api/media";
import type { PictureEdit } from "../api/pictures";

/** Where a span of `span` thousandths starts so its middle sits on `focus`, inside the picture. */
function place(focus: number, span: number): number {
  return Math.min(Math.max(Math.round(focus - span / 2), 0), 1000 - span);
}

/**
 * The square crop of `item` that a round avatar needs: the whole short
 * side, centred on the picture's focal point. `null` when the picture is
 * square already, or its size is unknown.
 */
export function squareCrop(item: MediaItem): PictureEdit | null {
  const { width, height } = item;
  if (width === null || width === undefined || height === null || height === undefined) {
    return null;
  }
  if (width === height || width === 0 || height === 0) {
    return null;
  }
  if (width > height) {
    const span = Math.round((height / width) * 1000);
    const left = place(item.focal_point.x, span);
    return { turn: 0, crop: { left, top: 0, width: span, height: 1000 } };
  }
  const span = Math.round((width / height) * 1000);
  const top = place(item.focal_point.y, span);
  return { turn: 0, crop: { left: 0, top, width: 1000, height: span } };
}

/** Whether `item` can be an author's photo: a picture the blog can draw in sizes. */
export function isPhoto(item: MediaItem): boolean {
  return ["image/jpeg", "image/png", "image/webp"].includes(item.type);
}
