#!/usr/bin/env python3
"""Finds, fetches and credits the photographs of the demo publications.

The photographs are not part of Khalid and are never committed: each
publication's photos.lock names them, and demo-photos.tar.gz on the demos
release holds them (fixtures/showcase-publications/README.md). This tool
is for whoever writes a publication:

  demo-photo.py commons <words> [--count N]
      Wikimedia Commons photos under CC0, public domain, CC BY or CC BY-SA,
      read from Commons' own file pages. Commons needs no key and allows
      far more searches a day than Openverse's 200, so start here. Each
      result's id starts with "commons:" and works with fetch.

  demo-photo.py search <words> [--wide] [--count N] [--source S]
      Openverse photos that allow commercial use and changes, with a
      licence of CC0, the Public Domain Mark, CC BY or CC BY-SA. It looks
      in Wikimedia Commons and the WordPress photo directory, which hand
      out full-size files; --source names others, or "any" for every one.

  demo-photo.py fetch <template> <openverse id> <file> [--width N]
      Reads the photo's licence from Openverse and checks it again on the
      photo's own page, downloads it, makes it at most N px wide (1600 by
      default) as a JPEG, keeps it in the photo cache at <template>/<file>,
      and prints its photos.lock entry. A photo narrower than --min-width
      (1200 by default) is refused: some sources hand out small copies only.
      Needs Pillow.

  demo-photo.py shrink <publications folder> [--width N]
      Makes every cached photo the publications' photos.lock files name at
      most N px wide (1600 by default) and writes its new checksum and size
      into photos.lock. Khalid makes each photo in up to eight widths, so a
      narrower original keeps the demos small. Then pack them again.

  demo-photo.py credits <publication folder>
      Writes ATTRIBUTION.md and the publication's photo credits page,
      content/en/pages/photo-credits/, from its photos.lock.

  demo-photo.py pack <publications folder> <demo-photos.tar.gz>
      Packs every photo the publications' photos.lock files name, from the
      cache, into one archive, the same bytes each time.

The cache is DEMO_PHOTOS_CACHE, by default ~/.cache/khalid/demo-photos, the
folder scripts/demo-publication.sh reads, so a fetched photo is used at
once. Never NC or ND: serpdad.com is a commercial site, and the photos are
cropped and resized.
"""

import argparse
import datetime
import gzip
import hashlib
import io
import json
import os
import pathlib
import re
import sys
import tarfile
import tomllib
import urllib.parse
import urllib.request

API = "https://api.openverse.org/v1/images/"
COMMONS = "https://commons.wikimedia.org/w/api.php"
AGENT = "khalid-demo-photos/1.0 (https://github.com/thebigasif/khalid)"
# The widest a demo photo is kept, and its JPEG quality. The widest picture
# a template draws is about 1216 px, and Khalid makes every smaller width
# from this one, so 1600 px looks sharp and keeps the demos small.
WIDTH = 1600
QUALITY = 80
LICENCES = {
    "cc0": ("CC0 1.0", "https://creativecommons.org/publicdomain/zero/1.0/", False),
    "pdm": ("Public Domain Mark 1.0", "https://creativecommons.org/publicdomain/mark/1.0/", False),
    "by": ("CC BY {v}", "https://creativecommons.org/licenses/by/{v}/", True),
    "by-sa": ("CC BY-SA {v}", "https://creativecommons.org/licenses/by-sa/{v}/", True),
}
# How each licence shows on a photo's own page, to check Openverse's record.
MARKS = {
    "cc0": ["publicdomain/zero", "CC0", "Public Domain", "public domain"],
    "pdm": ["publicdomain/mark", "Public Domain", "public domain", "no known copyright"],
    "by": ["licenses/by/", "CC BY", "cc-by", "Attribution"],
    "by-sa": ["licenses/by-sa/", "CC BY-SA", "cc-by-sa", "ShareAlike"],
}


