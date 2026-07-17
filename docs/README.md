## Building the Documentation Locally

This project uses **Sphinx** for documentation, and all required dependencies are listed in `pyproject.toml`.
We use **uv** to manage and run everything.

The project-direction documents are maintained separately in Markdown:

- `project-goal.md` — concise north star and product positioning;
- `conformance-policy.md` — behavioral contract and capability quality bar;
- `support-matrix.md` — authoritative capability statuses and their evidence;
- `reference-status.md` — adoption targets and the scorecard for earning reference status;
- `epoch-handling-plan.md` — measured architecture and incremental epoch migration plan;
- `conformance/time-ordering-contract.md` — normative ordering/duplicate rules and comparator gates;
- `conformance/epoch-field-inventory-oem-ocm.md` — XSD/book inventory for the first epoch slice;
- `conformance/epoch-field-inventory-acm-ocm.md` — ACM reference-epoch migration and remaining OCM context audit;
- `rust-core-review.md` — dated implementation review and remediation findings.

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
