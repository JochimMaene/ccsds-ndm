# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for Navigation Data Message (NDM) Python bindings.
"""

from pathlib import Path

import ccsds_ndm
import pytest


from ccsds_ndm import (
    Cdm,
    CombinedNdm,
    NdmValidationError,
    Oem,
    Tdm,
)


def test_combined_messages_reject_nested_or_cyclic_constituents_before_conversion():
    message = ccsds_ndm.CombinedNdm([])
    message.messages.append(message)
    for operation in (message.validate, lambda: message.to_str("xml")):
        with pytest.raises(ValueError, match="Nested combined"):
            operation()
    with pytest.raises(ValueError, match="Nested combined"):
        message.messages = [ccsds_ndm.CombinedNdm([])]


def test_combined_envelope_preserves_decoded_text_and_empty_fields():
    source = (
        '<?xml version="1.0" encoding="UTF-8"?><ndm>'
        "<MESSAGE_ID>A&amp;B</MESSAGE_ID><COMMENT><![CDATA[A<B]]></COMMENT>"
        "<?ignored instruction?><COMMENT/></ndm>"
    )
    message = CombinedNdm.from_str(source, format="xml")
    text = message.to_str("xml")
    assert "<MESSAGE_ID>A&amp;B</MESSAGE_ID>" in text
    assert "amp;amp" not in text
    assert "<COMMENT>A&lt;B</COMMENT>" in text
    assert CombinedNdm.from_str(text, format="xml").to_str("xml") == text


@pytest.mark.parametrize(
    "content", ['<COMMENT bad="x">a</COMMENT>', "<COMMENT><bad>x</bad></COMMENT>"]
)
def test_combined_envelope_rejects_unsupported_text_structure(content):
    source = f'<?xml version="1.0" encoding="UTF-8"?><ndm>{content}</ndm>'
    with pytest.raises(ccsds_ndm.NdmFormatError):
        CombinedNdm.from_str(source, format="xml")


DATA = Path(__file__).resolve().parents[3] / "ccsds-ndm/data"


class TestNdm:
    """Tests for NDM bindings."""

    def test_accepts_tdm_message(self):
        root = Path(__file__).parents[3]
        tdm = ccsds_ndm.from_file(
            str(root / "ccsds-ndm/data/xml/tdm_e21.xml"), format="xml"
        )

        combined = CombinedNdm([tdm])

        assert isinstance(combined.messages[0], Tdm)

    def test_combined_kvn_is_an_unsupported_notation(self):
        root = Path(__file__).parents[3]
        cdm_kvn = (root / "ccsds-ndm/data/kvn/cdm_362.kvn").read_text()

        with pytest.raises(ccsds_ndm.NdmUnsupportedNotationError) as parsed:
            ccsds_ndm.from_str(cdm_kvn + "\n" + cdm_kvn)
        assert parsed.value.code == "unsupported.notation"
        assert parsed.value.operation == "parse"
        assert parsed.value.notation == "kvn"
        assert parsed.value.message_kind == "ndm"

        ndm = CombinedNdm(messages=[Cdm.from_file(DATA / "kvn/cdm_362.kvn")])

        with pytest.raises(ccsds_ndm.NdmUnsupportedNotationError) as generated:
            ndm.to_str(format="kvn")
        assert generated.value.code == "unsupported.notation"
        assert generated.value.operation == "generate"
        assert generated.value.notation == "kvn"
        assert generated.value.source_edition == "combined"

    def test_typed_ndm_rejects_single_message_kvn(self):
        root = Path(__file__).parents[3]
        opm = (root / "ccsds-ndm/data/kvn/opm_g1.kvn").read_text()

        for format_hint in (None, "kvn"):
            with pytest.raises(ccsds_ndm.NdmUnsupportedNotationError, match="KVN"):
                CombinedNdm.from_str(opm, format=format_hint)

    def test_ndm_roundtrip_xml(self):
        oem = Oem.from_file(DATA / "kvn/oem_g11.kvn")
        cdm = Cdm.from_file(DATA / "kvn/cdm_362.kvn")

        ndm = CombinedNdm(messages=[oem, cdm])

        xml = ndm.to_str(format="xml")
        assert "<ndm" in xml
        assert "<oem" in xml
        assert "<cdm" in xml

        ndm2 = CombinedNdm.from_str(xml, format="xml")
        assert len(ndm2.messages) == 2

    def test_failed_file_generation_preserves_existing_file(self, tmp_path):
        source = (
            Path(__file__).parents[3] / "ccsds-ndm/data/kvn/oem_g11.kvn"
        ).read_text()
        oem = Oem.from_str(source, format="kvn")
        # One OEM keeps one time system throughout (ODM 5.2.4.5).
        oem.segments[1].metadata.time_system = "TAI"
        ndm = CombinedNdm([oem])
        path = tmp_path / "test.ndm"
        path.write_text("keep me")

        with pytest.raises(NdmValidationError, match="TIME_SYSTEM") as caught:
            ndm.to_file(str(path), "xml")

        assert caught.value.code == "validation.invalid_value"
        assert path.read_text() == "keep me"

    def test_ndm_message_and_comment_setters(self):
        oem = Oem.from_file(DATA / "kvn/oem_g11.kvn")
        cdm = Cdm.from_file(DATA / "kvn/cdm_362.kvn")

        ndm = CombinedNdm(messages=[oem], comments=["initial"])
        assert len(ndm.messages) == 1
        assert ndm.comments == ["initial"]

        ndm.messages = [oem, cdm]
        ndm.comments = ["updated", "suite"]
        ndm.id = "ENVELOPE-1"

        assert len(ndm.messages) == 2
        assert ndm.comments == ["updated", "suite"]
        assert ndm.id == "ENVELOPE-1"
        assert "<MESSAGE_ID>ENVELOPE-1</MESSAGE_ID>" in ndm.to_str("xml")
        ndm.id = None
        assert "ENVELOPE-1" not in ndm.to_str("xml")
