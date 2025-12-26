# SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
#
# SPDX-License-Identifier: MPL-2.0

import time
from pathlib import Path

import ccsds_ndm
import pytest


@pytest.fixture
def oem_kvn():
    """Provides a parsed OEM object from the example KVN file using from_str."""
    data_path = Path(__file__).parent / "data" / "example3.oem"
    assert data_path.exists(), f"Test data file not found at {data_path}"
    kvn_content = data_path.read_text()

    start = time.perf_counter()
    result = ccsds_ndm.from_str(kvn_content)
    duration = time.perf_counter() - start
    print(f"\nfrom_str() took {duration:.6f} seconds")
    return result


def test_parse_and_roundtrip_file_kvn(tmp_path, oem_kvn: ccsds_ndm.Oem):
    """Parse KVN OEM and round-trip via to_file → from_file."""
    # --- Assertions on the parsed data ---
    assert oem_kvn is not None

    # Header assertions
    assert oem_kvn.header.originator == "NASA/JPL"
    assert oem_kvn.header.creation_date == "2019-11-04T17:22:31"

    # Body and segment assertions
    assert len(oem_kvn.segments) == 1
    segment = oem_kvn.segments[0]

    # Metadata assertions
    assert segment.metadata.object_name == "MARS GLOBAL SURVEYOR"
    assert segment.metadata.object_id == "1996-062A"
    assert segment.metadata.center_name == "MARS BARYCENTER"
    assert segment.metadata.ref_frame == "EME2000"
    assert segment.metadata.time_system == "UTC"
    assert segment.metadata.interpolation_degree == 7

    # Data assertions
    assert len(segment.data.state_vectors) == 4
    sv1 = segment.data.state_vectors[0]
    assert sv1.epoch == "2019-12-28T21:29:07.267"
    assert sv1.x == -2432.166
    assert sv1.y == -63.042
    assert sv1.z == 1742.754
    assert sv1.x_dot == 7.33702
    assert sv1.y_dot == -3.495867
    assert sv1.z_dot == -1.041945

    # Round-trip using file I/O with unified API
    out_path = tmp_path / "roundtrip.oem.kvn"
    oem_kvn.to_file(str(out_path), "kvn")
    oem_round_trip = ccsds_ndm.from_file(str(out_path))

    # Compare generated strings for equality
    original_str = oem_kvn.to_str("kvn")
    roundtrip_str = oem_round_trip.to_str("kvn")
    assert roundtrip_str == original_str


@pytest.fixture
def oem_with_cov_kvn():
    """Provides a parsed OEM object from the KVN file with covariance via from_str."""
    data_path = Path(__file__).parent / "data" / "oem_with_cov.kvn"
    assert data_path.exists(), f"Test data file not found at {data_path}"
    kvn_content = data_path.read_text()
    return ccsds_ndm.from_str(kvn_content)


def test_parse_oem_with_covariance(oem_with_cov_kvn: ccsds_ndm.Oem):
    """Tests parsing a KVN OEM file that includes covariance matrices."""
    assert oem_with_cov_kvn is not None
    assert len(oem_with_cov_kvn.segments) == 1
    segment = oem_with_cov_kvn.segments[0]
    data = segment.data

    # Assert that covariance matrices were parsed
    assert len(data.covariance_matrices) == 2

    # Assertions on the first covariance matrix
    cov1 = data.covariance_matrices[0]
    assert cov1.epoch == "2019-12-28T21:29:07.267"
    assert cov1.cov_ref_frame == "EME2000"
    assert cov1.cx_x == pytest.approx(3.3313494e-04)
    assert cov1.cy_x == pytest.approx(4.6189273e-04)
    assert cov1.cz_dot_z_dot == pytest.approx(6.2244443e-10)

    # Assertions on the second covariance matrix
    cov2 = data.covariance_matrices[1]
    assert cov2.epoch == "2019-12-29T21:00:00"
    assert cov2.cx_x == pytest.approx(3.4424505e-04)
    assert cov2.cy_x == pytest.approx(4.5078162e-04)
    assert cov2.cz_dot_z_dot == pytest.approx(6.2244443e-10)
