#!/usr/bin/env bash
# The Playwright browsers and the Ubuntu packages they need, on a GitHub
# runner. Used by the demo jobs in .github/workflows/ci.yml.
#
# `playwright install --with-deps` runs `apt-get update` and then
# `apt-get install` on every run. The runner's first mirror,
# azure.archive.ubuntu.com, often stalls: each file waits about 30 s before
# apt falls back to archive.ubuntu.com, and an index download can hang for
# the whole step. A `timeout` around npx then stops npx but not the apt-get
# that sudo started, so the next try fails at once on the dpkg lock with
# exit code 100. This script avoids all three:
#
#   - The browsers come from Playwright's own download, with no apt at all,
#     and a timeout that stops the whole download.
#   - Only the packages the runner lacks are installed (for Chromium these
#     are a few fonts), from the package lists the runner image already has.
#     `apt-get update` runs only when that install fails.
#   - apt gives up on a silent mirror after 15 s and tries the next, waits
#     for the dpkg lock instead of failing, and runs under `sudo timeout`,
#     so the process that holds the lock is the one stopped.
#   - The downloaded .deb files and the browsers go in folders the workflow
#     keeps with actions/cache, under the key `key` prints. A run that finds
#     them installs without touching a mirror or the browser download.
#
# Usage:
#   scripts/ci/browsers.sh key <browser>...      cache key, to GITHUB_OUTPUT
#   scripts/ci/browsers.sh install <browser>...  browsers and packages
# Run `npm install --prefix scripts/demo-check` before `install`.
# On a laptop, `npx --prefix scripts/demo-check playwright install
# --with-deps <browser>` stays the simpler way.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"
tools=scripts/demo-check
# The workflow caches this folder and ~/.cache/ms-playwright.
debs="$HOME/.cache/khalid/apt-archives"

stop() {
  echo "$1" >&2
  exit 1
}

command="${1:-}"
shift || true
[ "$#" -gt 0 ] || stop "Name the browsers, for example: scripts/ci/browsers.sh install chromium"

# The key changes with the runner image, whose package lists decide which
# .deb files apt wants, and with the Playwright version, which decides the
# browsers.
key() {
  local version
  version="$(node -p "require('./$tools/package.json').devDependencies.playwright")"
  local key="browsers-${ImageOS:-linux}-${ImageVersion:-unknown}-playwright-$version-$(IFS=-; echo "$*")"
  if [ -n "${GITHUB_OUTPUT:-}" ]; then
    echo "key=$key" >> "$GITHUB_OUTPUT"
  else
    echo "$key"
  fi
}

# The packages Playwright would install, read from its dry run, whose one
# command line ends `apt-get install -y --no-install-recommends <packages>`.
wanted_packages() {
  npx --prefix "$tools" playwright install-deps --dry-run "$@" \
    | sed -n 's/.*--no-install-recommends//p' \
    | tr -d "\"'" \
    | tr ' ' '\n' \
    | sed '/^$/d' \
    | sort -u
}

# Settings for every apt-get below, and for Playwright's own if it runs.
apt_settings() {
  mkdir -p "$debs/partial"
  sudo tee /etc/apt/apt.conf.d/90khalid-ci > /dev/null << EOF
Acquire::Retries "3";
Acquire::http::Timeout "15";
Acquire::https::Timeout "15";
DPkg::Lock::Timeout "300";
APT::Keep-Downloaded-Packages "true";
Dir::Cache::Archives "$debs/";
EOF
}

# One apt-get run, stopped after its limit. A stopped run can leave dpkg
# half done, which the next run would refuse, so that is finished first.
apt_run() {
  local limit="$1"
  shift
  if sudo timeout -k 20 "$limit" apt-get "$@"; then
    return 0
  fi
  sudo dpkg --configure -a || true
  return 1
}

install_browsers() {
  local try
  for try in 1 2; do
    if timeout -k 10 240 npx --prefix "$tools" playwright install "$@"; then
      return 0
    fi
    echo "The browser download stalled or failed (try $try of 2)."
  done
  stop "The browsers did not download. Playwright's download server may be down; run the job again later."
}

install_packages() {
  local wanted missing=() package
  wanted="$(wanted_packages "$@" || true)"
  if [ -z "$wanted" ]; then
    # Playwright changed how its dry run reads. Let it install the packages
    # itself; the apt settings above still apply.
    echo "Could not read Playwright's package list, so Playwright installs the packages."
    timeout -k 20 600 npx --prefix "$tools" playwright install-deps "$@"
    return
  fi
  for package in $wanted; do
    dpkg-query -W -f='${db:Status-Abbrev}' "$package" 2> /dev/null | grep -q '^ii' || missing+=("$package")
  done
  if [ "${#missing[@]}" -eq 0 ]; then
    echo "The runner already has every package the browsers need."
    return 0
  fi
  echo "Installing ${#missing[@]} packages the runner lacks: ${missing[*]}"
  local install=(install -y --no-install-recommends "${missing[@]}")
  if apt_run 300 "${install[@]}"; then
    return 0
  fi
  echo "The install from the runner's package lists failed, so the lists are refreshed first."
  apt_run 180 update || echo "Refreshing the lists failed; installing with what came."
  apt_run 300 "${install[@]}" || stop "The browsers' packages did not install. An Ubuntu mirror may be down; run the job again later."
}

case "$command" in
  key)
    key "$@"
    ;;
  install)
    apt_settings
    install_browsers "$@"
    install_packages "$@"
    # apt left its files owned by root. actions/cache saves them as the
    # runner user.
    sudo rm -f "$debs/lock"
    sudo chown -R "$(id -u):$(id -g)" "$debs"
    ;;
  *)
    stop "Say key or install, then the browsers, for example: scripts/ci/browsers.sh install chromium"
    ;;
esac
