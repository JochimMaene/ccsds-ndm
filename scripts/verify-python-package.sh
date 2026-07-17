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
"${temporary}/venv/bin/python" -c \
    'import ccsds_ndm; assert hasattr(ccsds_ndm, "Opm"); assert hasattr(ccsds_ndm, "convert_opm")'
