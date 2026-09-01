# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""API consistency tests for class-level parse helpers."""

from pathlib import Path

import pytest

import ccsds_ndm
from ccsds_ndm import Acm, Aem, Apm, Cdm, Ndm, Ocm, Oem, Omm, Opm, Rdm, Tdm

ROOT = Path(__file__).resolve().parents[3]

CLASS_FIXTURES = [
    (Acm, ROOT / "ccsds-ndm/data/kvn/acm_g6.kvn"),
    (Aem, ROOT / "ccsds-ndm/data/kvn/aem_g4.kvn"),
    (Apm, ROOT / "ccsds-ndm/data/kvn/apm_g1.kvn"),
    (Cdm, ROOT / "ccsds-ndm/data/kvn/cdm_362.kvn"),
    (Ndm, ROOT / "ccsds-ndm/data/xml/ndm_g12.xml"),
    (Ocm, ROOT / "ccsds-ndm/data/kvn/ocm_g15.kvn"),
    (Oem, ROOT / "ccsds-ndm/data/kvn/oem_g11.kvn"),
    (Omm, ROOT / "ccsds-ndm/data/kvn/omm_g7.kvn"),
    (Opm, ROOT / "ccsds-ndm/data/kvn/opm_g1.kvn"),
    (Rdm, ROOT / "ccsds-ndm/data/kvn/rdm_c1.kvn"),
    (Tdm, ROOT / "ccsds-ndm/data/kvn/tdm_e1.kvn"),
]

LIVE_NESTED_CASES = [
    (Acm, ROOT / "ccsds-ndm/data/kvn/acm_g6.kvn", lambda value: value.segment.metadata),
    (
        Aem,
        ROOT / "ccsds-ndm/data/kvn/aem_g4.kvn",
        lambda value: value.segments[0].metadata,
    ),
    (Apm, ROOT / "ccsds-ndm/data/kvn/apm_g1.kvn", lambda value: value.segment.metadata),
    (
        Cdm,
        ROOT / "ccsds-ndm/data/kvn/cdm_362.kvn",
        lambda value: value.body.segments[0].metadata,
    ),
    (
        Ocm,
        ROOT / "ccsds-ndm/data/kvn/ocm_g15.kvn",
        lambda value: value.segment.metadata,
    ),
    (
        Oem,
        ROOT / "ccsds-ndm/data/kvn/oem_g12.kvn",
        lambda value: value.segments[0].metadata,
    ),
    (Omm, ROOT / "ccsds-ndm/data/kvn/omm_g7.kvn", lambda value: value.segment.metadata),
    (Opm, ROOT / "ccsds-ndm/data/kvn/opm_g1.kvn", lambda value: value.segment.metadata),
    (Rdm, ROOT / "ccsds-ndm/data/kvn/rdm_c1.kvn", lambda value: value.segment.metadata),
]

LIVE_LIST_CASES = [
    (Acm, ROOT / "ccsds-ndm/data/kvn/acm_g6.kvn", lambda value: value.segment.data.att),
    (Aem, ROOT / "ccsds-ndm/data/kvn/aem_g4.kvn", lambda value: value.segments),
    (
        Apm,
        ROOT / "ccsds-ndm/data/kvn/apm_g1.kvn",
        lambda value: value.segment.data.quaternion_state,
    ),
    (Cdm, ROOT / "ccsds-ndm/data/kvn/cdm_362.kvn", lambda value: value.body.segments),
    (Ndm, ROOT / "ccsds-ndm/data/xml/ndm_g12.xml", lambda value: value.messages),
    (
        Ocm,
        ROOT / "ccsds-ndm/data/kvn/ocm_g15.kvn",
        lambda value: value.segment.data.traj,
    ),
    (Oem, ROOT / "ccsds-ndm/data/kvn/oem_g11.kvn", lambda value: value.segments),
    (Tdm, ROOT / "ccsds-ndm/data/kvn/tdm_e1.kvn", lambda value: value.body.segments),
]


@pytest.mark.parametrize(("cls", "path"), CLASS_FIXTURES)
def test_type_specific_from_file_allows_default_format(cls, path):
    parsed = cls.from_file(str(path))
    assert isinstance(parsed, cls)


@pytest.mark.parametrize(("cls", "path"), CLASS_FIXTURES)
def test_type_specific_from_str_allows_default_format(cls, path):
    parsed = cls.from_str(path.read_text())
    assert isinstance(parsed, cls)


