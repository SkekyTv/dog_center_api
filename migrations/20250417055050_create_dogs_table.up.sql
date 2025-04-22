-- Add up migration script here
--
CREATE TABLE dogs (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL
)
