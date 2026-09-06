# Combined NDM conformance inventory

Status: `implemented-unverified`.

The combined instantiation is the XML `ndm` envelope defined by CCSDS 505.0-B-3, section 4.11,
and `ccsds-ndm/data/xsd/ndmxml-4.0.0-master-4.0.xsd`. It is the only representation this library
parses or writes.

CCSDS defines no KVN combined instantiation. Section 4.11 lives in the XML specification and
requires the `<ndm></ndm>` root element tags, while each KVN family section requires the message
header line to be the first non-blank line in the file (CDM 508.0-B-1 section 6.3.1.2, ODM
502.0-B-3 section 7.3.6, ADM 504.0-B-2 sections 3.2.2.3/4.2.2.3/5.3.2.3, RDM 508.1-B-1 section
5.3.2.2). The ODM and ADM books also prescribe separate *files* for multiple messages (ODM
section 2.5, ADM section 2.5.1). A previously shipped sequential-KVN convenience form was
therefore removed; combined KVN parsing and generation now fail with the `unsupported.notation`
diagnostic, and generic detection rejects any KVN document carrying more than one message header.

`just conformance-combined` establishes:

- exact root and constituent attribute contracts, document-root/trailing-content checks, schema
  child ordering, and rejection of unknown or nested message structure;
- ordered typed preservation of the `ndm_g12.xml` and `ndm_g21.xml` constituents, followed by
  official-master-XSD validation of generated XML;
- aggregate input-byte, XML-depth, history-record, and output-byte limits through direct Rust and
  Python entry points;
- complete-envelope generation preflight: invalid children or aggregate output limits write zero
  bytes before the normative XML representation is streamed;
- measured linear XML streaming allocation budgets from 10 to 1,000 constituents (at most twenty
  temporary allocations and 8 KiB of temporaries per additional OPM constituent); and
- refusal of combined KVN parsing and generation with an unsupported-notation diagnostic.

The shipped `ndm_g22.xml` is valid against the official master XSD, but its OPM constituent has a
maneuver without `spacecraftParameters/MASS`. The already-verified standalone OPM semantic gate
requires mass for maneuver validation. The focused test records both facts and rejects the
combined document; it does not weaken OPM validation or invent missing data.

## Normative envelope reconciliation

CCSDS 505.0-B-3 sections 4.11.3–4.11.8 define the envelope rather than a separate ICS table:

| Requirement | Decision and evidence |
| --- | --- |
| Root element is `ndm` | Exact-root and trailing-content tests reject any other document envelope. |
| Root carries no `id`/`version`; they apply to constituents | `ndmxml-4.0.0-ndm-4.0.xsd` `ndmType` declares no attributes and shipped `ndm_g12/g21.xml` carry none on the root; the strict root contract therefore accepts only namespace/schema attributes and rejects `id`/`version` there. Constituent `id`/`version` remain required per child. |
| Constituent application elements use only their own `id` and `version` attributes | Every registered child family is sequence-validated by its standalone parser; envelope tests reject illegal constituent attributes and nested message roots. |
| Any combination of constituent NDM types is permitted, in document order | The typed `Vec<Message>` preserves heterogeneous order; G12 and G21 prove multi-message and multi-family preservation. |
| A combined message should contain at least one constituent | This is a `SHOULD`, while the official XSD permits zero children. The parser accepts an empty envelope and documents the recommendation rather than inventing a schema-incompatible `SHALL`. |

The book's generic XML guidance calls for namespace declarations, while shipped combined examples
are inconsistent: G12 includes the `ndm` namespace and G21/G22 omit it. Parsing therefore accepts
both schema-valid spellings and generation is judged against the official master XSD instead of
requiring a namespace spelling absent from official examples.

Python combined construction/identity and aggregate limits are exercised by
`bindings/python/tests/test_ndm.py` and `test_parse_and_generation_options.py`. `package-python`
and `package-rust` are the shared built-artifact gates.

## Status

Combined NDM remains `implemented-unverified` under the [shared promotion policy](family-shared-contract.md#promotion-policy). Its family-specific blocker is the
XSD-valid G22 versus standalone OPM maneuver/`MASS` semantic conflict, which must be resolved
independently of the exact-cell review.
