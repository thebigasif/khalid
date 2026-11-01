# Changelog

What changed for people who use Khalid, one line per change. Each release is grouped by Added, Changed, Fixed and Security, and each group by area. Every pull request that changes behaviour adds a fragment in `changelog.d/`, folded in under Unreleased when it lands, and moved under its version at a release (`docs/standards/git.md`).

## Unreleased

## 1.1.0

Ten finished blog templates to choose from, a Look screen to pick one, pictures on every card and drawings as lead images, new blocks for posts, a fix for blogs under a folder of another site and for very large home pages, Khalid's own designed website with live demos, and a Deploy to Cloudflare button. Nothing changes for an existing blog until you pick a template. What is new, in plain words, with links to the live demos: `docs/releases/1.1.0.md`.

### Added

#### Templates

- `site.toml` takes a `[template]` table that names the look the site uses and that look's options. Without it a site keeps today's look, now named classic, so nothing changes on upgrade.
- Ten new templates, each with its own guide in `docs/templates/`. Pick one with `name = "<template>"` under `[template]`:
  - **Almanac**, for a library of long guides: category links, a featured guide with a call to action, the newest posts as short rows, a grid of guides beside the editor's picks, and posts with breadcrumbs, a wide lead picture, an "On this page" box and two sets of further reading.
  - **Atlas**, a portal for a blog with many topics: a sticky bar of topic tabs, the two newest featured posts large, editor's picks, the feed and a block per topic; each post shows its breadcrumbs, share links at the top and end, its contents in a sticky column, the author box, your call-to-action band and three related posts.
  - **Compass**, for a teaching blog: breadcrumbs, an open contents box and the key points before each guide, share links, the author box and the newsletter after it, and outlined picture cards on the lists.
  - **Folio**, a product team's essays under their own masthead: a centred blog name with a tagline and category links, a featured story, outlined picture cards, share links above and below each essay, and an "All posts" link in place of breadcrumbs.
  - **Harbor**, for a busy marketing blog: a featured post with more featured posts under it, rows of the newest posts, the sign-up form beside the blog's name, an offer card in each post and a sideways row of related reading.
  - **Keystone**, for a personal guide site: one big featured post, a grid of picture cards, numbered chapters, share links in a column beside each post and a newsletter form between sections.
  - **Ledger**, a calm, text-led blog with no pictures in its lists, for studies and long how-tos: your tagline, up to three facts, a row of topics and the latest posts as a ruled list; each post shows its topic above the title, an open contents list, the author box, five related posts and the newsletter form.
  - **Meridian**, a calm company journal: category tabs, wide rows with a square picture, several authors with their roles, and a reference layout for long guides that a post picks with `template: reference`.
  - **Wire**, for a news and columns site: a lead story beside tabs of the latest posts and the editor's picks that switch with no script, a one-column feed with the newsletter form inside it, and posts that open with a subtitle, the writer's role and a card with their bio.
  - **Workbench**, for blogs that review and compare tools: a featured post, the newest posts as rows, the newsletter twice on the front page, wide comparison tables, pros and cons, and a closing line with the dates.
- Templates can switch on shared page parts: the main site's logo in the header, breadcrumbs, a post's subtitle and why it was updated, five card shapes with reading time and author role, a featured post above lists, editor's picks, numbered page links, five layouts for related posts and five places for a post's contents. Classic switches none of them on, so existing blogs look the same. See `docs/templates.md`.
- Templates can show a row of category tabs on the home page: "All", then each category your posts use, in the order `categories.toml` lists them.
- Templates can show an "All posts" link above each post, back to the front page, in place of breadcrumbs.
- A template pack may bring its own post templates, such as `post-reference.html`, which a post picks with `template:` in its front matter.
- Every page shows the main site's icon in the browser tab when theme sync found one.
- A site may add its own CSS rules in `theme/site.css`. They join the one stylesheet after the template's rules, so they win, and `khalid theme sync` still sets the colours and fonts they read.

#### Pictures

- A lead image or author photo in SVG now shows at the top of its page, on every card and beside every byline, drawn at the size the file declares. Before, only JPEG, PNG and WebP showed.
- Cards on the home page, in featured slots, in editor's picks and in home sections now show each post's lead image and its author's photo, as list pages already did.

#### Calls to action, sharing and the newsletter

- `site.toml` takes `[template.cta]`: one call to action with `label`, `href` and an optional `note`. Templates show it as a band after each post or on the front page, with the `cta_band` option, and as a button in the header, with `cta_button`.
- A new block, `::cta`, places the band with your `[template.cta]` anywhere in a post or page, in every template.
- `site.toml` takes `[[nav.footer_groups]]`: columns of footer links, each under its own heading, beside the flat `[[nav.footer]]` list. Every template shows them, and a group can be for one language only.
- Templates can show share links under a post's title, after it, or both: plain links to each network's own share page and an email link, with no script and nothing loaded from the networks. The `share` and `share_networks` options choose where and which.
- Templates can show a newsletter sign-up form after each post, on the front page, or both, with the `newsletter` option. It shows only on a blog that takes sign-ups: server mode with `[mail]`. The form can show a note, such as how often the newsletter goes out.

#### Writing and content

- Every block in a post now has its own look: callouts, figures, galleries, videos, the FAQ, how-to steps, product, review and recipe cards, bookmarks, buttons and forms. Callouts gain three kinds: `summary` for key takeaways, `read` for further reading and `stat` for one number worth remembering. The hidden spam-trap field on forms no longer shows to readers, and comments in code are coloured.
- A columns block: `::::columns` around two to four `:::column{title="..."}` cards sets them side by side, and stacks them on a phone. Good for comparing two options.
- A pull quote, `:::pullquote{by="..."}`, sets a short line large with who said it, and pros and cons, `:::proscons`, shows a `+` list beside a `-` list under their own titles. Both read in full with scripts off.
- Long tables can fold: wrap a table in `:::table{fold=8}` and readers see the first eight rows with a "Show 16 more rows" line that opens the rest. It works with scripts off, and `more="..."` sets your own words for that line.

#### Editor and admin

- A Look screen in the admin: pick one of the built-in templates, see what each one turns on and off, and set that template's options. It saves to `site.toml`, and your blog shows the new look after Khalid restarts. See `docs/templates.md`.
- `GET /api/admin/v1/templates/` lists every built-in template with its options, and the settings route reads and saves the site's `template`.

#### Deploy

- A "Deploy to Cloudflare" button puts a small, ready Khalid blog in your GitHub account and on Cloudflare for free, rebuilt on every push. `docs/deploy/cloudflare.md` explains the button, the Cloudflare Pages steps, the blog's address and your own domain. The starter installs Khalid 1.1.0.

#### Website, demo and examples

- Khalid's own website, built with Khalid, in `site/`: the story on the home page, one page per feature group, the templates gallery, six use cases, the ways to start and a fair comparison with WordPress, Ghost and Webflow. It is published at https://thebigasif.github.io/khalid/, and every post on it passes `khalid lint` with no finding.
- A showcase demo site in `fixtures/showcase-site`: the blog of a fictional content studio, with sixteen posts in English and German, three authors, a pillar guide with its cluster, and every block Khalid has but audio, video files and recipes. It is live at https://thebigasif.github.io/khalid/demo/, and once in every template under https://thebigasif.github.io/khalid/templates/.
- Six example sites in `fixtures/examples/` to copy from: a software company's blog with guides, comparisons and product updates; a marketing agency's blog with case studies, services and named experts; a writer's newsletter with a free letter, a supporters' list and an archive; a local bike workshop's blog with opening hours, local business details, how-to guides and FAQs; a publication in English, German and Spanish with every post linked to its translations; and a darkroom club's membership blog with members-only posts, a password post and a guide whose last part is for mailing list readers. Each has a README that says who it is for and which features it shows.
- Every post of the showcase site and the six example sites has its own cover illustration, and every author a portrait.
- The showcase sets a call to action and a newsletter sign-up, so every template demo shows its call-to-action band, and a served showcase shows the newsletter form. Nothing is sent: the mail server is a reserved example name.
- `scripts/template-previews.sh` draws the website's template previews: a screenshot of each template demo's home page and of one post, taken with a headless Chromium, or a plain card where none runs.
- Tests keep the showcase, every example and the website free of lint findings and audit warnings.

### Changed

#### Templates

- Every template now shows each post's category: on the post under its dates, on each post card, and as a row of topic links on the front page, named as `categories.toml` names them in each language. Turn the row off with `topic_links = false` in `[reader]`.
- Templates share one set of words: "Latest posts" over the newest posts, "All posts" for the link to every post, and "All" as the first of a row of topics or categories.
- The Look screen's options include one-line text, such as Ledger's tagline, drawn as a single text field. The Admin API lists it as `line` with the most characters it takes.
- Every template keeps its links, stylesheets, newsletter form and share links under a blog's folder, such as `/blog/`, and the Look screen calls the Admin API there too. A test builds the demo in each template and fails on any address that leaves the folder.
- Every template now has a finished look:
  - **Almanac** looks like a well-lit library: the featured guide on a large tinted panel with its cover, rounded picture cards that warm to a tint when pointed at, numbered editor's picks, "On this page" in two columns with accent dots, and the newsletter and footer as bands across the screen.
  - **Atlas**: the two lead stories and the editor's picks are picture cards that lift on hover, topic labels and tabs take the indigo accent, topic blocks sit on tinted cards, and posts get a tinted contents panel, round share buttons, a framed pull quote and a footer across the window.
  - **Compass**: a teal strip along the top, the name after a drawn compass, the blog's title on a tinted band, outlined picture cards that lift on hover with a drawn cover for posts without a picture, key points with drawn check marks, round share buttons, and an author box and footer with the accent.
  - **Folio** opens under a soft glow of the accent behind its masthead, with outlined cards that rise when pointed at, a large featured story, and essays whose cover runs wider than the text.
  - **Harbor**: the blog's name large on a berry tint across the window, the featured post on a soft panel with a shadowed picture, cards and rows with a drawn cover when a post has no picture, labels in the accent, a tinted contents box, round share buttons that fill on hover and a footer across the window.
  - **Keystone**: a heavy name after an orange dot, menus that tint on hover, the featured post as a hero on a warm tint, topic pills, picture cards that zoom gently on hover with a drawn cover for posts without a picture, a centred post title over a rounded lead picture, and a warm author card and footer.
  - **Ledger**: serif headings, the latest posts numbered in the margin, a large masthead, pills that fill with the accent on hover, a framed lead image (now on by default), a contents box with an accent edge, an author box and a footer on the paper tint.
  - **Meridian** is calmer: large titles set close, square pictures that ease in on hover, the editor's picks on a soft panel, the post's cover under a hairline byline, and the newsletter between two hairlines.
  - **Wire** looks like a newsroom: an accent bar over the page, heavy headlines, pictures in the lead story and on the end side of every row, big numerals in the editor's picks, quotes set between heavy rules, the writer's card on a tint with an accent edge, and the footer as a band across the screen.
  - **Workbench** looks like a well-kept bench: the name and labels in the code font, a squared grid behind the front page's title, the featured post and the post's cover framed on a hard shadow of the accent, cards that lift onto it when pointed at, numbered contents, and an "at a glance" table with a tinted header. A post now shows its cover by default; set `lead_image = false` to hide it.
- Ledger, Atlas, Keystone, Compass and Harbor each bring their own colours: ink green on warm paper, indigo, warm orange, teal and berry. Ledger and Compass also bring their own heading type. A site's own colours and fonts in `theme/tokens.toml`, such as the ones theme sync writes, still win.

#### Website

- Khalid's website wears the Atlas template with its own look: a warm paper and ink palette with an orange accent, a sticky header with a button, page heroes, features and use cases as tiles with drawn icons, numbered ways to start and footer link columns.
- The website's home page is designed: a hero with the promise and a drawn lint report, the three ways to start with the Deploy to Cloudflare button, the figures Khalid holds itself to, a feature grid with drawn icons, a blog without checks beside a blog on Khalid, the use cases, how it works in three steps and a closing call to action.
- The website shows every template in a gallery, each with a picture of its demo's home page and of a post, and the home page shows six of them in a strip.

#### Search engines

- A post with a category now has its category as a step in its breadcrumb schema, between the home page and the post.

#### Libraries

- Khalid now reads its settings files with toml 1.1 and XML (sitemaps, WordPress exports) with roxmltree 0.21. Nothing changes for you; the newer XML reader no longer crashes on very deeply nested files.
- The library crates `khalid-core` and `khalid-lint` move to 0.2.0. `khalid-core` gained the template choice, reader settings and footer groups, and the two crates always share one version.

### Fixed

#### Serving

- A blog under a folder of another site, such as `/blog/`, now serves its admin at `/blog/admin/` and both APIs at `/blog/api/`, so the proxy rule `khalid init --stack` prints reaches them. Sign-in links, preview links, webhooks, the OpenAPI document and the newsletter form name the folder too. An old `/admin/` address moves there. A blog at `/` is unchanged.
- A blog with thousands of posts builds and serves again. Its home page lists every post, and it stopped with "template home.html failed while rendering" because a page had a fixed limit on its work. A page now gets more room for each post it lists, and a template that loops without end is still stopped.

#### Redirects and import

- `khalid redirects explain` now knows the old addresses listed in a post's `aliases`. It names the post, and a stored redirect that points at an alias ends at the post in one step, as `khalid serve` sends it. An alias of a post the linter blocks is shown as waiting for that post (KH-SEO-8).
- `khalid import` and `khalid redirects import` say "1 redirect" and "1 file" instead of "1 redirects" and "1 files".

#### Admin

- The admin's tests no longer fail now and then after every test has passed. Each test now waits for the writing editor to close before it ends.

## 1.0.0

