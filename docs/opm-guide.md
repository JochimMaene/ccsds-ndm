# OPM 2.0 and 3.0 Guide

The implemented scope is standalone CCSDS OPM 2.0 and 3.0 in KVN and XML. The
[support matrix](support-matrix.md) remains authoritative about verification status.

## Rust

Parse and validate with `Opm::from_kvn` or `Opm::from_xml`. Use
`Opm::from_kvn_with_options` / `from_xml_with_options` for an aggregate input bound or XML-depth
policy. Input bytes are unlimited by default; XML depth defaults to 16 because valid OPM has a small
fixed schema depth.

Generate with `Ndm::to_kvn` / `to_xml`, or use `VersionedNdm` with `GenerateOptions` for an explicit
target edition, output-byte bound, or streaming sink. Generation always validates first. A
configured streaming output limit is counted before the caller sink receives bytes; the unlimited
fast path performs one serialization pass.

`convert` composes strict detection, parsing, and validated target generation. `convert_file` and
`convert_to_file` replace a destination atomically only after conversion succeeds. Finite XML
values are rounded when necessary to the 16-digit KVN representation required by CCSDS ODM.

Errors expose stable `code()`, `field_path()`, and `diagnostic()` accessors. Diagnostic wording may
improve before 1.0; codes, enum meanings, and canonical paths are the machine interface.

## Python

Install distribution `ccsds-ndm-py` and import `ccsds_ndm`. `Opm.from_str` / `from_file` accept
the keyword-only `max_input_bytes` safety control; record-bearing messages additionally accept
`max_records`. `to_str` and `to_file` accept `max_output_bytes`. Python calls the Rust core for
every parse, validation, and generation decision.
Raised NDM exceptions expose `code`, `severity`, `operation`, `notation`, `message_kind`, editions,
field path, and available source location/token fields.

Use `ccsds_ndm.convert(data, "xml")` for strings and
`ccsds_ndm.convert_file(source, destination, "kvn")` for atomic file conversion. Input notation is
detected automatically. Both accept the same optional input, record, and output limits and delegate
directly to Rust.

Nested model properties are live and can be changed directly:

```python
message.segment.metadata.object_name = "UPDATED"
```

Generation is always validated; there is no unchecked `validate=False` mode.

## Construction and optional blocks

[`builder_demo.rs`](../ccsds-ndm/examples/builder_demo.rs) constructs a minimal state-vector OPM
using only public builders. The shipped [`opm_g4.kvn`](../ccsds-ndm/data/kvn/opm_g4.kvn) fixture demonstrates
Keplerian elements and covariance; [`opm_g2.kvn`](../ccsds-ndm/data/kvn/opm_g2.kvn) demonstrates spacecraft
parameters and maneuvers; [`opm_g3.kvn`](../ccsds-ndm/data/kvn/opm_g3.kvn) demonstrates user-defined
parameters. Parsing a fixture, changing typed public fields, calling `validate()`, and generating is
the shortest advanced workflow; generation revalidates because fields remain mutable before 1.0.

OPM, OEM, and OMM can target ODM 2.0 or 3.0. The 2.0 implementation is checked against the archived
official [SANA NDM/XML schema archive](https://sanaregistry.org/r/ndmxml_unqualified/). OPM and OEM
ODM 1.0 remain parse-only: no audited 1.0 schema-backed serializer is available, so the library
rejects attempts to relabel or convert them.

## Migrating the pre-0.0.9 API

- Replace `convert_opm` and `convert_opm_file` with the generic `convert` and `convert_file`.
- Remove the `validate` argument from `to_str` and `to_file`; output is always validated.
- Change nested Python fields directly; the former editor/proxy API has been removed.

## Performance and limits

OPM remains bounded-materialization software; streaming parsing is intentionally absent. KVN/XML
parse and generation benchmarks use the richest shipped fixtures, while KVN also measures repeated
maneuver scaling. Allocation tests protect materialized and streaming generation. The project makes
no “fastest” claim until an external reproducible comparison exists.
