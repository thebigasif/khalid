# Sample brands for the template demos

Each template's demo shows the showcase site (`fixtures/showcase-site/`) under a sample brand of its own, so every demo looks like a real blog and no two look alike. Each brand's design brief is the "Design" section of `docs/templates/<template>.md`.

Each `<template>/theme/` folder holds:

- `tokens.toml`: the brand's colours, font stacks and self-hosted font files, in the shape `khalid theme sync` writes. It sets only what a brand owns. Shape, type sizes and spacing stay the template's own.
- `fonts/`: the brand's font pair as Latin WOFF2 files, at most four, each family's `OFL.txt` licence beside them. Every file has its fallback metrics in `tokens.toml`, measured by the same code `theme sync` runs, so the page does not shift while it loads. Each file is cut to the letters its demo shows by `scripts/font-subset.py`, which keeps every demo page under the font budget (`docs/standards/performance.md`, "Fonts"). A font whose licence reserves its name is cut under a new one, such as PT Serif as "Ledger Serif". A brand may add `site.css`, the site's own rules, where its fonts need them: Chapter's draws its bold text in its one Alegreya Sans weight.
- `logo.svg`: a small mark drawn for the demo, with the showcase's name.

To build a branded demo, copy over a copy of the showcase site:

- `theme/tokens.toml` and `theme/fonts/` into its `theme/`
- `theme/logo.svg` to `content/en/pages/about/logo.svg`, the address `tokens.toml` names for the logo

Never edit the showcase site itself: the copy is what changes.

Every palette passes WCAG AA, as `ThemeTokens::fix_contrast` checks it: text, quiet text and links reach 4.5:1, and the accent 3:1, on both the background and the surface.
