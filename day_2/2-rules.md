**Hệ thống ownership:** 3 quy tắc vàng

- Mỗi value có duy nhất một owner (read/write) --- let mut a = 5; b = a;
- Chỉ có một owner tại một thời điểm
- Khi owner ra khỏi scope, value bị drop: { ... }

**References và borrowing:**

- Immutable reference `&T`: nhiều readers
- Mutable reference `&mut T`: một writer duy nhất
- Không thể có `&mut T` và `&T` cùng lúc

## Tính thời điểm (cùng lúc): là khoảng thời gian từ khi biến được khai báo đến thòi điểm cuối cùng sử dụng của biến (last used)

Smart Pointer