The first stable release. It holds everything in 0.9.1 and the fixes below, found by walking a new user's path from install to a published post. What it is, how to install it and what is still checked after launch: `docs/releases/1.0.0.md`.

### Changed

#### Install and setup

- `khalid init` now writes `theme/tokens.toml` with the light default colours, fonts and corners, so a new site's theme can be edited by hand at once, and it lists every file and folder it made (KH-INS-2).
- The `routes.toml` that `khalid init` writes explains the default addresses and holds an example to uncomment. It points at the new guide `docs/routes.md` instead of an internal planning file.

### Fixed

- The getting-started guide now says what `khalid build` does with a blocked post: it leaves that post out and writes the rest. It used to say the build wrote nothing.
- The nginx rule from `khalid init --stack nginx` no longer adds `proxy_ssl_server_name on;` when the blog's address is plain `http://`, where the line does nothing.

#### Install and setup

- `khalid staff owner` names the port `khalid serve` last ran on in its link for this computer, so a blog started with `--listen 127.0.0.1:3000` gets a link to port 3000 without being told. `--listen` still sets it by hand.
- `cargo build` warns, in three plain lines, when `admin/dist/` is empty, so a build from source without the npm steps says it has no admin and how to add it.
- The README's quick start starts `khalid serve` with `MIMALLOC_PURGE_DELAY=0`, as the Docker image and the systemd service do, so a blog started by hand gives unused memory back at once.
- The Windows download builds again. Windows changed the line endings of two saved queries on checkout, so the build could not find their checked shapes. The repository now keeps every SQL file in one form on every system.

#### Writing and content

- A post with no `slug:` line now works with `khalid serve` and `khalid db rebuild` as it already did with `khalid build`: its address comes from its title, the same in all three. Before, `serve` stopped and `db rebuild` refused the post with "has no slug". An author with no `slug:` takes one from their name the same way.
- `khalid new` leaves filler words and years out of a post's address, such as "that" or "2026", so its own linter no longer warns about the address it just made. A title made only of filler words keeps them.
- `khalid new` says a post shows after the next build, or at once while `khalid serve` runs.

#### Linting and audit

- The "links to none of your other posts" block mentions the home page only when the post really is the only one in its language. Otherwise it asks for a link to a related post and lists the best ones to try.
- `khalid audit` says which pages it counts, the ones readers and search engines reach, since `build` counts every page it writes. It also says the health score is the share of pages with no error, so warnings do not lower it.

#### Search engines

- A glossary term's page now has a meta description, taken from the first sentences of its definition, so search results show the definition and Lighthouse's SEO check passes.

#### Importing

- An import no longer sends old addresses to a post readers cannot see. When the linter blocks an imported post, its old addresses wait in the post's own aliases and lead to it the moment it is fixed. Old addresses outside the blog's folder wait in `import/held-redirects.csv`. The import summary names each blocked post and says where its addresses wait.
- An import that moves a blog to a new host now counts an address whose path stays the same, such as `/about/`, as one that stays the same. Khalid no longer stores a redirect that sends such an address to itself, because its page answers it.

#### Serving

- `khalid serve` answers the browser's request for `/favicon.ico` with "no content" when the site has no icon, so browsers stop logging an error on every page.
- A post saved while `khalid serve` is still starting now shows at once. On a 10,000-post site it used to wait about 40 seconds, for two or three renders of the whole site. The saved page now shows in about 15 ms, and the rest of the site follows when its render ends.

#### Speed

- Checking a post before it is published is about five times faster on a large site: 9 ms instead of 45 ms for one post on a 1,000-post site. The check for posts that copy each other now reads each other post once and skips the ones that cannot be copies, and the link-word checks no longer copy every link of the site for each link of the post. They find the same problems as before.

#### Checks behind the release

- The demo check (`scripts/demo-check.sh`) finishes well inside CI's 30 minutes: the HTML validator runs again, the not-found pages skip Lighthouse, which cannot score a 404, and the browser checks load each page once and check four at a time. axe runs with scripts on, where it can run, and the scripts-off run now proves each page shows the same text and links without JavaScript.

## 0.9.1

The first public release, published as 0.9.1 because the 0.9.0 build stopped before anything was released. It holds everything listed under 0.9.0 below, plus two fixes to how the release is built.

### Fixed

- The Docker image builds again: the admin's install step now has the API description it reads.
- Release builds for every platform compile again: the saved database query data is up to date.

## 0.9.0

The first public release. What it is, how to install it and what is known not to work yet: `docs/releases/0.9.1.md`.

### Added

#### Install and setup

- Release builds for Linux (x86_64 and ARM), macOS and Windows, with checksums, made from a version tag.
- `khalid init` prints the proxy rule a blog mounted under a main site needs. Pick your stack with `--stack` (or answer the question), set where the blog lives with `--blog`, and get the snippet for nginx, Caddy, Apache, Traefik, a Cloudflare Worker, Vercel, Next.js, Netlify, Webflow or Shopify (KH-INS-3).
- Each release publishes the Docker image to `ghcr.io/thebigasif/khalid` for x86_64 and ARM, so `docker compose up` can download it instead of building it.
- `khalid init [folder] --address <url>` sets up a site from four answers: the website's address, `--mount`, `--languages` and `--mode`. It asks each question in the terminal with a suggested answer, explains a wrong answer before asking again, and asks nothing with `--yes` or without a terminal. It checks every answer before writing and never touches a folder that already holds a site (KH-INS-2, KH-OPS-3).
- `khalid --site <folder> doctor` checks a site's settings, routes and content in one go and lists every problem, each with what happened, why it matters and what to do. It also checks that `data/site.db` opens, that `theme/tokens.toml` reads correctly with every text colour readable on its background, and that Khalid may write to the site, `content/`, `data/` and `theme/` folders. It changes nothing, exits with 1 when something needs you, and `--json` gives the same result for scripts (KH-OPS-2).
- `khalid doctor` fetches the blog at its public address, such as `https://www.example.com/blog/`, to prove the proxy rule on the main site works. It tells apart no answer, an error such as 404, the main site answering instead of the blog, and a private address, and says what to do for each. `--offline` skips this for a site that is not live yet (KH-INS-3, KH-OPS-2).
- `site.toml` accepts `mode = "headless"` for a site whose pages another app shows through the Content API. `khalid build` then writes only the sitemaps, `robots.txt`, `llms.txt`, the feeds and `_redirects`, and `khalid serve` answers the APIs and those files, with a short 404 for page addresses. The theme is not read, so a broken one stops nothing (KH-INS-5).
- `khalid serve --domain blog.example.com` serves your blog over HTTPS with a free Let's Encrypt certificate that Khalid gets and renews by itself. Port 80 sends visitors to the HTTPS address, and port 443 alone is enough to get a certificate. The first time, Khalid asks you to agree to Let's Encrypt's terms; `--accept-terms` agrees without asking, for services and containers (KH-INS-4).
- Khalid renews the certificate once two thirds of its life have passed and uses the new one from the next visit without a restart. If Let's Encrypt cannot be reached, it tries again an hour later. The certificate and account key are kept in `data/https/` in the site folder (KH-INS-4).
- `scripts/install.sh` installs Khalid with one line: it downloads the release for your machine, installs it only when its checksum matches `SHA256SUMS`, and with `--systemd` sets up the service.
- `khalid restore <archive> <folder>` rebuilds a site from a `khalid backup` archive, database included. It only fills a new or empty folder, so a live site is never overwritten, and it refuses an archive that is not a site backup.
- `docs/getting-started.md` takes a new user from a new site to a published post, then through admin sign-in, lint, build, audit, imports, and backup and restore, and every command on the page is tested. `CONTRIBUTING.md` explains how to build Khalid from source.

#### Writing and content

- `khalid --site <folder>` checks the site's settings and reads every content file: posts, pages, authors, glossary terms, case studies, changelog entries and snippets, as plain files or as folders with their images. A mistake is explained by file, line and field (KH-CON-1, KH-CON-2, KH-CON-3, KH-CON-4).
- `routes.toml` shapes the site's addresses with collections, channels and tag and category prefixes. A mistake is explained by line and field, and a site without the file gets the defaults (KH-CON-9).
- The site's index now includes its authors from `content/<language>/authors/` and its categories from `categories.toml`, on `khalid db rebuild` and after each change while `khalid serve` runs. A mistake in `categories.toml` names the file, the category or line, and how to fix it (KH-CON-10).
- Posts render to HTML with headings you can link to, coloured code, tables, footnotes, tip boxes (`:::callout`) and captioned images (`:::figure`), and every block writes back to markdown unchanged (KH-CON-5).
- Posts can end with questions and answers in a `::::faq` block. Readers open each answer with a click, and the block writes its own FAQ schema so the questions can show in search results (KH-CON-5, KH-SEO-3).
- Posts can show a business card with `::business`: name, postal address, phone, email and website, readable by search engines (KH-CON-5).
- An `updated:` date in a post's front matter, with an optional note on what changed, sets the date readers see, the date search engines and feeds get, and the stale-post check. Editing only the title or description never sets it.
- Posts that share a `translation` key link to each other with hreflang, plus `x-default` for readers in any other language. Every version carries the same complete list (KH-CON-8).
- Posts can hold a form with `:::form`. It works with scripts off, labels every field and carries a hidden trap for spam robots. Sending answers comes with the server (KH-CON-5).
- Posts can play a sound file with `::audio`, offer a download with `::file`, and show a grid of pictures with `:::gallery`, where every picture needs alt text (KH-CON-5).
- Posts can open a part with a banner, `:::header`, holding a picture and words (KH-CON-5).
- Posts can give numbered, named steps with a total time in a `::::howto` block, which search engines can read as a how-to (KH-CON-5).
- Posts can hold your own HTML with `:::html`. Khalid removes scripts, event handlers, frames and script links first, and members-only HTML stays hidden until readers can sign in (KH-CON-5).
- Posts can show a product card with `:::product`: picture, price, score and a buy button (KH-CON-5).
- Posts can hold a recipe with `::::recipe`: photo, servings, prep and cook times, ingredients and method, readable by search engines (KH-CON-5).
- Posts can rate a product, book, course or app with `:::review`, and search engines can show the stars (KH-CON-5).
- Posts can play their own video with `::video`. It needs a still picture, loads nothing until played, and search engines can read it as a video result (KH-CON-5).
- A post's front matter can name the phrase it wants to be found for with `focus_keyword`, and up to four close phrases with `secondary_keywords` (KH-LNT-4).
- Posts can show a formula on its own line with a `:::maths` block and an optional `label` as its caption. It is drawn when the site builds, so readers see it with scripts off, and a formula that cannot be drawn is refused with its file and line (KH-CON-5, KH-WEB-9).
- Snippets: `::snippet{slug="affiliate-disclosure"}` shows the blocks of `content/<lang>/snippets/affiliate-disclosure.md` in its place, so one change shows in every post that uses it. A post that names a snippet its language does not have is refused with the file and line.
- A post's front matter can name the topic cluster it belongs to with `cluster`, giving a `name` and a `role` of `pillar` or `member` (KH-LNT-2).
- Every glossary term gets its own page, and every post and term sits at the address `routes.toml` gives it (KH-CON-1, KH-CON-6).
- Maths between `$` signs, or between `$$` lines for a block, is drawn as formulas when the site is built, so readers see them with scripts off. Prices such as $5 stay plain text, and Khalid names the file and line of a formula it cannot draw.
- `khalid new --template comparison|alternatives|best-of|review` starts a post in the shape of its kind: the sections, a table where choices are weighed, and guidance in comments that never show on the page. A German post gets German headings and guidance; other languages get the English starter.
- A post can be password-protected or for members only, with `access` (`public` or `members`) in its front matter or through the Admin API. Readers see its title, description and a password form or a short note, never its body, not even in the contents, feeds or schema. The form is plain HTML that works without scripts, and a right password opens the post for 30 days (KH-EDT-3).
- A post can have its own head code, such as an analytics snippet, through `head_code` in front matter or the Admin API (KH-EDT-3).
- Post front matter takes a lead image (`image` with `src`, `alt` and an optional `caption`), `seo_title`, `tags` and `sitemap: false`, plus `co_authors`, `reviewed_by`, `review_state` (`draft`, `in_review` or `ready`), `share_cards` for X and Facebook, and `related` to pin up to five related posts. A lead image without alt text, a blank tag or a repeated tag is refused with what to write. The Admin API now saves co-authors, the lead image, review state and share cards into the post's file.
- Author files take a `photo`, kept beside the author's file in a folder such as `authors/lena-vogt/index.md`. It shows on the author page and as a small round photo in bylines, post cards and author lists.
- A post can link to another post, page or glossary term in the same language by its slug, with `[[slug]]` or `[[slug|your own words]]`. With no words of your own, the link takes the page's title and goes to the address `routes.toml` gives it. A `[[slug]]` that matches nothing is refused with the file and line (KH-CON-6).

#### Editor and admin

