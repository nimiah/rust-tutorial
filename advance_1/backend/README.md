
# Backend

## Config .env

### 🧠 Nguyên tắc cốt lõi

- Theo 12-Factor App:

> Config phải tách biệt hoàn toàn khỏi codebase.

#### Configuration Source

- .env
- .env.production
- Docker env
- Kubernetes ConfigMap
- Secret Manager

#### Configuration Loader

- dotenv (Node.js, Python, Rust, Go…)
- System environment (std::env trong Rust)
- Kubernetes inject env
- Docker inject env

### 🎯 Best-practice cho project

- .env chỉ dùng local, không dùng production.
- Commit .env.example, không commit .env.
- Production dùng system env hoặc secret manager.
- App luôn load config qua std::env (dotenv chỉ hỗ trợ local).
- Validate config khi startup (nếu thiếu → panic).
- Không hardcode secret trong code.

### 🧩 Workflow chuẩn, production ready

- OnceLock<AppConfig> → tạo global immutable config
- AppConfig::init() → load .env + parse config
- AppConfig::get() → lấy config ở mọi nơi trong code

⭐ **Ưu điểm**

- Global immutable config → không thể bị thay đổi sau khi init
- Thread-safe → OnceLock đảm bảo init đúng 1 lần
- Nhanh → không overhead runtime
- Dễ dùng → gọi AppConfig::get() ở bất kỳ đâu
- Tách biệt config khỏi code → đúng 12‑Factor App

⚠️ **Nhược điểm**

- Không reload được config khi runtime (nhưng 99% backend không cần)
- Nếu init fail → panic (nhưng đây là điều đúng: thiếu config thì app không chạy)
