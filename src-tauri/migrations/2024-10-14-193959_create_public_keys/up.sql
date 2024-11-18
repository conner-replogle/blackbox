-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE public_keys (
  key_id VARCHAR(255)  NOT NULL  PRIMARY KEY,
  nickname VARCHAR(255)  NOT NULL,
  metadata TEXT ,
  public_key TEXT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP 
)