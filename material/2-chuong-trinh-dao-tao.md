# CHƯƠNG TRÌNH HUẤN LUYỆN RUST

## Tổng quan

**Thời gian:** 8-12 tuần part-time (hoặc 5-7 ngày full-time)

**Mục tiêu:** Đào tạo lập trình viên có khả năng xây dựng ứng dụng Rust thực tế với kiến thức vững chắc về ownership, concurrency, và clean architecture.

---

## GIAI ĐOẠN 1: CƠ BẢN (4-6 tuần / 30-40 giờ)

### Module 1: Giới Thiệu & Cài Đặt (2-3 giờ)

**Mục tiêu:** Hiểu được lý do sử dụng Rust và thiết lập môi trường phát triển

#### Nội dung lý thuyết

- **Tại sao Rust?** Hiệu năng cao (zero-cost abstractions), memory safety không cần GC, fearless concurrency
- **Khi nào dùng Rust:** Systems programming, backend services, CLI tools, embedded systems, WebAssembly
- **Khi nào không nên dùng Rust:** Prototyping nhanh, team thiếu thời gian học, projects đơn giản (scripting), GUI applications (ecosystem chưa mature)
- Lịch sử ra đời của Rust
- **So sánh Rust:**
  - vs C/C++: An toàn hơn (no segfaults, data races), tooling tốt hơn, learning curve tương đương
  - vs Java/C#: Không GC (latency thấp hơn), hiệu năng cao hơn, control memory tốt hơn
  - vs Go: Hiệu năng cao hơn, type system mạnh hơn, nhưng compile chậm hơn và phức tạp hơn
  - vs Python: Nhanh hơn 10-100x, type-safe, nhưng development chậm hơn
- **Công cụ Rust:**
  - `rustup`: Quản lý phiên bản Rust (toolchain manager)
  - `rustc`: Compiler của Rust
  - `cargo`: Build system và package manager (build, run, test, dependencies)

#### Thực hành

- Cài đặt Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Tạo project: `cargo new hello_rust`
- Chạy: `cargo run` (compile + execute)
- Quality tools: `cargo check` (compile nhanh), `cargo fmt` (format), `cargo clippy` (linting)
- Khám phá cấu trúc: `Cargo.toml` (dependencies), `src/main.rs` (entry point)

---

### Module 2: Cú Pháp Cơ Bản & Control Flow (4-5 giờ)

**Mục tiêu:** Nắm vững cú pháp cơ bản của Rust

#### Nội dung lý thuyết

- Variables, mutability, và shadowing
- Kiểu dữ liệu scalars: integers, floats, booleans, characters
- Kiểu dữ liệu compounds: tuples, arrays
- Functions và expressions vs statements
- Control flow: `if/else`, `match`, `loop`, `while`, `for`
- Comments và documentation comments

#### Thực hành

- Bài tập nhỏ: calculator cơ bản
- Game đoán số (number guessing)
- Refactor code sử dụng `match`

---

### Module 3: Ownership, Borrowing & Lifetimes (6-8 giờ) ⭐ **MODULE CỐT LÕI**

**Mục tiêu:** Hiểu thấu đáo hệ thống ownership - linh hồn của Rust

> Đây là module quan trọng nhất - nơi bạn thực sự "hiểu" Rust

#### Nội dung lý thuyết

- **Stack vs heap memory:** Khi nào data được lưu ở đâu, tại sao quan trọng
- **Hệ thống ownership:** 3 quy tắc vàng
  - Mỗi value có duy nhất một owner
  - Chỉ có một owner tại một thời điểm
  - Khi owner ra khỏi scope, value bị drop
- **Move semantics:**
  - **Move (default):** Ownership được transfer, giá trị cũ không dùng được nữa
    - Áp dụng cho types phức tạp: `String`, `Vec<T>`, `Box<T>`
    - Ví dụ: `let s2 = s1;` → `s1` không còn valid
  - **Copy trait:** Value được copy, giá trị cũ vẫn dùng được
    - Áp dụng cho types đơn giản: integers, floats, bool, char, tuples of Copy types
    - Ví dụ: `let x2 = x1;` → cả `x1` và `x2` đều valid
  - **Clone trait:** Deep copy thủ công khi cần
    - Tạo bản sao hoàn chỉnh trên heap
    - Ví dụ: `let s2 = s1.clone();` → cả `s1` và `s2` đều valid
    - Expensive operation, chỉ dùng khi thực sự cần
