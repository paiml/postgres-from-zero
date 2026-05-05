#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."

PSQL="docker compose exec -T postgres psql -U postgres -d pagila"

EXISTS=$($PSQL -tAc "SELECT to_regclass('public.film') IS NOT NULL;" 2>/dev/null | tr -d ' ' || echo "f")
if [ "$EXISTS" = "t" ]; then
    COUNT=$($PSQL -tAc "SELECT COUNT(*) FROM film;" | tr -d ' ')
    if [ "$COUNT" = "1000" ]; then
        echo "[load-pagila] Pagila already loaded (film count = 1000) ✓"
        exit 0
    fi
fi

echo "[load-pagila] downloading Pagila…"
TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

curl -sL https://raw.githubusercontent.com/devrimgunduz/pagila/master/pagila-schema.sql -o "$TMPDIR/schema.sql"
curl -sL https://raw.githubusercontent.com/devrimgunduz/pagila/master/pagila-data.sql   -o "$TMPDIR/data.sql"

echo "[load-pagila] applying schema…"
$PSQL < "$TMPDIR/schema.sql" > /dev/null

echo "[load-pagila] applying data (this is the COPY-based variant, ~5–10s)…"
$PSQL < "$TMPDIR/data.sql" > /dev/null

COUNT=$($PSQL -tAc "SELECT COUNT(*) FROM film;" | tr -d ' ')
if [ "$COUNT" != "1000" ]; then
    echo "[load-pagila] FAILED: film count = $COUNT, expected 1000"
    exit 1
fi
echo "[load-pagila] film count = 1000 ✓"
