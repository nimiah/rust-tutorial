-- Add migration script here
-- Seed Articles cho từng User
-- User 1: Không có article nào (Không cần chạy lệnh INSERT)

-- User 2: 2 public & 2 unlisted
INSERT INTO articles_demo (owner_id, title, description, visibility)
SELECT id, 'Public Article U2 - 1', 'Desc 1', 'public'::article_visibility FROM users_demo WHERE name = 'Akira Nguyen' UNION ALL
SELECT id, 'Public Article U2 - 2', 'Desc 2', 'public'::article_visibility FROM users_demo WHERE name = 'Akira Nguyen' UNION ALL
SELECT id, 'Unlisted Article U2 - 1', 'Desc 3', 'unlisted'::article_visibility FROM users_demo WHERE name = 'Akira Nguyen' UNION ALL
SELECT id, 'Unlisted Article U2 - 2', 'Desc 4', 'unlisted'::article_visibility FROM users_demo WHERE name = 'Akira Nguyen';

-- User 3: 3 public
INSERT INTO articles_demo (owner_id, title, description, visibility)
SELECT id, 'Public Article U3 - 1', 'Desc', 'public'::article_visibility FROM users_demo WHERE name = 'Sample Nguyen' UNION ALL
SELECT id, 'Public Article U3 - 2', 'Desc', 'public'::article_visibility FROM users_demo WHERE name = 'Sample Nguyen' UNION ALL
SELECT id, 'Public Article U3 - 3', 'Desc', 'public'::article_visibility FROM users_demo WHERE name = 'Sample Nguyen';

-- User 4: 3 unlisted
INSERT INTO articles_demo (owner_id, title, description, visibility)
SELECT id, 'Unlisted Article U4 - 1', 'Desc', 'unlisted'::article_visibility FROM users_demo WHERE name = 'Test Nguyen' UNION ALL
SELECT id, 'Unlisted Article U4 - 2', 'Desc', 'unlisted'::article_visibility FROM users_demo WHERE name = 'Test Nguyen' UNION ALL
SELECT id, 'Unlisted Article U4 - 4', 'Desc', 'unlisted'::article_visibility FROM users_demo WHERE name = 'Test Nguyen';

INSERT INTO articles_demo (owner_id, title, description, visibility)
SELECT 
    -- 1. Lấy ngẫu nhiên ID từ users_demo (cột là 'id' chứ không phải 'owner_id')
    (SELECT id FROM users_demo ORDER BY random() LIMIT 1),

    -- 2. Tạo tiêu đề
    'Tiêu đề bài viết thứ ' || i,

    -- 3. Giả lập nội dung
    CASE 
        WHEN i % 5 = 0 THEN 'Nội dung cực kỳ dài... ' || REPEAT('Lorem ipsum dolor sit amet, consectetur adipiscing elit. ', 15)
        WHEN i % 3 = 0 THEN 'Nội dung vừa phải: ' || REPEAT('Rust is awesome! ', 5)
        ELSE 'Nội dung ngắn gọn chỉ vài chữ.'
    END,

    -- 4. Gán tag (Sửa dấu ngoặc đơn và Ép kiểu Enum)
    CASE 
        WHEN i % 5 = 0 THEN 'unlisted'::article_visibility
        ELSE 'public'::article_visibility
    END
FROM generate_series(1, 200) AS i;
