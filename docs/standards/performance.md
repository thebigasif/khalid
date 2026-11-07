# Performance budgets

Fast is a feature. These numbers are gates, not goals. CI measures them on every pull request against `perf/baseline.json`, and a regression beyond the allowed drift fails the build (BL-PERF-1). Updating the baseline is a deliberate change in its own pull request with the reason.

## Budgets

| Measure | Budget | Drift allowed | How CI measures it |
|---|---|---|---|
| Release binary, stripped, Linux x86_64 | under 30 MB | 2% | `ls -l` after `cargo build --release` |
| Cold start to first byte | under 100 ms | 10% | `hyperfine` on `khalid serve` with a 1,000-post fixture, first request |
| Idle memory, one site, 1,000 posts | under 50 MB RSS | 10% | resident set size of `khalid serve`, read after it settles, then five readings one second apart of which CI takes the lowest (CTO answers 2026-10-27, section 2) |
| Post page render, in process | p50 under 1 ms, p99 under 5 ms | 10% | `divan` benchmark over the fixture |
| Cached post page, 2 vCPU | at least 10,000 requests per second | 10% | `oha` for 30 seconds |
| Full rebuild, 10,000 posts | under 30 s | 10% | timed `khalid build` |
| Incremental rebuild, one post | under 50 ms | 10% | timed file change to updated page |
| Lint one post | under 20 ms | 10% | `divan` |
| Full site audit, 1,000 posts | under 60 s | 10% | timed `khalid audit` |
| Lighthouse, default post page | 100 in all four categories | none | Lighthouse CI against the demo site |
| Largest Contentful Paint, demo site on a 5 dollar VPS class runner | under 1 s | none | Lighthouse CI |
| Cumulative Layout Shift | 0 | none | Lighthouse CI |
| Public CSS, compressed | under 20 KB | 5% | size of the built stylesheet |
| Pictures on a template's demo home page, 390 px at 3x, without the hero | under 620 KB (Aperture 810 KB) | none | `scripts/page-weight.sh`, see "Page weight" |
| Pictures on a template's demo home page, 390 px at 1x, without the hero | under 140 KB | none | `scripts/page-weight.sh`, see "Page weight" |
| Pictures on a template's demo home page, 1440 px | under 1,250 KB (Aperture 2,100 KB) | none | `scripts/page-weight.sh`, see "Page weight" |
| Web fonts on a template's demo home page and one post, 390 and 1440 px | under 60 KB | none | `scripts/page-weight.sh`, see "Fonts" |
| JavaScript required to read a post | 0 bytes | none | scripts-off test |
| Admin initial JavaScript, compressed | under 350 KB | 5% | Vite build report |
| Test suite, CI | under 4 min | none | job time |
| Release build, CI | under 10 min with cache | none | job time |

## How we stay inside them

- **Compute at publish, not at request** (BL-PERF-2). Rendered HTML, schema, meta, link graph and search index are produced when content changes and cached by content hash. A request copies bytes.
- **Images at upload** (BL-PERF-5). Sizes and formats are generated when the file arrives, in `spawn_blocking`.
- **Streaming and bytes.** Responses are `Bytes`. Templates render into a reused buffer. Shared immutable data is `Arc<str>` or `Arc<[u8]>`.
- **SQLite tuned.** WAL, `synchronous=NORMAL`, prepared statements, one writer, a read pool, indexes checked with `EXPLAIN QUERY PLAN` in tests for every list query.
- **Precompressed assets.** Brotli and gzip at build, served with immutable cache headers and hashed names.
- **No surprises in the hot path.** No network, no disk except cache miss, no locks across awaits, no allocation storms. The render benchmark counts allocations and fails on a 10% rise.
- **mimalloc** in the binary, with its default settings. `khalid serve` lets it hand freed memory back once its work is done. See "Allocator settings" below.
- **Profiles before guesses.** `cargo flamegraph` or `samply` before any optimisation. The pull request shows the before and after numbers.

## How the idle memory is read

The 55 MB budget and the meaning in the CTO answers of 2026-10-27, section 2, stand. What changed on 2026-10-28 is only how the reading is taken, because one binary measured 51.43 to 81.49 MB across eight runs: mimalloc returns a freed page one second after it is freed, and only when a thread later frees a page or ends, so whether tokio's idle blocking threads had ended before the reading at 15 s decided the number.

`khalid-perf serve` now reads the server's thread count and resident memory each second after the first answer, with no traffic in between. The server has settled when the trace is at least 15 s long (the wait still outlasts tokio's blocking threads, which end 10 s after their last task) and the last 3 readings show the thread count unchanged and the resident reading not falling by more than half a megabyte. A platform whose `/proc` does not say the thread count settles on its memory alone. Nothing waits for ever: after 60 s the readings are taken anyway, so a wedged server cannot hang the measure.

Once settled, five readings are taken one second apart and the lowest is the measure, so a page the allocator hands back between two readings cannot raise it. The budget number was never changed.

## Allocator settings

