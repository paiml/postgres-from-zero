#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."

PSQL="docker compose exec -T postgres psql -U postgres -d pagila"

run() {
    echo ""
    echo "=== $1 ==="
    $PSQL -f - < "$1"
}

run sql/01-fundamentals/03-select-limit.sql
run sql/02-joins/01-traversal.sql
run sql/02-joins/02-inner-vs-left.sql
run sql/pagila-analytics/top-customers.sql
