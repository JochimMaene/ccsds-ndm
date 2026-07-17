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