def cache() -> pathlib.Path:
    named = os.environ.get("DEMO_PHOTOS_CACHE")
    if named:
        return pathlib.Path(named)
    base = os.environ.get("XDG_CACHE_HOME") or str(pathlib.Path.home() / ".cache")
    return pathlib.Path(base) / "khalid" / "demo-photos"


def get(url: str, limit: int = 60_000_000) -> bytes:
    request = urllib.request.Request(url, headers={"User-Agent": AGENT})
    with urllib.request.urlopen(request, timeout=60) as response:
        body = response.read(limit + 1)
    if len(body) > limit:
        sys.exit(f"{url} is larger than {limit} bytes, so it was not fetched.")
    return body


def commons_licence(meta: dict) -> str | None:
    """The demo licence code of a Commons file, or None when it is not one."""
    short = (meta.get("LicenseShortName", {}).get("value") or "").lower()
    code = (meta.get("License", {}).get("value") or "").lower()
    both = f"{short} {code}"
    if re.search(r"\bnc\b|-nc|nd\b|-nd|noncommercial|noderiv", both):
        return None
    if "cc0" in both:
        return "cc0"
    if "public domain" in both or code in ("pd", "pdm") or code.startswith("pd-"):
        return "pdm"
    if re.search(r"cc[- ]by[- ]sa", both):
        return "by-sa"
    if re.search(r"cc[- ]by\b|cc-by-[0-9]", both):
        return "by"
    return None


def commons_photo(title: str) -> dict:
    """A Commons file as Openverse describes a photo, from Commons itself."""
    query = {
        "action": "query", "format": "json", "titles": title, "prop": "imageinfo",
        "iiprop": "url|size|extmetadata", "iiurlwidth": "2000",
    }
    pages = json.loads(get(COMMONS + "?" + urllib.parse.urlencode(query)))["query"]["pages"]
    info = next(iter(pages.values()))["imageinfo"][0]
    meta = info.get("extmetadata", {})
    code = commons_licence(meta)
    version = re.search(r"(\d\.\d)", meta.get("LicenseShortName", {}).get("value") or "")
    plain = lambda html: re.sub(r"<[^>]+>", "", html or "").strip()
    return {
        "id": "commons:" + title,
        "license": code or "unknown",
        "license_version": version.group(1) if version else None,
        "url": info.get("thumburl") or info["url"],
        "width": info.get("width"),
        "title": plain(meta.get("ObjectName", {}).get("value")) or title.removeprefix("File:").rsplit(".", 1)[0],
        "creator": plain(meta.get("Artist", {}).get("value")) or "unknown",
        "foreign_landing_url": info.get("descriptionurl"),
        "source": "Wikimedia Commons",
        "checked": code is not None,
        "licence_name": meta.get("LicenseShortName", {}).get("value"),
        "licence_url": meta.get("LicenseUrl", {}).get("value"),
    }


def commons(args: argparse.Namespace) -> None:
    query = {
        "action": "query", "format": "json", "generator": "search", "gsrnamespace": "6",
        "gsrsearch": " ".join(args.words) + " filetype:bitmap", "gsrlimit": str(args.count * 3),
        "prop": "imageinfo", "iiprop": "size|extmetadata",
    }
    pages = json.loads(get(COMMONS + "?" + urllib.parse.urlencode(query))).get("query", {}).get("pages", {})
    shown = 0
    for page in sorted(pages.values(), key=lambda page: page.get("index", 0)):
        info = page.get("imageinfo", [{}])[0]
        meta = info.get("extmetadata", {})
        code = commons_licence(meta)
        if code is None or (info.get("width") or 0) < 1200:
            continue
        print(json.dumps({
            "id": "commons:" + page["title"],
            "licence": meta.get("LicenseShortName", {}).get("value"),
            "size": f'{info.get("width")}x{info.get("height")}',
            "title": page["title"],
            "creator": re.sub(r"<[^>]+>", "", meta.get("Artist", {}).get("value") or "").strip()[:80],
        }, ensure_ascii=False))
        shown += 1
        if shown == args.count:
            break


