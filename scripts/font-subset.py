#!/usr/bin/env python3
"""Cuts each demo brand's font files to the letters its demo shows.

The sample brands in fixtures/showcase-brands/ carry the Latin files Google
Fonts serves, with every letter of Latin-1 and more, and hinting for old
Windows screens. A demo page shows far fewer letters. This tool keeps, in
each file:

  - every printable letter of basic Latin (U+0020 to U+007E), the
    no-break space and a short list of punctuation (TYPOGRAPHY below);
  - every other letter the brand's built demo uses, in its pages or in
    its stylesheet's generated content, such as German umlauts or arrows;
  - the OpenType features the browser uses by default, such as kerning
    and ligatures, and the numeral and small-capital features the pack's
    stylesheet asks for (font-variant);

and keeps each file's hinting, the instructions that fit letters to the
pixel grid. Chromium on Linux and Windows follows them when it measures
text, so dropping them moves line breaks. Only the families in UNHINTED
lose it, where the demo is over its font budget otherwise and the layout
check passes without it. The outlines, metrics and names are kept, so the
page looks the same and the fallback metrics in tokens.toml still hold. Each file's
`range` in tokens.toml becomes the letters it now carries, so a letter it
lacks is drawn by the next font in the stack.

The OFL counts a cut file as a Modified Version, which may not carry a
Reserved Font Name (OFL FAQ 2.6). A font whose licence reserves its name
is only cut when RENAMED gives it a new one; the tool then renames the
family inside the file and in tokens.toml.

Usage:
  font-subset.py --sites <folder> [brand ...]
    --sites  the built demos, one folder per pack with the site in
             public/, such as target/page-weight/sites after
             scripts/page-weight.sh. Brand <name> is measured on the site
             folder of the same name.
    brand    the brands to cut; every brand with a built site by default.

Needs fontTools and brotli: pip install fonttools brotli
Cutting a file twice keeps it the same, so run it again after the demo
text changes. Then run scripts/page-weight.sh to see the font bytes.
"""

import argparse
import html
import io
import re
import sys
from pathlib import Path

from fontTools import subset
from fontTools.ttLib import TTFont

ROOT = Path(__file__).resolve().parent.parent
BRANDS = ROOT / "fixtures" / "showcase-brands"

# Always kept, used or not: basic Latin, the no-break space, and the
# punctuation and signs a writer types most, so a small edit to a demo does
# not fall back to another font.
TYPOGRAPHY = (
    set(range(0x20, 0x7F))
    | {0x00A0, 0x00A9, 0x00AB, 0x00B0, 0x00B7, 0x00BB, 0x00D7}
    | {0x2013, 0x2014, 0x2018, 0x2019, 0x201A, 0x201C, 0x201D, 0x201E}
    | {0x2022, 0x2026, 0x2039, 0x203A, 0x20AC, 0x2122, 0x2190, 0x2192, 0x2212}
)

# The features a browser turns on by itself for Latin text, as HarfBuzz
# does in Chromium, Firefox and Safari: kerning, ligatures, contextual
# forms, marks and the like. Fractions, small capitals and other numerals
# are drawn only when the stylesheet asks for them (CSS_FEATURES).
DEFAULT_FEATURES = [
    "abvm", "blwm", "calt", "ccmp", "clig", "curs", "dist", "kern", "liga",
    "locl", "ltra", "ltrm", "mark", "mkmk", "rclt", "rlig", "rvrn",
]

# CSS values that turn on OpenType features, and the features they need.
CSS_FEATURES = {
    "tabular-nums": ["tnum"],
    "proportional-nums": ["pnum"],
    "lining-nums": ["lnum"],
    "oldstyle-nums": ["onum"],
    "small-caps": ["smcp"],
    "all-small-caps": ["smcp", "c2sc"],
    "slashed-zero": ["zero"],
    "ordinal": ["ordn"],
}

# Families cut without their hinting, by the name their licence gives
# them, to keep their demo under the font budget. The layout check passes
# for both demos without it.
UNHINTED = {
    "PT Serif",  # Ledger: four faces on its home page, 82 KB hinted
    "Alegreya Sans",  # Chapter: three faces on a post, near the budget hinted
    "Petrona",  # Dialogue: three faces on every page, 67 KB hinted
}