One setting is on: `MIMALLOC_PURGE_DELAY=0`, set in the deployment image and in the server `khalid-perf serve` measures. mimalloc holds a page it frees in its arena for a later reuse, with a reset delay before giving it back; an idle server never touches its arenas again, which is why a heap of 16.5 to 26.8 MB live read 54 to 67 MB resident. With the setting, a freed page returns to the OS at once. A delay of a hundred milliseconds was measured and rejected: the cold start did not recover (120.08 ms) and the idle reading climbed back over the budget (60.48 MB), because the pages a delayed purge would return were never touched again by an idle server. `MIMALLOC_ABANDONED_PAGE_PURGE` was measured and rejected: it made both budgets worse (idle 58.44, cold start 117.23). The measurement that justifies the setting that is on is the eight CI runs in the pull request that added it.

The setting has a cost: every page freed is handed back at once and faulted in again when it is reused, so work that allocates and frees a lot costs more. Before it answers, `khalid serve` therefore does only what the first answer needs. Measured on 2026-10-31 with timing marks in a release build on the 1,000-post site, on a shared server, the start before the first answer was: walking the content folder 12 to 18 ms, reading the posts 22 to 34 ms, checking their bodies 8 to 12 ms, the runtime and database 5 ms, the home pages 6 to 17 ms. Three steps were moved out of that path:

- **The stylesheet's compressed copies** (11 ms). Brotli at its best quality with a 4 MB window sets up tens of megabytes of tables, and the server threw both copies away: it compresses its pages after it answers. It now builds the stylesheet's text and hashed name only.
- **The Content API and Admin API router** (8 to 9 ms, most of it the OpenAPI document). Since 2026-10-31 it is built on a blocking thread when the first request under `/api/` arrives, and that request waits for it (see "What an idle server holds" below).
- **The full render**, which used to start before the routers were built and competed with them. It starts once the routes are ready.
- **The body check** (8 to 12 ms) runs on a thread of its own while the runtime, the database and the home pages are set up, and the server waits for it before it listens, so a broken wikilink still stops it before it says where it answers.

The first request is answered within a fraction of a millisecond of the routes being ready. What is left is reading the content, which the first page needs.

## What an idle server holds

Measured on 2026-10-31 on the final bundle (integration 19), release build, 1,000-post site, with a temporary `dhat` heap profile 45 s after start and `khalid-perf serve`, alternating runs on a shared server (load average 7 to 14).

The live heap at idle was 22.8 MB. The largest holders were the API router's OpenAPI document (7.2 MB, held more than once, because the route that served it kept the whole struct and the router copies a route as it is put together), each post's head for the Content API (4.9 MB, G55) and the pages' Brotli copies (3.6 MB). Two changes (ADR "The APIs are built on their first request"):

- The APIs are built on the first request under `/api/`, not at start. That request waits about 8.6 ms for the build.
- The OpenAPI document is served as JSON bytes made once, so a copy of the route copies a pointer.

| Measure | Before | After |
|---|---|---|
| Idle memory, `khalid-perf serve` | 60.89, 60.38, 59.86, 61.40 MB | 50.63, 51.37, 51.51, 50.79 MB |
| Of it, anonymous / code and files | 38.54 / 22.35 MB | 30.84 / 19.79 MB |
| Cold start to first byte | 59.56, 57.14, 96.05, 65.65 ms | 96.84, 60.19, 62.62, 63.15 ms |
| Live heap at idle (`dhat`) | 22.8 MB | 15.5 MB |
| First API request | 0.23 ms | 8.6 ms (builds the APIs) |
| `GET /api/openapi.json` (257,432 bytes) | 7.5 to 8.7 ms | 0.33 ms |
| Idle memory after the APIs were asked | 61.76, 62.73 MB | 54.18, 54.36 MB |

The cold start moved within the noise between runs. Left as they are, with the reason:

- **Post heads (4.9 MB).** Every site's Content API hands them out. Kept as JSON text instead of tags they come to about the same size, since the JSON-LD graph is most of it, and making them on request would render at request time (BL-PERF-2). Update 2026-11-02, integration 29: the Content API's store now keeps each head as compressed JSON, packed while the store is built off the render path and decoded when a post is asked for. Idle memory on the 1,000-post site went from 52.89 and 52.82 MB to 49.40 and 49.38 MB, with first byte and rebuild unchanged. The heads were first packed with Brotli; they now use deflate, already in the binary for gzip, which keeps 128 KB of Brotli encoder out of the binary (see "Binary size") and measured 48.64 and 49.88 MB idle.
- **SQLite's page cache** (4.5 MB in the C library's heap), the database track's setting.
- **The Content API store** keeps each post twice, once by file and once in the store (about 1 MB).

## Allocator settings history

The CTO answers of 2026-10-27, section 2, allow tuning the allocator to hand memory back, as long as the setting is recorded here with the measurement behind it. Measured on 2026-10-27 with `khalid-perf serve`, release build, on the generated 1,000-post site, idle memory as defined in the table above. The server was shared and busy (load average 65 to 85 on 16 cores), so each setting ran twice where it mattered.

| Allocator | Idle memory, 1,000 posts | Idle memory, 10 posts |
|---|---|---|
| mimalloc 0.1.52, default settings (v3) | 60.3 MB, 63.6 MB | 35.0 MB |
| mimalloc with `no_thp` | 62.0 MB, 60.7 MB | |
| mimalloc with `v2` | 112.3 MB, 115.8 MB | |
| The system allocator (glibc), for reference | 64.5 MB | 23.4 MB |

What this shows:

