#!/usr/bin/env bash
# demo-4-rust.sh — Rust capstone: postgres-reports binary, three reports.
#
# Same SQL as scripts/demo-3-pagila.sh, but wrapped by a Rust binary that
# uses an async sqlx::PgPool and enforces named runtime contracts on every
# result before printing JSON. If the database disagrees with the schema's
# invariants, the process aborts loudly instead of shipping corrupt JSON.
#
# Contracts proved per call:
#   row_count == limit
#   top row count >= 1
#   ORDER BY DESC monotonic
#   per-report id > 0, text field non-empty
#
# Output: pretty-printed JSON to stdout AND to out/ as files.
#
# Usage:
#   scripts/demo-4-rust.sh             # default --limit 10
#   scripts/demo-4-rust.sh 5           # --limit 5
#
set -euo pipefail
cd "$(dirname "$0")/.."

LIMIT="${1:-10}"
OUT_DIR="out"

bar() { printf '\n\033[1;36m=== %s ===\033[0m\n\n' "$*"; }

if ! docker compose ps postgres 2>/dev/null | grep -q "Up"; then
    bar "starting postgres + loading Pagila (one-time)"
    make pagila >/dev/null
fi

bar "building postgres-reports (release)"
cargo build --release --bin postgres-reports

mkdir -p "$OUT_DIR"

export DATABASE_URL="${DATABASE_URL:-postgres://postgres:postgres@localhost:5432/pagila}"

for r in customers films actors; do
    bar "postgres-reports --report ${r} --limit ${LIMIT}"
    cargo run --release --quiet --bin postgres-reports -- \
        --report "$r" --limit "$LIMIT" --out "${OUT_DIR}/${r}.json"
    echo
    echo "wrote ${OUT_DIR}/${r}.json"
done

bar "capstone complete — JSON written to ${OUT_DIR}/"
ls -lh "$OUT_DIR"/*.json
