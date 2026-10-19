# SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""The same public workflow for every output-supported family and edition."""

from pathlib import Path

import pytest

import ccsds_ndm

DATA = Path(__file__).resolve().parents[3] / "ccsds-ndm/data"
CASES = [
    (ccsds_ndm.Acm, "acm_g6.kvn", None),
    (ccsds_ndm.Aem, "aem_g4.kvn", None),
    (ccsds_ndm.Apm, "apm_g1.kvn", None),
    (ccsds_ndm.Cdm, "cdm_362.kvn", None),
    (ccsds_ndm.Ocm, "ocm_g15.kvn", None),
    (ccsds_ndm.Oem, "oem_g11.kvn", "2.0"),
    (ccsds_ndm.Oem, "oem_g11.kvn", "3.0"),
    (ccsds_ndm.Omm, "omm_g9.kvn", "2.0"),
    (ccsds_ndm.Omm, "omm_g9.kvn", "3.0"),
    (ccsds_ndm.Opm, "opm_g2.kvn", "2.0"),
    (ccsds_ndm.Opm, "opm_g2.kvn", "3.0"),
    (ccsds_ndm.Rdm, "rdm_c1.kvn", None),
    (ccsds_ndm.Tdm, "tdm_e1.kvn", None),
    (ccsds_ndm.CombinedNdm, "ndm_g12.xml", None),
]


@pytest.mark.parametrize(("cls", "fixture", "edition"), CASES)
def test_public_workflow_is_consistent_in_every_supported_notation(
    cls, fixture, edition, tmp_path
):
    source_format = fixture.rsplit(".", 1)[1]
    source = (DATA / source_format / fixture).read_text()
    if edition == "2.0":
        source = "\n".join(
            line for line in source.splitlines() if not line.startswith("MESSAGE_ID")
        )
        source = source.replace("_VERS = 3.0", "_VERS = 2.0", 1)
    original = cls.from_str(source, format=source_format)
    original.validate()
    formats = ("xml",) if cls is ccsds_ndm.CombinedNdm else ("kvn", "xml")
    for format in formats:
        text = original.to_str(format)
        # Canonical XML exposes all typed fields, units, order and comments. Compare it
        # after every route so the assertion is stronger than class identity or counts.
        expected = original.to_str("xml")
        for parsed in (
            cls.from_str(text, format=format),
            cls.from_str(text),
            ccsds_ndm.from_str(text, format=format),
            ccsds_ndm.from_str(text),
        ):
            assert isinstance(parsed, cls)
            parsed.validate()
            assert parsed.to_str("xml") == expected
        path = tmp_path / f"message.{format}"
        original.to_file(path, format)
        for parsed in (
            cls.from_file(path),
            cls.from_file(path, format=format),
            ccsds_ndm.from_file(path),
            ccsds_ndm.from_file(path, format=format),
        ):
            assert isinstance(parsed, cls)
            assert parsed.to_str("xml") == expected
        for target in formats:
            converted = ccsds_ndm.convert(text, target)
            assert cls.from_str(converted).to_str("xml") == expected
            destination = tmp_path / f"converted.{target}"
            ccsds_ndm.convert_file(path, destination, target)
            assert cls.from_file(destination).to_str("xml") == expected

    if cls is not ccsds_ndm.CombinedNdm:
        original.header.originator = ""
        with pytest.raises(ccsds_ndm.NdmValidationError):
            original.validate()
        for format in formats:
            with pytest.raises(ccsds_ndm.NdmValidationError):
                original.to_str(format)
            protected = tmp_path / f"protected.{format}"
            protected.write_text("existing content")
            with pytest.raises(ccsds_ndm.NdmValidationError):
                original.to_file(protected, format)
            assert protected.read_text() == "existing content"


@pytest.mark.parametrize(
    ("cls", "fixture"),
    [
        (ccsds_ndm.Acm, "acm_g6.kvn"),
        (ccsds_ndm.Aem, "aem_g4.kvn"),
        (ccsds_ndm.Apm, "apm_g1.kvn"),
        (ccsds_ndm.Tdm, "tdm_e1.kvn"),
    ],
)
def test_unverified_legacy_editions_fail_explicitly(cls, fixture):
    message = cls.from_str((DATA / "kvn" / fixture).read_text())
    for format in ("kvn", "xml"):
        source = message.to_str(format)
        legacy = (
            source.replace("= 2.0", "= 1.0", 1)
            if format == "kvn"
            else source.replace('version="2.0"', 'version="1.0"', 1)
        )
        assert legacy != source
        for parser in (cls.from_str, ccsds_ndm.from_str):
            with pytest.raises(ccsds_ndm.NdmError, match="Unsupported.*1.0"):
                parser(legacy, format=format)
