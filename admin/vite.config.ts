import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vitest/config";

// The admin lives under admin/ in the blog's folder, such as /blog/admin/,
// which only the running server knows. So the built files name each other
// relatively, and the server points the page at its folder.
export default defineConfig({
  base: "./",
  plugins: [react(), tailwindcss()],
  // Tiptap's Link finds addresses with linkifyjs, 11 KB compressed with
  // every domain ending; src/editor/linkify.ts does it in a few lines.
  resolve: { alias: { linkifyjs: "/src/editor/linkify.ts" } },
  test: {
    environment: "jsdom",
    setupFiles: ["./test/setup.ts"],
    include: ["src/**/*.test.{ts,tsx}"],
    // Room for a findBy* that waits its full 5 s (test/setup.ts).
    testTimeout: 15000,
  },
});
