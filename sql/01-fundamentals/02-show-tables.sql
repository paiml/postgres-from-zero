-- The Postgres equivalents of MySQL's SHOW commands. Module 1.2.

-- list tables (\dt+ adds size, owner)
\dt

-- list schemas
\dn

-- list roles (users + groups)
\du

-- list functions
\df

-- Information_schema is the SQL-standard alternative.
SELECT table_name
FROM information_schema.tables
WHERE table_schema = 'public'
ORDER BY table_name;
