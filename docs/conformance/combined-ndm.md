# Combined NDM conformance

The combined instantiation is the XML `ndm` envelope defined by CCSDS 505.0-B-3, section 4.11,
and `ccsds-ndm/data/xsd/ndmxml-4.0.0-master-4.0.xsd`. It is the only output representation.
The NDM/XML 1.0 envelope is also accepted as input only, with its own structure checked; see
[Legacy envelope input](#legacy-envelope-input).

CCSDS defines no KVN combined instantiation. Section 4.11 lives in the XML specification and
requires the `<ndm></ndm>` root element tags (NDM/XML §§4.11.3–4.11.5;
ODM §§8.12.6–8.12.7), while each KVN family section requires the message
header line to be the first non-blank line in the file (CDM 508.0-B-1 section 6.3.1.2, ODM
502.0-B-3 section 7.3.6, ADM 504.0-B-2 sections 3.2.2.3/4.2.2.3/5.3.2.3, RDM 508.1-B-1 section
5.3.2.2). The ODM and ADM books also prescribe separate *files* for multiple messages (ODM
section 2.5, ADM section 2.5.1). A previously shipped sequential-KVN convenience form was
therefore removed; combined KVN parsing and generation now fail with the `unsupported.notation`
diagnostic, and generic detection rejects any KVN document carrying more than one message header.

The combined-NDM suites establish:

- exact root and constituent attribute contracts, document-root/trailing-content checks, schema
  child ordering, and rejection of unknown or nested message structure;
- envelope text decoding for XML entities, character references and CDATA, preservation of empty
  `MESSAGE_ID`/`COMMENT` strings, namespace-aware schema-location hints on those fields, and
  rejection of other attributes or nested elements;
- whitespace character references and CDATA accepted between envelope elements; references and
  CDATA outside the document root remain invalid XML (`envelope_accepts_decoded_whitespace_only_inside_the_root`);
- processing instructions and XML comments ignored throughout the envelope, including within
  text fields, consistently with the shared XML contract;
- ordered typed preservation of the `ndm_g12.xml` and `ndm_g21.xml` constituents, followed by
  official-master-XSD validation of generated XML;
- a fixed internal XML nesting safety limit through direct Rust and Python entry points;
- complete-envelope generation preflight: invalid children write zero bytes before the normative
  XML representation is streamed;
- measured linear XML streaming allocation budgets from 10 to 1,000 constituents (at most twenty
  temporary allocations and 8 KiB of temporaries per additional OPM constituent); and
- refusal of combined KVN parsing and generation with an unsupported-notation diagnostic.

The shipped `ndm_g22.xml` is valid against the official master XSD, but its OPM constituent has a
maneuver without `spacecraftParameters/MASS`. ODM §3.2.4.9 requires the conditional
spacecraft fields, including mass, when a maneuver is present; XML §4.11 grants constituents
no exception to their standalone semantics.
`ndm::generation::shipped_g22_is_schema_valid_but_rejected_by_the_verified_opm_semantic_gate`
records both facts and rejects the combined document; it does not weaken OPM validation or invent missing data. This is a resolved example
defect: XSD validity alone does not establish the book's cross-field semantics. A combined
envelope is verified by its own rules and the same validation gate for every constituent.

## Normative envelope reconciliation

CCSDS 505.0-B-3 sections 4.11.3–4.11.8 define the envelope rather than a separate ICS table:

| Requirement | Decision and evidence |
| --- | --- |
| Root element is `ndm` | `ndm::parsing::envelope_text_decodes_entities_cdata_and_empty_elements` and the envelope rejection tests enforce the root and trailing-content boundary. |
| Root carries no `id`/`version`; they apply to constituents | `ndmxml-4.0.0-ndm-4.0.xsd` `ndmType` declares no attributes and shipped `ndm_g12/g21.xml` carry none on the root; the strict root contract therefore accepts only namespace/schema attributes and rejects `id`/`version` there. Constituent `id`/`version` remain required per child. |
| Constituent application elements use only their own `id` and `version` attributes | Every registered child family is sequence-validated by its standalone parser; envelope tests reject illegal constituent attributes and nested message roots. |
| Any combination of constituent NDM types is permitted, in document order | The typed `Vec<Message>` preserves heterogeneous order; `ndm::parsing::combined_ndm_xml` fixture roundtrips for G12 and G21 prove multi-message and multi-family preservation. |
| A combined message should contain at least one constituent | This is a `SHOULD`, while the official XSD permits zero children. The parser accepts an empty envelope and documents the recommendation rather than inventing a schema-incompatible `SHALL` (`ndm::generation::empty_combined_envelope_is_accepted_and_generated_despite_the_should`). |

The book's generic XML guidance calls for namespace declarations, while shipped combined examples
are inconsistent: G12 includes the `ndm` namespace and G21/G22 omit it. Parsing therefore accepts
both schema-valid spellings and generation is judged against the official master XSD instead of
requiring a namespace spelling absent from official examples.

Python combined construction/identity and the fixed XML nesting limit are exercised by
`bindings/python/tests/test_ndm.py` and `test_parse_and_generation_options.py`. `package-python`
and `package-rust` are the shared built-artifact gates.

## Constituent editions in the bundled envelope

The output envelope is NDM/XML 4.0.0. Its included `opmType`, `ommType`, and `oemType`
fix `version` to `3.0`; `tdmType` fixes it to `2.0`. The other included families use their current editions. Standalone
ODM 2.0 parsing/generation remains supported with its own schema, but a 2.0 ODM constituent
cannot be written in the current combined envelope. XML input accepts the separate legacy
profile; materialized/streaming output refuses legacy constituents before writing bytes. The model is not silently
upgraded or relabeled. NDM/XML 1.0 is an additional input-only envelope profile.
`combined_schema_refuses_standalone_odm2_constituents_before_output` covers all three ODM families,
standalone controls, model validation, legacy XML input, and both output paths.
Standalone TDM 1.0 input also remains supported, but the 4.0.0 envelope rejects a TDM 1.0
constituent. `combined_schema_refuses_legacy_tdm_but_standalone_input_remains_supported`
checks typed/generic input and refusal before output. Notation-neutral model validation
can still succeed for valid standalone constituents; it does not promise envelope representability.

## Verification outcome

Constituent deserializer errors retain their family and translate line, column, byte offset,
and source excerpt back into the original combined XML, including inherited namespace
declarations inserted for standalone parsing. The human message identifies the zero-based
constituent index. Structural/envelope errors may still have no source location, as stated in
the shared diagnostic contract. Python rejects nested combined children before converting
their contents, so a self-reference cannot recurse through the live `messages` list.

An ODM 2.0 child is a supported standalone edition but unrepresentable in this 4.0.0 envelope;
that contextual constraint uses `validation.invalid_value` rather than declaring the standalone
edition unsupported. No automatic upgrade or version relabelling takes place.

Combined NDM meets the common [verification criteria](README.md#verification-criteria)
on Rust and Python for XML. KVN is an explicitly unsupported notation, not a missing
conversion implementation. The G22 example defect is resolved by rejecting its invalid
constituent consistently with standalone OPM.

Shared Rust/Python route coverage, including `test_conformance_matrix.py`, and installed
artifact checks are indexed once in the [common evidence table](README.md#shared-evidence).

Space-Track ODM 2.0 `<ndm>` exports are accepted under the legacy input profile. XML
well-formedness and namespace checks still run first; undeclared prefixes remain errors.
CelesTrak OMM with empty creation/originator fields remains rejected under ODM 7.5.1;
accepting its envelope never waives the requirement for populated provenance.

Namespace declarations on constituents are accepted because XML Namespaces does not
class them as ordinary application attributes. Constituent schema-location hints,
including `xsi:noNamespaceSchemaLocation`, are rejected under ODM 8.12.7
(`ndm::generation::constituent_namespace_declarations_are_namespace_scope_not_attributes`).
Empty `<ndm/>` can be read and generated. The published G22 OCM constituent independently violates
MAN_COMPOSITION table order (6.2.8.16), in addition to the OPM MASS issue recorded above
(`ndm::generation::shipped_g22_ocm_constituent_violates_man_composition_order`).

## Legacy envelope input

NDM/XML 1.0 envelopes (Space-Track exports) are accepted as **input only**. Reading one checks
the envelope against `ndmxml-1.0-ndm-1.0.xsd` and `ndmxml-1.0-master.xsd`, not merely each
constituent's standalone rules:

- the root and constituents are in no namespace: the 1.0 master schema declares `ndm` without
  a target namespace and `elementFormDefault="unqualified"`, so a root or constituent qualified
  with `urn:ccsds:schema:ndmxml` or with the 1.0 types URN
  `urn:ccsds:recommendation:navigation:schema:ndmxml` is rejected;
- the root carries no `id`/`version` (namespace declarations and schema hints only, as in 4.0.0);
- `ndmType` is `COMMENT*` followed by any of `aem`, `apm`, `oem`, `omm`, `opm`, `tdm`; there is
  no `MESSAGE_ID`, comments may not follow a constituent, and 4.0.0-only families (ACM, CDM,
  OCM, RDM) cannot appear;
- constituent editions are those the 1.0 schema includes: ODM 2.0 (OPM/OMM/OEM) and TDM 1.0.
  APM/AEM 1.0 remain rejected by their own edition decision, so an envelope containing them fails.

The envelope edition is inferred from the constituents: any 1.0-era constituent selects the 1.0
profile, and 1.0 and 4.0.0 editions cannot be mixed. An empty envelope is read under 4.0.0.

Output support is explicitly scoped: generation writes only the NDM/XML 4.0.0 envelope, so a
model holding ODM 2.0 or TDM 1.0 constituents is refused before any bytes are written. Model
`validate()` does not check this envelope representability. Reading a 1.0 envelope therefore
never promises that it can be regenerated unchanged; extract the constituents as standalone
ODM 2.0/TDM 1.0 XML or convert them to current editions explicitly.

Evidence: `ndm::parsing::ndm_xml_1_0_envelope_is_input_only_and_checked_against_its_own_schema`
(Space-Track-shaped fixture accepted, envelope violations and APM 1.0 rejected, output refusal),
`pre_release::legacy_envelopes_are_input_only_and_cannot_mix_editions`, and
`ndm::generation::combined_schema_refuses_standalone_odm2_constituents_before_output`.
