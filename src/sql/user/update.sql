BEGIN TRANSACTION;

UPDATE users SET username = ?1, password = ?2 WHERE uuid = ?3;

COMMIT;