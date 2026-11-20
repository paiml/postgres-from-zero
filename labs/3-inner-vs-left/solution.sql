CREATE VIEW lab3 AS
SELECT f.film_id, f.title FROM film f LEFT JOIN inventory i ON i.film_id = f.film_id WHERE i.inventory_id IS NULL;
