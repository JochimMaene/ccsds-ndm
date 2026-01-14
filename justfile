# Justfile for ccsds-ndm

default: test

# --- Setup ------------------------------------------------------------------

# Set up the Python development environment
setup-dev:
    uv sync --dev -p bindings/python/pyproject.toml

# Set up the Python documentation environment
setup-docs:
    uv sync -p pyproject.toml

# Set up both the development and documentation environments
setup: setup-dev setup-docs

# --- Linting and Formatting -------------------------------------------------

# Format the Rust code
fmt-rust:
    cargo fmt --manifest-path ccsds-ndm/Cargo.toml

# Lint the Rust code
lint-rust:
    cargo clippy --manifest-path ccsds-ndm/Cargo.toml -- -D warnings

# Format the Python code
fmt-python:
    cd bindings/python && uv run ruff format .

# Lint the Python code
lint-python:
    cd bindings/python && uv run ruff check .

# Format both Rust and Python code
fmt: fmt-rust fmt-python

# Lint both Rust and Python code
lint: lint-rust lint-python

# --- Testing ----------------------------------------------------------------

# Run the Rust tests
test-rust:
    cargo test --manifest-path ccsds-ndm/Cargo.toml --all-features

# Run the Python tests
test-python:
    cd bindings/python && uv run pytest

# Run both Rust and Python tests
test: test-rust test-python

# --- Build and Documentation ------------------------------------------------

# Build the Python bindings
build:
    cd bindings/python && uv run maturin build --release --out ../../dist

# Build the documentation
docs-build:
    uv run sphinx-build -b html docs docs/_build/html

# Serve the documentation locally
docs-serve:
    uv run sphinx-autobuild docs docs/_build/html

# --- Clean ------------------------------------------------------------------

# Remove build artifacts
clean:
    cargo clean --manifest-path ccsds-ndm/Cargo.toml
    rm -rf dist
    rm -rf docs/_build
