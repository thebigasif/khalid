# Khalid

**A blog that checks every post before it goes live.**

Khalid is an open-source blog engine you add to the website you already have. It ships with the SEO that big content teams build by hand, and it will not publish a post with the mistakes that hurt it in search.

[Website](https://thebigasif.github.io/khalid/) · [Live demo](https://thebigasif.github.io/khalid/demo/) · [Templates](https://thebigasif.github.io/khalid/templates/) · [Use cases](https://thebigasif.github.io/khalid/examples/saas/) · [Latest release](https://github.com/thebigasif/khalid/releases/latest)

[![Deploy to Cloudflare](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/thebigasif/khalid/tree/main/deploy/cloudflare)

## Why Khalid

- **SEO done for you.** Titles, descriptions, canonical links, sitemaps, schema markup for every kind of post, share images and redirects are all built in. Khalid's own site audit passes the Ahrefs and Semrush checks.
- **A linter that guards every post.** A post with a missing description, a skipped heading level, a link to nowhere or too little to say does not go live. Every refusal says what is wrong, why it matters and how to fix it.
- **Ready for AI search.** Answer-first writing checks, an AI info page and clean structured data help AI search find and quote your posts.
- **Templates that look professional.** Eleven layouts for every kind of blog, in your own colours, fonts and logo. [See them all](https://thebigasif.github.io/khalid/templates/).
- **Fits any website.** Run it at `blog.example.com` or under `example.com/blog/`, next to any website. `khalid init` writes the rule for Nginx, Caddy, Apache or Traefik for you.
- **One small program.** A single binary with no database server to run. Every post is a text file you can keep in git.
- **A web editor and a command line.** Writers use the admin in the browser. Developers and AI agents use the command line, the APIs and the MCP server.
- **Free and open source.** Run it yourself, on any server, in Docker, or free on Cloudflare.

## Start in a minute

```text
curl -fsSL https://raw.githubusercontent.com/thebigasif/khalid/main/scripts/install.sh | sh
khalid init blog --address https://blog.example.com --yes
khalid --site blog serve
```

Your blog is now at `http://127.0.0.1:8080/`. The full steps, Windows and Docker are below.

## Install

On Linux or a Mac, this one line downloads the newest release, checks it and puts `khalid` on your path. On Windows, download the `.exe` from the [releases page](https://github.com/thebigasif/khalid/releases). For Docker, see `docs/deploy.md`.

```text
curl -fsSL https://raw.githubusercontent.com/thebigasif/khalid/main/scripts/install.sh | sh
```

Or build Khalid from source. You need git, Rust through [rustup](https://rustup.rs/), a C compiler and Node.js 22. The npm steps build the admin, the web editor; `cargo` alone builds a blog without it. `CONTRIBUTING.md` explains each step.

```sh
git clone https://github.com/thebigasif/khalid.git
cd khalid/admin
npm ci
npm run build
cd ..
cargo install --path crates/khalid --locked
```

Then make a site and run it:

```sh
khalid init blog --address https://blog.example.com --yes
MIMALLOC_PURGE_DELAY=0 khalid --site blog serve
```

The blog is then at `http://127.0.0.1:8080/`. `MIMALLOC_PURGE_DELAY=0` makes Khalid give the memory it no longer needs back to the computer at once, so a quiet blog stays small. The Docker image and the systemd service set it for you. To sign in to the admin, make yourself the owner from a second terminal:

```sh
khalid --site blog staff owner --email you@example.com --name "Your Name"
```

Khalid shows the sign-in link only this once. Open the `localhost` address it shows while `khalid serve` runs.

To write a post, run `khalid --site blog new --draft "Why internal links matter"`. It writes `blog/drafts/en/why-internal-links-matter.md`, which readers never see. Its address leaves out filler words such as "the", which search engines skip. Write the post below its front matter, then run `khalid --site blog publish why-internal-links-matter`. The linter checks it first, and only a post it lets through moves into `blog/content/en/posts/` with today's date. Without `--draft`, `new` writes the post straight into `content/`, dated today, and it goes live at once.

`docs/getting-started.md` goes on from there: an author, a first post, publishing, the admin, checks, imports, and backup and restore. `docs/deploy.md` covers the systemd service, the Docker image and Docker Compose, and the ready files for Fly.io, Railway, Coolify, a Hetzner Cloud server and Cloudflare.

## A free blog on Cloudflare

This button copies a small, ready Khalid blog into your GitHub account and puts it on Cloudflare for free. Cloudflare builds it again on every change you push. The blog is static, so the admin, comments and forms, which need a running server, are not part of it. `docs/deploy/cloudflare.md` explains each step, your own domain, and the Cloudflare Pages way.

[![Deploy to Cloudflare](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/thebigasif/khalid/tree/main/deploy/cloudflare)

## Where things are

| | |
|---|---|
| `docs/getting-started.md` | How to use Khalid: a site, a post, the admin, checks, imports, backups |
| `docs/deploy.md` | Running Khalid on a server, in Docker, or on a hosting platform |
| `docs/plan.md` | The build plan, with every founder decision. Read it first |
| `docs/phases/` | What is being built right now, task by task |
| `docs/standards/` | The rule catalogue and the standards every change follows |
| `docs/adr/` | Architecture decisions |
| `CLAUDE.md` | The contract for the coding agents that write this code |
| `FRONTEND.md` | The front-end checklist every template follows |
| `.claude/skills/` | Agent skills: BeautifulRust's Rust skills, plus this repository's own |
| `lints/` | The clippy configuration, and the crate that proves every lint fires |

## How it is built

The code is written by Claude Opus 5.5 and reviewed by agents against a 10-point rubric before a person sees it. The rules come from [BeautifulRust](https://github.com/morganlinton/BeautifulRust) (MIT, Morgan Linton), the Rust API Guidelines, Effective Rust and Microsoft's Pragmatic Rust Guidelines, with this repository's own rules on top. Every machine-checkable rule is a lint or a CI gate. Performance has budgets, and the budgets are gates too.

## Licence

AGPL-3.0, see `LICENSE`. The name and logo are trademarks of SerpDad, see `TRADEMARK.md`. Contributors sign the short agreement in `CLA.md` once, through a bot on their first pull request.

## Security

See `SECURITY.md`. Please do not open public issues for vulnerabilities.
