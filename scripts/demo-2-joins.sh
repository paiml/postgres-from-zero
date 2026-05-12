#!/usr/bin/env bash
# demo-2-joins.sh — the three join patterns every Pagila query uses.
#
#   01-traversal.sql   customer → rental → inventory → film (the Sakila spine)
#   02-inner-vs-left.sql  what changes when you flip INNER ↔ LEFT
#   03-explain-analyze.sql  reading a query plan against real row counts
#
# Usage:
#   scripts/demo-2-joins.sh
#
set -euo pipefail
cd "$(dirname "$0")/.."

bar() { printf '\n\033[1;36m=== %s ===\033[0m\n\n' "$*"; }

if ! docker compose ps postgres 2>/dev/null | grep -q "Up"; then
    bar "starting postgres + loading Pagila (one-time)"
    make pagila >/dev/null
fi

PSQL=(docker compose exec -T postgres psql -U postgres -d pagila)

for f in sql/02-joins/*.sql; do
    bar "$(basename "$f")"
    "${PSQL[@]}" -f - < "$f"
done

bar "joins demo complete"
