# The admin size check measures the first download

Number: next free. Amends the 350 KB admin budget in `docs/standards/performance.md`.

**Status:** Accepted, 2026-11-10. CTO decision during the repair of integration 32 (pull request 1216).

## Context

`admin/scripts/size-budget.mjs` failed CI's Admin job when every `.js`, `.css` and `.html` file in `admin/dist` added up to more than 350 KB, gzipped. Integration 32 brought 42 pull requests of new admin screens, and the total reached 370.7 KB.

That total is not what a visitor waits for. A screen loaded with a lazy `import()` downloads only when someone opens it. Worse, the check punished lazy screens: each lazy file compresses on its own, so splitting a screen out added bytes to the total. The first repair made Forms, Tags and Navigation eager to pass, which made the first download larger (208.6 KB to 210.9 KB) and still failed at 366.4 KB. The performance budget already counts only what the admin loads at once.

## Decision

- **Two limits, both gzipped at level 9.** The first download, at most 250 KB: `index.html`, each script and stylesheet it loads at once, and every chunk those scripts import at the top. Every built file together, at most 450 KB, as a ceiling against runaway growth.
- **Lazy imports are not followed.** The script follows `import ... from "./x.js"` and `import "./x.js"`, never `import("./x.js")`.
- **Forms, Tags and Navigation are lazy again,** because that keeps the first download smaller.

## Consequences

- A new screen should load lazily. It then costs the first download nothing, and the ceiling still stops the admin as a whole from growing without notice.
- On 2026-11-10 the admin measures 208.6 KB first and 370.7 KB in all, leaving about 41 KB and 79 KB of room.
- The performance budget row "Admin initial JavaScript" still says 350 KB and counts scripts only. It is looser than the new 250 KB limit, so the Admin job is the one that fails first.
