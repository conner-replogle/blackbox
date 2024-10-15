-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE public_keys (
  key_id VARCHAR NOT NULL  PRIMARY KEY,
  nickname VARCHAR NOT NULL,
  public_key TEXT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP 
)