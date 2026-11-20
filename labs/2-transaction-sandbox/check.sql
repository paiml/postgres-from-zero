DO $$ BEGIN
  IF (SELECT sum(rental_rate) FROM film WHERE rating = 'G') IS DISTINCT FROM 514.22
  THEN RAISE EXCEPTION 'lab2: table changed (sum of G rental_rate is not 514.22)'; END IF;
END $$;
