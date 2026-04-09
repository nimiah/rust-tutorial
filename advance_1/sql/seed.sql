INSERT INTO users (name, email, password_hash, password_salt) VALUES
('Mary', 'mary@example.com', 'hash_mary', 'salt_mary'),
('Eric', 'eric@example.com', 'hash_eric', 'salt_eric'),
('Tieu Hoa', 'tieuhoa@example.com', 'hash_tieuhoa', 'salt_tieuhoa');

INSERT INTO articles (owner_id, visibility, title, body, description, views, likes) VALUES
(1, 'public',   'Mary Article 1', 'Body 1', 'Desc 1', 10, 2),
(1, 'public',   'Mary Article 2', 'Body 2', 'Desc 2', 20, 3),
(1, 'unlisted', 'Mary Article 3', 'Body 3', 'Desc 3', 5, 1),
(1, 'public',   'Mary Article 4', 'Body 4', 'Desc 4', 12, 0),
(1, 'unlisted', 'Mary Article 5', 'Body 5', 'Desc 5', 8, 4),
(2, 'public',   'Eric Article 1', 'Body 6', 'Desc 6', 30, 5),
(2, 'public',   'Eric Article 2', 'Body 7', 'Desc 7', 25, 2),
(2, 'unlisted', 'Eric Article 3', 'Body 8', 'Desc 8', 7, 1),
(2, 'public',   'Eric Article 4', 'Body 9', 'Desc 9', 18, 6),
(2, 'unlisted', 'Eric Article 5', 'Body 10', 'Desc 10', 3, 0);