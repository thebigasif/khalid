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
# seo-writing-guide/. Set CHROME to the browser to use; otherwise the first
# of google-chrome, chromium and chrome on the path runs. The server is
# Python's, which GitHub's runners have, as they have Chrome.
#
# Each picture is an SVG that holds the screenshot, so the gallery's
# addresses never change. The page is laid out 1280 pixels wide and drawn
# at half size, so a picture weighs tens of kilobytes, not hundreds. Where
# ImageMagick is here it becomes a WebP. Where no Chromium runs, or a page
# cannot be
# opened, the picture is a plain card with the template's name instead.
#
# The second form writes only the plain cards into <folder>. The copies in
# site/content/en/pages/templates/ are made this way, so the website shows
# a card for each template before the screenshots replace them on Pages.
set -euo pipefail

# Every look the gallery shows: classic first, then the ten templates.
names=(classic ledger atlas keystone compass harbor wire workbench almanac meridian folio)
width=1280
height=800

# The name with a capital first letter, as the gallery writes it.
title() { printf '%s%s' "$(printf '%s' "${1:0:1}" | tr '[:lower:]' '[:upper:]')" "${1:1}"; }

# card <name> <home|post> <file>: the plain card, a light browser window
# with the template's name and the outline of the page it stands for.
card() {
  local name="$1" kind="$2" file="$3" label shape
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
  done
  echo "Wrote ${#names[@]} pairs of plain cards into $into."
  exit 0
fi

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
python3 -m http.server "$port" --bind 127.0.0.1 --directory "$root" >"$work/server.log" 2>&1 &
server=$!
base="http://127.0.0.1:$port"
for _ in $(seq 50); do
  if python3 -c "import urllib.request; urllib.request.urlopen('$base$mount')" 2>/dev/null; then break; fi
  sleep 0.2
done

# Turns the screenshot into a WebP when ImageMagick is here; otherwise the
# picture holds the PNG as Chromium wrote it.
magick=""
if command -v magick >/dev/null 2>&1; then magick="magick"; elif command -v convert >/dev/null 2>&1; then magick="convert"; fi

# shot <name> <home|post> <address>: the picture of one page, or the plain
# card when the browser draws nothing.
shot() {
  local name="$1" kind="$2" address="$3" png="$work/$1-$2.png" file="$pages/templates/$1-$2.svg" kind_of="image/png" data
  timeout 90 "$chrome" --headless --no-sandbox --disable-gpu --hide-scrollbars \
    --force-device-scale-factor=0.5 --window-size="$width,$height" \
    --run-all-compositor-stages-before-draw --virtual-time-budget=3000 \
    --screenshot="$png" "$base$address" >/dev/null 2>&1 || true
  if [ ! -s "$png" ]; then
    echo "Chromium drew nothing for $address, so $name's $kind picture is the plain card."
    card "$name" "$kind" "$file"
    return
  fi
  if [ -n "$magick" ] && "$magick" "$png" -quality 82 "$work/$1-$2.webp" 2>/dev/null; then
    png="$work/$1-$2.webp"
    kind_of="image/webp"
  fi
  data="$(base64 <"$png" | tr -d '\n')"
  printf '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 %s %s" width="%s" height="%s" role="img"><title>The %s template: %s</title><image width="%s" height="%s" href="data:%s;base64,%s"/></svg>\n' \
    "$width" "$height" "$width" "$height" "$(title "$name")" "$([ "$kind" = home ] && echo "Home page" || echo "A post")" \
    "$width" "$height" "$kind_of" "$data" >"$file"
  echo "Drew $name's $kind page."
}

for name in "${names[@]}"; do
  if [ "$name" = classic ]; then demo="${mount}demo/"; else demo="${mount}templates/$name/"; fi
  if [ ! -f "$pages/${demo#"$mount"}index.html" ]; then
    echo "There is no $name demo in $pages, so its pictures are plain cards."
    card "$name" home "$pages/templates/$name-home.svg"
    card "$name" post "$pages/templates/$name-post.svg"
    continue
  fi
  shot "$name" home "$demo"
  shot "$name" post "$demo$post"
done
echo "The template previews are in $pages/templates."
