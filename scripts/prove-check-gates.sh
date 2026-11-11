#!/usr/bin/env bash
# Proves each search in check-gates.sh fires: every case builds a throwaway
# repository, runs the gates in it and checks the outcome. A gate that never
# fails guards nothing.
set -euo pipefail

gates="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/check-gates.sh"
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
failures=0

# case <name> <pass|fail> <file> <content>: one repository holding one file.
case_() {
  local name=$1 want=$2 file=$3 content=$4 repo="$scratch/$1"
  mkdir -p "$repo/$(dirname "$file")"
  printf '%s\n' "$content" >"$repo/$file"
  git -C "$repo" init -q
  git -C "$repo" add -A
  local got=pass
  (cd "$repo" && bash "$gates") >"$repo.log" 2>&1 || got=fail
  if [ "$got" = "$want" ]; then
    echo "ok   $name ($want)"
  else
    echo "FAIL $name: wanted $want, got $got"
    cat "$repo.log"
    failures=$((failures + 1))
  fi
}

case_ clean pass crates/a/src/lib.rs 'pub fn a() {}'

many=$(for n in $(seq 48); do echo "pub fn f$n(_: serde_json::Value) {}"; done)
case_ value-over-ceiling fail crates/a/src/lib.rs "$many"
case_ value-in-tests-file pass crates/a/src/lib_tests.rs "$many"
case_ value-at-ceiling pass crates/a/src/lib.rs "$(printf '%s\n' "$many" | sed 1d)"

case_ sql-unexplained fail crates/a/src/lib.rs 'fn q(t: &str) {
    let sql = format!("SELECT * FROM {t}");
    let _ = sqlx::query(AssertSqlSafe(sql));
}'
case_ sql-explained pass crates/a/src/lib.rs 'fn q(t: &str) {
    // The table name comes from the schema, never from input.
    let sql = format!("SELECT * FROM {t}");
    let _ = sqlx::query(AssertSqlSafe(sql));
}'
case_ sql-import-only pass crates/a/src/lib.rs 'use sqlx::AssertSqlSafe;'

case_ deny-warnings fail crates/a/src/lib.rs '#![deny(warnings)]'
case_ deny-warnings-in-list fail crates/a/src/main.rs '#![deny(missing_docs, warnings)]'
case_ deny-other-lint pass crates/a/src/lib.rs '#![deny(missing_docs)]'

if [ "$failures" -gt 0 ]; then
  echo "$failures case(s) went the wrong way."
  exit 1
fi
echo "Every gate fires."
