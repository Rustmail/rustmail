# Building from Source

This guide covers compiling Rustmail from source code.

---

## Prerequisites

### Rust Toolchain

Install Rust via rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Minimum version: **Rust 1.85+** (Edition 2024)

Verify installation:
```bash
rustc --version
cargo --version
```

### WASM Target (for Panel)

If building the web panel:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### System Dependencies

**Debian/Ubuntu:**
```bash
apt-get install build-essential pkg-config libssl-dev
```

**Fedora:**
```bash
dnf install gcc openssl-devel
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
- Visual Studio Build Tools with C++ workload
- Or use WSL2 with Linux instructions

---

## Clone Repository

```bash
git clone https://github.com/Rustmail/rustmail.git
cd rustmail
```

---

## Build Commands

### Bot Only (Quick)

```bash
cargo build -p rustmail --release
```

Output: `target/release/rustmail`

### Panel Only

```bash
cd rustmail_panel
trunk build --release --dist ../rustmail/static
```

Output: `rustmail/static/`

### Full Build (Panel + Bot)

The panel must be built first and placed in `rustmail/static/` so the bot can embed it:

```bash
# Build panel and output to bot's static folder
cd rustmail_panel
trunk build --release --dist ../rustmail/static
cd ..

# Build bot (embeds the panel)
cargo build -p rustmail --release
```

Output: `target/release/rustmail` (single binary with embedded panel)

---

## Development Build

For faster iteration during development:

```bash
# Debug build (faster compile, slower runtime)
cargo build -p rustmail

# Run directly
cargo run -p rustmail

# With automatic recompilation
cargo install cargo-watch
cargo watch -x "run -p rustmail"
```

### Panel Development

```bash
cd rustmail_panel

# Development server with hot reload
trunk serve

# Opens at http://localhost:8080
```

---

## Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p rustmail

# With output
cargo test -- --nocapture
```

---

## Checking Code

```bash
# Type checking without building
cargo check --workspace

# Linting
cargo clippy --workspace

# Formatting
cargo fmt --all --check
```

---

## Build Optimization

### Custom Release Profile

You can add a custom release profile to your workspace `Cargo.toml` for optimized builds:

```toml
[profile.release]
lto = true
codegen-units = 1
opt-level = 3
```

### Smaller Binary

For reduced binary size, use:

```toml
[profile.release]
strip = true
lto = true
codegen-units = 1
opt-level = "z"
```

Note: These profiles increase compile time but produce smaller/faster binaries.

---

## Cross-Compilation

### Linux to Windows

```bash
rustup target add x86_64-pc-windows-gnu
cargo build -p rustmail --release --target x86_64-pc-windows-gnu
```

### Linux to macOS

Cross-compilation to macOS requires additional setup. Consider using GitHub Actions or a macOS machine.

### Using Cross

For easier cross-compilation:

```bash
cargo install cross

# Build for various targets
cross build -p rustmail --release --target x86_64-unknown-linux-musl
cross build -p rustmail --release --target aarch64-unknown-linux-gnu
```

---

## Docker Build

Build the Docker image locally:

```bash
docker build -t rustmail:local .
```

Multi-platform build:

```bash
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t rustmail:local .
```

---

## Troubleshooting

### OpenSSL Errors

```
error: failed to run custom build command for `openssl-sys`
```

Install OpenSSL development files:
```bash
# Debian/Ubuntu
apt-get install libssl-dev

# Fedora
dnf install openssl-devel

# macOS
brew install openssl
```

### Trunk Not Found

```bash
cargo install trunk
```

Ensure `~/.cargo/bin` is in your PATH.

### WASM Build Errors

```bash
# Ensure target is installed
rustup target add wasm32-unknown-unknown

# Update trunk
cargo install trunk --force
```

### Out of Memory

Large builds may need more memory:

```bash
# Limit parallel jobs
cargo build -j 2 --release
```

### SQLx Compile-Time Verification

SQLx verifies queries at compile time. If you see database errors:

```bash
# Set DATABASE_URL for offline builds
export DATABASE_URL="sqlite:db.sqlite"
cargo build -p rustmail
```

Or use offline mode:
```bash
cargo sqlx prepare --workspace
```

---

## IDE Setup

### RustRover / IntelliJ

The repository includes a pre-configured run configuration in `.run/Run rustmail.run.xml`.

This configuration:
1. Builds the panel with Trunk
2. Outputs to `rustmail/static/`
3. Runs the bot

To use it:
1. Open the project in RustRover
2. Select "Run rustmail" from the run configurations dropdown
3. Click Run

The equivalent command:
```bash
cd rustmail_panel && trunk build --release --dist ../rustmail/static && cd .. && cargo run -p rustmail
```

### VS Code

Recommended extensions:
- rust-analyzer
- CodeLLDB (debugging)
- Even Better TOML

Settings (`.vscode/settings.json`):
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.check.command": "clippy"
}
```

---

## Project Layout Reference

```
rustmail/
├── Cargo.toml              # Workspace manifest
├── Cargo.lock              # Dependency lock file
├── rustmail/               # Main bot crate
│   ├── Cargo.toml
│   ├── src/
│   └── static/             # Panel build output (embedded)
├── rustmail_panel/         # Web panel crate (Yew/WASM)
│   ├── Cargo.toml
│   ├── Trunk.toml
│   ├── index.html
│   └── src/
├── rustmail_types/         # Shared types crate
│   ├── Cargo.toml
│   └── src/
├── rustmail-i18n/          # Internationalization resources
├── migrations/             # SQLite migrations
├── docs/                   # Documentation (mdBook)
├── .run/                   # IDE run configurations
├── config.example.toml     # Example configuration
└── Dockerfile
```
