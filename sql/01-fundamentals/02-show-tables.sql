-- The Postgres equivalents of MySQL's SHOW commands. Module 1.2.
\dt                   -- list tables (\dt+ adds size, owner)
\dn                   -- list schemas
\du                   -- list roles (users + groups)
\df                   -- list functions

-- Information_schema is the SQL-standard alternative.
SELECT table_name
FROM information_schema.tables
WHERE table_schema = 'public'
ORDER BY table_name;