- **References và borrowing:**
  - Immutable reference `&T`: nhiều readers
  - Mutable reference `&mut T`: một writer duy nhất
  - Không thể có `&mut T` và `&T` cùng lúc
- **String slices:** `&str` vs `String`, khi nào dùng gì
- **Scope và drop:**
  - **RAII (Resource Acquisition Is Initialization):** Resources được giải phóng tự động khi owner ra khỏi scope
  - **Drop trait:** Destructor được gọi tự động khi value out of scope
  - Ví dụ: File handles, network connections, heap memory tự động cleanup
  - Thứ tự drop: LIFO (Last In, First Out) - biến khai báo sau drop trước
  - Manual drop: `drop(value)` để giải phóng sớm
- **Lifetimes:**
  - **Khái niệm:** Lifetime là khoảng thời gian (scope) mà một reference còn valid
  - **Vấn đề:** Ngăn chặn dangling references (trỏ đến data đã bị drop)
  - **Lifetime elision rules:** Compiler tự suy luận lifetime trong 3 trường hợp phổ biến
    - Rule 1: Mỗi reference parameter có lifetime riêng
    - Rule 2: Nếu có 1 input lifetime → output lifetime giống input
    - Rule 3: Với methods (`&self`), output lifetime giống `&self`
  - **Explicit lifetime annotations `'a`:**
    - Syntax: `fn foo<'a>(x: &'a str, y: &'a str) -> &'a str`
    - `'a` đọc là "lifetime a" - tên generic cho lifetime
    - Nghĩa: "Giá trị trả về sống ít nhất bằng cả `x` và `y`"
    - Ví dụ: `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`
  - **Multiple lifetimes:** Có thể có nhiều lifetime parameters: `'a`, `'b`, `'c`
  - **Lifetime trong structs:** Struct chứa references cần lifetime annotations
- **Borrow checker:** Công cụ phân tích static đảm bảo ownership rules
  - **Các lỗi phổ biến:**
    - `error[E0382]: borrow of moved value` - Dùng giá trị đã bị move
      ```rust
      let s1 = String::from("hello");
      let s2 = s1;  // s1 moved to s2
      println!("{}", s1);  // ❌ Error: s1 đã invalid
      ```
    - `error[E0502]: cannot borrow as mutable because also borrowed as immutable`
      ```rust
      let mut s = String::from("hello");
      let r1 = &s;     // immutable borrow
      let r2 = &mut s; // ❌ Error: không thể có &mut khi đã có &
      ```
    - `error[E0499]: cannot borrow as mutable more than once`
      ```rust
      let mut s = String::from("hello");
      let r1 = &mut s;
      let r2 = &mut s; // ❌ Error: chỉ một &mut tại một thời điểm
      ```
    - `error[E0597]: borrowed value does not live long enough` - Dangling reference
      ```rust
      let r;
      {
          let x = 5;
          r = &x;  // ❌ Error: x sẽ drop khi ra khỏi scope
      }
      println!("{}", r);
      ```
  - **Cách sửa:** Hiểu message, clone khi cần, dùng references đúng cách, restructure code

#### Thực hành

- Sửa các lỗi ownership phổ biến
- Refactor code sử dụng quá nhiều cloning
- Viết functions với references
- Bài tập lifetime annotations cơ bản
- Phân tích memory model của các đoạn code

---

### Module 4: Structs, Enums & Pattern Matching (3-4 giờ)

**Mục tiêu:** Tạo và sử dụng custom types

#### Nội dung lý thuyết

- Structs: định nghĩa và sử dụng custom types
- Methods và associated functions (impl blocks)
- Tuple structs và unit-like structs
- Enums và sum types
- `Option<T>` và `Result<T, E>`
- Pattern matching và destructuring
- `if let` / `while let`

#### Thực hành

- Mô hình hóa domain sử dụng structs và enums
- Thay thế nulls/exceptions bằng `Option` & `Result`
- Bài tập xử lý lỗi thực tế

---

