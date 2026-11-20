CREATE INDEX idx_rental_out ON rental (return_date) WHERE return_date IS NULL;
ANALYZE rental;
