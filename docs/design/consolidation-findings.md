# Consolidation findings

What was tried, what worked, and — more usefully — what was tried and rejected with reasons. The
rejections are here so they are not re-attempted from scratch; the code alone does not record why
an obvious-looking abstraction is absent.

## The standard

A consolidation earns its place when it removes a concrete maintenance burden, demonstrates that
behaviour is preserved, and leaves only the mechanism that paid for itself. Smaller is not the
goal, and abstraction added for its own sake is a cost.

## Accepted

**Shared scalar predicates.** `nonNegativeDouble` and `positiveDouble` had twelve independent
implementations across wrapper types and inline message checks; now two, `require_non_negative` and
`require_positive`. `GM`'s three correct implementations collapse onto `Gm::validate_value`.

Deliberately *not* extended to the closed-range validators — `Angle`, `Percentage`, `Probability`,
`LatitudeRequired`, `LongitudeRequired`, `ElementSetNo`. Their bounds and upper-bound inclusivity
differ per type, so a shared helper carries both as parameters: the per-type fact survives and only
the error-construction boilerplate collapses. That is line reduction without fact reduction.

**OPM KVN keyword ordering.**

The representative case is OPM's **KVN keyword ordering**, which was stated three times in
`messages/opm.rs` and coupled through opaque numbers:

- `rank`, a 58-entry keyword-to-number table whose gaps (17 to 20, 26 to 30, 34 to 40, 61 to 70,
  76 to 80) silently encoded block boundaries;
- `comment_starts_block`, written in rank literals and ranges — `matches!(previous, 17 | 26 |
  30..=34 | 61 | 76)` — which are the closing ranks of each block that may precede;
- `allows_non_increasing`, written in bare ranks `70`, `76`, `80`, `25`.

Adding a keyword to the spacecraft block meant widening `30..=34` in two other functions, with
nothing but a test to catch a mistake, and nothing in the source saying the range meant "the
closing ranks of the spacecraft block".

`OPM_KVN_BLOCKS` now declares the layout once — keywords in order, how many leading and trailing
keywords are optional, whether the block repeats, whether a repeat restarts a comment run, and
whether the whole block may be absent — and all three predicates derive from it. The ranks became
an implementation detail rather than a maintained fact.

**Verification.** The refactor was done in three steps: hoist the nested functions unchanged and
confirm the suite is green; add the declaration and derived predicates alongside; prove
equivalence exhaustively over every keyword and every reachable rank before switching the parser
over. The original hand-written functions are retained in `#[cfg(test)]` scope as the golden
reference, so the equivalence proof is permanent. That is deliberately a second copy, in test
scope only: any future edit to the declaration must be shown not to change recorded behaviour.

**The model was wrong twice, and the equivalence test caught both**, which is the argument for
doing this as a checked refactor rather than a rewrite:

1. Treating the header as one block made `CLASSIFICATION` fail to open a comment run. The version
   keyword anchors the document and is its own group; the header content follows it.
2. "Repeatable" conflated two facts. Maneuvers repeat *and* a comment may open a fresh run;
   `USER_DEFINED_*` repeats *within* one logical block, so a comment there does not start a new
   one. These needed separate flags.

A third fact was missing entirely: a mandatory block between two others blocks the path, which is
why the header cannot immediately precede the metadata. Without it the derived predicate was more
permissive than the original.

**What this does not cover.** Only the KVN ordering strand. The XML sequence tables state the same
element order again, but per parent and nested, so they do not share this flat structure; the KVN
parser's match arms and the writers state it a third and fourth time. Whether those can share the
declaration is a separate experiment, and this pilot is not evidence that they can.

## Rejected, with reasons

### A macro for controlled-vocabulary enums

The intended pilot was a macro generating `Display`, `FromStr`, and the serde renames from one
variant list, which would remove the current three-way restatement of every wire spelling and
derive the `expected` diagnostic instead of hand-maintaining it.

Auditing the 33 hand-rolled `FromStr` impls first showed why the naive form is wrong. Nine
deliberately accept aliases their `expected` list does not advertise, and the leniency is
load-bearing:

