#!/usr/bin/env bash
# Draws the preview pictures the website's template gallery shows: the home
# page and one post of each template's demo, as built by scripts/pages.sh.
#
#   scripts/template-previews.sh <pages folder> [mount] [post path]
#   scripts/template-previews.sh --cards <folder>
#
# The first form serves the built pages on this computer, opens each demo
# in a headless Chromium and writes <pages folder>/templates/<name>-home.svg
# and <name>-post.svg, where the gallery reads them. Classic's demo is the
# showcase at <mount>demo/. The mount defaults to /khalid/ and the post to
# seo-writing-guide/, except where a template's demo shows its own
# publication, which draws the post its check-pages.txt names. Set CHROME
# to the browser to use; otherwise the first of google-chrome, chromium
# and chrome on the path runs. The server is
# Python's, which GitHub's runners have, as they have Chrome.
#
# Each picture is an SVG that holds the screenshot, so the gallery's
# addresses never change. The page is laid out 1280 pixels wide and drawn
# at full size, and the phone at twice its 390 px, so both stay sharp on a
# high-density screen. Where cwebp or ImageMagick is here it becomes a
# WebP of a hundred or two kilobytes; otherwise it stays a PNG. Where no
# Chromium runs, or a page cannot be opened, the picture is a plain card
# with the template's name instead.
#
# The pages name the site's own files, such as the logo, by full address,
# which may point at a host that no longer serves them. The local server
# changes each such address under the mount into a path on itself, so the
# picture shows the files of this build. It stops with the list of files
# it could not find, so no picture ships with a broken image.
#
# The second form writes only the plain cards into <folder>. The copies in
# site/content/en/pages/templates/ are made this way, so the website shows
# a card for each template before the screenshots replace them on Pages.
set -euo pipefail

# Every look the gallery shows: each template in crates/khalid-theme/packs/.
# shellcheck source=template-packs.sh
source "$(dirname "${BASH_SOURCE[0]}")/template-packs.sh"
read -ra names <<<"$(template_packs | tr '\n' ' ')"
width=1280
height=800
# The phone picture: a 390 px wide screen, as the layout check's phone.
phone_width=390
phone_height=844

# The name with a capital first letter, as the gallery writes it.
title() { printf '%s%s' "$(printf '%s' "${1:0:1}" | tr '[:lower:]' '[:upper:]')" "${1:1}"; }

# card <name> <home|post> <file>: the plain card, a light browser window
# with the template's name and the outline of the page it stands for.
card() {
  local name="$1" kind="$2" file="$3" label shape
  if [ "$kind" = phone ]; then
    cat >"$file" <<CARD
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 $phone_width $phone_height" width="$phone_width" height="$phone_height" role="img"><title>The $(title "$name") template: On a phone</title>
<rect width="$phone_width" height="$phone_height" fill="#f7f5f0"/><rect x="20" y="20" width="350" height="804" rx="36" fill="#fff" stroke="#e4dfd4" stroke-width="2"/>
<text x="195" y="300" text-anchor="middle" font-family="Georgia, serif" font-size="44" font-weight="700" fill="#16181d">$(title "$name")</text><text x="195" y="350" text-anchor="middle" font-family="system-ui, sans-serif" font-size="22" fill="#4f5563">On a phone</text>
<g fill="#efebe3"><rect x="56" y="420" width="278" height="150" rx="12"/><rect x="56" y="600" width="278" height="14" rx="7"/><rect x="56" y="630" width="220" height="14" rx="7"/></g>
</svg>
CARD
    return
  fi
  if [ "$kind" = home ]; then
    label="Home page"
    shape='<rect x="96" y="392" width="336" height="200" rx="14"/><rect x="472" y="392" width="336" height="200" rx="14"/><rect x="848" y="392" width="336" height="200" rx="14"/>'
  else
    label="A post"
    shape='<rect x="300" y="400" width="680" height="18" rx="9"/><rect x="300" y="440" width="640" height="18" rx="9"/><rect x="300" y="480" width="660" height="18" rx="9"/><rect x="300" y="520" width="520" height="18" rx="9"/>'
  fi
  cat >"$file" <<EOF
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 $width $height" width="$width" height="$height" role="img"><title>The $(title "$name") template: $label</title>
<rect width="$width" height="$height" fill="#f7f5f0"/><rect x="40" y="40" width="1200" height="720" rx="24" fill="#fff" stroke="#e4dfd4" stroke-width="2"/><path d="M40 112h1200" stroke="#e4dfd4" stroke-width="2"/><circle cx="84" cy="76" r="9" fill="#c2410c"/><circle cx="114" cy="76" r="9" fill="#e4dfd4"/><circle cx="144" cy="76" r="9" fill="#e4dfd4"/>
<text x="640" y="250" text-anchor="middle" font-family="Georgia, serif" font-size="88" font-weight="700" fill="#16181d">$(title "$name")</text><text x="640" y="316" text-anchor="middle" font-family="system-ui, sans-serif" font-size="34" fill="#4f5563">$label</text>
<g fill="#efebe3">$shape</g>
</svg>
EOF
}

