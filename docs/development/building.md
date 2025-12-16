# Building from Source

This guide covers compiling Rustmail from source code.

---

## Prerequisites

### Rust Toolchain

Install Rust via rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Minimum version: Rust 1.75+

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
trunk build --release
```

Output: `rustmail_panel/dist/`

### Full Build

```bash
# Build bot
cargo build -p rustmail --release

# Build panel
cd rustmail_panel
trunk build --release
cd ..
```

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

### Release Profile

The default release profile in `Cargo.toml`:

```toml
[profile.release]
lto = true
codegen-units = 1
opt-level = 3
```

### Smaller Binary

For reduced binary size:

```toml
[profile.release]
strip = true
lto = true
codegen-units = 1
opt-level = "z"
```

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

### IntelliJ/CLion

Install the Rust plugin. The project should be recognized automatically.

---

## Project Layout Reference

```
rustmail/
├── Cargo.toml              # Workspace manifest
├── Cargo.lock              # Dependency lock file
├── rustmail/               # Main bot crate
│   ├── Cargo.toml
│   └── src/
├── rustmail_panel/         # Panel crate
│   ├── Cargo.toml
│   ├── Trunk.toml
│   ├── index.html
│   └── src/
├── rustmail_types/         # Shared types crate
│   ├── Cargo.toml
│   └── src/
├── migrations/             # Database migrations
├── config.example.toml     # Example configuration
└── Dockerfile
```
