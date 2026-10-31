# Changelog fragments

Each pull request that changes behaviour adds one file here instead of editing `CHANGELOG.md`, so parallel pull requests never conflict on it.

- Name: `<branch-slug>.md`, for example `p1-blocks-embed.md`.
- Content: the section on the first line (`Added`, `Changed`, `Fixed` or `Removed`), then the line for `CHANGELOG.md`, written for people who use Khalid.

The CTO folds the fragments into `CHANGELOG.md` under Unreleased when the pull request lands, and deletes them.

At a release, the release pull request moves what Unreleased and the remaining fragments hold into the version's own section, grouped by Added, Changed, Fixed and Security and then by area, rewritten in plain words where needed, and deletes the fragments it folded in.
