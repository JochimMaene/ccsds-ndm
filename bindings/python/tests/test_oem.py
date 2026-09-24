# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Orbit Ephemeris Message (OEM) Python bindings.
"""

from pathlib import Path

import numpy as np
import pytest

from ccsds_ndm import (
    NdmValidationError,
    OdmHeader,
    Oem,
    OemCovarianceMatrix,
    OemData,
    OemMetadata,
    OemSegment,
    StateVectorAcc,
)

DATA_DIR = Path(__file__).resolve().parents[3] / "ccsds-ndm/data"


# Each OEM data section carries two parallel histories with the same accessor
# shape: the record list, the epochs view, and the NumPy view.
HISTORIES = {
    "state": ("state_vector", "state_vector_epochs", "state_vector_numpy"),
    "cov": ("covariance_matrix", "covariance_matrix_epochs", "covariance_matrix_numpy"),
}


class TestOem:
    """Tests for OEM bindings."""

    def test_ref_frame_epoch_is_validated_as_a_time_tag(self):
        # Like the other OEM epochs, the spelling parses and validation applies 7.5.10.
        metadata = OemMetadata(
            "SAT1",
            "2023-001A",
            "2023-01-01T00:00:00",
            "2023-01-01T01:00:00",
            center_name="EARTH",
            ref_frame="EME2000",
            time_system="UTC",
            ref_frame_epoch="123.5",
        )
        with pytest.raises(NdmValidationError, match="REF_FRAME_EPOCH"):
            metadata.validate()
        # Setters: `test_ref_frame_epoch_follows_the_time_system`.

    def test_contextual_epoch_validation_rejects_degenerate_values(self):
        metadata = self._create_valid_oem().segments[0].metadata
        metadata.start_time = "+"
        with pytest.raises(NdmValidationError, match="'START_TIME': '\\+'") as error:
            metadata.validate()
        assert error.value.field_path == "start_time"

        oem = self._create_valid_oem()
        oem.segments[0].data.state_vector[0].epoch = "2023-02-29T00:00:00"
        with pytest.raises(NdmValidationError, match="2023-02-29") as error:
            oem.to_str(format="xml")
        assert error.value.field_path == "body.segment[0].data.state_vector[0].epoch"

    def _create_valid_oem(self):
        header = OdmHeader("2023-01-01T00:00:00", "TEST", "UNCLASSIFIED", "ID", None)
        meta = OemMetadata(
            "SAT1",
            "2023-001A",
            "2023-01-01T00:00:00",
            "2023-01-01T01:00:00",
            center_name="EARTH",
            ref_frame="EME2000",
            time_system="UTC",
        )

        vec = StateVectorAcc(
            epoch="2023-01-01T00:00:00",
            x=7000.0,
            y=0.0,
            z=0.0,
            x_dot=0.0,
            y_dot=7.5,
            z_dot=0.0,
            x_ddot=None,
            y_ddot=None,
            z_ddot=None,
        )

        # OemCovarianceMatrix construction
        # Using 21 floats for 6x6 lower triangle
        cov_args = np.array([1.0] * 21, dtype=float)
        cov = OemCovarianceMatrix("2023-01-01T00:00:00", cov_args, "EME2000", [])

        data = OemData(state_vectors=[vec], comment=None)
        data.covariance_matrix = [cov]

        seg = OemSegment(meta, data)
        return Oem(header, [seg])

    def test_oem_data_numpy_api(self):
        epochs = ["2023-01-01T00:00:00", "2023-01-01T00:01:00"]
        state = np.array(
            [
                [7000.0, 0.0, 0.0, 0.0, 7.5, 0.0],
                [7001.0, 0.1, 0.2, 0.0, 7.5, 0.0],
            ],
            dtype=float,
        )
        cov_epochs = ["2023-01-01T00:00:00"]
        cov = np.eye(6, dtype=float).reshape(1, 6, 6)

        data = OemData.from_numpy(
            state_vector_epochs=epochs,
            state_vector_numpy=state,
            covariance_matrix_epochs=cov_epochs,
            covariance_matrix_numpy=cov,
            comment=[],
        )

        assert data.state_vector_epochs == epochs
        assert data.state_vector_numpy.shape == (2, 6)
        assert data.covariance_matrix_epochs == cov_epochs
        assert data.covariance_matrix_numpy.shape == (1, 6, 6)

        new_state = state + 1.0
        data.state_vector_numpy = new_state
        assert np.allclose(data.state_vector_numpy, new_state)

    def test_epoch_setters_do_not_create_records(self):
        data = OemData(state_vectors=[], covariance_matrices=[], comment=[])
        with pytest.raises(ValueError, match="must match"):
            data.state_vector_epochs = ["2023-01-01T00:00:00"]
        with pytest.raises(ValueError, match="must match"):
            data.covariance_matrix_epochs = ["2023-01-01T00:00:00"]
        # Assigning an empty snapshot back to empty data is a no-op, not an error.
        data.state_vector_epochs = data.state_vector_epochs
        data.covariance_matrix_epochs = data.covariance_matrix_epochs
        data.state_vector_numpy = data.state_vector_numpy
        data.covariance_matrix_numpy = data.covariance_matrix_numpy
        assert data.state_vector == [] and data.covariance_matrix == []

    def _numpy_data(self):
        epochs = ["2023-01-01T00:00:00", "2023-01-01T00:01:00"]
        return OemData.from_numpy(
            state_vector_epochs=epochs,
            state_vector_numpy=np.array(
                [
                    [7000.0, 0.0, 0.0, 0.0, 7.5, 0.0],
                    [7001.0, 0.1, 0.2, 0.0, 7.5, 0.0],
                ],
                dtype=float,
            ),
            covariance_matrix_epochs=epochs,
            covariance_matrix_numpy=np.stack([np.eye(6), 2 * np.eye(6)]),
        )

    @pytest.mark.parametrize(("history", "other"), [("state", "cov"), ("cov", "state")])
    def test_accessors_read_only_their_own_history(self, history, other):
        # Both histories are exposed as plain Python lists, so either can be left
        # holding an object of the wrong type. Reading one history used to rebuild
        # the whole data section, which let an unrelated malformed record in the
        # other history break access.
        records, epochs, array = HISTORIES[history]
        data = self._numpy_data()
        getattr(data, HISTORIES[other][0]).append("not a record")
        assert len(getattr(data, epochs)) == 2
        replacement = getattr(data, array) * 2
        record = getattr(data, records)[0]
        setattr(data, array, replacement)
        assert getattr(data, records)[0] is record
        np.testing.assert_array_equal(getattr(data, array), replacement)

        getattr(data, records).append("not a record")
        for accessor in (array, epochs):
            with pytest.raises(TypeError, match=rf"{records}\[2\] must be"):
                getattr(data, accessor)

    def test_covariance_numpy_assignment_keeps_record_metadata(self):
        data = self._numpy_data()
        record = data.covariance_matrix[0]
        record.cov_ref_frame = "RTN"
        record.comment = ["retained"]
        data.covariance_matrix_numpy = data.covariance_matrix_numpy * 2
        assert record.cov_ref_frame == "RTN"
        assert record.comment == ["retained"]
        assert record.cx_x == 2.0

    @pytest.mark.parametrize("history", ["state", "cov"])
    def test_epoch_setters_reject_bad_input_without_partial_writes(self, history):
        records, epochs, _ = HISTORIES[history]
        data = self._numpy_data()
        original = getattr(data, records)[0].epoch

        with pytest.raises(ValueError):
            setattr(data, epochs, ["2024-01-01T00:00:00", "not-a-timestamp"])
        assert getattr(data, records)[0].epoch == original

        getattr(data, records)[1] = "not a record"
        with pytest.raises(TypeError, match=rf"{records}\[1\] must be"):
            setattr(data, epochs, ["2024-01-01T00:00:00", "2024-01-01T00:01:00"])
        assert getattr(data, records)[0].epoch == original

    @pytest.mark.parametrize("history", ["state", "cov"])
    def test_numpy_setters_reject_bad_input_without_partial_writes(self, history):
        records, _, array = HISTORIES[history]
        data = self._numpy_data()
        before = getattr(data, array)
        second = getattr(data, records)[1]

        getattr(data, records)[1] = "not a record"
        with pytest.raises(TypeError, match=rf"{records}\[1\] must be"):
            setattr(data, array, before + 1)
        getattr(data, records)[1] = second
        np.testing.assert_array_equal(getattr(data, array), before)

    @pytest.mark.parametrize(
        "metadata", [{"cov_ref_frames": ["RTN"]}, {"cov_comments": [["retain me"]]}]
    )
    def test_numpy_rejects_orphan_covariance_metadata(self, metadata):
        with pytest.raises(ValueError, match="covariance_matrix_epochs is required"):
            OemData.from_numpy(
                state_vector_epochs=["2023-01-01T00:00:00"],
                state_vector_numpy=np.zeros((1, 6)),
                **metadata,
            )

    @pytest.mark.parametrize("shape", [(21,), (1, 21), (6, 6), (1, 6, 6)])
    def test_numpy_covariance_layouts_preserve_every_component(self, shape):
        lower = np.arange(1.0, 22.0)
        expected = np.zeros((6, 6))
        expected[np.tril_indices(6)] = lower
        expected += np.tril(expected, -1).T
        values = (lower if shape[-1] == 21 else expected).reshape(shape)
        # Exercise each shape as a strided view as well.
        padded = np.repeat(values, 2, axis=-1)
        values = padded[..., ::2]
        assert not values.flags.c_contiguous
        data = OemData.from_numpy(
            state_vector_epochs=["2023-01-01T00:00:00"],
            state_vector_numpy=np.zeros((1, 6)),
            covariance_matrix_epochs=["2023-01-01T00:00:00"],
            covariance_matrix_numpy=values,
        )
        np.testing.assert_array_equal(data.covariance_matrix_numpy[0], expected)
        data.covariance_matrix_numpy = (padded * 2)[..., ::2]
        np.testing.assert_array_equal(data.covariance_matrix_numpy[0], expected * 2)

    def test_mixed_accelerations_produce_nine_columns_with_nan_gaps(self):
        # The array width is shared by the whole history, so a single record
        # carrying accelerations widens the array and the records without them
        # are padded with NaN rather than dropped or zero-filled.
        common = dict(x=7000.0, y=0.0, z=0.0, x_dot=0.0, y_dot=7.5, z_dot=0.0)
        without = StateVectorAcc(epoch="2023-01-01T00:00:00", **common)
        with_accel = StateVectorAcc(
            epoch="2023-01-01T00:01:00",
            x_ddot=1e-6,
            y_ddot=2e-6,
            z_ddot=3e-6,
            **common,
        )

        data = OemData(state_vectors=[without, with_accel], comment=None)
        array = data.state_vector_numpy

        assert array.shape == (2, 9)
        assert np.all(np.isnan(array[0, 6:]))
        assert np.allclose(array[1, 6:], [1e-6, 2e-6, 3e-6])
        # The six always-present columns must survive the widening unchanged.
        assert np.allclose(array[0, :6], [7000.0, 0.0, 0.0, 0.0, 7.5, 0.0])
        assert np.allclose(array[1, :6], [7000.0, 0.0, 0.0, 0.0, 7.5, 0.0])

    def test_acceleration_setters_edit_explicit_unit_input(self):
        # Both edit paths update an existing acceleration read with explicit XML
        # units; a six-column array removes the accelerations.
        xml = (DATA_DIR / "xml/oem_g14.xml").read_text()
        xml = xml.replace("<X_DDOT>", '<X_DDOT units="km/s**2">')
        oem = Oem.from_str(xml, "xml")
        data = oem.segments[0].data
        record = data.state_vector[0]
        record.x_ddot = 0.5
        data.state_vector_numpy = data.state_vector_numpy
        assert data.state_vector[0].x_ddot == 0.5
        # Each acceleration has one fixed unit, so the output omits the attribute.
        assert oem.to_str("xml").count("<X_DDOT>") == xml.count("<X_DDOT ")

        data.state_vector_numpy = data.state_vector_numpy[:, :6]
        assert all(state.x_ddot is None for state in data.state_vector)

    def test_numpy_inputs_accept_integer_arrays_and_lists(self):
        epochs = ["2023-01-01T00:00:00"]
        rows = [[7000, 0, 0, 0, 7, 0]]
        for state in (rows, np.array(rows)):
            data = OemData.from_numpy(epochs, state)
            assert data.state_vector_numpy.dtype == np.float64
            assert np.array_equal(data.state_vector_numpy, rows)
        data.state_vector_numpy = [[1, 2, 3, 4, 5, 6]]
        assert data.state_vector[0].x == 1.0
        data.covariance_matrix = [OemCovarianceMatrix(epochs[0], list(range(21)))]
        data.covariance_matrix_numpy = np.ones((1, 21), dtype=int)
        assert data.covariance_matrix[0].cz_dot_z_dot == 1.0

    def test_covariance_fields_follow_the_kvn_row_order(self):
        # ODM 5.2.5.4: the rows hold the lower triangle row by row. Read the raw
        # numbers from the file so the check does not depend on the bindings.
        kvn = (DATA_DIR / "kvn/oem_g13.kvn").read_text()
        block = kvn.split("COVARIANCE_START")[1].splitlines()
        rows = [line for line in block if line and "=" not in line][:6]
        values = [float(value) for row in rows for value in row.split()]
        matrix = Oem.from_str(kvn, "kvn").segments[0].data.covariance_matrix[0]
        names = [
            "cx_x", "cy_x", "cy_y", "cz_x", "cz_y", "cz_z",
            "cx_dot_x", "cx_dot_y", "cx_dot_z", "cx_dot_x_dot",
            "cy_dot_x", "cy_dot_y", "cy_dot_z", "cy_dot_x_dot", "cy_dot_y_dot",
            "cz_dot_x", "cz_dot_y", "cz_dot_z", "cz_dot_x_dot", "cz_dot_y_dot", "cz_dot_z_dot",
        ]  # fmt: skip
        assert [getattr(matrix, name) for name in names] == values
        full = Oem.from_str(kvn, "kvn").segments[0].data.covariance_matrix_numpy[0]
        assert full[np.tril_indices(6)].tolist() == values

    def test_empty_state_history_keeps_the_six_column_shape(self):
        data = OemData(state_vectors=[], comment=None)
        assert data.state_vector_numpy.shape == (0, 6)

    def test_full_covariance_inputs_read_the_lower_triangle(self):
        # Filter output is symmetric only to within rounding, so the upper triangle is ignored
        # rather than compared for equality.
        asymmetric = np.eye(6, dtype=float)
        asymmetric[0, 1] = 1.0

        epochs = ["2023-01-01T00:00:00"]
        state = np.zeros((1, 6), dtype=float)
        data = OemData.from_numpy(
            state_vector_epochs=epochs,
            state_vector_numpy=state,
            covariance_matrix_epochs=epochs,
            covariance_matrix_numpy=asymmetric.reshape(1, 6, 6),
        )
        assert np.allclose(data.covariance_matrix_numpy[0], np.eye(6, dtype=float))

        matrix = OemCovarianceMatrix("2023-01-01T00:00:00", asymmetric, None, [])
        assert matrix.cx_x == 1.0
        assert matrix.cy_x == 0.0

        nearly = np.eye(6, dtype=float)
        nearly[1, 0] = 1e-17
        data.covariance_matrix_numpy = nearly.reshape(1, 6, 6)
        assert np.allclose(data.covariance_matrix_numpy[0], np.eye(6, dtype=float))

    def test_interpolation_degree_zero_is_rejected_not_dropped(self):
        with pytest.raises(ValueError, match="positive integer"):
            OemMetadata(
                "SAT1",
                "2023-001A",
                "2023-01-01T00:00:00",
                "2023-01-01T01:00:00",
                center_name="EARTH",
                ref_frame="EME2000",
                time_system="UTC",
                interpolation="LINEAR",
                interpolation_degree=0,
            )

        meta = OemMetadata(
            "SAT1",
            "2023-001A",
            "2023-01-01T00:00:00",
            "2023-01-01T01:00:00",
            center_name="EARTH",
            ref_frame="EME2000",
            time_system="UTC",
            interpolation="LINEAR",
            interpolation_degree=5,
        )
        assert meta.interpolation_degree == 5
        with pytest.raises(ValueError, match="positive integer"):
            meta.interpolation_degree = 0
        assert meta.interpolation_degree == 5

        meta.interpolation_degree = None
        assert meta.interpolation_degree is None

        oem = self._create_valid_oem()
        oem.segments[0].metadata.interpolation = "LINEAR"
        oem.segments[0].metadata.interpolation_degree = 8
        kvn = oem.to_str(format="kvn")
        assert "INTERPOLATION_DEGREE" in kvn

    def test_strided_state_arrays_survive_construction_and_assignment(self):
        # Six or nine columns do not imply contiguous storage; strided views
        # such as array[:, ::2] must not raise PanicException.
        epochs = ["2023-01-01T00:00:00", "2023-01-01T00:01:00"]
        strided = np.arange(24, dtype=float).reshape(2, 12)[:, ::2]
        assert not strided.flags["C_CONTIGUOUS"]
        assert strided.shape == (2, 6)

        data = OemData.from_numpy(
            state_vector_epochs=list(epochs),
            state_vector_numpy=strided,
            comment=[],
        )
        assert np.allclose(data.state_vector_numpy, strided)

        # Slice the replacement from a wider base so the assigned view is
        # genuinely strided: slicing must come last, since arithmetic on a
        # view materializes a contiguous array and would leave the setter
        # path unprotected.
        replacement = (np.arange(48, dtype=float).reshape(2, 24) + 100.0)[:, ::4]
        assert not replacement.flags["C_CONTIGUOUS"]
        assert replacement.shape == (2, 6)
        data.state_vector_numpy = replacement
        assert np.allclose(data.state_vector_numpy, replacement)

        # Nine-column histories (with accelerations) take the same path.
        strided_nine = np.arange(36, dtype=float).reshape(2, 18)[:, ::2]
        assert not strided_nine.flags["C_CONTIGUOUS"]
        assert strided_nine.shape == (2, 9)
        data_nine = OemData.from_numpy(
            state_vector_epochs=list(epochs),
            state_vector_numpy=strided_nine,
            comment=[],
        )
        assert np.allclose(data_nine.state_vector_numpy, strided_nine)

    def test_section_validation_raises_ndm_validation_error(self):
        # Section-level validate() reports rule violations the same way the
        # message-level validate() does.
        oem = self._create_valid_oem()
        segment = oem.segments[0]
        state_vector = segment.data.state_vector
        segment.data.state_vector = []
        for section in (segment, segment.data):
            with pytest.raises(NdmValidationError, match="stateVector"):
                section.validate()
        segment.data.state_vector = state_vector

        segment.metadata.start_time = "+"
        for section in (segment, segment.metadata):
            with pytest.raises(NdmValidationError, match="START_TIME"):
                section.validate()

    def test_ref_frame_epoch_follows_the_time_system(self):
        # ODM 7.5.11 interprets REF_FRAME_EPOCH in TIME_SYSTEM; MET values are durations.
        oem = self._create_valid_oem()
        segment = oem.segments[0]
        segment.metadata.ref_frame_epoch = "0000-000T00:00:00"
        with pytest.raises(NdmValidationError, match="REF_FRAME_EPOCH"):
            oem.validate()
        segment.metadata.time_system = "MET"
        segment.metadata.start_time = "0000-000T00:00:00"
        segment.metadata.stop_time = "0000-000T01:00:00"
        segment.data.state_vector[0].epoch = "0000-000T00:00:00"
        segment.data.covariance_matrix[0].epoch = "0000-000T00:00:00"
        oem.validate()
        meta = OemMetadata(
            "SAT1",
            "2023-001A",
            "0000-000T00:00:00",
            "0000-000T01:00:00",
            center_name="EARTH",
            ref_frame="EME2000",
            time_system="MET",
            ref_frame_epoch="0000-000T00:00:00",
        )
        assert meta.ref_frame_epoch == "0000-000T00:00:00"
        meta.validate()

    def test_covariance_matrix_metadata_is_optional(self):
        matrix = OemCovarianceMatrix("2023-01-01T00:00:00", np.ones(21))
        assert matrix.cov_ref_frame is None
        assert matrix.comment == []

    @pytest.mark.parametrize("shape", [(1, 21), (1, 6, 6), (20,)])
    def test_covariance_matrix_takes_exactly_one_matrix(self, shape):
        with pytest.raises(ValueError, match=r"\(21,\) or \(6,6\)"):
            OemCovarianceMatrix("2023-01-01T00:00:00", np.ones(shape))


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
