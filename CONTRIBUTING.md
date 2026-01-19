# Contributing to CCSDS NDM

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to the project.

## Development Setup

### Prerequisites

- **Rust** (stable, 1.70+): https://rustup.rs/
- **Python** (3.9+): https://www.python.org/
- **uv** (Python package manager): https://docs.astral.sh/uv/
- **prek** (Pre-commit hook runner): `pip install prek`

### Using `just` for Common Tasks

This project uses [`just`](https://github.com/casey/just) as a command runner. After installing `just` (`cargo install just`), you can run `just` from the project root to see all available recipes.

*   `just setup`: Installs all necessary Python dependencies for development and documentation.
*   `just pre-commit-install`: Installs the git hooks using `prek`.
*   `just dev`: Installs the Python bindings in development mode (`maturin develop`).
*   `just test`: Runs the full test suite for both Rust and Python.
*   `just check`: Runs all quality checks (linting, audit, stub check, sync check, license, and tests).
*   `just sync-docs`: Synchronizes docstrings from Rust core to Python bindings.
*   `just stubs`: Generates Python type stubs (`.pyi` files).
*   `just lint` / `just fmt`: Lints and formats both Rust and Python code.
*   `just docs`: Builds the documentation.
*   `just docs-serve`: Builds and serves the documentation locally with live-reload.

### Clone the Repository

```bash
git clone https://github.com/JochimMaene/ccsds-ndm.git
cd ccsds-ndm
just setup
just pre-commit-install
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
│   ├── ccsds_ndm/       # Python package and type stubs (.pyi)
│   └── tests/           # Python tests
├── data/                # Test data files
│   ├── kvn/             # Sample KVN files
│   └── xml/             # Sample XML files
└── docs/                # Sphinx documentation
```

## Making Changes

### Code Style

- **Rust**: Follow standard Rust conventions. Run `just fmt-rust` before committing.
- **Python**: Follow PEP 8. Documentation and type hints are automatically managed via `just sync-docs` and `just stubs`.
- **Documentation**: Use clear, concise language. Documentation in Rust source (`///`) is automatically synced to Python.

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

1. **Fork** the repository and create a feature branch from `main`.
2. **Make your changes** with clear, focused commits.
3. **Sync bindings**: If you modified Rust structures exposed to Python, run `just sync-docs` and `just stubs`.
4. **Add tests** for new functionality.
5. **Run the full check suite** locally:
   ```bash
   just check
   ```
6. **Open a Pull Request** with a clear description of the changes.

### Adding a New Message Type

When adding support for a new CCSDS message type:

1. **Rust implementation** (`ccsds-ndm/src/messages/`):
   - Create the message module (e.g., `aem.rs`).
   - Define structs matching the XSD schema and provide thorough docstrings (`///`).
   - Implement the `Ndm` trait (`to_kvn`, `from_kvn`, `to_xml`, `from_xml`).
   - Add to `messages/mod.rs` and `lib.rs`.

2. **Python bindings** (`bindings/python/src/`):
   - Create wrapper structs with `#[pyclass]`.
   - Expose properties with `#[getter]`/`#[setter]`.
   - Add `from_str`, `from_file`, `to_kvn`, `to_xml` methods.
   - Register the class in `bindings/python/src/lib.rs`.

3. **Synchronize**:
   - Run `just sync-docs` to copy docstrings from Rust to the Python binding source.
   - Run `just stubs` to generate the updated `__init__.pyi` file.
   - Run `just audit` to ensure all Rust fields are correctly exposed in Python.

4. **Tests & Data**:
   - Add sample files to `data/kvn/` and `data/xml/`.
   - Add Rust tests in the message module.
   - Add Python tests in `bindings/python/tests/`.

## Test Data

Sample KVN and XML files are in `data/`. These are based on examples from CCSDS standards documents. When adding test data:

- Use realistic but non-sensitive data.
- Include edge cases (optional fields, multiple segments, etc.).
- Reference the CCSDS document section if applicable.