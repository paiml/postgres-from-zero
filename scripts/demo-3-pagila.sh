#!/usr/bin/env bash
# demo-3-pagila.sh — three Pagila analytics queries as raw SQL.
#
# Each query is the same one the Rust capstone runs (see scripts/demo-4-rust.sh).
# This demo shows them in pure SQL so you can see exactly what the binary
# wraps with runtime contracts.
#
#   top-customers.sql  customer × rental                 → 10 by rental count
#   top-films.sql      film × inventory × rental         → 10 by rental count
#   top-actors.sql     actor × film_actor                → 10 by distinct films
#
# Usage:
#   scripts/demo-3-pagila.sh
#
set -euo pipefail
cd "$(dirname "$0")/.."

bar() { printf '\n\033[1;36m=== %s ===\033[0m\n\n' "$*"; }

if ! docker compose ps postgres 2>/dev/null | grep -q "Up"; then
    bar "starting postgres + loading Pagila (one-time)"
    make pagila >/dev/null
fi

PSQL=(docker compose exec -T postgres psql -U postgres -d pagila)

for q in top-customers top-films top-actors; do
    bar "pagila-analytics/${q}.sql"
    "${PSQL[@]}" -f - < "sql/pagila-analytics/${q}.sql"
done

bar "pagila analytics complete (run scripts/demo-4-rust.sh for the contract-enforced version)"
