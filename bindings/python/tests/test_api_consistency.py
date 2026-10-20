# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""API consistency tests for class-level parse helpers."""

from pathlib import Path

import pytest

import ccsds_ndm
from ccsds_ndm import Acm, Aem, Apm, Cdm, CombinedNdm, Ocm, Oem, Omm, Opm, Rdm, Tdm

ROOT = Path(__file__).resolve().parents[3]

CLASS_FIXTURES = [
    (Acm, ROOT / "ccsds-ndm/data/kvn/acm_g6.kvn"),
    (Aem, ROOT / "ccsds-ndm/data/kvn/aem_g4.kvn"),
    (Apm, ROOT / "ccsds-ndm/data/kvn/apm_g1.kvn"),
    (Cdm, ROOT / "ccsds-ndm/data/kvn/cdm_362.kvn"),
    (CombinedNdm, ROOT / "ccsds-ndm/data/xml/ndm_g12.xml"),
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
        # Single-segment fixture: renaming one segment's object in a multi-segment AEM now
        # trips the message-level OBJECT_NAME/OBJECT_ID consistency check.
        ROOT / "ccsds-ndm/data/kvn/aem_g5.kvn",
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
    (
        CombinedNdm,
        ROOT / "ccsds-ndm/data/xml/ndm_g12.xml",
        lambda value: value.messages,
    ),
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
    parsed = cls.from_file(path)
    assert isinstance(parsed, cls)


@pytest.mark.parametrize(("cls", "path"), CLASS_FIXTURES)
def test_type_specific_file_round_trip(cls, path, tmp_path):
    output = tmp_path / f"{cls.__name__.lower()}.xml"
    cls.from_file(path).to_file(output, "xml")
    assert isinstance(cls.from_file(output), cls)


def test_top_level_file_api_accepts_pathlike(tmp_path):
    source = ROOT / "ccsds-ndm/data/kvn/opm_g1.kvn"
    destination = tmp_path / "opm.xml"
    ccsds_ndm.convert_file(source, destination, "xml")
    assert isinstance(ccsds_ndm.from_file(destination), Opm)


@pytest.mark.parametrize(("cls", "path"), CLASS_FIXTURES)
def test_type_specific_from_str_allows_default_format(cls, path):
    parsed = cls.from_str(path.read_text())
    assert isinstance(parsed, cls)


def test_public_exceptions_follow_python_error_categories():
    value_errors = [
        ccsds_ndm.NdmFormatError,
        ccsds_ndm.NdmKvnParseError,
        ccsds_ndm.NdmXmlError,
        ccsds_ndm.NdmEpochError,
    ]
    assert all(issubclass(exception, ValueError) for exception in value_errors)
    assert issubclass(ccsds_ndm.NdmValidationError, ccsds_ndm.NdmError)
    assert issubclass(ccsds_ndm.NdmUnsupportedMessageError, ccsds_ndm.NdmError)
    assert issubclass(ccsds_ndm.NdmIoError, OSError)
    assert not issubclass(ccsds_ndm.NdmIoError, ValueError)


@pytest.mark.parametrize(
    "error_type",
    [
        ccsds_ndm.NdmError,
        ccsds_ndm.NdmFormatError,
        ccsds_ndm.NdmKvnParseError,
        ccsds_ndm.NdmXmlError,
        ccsds_ndm.NdmValidationError,
        ccsds_ndm.NdmEpochError,
        ccsds_ndm.NdmIoError,
        ccsds_ndm.NdmUnsupportedMessageError,
        ccsds_ndm.NdmUnsupportedNotationError,
    ],
)
def test_ndm_errors_expose_unavailable_diagnostics_as_none(error_type):
    error = error_type("failure without context")
    for field in (
        "operation",
        "notation",
        "message_kind",
        "source_edition",
        "code",
        "field_path",
        "line",
        "column",
        "byte_offset",
        "original_token",
        "expected",
        "requirement",
    ):
        assert getattr(error, field) is None


@pytest.mark.parametrize(
    ("cls", "args", "kwargs"),
    [
        (ccsds_ndm.AdditionalParameters, (), {"mass": 123.0}),
        (ccsds_ndm.QuaternionState, ("GCRF", "SC_BODY_1", 0, 0, 0, 1), {}),
        (ccsds_ndm.EulerAngleState, ("GCRF", "SC_BODY_1", "XYZ", 0, 0, 0), {}),
        (ccsds_ndm.AngVelState, ("GCRF", "SC_BODY_1", "A", 0, 0, 0), {}),
        (ccsds_ndm.SpinState, ("GCRF", "SC_BODY_1", 0, 0, 0, 0), {}),
        (ccsds_ndm.InertiaState, ("SC_BODY_1", 1, 1, 1, 0, 0, 0), {}),
    ],
)
def test_optional_constructor_fields_can_be_omitted(cls, args, kwargs):
    value = cls(*args, **kwargs)
    assert value.comment == []
    if cls is ccsds_ndm.AdditionalParameters:
        assert value.mass == 123.0
        assert value.area_pc is None
    elif cls is ccsds_ndm.QuaternionState:
        assert value.quaternion_dot is None


@pytest.mark.parametrize("format", [None, "kvn"])
@pytest.mark.parametrize("from_file", [False, True])
def test_wrong_family_errors_have_diagnostic_fields(format, from_file):
    source = ROOT / "ccsds-ndm/data/kvn/opm_g1.kvn"
    with pytest.raises(
        ccsds_ndm.NdmUnsupportedMessageError, match="expected OEM, found OPM"
    ) as caught:
        if from_file:
            Oem.from_file(source, format=format)
        else:
            Oem.from_str(source.read_text(), format=format)
    error = caught.value
    assert error.operation == "parse"
    assert error.notation == format
    assert error.message_kind == "oem"
    for field in (
        "source_edition",
        "code",
        "field_path",
        "line",
        "column",
        "byte_offset",
        "original_token",
        "expected",
        "requirement",
    ):
        assert getattr(error, field) is None


@pytest.mark.parametrize(
    "name",
    ["RelativeStateVector", "SpacecraftParameters", "TleParameters"],
)
def test_registered_model_types_are_exported_from_the_package(name):
    assert name in ccsds_ndm.__all__
    assert getattr(ccsds_ndm, name) is getattr(ccsds_ndm.ccsds_ndm, name)


def test_nested_model_changes_are_live_without_an_editor():
    opm = ccsds_ndm.from_file(str(ROOT / "ccsds-ndm/data/kvn/opm_g1.kvn"))
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

    cdm = ccsds_ndm.from_file(str(ROOT / "ccsds-ndm/data/kvn/cdm_362.kvn"))
    retained_segment = cdm.body.segments[0]
    retained_segment.metadata.object_name = "OBJECT-1"
    assert cdm.body.segments[0].metadata.object_name == "OBJECT-1"
    assert cdm.body.segments[0] is retained_segment
    assert any(
        line.startswith("OBJECT_NAME") and line.endswith("= OBJECT-1")
        for line in cdm.to_str("kvn").splitlines()
    )

    combined = CombinedNdm([opm])
    assert combined.messages[0] is opm
    assert isinstance(combined.messages[0], Opm)


@pytest.mark.parametrize(
    ("cls", "path", "get_nested"),
    LIVE_NESTED_CASES,
    ids=[case[0].__name__ for case in LIVE_NESTED_CASES],
)
def test_nested_identity_and_mutation_are_live_for_every_message_family(
    cls, path, get_nested
):
    message = ccsds_ndm.from_file(str(path))
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
    message = ccsds_ndm.from_file(str(ROOT / "ccsds-ndm/data/kvn/tdm_e1.kvn"))
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
    message = ccsds_ndm.from_file(str(path))
    records = get_records(message)
    assert isinstance(records, list)
    assert get_records(message) is records
    assert records

    retained = records.pop()
    records.append(retained)
    assert records[-1] is retained
    assert message.validate() is None


def test_live_lists_report_the_bad_index_at_the_generation_gate():
    message = ccsds_ndm.from_file(str(ROOT / "ccsds-ndm/data/kvn/oem_g12.kvn"))
    message.segments.append(object())

    with pytest.raises(TypeError, match=r"segments\[1\] must be OemSegment"):
        message.to_str("kvn")


def test_interpretation_metadata_has_no_implicit_defaults():
    with pytest.raises(TypeError):
        ccsds_ndm.OpmMetadata(object_name="S", object_id="I")
    with pytest.raises(TypeError):
        ccsds_ndm.OemMetadata("S", "I", "2023-01-01T00:00:00", "2023-01-01T01:00:00")
    with pytest.raises(TypeError):
        ccsds_ndm.AemMetadata(
            object_name="S",
            object_id="I",
            start_time="2023-01-01T00:00:00",
            stop_time="2023-01-01T01:00:00",
        )
    with pytest.raises(TypeError):
        ccsds_ndm.TdmMetadata(participant_1="P")
    with pytest.raises(TypeError):
        ccsds_ndm.OcmMetadata(epoch_tzero="2023-01-01T00:00:00")
    with pytest.raises(TypeError):
        ccsds_ndm.ApmMetadata(object_name="S", object_id="I")
    with pytest.raises(TypeError):
        ccsds_ndm.AcmMetadata(object_name="S", epoch_tzero="2023-01-01T00:00:00")
    with pytest.raises(TypeError):
        ccsds_ndm.RdmMetadata(
            object_name="S",
            international_designator="I",
            epoch_tzero="2023-01-01T00:00:00",
        )


def test_covariance_constructor_requires_all_entries():
    with pytest.raises(TypeError):
        ccsds_ndm.OpmCovarianceMatrix(cx_x=1.0)
    names = (
        "cx_x cy_x cy_y cz_x cz_y cz_z cx_dot_x cx_dot_y cx_dot_z "
        "cy_dot_x cy_dot_y cy_dot_z cz_dot_x cz_dot_y cz_dot_z "
        "cx_dot_x_dot cy_dot_x_dot cy_dot_y_dot cz_dot_x_dot cz_dot_y_dot cz_dot_z_dot"
    ).split()
    covariance = ccsds_ndm.OpmCovarianceMatrix(**dict.fromkeys(names, 0.0))
    assert covariance.cx_x == 0.0


def test_user_defined_entries_keep_comment_and_parameter_order():
    block = ccsds_ndm.UserDefined({"B": "2", "A": "1"})
    assert block.entries == [("B", "2"), ("A", "1")]
    block.entries = [("B", "2"), "between", ("A", "1")]
    assert block.entries == [("B", "2"), "between", ("A", "1")]
    block.user_defined = {"C": "3", "D": "4"}
    assert block.entries == [("C", "3"), "between", ("D", "4")]
    block.comment = ["edited"]
    assert block.entries == ["edited", ("C", "3"), ("D", "4")]


def test_enum_stubs_match_simple_pyo3_classes():
    import enum

    assert not issubclass(ccsds_ndm.YesNo, enum.Enum)
    assert isinstance(ccsds_ndm.YesNo.Yes, ccsds_ndm.YesNo)
    from pathlib import Path

    stub = (Path(__file__).parents[1] / "ccsds_ndm" / "__init__.pyi").read_text()
    assert "(enum.Enum)" not in stub
    assert "Yes: typing.ClassVar[YesNo]" in stub


@pytest.mark.parametrize(
    "constructor, arguments, missing",
    [
        (
            ccsds_ndm.AemMetadata,
            dict(
                object_name="S",
                object_id="I",
                ref_frame_a="A",
                ref_frame_b="B",
                start_time="2023-01-01T00:00:00",
                stop_time="2023-01-01T01:00:00",
                time_system="UTC",
            ),
            "attitude_type",
        ),
        (
            ccsds_ndm.OmmMetadata,
            dict(
                object_name="S",
                object_id="I",
                center_name="EARTH",
                ref_frame="TEME",
                time_system="UTC",
            ),
            "mean_element_theory",
        ),
    ],
)
def test_metadata_does_not_invent_interpretation(constructor, arguments, missing):
    with pytest.raises(TypeError, match=missing):
        constructor(**arguments)


def test_user_defined_interleaving_survives_xml_and_refuses_kvn():
    message = Opm.from_file(ROOT / "ccsds-ndm/data/kvn/opm_g1.kvn")
    block = ccsds_ndm.UserDefined()
    block.entries = ["before", ("B", "2"), "between", ("A", "1")]
    message.segment.data.user_defined_parameters = block
    xml = message.to_str(format="xml")
    parsed = Opm.from_str(xml, format="xml")
    assert parsed.segment.data.user_defined_parameters.entries == block.entries
    assert parsed.to_str(format="xml") == xml
    with pytest.raises(ccsds_ndm.NdmValidationError, match="comments must precede"):
        parsed.to_str(format="kvn")
