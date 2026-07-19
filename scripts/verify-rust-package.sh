#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="${root}/ccsds-ndm/Cargo.toml"
crate_id="$(cargo pkgid --manifest-path "${manifest}")"
version="${crate_id##*#}"
version="${version##*@}"
archive="${root}/ccsds-ndm/target/package/ccsds-ndm-${version}.crate"

# CI supplies a clean checkout. `--allow-dirty` also lets contributors verify the exact staged
# implementation before committing; it does not change archive selection or Cargo's verification.
cargo package --manifest-path "${manifest}" --locked --allow-dirty

temporary="$(mktemp -d)"
trap 'rm -rf "${temporary}"' EXIT
tar -xzf "${archive}" -C "${temporary}"
cargo install \
    --path "${temporary}/ccsds-ndm-${version}" \
    --locked \
    --root "${temporary}/install"
"${temporary}/install/bin/ccsds-ndm" --version

source="$(
    printf '%s\n' \
        'CCSDS_OPM_VERS = 3.0' \
        'CREATION_DATE = 2026-01-01T00:00:00' \
        'ORIGINATOR = PACKAGE_TEST' \
        'OBJECT_NAME = PACKAGE_TEST' \
        'OBJECT_ID = 2026-001A' \
        'CENTER_NAME = EARTH' \
        'REF_FRAME = EME2000' \
        'TIME_SYSTEM = UTC' \
        'EPOCH = 2026-01-01T00:00:00' \
        'X = 7000' \
        'Y = 0' \
        'Z = 0' \
        'X_DOT = 0' \
        'Y_DOT = 7.5' \
        'Z_DOT = 0'
)"
binary="${temporary}/install/bin/ccsds-ndm"
printf '%s\n' "${source}" | "${binary}" validate --format kvn -
xml="$(printf '%s\n' "${source}" | "${binary}" convert --to xml -)"
printf '%s\n' "${xml}" | "${binary}" validate --format xml -
kvn="$(printf '%s\n' "${xml}" | "${binary}" convert --to kvn -)"
printf '%s\n' "${kvn}" | "${binary}" validate --format kvn -
