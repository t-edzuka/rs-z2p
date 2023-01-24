dev:
    cargo watch -x check -x test -x run

lint:
    cargo fmt --version
    cargo fmt --all -- --check
    cargo clippy --version
    cargo clippy -- -D warnings
