# Khalid starter kit for Astro

This kit builds a blog with Astro from a Khalid site's Content API. Khalid keeps doing the rest: the editor, the linter, the schema, the sitemaps, the feeds and the redirects. Each post page carries the same meta tags and JSON-LD the Khalid theme writes, byte for byte.

## Set it up

You need Node 22.23.4 or later, which this version of Astro asks for.

1. In the Khalid site's `site.toml`, set `mode = "headless"`. Headless means Khalid writes no HTML pages of its own and leaves them to this app.
2. Make a content key, which can only read published posts:

   ```sh
   khalid keys create --kind content --name "Astro site"
   ```

3. Start Khalid with `khalid serve`. It prints the address it answers on, such as `http://127.0.0.1:2368/blog/`.
4. In this folder, make a file called `.env` with the address and the key. The file is ignored by git, so the key never reaches the repository.

   ```sh
   KHALID_API_URL=http://127.0.0.1:2368
   KHALID_CONTENT_KEY=<the key from step 2>
   SITE_ADDRESS=https://www.example.com
   ```

   `SITE_ADDRESS` is the `address` in `site.toml`. Variables already set in the shell win over the file, so a build server can set them there instead.
5. Install and build:

   ```sh
   npm install
   npm run build
   ```

6. Look at the result:

   ```sh
   npm run preview
   ```

   It prints the address, such as `http://localhost:4321/blog/`.

The pages land in `dist/`, one folder per post. Put that folder's contents under `/blog/` on your web server, and each post answers at the address Khalid gives it. Build again after you publish a post, or run the build from a Khalid webhook on `post.published`.

## What stays with Khalid

Send these addresses to `khalid serve` from your web server or host, so readers and search engines still find them:

- `/blog/sitemap.xml` and the sitemaps it lists, `/blog/robots.txt` and `/blog/llms.txt`
- the feeds: `/blog/feed.xml`, `/blog/atom.xml`, `/blog/feed.json` and each language's own
- `/blog/share/`, the share cards the `og:image` tags point at, and the images inside posts
- `/api/`, the Content API this kit reads
- old addresses that Khalid redirects

## How it works

`src/lib/khalid.js` reads the Content API and has no dependencies. `headHtml` turns a post's `head` into the tags the theme writes, and the layout prints them in `<head>`. The Content API has no head for the front page, so `homeHead` writes one: the site's title, a description, the canonical link and `Blog` schema listing the posts. The same file ships in the Next.js kit, and Khalid's own tests run it against the demo site on every change. Khalid's CI also builds this kit against its showcase site, with `scripts/starters-check.sh`, and checks that the front page and a post have a title, a meta description and schema.

## Licence

MIT, so you can copy any of it into your own site.
