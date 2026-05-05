-- EXPLAIN (ANALYZE, BUFFERS) reveals what the planner actually did.
-- Look for "Seq Scan" (full table) vs "Index Scan" / "Bitmap Heap Scan".

EXPLAIN (ANALYZE, BUFFERS)
SELECT customer_id, COUNT(*) FROM rental
WHERE rental_date >= '2022-04-01'
GROUP BY customer_id;

-- Build an index, re-run, watch the plan flip.
CREATE INDEX IF NOT EXISTS idx_rental_date ON rental (rental_date);

EXPLAIN (ANALYZE, BUFFERS)
SELECT customer_id, COUNT(*) FROM rental
WHERE rental_date >= '2022-04-01'
GROUP BY customer_id;
