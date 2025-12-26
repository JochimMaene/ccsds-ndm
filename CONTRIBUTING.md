# Contributing to CCSDS NDM

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to the project.

## Development Setup

### Prerequisites

- **Rust** (stable, 1.70+): https://rustup.rs/
- **Python** (3.9+): https://www.python.org/
- **uv** (Python package manager): https://docs.astral.sh/uv/

### Clone the Repository

```bash
git clone https://github.com/JochimMaene/ccsds-ndm.git
cd ccsds-ndm
```

### Rust Development

The core library is in `ccsds-ndm/`:

```bash
cd ccsds-ndm

# Run tests
cargo test

# Run lints
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt

# Run benchmarks
cargo bench
```

### Python Bindings Development

The Python bindings are in `bindings/python/`:

```bash
cd bindings/python

# Install dependencies and set up virtual environment
uv sync

# Build and install the extension in development mode
uv run maturin develop

# Run tests
uv run pytest

# Run specific test file
uv run pytest tests/test_oem.py -v

# Build release wheel
uv run maturin build --release
```

### Documentation

Documentation is built with Sphinx from the `docs/` directory:

```bash
# From project root
uv sync
uv run sphinx-build docs docs/_build

# Open docs/_build/index.html in your browser
```

## Project Structure

```
ccsds-ndm/
├── ccsds-ndm/           # Core Rust library
│   ├── src/
│   │   ├── lib.rs       # Public API
│   │   ├── messages/    # Message type implementations
│   │   ├── kvn/         # KVN parser and serializer
│   │   └── ...
│   └── tests/           # Integration tests
├── bindings/python/     # Python bindings (PyO3)
│   ├── src/             # Rust code for bindings
│   ├── ccsds_ndm/       # Python type stubs (.pyi)
│   └── tests/           # Python tests
├── data/                # Test data files
│   ├── kvn/             # Sample KVN files
│   └── xml/             # Sample XML files
└── docs/                # Sphinx documentation
```

## Making Changes

### Code Style

- **Rust**: Follow standard Rust conventions. Run `cargo fmt` before committing.
- **Python**: Follow PEP 8. Type hints are required for public APIs.
- **Documentation**: Use clear, concise language. Include code examples where helpful.

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add support for ACM message type
fix: correct epoch parsing for negative years
docs: update installation instructions
test: add roundtrip tests for TDM
chore: update dependencies
```

### Pull Request Process

1. **Fork** the repository and create a feature branch from `main`
2. **Make your changes** with clear, focused commits
3. **Add tests** for new functionality
4. **Update documentation** if needed
5. **Run the test suite** locally:
   ```bash
   # Rust
   cargo test
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt -- --check
   
   # Python
   cd bindings/python
   uv run maturin develop
   uv run pytest
   ```
6. **Open a Pull Request** with a clear description of the changes

### Adding a New Message Type

When adding support for a new CCSDS message type:

1. **Rust implementation** (`ccsds-ndm/src/messages/`):
   - Create the message module (e.g., `aem.rs`)
   - Define structs matching the XSD schema
   - Implement the `Ndm` trait (`to_kvn`, `from_kvn`, `to_xml`, `from_xml`)
   - Add to `messages/mod.rs` and `lib.rs`

2. **Python bindings** (`bindings/python/src/`):
   - Create wrapper structs with `#[pyclass]`
   - Expose properties with `#[getter]`/`#[setter]`
   - Add `from_str`, `from_file`, `to_kvn`, `to_xml` methods
   - Register in `lib.rs`

3. **Type stubs** (`bindings/python/ccsds_ndm/`):
   - Add `.pyi` file with type annotations

4. **Tests**:
   - Add sample files to `data/kvn/` and `data/xml/`
   - Add Rust tests in the message module
   - Add Python tests in `bindings/python/tests/`

5. **Documentation**:
   - Add API documentation in `docs/api/`
   - Update the README if needed

## Test Data

Sample KVN and XML files are in `data/`. These are based on examples from CCSDS standards documents. When adding test data:

- Use realistic but non-sensitive data
- Include edge cases (optional fields, multiple segments, etc.)
- Reference the CCSDS document section if applicable