- The admin app, served by `khalid serve` at `/admin/` in the light theme. A binary built without it says so at `/admin/` and explains how to add it.
- Sign in to the admin with a staff token made by `khalid keys create --kind staff`. The token is kept for that browser tab only, and you get a plain message when the blog refuses it (KH-API-2).
- The admin opens on a dashboard: how many posts are live, scheduled, drafts and blocked from publishing, the latest changes with who made them, and one focus point. Blocked posts come first; when there are none, a "Start a post" form takes their place (KH-EDT-4).
- A Posts screen lists every post, latest change first, with its author, checks, review stage and what search engines are told. Filter by checks, review stage and search visibility, page through 20 at a time, and bookmark a filtered list (KH-EDT-4).
- Start a post from a title, saved at once as a draft, and write it as rich text or markdown. It saves a few seconds after each change, asks before you leave with unsaved changes, and stops with a plain message when someone else changed the post. A post with blocks the rich-text view cannot show yet stays in markdown, so nothing is rewritten (KH-EDT-1, KH-EDT-2).
- A "Post settings" panel beside the checks: web address, author, other authors, the date readers see, the description and the review stage. Rarer settings sit under "More settings": feature image with its description and caption, canonical address, search visibility, old addresses, share cards for X and Facebook, and a time to take the post down. Only the settings you change are sent (KH-EDT-3, KH-EDT-4).
- The editor opens posts with directives, tables, footnotes and wikilinks in the rich-text view, each block as a card, and saving without edits writes the same markdown byte for byte. A post with a directive Khalid does not know still opens as markdown (KH-EDT-1).
- Writing aids in the editor: a slash menu on an empty line to add any block, a details form for each block, `@` to mention a post, `[[` to link one, link search on selected words with Ctrl+K or Cmd+K, and the list of markdown shortcuts with Ctrl+/ or Cmd+/. Every link to a post is written as a wikilink, so it keeps working when the post's address changes (KH-EDT-1).
- "Show the page" in the editor shows the post in the site's own design beside the text and reloads it each time the post saves (KH-EDT-2).
- Select blocks and press "Save as a snippet" to save them under a name and put the snippet in their place, or type /snippet to place one of the site's snippets (KH-EDT-1).
- Earlier versions of a post: each save that changes the title or text keeps the version it replaced, the 100 newest per post. "Earlier versions" in the editor shows who saved each and when and what changed, and brings one back with an ordinary save that can be undone (KH-EDT-2).
- Post locking: the first writer to open a post holds it, and anyone else reads it, sees who is writing, and can take over after the screen says what that costs. The lock lets go when the holder closes the post or a minute after their editor stops checking in, and a restart frees every post (KH-EDT-2).
- Tables open as a grid of cells by double-click or Enter, and `/table` adds a new one. Rows and columns are added or removed at the end, saving writes a standard markdown table, and a table the grid cannot read stays as written (KH-EDT-1).
- Emoji by name: type `:` and two letters of a name, such as `:tada`, and pick from the list with the arrow keys and Enter. The post holds the emoji itself (KH-EDT-1).
- "Picture from your library" in the block menu puts any picture you uploaded before into the post with its alt text and caption.
- "Turn or crop" on a library picture turns it in quarter turns and crops it to a square, wide, standing or "Your own box" shape, dragged over the picture or typed in percent. The post gets a new copy with the same name, alt text and caption, and the original stays, so other posts do not change (KH-EDT-1).
- Photo and GIF search: once the owner turns them on, `/unsplash` and `/gif` in the editor find a free photo or GIF and add it as a credited figure, after saying the picture loads from there. Turn them on with `unsplash = true` and `gifs = true` under `[media]` in `site.toml`, with keys in `KHALID_UNSPLASH_ACCESS_KEY` and `KHALID_GIPHY_API_KEY`. The Settings screen and `khalid doctor` say whether each search is off, ready or missing its key (KH-EDT-1).
- New admin screens: Comments to approve, answer and delete comments, Account to add a passkey, change the password and sign out everywhere, and Authors, API keys and Settings.
- A Webhooks screen to add a webhook, see its signing secret once and switch one off, with "See what it sent" showing the newest 50 messages, whether each was delivered, waiting or given up, and every try with the receiver's answer. A Team screen invites someone with a role, changes a role and removes someone.
- The owner can hand the site to someone else on the team from "Hand the site to someone else" on the Team screen, which says what changes before it asks (KH-EDT-7).
- An audit log of every change made through the Admin API, with who made it and when, on the Audit log screen and at `GET /api/admin/v1/audit/` (KH-EDT-6).
- A trash: a deleted post moves to `trash/` beside `content/` and can be put back or deleted for good from the Trash screen or `/api/admin/v1/trash/` (KH-EDT-6).
- Tick boxes on the posts table set the author of many posts at once or move them to the trash (KH-EDT-6).
- Signed-in staff see a toolbar at the foot of each blog page: edit this post, what the linter says about it, and how often it was read in the last 30 days (KH-EDT-6).
- The admin can show a calendar of published, scheduled and dated draft posts by day, up to 92 days at a time, with undated drafts beside it, through `GET /api/admin/v1/calendar/`. To move a post to another day, change its date or schedule (KH-EDT-4).
- Staff can comment on a draft, on one block or on the whole post, reply in a thread, and mark a thread resolved or open again. Readers never see these comments (KH-EDT-4).
- When `[mail]` is set in `site.toml`, a new comment on a draft is emailed to the site's editors, admins and owner, and to staff who wrote in its thread. The writer never gets an email about their own comment (KH-EDT-4).
- Review states follow roles: a writer or contributor moves a draft between draft and in review, and only an editor, an admin or the owner marks it ready. If anyone else changes an approved post, it goes back to review. AI agents and SerpDad can put a draft in review too (KH-EDT-4).
- The editor shows the post's score out of 100 and its checks after every save, not only when publishing is refused. Below them, "How it looks in Google" shows the post as a search result, with the title and description the page will carry and their lengths (KH-SEO-1, KH-LNT-4).
- A Connect to SerpDad page in the admin makes the key SerpDad uses to send drafts, shows it once beside the blog's address, and lists the last 20 drafts SerpDad sent with their checks. The Admin API offers the same at `/api/admin/v1/serpdad/`: read the connection, make a new key that replaces the old one, or turn the connection off (KH-API-6).

#### Publishing

- Publish from the admin's editor. When the linter stops a post, every finding shows beside it, blocking ones first, each saying what is wrong, why it matters and what to do (KH-LNT-3, KH-EDT-1).
- Schedule a post from its settings for a date and time in your own time zone, which turns Publish into Schedule. "Copy preview link" makes a link that lets someone read the draft without signing in, and says until when it works (KH-EDT-3).
- Posts made through the Admin API are drafts in `drafts/<language>/`, which readers never see. Sending `"status": "published"` runs the linter first: a blocked post stays a draft and the answer lists every finding, and a passing post moves into `content/` with today's date and is served within a second, with no restart (KH-API-2, KH-LNT-3).
- An Admin API edit, or the MCP `edit_post` tool, can mark a real update with `real_update` and a short note. That writes today's date to `updated:` and the note to `updated_note:` in the post file, and the served page shows the new date. An edit that leaves the text as it was, or touches a post that is not live, is refused with a plain reason. Without the switch the date never moves.
- `khalid publish <slug>` publishes a draft through the same linter gate as the Admin API, then sends the `post.published` webhook and tells IndexNow, or sends `lint.blocked` with every finding when the linter refuses. `--language` picks a draft in another language and `--json` answers as the Admin API does (KH-OPS-2, KH-LNT-1, KH-API-3).
- Git mode: `khalid new "Title" --branch` starts a `post/<slug>` branch, writes and commits the new post there, and tells you how to push it and open a pull request. Merging the branch publishes the post (KH-EDT-4).
- Git mode can publish on push: with `KHALID_GIT_HOOK_SECRET` set, `khalid serve` accepts a push notice from GitHub or GitLab at `POST /api/hooks/git` when it is signed with that secret, pulls the change in the background and serves it. An unsigned push is turned away (KH-EDT-4).
- `khalid plan` suggests a publishing day for each draft that passes the linter, at the site's own pace over the last twelve weeks or at `--per-week <count>`. It puts each topic group's main post first, keeps days already scheduled, lists the drafts that must wait and why, and warns when fewer posts went live in the last four weeks than in the four before. It changes nothing, and `--json` gives the same plan to agents.
- Preview links: the Admin API makes one with `POST /api/admin/v1/posts/{id}/preview-link/` and says when it expires. `khalid serve` opens it as the draft in the site's own design, kept out of search and never cached. Set how long links last with `preview_link_days` or `preview_link_hours` in `site.toml`, from 1 hour to 30 days (7 days by default). An expired link says so and asks for a new one (KH-API-2).
- When a post goes live through the Admin API, `khalid serve` tells the outside world once: it sends the `post.published` webhook to every webhook that asked for it, sends the post's address to IndexNow when `[seo.indexnow]` is set, and retries failed webhook deliveries every minute. Editing a post that is already live tells nobody (KH-API-3, KH-SEO-11).
- You can schedule a post through the Admin API: an edit with `status: scheduled` and a future `published_at` schedules a draft, `unpublish_at` schedules its removal, and `status: draft` cancels. A time in the past, or an unpublish time before the publish time, is refused and nothing changes (KH-API-2).
- `khalid serve` checks every 5 seconds and puts each scheduled post live when its time comes, or takes it down, telling webhooks and IndexNow as usual. Schedules are kept in the database, so a restart loses nothing and does nothing twice, and a post the linter refuses stays a draft. The `post.scheduled` webhook fires when a post is given a publish time (KH-API-2, KH-API-3).
- An Admin API edit with `status: draft` takes a live post down: its file moves back into `drafts/`, the `post.unpublished` webhook fires, and IndexNow is told its address changed (KH-API-2, KH-API-3, KH-SEO-11).

#### Linter and audit

- Writing checks for answers: a question heading needs a standalone answer of 40 to 60 words, a paragraph warns past 225 words, hedge words, vague openers and trailing "which" chains are flagged in answers, and a bare statistic is asked for its source or year. A site can read and change the word lists.
- Topic groups: `topic_group:` and `main_post: true` in a post's front matter, and a warning (`cluster-without-pillar`) for a post in a group that never links to the group's main post.
- A review queue: a `reviewed:` date in a post's front matter records that someone checked it without changing its dates, and `khalid lint` warns when a post has gone six months without a review (`review-months` in `lint.toml`). The dashboard can list posts due for review.
- The linter warns when a site has no About or contact page.
- `khalid audit` builds the site, crawls it from the home page the way Ahrefs and Semrush do, and prints a health score out of 100 with each problem found, its group, how many pages have it and the fix. It exits with 1 when it finds an error, and `--json` prints the score, the counts and every issue (KH-LNT-5).
- `khalid audit` keeps every issue in `data/audit/issues.csv`, the internal linking report in `data/audit/links.csv`, and each run's score in `data/audit/runs.jsonl`, and ends its report with the recent scores so you can see the trend (KH-LNT-5).
- `khalid audit --server <address>` crawls a running server, such as `khalid serve` on your own machine or the live site, instead of building the site. Over HTTP it also checks that HTTPS pages send `Strict-Transport-Security` (KH-LNT-5).
- The audit can also crawl a folder written by `khalid build`, answering each address the way a static host would, with its redirects and root files (KH-LNT-5).
- The audit checks each page's title, meta description and `h1`: missing, empty, repeated, a title over 70 characters or a description over 160. It also flags pages that do not declare their language, pages kept out of search, and pages whose canonical points elsewhere (KH-LNT-5).
- The audit checks each page's body: low word count, low text to HTML ratio, HTML over 2 MB, nofollow links to your own pages, too many links, images with no `alt`, HTTP links and files on an HTTPS page, incomplete Open Graph tags, no Twitter card, and missing or broken structured data (KH-LNT-5).
- The audit checks the blog's links: broken pages, links to broken pages or redirects, redirect chains and loops, broken images, stylesheets and scripts, pages more than three clicks from home, and pages with only one followed link to them. It also writes an internal linking report with each page's click depth, links in and out, and a link rank from 0 to 100, with a CSV export (KH-LNT-5).
- The audit checks every link and file on other sites that your pages use and names each dead one as a warning, with its status. `--offline` skips them so the audit fetches nothing from the internet (KH-LNT-5).
- The audit follows each page's canonical and `hreflang` addresses and flags a canonical to a broken page or redirect, a translation that is broken, redirects or does not link back, a page that lists translations but not itself, and pages that share a title or meta description (KH-LNT-5).
- The audit reads `robots.txt`, the sitemaps and `llms.txt` first, as a search engine does, and flags a missing `robots.txt` or sitemap, a sitemap that fails, and broken, redirecting, noindex or non-canonical pages listed in a sitemap. It also names orphan pages that a sitemap lists but no page links to (KH-LNT-5).
- The site audit weighs every page: the bytes of its HTML and of the stylesheets, scripts, images and other files it loads. `khalid audit` names the heaviest page, keeps every page's weight in `data/audit/weights.csv`, heaviest first, and `--json` adds the heaviest page and its path (KH-LNT-5).
- The `updated-date-wrong` warning flags a post whose `updated:` date comes before its publish date or after tomorrow, with a plain fix.
- The linter warns when the focus keyword appears under 5 or over 25 times per thousand words in a post of 100 words or more, tunable as `focus-min-per-thousand-words` and `focus-max-per-thousand-words` in `lint.toml`. It also warns about each secondary keyword the text never uses (KH-LNT-4).
- The linter gives each post an SEO score out of 100 and lists the rules that cost points: 5 per warning rule, 30 per block, and at most 40 for a post that cannot be published (KH-LNT-4).
- `khalid lint --fix` now moves links to the site's own address from `http://` to `https://`, leaving code blocks alone, which answers the `insecure-internal-link` warning (KH-LNT-3).
- New warning `focus-not-in-h1`: when a post sets `seo_title:`, the headline on the page is checked for the focus keyword, and `focus-not-in-title` now checks the title search results show (KH-LNT-2).
- New warning `no-focus-keyword` for a post with no `focus_keyword:`, so its score cannot reach 100 without its topic being checked. Turn it off with `no-focus-keyword = "off"` under `[rules]` in `lint.toml` (KH-LNT-4).
- New warnings `short-title` for a title under 20 characters and `short-description` for a description under 50, set in `lint.toml` with `title-min-chars` and `description-min-chars` (KH-SEO-1, KH-LNT-2).
- The linter is ready to publish on crates.io as a library at version 0.1.0, under the same AGPL-3.0 licence, so other programs such as SerpDad can use it (KH-LNT-6).
- Alerts in plain words: `khalid serve` checks each hour for a burst of 404s, an HTTPS certificate with under two weeks left or past its end, and posts unchanged for too long, and raises a scheduled post the linter refused as it happens. `khalid audit` raises the broken links it finds. Set the email address, the 404 spike size and `stale_after_days` under `[alerts]` in `site.toml`.
- Alerts go to the owner by email when `[alerts] email` and `[mail]` are set, and always to webhooks for the new `alert.raised` event. Each problem is raised once while it lasts, again only if it clears and comes back, and a restart does not send it again.
- `khalid --site <folder> lint` checks every post against the SEO rules and explains each problem in three plain sentences. It exits with 1 when a rule blocks publishing, `--json` prints the findings for scripts, and `lint.toml` turns rules into warnings or off (KH-LNT-1, KH-LNT-2, KH-LNT-3).
- `khalid lint` scores each post out of 100 and lists every post below 100 with its score; `--json` adds the scores and the rules that cost points. When the Admin API refuses to publish a post, its report carries the same score (KH-LNT-4).
- `khalid lint --fix` makes safe repairs in the post files, then checks again: it puts headings back in order, gives an image with no description its caption as the description, and takes filler words and years out of a post's address while keeping the old address as a redirect. It only repairs what a rule reported and only changes those lines (KH-LNT-3).
- New `khalid lint` warnings: a focus topic missing from the title, address, first 100 words, description or headings; two collections that could give two posts the same address; a link to your own site marked `nofollow` or starting with `http://`; more links than `max-links` (default 100); and a hard-to-read post of 100 words or more, measured with the LIX reading index (KH-LNT-2).
- More `khalid lint` warnings: a post address with accented letters, such as `café-menus`, on a site with plain addresses (the default), naming the plain form `cafe-menus`; a link to an old address listed in another post's `aliases`; a link to a sister site served by the same Khalid; and a form whose answers go nowhere (KH-LNT-2).
- The publish gate refuses a post whose structured data, the search result details Google reads, lacks a property Google needs, and names the type and the property. The linter checks each post's structured data the way `khalid build` makes it (KH-LNT-1, KH-SEO-3).
- Shape checks for comparison, alternatives, best-of and review posts, read from the title. The linter warns when a comparison has no table, when a "vs" post's opening does not name both sides, when a numbered list gives fewer picks than its title promises, and when a review has no review block with a rating.
- A post can have a brief in `briefs/<language>/<slug>.toml`: the `questions` it answers, the `terms` it uses, the `links` it makes and `notes` for the writer. The linter warns about each question no heading asks, each term never used and each page not linked (`brief-question-unanswered`, `brief-term-missing`, `brief-link-missing`). These warn by default, and setting one to `block` in `lint.toml` stops the build and publishing. A broken brief stops with a plain message.
- A fact registry: list your facts, such as a price, in `facts.toml` with their earlier values and the day each was last checked. The linter warns with `fact-old-value` when a post still gives an old value, and with `fact-not-checked` when a fact has gone unchecked longer than `fact-check-months` in `lint.toml` (six months by default). Build and every way of publishing honour these rules, so a rule set to `block` stops the post.
- Page-or-section checks: the linter suggests moving a long section that never names the post's subject to its own page, asks a section that covers another post's whole subject to shrink to a summary and link there, and asks a short post that is already a section elsewhere to move into it.
- The linter warns when a post's `author:` has no profile in `content/<language>/authors/`, which used to leave bylines linking to a missing author page.
- Two new warnings: `link-text-misses-target` when link words share no word with the title of the post they open, and `link-text-is-own-focus` when a post links elsewhere using its own focus keyword. Both can be tuned in `lint.toml`.

