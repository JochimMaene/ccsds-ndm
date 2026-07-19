# CCSDS NDM

[![Python](https://github.com/JochimMaene/ccsds-ndm/actions/workflows/python.yml/badge.svg)](https://github.com/JochimMaene/ccsds-ndm/actions/workflows/python.yml)
[![Rust](https://github.com/JochimMaene/ccsds-ndm/actions/workflows/rust.yml/badge.svg)](https://github.com/JochimMaene/ccsds-ndm/actions/workflows/rust.yml)
[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/JochimMaene/ccsds-ndm?utm_source=badge)
[![codecov](https://codecov.io/gh/JochimMaene/ccsds-ndm/branch/main/graph/badge.svg)](https://codecov.io/gh/JochimMaene/ccsds-ndm)
[![PyPI](https://img.shields.io/pypi/v/ccsds-ndm-py)](https://pypi.org/project/ccsds-ndm-py/)
[![crates.io](https://img.shields.io/crates/v/ccsds-ndm)](https://crates.io/crates/ccsds-ndm)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

A high-performance Rust library with Python bindings for parsing, validating, and generating [CCSDS Navigation Data Messages (NDM)](https://public.ccsds.org/Pubs/500x0g4.pdf) in both KVN (Key-Value Notation) and XML formats.

## Implemented Message Families

The repository contains typed models, parsers, and serializers for the message families below. Inclusion in this table does not by itself claim that every edition, notation, operation, or public surface has completed the project's conformance quality bar. Exact support claims will be published in the support and conformance matrix.

| Message | Description |
|---------|-------------|
| **OPM** | Orbit Parameter Message – Single state vector and orbital parameters |
| **OMM** | Orbit Mean-Elements Message – Mean orbital elements (e.g., TLE-like) |
| **OEM** | Orbit Ephemeris Message – Orbit state time series with covariance |
| **OCM** | Orbit Comprehensive Message – Detailed orbit data with maneuvers |
| **CDM** | Conjunction Data Message – Collision assessment data |
| **TDM** | Tracking Data Message – Ground station tracking measurements |
| **RDM** | Reentry Data Message – Reentry prediction information |
| **APM** | Attitude Parameter Message – Single attitude state and attitude parameters |
| **AEM** | Attitude Ephemeris Message – Attitude state time series |
| **ACM** | Attitude Comprehensive Message – Detailed attitude data with maneuvers and covariance |
| **NDM** | Combined NDM Instantiation – Container for multiple CCSDS messages |

## Installation

**Python:**
```bash
pip install ccsds-ndm-py
```

**Rust:**
```bash
cargo add ccsds-ndm
```

**Command line:**
```bash
cargo install ccsds-ndm
```

## Quick Start

### Python

```python
import ccsds_ndm

# Parse any NDM file (auto-detects format and type)
msg = ccsds_ndm.from_file("example.ndm")

if isinstance(msg, ccsds_ndm.Opm):
    print(f"Object: {msg.segment.metadata.object_name}")
    print(f"Epoch: {msg.segment.data.state_vector.epoch}")

    # Nested model properties are owned snapshots. edit() writes changes back.
    ccsds_ndm.edit(msg).segment.metadata.object_name = "UPDATED"

    # Validate explicitly when useful; generation always validates.
    msg.validate()

    # Serialize
    msg.to_file("output.opm", "kvn")
    msg.to_file("output.xml", "xml")
```

### Rust

```rust
use ccsds_ndm::traits::{Ndm, Validate};
use ccsds_ndm::{from_file, MessageType};

fn main() -> ccsds_ndm::error::Result<()> {
    // Parse any NDM file
    let msg = from_file("example.ndm")?;

    // Match on the message type
    if let MessageType::Opm(opm) = msg {
        opm.validate()?;
        println!("Object: {}", opm.body.segment.metadata.object_name);

        // Serialize
        let xml = opm.to_xml()?;
        std::fs::write("output.xml", xml)?;
    }

    Ok(())
}
```

### Command line

```bash
ccsds-ndm validate example.opm
ccsds-ndm convert --to xml example.opm -o example.xml
```

Use `--target-version 2.0` or `3.0` for an explicit ODM edition. OPM, OEM, and OMM
support edition-correct 2.0 and 3.0 output; unsupported cross-edition changes fail instead of
relabeling the document.

## Features

- **Type-safe**: Strongly typed structures matching CCSDS XSD schemas
- **Auto-detection**: Automatically detects message format and type
- **Validation**: Semantic validation via shared Rust core
- **CCSDS units**: Required units are checked instead of silently reinterpreted
- **Python bindings**: Native Python API via PyO3 and maturin

## Documentation

- [User Guide & API Reference](https://jochimmaene.github.io/ccsds-ndm/)
- [CCSDS NDM Standards](https://public.ccsds.org/Publications/BlueBooks.aspx)
- [Project goal](docs/project-goal.md)
- [Support matrix](docs/support-matrix.md)

## License

This project is licensed under the [Mozilla Public License 2.0](LICENSE.txt).
