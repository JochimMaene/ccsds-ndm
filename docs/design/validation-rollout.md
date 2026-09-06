# Validation: outstanding work

The validation rollout is complete; this records only what is unresolved. The implementation
history is in git, the enforcement rules are in [the validation contract](validation-contract.md),
the per-family evidence is in [the conformance set](../conformance/), and what was tried and
rejected during consolidation is in [Consolidation findings](consolidation-findings.md).

**Complete.**

| # | Category | Evidence |
| --- | --- | --- |
| 1 | Shared RDM/CDM blocks | `OdParameters`, `RdmSpacecraftParameters`, `GroundImpactParameters`, routed from both roots |
| 2 | TDM metadata and observations | Both accepted editions; 503.0-B-1 checked directly and agrees with 503.0-B-2 |
| 3 | ADM repeated states and block values | AEM's nine choices, APM's spin branches and Euler/angular-velocity/inertia blocks, ACM physical and maneuver values |
| 5 | OMM/OEM/OPM/CDM residual accounting | Root-mutation accounting per family; `ELEMENT_SET_NO`, CDM relative metadata, `Vec3Double`, RDM covariance closed |
| 6 | Python delegation | Eight routing shapes, one mutation each, all three surfaces agreeing with Rust |
| 7 | Conformance-claim reconciliation | 29 files audited; overclaims, dead references and duplicated policy corrected |

**Complete with recorded non-claims** — category 4 (OCM). Every block validator covers its own
editable values. `TIME_SPAN` is deliberately unvalidated (operands may carry different time
systems and the block supplies no context to compare them); `OEB_MAX/INT/MIN` ordering is
descriptive rather than a `shall`; `OEB_Q*` has no norm rule because `-999` is the tumbling flag.

**Outstanding.**

1. **Assess the XML, parser and writer duplication separately.** Pilot E settled the KVN ordering
   strand for OPM. The XML sequence tables state the same element order again, per parent and
   nested; the KVN parser match arms and the writers state it a third and fourth time. Whether any
   of those can share the declaration is a separate experiment, and Pilot E is not evidence that
   they can.
2. **Extend performance evidence where real workloads warrant it.** There is one reproducible
   Python baseline, on one machine, for one family, with no allocation counts. The Rust suite has
   allocation budgets for the history-bearing families; wall-clock evidence beyond OEM does not
   exist. Extend it where a workload actually matters rather than for coverage.
3. **Retire temporary machinery and state documents once they stop serving active work.** The
   migration-era duplicate of the OPM ordering implementation has already been removed and
   replaced with standard-derived cases. This document is itself in that category: once the
   outstanding items close, the Status section belongs in the conformance set and the pilot
   narrative belongs in history.
4. **Validate packaged artifacts before treating the result as release-ready.** `package-rust` and
   `package-python` both pass on this snapshot: the crate builds and its doctests run from a
   packaged checkout, and the `manylinux_2_34` abi3 wheel installs into a clean interpreter. That
   is a gate passed, not a release decision.

Consolidation pilots A (constrained scalar) and E (OPM KVN ordering) closed positively; B (enum
macro) and D (Python wrapper generation) closed as reasoned negatives; C found a reachability
defect rather than duplication. The standard Pilot E sets is the one to apply to the remaining
strands: remove a concrete maintenance burden, demonstrate preserved behaviour, and keep only the
mechanism that earned its place. Abstraction for its own sake is not the goal.

**Decided, not open.** TDM edition 1.0 (agrees with 2.0); RDM `NOMINAL_IMPACT_ALT`, OCM phase
angles and OCM `DAYS_SINCE_*_OBS` (P3 finiteness, P4 refusal); OCM `MAN_*` numeric columns (Orekit
parity); TDM 1.0 keyword leniency (accepted); OPM `MAN_DELTA_MASS` (resolved per edition, with
citations, in the contract);
assignment-time validation (setters defer to the root).


## Unresolved decisions

These are recorded where they are enforced, and listed here so they are not lost:

| Decision | Recorded in |
| --- | --- |
| OCM `TIME_SPAN` is not compared against `START_TIME`/`STOP_TIME`; the operands may carry different time systems | [ocm-3.0.md](../conformance/ocm-3.0.md) |
| RDM lifetime and re-entry epochs are not compared; RDM §§3.5.8–3.5.9 is advisory | [rdm-1.0.md](../conformance/rdm-1.0.md) |
| OCM `OEB_MAX/INT/MIN` ordering is descriptive, not a `shall`; `OEB_Q*` has no norm rule because `-999` is the tumbling flag | [ocm-3.0.md](../conformance/ocm-3.0.md) |
| TDM 1.0 input may carry 2.0-only keywords; accepted leniency rather than a per-edition keyword inventory | [tdm-2.0.md](../conformance/tdm-2.0.md) |
| The four book/XSD conflicts and their per-edition resolutions, with citations | [validation-contract.md](validation-contract.md) |

## Retiring this document

This file is itself temporary machinery. When the outstanding items close, the status table belongs
in the support matrix and this file should be deleted rather than maintained.
