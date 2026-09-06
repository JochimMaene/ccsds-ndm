# Python object model

Status: accepted

## Decision

Python messages will behave like ordinary mutable Python object graphs:

```python
message.segment.metadata.object_name = "UPDATED"
message.segments[0].data.state_vector[0].x = 7000.0
```

Nested getters return live child objects. Repeated model fields are live Python collections, so
structural changes such as `append`, item assignment, and deletion affect the owning message. A
separate editor, commit operation, or `.value` escape hatch is not part of the API.

Large numeric value sequences remain compact Rust values and are exposed as snapshots or NumPy
arrays. Updating those uses whole-field assignment or the documented NumPy setter. Eagerly boxing
every numeric value as a Python object would harm the large-history workload without improving the
usual record-editing workflow.

The binding graph is authoritative while Python owns the message. Validation and generation build
the corresponding Rust value and always pass it through the Rust core's existing correctness gate.
The Python layer does not duplicate CCSDS parsing, validation, or generation rules.

Python object assignment follows normal Python reference semantics. Assigning the same child to two
places creates an intentional shared reference; users who want an independent object can copy it.
This is simpler and more familiar than introducing a custom ownership rule solely for the binding.

## Why this design

- Generation from caller data is the primary workflow; parsing remains core. Direct mutation stays supported and is understood by Python users without
  library-specific concepts, but it is no longer claimed to be the shortest common workflow.
- Child identity makes retained references predictable, including after a child is removed from a
  collection.
- The complete edited graph is what validation and generation observe, so there is no stale
  shadow copy or forgotten commit.
- Ordinary lists are preferable to a custom repeated-field framework unless a benchmark proves
  that a specialized representation is needed. The reproducible benchmark shows extra per-record
  cost at large sizes, but no equivalent Rust-owned alternative has been measured. The existing
  model therefore remains provisional.

The rejected long-term alternatives are immutable rebuilding and an editor/proxy API. Immutable
rebuilding adds ceremony to routine edits. An editor hides copy/write-back behavior behind a second
type, weakens static typing, and makes large-history edit cost easy to miss.

## Performance evidence

OEM was the deciding workload because its state-vector histories can be large. The
time-sensitive paths were measured before extending the graph to every message family:

- parse and generation time for representative small and large OEM files;
- repeated scalar edits through a retained state-vector reference; and
- the same end-to-end workloads in the competing Python package.

Those historical measurements were not retained and do not support a comparative claim.
`just bench-python-object-model` measures OEM parsing, construction from records and NumPy arrays,
validation, generation, numeric access, editing, and isolated-process peak RSS. Results are local
comparison evidence, not release thresholds.

A comparison with another package is valid only when both libraries process the same generated
document on the same machine. A specialized native repeated sequence still must preserve direct
mutation and the Rust-core validation/generation gate, and must demonstrate a material improvement
on this workload before earning its complexity.

## Parsing resource controls

Python parsing exposes `max_input_bytes` and, for record-bearing formats, `max_records` as
keyword-only advanced controls. They solve real boundary and batch-processing needs and correspond
to limits users can reason about.

XML nesting depth remains a safe internal parser limit. CCSDS document depth is determined by the
schema, so asking each caller to select `max_xml_depth` adds API surface without improving the
normal workflow. The Rust core retains its parser option for lower-level and non-Python uses.
