#!/usr/bin/env bash
# The picture review: full-page pictures of each template's demo, for the
# founder to approve a design from. For each pack, the home page, the post
# /content-benchmark/ and /category/content-strategy/ at 2000, 1440 and
# 390 px, in the pack's sample brand and with the pack's own defaults and no
# theme, beside stored pictures of reference blogs of the same kind. One
# sheet per template, an HTML page and a PNG of it, goes to the output
# folder with every picture. scripts/demo-check/pictures.mjs takes them and
# says how a reference picture is placed by its file name.
#
# Needs Node 22 or later and Playwright with Chromium, as the layout check:
#   npm install --prefix scripts/demo-check
#   npx --prefix scripts/demo-check playwright install --with-deps chromium
#
# Usage: scripts/demo-pictures.sh --out <folder> [--packs keystone,wire]
#                                 [--refs <folder>] [--widths 2000,1440,390]
# Keep --out outside the repository: the pictures are for review, never
# committed. --refs holds one folder of reference pictures per pack.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
tools="$root/scripts/demo-check"
work="$root/target/demo-pictures"
# shellcheck source=demo-check/showcase.sh
source "$tools/showcase.sh"
# The review shows every sample brand, ready or not (scripts/demo-brand.sh).
export DEMO_BRAND_ALL=1

only=""
args=("$@")
while [ $# -gt 0 ]; do
  case "$1" in
    --packs) only="${2:-}"; shift 2 ;;
    *) shift ;;
  esac
done
case " ${args[*]} " in
  *" --out "*) ;;
  *) stop "Name the folder the pictures go to with --out, outside the repository, then run this again." ;;
esac
command -v node >/dev/null || stop "Node is missing. The pictures are taken with it. Install Node 22 or later, then run this again."
(cd "$tools" && node -e 'require("playwright")' 2>/dev/null) || stop "Playwright is not installed. The browser comes from it. Run: npm install --prefix scripts/demo-check"

# A release binary handed over as KHALID_BIN is used as it is, as the
# layout check does. Without it, the binary is built here.
if [ -n "${KHALID_BIN:-}" ]; then
  [ -x "$KHALID_BIN" ] || stop "KHALID_BIN is $KHALID_BIN, which is not a program these pictures can be taken with. Point it at a built khalid, or unset it to build one."
  bin="$KHALID_BIN"
else
  cargo build --release -p khalid
  bin="${CARGO_TARGET_DIR:-$root/target}/release/khalid"
fi
rm -rf "$work"
mkdir -p "$work/sites"
packs="$(showcase_packs "$bin" "$work")"
[ -n "$packs" ] || stop "This Khalid did not list its templates, so there is nothing to picture. Check that $bin runs."
for pack in $packs; do
  if [ -n "$only" ] && [[ ",$only," != *",$pack,"* ]]; then continue; fi
  showcase_start "$bin" "$work" "$pack" "$pack" brand
  showcase_start "$bin" "$work" "$pack-plain" "$pack" plain
done
showcase_finish

node "$tools/pictures.mjs" --sites "$work/sites" "${args[@]}"
