# Common tasks. `just` is the one tool to learn. CI runs the same recipes.

set shell := ["bash", "-euo", "pipefail", "-c"]

# Everything CI checks, in the same order.
check:
    ./scripts/check.sh

# Format all Rust and TOML.
fmt:
    cargo fmt --all
    taplo fmt

# Clippy with warnings as errors on every target and feature.
clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Tests with nextest, then doc tests.
test *ARGS:
    cargo nextest run --workspace --all-features {{ARGS}}
    cargo test --workspace --doc

# Docs with warnings as errors.
doc:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features

# Dependency audit: advisories, licences, bans, sources.
deny:
    cargo deny check

# Package khalid-core and khalid-lint and build them from the packages, as
# crates.io would. Uploads nothing.
publish-check:
    cargo publish --dry-run --allow-dirty --locked -p khalid-core -p khalid-lint

# Prove every lint in lints/lints.toml fires.
lints:
    ./scripts/check-lints.sh

# Prove that removing any one lint from the table makes `just lints` fail.
prove-lints:
    ./scripts/prove-lint-table.sh

# Benchmarks against the baseline.
bench:
    ./scripts/perf.sh

# The demo site through the validator, Lighthouse and axe at six widths, with
# scripts on and off. Runs on a server or in CI, never on a laptop.
demo-check:
    ./scripts/demo-check.sh

# The showcase in every template pack, in Chromium, Firefox and WebKit at
# 390, 768, 1440 and 2000 px: text over text or a picture, markers over
# text, sideways scroll, crushed table columns, broken pictures, a frame
# narrower than the template's brief, post text too small or lines too
# short or long, and stylesheets of 20 KB or more. Pass options on, such
# as `just layout-check --browsers chromium,firefox`.
layout-check *options:
    ./scripts/layout-check.sh {{options}}

# Proves the layout check's frame, reading and stylesheet checks on small
# sites with a planted fault each, in Chromium.
layout-selftest:
    node scripts/demo-check/layout-selftest.mjs

# Proves the layout check's network capture finds a live player frame and
# a live script, and nothing where they wait for consent, in Chromium.
network-selftest:
    node scripts/demo-check/network-selftest.mjs

# Proves the demo publication step and the layout check's second pass on
# the sample publication in fixtures/test-publications/, with no browser.
publication-selftest:
    bash scripts/demo-check/publication-selftest.sh

# Proves the map scripts/pages-forward.sh uses to send each old GitHub
# Pages address to serpdad.com, with no network.
pages-forward-selftest:
    bash scripts/pages-forward.sh --selftest

# Ghost's own Content API client, at the version
# scripts/ghost-client/package.json pins, against the showcase with
# Ghost's Content API switched on (docs/ghost-api.md).
ghost-client:
    ./scripts/ghost-client.sh

# Full-page pictures of each template's demo at 2000, 1440 and 390 px, as
# one sheet per template, for the founder to approve. Pass --out, outside
# the repository, such as `just demo-pictures --out ~/shots --packs wire`.
demo-pictures *options:
    ./scripts/demo-pictures.sh {{options}}

# Line coverage against the floor.
coverage:
    cargo llvm-cov nextest --workspace --all-features --summary-only

# Refresh .sqlx/, the checked query shapes CI compiles against, after changing
# a query or adding a migration. Needs sqlx-cli with the sqlite feature.
db-prepare:
    rm -f target/sqlx-prepare.db
    DATABASE_URL=sqlite:target/sqlx-prepare.db sqlx database create
    DATABASE_URL=sqlite:target/sqlx-prepare.db sqlx migrate run --source crates/khalid-db/migrations
    DATABASE_URL=sqlite:target/sqlx-prepare.db cargo sqlx prepare --workspace -- --all-targets --all-features

# Run the binary in a scratch site with a demo fixture.
dev:
    cargo run -p khalid -- serve --site fixtures/demo-site
