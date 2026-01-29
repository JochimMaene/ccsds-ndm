# CCSDS NDM

A high-performance, type-safe library for parsing and generating CCSDS Navigation Data Messages (NDM) in both KVN (Key-Value Notation) and XML formats.

This crate is designed for mission-critical space applications where correctness, performance, and adherence to standards are paramount.

## Key Features

- **Comprehensive Support**: Full support for OPM, OMM, OEM, OCM, CDM, TDM, RDM, AEM, APM, and ACM messages.
- **Format Agnostic**: Seamlessly convert between KVN and XML formats.
- **Type Safety**: Strictly typed units (e.g., `km`, `deg`, `s`) prevent physical unit errors.
- **High Performance Parsing**: Utilizes `winnow` and `quick-xml` for efficient, low-allocation parsing.
- **Ergonomic Construction**: Uses the builder pattern (via the [`bon`](https://docs.rs/bon) crate) for safe and easy message creation.
- **Standard Compliant**: Validates messages against CCSDS 502.0-B-3 and related standards.

## Architecture

The library is organized around a few core concepts:

- **`Ndm` Trait**: The unifying interface for all message types. It defines the standard `to_kvn`, `from_kvn`, `to_xml`, and `from_xml` methods.
- **`MessageType` Enum**: A container that holds any valid NDM. This is the primary return type when parsing files with unknown contents (auto-detection).
- **Strong Typing**: All physical quantities (Distance, Velocity, Mass, etc.) are wrapped in the `UnitValue` struct, ensuring that units are always tracked and validated.

## Quick Start

### 1. Parse any NDM file (auto-detection)

The library automatically detects whether the input is KVN or XML and what message type it contains.

```rust
use ccsds_ndm::{from_file, MessageType};

let ndm = from_file("example.opm").unwrap();

match ndm {
    MessageType::Opm(opm) => {
        println!("Object: {}", opm.body.segment.metadata.object_name);
    }
    MessageType::Oem(oem) => {
        println!("Ephemeris points: {}", oem.body.segment[0].data.state_vector.len());
    }
    _ => println!("Other message type"),
}
```

### 2. Parse a specific message type

If you know the message type in advance, you can parse it directly:

```rust
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;

// Parses strict KVN for OPM
let opm = Opm::from_kvn("CCSDS_OPM_VERS = 3.0\n...").unwrap();
```

### 3. Generate a message using the Builder Pattern

Creating messages from scratch is safe and verbose-free using the `builder()` methods.

```rust
use ccsds_ndm::messages::opm::{Opm, OpmBody, OpmSegment, OpmMetadata, OpmData};
use ccsds_ndm::common::{OdmHeader, StateVector};
use ccsds_ndm::types::{Epoch, Position, Velocity};
use ccsds_ndm::traits::Ndm;

let opm = Opm::builder()
    .version("3.0")
    .header(OdmHeader::builder()
        .creation_date("2024-01-01T00:00:00".parse().unwrap())
        .originator("GEMINI")
        .build())
    .body(OpmBody::builder()
        .segment(OpmSegment::builder()
            .metadata(OpmMetadata::builder()
                .object_name("SATELLITE")
                .object_id("2024-001A")
                .center_name("EARTH")
                .ref_frame("GCRF")
                .time_system("UTC")
                .build())
            .data(OpmData::builder()
                .state_vector(StateVector::builder()
                    .epoch("2024-01-01T12:00:00".parse().unwrap())
                    .x(Position::new(7000.0, None))
                    .y(Position::new(0.0, None))
                    .z(Position::new(0.0, None))
                    .x_dot(Velocity::new(0.0, None))
                    .y_dot(Velocity::new(7.5, None))
                    .z_dot(Velocity::new(0.0, None))
                    .build())
                .build())
            .build())
        .build())
    .build();

// Convert to KVN string
println!("{}", opm.to_kvn().unwrap());
```

### 4. Serialize to KVN or XML

```rust
use ccsds_ndm::{from_file, MessageType};

let ndm = from_file("example.opm").unwrap();

// Serialize to string
let kvn_string = ndm.to_kvn().unwrap();
let xml_string = ndm.to_xml().unwrap();

// Write to file
ndm.to_xml_file("output.xml").unwrap();
```

## License

MPL-2.0

```
