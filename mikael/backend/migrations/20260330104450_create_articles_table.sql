-- Add migration script here
-- 1. Tạo kiểu ENUM cho visibility
CREATE TYPE article_visibility AS ENUM ('public', 'unlisted');

-- 2. Tạo bảng Article
CREATE TABLE IF NOT EXISTS articles_demo (
    id SERIAL PRIMARY KEY,
    -- FK liên kết tới bảng users_demo (id)
    owner_id INTEGER NOT NULL REFERENCES users_demo(id) ON DELETE CASCADE,
    
    title TEXT NOT NULL,
    description TEXT,
    body TEXT,
    
    -- Mặc định dùng Enum đã tạo ở trên
    visibility article_visibility NOT NULL DEFAULT 'public',
    
    -- Dùng i64 (BigInt trong Postgres) cho views/likes để tránh bị tràn số
    views BIGINT NOT NULL DEFAULT 0,
    likes BIGINT NOT NULL DEFAULT 0,
    
    time_created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 3. Đánh INDEX cho owner_id và title để search nhanh hơn
CREATE INDEX idx_articles_demo__owner_id ON articles_demo(owner_id);
CREATE INDEX idx_articles_demo__title ON articles_demo USING gin(to_tsvector('english', title));
