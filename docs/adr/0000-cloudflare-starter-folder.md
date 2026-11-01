# 0000. The Cloudflare deploy is a starter folder that installs a pinned release

**Number:** next free. **Status:** Proposed, 2026-11-01. Plan 4.7 one-click deploys, founder request for a free Cloudflare host (KH-INS-1).

## Context

The founder wants a free, one-click Khalid blog on Cloudflare, by the button and by the GitHub plus Cloudflare Pages steps in their guide. Cloudflare's "Deploy to Cloudflare" button deploys Workers only, not Pages. It takes a GitHub or GitLab address, which may point into a subfolder; that subfolder must stand alone, and Cloudflare copies only it into a new repository in the person's account. The build and deploy commands come from `package.json` scripts, with `npx wrangler deploy` when there is no deploy script. Workers Builds and Pages both build on Ubuntu x86_64 with `curl`. A Worker with an `assets` folder and no `main` script serves the files alone, and reads `_redirects` and `_headers` there. Nothing of this can be tried from the build server, which has no Cloudflare account and no browser.

## Decision

- **A starter folder, not the engine repository.** `deploy/cloudflare/` is a whole site: `site.toml` in static mode at the host root, the default `routes.toml`, a welcome post, an about page and an author, in the classic template (the only template today). The button points at it with `/tree/main/deploy/cloudflare`, so a person gets a small repository of their own content, not the Khalid source.
- **The build installs a pinned release.** `build.sh` downloads `scripts/install.sh` from the same release tag and runs it with `--version` and `--dir .khalid/bin`, so the binary is checked against that release's `SHA256SUMS`, then runs `khalid build` into `public/`. `package.json` runs it as `npm run build` for the button; Pages runs `sh build.sh`. The pin is the workspace version, kept equal by a test, so the release bump updates it with every other version string.
- **Static assets only.** `wrangler.jsonc` names `./public` and `not_found_handling = "404-page"` for the build's `404.html`. The Worker has no code, so every request is a free static asset request, and the `_redirects` file the build writes for aliases (ADR 0031) works there too.
- **The address.** Cloudflare's address for a Worker includes the account's subdomain, which the build cannot know, and no build variable carries it. The starter keeps `https://blog.example.com`; internal links are relative, so the first deploy works, and the guide's first step is to set `address`, or the `KHALID_ADDRESS` build variable that `Config` already reads.
- **Pages.** The Pages steps use the same folder: a fork of this repository with the root directory `deploy/cloudflare`, or the repository the button made with no root directory. "Use this template" needs the repository marked as a template, or a separate starter repository; that is the founder's call and is listed in the pull request.

## Consequences

- A test builds the starter with the current binary and checks lint and audit are clean, and that the folder `wrangler.jsonc` serves is the one the build writes. Whether Cloudflare accepts the files is proven only by a real deploy.
- Between a version bump landing on `main` and its release being published, the button installs a release that is not there yet and the build fails. The release steps should publish soon after the bump.
- A starter blog keeps its pinned release until its owner changes the line in `build.sh`; the guide says how.
- The starter must stay readable by the pinned release, so a new `site.toml` field or front matter field goes into the starter only after a release carries it.
