-- The first commands you run after `psql -d pagila`. Module 1.1.
-- Note: psql backslash commands consume the rest of the line, so put
-- comments on their own line above each command — never trailing.

-- list databases
\l

-- connect to pagila
\c pagila

-- list tables in current database
\dt

-- describe the film table
\d film

SELECT version();
