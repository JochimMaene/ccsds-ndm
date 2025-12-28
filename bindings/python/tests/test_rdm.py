# SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for RDM (Re-entry Data Message) Python bindings.
"""

from pathlib import Path

import ccsds_ndm
import pytest
from ccsds_ndm import (
    AtmosphericReentryParameters,
    GroundImpactParameters,
    Rdm,
    RdmData,
    RdmHeader,
    RdmMetadata,
    RdmSegment,
    RdmSpacecraftParameters,
)

# =============================================================================
# Fixtures
# =============================================================================


@pytest.fixture
def rdm_kvn_path():
    return Path(__file__).parents[3] / "data" / "kvn" / "rdm_c1.kvn"


@pytest.fixture
def rdm_parsed(rdm_kvn_path):
    """Parse RDM using unified API from_str."""
    assert rdm_kvn_path.exists(), f"Test data file not found at {rdm_kvn_path}"
    kvn_content = rdm_kvn_path.read_text()
    result = ccsds_ndm.from_str(kvn_content)
    assert isinstance(result, Rdm)
    return result


# =============================================================================
# Construction Tests
# =============================================================================


class TestRdmConstruction:
    """Tests for constructing RDM objects from Python."""

    def test_construct_atmospheric_reentry_parameters(self):
        """Test constructing AtmosphericReentryParameters."""
        params = AtmosphericReentryParameters(
            orbit_lifetime=5.5,
            reentry_altitude=120.0,
            comment=["Test reentry parameters"],
        )
        assert params.orbit_lifetime == 5.5
        assert params.reentry_altitude == 120.0
        assert params.comment == ["Test reentry parameters"]
        assert "AtmosphericReentryParameters" in repr(params)

    def test_construct_ground_impact_parameters(self):
        """Test constructing GroundImpactParameters."""
        params = GroundImpactParameters(
            probability_of_impact=0.75,
            probability_of_burn_up=0.95,
            comment=["Test impact parameters"],
        )
        assert params.probability_of_impact == 0.75
        assert params.probability_of_burn_up == 0.95
        assert params.comment == ["Test impact parameters"]
        assert "GroundImpactParameters" in repr(params)

    def test_construct_spacecraft_parameters(self):
        """Test constructing RdmSpacecraftParameters."""
        params = RdmSpacecraftParameters(
            wet_mass=1000.0, dry_mass=800.0, comment=["Test spacecraft"]
        )
        assert params.wet_mass == 1000.0
        assert params.dry_mass == 800.0
        assert params.comment == ["Test spacecraft"]
        assert "RdmSpacecraftParameters" in repr(params)

    def test_construct_rdm_data(self):
        """Test constructing RdmData."""
        atm_params = AtmosphericReentryParameters(
            orbit_lifetime=3.0, reentry_altitude=100.0
        )
        ground_params = GroundImpactParameters(probability_of_impact=0.5)

        data = RdmData(
            atmospheric_reentry_parameters=atm_params,
            ground_impact_parameters=ground_params,
            comment=["Test RDM data"],
        )

        assert data.atmospheric_reentry_parameters.orbit_lifetime == 3.0
        assert data.ground_impact_parameters is not None
        assert data.ground_impact_parameters.probability_of_impact == 0.5
        assert "RdmData" in repr(data)

    def test_construct_rdm_metadata(self):
        """Test constructing RdmMetadata."""
        metadata = RdmMetadata(
            object_name="TIANGONG-1",
            international_designator="2011-053A",
            controlled_reentry="NO",
            center_name="EARTH",
            time_system="UTC",
            epoch_tzero="2018-03-01T00:00:00.000Z",
            comment=["Test metadata"],
        )

        assert metadata.object_name == "TIANGONG-1"
        assert metadata.international_designator == "2011-053A"
        assert (
            metadata.controlled_reentry == "YES" or metadata.controlled_reentry == "NO"
        )
        assert metadata.center_name == "EARTH"
        assert metadata.time_system == "UTC"
        assert "RdmMetadata" in repr(metadata)

    def test_construct_rdm_header(self):
        """Test constructing RdmHeader."""
        header = RdmHeader(
            originator="ESA/ESOC",
            creation_date="2023-01-15T10:30:00.000Z",
            message_id="RDM_001",
            comment=["Test header"],
        )

        assert header.originator == "ESA/ESOC"
        assert "2023-01-15" in header.creation_date
        assert header.message_id == "RDM_001"
        assert header.comment == ["Test header"]
        assert "RdmHeader" in repr(header)

    def test_construct_rdm_segment(self):
        """Test constructing RdmSegment."""
        metadata = RdmMetadata(
            object_name="TEST_OBJECT",
            international_designator="2020-001A",
            controlled_reentry="YES",
            center_name="EARTH",
            time_system="UTC",
            epoch_tzero="2023-06-01T00:00:00.000Z",
        )
        atm_params = AtmosphericReentryParameters(
            orbit_lifetime=10.0, reentry_altitude=80.0
        )
        data = RdmData(atmospheric_reentry_parameters=atm_params)

        segment = RdmSegment(metadata=metadata, data=data)

        assert segment.metadata.object_name == "TEST_OBJECT"
        assert segment.data.atmospheric_reentry_parameters.orbit_lifetime == 10.0
        assert "RdmSegment" in repr(segment)

    def test_construct_full_rdm(self):
        """Test constructing a complete RDM message."""
        # Header
        header = RdmHeader(
            originator="NASA/ODPO",
            creation_date="2023-01-01T00:00:00.000Z",
            message_id="RDM_TEST_001",
        )

        # Metadata
        metadata = RdmMetadata(
            object_name="STARLINK-1234",
            international_designator="2021-999Z",
            controlled_reentry="YES",
            center_name="EARTH",
            time_system="UTC",
            epoch_tzero="2023-06-15T12:00:00.000Z",
            comment=["Controlled de-orbit"],
        )

        # Atmospheric reentry parameters
        atm_params = AtmosphericReentryParameters(
            orbit_lifetime=2.5, reentry_altitude=120.0
        )

        # Ground impact parameters
        ground_params = GroundImpactParameters(
            probability_of_impact=0.85, probability_of_burn_up=0.99
        )

        # Data
        data = RdmData(
            atmospheric_reentry_parameters=atm_params,
            ground_impact_parameters=ground_params,
        )

        # Segment
        segment = RdmSegment(metadata=metadata, data=data)

        # Full RDM
        rdm = Rdm(header=header, segment=segment)

        # Verify structure
        assert rdm.header.originator == "NASA/ODPO"
        assert rdm.header.message_id == "RDM_TEST_001"
        assert rdm.segment.metadata.object_name == "STARLINK-1234"
        assert rdm.segment.data.atmospheric_reentry_parameters.orbit_lifetime == 2.5
        assert rdm.segment.data.ground_impact_parameters is not None
        assert "Rdm" in repr(rdm)


class TestRdmEndToEnd:
    """End-to-end Python workflow tests for RDM bindings."""

    def test_roundtrip_kvn_file(self, tmp_path, rdm_parsed: Rdm):
        out_path = tmp_path / "roundtrip.rdm.kvn"
        rdm_parsed.to_file(str(out_path), "kvn")
        reparsed = ccsds_ndm.from_file(str(out_path))
        assert isinstance(reparsed, Rdm)
        assert reparsed.header.originator == rdm_parsed.header.originator
        assert (
            reparsed.segment.metadata.time_system
            == rdm_parsed.segment.metadata.time_system
        )


# =============================================================================
# Getter/Setter Tests
# =============================================================================


class TestRdmGettersSetters:
    """Tests for RDM property getters and setters."""

    def test_header_setters(self):
        """Test setting header fields."""
        header = RdmHeader(
            originator="NASA",
            creation_date="2023-01-01T00:00:00.000Z",
            message_id="MSG_001",
        )

        header.originator = "ESA"
        assert header.originator == "ESA"

        header.message_id = "MSG_002"
        assert header.message_id == "MSG_002"

        header.comment = ["Updated comment"]
        assert header.comment == ["Updated comment"]

    def test_metadata_setters(self):
        """Test setting metadata fields."""
        metadata = RdmMetadata(
            object_name="TEST",
            international_designator="2020-001A",
            controlled_reentry="YES",
            center_name="EARTH",
            time_system="UTC",
            epoch_tzero="2023-01-01T00:00:00.000Z",
        )

        metadata.object_name = "NEW_NAME"
        assert metadata.object_name == "NEW_NAME"

        metadata.center_name = "MOON"
        assert metadata.center_name == "MOON"

        metadata.comment = ["New comment"]
        assert metadata.comment == ["New comment"]

    def test_atmospheric_params_setters(self):
        """Test setting atmospheric reentry parameters."""
        params = AtmosphericReentryParameters(
            orbit_lifetime=5.0, reentry_altitude=100.0
        )

        params.orbit_lifetime = 7.5
        assert params.orbit_lifetime == 7.5

        params.reentry_altitude = 150.0
        assert params.reentry_altitude == 150.0

    def test_ground_impact_params_setters(self):
        """Test setting ground impact parameters."""
        params = GroundImpactParameters(probability_of_impact=0.5)

        params.probability_of_impact = 0.75
        assert params.probability_of_impact == 0.75

        params.probability_of_burn_up = 0.90
        assert params.probability_of_burn_up == 0.90


# =============================================================================
# Validation Tests
# =============================================================================


class TestRdmValidation:
    """Tests for RDM validation."""

    def test_orbit_lifetime_must_be_positive(self):
        """Test that orbit_lifetime must be > 0."""
        with pytest.raises(ValueError):
            AtmosphericReentryParameters(
                orbit_lifetime=-1.0,  # Invalid: must be > 0
                reentry_altitude=100.0,
            )

    def test_invalid_format_in_to_str_raises(self, rdm_parsed: Rdm):
        """Invalid format argument should raise ValueError."""
        with pytest.raises(ValueError):
            rdm_parsed.to_str("json")

    def test_controlled_reentry_validation(self):
        """Test that controlled_reentry must be YES or NO."""
        with pytest.raises(ValueError):
            RdmMetadata(
                object_name="TEST",
                international_designator="2020-001A",
                controlled_reentry="MAYBE",  # Invalid
                center_name="EARTH",
                time_system="UTC",
                epoch_tzero="2023-01-01T00:00:00.000Z",
            )


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
