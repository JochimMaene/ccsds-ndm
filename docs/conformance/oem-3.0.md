# OEM 3.0 conformance

This page explains how the library handles OEM 3.0, and which tests prove it. It covers
parsing, validation, writing and conversion, in both KVN and XML. OEM 2.0 and 1.0 are covered
at the end; see [OEM 2.0](#oem-20) and [OEM 1.0](#oem-10).

The sources are CCSDS 502.0-B-3 and the NDM/XML 4.0.0 schemas. The later
[EC2 edition](https://ccsds.org/publications/allpubs/entry/3073/) of the book doesn't change
anything for OEM. Our local copy, `docs/ccsds-books/odm.rst`, stops at section 8.7.3 and leaves
out examples G-12 to G-14. Where this page cites those parts, it refers to the published book.

OEM 3.0 is **Verified**; see [Verification outcome](#verification-outcome). [Limits](#limits)
lists what the library deliberately leaves to the parties exchanging the message.

## What is implemented

| Area | Source | Behavior | Tests |
| --- | --- | --- | --- |
| Message structure | ODM 5.1–5.2, tables 5-1 to 5-4 | One OEM root with header, body and segments in order. All segments describe the same object and use the same time system. | `oem::parsing`, `oem::diagnostics`, `oem::validation` |
| KVN layout | ODM 5.2.4–5.2.5, 7.3–7.9, A2.5.3 | Printable ASCII only, lines of at most 254 characters, and lines terminated by LF, CR, CRLF or LFCR; reading also accepts an unterminated last line (see [More lenient than the book](#more-lenient-than-the-book)). Blank lines may appear anywhere and trailing blanks are ignored. Keywords come in the fixed order. An ephemeris line has 7 or 10 fields, and each covariance row has as many values as its row number. The covariance section may be empty. An optional keyword with no value counts as absent. Comments may only appear where the book allows them. | `oem::parsing`, `oem::generation`, `kvn::parser` unit tests |
| XML layout | ODM 8, NDM/XML 4.2–4.3, `ndmxml-4.0.0-oem-3.0.xsd` | The document must start with exactly `<?xml version="1.0" encoding="UTF-8"?>`; a line break after it is not required, so single-line documents parse, and the root must declare `xmlns:xsi`. Elements below the root are either all unqualified or all in the `urn:ccsds:schema:ndmxml` namespace; the root itself may be prefixed or not, as the books show both. Namespace declarations and schema-location hints are allowed on any element. Elements must come in schema order. Unknown content, a DTD, or a second document is rejected. | `oem::parsing`, `library::xml` |
| Time | ODM 3.2.3.2, 5.1.3, 5.2.3–5.2.5, 7.5.10 | Epochs use the calendar or day-of-year form with at most 16 fraction digits. MET and MRT also accept durations spelled `YYYY-DDDThh:mm:ss` with day 000 to 999. Start and stop times must be in order. Complete useable spans of consecutive segments may touch but not overlap; segments need not be in time order. State epochs must fall inside the segment's total span; covariance epochs need not (see [Where sources conflict](#where-sources-conflict)). Covariance epochs must increase, but equal epochs are allowed. | `oem::validation` |
| Values | ODM 5.2, 7.5, 8.13 | Required fields must be present. In KVN, the time systems and frames the book lists must be all upper or all lower case. XML numbers follow `xsd:double` (see [Where the book is unclear](#where-the-book-is-unclear)). KVN numbers must be finite and fit in a double; reading accepts more than 16 digits (see [More lenient than the book](#more-lenient-than-the-book)). Units are fixed and filled in when missing, and XML output omits the optional `units` attributes. | `oem::parsing`, `oem::validation`, `oem::conversion` |
| Writing KVN | ODM 5.2, 7.3–7.9 | The same message always gives the same output, and it is checked before anything is written. Numbers are rounded to at most 16 significant digits. Accelerations are written for all three axes or none. An empty `INTERPOLATION` or `COV_REF_FRAME` is rejected, because KVN would read it back as absent. | `oem::generation`, `oem_kvn_allocations` |
| Writing XML | OEM 3.0 XSD, NDM/XML 4.2–4.3 | The same message always gives the same output, and it is checked first. The root declares `xmlns:xsi` and `xmlns:ndm`. Output for every shipped OEM example passes the official schema. | `oem::generation`, `oem::validation` |
| Conversion | ODM 5 | Round trips of the example files keep the whole model and the edition. Converting to KVN fails for data KVN can't hold: partial accelerations, non-finite numbers, an empty `INTERPOLATION` or `COV_REF_FRAME`, and comments on any covariance matrix after the first. KVN also drops some whitespace (see [Conversion between notations](#conversion-between-notations)). | `oem::conversion` |
| Resources | Project policy | A standalone OEM can't nest beyond its schema, because unknown children are rejected; combined NDM XML has a fixed nesting limit. Files are replaced in one step, so a failed write never leaves half a file behind. Writing KVN doesn't use more memory as the record count grows. Reading XML doesn't use extra memory per state record, unless the input spells out `units` attributes. | `ndm::parsing`, `library::output`, `oem_kvn_allocations`, `oem_xml_allocations` |
| Scale | Project policy | Benchmarks parse and write 100, 10,000 and 50,000 records in KVN, and 100 and 10,000 in XML. The timings are for information only. | `cargo bench -p ccsds-ndm --bench kvn_benches -- oem_kvn_history_scaling`, `cargo bench -p ccsds-ndm --bench xml_benches -- oem_xml_history_scaling` |

## Decisions

Each decision below names the clause it rests on. When the book is unclear, or two sources
disagree, the parser takes the lenient reading. This is the general rule in the
[validation contract](../design/validation-contract.md#authority-and-representability). The few
cases where we are stricter are listed under [Stricter than required](#stricter-than-required),
and the two where reading is looser than a clear rule under
[More lenient than the book](#more-lenient-than-the-book).

### Where the book is unclear

- 5.2.4.4 says the useable spans of consecutive segments must not overlap, except at a shared
  endpoint. It does not order the segments, so a segment may come before the one it follows in
  time. A useable span is known only when both of its bounds are given, so two segments are
  compared only when both spans are complete. Total spans may overlap.
- The object must be the same in every segment (5.1.3). `OBJECT_NAME` and `OBJECT_ID` are
  compared ignoring case, and underscores and runs of blanks count as the same (7.5.9). So
  `Mars Global Surveyor` and `MARS_GLOBAL_SURVEYOR` are one object. The time system comparison
  (5.2.4.5) also ignores case and, in XML, surrounding blanks (` UTC ` names UTC); the text is
  kept as written.
- 7.5.3 says every normative KVN text value, meaning anything but comments and free text, is all
  upper or all lower case. Which fields are free text is unclear: the book gives no list, and
  values such as mission frames are defined by an ICD rather than the book. We read this leniently
  and check only the values the book itself lists: the time systems in 3.2.3.2, the frames in
  3.2.3.3, and `RSW`, `RTN` and `TNW` for `COV_REF_FRAME` (3.2.4.11). Mixed-case forms of those,
  such as `Utc`, are rejected in KVN when reading and writing. Other values keep their spelling,
  so `CENTER_NAME = Earth` is accepted even though annex B2 lists the natural bodies.
- 5.2.4.7 says every segment "must contain enough ephemeris data records to allow the recommended
  interpolation method to be carried out consistently". The book doesn't define the methods:
  `INTERPOLATION` is free text, and HERMITE, LINEAR and LAGRANGE are only examples in table 5-3.
  So it never says how many records a method needs, or what "consistently" means near a segment
  edge. We don't invent a count. We check what the book does define: a method comes with a
  positive degree.
- 5.2.5.7 says covariance matrices come "by increasing time tag". It doesn't say whether two
  matrices may share an epoch, so we allow it. This happens, for example, with two frames of the
  same navigation solution.
- `REF_FRAME_EPOCH` uses the segment's time system like every other OEM epoch (7.5.11). Under MET
  or MRT it is therefore a duration.
- 3.2.3.2 says MET and MRT times "should" use three-digit days. That is a recommendation, so we
  accept calendar times under MET and MRT as well.
- 7.5.10 allows as many fraction digits as a fixed-point number, and 7.5.6 caps those at 16. We
  apply the cap to the fraction digits only. Unlike data numbers, epochs keep this cap when
  reading.
- 7.5.1 only requires values for mandatory keywords. An optional OEM metadata or covariance
  keyword with an empty value is treated as absent. The shared header parser is the exception: an
  empty `CLASSIFICATION` or `MESSAGE_ID` read from KVN stays an empty string. In XML an empty
  `<INTERPOLATION/>` is an empty string. KVN can't tell an empty `INTERPOLATION` or
  `COV_REF_FRAME` from a missing one, so writing KVN rejects those empty values instead of
  dropping them.
- A bare `COMMENT` line is an empty comment. 7.8.5 wants a blank after the keyword, but 7.4.7 says
  trailing blanks don't matter. We write `COMMENT ` with the blank.
- Comments in a KVN covariance section with no matrices have nowhere to go in the model or the
  schema. They are added to the segment's data comments, so rewritten KVN puts them before the
  ephemeris lines.
- KVN numbers must fit in a double (7.5.7e). A value that would overflow to infinity is rejected,
  and so is a non-zero value that would round to zero. Very small numbers that still fit are
  fine. The 16-digit limit and the 32-bit integer range are not applied when reading (see
  [More lenient than the book](#more-lenient-than-the-book)).
- 7.5.7 doesn't say whether a mantissa may end in its decimal point. 7.5.7c calls a mantissa
  without exponent a fixed-point value, and 7.5.6 requires a digit after the point there, so we
  reject `1.` and `1.e3` in KVN alike.
- An empty XML element for an optional epoch or number is rejected, not read as absent. The
  schema's epoch pattern and `xsd:double` don't allow an empty value; omit the element instead.
- XML numbers follow `xsd:double` (8.13.1, 8.13.4), so state and covariance values may be `INF`,
  `-INF` or `NaN`. They must be spelled exactly that way, whether written directly, as character
  references, or in CDATA, and XML output writes them that way. KVN numbers must be finite
  (7.5.5–7.5.7), so these values can't be written to KVN.
- In a combined NDM, 8.12.7 only allows `id` and `version` on each message's root tag. A
  schema-location hint is therefore rejected there, though it is fine on a standalone OEM.
  Namespace declarations are still allowed, because XML doesn't treat them as attributes.
- 8.3.6 wants `id` and `version` as the last root attributes, but XML says attribute order has no
  meaning (XML 1.0, 3.1). We don't check the order.
- A byte-order mark before the XML declaration is accepted.
- Whitespace between XML elements may be written directly, as character references, or in CDATA.
  XML treats all three the same way
  ([XSD 1.0 3.4.4](https://www.w3.org/TR/xmlschema-1/#cvc-complex-type),
  [XML Infoset 2.6](https://www.w3.org/TR/xml-infoset/#infoitem.character)).

### Where sources conflict

- NDM/XML 3.6.1 says all XML text values, except comments (3.6.2), must be all upper or all
  lower case. ODM 7.5.2 lets free-text values use any case, and table 5-2's own `CLASSIFICATION`
  example ('Operator-proprietary data; secondary distribution not permitted') is mixed case. The
  schema leaves these fields as plain strings (ODM 8.13.5). So XML accepts mixed case, and only
  converting a book-listed value such as `Utc` to KVN fails.
- NDM/XML 4.3.4 says `xmlns:ndm` "must next be coded", but ODM 8.3.3 and example G-14 leave it
  out. We don't require it when reading, and we always write it. The books show the qualified
  root both with and without a prefix, so both are accepted.
- Table 5-3 says `START_TIME` and `STOP_TIME` bound the "ephemeris data and covariance data",
  but the book's own example G-14 has a covariance epoch (22:28) after its `STOP_TIME` (21:28).
  With the book in conflict, covariance epochs are not checked against the total span; state
  epochs still are. `oem_g14.xml` is the example as published.
- The schema lets `X_DDOT`, `Y_DDOT` and `Z_DDOT` appear independently, and it gives every
  covariance matrix its own comments. ODM 8.10.8 says XML follows the same rules as KVN, where
  accelerations are all or nothing (7.4.1.2) and covariance comments only open the section
  (7.8.9). We follow the schema for XML. Writing such data to KVN fails, because KVN can't
  represent it.

### Stricter than required

- **Epochs.** The schema's `epochType` also allows time-zone offsets like `+01:00`, years with
  more than four digits, an empty fraction, and a bare number. 7.5.10 only allows `Z`. We follow
  7.5.10 in both notations, like OPM. All message types share this rule, so relaxing it has to
  happen for all of them at once.
- **OEM 1.0** is read only, and some valid 1.0 files are rejected; see [OEM 1.0](#oem-10).
- **`INTERPOLATION_DEGREE`.** Table 5-3 only asks for "an integer value", while the schema wants
  a positive integer. We require a positive degree in both notations, so every degree can also be
  written as valid XML. In KVN the range is 1 to `i32::MAX` (7.5.4). XML degrees up to `u32::MAX`
  are read, but those above `i32::MAX` can't be written to KVN. XML degrees above `u32::MAX` are
  allowed by the schema but rejected.
- **`xsi:type`** is rejected. The schema allows it when it names the declared type, but we don't
  track types.
- **DTDs** are rejected. The books don't mention them, and accepting them would open the door to
  attacks that blow up a small file into huge amounts of memory.
- **Broken XML.** We reject a literal `]]>` in text, a namespace prefix bound to an empty
  name, the same schema-location hint given twice through different prefixes, and processing
  instructions with an empty or `xml` target, as [XML 1.0](https://www.w3.org/TR/xml/) and
  [XML Namespaces](https://www.w3.org/TR/xml-names/) require. `library::xml` tests these for OEM,
  OPM and CDM. We don't check everything, though: the XML parser can still accept an invalid
  name in a processing instruction or namespace declaration that we otherwise ignore.

### More lenient than the book

These rules are clear, but real OEM files commonly break them without losing information, so
reading accepts them. Writing still follows the book, so everything the library writes is valid.

- **Unterminated last line.** 7.3.7 terminates every line, including the last. Many files lack
  the final line ending, for example after editing. Reading accepts such a file; writing always
  ends with a line terminator.
- **More than 16 digits in a number.** 7.5.6 and 7.5.7b cap numbers at 16 digits, and 7.5.4
  limits integers to the signed 32-bit range. Many producers write the shortest spelling that
  reproduces a double exactly, which often has 17 digits (for example `-2757.3016318893897`).
  Reading accepts any number of digits, and integer-form values of any size, in ephemeris and
  covariance lines. The rest of the number grammar still applies: `1e3` and `12.5e3` are still
  rejected. Writing rounds to 16 significant digits, so the last digit of such a value can change
  when it is written back to KVN. `INTERPOLATION_DEGREE` keeps the 32-bit range, and epochs keep
  the 16-digit fraction cap.

### Conversion between notations

- A value whose 16-digit KVN spelling would round past the largest double is rejected when
  writing KVN, because it would read back as infinity.
- XML to KVN keeps the whole model except for whitespace KVN can't hold. Trailing blanks in
  comments are dropped (7.4.7), and so are leading and trailing blanks in text values. A line
  break inside an XML comment turns it into two KVN comments.

### Limits

- MET and MRT durations are checked for format and order only. What they are relative to comes
  from a comment or the ICD (3.2.3.2), and we don't read that.
- Frames, centers, originators and time systems are kept as given, not looked up in a registry.
  Whether a frame needs `REF_FRAME_EPOCH` depends on the frame or the agreement between the
  parties. We don't convert frames or time systems, interpolate, or check whether a covariance is
  physically sensible.
- The Python binding adds one OEM rule of its own: in the nine-column `state_vector_numpy` form,
  a NaN acceleration means "absent". An explicit XML NaN acceleration (8.13.4) is therefore lost
  when that array is assigned back; the `state_vector` records keep it. The binding is tested by
  `test_oem.py`, the shared binding tests, and the packaged-wheel check in
  `just package-python`.

## ICS rows (annex A2.5.3)

| Rows | Subject | Coverage |
| --- | --- | --- |
| 1–7 | Header | Version, creation date and originator are required. Comments, classification and message ID are optional. Order and edition rules are checked. Optional XML strings are kept exactly as written. |
| 8–16, 23 | Metadata | Identity, frame and time fields are required. Frame epoch and comments are optional. Block markers must be exact. Object and time system must match across segments. Book-listed time systems and frames must be single-case in KVN. Frame-specific rules depend on the agreement between the parties. |
| 17–20 | Time bounds | Total and useable times must be in order, and useable times must fall inside the total span (the table 5-3 reading of TOTAL and USEABLE). State records must fall inside the total span. Complete useable spans of consecutive segments may touch but not overlap. In 2.0, a useable start may not precede the previous segment's useable stop (see [OEM 2.0](#oem-20)). |
| 21–22 | Interpolation | A method requires a degree, in Rust, Python and both notations. The degree must be positive (see [Stricter than required](#stricter-than-required)). Whether there are enough records is not checked, because the book doesn't define it (see [Where the book is unclear](#where-the-book-is-unclear)). |
| 24–25 | Ephemeris | At least one state record per segment, epochs valid for the time system, position and velocity with optional acceleration, and fixed units. KVN lines must have exactly 7 or 10 fields. In XML each acceleration component is optional on its own. Unlike AEM, OEM doesn't require increasing epochs. |
| 26–31 | Covariance | Optional section, which may be empty (row 30). Each matrix has all 21 lower-triangle values, an epoch, an optional frame and comments. KVN rows must have the right number of values. Epochs increase; they are not bounded by the total span. XML units are optional. Python accepts a full 6×6 matrix and uses its lower triangle. |

## Scale

Parsing reads the whole message into memory, so memory grows with message size. Limit input size
before reading untrusted messages. The Rust writers stream their output, but they work on a
message already in memory. Python copies data between its objects and the Rust model, and NumPy
arrays are copies. The Rust parsing, validation, generation, and file I/O run with the GIL
released, so other Python threads keep running; building and reading Python objects still holds
it. `OemData.from_numpy` holds it throughout: it reads the caller's arrays, which another thread
could change, and at 200,000 states only about a third of its time is Rust-side conversion. In a local measurement two threads processed large OEMs about 1.65 times as fast as one; this
is informational, not a guarantee. The allocation tests check
how allocations grow from 10 to 1,000 records for KVN (states and covariance matrices, including
`COV_REF_FRAME`) and from 100 to 2,000 state records for XML. They don't set a peak-memory limit.

## Verification outcome

OEM 3.0 is verified on the Rust and Python surfaces: strict parsing, self-contained validation,
both generation notations, conversion, diagnostics, allocation budgets, and packaged artifacts
have received message-level review against CCSDS 502.0-B-3 and the OEM 3.0 schema, subject to
the exceptions and limits recorded above. "Verified" means that review, not unrestricted
standards compliance, exhaustive mutation of every editable value, or a throughput guarantee.

### OEM 2.0

OEM 2.0 is also **Verified**, against CCSDS 502.0-B-2 with Technical Corrigendum 1
([Silver Book](https://ccsds.org/Pubs/502x0b2s.pdf)) and `ndmxml-2.0.0-oem-2.0.xsd`. Its OEM
section (5.2) and syntax rules (section 6) match 3.0 except in these points:

- **Header.** Table 5-2 has no `CLASSIFICATION` or `MESSAGE_ID`, and neither has the schema's
  `ndmHeader`. Both are rejected in 2.0, when reading and writing.
- **Case.** 6.5.6 says every text value must be all upper or all lower case, including names
  such as `OBJECT_NAME`. 3.0 narrowed this to normative values (7.5.3), and a mixed-case name
  loses nothing, so we apply the 3.0 reading to 2.0 as well. This is looser than a clear 2.0 rule.
- **Special numbers.** A note to 6.5.5 says `NaN`, `-Inf`, `+Inf` and `-0` are "not supported in
  the ODM". KVN numbers are already finite. We still read `-0` (producers often write `-0.000000`),
  and 2.0 XML still takes the schema's `xsd:double`, including `NaN` and `INF`. Both are looser
  than the note.
- **Useable order.** Corrigendum 1 adds to table 5-3: "The USEABLE_START_TIME time tag at a new
  block of ephemeris data must be greater than or equal to the USEABLE_STOP_TIME time tag of the
  previous block." 3.0 dropped the sentence. The rule names only those two values, so in 2.0 it
  applies whenever both are given, even when the other useable bounds are absent; a shared
  endpoint is allowed. The segments must therefore be in time order where both are given. When
  either is missing the rule is silent, and we don't substitute `START_TIME` or `STOP_TIME`,
  because 5.2.4.4 lets total spans overlap.

Everything else on this page applies unchanged, including the 5.2.4.4 rule that complete useable
spans of consecutive segments must not overlap. `conformance::odm2` checks every shipped OEM
example as 2.0: it must be valid against the 2.0 schema, round-trip through KVN and XML, and
reject both 3.0-only header fields and a useable start before the previous useable stop.

### OEM 1.0

OEM 1.0 ([502.0-B-1 Silver Book](https://ccsds.org/Pubs/502x0b1s.pdf), 2004) is read but not
written, and it is not verified. It is a KVN-only format, so XML with `version="1.0"` is
rejected. 1.0 has no `CLASSIFICATION`, `MESSAGE_ID`, `REF_FRAME_EPOCH`, accelerations or
covariance (502.0-B-2 annex E1, items 2, 4 and 11, and annex F2), so a 1.0 file carrying any of them is rejected.
Otherwise it is read with the 2.0 rules, which are stricter than 1.0 in a few places. These valid
1.0 files are therefore rejected:

- comments anywhere except between ephemeris lines (1.0 section 4.3.1d), for example between
  metadata keywords;
- Julian Date epochs such as `2451534.29812` (table 4-3), which 2.0 withdrew;
- mixed-case values of the book-listed time systems and frames, which 1.0 allows (4.3.3g).

1.0 also requires increasing, non-repeating ephemeris epochs and segments that don't overlap
(4.3.5c, table 4-3). We don't check either, as for 2.0 and 3.0.

Some tools read only 1.0. GMAT R2026a, for example, accepts OEM 2.0 only in its testing run mode
and rejects 3.0 (`CCSDSOEMReader::IsValidVersion`). Since the library doesn't write 1.0, such
tools can't read its output.
