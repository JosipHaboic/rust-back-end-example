BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS users (
  uuid TEXT NOT NULL,
  username TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  inserted_at REAL NOT NULL DEFAULT(datetime('now')),

  PRIMARY KEY (uuid),
  CHECK(length(password) >= 8)
);

CREATE INDEX IF NOT EXISTS idx_user_uuid ON users(uuid);

CREATE TABLE  IF NOT EXISTS users_history (
  uuid INTEGER NOT NULL,
  user_uuid TEXT NOT NULL,
  action_type TEXT NOT NULL,
  action_timestamp TEXT NOT NULL DEFAULT(datetime('now')),

  PRIMARY KEY (uuid)
);

CREATE TRIGGER IF NOT EXISTS track_user_inserts
AFTER INSERT ON users
BEGIN
  INSERT INTO users_history (user_uuid, action_type) VALUES (
    new.uuid,
    'INSERT'
  );
END;

CREATE TRIGGER IF NOT EXISTS track_user_updates
AFTER UPDATE ON users
BEGIN
  INSERT INTO users_history (user_uuid, action_type) VALUES (
    new.uuid,
    'UPDATE'
  );
END;

CREATE TRIGGER IF NOT EXISTS track_user_deletes
AFTER DELETE ON users
BEGIN
  INSERT INTO users_history (user_uuid, action_type) VALUES (
    old.uuid,
    'DELETE'
  );
END;

COMMIT;