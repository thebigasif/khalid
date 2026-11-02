#!/usr/bin/env bash
# Builds the demos SerpDad serves under https://serpdad.com/khalid/ and
# packs them as khalid-demos.tar.gz, with paths relative to /khalid/:
#
#   demo/                the showcase site, fixtures/showcase-site
#   templates/           the template gallery, its pictures and its assets
#   templates/<name>/    the showcase once per template the binary has
#   examples/<slug>/     each site in fixtures/examples/
#
# serpdad.com/khalid/ itself is SerpDad's product page, so the website's
# other pages are left out. Every demo page says noindex, follow and every
# demo sitemap is empty; the gallery stays indexable, with a canonical
# link to itself. scripts/pages.sh does the building.
#
# Usage: scripts/demos-bundle.sh <khalid binary> <output folder> [pictures folder]
#
# The tree is built in <output folder>, which must not exist yet, and the
# archive is written beside it as khalid-demos.tar.gz. The pictures folder
# holds gallery pictures scripts/template-previews.sh already drew, such as
# the templates folder of the GitHub Pages build. Without it they are drawn
# here, as plain cards where no Chromium runs.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
khalid="${1:?Name the khalid binary to build with.}"
out="${2:?Name a new folder to build the demos in.}"
pictures="${3:-}"

bash "$root/scripts/pages.sh" --demos-only "$khalid" "$out" https://serpdad.com /khalid/
if [ -n "$pictures" ]; then
  cp "$pictures"/*-home.svg "$pictures"/*-post.svg "$out/templates/"
else
  bash "$root/scripts/template-previews.sh" "$out" /khalid/
fi
tarball="$(cd "$(dirname "$out")" && pwd)/khalid-demos.tar.gz"
tar --sort=name --owner=0 --group=0 --numeric-owner -czf "$tarball" -C "$out" demo templates examples
echo "The demos for https://serpdad.com/khalid/ are packed in $tarball."