#### SEO

- Every page gets a title, a description, a canonical address and robots directives from templates per page type, with a per-page override. Search results, date archives and thin tag or author archives stay out of search results by default (KH-SEO-1, KH-SEO-9).
- Every page gets Open Graph and Twitter card tags, so a shared link shows the right title, summary and image. A site sets one default share image and a post can set its own (KH-SEO-2).
- A page with no image of its own and no site default gets a generated share card. Each card is drawn once and kept in `data/share/`, redrawn only when its title or colours change, written to `share/` by `khalid build` and served by `khalid serve` (KH-SEO-2).
- Every page carries schema markup as one JSON-LD graph: the publisher (a company or a person, with logo and profiles), the site with its search box, the breadcrumb trail from its language's home page, and the page type (web page, collection, profile or search results) with its main image and the parts a voice assistant may read aloud (KH-SEO-3).
- Every post's schema is an article of the right type, with its headline, images, dates, word count, section and author, and names the things the post is about and mentions (KH-SEO-3).
- A post's author is described in full in schema markup: job title, bio, topics, their page on the site, their profiles elsewhere, and a new `credentials:` field in the author's front matter (KH-SEO-3).
- Recipe, how-to, video embed and review blocks write their own schema: ingredients, steps, times, servings and calories for recipes; steps, tools, supplies, time and cost for how-tos; the video's details; and the reviewer's and readers' ratings (KH-SEO-3).
- A local business can list each location in schema markup, with its business type, address, opening hours, phone, price range and map (KH-SEO-3).
- Glossary terms are marked up as defined terms of the glossary, in each language (KH-SEO-3).
- An XML sitemap at `sitemap.xml` lists every page search engines may index, with its last change, language versions, images and videos. It is split into one file per section and language at 50,000 addresses, and leaves out drafts, scheduled posts, thin archives and pages marked to stay out (KH-SEO-5).
- Redirects: moved for good (301), for now (302 or 307), gone (410) or blocked for a legal reason (451). A pattern can match many old addresses, such as every `/2019/05/<slug>/` to `/<slug>/`. A chain answers in one hop, a loop is refused, and a debug view shows what each rule answers (KH-SEO-8).
- A Redirects screen in the admin. Send an old address to a live post in one form, and Khalid adds it to the post's file so the redirect moves with the post. The screen lists kept redirects with who made them, removes one in a click, downloads and uploads them as CSV (the whole file or nothing), and lists the addresses that found nothing, most visited first, each with a suggested page to redirect to in one click (KH-SEO-8, KH-CON-9).
- `khalid bing sitemap` hands your sitemap to Bing Webmaster Tools, using the API key in `KHALID_BING_API_KEY`. A missing or refused key gets a plain message that says where to make one.
- `khalid bing report` shows Bing's last crawl day, the sitemaps Bing holds and each address Bing had trouble crawling, in plain words or as JSON.
- While `khalid serve` runs and `[seo.indexnow]` is on, a live post whose file gets a new or later `updated:` date, and a post file that goes live, are sent to IndexNow. Other edits are not. An address sent in the last ten minutes is not sent again.
- A post's schema lists its approved comments, each with its author, text, date and replies, and counts them in `commentCount`.
- A product card with a price or a score now adds product details for search engines: the price with its currency and shop link, and the score as a review by the post's author. A card with neither adds nothing, so it never holds a post back.
- A collection in `routes.toml` can say `news = true`. Its posts are marked as news articles, and the site gets a news sitemap at `sitemaps/news.xml` with the news posts of the last two days, listed in the sitemap index.
- The sitemap lists each post's images: its lead image, then each image in its body, once each.
- In the editor you can set the title search results show (`seo_title`) and the post's focus keyword. A Schema panel shows the kinds of thing the page's structured data describes, the JSON-LD itself, a link to Google's Rich Results Test once the post is live, and a field for your own JSON-LD. Khalid checks your JSON-LD and search title before saving and names the line and column of a slip (KH-SEO-1, KH-SEO-4, KH-LNT-4).
- A new Schema templates screen in the admin explains what schema is and lets you give every post in a collection the same schema.org type, such as "Every post in guides is a Course". You can add and remove templates; a template Google could not read is refused in plain words, and saved changes apply after Khalid restarts (KH-SEO-4).
- An HTML sitemap page at `/sitemap/` in each language links every page search engines may list, grouped as posts, pages, and topics and archives (KH-SEO-5).
- `khalid serve` now answers the sitemaps, `robots.txt`, `llms.txt` and the feeds at the same addresses `khalid build` writes them, sends each post alias on to its post with a permanent redirect, and gives served pages the same SEO head as built pages (KH-SEO-5, KH-SEO-6, KH-SEO-7, KH-SEO-8).
- `site.toml` takes a `[seo.indexnow]` table with your `key` and an optional `endpoint`. IndexNow tells Bing and the other search engines that share it about a changed page at once; Khalid makes the key file and retries a busy search engine after 10 seconds, 1 minute and 5 minutes. Khalid sends no sitemap pings, because Google and Bing have turned them off (KH-SEO-11).
- `site.toml` takes a `[seo.links]` table for links to other sites: the `rel` values they get (`nofollow`, `sponsored`, `ugc`), whether they open in a new tab, affiliate domains that are always `sponsored`, and domains that count as your own site. `khalid build` and `khalid serve` apply it to every post, page, glossary term and author bio (KH-SEO-10).
- The admin has a Search Console screen at `/admin/search-console`. It shows how many long searches no heading answers yet, then each post's long searches from the last 28 days with the heading that answers each, or "No heading yet". The same list is in the Admin API at `GET /api/admin/v1/search-console/follow-ups/` for a key with `settings:write`. A site that has not set up Search Console is told to set it up and restart the server.
- A post's lead image shows at the top of its page in every size, loads first, and becomes the page's Open Graph and X image. Its `seo_title` becomes the title in search results and the browser tab and may use template variables such as `{site}`, and a post with `sitemap: false` is left out of the XML sitemap.
- A reviewed post's structured data names its reviewers, each linked to their author page. An author who also reviewed the post is listed once (KH-SEO-3).
- Redirects you make, import, or that Khalid adds are kept in `data/site.db` with who made them and when, so they outlast a restart and a rebuild. Each is a 301, 302, 307, 410 or 451, for an exact address or a regular expression (KH-SEO-8).
- `khalid serve` answers stored redirects in one step for the reader, from the moment they are stored. A page always answers before a redirect, and a change that would make a loop is ignored so the last good set keeps working (KH-SEO-8).
- When a post's address changes, `khalid serve` adds a permanent redirect from the old address on its own, whether the change happened while it ran or while it was stopped. Moving a post back to its old address removes the redirect that would now loop (KH-SEO-8).
- `khalid serve` keeps a log of every address under the blog that answered "not found", with how often and when, without slowing any request. `GET /api/admin/v1/not-found/` lists them, most requested first, each with the nearest page by shared words or close spelling, or none when nothing is close (KH-SEO-8).
- "Redirect this": `POST /api/admin/v1/not-found/redirect/` turns an address that answered "not found" into a permanent move to the nearest page, or a page you choose, and takes it off the log. When no page is near, Khalid says so instead of guessing (KH-SEO-8).
- The Admin API manages redirects: `GET /api/admin/v1/redirects/` lists them with status, match, origin and date for any key with `settings:write`, `POST` adds one, and `DELETE /api/admin/v1/redirects/{id}/` removes one. A redirect that would close a loop or take an address that already has one is refused with a plain message, the same check the command line uses (KH-SEO-8).
- Redirect lists move in and out as CSV with the columns `from,to,status,match`, the same file every importer writes and the same four columns the database keeps. `POST /api/admin/v1/redirects/import/` stores a whole list or none of it, a bad row is refused with its line number, and `GET /api/admin/v1/redirects/export/` gives every stored redirect back in the same format (KH-SEO-8).
- A redirect debug view, `GET /api/admin/v1/redirects/explain/?path=`, shows which stored redirect an address matched, each hop of its chain, and the one step a visitor gets, in plain words too (KH-SEO-8).
- `khalid build` writes `_redirects`, which Netlify and Cloudflare Pages follow as it is: one 301 line for each old address a post lists under `aliases`, then the stored 301, 302 and 307 redirects, each chain in one step. Stored 410, 451 and regex rules need `khalid serve`. An alias outside the blog's folder, or one claimed by two posts, stops the build with a plain message (KH-SEO-8).
- `khalid build` gives every page its search tags and schema markup from the post's front matter and `site.toml`: title and description, robots, canonical, links between language versions, Open Graph and Twitter cards, and one JSON-LD graph (KH-SEO-2, KH-SEO-3, KH-SEO-9).
- `khalid build` writes a feed for each language in three formats beside its home page: RSS at `feed.xml`, Atom at `atom.xml` and JSON Feed at `feed.json`, each with the newest posts in full (KH-SEO-6).
- A post's front matter takes `robots`, written like the meta tag, such as `robots: noindex, nofollow`, and `canonical`, the full address of the original when the post first appeared elsewhere. Both go into the post's head, a `noindex` or re-pointed post leaves the sitemap, and a misspelt or contradicting directive gets a plain message (KH-SEO-9).
- `[seo.index]` in `site.toml` sets tag, category, author, date and search pages to `"index"` or `"noindex"`, and `thin_archives_below` keeps a tag or author archive with few posts out of search (KH-SEO-9).
- `[seo.titles]` in `site.toml` sets every page's title and description templates per page type, with the separator and the tagline. Every command checks the SEO settings when it starts and names the line of a wrong value.
- `[seo.crawlers]` in `site.toml` shapes `robots.txt`: paths no crawler may read, and crawlers allowed or blocked by name, such as `GPTBot`. `[seo.feeds]` sets what every feed carries: whole posts or excerpts, text around each item, a line naming the original post, and how many posts it holds.
- `[seo.social]` and `[seo.verification]` in `site.toml` reach every page: the default share image with its size and alt text, the Twitter account, and the verification codes for Google, Bing, Yandex, Baidu and Pinterest on the home page.
- `[seo.publisher]` in `site.toml` says who is behind the site, a company or a person, with logo, profiles, contact details and founding date, and `[[seo.locations]]` adds a shop, office or clinic with its address and opening hours. Both go into every page's schema.
- Schema templates per category or collection with `[[seo.schema]]`: every post in "Recipes" or `guides` can carry a schema.org item, such as a Course, filled from the post's title, summary and site name.
- FAQ, how-to, review, recipe and video blocks now write their own schema into the page, in build and serve. Videos also go to the video sitemap, links to Wikipedia or Wikidata become the page's `mentions`, and a review with no author of its own is credited to the post's author.
- A post's `schema` field, and a site-wide one, hold hand-written JSON-LD for types no block writes, such as an Event, a Course or a Product. Broken JSON stops the build with the file, line and column, and examples for nine such types come with the source.
- For a blog in a folder, such as `/blog/`, `khalid build` writes the host's `robots.txt` and any verification files into `_root/` in the output folder and says to copy them to the root of the main site, where search engines look. `khalid serve` answers `/robots.txt` at the root of its host too (KH-SEO-5).
- `khalid doctor` checks the main site's `robots.txt` for a blog in a folder and reports when it is missing or does not name the blog's sitemap, with the line to add or the file to copy. `--offline` skips this check (KH-SEO-5, KH-OPS-2).