if [ "${1:-}" = "--cards" ]; then
  into="${2:?Name the folder to write the cards into.}"
  mkdir -p "$into"
  for name in "${names[@]}"; do
    card "$name" home "$into/$name-home.svg"
    card "$name" post "$into/$name-post.svg"
    card "$name" phone "$into/$name-phone.svg"
  done
  echo "Wrote ${#names[@]} pairs of plain cards into $into."
  exit 0
fi

repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
pages="${1:?Name the folder scripts/pages.sh built.}"
mount="${2:-/khalid/}"
post="${3:-seo-writing-guide/}"
if [ ! -d "$pages/templates" ]; then
  echo "$pages has no templates folder, so it is not a folder scripts/pages.sh built. Build the pages first, then run this again."
  exit 1
fi

chrome="${CHROME:-}"
if [ -z "$chrome" ]; then
  for candidate in google-chrome google-chrome-stable chromium chromium-browser chrome; do
    if command -v "$candidate" >/dev/null 2>&1; then
      chrome="$candidate"
      break
    fi
  done
fi

if [ -z "$chrome" ]; then
  echo "No headless Chromium here, so the gallery shows a plain card for each template. Install Chromium, or set CHROME, for screenshots."
  for name in "${names[@]}"; do
    card "$name" home "$pages/templates/$name-home.svg"
    card "$name" post "$pages/templates/$name-post.svg"
  done
  exit 0
fi

work="$(mktemp -d)"
server=""
cleanup() {
  if [ -n "$server" ]; then kill "$server" 2>/dev/null || true; fi
  rm -rf "$work"
}
trap cleanup EXIT

# The pages link each other by full path under the mount, so the server's
# root holds the mount as a folder that leads to the built pages.
root="$work/root"
mkdir -p "$root"
inside="${mount#/}"
inside="${inside%/}"
if [ -n "$inside" ]; then
  mkdir -p "$root/$(dirname "$inside")"
  ln -s "$(cd "$pages" && pwd)" "$root/$inside"
else
  rmdir "$root"
  ln -s "$(cd "$pages" && pwd)" "$root"
fi
port="$(python3 -c 'import socket; s = socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1])')"
# Python's file server, which turns the site's own full addresses under the
# mount in pages and stylesheets into paths, and writes each path it could
# not find to missing.txt.
cat >"$work/serve.py" <<'PY'
import functools
import http.server
import pathlib
import re
import sys

port, root, mount, missing = int(sys.argv[1]), sys.argv[2], sys.argv[3], sys.argv[4]
own = re.compile(rb"https?://[^/\s\"'<>()]+(?=" + re.escape(mount.encode()) + rb")")


