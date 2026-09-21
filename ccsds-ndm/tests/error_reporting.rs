use ccsds_ndm::error::{CcsdsNdmError, ValidationError};
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::Ndm;

#[test]
fn validation_error_is_preserved() {
    // WET_MASS negative -> OutOfRange Validation Error
    let kvn = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nPHYS_START\nWET_MASS = -100 [kg]\nPHYS_STOP\n";
    let err = Ocm::from_kvn(kvn).unwrap_err();

    // Should be CcsdsNdmError::Validation
    match err {
        CcsdsNdmError::Validation(val_err) => match *val_err {
            ValidationError::OutOfRange { name, value, .. } => {
                assert_eq!(name, "Mass");
                assert_eq!(value, "-100");
            }
            _ => panic!("Expected OutOfRange, got {:?}", val_err),
        },
        _ => panic!("Expected CcsdsNdmError::Validation, got {:?}", err),
    }
}

#[test]
fn enum_error_is_preserved() {
    // CENTER_NAME and TIME_SYSTEM are Strings. OBJECT_TYPE has a fallback.
    // COV_ORDERING is a strict enum.
    let kvn = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nOBJECT_TYPE = PAYLOAD\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nCOV_START\nCOV_REF_FRAME = EME2000\nCOV_TYPE = POSITION_COVARIANCE\nCOV_ORDERING = INVALID_ORDERING\nCOV_STOP\n";
    let err = Ocm::from_kvn(kvn).unwrap_err();

    let CcsdsNdmError::Format(format_error) = err else {
        panic!("Expected FormatError");
    };
    if let ccsds_ndm::error::FormatError::Enum(enum_err) = *format_error {
        assert_eq!(enum_err.value, "INVALID_ORDERING");
        assert_eq!(enum_err.field, "COV_ORDERING");
    } else {
        panic!("Expected EnumParseError, got {format_error:?}");
    }
}

#[test]
fn display_includes_structured_operation_context() {
    let parse_error = Opm::from_kvn("CCSDS_OPM_VERS = 3.0\nBROKEN").unwrap_err();
    let parse_display = parse_error.to_string();
    assert!(parse_display.contains("failed to parse OPM KVN 3.0"));

    let mut message = Opm::from_kvn(include_str!("../data/kvn/opm_g1.kvn")).unwrap();
    message.body.segment.metadata.object_name.clear();
    let generation_display = message.to_xml().unwrap_err().to_string();
    assert!(generation_display.contains("failed to generate OPM XML 3.0"));
}
