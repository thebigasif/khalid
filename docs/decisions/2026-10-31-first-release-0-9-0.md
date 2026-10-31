# The first public release is 0.9.0, 2026-10-31

The 1.0 readiness check (`khalid-ops/ready-1.0.md`, finding 1) found that the README's install line cannot work: there is no GitHub release, so `scripts/install.sh` has nothing to download. The CTO decided how the first release is cut, under the founder's standing yes of 2026-10-27.

## The decision

- **The first release is 0.9.0.** Most of the readiness findings are fixed, but not all of them, and the speed, memory and demo checks are not yet green on one build. 0.9.0 says "nearly 1.0": everything is in, and a few known gaps remain, listed in `docs/releases/0.9.0.md`.
- **It is a full release, not a pre-release.** `scripts/install.sh` asks GitHub for `/releases/latest`, and GitHub leaves pre-releases out of that answer. A pre-release would leave the install line as broken as it is today. The tag is therefore `v0.9.0`, with no `-rc` or `-beta` part, because `.github/workflows/release.yml` marks any tag with a `-` as a pre-release.
- **1.0.0 follows** once the readiness report's remaining items are closed and the performance budgets and the demo check are green on the 1.0 candidate.

## What the version covers

- Every crate in the workspace takes its version from `[workspace.package]` in `Cargo.toml`, so `khalid --version` prints `khalid 0.9.0`. The TypeScript SDK, the admin's `package.json` and the API's OpenAPI document carry the same number.
- `khalid-core` and `khalid-lint` keep their own version, 0.1.0. They are the two crates published to crates.io, with their own API promise (ADR "the lint crates are published under the AGPL"); their number moves when their API does, and reaches 1.0.0 with the product's 1.0.
- `CHANGELOG.md` gets a `## 0.9.0` section that holds every change so far, and `changelog.d/` is emptied into it. New fragments after this go under `## Unreleased` as before.

## Who does what

- Agents prepare the release: version, changelog, release notes. They never push a tag and never publish a release.
- The founder, or the CTO session on the founder's word, pushes the tag `v0.9.0` on `main` once the release pull request has landed. The release workflow then builds every platform and opens a **draft** release.
- The founder reads the draft, pastes `docs/releases/0.9.0.md` as its text if wanted, and presses Publish. Only a published release counts as `/releases/latest`, so the install line works from that moment.
