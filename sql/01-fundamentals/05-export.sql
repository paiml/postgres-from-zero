-- The export-to-disk pattern. COPY ... TO STDOUT is the Postgres
-- equivalent of MySQL's INTO OUTFILE; it's the bridge from psql to
-- the Unix toolbox (jq, awk, a python http.server, etc.).
\copy (SELECT * FROM actor) TO '/tmp/actors.csv' CSV HEADER
