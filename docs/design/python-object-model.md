# Python object model

Status: accepted

## Decision

Python messages will behave like ordinary mutable Python object graphs:

```python
opm.segment.metadata.object_name = "UPDATED"
oem.segments[0].data.state_vector[0].x = 7000.0
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
places creates an intentional shared reference. Python `copy` and `pickle` are not supported;
duplicating a complete message requires generation and parsing, which validates and normalizes
the data and retains only what that notation represents. See the
[editing workflow](../guide/workflows.rst) for the public contract.

## Why this design

- Generation from caller data is the primary workflow; parsing remains core. Direct mutation stays supported and is understood by Python users without
  library-specific concepts, but it is no longer claimed to be the shortest common workflow.
- Child identity makes retained references predictable, including after a child is removed from a
  collection.
- The complete edited graph is what validation and generation observe, so there is no stale
  shadow copy or forgotten commit.
- Ordinary lists are preferable to a custom repeated-field framework unless a benchmark proves
  that a specialized representation is needed.

The rejected long-term alternatives are immutable rebuilding and an editor/proxy API. Immutable
rebuilding adds ceremony to routine edits. An editor hides copy/write-back behavior behind a second
type, weakens static typing, and makes large-history edit cost easy to miss.

## Performance evidence

OEM was the deciding workload because its state-vector histories can be large. The
time-sensitive paths were measured before extending the graph to every message family:

- parse and generation time for representative small and large OEM files;
- repeated scalar edits through a retained state-vector reference; and
- the same end-to-end workloads in the competing Python package.

These measurements were local comparison evidence, not release thresholds.

A 2026-09-09 release-build experiment compared the live model with Rust-owned copy-on-write OEM
histories at 100,000 records. The prototype made prepared-NumPy construction 73%, bulk replacement
75%, and validation 40% faster; XML generation improved 9% and its peak RSS fell from 224 to 160
MiB. NumPy export became 82% slower, while record construction used 138 rather than 118 MiB. It
removed only nine production lines and added snapshot semantics and explicit update methods, so it
was rejected for increasing ownership complexity despite useful performance gains.

A comparison with another package is valid only when both libraries process the same generated
document on the same machine. A specialized native repeated sequence still must preserve direct
mutation and the Rust-core validation/generation gate, and must demonstrate a material improvement
on this workload before earning its complexity.

## Parsing resource controls

Parsing exposes no resource-policy knobs. Callers that accept untrusted input should enforce size
limits at their system boundary; XML nesting retains a fixed internal safety limit.

`UserDefined.entries` is an ordered snapshot of strings (comments) and `(name, value)`
tuples (parameters). Assign the list after editing it. `comment` and `user_defined` remain
compatibility snapshots. Replacing parameters preserves existing comment positions;
assigning `comment` places all supplied comments before parameters and keeps parameter order.
Dictionary construction and replacement follow Python insertion order. XML preserves comment
interleaving; KVN generation refuses interleaving that its grammar cannot represent.
The dictionary view keeps the last value for repeated names; use `entries` to preserve duplicates.

`OcmUserDefined.entries` is a live ordered list of comment strings and
`OcmUserDefinedParameter` objects. The `user_defined` compatibility list is a snapshot with
live parameter objects: assign it after adding/removing items, or edit `entries` directly.
Opaque sequence records remain attached to their parameter.

Python constructors require the message's mandatory interpretation fields; they do not infer
AEM attitude type, OMM mean-element theory, CDM ephemeris name, or missing covariance entries.
`OpmCovarianceMatrix` requires all 21 lower-triangle values, including explicit zeroes.
The exposed PyO3 categorical classes have named members but are not Python `enum.Enum`
subclasses; the generated stubs describe their actual class/member surface.
