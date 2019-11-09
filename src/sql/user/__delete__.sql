DROP TABLE IF EXISTS users;
DROP TABLE  IF EXISTS users_history;
DROP INDEX IF EXISTS ON idx_user_uuid ON users(uuid);
DROP TRIGGER IF EXISTS track_user_inserts;
DROP TRIGGER IF EXISTS track_user_updates;
DROP TRIGGER IF EXISTS track_user_deletes;