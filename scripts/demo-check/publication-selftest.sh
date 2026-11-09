#!/usr/bin/env bash
# Proves scripts/demo-publication.sh and the layout check's second pass on
# the sample publication in fixtures/test-publications/, with no browser
# and no network. crates/khalid/tests/demo_publication.rs builds the
# sample's demo; this covers the rest: the photo cache, a photo the
# archive lacks, a photo outside content/, the settings a publication
# always replaces, which publications the second pass checks, and how
# check-pages.txt is read.
#
# Usage: scripts/demo-check/publication-selftest.sh, from anywhere. Exits 1
# when a case goes wrong, and names it.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
work="$root/target/publication-selftest"
sample="$root/fixtures/test-publications"
rm -rf "$work"
mkdir -p "$work"
wrong=0

fail() { echo "WRONG: $1"; wrong=$((wrong + 1)); }

# overlay <publications> <photos url> <cache> <copy>: the script on a
# fresh copy of the showcase site, its output in $work/said.
overlay() {
  rm -rf "$4"
  cp -R "$root/fixtures/showcase-site" "$4"
  DEMO_PUBLICATIONS="$1" DEMO_PHOTOS_URL="$2" DEMO_PHOTOS_CACHE="$3" \
    bash "$root/scripts/demo-publication.sh" "$4" harbor >"$work/said" 2>&1
}

# The first run fills the cache; the second finds every photo there and
# fetches nothing, so an address with nothing behind it does no harm.
overlay "$sample" "$sample/demo-photos.tar.gz" "$work/cache" "$work/first" || fail "the sample did not apply: $(cat "$work/said")"
[ -f "$work/first/content/en/posts/2026-10-13-herbs-in-paper/lead.png" ] || fail "the photo is not in the copy's content"
overlay "$sample" "$work/no-such-archive.tar.gz" "$work/cache" "$work/again" || fail "a full cache still fetched: $(cat "$work/said")"

# A photo photos.lock names that the archive lacks stops the build.
cp -R "$sample" "$work/lacking"
sed -i 's#herbs-in-paper/lead.png#herbs-in-paper/second.png#' "$work/lacking/harbor/photos.lock"
if overlay "$work/lacking" "$sample/demo-photos.tar.gz" "$work/cache-lacking" "$work/copy"; then
  fail "a photo the archive lacks did not stop the build"
fi
grep -q "second.png, which the harbor publication's photos.lock names, is not in demo-photos.tar.gz" "$work/said" || fail "the missing photo's message: $(cat "$work/said")"

# A photo whose place leaves content/ is refused before anything is fetched.
cp -R "$sample" "$work/outside"
sed -i 's#^file = "en/posts/2026-10-06#file = "../../escape/2026-10-06#' "$work/outside/harbor/photos.lock"
if overlay "$work/outside" "$work/no-such-archive.tar.gz" "$work/cache-outside" "$work/copy"; then
  fail "a photo outside content/ was taken"
fi
grep -q "outside the publication's content folder" "$work/said" || fail "the outside photo's message: $(cat "$work/said")"

# A fragment with only a name still takes out the showcase's menus,
# announcement and call to action, and a table it sets replaces the
# tables inside it too.
cp -R "$sample" "$work/bare"
printf 'name = "Bare"\n\n[seo.publisher]\ntype = "organization"\nname = "Bare"\n' >"$work/bare/harbor/site.toml"
overlay "$work/bare" "$sample/demo-photos.tar.gz" "$work/cache" "$work/bare-copy" || fail "the bare fragment did not apply: $(cat "$work/said")"
for gone in '^\[\[nav\.primary\]\]' '^\[\[nav\.footer\]\]' '^\[announcement\]' '^\[template\.cta\]' '^\[seo\.publisher\.logo\]' '^languages = '; do
  if grep -q "$gone" "$work/bare-copy/site.toml"; then fail "the showcase's $gone stayed"; fi
done
grep -q '^\[template\.preferred_source\]' "$work/bare-copy/site.toml" || fail "a table the publication does not own was taken out"
names="$(awk '/^\[/ { exit } /^name = / { print }' "$work/bare-copy/site.toml")"
[ "$names" = 'name = "Bare"' ] || fail "the copy's own name is $names"

# The second pass checks ready publications only, unless every one is asked
# for, and none in the real folder that lacks the file "ready".
# shellcheck source=showcase.sh
source "$root/scripts/demo-check/showcase.sh"
cp -R "$sample" "$work/review"
rm "$work/review/harbor/ready"
[ "$(DEMO_PUBLICATIONS="$sample" showcase_publications wire harbor)" = harbor ] || fail "the ready sample is not in the second pass"
[ -z "$(DEMO_PUBLICATIONS="$work/review" showcase_publications wire harbor)" ] || fail "a publication not yet ready is in the second pass"
[ "$(DEMO_PUBLICATIONS="$work/review" DEMO_PUBLICATION_ALL=1 showcase_publications harbor)" = harbor ] || fail "DEMO_PUBLICATION_ALL=1 left the publication out"
real="$root/fixtures/showcase-publications"
want="$(for folder in "$real"/*/; do [ -f "$folder/ready" ] && basename "$folder"; done || true)"
# shellcheck source=../template-packs.sh
source "$root/scripts/template-packs.sh"
read -ra every_pack <<<"$(template_packs | tr '\n' ' ')"
[ "$(DEMO_PUBLICATIONS="$real" showcase_publications "${every_pack[@]}")" = "$want" ] ||
  fail "the second pass does not match the ready publications in $real"

# layout.mjs and pictures.mjs read the copy's check-pages.txt, home first.
if command -v node >/dev/null; then
  pages="$(node --input-type=module -e "
    const { checkPages } = await import(process.argv[1]);
    console.log(JSON.stringify([checkPages(process.argv[2]), checkPages(process.argv[3])]));
  " "$root/scripts/demo-check/site-files.mjs" "$work/first" "$root/fixtures/showcase-site")"
  expected='[[["home",""],["post","plan-a-week-of-dinners/"],["table","herbs-in-paper/"],["category","category/weeknight-cooking/"],["tag","tag/weeknights/"],["author","authors/ruth-calder/"]],null]'
  [ "$pages" = "$expected" ] || fail "check-pages.txt read as $pages"
else
  echo "Node is missing, so how check-pages.txt is read was not checked."
fi

if [ "$wrong" -gt 0 ]; then
  echo "$wrong cases went wrong. Fix scripts/demo-publication.sh or the checks it feeds, then run this again."
  exit 1
fi
echo "The demo publication step and the layout check's second pass work as fixtures/showcase-publications/README.md says."
