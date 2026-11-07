#!/usr/bin/env bash
# Builds everything GitHub Pages publishes for Khalid into one folder:
#
#   <mount>                      the website, site/
#   <mount>demo/                 the showcase site, fixtures/showcase-site
#   <mount>examples/<slug>/      each site in fixtures/examples/
#   <mount>templates/<name>/     the showcase once per template the binary has,
#                                or the template's ready demo publication
#                                (scripts/demo-publication.sh), in its
#                                sample brand (scripts/demo-brand.sh)
#
# Usage: scripts/pages.sh [options] <khalid binary> <output folder> [address] [mount]
# The address defaults to https://thebigasif.github.io and the mount to
# /khalid/. The output folder must not exist yet. Run from anywhere.
#
# The demos, templates and examples are sample blogs, so each is built with
# `[seo.index] site = "noindex"`: every page says noindex, follow, and the
# sitemaps list nothing. The website's own pages stay indexable.
#
#   --canonical <address>  every page names this address, with the same
#                          path, as its canonical one, such as
#                          https://serpdad.com for the GitHub Pages copy
#   --demos-only           leaves out the website's pages but the template
#                          gallery at <mount>templates/, and builds each
#                          template but classic once more at
#                          <mount>demos/<name>/, for the copy
#                          scripts/demos-bundle.sh packs for serpdad.com
set -euo pipefail

canonical=""
demos_only=""
while [ $# -gt 0 ]; do
  case "$1" in
    --canonical) canonical="${2:?Name the canonical address, such as https://serpdad.com.}"; canonical="${canonical%/}"; shift 2 ;;
    --demos-only) demos_only=1; shift ;;
    *) break ;;
  esac
done

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=demo-media.sh
source "$root/scripts/demo-media.sh"
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

# Drops the [template] table and its options from a site.toml. The call to
# action in [template.cta] stays, since every template takes it.
without_template() {
  awk '
    /^\[/ { skip = ($0 ~ /^\[template(\]|\.options\])/) }
    !skip
  ' "$1" >"$1.new"
  mv "$1.new" "$1"
}

# Prints the lines of the [template.options] table in the site.toml $1,
# without its heading, or nothing when it has none.
template_options() {
  awk '
    /^\[/ { in_options = ($0 == "[template.options]"); next }
    in_options && NF
  ' "$1"
}

# Keeps every page of the site whose site.toml is $1 out of search results
# and sitemaps, with the setting `[seo.index] site = "noindex"`.
noindex() {
  if grep -qx '\[seo\.index\]' "$1"; then
    sed -i '/^\[seo\.index\]$/a site = "noindex"' "$1"
  else
    printf '\n[seo.index]\nsite = "noindex"\n' >>"$1"
  fi
}

# Points the canonical and language links of every page in the built folder
# $1 at the same path on the --canonical address, so search engines credit
# that copy. Links between the pages stay as they are, so the copy here
# keeps working.
with_canonical() {
  find "$1" -type f -name '*.html' -exec \
    sed -i -E "s#(<link rel=\"(canonical|alternate)\"[^>]* href=\")${address%/}/#\1${canonical}/#g" {} +
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
  # A template demo shows the template's own publication, where it has a
  # ready one (scripts/demo-publication.sh), then wears its sample brand,
  # both before the address moves, so their own addresses move with it.
  if [ -n "$template" ]; then
    bash "$root/scripts/demo-publication.sh" "$site" "$template"
    bash "$root/scripts/demo-brand.sh" "$site" "$template"
  fi
  # Pages serves files only, so every copy is a static site: forms show a
  # note and the build lists the features that need a server.
  sed -i "s#^address = .*#address = \"$address\"#; s#^mount = .*#mount = \"$at\"#; s#^mode = .*#mode = \"static\"#" "$site/site.toml"
  # Menus and posts link to the site's own pages by path, such as
  # "/blog/about/", so each path follows the site to its new mount. A path
  # inside a full address, such as https://example.com/blog/, stays.
  local was origin
  was="$(sed -n 's#^mount = "\(.*\)"#\1#p' "$from/site.toml")"
  origin="$(sed -n 's#^address = "\(.*\)"#\1#p' "$from/site.toml")"
  origin="${origin%/}"
  if [ -n "$was" ] && [ "$was" != "$at" ]; then
    find "$site" -type f \( -name '*.md' -o -name '*.toml' \) -exec \
      sed -i -E "s#(^|[^A-Za-z0-9._~:/-])${was}#\1${at}#g" {} +
  fi
  # A full address of the site's own file, such as the logo in
  # theme/tokens.toml, follows the site to its new address and mount.
  if [ -n "$was" ] && [ -n "$origin" ]; then
    find "$site" -type f -name '*.toml' -exec \
      sed -i "s#${origin}${was}#${address}${at}#g" {} +
  fi
  if [ -n "$template" ]; then
    # The options a template's publication sets, such as its tagline, stay.
    # The showcase sets none, so a demo without a publication has none.
    local options
    options="$(template_options "$site/site.toml")"
    without_template "$site/site.toml"
    printf '\n[template]\nname = "%s"\n' "$template" >>"$site/site.toml"
    # Each demo shows the showcase's call to action as a band on its front
    # page and after each post, so every template shows off that part,
    # unless its publication chose otherwise. Classic has no options at all.
    if [ "$template" != "classic" ]; then
      printf '\n[template.options]\n' >>"$site/site.toml"
      if ! grep -q '^cta_band *=' <<<"$options"; then
        printf 'cta_band = "both"\n' >>"$site/site.toml"
      fi
      if [ -n "$options" ]; then
        printf '%s\n' "$options" >>"$site/site.toml"
      fi
    fi
  else
    local named
    named="$(awk '/^\[/ { in_t = ($0 == "[template]") } in_t && /^name = / { gsub(/"/, "", $3); print $3 }' "$site/site.toml")"
    if [ -n "$named" ] && ! grep -qx "$named" "$work/templates"; then
      echo "This Khalid has no ${named} template yet, so ${at} uses classic."
      without_template "$site/site.toml"
    fi
  fi
  if [ "$into" != "." ]; then
    noindex "$site/site.toml"
    # The demos offer their pictures in AVIF too, made once for them all.
    demo_media_in "$site"
  fi
  "$khalid" --site "$site" build
  demo_media_out "$site"
  # Project pages cannot serve files at the host root.
  rm -rf "$site/public/_root"
  if [ -n "$canonical" ]; then
    with_canonical "$site/public"
  fi
  if [ "$into" = "." ]; then
    cp -r "$site/public" "$out"
  else
    mkdir -p "$(dirname "$out/$into")"
    cp -r "$site/public" "$out/$into"
  fi
}

