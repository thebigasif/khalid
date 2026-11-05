#!/usr/bin/env bash
# The starter kits, end to end (U14, R12). Serves the showcase site in
# headless mode with `khalid serve`, then for the Astro kit and the Next.js
# kit, in turn: installs it, builds it as its README says, serves the
# build, and checks its home page and one post with
# scripts/starters-check.mjs. Each page needs a title and a meta
# description in its head, and JSON-LD schema that parses. The content key
# must not reach anything a reader's browser gets.
#
# Needs Node 22.23.4 or later, which the Astro kit asks for.
#
# Usage: scripts/starters-check.sh
# KHALID_BIN names a built khalid to use instead of building one.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
work="$root/target/starters-check"
port=18368

stop() { echo "$1"; exit 1; }
command -v node >/dev/null || stop "Node is missing. The starter kits are built with it. Install Node 22.23.4 or later, then run this again."

# CI hands over the release binary it already built as KHALID_BIN, so this
# check does not build it again. Without it, the binary is built here.
if [ -n "${KHALID_BIN:-}" ]; then
  [ -x "$KHALID_BIN" ] || stop "KHALID_BIN is $KHALID_BIN, which is not a program this check can run. Point it at a built khalid, or unset it to build one."
  bin="$KHALID_BIN"
else
  cargo build --release -p khalid
  bin="${CARGO_TARGET_DIR:-$root/target}/release/khalid"
fi

rm -rf "$work"
mkdir -p "$work"
cp -r fixtures/showcase-site "$work/site"
sed -i 's/^mode = "server"$/mode = "headless"/' "$work/site/site.toml"
grep -q '^mode = "headless"$' "$work/site/site.toml" || stop "The showcase's site.toml has no mode = \"server\" line to switch. The kits need a headless site. Update this script to the new site.toml."

# A content key made for this run only. It is never printed.
key="$("$bin" --site "$work/site" keys create --kind content --name "Starter check" --json \
  | node -e 'let s = ""; process.stdin.on("data", (d) => (s += d)).on("end", () => process.stdout.write(JSON.parse(s).key))')"
[ -n "$key" ] || stop "khalid keys create gave no key. The kits cannot read posts without one. Run it by hand on $work/site to see why."

servers=()
trap 'for pid in "${servers[@]}"; do kill "$pid" 2>/dev/null || true; done' EXIT

# Waits up to 60 seconds for `url` to answer, while `pid` still runs.
wait_for() {
  local url="$1" pid="$2" log="$3"
  for _ in $(seq 1 120); do
    curl -fsS -o /dev/null "$url" 2>/dev/null && return 0
    kill -0 "$pid" 2>/dev/null || stop "A server stopped before it answered. Its output is in $log. Fix what it says, then run this again."
    sleep 0.5
  done
  stop "A server did not answer within 60 seconds. Nothing can be checked without it. Read $log, fix what it says, then run this again."
}

api="http://127.0.0.1:$port"
"$bin" --site "$work/site" serve --listen "127.0.0.1:$port" > "$work/serve.log" 2>&1 &
servers+=($!)
wait_for "$api/api/content/v1/settings/?key=$key" "$!" "$work/serve.log"

failed=0
# kit, its env file, the folders a browser gets, the command that serves it
# and its port.
kits=(
  "astro .env dist npx astro preview --host 127.0.0.1 --port 14321"
  "nextjs .env.local .next/static|.next/server/app npx next start --hostname 127.0.0.1 --port 13000"
)
for line in "${kits[@]}"; do
  read -r kit env_file public serve <<< "$line"
  kit_port="${serve##* }"
  echo "== starters/$kit"
  cp -r "starters/$kit" "$work/$kit"
  printf 'KHALID_API_URL=%s\nKHALID_CONTENT_KEY=%s\nSITE_ADDRESS=https://www.example.com\n' "$api" "$key" > "$work/$kit/$env_file"
  (cd "$work/$kit" && npm install --no-audit --no-fund && npm run build) || { echo "starters/$kit did not build."; failed=1; continue; }
  # No key in a file a browser gets.
  for folder in ${public//|/ }; do
    if grep -rlF -- "$key" "$work/$kit/$folder"; then
      echo "starters/$kit wrote the content key into the files above, which readers can download."
      failed=1
    fi
  done
  (cd "$work/$kit" && exec $serve) > "$work/$kit.log" 2>&1 &
  servers+=($!)
  wait_for "http://127.0.0.1:$kit_port/blog/" "$!" "$work/$kit.log"
  node scripts/starters-check.mjs "http://127.0.0.1:$kit_port" || failed=1
done

[ "$failed" = 0 ] || stop "A starter kit failed. The lines above say which page and what is missing."
echo "Both starter kits built, and their pages carry a title, a meta description and schema."
