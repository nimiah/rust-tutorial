{
  inputs = {
    nixpkgs.url                         = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url                    = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url                     = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Rust toolchain — đọc từ rust-toolchain.toml nếu có, hoặc định nghĩa ở đây
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"        # cần cho rust-analyzer
            "rust-analyzer"   # LSP
            "clippy"          # linter
            "rustfmt"         # formatter
          ];
          targets = [ "wasm32-unknown-unknown" ];  # bỏ nếu không dùng WASM
        };

      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # ── Rust ──────────────────────────────────────────────
            rustToolchain

            # ── Build dependencies ────────────────────────────────
            pkg-config     # cần để link C libraries
            openssl        # sqlx cần
            openssl.dev

            # ── PostgreSQL ────────────────────────────────────────
            postgresql     # psql CLI + server
            sqlx-cli       # sqlx migrate run, sqlx prepare...
            dbeaver-bin # GUI để quan sát và tương tác database

            # ── Dev tools ─────────────────────────────────────────
            cargo-watch    # auto-reload khi code thay đổi
            cargo-expand   # xem macro expansion
            cargo-audit    # check security vulnerabilities
            lldb           # debug Rust
            python312      # MUST have to show values of Rust Debug Variables
          ];

          # Environment variables
          env = {
            # SQLX_OFFLINE nên để false nếu bạn muốn query! macro check trực tiếp với DB cục bộ này
            SQLX_OFFLINE = "false"; 
            # Hoặc dùng SQLX_OFFLINE=true + sqlx prepare
            # SQLX_OFFLINE = "true";

            # OpenSSL
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            OPENSSL_DIR      = "${pkgs.openssl.dev}";
          };

          shellHook = ''
            echo "🦀 Rust $(rustc --version)"
            echo "📦 Cargo $(cargo --version)"
            echo "🐘 PostgreSQL $(psql --version)"
            echo ""
            echo "Commands:"
            echo "💡 Dùng 'make pg-start' để bật, 'make pg-stop' để tắt DB."
            echo "  cargo run          → start server"
            echo "  cargo watch -x run → auto-reload"
            echo "  sqlx migrate run   → run migrations"
            echo "  cargo test         → run tests"
          '';
        };
      }
    );
}
