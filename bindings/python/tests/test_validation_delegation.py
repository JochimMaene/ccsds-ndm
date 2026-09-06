# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Python delegation evidence for the validation rollout.

Each test covers one *routing shape* — one way a constrained value is reached from a root — by
editing through the live Python object graph and asserting that ``validate``, KVN generation, and
XML generation all agree with the Rust core. This deliberately does not duplicate the Rust
mutation matrix for every field; it proves that reconstruction and delegation preserve the
enforcement boundary for each shape.

See ``docs/design/validation-contract.md`` and the per-family conformance documents.
"""

import pathlib

import ccsds_ndm
import pytest

KVN_DIR = pathlib.Path(__file__).resolve().parents[3] / "ccsds-ndm" / "data" / "kvn"


def load(name, cls):
    return cls.from_str((KVN_DIR / name).read_text(), format="kvn")


def assert_all_surfaces_reject(message, expected_field):
    """Rust rejects at validate, KVN, and XML; Python must agree on all three."""
    with pytest.raises(ccsds_ndm.NdmValidationError) as excinfo:
        message.validate()
    assert expected_field in str(excinfo.value), str(excinfo.value)

    with pytest.raises(ccsds_ndm.NdmValidationError):
        message.to_str(format="kvn")
    with pytest.raises(ccsds_ndm.NdmValidationError):
        message.to_str(format="xml")


class TestRoutingShapes:
    """One mutation per distinct route from a root to a constrained value."""

    def test_repeated_record_beyond_index_zero(self):
        """Root -> segment -> repeated record[1]. Index 0 is not the only reachable record."""
        oem = load("oem_g13.kvn", ccsds_ndm.Oem)
        assert len(oem.segments[0].data.state_vector) > 2
        oem.segments[0].data.state_vector[1].x = float("nan")
        assert_all_surfaces_reject(oem, "X")

    def test_repeated_segment_then_record_then_choice_branch(self):
        """The hardest route: segment[1] -> record[1] -> attitude choice -> constrained value."""
        aem = load("aem_g4.kvn", ccsds_ndm.Aem)
        assert len(aem.segments) > 1
        states = aem.segments[1].data.attitude_states
        assert len(states) > 1
        broken = list(states[1].values)
        broken[0] = 5.0  # destroys quaternion normalisation
        states[1].values = broken
        assert_all_surfaces_reject(aem, "Quaternion not normalized")

    def test_required_scalar_on_a_required_block(self):
        """CDM relative metadata is required, not optional, and its numbers must be finite.

        This shape previously disagreed across surfaces: the KVN writer rejected a non-finite
        value while validate and the XML writer accepted it and emitted the lexical ``inf``.
        """
        cdm = load("cdm_363.kvn", ccsds_ndm.Cdm)
        cdm.body.relative_metadata_data.miss_distance = float("inf")
        assert_all_surfaces_reject(cdm, "MISS_DISTANCE")

    def test_fixed_size_vector_component(self):
        """A fixed-length vector whose components are reached by list assignment."""
        acm = load("acm_g8.kvn", ccsds_ndm.Acm)
        assert len(acm.segment.data.phys.cp) == 3
        acm.segment.data.phys.cp = [0.04, float("nan"), -0.023]
        assert_all_surfaces_reject(acm, "CP")

    def test_unguarded_vector_type_inside_an_optional_block(self):
        """``Vec3Double`` has public components and a constructor that checks nothing, so the
        containing block validator is the only guard. Emitting ``inf 0 0`` fails the schema's
        ``vec3Double`` list type."""
        ocm = load("ocm_g18.kvn", ccsds_ndm.Ocm)
        ocm.segment.data.man[0].dc_ref_dir = [float("inf"), 0.0, 0.0]
        assert_all_surfaces_reject(ocm, "DC_REF_DIR")

    def test_cross_field_rule(self):
        """A rule over several fields, not a single leaf: each confidence interval is all-or-none."""
        rdm = load("rdm_c2.kvn", ccsds_ndm.Rdm)
        parameters = rdm.segment.data.ground_impact_parameters
        assert parameters.impact_1_confidence is None
        parameters.impact_1_confidence = 50.0  # populated without its start/stop bounds
        assert_all_surfaces_reject(rdm, "IMPACT_1")

    def test_edition_conditional_rule(self):
        """A rule that exists only for one edition. ODM 502.0-B-2 requires a negative
        MAN_DELTA_MASS, while the 3.0 schema's ``deltamassTypeZ`` also admits zero."""
        opm = load("opm_g2.kvn", ccsds_ndm.Opm)
        opm.segment.data.maneuver_parameters[0].man_delta_mass = 0.0
        opm.validate()  # accepted for the 3.0 edition the fixture declares

        opm.version = "2.0"
        assert_all_surfaces_reject(opm, "MAN_DELTA_MASS")

    def test_optional_block_constructed_from_scratch(self):
        """A block the fixture omits entirely must still be reached once the caller adds it."""
        ocm = load("ocm_g18.kvn", ccsds_ndm.Ocm)
        ocm.segment.data.od = ccsds_ndm.OcmOdParameters(
            od_id="OD-1",
            od_method="LEAST_SQUARES",
            od_epoch="2022-12-06T11:17:33",
        )
        ocm.segment.data.od.weighted_rms = float("nan")
        assert_all_surfaces_reject(ocm, "WEIGHTED_RMS")


