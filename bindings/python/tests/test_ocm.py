# SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for OCM (Orbit Comprehensive Message) Python bindings.
"""

import time
from pathlib import Path

import ccsds_ndm
import pytest
from ccsds_ndm import (
    Ocm,
    OcmData,
    OcmMetadata,
    OcmPhysicalDescription,
    OcmSegment,
    OcmTrajState,
    OdmHeader,
    TrajLine,
)

# =============================================================================
# Fixtures
# =============================================================================


@pytest.fixture
def ocm_kvn_path():
    """Path to the OCM KVN test file."""
    return Path(__file__).parent / "data" / "ocm_g15.kvn"


@pytest.fixture
def ocm_parsed(ocm_kvn_path):
    """Provides a parsed OCM object from the example KVN file."""
    assert ocm_kvn_path.exists(), f"Test data file not found at {ocm_kvn_path}"
    kvn_content = ocm_kvn_path.read_text()
    start = time.perf_counter()
    result = ccsds_ndm.from_str(kvn_content)
    duration = time.perf_counter() - start
    print(f"\nfrom_str() took {duration:.6f} seconds")
    return result


# =============================================================================
# Parsing Tests
# =============================================================================


class TestOcmParsing:
    """Tests for parsing OCM files."""

    def test_parse_returns_ocm_type(self, ocm_parsed):
        """Verify parse returns an Ocm instance."""
        assert ocm_parsed is not None
        assert isinstance(ocm_parsed, Ocm)

    def test_ocm_header_fields(self, ocm_parsed):
        """Test that header fields are correctly parsed."""
        header = ocm_parsed.header
        assert header.originator == "JAPAN AEROSPACE EXPLORATION AGENCY"
        assert header.creation_date == "2022-11-06T09:23:57"

    def test_ocm_metadata_time_system(self, ocm_parsed):
        """Test that metadata time_system is correctly parsed."""
        metadata = ocm_parsed.segment.metadata
        assert metadata.time_system == "UTC"

    def test_ocm_metadata_epoch_tzero(self, ocm_parsed):
        """Test that metadata epoch_tzero is correctly parsed."""
        metadata = ocm_parsed.segment.metadata
        assert metadata.epoch_tzero == "2022-12-18T14:28:15.1172"

    def test_ocm_trajectory_details(self, ocm_parsed):
        """Test trajectory state details."""
        data = ocm_parsed.segment.data
        traj_states = data.traj
        assert len(traj_states) == 1

        traj = traj_states[0]
        assert traj.center_name == "EARTH"
        assert traj.traj_ref_frame == "ITRF2000"
        assert traj.traj_type == "CARTPV"

        # Check trajectory lines
        lines = traj.traj_lines
        assert len(lines) == 4

        # First line
        assert lines[0].epoch == "0.0"
        assert len(lines[0].values) >= 6  # x, y, z, vx, vy, vz

        # Last line
        assert lines[3].epoch == "86400.0"


# =============================================================================
# Construction Tests
# =============================================================================


class TestOcmConstruction:
    """Tests for constructing OCM objects from Python."""

    def test_construct_traj_line(self):
        """Test constructing a TrajLine."""
        line = TrajLine(epoch="0.0", values=[1000.0, 2000.0, 3000.0, 1.0, 2.0, 3.0])
        assert line.epoch == "0.0"
        assert line.values == [1000.0, 2000.0, 3000.0, 1.0, 2.0, 3.0]
        assert "TrajLine" in repr(line)

    def test_construct_ocm_traj_state(self):
        """Test constructing an OcmTrajState."""
        lines = [
            TrajLine(epoch="0.0", values=[1000.0, 2000.0, 3000.0, 1.0, 2.0, 3.0]),
            TrajLine(epoch="60.0", values=[1100.0, 2100.0, 3100.0, 1.1, 2.1, 3.1]),
        ]

        traj = OcmTrajState(
            center_name="EARTH",
            traj_ref_frame="J2000",
            traj_type="CARTPV",
            traj_lines=lines,
            comment=["Test trajectory"],
        )

        assert traj.center_name == "EARTH"
        assert traj.traj_ref_frame == "J2000"
        assert traj.traj_type == "CARTPV"
        assert len(traj.traj_lines) == 2
        assert traj.comment == ["Test trajectory"]
        assert "OcmTrajState" in repr(traj)

    def test_construct_ocm_physical_description(self):
        """Test constructing an OcmPhysicalDescription."""
        phys = OcmPhysicalDescription(
            manufacturer="Test Manufacturer", comment=["Test physical description"]
        )

        assert phys.manufacturer == "Test Manufacturer"
        assert phys.comment == ["Test physical description"]
        assert "OcmPhysicalDescription" in repr(phys)

    def test_construct_ocm_metadata(self):
        """Test constructing OcmMetadata with required fields."""
        metadata = OcmMetadata(
            time_system="UTC",
            epoch_tzero="2023-01-01T00:00:00.000Z",
            object_name="TEST_SATELLITE",
        )

        assert metadata.time_system == "UTC"
        assert "2023-01-01" in metadata.epoch_tzero
        assert metadata.object_name == "TEST_SATELLITE"
        assert "OcmMetadata" in repr(metadata)

    def test_construct_ocm_data(self):
        """Test constructing OcmData with trajectory states."""
        # Create trajectory
        lines = [TrajLine(epoch="0.0", values=[1000.0, 2000.0, 3000.0, 1.0, 2.0, 3.0])]
        traj = OcmTrajState(
            center_name="EARTH",
            traj_ref_frame="J2000",
            traj_type="CARTPV",
            traj_lines=lines,
        )

        # Create data and set trajectory
        data = OcmData()
        data.traj = [traj]

        assert data.traj[0].center_name == "EARTH"

    def test_construct_ocm_segment(self):
        """Test constructing OcmSegment."""
        metadata = OcmMetadata(
            time_system="UTC", epoch_tzero="2023-01-01T00:00:00.000Z"
        )
        data = OcmData()

        segment = OcmSegment(metadata=metadata, data=data)

        assert segment.metadata.time_system == "UTC"
        assert "OcmSegment" in repr(segment)

    def test_construct_full_ocm(self):
        """Test constructing a complete OCM message."""
        # Header
        header = OdmHeader(
            originator="TEST_ORIGINATOR", creation_date="2023-01-01T00:00:00.000Z"
        )

        # Metadata
        metadata = OcmMetadata(
            time_system="UTC",
            epoch_tzero="2023-01-01T00:00:00.000Z",
            object_name="TEST_SATELLITE",
            international_designator="2023-001A",
        )

        # Data with trajectory
        lines = [
            TrajLine(epoch="0.0", values=[7000.0, 0.0, 0.0, 0.0, 7.5, 0.0]),
            TrajLine(epoch="600.0", values=[6000.0, 3000.0, 0.0, -1.0, 7.0, 0.0]),
        ]
        traj = OcmTrajState(
            center_name="EARTH",
            traj_ref_frame="ICRF",
            traj_type="CARTPV",
            traj_lines=lines,
            comment=["Test orbit data"],
        )

        data = OcmData()
        data.traj = [traj]

        # Segment
        segment = OcmSegment(metadata=metadata, data=data)

        # Full OCM
        ocm = Ocm(header=header, segment=segment)

        # Verify structure
        assert ocm.header.originator == "TEST_ORIGINATOR"
        assert ocm.segment.metadata.object_name == "TEST_SATELLITE"
        assert len(ocm.segment.data.traj[0].traj_lines) == 2
        assert "Ocm" in repr(ocm)


# =============================================================================
# Getter/Setter Tests
# =============================================================================


class TestOcmGettersSetters:
    """Tests for OCM property getters and setters."""

    def test_metadata_setters(self):
        """Test setting metadata fields after construction."""
        metadata = OcmMetadata(
            time_system="UTC", epoch_tzero="2023-01-01T00:00:00.000Z"
        )

        # Test all setter methods
        metadata.object_name = "NEW_NAME"
        assert metadata.object_name == "NEW_NAME"

        metadata.international_designator = "2023-999X"
        assert metadata.international_designator == "2023-999X"

        metadata.catalog_name = "NORAD"
        assert metadata.catalog_name == "NORAD"

        metadata.comment = ["Updated comment"]
        assert metadata.comment == ["Updated comment"]

    def test_traj_state_setters(self):
        """Test setting trajectory state fields."""
        lines = [TrajLine(epoch="0.0", values=[1.0, 2.0, 3.0])]
        traj = OcmTrajState(
            center_name="EARTH",
            traj_ref_frame="J2000",
            traj_type="CARTPV",
            traj_lines=lines,
        )

        traj.center_name = "MOON"
        assert traj.center_name == "MOON"

        traj.traj_ref_frame = "ICRF"
        assert traj.traj_ref_frame == "ICRF"

        traj.traj_type = "CARTPVA"
        assert traj.traj_type == "CARTPVA"

        new_lines = [TrajLine(epoch="100.0", values=[4.0, 5.0, 6.0])]
        traj.traj_lines = new_lines
        assert len(traj.traj_lines) == 1
        assert traj.traj_lines[0].epoch == "100.0"

    def test_traj_line_setters(self):
        """Test setting trajectory line fields."""
        line = TrajLine(epoch="0.0", values=[1.0, 2.0, 3.0])

        line.epoch = "123.456"
        assert line.epoch == "123.456"

        line.values = [10.0, 20.0, 30.0, 40.0]
        assert line.values == [10.0, 20.0, 30.0, 40.0]


# =============================================================================
# Data Helper Property Tests
# =============================================================================


class TestOcmDataHelpers:
    """Tests for OcmData helper properties."""

    def test_data_helper_properties(self):
        """Test OcmData count and has_* properties."""
        data = OcmData()

        # Initial state
        assert data.has_phys == False
        assert data.has_pert == False
        assert data.has_od == False

    def test_data_with_physical_description(self):
        """Test OcmData with physical description set."""
        data = OcmData()
        phys = OcmPhysicalDescription(manufacturer="Test")

        data.phys = phys

        assert data.has_phys == True
        assert data.phys.manufacturer == "Test"

        # Test clearing
        data.phys = None
        assert data.has_phys == False


# =============================================================================
# Round-trip Tests
# =============================================================================


class TestOcmRoundTrip:
    """Tests for OCM round-trip (parse → generate → parse)."""

    def test_kvn_round_trip(self, tmp_path, ocm_parsed):
        """Round-trip OCM via to_file/from_file and compare key fields."""
        out_path = tmp_path / "roundtrip.ocm.kvn"
        ocm_parsed.to_file(str(out_path), "kvn")
        reparsed = ccsds_ndm.from_file(str(out_path))

        # Compare key fields
        assert reparsed.header.originator == ocm_parsed.header.originator
        assert (
            reparsed.segment.metadata.time_system
            == ocm_parsed.segment.metadata.time_system
        )


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
