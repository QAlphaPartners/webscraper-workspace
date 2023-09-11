-- User
CREATE TABLE user (
  id INTEGER PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  -- Auth
  pwd TEXT,
  pwd_salt TEXT NOT NULL,
  token_salt TEXT NOT NULL
);