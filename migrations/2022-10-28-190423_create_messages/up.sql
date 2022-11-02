-- Your SQL goes here
-- diesel migration run
CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  subaddress VARCHAR(64) NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)
-- lookup table for sharding messages
-- nodes without all message should be
-- able to locate them
CREATE TABLE lookup (
  id SERIAL PRIMARY KEY,
  node VARCHAR NOT NULL
)
