-- Add migration script here
CREATE TABLE IF NOT EXISTS users_demo (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT
);

-- Seed first users
INSERT INTO users_demo (name, email) VALUES 
    ('Michael Nguyen', 'michael@example.com'),
    ('Akira Nguyen', 'akira@example.com')
ON CONFLICT DO NOTHING;

-- Add password
ALTER TABLE users_demo ADD COLUMN password TEXT NOT NULL DEFAULT 'temporary_password';

-- Seed new users
INSERT INTO users_demo (name, email, password) 
VALUES 
    ('Sample Nguyen', 'sample@example.com', 'some_password'),
    ('Test Nguyen', 'test@example.com', 'no_password'),
    ('Smt Nguyen', 'smt@ex.com', 'no')
ON CONFLICT DO NOTHING;
