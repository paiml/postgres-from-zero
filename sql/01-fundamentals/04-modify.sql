-- INSERT, UPDATE, DELETE on a real schema.
BEGIN;

INSERT INTO actor (first_name, last_name)
VALUES ('TEST', 'ACTOR') RETURNING actor_id;

UPDATE actor SET first_name = 'UPDATED' WHERE last_name = 'ACTOR';

DELETE FROM actor WHERE last_name = 'ACTOR';

ROLLBACK;     -- so the demo doesn't pollute the fixture