#### AI search

- `khalid audit` checks articles the way an answer engine reads them: a question heading without a 40 to 60 word answer, a paragraph past 225 words, an answer that starts folded away in a closed `<details>`, hedging answers, numbers with no linked source or year, and an article whose latest date is hidden or more than 12 months old. It reads the limits from `lint.toml`, so the linter and the audit judge a post alike.
- An AI info page, made from the site's own settings and linked in every page's footer, tells readers and AI models who runs the site, what it covers, how to cite it and how to reach the team. Switch it off with `[seo.ai_page] off = true`.
- `llms.txt` gives language models a plain list of the blog's posts and pages with their summaries (KH-SEO-7).
- `robots.txt` lets every crawler in by default, names the sitemap, and can block or allow each crawler by name, such as the AI crawlers, or keep a folder private from all of them (KH-SEO-7).
- `khalid crawl first` shows how long each new post waited for each checked search or AI crawler to fetch it. Khalid remembers each page a crawler fetched, with its first and last fetch, even after the visitor log erases its rows. Needs the visitor log on.
- `khalid crawl health` shows each search and AI crawler's visits over the kept days, by what the site answered. Visits from addresses that only borrowed a crawler's name are set apart and never counted as that crawler. Needs the visitor log on.
- `khalid crawl errors` lists each address that answered search and AI crawlers with an error (4xx or 5xx) over the kept days, most hits first. Needs the visitor log on.
- While the visitor log is on, `khalid serve` raises an alert when Googlebot or Bingbot has not fetched a page for a week, and when crawlers got server errors in the last hour.
- Views from AI search now name the assistant that sent the reader: ChatGPT, Perplexity, Gemini, Copilot, Claude, DeepSeek, Le Chat, Meta AI, Grok, You.com or Phind. Views counted before this change show as "AI assistant, not named" (KH-GRO-4).
- `khalid analytics ai` shows how many readers each AI assistant sent in the last 28 days (or `--days`), against the days before, with the pages they landed on. The Admin API gives the same at `GET /api/admin/v1/analytics/ai-visitors/`, which needs `settings:write`.
- `khalid analytics uncited` lists the pages AI bots read in the last 30 days that no AI assistant sent a reader to, with the bots that read them. A bot's visit never counts as a citation. `--days` picks up to 90 days and `--json` prints the list for other tools.
- `khalid search-console follow-ups` lists the long searches of 8 words or more, the way people ask an AI assistant, that showed each post in the last 28 days, and names the heading that answers each, or says none does yet. `--json` prints the full list.

#### Themes and pages

- Fonts a site's licence lets it host are served by the site itself: `theme sync` downloads them into `theme/fonts/`, and `khalid build` serves them under `assets/fonts/` with `font-display: swap`. A font file that cannot be read is skipped and the blog keeps working with the next font in the list.
- Every page shows the menus written in `site.toml`: `[[nav.primary]]` in the header and `[[nav.footer]]` in the footer, in the order written, each link with a `label`, an `href` and an optional `language`. A link to the page shown is marked as the current page. A label or link a page could not show is refused with its line.
- An `[announcement]` in `site.toml` shows one line, linked or not, in a band above the header of every page or of one language's pages. It works with scripts off and is left out of print.
- Click to enlarge: a body image wider than the text column links to its widest size. With scripts on it opens in a dialog, with scripts off the link opens the image. `enlarge_images = false` under `[reader]` turns it off.
- Under each post, "Posts that link here" lists the site's posts that link to it, newest first. Turn it off with `linked_mentions` under `[reader]`.
- A link in a post to another page of the site shows that page's summary while it is pointed at or focused, and Escape hides it. With scripts off it is a plain link. Turn it off with `link_previews` under `[reader]`.
- A `[code]` table in `site.toml` adds the site's own code to every page: `head` for the end of the head and `body` (scripts only) for the end of the body. With `consent = "ask"`, the default, every page ends with an Allow or Do not allow banner that works with scripts off, and the site's code and a post's own head code run only for readers who allow it. With `consent = "not-needed"` the code runs for every reader (KH-SEO-12).
- `khalid build` and `khalid serve` make an archive of every post, twenty to a page, and a page for every author at `/authors/`. The archive is kept out of search results because the home page lists the same posts.
- Each collection gets its own list page where `routes.toml` puts it, such as `/guides/`, with the title, description and text of its `_index.md` above its posts (KH-CON-2).
- A collection whose address has date folders, such as `/news/{year}/{month}/{slug}/`, gets a page for each year and month, such as `/news/2026/09/` titled "September 2026". Date pages are kept out of search results unless `[seo.index]` says otherwise.
- Each tag gets a list page at `/tag/<slug>/`, each category at `/category/<slug>/`, and each channel in `routes.toml` at its own path, such as `/featured/`. You can rename the `tag` and `category` folders in `routes.toml`, and related posts now count shared tags.
- A post's own head code, such as an analytics snippet, goes into that post's page head after the SEO tags and runs on that post alone. On `khalid serve` the page's security policy allows just that code and the page is never cached (KH-EDT-3).
- A theme font that starts with Inter or Roboto gets a tuned local fallback, so text takes the same space before and after the web font arrives and the page does not shift (KH-WEB-5).
- A post can choose its own template, such as `template: wide` for `theme/templates/post-wide.html`. A theme may hold extra `post-<name>.html` files, and a post that asks for a missing one stops the build with a message naming both files (KH-EDT-3).
- Reading aids on the default templates: a `#` link after every H2 and H3 heading to share one section, a table of contents for posts with three or more sections (collapsible on phones, a column that stays in view on wide screens), a reading time such as "8 min read" beside the date, and body images that link to their largest version. All work with scripts off.
- Author features on the default templates: a byline with each author's photo, name and job title, then "Reviewed by"; an author box at the end of each post with photo, bio, profile links and reviewers, which a theme can replace with `author_box.html`; an authors page listing every author with their post count; and the site's other authors at the end of each author page.
- New lists on the default templates: an archive page of every published post, newest first; post cards showing the author's name and photo, the date and the main category; three to five related posts under each post, chosen from links, categories and tags, the same every build; and home page sections per category, collection or channel listed under `[[reader.home_sections]]` in `site.toml`, each with a "View all" link and a row of links to every topic below.
- A `[reader]` table in `site.toml` switches off any reader feature by name, such as `table_of_contents = false`. Every feature is on by default.
- An optional reader script adds polish to pages that read fully without it: a heading's `#` link copies the address, an enlarged image opens in a dialog that Escape closes, `/`, Ctrl+K or Cmd+K opens search, and the contents column marks the section being read. It is one small file checked by its hash.
- `khalid theme sync <url>` reads your main site's colours, fonts, logo and icon and saves them in `theme/tokens.toml`, saying where the file is. When the site cannot be reached, has no stylesheet, or points to a private address, it says what happened and what to do, and changes nothing. `--json` gives the same result for scripts (KH-OPS-2, KH-WEB-8).
- Theme sync checks the text, link and accent colours it copies against WCAG AA, the web's standard for readable pages. A colour that is too faint gets the closest shade that passes, and you are told what changed and how to choose another (KH-WEB-4, KH-WEB-8).
- `khalid theme sync <url> --from-css <file>` takes colours and fonts from a CSS file you saved instead of reading the site, useful for a site that draws its pages with JavaScript.
- A `locked` list at the top of `theme/tokens.toml`, such as `locked = ["colors.link"]` or `"brand.logo"`, names values you set by hand. Running theme sync again keeps them, takes the main site's new values for the rest, and prints one line per changed value. An unknown name or a mistake in the file is reported with its line (KH-OPS-2, KH-WEB-8).
- `khalid theme sync` draws a preview page, a sample post in the copied colours and fonts, at `data/theme-preview/index.html`, so you can see the look in a browser before you publish.
- `khalid theme check` compares `theme/tokens.toml` with your main site and with WCAG AA, lists each value that no longer matches and each colour too faint to read, and changes nothing. It exits with 1 when something needs you, so a scheduled job can watch for it, and `--json` gives the same result (KH-OPS-2, KH-WEB-8).

#### Search

- Every language of a blog has a search page at `search/`, such as `/blog/search/`, linked from every page's header. It is a plain form that works with scripts off, and the site tells search engines about it so they may show a search box. Search pages ask not to be listed in search engines unless `[seo.index] search` says otherwise (KH-WEB-2).
- Search results list the posts that hold every word, best first: a word in a title counts more than one in a heading, a heading more than the body, and a rare word more than a common one. Each result shows about thirty words of the post with the matches marked, each language searches its own posts, and a search stays fast on a large blog (KH-WEB-2).
- Search matches English and German words by their stem, so a search for "links" finds "linking" (KH-WEB-2).
- `khalid serve` keeps the search index up to date in the background: at start and after each save, for changed posts only. Drafts never show in search, and results are never cached, so a save shows at once (KH-WEB-2).
- With the reader script on, results show while a reader types, and the word being typed matches the start of longer words, so "sitem" finds "sitemaps". With scripts off the form works as before (KH-WEB-2).
- On a static site, the search page shows a note that search needs a running server, why, and the command that turns it on, and `khalid build` writes a small `search/index.json` beside each search page (KH-WEB-2, KH-OPS-1).

#### Comments

- `khalid serve` shows approved comments under every post with one level of replies, a reply form under each comment and a form for a new comment. The forms are plain HTML and work with scripts off. A name and the comment are needed, the email is optional and never shown, and a hidden field catches bots. Static and headless sites take no comments.
- A sent comment takes the reader back to the post with a thank-you note, where it waits for approval. If something is wrong, the reader gets a short page in their language listing every mistake, with the form filled in again so nothing is lost.
- Comments follow one set of rules wherever they come from: a reader's comment waits for approval, a reply answers a shown comment on the same post, a staff reply shows at once and approves what it answers, and deleting a comment removes its replies. The post page updates a moment later, without a restart.

#### Newsletters and forms

- Posts can ask readers to subscribe with `::newsletter`, and keep part of a post for subscribers with `:::gated`, which shows a note and the sign-up form but never the hidden part (KH-CON-5).
- Mail sending: a `[mail]` table in `site.toml` sets the from address, an optional sender name, and one way to send, `[mail.smtp]`, `[mail.resend]` or `[mail.postmark]`. With it, `khalid serve` opens newsletter sign-ups and sends the confirmation email. Without it, sign-ups stay closed as before.
- A site can have several newsletters: each folder in `content/newsletters/`, such as `content/newsletters/weekly/`, is one newsletter beside `main`, and `::newsletter{label="..." button="..." list="weekly"}` signs readers up for it. A form that names a newsletter the site does not have saves nothing and says so (KH-GRO-2).
- `khalid newsletter send <slug>` emails a published post once to a newsletter's confirmed subscribers, with `--list` to pick the newsletter and `--label` to narrow it to a segment. Each copy holds the title, a link, the text, a one-click unsubscribe link and the site's postal address, and an edit never sends a post again (KH-GRO-2).
- `postal_address` in `[mail]` sets the postal address every newsletter email ends with. Khalid sends no newsletter or welcome email until it is set.
- Welcome emails: a reader who confirms gets the welcome email of each newsletter they joined, written in `content/newsletters/<name>/welcome.md` with the subject as its first `# ` heading (KH-GRO-2).
- RSS to email: `khalid newsletter digest` emails the posts published since the last digest and sends nothing when nothing is new, so a timer can run it (KH-GRO-3).
- `khalid newsletter embed --list <name>` prints a plain HTML sign-up form to paste into another website, such as the main site. It works with scripts off (KH-GRO-3).
- Subscriber labels and segments: `khalid newsletter label <email> <label>` gives a subscriber a label and `--remove` takes it away, and `khalid newsletter segment --list <name> --label <label>` counts the confirmed subscribers who carry every label named (KH-GRO-2).
- `khalid newsletter numbers` shows each email that went out with copies sent, copies refused and readers who unsubscribed through it. Totals per email only, never per person, and opens are not counted (KH-GRO-2).
- Gated content opens on `khalid serve` with scripts off: a reader signs up in the form on the post, confirms from the email, and lands back on the post with the gated part open. A reader cookie keeps gated parts open on every post for 180 days, and a returning subscriber on a new device gets an emailed link that works for two days.
- Forms work on `khalid serve`: a form block posts its answers, the reader sees a thank-you page, and the message is kept in the site database. Khalid then emails the answer to `forms_to` under `[mail]`, or to the `from` address, and sends the `form.submitted` webhook with every field, without making the reader wait (KH-GRO-1).
- Newsletter sign-ups with double opt-in: a sign-up sends one confirmation email, in English or German, whose link confirms within two days or removes the address at once. Repeat sign-ups within ten minutes send one email, and an already confirmed reader gets none. Each email can carry its own one-click unsubscribe link that takes the reader off every newsletter of the site and works from mail programs too (KH-GRO-2).
- `khalid serve` answers the sign-up form at `subscribe` and `/subscribe`, where the `::newsletter` block posts, and shows short pages in English and German for each step: "Check your inbox", confirm, unsubscribe, an old or broken link, and a site that cannot send email yet. Each page works with scripts off and is kept out of search. Until the site has mail settings, every newsletter page says "Signing up is not open yet" and nothing is stored (KH-GRO-2).
- On a static site, each sign-up, contact and search form becomes one short sentence in the page's language, such as "Signing up by email is not open on this site.", because a static host cannot answer it. Gated content keeps its note, and a form that posts to another site stays (KH-OPS-1).

