-- ══════════════════════════════════════════════════════════════
-- Init: users_demo + articles_demo
-- ══════════════════════════════════════════════════════════════

-- ── 1. Users ──────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS users_demo (
    id            SERIAL      PRIMARY KEY,
    name          TEXT        NOT NULL,
    email         TEXT        NOT NULL UNIQUE,
    password_hash TEXT        NOT NULL DEFAULT 'hash',
    password_salt TEXT        NOT NULL DEFAULT 'salt',
    created_at    TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users_demo (name, email) VALUES
    ('Michael Nguyen', 'michael@example.com'),
    ('Akira Nguyen',   'akira@example.com'),
    ('Sample Nguyen',  'sample@example.com'),
    ('Test Nguyen',    'test@example.com'),
    ('Smt Nguyen',     'smt@ex.com')
ON CONFLICT DO NOTHING;

-- ── 2. Articles ───────────────────────────────────────────────

CREATE TYPE article_visibility AS ENUM ('public', 'unlisted');

CREATE TABLE IF NOT EXISTS articles_demo (
    id           SERIAL              PRIMARY KEY,
    owner_id     INTEGER             NOT NULL REFERENCES users_demo(id) ON DELETE CASCADE,
    title        TEXT                NOT NULL,
    description  TEXT,
    body         TEXT,
    visibility   article_visibility  NOT NULL DEFAULT 'public',
    views        BIGINT              NOT NULL DEFAULT 0,
    likes        BIGINT              NOT NULL DEFAULT 0,
    time_created TIMESTAMPTZ         NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_articles_demo__owner_id ON articles_demo(owner_id);
CREATE INDEX idx_articles_demo__title    ON articles_demo USING gin(to_tsvector('english', title));

-- ── 3. Seed articles ──────────────────────────────────────────

-- Akira: 2 public + 2 unlisted
INSERT INTO articles_demo (owner_id, title, description, visibility)
SELECT id, 'Public Article U2 - 1',   'Desc 1', 'public'::article_visibility   FROM users_demo WHERE name = 'Akira Nguyen' UNION ALL
SELECT id, 'Public Article U2 - 2',   'Desc 2', 'public'::article_visibility   FROM users_demo WHERE name = 'Akira Nguyen' UNION ALL
SELECT id, 'Unlisted Article U2 - 1', 'Desc 3', 'unlisted'::article_visibility FROM users_demo WHERE name = 'Akira Nguyen' UNION ALL
SELECT id, 'Unlisted Article U2 - 2', 'Desc 4', 'unlisted'::article_visibility FROM users_demo WHERE name = 'Akira Nguyen';

-- Sample: 3 public
INSERT INTO articles_demo (owner_id, title, description, visibility)
SELECT id, 'Public Article U3 - 1', 'Desc', 'public'::article_visibility FROM users_demo WHERE name = 'Sample Nguyen' UNION ALL
SELECT id, 'Public Article U3 - 2', 'Desc', 'public'::article_visibility FROM users_demo WHERE name = 'Sample Nguyen' UNION ALL
SELECT id, 'Public Article U3 - 3', 'Desc', 'public'::article_visibility FROM users_demo WHERE name = 'Sample Nguyen';

-- Test: 3 unlisted
INSERT INTO articles_demo (owner_id, title, description, visibility)
SELECT id, 'Unlisted Article U4 - 1', 'Desc', 'unlisted'::article_visibility FROM users_demo WHERE name = 'Test Nguyen' UNION ALL
SELECT id, 'Unlisted Article U4 - 2', 'Desc', 'unlisted'::article_visibility FROM users_demo WHERE name = 'Test Nguyen' UNION ALL
SELECT id, 'Unlisted Article U4 - 3', 'Desc', 'unlisted'::article_visibility FROM users_demo WHERE name = 'Test Nguyen';
