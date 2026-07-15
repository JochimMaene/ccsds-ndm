# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

from pathlib import Path

import pytest

import ccsds_ndm


REPOSITORY_ROOT = Path(__file__).parents[3]
OPM_KVN = (REPOSITORY_ROOT / "data/kvn/opm_g1.kvn").read_text()
OPM_XML = (REPOSITORY_ROOT / "data/xml/opm_g5.xml").read_text()
PERMISSIVE_XML = (REPOSITORY_ROOT / "data/xml/ndm_g22.xml").read_text()


def test_strict_parsing_is_the_default():
    with pytest.raises(ccsds_ndm.NdmValidationError):
        ccsds_ndm.from_str(PERMISSIVE_XML)


def test_permissive_parsing_emits_typed_warnings():
    with pytest.warns(ccsds_ndm.NdmParseWarning):
        message = ccsds_ndm.from_str(PERMISSIVE_XML, strict=False)

    assert isinstance(message, ccsds_ndm.Ndm)


def test_permissive_parsing_rejects_unsupported_versions():
    unsupported = OPM_XML.replace('version="3.0"', 'version="99.0"')
    with pytest.raises(ccsds_ndm.NdmValidationError):
        ccsds_ndm.Opm.from_str(unsupported, format="xml", strict=False)


def test_permissive_validation_returns_every_error():
    invalid = OPM_XML.replace(
        "<OBJECT_NAME>OSPREY 5</OBJECT_NAME>", "<OBJECT_NAME></OBJECT_NAME>"
    ).replace("<OBJECT_ID>2022-999A</OBJECT_ID>", "<OBJECT_ID></OBJECT_ID>")
    with pytest.warns(ccsds_ndm.NdmParseWarning):
        message = ccsds_ndm.Opm.from_str(invalid, format="xml", strict=False)

    errors = message.validate(strict=False)
    assert errors is not None
    assert len(errors) == 2


def test_generation_preserves_source_version_or_upgrades_explicitly():
    legacy = OPM_KVN.replace("3.0", "2.0", 1)
    message = ccsds_ndm.Opm.from_str(legacy, format="kvn")

    with pytest.raises(ccsds_ndm.NdmValidationError, match="output version 2.0"):
        message.to_kvn()

    upgraded = message.to_kvn(version="latest")
    assert upgraded.splitlines()[0].endswith("3.0")


def test_unchecked_generation_is_rejected():
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")

    with pytest.raises(ValueError, match="unchecked generation"):
        message.to_str("kvn", validate=False)


def test_unsupported_file_format_has_no_side_effect(tmp_path):
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")
    output = tmp_path / "output.ndm"

    with pytest.raises(ValueError, match="Unsupported format"):
        message.to_file(str(output), "json")

    assert not output.exists()


def test_failed_generation_preserves_existing_file(tmp_path):
    message = ccsds_ndm.Opm.from_str(OPM_KVN, format="kvn")
    output = tmp_path / "output.ndm"
    output.write_text("keep me")

    with pytest.raises(ccsds_ndm.NdmValidationError, match="output version 1.0"):
        message.to_file(str(output), "kvn", version="1.0")

    assert output.read_text() == "keep me"
