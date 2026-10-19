# Supported messages

Every current message family below uses the same
[conformance verification criteria](conformance/README.md#verification-criteria) on both the Rust
and Python APIs. **Verified** means the stated publication baseline, self-contained requirements,
notation rules, public workflows, and packaged artifacts have been reviewed and tested. Each
family record states its authority and limits; this is not certification of mission truth or of
behavior governed by an external algorithm, registry, or ICD.

## Current support

| Message | Editions | KVN | XML | Requirements and decisions |
| --- | --- | --- | --- | --- |
| OPM | 2.0, 3.0 | Parse, write, convert | Parse, write, convert | [Orbit parameters](conformance/opm-3.0.md) |
| OMM | 2.0, 3.0 | Parse, write, convert | Parse, write, convert | [Mean elements](conformance/omm-3.0.md) |
| OEM | 2.0, 3.0 | Parse, write, convert | Parse, write, convert | [Ephemerides](conformance/oem-3.0.md) |
| OCM | 3.0 | Parse, write, convert | Parse, write, convert within the bundled schema | [Comprehensive orbit data](conformance/ocm-3.0.md) |
| CDM | 1.0 | Parse, write, convert | Parse, write, convert | [Conjunction data](conformance/cdm-1.0.md) |
| TDM | 2.0 | Parse, write, convert | Parse, write, convert | [Tracking data](conformance/tdm-2.0.md) |
| RDM | 1.0 | Parse, write, convert | Parse, write, convert | [Reentry data](conformance/rdm-1.0.md) |
| APM | 2.0 | Parse, write, convert | Parse, write, convert | [Attitude parameters](conformance/apm-2.0.md) |
| AEM | 2.0 | Parse, write, convert | Parse, write, convert | [Attitude ephemerides](conformance/aem-2.0.md) |
| ACM | 2.0 | Parse, write, convert; relative histories | Parse, write, convert; relative histories | [Comprehensive attitude data](conformance/acm-2.0.md) |
| Combined NDM | XML envelope | Not defined | Parse and write | [Combined instantiation](conformance/combined-ndm.md) |

Conversion preserves the supported model subject to documented normalization: ordinary numbers
may round to KVN's 16-digit precision, optional fixed-unit annotations may be omitted, and OPM's
data/state-vector comment positions merge in KVN. It is not an exact model or byte round trip;
see [preservation and normalization](design/validation-contract.md#preservation-and-normalization).
Other information the target cannot represent causes an explicit refusal. For
example, OCM's June 2026 technical corrigendum adds contact fields, covariance-to-trajectory
identifiers, and user-defined sequences that NDM/XML 4.0.0 cannot express. These remain available
in KVN; XML generation refuses them before output. CDM and RDM also document comment associations
that cannot be retained in flattened KVN. The library does not discard those values to force a
conversion to succeed.

Combined NDM is XML-only. Requests for combined KVN fail with `unsupported.notation`; several KVN
messages need application framing, such as one file per message. Constituents must satisfy their
standalone requirements.

## Legacy compatibility

OEM 1.0 has a restricted **KVN input-only compatibility reader**, separate from the verified
current editions. It accepts calendar/day-of-year epochs, checks the edition's record order and
segment spans, and rejects later-edition fields. Valid 1.0 Julian Date epochs remain unsupported.
There is no OEM 1.0 XML representation, and generation/conversion of a stored 1.0 message is
rejected. See [the complete compatibility limits](conformance/oem-3.0.md#oem-10).

OPM, APM, AEM, and TDM 1.0 input is not supported: their older requirements or syntax have not
been implemented and verified as separate editions. ACM was introduced in ADM issue 2; the
library accepts its defined 2.0 representation. A version label on a newer-format example is not
evidence of an older format's support. Unsupported versions fail explicitly rather than being
silently treated as current messages.

## Common guarantees and boundaries

Unsupported message content, duplicate singleton fields, malformed field values, and violations
of the documented self-contained rules are rejected. Ordering and producer compatibility follow each family's
record. Reading takes the permissive interpretation of an unclear requirement when doing so does
not require guessing or losing data; writers follow the clear representation rules. See the
[validation contract](design/validation-contract.md#authority-and-representability).

The library retains the meaning of units, reference frames, time systems, and interpolation declarations. It
does not perform general unit conversion, coordinate or time-scale transformations, propagation,
or interpolation. Runtime validation is offline; a valid message can still describe an
operationally incorrect situation.

The shared [XML parser limits](design/validation-contract.md#xml-parser-limits) apply to all
families. ACM attitude/covariance history records currently support relative time tags only;
the book's absolute history form is not implemented. Family records list further checks left
to the caller, including physical consistency and externally defined algorithms.

Run `just verify` for the quality suite and installed-artifact checks. The
[conformance index](conformance/README.md) maps shared checks to the family records; resource tests
and benchmarks measure implementation behavior without turning timing observations into
standards requirements.