def search(args: argparse.Namespace) -> None:
    query = {
        "q": " ".join(args.words),
        "license": "cc0,pdm,by,by-sa",
        "license_type": "commercial,modification",
        "size": "large",
        "page_size": str(args.count),
        "mature": "false",
    }
    if args.wide:
        query["aspect_ratio"] = "wide"
    if args.source != "any":
        query["source"] = args.source
    found = json.loads(get(API + "?" + urllib.parse.urlencode(query)))
    for photo in found.get("results", []):
        print(json.dumps({
            "id": photo["id"],
            "licence": f'{photo["license"]} {photo.get("license_version") or ""}'.strip(),
            "size": f'{photo.get("width")}x{photo.get("height")}',
            "title": photo.get("title"),
            "creator": photo.get("creator"),
            "provider": photo.get("provider"),
            "page": photo.get("foreign_landing_url"),
        }, ensure_ascii=False))


def licence_of(photo: dict) -> tuple[str, str, bool]:
    code = photo.get("license", "")
    if code not in LICENCES:
        sys.exit(f'Photo {photo.get("id")} is under "{code}", which the demos never use. Pick one under CC0, the Public Domain Mark, CC BY or CC BY-SA.')
    name, url, needs_credit = LICENCES[code]
    version = photo.get("license_version") or "4.0"
    return name.format(v=version), url.format(v=version), needs_credit


def checked_on_page(photo: dict) -> bool:
    page = photo.get("foreign_landing_url")
    if not page:
        return False
    try:
        text = get(page, 8_000_000).decode("utf-8", "replace")
    except OSError:
        return False
    if re.search(r"licenses/[a-z-]*-n[cd][/-]|NonCommercial|NoDerivatives", text) and photo["license"] in ("by", "by-sa"):
        # A page that also names a non-commercial or no-derivatives licence
        # needs a person to read it.
        return False
    return any(mark in text for mark in MARKS[photo["license"]])


def fetch(args: argparse.Namespace) -> None:
    from PIL import Image, ImageOps

    if ".." in pathlib.PurePosixPath(args.file).parts or args.file.startswith("/"):
        sys.exit(f"{args.file} is not a path inside content/. Name it as en/posts/<post>/lead.jpg.")
    if not args.file.endswith(".jpg"):
        sys.exit(f"{args.file} should end in .jpg: every demo photo is kept as a JPEG.")
    if args.id.startswith("commons:"):
        photo = commons_photo(args.id.removeprefix("commons:"))
    else:
        photo = json.loads(get(API + urllib.parse.quote(args.id) + "/"))
    name, url, needs_credit = licence_of(photo)
    # A Commons file names its exact licence, such as a country's version.
    name = photo.get("licence_name") or name
    url = photo.get("licence_url") or url
    if not photo.get("checked") and not checked_on_page(photo):
        sys.exit(
            f'The page of photo {args.id}, {photo.get("foreign_landing_url")}, does not show its {name} licence where this tool can read it. '
            "Open the page, and only use the photo when the page says the same licence; otherwise pick another."
        )
    picture = Image.open(io.BytesIO(get(photo["url"])))
    picture = ImageOps.exif_transpose(picture).convert("RGB")
    if picture.width < args.min_width:
        sys.exit(f"Photo {args.id} comes only {picture.width} px wide from {photo['url']}, under the {args.min_width} px a demo needs. Pick another, such as one from --source wikimedia.")
    if picture.width > args.width:
        picture = picture.resize((args.width, round(picture.height * args.width / picture.width)), Image.LANCZOS)
    out = io.BytesIO()
    picture.save(out, "JPEG", quality=QUALITY, optimize=True, progressive=True)
    data = out.getvalue()
    target = cache() / args.template / args.file
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(data)
    today = datetime.date.today().isoformat()
    # A source's field can hold several lines, such as a contact note under
    # the author's name: the credit keeps the first.
    first = lambda text, fallback: (str(text or "").strip().splitlines() or [fallback])[0].strip() or fallback
    title = first(photo.get("title"), "Untitled")
    creator = first(photo.get("creator"), "unknown")
    provider = photo.get("source") or photo.get("provider") or "Openverse"
    # A TOML basic string, with quotes and backslashes escaped.
    text = lambda value: json.dumps(str(value), ensure_ascii=False)
    print("[[photos]]")
    print(f"file = {text(args.file)}")
    print(f'sha256 = "{hashlib.sha256(data).hexdigest()}"')
    print(f"source = {text(photo.get('foreign_landing_url'))}")
    print(f"licence = {text(f'{name}, read on the photo' + chr(39) + f's page {today}: {url}')}")
    print(f"credit = {text(f'{title} by {creator}, via {provider}')}")
    print(f"title = {text(title)}")
    print(f"creator = {text(creator)}")
    print(f"openverse = {text(args.id)}")
    print(f"needs_credit = {'true' if needs_credit else 'false'}")
    print(f"size = \"{picture.width}x{picture.height}\"")
    print()


