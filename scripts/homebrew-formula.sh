#!/bin/sh
# Writes the Homebrew formula for one Khalid release to standard output.
# A formula is the recipe `brew install` follows: where to download each
# file, its checksum, and what to put where. It points at the binaries the
# release already has, so Homebrew builds nothing.
#
#   scripts/homebrew-formula.sh v1.9.0 SHA256SUMS > khalid.rb
#
# The checksums come from the release's own SHA256SUMS. The Homebrew
# workflow runs this when a release is published and puts the file in the
# tap repository, thebigasif/homebrew-khalid (docs/deploy.md).
set -eu

repo="https://github.com/thebigasif/khalid"

# Every message says what happened, why, and what to do.
fail() {
  printf 'No formula was written. %s\n' "$1" >&2
  shift
  for line in "$@"; do printf '%s\n' "$line" >&2; done
  exit 1
}

[ $# -eq 2 ] || fail "The script needs a release name and its SHA256SUMS file." \
  "Run it like: scripts/homebrew-formula.sh v1.9.0 SHA256SUMS"
tag="$1"
sums="$2"

version="${tag#v}"
case "$tag" in
  v*-*) fail "$tag is a prerelease, and Homebrew only gets finished releases." \
    "Publish a release without a dash in its name." ;;
esac
case "$version" in
  *[!0-9.]* | *..* | .* | *.) version="" ;;
esac
case "$tag" in
  v[0-9]*.[0-9]*.[0-9]*) [ -n "$version" ] || fail "$tag is not a release name." "Write it like v1.9.0." ;;
  *) fail "$tag is not a release name." "Write it like v1.9.0." ;;
esac

[ -r "$sums" ] || fail "The file $sums cannot be read." \
  "Download SHA256SUMS from the release's page, then run the script again."

# The checksum of the release file `name`, or a refusal that names it.
sum_of() {
  found=$(awk -v name="$1" '$2 == name || $2 == "*" name { print $1 }' "$sums")
  [ -n "$found" ] || fail "$sums has no line for $1, so its download could not be checked." \
    "Use the SHA256SUMS of a release built by the release workflow."
  # A second line for the same file shows up as a newline, which is not hex.
  case "$found" in
    *[!0-9a-f]*) found="" ;;
  esac
  [ ${#found} -eq 64 ] || fail "The line for $1 in $sums is not one SHA-256 checksum." \
    "Use the SHA256SUMS of a release built by the release workflow."
  printf '%s' "$found"
}

# The url and sha256 lines for the binary of one platform, as named by
# the build matrix in .github/workflows/release.yml.
binary() {
  name="khalid-$tag-$1"
  sum=$(sum_of "$name")
  printf '      url "%s"\n' "$repo/releases/download/$tag/$name"
  printf '      sha256 "%s"\n' "$sum"
}

mac_arm=$(binary macos-aarch64)
mac_intel=$(binary macos-x86_64)
linux_arm=$(binary linux-aarch64)
linux_intel=$(binary linux-x86_64)
man_name="khalid-$tag-man.tar.gz"
man_sum=$(sum_of "$man_name")

cat <<EOF
# Written by scripts/homebrew-formula.sh in thebigasif/khalid for $tag.
# Each release writes this file again, so change the script, not this file.
class Khalid < Formula
  desc "Open-source blog engine with SEO defaults built in, by SerpDad"
  homepage "$repo"
  version "$version"
  license "AGPL-3.0-only"

  on_macos do
    on_arm do
$mac_arm
    end
    on_intel do
$mac_intel
    end
  end

  on_linux do
    on_arm do
$linux_arm
    end
    on_intel do
$linux_intel
    end
  end

  resource "man" do
    url "$repo/releases/download/$tag/$man_name"
    sha256 "$man_sum"
  end

  def install
    bin.install Dir["khalid-v*"].first => "khalid"
    chmod 0755, bin/"khalid"
    resource("man").stage { man1.install Dir["*.1"] }
    generate_completions_from_executable(bin/"khalid", "completions")
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/khalid --version")
    system bin/"khalid", "init", testpath/"blog", "--address", "https://blog.example.com", "--yes"
    assert_path_exists testpath/"blog/site.toml"
  end
end
EOF
