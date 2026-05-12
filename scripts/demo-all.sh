#!/usr/bin/env bash
# demo-all.sh — run every demo in sequence.
#
#   1. SQL fundamentals      (scripts/demo-1-fundamentals.sh)
#   2. Joins                 (scripts/demo-2-joins.sh)
#   3. Pagila analytics SQL  (scripts/demo-3-pagila.sh)
#   4. Rust capstone         (scripts/demo-4-rust.sh)
#
# Usage:
#   scripts/demo-all.sh
#
set -euo pipefail
cd "$(dirname "$0")/.."

bar() { printf '\n\033[1;33m################ %s ################\033[0m\n' "$*"; }

bar "DEMO 1/4 — SQL fundamentals"
scripts/demo-1-fundamentals.sh

bar "DEMO 2/4 — Joins"
scripts/demo-2-joins.sh

bar "DEMO 3/4 — Pagila analytics"
scripts/demo-3-pagila.sh

bar "DEMO 4/4 — Rust capstone"
scripts/demo-4-rust.sh

bar "ALL DEMOS COMPLETE"
