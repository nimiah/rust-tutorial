# Cấu trúc Module trong Rust

## 1. Module là gì?

Module trong Rust là cách tổ chức code thành các nhóm logic, giúp:
- Quản lý namespace
- Kiểm soát visibility (public/private)
- Tái sử dụng code

## 2. Khai báo Module

### Inline module

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn private_helper() {
        // Chỉ truy cập được trong module math
    }
}

fn main() {
    let result = math::add(1, 2);
}
```

### Module trong file riêng

```
src/
├── main.rs
├── math.rs        // Module math
└── utils/
    ├── mod.rs     // Module utils
    └── helper.rs  // Submodule helper
```

**main.rs:**
```rust
mod math;      // Tìm file math.rs hoặc math/mod.rs
mod utils;     // Tìm thư mục utils/mod.rs

fn main() {
    math::add(1, 2);
    utils::helper::do_something();
}
```

## 3. Visibility (Phạm vi truy cập)

| Keyword | Ý nghĩa |
|---------|---------|
| (mặc định) | Private - chỉ trong module hiện tại |
| `pub` | Public - truy cập từ bên ngoài |
| `pub(crate)` | Public trong crate hiện tại |
| `pub(super)` | Public cho module cha |
| `pub(in path)` | Public cho path cụ thể |

```rust
mod outer {
    pub mod inner {
        pub fn public_fn() {}
        fn private_fn() {}
        pub(crate) fn crate_only() {}
        pub(super) fn parent_only() {}
    }
}
```

## 4. Sử dụng `use` để import

```rust
// Import một item
use std::collections::HashMap;

// Import nhiều items
use std::io::{Read, Write};

// Import tất cả (không khuyến khích)
use std::collections::*;

// Đổi tên với as
use std::io::Result as IoResult;

// Re-export
pub use crate::math::add;
```

## 5. Cấu trúc thư mục chuẩn

### Cách 1: File cùng cấp (Rust 2018+)
```
src/
├── main.rs
├── lib.rs
├── config.rs
└── config/
    └── database.rs
```

### Cách 2: mod.rs (cách cũ, vẫn hỗ trợ)
```
src/
├── main.rs
└── config/
    ├── mod.rs
    └── database.rs
```

## 6. Phân biệt Crate và Module

| Crate | Module |
|-------|--------|
| Đơn vị biên dịch | Đơn vị tổ chức code |
| Binary hoặc Library | Namespace trong crate |
| Có Cargo.toml riêng | Không có file config riêng |

```
my_project/           # Một crate
├── Cargo.toml
└── src/
    ├── lib.rs        # Crate root
    ├── module_a.rs   # Module
    └── module_b/     # Module với submodules
        ├── mod.rs
        └── sub.rs
```

## 7. Path trong Rust

```rust
// Absolute path - bắt đầu từ crate root
crate::module::function();

// Relative path - từ module hiện tại
self::function();      // Trong module hiện tại
super::function();     // Module cha

// External crate
std::io::Read;
```

## 8. Ví dụ thực tế

```rust
// src/lib.rs
pub mod models;
pub mod services;
mod utils;  // Private module

pub use models::User;  // Re-export để dễ sử dụng

// src/models/mod.rs
mod user;
mod product;

pub use user::User;
pub use product::Product;

// src/models/user.rs
pub struct User {
    pub name: String,
    pub(crate) email: String,  // Chỉ trong crate
    password: String,           // Private
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        Self { name, email, password }
    }
}
```

## 9. Best Practices

1. **Một module một trách nhiệm** - Giữ module nhỏ và tập trung
2. **Sử dụng `pub use` để tạo API đẹp** - Ẩn cấu trúc nội bộ
3. **Đặt tên rõ ràng** - Module name nên mô tả chức năng
4. **Hạn chế `pub`** - Chỉ expose những gì cần thiết
5. **Tránh circular dependencies** - Module A không nên phụ thuộc B và ngược lại

## 10. Lỗi thường gặp

```rust
// Lỗi: Không tìm thấy module
mod missing;  // Cần có file missing.rs hoặc missing/mod.rs

// Lỗi: Private item
mod inner {
    fn private() {}
}
inner::private();  // Error: private function

// Lỗi: Sai path
use crate::wrong::path;  // Error: module not found
```
