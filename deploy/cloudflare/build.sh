#!/bin/sh
# Builds this blog on Cloudflare: installs one Khalid release, then writes
# the pages to public/, the folder wrangler.jsonc serves. Workers Builds
# runs it as `npm run build`; Cloudflare Pages runs it as `sh build.sh`.
#
# The release is pinned, so a new Khalid never changes the blog by
# surprise. The install script checks the download against the release's
# SHA256SUMS before it runs. To update, change the version below to a
# newer release from https://github.com/thebigasif/khalid/releases.
set -eu

version="v1.1.1"
installer="https://raw.githubusercontent.com/thebigasif/khalid/$version/scripts/install.sh"

here=$(cd "$(dirname "$0")" && pwd)
bin="$here/.khalid/bin"

mkdir -p "$bin"
if ! curl -fsSL -o "$here/.khalid/install.sh" "$installer"; then
  printf 'The blog was not built. The Khalid installer for %s could not be downloaded.\n' "$version" >&2
  printf 'Check that %s is on https://github.com/thebigasif/khalid/releases, then build again.\n' "$version" >&2
  exit 1
fi
sh "$here/.khalid/install.sh" --version "$version" --dir "$bin"
"$bin/khalid" --site "$here" build
