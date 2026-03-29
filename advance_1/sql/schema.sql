CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    password_salt TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    owner_id INT NOT NULL,
    time_created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    visibility TEXT NOT NULL CHECK (visibility IN ('public', 'unlisted')),
    title TEXT NOT NULL,
    body TEXT,
    description TEXT,
    views BIGINT NOT NULL DEFAULT 0,
    likes BIGINT NOT NULL DEFAULT 0,
    CONSTRAINT fk_articles_owner
        FOREIGN KEY (owner_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);