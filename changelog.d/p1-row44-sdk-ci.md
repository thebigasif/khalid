Added
- The TypeScript SDK now has a `package-lock.json`, so `npm ci` installs the exact Biome, TypeScript and Node types it is checked with. CI runs Biome and the TypeScript compiler on the SDK on every pull request, and a Rust test fails when the lockfile no longer matches `package.json`.
- CI now runs `cargo publish --dry-run` for `khalid-core` and `khalid-lint`, which only the local `scripts/check.sh` ran before, so a crate that cannot be packaged alone is caught before release.
