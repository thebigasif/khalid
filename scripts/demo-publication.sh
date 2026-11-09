#!/usr/bin/env bash
# The demo publication step: gives a copy of the showcase site a template's
# own publication, its posts, menus, call to action and photos, so the
# template's demo reads like a real blog of its kind. The publication is
# fixtures/showcase-publications/<template>/, whose README sets out its
# files. This step:
#
#   1. checks every photo photos.lock names against its checksum, fetching
#      demo-photos.tar.gz first when the cache lacks one or holds another;
#   2. replaces the copy's whole content/ folder, every language, with the
#      publication's, and puts each photo in its place there;
#   3. replaces categories.toml and routes.toml;
#   4. merges the publication's site.toml into the copy's: each key and
#      table the publication sets replaces the copy's, and the copy's name,
#      languages, menus, announcement and call to action go even when the
#      publication leaves one out, so no showcase page is named;
#   5. copies check-pages.txt beside the copy's site.toml, where the layout
#      check and the picture review find the pages to open.
#
# A template without a publication keeps the showcase, and this does
# nothing. A publication applies only once its template's sheets are
# approved, which adds the file "ready"; the picture review sets
# DEMO_PUBLICATION_ALL=1 to show every publication. It runs before
# scripts/demo-brand.sh, so the brand's logo lands on the publication's
# about page. scripts/pages.sh and scripts/demo-check/showcase.sh call it.
#
# Usage: scripts/demo-publication.sh <copy of the showcase site> <template>
#
#   DEMO_PUBLICATIONS   the folder of publications, by default
#                       fixtures/showcase-publications
#   DEMO_PHOTOS_URL     where demo-photos.tar.gz comes from: an https
#                       address, by default the demos release, or a file
#   DEMO_PHOTOS_CACHE   where the photos are kept between runs, outside the
#                       repository, by default ~/.cache/khalid/demo-photos
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
copy="${1:?Name the copy of the showcase site to change, then the template.}"
template="${2:?Name the template whose publication to use.}"
publication="${DEMO_PUBLICATIONS:-$root/fixtures/showcase-publications}/$template"
url="${DEMO_PHOTOS_URL:-https://github.com/thebigasif/khalid/releases/download/demos/demo-photos.tar.gz}"
cache="${DEMO_PHOTOS_CACHE:-${XDG_CACHE_HOME:-$HOME/.cache}/khalid/demo-photos}"

stop() { echo "$1"; exit 1; }

[ -d "$publication" ] || exit 0
[ -f "$publication/ready" ] || [ "${DEMO_PUBLICATION_ALL:-}" = 1 ] || exit 0
[ -f "$copy/site.toml" ] ||
  stop "$copy holds no site.toml, so it is not a copy of the showcase site. Name the copy the build makes, then run this again."
for part in site.toml content check-pages.txt; do
  [ -e "$publication/$part" ] ||
    stop "The $template publication has no $part, and every publication needs one. Add it as fixtures/showcase-publications/README.md says, then run this again."
done

# Each photo photos.lock names, one "<sha256> <file>" line each. A photo
# whose place leaves content/ is refused, since the archive is not ours.
photos() {
  [ -f "$publication/photos.lock" ] || return 0
  awk '
    function flush() {
      if (seen && (file == "" || sum == "")) { print "BAD " NR; exit }
      if (seen) print sum " " file
      seen = 0; file = ""; sum = ""
    }
    /^\[\[photos\]\]/ { flush(); seen = 1; next }
    /^\[/ { flush(); next }
    seen && /^file *= *"/ { file = $0; sub(/^file *= *"/, "", file); sub(/".*$/, "", file) }
    seen && /^sha256 *= *"/ { sum = $0; sub(/^sha256 *= *"/, "", sum); sub(/".*$/, "", sum) }
    END { flush() }
  ' "$publication/photos.lock"
}
lock="$(photos)"
case "$lock" in
  *BAD*) stop "An entry in the $template publication's photos.lock has no file or no sha256, so its photo cannot be checked. Give every [[photos]] entry both, then run this again." ;;
