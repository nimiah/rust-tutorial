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
SELECT id, 'Unlisted Article U4 - 3', 'Desc', 'unlisted'::article_visibility FROM users_demo WHERE name = 'Test Nguyen';
