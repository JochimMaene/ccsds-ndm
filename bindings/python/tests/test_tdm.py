# SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for TDM (Tracking Data Message) Python bindings.
"""

from pathlib import Path

import ccsds_ndm
import pytest
from ccsds_ndm import (
    Tdm,
    TdmBody,
    TdmData,
    TdmHeader,
    TdmMetadata,
    TdmObservation,
    TdmSegment,
)

# =============================================================================
# Fixtures
# =============================================================================


@pytest.fixture
def tdm_kvn_path():
    """Path to a TDM KVN test file from repository data/kvn."""
    return Path(__file__).parents[3] / "data" / "kvn" / "tdm_e1.kvn"


@pytest.fixture
def tdm_parsed(tdm_kvn_path):
    """Provides a parsed TDM object from the example KVN file via from_str."""
    assert tdm_kvn_path.exists(), f"Test data file not found at {tdm_kvn_path}"
    kvn_content = tdm_kvn_path.read_text()
    result = ccsds_ndm.from_str(kvn_content)
    assert isinstance(result, Tdm)
    return result


# =============================================================================
# Construction Tests
# =============================================================================


class TestTdmConstruction:
    """Tests for constructing TDM objects from Python."""

    def test_construct_tdm_observation(self):
        """Test constructing a TdmObservation."""
        obs = TdmObservation(
            epoch="2023-01-01T12:00:00.000Z", keyword="RANGE", value=12345.678
        )
        assert obs.epoch == "2023-01-01T12:00:00.000Z"
        assert obs.keyword == "RANGE"
        assert obs.value == 12345.678
        assert "TdmObservation" in repr(obs)

    def test_construct_tdm_observation_doppler(self):
        """Test constructing a Doppler observation."""
        obs = TdmObservation(
            epoch="2023-01-01T12:00:00.000Z",
            keyword="DOPPLER_INSTANTANEOUS",
            value=-0.123,
        )
        assert obs.keyword == "DOPPLER_INSTANTANEOUS"
        assert obs.value == -0.123

    def test_construct_tdm_data(self):
        """Test constructing TdmData with observations."""
        obs1 = TdmObservation(
            epoch="2023-01-01T12:00:00.000Z", keyword="RANGE", value=12345.0
        )
        obs2 = TdmObservation(
            epoch="2023-01-01T12:01:00.000Z", keyword="RANGE", value=12346.0
        )

        data = TdmData(observations=[obs1, obs2], comment=["Test tracking data"])

        assert data.observation_count == 2
        assert len(data.observations) == 2
        assert data.comment == ["Test tracking data"]
        assert "TdmData" in repr(data)

    def test_construct_tdm_metadata(self):
        """Test constructing TdmMetadata with required fields."""
        metadata = TdmMetadata(
            time_system="UTC",
            participant_1="DSS-25",
            participant_2="MARS_GLOBAL_SURVEYOR",
            mode="SEQUENTIAL",
            path="1,2,1",
        )

        assert metadata.time_system == "UTC"
        assert metadata.participant_1 == "DSS-25"
        assert metadata.participant_2 == "MARS_GLOBAL_SURVEYOR"
        assert metadata.mode == "SEQUENTIAL"
        assert metadata.path == "1,2,1"
        assert "TdmMetadata" in repr(metadata)

    def test_construct_tdm_segment(self):
        """Test constructing TdmSegment."""
        metadata = TdmMetadata(time_system="UTC", participant_1="DSS-25")
        data = TdmData()

        segment = TdmSegment(metadata=metadata, data=data)

        assert segment.metadata.participant_1 == "DSS-25"
        assert segment.data.observation_count == 0
        assert "TdmSegment" in repr(segment)

    def test_construct_tdm_header(self):
        """Test constructing TdmHeader."""
        header = TdmHeader(
            originator="NASA/JPL",
            creation_date="2023-01-01T00:00:00.000Z",
            message_id="TDM_TEST_001",
        )

        assert header.originator == "NASA/JPL"
        assert "2023-01-01" in header.creation_date
        assert header.message_id == "TDM_TEST_001"
        assert "TdmHeader" in repr(header)

    def test_construct_tdm_body(self):
        """Test constructing TdmBody."""
        metadata = TdmMetadata(time_system="UTC", participant_1="DSS-25")
        data = TdmData()
        segment = TdmSegment(metadata=metadata, data=data)

        body = TdmBody(segments=[segment])

        assert len(body.segments) == 1
        assert "TdmBody" in repr(body)

    def test_construct_full_tdm(self):
        """Test constructing a complete TDM message."""
        # Header
        header = TdmHeader(
            originator="NASA/JPL", creation_date="2023-01-01T00:00:00.000Z"
        )

        # Metadata with tracking configuration
        metadata = TdmMetadata(
            time_system="UTC",
            participant_1="DSS-25",
            participant_2="MGS",
            mode="SEQUENTIAL",
            path="1,2,1",
            transmit_band="X",
            receive_band="X",
            range_units="km",
        )

        # Data with observations
        observations = [
            TdmObservation(
                epoch="2023-01-01T12:00:00.000Z", keyword="RANGE", value=123456789.0
            ),
            TdmObservation(
                epoch="2023-01-01T12:01:00.000Z", keyword="RANGE", value=123456790.0
            ),
            TdmObservation(
                epoch="2023-01-01T12:00:00.000Z",
                keyword="DOPPLER_INSTANTANEOUS",
                value=-0.0125,
            ),
        ]
        data = TdmData(observations=observations)

        # Segment
        segment = TdmSegment(metadata=metadata, data=data)

        # Body
        body = TdmBody(segments=[segment])

        # Full TDM
        tdm = Tdm(header=header, body=body)

        # Verify structure
        assert tdm.header.originator == "NASA/JPL"
        assert len(tdm.segments) == 1
        assert tdm.segments[0].metadata.participant_1 == "DSS-25"
        assert tdm.segments[0].data.observation_count == 3
        assert "Tdm" in repr(tdm)


class TestTdmEndToEnd:
    """End-to-end Python workflow tests for TDM bindings."""

    def test_roundtrip_kvn_file(self, tmp_path, tdm_parsed: Tdm):
        """Round-trip TDM via to_file/from_file and compare key fields."""
        out_path = tmp_path / "roundtrip.tdm.kvn"
        tdm_parsed.to_file(str(out_path), "kvn")
        reparsed = ccsds_ndm.from_file(str(out_path))
        assert isinstance(reparsed, Tdm)
        assert reparsed.header.originator == tdm_parsed.header.originator
        assert len(reparsed.segments) == len(tdm_parsed.segments)
        assert (
            reparsed.segments[0].metadata.time_system
            == tdm_parsed.segments[0].metadata.time_system
        )


# =============================================================================
# Getter/Setter Tests
# =============================================================================


class TestTdmGettersSetters:
    """Tests for TDM property getters and setters."""

    def test_header_setters(self):
        """Test setting header fields."""
        header = TdmHeader(
            originator="NASA/JPL", creation_date="2023-01-01T00:00:00.000Z"
        )

        header.originator = "ESA/ESOC"
        assert header.originator == "ESA/ESOC"

        header.message_id = "NEW_ID"
        assert header.message_id == "NEW_ID"

        header.comment = ["Updated comment"]
        assert header.comment == ["Updated comment"]

    def test_metadata_setters(self):
        """Test setting metadata fields."""
        metadata = TdmMetadata(time_system="UTC", participant_1="DSS-25")

        metadata.participant_2 = "MGS"
        assert metadata.participant_2 == "MGS"

        metadata.mode = "SIMULTANEOUS"
        assert metadata.mode == "SIMULTANEOUS"

        metadata.transmit_band = "S"
        assert metadata.transmit_band == "S"

        metadata.receive_band = "X"
        assert metadata.receive_band == "X"

    def test_observation_setters(self):
        """Test setting observation fields."""
        obs = TdmObservation(
            epoch="2023-01-01T12:00:00.000Z", keyword="RANGE", value=12345.0
        )

        obs.epoch = "2023-01-01T13:00:00.000Z"
        assert "2023-01-01" in obs.epoch

    def test_data_setters(self):
        """Test setting data fields."""
        data = TdmData()

        obs = TdmObservation(
            epoch="2023-01-01T12:00:00.000Z", keyword="RANGE", value=12345.0
        )
        data.observations = [obs]
        assert data.observation_count == 1

        data.comment = ["New comment"]
        assert data.comment == ["New comment"]


# =============================================================================
# Observation Keyword Tests
# =============================================================================


class TestTdmObservationKeywords:
    """Tests for different TDM observation keywords."""

    @pytest.mark.parametrize(
        "keyword",
        [
            "RANGE",
            "DOPPLER_INSTANTANEOUS",
            "DOPPLER_INTEGRATED",
            "CARRIER_POWER",
            "ANGLE_1",
            "ANGLE_2",
            "RECEIVE_FREQ",
            "TRANSMIT_FREQ_1",
            "TEMPERATURE",
            "PRESSURE",
            "TROPO_DRY",
            "TROPO_WET",
        ],
    )
    def test_observation_keywords(self, keyword):
        """Test that various observation keywords work."""
        obs = TdmObservation(
            epoch="2023-01-01T12:00:00.000Z", keyword=keyword, value=100.0
        )
        assert obs.keyword == keyword
        assert obs.value == 100.0

    def test_invalid_keyword_raises_error(self):
        """Test that invalid keyword raises error."""
        with pytest.raises(ValueError):
            TdmObservation(
                epoch="2023-01-01T12:00:00.000Z", keyword="INVALID_KEYWORD", value=100.0
            )


class TestTdmErrorPropagation:
    """Tests that errors from Rust propagate as Python exceptions."""

    def test_invalid_format_in_to_str_raises(self, tdm_parsed: Tdm):
        with pytest.raises(ValueError):
            # Invalid format should raise
            tdm_parsed.to_str("json")


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
