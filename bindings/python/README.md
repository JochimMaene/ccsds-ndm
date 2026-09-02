# CCSDS NDM

[![Python](https://github.com/JochimMaene/ccsds-ndm/actions/workflows/python.yml/badge.svg)](https://github.com/JochimMaene/ccsds-ndm/actions/workflows/python.yml)
[![codecov](https://codecov.io/gh/JochimMaene/ccsds-ndm/branch/main/graph/badge.svg)](https://codecov.io/gh/JochimMaene/ccsds-ndm)
[![PyPI](https://img.shields.io/pypi/v/ccsds-ndm-py)](https://pypi.org/project/ccsds-ndm-py/)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

Python bindings for the `ccsds-ndm` Rust core, for parsing, validating, and generating [CCSDS Navigation Data Messages (NDM)](https://public.ccsds.org/Pubs/500x0g4.pdf) in both KVN and XML formats.

## Implemented Message Families

The package exposes typed models, parsers, and serializers for the message families below. Inclusion in this table does not by itself claim verified support for every edition, notation, or operation. Exact support claims will be published in the project support and conformance matrix.

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

```bash
pip install ccsds-ndm-py
```

Requires Python 3.10+.

## Quick Start

```python
import ccsds_ndm

# Parse any NDM file (auto-detects format and type)
msg = ccsds_ndm.from_file("example.ndm")

if isinstance(msg, ccsds_ndm.Opm):
    print(f"Object: {msg.segment.metadata.object_name}")
    print(f"Epoch: {msg.segment.data.state_vector.epoch}")

    # Edit nested values directly and validate explicitly when useful.
    msg.segment.metadata.object_name = "UPDATED"
    msg.validate()

    # Generation always validates.
    ccsds_ndm.to_file(msg, "output.opm", "kvn")
    ccsds_ndm.to_file(msg, "output.xml", "xml")

# Input notation is detected; only choose the output notation.
xml = ccsds_ndm.convert(msg.to_str("kvn"), "xml")
ccsds_ndm.convert_file("input.opm", "output.xml", "xml")
```

## Features

- **Type-safe**: Strongly typed structures matching CCSDS XSD schemas
- **Auto-detection**: Automatically detects message format and type
- **Strict parsing**: Rejects unknown, duplicate, reordered, and malformed content
- **Validated output**: Invalid typed models are never silently serialized
- **Native bindings**: PyO3 + maturin wrapping the Rust core implementation

## Documentation

- [User Guide & API Reference](https://jochimmaene.github.io/ccsds-ndm/)
- [CCSDS NDM Standards](https://public.ccsds.org/Publications/BlueBooks.aspx)

## License

MPL-2.0
