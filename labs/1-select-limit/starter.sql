-- Lab 1 (psql basics). Create a view lab1 holding the 5 longest films:
-- columns title, length; longest first, ties broken by title A-Z.
CREATE VIEW lab1 AS SELECT title, length FROM film LIMIT 5;
