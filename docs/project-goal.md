# Project Goal

`ccsds-ndm` exists to make CCSDS Navigation Data Message exchange trustworthy, easy to adopt, and operationally predictable.

Our ambition is to become the default open-source implementation for inspecting, parsing, validating, converting, and generating CCSDS NDM data, and a shared conformance test engine for the wider ecosystem.

Users should choose `ccsds-ndm` because the safe path is also the easy path: installation takes minutes, common workflows require little ceremony, diagnostics explain exactly what happened, large messages have predictable resource use, and every advertised capability links to reproducible conformance evidence.

We will earn reference status through demonstrated interoperability, independent operational adoption, dependable releases, transparent governance, and sustained agreement between the Rust, Python, and command-line surfaces. Applicable CCSDS schemas and publications remain authoritative; this project does not claim CCSDS endorsement or replace the standards.

## Status of This Document

This document describes the target product and the position the project intends to earn. It is not a statement that every target capability exists today.

The published [support and conformance matrix](support-matrix.md) is the authoritative statement of current capabilities. A message model, parser, serializer, example, or test in the repository does not by itself establish advertised conformance. Until a capability appears in that matrix with status `verified`, it remains planned, implemented but unverified, experimental, or unsupported as applicable.

The detailed [conformance and product policy](conformance-policy.md) defines the project contract. The [reference-status scorecard](reference-status.md) defines how the project will decide whether it has earned its stated position.

## Who We Serve

- Mission operators and data-exchange engineers who need reliable inspection, validation, conversion, and generation.
- Application and library developers who need stable, idiomatic, high-performance Rust and Python APIs.
- Other NDM implementers who need an independent, reproducible conformance test engine and interoperability reference.

## Why Users Should Choose This Project

- **The safe path is the easy path.** Installation, first validation, conversion, diagnostics, and CI integration work without requiring users to learn the entire CCSDS object model.
- **Claims come with evidence.** Every advertised capability identifies the exact standard issue, corrigendum, notation, operation, public surface, and executable evidence behind it.
- **Failures are actionable.** Diagnostics are precise, stable, machine-readable, and suitable for both people and automation.
- **Real-world inputs are handled honestly.** Strict processing never guesses. Explicit permissive processing applies only documented, deterministic recoveries and reports every one.
- **Scale is predictable.** Large history products, malformed inputs, and adversarial inputs have measured performance and bounded resource behavior.
- **The project is dependable.** Interfaces, releases, compatibility, security handling, governance, and succession are transparent enough for long-lived operational adoption.

## Product Shape

The applicable official CCSDS schemas define notation-specific wire constraints. The applicable
CCSDS publications and corrigenda define semantics and requirements not expressed by those schemas.
Requirement-traceable tests and a support and conformance matrix turn those inputs into executable
evidence in the shared Rust core.

The Rust API, Python API, and standalone CLI expose the same parsing, validation, conversion, generation, and diagnostic decisions through interfaces appropriate to their environments. The Rust core is the behavioral source of truth; public surfaces do not develop independent conformance semantics.

The long-term scope follows the complete CCSDS NDM family as it evolves. Work is prioritized by operational demand, conformance risk, interoperability value, and measured scale rather than by the appearance of broad coverage.

## Guiding Principles

1. **CCSDS is authoritative.** Official schemas decide notation-specific wire constraints; the applicable publications and corrigenda decide semantics not expressed by those schemas. Other implementations are interoperability references.
2. **Correctness and usability reinforce each other.** Prevention of silent data loss is non-negotiable, and correct common workflows should require minimal ceremony.
3. **Support is exact and evidence-based.** Partial, historical, experimental, and surface-specific capabilities are useful when labeled precisely.
4. **No recovery is silent.** Ambiguous input is rejected; deterministic permissive recovery is always reported.
5. **Generation is a conformance boundary.** Public writers validate the complete message for the selected edition before emitting data.
6. **Performance is product behavior.** Claims require reproducible measurements and enforced resource budgets.
7. **Complexity must be earned.** Features, abstractions, language bindings, and compatibility modes require demonstrated user or conformance value.
