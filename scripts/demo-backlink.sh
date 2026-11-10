#!/usr/bin/env bash
# The link back from a template's demo to the template's own page on
# serpdad.com, where a visitor reads how to use it. It becomes the demo's
# announcement, the line above the header on every page, in place of the
# showcase's or the publication's own, and reads:
#
#   You are viewing <Template>, a free Khalid template. See how to use it.
#
# The line names no language, so every language of the demo shows it.
# It runs after scripts/demo-publication.sh, so a publication's own
# announcement gives way to it. scripts/pages.sh and
# scripts/demo-check/showcase.sh call it.
#
# Usage: scripts/demo-backlink.sh <copy of the showcase site> <template>
#
#   DEMO_TEMPLATE_PAGES   where the template pages live, by default
#                         https://serpdad.com/khalid/templates/
set -euo pipefail

copy="${1:?Name the copy of the showcase site to change, then the template.}"
template="${2:?Name the template whose page the demo links to.}"
pages="${DEMO_TEMPLATE_PAGES:-https://serpdad.com/khalid/templates/}"
toml="$copy/site.toml"

[ -f "$toml" ] || {
  echo "$copy holds no site.toml, so it is not a copy of the showcase site. Name the copy the build makes, then run this again."
  exit 1
}

# The template's name with a capital first letter, as the gallery writes it.
title="$(printf '%s' "${template:0:1}" | tr '[:lower:]' '[:upper:]')${template:1}"

# Drops the [announcement] table, its heading and its keys, then adds the
# link back at the end.
awk '
  /^\[/ { skip = ($0 == "[announcement]") }
  !skip
' "$toml" >"$toml.new"
mv "$toml.new" "$toml"
printf '\n[announcement]\ntext = "You are viewing %s, a free Khalid template. See how to use it."\nhref = "%s%s/"\n' \
  "$title" "$pages" "$template" >>"$toml"