#### Analytics

- An Analytics screen in the admin: page views for the last 7, 28 or 90 days against the days before, as a chart with a written summary and a table, where readers came from, the most read pages, the readers each AI assistant sent, and the pages AI bots read but no AI assistant cited.
- `GET /api/admin/v1/analytics/visitors/?days=28` counts page views per day, per source (direct, search, social, AI, other sites, the site itself) and per page against the days before, and `GET /api/admin/v1/analytics/uncited/?days=30` lists pages AI bots read that no AI assistant sent a reader to. Both need `settings:write` (KH-GRO-4).
- A visitor log for `khalid serve`, off until you set `[visitor_log] on = true` in `site.toml`. It keeps one row per visit for 7 to 14 days with a shortened address, a visitor code that changes every day, and whether a bot's address is on its company's published list. No cookies. The privacy design is in `docs/design/visitor-log.md`.
- `khalid serve` counts page views on the server with no cookies and no script, so readers with scripts blocked are counted too. Each view is kept only as the day, the page and where the reader came from (search, social, AI search such as ChatGPT, Perplexity, Gemini, Copilot and Claude, another site, the site itself or nowhere). Error pages, HEAD requests and browser prefetches are not views (KH-GRO-4).
- Bots are named from their user agent, such as Googlebot, Bingbot, GPTBot, ClaudeBot and PerplexityBot, and never counted as readers. Each bot visit goes to a crawl log with the address and the status it got, kept for 90 days with at most 10,000 rows a day (KH-GRO-4).
- An optional Google Search Console connection with your own Google account: add `[search_console]` to `site.toml` and set `KHALID_GOOGLE_CLIENT_SECRET` and `KHALID_GOOGLE_REFRESH_TOKEN`. Without them Khalid asks Google nothing.
- `khalid search-console check`, `report`, `post <slug>`, `index` and `email` show each post's clicks, impressions, click rate and position for the last 28 days, the searches behind a post, index status, the week's winners and losers, posts whose clicks fell three months in a row, and pages missing or set to noindex that still get search traffic. A weekly email and "Go deeper in SerpDad" links are included.

#### API, webhooks and SDK

- `khalid serve` answers the Content API at `/api/content/v1/` with a content key, and an edit shows in the API without a restart (KH-API-1).
- The Content API lists and reads the site's authors, tags, categories, collections and menus, by identity (such as `en.asif`) or slug, and answers `/api/content/v1/settings/` with the site's name, front page address and languages. Authors take the same `filter`, `order`, `limit` and `page` as posts (KH-API-1).
- `khalid serve` answers the Admin API at `/api/admin/v1/` with an integration key or staff token. New posts need an `author`, since the linter refuses a post without one (KH-API-2).
- `GET /api/admin/v1/posts/` lists posts in every status with their title, status, review state, dates, check counts and search visibility, and takes `filter`, `order`, `limit` and `page`, such as `?filter=gate:refuses+review_state:in_review`. A page holds at most 100 posts, so `limit=all` is refused (KH-API-2).
- An Admin API edit can change any of a post's settings: slug, author, date, co-authors, excerpt, feature image, featured, review state, canonical, robots, share cards and aliases. A setting left out keeps its value, `null` clears it, a setting that cannot be right is refused with a message that names it, and the post's file keeps every other line as it was (KH-EDT-3, KH-API-2).
- The Admin API creates a post from HTML with `source=html`. Khalid cleans the HTML first, removing scripts, frames, forms and unsafe links, then stores it as markdown (KH-API-2).
- Webhooks: 32 named events, each delivery signed with `Khalid-Signature`, retried for up to a day, and kept in a delivery log (KH-API-3).
- The Admin API moderates comments: list them newest first by status at `GET /api/admin/v1/comments/?status=pending`, approve with `POST /api/admin/v1/comments/{id}/approve/`, reply as staff with `POST /api/admin/v1/comments/{id}/replies/`, and delete a comment with its replies with `DELETE /api/admin/v1/comments/{id}/`. A reader's email shows only to moderators.
- Every new comment, from a reader or a staff reply, fires the `comment.added` webhook once. The reader's email is never sent.
- The Admin API manages pages: list them with `GET /api/admin/v1/pages/`, create one with `POST`, and read, edit or delete one at `/api/admin/v1/pages/{id}/`, such as `en.about`. A new page is live at once as `content/<language>/pages/<slug>.md`, an edit changes only the fields sent, and an edit or delete made on an older copy is refused with a 409 (KH-API-2).
- The Admin API manages snippets at `/api/admin/v1/snippets/`, such as `en.affiliate-disclosure`: list, create, read, edit and delete, kept as markdown files in `content/<language>/snippets/`. A change made on an older copy is refused (KH-API-2).
- The Admin API manages authors at `/api/admin/v1/authors/`: list every author with their role, credentials, profiles, photo and bio, and create, read, edit or delete one, such as `en.asif`. Only the fields sent change, and an edit shows on the author's page at once (KH-API-2).
- The Admin API manages categories at `/api/admin/v1/categories/`: list them with their names in each language and post counts, create, read, rename and delete. `categories.toml` is edited in place so its comments stay, and a category posts still use cannot be deleted (KH-API-2).
- The Admin API manages tags at `/api/admin/v1/tags/`: list every tag with how many posts have it, drafts included, rename a tag in every post (merging it into a tag of the same name), or remove it from them all. Only the `tags` line of each post changes, and tag pages follow without a restart (KH-API-2).
- `DELETE /api/admin/v1/posts/{id}/?updated_at=...` removes a post in any status, and its page then answers 404. A delete made on an older copy is refused, and a live or scheduled post needs `posts:publish` (KH-API-2).
- The Admin API reads and changes the site's name, address, mount and languages at `/api/admin/v1/settings/`. Only the fields sent change, a value the site could not start with is refused naming the field, `site.toml` keeps its comments, and the answer says whether a restart is needed to apply it (KH-API-2).
- Add, list and switch off webhooks through the Admin API at `/api/admin/v1/webhooks/` with a key that has `webhooks:manage`. A new webhook's secret is shown once, a list never shows secrets, a switched-off webhook keeps its delivery log, and changes take effect on a running `khalid serve` at once (KH-API-2, KH-API-3).
- `GET /api/admin/v1/webhooks/{id}/deliveries/` lists the 50 newest messages a webhook sent, each with every try and whether it was delivered, waiting or given up (KH-API-3).
- New post webhook events: `post.published.edited` when an edit changes a live post (which also tells IndexNow), `lint.blocked` with every finding when the linter refuses a post, scheduled ones included, and `post.deleted` when a post is deleted through the Admin API. An edit that changes nothing is not announced (KH-API-3, KH-LNT-1).
- New page webhook events: `page.published`, `page.published.edited`, `page.deleted` and `page.unpublished` (a file moved to `drafts/`), with the same events for glossary terms, case studies and changelog entries (KH-API-3).
- More webhook events: `tag.added` when a live post first uses a new tag, `subscriber.added` when a reader confirms a sign-up (without their email address), `site.changed` naming the `site.toml` settings that changed (never their values), and `audit.finished` with the health score and counts after `khalid audit` (KH-API-3).
- The Admin API lists, reads, edits and uploads media library files, telling duplicates and refused files apart (KH-EDT-5). `POST /api/admin/v1/media/{id}/copies/` makes a turned or cropped copy of a picture, and `GET /api/admin/v1/media/search/` searches Unsplash and GIPHY through the blog (KH-EDT-1).
- `GET /api/admin/v1/posts/{id}/versions/` lists a post's earlier versions, and `POST` and `DELETE /api/admin/v1/posts/{id}/presence/` check in and leave a post being edited (KH-EDT-2).
- A TypeScript SDK, `@khalid/sdk` in `sdk/typescript/`, made from the API's OpenAPI document. `createClient` gives a client typed for every Content API and Admin API operation that signs calls with the content key or the admin key (KH-API-4).
- `GET /api/admin/v1/posts/{id}/check/` checks a post as it would go live now and saves nothing: every lint finding with the score out of 100, the search result its page would make, and the full JSON-LD the page would carry (KH-SEO-1, KH-SEO-4, KH-LNT-4).
- `GET` and `PUT /api/admin/v1/schema-templates/` read and replace the site's schema templates, which are kept as `[[seo.schema]]` in `site.toml` with every other line and comment left as it was. They need `settings:write`, and a change made on an old copy is refused (KH-SEO-4).
- A finished article sent from SerpDad lands as a linted draft in under 5 seconds: creating a post through `POST /api/admin/v1/posts/` now answers with its lint findings and where it opens in the admin, and a new post can take an `excerpt`, kept as its description (KH-API-6).
- Two new API errors: `not-set-up` (409) when the site has not set up what a request needs, and `upstream` (502) when a service Khalid asked, such as Google, gave no usable answer.
- `khalid mcp` lets AI agents such as Claude Code work on a blog through MCP (Model Context Protocol, a standard way for agents to use tools). Started with a key in `KHALID_KEY`, an agent can list and read posts, save drafts from markdown or cleaned HTML, edit a post's title or body, check a post with `lint_post`, and publish a draft only when the linter lets it through. A post it has just published is listed at once, and a failed call is explained plainly so the agent can correct it (KH-API-5).
- Each post in the Content API now has a `head`: the title, meta and link tags its page carries, in order, and its JSON-LD. A headless front end keeps the same search tags and schema as the built-in theme (KH-INS-5).
- Starter kits for a headless blog that reads the Content API: an Astro site in `starters/astro/` and a Next.js app in `starters/nextjs/`, each showing one page per post with the theme's meta tags and JSON-LD (KH-INS-5).

#### Accounts and sign-in

- Staff accounts, kept in the site's database: each has an email, a name and one of five roles (owner, admin, editor, writer, contributor), and a site has exactly one owner. Staff sign in with email and password, and sign out again (KH-EDT-7).
- `khalid staff owner --email <address> --name <name>` makes the site's first owner and shows a sign-in link once, valid for 15 minutes (KH-EDT-7).
- Staff can add a passkey, unlocked by a fingerprint, a face or a PIN, and then sign in with it instead of a password (KH-EDT-7).
- Staff can ask for a sign-in link by email and use it once, within 15 minutes. A site that cannot send mail says so in plain words (KH-EDT-7).
- Staff can sign out on every device at once, for a lost laptop or a stolen cookie, and change their password, which signs out every other device (KH-EDT-7).
- Owners, admins and editors can list staff, invite someone, change a role or remove an account, only within the roles they may manage. The owner can hand the site to another account and becomes an admin (KH-EDT-7).
- A Passkeys screen in the admin. You can add a passkey from the device you are on, name each passkey, and remove one you no longer have. The Admin API gains routes to list, rename and remove your own passkeys.

#### Media

- A Media screen lists the library newest first with each file's folder, kind, size and alt text, and one switch shows the images still missing alt text. It uploads into a folder, says when the library already had the file, asks for alt text on a new image, and says in plain words why a file was refused (KH-EDT-5).
- Each media file has a details page: copy its address, change its alt text, caption and folder, and set an image's focal point, the part every crop keeps, with a click or two numbers (KH-EDT-5).
- A post kept as a folder publishes the files beside its `index.md` at the post's address, such as `/blog/fixing-slow-pages/page-weight.png`, and a body image named by its file name points there in the page and the feeds. PNG, JPEG, WebP, AVIF, GIF, SVG and PDF files are sent with their own type and never compressed again (KH-CON-3).
- Images beside a folder post are offered in several sizes and as WebP, cached for a year. The page uses a `picture` with width, height and alt text set, so the page does not move while images load (KH-WEB-5, KH-CON-3).
- A media library in the Admin API under `/api/admin/v1/media/`. Upload images, video, audio and other files, file them in folders, and give each alt text, a caption and a focal point. Files are checked by their content, have a size limit per kind, are never stored twice, and can be listed with `filter`, `order`, `limit` and `page`. Every route needs a key with `media:write`.
- `khalid serve` serves uploaded media from the blog's own address under `media/library/`, cached by browsers for a year since a changed file gets a new address. With a `[media.s3]` table in `site.toml`, the library lives in an S3-compatible bucket such as Cloudflare R2, with its keys read from `KHALID_S3_ACCESS_KEY_ID` and `KHALID_S3_SECRET_ACCESS_KEY`, and readers are sent to the bucket's public address. Without it, media stays on disk in `media/`.
- A post's lead image shows on its cards on author pages, lists and related posts, loaded as it nears the screen. Home page cards have no image yet.

#### Import and export

