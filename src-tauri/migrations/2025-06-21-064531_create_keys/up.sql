-- Your SQL goes here
CREATE TABLE keys (
  key_id VARCHAR(255) NOT NULL PRIMARY KEY,
  nickname VARCHAR(255) NOT NULL,
  metadata TEXT,
  is_me TINYINT NOT NULL DEFAULT 0,
  private_key TEXT,
  public_key TEXT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
