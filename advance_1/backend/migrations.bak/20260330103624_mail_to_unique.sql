-- Add migration script here
-- 1. Xử lý các dòng dữ liệu cũ nếu có email bị NULL (Tránh lỗi khi set NOT NULL)
UPDATE users_demo SET email = 'placeholder_' || id || '@example.com' WHERE email IS NULL;

-- 2. Thêm ràng buộc NOT NULL và UNIQUE
ALTER TABLE users_demo 
    ALTER COLUMN email SET NOT NULL,
    ADD CONSTRAINT users_demo__email_unique UNIQUE (email);
