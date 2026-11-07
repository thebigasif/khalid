#!/usr/bin/env bash
# Builds the demos SerpDad serves under https://serpdad.com/khalid/ and
# packs them as khalid-demos.tar.gz, with paths relative to /khalid/:
#
#   demo/                the showcase site, fixtures/showcase-site, which
#                        is Classic's demo
#   demos/<name>/        the demo of each designed template, every template
#                        but classic, with its preview pictures in _previews/
#   templates/           the template gallery, its pictures and its assets.
#                        The build's templates/<name>/ copies of each demo
#                        stay out of the archive: serpdad.com serves the
#                        demos from demos/.
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
# Before it packs, the build leaves out every picture no page shows: the
# originals each post's folder holds, which pages name only in their
# structured data, never in a picture. A page loads the sized copies in
# media/ instead. The structured data then names only the pictures that
# stay. Only raster pictures are left out; SVG files, such as the
# gallery's previews, all stay.
#
# --check reads a built bundle and stops unless templates.json lists every
# template the bundle has, each with its demo and with picture files that
# are there, and unless every picture, video and sound a page, stylesheet,
# feed or templates.json names is in the bundle. Given the archive, it also
# stops when the archive is over its size budget. A build runs the same
# checks before and after it packs.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# The most khalid-demos.tar.gz may weigh, in megabytes of 1,000,000 bytes
# (docs/standards/performance.md, "Demo bundle"). Never raise it to make a
# build pass; find what grew instead.
budget_mb=725

# The kind of blog each template suits, as the gallery groups them, and the
# heading it shows over each group, from the template's own pack.toml.
# shellcheck source=template-packs.sh
source "$root/scripts/template-packs.sh"

# The name with a capital first letter, as the gallery writes it.
title() { printf '%s%s' "$(printf '%s' "${1:0:1}" | tr '[:lower:]' '[:upper:]')" "${1:1}"; }

# check <folder>: stops with what is wrong unless templates.json in the
# built bundle <folder> lists every template with a demo, Classic's at
# demo/ and each other one under demos/, gives each a
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

built = {p.name for p in (bundle / "demos").iterdir() if p.is_dir()}
if (bundle / "demo" / "index.html").is_file():
    built.add("classic")
names = [entry.get("name") for entry in listed]
for name in sorted(built - set(names)):
    faults.append(f"{name} has a demo in the bundle, but templates.json leaves it out.")
for name in sorted(set(names) - built):
    faults.append(f"templates.json lists {name}, but the bundle has no demo for it.")
if len(names) != len(set(names)):
    faults.append("templates.json lists a template twice.")
for entry in listed:
    name = entry.get("name")
    if not entry.get("kind") or not entry.get("kind_label"):
        faults.append(f"{name} has no kind. Give it kind and kind_label in crates/khalid-theme/packs/{name}/pack.toml.")
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
  media check "$1"
}

