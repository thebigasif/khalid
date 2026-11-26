#!/usr/bin/env python3
"""Writes crates/khalid-seo/src/schema/vocabulary.txt from schema.org's own
published tables, so Khalid knows every type and property.

Usage: scripts/schema-vocabulary.py <types.csv> <properties.csv> <release>

The tables are schemaorg-all-https-types.csv and
schemaorg-all-https-properties.csv from
https://github.com/schemaorg/schemaorg/tree/main/data/releases/<release>.
schema.org publishes them under CC BY-SA 3.0.

One record a line, fields split by spaces:

  T Name Parent,Parent [>Newer]    a type and the types it extends
  E Member Enumeration [>Newer]     a member of an enumeration
  P name Domain,Domain Range,Range [>Newer]
                                    a property, the types it belongs to and
                                    the types its values may take

`>Newer` names what replaces a superseded name. Lines are sorted by kind,
then name.
"""
import csv
import sys

PREFIX = "https://schema.org/"


def short(value):
    names = [v.strip() for v in value.split(",") if v.strip()]
    out = []
    for name in names:
        if not name.startswith(PREFIX):
            continue
        out.append(name[len(PREFIX):])
    return ",".join(sorted(set(out)))


def newer(row):
    by = short(row["supersededBy"])
    return f" >{by.split(',')[0]}" if by else ""


def main():
    types_csv, properties_csv, release = sys.argv[1:4]
    lines = []
    with open(types_csv, newline="", encoding="utf-8") as file:
        for row in csv.DictReader(file):
            name = row["label"]
            if not row["id"].startswith(PREFIX):
                continue
            enumeration = short(row["enumerationtype"])
            if enumeration:
                lines.append(f"E {name} {enumeration}{newer(row)}")
            else:
                parents = short(row["subTypeOf"]) or "-"
                lines.append(f"T {name} {parents}{newer(row)}")
    with open(properties_csv, newline="", encoding="utf-8") as file:
        for row in csv.DictReader(file):
            name = row["label"]
            domains = short(row["domainIncludes"]) or "-"
            ranges = short(row["rangeIncludes"]) or "-"
            lines.append(f"P {name} {domains} {ranges}{newer(row)}")
    order = {"T": 0, "E": 1, "P": 2}
    lines.sort(key=lambda line: (order[line[0]], line.split(" ")[1]))
    head = [
        f"# schema.org {release}, every type, enumeration member and property.",
        "# Made by scripts/schema-vocabulary.py from schema.org's published tables.",
        "# schema.org is published under CC BY-SA 3.0, https://schema.org/docs/terms.html",
    ]
    sys.stdout.write("\n".join(head + lines) + "\n")


if __name__ == "__main__":
    main()
