# Example: a software company's blog

This is the blog of Fernhour, a made-up company that sells a staff scheduling app to small cafes, shops and clinics. Copy it if you run a software product and want a blog that teaches your customers and tells them what is new.

The brand, its people and its app are invented. The advice on rotas is real, and every rule about working hours links to the official page.

## Who it is for

A software company with a small team that writes its own posts: guides that help customers do their job, fair comparisons, and short notes on each new feature.

## What it shows

| Feature | Where |
|---|---|
| The Harbor template, with a call to action in the header, under each post and on the front page | `site.toml`, `[template]` |
| Guides with step-by-step blocks that search engines read as how-tos | `content/en/posts/`, such as the staff rota guide |
| Comparison pages that open with the verdict and set the choices side by side in a table, with pros and cons | the two posts in the Comparisons category |
| Product updates in their own collection, at `/blog/updates/<slug>/` | `content/en/updates/`, `routes.toml` |
| Newsletter sign-up forms at the end of posts | `::newsletter` in posts, `[mail]` in `site.toml` |
| Front page sections for each category and for the updates | `[reader] home_sections` |
| Two authors with roles and credentials, and an about page | `content/en/authors/`, `content/en/pages/about.md` |
| Callouts: tips, notes and warnings | in most posts |

Every post links to two or more others and to one outside source, so `khalid lint` and `khalid audit` find nothing to fix.

## Try it

```sh
cargo run -p khalid -- --site fixtures/examples/saas build
```

The pages are in `fixtures/examples/saas/public/`. Until Khalid has the Harbor template, remove the `[template]` and `[template.cta]` tables from `site.toml` first; the site then uses the classic look. The newsletter form needs a real mail server in `[mail]` before it can take sign-ups.

`crates/khalid/tests/examples.rs` builds this site and checks it stays clean.
