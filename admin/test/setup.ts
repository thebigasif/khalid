import "@testing-library/jest-dom/vitest";
import { cleanup, configure } from "@testing-library/react";
import { afterEach, vi } from "vitest";
import { settleRouters } from "./routers";

// A route's code loads on first use. On a busy CI runner that first load
// took longer than Testing Library's 1 s wait, so a findBy* saw an empty
// page. Five seconds still fails fast when the text truly never comes.
configure({ asyncUtilTimeout: 5000 });

// jsdom has no layout, so its Range lacks the two measuring methods. The
// editor (ProseMirror) calls them to scroll the cursor into view, and a test
// that leaves an editor open long enough would fail with
// "target.getClientRects is not a function". An empty box is enough here.
const emptyBox = { x: 0, y: 0, width: 0, height: 0, top: 0, right: 0, bottom: 0, left: 0 };
Range.prototype.getClientRects ??= () => [] as unknown as DOMRectList;
Range.prototype.getBoundingClientRect ??= () => ({ ...emptyBox, toJSON: () => emptyBox });

afterEach(async () => {
  // Let a page the test opened finish loading while the router is still
  // mounted, so nothing of it runs after the test file's jsdom is gone.
  await settleRouters();
  cleanup();
  // @tiptap/react destroys an unmounted editor on a 1 ms timer. Wait for it
  // here, while the test still owns jsdom: once the file ends and jsdom is
  // gone, that timer fails the run with "window is not defined".
  await new Promise((resolve) => setTimeout(resolve, 2));
  sessionStorage.clear();
  vi.unstubAllGlobals();
});
