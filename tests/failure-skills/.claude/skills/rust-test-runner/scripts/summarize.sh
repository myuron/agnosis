#!/usr/bin/env bash
# Summarize a cargo test log: print the result line and failing test names.
set -euo pipefail

log="${1:?usage: summarize.sh <cargo-test-log>}"

grep -E '^test result:' "$log" || true
echo "--- failures ---"
grep -E '^test .* \.\.\. FAILED$' "$log" | sed -E 's/^test (.*) \.\.\. FAILED$/\1/' || true
