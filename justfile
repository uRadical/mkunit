# Default recipe
default: check

# Run all checks
check: fmt-check lint test audit

# Format check
fmt-check:
    cargo fmt --check

# Format code
fmt:
    cargo fmt

# Run clippy
lint:
    cargo clippy -- -D warnings

# Run tests
test:
    cargo test

# Run security audit
audit:
    cargo audit

# Run cargo-deny
deny:
    cargo deny check

# Check for outdated dependencies
outdated:
    cargo outdated

# Check for unused dependencies
machete:
    cargo machete

# Build release
build:
    cargo build --release

# Install locally
install:
    cargo install --path .

# Clean build artifacts
clean:
    cargo clean

# Run all fixes
fix: fmt
    cargo clippy --fix --allow-dirty

# Generate shell completions
completions:
    mkdir -p completions
    cargo run -- completions bash > completions/mkunit.bash
    cargo run -- completions zsh > completions/_mkunit
    cargo run -- completions fish > completions/mkunit.fish

# Generate man page
man:
    GENERATE_MAN=1 cargo build --release
    cp target/release/build/mkunit-*/out/man/mkunit.1 .

# Full CI check
ci: fmt-check lint test audit deny

# Watch for changes and run tests
watch:
    cargo watch -x test

# Run with verbose output
run *ARGS:
    cargo run -- -v {{ARGS}}
