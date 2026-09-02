# Python Release Policy

The `ccsds-ndm-py` distribution imports as `ccsds_ndm` and is pre-1.0. Its API may change between
releases; breaking changes are recorded in release notes when a replacement is not evident.

The extension uses PyO3's CPython 3.10 stable ABI and package metadata permits Python 3.10 and newer.
CI builds the configured wheel matrix, but a built cross-platform wheel is availability evidence,
not proof that every operation ran on that target. The verified OPM, OEM, and OMM Python surfaces
are tested end to end on Ubuntu x86_64 with CPython 3.10. Release verification also installs the
produced wheel in an isolated environment. Other package targets may work but are not verified
until equivalent runtime evidence exists.

Run `just package-python` before release. It builds the wheel, creates an isolated environment,
installs the artifact with its dependencies, and exercises the public OPM, OEM, OMM, and conversion
APIs in both notations. Field parity, stubs, and synchronized docstrings are enforced separately by
`just audit`, `just stubs-check`, and `just sync-docs-check`.

Defects use the same public-only reporting scope documented in the
[Rust release policy](rust-release-policy.md). A private security process and broader maintained
platform matrix should be added only when adoption and maintainership justify their ongoing cost.
