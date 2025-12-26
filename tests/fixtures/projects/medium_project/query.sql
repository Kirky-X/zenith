SELECT users.id, users.name, users.email, orders.total
FROM users
LEFT JOIN orders ON users.id = orders.user_id
WHERE users.created_at > '2024-01-01'
ORDER BY users.name ASC;
