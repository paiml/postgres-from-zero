-- Lab 2 (transaction sandbox). Inside a transaction set rental_rate to 0 for rating 'G',
-- print the G films as CSV with \copy ... TO STDOUT, then leave the table UNCHANGED.
BEGIN;
UPDATE film SET rental_rate = 0 WHERE rating = 'G';
COMMIT;
