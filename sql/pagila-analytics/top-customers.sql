SELECT c.customer_id,
       c.first_name || ' ' || c.last_name AS name,
       COUNT(r.rental_id) AS rental_count,
       c.email
FROM customer c
LEFT JOIN rental r ON r.customer_id = c.customer_id
GROUP BY c.customer_id, c.first_name, c.last_name, c.email
ORDER BY rental_count DESC
LIMIT 10;