# The website's template gallery alone at <mount>templates/, for a copy
# without the website's other pages. Its menu leads to the product page at
# the mount and to the demos, any other link to the website's own pages
# leads to the mount, and its stylesheet and script move into
# templates/assets/, so the copy holds only demo/, templates/ and examples/.
gallery() {
  local src="$work/gallery-src" built="$work/gallery-out" keep="$out" file
  cp -r "$root/site" "$src"
  awk '/^\[/ { skip = ($0 ~ /^\[\[nav\.(primary|footer_groups)/) } !skip' "$src/site.toml" >"$src/site.toml.new"
  mv "$src/site.toml.new" "$src/site.toml"
  printf '\n[[nav.primary]]\nlabel = "Khalid"\nhref = "/khalid/"\n\n[[nav.primary]]\nlabel = "Templates"\nhref = "/khalid/templates/"\n\n[[nav.primary]]\nlabel = "Demo blog"\nhref = "/khalid/demo/"\n' >>"$src/site.toml"
  out="$built"
  build "$src" "$mount" "."
  out="$keep"
  mkdir -p "$out/templates/assets"
  cp "$built/templates/index.html" "$built/templates/"*.svg "$out/templates/"
  for file in $(grep -oE "\"${mount}assets/[^\"]+\"" "$built/templates/index.html" | tr -d '"' | sort -u); do
    cp "$built/${file#"$mount"}"* "$out/templates/assets/"
  done
  sed -i -E "s%\"${mount}assets/%\"${mount}templates/assets/%g; s%href=\"${mount}(demo|templates|examples)/%href=\"KEEP-MOUNT\1/%g; s%href=\"${mount}[^\"]+/(#[^\"]*)?\"%href=\"${mount}\"%g; s%KEEP-MOUNT%${mount}%g" "$out/templates/index.html"
}

templates >"$work/templates"
if [ ! -s "$work/templates" ]; then
  echo "This Khalid did not list its templates, so the template demos cannot be built. Check that $khalid runs."
  exit 1
fi
echo "Templates: $(paste -sd ' ' "$work/templates")"

mkdir -p "$(dirname "$out")"
if [ -n "$demos_only" ]; then
  mkdir -p "$out"
  gallery
else
  build "$root/site" "$mount" "."
fi
build "$root/fixtures/showcase-site" "${mount}demo/" "demo"
for example in "$root"/fixtures/examples/*/; do
  [ -f "$example/site.toml" ] || continue
  slug="$(basename "$example")"
  build "$example" "${mount}examples/$slug/" "examples/$slug"
done
for name in $(cat "$work/templates"); do
  build "$root/fixtures/showcase-site" "${mount}templates/$name/" "templates/$name" "$name"
  if [ -n "$demos_only" ] && [ "$name" != classic ]; then
    build "$root/fixtures/showcase-site" "${mount}demos/$name/" "demos/$name" "$name"
  fi
done
echo "Pages are ready in $out."
