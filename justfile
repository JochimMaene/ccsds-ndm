# Justfile for ccsds-ndm

# Default to listing available recipes
default:
    @just --list

# --- Variables --------------------------------------------------------------

rust_dir := "ccsds-ndm"
rust_manifest := rust_dir + "/Cargo.toml"
python_dir := "bindings/python"
python_manifest := python_dir + "/Cargo.toml"

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

# Generate Python type stubs (.pyi)
[private]
stubs:
    cd {{python_dir}} && uv run python stubs.py

# Check if Python type stubs are up to date
[private]
stubs-check:
    cd {{python_dir}} && uv run python stubs.py --check

# Sync docstrings from Rust to Python
[private]
sync-docs:
    cd {{python_dir}} && uv run python sync_docstrings.py

# Check if docstrings are in sync
[private]
sync-docs-check:
    cd {{python_dir}} && uv run python sync_docstrings.py --check

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

# Run the OPM 3.0 Rust XML-generation conformance slice
[private]
conformance-opm-xml:
    cargo test --manifest-path {{rust_manifest}} --test opm_3_xml_generation_conformance
    cargo test --manifest-path {{rust_manifest}} --test opm_epoch_xml_generation
    cargo test --manifest-path {{rust_manifest}} --test opm_keplerian_xml_generation
    cargo test --manifest-path {{rust_manifest}} --test opm_maneuver_duration_units
    cargo test --manifest-path {{rust_manifest}} --test opm_xml_root_envelope
    cargo test --manifest-path {{rust_manifest}} --test opm_xml_writer_failure
    cargo test --manifest-path {{rust_manifest}} --test opm_xml_allocations

# Run the OPM 3.0 Rust KVN-generation conformance slice
[private]
conformance-opm-kvn:
    cargo test --manifest-path {{rust_manifest}} --test opm_3_kvn_generation_conformance
    cargo test --manifest-path {{rust_manifest}} --test opm_kvn_writer_failure
    cargo test --manifest-path {{rust_manifest}} --test opm_kvn_allocations

# Run strict OPM parsing, validation, and conversion evidence
[private]
conformance-opm-parse:
    cargo test --manifest-path {{rust_manifest}} --test opm_strict_kvn_parsing
    cargo test --manifest-path {{rust_manifest}} --test opm_strict_xml_parsing
    cargo test --manifest-path {{rust_manifest}} --test opm_parse_diagnostics
    cargo test --manifest-path {{rust_manifest}} --test opm_parse_limits

[private]
conformance-opm-validation:
    cargo test --manifest-path {{rust_manifest}} --test opm_validation

[private]
conformance-opm-conversion:
    cargo test --manifest-path {{rust_manifest}} --test opm_conversion

[private]
conformance-opm-python:
    cd {{python_dir}} && uv run pytest tests/test_opm.py tests/test_parse_and_generation_options.py

# Run the focused OEM 3.0 Rust parsing, validation, generation, conversion, and resource evidence
[private]
conformance-oem:
    cargo test --manifest-path {{rust_manifest}} --test oem_strict_parsing
    cargo test --manifest-path {{rust_manifest}} --test oem_parse_diagnostics
    cargo test --manifest-path {{rust_manifest}} --test oem_validation
    cargo test --manifest-path {{rust_manifest}} --test oem_generation_conformance
    cargo test --manifest-path {{rust_manifest}} --test oem_conversion
    cargo test --manifest-path {{rust_manifest}} --test oem_kvn_allocations

# Run the focused OMM 3.0 strictness, preservation, and generation evidence
[private]
conformance-omm:
    cargo test --manifest-path {{rust_manifest}} --test omm_conformance
    cargo test --manifest-path {{rust_manifest}} --test fixed_family_allocations

# Run the focused OEM/OMM Python adapter evidence
[private]
conformance-odm-surfaces:
    cd {{python_dir}} && uv run pytest tests/test_verified_odm_surfaces.py tests/test_parse_and_generation_options.py

# Run the focused standalone OCM 3.0 conformance and history-allocation evidence.
[private]
conformance-ocm:
    cargo test --manifest-path {{rust_manifest}} --test ocm_conformance
    cargo test --manifest-path {{rust_manifest}} --test ocm_kvn_allocations

# Run the focused standalone CDM 1.0 conformance evidence.
[private]
conformance-cdm:
    cargo test --manifest-path {{rust_manifest}} --test cdm_conformance

# Run the focused standalone AEM 2.0 conformance and history-allocation evidence.
[private]
conformance-aem:
    cargo test --manifest-path {{rust_manifest}} --test aem_conformance
    cargo test --manifest-path {{rust_manifest}} --test aem_semantic_validation
    cargo test --manifest-path {{rust_manifest}} --test aem_kvn_allocations