# media prune|check <folder>: reads every page, stylesheet, feed and
# templates.json the built bundle <folder> packs, and every address of a
# file in the bundle they name: in src, srcset, href, poster, url(), meta
# tags, feeds and structured data. "prune" removes each raster picture
# nothing but structured data names, and takes it out of that structured
# data. "check" stops when a picture, video or sound something names is
# not in the bundle.
media() {
  python3 -I - "$1" "$2" <<'EOF'
import html
import json
import os
import pathlib
import re
import sys
import urllib.parse

mode, bundle = sys.argv[1], pathlib.Path(sys.argv[2]).resolve()
ADDRESS, MOUNT = "https://serpdad.com", "/khalid/"
PICTURES = {".avif", ".webp", ".jpg", ".jpeg", ".png", ".gif"}
MEDIA = PICTURES | {".svg", ".mp4", ".webm", ".mp3", ".ogg", ".wav"}
TEXT = {".html", ".css", ".xml", ".json", ".js", ".svg", ".webmanifest", ".txt"}
ABSOLUTE = re.compile(r"""(?:https?://serpdad\.com)?(/khalid/[^"'\s,()<>&\\]*)""")
ATTRIBUTE = re.compile(r"""\b(src|srcset|href|poster|data-[a-z-]+)\s*=\s*"([^"]*)\"""")
CSS_URL = re.compile(r"""url\(\s*['"]?([^'")]+)['"]?\s*\)""")
STRUCTURED = re.compile(r'(<script type="application/ld\+json">)(.*?)(</script>)', re.S)


def packed(path):
    """Whether the archive holds the file at path, as the build packs it."""
    parts = path.relative_to(bundle).parts
    if parts[0] in ("demo", "demos", "examples"):
        return True
    if parts == ("templates.json",):
        return True
    return parts[0] == "templates" and (len(parts) == 2 or parts[1] == "assets")


def ours(path):
    """Whether a file named belongs in the bundle's folders. Other files
    under /khalid/, such as the product page's, are serpdad.com's own."""
    parts = path.relative_to(bundle).parts
    return parts[0] in ("demo", "demos", "examples", "templates", "templates.json")


def files():
    for path in sorted(bundle.rglob("*")):
        if path.is_file() and packed(path):
            yield path


def resolve(folder, address):
    """The file in the bundle an address names, or None for one outside it."""
    address = urllib.parse.unquote(address.split("#")[0].split("?")[0].strip())
    if address.startswith(ADDRESS):
        address = address[len(ADDRESS):]
    if address.startswith(MOUNT):
        path = bundle / address[len(MOUNT):]
    elif not address or address.startswith(("/", "data:")) or ":" in address:
        return None
    else:
        path = folder / address
    path = pathlib.Path(os.path.normpath(path))
    return path if path.is_relative_to(bundle) and path != bundle else None


def addresses(text, folder, relative):
    """Every file in the bundle a text names. Addresses relative to the
    file's folder count only where relative is true: in a page, a
    stylesheet or an SVG, but not in a feed, whose posts are relative to
    their own pages."""
    text = text.replace("\\/", "/")
    found = {resolve(folder, match.group(1)) for match in ABSOLUTE.finditer(text)}
    if relative:
        for match in ATTRIBUTE.finditer(text):
            name, value = match.group(1), html.unescape(match.group(2))
            candidates = value.split(",") if name.endswith("srcset") else [value]
            for candidate in candidates:
                words = candidate.split() if name.endswith("srcset") else [candidate]
                if words:
                    found.add(resolve(folder, words[0]))
        for match in CSS_URL.finditer(text):
            found.add(resolve(folder, match.group(1)))
    found.discard(None)
    return found


named, structured = set(), {}
for path in files():
    if path.suffix not in TEXT:
        continue
    text = path.read_text(encoding="utf-8", errors="replace")
    if path.name == "templates.json":
        for entry in json.loads(text)["templates"]:
            for value in [entry.get("demo") or ""] + list((entry.get("pictures") or {}).values()):
                found = resolve(bundle, value)
                if found:
                    named.add(found)
        continue
    if path.suffix == ".html":
        for block in STRUCTURED.finditer(text):
            structured[path] = structured.get(path, set()) | addresses(block.group(2), path.parent, False)
        text = STRUCTURED.sub("", text)
    named |= addresses(text, path.parent, path.suffix in (".html", ".css", ".svg"))


def escaped(data):
    """JSON for a script tag: no tag can end inside it."""
    text = json.dumps(data, ensure_ascii=False, separators=(",", ":"))
    return text.replace("<", "\\u003c").replace(">", "\\u003e").replace("&", "\\u0026")


GONE = object()


def without(data, gone, folder):
    """The structured data without the addresses of the files in gone. A
    list or key left empty by that goes too; nothing else changes."""
    if isinstance(data, list):
        kept = [item for item in (without(item, gone, folder) for item in data) if item is not GONE]
        return GONE if data and not kept else kept
    if isinstance(data, dict):
        kept = {key: without(value, gone, folder) for key, value in data.items()}
        return {key: value for key, value in kept.items() if value is not GONE}
    if isinstance(data, str) and resolve(folder, data) in gone:
        return GONE
    return data


if mode == "prune":
    gone, weight = set(), 0
    for path in files():
        if path.suffix.lower() in PICTURES and path not in named:
            weight += path.stat().st_size
            gone.add(path)
            path.unlink()
    for page, found in structured.items():
        if not found & gone:
            continue
        text = page.read_text(encoding="utf-8")
        page.write_text(
            STRUCTURED.sub(
                lambda block: block.group(1) + escaped(without(json.loads(block.group(2)), gone, page.parent)) + block.group(3),
                text,
            ),
            encoding="utf-8",
        )
    print(f"Left out {len(gone)} pictures no page shows, {weight / 1e6:.1f} MB.")
    sys.exit(0)

missing = {
    path
    for path in named.union(*structured.values())
    if path.suffix.lower() in MEDIA and ours(path) and not (path.is_file() and packed(path))
}
if missing:
    shown = sorted(str(path.relative_to(bundle)) for path in missing)
    print(f"Pages name {len(shown)} pictures, videos or sounds the bundle does not hold:", *shown[:20], sep="\n  ", file=sys.stderr)
    sys.exit(1)
print("Every picture, video and sound the pages name is in the bundle.")
EOF
}

# budget <archive>: stops when the archive weighs more than budget_mb.
budget() {
  local bytes
  bytes="$(stat -c %s "$1" 2>/dev/null || stat -f %z "$1")"
  if [ "$bytes" -gt $((budget_mb * 1000000)) ]; then
    echo "$1 weighs $((bytes / 1000000)) MB, over its budget of $budget_mb MB (docs/standards/performance.md, \"Demo bundle\"). Find what grew, such as a new picture width or format, and leave out what no page shows. Never raise the budget to make the build pass." >&2
    exit 1
  fi
  echo "$1 weighs $((bytes / 1000000)) MB, inside its budget of $budget_mb MB."
}

if [ "${1:-}" = "--check" ]; then
  given="${2:?Name a built bundle folder or khalid-demos.tar.gz to check.}"
  if [ -f "$given" ]; then
    budget "$given"
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
for dir in "$out"/demos/*/; do
  names+=("$(basename "$dir")")
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
    printf '%s\n    {"name": "%s", "title": "%s", "kind": "%s", "kind_label": "%s", "publication_ready": %s, "demo": "%s", "pictures": {"home": "%s_previews/home.svg", "post": "%s_previews/post.svg", "phone": "%s_previews/phone.svg"}}' \
      "$separator" "$name" "$(title "$name")" "$(template_pack_value "$name" kind)" "$(template_pack_value "$name" kind_label)" "$ready" "$demo" "$demo" "$demo" "$demo"
    separator=","
  done
  printf '\n  ]\n}\n'
} >"$out/templates.json"
check "$out"
media prune "$out"
media check "$out"

tarball="$(cd "$(dirname "$out")" && pwd)/khalid-demos.tar.gz"
# The gallery's own files, without the build's templates/<name>/ copies.
mapfile -t gallery < <(cd "$out" && find templates -maxdepth 1 -type f | LC_ALL=C sort)
tar --sort=name --owner=0 --group=0 --numeric-owner -czf "$tarball" -C "$out" \
  demo demos "${gallery[@]}" templates/assets examples templates.json
echo "The demos for https://serpdad.com/khalid/ are packed in $tarball."
budget "$tarball"
