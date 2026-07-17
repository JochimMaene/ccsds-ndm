# Python Release Policy

The `ccsds-ndm-py` distribution imports as `ccsds_ndm` and is pre-1.0. Its API may change between
releases; breaking changes are recorded in release notes when a replacement is not evident.

The extension uses PyO3's CPython 3.9 stable ABI and package metadata permits Python 3.9 and newer.
CI builds the configured wheel matrix, but a built cross-platform wheel is availability evidence,
not proof that every OPM operation ran on that target. The OPM Python conformance surface is tested
end to end on Ubuntu x86_64 with CPython 3.12, including an isolated installation of the produced
wheel. Other package targets may work but are not part of that exact verified environment until
equivalent runtime evidence exists.

Run `just package-python` before release. It builds the wheel, creates an isolated environment,
installs the artifact with its dependencies, and imports the public OPM and conversion APIs. Field
parity, stubs, and synchronized docstrings are enforced separately by `just audit-strict`,
`just stubs-check`, and `just sync-docs-check`.

Defects use the same public-only reporting scope documented in the
[Rust release policy](rust-release-policy.md). A private security process and broader maintained
platform matrix are reference-status goals, not implied by technical OPM capability verification.
