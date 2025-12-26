## Building the Documentation Locally

This project uses **Sphinx** for documentation, and all required dependencies are listed in `pyproject.toml`.
We use **uv** to manage and run everything.

---

## 1. Install dependencies

Install all documentation dependencies by running:

```
uv sync
```

This creates (or updates) a `.venv/` environment and installs Sphinx plus any Sphinx extensions.


## 2. Build the HTML documentation

Run Sphinx directly through uv:

```
uv run sphinx-build -b html docs docs/_build/html
```

The generated HTML files will be available under:

```
docs/_build/html/
```


## 3. Live-reload documentation (autobuild)

If you have **sphinx-autobuild** installed (declared in your `pyproject.toml`), you can start a live-reloading server:

```
uv run sphinx-autobuild docs docs/_build/html
```

## 4. Clean previous builds

To remove all generated documentation output:

```
rm -rf docs/_build
```