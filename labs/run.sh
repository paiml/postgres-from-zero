#!/bin/bash
# usage: labs/run.sh <lab-dir> <starter|solution>   exit 0 = the lab's check passes on that side
set -u
cd "$(dirname "$0")/.."
DB="lab_$(basename "$1" | tr -c 'a-z0-9\n' _)_$2"
P="docker compose exec -T postgres psql -U postgres -v ON_ERROR_STOP=1"
$P -d postgres -qc "DROP DATABASE IF EXISTS $DB" -c "CREATE DATABASE $DB TEMPLATE pagila" || exit 2
OUT=$($P -d "$DB" -q -f - < "labs/$1/$2.sql" 2>&1); RC=$?
[ $RC -eq 0 ] || { echo "script error: $OUT"; exit 1; }
$P -d "$DB" -q -f - < "labs/$1/check.sql" || { echo "CHECK FAILED ($1 $2)"; exit 1; }
if [ "$1" = 2-transaction-sandbox ]; then
  [ "$(echo "$OUT" | grep -c '^[0-9]*,')" = 178 ] || { echo "CHECK FAILED ($1 $2): expected 178 CSV rows from \\copy TO STDOUT"; exit 1; }
fi
echo "CHECK PASSED ($1 $2)"
$P -d postgres -qc "DROP DATABASE $DB"
