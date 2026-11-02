# 0000. Charts, allowed embeds, tracking tags, the preferred source button and the newsletter page

**Number:** next free. **Status:** Proposed, 2026-11-02. Phase 1, the engagement gaps from the Ahrefs blog teardown.

## Context

The founder compared Khalid with the Ahrefs blog and found five things nobody had planned: charts in posts, embedded interactive tools, a newsletter page with an archive, a "preferred source on Google" button, and tracking tags on links to the owner's product. The brief left the shapes open. Each needed a choice the plan does not cover.

## Decision

- **Charts are drawn at build time as SVG, from CSV lines.** One format (CSV lines in the block, or a CSV file beside the post) rather than CSV or TOML: fewer rules to learn, and a spreadsheet exports it. The SVG uses shares of the width and a fixed height, so text keeps its size on a phone and nothing shifts. Colours are classes the shared block stylesheet fills from tokens, so every template colours charts its own way. A visually hidden table carries the numbers for screen readers; the SVG is `aria-hidden`. Values are checked numbers kept as written, so the block round-trips. The build hands the renderer a copy of the body with the file's lines; the file on disk never changes (BL-FS-1).
- **The linter reads a chart's caption, not its numbers.** The drawing and its table are data. Like a formula's MathML, they are left out of the text the linter reads, and the caption must say what and when. Without this, every chart breaks `sourced-statistics`.
- **Allowed embeds are exact hosts in `[media] embed_domains`,** `https` only, framed lazily with a fixed height and a sandbox of scripts, forms, popups and the page's own origin. Only those hosts join the CSP's `frame-src`. The renderer takes them through a new `render_framed`; `render_gated` keeps its signature and allows none.
- **Tracking tags live in `[seo.links.tracking]`,** beside the link policy, and are applied in the same pass as it. The campaign defaults to the post's slug, `blog` for other pages and `cta` for the call to action, so a site gets per-post numbers without setting anything but the domains.
- **The preferred source button is a `[template.preferred_source]` table,** not a key in `[template.options]`. Classic takes no options, and `scripts/pages.sh` drops `[template.options]` when it builds each template's demo; a table beside `[template.cta]` reaches every template and every demo. The binary hands it to templates as two template options that every pack reads, so the render structs did not change. The link is `https://www.google.com/preferences/source?q=<host>`.
- **Newsletter issues are content files** in `content/newsletters/main/issues/<day>-<slug>.md`, written by the owner, beside `welcome.md` and in its shape (a `# ` subject, then the text). Not copies the send command writes: content files are the source of truth, and a static site has no database to read sends from. The page is built only when `page.md` exists, in the main language only, like the AI page. The sign-up form shows only on a server with `[mail]`.

## Consequences

- A chart in the Content API shows its caption and link only when its numbers are in a file, since the API renders bodies without the files beside them.
- The admin editor's block list (`admin/src/editor/blocks/specs.ts`) has neither the chart block nor the embed's `height` yet. Adding them needs the admin's strings, samples and Vitest suite, which cannot run on the build server, so it is a follow-up for a session that can run them.
- Writers keep an issue's file by hand after sending it. A later task could offer to write it.
- The newsletter page has no German copy on a two-language site; its words come from the main language.
