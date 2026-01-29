use ccsds_ndm::common::{OdmHeader, StateVector};
use ccsds_ndm::messages::opm::{Opm, OpmBody, OpmData, OpmMetadata, OpmSegment};
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::{Position, Velocity};

fn main() {
    // 1. Construct OPM using the Builder Pattern
    let opm = Opm::builder()
        .version("3.0")
        .header(
            OdmHeader::builder()
                .creation_date("2024-01-01T00:00:00".parse().unwrap())
                .originator("EXAMPLE")
                .build(),
        )
        .body(
            OpmBody::builder()
                .segment(
                    OpmSegment::builder()
                        .metadata(
                            OpmMetadata::builder()
                                .object_name("SAT")
                                .object_id("2023-001A")
                                .center_name("EARTH")
                                .ref_frame("GCRF")
                                .time_system("UTC")
                                .build(),
                        )
                        .data(
                            OpmData::builder()
                                .state_vector(
                                    StateVector::builder()
                                        .epoch("2024-01-01T12:00:00".parse().unwrap())
                                        .x(Position::new(7000.0, None))
                                        .y(Position::new(0.0, None))
                                        .z(Position::new(0.0, None))
                                        .x_dot(Velocity::new(0.0, None))
                                        .y_dot(Velocity::new(7.5, None))
                                        .z_dot(Velocity::new(0.0, None))
                                        .build(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();

    // 2. Serialize to KVN
    match opm.to_kvn() {
        Ok(kvn) => {
            println!("Generated KVN OPM:\n");
            println!("{}", kvn);
        }
        Err(e) => eprintln!("Failed to generate KVN: {}", e),
    }
}
