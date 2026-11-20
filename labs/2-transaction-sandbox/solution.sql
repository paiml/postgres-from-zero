BEGIN;
UPDATE film SET rental_rate = 0 WHERE rating = 'G';
\copy (SELECT film_id, title, rental_rate FROM film WHERE rating = 'G' ORDER BY film_id) TO STDOUT WITH (FORMAT csv, HEADER)
ROLLBACK;