- `khalid db export` writes every table of the site as a CSV file and a JSON file into `data/export/` or the folder `--out` names, leaving out sign-in tables and every secret.
- `khalid import wordpress|ghost <export>` brings a blog in, keeps every slug, and stores the redirects for old addresses. `--dry-run` reports what would happen and writes nothing (KH-API-7).
- `khalid import wordpress https://<site>` reads a live WordPress site's published posts and pages, including Yoast's search title, description, canonical address and robots setting (KH-API-7).
- `khalid import markdown <folder>` brings in a Hugo, Jekyll, Astro or Eleventy blog with YAML or TOML front matter. Dated files become posts, undated files pages, and drafts stay drafts. Slugs are kept, old addresses become aliases or redirects, and Hugo and Jekyll `figure`, `youtube`, `vimeo` and `highlight` tags become Khalid blocks; any other tag or unreadable file is named (KH-API-7).
- `khalid import substack <folder>` brings in an unpacked Substack export with a dry-run report. Posts keep their slugs, paid posts become members-only, images become figures, YouTube and Vimeo players become embeds, and each old `/p/<slug>` address redirects. Threads, subscribe boxes and buttons are left out with the reason (KH-API-7).
- `khalid import medium <folder>` brings in an unpacked Medium export. Every import now stores its redirects in the site's database, so no second command is needed (KH-API-7, KH-SEO-8).
- The Ghost importer reads Ghost JSON exports from every version, turning Lexical, Mobiledoc and HTML posts into markdown: headings, lists, quotes, links, code with its spacing, images with captions, and embed, bookmark, callout, button, toggle, gallery, header, audio, file and video cards. A file that is not a Ghost export is refused with the reason (KH-API-7).
- A Ghost import keeps each post's slug, date, summary, feature image, canonical address, meta title and description, and social card fields, and writes one file per author with their bio and profiles. Drafts, scheduled and members-only posts become drafts, so nothing paid is published, and a post whose slug changed keeps its old address as an alias (KH-API-7).
- A Ghost import writes `import/ghost-redirects.csv`, mapping every old post, page and author address to its new one, and a dry-run report in plain sentences: what comes in, which slugs changed, what was left out or failed and why. Nothing is written if any of its files already exists (KH-API-7).
- Khalid can import a Medium export. It reads each post's title, subtitle, body, date, address, author and draft state, and turns the body into Khalid markdown: headings, code with its language, images with captions, videos as embeds and link cards as bookmarks. Published posts go to `content/<lang>/posts/`, drafts to `drafts/<lang>/`, the subtitle becomes the description and each slug is kept without Medium's id (KH-API-7).
- A Medium import maps every old address of each post, on medium.com, on a publication's own domain and at the short `/p/` address, to its new one, and writes the list to `import/medium-redirects.csv`. Each author gets a file with their Medium profile, and a dry-run report says in plain sentences what comes in, what was left out, which files were not posts and which old addresses Medium cannot redirect (KH-API-7).
- The WordPress importer reads a WordPress export file: the site, authors, categories, tags, and every post, page and attachment with its slug, date, body, summary and custom fields. A file that is not a WordPress export, or a bad value, is refused with its line (KH-API-7).
- The WordPress importer turns post bodies into markdown, from block editor and classic editor posts, with video frames as embed blocks. Scripts and unsafe links are dropped, and shortcodes stay as text and are named in the report (KH-API-7).
- The WordPress importer brings in published posts, pages, drafts, posts waiting for review and scheduled posts, which wait in `drafts/` with the time they were due, plus one file per author and `categories.toml`. Every slug that fits the site's rules is kept, and a new one is made only when it must (KH-API-7).
- The WordPress importer carries each post's SEO settings from Yoast SEO, Rank Math, All in One SEO and SEOPress: search title and description, canonical, robots, focus keyword and primary category. When a post holds both Rank Math and Yoast fields, Rank Math wins (KH-API-7).
- A WordPress import keeps old links working: every old address of a post, page, image page or category archive gets a redirect or is listed as kept, and the Redirection plugin's rules come in too. Everything that redirects is written to `import/wordpress-redirects.csv`, and rules Khalid cannot answer are listed with the reason (KH-API-7, KH-SEO-8).
- A WordPress import has a dry-run report: how many posts, drafts, pages, authors and categories come in, which slugs are kept, made or changed, how many old addresses move, and everything left behind, grouped by reason in plain words. A test with 500 posts shows nothing is lost (KH-API-7).
- A WordPress import writes into a site folder one file at a time, safely, and writes nothing when a file already exists or a folder leads outside the site, naming the files in the way (KH-API-7).

#### Speed and hosting

- Every built page carries its referrer policy as a meta tag, for hosts that drop headers, and `khalid serve` tells the browser to preload the stylesheet, which a server in front can send early as a 103 Early Hints.
- `khalid serve` checks the blog's home page at its public address every 5 minutes, as a reader would, and alerts you by email and the `alert.raised` webhook when the site stops answering (`site-down`) or answers slowly for an hour (`slow-site`). Set `watch_minutes` and `slow_ms` in `[alerts]`; 0 switches each off.
- `khalid watch` shows how fast and how reliably the blog answered over the last day and the last 30 days, in plain words or as JSON.
- Pages load a small reader script only when a feature it improves is on (heading links, click to enlarge, the search shortcut, the contents). Browsers may keep it for a year, and pages read completely without it.
- Templates to run Khalid on Fly.io (`fly.toml`), Railway (`railway.json`), Coolify (`packaging/coolify/compose.yaml`) and a Hetzner Cloud server (`packaging/hetzner/cloud-config.yaml`), with steps in `docs/deploy.md`.
- `khalid serve --init` makes the site on the first start when the folder is empty, at the address in `KHALID_ADDRESS`, so a host that starts Khalid on an empty disk needs no extra step.
- Ready-made hosting files: a `Dockerfile` that builds Khalid as one small static image that runs as a normal user, a `compose.yaml` that serves `./site`, and a systemd unit in `packaging/systemd/` that runs `khalid serve` as its own user with the site at `/var/lib/khalid`. `docs/deploy.md` explains the setup.
- `khalid build` on a static site (`mode = "static"` in `site.toml`) lists the six features that need a server: comments, newsletter sign-up, forms, search, the admin and scheduled publishing, each with why and how to turn it on. `khalid doctor` lists the same and still exits with 0, and `--json` gives the list under `off` (KH-OPS-1, KH-OPS-2).
- `khalid --site <folder> serve` runs the blog on `127.0.0.1:8080`, or the address given with `--listen`. It shows a saved edit within about half a second without a restart and stops cleanly on Ctrl+C or a stop signal (KH-OPS-5).

#### Command line

- `khalid --help` lists every command with a one-line summary. A mistyped command gets a plain explanation and exit code 2 instead of a wall of usage text.
- `--json` on every command prints the same message as JSON, for scripts and agents.
- `khalid --site <folder> db migrate` creates the site's database at `data/site.db` or brings it up to date, and `khalid --site <folder> db rebuild` refills the site's index from its files while keeping comments, subscribers and other visitor data.
- `khalid keys create` makes a content key, an integration key with scopes, or a staff token with a role, and shows it once. `khalid keys revoke <id>` stops a key at once (KH-API-2).
- `khalid --site <folder> build` writes the site as HTML into `public/`, or the folder given with `--out`: a home page and a 404 page per language, a page for every dated post, each author's page at `/<mount>/authors/<slug>/` and each plain page such as About at `/<mount>/<slug>/`. Posts with no date are drafts and are left out, and it never writes into `content/` (KH-OPS-3).
- `khalid --site <folder> new "Title"` writes a new post to `content/<language>/posts/<date>-<slug>.md` with its title, slug and today's date. `--language <code>` picks another of the site's languages, and the slug follows that language's rules. It never replaces an existing post (KH-OPS-3).
- `khalid webhooks add --event <event> --url <address>` adds a webhook and shows its secret once, `khalid webhooks list` shows the webhooks that are on, and `khalid webhooks remove <id>` switches one off. Each takes `--json` (KH-API-3, KH-OPS-2).
- `khalid backup` writes one `.tar.gz` archive of the site folder, with a safe copy of the database taken while `khalid serve` runs, so comments, subscribers and keys are kept. It leaves out what Khalid can make again and never writes over a file, and `--out` names the archive (KH-OPS-2).
- `khalid export` writes a static site into `public/` whole: it swaps in the new site only once every page is written, so a host never copies half a site and pages of deleted posts are gone. A refused build leaves the earlier export in place (KH-OPS-1, KH-OPS-2).
- `khalid pull` brings a git-mode site forward to what was pushed to its repository, checks it, and rebuilds a static site through the publish gate. It only moves forward, so local changes stop it instead of being merged or lost (KH-OPS-2).
- `khalid update` finds the newest release for this system and installs it once the download matches the release's `SHA256SUMS`, keeping the old version beside it. `khalid update --rollback` puts the earlier version back, and a second rollback undoes the first (KH-OPS-2).
- `khalid agents` prints the built-in guide for AI agents: how a site's files are laid out, how to connect an agent with a key, what each MCP tool does, and how to get a post past the linter (KH-API-5).
- `khalid new --author "Sam Lee"` writes `author: sam-lee` into the post and creates the author's profile when it does not exist. Without `--author`, a new post names the language's only author, or says how to add one.
- `khalid new --draft "Title"` writes an undated draft in `drafts/<language>/<slug>.md`, which readers never see, and prints the `khalid publish` command that takes it live. Without `--draft`, `new` says the post goes live at the next build and where drafts go instead (KH-EDT-4, KH-OPS-3).
- `khalid redirects import <file>` stores a CSV list of redirects whole or not at all and says how many start outside the blog's folder. `khalid redirects export` prints every stored redirect as CSV, and `khalid redirects explain <path>` says which redirect an address matches and where a reader ends up (KH-SEO-8).

#### Many sites

- One Khalid process can serve many sites. `khalid sites add <domain>` turns a folder into a sites folder with one folder per domain, `khalid sites admin <email>` names a super admin, and `khalid serve` answers each site on its own domain. A super admin lists and adds sites through `/api/admin/v1/sites/`, and a new site answers without a restart (KH-EDT-7).
- The admin has a Sites page at `/admin/sites` for a server that runs many sites. A super admin signs in with an emailed link, sees every site with the home site first, adds a site by its domain, and is told what to do next for DNS and the new site's owner (KH-EDT-7).

### Changed

#### Install and setup

- Release binaries for Linux are now fully static, so one file runs on any Linux. Every release binary holds the admin, and there is a new build for Intel Macs, which the install script now installs.
- `khalid init` says how to make the owner of a server site, and the README and `docs/deploy.md` explain the first sign-in.

#### Writing and content

- When a file has a field Khalid does not know, the fix suggests the nearest field name, such as "rename it `title`", and lists the allowed fields only when there are 12 or fewer.

#### Editor and admin

- Deleting a post through the Admin API now moves it to the trash instead of removing its file.

#### Publishing

- While `khalid serve` runs, a saved post's page shows early only when nothing else on it can change. Otherwise it shows once the whole site has rendered again, so you never see a page that the full render would change.
- A post the linter blocks no longer stops the whole build. `khalid build` and `khalid export` leave it out, write the rest of the site, and list what they left out and why, and `khalid serve` leaves out the same posts.

#### Linter and audit

- Every `khalid audit` issue now says why it matters, between what is wrong and how to fix it: in the report, as `why` in `--json`, and as a column in `data/audit/issues.csv`.
- `llms.txt` problems in the audit are now notices, since `llms.txt` is not the main way AI search reads a site.
- The audit no longer checks the meta description of a page kept out of search results, such as the search page, because such a page shows no snippet (KH-LNT-5).
- The linter checks the focus keyword in six places, each with its own message: the title, the description, the first paragraph, a subheading, the web address and an image description. These replace `focus-topic-missing` (KH-LNT-4).
- `keyword-stuffing` leaves the focus keyword to `focus-too-often`, so you get one warning for one problem.
- `khalid lint`, `khalid build` and the publish check now read the fact registry (`facts.toml`) and each post's brief (`briefs/`) together, so a post is held to both.
- A site's first post can now follow every lint message: a link to the home page counts as an internal link while it is the only post, and a language's only post is no longer called an orphan.

#### SEO

- Search Console errors now end with "try again" rather than "run the command again", since the admin shows them too.

#### Themes and pages

- FAQ and toggle blocks show their answers open by default, so a reader with scripts off, and an engine reading the page, sees every answer without clicking.
- On a static site, comments and gated content each become one short note in a grey card, such as "Comments are not open on this site." in the post's language, instead of a section that cannot work. `khalid build` and `khalid doctor` tell the owner.
- Templates print a page's SEO tags and schema from `meta.seo_head` in place of the base layout's smaller set. A theme that replaces `base.html` must print `{{ meta.seo_head }}` to keep them.
- `khalid theme sync` on a site that draws its pages with JavaScript now saves the clean default look, the logo and the icon instead of stopping, and says how to use the site's own colours with `--from-css`.
- `khalid theme sync` names each font the blog may not be allowed to serve, such as an Adobe Fonts family, and says the blog shows the next font in its list instead.

#### Search

- On a static site, the search page now tells readers in one sentence that search is not available. The steps to turn search on stay in `khalid build` and `khalid doctor`, for the owner.

#### Comments

- A reader's email is now optional on a comment, and each comment records whether a reader or the site's staff wrote it.

#### Newsletters and forms

- Form answers, newsletter sign-ups and alerts now share one way of sending mail, so a site with `[mail]` set up gets all three from one setting.

#### API, webhooks and SDK

