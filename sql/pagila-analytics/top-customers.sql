-- Pagila top-N customers by rental count.
-- Joins customer to rental and ranks customers by the number of rentals
-- they appear in. The Rust binary `postgres-reports --report customers`
-- executes the same query and asserts runtime contracts on the result
-- (see crates/postgres-reports/src/lib.rs).
SELECT c.customer_id,
       c.first_name || ' ' || c.last_name AS name,
       COUNT(r.rental_id) AS rental_count,
       c.email
FROM customer c
LEFT JOIN rental r ON r.customer_id = c.customer_id
GROUP BY c.customer_id, c.first_name, c.last_name, c.email
ORDER BY rental_count DESC
LIMIT 10;
