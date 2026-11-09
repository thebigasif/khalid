#!/usr/bin/env bash
# Runs the server packages the way docs/deploy.md tells a reader to, and
# fetches the home page from each (gap G34). The Rust tests in
# crates/khalid/tests/packages.rs check each package's command line; this
# runs the packages themselves, which needs Docker, or root and systemd.
#
# Usage:
#   scripts/packages-check.sh docker IMAGE
#     The image, as the Docker section says: a site made in ./site, the
#     owner made, the blog served on port 8080. Then the first start that
#     Fly.io, Railway and Coolify use: an empty volume, KHALID_ADDRESS, and
#     the locked-down container compose.yaml asks for.
#   scripts/packages-check.sh systemd BINARY
#     The install script with --systemd, from a release made of BINARY on
#     this machine, then the systemd section's steps. Needs sudo, and
#     changes this machine: run it on a throwaway one, such as CI's.
#
# The owner's sign-in link is a secret, so it is never printed.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
work="$root/target/packages-check"

stop() { echo "$1" >&2; exit 1; }

# Waits up to 60 seconds for `url` to answer 200 with an h1 on the page.
home_answers() {
  local url="$1" page="$work/home.html"
  for _ in $(seq 1 120); do
    if curl -fsS -o "$page" "$url" 2>/dev/null; then
      grep -q '<h1' "$page" || stop "$url answered, but the page has no h1, so it is not a blog home page. It is in $page."
      echo "$url answered the home page."
      return 0
    fi
    sleep 0.5
  done
  return 1
}

check_docker() {
  local image="$1" site="$work/site" me volume="khalid-packages-check"
  me="$(id -u):$(id -g)"
  docker image inspect "$image" >/dev/null || stop "There is no image $image here. Build it with docker build -t $image . first."
  rm -rf "$work"
  mkdir -p "$site"
  # The trap runs after this function ends, so it names the volume itself.
  trap 'docker rm -f khalid-check khalid-check-first >/dev/null 2>&1 || true; docker volume rm -f khalid-packages-check >/dev/null 2>&1 || true' EXIT

  # The Docker section of docs/deploy.md, line by line.
  docker run --rm -u "$me" -v "$site:/site" "$image" init /site --address https://blog.example.com --yes
  docker run --rm -u "$me" -v "$site:/site" "$image" staff owner --email owner@example.com --name "Check Owner" >/dev/null \
    || stop "The image could not make the owner, so nobody could sign in to the admin. Run the same docker run line by hand to see why."
  docker run -d --name khalid-check -u "$me" -p 8080:8080 -v "$site:/site" "$image" >/dev/null
  if ! home_answers http://localhost:8080/; then
    docker logs khalid-check >&2 || true
    stop "The image did not serve the site made as docs/deploy.md says within 60 seconds. Its log is above."
  fi

  # The hosts' first start: an empty volume owned by the image's own user,
  # the address from the environment, and no rights beyond the site folder.
  docker volume create "$volume" >/dev/null
  docker run -d --name khalid-check-first -p 8081:8080 -v "$volume:/site" \
    -e KHALID_ADDRESS=http://localhost:8081 \
    --read-only --tmpfs /tmp --cap-drop ALL --security-opt no-new-privileges:true "$image" >/dev/null
  if ! home_answers http://localhost:8081/; then
    docker logs khalid-check-first >&2 || true
    stop "The image did not make and serve a site on an empty volume within 60 seconds. Fly.io, Railway and Coolify start it that way. Its log is above."
  fi
}

check_systemd() {
  local binary="$1" version="v0.0.0" platform release units file
  [ -x "$binary" ] || stop "$binary is not a program this check can run. Point it at a built khalid."
  case "$(uname -m)" in
    x86_64 | amd64) platform="linux-x86_64" ;;
    aarch64 | arm64) platform="linux-aarch64" ;;
    *) stop "The install script has no build for a $(uname -m) processor, so it cannot be checked here." ;;
  esac
  rm -rf "$work"
  release="$work/releases/$version"
  units="$work/units/$version/packaging/systemd"
  file="khalid-$version-$platform"
  mkdir -p "$release" "$units"
  # A release made of this binary, named as on GitHub, for the script's
  # KHALID_RELEASES and KHALID_UNIT_SOURCE.
  cp "$binary" "$release/$file"
  (cd "$release" && sha256sum "$file" > SHA256SUMS)
  cp "$root/packaging/systemd/khalid.service" "$root/packaging/systemd/khalid.sysusers" "$units/"
  # On any failure, what the service printed comes last.
  trap 'status=$?; [ "$status" -eq 0 ] || sudo journalctl -u khalid --no-pager -n 50 >&2 || true' EXIT

  # The systemd section of docs/deploy.md, through the install script.
  sudo env KHALID_RELEASES="file://$work/releases" KHALID_UNIT_SOURCE="file://$work/units" \
    sh "$root/scripts/install.sh" --version "$version" --systemd
  sudo -u khalid khalid init /var/lib/khalid --address https://blog.example.com --yes
  sudo systemctl enable --now khalid
  if ! home_answers http://127.0.0.1:8080/; then
    sudo systemctl status khalid --no-pager >&2 || true
    stop "The khalid service did not serve the site within 60 seconds. Its status is above and its log below."
  fi
  sudo -u khalid khalid --site /var/lib/khalid staff owner --email owner@example.com --name "Check Owner" >/dev/null \
    || stop "staff owner failed next to the running service, so nobody could sign in to the admin. Run it by hand to see why."
  # How much of the machine the unit can reach, for the log. Lower is safer.
  systemd-analyze security khalid --no-pager | tail -n 1
  sudo systemctl stop khalid
  [ "$(systemctl show khalid -p Result --value)" = "success" ] \
    || stop "The khalid service did not stop cleanly when asked. Its log is below."
  echo "The khalid service started, served the home page and stopped cleanly."
}

case "${1:-}" in
  docker) [ $# -eq 2 ] || stop "Name the image: scripts/packages-check.sh docker khalid"; check_docker "$2" ;;
  systemd) [ $# -eq 2 ] || stop "Name the binary: scripts/packages-check.sh systemd target/release/khalid"; check_systemd "$2" ;;
  *) stop "Say which package to check: scripts/packages-check.sh docker IMAGE, or scripts/packages-check.sh systemd BINARY." ;;
esac
