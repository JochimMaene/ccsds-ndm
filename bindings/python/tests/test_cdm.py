# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Conjunction Data Message (CDM) Python bindings.
"""

from pathlib import Path

import ccsds_ndm
import numpy as np
import pytest

from ccsds_ndm import (
    AdditionalParameters,
    Cdm,
    CdmBody,
    CdmCovarianceMatrix,
    CdmData,
    CdmHeader,
    CdmMetadata,
    CdmSegment,
    CdmStateVector,
    OdParameters,
    RelativeMetadataData,
    RelativeStateVector,
)


class TestCdm:
    """Tests for CDM bindings."""

    def _create_valid_cdm(self):
        header = CdmHeader(
            creation_date="2023-01-01T00:00:00",
            originator="TEST",
            message_id="ID",
            message_for="SATELLITE",
            comment=[],
        )

        rel_meta = RelativeMetadataData(
            tca="2023-01-01T12:00:00",
            miss_distance=100.0,
            collision_probability=0.001,
            collision_probability_method="FOSTER-1992",
            # Strict mode requires defaults or we assumed signature handles None?
            # RelativeMetadataData has pyo3 signature defaults, so this should be fine.
        )

        # Object 1
        meta1 = CdmMetadata(
            object="OBJECT1",
            object_designator="12345",
            catalog_name="SATCAT",
            object_name="SAT1",
            international_designator="2023-001A",
            ref_frame="EME2000",
            covariance_method="CALCULATED",
            maneuverable="YES",
        )
        vector1 = CdmStateVector(
            x=7000.0, y=0.0, z=0.0, x_dot=0.0, y_dot=7.5, z_dot=0.0
        )

        # CdmCovarianceMatrix: 21 mandatory floats (6x6 lower tri), 24 optional floats (7-9), comment
        cov_args = [
            # 6x6 Lower Triangle (21 elements)
            # R, T, N order usually? Rust struct fields: cr_r, ct_r, ct_t, ...
            # Row 1: cr_r
            1.0,
            # Row 2: ct_r, ct_t
            0.0,
            1.0,
            # Row 3: cn_r, cn_t, cn_n
            0.0,
            0.0,
            1.0,
            # Row 4: crdot_r... crdot_rdot
            0.0,
            0.0,
            0.0,
            0.01,
            # Row 5: ctdot_r... ctdot_tdot
            0.0,
            0.0,
            0.0,
            0.0,
            0.01,
            # Row 6: cndot_r... cndot_ndot
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.01,
            # Rows 7, 8, 9 (24 optionals) -> All None
            *[None] * 24,
            # Comment
            None,
        ]

        cov1 = CdmCovarianceMatrix(*cov_args)

        data1 = CdmData(
            state_vector=vector1,
            covariance_matrix=cov1,
            comment=[],  # Mandatory positional
        )
        seg1 = CdmSegment(metadata=meta1, data=data1)

        # Object 2
        meta2 = CdmMetadata(
            object="OBJECT2",
            object_designator="67890",
            catalog_name="SATCAT",
            object_name="DEBRIS",
            international_designator="UNKNOWN",
            ref_frame="EME2000",
            covariance_method="CALCULATED",
            maneuverable="YES",
        )
        vector2 = CdmStateVector(
            x=7100.0, y=0.0, z=0.0, x_dot=0.0, y_dot=7.4, z_dot=0.0
        )
        cov2 = CdmCovarianceMatrix(*cov_args)
        data2 = CdmData(state_vector=vector2, covariance_matrix=cov2, comment=[])
        seg2 = CdmSegment(metadata=meta2, data=data2)

        body = CdmBody(relative_metadata_data=rel_meta, segments=[seg1, seg2])

        return Cdm(header=header, body=body)

    def test_roundtrip_kvn(self):
        try:
            cdm = self._create_valid_cdm()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        kvn = cdm.to_str(format="kvn")
        assert "CCSDS_CDM_VERS" in kvn

        cdm2 = Cdm.from_str(kvn, format="kvn")
        assert cdm2.header.originator == "TEST"
        assert len(cdm2.body.segments) == 2

    def test_roundtrip_xml(self):
        try:
            cdm = self._create_valid_cdm()
        except TypeError as e:
            pytest.fail(f"Constructor failed: {e}")

        xml = cdm.to_str(format="xml")
        assert "<cdm" in xml

        cdm2 = Cdm.from_str(xml, format="xml")
        assert cdm2.header.originator == "TEST"

    def test_file_io(self, tmp_path):
        cdm = self._create_valid_cdm()
        kvn_path = tmp_path / "test.cdm"

        # Test to_file
        cdm.to_file(str(kvn_path), "kvn")
        assert kvn_path.exists()

        # Test from_file
        cdm2 = ccsds_ndm.from_file(str(kvn_path), format="kvn")
        assert cdm2.header.originator == "TEST"

    def test_cdm_data_numpy_api(self):
        state = np.array([7000.0, 0.0, 0.0, 0.0, 7.5, 0.0], dtype=float)
        cov = np.eye(6, dtype=float)

        data = CdmData.from_numpy(
            state_vector=state,
            covariance_matrix=cov,
            comment=[],
        )

        assert data.state_vector_numpy.shape == (6,)
        assert data.covariance_matrix_numpy.shape == (6, 6)
        assert np.allclose(data.state_vector_numpy, state)

        retained = data.state_vector
        retained.comment = ["Keep state provenance"]
        new_state = state + 1.0
        data.state_vector_numpy = new_state
        assert np.allclose(data.state_vector_numpy, new_state)
        assert data.state_vector is retained
        assert retained.comment == ["Keep state provenance"]

        # Allow clearing covariance matrix through numpy setter.
        data.covariance_matrix_numpy = None
        with pytest.raises(ValueError, match="COVARIANCE_MATRIX is missing"):
            _ = data.covariance_matrix_numpy

    def test_numpy_construction_keeps_optional_children_live(self):
        od = OdParameters(obs_available=10, obs_used=5)
        additional = AdditionalParameters(mass=100.0)
        data = CdmData.from_numpy(
            [7000, 0, 0, 0, 7.5, 0],
            np.eye(6),
            od_parameters=od,
            additional_parameters=additional,
        )
        assert data.od_parameters is od
        assert data.additional_parameters is additional
        od.obs_used = 6
        additional.mass = 250.0

        message = self._create_valid_cdm()
        message.body.segments[0].data = data
        for format in ("kvn", "xml"):
            written = Cdm.from_str(message.to_str(format))
            written_data = written.body.segments[0].data
            assert written_data.od_parameters.obs_used == 6
            assert written_data.additional_parameters.mass == 250.0

    def test_relative_state_edits_reach_generation(self):
        message = self._create_valid_cdm()
        message.body.relative_metadata_data = RelativeMetadataData(
            tca="2023-01-01T12:00:00",
            miss_distance=100.0,
            relative_position=[1, 2, 3],
            relative_velocity=[4, 5, 6],
        )
        for candidate in (message, Cdm.from_str(message.to_str("xml"))):
            relative = candidate.body.relative_metadata_data
            state = relative.relative_state_vector
            assert state is relative.relative_state_vector
            state.relative_position_r = 100.0
            written = Cdm.from_str(candidate.to_str("kvn"))
            assert (
                written.body.relative_metadata_data.relative_state_vector.relative_position_r
                == 100.0
            )

            replacement = RelativeStateVector(10, 20, 30, 40, 50, 60)
            relative.relative_state_vector = replacement
            assert relative.relative_state_vector is replacement
            replacement.relative_velocity_n = 70.0
            written = Cdm.from_str(candidate.to_str("xml"))
            assert (
                written.body.relative_metadata_data.relative_state_vector.relative_velocity_n
                == 70.0
            )
            relative.relative_state_vector = None
            assert relative.relative_state_vector is None

    @pytest.mark.parametrize("field", ["relative_position", "relative_velocity"])
    def test_relative_state_requires_position_and_velocity(self, field):
        with pytest.raises(ValueError, match="must be supplied together"):
            RelativeMetadataData(
                tca="2023-01-01T12:00:00", miss_distance=100.0, **{field: [1, 2, 3]}
            )

    def test_epoch_fields_reject_numeric_xsd_branch(self):
        with pytest.raises(ValueError):
            CdmHeader(
                creation_date="123.5",
                originator="TEST",
                message_id="ID",
            )

        with pytest.raises(ValueError):
            RelativeMetadataData(tca="123.5", miss_distance=100.0)

    def test_cdm_data_numpy_shape_validation(self):
        with pytest.raises(ValueError, match="State vector must be shape"):
            CdmData.from_numpy(
                state_vector=np.zeros((2, 6), dtype=float),
                covariance_matrix=None,
                comment=[],
            )

        data = CdmData.from_numpy(
            state_vector=np.array([7000.0, 0.0, 0.0, 0.0, 7.5, 0.0], dtype=float),
            covariance_matrix=None,
            comment=[],
        )
        with pytest.raises(ValueError, match="Covariance matrix must be"):
            data.covariance_matrix_numpy = np.zeros((5, 5), dtype=float)

    def test_partial_optional_covariance_row_is_rejected_without_panicking(self):
        cdm = self._create_valid_cdm()
        covariance = cdm.body.segments[0].data.covariance_matrix
        covariance.cdrg_r = 1.0

        with pytest.raises(ccsds_ndm.NdmValidationError, match="CDRG_T"):
            covariance.to_numpy()

    def test_relative_metadata_screen_volume_setters(self):
        rel = RelativeMetadataData(
            tca="2023-01-01T12:00:00",
            miss_distance=100.0,
        )
        assert rel.screen_volume_x is None
        assert rel.screen_volume_y is None
        assert rel.screen_volume_z is None

    def test_cdm_nested_setter_parity(self):
        cdm = self._create_valid_cdm()

        header = cdm.header
        header.originator = "UPDATED"
        cdm.header = header
        assert cdm.header.originator == "UPDATED"

        body = cdm.body
        rel = body.relative_metadata_data
        rel.miss_distance = 150.0
        body.relative_metadata_data = rel
        assert body.relative_metadata_data.miss_distance == pytest.approx(150.0)

        segments = body.segments
        seg0 = segments[0]
        meta0 = seg0.metadata
        meta0.object_name = "SAT1-UPDATED"
        seg0.metadata = meta0

        data0 = seg0.data
        sv0 = data0.state_vector
        sv0.x = 7050.0
        data0.state_vector = sv0
        data0.covariance_matrix = None
        seg0.data = data0

        segments[0] = seg0
        body.segments = segments
        cdm.body = body

        assert cdm.body.segments[0].metadata.object_name == "SAT1-UPDATED"
        assert cdm.body.segments[0].data.state_vector.x == pytest.approx(7050.0)
        assert cdm.body.segments[0].data.covariance_matrix is None

        rel.screen_volume_x = 10.0
        rel.screen_volume_y = 20.0
        rel.screen_volume_z = 30.0
        assert rel.screen_volume_x == 10.0
        assert rel.screen_volume_y == 20.0
        assert rel.screen_volume_z == 30.0

        rel.screen_volume_x = None
        rel.screen_volume_y = None
        rel.screen_volume_z = None
        assert rel.screen_volume_x is None
        assert rel.screen_volume_y is None
        assert rel.screen_volume_z is None


class TestCdmRealWorldCorpus:
    """An operational CDM that strict parsing rejects, kept as evidence.

    `test_cdm.xml` is an unedited CSpOC conjunction message. It spells its
    null-valued elements as a bare `nil="true"` attribute, 35 times, where the
    NDM XSD requires the namespaced `xsi:nil`. Strict parsing rejects it, and
    that rejection is the point: the document records what real operator output
    looks like and pins the cost of the strict-parsing decision, so the choice
    stays visible rather than being rediscovered against a live feed.
    """

    def test_operational_cdm_with_bare_nil_is_rejected(self):
        path = Path(__file__).resolve().parent / "test_cdm.xml"

        # An unused xmlns:xsi is optional, but a bare nil attribute is still invalid.
        with pytest.raises(ccsds_ndm.NdmFormatError, match="unknown attribute 'nil'"):
            ccsds_ndm.from_file(str(path), format="xml")

        # Adding the namespace declaration does not qualify the bare attribute.
        declared = path.read_text().replace(
            "<cdm ", '<cdm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" ', 1
        )
        with pytest.raises(ccsds_ndm.NdmFormatError) as excinfo:
            ccsds_ndm.from_str(declared, format="xml")
        assert "nil" in str(excinfo.value)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])


@pytest.mark.parametrize("missing", ["covariance_method", "maneuverable"])
def test_metadata_requires_operational_facts(missing):
    arguments = dict(
        object="OBJECT1",
        object_designator="12345",
        catalog_name="SATCAT",
        object_name="SAT1",
        international_designator="2023-001A",
        ref_frame="EME2000",
        covariance_method="CALCULATED",
        maneuverable="NO",
    )
    del arguments[missing]
    with pytest.raises(TypeError, match=missing):
        CdmMetadata(**arguments)


def test_metadata_preserves_explicit_operational_facts():
    metadata = CdmMetadata(
        object=ccsds_ndm.CdmObjectType.Object1,
        object_designator="12345",
        catalog_name="SATCAT",
        object_name="SAT1",
        international_designator="2023-001A",
        ref_frame=ccsds_ndm.ReferenceFrameType.Eme2000,
        covariance_method=ccsds_ndm.CovarianceMethodType.Default,
        maneuverable=ccsds_ndm.ManeuverableType.No,
    )
    assert metadata.covariance_method == ccsds_ndm.CovarianceMethodType.Default
    assert metadata.maneuverable == ccsds_ndm.ManeuverableType.No


@pytest.mark.parametrize("field", ["miss_distance", "relative_speed"])
def test_distance_and_speed_norms_reject_negative_scalars(field):
    message = TestCdm()._create_valid_cdm()
    relative = message.body.relative_metadata_data
    setattr(relative, field, -1.0)
    for operation in (
        message.validate,
        lambda: message.to_str("kvn"),
        lambda: message.to_str("xml"),
    ):
        with pytest.raises(ccsds_ndm.NdmValidationError, match=field.upper()):
            operation()
    setattr(relative, field, 0.0)
    message.validate()


def test_computed_cdm_values_follow_the_shared_numeric_rounding_policy():
    message = TestCdm()._create_valid_cdm()
    message.body.relative_metadata_data.miss_distance = 0.1 + 0.2
    message.validate()
    for notation, expected in [("kvn", 0.3), ("xml", 0.1 + 0.2)]:
        parsed = Cdm.from_str(message.to_str(notation), format=notation)
        assert parsed.body.relative_metadata_data.miss_distance == expected
