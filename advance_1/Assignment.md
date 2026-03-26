## Assignment

### Sun - 22-03-2026

#### Questions

1. Thực hành thêm column và kiểm tra code mới trên database của mình đảm bảo chạy được
2. Kiểm tra và sửa lỗi tại sao api get_users lại return là []
3. Thiết kế 2 tables: user và article theo yêu cầu từ file excalidraw

```plaintext
Dự án: Xây dựng một WebApp đưa tin giống Facebook

- Homepage: liệt kê tất cả các bản tin của người dùng
dưới dạng là public
- Nếu user login: liệt kê hết tất cả các dạng bản tin

- User (người dùng)
    - ...
- Article (bản tin)
    - time_created
    - visibility: public / unlisted / draft
    - created_by_user
```

#### Answers

##### 1. Thực hành thêm column

Giải pháp:

```sql
-- add password to Schema
CREATE TABLE IF NOT EXISTS users_demo (
    id serial primary key,
    name TEXT NOT NULL,
    password TEXT NOT NULL,
    email TEXT
);

-- ALTER TABLE users_demo ADD password TEXT;
CREATE INDEX IF NOT EXISTS user_demo__name_index ON users_demo (name);
```

Result:

```bash
$ psql "postgresql://admin:admin123@localhost:5432/mydatabase"
psql (17.9)
Type "help" for help.

mydatabase=# \dt
          List of relations
 Schema |    Name    | Type  | Owner
--------+------------+-------+-------
 public | users_demo | table | admin
(1 row)

mydatabase=# \d users_demo
                             Table "public.users_demo"
  Column  |  Type   | Collation | Nullable |                Default
----------+---------+-----------+----------+----------------------------------------
 id       | integer |           | not null | nextval('users_demo_id_seq'::regclass)
 name     | text    |           | not null |
 password | text    |           | not null |
 email    | text    |           |          |
Indexes:
    "users_demo_pkey" PRIMARY KEY, btree (id)
    "user_demo__name_index" btree (name)

```

##### 2.A. Kiểm tra tại sao api get_users lại return là []

Checking Workflow:

- Run advance_1/backend --> ✅
- On Swagger UI, call `POST /api/user` --> ✅
- Check Swagger result --> ❌ 500 - Error: Internal Server Error

```json
Missing request extension: Extension of type `backend::models::user::User` was not found. Perhaps you forgot to add it? See `axum::Extension`.
```

- Check database result --> ❌

```bash
# access Postgres database
$ psql "postgresql://admin:admin123@localhost:5432/mydatabase"

mydatabase=# \dt
mydatabase=# \d users_demo

# list users --> 0 - No results
mydatabase=# SELECT id, name, email, password FROM users_demo ORDER BY id;
 id | name | email | password
----+------+-------+----------
(0 rows)
```

--> FAILED to create new user --> `GET /api/users` return []

Root cause:

- `auth.rs - line.25` - `auth_header` == None --> `auth_value` == None
- --> `trans.rs - line.36` - `response.status()` != is_success --> `tx.rollback()`

Further issue:

- Database requires NOT-NULL password <-> conflict create_user & login not pass password
- `POST /api/user` requires Token to complete API --> NOT appropriated

#### 2.B. Sửa lỗi tại sao api get_users lại return là []

**Giải pháp**:

- 2 APIs `POST /api/user` & `POST /api/auth/login` cần truyền password, 4 APIs còn lại ko-bắt-buộc truyền password
- 2 APIs `POST /api/user` & `POST /api/auth/login` ko cần truyền Authentication, 4 APIs còn lại nên có Authentication

**Fixing Phases**:

- Add required-password to APIs --> ✅
- Remove Authentication/security from `POST /api/user` --> ✅
- Remove required-password to 4 APIs --> ✅
- Add Authentication/security to 4 APIs --> ✅
- Remove returned-password from `GET /api/users`

##### 3. Thiết kế 2 tables: user và article theo yêu cầu từ file excalidraw

Table “đơn giản” (Basic) cho User & Article

###### 1) User (Basic)

Mục tiêu: auth + xác định ownership (created_by_user).

Các cột tối thiểu nên có:

```plaintext
- id (PK)
- name (text)
- email (unique, text) (để login tìm nhanh)
- password_hash (text) (khuyến nghị; trong tutorial đang dùng password tạm thời nhưng về lâu dài nên hash)
- created_at (timestamptz)
- (tuỳ chọn) updated_at
```

