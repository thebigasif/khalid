#!/usr/bin/env bash
# Builds the demos SerpDad serves under https://serpdad.com/khalid/ and
# packs them as khalid-demos.tar.gz, with paths relative to /khalid/:
#
#   demo/                the showcase site, fixtures/showcase-site, which
#                        is Classic's demo
#   demos/<name>/        the demo of each designed template, every template
#                        but classic, with its preview pictures in _previews/
#   templates/           the template gallery, its pictures and its assets
#   templates/<name>/    the showcase once per template the binary has, kept
#                        until serpdad.com serves the demos from demos/
#   examples/<slug>/     each site in fixtures/examples/
#   templates.json       every template: its name, the kind of blog it suits,
#                        whether its demo publication is ready, its demo and
#                        its preview pictures
#
# serpdad.com/khalid/ itself is SerpDad's product page, so the website's
# other pages are left out. Every demo page says noindex, follow and every
# demo sitemap is empty; the gallery stays indexable, with a canonical
# link to itself. scripts/pages.sh does the building.
#
# Usage: scripts/demos-bundle.sh <khalid binary> <output folder> [pictures folder]
#        scripts/demos-bundle.sh --check <output folder or khalid-demos.tar.gz>
#
# The tree is built in <output folder>, which must not exist yet, and the
# archive is written beside it as khalid-demos.tar.gz. The pictures folder
# holds gallery pictures scripts/template-previews.sh already drew, such as
# the templates folder of the GitHub Pages build. Without it they are drawn
# here, as plain cards where no Chromium runs.
#
# --check reads a built bundle and stops unless templates.json lists every
# template the bundle has, each with its demo and with picture files that
# are there. A build runs the same check before it packs.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# The kind of blog each template suits, as the gallery groups them, and the
# heading it shows over each group. A new template needs a line here.
kind_of() {
  case "$1" in
    wire) echo news ;;
    atlas | meridian) echo magazine ;;
    folio | harbor | keystone) echo company ;;
    almanac | compass) echo guides ;;
    workbench) echo reviews ;;
    classic | ledger) echo journal ;;
    *) echo "" ;;
  esac
}
kind_label() {
  case "$1" in
    news) echo "News" ;;
    magazine) echo "Magazine" ;;
    company) echo "Company and studio" ;;
    guides) echo "Guides and teaching" ;;
    reviews) echo "Reviews" ;;
    journal) echo "Personal journal" ;;
  esac
}

# The name with a capital first letter, as the gallery writes it.
title() { printf '%s%s' "$(printf '%s' "${1:0:1}" | tr '[:lower:]' '[:upper:]')" "${1:1}"; }

# check <folder>: stops with what is wrong unless templates.json in the
# built bundle <folder> lists every template under templates/, gives each a
# known kind and a demo that has a front page, and names picture files that
# are in the bundle.
check() {
  python3 -I - "$1" <<'EOF'
import json
import pathlib
import sys

bundle = pathlib.Path(sys.argv[1]).resolve()
faults = []
try:
    listed = json.loads((bundle / "templates.json").read_text(encoding="utf-8"))["templates"]
except (OSError, ValueError, KeyError, TypeError) as error:
    sys.exit(f"The bundle in {bundle} has no readable templates.json ({error}). Build it again with scripts/demos-bundle.sh.")

def inside(path):
    """The file a path in templates.json names, or None when it leaves the bundle."""
    file = (bundle / path).resolve()
    return file if file.is_relative_to(bundle) else None

built = {p.name for p in (bundle / "templates").iterdir() if p.is_dir() and p.name != "assets"}
names = [entry.get("name") for entry in listed]
for name in sorted(built - set(names)):
    faults.append(f"{name} is built under templates/ but templates.json leaves it out.")
for name in sorted(set(names) - built):
    faults.append(f"templates.json lists {name}, but the bundle has no templates/{name}/.")
if len(names) != len(set(names)):
    faults.append("templates.json lists a template twice.")
for entry in listed:
    name = entry.get("name")
    if not entry.get("kind") or not entry.get("kind_label"):
        faults.append(f"{name} has no kind. Give it one in kind_of in scripts/demos-bundle.sh.")
    if not isinstance(entry.get("publication_ready"), bool):
        faults.append(f"{name} does not say whether its publication is ready.")
    demo = inside(entry.get("demo") or "/")
    if demo is None or not (demo / "index.html").is_file():
        faults.append(f"{name}'s demo {entry.get('demo')} has no front page in the bundle.")
    pictures = entry.get("pictures") or {}
    if set(pictures) != {"home", "post", "phone"}:
        faults.append(f"{name} should have a home, a post and a phone picture, and has {sorted(pictures)}.")
    for kind, path in pictures.items():
        file = inside(path)
        if file is None or not file.is_file() or file.stat().st_size == 0:
            faults.append(f"{name}'s {kind} picture {path} is not in the bundle.")
if faults:
    print("The demos bundle is not ready for serpdad.com:", *faults, sep="\n  ", file=sys.stderr)
    sys.exit(1)
print(f"templates.json lists all {len(names)} templates, and every demo and picture it names is in the bundle.")
EOF
}

