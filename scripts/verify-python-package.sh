#!/usr/bin/env bash
set -euo pipefail

if [[ $# -gt 1 ]]; then
    echo "usage: $0 [wheel-or-source-distribution]" >&2
    exit 1
fi

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
version="$(awk -F '"' '/^version = / { print $2; exit }' "${root}/bindings/python/Cargo.toml")"
if [[ $# -eq 1 ]]; then
    artifact="$1"
else
    wheels=("${root}"/dist/ccsds_ndm_py-"${version}"-cp310-abi3-*.whl)
    if [[ ${#wheels[@]} -ne 1 || ! -f "${wheels[0]}" ]]; then
        echo "expected exactly one CPython 3.10 stable-ABI ccsds-ndm-py wheel in dist" >&2
        exit 1
    fi
    artifact="${wheels[0]}"
fi

temporary="$(mktemp -d)"
trap 'rm -rf "${temporary}"' EXIT
uv venv --python "${CCSDS_NDM_VERIFY_PYTHON:-3.10}" "${temporary}/venv"
uv pip install --python "${temporary}/venv" "${artifact}" 'pytest>=8.3.5'
venv_python="${temporary}/venv/bin/python"
if [[ ! -x "${venv_python}" ]]; then
    venv_python="${temporary}/venv/Scripts/python.exe"
fi
"${venv_python}" - "${root}" <<'PY'
from pathlib import Path
import sys

import ccsds_ndm
import pytest

root = Path(sys.argv[1])
assert Path(ccsds_ndm.__file__).resolve().is_relative_to(Path(sys.prefix).resolve()), (
    "package verification imported a checkout instead of the installed artifact"
)
# Only the tooling tests need checkout imports; the installed package is already loaded.
sys.path.append(str(root / "bindings/python"))
raise SystemExit(pytest.main(["-q", str(root / "bindings/python/tests")]))
PY
