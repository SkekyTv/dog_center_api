-- Add up migration script here
create table users (
  id UUID PRIMARY KEY,
  email text NOT NULL UNIQUE,
  pdw_hash text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz
);