def test_public_exceptions_follow_python_error_categories():
    value_errors = [
        ccsds_ndm.NdmFormatError,
        ccsds_ndm.NdmKvnParseError,
        ccsds_ndm.NdmXmlError,
        ccsds_ndm.NdmValidationError,
        ccsds_ndm.NdmEpochError,
        ccsds_ndm.NdmUnsupportedMessageError,
    ]
    assert all(issubclass(exception, ValueError) for exception in value_errors)
    assert issubclass(ccsds_ndm.NdmIoError, OSError)
    assert not issubclass(ccsds_ndm.NdmIoError, ValueError)


@pytest.mark.parametrize(
    "name",
    ["RelativeStateVector", "SpacecraftParameters", "TleParameters"],
)
def test_registered_model_types_are_exported_from_the_package(name):
    assert name in ccsds_ndm.__all__
    assert getattr(ccsds_ndm, name) is getattr(ccsds_ndm.ccsds_ndm, name)


def test_nested_model_changes_are_live_without_an_editor():
    opm = Opm.from_file(str(ROOT / "ccsds-ndm/data/kvn/opm_g1.kvn"))
    assert opm.segment is opm.segment
    assert opm.segment.metadata is opm.segment.metadata

    opm.segment.metadata.object_name = "UPDATED"
    opm.segment.data.state_vector.x = 7000.0
    assert opm.segment.metadata.object_name == "UPDATED"
    assert opm.segment.data.state_vector.x == pytest.approx(7000.0)
    kvn = opm.to_str("kvn")
    assert any(
        line.startswith("OBJECT_NAME") and line.endswith("= UPDATED")
        for line in kvn.splitlines()
    )
    assert any(
        line.startswith("X ") and line.endswith("= 7000.0") for line in kvn.splitlines()
    )

    cdm = Cdm.from_file(str(ROOT / "ccsds-ndm/data/kvn/cdm_362.kvn"))
    retained_segment = cdm.body.segments[0]
    retained_segment.metadata.object_name = "OBJECT-1"
    assert cdm.body.segments[0].metadata.object_name == "OBJECT-1"
    assert cdm.body.segments[0] is retained_segment
    assert any(
        line.startswith("OBJECT_NAME") and line.endswith("= OBJECT-1")
        for line in cdm.to_str("kvn").splitlines()
    )

    combined = Ndm([opm])
    assert combined.messages[0] is opm
    assert isinstance(combined.messages[0], Opm)


def test_editor_api_is_not_part_of_the_public_surface():
    assert "edit" not in ccsds_ndm.__all__
    assert "Editor" not in ccsds_ndm.__all__
    assert not hasattr(ccsds_ndm, "edit")
    assert not hasattr(ccsds_ndm, "Editor")


@pytest.mark.parametrize(
    ("cls", "path", "get_nested"),
    LIVE_NESTED_CASES,
    ids=[case[0].__name__ for case in LIVE_NESTED_CASES],
)
def test_nested_identity_and_mutation_are_live_for_every_message_family(
    cls, path, get_nested
):
    message = cls.from_file(str(path))
    nested = get_nested(message)
    assert get_nested(message) is nested

    nested.object_name = "UPDATED"
    assert get_nested(message).object_name == "UPDATED"
    assert any(
        key.strip() == "OBJECT_NAME" and value.strip() == "UPDATED"
        for key, value in (
            line.split("=", 1)
            for line in message.to_str("kvn").splitlines()
            if "=" in line
        )
    )


def test_tdm_nested_identity_and_mutation_are_live():
    message = Tdm.from_file(str(ROOT / "ccsds-ndm/data/kvn/tdm_e1.kvn"))
    metadata = message.body.segments[0].metadata
    assert message.body.segments[0].metadata is metadata

    metadata.time_system = "TAI"
    assert message.body.segments[0].metadata.time_system == "TAI"
    assert any(
        key.strip() == "TIME_SYSTEM" and value.strip() == "TAI"
        for key, value in (
            line.split("=", 1)
            for line in message.to_str("kvn").splitlines()
            if "=" in line
        )
    )


@pytest.mark.parametrize(
    ("cls", "path", "get_records"),
    LIVE_LIST_CASES,
    ids=[case[0].__name__ for case in LIVE_LIST_CASES],
)
def test_repeated_model_fields_are_live_python_lists(cls, path, get_records):
    message = cls.from_file(str(path))
    records = get_records(message)
    assert isinstance(records, list)
    assert get_records(message) is records
    assert records

    retained = records.pop()
    records.append(retained)
    assert records[-1] is retained
    assert message.validate() is None


def test_live_lists_report_the_bad_index_at_the_generation_gate():
    message = Oem.from_file(str(ROOT / "ccsds-ndm/data/kvn/oem_g12.kvn"))
    message.segments.append(object())

    with pytest.raises(ValueError, match=r"segments\[1\] must be OemSegment"):
        message.to_str("kvn")