- **No setting is changed.** `no_thp` makes no difference, because this kernel's huge pages are `madvise` only and mimalloc does not ask for them. `v2` holds almost twice as much. mimalloc's purge delay and decommit options can only be set through environment variables or `unsafe` calls, which the workspace forbids, so they were not measured.
- **The allocator is not what holds the memory on a 1,000-post site.** The system allocator holds as much as mimalloc there. The rendered pages are 6.9 MB plain, with their Brotli copies on top. The server's fixed cost is about 35 MB, measured on a 10-post site. The rest grows with the posts and has not been broken down yet; the Content API's store and its index refresh, the subject of the CTO answers' section 3, are the largest part not yet measured.
- **mimalloc costs about 12 MB more than the system allocator on a small site**, from its per-thread heaps. That is worth measuring again once the 1,000-post site is inside its budget.

Measured again on 2026-10-27 (perf track, task 1.21), with the server answering before its full render and each page compressed with a window of at least 128 KB, on a quiet server (load average 3 to 9), two runs each, idle memory as defined above:

| Setting | Idle memory, 1,000 posts |
|---|---|
| mimalloc, default settings | 57.85 MB, 59.47 MB |
| with `no_thp` | 59.06 MB, 60.53 MB |
| tokio blocking threads kept 1 s instead of 10 s | 58.22 MB, 59.80 MB |
| the full render on a thread of its own, which ends after it | 58.08 MB, 59.48 MB |
| two tokio worker threads, on top of one copy per page | 63.98 MB, 70.94 MB, against 65.82 MB |

None of them changes idle memory beyond the noise between runs, so still no setting is changed. A `dhat` heap profile of the running server, 14 s after start, holds 26.8 MB live, about half of it each page's plain bytes beside its Brotli copy; `khalid-server` now keeps one copy per page. Brotli's own buffers are the other large cost: below a 128 KB window the `brotli` crate set up a 32 MB table for each page.

### Handing freed memory back (task 1.21, 2026-10-27)

Why idle memory moved by 30 MB between runs of one binary: mimalloc returns a freed page to the system one second after it is freed, and only when a thread later frees a page or ends. A server with no visitors does neither. A trace of resident memory every half second showed it at 70 to 80 MB from the end of the start's render until tokio's idle blocking threads ended, 10 s after their last task, and then falling to 51 to 62 MB. Whether that fall came before the reading at 15 s decided the number. A `dhat` profile at 14 s held 16.5 MB of live data.

`khalid serve` now does two things (ADR "khalid serve hands freed memory back once its work is done"):

- Its blocking threads end 1 s after their last task instead of 10 s.
- 2 s after its pages are replaced or the Content API's feed finishes a catalog, with nothing else in between, it ends one short-lived thread, which makes mimalloc return everything freed by then.

Measured with `khalid-perf serve`, release build, generated 1,000-post site, the three settings run in turn three times on a busy server (load average about 65 on 16 cores):

| Setting | Idle memory, 1,000 posts |
|---|---|
| The thread ended after work, alone | 64.35 MB, 58.61 MB, 63.91 MB |
| With blocking threads kept 1 s | 56.07 MB, 52.48 MB, 51.36 MB |
| With that, and the content read on a thread that ends | 56.34 MB, 52.00 MB, 59.45 MB |

Main measured 51.43 to 81.49 MB in eight runs. Reading the content on its own thread made no difference and was dropped. Neither did mimalloc's `no_thp` here (55.0, 57.6 and 57.2 MB, against 49.8, 53.2 and 68.5 MB without it), although one run without it had 12 MB in huge pages at idle.

CI's runner tells another story. With the hand-back, its trace (run 40470523235) fell to 60.22 MB six seconds after the first answer, then rose to 71.82 MB at eleven seconds and stayed there, with 24.00 MB of the 51.07 MB of anonymous memory in 2 MB huge pages. On the 10,000-post site, 172 MB of 270.50 MB was. mimalloc asks Linux for huge pages over its whole arena, and the kernel's background collapser, which looks every ten seconds, then fills whole 2 MB pages around the little memory still in use there. So the binary now builds mimalloc with `no_thp`, which stops it asking. That alone took CI to 63.52 MB with 14.00 MB still in huge pages (run 40471366129), because CI's runners are set to `always`: Linux hands out huge pages there without being asked. So `khalid serve` also switches huge pages off for its own process before it reads the site (`prctl(PR_SET_THP_DISABLE)`, through `rustix`, which wraps it safely). `khalid-perf serve` prints the system's huge-page mode beside the trace.

`khalid-perf serve` prints, beside its measures and so in CI's log, the resident memory each second of the 15-second wait and the split at its end: anonymous memory, the part of it in huge pages, and code and mapped files.

What a settled server holds, from `/proc/<pid>/smaps` at 51 MB: 27 MB in mimalloc's arena, 16.4 MB of the binary's code, 5.3 MB in the C allocator's heap, where SQLite keeps its page caches, and 1.4 MB of the C library.

## Binary size

The budget is 31,457,280 bytes (30 MB), checked by the "Binary size, budget under 30 MB" job in CI after `cargo build --release -p khalid`. On 2026-10-31 integration branch 15 built 33,249,512 bytes and integration branch 16 built 31,639,784. Both merged together built 33,755,112 bytes.

