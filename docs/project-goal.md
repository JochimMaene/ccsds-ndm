# Project Goal

`ccsds-ndm` aims to be the best library for working with CCSDS Navigation Data Messages in Rust
and Python.

It focuses on the common tasks users need: parsing messages, validating their contents, converting
between KVN and XML, modifying typed data, and generating valid output.

Users should choose `ccsds-ndm` because it is:

- **Correct.** Supported messages follow the applicable CCSDS publications and schemas. The library
  rejects ambiguous or invalid data instead of guessing, never silently loses information, and
  validates messages before generating output.
- **Easy to use.** Installation is straightforward, common workflows require little code, and
  errors explain what is wrong and where.
- **Fast and predictable.** Typical messages are processed efficiently, while large messages and
  malformed inputs have measured and bounded resource behavior where scale requires it.
- **Dependable.** Supported behavior is documented, tested, and released consistently.

## Scope

The Rust core is the single implementation of parsing, validation, conversion, and generation
behavior. The Python bindings and command-line interface reuse that behavior rather than developing
independent interpretations of the standards.

The project intends to support the CCSDS NDM family in KVN and XML. Work is prioritized by common
user workflows, correctness risk, and practical value. Complete, well-tested support for important
message types is more valuable than broad but unreliable coverage.

The project handles NDM representation and exchange. Orbit propagation, frame transformation,
conjunction analysis, attitude dynamics, and other astrodynamics computations are outside its
scope.

## Current Capabilities

This document describes the direction of the project, not the capabilities of a particular
release. The [support and conformance matrix](support-matrix.md) is the authoritative statement of
what is currently supported and verified.

## Guiding Principles

1. **CCSDS is authoritative.** Official publications, corrigenda, and schemas define correct
   behavior.
2. **No silent data loss.** Unsupported, ambiguous, or invalid data must not be silently discarded
   or reinterpreted.
3. **The safe path is the easy path.** Correct parsing and generation should be the default and
   should not require unnecessary ceremony.
4. **Generation validates first.** Public writers emit valid output or return an actionable error.
5. **Performance is measured.** Optimization follows representative workloads and must not weaken
   correctness or clarity.
6. **Complexity must earn its place.** Features, abstractions, compatibility modes, and additional
   integrations are added only when they solve a demonstrated practical need.
