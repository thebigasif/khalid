#!/usr/bin/env bash
# Ghost's own Content API client against Khalid (U26, KTD13). Serves a
# copy of the showcase site with `ghost_content_api = true`, makes a
# content key for this run only, and runs scripts/ghost-client/check.mjs
# with @tryghost/content-api at the version scripts/ghost-client/package.json
# pins. The key is never printed.
#
# Needs Node 22 or later.
#
# Usage: scripts/ghost-client.sh
# KHALID_BIN names a built khalid to use instead of building one.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
work="$root/target/ghost-client"
port=18369

stop() { echo "$1"; exit 1; }
command -v node >/dev/null || stop "Node is missing. Ghost's client runs on it. Install Node 22 or later, then run this again."

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
# The setting is a top-level key, so it goes above the first table.
printf 'ghost_content_api = true\n%s\n' "$(cat "$work/site/site.toml")" > "$work/site/site.toml"
mount="$(sed -n 's/^mount = "\(.*\)"$/\1/p' "$work/site/site.toml")"
mount="${mount%/}"

# A content key made for this run only. It is never printed.
key="$("$bin" --site "$work/site" keys create --kind content --name "Ghost client check" --json \
  | node -e 'let s = ""; process.stdin.on("data", (d) => (s += d)).on("end", () => process.stdout.write(JSON.parse(s).key))')"
[ -n "$key" ] || stop "khalid keys create gave no key. Ghost's client cannot read without one. Run it by hand on $work/site to see why."

"$bin" --site "$work/site" serve --listen "127.0.0.1:$port" > "$work/serve.log" 2>&1 &
server=$!
trap 'kill "$server" 2>/dev/null || true' EXIT

blog="http://127.0.0.1:$port$mount"
for _ in $(seq 1 120); do
  curl -fsS -o /dev/null -H "Accept-Version: v5.0" "$blog/ghost/api/content/settings/?key=$key" 2>/dev/null && break
  kill -0 "$server" 2>/dev/null || stop "khalid serve stopped before it answered. Its output is in $work/serve.log. Fix what it says, then run this again."
  sleep 0.5
done
curl -fsS -o /dev/null -H "Accept-Version: v5.0" "$blog/ghost/api/content/settings/?key=$key" \
  || stop "Khalid did not answer Ghost's settings within 60 seconds. Read $work/serve.log, fix what it says, then run this again."

npm ci --prefix scripts/ghost-client --no-audit --no-fund --silent
node scripts/ghost-client/check.mjs "$blog" "$key"