What the release profile already did: fat LTO, one codegen unit, symbols stripped, `panic = "abort"`, and opt-level `"s"` for a few cold crates (the share image stack, the font reader, the import command's HTML reader, and on branch 15 the API crate). The panic behaviour is unchanged.

Where the bytes were, from an unstripped build of both branches merged: 25.8 MB of machine code, 3.5 MB of read-only data, 1.5 MB of unwind tables and 1.4 MB of relocations. The biggest code was the API's router builders (`khalid_api::api::assemble` alone was 650 KB), the `khalid` crate's commands (2.3 MB), image resizing (0.6 MB), and 2.4 MB of C: SQLite, libwebp and mimalloc. Nearly all of it was built at opt-level 3, which trades size for speed even where speed does not matter.

What changed (task/p1-binary-size), each measured on both integration branches merged:

| Profile | Bytes | Saving |
|---|---|---|
| Before | 33,755,112 | |
| Everything at `"s"` | 25,528,680 | 8,226,432 |
| `"s"` by default, the reader's path at 3 (kept) | 27,490,792 | 6,264,320 |
| As kept, plus the `khalid` crate at 3 | 35,331,752 | none, 1,576,640 more |
| As kept, plus SQLite, sqlx, toml and five more Khalid crates at 3 | 28,693,480 | 5,061,632 |

The kept profile builds everything at `"s"` except the crates on a reader's path and on the build's path, which stay at 3: tokio, hyper, axum, http, tower and the allocator; `khalid-core`, `khalid-content`, `khalid-render`, `khalid-static`, `khalid-lint` and `khalid-search`; and the parsers and compressors they run for every post (`comrak`, `minijinja`, `syntect`, the regex crates, `serde` and `serde_json`, the YAML front matter reader, and the brotli and gzip compressors). `opt-level` also reaches the C that a crate's build script compiles, so SQLite and libwebp are now built for size and mimalloc for speed.

The `khalid` crate must stay at `"s"`. With fat LTO the whole program is optimised again at the binary crate's level, so opt-level 3 there inlines the size-built crates back into it, and the binary grew past where it started. Putting the start-up path's crates back at 3 (the last row) cost 1.2 MB and did not move the first byte.

Speed and memory, `scripts/perf.sh` on both branches merged, two runs each, on this server with other builds running:

| Measure | Before | After |
|---|---|---|
| Cold start to first byte, 1,000 posts | 72.94 ms, 75.06 ms | 77.52 ms, 77.44 ms |
| Idle memory, 1,000 posts | 75.12 MB, 72.34 MB | 66.36 MB, 65.94 MB |
| `khalid --help` | 1.78 ms, 1.85 ms | 1.81 ms, 2.29 ms |
| Post page render p50 | 0.0282 ms, 0.0285 ms | 0.0277 ms, 0.0271 ms |
| Full rebuild, 10,000 posts | 13.26 s, 11.92 s | 14.77 s, 12.22 s |
| Incremental rebuild | 13.24 ms, 13.44 ms | 13.81 ms, 14.82 ms |

Idle memory falls by about 8 MB because less code is mapped in: a settled server holds the binary's code in its resident memory (see above). The first byte is about 3.5 ms slower, well inside its 100 ms budget. Render and rebuild differences are within the noise between runs.

Looked at and left alone:

- **Duplicate crate versions.** html5ever 0.38 (through htmd) beside 0.40 (through ammonia and scraper), png 0.17 (through tiny-skia) beside 0.18, and argon2 0.5 (through webauthn-rs) beside 0.6. Each older copy comes from a crate whose newest release still needs it, so removing one means dropping a feature or downgrading a security crate such as ammonia, for at most about 200 KB.
- **Dependency features.** The direct dependencies already turn off default features and name only what the code uses.
- **A non-PIE binary** would drop 1.4 MB of relocations but give up address space layout randomisation for the binary. No.

### Integration 29: AVIF output

On 2026-11-02 integration branch 29 (PR #924) built 31,655,096 bytes in CI at commit 3e389f0a, over the budget. The AVIF encoder (`ravif` with `rav1e`, in `khalid-media`) had added about 1.55 MB, and packing the Content API's post heads with Brotli about 154 KB more. AVIF output is a shipped feature, so it stays. Each step below is `cargo build --release -p khalid` on this server, which builds about 25 KB less than CI because `admin/dist` is empty here:

| Change | Bytes | Saving |
|---|---|---|
| Before (3e389f0a) | 31,630,520 | |
| Post heads packed with deflate, `brotli` out of `khalid-api` (kept) | 31,502,776 | 127,744 |
| Plus `rav1e`, `ravif`, `av-scenechange`, `av1-grain` and `v_frame` at `"z"` (kept) | 31,160,824 | 341,952 more |
| Plus `fast_image_resize`, `image`, `zune-jpeg` and `image-webp` at `"z"` | 31,209,720 | none, 48,896 more |
| Plus `fast_image_resize` alone at `"z"` | 31,159,800 | 1,024, not worth it |
| Plus `khalid-api` at `"z"` instead of `"s"` (kept) | 30,746,872 | 413,952 more |

All kept: 30,746,872 bytes, 883,648 less than before and 710,408 under the budget.

The AVIF encoder runs when an image is saved, never on a reader's path. At `"z"` the test that makes every width of a 900 by 600 photo in AVIF, WebP and JPEG (`khalid-media`'s `a_site_that_asks_for_avif_gets_every_width_in_avif_first`, release build) went from 0.52 to 0.56 s to 0.66 to 0.71 s, about 28% slower. The API crate answers agents, scripts and the admin, and its answers carry ETags. Idle memory on the 1,000-post site, `khalid-perf serve` as `scripts/perf.sh` runs it, was 48.64 and 49.88 MB with every kept change, against 49.40 and 49.38 MB before.
- **Unwind tables** (1.5 MB) can only be dropped with a `rustflags` setting for every build, and backtraces need them.
- **The embedded admin** is only in a binary built after `npm run build`, which CI's size step does not run.

### Integration 30: Yoast analysis and SEO screens

On 2026-11-02 integration branch 30 merged with main built 31,525,496 bytes on this server with `admin/dist` empty, about 780 KB more than main. The growth is ordinary code: the analysis in `khalid-lint`, the SEO endpoints in `khalid-api`, the commands in `khalid` and the head tags in `khalid-seo`. Each step below is `cargo build --release -p khalid` on this server:

| Change | Bytes | Saving |
|---|---|---|
| Before | 31,525,496 | |
| Import, MCP, mail, sign-in, Search Console, webhooks, alerts and HTTPS crates at `"z"` (kept) | 31,422,840 | 102,656 |
| Plus audit, newsletter and outbound crates, `lettre`, `webauthn-rs`, `htmd`, `utoipa`, `instant-acme`, `rcgen`, `x509-parser`, `der-parser` and `reqwest` at `"z"` (kept) | 31,280,568 | 142,272 more |
| Plus `khalid-db`, analytics, comments and forms crates, `sqlx-core` and `sqlx-sqlite` at `"z"` (kept) | 31,233,272 | 47,296 more |
| Plus `khalid-lint` at `"s"` instead of 3 | 31,129,080 | 104,192 more, not kept |
| Plus `khalid-seo` and the share image stack (`khalid-share-image`, `rustybuzz`, `tiny-skia`, `ttf-parser`) at `"z"`, linter at 3 (kept) | 31,032,440 | 200,832 more |

All kept: 31,032,440 bytes, 493,056 less than before. The linter stays at 3: at `"s"` lint one post took 14.89 and 14.77 ms against 8.87 and 9.01 ms at 3, back to back. With every kept change `scripts/perf.sh` measured idle memory 49.39 MB, lint one post 9.04 ms, a full 10,000-post rebuild 14.84 s and the first byte 64.36 ms, every budget held.

## Page weight

Pictures are most of what a reader downloads. Every picture is offered in several widths (`srcset`), and its `sizes` tells the browser how wide the page draws it, so the browser fetches the smallest file that fills the slot. A `sizes` wider than the slot fetches a bigger file than it draws. One narrower makes the picture blurry.

### How a template says its widths

- The `picture` macro takes `sizes` as an optional second argument: `picture(post.image, "(min-width: 64rem) 22rem, 90vw")`. Called with the image alone, as before, the image keeps its own `sizes`: the text column for a post's lead image (`(min-width: 48rem) 48rem, 100vw`), and on a card a small card width, `(min-width: 64rem) 24rem, (min-width: 40rem) 50vw, 100vw` (`CARD` in `crates/khalid/src/post_photos.rs`). A slot drawn wider than a card, such as a lead story across the page, must pass its width.
- `cards`, `featured_slot` and the section parts that draw pictures (`picture_grid`, `poster_grid`, `cover_shelf`, `session_cards`, `spread`, `video_grid`) take `sizes=none` last and hand it on.
- Each pack keeps its widths in one file, `crates/khalid-theme/packs/<pack>/picture_sizes.html`, as the macro `slot_sizes(place, shape)`. The shared templates ask it for their places (`lead`, `related`, `featured`, `home`, `panel`, `list`, `author`, `photo`, listed in `crates/khalid-render/templates/picture_sizes.html`), and the pack's own templates add their own places, such as `lead_story` or `shelf`. The shared default holds Classic's widths.
- The widths were measured, not guessed: each pack's demo was opened in Chromium at every width from 320 to 2560 px (8 px apart up to 1120 px, then 16 px), the drawn width of every picture recorded, and each slot given the smallest `sizes` that is never narrower than what is drawn and at most a quarter wider. Measure again the same way when a stylesheet moves a picture.
- When the same post shows in two slots of one page, such as Channel's playlist shelves and its latest videos, the smaller slot says the larger one's width, so the browser fetches each picture once and not twice at two widths.

### The budget

`scripts/page-weight.sh` (with `scripts/demo-check/weight.mjs`) builds each template's demo as it is published, its ready demo publication in its sample brand with its pictures in AVIF too, and opens the home page in Chromium three times: at 390 px with three device pixels per CSS pixel, as most phones sold now have, at 390 px with one, and at 1440 px with one. It scrolls to the bottom so every lazy picture loads, and adds up the bytes of every picture the page downloads. CI runs it in the demo job after the layout check, on the sites the layout check built.

| Measure | Budget |
|---|---|
| 390 px at 3x, every picture but the hero (the first picture, fetched with high priority) | under 620 KB |
| 390 px at 3x, Aperture | under 810 KB |
| 390 px at 1x, every picture but the hero | under 140 KB |
| 1440 px, every picture | under 1,250 KB |
| 1440 px, Aperture, whose design is five photographs across a 1,296 px frame | under 2,100 KB |
| Fonts, home page and one post, at 390 px and at 1440 px | under 60 KB |

The desktop budgets were set on 2026-11-07 with about 10% headroom over the heaviest template after the change below, Atlas at 1,136 KB. The phone budgets were set again the same day, once the demos offered AVIF (see "Pictures on high-density phones"), with about 10% headroom: at 3x over Itinerary at 561 KB and Aperture at 734 KB, and at 1x over Aperture and Atlas at 125 KB. The 1x budget was 270 KB before; the 3x measure is new. A template over a budget almost always tells the browser a slot is wider than it is. The failure names the heaviest pictures and what to fix. A budget is never raised to make the check pass; a new budget is a deliberate change in its own pull request with the reason.

### Measured on 2026-11-07

The 31 template demos together, before and after each slot said its own width, in KB of pictures:

| Width | Before | After |
|---|---|---|
| 390 px, all pictures | 4,317 | 3,685 |
| 390 px, without the hero | 3,698 | 3,096 |
| 1440 px, all pictures | 13,136 | 7,901 |

Per template numbers are in the pull request that added the check (perf/picture-sizes). Keystone's desktop page grew from 558 to 601 KB and Atlas's from 1,106 to 1,136 KB, because their lead stories are drawn wider than the old 48rem said and were blurry; they now fetch a file as wide as they are drawn.

### Pictures on high-density phones (2026-11-07)

Most phones draw three device pixels per CSS pixel, so a card 326 px wide needs a file 978 px wide to look sharp, nine times the pixels of the 1x phone the budget first measured. Measured in Chromium at 390 px and 3x, the 31 demo home pages downloaded 16,258 KB of pictures, Aperture alone 1,507 KB. Three changes (perf/phone-pictures):

- **A width for 3x phone cards.** With `sizes` from perf/picture-sizes, Chromium already fetched a file close to slot width times 3: at most 1.31 times what it drew, and that only where a slot's `sizes` rounds up. But a card 344 to 384 px wide needs 1,032 to 1,152 pixels, just over the 1,024 px file, so it fetched the 1,280 px one. `khalid-media` now also makes 1,152 px. The widths are 320, 480, 640, 800, 1,024, 1,152, 1,280, 1,600 and 2,048; for every slot from 320 to 420 px at 3x the file fetched is at most an eighth wider than needed (the `plan` test checks it). 960 was not added: between 800 and 1,024 it would save at most 6% of a file.
- **AVIF for the demos.** Every template demo, as published and as the checks build it, sets `avif = true` (`scripts/demo-media.sh`). Each picture is offered in AVIF, then WebP, then the original's format. AVIF stays off for other sites, because of its cost (below).
- **AVIF quality 55 instead of 60.** WebP stays at 78.

How the quality was chosen. Six demo photos (city streets, a harbour cliff, a gallery stair, a film still, seed potatoes) were encoded at 1,024 and 1,152 px and compared with the resized original, by SSIM on lightness in Chromium's own decoder and side by side at full size and at 2x zoom:

| Encoding, 1,152 px, six photos | Bytes | SSIM, lowest to highest | Encode, one core |
|---|---|---|---|
| WebP 78 (unchanged) | 535,540 | 0.941 to 0.974 | 0.05 to 0.08 s |
| AVIF 60, speed 8 (before) | 285,065 | 0.878 to 0.959 | 1.75 to 2.60 s |
| AVIF 55, speed 8 (now) | 251,790 | 0.862 to 0.956 | 1.08 to 1.71 s |
| AVIF 55, speed 9 | 268,895 | 0.861 to 0.953 | 0.54 to 0.83 s |
| AVIF 50, speed 8, at 1,024 px | | 0.848 to 0.956 | |

AVIF scores lower on SSIM than WebP at the same look, because it smooths fine grain instead of blocking it. At full size AVIF 55 and 60 could not be told apart; at 50 grass and the skin of the potatoes lost texture, so 50 was not taken. At 3x a phone shows each file pixel a third as wide, which hides the difference further. Speed 9 halves the time but makes files 5 to 8% larger at the same quality.

What the encoding costs. The Atlas demo has 33 photos, most 1,600 px wide. `khalid build` from an empty cache, on the build server (16 cores, shared, load 20 to 40):

| Atlas demo build | Wall | CPU, all cores |
|---|---|---|
| WebP only | 6 to 14 s | 16 to 25 s |
| With AVIF 60, eight widths (before) | 60 to 113 s | 665 s |
| With AVIF 55, nine widths (now) | 63 to 139 s | 574 to 578 s |

About 17 s of one core per photo. A site with 300 photos on a two-core server would wait about 45 minutes on its first build, so AVIF is not switched on for every site. Each file is named by a hash of its photo and kept, so later builds read it from the cache. For the demos, `scripts/demo-media.sh` links each build's files into one cache shared by all demo builds, `~/.cache/khalid/demo-media`, and CI keeps it between runs (key: the encoder's source and the demos' `photos.lock` files). Building every template demo from an empty cache cost 8,298 s of CPU (815 s wall on the build server); on a four-core CI runner that is about 35 minutes, paid when a photo or the encoder changes. The demo job's time limit was raised from 75 to 120 minutes for such a run. One encoder thread per file (`with_num_threads(Some(1))`) was measured and left out: 23% less CPU, but twice the wall time on 16 cores.

The 31 template demo home pages, in KB of pictures, before and after (the "before" is perf/picture-sizes):

| Measure | Before | After |
|---|---|---|
| 390 px at 3x, all pictures | 16,258 | 7,766 |
| 390 px at 3x, without the hero | 13,415 | 6,422 |
| 390 px at 1x, all pictures | 3,685 | 2,027 |
| 390 px at 1x, without the hero | 3,096 | 1,709 |
| 1440 px, all pictures | 7,901 | 4,364 |

At 3x, 23 of the 31 home pages download under 400 KB of pictures in all, against 14 before. The eight over it are the photo-led designs: Aperture 827 KB, Itinerary 645, Atlas 561, Compass 470, Lattice 443, Plinth 413, Beacon 404 and Keystone 403. Per template numbers are in the pull request.

### Fonts

The same check counts every web font file the page downloads, on each demo's home page and on one post (the first post its demo publication's `check-pages.txt` names), at 390 px and at 1440 px. A browser fetches a face only when some text on the page is set in it, so a page with no italic does not pay for the italic file.

