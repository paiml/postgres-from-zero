-- INNER drops customers with zero rentals; LEFT keeps them with NULL.
-- Pick by the question, not by habit.

-- INNER: only customers who have rented something.
SELECT c.customer_id, COUNT(r.rental_id) AS rentals
FROM customer c
INNER JOIN rental r ON r.customer_id = c.customer_id
GROUP BY c.customer_id
ORDER BY rentals ASC LIMIT 5;

-- LEFT: all customers, even the ones who never rented.
SELECT c.customer_id, COUNT(r.rental_id) AS rentals
FROM customer c
LEFT JOIN rental r ON r.customer_id = c.customer_id
GROUP BY c.customer_id
ORDER BY rentals ASC LIMIT 5;