esac
while read -r sum file; do
  [ -n "$file" ] || continue
  case "/$file/" in
    *"/../"* | //*) stop "photos.lock names the photo $file, which is outside the publication's content folder. Name it by its place inside content/, such as en/posts/<post>/lead.jpg." ;;
  esac
  [[ "$sum" =~ ^[0-9a-f]{64}$ ]] ||
    stop "photos.lock gives the photo $file the checksum $sum, which is not a sha256. Write the 64 letters and digits sha256sum prints for it."
done <<<"$lock"

# The photos the cache lacks or holds a different copy of, one per line.
unchecked() {
  local sum file
  while read -r sum file; do
    [ -n "$file" ] || continue
    if [ ! -f "$cache/$template/$file" ]; then
      echo "missing $file"
    elif [ "$(sha256sum "$cache/$template/$file" | cut -d' ' -f1)" != "$sum" ]; then
      echo "changed $file"
    fi
  done <<<"$lock"
}

# A GitHub release file fetched through the API instead of its public
# address, which GitHub sometimes answers with 504 for every release file.
# Needs `gh` and a token in GH_TOKEN, as CI has; elsewhere it does nothing.
from_api() {
  local out="$1"
  [[ "$url" =~ ^https://github\.com/([^/]+/[^/]+)/releases/download/([^/]+)/([^/]+)$ ]] || return 1
  [ -n "${GH_TOKEN:-}" ] && command -v gh >/dev/null || return 1
  echo "The public address failed; fetching ${BASH_REMATCH[3]} through the GitHub API"
  gh release download "${BASH_REMATCH[2]}" --repo "${BASH_REMATCH[1]}" \
    --pattern "${BASH_REMATCH[3]}" --output "$out" --clobber
}

# Fetches demo-photos.tar.gz and unpacks each publication's folder of it
# into the cache, in place of the old one.
fetch() {
  local work archive
  work="$(mktemp -d "$cache/.fetch.XXXXXX")"
  archive="$work/demo-photos.tar.gz"
  case "$url" in
    https://* | http://*)
      echo "Fetching the demo photos from $url"
      curl -fsSL --retry 2 --connect-timeout 20 --max-time 600 --max-filesize 1073741824 -o "$archive" "$url" ||
        from_api "$archive" ||
        stop "The demo photos could not be fetched from $url, so the $template demo has no photos. Check the address and the connection, or set DEMO_PHOTOS_URL to a copy of demo-photos.tar.gz, then run this again."
      ;;
    *)
      local file="${url#file://}"
      [ -f "$file" ] ||
        stop "DEMO_PHOTOS_URL names $url, and there is no file there, so the $template demo has no photos. Point it at demo-photos.tar.gz, then run this again."
      cp "$file" "$archive"
      ;;
  esac
  mkdir "$work/files"
  tar -xzf "$archive" -C "$work/files" --no-same-owner --no-same-permissions ||
    stop "$url is not a gzip tar archive, so the demo photos could not be unpacked. Point DEMO_PHOTOS_URL at demo-photos.tar.gz, then run this again."
  local folder
  for folder in "$work/files"/*/; do
    [ -d "$folder" ] || continue
    rm -rf "$cache/$(basename "$folder")"
    mv "$folder" "$cache/$(basename "$folder")"
  done
  rm -rf "$work"
}

if [ -n "$lock" ]; then
  mkdir -p "$cache"
  # Builds run four at a time, so one fetch at a time into the cache.
  if command -v flock >/dev/null; then
    exec 9>"$cache/.lock"
    flock 9
  fi
  [ -z "$(unchecked)" ] || fetch
  problem="$(unchecked | head -n 1)"
  case "$problem" in
    missing*) stop "The photo ${problem#missing }, which the $template publication's photos.lock names, is not in demo-photos.tar.gz from $url. Add it to the archive under $template/, or take it out of photos.lock, then run this again." ;;
    changed*) stop "The photo ${problem#changed } in demo-photos.tar.gz does not match its checksum in photos.lock, so it may be a wrong or damaged copy. Check the photo, then put the right one in the archive or its new sha256 in photos.lock, and run this again." ;;
  esac
  if command -v flock >/dev/null; then exec 9>&-; fi
fi

# The showcase's theme names its logo on the about page. Where no sample
# brand puts its own logo there, the showcase's stays, unless the
# publication brings one.
logo="content/en/pages/about/logo.svg"
kept=""
if [ -f "$copy/$logo" ]; then
  kept="$(mktemp)"
  cp "$copy/$logo" "$kept"
fi
rm -rf "$copy/content"
cp -R "$publication/content" "$copy/content"
if [ -n "$kept" ]; then
  if [ ! -f "$copy/$logo" ] && [ -d "$(dirname "$copy/$logo")" ]; then cp "$kept" "$copy/$logo"; fi
  rm -f "$kept"
fi
while read -r sum file; do
  [ -n "$file" ] || continue
  mkdir -p "$(dirname "$copy/content/$file")"
  cp "$cache/$template/$file" "$copy/content/$file"
done <<<"$lock"
for part in categories.toml routes.toml; do
  rm -f "$copy/$part"
  if [ -f "$publication/$part" ]; then cp "$publication/$part" "$copy/$part"; fi
done
cp "$publication/check-pages.txt" "$copy/check-pages.txt"

# The merge. A key is one line, `name = value`, before the first table; a
# table is everything from its [header] to the next. The publication's
# keys go first, then the copy's other keys and tables, then the
# publication's tables. A table the publication sets replaces the copy's
# table of that name and every table inside it, such as [seo.publisher]
# and [seo.publisher.logo].
awk \
  -v owned_keys="name languages" \
  -v owned_tables="announcement nav.primary nav.footer nav.footer_groups template.cta" '
  function header(line,   name) { name = line; sub(/^[ \t]*\[+[ \t]*/, "", name); sub(/[ \t]*\].*$/, "", name); return name }
  function key(line,   name) { name = line; sub(/[ \t]*=.*$/, "", name); return name }
  function replaced(name,   t) { for (t in tables) if (name == t || index(name, t ".") == 1) return 1; return 0 }
  BEGIN {
    n = split(owned_keys, list, " "); for (i = 1; i <= n; i++) keys[list[i]] = 1
    n = split(owned_tables, list, " "); for (i = 1; i <= n; i++) tables[list[i]] = 1
  }
  FNR == NR {
    if ($0 ~ /^[ \t]*\[/) { in_tables = 1; tables[header($0)] = 1 }
    if (in_tables) { their_tables = their_tables $0 "\n"; next }
    if ($0 ~ /^[A-Za-z0-9_-]+[ \t]*=/) keys[key($0)] = 1
    their_keys = their_keys $0 "\n"
    next
  }
  # A comment, with the blank lines around it, goes or stays with the line
  # after it, so a dropped table takes its note along.
  /^[ \t]*(#|$)/ { held = held $0 "\n"; next }
  {
    if ($0 ~ /^[ \t]*\[/) { in_copy_tables = 1; skip = replaced(header($0)) }
    if (!in_copy_tables) {
      if (!($0 ~ /^[A-Za-z0-9_-]+[ \t]*=/ && (key($0) in keys))) our_keys = our_keys held $0 "\n"
    } else if (!skip) {
      our_tables = our_tables held $0 "\n"
    }
    held = ""
  }
  END { printf "%s\n%s%s\n%s", their_keys, our_keys, our_tables, their_tables }
' "$publication/site.toml" "$copy/site.toml" >"$copy/site.toml.new"
mv "$copy/site.toml.new" "$copy/site.toml"
