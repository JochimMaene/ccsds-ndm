#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
crate_id="$(cargo pkgid --manifest-path "${root}/ccsds-ndm/Cargo.toml")"
version="${crate_id##*#}"
version="${version##*@}"
wheels=("${root}"/dist/ccsds_ndm_py-"${version}"-*.whl)
if [[ ${#wheels[@]} -ne 1 || ! -f "${wheels[0]}" ]]; then
    echo "expected exactly one ccsds-ndm-py wheel in dist" >&2
    exit 1
fi

temporary="$(mktemp -d)"
trap 'rm -rf "${temporary}"' EXIT
uv venv "${temporary}/venv"
uv pip install --python "${temporary}/venv/bin/python" "${wheels[0]}"
"${temporary}/venv/bin/python" - <<'PY'
import ccsds_ndm

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
PY