# Features a brand's files leave out although its pack asks for them, so
# the brand's pages stay under the font budget. The browser then draws the
# effect itself, such as small capitals scaled from capitals.
DROPPED_FEATURES = {
    "chapter": {"smcp"},  # EB Garamond's small capitals: 6 KB in each face
}

# Fonts whose licence reserves their name, cut under a new family name
# (OFL FAQ 2.6). The key is the family inside the file and in tokens.toml.
RENAMED = {
    "PT Serif": "Ledger Serif",
    "IBM Plex Sans": "Workbench Sans",
    "Lora": "Showcase Serif",  # Masthead, Quill
    "Playfair Display": "Showcase Display",  # Glossy, Masthead
}


def page_letters(site: Path) -> set[int]:
    """Every character in the site's pages and in the strings of its
    stylesheets' `content` properties."""
    letters: set[int] = set()
    for page in site.rglob("*.html"):
        text = page.read_text(encoding="utf-8", errors="replace")
        text = re.sub(r"<(script|style)\b[^>]*>.*?</\1>", " ", text, flags=re.S)
        letters.update(ord(c) for c in html.unescape(text))
    for sheet in site.rglob("*.css"):
        css = sheet.read_text(encoding="utf-8", errors="replace")
        for value in re.findall(r"content\s*:\s*([^;}]*)", css):
            value = re.sub(r"\\([0-9a-fA-F]{1,6})\s?", lambda m: chr(int(m.group(1), 16)), value)
            for double, single in re.findall(r"\"([^\"]*)\"|'([^']*)'", value):
                letters.update(ord(c) for c in double + single)
    return {c for c in letters if c >= 0x20}


def css_features(site: Path) -> list[str]:
    """The OpenType features the site's stylesheets ask for."""
    wanted: set[str] = set()
    for sheet in site.rglob("*.css"):
        css = sheet.read_text(encoding="utf-8", errors="replace")
        for value in re.findall(r"font-variant[a-z-]*\s*:\s*([^;}]*)", css):
            for word in value.split():
                wanted.update(CSS_FEATURES.get(word, []))
        for value in re.findall(r"font-feature-settings\s*:\s*([^;}]*)", css):
            wanted.update(re.findall(r"[\"']([a-z0-9]{4})[\"']", value))
    return sorted(wanted)


def reserved_names(licence: Path) -> list[str]:
    """The Reserved Font Names an OFL file declares."""
    text = licence.read_text(encoding="utf-8", errors="replace")
    names: list[str] = []
    for match in re.finditer(r"Reserved Font Names?\s+(.*?)(?:\.\s|\n\n|$)", text, flags=re.S):
        names.extend(re.findall(r"[\"'\u201c\u2018]([^\"'\u201d\u2019]+)[\"'\u201d\u2019]", match.group(1)))
    return names


def to_ranges(codes: list[int]) -> str:
    """Code points as a CSS unicode-range, such as U+0020-007E, U+00E4."""
    parts: list[str] = []
    start = previous = None
    for code in sorted(codes):
        if previous is not None and code == previous + 1:
            previous = code
            continue
        if start is not None:
            parts.append(f"U+{start:04X}" if start == previous else f"U+{start:04X}-{previous:04X}")
        start = previous = code
    if start is not None:
        parts.append(f"U+{start:04X}" if start == previous else f"U+{start:04X}-{previous:04X}")
    return ", ".join(parts)


def rename(font: TTFont, old: str, new: str) -> None:
    """Puts `new` in place of `old` in every name the font shows, and in its
    PostScript names without spaces."""
    for record in font["name"].names:
        if record.nameID in (0, 7, 13, 14):
            continue  # copyright, trademark and licence stay as written
        text = record.toUnicode()
        changed = text.replace(old, new).replace(old.replace(" ", ""), new.replace(" ", ""))
        if changed != text:
            record.string = changed


def cut(path: Path, letters: set[int], features: list[str], new_name: str | None, old_name: str) -> tuple[int, str]:
    """Cuts the WOFF2 file at `path` in place. Returns its new size and the
    unicode-range of the letters it now carries."""
    font = TTFont(path)
    kept = sorted(c for c in font.getBestCmap() if c in letters)
    options = subset.Options()
    options.flavor = "woff2"
    options.layout_features = DEFAULT_FEATURES + features
    options.hinting = old_name not in UNHINTED
    options.name_IDs = ["*"]
    options.name_languages = ["*"]
    options.notdef_outline = True
    options.glyph_names = False
    subsetter = subset.Subsetter(options)
    subsetter.populate(unicodes=kept)
    subsetter.subset(font)
    if new_name:
        rename(font, old_name, new_name)
    buffer = io.BytesIO()
    font.flavor = "woff2"
    font.save(buffer)
    path.write_bytes(buffer.getvalue())
    return len(buffer.getvalue()), to_ranges(kept)


