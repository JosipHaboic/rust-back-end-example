CREATE TABLE IF NOT EXISTS users (
  uuid TEXT NOT NULL,
  username TEXT NOT NULL,
  password TEXT NOT NULL,
  created_at REAL NOT NULL DEFAULT(datetime('now')),

  PRIMARY KEY (uuid),
  CHECK(length(password) >= 8)
);