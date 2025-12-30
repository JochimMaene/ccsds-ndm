# CCSDS NDM

[![Python](https://github.com/JochimMaene/ccsds-ndm/actions/workflows/python.yml/badge.svg)](https://github.com/JochimMaene/ccsds-ndm/actions/workflows/python.yml)
[![Rust](https://github.com/JochimMaene/ccsds-ndm/actions/workflows/rust.yml/badge.svg)](https://github.com/JochimMaene/ccsds-ndm/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/JochimMaene/ccsds-ndm/branch/main/graph/badge.svg)](https://codecov.io/gh/JochimMaene/ccsds-ndm)
[![PyPI](https://img.shields.io/pypi/v/ccsds-ndm-py)](https://pypi.org/project/ccsds-ndm-py/)
[![crates.io](https://img.shields.io/crates/v/ccsds-ndm)](https://crates.io/crates/ccsds-ndm)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

A Rust library with Python bindings for parsing and generating [CCSDS Navigation Data Messages (NDM)](https://public.ccsds.org/Pubs/500x0g4.pdf) in both KVN (Key-Value Notation) and XML formats.

## Supported Message Types

| Message | Description |
|---------|-------------|
| **OPM** | Orbit Parameter Message – Single state vector and orbital parameters |
| **OMM** | Orbit Mean-Elements Message – Mean orbital elements (e.g., TLE-like) |
| **OEM** | Orbit Ephemeris Message – Orbit state time series with covariance |
| **OCM** | Orbit Comprehensive Message – Detailed orbit data with maneuvers |
| **CDM** | Conjunction Data Message – Collision assessment data |
| **TDM** | Tracking Data Message – Ground station tracking measurements |
| **RDM** | Reentry Data Message – Reentry prediction information |

## Installation

**Python:**
```bash
pip install ccsds-ndm-py
```

## Quick Start

```python
import ccsds_ndm

# Parse any NDM file (auto-detects format and type)
ndm = ccsds_ndm.from_file("example.opm")

# Or parse from a string
opm = ccsds_ndm.from_str(kvn_content)

# Access data
print(f"Object: {opm.segment.metadata.object_name}")
print(f"Epoch: {opm.segment.data.state_vector.epoch}")
print(f"Position: ({opm.segment.data.state_vector.x}, {opm.segment.data.state_vector.y}, {opm.segment.data.state_vector.z})")

# Write to KVN or XML
opm.to_file("output.opm", "kvn")
opm.to_file("output.xml", "xml")
```

## Features

- **Type-safe**: Strongly typed structures matching CCSDS XSD schemas
- **Auto-detection**: Automatically detects message format and type
- **Python bindings**: Native Python API via PyO3
- **Zero-copy parsing**: Efficient KVN tokenizer

## Documentation

- [User Guide & API Reference](https://jochimmaene.github.io/ccsds-ndm/)
- [CCSDS NDM Standards](https://public.ccsds.org/Publications/BlueBooks.aspx)

## License

This project is licensed under the [Mozilla Public License 2.0](LICENSE.txt).