def renamed_stack(stack: str, family: str, new_family: str) -> str:
    """A tokens.toml font stack, the text between its quotes, with `family`
    named as `new_family`, quoted or not, such as `Lora, Georgia`."""
    names = [name.strip() for name in stack.split(",")]
    plain = lambda name: name.replace('\\"', "").strip("'")
    names = [f'\\"{new_family}\\"' if plain(name) == family else name for name in names]
    return '"' + ", ".join(names) + '"'


def set_toml(tokens: str, file_name: str, range_text: str, family: str | None, new_family: str | None) -> str:
    """tokens.toml with the [[fonts.files]] entry for `file_name` given its
    new range, and every mention of a renamed family changed."""
    blocks = re.split(r"(?=^\[\[fonts\.files\]\])", tokens, flags=re.M)
    for i, block in enumerate(blocks):
        if re.search(rf'^path = "{re.escape(file_name)}"$', block, flags=re.M):
            blocks[i] = re.sub(r'^range = ".*"$', f'range = "{range_text}"', block, flags=re.M)
    tokens = "".join(blocks)
    if family and new_family:
        tokens = tokens.replace(f'"{family}"', f'"{new_family}"').replace(f'\\"{family}\\"', f'\\"{new_family}\\"')
        tokens = re.sub(r'^((?:heading|body|mono) = )"(.*)"$', lambda m: m[1] + renamed_stack(m[2], family, new_family), tokens, flags=re.M)
    return tokens


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--sites", required=True, type=Path)
    parser.add_argument("brands", nargs="*")
    args = parser.parse_args()
    brands = args.brands or sorted(
        b.name for b in BRANDS.iterdir() if (b / "theme" / "fonts").is_dir() and (args.sites / b.name / "public").is_dir()
    )
    for brand in brands:
        site = args.sites / brand / "public"
        if not site.is_dir():
            print(f"{brand}: no built demo in {site}. Build the demos first with scripts/page-weight.sh.")
            return 1
        letters = TYPOGRAPHY | page_letters(site)
        features = [tag for tag in css_features(site) if tag not in DROPPED_FEATURES.get(brand, set())]
        folder = BRANDS / brand / "theme"
        tokens_path = folder / "tokens.toml"
        tokens = tokens_path.read_text(encoding="utf-8")
        entries = re.findall(r'^family = "(.*)"\n(?:.*\n)*?^path = "(.*)"$', tokens, flags=re.M)
        for family, file_name in entries:
            path = folder / "fonts" / file_name
            original = RENAMED_BACK.get(family, family)
            # Each family's licence sits beside its files, as
            # pt-serif-OFL.txt beside pt-serif-latin-400.woff2.
            licence = folder / "fonts" / f"{file_name.split('-latin')[0]}-OFL.txt"
            if not licence.is_file():
                print(f"{brand}: {file_name} has no licence beside it at {licence.name}, so it is left as it is.")
                continue
            reserved = reserved_names(licence)
            clash = [name for name in reserved if name.lower() in original.lower()]
            new_name = None
            if clash:
                new_name = RENAMED.get(original)
                if not new_name:
                    print(f"{brand}: {file_name} keeps its size: its licence reserves the name {clash[0]}, so a cut copy needs a new name in RENAMED.")
                    continue
            before = path.stat().st_size
            size, range_text = cut(path, letters, features, new_name if family == original else None, original)
            tokens = set_toml(tokens, file_name, range_text, original if new_name and family == original else None, new_name)
            print(f"{brand}: {file_name} {before / 1024:.1f} KB to {size / 1024:.1f} KB")
        tokens_path.write_text(tokens, encoding="utf-8")
    return 0


# A family already renamed, mapped back to the name its licence reserves.
RENAMED_BACK = {new: old for old, new in RENAMED.items()}

if __name__ == "__main__":
    sys.exit(main())