- The Content API now refuses `include` and `fields` with an error that names the parameter, instead of accepting them and sending every field anyway.
- `khalid mcp` now offers a tool for every API operation a key may call. Tools are renamed to match the API: `list_posts`, `read_post`, `create_draft` and `edit_post` are now `browse_posts`, `read_admin_post`, `create_admin_post` and `edit_admin_post`, while `publish_post` and `lint_post` stay (KH-API-5).
- Failed webhook deliveries are retried to the webhook list as it stands at each round, so a webhook added while `khalid serve` runs is retried and one removed is left alone. A webhook list that cannot be read no longer stops `khalid serve` from starting (KH-API-3).

#### Accounts and sign-in

- `khalid staff owner` also shows the sign-in link on `localhost`, where `khalid serve` answers, so the owner can sign in before the site is online. `--listen` names another address, and `--json` adds `local_link`.

#### Speed and hosting

- `khalid serve` answers its first request sooner: it serves pages before the Content API and Admin API are ready, and a request in the first moments waits for the posts instead of getting an empty list (KH-API-1, KH-OPS-4).
- Saving a post costs the same on a blog of ten posts or ten thousand: `khalid serve` redoes only the posts whose files changed, and renders post bodies for the API only when they are asked for, so it does less work and holds less memory (KH-API-1, KH-OPS-4).
- The server's idle memory is back under budget: the setting `MIMALLOC_PURGE_DELAY=0` is now set in the deployment image, the systemd unit and `compose.yaml`, and the install docs name it for servers you start by hand.
- The release binary is about 6 MB smaller, under its 30 MB budget again, while pages for readers keep full speed, and an idle server holds about 8 MB less memory.
- The container image now starts with `serve --init`, so it makes the site on an empty disk at the address in `KHALID_ADDRESS`.
- An idle `khalid serve` on a 1,000-post site uses about 9.5 MB less memory, back under its 55 MB budget, because the APIs are set up when first used. `/api/openapi.json` answers in a third of a millisecond.
- `khalid serve` starts faster: it reads the content files once instead of twice, answers the home page before every post is rendered, and adds compressed copies in the background. On a 1,000-post site the first page comes about three times sooner. A request for another page in that moment waits for it instead of getting "not found" (KH-OPS-4).
- `khalid serve` holds far less memory: it keeps one compressed copy of each page instead of two, compresses about seven times faster with less memory at its peak, hands freed memory back to the system about two seconds after work ends, and no longer holds 2 MB huge pages. On a 1,000-post site, keeping one copy saves about 15 MB and reading posts once about 20 MB (KH-OPS-4).
- Saved edits show sooner under `khalid serve`: it hears a saved file the moment it is written instead of checking twice a second, and a saved post's page shows in about 20 ms on a 1,000-post site, with the home page and lists catching up a moment later. Where the system gives no file events, such as some network drives, it still checks every half second (KH-OPS-4).

### Fixed

#### Install and setup

- `khalid init` also makes `drafts/<language>/`, and its next step shows how to start a draft and publish it (KH-INS-2).
- The proxy rule `khalid init` prints for nginx, Caddy, Apache or Traefik with no `--blog` now points at `http://127.0.0.1:8080`, where `khalid serve` answers, instead of a made-up host. For a hosted stack the guessed `blog.` address is now flagged for you to change, and `--json` lists it under `guessed` (KH-INS-3).

#### Writing and content

- A `[[slug]]` link in a post is now a real link on the page, in `khalid serve` and in the feeds, instead of plain text. A broken one stops `khalid serve` from showing a saved edit and names the file and line (KH-CON-6).

#### Publishing

- `khalid publish` no longer says "Published" for a post it did not publish. A post in `content/` with no date counts as a draft, as the build already treats it, and publishing it adds the date. A post already live is refused with how to take it down. A refused `khalid build` now says to move a post to `drafts/<language>/` to keep it a draft (KH-LNT-1, KH-OPS-3).

#### Linter and audit

- The heading check no longer warns on every heading about kinds or differences. It now reads the lists and tables under each heading from the page, and only whole words count, so "always" no longer reads as "ways".
- The linter no longer reads the letters and signs of a maths formula as words, so formulas cannot trip the writing checks.
- `khalid lint` warns when a post in a topic cluster never links to the cluster's pillar post, and reads each link's `rel` as the `[seo.links]` policy writes it on the page (KH-LNT-2).
- `khalid lint`, the publish gate and the Admin API now check each post at the address its page is built at, following `routes.toml` and naming a post without `slug:` from its title. A post reached only by a `[[slug]]` link is no longer called an orphan, and a post the home page does not list gets `too-deep` (KH-LNT-2, KH-CON-9).
- The focus keyword and secondary keyword warnings now fire, because the linter reads `focus_keyword` and `secondary_keywords` as written. A form whose answers go nowhere and a draft with a `[[slug]]` link to no page are now reported in the Admin API and MCP lint results (KH-LNT-2, KH-LNT-4).
- A brand-new site now passes its own audit with no warning. The home page, lists, author pages, glossary index, search and sitemap page get a meta description in the page's language, and every footer links to the sitemap page.

#### SEO

- A post's old addresses now redirect to the address `routes.toml` gives the post, in `_redirects` and in `khalid serve`. Before, a collection routed as `/{year}/{slug}/` sent every old address to `/{slug}/`, a page that does not exist (KH-SEO-8).
- With `[seo.indexnow]` set, `khalid build` writes the IndexNow key file, `<key>.txt`, into the blog's folder and `khalid serve` answers it. Before, the file was missing, so search engines refused every IndexNow ping (KH-SEO-11).
- An author page now tells Google which person it is about, so it can show as a profile page, and a review of an app or product no longer asks for the app's price.

#### Themes and pages

- Blog pages are styled again. The stylesheet gets a name that changes with its content, so a new stylesheet reaches readers at once, and values in `theme/tokens.toml` become CSS custom properties; a mistake in that file stops the build with the line to fix (KH-WEB-6, KH-WEB-8).
- A post whose `translation` is `archive` or `sitemap` no longer clashes with the list pages of the same name.
- Theme sync again skips stylesheets meant for print, a dark colour scheme, or an alternate style, so the blog takes only the colours a visitor sees on screen.
- `khalid theme sync` no longer takes the grey of a code box, a table, a status message or a panel as the page background or text colour when the site leaves the page itself to the browser's defaults.
- `khalid theme sync` names only the fonts the blog's text uses, never an icon font or a font embedded in the site's CSS. For a font service other than Adobe Fonts it says Khalid cannot tell the licence, rather than claiming copying is forbidden.

#### API, webhooks and SDK

- `khalid serve` now sends `comment.added` to the webhooks that asked for it each time a reader leaves a comment, without the reader's email (KH-API-3).
- The Content API's `updated_at` for a post is now its `updated:` date when that is later than the publish date, instead of always the publish date.
- `khalid serve` no longer sends a second `.published` webhook, or a false `.deleted`, for a post saved while the server was still starting.
- The Admin API lists keys made in the same second in the order they were made, so the list no longer changes order between two reads.

#### Media

- A phone photo taken sideways now shows upright in every size Khalid makes of it.

#### Import and export

- An imported video or other frame becomes an embed block again, instead of showing as a line of code-like text on the page (KH-API-7).
- Imported code keeps its indentation and every blank line, where WordPress code blocks used to lose leading spaces after a blank line (KH-API-7).
- An imported link or image with an empty address keeps only its words, instead of becoming a link that leads nowhere.
- `khalid import --dry-run` lists each item whose fields its file cannot hold yet, by slug with the fields, instead of only giving a count.

#### Speed and hosting

- A post saved while `khalid serve` was starting could stay unseen until the next save. It now shows.

### Security

#### Install and setup

- Requests to Let's Encrypt and to a live WordPress site you import from go through the shared outbound client, with its timeouts and its guard against reaching private networks (KH-INS-4, KH-API-7).
- The Let's Encrypt account key and the certificate key in `data/https/` can be read by their owner only, and each file is written whole before it replaces the old one (KH-INS-4).

#### Writing and content

- Post passwords are never stored or sent: only a salted Argon2id hash (a one-way scramble of the password) is kept. The cookie that opens a protected post is signed with the site's key, works for one post for 30 days, and stops working once the password changes. Only ten wrong tries a minute are allowed per post, and the full page is never kept by a shared cache (KH-EDT-3).
- A post's head code is checked before it is kept: only `script`, `style`, `meta` and `link` are allowed, and text, body elements, `<base>`, `<meta http-equiv>`, event attributes such as `onload` and unclosed tags are refused with a message that names the problem. Only an owner, an admin or an `admin:all` key may change head code (KH-EDT-3, KH-API-2).

#### Editor and admin

- Unsplash and GIPHY keys come from the environment only and never print, and photo search runs through the blog so the keys never reach the browser.

#### Publishing

- Preview links are signed with a random key each site makes once and keeps in its database. A link opens only its own post, only until it expires, cannot be made or stretched without the key, and keeps working after a restart (KH-API-2).

#### SEO

- `KHALID_BING_API_KEY` holds your Bing Webmaster Tools API key. Khalid keeps it secret and never shows it in a log.
- The "not found" log holds up to 10,000 addresses, so a bot trying random addresses cannot fill the disk. Known addresses are still counted when it is full (KH-SEO-8).

#### Themes and pages

- A downloaded font file is checked by a parser that refuses a malformed file, and a file that is not a readable font is never used to tune the fallback font.
- The consent banner's form refuses a form posted from another site with 403, and a return address outside the blog goes to the blog's home. A static site that would ask for consent is refused at start, because a static host cannot keep a reader's choice (KH-SEO-12).

#### Comments

- One network address can send at most three comments in ten minutes and ten in a day. Khalid keeps only a salted hash of the address, made new each day, never the address itself.

#### Newsletters and forms

- The password or key for sending mail comes only from the environment, as `KHALID_SMTP_PASSWORD`, `KHALID_RESEND_API_KEY` or `KHALID_POSTMARK_TOKEN`, and is never printed. Khalid refuses to start when the one it needs is missing.
- Mail goes out through the shared outbound client with its timeouts, size limits and private-network guard, and the email text and key never reach a log. SMTP uses STARTTLS on port 587 by default or TLS from the first byte, checks the certificate, never falls back to plain text, and refuses unencrypted sending unless `private_host = true` says the server is on the site's own network.
- The reader cookie for gated content and the two-day pass link are signed with the site's key and hold no email address. Return addresses off the blog are ignored, so a sign-up link can never send a reader to another site.
- Forms quietly drop what spam robots send through a hidden trap, and one address that sends too many is asked to wait (KH-GRO-1).
- Newsletter confirmation and unsubscribe links use long random tokens that are stored only as hashes, and a confirmation link works once. Newsletter pages are never cached, shown inside other sites or passed on as a referrer, and a filled trap field sends nothing, which stops bots (KH-GRO-2).

#### API, webhooks and SDK

- API keys are stored in `data/site.db` only as a hash, never the key itself. A revoked key stops working at once and its identity is never reused (KH-API-2).
- The Admin API manages API keys, for a key with `admin:all`: `POST /api/admin/v1/keys/` makes a content or integration key with the scopes it names and shows it only this once, storing just its hash. `GET /api/admin/v1/keys/` lists working keys without their secrets, and `DELETE /api/admin/v1/keys/{id}/` revokes one at once, but never the key making the request (KH-API-2).
- Publishing through the Admin API needs the new `posts:publish` permission. The owner, admins and editors have it; writers and contributors do not, and get a clear refusal that tells them to ask an editor. Publishing, scheduling, taking a post down and editing a live post all need it, while writing a draft still needs only `posts:write`. A key made with only `posts:write` can no longer publish: make a new one with `khalid keys create --scope posts:write --scope posts:publish` (KH-EDT-4, KH-EDT-7).
- Each new webhook gets its own random secret, and every delivery is signed with it. Webhooks that are switched off get nothing (KH-API-3).
- The agent key for `khalid mcp` is read from `KHALID_KEY` and never appears in a log or debug line (KH-API-5).
- Rate limits on the Content API and the Admin API: 600 Admin API requests a minute per key, 6,000 Content API requests a minute per content key, and 10 wrong keys a minute per address. Past a limit the API answers 429 with `Retry-After` and says how long to wait.

#### Accounts and sign-in

- Staff passwords need at least 12 characters and are stored only as an Argon2id hash. A wrong email and a wrong password get the same answer and take as long, and asking for a sign-in link looks the same whether or not the email has an account, so nobody can tell which emails have accounts (KH-EDT-7).
- After five failed sign-ins in 15 minutes for one email or from one address, Khalid refuses more tries for a while and says how many minutes to wait. At most five sign-in links go to one email in 15 minutes.
- The session cookie cannot be read by scripts, is sent only over HTTPS and only to this site, and ends after 14 days, or 12 hours unused, or when the account is removed. Only a hash of it is stored, so a copy of the database signs nobody in (KH-EDT-7).
- The Admin API accepts a signed-in session only with its CSRF token, a code that proves the request came from the admin itself, in the `x-khalid-csrf` header, so another site cannot act for a signed-in person (KH-API-2).
- One permission table sets what each of the five roles may do and whose accounts each may manage, so a staff token and a signed-in person with the same role may do the same. Nobody can invite or create a second owner (KH-EDT-7).
- Passkeys are now checked by `webauthn-rs`, a widely used library, instead of Khalid's own code, and Khalid no longer needs OpenSSL.

#### Import and export

- Imported Ghost text and markdown and HTML cards are cleaned like the rest of the body, so they cannot carry a script or an unsafe link. Every address in an imported card is checked first, and Ghost code injection is never carried over (KH-API-7).

#### Speed and hosting

- The HTTPS server closes a connection whose secure handshake stalls for ten seconds, so a slow or hostile client cannot hold up other visitors (KH-INS-4).
- `khalid serve` sends a Content Security Policy with a fresh nonce on every response. The nonce is a one-time code that tells the browser which scripts on the page are the blog's own.
