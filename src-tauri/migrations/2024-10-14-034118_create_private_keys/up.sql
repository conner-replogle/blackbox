-- Your SQL goes here
CREATE TABLE private_keys (
  key_id VARCHAR NOT NULL  PRIMARY KEY,
  nickname VARCHAR NOT NULL,
  private_key TEXT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP 
)