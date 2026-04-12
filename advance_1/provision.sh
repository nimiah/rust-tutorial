sudo apt-get update -qq
sudo apt-get install -y \
  vim nano curl wget git zsh \
  build-essential pkg-config \
  ca-certificates unzip libssl-dev \
  lldb gdb \
  postgresql-client

# mise list installed
mise current

cargo install cargo-watch
cargo install sqlx-cli --no-default-features --features postgres

# list system installed tools(lldb postgresql-client)??
dpkg -l | grep -E "postgresql-client|lldb"

# rustup list installed
rustup component list | grep installed

# cargo list installed
cargo install --list
