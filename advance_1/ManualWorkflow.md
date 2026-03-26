# Work flow

## Prerequisites

- install Nix Determinate:

```bash
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```

- then initialize Dev Env:

```bash
cd rust-tutorial
nix develop
```

## 🚀 setup lần đầu (first-time setup)

1. **khởi động hạ tầng:**

```bash
cd docker && docker-compose up -d && cd ../backend
```

2. **Cấu hình môi trường:**

```bash
# Tạo file .env và sửa DATABASE_URL cho khớp với docker-compose
echo 'DATABASE_URL=postgres://admin:admin123@localhost:5432/mydatabase' > .env
```

3. **Khởi tạo Database & Schema:**

```bash
# Tạo DB dựa trên DATABASE_URL trong .env
sqlx database create

# Tạo migration đầu tiên từ file schema có sẵn
sqlx migrate add init_schema
cat docs/initial_db.sql > migrations/*_init_schema.sql

# Thực thi migration
sqlx migrate run
```

## 🛠 Workflow hằng ngày (Daily Workflow)

- Khi thay đổi Database Schema:

```bash
sqlx migrate add <ten_migration>
# Sửa file .sql vừa tạo trong thư mục migrations/
sqlx migrate run
```

- Khi thay đổi code Rust (SQL macro):

```bash
# Cập nhật cache offline cho CI/CD
cargo sqlx prepare
# Kiểm tra Type-safe
cargo check
```

- Chạy Server:

```bash
cargo watch -x run
```

## (Optional) Clean Remove DB

```bash
# Xóa DB hiện tại và tạo lại cái mới sạch sẽ
rm -rf migrations
sqlx database drop -y
# rồi thao tác như [🚀 setup lần đầu]
```

## Test

### Cách 1: Sử dụng Swagger UI

1. Mở Browser (Chrome/Firefox).
2. Truy cập: `http://localhost:3000/swagger-ui`
3. Tại đây, bạn sẽ thấy danh sách các API: POST /api/user, GET /api/users/{id}, v.v.
4. Bạn có thể nhấn **"Try it out"**, nhập dữ liệu JSON và nhấn **"Execute"**.

### Cách 2: Sử dụng HTTPie/Postman

1. Tạo User mới (POST):

```bash
http POST :3000/api/user name="Michael" email="michael@staff.com"
```

2. Lấy danh sách Users (GET):

```bash
http GET :3000/api/users
```

3. Lấy chi tiết 1 User (GET):

```bash
# Giả sử ID là 1
http GET :3000/api/users/1
```

4. Cập nhật User (PUT):

```bash
http PUT :3000/api/users/1 name="Michael Refactored" email="michael@new.com"
```

### Cách 3: Sử dụng cURL

```bash
curl -X POST http://localhost:3000/api/user \
     -H "Content-Type: application/json" \
     -d '{"name": "Michael", "email": "michael@staff.com"}'
```

- Local check:

```bash
# Start Postgres
pg_ctl status > /dev/null 2>&1 || pg_ctl start -l $(PGDATA)/postgres.log -o "-c listen_addresses='localhost' -p 5432"
# Check Postgres connection
pg_isready -h localhost -p 5432
# Open Postgres
psql postgresql://admin:admin123@localhost:5432/mydatabase
sqlx migrate run
sqlx migrate info
```

Docker check:

```bash
# Start Postgres
docker-compose up -d
docker exec -it <container_name> psql -U admin -d mydatabase
sqlx migrate run
sqlx migrate info
```