class TestNotationRepresentability:
    """P4 rules: the model holds a book-valid value that one notation cannot express."""

    def test_nominal_impact_altitude_is_semantic_at_p3_and_representability_at_p4(self):
        """RDM states no range and permits non-Earth body-fixed frames; the common 4.0 XSD's
        altRange is Earth-derived. The value is preserved and the conversion is refused."""
        rdm = load("rdm_c2.kvn", ccsds_ndm.Rdm)
        parameters = rdm.segment.data.ground_impact_parameters
        parameters.impact_ref_frame = "ITRF2000"
        parameters.nominal_impact_lon = 10.0
        parameters.nominal_impact_lat = 20.0

        for outside in (9000.0, -431.0):
            parameters.nominal_impact_alt = outside
            assert (
                parameters.nominal_impact_alt == outside
            )  # value is preserved, not clamped
            rdm.validate()
            rdm.to_str(format="kvn")
            with pytest.raises(
                ccsds_ndm.NdmValidationError, match="NOMINAL_IMPACT_ALT"
            ):
                rdm.to_str(format="xml")

        parameters.nominal_impact_alt = float("nan")
        assert_all_surfaces_reject(rdm, "NOMINAL_IMPACT_ALT")

        for boundary in (-430.5, 8848.0):
            parameters.nominal_impact_alt = boundary
            rdm.validate()
            rdm.to_str(format="xml")


class TestAssignmentDefersToRootValidation:
    """Setters store what they are given; the root decides.

    The validation contract allows invalid intermediate models and puts enforcement at parsing and
    generation. Every Python setter follows that uniformly: assignment never raises on a value
    domain, and the same value is rejected at ``validate``, KVN, and XML. CDM's additional
    parameters and OCM's ``GM`` previously raised on assignment instead; they were aligned to the
    contract so the surface behaves the same way everywhere.
    """

    def test_setters_do_not_reject_on_assignment(self):
        ocm = load("ocm_g18.kvn", ccsds_ndm.Ocm)
        ocm.segment.data.phys.wet_mass = -1.0
        assert ocm.segment.data.phys.wet_mass == -1.0
        assert_all_surfaces_reject(ocm, "WET_MASS")

        acm = load("acm_g8.kvn", ccsds_ndm.Acm)
        acm.segment.data.phys.wet_mass = -1.0
        assert_all_surfaces_reject(acm, "WET_MASS")

    @pytest.mark.parametrize("segment", [0, 1])
    def test_cdm_additional_parameters_defer_to_root(self, segment):
        """These setters used to raise on assignment.

        One field is enough: all four `AdditionalParameters` setters are the same struct-literal
        assignment. Both segments are kept because the second one is a *routing* fact — the
        repeated container must be revisited — not another domain sample. The domains themselves
        belong to `types.rs::test_mass_validation` and the Rust CDM conformance mutation test.
        """
        cdm = load("cdm_363.kvn", ccsds_ndm.Cdm)
        parameters = cdm.body.segments[segment].data.additional_parameters
        parameters.mass = -5.0
        assert parameters.mass == -5.0  # stored, not rejected at assignment
        assert_all_surfaces_reject(cdm, "MASS")

    def test_cdm_additional_parameters_constructor_defers_to_root(self):
        parameters = ccsds_ndm.AdditionalParameters(
            area_pc=-5.0,
            area_drg=None,
            area_srp=None,
            mass=None,
            cd_area_over_mass=None,
            cr_area_over_mass=None,
            thrust_acceleration=None,
            sedr=None,
            comment=[],
        )
        assert parameters.area_pc == -5.0

        cdm = load("cdm_363.kvn", ccsds_ndm.Cdm)
        cdm.body.segments[0].data.additional_parameters = parameters
        assert_all_surfaces_reject(cdm, "AREA_PC")

    def test_ocm_gm_defers_to_root(self):
        """Zero is the interesting value: `gmType` is a positiveDouble, so it distinguishes this
        setter from the non-negative ones. The rest of the domain is `types.rs::test_gm_validation`.
        """
        ocm = load("ocm_g18.kvn", ccsds_ndm.Ocm)
        ocm.segment.data.pert.gm = 0.0
        assert ocm.segment.data.pert.gm == 0.0
        assert_all_surfaces_reject(ocm, "GM")


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
