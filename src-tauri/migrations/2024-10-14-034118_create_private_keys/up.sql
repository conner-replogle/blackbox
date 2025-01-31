
CREATE TABLE private_keys (
  key_id VARCHAR(255) NOT NULL PRIMARY KEY,
  nickname VARCHAR(255) NOT NULL,
  metadata TEXT,
  private_key TEXT NOT NULL,
  public_key_id VARCHAR(255) NOT NULL REFERENCES public_keys(key_id) ON DELETE CASCADE,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