if [ "${1:-}" = "--check" ]; then
  given="${2:?Name a built bundle folder or khalid-demos.tar.gz to check.}"
  if [ -f "$given" ]; then
    unpacked="$(mktemp -d)"
    trap 'rm -rf "$unpacked"' EXIT
    tar -xzf "$given" -C "$unpacked"
    given="$unpacked"
  fi
  check "$given"
  exit 0
fi

khalid="${1:?Name the khalid binary to build with.}"
out="${2:?Name a new folder to build the demos in.}"
pictures="${3:-}"

bash "$root/scripts/pages.sh" --demos-only "$khalid" "$out" https://serpdad.com /khalid/
if [ -n "$pictures" ]; then
  cp "$pictures"/*-home.svg "$pictures"/*-post.svg "$pictures"/*-phone.svg "$out/templates/"
else
  bash "$root/scripts/template-previews.sh" "$out" /khalid/
fi

# Each demo's pictures beside it, and templates.json, classic first and the
# rest as the binary built them. A publication is ready once its folder in
# fixtures/showcase-publications/ has a ready file; Classic's demo is the
# showcase itself, so it is always ready.
names=(classic)
for dir in "$out"/templates/*/; do
  name="$(basename "$dir")"
  case "$name" in classic | assets) ;; *) names+=("$name") ;; esac
done
{
  printf '{\n  "templates": ['
  separator=""
  for name in "${names[@]}"; do
    if [ "$name" = classic ]; then demo="demo/"; ready=true; else
      demo="demos/$name/"
      ready=false
      [ -f "$root/fixtures/showcase-publications/$name/ready" ] && ready=true
    fi
    if [ -d "$out/$demo" ]; then
      mkdir -p "$out/${demo}_previews"
      for page in home post phone; do
        if [ -f "$out/templates/$name-$page.svg" ]; then
          cp "$out/templates/$name-$page.svg" "$out/${demo}_previews/$page.svg"
        fi
      done
    fi
    kind="$(kind_of "$name")"
    printf '%s\n    {"name": "%s", "title": "%s", "kind": "%s", "kind_label": "%s", "publication_ready": %s, "demo": "%s", "pictures": {"home": "%s_previews/home.svg", "post": "%s_previews/post.svg", "phone": "%s_previews/phone.svg"}}' \
      "$separator" "$name" "$(title "$name")" "$kind" "$(kind_label "$kind")" "$ready" "$demo" "$demo" "$demo" "$demo"
    separator=","
  done
  printf '\n  ]\n}\n'
} >"$out/templates.json"
check "$out"

tarball="$(cd "$(dirname "$out")" && pwd)/khalid-demos.tar.gz"
tar --sort=name --owner=0 --group=0 --numeric-owner -czf "$tarball" -C "$out" demo demos templates examples templates.json
echo "The demos for https://serpdad.com/khalid/ are packed in $tarball."
