-- The Sakila spine: customer → rental → inventory → film.
SELECT c.first_name, c.last_name, f.title
FROM customer c
INNER JOIN rental r    ON r.customer_id = c.customer_id
INNER JOIN inventory i ON i.inventory_id = r.inventory_id
INNER JOIN film f      ON f.film_id = i.film_id
LIMIT 10;
