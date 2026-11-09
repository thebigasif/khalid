#!/usr/bin/env bash
# Turns the GitHub Pages copy of Khalid into forwarders to serpdad.com
# (plan U16, KTD6). serpdad.com/khalid/ is Khalid's one home, so Pages keeps
# only one small page per old address. Each page has a meta refresh, a
# canonical link and a visible link to the new address, and says noindex.
#
# Usage: scripts/pages-forward.sh <pages folder> <bundle folder> <output folder>
#        scripts/pages-forward.sh --check <output folder>
#        scripts/pages-forward.sh --selftest
#
# <pages folder> is a full Pages build from scripts/pages.sh: its pages are
# the old addresses. <bundle folder> is the tree scripts/demos-bundle.sh
# packs for serpdad.com: a path it has is a path serpdad.com serves. The
# forwarders are written to <output folder>, which must not exist yet.
#
# Where each old address goes, under https://serpdad.com/khalid/:
#
#   demo/<path>/              the same path when the bundle has it, else demo/
#   examples/<slug>/<path>/   the same path when the bundle has it, else the
#                             example's front page, else the landing page's demos
#   templates/                the template gallery, templates/
#   templates/<name>/         the template's page, templates/<name>/
#   templates/<name>/<path>/  the same path in the template's demo when the
#                             bundle has it, else the demo's front page: the
#                             demo is demos/<name>/, or demo/ for classic
#   the website's own pages   start/ to docs/getting-started/, compare/ to
#                             compare/, each use case to its example, each
#                             feature to its section of the landing page,
#                             and every other page to the landing page
#
# Any other address, such as an old docs page, meets 404.html, which sends
# the same path to serpdad.com. Files that are not pages, such as
# stylesheets, feeds and the old khalid-demos.tar.gz, are left out: the
# serpdad.com build reads the bundles from the release named "demos".
#
# --check opens every distinct new address the forwarders name and stops
# unless each answers 200 without a redirect. Run it after serpdad.com
# serves the same bundle, since a page new in this commit is not there yet.
# --selftest builds forwarders from a small made-up Pages build and bundle,
# with no network, and stops on any address that goes to the wrong place.
set -euo pipefail

forward() {
  python3 -I - "$@" <<'EOF'
import html
import pathlib
import sys

HOME = "https://serpdad.com/khalid/"

# The website's own pages that have a better place than the landing page.
SITE = {
    "start/": "docs/getting-started/",
    "compare/": "compare/",
    "templates/": "templates/",
    "use-cases/": "#demos",
    "features/seo-ai-search/": "#seo",
    "features/ai-visitors/": "#seo",
    "features/blog-hosting/": "#speed",
    "features/blog-backups/": "#security",
}

# Each use case's example under examples/.
USE_CASES = {
    "agency-blog": "agency",
    "local-business-blog": "local-business",
    "membership-blog": "membership",
    "multilingual-blog": "multilingual",
    "newsletter-writers": "newsletter",
    "saas-blog": "saas",
}

old, bundle, out = (pathlib.Path(arg) for arg in sys.argv[1:4])
if out.exists():
    sys.exit(f"The output folder {out} already exists. Forwarders are written into a new folder, so nothing old is published. Remove it or name another folder.")
for folder in (old, bundle):
    if not (folder / "index.html").is_file() and not (folder / "demo" / "index.html").is_file():
        sys.exit(f"{folder} holds no built site. Name the Pages build from scripts/pages.sh and the bundle from scripts/demos-bundle.sh.")


def served(path):
    """Whether serpdad.com serves the page at `path` from the bundle."""
    page = bundle / path / "index.html" if path.endswith("/") or path == "" else bundle / path
    return page.is_file()


def first_served(*paths):
    """The first of `paths` the bundle serves, or None."""
    return next((path for path in paths if path is not None and served(path)), None)


def target(path):
    """The new address, under /khalid/, of the old address `path`."""
    parts = path.split("/")
    top = parts[0]
    if top == "demo":
        return first_served(path) or "demo/"
    if top == "examples" and len(parts) > 2:
        return first_served(path, f"examples/{parts[1]}/") or "#demos"
    if top == "templates" and len(parts) > 2:
        name, rest = parts[1], "/".join(parts[2:])
        if rest == "":
            return f"templates/{name}/"
        homes = [f"demos/{name}/"] + (["demo/"] if name == "classic" else [])
        for home in homes:
            if served(home):
                return first_served(home + rest) or home
        return f"templates/{name}/"
    if top == "use-cases" and len(parts) == 3:
        example = USE_CASES.get(parts[1])
        return f"examples/{example}/" if example and served(f"examples/{example}/") else "#demos"
    return SITE.get(path, "")


def page(to):
    """A forwarder to the full address `to`."""
    to = html.escape(to, quote=True)
    return (
        '<!doctype html>\n<html lang="en"><head><meta charset="utf-8">'
        '<meta name="viewport" content="width=device-width, initial-scale=1">'
        f'<title>Khalid moved to serpdad.com</title><meta name="robots" content="noindex">'
        f'<link rel="canonical" href="{to}"><meta http-equiv="refresh" content="0; url={to}">'
        '</head><body><p>Khalid lives at serpdad.com now. '
        f'This page moved to <a href="{to}">{to}</a>.</p></body></html>\n'
    )


count = 0
for file in sorted(old.rglob("*.html")):
    relative = file.relative_to(old).as_posix()
    if relative == "404.html":
        continue
    path = relative[: -len("index.html")] if relative.endswith("index.html") else relative
    written = out / relative
    written.parent.mkdir(parents=True, exist_ok=True)
    written.write_text(page(HOME + target(path)), encoding="utf-8")
    count += 1
if count == 0:
    sys.exit(f"{old} has no pages, so there is nothing to forward.")

# Any address the old build did not have keeps its path on serpdad.com.
(out / "404.html").write_text(
    '<!doctype html>\n<html lang="en"><head><meta charset="utf-8">'
    '<meta name="viewport" content="width=device-width, initial-scale=1">'
    '<title>Khalid moved to serpdad.com</title><meta name="robots" content="noindex">'
    f'<meta http-equiv="refresh" content="1; url={HOME}">'
    '<script>location.replace("https://serpdad.com" + location.pathname + location.search + location.hash);</script>'
    f'</head><body><p>Khalid lives at serpdad.com now: <a href="{HOME}">{HOME}</a>.</p></body></html>\n',
    encoding="utf-8",
)
(out / "robots.txt").write_text("User-agent: *\nAllow: /\n", encoding="utf-8")
print(f"Wrote {count} forwarders to serpdad.com in {out}.")
EOF
}

