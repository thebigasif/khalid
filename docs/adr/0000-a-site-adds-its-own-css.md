# 0000. A site adds its own CSS rules in theme/site.css

**Number:** next free. **Status:** Proposed, 2026-11-01. Website visual task (phase 1).

## Context

A site can replace any template file in `theme/templates/` and set every token in `theme/tokens.toml`, but it had no way to add a CSS rule. Khalid's own website needs a designed home page, a template gallery and cards with icons on top of the Atlas template, and those need rules no template has. The options were an inline `<style>` in a copied `base.html`, a second stylesheet linked from a copied `base.html`, or one file the build adds to the stylesheet it already makes.

## Decision

- `theme/site.css`, when it exists, is added to the one public stylesheet after the template pack's rules. The same function feeds `khalid build` and `khalid serve`, so both serve the same file at the same hashed address.
- The file goes through the same minifier as the shared rules, so comments and blank lines are removed.
- The path is checked like `theme/tokens.toml`: a symlink that leads outside the site folder stops the build with a message that names the file.

## Consequences

- A site keeps one stylesheet, one request and one hashed name, and the 20 KB compressed budget measures the whole of it.
- A site's rules read the token variables, so theme sync still changes the colours and fonts of a site that has its own rules.
- A copied `base.html` is no longer needed for a few rules, so sites follow engine changes to `base.html`.
- Khalid does not check the file's CSS. A broken rule is the browser's to ignore, as with any stylesheet.
