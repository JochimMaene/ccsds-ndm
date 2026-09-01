#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="${root}/ccsds-ndm/Cargo.toml"
version="$(awk -F '"' '/^version = / { print $2; exit }' "${manifest}")"
target_dir="${CARGO_TARGET_DIR:-${root}/ccsds-ndm/target}"
if [[ "${target_dir}" != /* ]]; then
    target_dir="$(pwd)/${target_dir}"
fi
archive="${target_dir}/package/ccsds-ndm-${version}.crate"

# CI supplies a clean checkout. `--allow-dirty` also lets contributors verify the exact staged
# implementation before committing; it does not change archive selection or Cargo's verification.
cargo package --manifest-path "${manifest}" --allow-dirty

temporary="$(mktemp -d)"
trap 'rm -rf "${temporary}"' EXIT
tar -xzf "${archive}" -C "${temporary}"
CARGO_TARGET_DIR="${temporary}/target" cargo test \
    --manifest-path "${temporary}/ccsds-ndm-${version}/Cargo.toml" \
    --all-features \
    --locked
