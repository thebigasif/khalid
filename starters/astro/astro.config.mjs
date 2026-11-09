// @ts-check
import { existsSync } from "node:fs";
import { defineConfig } from "astro/config";

// The pages read the Khalid site's address and key from process.env, which
// Astro never copies into the built files. A `.env` file fills in what the
// shell has not set already.
if (existsSync(".env")) {
  process.loadEnvFile(".env");
}

// The blog lives in the same folder Khalid's `mount` names in site.toml, so
// every address in a post's head, which Khalid writes, is this site's own.
export default defineConfig({
  site: process.env.SITE_ADDRESS ?? "https://www.example.com",
  base: "/blog",
  trailingSlash: "always",
  output: "static",
});