### Module 5: Xử Lý Lỗi (2-3 giờ)

**Mục tiêu:** Xử lý lỗi an toàn và hiệu quả

#### Nội dung lý thuyết

- **Recoverable vs unrecoverable errors:**
  - **Unrecoverable errors (panic):**
    - Lỗi nghiêm trọng, không thể recovery: out of bounds, assertion failures
    - Program crash, print stack trace, cleanup resources
    - Ví dụ: `panic!("Critical error!")`, array access out of bounds
    - Khi dùng: bugs, programmer errors, không thể tiếp tục an toàn
  - **Recoverable errors (Result):**
    - Lỗi có thể xử lý: file not found, network timeout, parse errors
    - Trả về `Result<T, E>` để caller quyết định xử lý
    - Ví dụ: `File::open()` trả về `Result<File, Error>`
    - Khi dùng: expected errors, I/O operations, user input validation
- **`panic!` và unwinding:**
  - `panic!`: Macro để trigger unrecoverable error
  - **Unwinding (default):** Cleanup stack, drop values, chạy destructors
  - **Abort:** Terminate ngay lập tức, không cleanup (config `panic = 'abort'`)
  - `catch_unwind`: Bắt panic trong rare cases
- **`Result<T, E>` chi tiết:**
  - Enum: `Ok(T)` cho success, `Err(E)` cho error
  - Methods: `.unwrap()`, `.expect()`, `.unwrap_or()`, `.unwrap_or_else()`
  - Pattern matching: `match result { Ok(val) => ..., Err(e) => ... }`
  - Combinators: `.map()`, `.map_err()`, `.and_then()`, `.or_else()`
- **Toán tử `?`:**
  - Syntax sugar cho early return khi error
  - `let file = File::open("data.txt")?;` → return `Err` nếu fail
  - Tự động convert error types với `From` trait
  - Chỉ dùng trong functions trả về `Result` hoặc `Option`
- **Custom error types:**
  - Define struct/enum cho domain-specific errors
  - Implement `Error` trait và `Display`
  - Sử dụng crates: `thiserror` (derive macros), `anyhow` (dynamic errors)
- **Best practices cho error propagation:**
  - Dùng `?` thay vì `unwrap()` trong library code
  - Return `Result` từ functions có thể fail
  - Tạo meaningful error messages
  - Chain errors để preserve context
  - Application errors vs library errors

#### Thực hành

- Xây dựng error handling vững chắc
- Tạo custom error types
- Sử dụng `?` operator hiệu quả

---

### Module 6: Collections & Standard Library (3-4 giờ)

**Mục tiêu:** Làm việc hiệu quả với collections

#### Nội dung lý thuyết

- `Vec<T>` - Vectors
- `String` vs `&str`
- `HashMap<K, V>` và `HashSet<T>`
- Khi nào sử dụng từng loại collection
- Iterators vs loops
- Iterator adapters: `map`, `filter`, `fold`
- Ownership với collections

#### Thực hành

- Bài tập xử lý dữ liệu với collections
- Refactor imperative code sang functional style với iterators
- Xây dựng ứng dụng quản lý dữ liệu đơn giản

---

### Module 7: Modules, Crates & Package Management (2-3 giờ)

**Mục tiêu:** Tổ chức code và quản lý dependencies

#### Nội dung lý thuyết

- Hệ thống module: `mod`, `pub`, `use`
- Quy tắc privacy và visibility
- Re-exports
- Tạo và tổ chức packages và crates
- Làm việc với external dependencies
- Cấu trúc crate best practices
- Workspaces cơ bản

#### Thực hành

- Refactor project thành modules có cấu trúc
- Tạo library crate
- Sử dụng external crates từ crates.io

---

### Module 8: Testing Cơ Bản (2-3 giờ)

**Mục tiêu:** Viết tests để đảm bảo chất lượng code

#### Nội dung lý thuyết

- Unit tests và integration tests
- Thuộc tính `#[test]` và assert macros
- Tổ chức test và best practices
- Documentation tests
- Test-driven development (TDD) cơ bản

#### Thực hành

- Viết unit tests cho existing code
- Tạo integration tests
- Thực hành TDD với bài tập nhỏ

---

### Dự Án Giai Đoạn 1: CLI Tool (4-6 giờ)

