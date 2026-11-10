#!/bin/sh
# Installs Khalid: downloads the binary for this machine from a GitHub
# release, checks it against the release's SHA256SUMS, and puts it in a
# folder on your PATH. With --systemd it also sets up the service.
#
#   curl -fsSL https://raw.githubusercontent.com/thebigasif/khalid/main/scripts/install.sh | sh
#
# Options:
#   --version vX.Y.Z   install this release instead of the newest one
#   --dir FOLDER       install into FOLDER [default: /usr/local/bin as root,
#                      else ~/.local/bin]
#   --systemd          also install the systemd unit and its user (root only)
#   --no-man           leave out the manual pages
#   --no-completions   leave out Tab completion for your shell
#
# It also installs the manual pages and Tab completion for bash, zsh or
# fish: in your home folder, or for everyone under /usr/local/share as
# root. Either one that cannot be done is skipped with a note, and the
# binary stays installed.
#
# KHALID_RELEASES points at another copy of the releases, for mirrors and
# tests. Its files are named as on GitHub: <tag>/khalid-<tag>-<platform>
# and <tag>/SHA256SUMS. KHALID_UNIT_SOURCE does the same for the systemd
# files, read from <tag>/packaging/systemd/.
set -eu

repo="https://github.com/thebigasif/khalid"
releases="${KHALID_RELEASES:-$repo/releases/download}"
unit_source="${KHALID_UNIT_SOURCE:-https://raw.githubusercontent.com/thebigasif/khalid}"

# Every message says what happened, why, and what to do.
fail() {
  printf 'Khalid was not installed. %s\n' "$1" >&2
  shift
  for line in "$@"; do printf '%s\n' "$line" >&2; done
  exit 1
}

version=""
dir=""
systemd="no"
man="yes"
completions="yes"
while [ $# -gt 0 ]; do
  case "$1" in
    --version) [ $# -ge 2 ] || fail "--version needs a release name." "Write it like --version v0.1.0."
      version="$2"; shift 2 ;;
    --dir) [ $# -ge 2 ] || fail "--dir needs a folder." "Write it like --dir /usr/local/bin."
      dir="$2"; shift 2 ;;
    --systemd) systemd="yes"; shift ;;
    --no-man) man="no"; shift ;;
    --no-completions) completions="no"; shift ;;
    *) fail "The option $1 is not one this script knows." "Use --version, --dir, --systemd, --no-man or --no-completions." ;;
  esac
done

case "$(uname -s)" in
  Linux) os="linux" ;;
  Darwin) os="macos" ;;
  *) fail "This script installs on Linux and macOS only." "On Windows, download khalid.exe from $repo/releases." ;;
esac
case "$(uname -m)" in
  x86_64 | amd64) arch="x86_64" ;;
  aarch64 | arm64) arch="aarch64" ;;
  *) fail "There is no Khalid build for a $(uname -m) processor yet." "Build it from source with cargo, as CONTRIBUTING.md says." ;;
esac
platform="$os-$arch"

command -v curl >/dev/null 2>&1 || fail "This script needs curl to download Khalid." "Install curl, then run it again."
if command -v sha256sum >/dev/null 2>&1; then
  sum() { sha256sum "$1" | cut -d ' ' -f 1; }
elif command -v shasum >/dev/null 2>&1; then
  sum() { shasum -a 256 "$1" | cut -d ' ' -f 1; }
else
  fail "This script needs sha256sum or shasum to check the download." "Install coreutils, then run it again."
fi

if [ -z "$version" ]; then
  # GitHub sends /releases/latest on to the newest release's own page.
  latest=$(curl -fsSLI -o /dev/null -w '%{url_effective}' "$repo/releases/latest") ||
    fail "GitHub did not say which release is the newest." "Check the connection, or name one with --version."
  version="${latest##*/}"
fi
case "$version" in
  v[0-9]*) ;;
  *) fail "$version is not a release name." "Release names look like v0.1.0." ;;
