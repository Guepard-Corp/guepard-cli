-- Tenet demo schema and data for PostgreSQL
-- Run against a Postgres instance for testing the proxy

DROP TABLE IF EXISTS customers;
CREATE TABLE customers (
    id SERIAL PRIMARY KEY,
    email TEXT,
    phone_number TEXT,
    address TEXT
);
INSERT INTO customers (email, phone_number, address) VALUES
    ('john.doe@company.com', '555-123-4567', '742 Evergreen Terrace, Springfield'),
    ('jane.smith@gmail.com', '555-987-6543', '221B Baker Street, London'),
    ('alice@example.org', '+1-212-555-0100', '123 Main St, New York');

DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email TEXT,
    phone_number TEXT,
    credit_card TEXT,
    ssn TEXT
);
INSERT INTO users (email, phone_number, credit_card, ssn) VALUES
    ('secret@hidden.org', '555-111-2222', '4532-1234-5678-9012', '123-45-6789'),
    ('private@email.net', '555-333-4444', '5425-9876-5432-1098', '987-65-4321');

DROP TABLE IF EXISTS profiles;
CREATE TABLE profiles (
    id SERIAL PRIMARY KEY,
    data JSONB
);
INSERT INTO profiles (data) VALUES
    ('{"user": {"email": "nested@json.com", "name": "Test"}, "payment": {"cc": "1111-2222-3333-4444"}}');

DROP TABLE IF EXISTS tags;
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    values TEXT[]
);
INSERT INTO tags (values) VALUES
    (ARRAY['normal_tag', 'array@email.com', '9999-8888-7777-6666']);

-- identities: first_name, last_name, full_name, dob, passport
DROP TABLE IF EXISTS identities;
CREATE TABLE identities (
    id SERIAL PRIMARY KEY,
    first_name TEXT,
    last_name TEXT,
    full_name TEXT,
    dob DATE,
    passport TEXT
);
INSERT INTO identities (first_name, last_name, full_name, dob, passport) VALUES
    ('John', 'Doe', 'John Doe', '1985-03-15', 'AB1234567'),
    ('Jane', 'Smith', 'Jane Smith', '1990-07-22', 'CD9876543');

-- network_logs: ip
DROP TABLE IF EXISTS network_logs;
CREATE TABLE network_logs (
    id SERIAL PRIMARY KEY,
    ip_address TEXT
);
INSERT INTO network_logs (ip_address) VALUES
    ('192.168.1.100'),
    ('10.0.0.255');

-- sensitive_misc: keep_first_char, redacted, random, hash, partial_mask, date_shift
DROP TABLE IF EXISTS sensitive_misc;
CREATE TABLE sensitive_misc (
    id SERIAL PRIMARY KEY,
    secret_key TEXT,
    token TEXT,
    random_val TEXT,
    hash_input TEXT,
    short_code TEXT,
    event_date DATE
);
INSERT INTO sensitive_misc (secret_key, token, random_val, hash_input, short_code, event_date) VALUES
    ('api_key_xyz123', 'Bearer secret123', 'deterministic', 'input_to_hash', 'ABCD1234', '2024-06-15'),
    ('session_abc', 'AuthToken 456', 'value', 'another_input', 'XY99', '2023-12-01');