Each sample brand in `fixtures/showcase-brands/` carries its fonts as WOFF2 files. Google Fonts serves a "Latin" file with about 330 characters and, for some families, hinting for old Windows screens. `scripts/font-subset.py` cuts each file to what its demo shows:

- **Letters.** Basic Latin, the no-break space and a short list of punctuation are always kept. Any other letter is kept only when the brand's built demo uses it, in its pages or in its stylesheet's generated content, such as the German umlauts and ß of the German pages, or the arrows the stylesheet draws.
- **Features.** The OpenType features a browser applies by itself to Latin text (kerning, ligatures, contextual forms, marks), and only the numeral and small-capital features the pack's stylesheet asks for with `font-variant`.
- **Hinting kept.** Hinting fits letters to the pixel grid, and Chromium on Linux and Windows follows it when it measures text: a first cut without it moved line breaks in Compass and Lattice enough to fail the layout check. So every file keeps its hinting, except PT Serif (Ledger), whose home page would weigh 82 KB with it, Alegreya Sans (Chapter), which leaves Chapter room under the budget, and Petrona (Dialogue), whose three faces would weigh 67 KB with it. These demos pass the layout check without it. Outlines, metrics and names stay as they were, so the page looks the same and the fallback metrics in `tokens.toml` still match (the cut leaves `hhea` and `OS/2` untouched).
- **Ranges.** Each file's `range` in `tokens.toml`, its `unicode-range`, becomes exactly the letters it carries, so a letter it lacks is drawn by the next font in the stack. Every `@font-face` rule says `font-display: swap`.
- **Licences.** Every font stays under the OFL, with its licence beside it, and its copyright and licence names inside it. The OFL counts a cut file as a Modified Version, which may not carry a Reserved Font Name (OFL FAQ 2.6). So PT Serif is cut as "Ledger Serif", IBM Plex Sans as "Workbench Sans", Lora as "Showcase Serif" (Masthead, Quill) and Playfair Display as "Showcase Display" (Glossy, Masthead): the same design under a new name, in the file and in `tokens.toml`. Source Sans 3 (Keystone) reserves "Source" and is under the budget uncut, so it is left as Google serves it.
- **Variable files.** Brands that use a variable file keep it. After the cut, a variable file is about as large as the two static weights it would replace: Figtree 400 to 700 as one variable file is 12.4 KB (Vital), and its 400 and 700 static files are 12.8 KB together (Syllabus). A swap would save under 4%, so the static files stay where a brand has them.