- `DisintegrationType` accepts `MASS-LOSS + BREAKUP`, which no XSD permits, because the published
  example `data/xml/rdm_c4.xml` uses that spelling. Generation always emits the canonical
  `BREAK-UP`, so the non-conforming spelling is normalised rather than propagated.
- `TdmReferenceFrame` accepts `ITRF1993` and `TOD_EARTH` because those are the *book's* spellings,
  while `ITRF-93` and `TOD` are the *XSD's*. Both authorities are legitimate.

A macro deriving `expected` from the accepted spellings would advertise a non-conforming spelling
as valid; one deriving it from canonical spellings only would silently drop the alias arm. The
distinction between "spellings to advertise" and "spellings to accept" is a real per-enum decision
that no mechanical rule reproduces. The macro was therefore not written, and the 22 alias-free
enums are not worth a macro on their own once the 9 must stay hand-written — the value was in
uniformity, and uniformity is unreachable.

The audit paid for itself anyway: `TdmReferenceFrame`'s diagnostic listed only the XSD spellings,
so a reader of the book who mistyped a frame was told to use vocabulary their authority does not
contain. It now lists both. `DisintegrationType`'s alias now records why it exists.

**Two Python binding defects surfaced from the same thread**, both fixed with regression tests:

- `OcmTrajState.__init__` documented and accepted `traj_basis` and `orb_revnum_basis`, then
  silently discarded both, on the false premise that the enums lacked `FromStr`. Caller data
  vanished with no error. `audit_bindings.py` passes on this because the fields *are* exposed —
  the audit checks presence, not that a constructor argument reaches the model.
- Seven enum getters formatted with Rust's `Debug`, so `orb_revnum_basis` read back as `'One'`
  when the wire value is `1`, and assigning a property's own value straight back raised. They now
  use `Display`, i.e. the wire spelling, and round-trip.

### Generating the Python wrapper layer

The wrapper layer is ~27,600 lines and 921 setters across 17 files. Normalising every setter body
gives **172 distinct shapes**, and the top eight cover only 545 of the 921. That distribution is
the answer to whether the layer can be generated or collapsed: the long tail is 164 shapes over
~376 setters, each encoding something specific about its field. A generator would have to
reproduce that tail or silently change behaviour, and the two defects in "Pilot B" show that
behaviour changes in this layer are not obvious from reading it. No generator was written.

What the survey was good for was finding where the layer disagrees with itself.

**Completing the assignment-time alignment.** The earlier fix covered only the nine
`api::checked_optional` call sites. The same divergence existed in **47 more setters** using a
hand-written `T::new(x).map_err(...)` idiom — `Angle` (19), `LongitudeRequired` (7),
`LatitudeRequired` (7), `Probability` (5), `PercentageRequired` (3), `DayInterval` (2), `Gm` (2),
`Percentage` (1), `AltitudeRequired` (1). Forty-six were aligned to the contract. Each was then
verified through Python to be rejected by `validate`, KVN, and XML, because loosening a setter is
only safe when the root already catches the value.

`nominal_impact_alt` was **deliberately left eager**. `AltitudeRequired` enforces the XSD's
Earth-derived `[-430.5, 8848]`, and RDM's root does not validate it at all — that is the open
book/XSD representation conflict. Its setter is currently the only enforcement, so aligning it
would open a hole rather than close an inconsistency. It should be aligned once the representation
decision is made, not before.

**Two more root-reachability defects, found by the loosening.** Removing the eager setters exposed
which fields had no other guard. APM's `EulerAngleState`, `AngVelState`, and `InertiaState` had no
`Validate` implementation at all and were never reached from `ApmData::validate`, which visited
only the quaternion and spin blocks. An Euler angle of 400 degrees passed `validate`, KVN, and XML,
and `xmllint` rejected the emitted document on `[facet 'maxExclusive']`. All three now have
validators and are routed, with regressions covering the out-of-range, below-range, and NaN cases,
a later Euler angle, an infinite inertia moment, and the accepted `-360` boundary through the XSD
oracle.

This is the same lesson as Pilot C from the other direction: an eager wrapper setter can mask a
missing root validator indefinitely, because the surface most users touch appears to enforce the
rule.
