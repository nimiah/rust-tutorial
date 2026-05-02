CREATE TABLE IF NOT EXISTS users_demo (
    id serial primary key,
    name TEXT NOT NULL,
    -- password TEXT NOT NULL,
    email TEXT
);

ALTER TABLE users_demo ADD password TEXT;
CREATE INDEX user_demo__name_index ON users_demo (name);


