-- Add migration script here
-- Thêm các cột bảo mật và thời gian
ALTER TABLE users_demo 
    ADD COLUMN password_hash TEXT NOT NULL DEFAULT 'hash',
    ADD COLUMN password_salt TEXT NOT NULL DEFAULT 'salt',
    ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP;

-- Xóa cột password cũ
ALTER TABLE users_demo DROP COLUMN password;
