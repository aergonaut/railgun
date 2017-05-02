CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE pull_requests (
  -- id uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  id SERIAL PRIMARY KEY,
  repository VARCHAR NOT NULL,
  number VARCHAR NOT NULL,
  head_sha VARCHAR NOT NULL,
  status VARCHAR NOT NULL
);

CREATE UNIQUE INDEX index_pull_requests_on_repository_and_number ON pull_requests USING btree (repository, number);
