# Documentation

User-facing documentation is built with Sphinx:

```console
uv sync
uv run sphinx-build -b html docs docs/_build/html
```

The main project documents are:

- [`project-goal.md`](project-goal.md) — product direction, scope, and feature bar;
- [`support-matrix.md`](support-matrix.md) — current user-facing support claims;
- [`opm-guide.md`](opm-guide.md) — focused Rust, Python, and CLI workflows.

The [`conformance/`](conformance/) directory contains maintainer evidence connecting CCSDS
requirements to tests and benchmarks. It supports the public matrix but is not required reading for
library users.
