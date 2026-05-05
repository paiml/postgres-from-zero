SELECT a.actor_id, a.first_name, a.last_name,
       COUNT(DISTINCT fa.film_id) AS film_count
FROM actor a
LEFT JOIN film_actor fa ON fa.actor_id = a.actor_id
GROUP BY a.actor_id, a.first_name, a.last_name
ORDER BY film_count DESC
LIMIT 10;
