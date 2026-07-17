# Earning Reference Status

`ccsds-ndm` may state its ambition from the first release, but reference status is earned through evidence, independent use, and dependable stewardship. It is not established by download counts, stars, benchmark wins, broad model coverage, or self-description.

## How the Position Is Earned

- **Trustworthy by evidence.** Every advertised capability links to normative requirements and executable tests. Release evidence is public and reproducible.
- **Easy to adopt.** Supported Python installations provide native wheels, while Rust and CLI installations use documented ecosystem-standard paths. Common workflows and error recovery do not require prior mastery of the complete CCSDS object model.
- **Interoperable in practice.** Releases are tested with official examples, contributed operational messages, and independent implementations. Disagreements are resolved against the applicable CCSDS publication rather than majority behavior.
- **Dependable over time.** APIs, diagnostic codes, editions, deprecations, security handling, and release compatibility follow published policies with clear migrations.
- **Fast and resource-predictable.** Public benchmarks cover small-message latency, large history products, streaming throughput, allocations, peak memory, and language-boundary costs.
- **Useful across environments.** Rust and Python are first-class library surfaces. The standalone CLI supports validation, inspection, conversion, and canonical generation through files or standard input/output with stable exit codes and machine-readable diagnostics.
- **Sustainable.** Decisions, contribution paths, licensing, release responsibilities, succession, and governance allow organizations beyond the original author to adopt and help maintain the project.

## Adoption and Usability Targets

Reference-quality software must be attractive to use as well as correct. The project therefore measures whether:

- a new user can install an advertised artifact and validate a representative message from a clean environment in under ten minutes by following the primary quickstart;
- common validation and conversion workflows have copyable Rust, Python, and CLI examples wherever those surfaces are advertised;
- diagnostics provide enough structured information to drive CI, editors, batch processing, and support investigations without parsing prose;
- evaluation and migration guidance addresses the Python distribution-name collision and interoperability with other established implementations;
- operational users can contribute sanitized fixtures, deviations, and interoperability reports through a documented process; and
- independent contributors can understand the conformance evidence and complete meaningful changes without private project knowledge.

These targets are evaluated with reproducible installation checks, documentation tests, external feedback, and observed adoption rather than assertion.

## Minimum Reference-Status Gates

The project does not claim that it has achieved reference status until a public scorecard demonstrates all of the following:

- every advertised capability appears with status `verified` in the [support and conformance matrix](support-matrix.md), with normative and executable evidence;
- the conformance suite can be reproduced independently and used to test another implementation;
- continuous interoperability testing covers at least two independent implementations where equivalent capabilities exist;
- at least three unaffiliated organizations have independently adopted the project in operational or pre-operational workflows, supported by public evidence or independently verifiable attestations;
- at least two people can perform a complete release, including at least one maintainer other than the original author;
- a stable release line has been maintained for at least twelve months under published compatibility, deprecation, migration, release, and security policies;
- supported Python platforms provide installable wheels that require no Rust toolchain;
- the Rust crate and CLI provide documented, reproducible installation paths;
- Rust, Python, and CLI parity is continuously checked for every capability advertised on those surfaces;
- representative performance and resource budgets are published and enforced as release regressions; and
- the primary installation and validation quickstarts pass from clean environments as release gates.

These are minimum gates, not popularity targets. Meeting them permits a reference-status claim; continued evidence and independent use are required to retain it.
