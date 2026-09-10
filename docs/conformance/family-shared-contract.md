# Shared NDM family contract evidence

Status: implemented shared plumbing, not blanket CCSDS conformance. The verified OPM 3.0, OEM 3.0,
and OMM 3.0 cells remain governed by their message-specific inventories and focused surface
evidence.

This inventory records the cross-family mechanics that are now exercised for every standalone
message and the combined NDM envelope. It deliberately does not substitute shared tests for the
message-by-message normative mapping required to promote a capability cell to `verified`.

| Shared requirement | Current evidence | Remaining verification work |
| --- | --- | --- |
| Version-aware generation gate | Every standalone `Message` output dispatches through `Ndm` and preserves its stored edition. | Prove each advertised edition against its exact publication and notation rules. |
| Validated generation | String, streaming, Python, and conversion paths delegate to the Rust `Ndm` boundary. | Complete message-specific semantic and notation inventories, invalid-model tests, KVN golden ordering, and official-XSD generation evidence. |
| Bounded parsing | `ParseOptions` applies input, XML-depth, and applicable history-record limits; bounded file reads occur before full materialisation. | Establish exact record semantics and adversarial boundary cases for each history-bearing message. |
| Structured diagnostics | Shared parsing and generation wrappers retain notation, message kind, edition context, stable resource codes, and bounded token excerpts. | Map field paths and normative requirements for each message's syntax and semantic failures. |
| Rust/Python consistency | `family_contract` and `test_parse_and_generation_options.py` exercise generic dispatch and shared limits without per-message adapter semantics. | Add exact capability-cell surface evidence and packaged-artifact checks before advertising parity. |
| Strict XML envelope | Standalone roots, attributes, trailing documents, unknown structural fields, and the normative combined `ndm` envelope are checked without accepting arbitrary wrapper flattening. | Audit every nested message-specific XML type and sequence against the applicable XSD and book. |
| Reproducible performance workload | `kvn_message_matrix` covers KVN parse/generate for all ten standalone families; `xml_message_matrix` covers XML parse/generate for those ten plus combined NDM. `just bench-family` reproduces both, and the existing CodSpeed workflow executes the benchmark targets. | Record reviewed baselines and add targeted allocation/scaling gates only for measured hotspots. Wall-clock thresholds remain informational. |

`family_generation_evidence` additionally runs every shipped fixture for the eight remaining
standalone families through deterministic KVN and XML generation, reparses both outputs, and checks
all generated XML against the official master XSD. It regression-tests preservation of the ACM
physical-description block, which was previously lost by applying the scalar nullable adapter to a
complex XML element. The focused OCM evidence applies the same correction to its perturbation,
orbit-determination, and user-defined blocks, and separately proves the schema's scalar lexical form
for TIME_AND_ANGLE direction vectors, which is described in [ocm-3.0.md](ocm-3.0.md).

This evidence intentionally does not claim complete cross-notation semantic preservation. Two
families resolve a notation asymmetry in a way that is documented where the decision belongs, not
restated here: CDM's delimiter-free COMMENT decision in [cdm-1.0.md](cdm-1.0.md), and AEM's
normalisation of optional fixed XML units to omission across an XML-to-KVN-to-XML hop in
[aem-2.0.md](aem-2.0.md). Both are backed by dedicated regressions.

CDM and RDM now share the routed-block KVN parser that associates a comment run with the nested
logical block selected by the following keyword; each message retains only its keyword routing.
RDM additionally rejects the XML-only outer data comment position at every KVN generation gate,
because flattened KVN cannot distinguish it from the first atmospheric-block comment.

## Externally governed values

Every family delegates some vocabulary to sources outside the message. The policy is one rule, not
a per-family decision, and it is stated here so the family documents do not restate it:

- Values the book delegates to a **living SANA registry** remain caller-supplied open strings. The
  library validates locally decidable syntax and message semantics, and does not embed a mutable
  registry snapshot. Runtime validation is offline, so current registry membership cannot be an
  acceptance boundary.
- Values a book defines as a **closed edition-specific list** are rejected when unknown. CDM and
  RDM `OBJECT_TYPE` are the worked example; OCM's `OBJECT_TYPE` is registry-governed instead. The
  authoritative statement is "Registry-governed values" in the
  [validation contract](../design/validation-contract.md).
- **Mission truth, physical correctness, sensor models, and external identifier resolution** are
  outside self-contained message validation in every family. A message can be fully valid and still
  describe an operationally wrong situation.

A family document should record only its own exception to this policy, not the policy itself.

## Promotion policy

Most families in these documents are marked `implemented-unverified`, which is the same state the
[support matrix](../support-matrix.md) calls **Available**: implemented and tested, but without the
complete message-specific review that promotion requires. The two vocabularies are reconciled here
so they do not drift apart.

Grouped ICS reconciliation and shared artifact gates are evidence *for* that later review; they do
not themselves promote a capability cell. Promotion requires review into exact
operation × notation × surface cells.

A family document should therefore state its status in one line and then record only its own
promotion blockers — the parts that actually differ between families. Known family-specific
blockers today are AEM's interpolation-degree/example conflict, TDM's unverified edition 1.0
exposure, and combined NDM's XSD-valid/book-invalid conflict.

The shared family tests use one representative fixture per message to prove registration and
contract routing. They do not prove complete valid-input coverage, semantic preservation of every
optional field, or normative conformance. Those remain message-specific work and keep the
capabilities below `verified`.
