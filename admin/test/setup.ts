import "@testing-library/jest-dom/vitest";
import { cleanup } from "@testing-library/react";
import { afterEach, vi } from "vitest";

afterEach(async () => {
  cleanup();
  // @tiptap/react destroys an unmounted editor on a 1 ms timer. Wait for it
  // here, while the test still owns jsdom: once the file ends and jsdom is
  // gone, that timer fails the run with "window is not defined".
  await new Promise((resolve) => setTimeout(resolve, 2));
  sessionStorage.clear();
  vi.unstubAllGlobals();
});
