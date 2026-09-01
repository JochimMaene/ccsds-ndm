#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
version="$(awk -F '"' '/^version = / { print $2; exit }' "${root}/bindings/python/Cargo.toml")"
wheels=("${root}"/dist/ccsds_ndm_py-"${version}"-cp310-abi3-*.whl)
if [[ ${#wheels[@]} -ne 1 || ! -f "${wheels[0]}" ]]; then
    echo "expected exactly one CPython 3.10 stable-ABI ccsds-ndm-py wheel in dist" >&2
    exit 1
fi

temporary="$(mktemp -d)"
trap 'rm -rf "${temporary}"' EXIT
uv venv --python 3.10 "${temporary}/venv"
uv pip install --python "${temporary}/venv/bin/python" "${wheels[0]}"
"${temporary}/venv/bin/python" - \
    "${root}/ccsds-ndm/data/kvn/oem_g11.kvn" \
    "${root}/ccsds-ndm/data/xml/oem_g14.xml" \
    "${root}/ccsds-ndm/data/kvn/omm_g9.kvn" \
    "${root}/ccsds-ndm/data/xml/omm_g10.xml" <<'PY'
import ccsds_ndm
from pathlib import Path
import sys

source = """\
CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2026-01-01T00:00:00
ORIGINATOR = PACKAGE_TEST
OBJECT_NAME = PACKAGE_TEST
OBJECT_ID = 2026-001A
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
EPOCH = 2026-01-01T00:00:00
X = 7000
Y = 0
Z = 0
X_DOT = 0
Y_DOT = 7.5
Z_DOT = 0
"""

message = ccsds_ndm.from_str(source, format="kvn")
assert isinstance(message, ccsds_ndm.Opm)
message.segment.metadata.object_name = "EDITED"
assert message.segment.metadata.object_name == "EDITED"
assert message.validate() is None
xml = message.to_str("xml")
assert "<OBJECT_NAME>EDITED</OBJECT_NAME>" in xml
kvn = ccsds_ndm.convert(xml, "kvn")
assert isinstance(ccsds_ndm.from_str(kvn, format="kvn"), ccsds_ndm.Opm)
assert isinstance(ccsds_ndm.from_str("<ndm/>", format="xml"), ccsds_ndm.Ndm)

for wrapper, kvn_path, xml_path in [
    (ccsds_ndm.Oem, sys.argv[1], sys.argv[2]),
    (ccsds_ndm.Omm, sys.argv[3], sys.argv[4]),
]:
    kvn_source = Path(kvn_path).read_text()
    xml_source = Path(xml_path).read_text()
    assert isinstance(wrapper.from_str(kvn_source, format="kvn"), wrapper)
    assert isinstance(wrapper.from_str(xml_source, format="xml"), wrapper)
    generated_xml = ccsds_ndm.convert(kvn_source, "xml")
    generated_kvn = ccsds_ndm.convert(generated_xml, "kvn")
    assert isinstance(wrapper.from_str(generated_kvn, format="kvn"), wrapper)
PY
