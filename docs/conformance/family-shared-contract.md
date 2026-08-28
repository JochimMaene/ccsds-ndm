# Shared NDM family contract evidence

Status: implemented shared plumbing, not blanket CCSDS conformance. The verified OPM 3.0, OEM 3.0,
and OMM 3.0 cells remain governed by their message-specific inventories and focused surface
evidence.

This inventory records the cross-family mechanics that are now exercised for every standalone
message and the combined NDM envelope. It deliberately does not substitute shared tests for the
message-by-message normative mapping required to promote a capability cell to `verified`.

| Shared requirement | Current evidence | Remaining verification work |
| --- | --- | --- |
| Version-aware generation gate | Every standalone `MessageType` output dispatches through `VersionedNdm`; source, latest, and exact target policy is centralised in `generation.rs`. | Prove each advertised source/target edition against its exact publication and notation rules. |
| Validated generation | String, streaming, Python, and conversion paths delegate to the Rust `Ndm` / `VersionedNdm` boundary. | Complete message-specific semantic and notation inventories, invalid-model tests, KVN golden ordering, and official-XSD generation evidence. |
| Bounded parsing | `ParseOptions` applies input, XML-depth, and applicable history-record limits; bounded file reads occur before full materialisation. | Establish exact record semantics and adversarial boundary cases for each history-bearing message. |
| Bounded generation | `GenerateOptions` applies exact aggregate output limits; configured streaming limits use a counting-writer preflight before writing. | Add message-specific allocation budgets where history size can materially affect memory behaviour. |
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
for TIME_AND_ANGLE direction vectors.

This evidence intentionally does not claim complete cross-notation semantic preservation. CDM now
retains the distinct header, relative, metadata, additional-parameter, state-vector, and covariance
comment associations and has a KVN model fixed-point regression. KVN supplies no delimiter between
the outer data comments and the immediately following first nested block's comments, however, so
strict parsing preserves that leading run on the outer block and every KVN generation path rejects
a populated first-nested comment rather than guessing a split. AEM's fixed XML unit
attributes are optional (504.0-B-2 section 7.6.10), while units are forbidden in AEM KVN data lines
(section 6.9.2); an XML-to-KVN-to-XML hop therefore deliberately normalises explicit unit
annotations to omission. The dedicated regression verifies that this output remains schema-valid.

CDM and RDM now share the routed-block KVN parser that associates a comment run with the nested
logical block selected by the following keyword; each message retains only its keyword routing.
RDM additionally rejects the XML-only outer data comment position at every KVN generation gate,
because flattened KVN cannot distinguish it from the first atmospheric-block comment.

The shared family tests use one representative fixture per message to prove registration and
contract routing. They do not prove complete valid-input coverage, semantic preservation of every
optional field, or normative conformance. Those remain message-specific work and keep the
capabilities below `verified`.
