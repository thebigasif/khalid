# Example: a marketing agency's blog

This is the blog of Kestrel & Lime, a made-up marketing agency that helps clinics, trades and shops get found nearby and keep in touch with their customers. Copy it if you run an agency or a consultancy and want a blog that shows your work and the people who do it.

The agency, its people and its clients are invented. The case studies give no figures, because an example should not show numbers no one can check. The advice is real, and every rule it mentions links to the official page.

## Who it is for

An agency or a small consultancy whose clients buy the people, not only the service. Each post is written by the expert on the team who does that work, and a second expert checks it.

## What it shows

| Feature | Where |
|---|---|
| The Compass template, with key takeaways at the top of each post and a call to action | `site.toml`, `[template]` |
| Case studies in their own collection, at `/blog/work/<slug>/` | `content/en/work/`, `routes.toml` |
| Author expertise: three authors with roles and credentials, and a named reviewer on every post | `content/en/authors/`, `reviewed_by:` in each post |
| Key takeaways boxes, notes, warnings and a pull quote | `:::callout{kind=summary}` and `:::pullquote` in posts |
| A services page with the agency's address for search engines and a contact form | `content/en/pages/services.md` |
| An about page that says how posts are checked | `content/en/pages/about.md` |
| Front page sections for case studies and each topic | `[reader] home_sections` |
| Form answers sent by email | `[mail]` in `site.toml` |

Every post links to two or more others and to one outside source, so `khalid lint` and `khalid audit` find nothing to fix.

## Try it

```sh
cargo run -p khalid -- --site fixtures/examples/agency build
```

The pages are in `fixtures/examples/agency/public/`. Until Khalid has the Compass template, remove the `[template]` and `[template.cta]` tables from `site.toml` first; the site then uses the classic look. The contact form needs a real mail server in `[mail]` before its answers reach anyone.

`crates/khalid/tests/examples.rs` builds this site and checks it stays clean.
