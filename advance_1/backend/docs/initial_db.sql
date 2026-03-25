CREATE TABLE IF NOT EXISTS users_demo (
    id serial primary key,
    name TEXT NOT NULL,
    password TEXT NOT NULL,
    email TEXT,
    -- CAP NHAT (bai 1): them cot phone de luu thong tin lien he cua user.
    phone TEXT
);

-- CAP NHAT (bai 1): neu database da ton tai truoc do thi van bo sung duoc cot phone.
ALTER TABLE users_demo ADD COLUMN IF NOT EXISTS phone TEXT;
CREATE INDEX user_demo__name_index ON users_demo (name);

-- CAP NHAT (bai 3): tao bang articles_demo de luu bai viet.
-- Bang nay phuc vu 2 use case trong de:
-- 1. Homepage: liet ke bai public
-- 2. User da login: xem tat ca bai viet do user do tao
CREATE TABLE IF NOT EXISTS articles_demo (
    id serial primary key,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    time_created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    visibility TEXT NOT NULL CHECK (visibility IN ('public', 'unlisted', 'draft')),
    created_by_user INTEGER NOT NULL REFERENCES users_demo(id) ON DELETE CASCADE
);

-- CAP NHAT (bai 3): index cho trang homepage, uu tien loc theo visibility va sap xep theo time_created.
CREATE INDEX IF NOT EXISTS articles_demo__visibility_time_created_index
    ON articles_demo (visibility, time_created DESC);
-- CAP NHAT (bai 3): index cho man hinh liet ke bai viet theo user tao.
CREATE INDEX IF NOT EXISTS articles_demo__created_by_user_index
    ON articles_demo (created_by_user);
