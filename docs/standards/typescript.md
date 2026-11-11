# TypeScript in this repository

The admin is the one place JavaScript is allowed, because the editor needs it. Everything else on the public site is HTML and CSS from Rust, with small optional enhancements.

## Stack

| Need | Choice |
|---|---|
| Build | Vite |
| UI | React 19 with TypeScript in `strict` mode, `noUncheckedIndexedAccess` on |
| Editor | Tiptap, markdown as the stored format, one extension per block, each with its directive serialiser |
| Data | TanStack Query against the Admin API, types generated from `/api/openapi.json` with `openapi-typescript`. No hand-written API types |
| Styling | Tailwind 4 with the tokens from `theme/tokens.toml` exposed as CSS variables. Light theme only |
| Routing | TanStack Router |
| Lint and format | Biome, `recommended` plus `a11y` rules as errors |
| Tests | Vitest and Testing Library for components, Playwright for journeys, axe for accessibility |
| Public-site enhancements | Plain TypeScript modules under `web/enhance/`, each under 5 KB compressed, each a progressive enhancement that does nothing when absent |

## Rules

- No `any`, no `as unknown as`, no non-null assertions. Narrow with type guards.
- Every client in `admin/src/api/` takes its types and request bodies from `components["schemas"]` in the generated `schema.d.ts`; a test fails a client that does not. Where the document leaves a field open, such as the `[seo]` table, the client narrows only that field with `Omit` and says why. A field the document marks optional is read as possibly missing, even when today's server always sends it.
- Components under 200 lines, one component per file, named exports. Default exports only for route files.
- Every interactive element is reachable by keyboard and has a visible focus ring. Every form field has a label. Every icon-only button has `aria-label`.
- Every user-visible string lives in `admin/src/strings/en.ts`, so a second language is a file, not a refactor.
- No inline styles. No colour literals outside the token file.
- State that belongs to the server lives in TanStack Query. State that belongs to a screen lives in that screen. No global store until two screens need the same client state.
- Error boundaries on every route. A caught error shows a `UserMessage`: what, why, fix.
- Autosave every few seconds and on blur, with an unsaved-changes guard on navigation.
- The editor round-trips: a document opened and saved without edits produces the identical markdown. One test per block.
- The admin stays inside both limits in `performance.md`, "Admin size check": the first download and every built file together. Code-split by route: a lazy screen costs the first download nothing. No dependency over 50 KB compressed without a sentence in the pull request.
- Accessibility tests run on every screen in CI with axe. Zero violations.

## Mockups first

An admin screen starts as a static HTML mockup under `docs/design/admin-mockups/`, built with the same tokens. The React screen is then built to match it without waiting for a yes, and the founder sees both and asks for changes after. The mockup is the acceptance test's picture.
