# shellcheck shell=bash
# The templates Khalid ships, read from the packs folder, for the scripts
# that list or describe them. Sourced, not run.
#
# A shipped template is a folder in crates/khalid-theme/packs/ with a
# pack.toml. A test-only pack, such as trial, has none. pack.toml holds what
# the scripts know about a template and the binary does not read:
#
#   kind        the kind of blog it suits, as the demos gallery groups them
#   kind_label  the heading the gallery shows over that kind
#   frame       its minimum frame in px at 1440 px and wider (layout.mjs)
#
# A new template needs only its own pack.toml; no script lists it by name.

template_packs_folder="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/crates/khalid-theme/packs"

# template_packs: every shipped template, one per line, in name order.
template_packs() {
  local folder
  for folder in "$template_packs_folder"/*/; do
    [ -f "$folder/pack.toml" ] && basename "$folder"
  done
  return 0
}

# template_pack_value <name> <key>: the value of one plain line
# `key = "text"` or `key = 123` in the template's pack.toml, or nothing.
template_pack_value() {
  local file="$template_packs_folder/$1/pack.toml"
  [ -f "$file" ] || return 0
  sed -n -E "s/^$2 = \"?([^\"]*)\"?\$/\\1/p" "$file" | head -n 1
}
