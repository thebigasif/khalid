#!/usr/bin/env bash
# Packs Khalid's user docs for https://serpdad.com/khalid/docs/ as
# khalid-docs.tar.gz. scripts/docs-pack.md, packed as the README, sets out
# the layout the website build reads.
#
# A doc is packed when its front matter has an order, beside its title. Links
# between packed docs become /khalid/docs/<slug>/ addresses, pictures under
# docs/ are copied in and linked at /khalid/docs/<path>, and links to other
# files of the repository become their GitHub addresses. A link or picture
# that resolves to nothing stops the pack, with the file and the line.
#
# Usage: scripts/docs-bundle.sh <output folder> [repository root]
#
# The pack is built in <output folder>, which must not exist yet, and the
# archive is written beside it. The repository root defaults to this one.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
out="${1:?Name a new folder to build the docs pack in.}"
root="$(cd "${2:-$here}" && pwd)"
base=/khalid/docs/
github=https://github.com/thebigasif/khalid
tab=$'\t'

if [ -e "$out" ]; then
  echo "$out already exists. Name a new folder, so nothing in it is overwritten." >&2
  exit 1
fi
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

# Prints "slug, title, order" for a doc whose front matter has an order.
front() {
  awk -v slug="$1" -v src="docs/$1.md" '
    NR == 1 && $0 != "---" { exit }
    NR > 1 && $0 == "---" { closed = 1; exit }
    /^title: / { title = $0; if (title ~ /^title: "[^"\\]+"$/) title = substr(title, 9, length(title) - 9); else title = "" }
    /^order: / { order = substr($0, 8); if (order !~ /^[0-9]+$/) bad = 1 }
    END {
      if (!closed || order == "") exit
      if (bad || title == "") {
        print src ": the front matter needs title: \"<words without quotes>\" and order: <a whole number>." > "/dev/stderr"
        exit 1
      }
      print slug "\t" title "\t" (order + 0)
    }
  ' "$2"
}

failed=0
while IFS= read -r file; do
  slug="${file#"$root"/docs/}"
  front "${slug%.md}" "$file" >>"$work/docs" || failed=1
done < <(find "$root/docs" -name '*.md' -type f | LC_ALL=C sort)
if [ ! -s "$work/docs" ]; then
  echo "No doc under $root/docs has a title and an order in its front matter, so there is nothing to pack." >&2
  exit 1
fi

mkdir -p "$out/pages" "$out/pictures"
cp "$here/scripts/docs-pack.md" "$out/README.md"

# Rewrites one doc's links outside code. After a line holding only a form
# feed, it prints each picture the doc uses.
rewrite() {
  awk -v src="$1" -v root="$root" -v base="$base" -v github="$github" -v docs="$work/docs" -v q="'" '
    function fail(why) { printf "%s line %d: %s\n", src, FNR, why > "/dev/stderr"; bad = 1 }
    function exists(path, kind) { return index(path, q) == 0 && system("test -" kind " " q root "/" path q) == 0 }
    function normal(path,   n, i, part, kept, k) {
      n = split(path, part, "/"); k = 0
      for (i = 1; i <= n; i++) {
        if (part[i] == "" || part[i] == ".") continue
        if (part[i] == "..") { if (k == 0) return ""; k--; continue }
        kept[++k] = part[i]
      }
      path = ""
      for (i = 1; i <= k; i++) path = path (i > 1 ? "/" : "") kept[i]
      return path
    }
    function resolve(target,   path, frag, at, slug) {
      if (target ~ /^[A-Za-z][A-Za-z0-9+.-]*:/ || target ~ /^#/) return target
      if (target ~ /^\//) { fail("the link (" target ") starts at a site root, which the docs pages do not share. Link the doc file, or write a full address."); return target }
      path = target; frag = ""
      at = index(path, "#"); if (at) { frag = substr(path, at); path = substr(path, 1, at - 1) }
      path = normal(dir "/" path)
      if (path == "") { fail("the link (" target ") leaves the repository."); return target }
      slug = path; sub(/^docs\//, "", slug); sub(/\.md$/, "", slug)
      if (path ~ /^docs\/.*\.md$/ && (slug in packed)) return base slug "/" frag
      if (tolower(path) ~ /\.(png|jpe?g|gif|svg|webp|avif)$/) {
        if (path !~ /^docs\// || !exists(path, "f")) { fail("the picture (" target ") is not a file under docs/. Put the picture in docs/ beside the doc."); return target }
        sub(/^docs\//, "", path); pictures[path] = 1; return base path
      }
      if (exists(path, "f")) return github "/blob/main/" path frag
      if (exists(path, "d")) return github "/tree/main/" path frag
      fail("the link (" target ") points to nothing in the repository. Fix the path, or write a full address.")
      return target
    }
    # Rewrites the link target at the front of rest, and keeps what follows.
    function target(rest,   n, depth, c) {
      if (substr(rest, 1, 1) == "<") { n = index(rest, ">"); if (n) return "<" resolve(substr(rest, 2, n - 2)) ">" substr(rest, n + 1); return rest }
      depth = 0
      for (n = 1; n <= length(rest); n++) {
        c = substr(rest, n, 1)
        if (c == " " || c == "\t") break
        if (c == "(") depth++
        if (c == ")") { if (depth == 0) break; depth-- }
      }
      return resolve(substr(rest, 1, n - 1)) substr(rest, n)
    }
    # One line outside a fence: reference definitions, then inline links
    # and pictures, skipping code spans.
    function line(text,   done, i, run, end) {
      if (match(text, /^ ? ? ?\[[^]]+\]:[ \t]*/)) return substr(text, 1, RLENGTH) target(substr(text, RLENGTH + 1))
      done = ""; i = 1
      while (i <= length(text)) {
        if (substr(text, i, 1) == "`") {
          match(substr(text, i), /^`+/); run = substr(text, i, RLENGTH)
          end = index(substr(text, i + RLENGTH), run)
          if (!end) { done = done substr(text, i); break }
          end = 2 * RLENGTH + end - 1
          done = done substr(text, i, end); i += end; continue
        }
        if (substr(text, i, 2) == "](") {
          text = substr(text, 1, i + 1) target(substr(text, i + 2))
          done = done "]("; i += 2; continue
        }
        done = done substr(text, i, 1); i++
      }
      return done
    }
    BEGIN {
      while ((getline row < docs) > 0) { split(row, field, "\t"); packed[field[1]] = 1 }
      dir = src; sub(/\/[^\/]*$/, "", dir)
    }
    FNR == 1 { head = ($0 == "---"); print; next }
    head { if ($0 == "---") head = 0; print; next }
    fence == "" && match($0, /^[ \t]*(```+|~~~+)/) { fence = substr($0, RSTART, RLENGTH); sub(/^[ \t]*/, "", fence); print; next }
    fence != "" { probe = $0; sub(/^[ \t]*/, "", probe); if (index(probe, fence) == 1 && probe ~ /^(`+|~+)[ \t]*$/) fence = ""; print; next }
    { print line($0) }
    END { printf "\f\n"; for (p in pictures) print p; exit bad }
  ' "$root/$1"
}

# Adds the slug, the parent doc and the source file to the front matter.
stamp() {
  awk -v slug="$1" -v parent="$2" '
    NR > 1 && $0 == "---" && !done { print "slug: " slug; if (parent != "") print "parent: " parent; print "source: docs/" slug ".md"; done = 1 }
    { print }'
}

while IFS="$tab" read -r slug title order; do
  parent=""
  case "$slug" in */*) parent="${slug%/*}" ;; esac
  if [ -n "$parent" ] && ! grep -q "^$parent$tab" "$work/docs"; then
    echo "docs/$slug.md sits under docs/$parent/, and docs/$parent.md is not packed. Give it a title and an order." >&2
    failed=1
    continue
  fi
  if ! rewritten="$(rewrite "docs/$slug.md")"; then
    failed=1
    continue
  fi
  mkdir -p "$(dirname "$out/pages/$slug.md")"
  body="${rewritten%%$'\f'*}"
  printf '%s\n' "${body%$'\n'}" | stamp "$slug" "$parent" >"$out/pages/$slug.md"
  pictures="$(printf '%s\n' "${rewritten#*$'\f'}" | sed '/^$/d' | LC_ALL=C sort)"
  for picture in $pictures; do
    mkdir -p "$(dirname "$out/pictures/$picture")"
    cp "$root/docs/$picture" "$out/pictures/$picture"
  done
  printf '%s\t%s\t%s\t%s\t%s\n' "$slug" "$title" "$order" "$parent" "$(printf '%s' "$pictures" | tr '\n' ' ')" >>"$work/rows"
done <"$work/docs"
if [ "$failed" -ne 0 ]; then
  echo "The docs pack stopped: fix what the lines above name, then run it again." >&2
  exit 1
fi

# The doc list in sidebar order: each top doc by its order, then its guides
# by theirs.
awk -F'\t' '
  { order[NR] = $3; parent[NR] = $4; if ($4 == "") top[$1] = $3 }
  END { for (i = 1; i <= NR; i++) printf "%010d\t%010d\t%d\n", (parent[i] == "" ? order[i] : top[parent[i]]), (parent[i] == "" ? 0 : order[i] + 1), i }
' "$work/rows" | LC_ALL=C sort | cut -f3 >"$work/order"
awk -F'\t' -v base="$base" '
  FNR == NR { want[++n] = $1; next }
  { row[FNR] = $0 }
  END {
    printf "{\n  \"base\": \"%s\",\n  \"docs\": [\n", base
    for (i = 1; i <= n; i++) {
      split(row[want[i]], f, "\t")
      count = split(f[5], p, " "); list = ""
      for (j = 1; j <= count; j++) list = list (j > 1 ? ", " : "") "\"pictures/" p[j] "\""
      printf "    {\"slug\": \"%s\", \"title\": \"%s\", \"order\": %d, \"parent\": %s, \"url\": \"%s%s/\", \"file\": \"pages/%s.md\", \"pictures\": [%s]}%s\n", \
        f[1], f[2], f[3], (f[4] == "" ? "null" : "\"" f[4] "\""), base, f[1], f[1], list, (i < n ? "," : "")
    }
    printf "  ]\n}\n"
  }' "$work/order" "$work/rows" >"$out/docs.json"

tarball="$(cd "$(dirname "$out")" && pwd)/khalid-docs.tar.gz"
tar --sort=name --owner=0 --group=0 --numeric-owner -czf "$tarball" -C "$out" README.md docs.json pages pictures
echo "The docs for https://serpdad.com$base are packed in $tarball."
