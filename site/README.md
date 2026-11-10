# Khalid's website

This folder is Khalid's old website, built with Khalid. Khalid's home is now https://serpdad.com/khalid/, and https://thebigasif.github.io/khalid/ only forwards there (see "The forwarders on GitHub Pages" below). It tells the story of the product: who it is for, every feature group, the templates, the use cases, how to start and how it compares.

## What is where

- `site.toml` puts the site in the Atlas template, with a "Start a blog" button and footer link columns.
- `theme/tokens.toml` holds the colours, fonts and sizes: paper, ink and an orange accent.
- `theme/site.css` holds the site's own rules: the hero, the tiles, the gallery and the bands. Every rule reads the tokens.
- `theme/templates/home.html` replaces the home page, which would otherwise list posts, with the story in sections.
- `theme/templates/collection.html` lists features and use cases as tiles, and holds the drawn icons home.html imports.
- `theme/templates/page.html` gives every page its hero, and the Templates page its gallery.
- `content/en/features/` holds one post per feature group, listed at `/khalid/features/`.
- `content/en/use-cases/` holds one post per use case, listed at `/khalid/use-cases/`.
- `content/en/pages/` holds Templates, Start, Compare and About.
- `routes.toml` gives the two collections their addresses.

## The template pictures

The gallery shows a picture of each template demo's home page and of one post, at `/khalid/templates/<name>-home.svg` and `<name>-post.svg`. The files beside `content/en/pages/templates/index.md` are plain cards with each template's name, so the site always has a picture. After `scripts/pages.sh` builds everything, `scripts/template-previews.sh <pages folder>` replaces them in the built folder with screenshots taken in a headless Chromium, in a window 1440 pixels wide, so each template shows its wide-screen frame. Where no Chromium runs, it writes the plain cards again. To remake the committed cards after changing them in the script, run `scripts/template-previews.sh --cards site/content/en/pages/templates`.

## The sites built beside it

The pages link to sites built from other folders under the same mount. The Pages workflow builds them all for their old addresses:

| Address | Built from |
|---|---|
| `/khalid/demo/` | the showcase demo site |
| `/khalid/templates/<name>/` | the demo site in each template |
| `/khalid/examples/<slug>/` | the use case example sites |

## The copy on serpdad.com

Khalid's main home is https://serpdad.com/khalid/, SerpDad's product page, and the demos live below it. `scripts/demos-bundle.sh <khalid binary> <new folder>` builds the demos, the template gallery and its pictures for that address and packs them as `khalid-demos.tar.gz`, with `demo/`, `templates/` and `examples/` at the top. The website's other pages are left out. The Pages workflow publishes the archive on the GitHub release named `demos`, which the serpdad.com build downloads.

The demos are made-up sample blogs, so every demo page, in both copies, carries `noindex, follow` and no demo sitemap lists a page. The gallery stays indexable.
## The forwarders on GitHub Pages

GitHub Pages publishes no copy of these sites any more. The Pages workflow still builds this site and the demos for their old addresses, then `scripts/pages-forward.sh` writes one small page per old address. Each page sends visitors and search engines to the matching page on serpdad.com, with a meta refresh, a canonical link and a visible link, and says noindex. A demo page goes to the same page of the template's new demo where it still exists, and to the demo's front page otherwise. The website's pages go to the landing page, the getting started guide and the comparison. Any other address keeps its path on serpdad.com.

`scripts/pages-forward.sh --selftest` checks the map with no network. After serpdad.com serves the same bundle, `scripts/pages-forward.sh --check <forwarders folder>` checks that every new address answers 200.

## Checking it

Run these from the repository root:

```sh
cargo run -p khalid -- --site site lint
cargo run -p khalid -- --site site audit --offline
```

The linter must report 0 blocks and 0 warnings. The audit of this folder alone reports the links to the sites above as broken, because they are built elsewhere; it has no other error and no warning. `crates/khalid/tests/website.rs` checks both, and that every page links its live demo.

## Rules for new words here

Short sentences, one idea each. Say what a feature does for the reader before how it works. Describe only what Khalid does in the version on `main`, and link a source for every claim about another product.
