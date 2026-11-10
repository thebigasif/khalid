# The showcase site built once per template pack, for scripts/layout-check.sh
# and scripts/demo-pictures.sh. Sourced, not run. Each copy is static, as
# GitHub Pages serves the demos, and lives at its own address,
# http://<name>.localhost, which the browser scripts serve from its files.

showcase_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
showcase_site="$showcase_root/fixtures/showcase-site"
# shellcheck source=../demo-media.sh
source "$showcase_root/scripts/demo-media.sh"

stop() { echo "$1"; exit 1; }

# showcase_packs <khalid> <work>: every template the binary has, read from
# its own answer to a template it does not have, as scripts/pages.sh does.
showcase_packs() {
  mkdir -p "$2/probe"
  cp "$showcase_site/site.toml" "$2/probe/site.toml"
  printf '\n[template]\nname = "no-such-template"\n' >>"$2/probe/site.toml"
  "$1" --json --site "$2/probe" lint 2>/dev/null |
    sed -n 's/.*use one of: \([a-z0-9, -]*\)\..*/\1/p' | tr ',' ' ' || true
}

# showcase_publications <pack>...: each pack whose demo publication
# applies, as scripts/demo-publication.sh decides: it has one, and it is
# ready or DEMO_PUBLICATION_ALL is 1.
showcase_publications() {
  local folder="${DEMO_PUBLICATIONS:-$showcase_root/fixtures/showcase-publications}" pack
  for pack in "$@"; do
    [ -f "$folder/$pack/site.toml" ] || continue
    [ -f "$folder/$pack/ready" ] || [ "${DEMO_PUBLICATION_ALL:-}" = 1 ] || continue
    echo "$pack"
  done
}

# showcase_build <khalid> <work> <name> <pack> <look> [contents] [text]:
# the showcase with the pack in <work>/sites/<name>. <look> is "brand" for
# the pack's sample brand, by scripts/demo-brand.sh, or "plain" for the
# pack's own defaults with no theme at all. [contents] places the contents
# list. [text] "publication" puts the pack's demo publication in place of
# the showcase's posts first, by scripts/demo-publication.sh, which leaves
# the showcase as it is for a pack without one.
showcase_build() {
  local bin="$1" work="$2" name="$3" pack="$4" look="$5" contents="${6:-}" text="${7:-}"
  local copy="$work/sites/$name" address="http://$name.localhost"
  cp -r "$showcase_site" "$copy"
  rm -rf "$copy/public" "$copy/data"
  if [ "$text" = "publication" ]; then
    bash "$showcase_root/scripts/demo-publication.sh" "$copy" "$pack" >"$work/publication-$name.log" 2>&1 ||
      stop "The $pack demo publication could not be put in place: $(tail -n 1 "$work/publication-$name.log")"
  fi
  if [ "$look" = "plain" ]; then
    rm -rf "$copy/theme"
  else
    bash "$showcase_root/scripts/demo-brand.sh" "$copy" "$pack"
  fi
  # The announcement links back to the template's page, as on Pages.
  bash "$showcase_root/scripts/demo-backlink.sh" "$copy" "$pack"
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
  # Pictures in AVIF too, as the published demos offer them.
  demo_media_in "$copy"
  "$bin" --site "$copy" build --out "$copy/public" >"$work/build-$name.log" 2>&1 ||
    stop "The showcase did not build with $pack. Its output is in $work/build-$name.log. Fix what it says, then run this again."
  demo_media_out "$copy"
}

# Four builds at a time with showcase_start; each runs in its own shell, so
# a failed one stops only itself, and showcase_finish waits for them all
# and stops when one failed.
showcase_pids=()
showcase_start() {
  while [ "$(jobs -rp | wc -l)" -ge 4 ]; do wait -n || true; done
  showcase_build "$@" &
  showcase_pids+=("$!")
}
showcase_finish() {
  for pid in "${showcase_pids[@]}"; do wait "$pid" || exit 1; done
  showcase_pids=()
}
