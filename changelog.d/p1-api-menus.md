Fixed
- The Content API's `navigation` list now holds each menu's links. Before, `en.primary` and `en.footer` always came back empty, even when `site.toml` had `[[nav.primary]]` and `[[nav.footer]]` links. Each language's menus hold the links its pages show, in the order written, and the footer ends with the AI info page while it is on.
