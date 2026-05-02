# Mikael Feed Clone

## Dev Phase

Để xây dựng một dự án Fullstack Feed Clone một cách bài bản, tránh việc bị "ngợp" giữa đống code, nên chia lộ trình theo mô hình **"Skeleton-First"** (Xây bộ khung trước, đắp thịt sau).

Dưới đây là tầm nhìn hệ thống gồm 6 giai đoạn chính:

---

### Giai đoạn 1: Thiết lập "Hạ tầng lõi" (Infrastructure)
Không vội code logic ngay. Đầu tiên đảm bảo môi trường phát triển (DX) thật mượt.
*   **Database:** Hoàn thiện `docker-compose` và `flake.nix` (như Michael đang làm). Chạy được migrations và seed dữ liệu mẫu (200 bài viết) vào Postgres.
*   **Contracts (API Spec):** Xác định cấu trúc JSON mà BE sẽ trả về. Ví dụ: `GET /api/articles` trả về mảng articles có những field nào.
*   **BE Skeleton:** Khởi tạo project Axum, kết nối thành công SQLx tới DB, chạy được server ở port 3000 với 1 endpoint "Health Check".

### Giai đoạn 2: FE Mockup & BE "Hello Feed" (Mock test)
Giai đoạn này giúp bạn nhìn thấy sản phẩm nhanh nhất.
*   **FE (Mock Data):** Dùng React/Vue/Svelte tạo UI. Chưa cần fetch API, cứ tạo một file `mock_data.json` và render nó lên màn hình. Mục tiêu: UI hiển thị đúng danh sách bài viết, phân trang, và layout.
*   **BE (Static DB Query):** Viết duy nhất một API `GET /api/articles`. Backend query thẳng vào bảng `articles_demo` vừa tạo và trả về JSON. Chưa cần quan tâm đến Auth (đăng nhập) hay Filter (lọc).

### Giai đoạn 3: Cuộc "bắt tay" (The Handshake)
Đây là lúc nối hai nửa lại với nhau.
*   **CORS:** Cấu hình Axum cho phép FE (port 5173/3000) truy cập.
*   **Frontend Fetch:** Thay thế mock data bằng lệnh gọi `fetch()` hoặc `axios` tới Backend.
*   **Validation cơ bản:** Đảm bảo kiểu dữ liệu (Typescript ở FE và Struct ở Rust) khớp nhau hoàn toàn để tránh lỗi "undefined" trên màn hình.

### Giai đoạn 4: Hệ thống xác thực (The Gatekeeper)
Đây là phần khó và quan trọng nhất.
*   **Database:** Tạo bảng `users`.
*   **BE Logic:** Viết API `POST /api/users/login` và `register`. Triển khai **JWT (JSON Web Token)**.
*   **FE State Management:** Lưu Token vào LocalStorage/Cookie. Thiết lập "Private Routes" (chỉ ai đăng nhập mới thấy nút "New Post").

### Giai đoạn 5: Hoàn thiện CRUD & Features
Khi đã có Auth, đắp các tính năng còn lại theo vòng lặp:
1.  **Create:** Tạo bài viết mới.
2.  **Update/Delete:** Chỉ chủ bài viết mới được sửa/xóa.
3.  **Interactions:** Like, Comment, Follow (Các mối quan hệ bảng trung gian trong DB).
4.  **Filters:** Lọc bài viết theo Tag, theo Tác giả (Query SQL phức tạp hơn).

### Giai đoạn 6: Tối ưu & Chuyên nghiệp hóa
*   **Error Handling:** Trả về mã lỗi 400, 401, 404 một cách thống nhất.
*   **API Documentation:** Tích hợp **Swagger/OpenAPI** để người khác nhìn vào là biết cách dùng API.
*   **Testing:** Viết Integration Test cho Backend (dùng SQLx để test các câu query quan trọng).

### Lộ trình tóm tắt

| Bước | Thành phần | Công việc chính |
| :--- | :--- | :--- |
| **1** | **DB & Seed** | Fix xong lỗi `generate_series` để có 200 bài viết mẫu. |
| **2** | **BE API** | Viết endpoint `/api/articles` lấy dữ liệu từ DB trả về JSON. |
| **3** | **FE UI** | Dựng khung Feed, fetch dữ liệu từ BE và render ra list. |
| **4** | **Auth** | Làm chức năng Login/Register với JWT. |
| **5** | **Full Logic** | Post bài, Comment, Like, Tag. |
