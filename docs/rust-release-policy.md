# Rust Release Policy

This policy applies to the `ccsds-ndm` Rust crate while its version is below `1.0`. It records the
release guarantees that exist today and, equally importantly, the guarantees that are not yet
claimed.

## Tested toolchain and platform

The Rust CI and crate-packaging gates run with the current stable Rust toolchain on
`ubuntu-latest`. That is the only tested Rust release environment.

The crate does not currently declare a minimum supported Rust version (MSRV). Other Rust versions
and operating systems may work, but they are not advertised as supported until they have explicit
CI coverage. This scope should be expanded only when the project is prepared to maintain the
corresponding CI and compatibility commitment.

## Compatibility and migration

The crate is pre-`1.0`, and its public Rust API may change between releases. In particular, a
`0.0.x` version must not be treated as source-compatible with another `0.0.x` version. Breaking
changes should be called out in the release notes, with migration guidance when the replacement is
not evident from the API documentation.

The executable public-signature tests protect intentionally exposed entry points from accidental
changes within development, but they do not establish a stable-API or semantic-versioning
guarantee.

## Crate packaging

Run `just package-rust` from a clean checkout before publishing. It runs `cargo package --locked`,
which applies Cargo's package-file selection and verifies that the unpacked crate builds using the
locked dependency resolution. The Rust release workflow runs this command in a separate
clean-checkout job and will not publish unless it succeeds. It also requires the release tag to
equal `v` followed by the version in `Cargo.toml`.

This is package construction and buildability evidence. It is not evidence of byte-for-byte
reproducible artifacts, installation on every platform, or supply-chain provenance. Those remain
release gaps.

## Security reporting

The project does not yet document a private vulnerability-reporting channel. Non-sensitive defects
may be reported through the public issue tracker, but undisclosed vulnerability details should not
be posted publicly. Until a private reporting route and a response policy are established, the
security release gate remains open and the project does not claim mature security handling.
