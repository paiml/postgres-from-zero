#!/usr/bin/env bash
# demo-1-fundamentals.sh — first commands you run after `psql -d pagila`.
#
# Walks through the five SQL files in sql/01-fundamentals/:
#   01-connect.sql     psql conventions, schema introspection
#   02-show-tables.sql listing tables, describing a table
#   03-select-limit.sql safe exploratory SELECT against a 16k-row table
#   04-modify.sql      INSERT / UPDATE / DELETE inside a transaction
#   05-export.sql      \copy a query result to a CSV file
#
# Usage:
#   scripts/demo-1-fundamentals.sh
#
set -euo pipefail
cd "$(dirname "$0")/.."

bar() { printf '\n\033[1;36m=== %s ===\033[0m\n\n' "$*"; }

# bring up postgres + load Pagila if not already done
if ! docker compose ps postgres 2>/dev/null | grep -q "Up"; then
    bar "starting postgres + loading Pagila (one-time)"
    make pagila >/dev/null
fi

PSQL=(docker compose exec -T postgres psql -U postgres -d pagila)

for f in sql/01-fundamentals/*.sql; do
    bar "$(basename "$f")"
    "${PSQL[@]}" -f - < "$f"
done

bar "fundamentals demo complete"