esac

if [ "$systemd" = "yes" ] && [ "$(id -u)" != "0" ]; then
  fail "--systemd sets up a service, which needs root." "Run the script with sudo."
fi
if [ -z "$dir" ]; then
  if [ "$(id -u)" = "0" ]; then dir="/usr/local/bin"; else dir="$HOME/.local/bin"; fi
fi

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
file="khalid-$version-$platform"
printf 'Downloading Khalid %s for %s.\n' "$version" "$platform"
curl -fsSL -o "$work/$file" "$releases/$version/$file" ||
  fail "The file $file could not be downloaded." "Check the release name at $repo/releases."
curl -fsSL -o "$work/SHA256SUMS" "$releases/$version/SHA256SUMS" ||
  fail "The release's SHA256SUMS could not be downloaded, so the file cannot be checked." "Try again in a minute."

expected=$(awk -v name="$file" '$2 == name || $2 == "*" name { print $1 }' "$work/SHA256SUMS")
[ -n "$expected" ] || fail "SHA256SUMS has no line for $file, so it cannot be checked." "Report this at $repo/issues."
[ "$(sum "$work/$file")" = "$expected" ] ||
  fail "The download does not match its checksum, so it may be damaged or changed." "Nothing was written. Run the script again."

mkdir -p "$dir"
chmod 0755 "$work/$file"
# Copied beside the old binary first, so the swap is one rename.
cp "$work/$file" "$dir/.khalid.new"
mv -f "$dir/.khalid.new" "$dir/khalid"
printf 'Installed Khalid %s at %s/khalid.\n' "$version" "$dir"

# The extras below never stop the install. Each one says what it did, or
# why it was skipped and what to do instead.
note() { printf '%s\n' "$@"; }
if [ "$(id -u)" = "0" ]; then
  share="/usr/local/share"
else
  share="${XDG_DATA_HOME:-$HOME/.local/share}"
fi

# man finds ~/.local/share/man and /usr/local/share/man on its own, as
# they sit beside a bin folder on the PATH.
install_man() {
  pages="khalid-$version-man.tar.gz"
  mandir="$share/man/man1"
  if ! command -v tar >/dev/null 2>&1; then
    note "The manual pages were not installed. This system has no tar to unpack them." "khalid <command> --help shows the same help."
    return 0
  fi
  if ! curl -fsSL -o "$work/$pages" "$releases/$version/$pages" 2>/dev/null; then
    note "The manual pages were not installed. Release $version has none to download." "khalid <command> --help shows the same help."
    return 0
  fi
  expected=$(awk -v name="$pages" '$2 == name || $2 == "*" name { print $1 }' "$work/SHA256SUMS")
  if [ -z "$expected" ] || [ "$(sum "$work/$pages")" != "$expected" ]; then
    note "The manual pages were not installed. Their download does not match SHA256SUMS, so it may be damaged." "Run the script again to retry."
    return 0
  fi
  # Unpacked aside first, so only the pages themselves reach the man folder.
  mkdir -p "$work/man"
  if ! tar -xzf "$work/$pages" -C "$work/man" 2>/dev/null ||
    ! mkdir -p "$mandir" 2>/dev/null || ! cp "$work"/man/*.1 "$mandir/" 2>/dev/null; then
    note "The manual pages were not installed. They could not be written to $mandir." "Check that you can write there, or run the script with sudo."
    return 0
  fi
  note "Installed the manual pages in $mandir. Read one with: man khalid-init"
}

# Tab completion means the shell finishes a khalid command when you press
# Tab. The installed binary writes the script for the shell you log in with.
install_completions() {
  shell="${SHELL:-}"
  shell="${shell##*/}"
  case "$shell:$(id -u)" in
    bash:*) target="$share/bash-completion/completions/khalid" ;;
    zsh:0) target="$share/zsh/site-functions/_khalid" ;;
    zsh:*) target="$HOME/.zfunc/_khalid" ;;
    fish:0) target="$share/fish/vendor_completions.d/khalid.fish" ;;
    fish:*) target="${XDG_CONFIG_HOME:-$HOME/.config}/fish/completions/khalid.fish" ;;
    *)
      note "Tab completion was not set up. The script sets it up for bash, zsh and fish, and your shell is ${shell:-unknown}." \
        "khalid completions --help lists the other shells, and docs/cli.md shows how to add one."
      return 0 ;;
  esac
  if ! "$dir/khalid" completions "$shell" >"$work/completion" 2>/dev/null; then
    note "Tab completion was not set up. Khalid $version cannot write a completion script yet." "Install a newer release to get it."
    return 0
  fi
  if ! mkdir -p "${target%/*}" 2>/dev/null || ! cp "$work/completion" "$target" 2>/dev/null; then
    note "Tab completion was not set up. It could not be written to $target." "Check that you can write there, then run the script again."
    return 0
  fi
  note "Set up Tab completion for $shell in $target: your shell now finishes khalid commands when you press Tab. Open a new terminal to use it."
  if [ "$target" = "${HOME:-}/.zfunc/_khalid" ] && ! grep -qs 'zfunc' "$HOME/.zshrc"; then
    note "zsh looks there once ~/.zshrc has this line before compinit:" '  fpath=(~/.zfunc $fpath)'
  fi
}

