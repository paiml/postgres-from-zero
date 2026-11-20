DO $$ DECLARE plan text; BEGIN
  EXECUTE 'EXPLAIN (FORMAT JSON) SELECT * FROM rental WHERE return_date IS NULL' INTO plan;
  IF plan NOT LIKE '%Index%Scan%' THEN RAISE EXCEPTION 'lab4: plan has no index scan: %', left(plan, 120); END IF;
END $$;
