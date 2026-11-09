import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vitest/config";

// The admin lives under admin/ in the blog's folder, such as /blog/admin/,
// which only the running server knows. So the built files name each other
// relatively, and the server points the page at its folder.
export default defineConfig({
  base: "./",
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: [
      // @tiptap/react asks for the shim that lets React 16 and 17 use
      // useSyncExternalStore. React 18 and later have it built in, and the
      // shim then only hands back React's own, so take React's directly.
      {
        find: /^use-sync-external-store\/shim\/with-selector(\.js)?$/,
        replacement: "use-sync-external-store/with-selector",
      },
      { find: /^use-sync-external-store\/shim(\/index\.js)?$/, replacement: "react" },
      // The one comparison @tiptap/react takes from fast-equals, without the
      // rest of the library (src/editor/sameState.ts says why). A path that
      // starts with / starts at this folder.
      { find: /^fast-equals$/, replacement: "/src/editor/sameState.ts" },
      // Tiptap's Link finds addresses with linkifyjs, 11 KB compressed with
      // every domain ending; src/editor/linkify.ts does it in a few lines.
      { find: /^linkifyjs$/, replacement: "/src/editor/linkify.ts" },
    ],
  },
  test: {
    environment: "jsdom",
    setupFiles: ["./test/setup.ts"],
    include: ["src/**/*.test.{ts,tsx}"],
    // Room for a findBy* that waits its full 5 s (test/setup.ts).
    testTimeout: 15000,
  },
});
