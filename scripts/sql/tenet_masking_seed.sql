CREATE TABLE IF NOT EXISTS tenet_masking_test (
  id serial PRIMARY KEY,
  email text,
  phone text,
  secret text
);
INSERT INTO tenet_masking_test (email, phone, secret)
VALUES ('alice@example.com', '+1-555-0100', 'top-secret');
