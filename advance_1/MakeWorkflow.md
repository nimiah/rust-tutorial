4 kịch bản (Flows) sử dụng Makefile này để quản lý vòng đời phát triển dự án của bạn:

## Kịch bản 1: Lần đầu Setup (Onboarding Flow)

Dành cho bạn khi vừa clone project về hoặc cho một thành viên mới gia nhập team.

1. Vào môi trường: make shell (Nếu bạn không dùng direnv).
2. Bật hạ tầng: make db-up. (Lệnh này bật Postgres và chờ 3s để DB sẵn sàng).
3. Khởi tạo DB: make db-reset. (Xóa sạch dấu vết cũ, tạo DB mới và chạy migration từ đầu).
4. Kiểm tra: make check. (Để đảm bảo Rust compiler và SQLx macro có thể kết nối DB thành công).

## Kịch bản 2: Phát triển tính năng mới (Feature Development Flow)

Đây là quy trình bạn sẽ lặp lại hằng ngày khi code logic và thay đổi DB.

1. Chạy server chế độ dev: make watch. (Server sẽ tự reload mỗi khi bạn nhấn Ctrl+S).
2. Khi cần thêm bảng/cột mới:

- make migrate-add name=create_orders_table. (Tạo file SQL trắng).
- Bạn viết SQL vào file vừa tạo.
- make migrate-run. (Áp dụng thay đổi vào DB ngay lập tức).

3. Viết code Rust dùng bảng mới: Rust compiler (thông qua make check) sẽ báo lỗi ngay nếu bạn gõ sai tên cột vừa tạo trong file SQL.

## Kịch bản 3: Trước khi Push Code (Quality Gate Flow)

Để đảm bảo code của bạn không làm hỏng CI/CD.

1. Chạy Test: make test. (Chạy các unit test và integration test).
2. Cập nhật Offline Cache: make prepare.

- Tại sao? Vì trên Server CI (như GitHub Actions) thường không có sẵn Postgres. Lệnh này lưu kết quả query vào file JSON để Rust có thể compile "chay" mà không cần DB thật.

3. Kiểm tra cuối: make check.

## Kịch bản 4: Xử lý sự cố (Troubleshooting Flow)

Khi bạn làm loạn các file migration hoặc DB bị "rác" data.

1. Làm sạch hoàn toàn: make db-down. (Sập container, xóa sạch volumes).
2. Dựng lại từ đầu: make db-up -> make db-reset.
3. Dọn dẹp rác Rust: make clean. (Xóa thư mục target nếu gặp lỗi biên dịch kỳ lạ).

```bash
cd advance_1
make shell          # 1. Vào môi trường Nix
make pg-start       # 2. Bật engine Postgres (Bắt buộc phải có trước khi dùng sqlx)

# Bước quan trọng nếu là lần đầu hoặc vừa reset:
make db-reset       # 3. Lệnh này thực hiện: sqlx database drop (nếu có) -> create -> migrate run
                    #    (Nó đảm bảo DB tồn tại và Schema được nạp vào)

make prepare        # 4. Sinh cache cho macro (Giúp Rust analyzer và compiler ko bị đỏ)
make check          # 5. Kiểm tra Type-safe toàn bộ project
make watch          # 6. Chạy code và bắt đầu phát triển
... code ...
make test           # 7. Chạy test
...
make pg-stop        # 8. Dọn dẹp: Tắt DB
exit                # 9. Thoát shell
```
