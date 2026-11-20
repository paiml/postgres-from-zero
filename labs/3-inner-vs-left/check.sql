DO $$ BEGIN
  IF (SELECT count(*) FROM lab3) <> 42 THEN RAISE EXCEPTION 'lab3: expected 42 unstocked films'; END IF;
  IF EXISTS (SELECT 1 FROM lab3 l JOIN inventory i ON i.film_id = l.film_id) THEN RAISE EXCEPTION 'lab3: a stocked film is listed'; END IF;
END $$;
