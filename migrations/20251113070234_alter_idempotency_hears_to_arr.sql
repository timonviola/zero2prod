ALTER TABLE idempotency DROP COLUMN response_headers;
ALTER TABLE idempotency ADD COLUMN response_headers header_pair[] NOT NULL;

