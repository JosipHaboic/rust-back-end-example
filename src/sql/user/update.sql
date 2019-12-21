BEGIN TRANSACTION;

UPDATE users SET username = ?3, password = ?2 WHERE uuid = ?1;

COMMIT;