**Mục tiêu:** Áp dụng tất cả kiến thức đã học vào project thực tế

#### Yêu cầu dự án

Xây dựng một trong các CLI tools sau:

- Todo list manager
- File search tool
- CSV data processor
- Log analyzer
- Text-based calculator nâng cao

#### Focus

- Cấu trúc code với modules
- Error handling toàn diện
- Unit tests và integration tests
- Documentation chuẩn
- CLI arguments parsing (sử dụng crate như `clap`)

---

## GIAI ĐOẠN 2: NÂNG CAO (4-6 tuần / 30-35 giờ)

### Module 9: Traits & Generics (6-8 giờ)

**Mục tiêu:** Viết code có tính tái sử dụng cao và abstraction

#### Nội dung lý thuyết

- Generic data types và functions
- Định nghĩa và triển khai traits
- Trait bounds và where clauses
- `impl Trait` syntax
- Static vs dynamic dispatch
- Trait objects (`dyn Trait`)
- Associated types
- Default implementations
- Derive macros (`Debug`, `Clone`, `Serialize`, etc.)

#### Thực hành

- Tạo generic data structures
- Implement traits cho custom types
- So sánh performance: trait objects vs generics
- Xây dựng polymorphic APIs

---

### Module 10: Iterators & Closures (4-5 giờ)

**Mục tiêu:** Functional programming trong Rust

#### Nội dung lý thuyết

- Closure syntax và capture modes
- Iterator trait và các phương thức phổ biến
- Tạo custom iterators
- Zero-cost abstractions
- Performance considerations
- Functional programming patterns

#### Thực hành

- Implement custom Iterator
- Data processing pipeline với iterators
- Refactor loops thành iterator chains
- Performance benchmarking

---

### Module 11: Smart Pointers (3-4 giờ)

**Mục tiêu:** Hiểu và sử dụng smart pointers hiệu quả

#### Nội dung lý thuyết

- `Box<T>` cho heap allocation
- `Rc<T>` và `Arc<T>` cho reference counting
- `RefCell<T>` và interior mutability
- `Cell<T>` cho copy types
- `Mutex<T>` và `RwLock<T>`
- Khi nào sử dụng từng smart pointer
- Circular references và `Weak<T>`

#### Thực hành

- Xây dựng cấu trúc dữ liệu phức tạp (tree, graph)
- Sử dụng `Rc` và `RefCell` kết hợp
- Thread-safe shared state với `Arc` và `Mutex`

---

### Module 12: Concurrency & Parallelism (5-6 giờ)

**Mục tiêu:** Viết concurrent code an toàn

#### Nội dung lý thuyết

- Fearless concurrency concept
- Threads và `std::thread`
- Message passing với `mpsc` channels
- Shared state: `Arc`, `Mutex`, `RwLock`
- Traits `Send` và `Sync`
- Thread safety guarantees
- Common concurrency patterns
- Data races và cách Rust ngăn chặn

#### Thực hành

- Multi-threaded worker pool
- Producer-consumer pattern
- Parallel data processing
- Sửa data races sử dụng type system

---

### Module 13: Async Rust (5-6 giờ)

**Mục tiêu:** Lập trình bất đồng bộ với async/await

#### Nội dung lý thuyết

- Async programming model
- Futures và `Future` trait
- `async` / `await` syntax
- Async runtimes: Tokio overview
- Async vs threads: khi nào dùng gì
- Common async patterns
- Error handling trong async context
- Các pitfalls phổ biến

#### Thực hành

- Async HTTP client
- Simple async web service với Tokio
- Concurrent async operations
- Async file I/O

---

### Module 14: Advanced Topics (4-6 giờ)

**Mục tiêu:** Các chủ đề nâng cao cho use cases đặc biệt

#### Nội dung lý thuyết

- Unsafe Rust: khi nào và tại sao
- Raw pointers
- FFI (Foreign Function Interface) - gọi C code
- Safety contracts
- Macros: declarative (`macro_rules!`)
- Procedural macros overview
- Performance optimization techniques
- Profiling basics với `cargo flamegraph`
- Benchmarking với `criterion`

#### Thực hành

