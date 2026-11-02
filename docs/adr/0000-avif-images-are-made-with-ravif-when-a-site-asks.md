# 0000. AVIF images are made with ravif, when a site asks for them

**Number:** next free. **Status:** Proposed, 2026-11-02. Phase 1 task 1.12 (images), the admin gaps task (adminfix), plan section 4.4.

## Context

The plan says images are converted to WebP and AVIF at publish. `khalid-media` has made WebP and the original's format since task 1.12; AVIF waited on a question in the phase brief, because every pure-Rust AVIF encoder is built on `rav1e`, and three of its parts fail `deny.toml`: `libfuzzer-sys` carries the NCSA licence alongside MIT or Apache-2.0, `av1-grain` and `av-scenechange` use `anyhow`, which is banned outside the binary, and `paste` is flagged unmaintained (RUSTSEC-2024-0436). The brief offered two answers: allow the three, or ship WebP only. The founder said yes in advance to every choice in this task (2026-10-27), and the task asks for AVIF.

Encoding is slow. Measured on the build server with `ravif` 0.13 at speed 8, quality 60, built at opt-level 3: 0.5 s for an 800 px wide image and 1.7 s at 2048 px. WebP takes milliseconds. One image makes up to eight sizes.

## Decision

- **`ravif` is the encoder**, behind `khalid-media`'s `Format::Avif`, at speed 8 and quality 60: speed 8 gives files about 40% smaller than speed 10 for under three times the time, and speeds below 8 gave almost nothing more.
- **AVIF is off unless a site turns it on**, with `avif = true` under `[media]` in `site.toml`. The build-time budget holds for every site that does not ask, and a site that does pays the cost once per image: each file is named by a hash of the original and cached, so an image that did not change is never encoded again.
- **A page offers AVIF first, then WebP, then the original's format**, as `<source>` elements in one `<picture>`, so every browser takes the smallest file it can read and an old one still gets the JPEG or PNG.
- **`deny.toml` gains three narrow exceptions**: NCSA for `libfuzzer-sys` only, `av1-grain` and `av-scenechange` as `anyhow` wrappers, and the `paste` advisory with a review date of 2027-05-02, each with its reason. `paste` is a macro used at compile time and runs on no input of ours; `anyhow` stays inside the encoder's own errors, which `khalid-media` turns into its `MediaError`.

## Consequences

- The release binary carries the AV1 encoder. Its size is measured in the pull request against the 30 MB budget.
- A site with many photos that turns AVIF on builds slower the first time; later builds read the cache.
- When a maintained encoder drops `paste` and `anyhow`, the exceptions go.
