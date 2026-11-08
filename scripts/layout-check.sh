#!/usr/bin/env bash
# The layout check: the showcase site once per template pack, in the pack's
# sample brand, opened in Chromium, Firefox and WebKit at 390, 768, 1440 and
# 2000 px. It fails on text over text or a picture, a list marker over its
# text, a page that scrolls sideways, a table column narrower than its
# longest word, a picture that did not load, a frame narrower than the
# template's brief on a wide screen, post text too small or lines too short
# or long on a wide screen, and a page whose stylesheets weigh more than
# 10 KB together with Brotli.
# scripts/demo-check/layout.mjs does the checking and says what each check
# means, and each pack's pack.toml in crates/khalid-theme/packs/ holds its
# minimum frame.
#
# Each pack is built with its own options, then once more with the contents
# list in a side column, where the pack's own default puts it elsewhere, so
# the side column is checked in every pack. The second build checks posts
# only. Each site is static, as GitHub Pages serves the demos.
#
# A second pass checks each template's ready demo publication
# (fixtures/showcase-publications/, scripts/demo-publication.sh) in its own
# pack, as <pack>-publication, on the pages its check-pages.txt names. The
# showcase pass above stays the floor every pack is held to.
#
# Last, scripts/demo-check/network.mjs opens each site's pages in Chromium
# with no consent given, scripts on and then off, and fails on any request
# to an address other than the site's own (privacy plan R1).
#
# Needs Node 22 or later and the pinned tools, with the browsers:
#   npm install --prefix scripts/demo-check
#   npx --prefix scripts/demo-check playwright install --with-deps chromium firefox webkit
#
# Usage: scripts/layout-check.sh [--browsers chromium,firefox] [--packs keystone,wire]
#                                [--widths 390,768,1440,2000] [--shots <folder>]
#                                [--frames <file>]
# The options go to layout.mjs as they are. A screenshot of each failing
# page, around its first problem, goes to --shots, by default
# target/layout-check/shots, and every failure to target/layout-check/report.json.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
tools="$root/scripts/demo-check"
work="$root/target/layout-check"
# shellcheck source=demo-check/showcase.sh
source "$tools/showcase.sh"

command -v node >/dev/null || stop "Node is missing. The browser checks run on it. Install Node 22 or later, then run this again."
(cd "$tools" && node -e 'require("playwright")' 2>/dev/null) || stop "Playwright is not installed. The browsers come from it. Run: npm install --prefix scripts/demo-check"

# CI hands over the release binary it already built as KHALID_BIN, so this
# check does not build it again. Without it, the binary is built here.
if [ -n "${KHALID_BIN:-}" ]; then
  [ -x "$KHALID_BIN" ] || stop "KHALID_BIN is $KHALID_BIN, which is not a program this check can run. Point it at a built khalid, or unset it to build one."
  bin="$KHALID_BIN"
else
  cargo build --release -p khalid
  bin="${CARGO_TARGET_DIR:-$root/target}/release/khalid"
fi
rm -rf "$work"
since=$SECONDS
mkdir -p "$work/sites"

packs="$(showcase_packs "$bin" "$work")"
[ -n "$packs" ] || stop "This Khalid did not list its templates, so there is nothing to check. Check that $bin runs."
# With --packs, only those packs are built, as only they are checked.
only=""
previous=""
for argument in "$@"; do
  [ "$previous" = "--packs" ] && only="$argument"
  previous="$argument"
done
if [ -n "$only" ]; then
  packs="$(for pack in $packs; do [[ ",$only," == *",$pack,"* ]] && echo "$pack"; done || true)"
  [ -n "$packs" ] || stop "None of the packs named in --packs ($only) is a template this Khalid has. Name packs it lists."
fi

for pack in $packs; do showcase_start "$bin" "$work" "$pack" "$pack" brand; done
showcase_finish
# A pack whose posts show no side column gets a second build with one.
for pack in $packs; do
  if ! grep -q 'class="post post-with-toc' "$work/sites/$pack/public/ai-search-guide/index.html"; then
    showcase_start "$bin" "$work" "$pack-side-contents" "$pack" brand side-end
  fi
done
showcase_finish
# The second pass: each ready demo publication in its own pack, on the
# pages its check-pages.txt names. With none ready it builds nothing.
# shellcheck disable=SC2086 # one pack per word
publications="$(showcase_publications $packs)"
for pack in $publications; do showcase_start "$bin" "$work" "$pack-publication" "$pack" brand "" publication; done
showcase_finish
echo "Built the showcase with $(echo "$packs" | wc -w) packs and $(echo "$publications" | wc -w) demo publications in $((SECONDS - since)) seconds."

failed=0
node "$tools/layout.mjs" --sites "$work/sites" --shots "$work/shots" "$@" || failed=1
# The network capture: with no consent given, scripts on and off, no page
# may ask another address for anything (scripts/demo-check/network.mjs).
node "$tools/network.mjs" --sites "$work/sites" "$@" || failed=1
exit "$failed"