class Handler(http.server.SimpleHTTPRequestHandler):
    def send_head(self):
        path = pathlib.Path(self.translate_path(self.path))
        if path.is_dir() and self.path.split("?")[0].endswith("/"):
            path = path / "index.html"
        if path.suffix not in (".html", ".css") or not path.is_file():
            return super().send_head()
        body = own.sub(b"", path.read_bytes())
        self.send_response(200)
        self.send_header("Content-Type", "text/css" if path.suffix == ".css" else "text/html; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)
        return None

    def do_GET(self):
        head = self.send_head()
        if head:
            try:
                self.copyfile(head, self.wfile)
            finally:
                head.close()

    def log_request(self, code="-", size="-"):
        if str(code) == "404":
            with open(missing, "a", encoding="utf-8") as out:
                out.write(self.path + "\n")


http.server.ThreadingHTTPServer(("127.0.0.1", port), functools.partial(Handler, directory=root)).serve_forever()
PY
: >"$work/missing.txt"
python3 -I "$work/serve.py" "$port" "$root" "$mount" "$work/missing.txt" >"$work/server.log" 2>&1 &
server=$!
base="http://127.0.0.1:$port"
for _ in $(seq 50); do
  if python3 -c "import urllib.request; urllib.request.urlopen('$base$mount')" 2>/dev/null; then break; fi
  sleep 0.2
done

# Turns the screenshot into a WebP with cwebp or ImageMagick, whichever is
# here; otherwise the picture holds the PNG as Chromium wrote it.
webp() {
  if command -v cwebp >/dev/null 2>&1; then
    cwebp -quiet -q 82 "$1" -o "$2"
  elif command -v magick >/dev/null 2>&1; then
    magick "$1" -quality 82 "$2"
  elif command -v convert >/dev/null 2>&1; then
    convert "$1" -quality 82 "$2"
  else
    return 1
  fi
}

# shot <name> <home|post|phone> <address>: the picture of one page, or the
# plain card when the browser draws nothing. The phone picture is the home
# page in a 390 px window, drawn at full size.
shot() {
  local name="$1" kind="$2" address="$3" png="$work/$1-$2.png" file="$pages/templates/$1-$2.svg" kind_of="image/png" data
  local w="$width" h="$height" scale=1 label
  if [ "$kind" = phone ]; then w="$phone_width" h="$phone_height" scale=2; fi
  case "$kind" in home) label="Home page" ;; post) label="A post" ;; *) label="On a phone" ;; esac
  # Ten seconds of page time lets every photo in the first screen arrive
  # and draw; three left Harbor's phone picture with an empty photo box.
  timeout 90 "$chrome" --headless --no-sandbox --disable-gpu --hide-scrollbars \
    --force-device-scale-factor="$scale" --window-size="$w,$h" \
    --run-all-compositor-stages-before-draw --virtual-time-budget=10000 \
    --screenshot="$png" "$base$address" >/dev/null 2>&1 || true
  if [ ! -s "$png" ]; then
    echo "Chromium drew nothing for $address, so $name's $kind picture is the plain card."
    card "$name" "$kind" "$file"
    return
  fi
  if webp "$png" "$work/$1-$2.webp" 2>/dev/null && [ -s "$work/$1-$2.webp" ]; then
    png="$work/$1-$2.webp"
    kind_of="image/webp"
  fi
  data="$(base64 <"$png" | tr -d '\n')"
  printf '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 %s %s" width="%s" height="%s" role="img"><title>The %s template: %s</title><image width="%s" height="%s" href="data:%s;base64,%s"/></svg>\n' \
    "$w" "$h" "$w" "$h" "$(title "$name")" "$label" \
    "$w" "$h" "$kind_of" "$data" >"$file"
  echo "Drew $name's $kind page."
}

# post_of <name> <demo>: the post to draw for a template. A demo that shows
# the template's own publication draws the post its check-pages.txt names;
# any other draws the showcase's post.
post_of() {
  local list="$repo/fixtures/showcase-publications/$1/check-pages.txt" path
  path="$(sed -n 's#^post /##p' "$list" 2>/dev/null | head -n 1)"
  if [ -n "$path" ] && [ -f "$pages/${2#"$mount"}${path}index.html" ]; then
    printf '%s' "$path"
  else
    printf '%s' "$post"
  fi
}

for name in "${names[@]}"; do
  if [ "$name" = classic ]; then demo="${mount}demo/"; else demo="${mount}templates/$name/"; fi
  if [ ! -f "$pages/${demo#"$mount"}index.html" ]; then
    echo "There is no $name demo in $pages, so its pictures are plain cards."
    card "$name" home "$pages/templates/$name-home.svg"
    card "$name" post "$pages/templates/$name-post.svg"
    card "$name" phone "$pages/templates/$name-phone.svg"
    continue
  fi
  shot "$name" home "$demo"
  shot "$name" post "$demo$(post_of "$name" "$demo")"
  shot "$name" phone "$demo"
done
# A browser asks every host for /favicon.ico, which a site under a mount
# never has at the root.
sed -i.bak '\#^/favicon.ico$#d' "$work/missing.txt"
if [ -s "$work/missing.txt" ]; then
  echo "The previews asked for files this build does not have, so a picture shows a broken image:"
  sort -u "$work/missing.txt" | sed 's/^/  /'
  echo "Build the pages again, or fix the address that names each file, then run this again."
  exit 1
fi
echo "The template previews are in $pages/templates."
