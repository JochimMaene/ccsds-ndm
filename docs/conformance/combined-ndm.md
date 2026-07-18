# Combined NDM conformance inventory

Status: `implemented-unverified`.

The normative combined instantiation is the XML `ndm` envelope defined by CCSDS 505.0-B-3,
section 4.11, and `data/xsd/ndmxml-4.0.0-master-4.0.xsd`. Sequential KVN is a library
convenience representation, not an advertised CCSDS envelope.

`just conformance-combined` establishes:

- exact root and constituent attribute contracts, document-root/trailing-content checks, schema
  child ordering, and rejection of unknown or nested message structure;
- ordered typed preservation of the `ndm_g12.xml` and `ndm_g21.xml` constituents, followed by
  official-master-XSD validation of generated XML;
- aggregate input-byte, XML-depth, history-record, and output-byte limits through direct Rust,
  Python, and CLI entry points;
- complete-envelope generation preflight: invalid children or aggregate output limits write zero
  bytes before the normative XML or sequential KVN representation is streamed;
- measured linear KVN streaming allocation budgets from 10 to 1,000 constituents (at most ten
  temporary allocations and 128 temporary bytes per additional OPM constituent); and
- explicit loss rejection when XML-only `MESSAGE_ID` data would be sent to sequential KVN.

The shipped `ndm_g22.xml` is valid against the official master XSD, but its OPM constituent has a
maneuver without `spacecraftParameters/MASS`. The already-verified standalone OPM semantic gate
requires mass for maneuver validation. The focused test records both facts and rejects the
combined document; it does not weaken OPM validation or invent missing data.

## Normative envelope reconciliation

CCSDS 505.0-B-3 sections 4.11.3–4.11.8 define the envelope rather than a separate ICS table:

| Requirement | Decision and evidence |
| --- | --- |
| Root element is `ndm` | Exact-root and trailing-content tests reject any other document envelope. |
| Root uses the standard attributes; `id` and `version` apply to the root | The strict attribute contract accepts only the schema/root namespace attributes plus `id` and `version`; unknown attributes are rejected. |
| Constituent application elements use only their own `id` and `version` attributes | Every registered child family is sequence-validated by its standalone parser; envelope tests reject illegal constituent attributes and nested message roots. |
| Any combination of constituent NDM types is permitted, in document order | The typed `Vec<MessageType>` preserves heterogeneous order; G12 and G21 prove multi-message and multi-family preservation. |
| A combined message should contain at least one constituent | This is a `SHOULD`, while the official XSD permits zero children. The parser accepts an empty envelope and documents the recommendation rather than inventing a schema-incompatible `SHALL`. |

The book's generic XML guidance calls for namespace declarations, while shipped combined examples
are inconsistent: G12 includes the `ndm` namespace and G21/G22 omit it. Parsing therefore accepts
both schema-valid spellings and generation is judged against the official master XSD instead of
requiring a namespace spelling absent from official examples.

Python combined construction/identity and aggregate limits are exercised by
`bindings/python/tests/test_ndm.py` and `test_parse_and_generation_options.py`. `combined_cli`
executes generic validation/conversion and aggregate limit behavior through the CLI binary.
`package-python` and `package-rust` are the shared built-artifact installation gates.

## Status

Combined NDM remains `implemented-unverified`. Exact-cell review and resolution of the XSD-valid
G22 versus standalone OPM maneuver/MASS semantic conflict remain explicit promotion blockers.
