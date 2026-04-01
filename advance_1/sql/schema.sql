DROP TABLE IF EXISTS articles CASCADE;
DROP TABLE IF EXISTS users CASCADE;
CREATE TABLE users (
    id SERIAL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    password_salt TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT pk__users PRIMARY KEY (id),
    CONSTRAINT uq__users__email UNIQUE (email)
);

CREATE INDEX idx__users__email ON users(email);

CREATE TABLE articles (
    id SERIAL,
    owner_id INT NOT NULL,
    time_created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    visibility TEXT NOT NULL,
    title TEXT NOT NULL,
    body TEXT,
    description TEXT,
    views BIGINT NOT NULL DEFAULT 0,
    likes BIGINT NOT NULL DEFAULT 0,

    CONSTRAINT pk__articles PRIMARY KEY (id),
    CONSTRAINT fk__articles__owner_id
        FOREIGN KEY (owner_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE INDEX idx__articles__owner_id ON articles(owner_id);
CREATE INDEX idx__articles__visibility ON articles(visibility);