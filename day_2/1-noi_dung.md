- Stack vs Heap trong Rust

1. Stack: lưu các biến primitive types, ví dụ: int, float..., char, boolean
2. Ngoài đó ra: Heap là nơi lưu giá trị, Stack là nơi lưu địa chỉ đến vùng nhớ trên Heap

Chuỗi - String

String vs &str

Valid của variable trong Rust:

1. Khai báo một biến - Hệ thống ownership: 3 quy tắc vàng

- Mỗi value có duy nhất một owner
- Chỉ có một owner tại một thời điểm
- Khi owner ra khỏi scope, value bị drop, { } - scope
