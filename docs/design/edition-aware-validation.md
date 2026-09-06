# Edition-aware validation

Status: design note. It records why the TDM legacy-edition question is open, what the crate
already does, what a mature implementation does, and what closing it would cost.

## The question

`ccsds-ndm` accepts more than one input edition for most families. From `versioning.rs`:

| Family | Input editions | Output editions |
| --- | --- | --- |
| OPM | 1.0, 2.0, 3.0 | 2.0, 3.0 |
| OEM | 1.0, 2.0, 3.0 | 2.0, 3.0 |
| OMM | 2.0, 3.0 | 2.0, 3.0 |
| OCM | 3.0 | 3.0 |
| AEM, APM, ACM | 1.0, 2.0 | 2.0 |
| TDM | 1.0, 2.0 | 2.0 |
| CDM, RDM | 1.0 | 1.0 |

Six families accept a legacy input edition. A rule whose *value domain* differs between two
accepted editions cannot be enforced edition-blind: the strict domain rejects valid legacy
documents, and the loose domain accepts what the current edition forbids.

## The crate already has the mechanism

`Validate::validate` is **not** edition-blind, contrary to what an earlier draft of the rollout
said. Three families already run edition-conditional semantic rules from inside their root
validator:

| Root | Call site | Rule |
| --- | --- | --- |
| `Opm::validate` | `messages/opm.rs:79` | `validate_opm_edition` |
| `Oem::validate` | `messages/oem.rs:122`, `:607` | `validate_oem_edition` |
| `Omm::validate` | `messages/omm.rs:326` | `validate_omm_edition` |

The shape is a per-family function in `versioning.rs` that returns early unless the message
carries the edition in question, then applies that edition's rules with full field paths:

```rust
pub(crate) fn validate_opm_edition(message: &Opm) -> Result<()> {
    if message.version != "2.0" {
        return Ok(());
    }
    validate_odm_2_header(&message.header)?;
    for (index, maneuver) in message.body.segment.data.maneuver_parameters.iter().enumerate() {
        if maneuver.man_delta_mass.value == 0.0 {
            return Err(ValidationError::OutOfRange {
                name: "MAN_DELTA_MASS".into(),
                value: "0".into(),
                expected: "< 0 for CCSDS 502.0-B-2".into(),
                line: None,
            }
            .at_path(format!("body.segment.data.maneuver_parameters[{index}].man_delta_mass"))
            .into());
        }
    }
    Ok(())
}
```

Note the direction: this applies a *stricter* rule to the *older* edition, because 502.0-B-2
required a negative `MAN_DELTA_MASS` while the 3.0 XSD's `deltamassTypeZ` also admits zero.
`validate_omm_edition` similarly requires `NORAD_CAT_ID`, `ELEMENT_SET_NO`, and `REV_AT_EPOCH`
for OMM 2.0 only. So "this edition has a different value domain" is already an expressible,
tested, precedented rule here.

[The validation contract](validation-contract.md) has been corrected accordingly: its P3
definition now reads as notation-neutral but *not* edition-neutral, because P3 already resolves
edition-specific semantics from the edition stored on the message.

Two limits of the current mechanism, both minor:

- The edition is a `String` compared with `==`, so rules are keyed by exact spelling rather than
  by an ordered edition value. A `version < 2.0` style rule is not directly expressible.
- Only the root has access, so a rule reached through a deep optional block has to be written as a
  traversal inside `versioning.rs` rather than in the block's own validator. The `OPM` maneuver
  loop above shows what that costs at one level of nesting.

Neither blocks the TDM case.

## The case that surfaced it — now resolved

Category 2 narrowed four TDM observation domains to their 503.0-B-2 prose values. `CCSDS_TDM_VERS`
of `1.0` is an accepted input edition, so those rules applied to parsed TDM 1.0 documents on the
strength of a 2.0 book alone.

503.0-B-1 was subsequently obtained and checked. **The editions agree**: it specifies the same
`-180.0 <= ANGLE_1 < 360.0` bound for both angles, the same "positive double precision" wording for
`TEMPERATURE` and `STEC`, and it contains no `RCS` keyword, so that rule is vacuous for 1.0 input
rather than conflicting. No edition-conditional rule was needed, and the cheapest path in the
recommendation below is the one that closed it.

The case is still worth keeping as the worked example, because the question was real and the answer
was not knowable from the bundled material.

## What other implementations do

**Orekit** does not enforce these ranges at all. `ObservationType` is an enum carrying units —
`ANGLE_1(Unit.DEGREE)`, `RCS(Units.M2)`, `STEC(Unit.TOTAL_ELECTRON_CONTENT_UNIT)`,
`TEMPERATURE(Unit.ONE)`. The javadoc quotes `-180.0 ≤ ANGLE_1 < 360.0`, but `rawToSI()` and
`siToRaw()` only convert units; there is no bounds check. `TdmMetadata.validate(version)` checks
that `PARTICIPANT_1` is present and nothing else. Orekit accepts `ANGLE_1 = -300`,
`TEMPERATURE = 0`, and negative `RCS` from either edition, and never faces the question.

Orekit does thread the format version through every logical block's validator —
`validate(double version)` throughout — and branches where editions differ, e.g. in `ApmData`:

```java
if (version < 2.0) {
    // quaternion block is mandatory in ADM V1
    if (quaternionBlock == null) { new ApmQuaternion(null).validate(version); }
} else {
    // at least one logical block is mandatory in ADM V2
    ...
}
```

That is a more thorough version of what `versioning.rs` does: the edition reaches every block
rather than only the root, and comparisons are ordered rather than string equality. It is the
shape to copy if the two limits listed above ever start to bite.

**egemenimre/ccsds-ndm** (Python) is generated from the NDM XSDs with xsdata, so its accepted
domain is the schema's by construction: `angleType` is `[-360, 360)`. Prose-only narrowing is not
represented.

On this rule we are stricter than both. That is consistent with the project's goal, but it means
there is no interop precedent to appeal to in either direction.

Evidence limits: the Orekit code was read from `develop`, not a tagged release, and its javadoc
range almost certainly cites 503.0-B-2, so it says nothing about 503.0-B-1.

## Recommendation

The cost of an edition-conditional fix is low — a `validate_tdm_edition` alongside the three that
exist — so the decision turns entirely on evidence, not on architecture.

1. **Obtain 503.0-B-1.** If the ranges match, the question closes with a provenance line and no
   code change at all. This remains the cheapest path by a wide margin.
2. If the editions genuinely differ, add `validate_tdm_edition` following the existing pattern:
   apply the `[-180, 360)` bound for `2.0` and the schema's `[-360, 360)` for `1.0`, with the
   observation index in the field path.
3. Until one of those happens, leave the rules as they are and keep the exposure recorded in the
   rollout. Guessing that 1.0 was looser is no better founded than the current assumption that it
   was the same, and the current assumption at least fails safe — it rejects rather than silently
   accepting.

What to avoid is an inline `if version == "1.0"` at the observation validator. Edition rules live
in `versioning.rs` in this crate; scattering them would lose the one property that makes the
current three maintainable.

## References

- [Validation contract](validation-contract.md), enforcement boundaries P1–P4
- [tdm-2.0.md](../conformance/tdm-2.0.md), which records that both accepted TDM editions are covered
- `ccsds-ndm/src/versioning.rs`, edition specs and the three existing edition validators
- Orekit `ObservationType.java`, `TdmMetadata.java`, `ApmData.java` (branch `develop`)
