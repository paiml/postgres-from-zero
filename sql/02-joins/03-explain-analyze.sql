-- EXPLAIN (ANALYZE, BUFFERS) reveals what the planner actually did.
-- Look for "Seq Scan" (full table) vs "Index Scan" / "Bitmap Heap Scan".

-- Wide predicate: 16k rows, ~99% match. Even after we add the index,
-- the planner stays on Seq Scan because reading the whole table
-- sequentially is faster than chasing index pointers for that many rows.
EXPLAIN (ANALYZE, BUFFERS)
SELECT customer_id, COUNT(*) FROM rental
WHERE rental_date >= '2022-04-01'
GROUP BY customer_id;

CREATE INDEX IF NOT EXISTS idx_rental_date ON rental (rental_date);

-- Same query, post-index: still Seq Scan. The index didn't help here.
-- Lesson: indexes earn their cost when the predicate is selective.
EXPLAIN (ANALYZE, BUFFERS)
SELECT customer_id, COUNT(*) FROM rental
WHERE rental_date >= '2022-04-01'
GROUP BY customer_id;

-- Narrow predicate: a single day's worth of rentals (a few rows out of
-- 16k). Now the plan flips to Index Scan / Bitmap Heap Scan and the
-- index pays for itself.
EXPLAIN (ANALYZE, BUFFERS)
SELECT * FROM rental
WHERE rental_date BETWEEN '2022-05-25' AND '2022-05-26';
