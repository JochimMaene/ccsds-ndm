# Support Matrix

This is the user-facing statement of what `ccsds-ndm` supports today. Parsing is strict by
default: unknown, duplicate, reordered, malformed, or semantically invalid content is rejected
rather than silently discarded.

## Status

- **Verified** means the edition has been checked against its CCSDS requirements, official schema
  where applicable, strictness tests, round-trip or conversion tests, resource tests, and the named
  package surfaces.
- **Available** means the feature is implemented and tested, but its complete message-specific
  evidence has not yet received the same review. It is useful, but not advertised as a complete
  conformance claim.

## Message support

| Message / edition | KVN | XML | Rust | Python | Status |
| --- | --- | --- | --- | --- | --- |
| OPM 3.0 | Parse, write, convert | Parse, write, convert | Yes | Yes | **Verified** |
| OEM 3.0 | Parse, write, convert | Parse, write, convert | Yes | Yes | **Verified** |
| OPM 2.0 | Parse, write, convert | Parse, write, convert | Yes | Yes | Available |
| OEM 2.0 | Parse, write, convert | Parse, write, convert | Yes | Yes | Available |
| OMM 3.0 | Parse, write, convert | Parse, write, convert | Yes | Yes | **Verified** |
| OMM 2.0 | Parse, write, convert | Parse, write, convert | Yes | Yes | Available |
| OCM 3.0 | Parse, write, convert | Parse, write, convert | Yes | Yes | Available |
| CDM 1.0 | Parse, write, convert | Parse, write, convert | Yes | Yes | Available |
| TDM 2.0 | Parse, write, convert | Parse, write, convert | Yes | Yes | Available |
| TDM 1.0 | Parse only | Parse only | Yes | Yes | Available |
| RDM 1.0 | Parse, write, convert | Parse, write, convert | Yes | Yes | Available |
| APM 2.0 | Parse, write, convert | Parse, write, convert | Yes | Yes | Available |
| APM 1.0 | Parse only | Parse only | Yes | Yes | Available |
| AEM 2.0 | Parse, write, convert | Parse, write, convert | Yes | Yes | Available |
| AEM 1.0 | Parse only | Parse only | Yes | Yes | Available |
| ACM 2.0 | Parse, write, convert | Parse, write, convert | Yes | Yes | Available |
| ACM 1.0 | Parse only | Parse only | Yes | Yes | Available |
| Combined NDM | Not defined | Parse and write | Yes | Yes | Available |

Combined NDM is defined only as the XML `ndm` envelope, so KVN parsing and generation fail with
the `unsupported.notation` diagnostic and exchanging several KVN messages together needs
application-level framing such as one message per file; see
[`docs/conformance/combined-ndm.md`](conformance/combined-ndm.md).

OPM, OEM, AEM, APM, ACM, and TDM edition 1.0 remain parse-only. They cannot be written back or
converted to a newer edition; callers must supply a supported current-edition message instead. The
library rejects unsupported output editions instead of relabeling a document. OPM, OEM, and OMM
generation supports explicit 2.0 and 3.0 targets where the typed content can be represented without
loss.

## Evidence

Maintainer-facing evidence lives in [`docs/conformance/`](conformance/). Those inventories link
CCSDS requirements to focused tests, official XSD checks, diagnostics, resource limits, allocation
budgets, and benchmarks. They support this table without making users navigate hundreds of
capability cells.

Run `just check` for the complete quality suite and `just bench` for the benchmark suite.
