# 0000. Templates are built-in packs chosen in site.toml

**Number:** next free. **Status:** Proposed, 2026-11-01. Templates spec sections 4.1 and 4.3.

## Context

The founder wants ten templates, each with the structure and features of a leading marketing blog in the site's own colours, fonts and logo. The spec asks for a `[template]` table in `site.toml`, a pack per template that holds only the files it changes, and new type-scale and spacing tokens. The CTO answered two questions on 2026-11-01: a site picks one template and adjusts that template's options, with no mixing across templates, and there are no public view counts. The plan has no themes marketplace (line 672) and defers theme packaging (line 881).

## Decision

- **A pack is a built-in look, not an installable theme.** Every pack is compiled into the binary. A pack is a `khalid_theme::Pack`: its own CSS rules, the template files it replaces, its token defaults as a `tokens.toml` fragment, the reader switches it turns on or off, and its options. Classic, today's look, is the empty pack and every site's default, so no site changes on upgrade.
- **Packs live in `khalid-theme`.** It is the crate that already owns the look of a site. `khalid-render` does not depend on it: the binary hands the renderer the pack's CSS, template files and token defaults as plain values. So the renderer stays free of theme sync's network code.
- **`site.toml` is read in two steps.** `khalid-core` reads `[template]` as written: a plain-word name and options as on or off, whole numbers, words, lists of words or small tables, and refuses anything else with its line. `khalid-theme` then checks the name against the packs and each option against the pack's list, fills every option the site leaves out with its default, and refuses an unknown name, an unknown option or a wrong value with a plain sentence. The binary does this as soon as it reads the settings, in the same place as the SEO checks, so a mistake stops every command with its line.
- **Options are data, not one Rust struct per template.** Each pack lists its options as `OptionSpec`s: key, one sentence of help, and the kind (switch, choice from a list, bounded count, short list of lines) with its default. Invalid values are still unrepresentable after the check, and the same list feeds the docs test, a future `GET /api/v1/templates` options schema and the admin's "Look" screen without a second copy.
- **Order of what wins, lowest first:** the shared defaults, the pack, then the site. For tokens, a value in `theme/tokens.toml` wins over the pack's default. For reader switches, a key the site writes in `[reader]` wins over the pack's default. For templates, a file in `theme/templates/` wins over the pack's file, which wins over the shared one.
- **A site ships only its own pack's styles.** The stylesheet is the tokens, the shared rules, then the chosen pack's rules. The 20 KB compressed budget applies to that file, so a test measures it for every pack.
- **The proof pack is test-only.** A `trial` pack exists only with the `test-util` feature of `khalid-theme`, which only test builds enable. It changes one template, one rule, one size and one switch, so tests prove a switch changes the build and classic does not.

## Consequences

- Adding a template is adding a `Pack` and its files to `khalid-theme`, with no change to the renderer or the binary.
- The pack's token defaults are checked by the renderer's own `tokens.toml` reader when the binary starts; a test builds every pack so a broken default never ships.
- A future template that needs a new kind of option adds an `OptionKind` variant and its check in one place.