check() {
  python3 -I - "$1" <<'EOF'
import concurrent.futures
import pathlib
import re
import sys
import urllib.error
import urllib.request

out = pathlib.Path(sys.argv[1])
targets = {}
for file in out.rglob("*.html"):
    if file.name == "404.html":
        continue
    found = re.search(r'http-equiv="refresh" content="0; url=([^"]+)"', file.read_text(encoding="utf-8"))
    if not found:
        sys.exit(f"{file} is not a forwarder. Build the folder again with scripts/pages-forward.sh.")
    targets.setdefault(found.group(1).split("#")[0], file.relative_to(out).as_posix())


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, *args):
        return None


opener = urllib.request.build_opener(NoRedirect)


def status(url):
    request = urllib.request.Request(url, headers={"User-Agent": "khalid-pages-forward-check"})
    try:
        with opener.open(request, timeout=30) as response:
            return response.status
    except urllib.error.HTTPError as error:
        return error.code
    except OSError as error:
        return str(error)


with concurrent.futures.ThreadPoolExecutor(8) as pool:
    answers = dict(zip(targets, pool.map(status, targets)))
bad = {url: code for url, code in answers.items() if code != 200}
for url, code in sorted(bad.items()):
    print(f"{code} {url} (forwarded from {targets[url]})")
if bad:
    sys.exit(f"{len(bad)} of {len(answers)} new addresses do not answer 200.")
print(f"All {len(answers)} new addresses answer 200.")
EOF
}

selftest() {
  local work
  work="$(mktemp -d)"
  trap 'rm -rf "$work"' RETURN
  local page
  for page in index.html start/index.html features/blog-editor/index.html features/seo-ai-search/index.html \
    use-cases/saas-blog/index.html use-cases/index.html demo/index.html demo/posts/kept/index.html \
    demo/posts/gone/index.html examples/saas/index.html examples/saas/about/index.html \
    templates/index.html templates/harbor/index.html templates/harbor/posts/kept/index.html \
    templates/harbor/posts/gone/index.html templates/classic/posts/kept/index.html 404.html; do
    mkdir -p "$(dirname "$work/old/$page")"
    echo old >"$work/old/$page"
  done
  for page in demo/index.html demo/posts/kept/index.html examples/saas/index.html \
    demos/harbor/index.html demos/harbor/posts/kept/index.html; do
    mkdir -p "$(dirname "$work/bundle/$page")"
    echo new >"$work/bundle/$page"
  done
  forward "$work/old" "$work/bundle" "$work/out" >/dev/null
  local failed=0 expected
  while read -r old expected; do
    if ! grep -qF "content=\"0; url=https://serpdad.com/khalid/$expected\"" "$work/out/$old"; then
      echo "FAIL $old should go to /khalid/$expected"
      failed=1
    fi
  done <<'MAP'
index.html
start/index.html docs/getting-started/
features/blog-editor/index.html
features/seo-ai-search/index.html #seo
use-cases/saas-blog/index.html examples/saas/
use-cases/index.html #demos
demo/posts/kept/index.html demo/posts/kept/
demo/posts/gone/index.html demo/
examples/saas/about/index.html examples/saas/
templates/index.html templates/
templates/harbor/index.html templates/harbor/
templates/harbor/posts/kept/index.html demos/harbor/posts/kept/
templates/harbor/posts/gone/index.html demos/harbor/
templates/classic/posts/kept/index.html demo/posts/kept/
MAP
  if ! grep -q 'location.replace("https://serpdad.com" + location.pathname' "$work/out/404.html"; then
    echo "FAIL 404.html should send any other path to serpdad.com"
    failed=1
  fi
  if [ "$(find "$work/out" -type f | wc -l)" -ne 18 ]; then
    echo "FAIL the output should hold 16 forwarders, 404.html and robots.txt"
    failed=1
  fi
  [ "$failed" -eq 0 ] && echo "The forward map sends every old address to the right place."
  return "$failed"
}

case "${1:-}" in
  --check) check "${2:?Name the folder of forwarders to check.}" ;;
  --selftest) selftest ;;
  *)
    if [ $# -ne 3 ]; then
      echo "Usage: scripts/pages-forward.sh <pages folder> <bundle folder> <output folder>, --check <output folder> or --selftest"
      exit 2
    fi
    forward "$@"
    ;;
esac
