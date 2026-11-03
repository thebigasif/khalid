#!/usr/bin/env bash
# The layout check: the showcase site once per template pack, opened in
# Chromium, Firefox and WebKit at 390, 768 and 1440 px. It fails on text
# over text or a picture, a list marker over its text, a page that scrolls
# sideways, a table column narrower than its longest word and a picture
# that did not load. scripts/demo-check/layout.mjs does the checking and
# says what each check means.
#
# Each pack is built with its own options, then once more with the contents
# list in a side column, where the pack's own default puts it elsewhere, so
# the side column is checked in every pack. The second build checks posts
# only. Each site is static, as GitHub Pages serves the demos.
#
# Needs Node 22 or later and the pinned tools, with the browsers:
#   npm install --prefix scripts/demo-check
#   npx --prefix scripts/demo-check playwright install --with-deps chromium firefox webkit
#
# Usage: scripts/layout-check.sh [--browsers chromium,firefox] [--packs keystone,wire]
#                                [--widths 390,768,1440] [--shots <folder>]
# The options go to layout.mjs as they are. A screenshot of each failing
# page, around its first problem, goes to --shots, by default
# target/layout-check/shots, and every failure to target/layout-check/report.json.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
site="fixtures/showcase-site"
tools="$root/scripts/demo-check"
work="$root/target/layout-check"

stop() { echo "$1"; exit 1; }
command -v node >/dev/null || stop "Node is missing. The browser checks run on it. Install Node 22 or later, then run this again."
(cd "$tools" && node -e 'require("playwright")' 2>/dev/null) || stop "Playwright is not installed. The browsers come from it. Run: npm install --prefix scripts/demo-check"

cargo build --release -p khalid
bin="${CARGO_TARGET_DIR:-$root/target}/release/khalid"
rm -rf "$work"
since=$SECONDS
mkdir -p "$work/sites"

# Every template this binary has, one per line, read from its own answer to
# a template it does not have, as scripts/pages.sh reads it.
mkdir -p "$work/probe"
cp "$site/site.toml" "$work/probe/site.toml"
printf '\n[template]\nname = "no-such-template"\n' >>"$work/probe/site.toml"
packs="$("$bin" --json --site "$work/probe" lint 2>/dev/null |
  sed -n 's/.*use one of: \([a-z0-9, -]*\)\..*/\1/p' | tr ',' ' ' || true)"
[ -n "$packs" ] || stop "This Khalid did not list its templates, so there is nothing to check. Check that $bin runs."

# build <name> <pack> [contents]: the showcase with the pack, at its own
# address, http://<name>.localhost, which layout.mjs serves from the files.
build() {
  local name="$1" pack="$2" contents="${3:-}" copy="$work/sites/$1"
  local address="http://$name.localhost"
  cp -r "$site" "$copy"
  rm -rf "$copy/public" "$copy/data"
  # The address moves, and so does every full address of the site's own
  # files, such as the logo in theme/tokens.toml.
  find "$copy" -type f -name '*.toml' -exec sed -i "s#https://www\.example\.com\([/\"]\)#$address\1#g" {} +
  sed -i 's#^mode = .*#mode = "static"#' "$copy/site.toml"
  # Drops the showcase's [template] table and its options, as pages.sh does.
  awk '/^\[/ { skip = ($0 ~ /^\[template(\]|\.options\])/) } !skip' "$copy/site.toml" >"$copy/site.toml.new"
  mv "$copy/site.toml.new" "$copy/site.toml"
  printf '\n[template]\nname = "%s"\n' "$pack" >>"$copy/site.toml"
  if [ -n "$contents" ]; then
    printf '\n[template.options]\ncontents = "%s"\n' "$contents" >>"$copy/site.toml"
  fi
  "$bin" --site "$copy" build --out "$copy/public" >"$work/build-$name.log" 2>&1 ||
    stop "The showcase did not build with $pack. Its output is in $work/build-$name.log. Fix what it says, then run this again."
}

# Four builds at a time; each runs in its own shell, so a failed one stops
# only itself, and waiting on it stops the check.
pids=()
start() {
  while [ "$(jobs -rp | wc -l)" -ge 4 ]; do wait -n || true; done
  build "$@" &
  pids+=("$!")
}
finish() {
  for pid in "${pids[@]}"; do wait "$pid" || exit 1; done
  pids=()
}
for pack in $packs; do start "$pack" "$pack"; done
finish
# A pack whose posts show no side column gets a second build with one.
for pack in $packs; do
  if ! grep -q 'class="post post-with-toc' "$work/sites/$pack/public/ai-search-guide/index.html"; then
    start "$pack-side-contents" "$pack" side-end
  fi
done
finish
echo "Built the showcase with $(echo "$packs" | wc -w) packs in $((SECONDS - since)) seconds."

node "$tools/layout.mjs" --sites "$work/sites" --shots "$work/shots" "$@"