Each page also preloads its body font, the one file every page uses: the upright face of the first family in the body stack, at weight 400 or in a range that holds it. The head says `<link rel="preload" as="font" type="font/woff2" crossorigin>` at the address the stylesheet fetches, so the file is fetched once and early. No other font is preloaded.

Measured on 2026-11-07, in KB of fonts, over the 31 template demos, each on its home page and one post at 390 and 1440 px (124 pages):

| | Before | After |
|---|---|---|
| Heaviest page | 186 (Chapter post) | 54 (Research home page) |
| Ledger home page, 390 px | 124 | 45 |
| Average page | 59.7 | 33.1 |
| Pages over 60 KB | 50 | 0 |

Chapter needed a small design change to fit. Its excerpts set the text in EB Garamond with small capitals and italics, beside Alegreya Sans in two weights: four files, 73 KB after the cut, and still 60.0 KB with no small capitals. So the Chapter demo now serves Alegreya Sans in 400 only. Its brand stylesheet, `fixtures/showcase-brands/chapter/theme/site.css`, sets `font-synthesis-weight: none`, so text the template sets bold is drawn in 400 and never in a bold the browser fakes, and gives the small capital labels a little more letter-spacing. EB Garamond also drops its small-capital feature, so the browser draws those small capitals from scaled capitals. A Chapter post now weighs 47 KB. The pack itself is unchanged: a site that serves its own bold keeps it.

