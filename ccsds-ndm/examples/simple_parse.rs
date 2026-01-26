use ccsds_ndm::{from_str, MessageType};

fn main() {
    let input = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = EXAMPLE
OBJECT_NAME = SATELLITE
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T12:00:00
X = 6500.0 [km]
Y = 0.0 [km]
Z = 0.0 [km]
X_DOT = 0.0 [km/s]
Y_DOT = 7.5 [km/s]
Z_DOT = 0.0 [km/s]
"#;

    match from_str(input) {
        Ok(message) => match message {
            MessageType::Opm(opm) => {
                println!(
                    "Parsed OPM: Object {} ({})",
                    opm.body.segment.metadata.object_name, opm.body.segment.metadata.object_id
                );
                println!(
                    "State Vector at {}:",
                    opm.body.segment.data.state_vector.epoch
                );
                println!("  Pos: {} km", opm.body.segment.data.state_vector.x);
                println!("  Vel: {} km/s", opm.body.segment.data.state_vector.y_dot);
            }
            _ => println!("Parsed a different message type: {:?}", message),
        },
        Err(e) => eprintln!("Failed to parse message: {}", e),
    }
}