Điều lưu ý quan trọng nhất cho User

- email nên UNIQUE
- password lưu dạng hash (đừng lưu plaintext)

###### 2) Article (Basic)

Mục tiêu: feed theo visibility + ownership theo created_by_user.

Các cột tối thiểu nên có:

```plaintext
- id (PK)
- created_by_user (FK → users.id)
- time_created (timestamptz)
- visibility (enum/text): public | unlisted | private | draft
- title (text, nếu muốn feed dễ hiển thị)
- content hoặc body (text)
```

Điều lưu ý quan trọng nhất cho Article

- created_by_user phải có FK (đảm bảo không có “article mồ côi”)
- visibility nên ràng buộc CHECK/enum để tránh giá trị sai
- query feed phải lọc theo visibility & ownership đúng policy
- Gợi ý query logic “homepage” theo policy:
  - Anonymous: `WHERE visibility='public' ORDER BY time_created DESC`
  - Logged-in user (my feed): `WHERE created_by_user = :uid AND visibility IN ('public','unlisted','draft') ORDER BY time_created DESC`

###### Các query “cơ bản” hiện tại (dùng cho Basic design)

Basic thì thường sẽ dùng các câu query/pattern này:

1. Anonymous homepage (public feed)
   `SELECT ... FROM articles WHERE visibility='public' ORDER BY time_created DESC;`
2. My feed (user login)
   `SELECT ... FROM articles WHERE created_by_user=:uid AND visibility IN ('public','unlisted','draft') ORDER BY time_created DESC;`
3. Create article
   `INSERT INTO articles(created_by_user, time_created, visibility, title, content) VALUES (...) RETURNING id;`
4. Update article (owner)
   `UPDATE articles SET ... WHERE id=:id AND created_by_user=:uid;`
5. Delete article (owner)
   `DELETE FROM articles WHERE id=:id AND created_by_user=:uid;`
6. View article by id (public)
   `SELECT ... FROM articles WHERE id=:id AND visibility='public';`
7. View “unlisted/draft”
   - Basic design (chỉ có visibility) thường sẽ không cấp quyền xem cho người khác, trừ khi là “owner” hoặc có token/link.
   - Nếu policy “unlisted = link/token share”:
     - lúc này cần thêm lookup theo token/link, tức là query sẽ khác (thường JOIN với bảng token).
8. List articles by author (nếu có UI author profile)
   `SELECT ... FROM articles WHERE created_by_user=:uid AND visibility='public' ORDER BY time_created DESC; (hoặc IN tùy login)`

###### Basic này giải quyết được bao nhiêu % bài toán “giống Facebook”?

- Nếu chấp nhận unlisted = chỉ chủ bài xem (không cần người khác xem qua link):
  - Basic có thể đạt khoảng 70–80% cho phần “feed + visibility + CRUD + ownership”.

- Với bài toán của hiện tại: unlisted = link/token share cho người khác xem
  - Basic “chỉ visibility + created_by_user” chưa đủ để cho người khác xem unlisted bằng link.
  - Trong trường hợp này, Basic thường đạt khoảng **50–60%**.

###### 40–50% bài toán còn lại thường là gì?

Phần còn lại rơi vào các nhóm sau:

1. Unlisted link/token share (quyền xem dựa trên bí mật trong URL) ~20%

- Cần bảng token/link (vd article_unlisted_links) hoặc ít nhất cơ chế lưu token và lookup token → article.
- Cần query + validate token + expiry + revoke.

2. Draft publish thành public/unlisted (versioning/publish workflow) ~10–20%

- Basic kiểu “một row, visibility đổi trạng thái” là đơn giản (nhưng không giữ lịch sử).
- Versioning đúng nghĩa (article_versions) thường tốn thêm.

3. Performance & scale feed ~10–15%

- Index đúng composite cho feed
- Pagination (cursor-based) thay vì “fetch hết”
- (sau này) caching, read replicas, v.v.

4. Privacy/security & API contract ~5–10%

- Tách entity/DTO để không lộ dữ liệu nhạy cảm
- Return đúng 401/403 thay vì rollback/500 mơ hồ

###### Basic có scale up tương lai được không?

Khả dĩ. Sau này scale up theo hướng:

- thêm bảng token để unlisted share thật
- thêm bảng version để draft/publish chuẩn hơn
- thêm index/pagination khi data lớn
