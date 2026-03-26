# Rust Tutorial

## Syncing a Fork

### 1. Thiết lập Remote "Upstream"

Đầu tiên, cần cho Git biết đâu là nguồn gốc (repo của Author). Chỉ cần làm bước này **một lần duy nhất**.

```bash
# Thêm repo của Author vào danh sách quản lý với tên là 'upstream'
git remote add upstream https://github.com/Author/ten-repo-goc.git

# Kiểm tra lại, bạn sẽ thấy có 2 remotes: origin (của Michael) và upstream (của Author)
git remote -v
```

### 2. Quy trình Cập nhật (The Professional Sync Flow)

Nên sử dụng lệnh **`rebase`** thay vì `merge`.

- **Merge:** Tạo ra một commit rác "Merge branch..." làm rối lịch sử.
- **Rebase:** Đặt các bản refactor của mình lên trên cùng những update mới nhất của Author. Nó giúp lịch sử code trông như thể Michael luôn làm việc dựa trên bản mới nhất của Author.

**Các bước thực hiện:**

```bash
# 1. Lưu tạm các code đang sửa dở (nếu có)
git stash

# 2. Lấy code mới nhất từ Author về máy (nhưng chưa áp dụng vào code mình)
git fetch upstream

# 3. Đưa các update của Author vào nhánh hiện tại và "đẩy" commit của Michael lên trên
git rebase upstream/main  # (hoặc tên nhánh chính của Author như 'master')

# 4. Nếu có conflict (xung đột code)
# Git sẽ dừng lại ở file bị xung đột. Michael mở file đó lên, chọn phần code đúng.
# Sau đó:
git add <file_da_sua>
git rebase --continue

# 5. Lấy code đang sửa dở lại
git stash pop
```
