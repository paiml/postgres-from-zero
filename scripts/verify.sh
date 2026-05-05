#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."

PSQL="docker compose exec -T postgres psql -U postgres -d pagila -tAc"

assert_count() {
    local label="$1" expected="$2" sql="$3"
    local actual=$($PSQL "$sql" | tr -d ' ')
    if [ "$actual" = "$expected" ]; then
        echo "[verify] $label = $actual ✓"
    else
        echo "[verify] $label expected $expected, got $actual ✗"
        exit 1
    fi
}

assert_count "film count"      "1000"  "SELECT COUNT(*) FROM film;"
assert_count "actor count"     "200"   "SELECT COUNT(*) FROM actor;"
assert_count "rental count"    "16044" "SELECT COUNT(*) FROM rental;"
assert_count "language count"  "6"     "SELECT COUNT(*) FROM language;"

echo "[verify] all assertions passed"
