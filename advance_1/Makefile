# Biến số để dễ quản lý
DOCKER_COMPOSE = docker-compose -f docker/docker-compose.yaml
DATABASE_URL = $(shell grep DATABASE_URL backend/.env | cut -d '=' -f2)
PGDATA ?= $(shell pwd)/.pgdata

.PHONY: help shell db-up db-down migrate-add migrate-run prepare check run test clean

default: help

help: ## Hiển thị các lệnh hiện có
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

shell: ## Vào môi trường phát triển (Nix Develop)
	nix develop

## --- Hạ tầng (Local Postgres) ---

cluster-create: ## Khởi tạo Cluster(.pgdata)
	initdb -D .pgdata --auth=trust

cluster-config: ## Tùy chỉnh config
	@echo "unix_socket_directories = '$(shell pwd)/.pgdata'" >> .pgdata/postgresql.conf

pg-start: ## Bật Postgres local
	pg_ctl -D "$(PGDATA)" \
		  -l "$(PGDATA)/postgres.log" \
		  -o "-c listen_addresses='localhost' -p 5432" \
		  start

pg-stop: ## Tắt Postgres local
	@pg_ctl -D "$(PGDATA)" stop

pg-status: ## Xem trạng thái Postgres
	@pg_ctl -D "$(PGDATA)" status

pg-isready: ## Xem Postgres có sẵn sàng cho kết nối không
	@pg_isready -h localhost -p 5432

role-create: ## Tạo role admin
	psql -h $(PGDATA) -d postgres -c "CREATE ROLE admin WITH LOGIN SUPERUSER PASSWORD 'admin123';"

db-create: ## Tạo database mydatabase
	psql -h $(PGDATA) -d postgres -c "CREATE DATABASE mydatabase OWNER admin;"

db-drop: ## Xóa database mydatabase
	psql -h $(PGDATA) -d postgres -c "DROP DATABASE mydatabase;"

schema-init: ## Import schema từ docs/initial_db.sql
	psql -h $(PGDATA) -d mydatabase -U admin < backend/docs/initial_db.sql

## --- Hạ tầng (Docker/Podman Postgres) ---

db-up: ## Khởi động Postgres Container
	$(DOCKER_COMPOSE) up -d
	@echo "Đang chờ Postgres sẵn sàng..."
	@sleep 3

db-down: ## Dừng và xóa Postgres Container
	$(DOCKER_COMPOSE) down

## --- Database (SQLx) ---

db-reset: ## Xóa sạch DB và chạy lại migration từ đầu (Cẩn thận!)
	# Bắt buộc cd vào backend/ để có .env(DATABASE_URL) moi drop & setup database được
	cd backend && sqlx database drop -y
	cd backend && sqlx database setup

migrate-run: ## Chạy các file migration mới
	cd backend && sqlx migrate run

migrate-info: ## Kiểm tra trạng thái migration
	cd backend && sqlx migrate info

## --- Phát triển Rust ---

prepare: ## Tạo sqlx offline cache cho macro (Cần khi build CI)
	cd backend && cargo sqlx prepare

check: ## Kiểm tra lỗi cú pháp và Type-safe (Bao gồm SQL check)
	cd backend && cargo check

build:
	cd backend && cargo clean && cargo build

watch: ## Chạy server với chế độ auto-reload
	cd backend && cargo watch -x run

test: ## Chạy toàn bộ test suite
	cd backend && cargo test

clean: ## Dọn dẹp thư mục target
	cd backend && cargo clean