- Đọc và phân tích unsafe code
- Wrap unsafe logic một cách an toàn
- Tạo simple declarative macro
- Profile và optimize một function
- Benchmark performance improvements

---

### Module 15: Clean Architecture & Design Patterns (4-5 giờ)

**Mục tiêu:** Thiết kế ứng dụng với clean architecture

#### Nội dung lý thuyết

- Clean architecture principles trong Rust
- Domain-driven design (DDD)
- Dependency injection patterns
- Repository pattern
- Service layer design
- Error handling strategies ở architecture level
- Testing strategies (unit, integration, e2e)
- Cấu trúc project best practices

#### Thực hành

- Refactor monolithic code sang layered architecture
- Implement repository pattern
- Dependency injection without frameworks
- Mock và test doubles

---

### Module 16: Ecosystem & Best Practices (2-3 giờ)

**Mục tiêu:** Làm việc hiệu quả với Rust ecosystem

#### Nội dung lý thuyết

- Popular crates overview (serde, tokio, actix, axum, sqlx, etc.)
- Crates.io ecosystem
- Code formatting với `rustfmt`
- Linting với `clippy`
- Documentation với `rustdoc`
- CI/CD cho Rust projects
- Community resources

#### Thực hành

- Configure clippy và rustfmt
- Generate và deploy documentation
- Setup GitHub Actions cho Rust project

---

### Dự Án Cuối Khóa: Microservice Backend (8-12 giờ)

**Mục tiêu:** Xây dựng một backend service hoàn chỉnh

#### Yêu cầu dự án

Xây dựng một REST API microservice với:
Clean Architecture

**Chức năng:**

- CRUD operations cho một domain (user, product, order, etc.)
- Authentication & authorization
- Database integration (PostgreSQL/MySQL)
- Input validation
- Error handling toàn diện
- Logging và monitoring
- API documentation

**Công nghệ stack:**

- Web framework: Axum hoặc Actix-web
- Database: sqlx hoặc diesel
- Serialization: serde
- Validation: validator
- Testing: mockall, wiremock
- Documentation: utoipa (OpenAPI)

**Architecture:**

- Clean architecture / Layered architecture
- Domain layer
- Application/Service layer
- Infrastructure layer
- Presentation/API layer

**Best practices:**

- Error handling với custom error types
- Async/await throughout
- Unit tests + integration tests
- API documentation
- Docker containerization
- CI/CD pipeline

---

## Kết Quả Đào Tạo

### Sau Giai Đoạn 1 (Cơ Bản)

Học viên có thể:

- Hiểu ownership, borrowing, và lifetimes
- Viết safe Rust code
- Xử lý lỗi đúng cách
- Làm việc với collections và iterators
- Tổ chức code với modules
- Xây dựng CLI tools đơn giản
- Viết tests cơ bản

### Sau Giai Đoạn 2 (Nâng Cao)

Học viên có thể:

- Thiết kế APIs với traits và generics
- Viết concurrent và async code
- Sử dụng smart pointers hiệu quả
- Tối ưu hóa performance
- Áp dụng clean architecture
- Xây dựng microservice backend
- Đọc và review production code
- Làm việc với Rust ecosystem

---

## Phương Pháp Giảng Dạy

### Nguyên tắc

- **Compiler errors là công cụ học tập** - embrace them
- Giải thích _tại sao_ trước _cái gì_
- Sử dụng mô hình memory trực quan (diagrams)
- Hands-on coding chiếm 60-70% thời gian
- Code review và pair programming
- Khuyến khích đọc production code

### Đánh giá

- Quizzes sau mỗi module
- Code review cho bài tập
- Presentation cho dự án cuối
- Peer review

---

## Tài Nguyên Học Tập

### Sách

- The Rust Programming Language (The Book)
- Rust by Example
- Programming Rust (O'Reilly)

### Online

- Rustlings exercises
- Exercism Rust track
- Rust documentation
- This Week in Rust

### Community

- Rust Users Forum
- r/rust subreddit
- Discord channels
- Local meetups

---

**Lưu ý:** Chương trình này có thể điều chỉnh linh hoạt tùy theo:

- Background của học viên (C++/Java/Go developers)
- Mục tiêu cụ thể (backend/systems/embedded)
- Thời gian có sẵn (full-time/part-time)
