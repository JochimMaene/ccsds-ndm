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
extracts the produced `.crate`, installs the CLI from that extracted artifact into a temporary
prefix, and executes its version command. This applies Cargo's package-file selection and verifies
that the artifact—not merely the working tree—builds and installs using the locked dependency
resolution. The Rust release workflow runs this command in a separate clean-checkout job and will
not publish unless it succeeds. It also requires the release tag to equal `v` followed by the
version in `Cargo.toml`.

This is package construction and current-stable Ubuntu installation evidence. Byte-for-byte
reproducible artifacts and supply-chain provenance are not required before 1.0 and are not claimed.

## Security reporting

Defects are currently reported through the public issue tracker. The project does not offer a
private vulnerability-intake channel and therefore asks reporters not to post secrets, credentials,
or other sensitive operational data. This explicit pre-1.0 scope describes how the current release
is maintained; it is not a claim of confidential disclosure handling or mature security response.

A private route and response-time policy should be added only when maintainership and operational
use justify sustaining them. Mature security handling remains a reference-status goal rather than
a technical OPM conformance gate.