To add or change a brand's fonts: put the Latin WOFF2 files and their `OFL.txt` in `theme/fonts/`, list them in `tokens.toml`, build the demos (`scripts/page-weight.sh` leaves them in `target/page-weight/sites`), then run `scripts/font-subset.py --sites target/page-weight/sites <brand>` and the check again. Cutting a file twice changes nothing, so run it again when the demo's text gains new letters.

## Fast to build

- `sccache` and `mold` in CI. `cargo nextest` with the default partitioning.
- `cargo check` on every push, the full suite on pull requests.
- `cargo bloat --release --crates` in the job summary, so a heavy dependency is seen the day it arrives.

## Baseline

`perf/baseline.json` holds the last accepted numbers and the fixture version. `scripts/perf.sh` runs every measure that can run on a CI runner and writes `perf/current.json`. The gate compares the two.

How `scripts/perf.sh` takes the four measures added for brief G9, all through `khalid-perf`:

- **Lint one post**: the median of 100 posts of the 1,000-post site, spread evenly, each linted against the whole site (`khalid-perf lint`). The `divan` benchmark in `crates/khalid-perf/benches/lint.rs` times the same thing.
- **Cached post page**: one post page of the 1,000-post site, asked for once so it is cached, then over 50 keep-alive connections for 30 seconds, `oha`'s defaults, with the client on the same machine (`khalid-perf requests`).
- **Public CSS, compressed**: the Brotli copy of each stylesheet the built home page links, from the 10,000-post build (`khalid-perf sizes`).
- **Admin initial JavaScript, compressed**: each script `admin/dist/index.html` loads at once (`<script src>` and `modulepreload`), gzipped at level 9 as `admin/scripts/size-budget.mjs` counts. `perf.sh` does not run npm, so it is measured only when `admin/dist` holds a build; otherwise the gate lists it as not measured.

