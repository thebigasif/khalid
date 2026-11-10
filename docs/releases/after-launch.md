# Checks still open after launch

Khalid 1.0.0 shipped on 2026-11-01 with four checks left for after launch (`docs/decisions/2026-11-01-ship-1-0-0.md`). Each needs a real public server, an outside account or a person, so none can run on our test machines. None of them can lose your posts.

The 1.0.0 notes promised to report what these checks find in the next release's notes. Releases 1.1.0 to 1.8.0 did not. This page is that report. It is kept up to date, and every release from 1.9.0 on links to it while a check is open.

Each check below says where it stands, what our tests prove in the meantime, what to do if it matters to you, and how the check gets run.

## Ahrefs and Semrush Site Audit on the demo site

Status: open. Not yet run, as of 2026-11-10.

- **Proved meanwhile.** `khalid audit` crawls a site the way Ahrefs and Semrush do and checks the same things, in their categories. It gives a new site a health score of 100 with no errors and no warnings. On every change, CI's tests audit the demo site and the showcase site, and both must score 100.
- **If it matters to you.** Run `khalid audit` on your own site. It explains each problem and how to fix it.
- **How it gets run.** Someone with an Ahrefs or a Semrush account starts a Site Audit of one demo blog under `serpdad.com/khalid/demos/`. The demos ask search engines not to list them, so the audit must be set to crawl pages marked "noindex". The goal (KH-LNT-5) is 100 with zero errors and zero warnings. Each error or warning becomes a GitHub issue, and the result goes on this page.

## HTTPS with a live Let's Encrypt certificate

Status: open. Not yet run, as of 2026-11-10.

- **Proved meanwhile.** On every change to the HTTPS code, CI runs `khalid serve --domain` against pebble, the test server Let's Encrypt publishes. It gets a certificate, answers over HTTPS, and renews the certificate while it keeps running.
- **If it matters to you.** If HTTPS fails on your server, put Khalid behind a web server that handles HTTPS, as `docs/deploy.md` shows.
- **How it gets run.** On a server with a public address and a domain that points to it, follow "A Hetzner Cloud server" in `docs/deploy.md`, or run `khalid serve --domain <your domain>`. The check passes when a browser opens `https://<your domain>/` with no warning (KH-INS-4). The renewal is checked when the certificate is 60 days old.

## The Astro and Next.js starter kits, deployed

Status: open. Not yet run against a live blog, as of 2026-11-10.

- **Proved meanwhile.** Since 1.4.0, CI builds both kits against the showcase blog in headless mode, as their READMEs say, whenever the kits, the API or the page renderer change. Each kit's home page and one post must carry a title, a meta description and schema (KH-INS-5).
- **If it matters to you.** Use the kits as examples, and follow their README.
- **How it gets run.** Deploy one kit, for example to Netlify or Vercel, pointed at a live Khalid blog with a Content API key. The check passes when the deployed home page and a post show the same title, description and schema as the blog itself.

## SerpDad's own blog on Khalid

Status: open. serpdad.com/blog has not moved yet, as of 2026-11-10.

- **Proved meanwhile.** The template demos already run at `serpdad.com/khalid/demos/`, built by `khalid build`.
- **If it matters to you.** Nothing changes for your blog. This is SerpDad's own move.
- **How it gets run.** SerpDad moves its blog over with `khalid import`, keeps every old address with redirects, and checks it with `khalid audit` and Search Console. The result goes on this page.

## When a check is done

Change its status line to `Status: done, <date>.`, say what it found in one or two sentences, and name the release whose notes report it. When all four say done, releases stop linking here.
