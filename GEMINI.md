# CCSDS NDM

Rust library with Python bindings for parsing and generation of CCSDS Navigation Data Messages (NDM).

## Project Structure

- `ccsds-ndm/`: **Core Rust Crate**. Implements parsing, logic, and data structures.
    - *Tools*: `cargo`
- `bindings/python/`: **Python Bindings**. Exposes the Rust library to Python via PyO3.
    - *Tools*: `maturin` (build backend), `uv` (dependency management).
- `docs/`: **Documentation**. Sphinx documentation for the project.
    - *Tools*: `sphinx` (via `uv`).

## Development Workflows

### 1. Rust Core (`ccsds-ndm/`)
To work on the core library logic (parsing, data structures):

```bash
cd ccsds-ndm
cargo test      # Run unit tests
cargo check     # Quick syntax/type check
cargo build     # Build the library
cargo bench     # Run benchmarks
```

### 2. Python Bindings (`bindings/python/`)
To work on the Python API or integration:

**Setup environment:**
```bash
cd bindings/python
uv sync         # Install dependencies
```

**Build & Test:**
The most efficient way to develop is using `maturin develop` which compiles the Rust crate and installs it directly into the virtual environment.

```bash
# In bindings/python/
uv run maturin develop    # Build and install in editable/dev mode
uv run pytest             # Run Python tests
```

**Release Build:**
```bash
uv run maturin build --release
```

### 3. Documentation (`root`)
To build and preview the documentation:

```bash
# From project root
uv run sphinx-build docs docs/_build
```
Open `docs/_build/index.html` in your browser.

## Git Usage & Version Control

- **Branching**: Use feature branches (e.g., `feat/add-cdm-parsing`, `fix/parsing-error`).
- **Commits**: Follow [Conventional Commits](https://www.conventionalcommits.org/):
    - `feat: ...` for new features
    - `fix: ...` for bug fixes
    - `docs: ...` for documentation changes
    - `chore: ...` for maintenance (dependencies, config)

DO use git commands for debugging and information gathering:
git status - Check current state
git diff - Compare changes
git log - View commit history
git diff master - Compare to master branch
git show <commit> - View specific commits
DO NOT create git commits - the user will review the code and commit it themselves

## Tools Summary
- **Rust**: `cargo` (standard)
- **Python**: `uv` (package manager), `maturin` (build backend for Rust bindings)
