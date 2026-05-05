-- Always cap an unfamiliar table with LIMIT before SELECT *.
SELECT * FROM film LIMIT 5;
SELECT COUNT(*) FROM film;        -- 1000
SELECT COUNT(*) FROM rental;      -- 16044
SELECT COUNT(*) FROM actor;       -- 200
