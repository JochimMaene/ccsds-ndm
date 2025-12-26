# SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
#
# SPDX-License-Identifier: MPL-2.0

import time
from pathlib import Path

import ccsds_ndm
import pytest


@pytest.fixture
def cdm_kvn():
    """Provides a parsed CDM object from the example KVN file using from_str."""
    data_path = Path(__file__).parent / "data" / "cdm_example_section4.kvn"
    assert data_path.exists(), f"Test data file not found at {data_path}"
    kvn_content = data_path.read_text()

    start = time.perf_counter()
    result = ccsds_ndm.from_str(kvn_content)
    duration = time.perf_counter() - start
    print(f"\nfrom_str() took {duration:.6f} seconds")
    return result


def test_parse_cdm_kvn(cdm_kvn):
    assert isinstance(cdm_kvn, ccsds_ndm.Cdm)
    assert cdm_kvn.header.originator == "JSPOC"
    assert cdm_kvn.body.relative_metadata_data.miss_distance == 715
    assert len(cdm_kvn.body.segments) == 2
    assert cdm_kvn.body.segments[0].metadata.object_name == "SATELLITE A"
    assert cdm_kvn.body.segments[1].metadata.object_name == "FENGYUN 1C DEB"


def test_cdm_roundtrip_kvn_file(tmp_path, cdm_kvn: ccsds_ndm.Cdm):
    """Round-trip CDM via to_file and from_file using KVN format."""
    out_path = tmp_path / "roundtrip.cdm.kvn"
    cdm_kvn.to_file(str(out_path), "kvn")
    cdm_round = ccsds_ndm.from_file(str(out_path))
    # Compare key fields to avoid comment/order diffs in serialization
    assert isinstance(cdm_round, ccsds_ndm.Cdm)
    assert cdm_round.header.originator == cdm_kvn.header.originator
    assert (
        cdm_round.body.relative_metadata_data.miss_distance
        == cdm_kvn.body.relative_metadata_data.miss_distance
    )
    assert len(cdm_round.body.segments) == len(cdm_kvn.body.segments)
    assert (
        cdm_round.body.segments[0].metadata.object_name
        == cdm_kvn.body.segments[0].metadata.object_name
    )
