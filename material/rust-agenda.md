Thời gian:

- **5–7 ngày full-time**, hoặc
- **10–14 buổi nửa ngày**, hoặc
- **8–12 tuần part-time**

---

## Mục Tiêu Khóa Học

Sau khóa học, bạn sẽ:

- Hiểu được **mô hình tư duy của Rust** (ownership, borrowing, lifetimes)
- Viết **Rust an toàn và chuẩn mực**
- Xây dựng **ứng dụng thực tế** (CLI, service, hoặc library)
- Suy luận về **hiệu năng và concurrency**
- Tự tin đọc code Rust trong các dự án production
- Hiểu được **cấu trúc clean** trong Rust
- Xây dựng microservice Rust backend

---

# Chương Trình Khóa Học Rust

## 1. Giới Thiệu & Tư Duy Rust (½ ngày)

### Nội dung

- Tại sao lại là Rust? (An toàn, hiệu năng, concurrency)
- Rust ra đời như thế nào? [https://youtu.be/LAy7EfGF_1w?si=eqmjQL2DpnVrYT6t]
- Rust vs C/C++ / Java / Go / Python
- Khi _nào không nên_ dùng Rust
- Tổng quan về công cụ Rust

  - `rustc`, `cargo`, `rustup`

### Thực hành

- Cài đặt Rust
- Tạo Cargo project đầu tiên
- Build & run
- Khám phá `cargo check`, `cargo fmt`, `cargo clippy`

---

## 2. Cú Pháp Cơ Bản & Control Flow (½ ngày)

### Nội dung

- Variables & immutability
- Shadowing
- Scalar & compound types
- Functions & expressions
- `if`, `match`, loops (`loop`, `while`, `for`)
- Comments & documentation comments

### Thực hành

- Bài tập nhỏ (calculator, number guessing)
- Refactor code sử dụng `match`

---

## 3. Ownership, Borrowing & Lifetimes (2 ngày – **module cốt lõi**)

> Đây là nơi bạn "hiểu thấu" Rust hoặc không.

### Nội dung

- Stack vs heap
- Quy tắc ownership
- Move vs copy
- Borrowing (`&T`, `&mut T`)
- Quy tắc mutable aliasing
- Scope và drop
- Lifetimes (khái niệm → tường minh)
- Lỗi compiler phổ biến & cách đọc chúng

### Thực hành

- Sửa lỗi ownership
- Refactor code sử dụng quá nhiều cloning
- Viết functions với references
- Lifetime annotations đơn giản

---

## 4. Structs, Enums & Pattern Matching (1 ngày)

### Nội dung

- Structs & impl blocks
- Tuple structs & unit structs
- Enums là _sum types_
- `Option<T>` và `Result<T, E>`
- Pattern matching & destructuring
- `if let` / `while let`

### Thực hành

- Mô hình hóa domain sử dụng enums
- Thay thế nulls/exceptions bằng `Option` & `Result`
- Bài tập xử lý lỗi

---

## 5. Modules, Crates & Visibility (½ ngày)

### Nội dung

- Modules và `mod`
- `pub` và quy tắc privacy
- `use` và re-exports
- Best practices về cấu trúc crate
- Cơ bản về Workspace

### Thực hành

- Refactor project thành modules
- Tạo library crate

---

## 6. Traits & Generics (1 ngày)

### Nội dung

- Định nghĩa và triển khai traits
- Trait bounds
- Generic types & functions
- `impl Trait`
- Static vs dynamic dispatch
- Trait objects (`dyn Trait`)
- Derive macros (`Debug`, `Clone`, v.v.)

### Thực hành

- Generic data structures
- Implement traits cho custom types
- So sánh trait objects vs generics

---

## 7. Xử Lý Lỗi & Testing (½ ngày)

### Nội dung

- Best practices với `Result`
- Custom error types
- `thiserror` / `anyhow` (khái niệm)
- Panic vs recoverable errors
- Unit tests
- Integration tests
- Doc tests

### Thực hành

- Xây dựng xử lý lỗi vững chắc
- Viết tests cho code hiện có

---

## 8. Collections & Standard Library (½ ngày)

### Nội dung

- `Vec`, `HashMap`, `HashSet`
- String vs `&str`
- Iterators vs loops
- Iterator adapters (`map`, `filter`, `fold`)
- Ownership với collections

### Thực hành

- Xử lý dữ liệu với iterators
- Refactor imperative code sang functional style

---

## 9. Concurrency & Parallelism (1 ngày)

### Nội dung

- Khái niệm fearless concurrency
- Threads & `std::thread`
- Message passing (`mpsc`)
- Shared state (`Arc`, `Mutex`, `RwLock`)
- `Send` và `Sync`
- Giới thiệu async/await (khái niệm)

### Thực hành

- Multi-threaded worker pool
- Sửa data races sử dụng đảm bảo của Rust

---

## 10. Async Rust (Tùy chọn / Nâng cao – 1 ngày)

### Nội dung

- Mô hình Async & futures
- `async` / `await`
- Executors (tổng quan Tokio)
- Async vs threads
- Các lỗi phổ biến

### Thực hành

- Async HTTP client
- Simple async service

---

## 11. Unsafe Rust & FFI (Nâng cao – ½ ngày)

### Nội dung

- `unsafe` có nghĩa là gì (và không có nghĩa là gì)
- Raw pointers
- Khi nào unsafe được chứng minh
- Gọi C code (FFI)
- Safety contracts

### Thực hành

- Đọc unsafe code
- Wrap unsafe logic một cách an toàn

---

## 12. Performance & Memory (½ ngày)

### Nội dung

- Zero-cost abstractions
- Kiểm soát heap allocation
- Copy vs clone
- Cơ bản về Profiling
- Benchmarks (`criterion`)

### Thực hành

- Tối ưu hóa một function chậm
- Đo lường tác động hiệu năng

---

## 13. Dự Án Rust Thực Tế (1–2 ngày)

### Lựa chọn

- CLI tool
- REST API
- Log parser
- Concurrent file processor
- Library với public API

### Tập trung vào

- Code review
- Idiomatic Rust
- Error handling
- Tests & docs

---

## Mẹo Giảng Dạy

- **Compiler errors là công cụ giảng dạy**—hãy chấp nhận chúng
- Giải thích _tại sao_ các quy tắc tồn tại trước cú pháp
- Sử dụng **mô hình memory trực quan** (sơ đồ ownership)
- Khuyến khích đọc code Rust thực tế (std, popular crates)

---

## Kết Quả Mục Tiêu Theo Cấp Độ

### Junior Developers

- Hiểu ownership & borrowing
- Viết safe Rust mà không phải đấu tranh với compiler
- Xây dựng small tools một cách tự tin

### Mid-Senior Developers

- Thiết kế APIs sử dụng traits & lifetimes
- Suy luận về performance & concurrency
- Đọc và review production Rust code

---

Nếu bạn muốn, tôi cũng có thể:

- Điều chỉnh cho **backend**, **systems**, hoặc **embedded**
- Tạo **slides + exercises**
- Cung cấp **kế hoạch theo tuần**
- Điều chỉnh cho **developers C++/Java/Go**

Chỉ cần cho tôi biết đối tượng học viên và giới hạn thời gian của bạn 🙂