## Demo bundle

`khalid-demos.tar.gz`, which serpdad.com/khalid/ is built from, has a budget of **1,055 MB** (MB of 1,000,000 bytes). `scripts/demos-bundle.sh` fails the Pages build when the archive is over it, and `scripts/demos-bundle.sh --check khalid-demos.tar.gz` checks a downloaded one. A budget is never raised to make the build pass.

Before it packs, the bundle step leaves out every raster picture that only structured data names, such as the original photo in each post's folder, and takes it out of that structured data. Pages load the sized copies in `media/` instead. It then fails if a page, stylesheet, feed or `templates.json` names a picture, video or sound the bundle does not hold.

Measured on 2026-11-07 (perf/demo-bundle):

| Bundle | Archive |
|---|---|
| Published, before AVIF reached the demos | 600.5 MB |
| Main with AVIF and 1,152 px, no pruning | 835.7 MB |
| Main with pruning (498 originals, 141.1 MB, left out) | 704.9 MB |
| Main with pruning and the 18 templates of batch 5 (49 demos; 696 originals, 205.6 MB, left out) | 1,024.4 MB |

The budget is the last result, what Pages publishes once batch 5 lands, with about 3% headroom. What stays is nearly all pictures pages load: with batch 5, WebP 514 MB, AVIF 271 MB, the 1,280 px JPEG fallback 185 MB and the share cards 83 MB, unpacked. A browser picks almost every width: across viewports 320 to 2,560 px at 1x to 3x, only 4 MB of files are never a browser's choice. A smaller bundle needs fewer formats, not fewer widths.

## CSS weight

Budget: each page downloads at most 10 KB of CSS, all the stylesheets it links together, compressed with Brotli at its best quality as Khalid serves them. It holds for the home page, a post, a category page and an author page of every template.

How it is held:

- **Each page links the rules it uses** (ADR "each page links the rules it uses"). After a page is rendered, the rules whose selectors name a class none of its elements has are left out of its stylesheet. Pages with the same classes share one sheet. The site's own rules in `theme/site.css` and the classes the reader script adds stay on every page.
- **`crates/khalid/tests/css_weight.rs`** builds the showcase site in every template, in its sample brand, and fails when one of the four pages links more than 10 KB. It runs with the other tests, with no browser.
- **The layout check** (`scripts/layout-check.sh`) holds every page it opens on every template demo, demo publications included, to the same 10 KB.

Measured on 2026-11-07 on the 31 template demos as `scripts/layout-check.sh` builds them, 124 pages (home, post, category and author of each):

| Measure | Before | After |
|---|---|---|
| Brotli, worst page | 14,342 bytes (Compass, home) | 8,919 bytes (Keystone, post) |
| Brotli, average page | 11,395 bytes | 5,886 bytes |
| Pages over 10 KB with Brotli | 112 of 124 | 0 of 124 |
| Gzip, average page | 13,274 bytes | 6,725 bytes |

On every page the layout check opens in all 87 demo builds, no selector that can match an element of the page is left out of its stylesheet, and the computed style of every element, and of its `::before`, `::after` and `::marker`, at 390 and 1440 px in Chromium, was the same before and after (1,732 page views; a view that differed on a first read because a picture loaded late was read again). A full build of the generated 10,000-post site took the same time before and after (about 24 to 26 s on the shared build server) and wrote 8 stylesheets. A page that moves to another kind of page downloads that page's sheet once; before, one cached sheet served both. The budget is never raised to make a check pass.