def credits(args: argparse.Namespace) -> None:
    folder = pathlib.Path(args.publication)
    lock = tomllib.loads((folder / "photos.lock").read_text(encoding="utf-8"))
    site = tomllib.loads((folder / "site.toml").read_text(encoding="utf-8"))
    name = site.get("name", folder.name)
    photos = lock.get("photos", [])
    lines = [
        f"# Photographs in {name}",
        "",
        f"{name} is a made-up publication that shows Khalid's {folder.name.capitalize()} template. "
        "Its photographs are not part of Khalid and are not under the AGPL. Each keeps the licence its photographer gave it, listed here. "
        "The photographs are cropped and resized for the demo.",
        "",
    ]
    page = [
        "---",
        "title: Photo credits",
        "slug: photo-credits",
        f"description: Who took the photographs in {name}, and under which licence each one is shared.",
        "---",
        "",
        f"The photographs in {name} come from photographers who share their work under open licences. Each one is listed here with its licence and a link to where it was first published. The photographs are cropped and resized to fit this site.",
        "",
    ]
    for photo in photos:
        licence, _, rest = photo["licence"].partition(", read on the photo's page ")
        licence_url = rest.split(": ", 1)[1] if ": " in rest else ""
        title = photo.get("title", "Untitled")
        creator = photo.get("creator", "unknown")
        lines.append(f'- `{photo["file"]}`: "{title}" by {creator}, under {licence} ({licence_url}), from {photo["source"]}')
        page.append(f'- "[{title}]({photo["source"]})" by {creator}, under [{licence}]({licence_url}).')
    (folder / "ATTRIBUTION.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    target = folder / "content" / "en" / "pages" / "photo-credits"
    target.mkdir(parents=True, exist_ok=True)
    (target / "index.md").write_text("\n".join(page) + "\n", encoding="utf-8")
    print(f"Credited {len(photos)} photographs in {folder / 'ATTRIBUTION.md'} and {target / 'index.md'}.")


def shrink(args: argparse.Namespace) -> None:
    from PIL import Image

    shrunk = 0
    for lock_file in sorted(pathlib.Path(args.publications).glob("*/photos.lock")):
        template = lock_file.parent.name
        lock = lock_file.read_text(encoding="utf-8")
        for photo in tomllib.loads(lock).get("photos", []):
            path = cache() / template / photo["file"]
            data = path.read_bytes() if path.is_file() else b""
            if hashlib.sha256(data).hexdigest() != photo["sha256"]:
                sys.exit(f"{template}/{photo['file']} is missing from {cache()} or differs from its photos.lock checksum. Fetch it again.")
            picture = Image.open(io.BytesIO(data))
            if picture.width <= args.width:
                continue
            picture = picture.convert("RGB").resize((args.width, round(picture.height * args.width / picture.width)), Image.LANCZOS)
            out = io.BytesIO()
            picture.save(out, "JPEG", quality=QUALITY, optimize=True, progressive=True)
            path.write_bytes(out.getvalue())
            entry = f'file = {json.dumps(photo["file"], ensure_ascii=False)}\nsha256 = "{photo["sha256"]}"'
            if lock.count(entry) != 1:
                sys.exit(f"{lock_file} does not hold one entry for {photo['file']} with its file and sha256 lines together. Fix the entry, then run this again.")
            lock = lock.replace(entry, f'file = {json.dumps(photo["file"], ensure_ascii=False)}\nsha256 = "{hashlib.sha256(out.getvalue()).hexdigest()}"')
            lock = re.sub(
                rf'(file = {re.escape(json.dumps(photo["file"], ensure_ascii=False))}\n(?:.*\n)*?size = )"[0-9]+x[0-9]+"',
                lambda match: f'{match.group(1)}"{picture.width}x{picture.height}"',
                lock,
                count=1,
            )
            shrunk += 1
        lock_file.write_text(lock, encoding="utf-8")
    print(f"Made {shrunk} photographs at most {args.width} px wide. Pack them again and upload the archive.")


def pack(args: argparse.Namespace) -> None:
    root = pathlib.Path(args.publications)
    entries = []
    for lock_file in sorted(root.glob("*/photos.lock")):
        template = lock_file.parent.name
        for photo in tomllib.loads(lock_file.read_text(encoding="utf-8")).get("photos", []):
            path = cache() / template / photo["file"]
            data = path.read_bytes() if path.is_file() else b""
            if hashlib.sha256(data).hexdigest() != photo["sha256"]:
                sys.exit(f"{template}/{photo['file']} is missing from {cache()} or differs from its photos.lock checksum. Fetch it again.")
            entries.append((f"{template}/{photo['file']}", data))
    raw = io.BytesIO()
    with tarfile.open(fileobj=raw, mode="w", format=tarfile.PAX_FORMAT) as archive:
        for name, data in sorted(entries):
            info = tarfile.TarInfo(name)
            info.size, info.mtime, info.mode = len(data), 0, 0o644
            archive.addfile(info, io.BytesIO(data))
    with open(args.out, "wb") as out, gzip.GzipFile(fileobj=out, mode="wb", mtime=0) as packed:
        packed.write(raw.getvalue())
    print(f"Packed {len(entries)} photographs into {args.out}, {os.path.getsize(args.out)} bytes.")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    sub = parser.add_subparsers(dest="command", required=True)
    s = sub.add_parser("search")
    s.add_argument("words", nargs="+")
    s.add_argument("--wide", action="store_true")
    s.add_argument("--count", type=int, default=10)
    s.add_argument("--source", default="wikimedia,wordpress")
    m = sub.add_parser("commons")
    m.add_argument("words", nargs="+")
    m.add_argument("--count", type=int, default=10)
    f = sub.add_parser("fetch")
    f.add_argument("template")
    f.add_argument("id")
    f.add_argument("file")
    f.add_argument("--width", type=int, default=WIDTH)
    f.add_argument("--min-width", type=int, default=1200)
    r = sub.add_parser("shrink")
    r.add_argument("publications")
    r.add_argument("--width", type=int, default=WIDTH)
    c = sub.add_parser("credits")
    c.add_argument("publication")
    p = sub.add_parser("pack")
    p.add_argument("publications")
    p.add_argument("out")
    args = parser.parse_args()
    {"commons": commons, "search": search, "fetch": fetch, "shrink": shrink, "credits": credits, "pack": pack}[args.command](args)


if __name__ == "__main__":
    main()
