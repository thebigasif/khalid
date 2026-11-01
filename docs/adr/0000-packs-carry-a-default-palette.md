# 0000. A template pack carries its own default colours

**Number:** next free. **Status:** Accepted, 2026-11-01. Phase 1, templates visual pass (task/p1-visual-a).

## Context

The founder judged the first template demos "pretty average": the live demos at thebigasif.github.io/khalid/templates/ looked plain, with almost no colour. The spec (section 3.0) says every colour comes from the seven colour tokens and no template ships a fixed colour, so each pack's stylesheet only mixes tints of the site's tokens. A site with no `theme/tokens.toml`, such as every demo, then gets the built-in defaults: white, pale grey and one blue. All ten templates looked like the same blue site.

A pack's `tokens` text already sets defaults for `[type]`, `[space]` and `[shape]`, which a site's own `tokens.toml` overrides key by key (`Tokens::load_over`).

## Decision

- **A pack may also set `[colors]` and `[fonts]` in its token defaults.** They are defaults like its type scale: any key the site's `tokens.toml` sets, by hand or by `khalid theme sync`, wins. The stylesheet rule stays: no colour is written in a pack's CSS, every colour is a token or a `color-mix()` tint of one.
- **Five packs get their own palette now:** Ledger (warm paper, ink green, serif headings), Atlas (indigo), Keystone (warm orange), Compass (teal, rounded headings), Harbor (berry). Each is our own choice, not taken from any reference site.
- **Every pack's palette is tested** (`look.rs`): text, muted and link colours at 4.5 to 1 on the background (WCAG 2.2 AA), the link at 4.5 to 1 on the surface, and the accent at 3 to 1 on both the background and the surface, for buttons, rules and focus rings. Button text uses `--color-on-accent`, which the build already picks.

## Consequences

- A site that syncs its main site's colours sees its own colours, as before. A site that sets only some colours mixes them with the pack's for the rest; theme sync always writes all seven, so this only happens with a hand-written file.
- Switching template on a site with no colour tokens changes its colours. That is the point of a look, and the Look screen's preview shows it.
- The other five packs keep the built-in defaults until their own visual pass gives them a palette.
