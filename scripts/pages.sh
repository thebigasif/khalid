#!/usr/bin/env bash
# Builds everything GitHub Pages publishes for Khalid into one folder:
#
#   <mount>                      the website, site/
#   <mount>demo/                 the showcase site, fixtures/showcase-site
#   <mount>examples/<slug>/      each site in fixtures/examples/
#   <mount>templates/<name>/     the showcase once per template the binary has
#
# Usage: scripts/pages.sh <khalid binary> <output folder> [address] [mount]
# The address defaults to https://thebigasif.github.io and the mount to
# /khalid/. The output folder must not exist yet. Run from anywhere.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
khalid="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
out="$2"
address="${3:-https://thebigasif.github.io}"
mount="${4:-/khalid/}"

if [ -e "$out" ]; then
  echo "The output folder $out already exists. Pages are built into a new folder, so nothing old is published. Remove it or name another folder."
  exit 1
fi
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

# Every template this binary has, one per line, read from its own answer to
# a template it does not have: "... use one of: classic, harbor, wire."
templates() {
  mkdir -p "$work/probe"
  cp "$root/fixtures/showcase-site/site.toml" "$work/probe/site.toml"
  printf '\n[template]\nname = "no-such-template"\n' >>"$work/probe/site.toml"
  "$khalid" --json --site "$work/probe" lint 2>/dev/null |
    sed -n 's/.*use one of: \([a-z0-9, -]*\)\..*/\1/p' |
    tr ',' '\n' | tr -d ' ' | sed '/^$/d' || true
}

# Drops the [template] table and its sub-tables from a site.toml.
without_template() {
  awk '
    /^\[/ { skip = ($0 ~ /^\[template(\]|\.)/) }
    !skip
  ' "$1" >"$1.new"
  mv "$1.new" "$1"
}

# Builds the site folder $1 at the mount $2 into $out/$3, with the template
# $4 when given. A site that names a template this binary lacks is built
# with classic, and says so.
build() {
  local from="$1" at="$2" into="$3" template="${4:-}"
  local site
  site="$work/site-$(echo "$into" | tr '/' '-')"
  echo "==> ${at}"
  cp -r "$from" "$site"
  rm -rf "$site/public" "$site/data"
  # Pages serves files only, so every copy is a static site: forms show a
  # note and the build lists the features that need a server.
  sed -i "s#^address = .*#address = \"$address\"#; s#^mount = .*#mount = \"$at\"#; s#^mode = .*#mode = \"static\"#" "$site/site.toml"
  # Menus and posts link to the site's own pages by path, such as
  # "/blog/about/", so each path follows the site to its new mount. A path
  # inside a full address, such as https://example.com/blog/, stays.
  local was
  was="$(sed -n 's#^mount = "\(.*\)"#\1#p' "$from/site.toml")"
  if [ -n "$was" ] && [ "$was" != "$at" ]; then
    find "$site" -type f \( -name '*.md' -o -name '*.toml' \) -exec \
      sed -i -E "s#(^|[^A-Za-z0-9._~:/-])${was}#\1${at}#g" {} +
  fi
  if [ -n "$template" ]; then
    without_template "$site/site.toml"
    printf '\n[template]\nname = "%s"\n' "$template" >>"$site/site.toml"
  else
    local named
    named="$(awk '/^\[/ { in_t = ($0 == "[template]") } in_t && /^name = / { gsub(/"/, "", $3); print $3 }' "$site/site.toml")"
    if [ -n "$named" ] && ! grep -qx "$named" "$work/templates"; then
      echo "This Khalid has no ${named} template yet, so ${at} uses classic."
      without_template "$site/site.toml"
    fi
  fi
  "$khalid" --site "$site" build
  # Project pages cannot serve files at the host root.
  rm -rf "$site/public/_root"
  if [ "$into" = "." ]; then
    cp -r "$site/public" "$out"
  else
    mkdir -p "$(dirname "$out/$into")"
    cp -r "$site/public" "$out/$into"
  fi
}

templates >"$work/templates"
if [ ! -s "$work/templates" ]; then
  echo "This Khalid did not list its templates, so the template demos cannot be built. Check that $khalid runs."
  exit 1
fi
echo "Templates: $(paste -sd ' ' "$work/templates")"

mkdir -p "$(dirname "$out")"
build "$root/site" "$mount" "."
build "$root/fixtures/showcase-site" "${mount}demo/" "demo"
for example in "$root"/fixtures/examples/*/; do
  [ -f "$example/site.toml" ] || continue
  slug="$(basename "$example")"
  build "$example" "${mount}examples/$slug/" "examples/$slug"
done
for name in $(cat "$work/templates"); do
  build "$root/fixtures/showcase-site" "${mount}templates/$name/" "templates/$name" "$name"
done
echo "Pages are ready in $out."
