-- Lab 3 (INNER vs LEFT JOIN). Create a view lab3 (film_id, title) of films with NO inventory row.
CREATE VIEW lab3 AS
SELECT f.film_id, f.title FROM film f JOIN inventory i ON i.film_id = f.film_id WHERE i.inventory_id IS NULL;
