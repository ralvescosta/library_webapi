-- Your SQL goes here
CREATE TABLE books (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  subject VARCHAR NOT NULL,
  author VARCHAR NOT NULL,
  published_date TIMESTAMPTZ NULL,
  editor VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  deleted_at TIMESTAMPTZ NULL
);