# Run the focused standalone ACM 2.0 conformance and history-allocation evidence.
[private]
conformance-acm:
    cargo test --manifest-path {{rust_manifest}} --test acm_conformance
    cargo test --manifest-path {{rust_manifest}} --test acm_kvn_allocations

# Run the focused combined NDM envelope, surface, and allocation evidence.
[private]
conformance-combined:
    cargo test --manifest-path {{rust_manifest}} --test combined_conformance
    cargo test --manifest-path {{rust_manifest}} --test combined_ndm
    cargo test --manifest-path {{rust_manifest}} --test combined_allocations

# Run the focused standalone APM 2.0 conformance evidence.
[private]
conformance-apm:
    cargo test --manifest-path {{rust_manifest}} --test apm_conformance
    cargo test --manifest-path {{rust_manifest}} --test fixed_family_allocations

# Run the focused standalone RDM 1.0 conformance evidence.
[private]
conformance-rdm:
    cargo test --manifest-path {{rust_manifest}} --test rdm_conformance
    cargo test --manifest-path {{rust_manifest}} --test fixed_family_allocations

# Run the focused standalone TDM 2.0 conformance and history-allocation evidence.
[private]
conformance-tdm:
    cargo test --manifest-path {{rust_manifest}} --test tdm_conformance
    cargo test --manifest-path {{rust_manifest}} --test tdm_kvn_allocations

# Reproduce the complete OEM 3.0 Rust technical verification and artifact evidence
[private]
verify-oem:
    just check
    just conformance-oem
    just conformance-odm-surfaces
    cargo check --manifest-path {{rust_manifest}} --all-features --benches
    just docs
    just package-rust
    just package-python

# Reproduce the complete OMM 3.0 technical and packaged-surface evidence
[private]
verify-omm:
    just check
    just conformance-omm
    just conformance-odm-surfaces
    cargo check --manifest-path {{rust_manifest}} --all-features --benches
    just docs
    just package-rust
    just package-python

# Reproduce the complete OPM 3.0 technical verification and artifact evidence
[private]
verify-opm:
    just check
    just conformance-opm-xml
    just conformance-opm-kvn
    just conformance-opm-parse
    just conformance-opm-validation
    just conformance-opm-conversion
    just conformance-opm-python
    cargo check --manifest-path {{rust_manifest}} --all-features --benches
    just docs
    just package-rust
    just package-python

# Run all quality checks
check: lint audit stubs-check sync-docs-check test docs

# --- Benchmarking -----------------------------------------------------------

# Run Rust benchmarks
bench:
    cargo bench --manifest-path {{rust_manifest}}

# Benchmark materialized and streaming OPM XML generation
[private]
bench-opm-xml:
    cargo bench --manifest-path {{rust_manifest}} --bench xml_benches -- xml_generate_opm

# Benchmark materialized and streaming OPM KVN generation
[private]
bench-opm-kvn:
    cargo bench --manifest-path {{rust_manifest}} --bench kvn_benches -- kvn_generate_opm

# Benchmark OPM strict parsing and typed validation
[private]
bench-opm-parse:
    cargo bench --manifest-path {{rust_manifest}} --bench kvn_benches -- kvn_parse_opm
    cargo bench --manifest-path {{rust_manifest}} --bench xml_benches -- xml_parse_opm

[private]
bench-opm-validation:
    cargo bench --manifest-path {{rust_manifest}} --bench kvn_benches -- opm_validate

# Reproduce OEM parsing and generation scaling; timings are informational
[private]
bench-oem:
    cargo bench --manifest-path {{rust_manifest}} --bench kvn_benches -- kvn_scaling
    cargo bench --manifest-path {{rust_manifest}} --bench xml_benches -- xml_scaling

# Reproduce parse/generate workloads for every standalone message and combined XML NDM
[private]
bench-family:
    cargo bench --manifest-path {{rust_manifest}} --bench kvn_benches -- kvn_message_matrix
    cargo bench --manifest-path {{rust_manifest}} --bench xml_benches -- xml_message_matrix

# --- Coverage ---------------------------------------------------------------

# Generate code coverage report
coverage:
    cargo llvm-cov --manifest-path {{rust_manifest}} --all-features --workspace --codecov --output-path codecov.json

# --- CodSpeed ---------------------------------------------------------------

# Build benchmarks for CodSpeed
[private]
bench-build:
    cd {{rust_dir}} && cargo codspeed build

# Run benchmarks for CodSpeed
[private]
bench-run:
    cd {{rust_dir}} && cargo codspeed run

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
