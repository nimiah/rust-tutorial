# Load .env rồi dev.env (dev.env override .env)
include .env
-include dev.env
export

# ─── Config ──────────────────────────────────────────────────────────────────

DOCKER_COMPOSE = docker compose -f ./docker-compose.yaml
MIGRATIONS_DIR = ./backend/migrations

# ─── Defaults ────────────────────────────────────────────────────────────────

.DEFAULT_GOAL := help
.PHONY: help \
        db-up db-down db-reset db-connect \
        db-add db-migrate db-migrate-revert db-migrate-info \
        dev build test fmt lint clean

# ─── Help ────────────────────────────────────────────────────────────────────

help: ## Hiển thị các lệnh hiện có
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' Makefile \
		| sort \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-22s\033[0m %s\n", $$1, $$2}'

# ─── Database: Container ─────────────────────────────────────────────────────

debug-env: ## In ra DATABASE_URL đang dùng
	@echo "DATABASE_URL = $(DATABASE_URL)"

# sudo systemctl stop open-webui
check-port: ## Kiểm tra xem cổng 5432 có bị chiếm không
	@lsof -i :5432 > /dev/null && echo "⚠️ Cảnh báo: Cổng 5432 đang bị chiếm bởi một tiến trình khác!" || echo "✓ Cổng 5432 sẵn sàng."

db-up: ## Khởi động Postgres container
	$(DOCKER_COMPOSE) up -d
	@echo "Đang chờ Postgres sẵn sàng..."
	@sleep 3
	@echo "✓ Postgres ready"

db-down: ## Dừng Postgres container (giữ data)
	$(DOCKER_COMPOSE) down

db-reset: ## Xóa toàn bộ data và tạo lại từ đầu
	$(DOCKER_COMPOSE) down -v
	$(MAKE) db-up
	# $(MAKE) db-migrate

db-connect: ## Kết nối psql vào database
	$(DOCKER_COMPOSE) exec postgres psql "$(DATABASE_URL)"

# ─── Database: Migration ─────────────────────────────────────────────────────

db-add: ## Tạo migration mới — dùng: make db-add name=create_users
	cd backend && cargo sqlx migrate add $(name)

db-migrate: ## Chạy tất cả migration chưa apply
	cd backend && cargo sqlx migrate run --database-url $(DATABASE_URL)

db-migrate-revert: ## Revert migration gần nhất
	cd backend && cargo sqlx migrate revert --database-url $(DATABASE_URL)

db-migrate-info: ## Xem trạng thái các migration
	cd backend && cargo sqlx migrate info --database-url $(DATABASE_URL)

db-prepare: ## Generate sqlx offline cache (cho CI không cần DB)
	cd backend && cargo sqlx prepare --database-url $(DATABASE_URL)

# ─── Development ─────────────────────────────────────────────────────────────

debug: ## Build rồi chờ debugger attach (F5 trong VSCode)
	cd backend && cargo build
	@echo "✓ Binary built — nhấn F5 trong VSCode để debug"
	@echo "Binary: $(shell pwd)/target/debug/backend"

be-dev: ## Khởi động DB rồi chạy server với auto-reload
	cd backend && cargo watch -x run

be-build: ## Build release binary
	cd backend && cargo build --release

be-test: ## Chạy tests (cần DB đang chạy)
	cd backend && cargo test

be-fmt: ## Format code
	cd backend && cargo fmt

be-lint: ## Kiểm tra lỗi với clippy
	cd backend && cargo clippy -- -D warnings

be-clean: ## Xóa build artifacts
	cd backend && cargo clean

fe-setup: ## Cài đặt dependencies
	cd frontend/webapp && npm install

fe-dev: ## Chạy FE
	cd frontend/webapp && npm run dev

# ─── Setup ───────────────────────────────────────────────────────────────────

setup: ## Setup lần đầu: tạo dev.env + khởi động DB + migrate
	@[ -f dev.env ] || cp dev.env.example dev.env && echo "✓ dev.env created from example"
	$(MAKE) db-up
	$(MAKE) db-migrate
	@echo "✓ Setup complete — run 'make dev' to start"
