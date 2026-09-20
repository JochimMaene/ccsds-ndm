# Justfile for ccsds-ndm

# Default to listing available recipes
default:
    @just --list

# --- Variables --------------------------------------------------------------

rust_dir := "ccsds-ndm"
rust_manifest := rust_dir + "/Cargo.toml"
python_dir := "bindings/python"
python_manifest := python_dir + "/Cargo.toml"

# Interpreter used to build the stub generator; must ship a shared libpython.
stub_python := env_var_or_default("PYO3_PYTHON", "python3")

# --- Setup ------------------------------------------------------------------

# Set up the Python development environment
[private]
setup-dev:
    cd {{python_dir}} && uv sync --dev

# Set up the Python documentation environment
[private]
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

# Generate Python type stubs (.pyi) with pyo3-stub-gen.
#
# The generator is an ordinary binary, so it links libpython: point `stub_python`
# at an interpreter that ships a shared library (a uv-managed CPython reports a
# LIBDIR that does not exist). `abi3-py310` only sets the ABI floor, so any
# supported version works.
[private]
stubs:
    #!/usr/bin/env bash
    set -euo pipefail
    libdir="$({{stub_python}} -c 'import sysconfig; print(sysconfig.get_config_var("LIBDIR"))')"
    if [ ! -d "$libdir" ]; then
        echo "{{stub_python}} reports LIBDIR=$libdir, which does not exist." >&2
        echo "Set stub_python to an interpreter with a shared libpython." >&2
        exit 1
    fi
    PYO3_PYTHON="{{stub_python}}" LD_LIBRARY_PATH="$libdir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
        cargo run --quiet --manifest-path {{python_manifest}} --bin stub_gen
    uv run --project {{python_dir}} ruff format {{python_dir}}/ccsds_ndm/__init__.pyi

# Check the committed stubs match what the generator produces.
# Generates into a temporary directory so the committed file is never rewritten.
[private]
stubs-check:
    #!/usr/bin/env bash
    set -euo pipefail
    libdir="$({{stub_python}} -c 'import sysconfig; print(sysconfig.get_config_var("LIBDIR"))')"
    out="$(mktemp -d)"
    trap 'rm -rf "$out"' EXIT
    mkdir -p "$out/ccsds_ndm"
    PYO3_PYTHON="{{stub_python}}" LD_LIBRARY_PATH="$libdir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
        cargo run --quiet --manifest-path {{python_manifest}} --bin stub_gen -- "$out"
    uv run --project {{python_dir}} ruff format --config {{python_dir}}/pyproject.toml "$out/ccsds_ndm/__init__.pyi"
    diff -u {{python_dir}}/ccsds_ndm/__init__.pyi "$out/ccsds_ndm/__init__.pyi"

# Type-check the generated stubs. mypy caught malformed override annotations that
# aggregate diffing did not, so this guards the generator's output.
[private]
typecheck:
    cd {{python_dir}} && uv run mypy --strict ccsds_ndm/__init__.pyi

# Audit Python bindings against Rust core structs
[private]
audit:
    cd {{python_dir}} && uv run python audit_bindings.py

# --- Linting and Formatting -------------------------------------------------

# Format the Rust code
[private]
fmt-rust:
    cargo fmt --manifest-path {{rust_manifest}}
    cargo fmt --manifest-path {{python_manifest}}

# Check the formatting of the Rust code
[private]
fmt-rust-check:
    cargo fmt --manifest-path {{rust_manifest}} -- --check
    cargo fmt --manifest-path {{python_manifest}} -- --check

# Lint the Rust code
[private]
lint-rust:
    cargo clippy --manifest-path {{rust_manifest}} --all-targets --all-features -- -D warnings
    cargo clippy --manifest-path {{python_manifest}} -- -D warnings

# Format the Python code
[private]
fmt-python:
    cd {{python_dir}} && uv run ruff format .

# Lint the Python code
[private]
lint-python:
    cd {{python_dir}} && uv run ruff check .

# Format both Rust and Python code
fmt: fmt-rust fmt-python

# Lint both Rust and Python code
lint: lint-rust lint-python

# --- Testing ----------------------------------------------------------------

# Run the Rust tests
[private]
test-rust:
    cargo test --manifest-path {{rust_manifest}} --all-features

# Run the Python tests
[private]
test-python:
    cd {{python_dir}} && uv run pytest

# Run both Rust and Python tests
test: test-rust test-python

# Complete verification path: full quality checks plus packaged-artifact gates.
verify: check package-rust package-python

# Run all quality checks
check: lint audit stubs-check typecheck test docs

# --- Benchmarking -----------------------------------------------------------

# Run Rust benchmarks
bench:
    cargo bench --manifest-path {{rust_manifest}}

# --- CodSpeed ---------------------------------------------------------------

# Build benchmarks for CodSpeed
[private]
bench-build:
    cd {{rust_dir}} && cargo codspeed build

# Run benchmarks for CodSpeed
[private]
bench-run:
    cd {{rust_dir}} && cargo codspeed run

# Run the Python binding benchmark; pass --codspeed under the CodSpeed action
bench-python *args:
    cd {{python_dir}} && uv run pytest benchmarks {{args}}

# --- Build and Documentation ------------------------------------------------

# Build the Python bindings
build:
    cd {{python_dir}} && uv run --with maturin maturin build --release --strip --out ../../dist

# Build and import the wheel in an isolated environment
package-python: build
    bash scripts/verify-python-package.sh

# Build and verify the publishable Rust crate artifact (CI supplies a clean checkout)
package-rust:
    bash scripts/verify-rust-package.sh

# Build the documentation
docs:
    uv run sphinx-build -E -W -b html docs docs/_build/html

# Serve the documentation locally
docs-serve:
    uv run sphinx-autobuild docs docs/_build/html

# Run fuzz testing
fuzz-all duration="30":
    cd {{rust_dir}} && cargo +nightly fuzz run fuzz_from_str -- -max_total_time={{duration}}
    cd {{rust_dir}} && cargo +nightly fuzz run fuzz_kvn -- -max_total_time={{duration}}
    cd {{rust_dir}} && cargo +nightly fuzz run fuzz_xml -- -max_total_time={{duration}}

# --- Clean ------------------------------------------------------------------

# Remove build, documentation, and profiling artifacts
clean:
    rm -rf target {{rust_dir}}/target {{rust_dir}}/fuzz/target {{python_dir}}/target dist docs/_build
    rm -f perf.data perf.data.old {{rust_dir}}/perf.data {{rust_dir}}/perf.data.old
