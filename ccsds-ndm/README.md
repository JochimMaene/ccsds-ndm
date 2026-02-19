# CCSDS NDM

[![Rust](https://github.com/JochimMaene/ccsds-ndm/actions/workflows/rust.yml/badge.svg)](https://github.com/JochimMaene/ccsds-ndm/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/ccsds-ndm)](https://crates.io/crates/ccsds-ndm)
[![docs.rs](https://img.shields.io/docsrs/ccsds-ndm)](https://docs.rs/ccsds-ndm)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

A Rust crate for parsing, validating, and generating CCSDS Navigation Data Messages (NDM) in KVN and XML formats.

## Installation

```bash
cargo add ccsds-ndm
```

## Supported Message Types

| Message | Description |
|---------|-------------|
| **OPM** | Orbit Parameter Message - single state vector and orbital parameters |
| **OMM** | Orbit Mean-Elements Message - mean orbital elements (TLE-like) |
| **OEM** | Orbit Ephemeris Message - orbit state time series with covariance |
| **OCM** | Orbit Comprehensive Message - detailed orbit data |
| **CDM** | Conjunction Data Message - collision assessment data |
| **TDM** | Tracking Data Message - tracking measurements |
| **RDM** | Reentry Data Message - reentry prediction information |
| **APM** | Attitude Parameter Message - single attitude state and parameters |
| **AEM** | Attitude Ephemeris Message - attitude state time series |
| **ACM** | Attitude Comprehensive Message - detailed attitude data |
| **NDM** | Combined NDM Instantiation - container for multiple messages |

## Quick Start

```rust
use ccsds_ndm::traits::{Ndm, Validate};
use ccsds_ndm::{from_file, MessageType};

fn main() -> ccsds_ndm::error::Result<()> {
    // Auto-detect format (KVN/XML) and message type
    let msg = from_file("example.ndm")?;

    if let MessageType::Opm(opm) = msg {
        opm.validate()?;
        println!("Object: {}", opm.body.segment.metadata.object_name);
        std::fs::write("output.xml", opm.to_xml()?)?;
    }

    Ok(())
}
```

Parse a specific type directly when you already know it:

```rust
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;

let opm = Opm::from_kvn("CCSDS_OPM_VERS = 3.0\n...").unwrap();
```

## Features

- Type-safe message model with units and enums
- Auto-detection of input format and message type
- Semantic validation via the shared `Validate` trait
- KVN and XML read/write support
- Builder APIs for constructing messages
```

## License

MPL-2.0
