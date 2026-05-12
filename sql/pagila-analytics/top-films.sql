-- Pagila top-N films by rental count.
-- Walks film -> inventory -> rental to count how many times each film
-- was rented. The Rust binary `postgres-reports --report films` runs
-- the same query and enforces runtime contracts on the result.
SELECT f.film_id, f.title, COUNT(r.rental_id) AS rental_count
FROM film f
LEFT JOIN inventory i ON i.film_id = f.film_id
LEFT JOIN rental r    ON r.inventory_id = i.inventory_id
GROUP BY f.film_id, f.title
ORDER BY rental_count DESC
LIMIT 10;
