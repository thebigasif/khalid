# Example: a writer's blog that is also a newsletter

This is The Allotment Letter, a made-up weekly letter by Margot Ellery about growing vegetables on a small allotment. Copy it if you write a newsletter and want every issue to live on your own site too, with an archive anyone can read and a way to sign up on every page.

The writer and her plot are invented. The growing advice is real, and each letter links to a source you can check.

## Who it is for

One writer, or a writer with a small team, who sends a regular letter by email. Some readers want the free letter. Others want to support the writer and get something extra. This site shows both.

## What it shows

| Feature | Where |
|---|---|
| The Keystone template: a large featured letter on the front page, a picture grid, a sign-up line and lines about the writer after each letter | `site.toml`, `[template]` |
| Two newsletters: the free Thursday letter and a paid-style supporters' list | `content/newsletters/main/` and `content/newsletters/supporters/` |
| A welcome email for each newsletter, sent when a reader confirms | `welcome.md` in each newsletter folder |
| Sign-up forms in letters and on a subscribe page, one for each newsletter | `::newsletter` blocks, `content/en/pages/subscribe.md` |
| A section only subscribers can open, with their email address and no password | `:::gated` in the first-year letter |
| A "Start here" page with its own feed, listing the featured letters | `routes.toml`, `featured: true` |
| The archive of every letter, by date | `/blog/archive/` in the built site |
| A person as the publisher, with an about page | `[seo.publisher]`, `content/en/pages/about.md` |
| Step-by-step blocks, a comparison table and tips, notes and warnings | in the letters |

Every letter links to two or more others and to one outside source, so `khalid lint` and `khalid audit` find nothing to fix.

## Try it

```sh
cargo run -p khalid -- --site fixtures/examples/newsletter build
```

The pages are in `fixtures/examples/newsletter/public/`. Until Khalid has the Keystone template, remove the `[template]`, `[template.options]` and `[template.cta]` tables from `site.toml` first; the site then uses the classic look. Sign-ups, welcome emails and the subscribers' section need `khalid serve` and a real mail server in `[mail]`. To send a letter to the supporters' list, see `docs/newsletters.md`.

`crates/khalid/tests/examples.rs` builds this site and checks it stays clean.
