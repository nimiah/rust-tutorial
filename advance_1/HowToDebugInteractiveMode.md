# How To Debug Interactive Mode in VSCode

## Prerequisite:

- rust toolchain: rust-src rust-analyzer
- lldb
- python3

## VSCode with Extension:

- CodeLLDB
- Rust-Analyzer
- (direnv)

## VSCode config:

```json
// settings.json
{
  "lldb.library": "${LLDB_PATH}/lib/liblldb.so",
  ...
}
```

```json
// launch.json
{
  // Use IntelliSense to learn about possible attributes.
  // Hover to view descriptions of existing attributes.
  // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Launch Debug Interactive Mode",
      "type": "lldb",
      "request": "launch",
      "program": "${workspaceRoot}/advance_1/backend/target/debug/backend",
      "cwd": "${workspaceRoot}/advance_1/backend",
      "sourceLanguages": ["rust"], // MUST have to show values of Rust Debug Variables
      // (Optional) depends on personal-env
      "sourceMap": {
        "/data/20_Workspace/10_Active/rust-tutorial": "/home/michael/20_Workspace/10_Active/rust-tutorial"
      },
      "args": [],
      "env": {
        "RUST_BACKTRACE": "1",
        "DATABASE_URL": "postgres://admin:admin123@localhost:5432/mydatabase",
        "SERVER_PORT": "3000",
        "APP_ENV": "development",
        "MAX_DB_CONNECTIONS": "5"
      }
    }
  ]
}
```
