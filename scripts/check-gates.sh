#!/usr/bin/env bash
# Source searches that RULES.md promises and no compiler lint can make.
# check.sh runs this from the repository root; prove-check-gates.sh runs it
# in throwaway repositories to show each search fires. Every failure prints
# what was found and how to fix it, and the script stops at the first one.
set -euo pipefail

step() { printf '\n==> %s\n' "$1"; }

# How many lines in library and binary sources may name serde_json::Value.
# A typed struct is the default; Value is for the edges only. Lower this
# number when you remove one. Raising it needs a reason in the pull request.
# Main had 45 on 2026-11-11 and integration 33 brings 2 more.
value_ceiling=47

step "serde_json::Value stays at the edges (BL-TYP-2): at most $value_ceiling lines"
value_lines=$(git grep -nE 'serde_json::(value::)?Value\b|serde_json::\{[^}]*\bValue\b' \
  -- 'crates/*/src/**' ':!*_tests.rs' ':!**/tests.rs' || true)
value_count=$(printf '%s' "$value_lines" | grep -c . || true)
if [ "$value_count" -gt "$value_ceiling" ]; then
  echo "$value_lines"
  echo "$value_count lines name serde_json::Value, and the ceiling is $value_ceiling."
  echo "Read the JSON into a typed struct instead (BL-TYP-2)."
  exit 1
fi
echo "$value_count of $value_ceiling."

step "SQL built at run time says why it is safe (BL-RUN-3)"
# sqlx takes only &'static str as SQL, so text joined at run time must be
# wrapped in AssertSqlSafe. Each wrap needs a comment on its line or on one
# of the three lines above it, saying where every joined piece comes from.
unexplained=$(git ls-files -z -- '*.rs' ':!vendor/**' | xargs -0 -r awk '
  FNR == 1 { c1 = 0; c2 = 0; c3 = 0 }
  /AssertSqlSafe\(/ && !/^[[:space:]]*use / && !/\/\// && !(c1 || c2 || c3) {
    print FILENAME ":" FNR ": " $0
  }
  { c3 = c2; c2 = c1; c1 = ($0 ~ /^[[:space:]]*\/\//) }
')
if [ -n "$unexplained" ]; then
  echo "$unexplained"
  echo "Bind every value with .bind(). If the text joins only names Khalid owns,"
  echo "say where they come from in a comment just above AssertSqlSafe."
  exit 1
fi

step "No #![deny(warnings)] in source (BL-LNT-2)"
if git grep -nE 'deny\([^)]*\bwarnings\b' -- '*.rs' ':!vendor/**' >/dev/null; then
  git grep -nE 'deny\([^)]*\bwarnings\b' -- '*.rs' ':!vendor/**'
  echo "A new compiler warning would break every older build. CI makes warnings"
  echo "errors with -D warnings; take the attribute out."
  exit 1
fi
