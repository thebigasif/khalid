# Khalid's website

This folder is Khalid's own website, built with Khalid and published at https://thebigasif.github.io/khalid/. It tells the story of the product: who it is for, every feature group, the templates, the use cases, how to start and how it compares.

## What is where

- `theme/templates/home.html` replaces the home page, which would otherwise list posts, with the story.
- `content/en/features/` holds one post per feature group, listed at `/khalid/features/`.
- `content/en/use-cases/` holds one post per use case, listed at `/khalid/use-cases/`.
- `content/en/pages/` holds Templates, Start, Compare and About.
- `routes.toml` gives the two collections their addresses.

## The sites published beside it

The pages link to sites built from other folders and published under the same address:

| Address | Built from |
|---|---|
| `/khalid/demo/` | the showcase demo site |
| `/khalid/templates/<name>/` | the demo site in each template |
| `/khalid/examples/<slug>/` | the use case example sites |

## Checking it

Run these from the repository root:

```sh
cargo run -p khalid -- --site site lint
cargo run -p khalid -- --site site audit --offline
```

The linter must report 0 blocks and 0 warnings. The audit of this folder alone reports the links to the sites above as broken, because they are built elsewhere; it has no other error and no warning. `crates/khalid/tests/website.rs` checks both, and that every page links its live demo.

## Rules for new words here

Short sentences, one idea each. Say what a feature does for the reader before how it works. Describe only what Khalid does in the version on `main`, and link a source for every claim about another product.