if [ "$man" = "yes" ]; then install_man; fi
if [ "$completions" = "yes" ]; then install_completions; fi

if [ "$systemd" = "yes" ]; then
  if ! curl -fsSL -o "$work/khalid.service" "$unit_source/$version/packaging/systemd/khalid.service" ||
    ! curl -fsSL -o "$work/khalid.sysusers" "$unit_source/$version/packaging/systemd/khalid.sysusers"; then
    printf 'The khalid service was not set up. Its files for %s could not be downloaded.\n' "$version" >&2
    printf 'The binary is installed. Copy the files from packaging/systemd/ by hand, as docs/deploy.md says.\n' >&2
    exit 1
  fi
  sed "s#/usr/local/bin/khalid#$dir/khalid#" "$work/khalid.service" >/etc/systemd/system/khalid.service
  # Ubuntu has systemd-sysusers but no /etc/sysusers.d until something
  # puts a file there. Older systems have no systemd-sysusers at all.
  mkdir -p /etc/sysusers.d
  cp "$work/khalid.sysusers" /etc/sysusers.d/khalid.conf
  if command -v systemd-sysusers >/dev/null 2>&1; then
    systemd-sysusers /etc/sysusers.d/khalid.conf
  elif ! id khalid >/dev/null 2>&1; then
    if ! command -v useradd >/dev/null 2>&1; then
      printf 'The khalid service was not set up. It runs as a khalid user, and this system has no systemd-sysusers or useradd to make one.\n' >&2
      printf 'The binary is installed. Make a system user named khalid with the home folder /var/lib/khalid, then run the script again.\n' >&2
      exit 1
    fi
    useradd --system --user-group --home-dir /var/lib/khalid --no-create-home \
      --shell "$(command -v nologin || echo /bin/false)" --comment "Khalid blog engine" khalid
  fi
  install -d -o khalid -g khalid -m 0750 /var/lib/khalid
  systemctl daemon-reload
  printf 'Installed the khalid service. Make a site, then start it:\n'
  printf '  sudo -u khalid %s/khalid init /var/lib/khalid --address https://blog.example.com --yes\n' "$dir"
  printf '  sudo systemctl enable --now khalid\n'
  exit 0
fi

case ":$PATH:" in
  *":$dir:"*) printf 'Run khalid init to make a site.\n' ;;
  *) printf '%s is not on your PATH. Add it, or run %s/khalid init to make a site.\n' "$dir" "$dir" ;;
esac
