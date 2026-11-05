#!/usr/bin/env bash
# The sample-brand step: dresses a copy of the showcase site in a template's
# sample brand, its colours, logo and fonts, so its demo looks like a real
# blog's. The brand is fixtures/showcase-brands/<pack>/theme/, copied over
# the copy's theme/ folder; a file of the same name is replaced. A template
# without a sample brand keeps the showcase's own theme, and this does
# nothing. scripts/pages.sh, scripts/layout-check.sh and
# scripts/demo-pictures.sh all call it, so the pictures the founder approves
# show what the checks test and what GitHub Pages serves.
#
# Usage: scripts/demo-brand.sh <copy of the showcase site> <pack>
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
copy="${1:?Name the copy of the showcase site to dress, then the template.}"
pack="${2:?Name the template whose sample brand to use.}"
brand="$root/fixtures/showcase-brands/$pack/theme"

[ -d "$brand" ] || exit 0
# A brand dresses the Pages demo and the layout check only once its
# template's redesign has landed, which adds the file "ready" beside
# theme/. The picture review sets DEMO_BRAND_ALL=1 to show every brand.
[ -f "$brand/../ready" ] || [ "${DEMO_BRAND_ALL:-}" = 1 ] || exit 0
if [ ! -f "$copy/site.toml" ]; then
  echo "$copy holds no site.toml, so it is not a copy of the showcase site. Name the copy the build makes, then run this again."
  exit 1
fi
mkdir -p "$copy/theme"
cp -R "$brand/." "$copy/theme/"
# The brand names its logo at the about page's address, so the logo goes
# there too, in place of the showcase's own.
if [ -f "$brand/logo.svg" ]; then
  cp "$brand/logo.svg" "$copy/content/en/pages/about/logo.svg"
  rm -f "$copy/theme/logo.svg"
  # A logo spells the showcase's name. Where a demo publication has put its
  # own name in site.toml, the logo spells that one, its letters spaced to
  # the same width per letter.
  python3 -I - "$copy/site.toml" "$copy/content/en/pages/about/logo.svg" <<'PY'
import html
import re
import sys
import tomllib

site, logo = sys.argv[1], sys.argv[2]
with open(site, "rb") as file:
    name = tomllib.load(file).get("name", "")
svg = open(logo, encoding="utf-8").read()
old = "Larkspur Ledger"
if name and name != old and old in svg:
    def fit(match):
        width = float(match.group(1)) * len(name) / len(old)
        return f'textLength="{min(width, 250):g}"'
    svg = re.sub(r'textLength="([0-9.]+)"', fit, svg, count=1)
    svg = svg.replace(old, html.escape(name))
    open(logo, "w", encoding="utf-8").write(svg)
PY
fi
