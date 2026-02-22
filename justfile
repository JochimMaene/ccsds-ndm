# Justfile for ccsds-ndm

# Default to listing available recipes
default:
    @just --list

# --- Variables --------------------------------------------------------------

rust_dir := "ccsds-ndm"
rust_manifest := rust_dir + "/Cargo.toml"
python_dir := "bindings/python"

# --- Setup ------------------------------------------------------------------

# Set up the Python development environment
setup-dev:
    cd {{python_dir}} && uv sync --dev

# Set up the Python documentation environment
setup-docs:
    uv sync

# Set up both the development and documentation environments
setup: setup-dev setup-docs

# --- Pre-commit -------------------------------------------------------------

# Install pre-commit hooks using prek
pre-commit-install:
    prek install

# Run prek on all files
prek:
    prek run --all-files

# --- Development ------------------------------------------------------------

# Install the Python bindings in development mode
dev:
    cd {{python_dir}} && uv run --with maturin maturin develop

# Generate Python type stubs (.pyi)
stubs:
    cd {{python_dir}} && uv run python stubs.py

# Check if Python type stubs are up to date
stubs-check:
    cd {{python_dir}} && uv run python stubs.py --check

# Sync docstrings from Rust to Python
sync-docs:
    cd {{python_dir}} && uv run python sync_docstrings.py

# Check if docstrings are in sync
sync-docs-check:
    cd {{python_dir}} && uv run python sync_docstrings.py --check

# Audit Python bindings against Rust core structs
audit:
    cd {{python_dir}} && uv run python audit_bindings.py

# Audit Python bindings against Rust core structs (strict mode)
audit-strict:
    cd {{python_dir}} && uv run python audit_bindings.py --strict

# --- Linting and Formatting -------------------------------------------------

# Format the Rust code
fmt-rust:
    cargo fmt --manifest-path {{rust_manifest}}

# Check the formatting of the Rust code
fmt-rust-check:
    cargo fmt --manifest-path {{rust_manifest}} -- --check

# Lint the Rust code
lint-rust:
    cargo clippy --manifest-path {{rust_manifest}} -- -D warnings

# Format the Python code
fmt-python:
    cd {{python_dir}} && uv run ruff format .

# Lint the Python code
lint-python:
    cd {{python_dir}} && uv run ruff check .

# Format both Rust and Python code
fmt: fmt-rust fmt-python

# Lint both Rust and Python code
lint: lint-rust lint-python

# --- Testing ----------------------------------------------------------------

# Run the Rust tests
test-rust:
    cargo test --manifest-path {{rust_manifest}} --all-features

# Run the Python tests
test-python:
    cd {{python_dir}} && uv run pytest

# Run both Rust and Python tests
test: test-rust test-python

# Run all quality checks (lint, audit, stubs-check, sync-docs-check, test)
check: lint audit-strict stubs-check sync-docs-check test

# Backward-compatible CI alias
check-ci: check

# --- Benchmarking -----------------------------------------------------------

# Run Rust benchmarks
bench:
    cargo bench --manifest-path {{rust_manifest}}

# --- Coverage ---------------------------------------------------------------

# Generate code coverage report
coverage:
    cargo llvm-cov --manifest-path {{rust_manifest}} --all-features --workspace --codecov --output-path codecov.json

# --- CodSpeed ---------------------------------------------------------------

# Build benchmarks for CodSpeed
bench-build:
    cd {{rust_dir}} && cargo codspeed build

# Run benchmarks for CodSpeed
bench-run:
    cd {{rust_dir}} && cargo codspeed run

# --- Build and Documentation ------------------------------------------------

# Build the Python bindings
build:
    cd {{python_dir}} && uv run maturin build --release --out ../../dist

# Build the documentation
docs:
    uv run sphinx-build -b html docs docs/_build/html

# Serve the documentation locally
docs-serve:
    uv run sphinx-autobuild docs docs/_build/html

# Run fuzz testing
fuzz-all duration="30":
    cd {{rust_dir}} && cargo +nightly fuzz run fuzz_from_str -- -max_total_time={{duration}}
    cd {{rust_dir}} && cargo +nightly fuzz run fuzz_kvn -- -max_total_time={{duration}}
    cd {{rust_dir}} && cargo +nightly fuzz run fuzz_xml -- -max_total_time={{duration}}

# --- Clean ------------------------------------------------------------------

# Remove build artifacts
clean:
    cargo clean --manifest-path {{rust_manifest}}
    rm -rf dist
    rm -rf docs/_build
