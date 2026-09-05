use ccsds_ndm::error::{CcsdsNdmError, ValidationError};
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::Ndm;

#[test]
fn test_validation_error_is_preserved() {
    // WET_MASS negative -> OutOfRange Validation Error
    let kvn = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nPHYS_START\nWET_MASS = -100 [kg]\nPHYS_STOP\n";
    let res = Ocm::from_kvn(kvn);
    assert!(res.is_err());
    let err = res.unwrap_err();

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
fn test_enum_error_is_preserved() {
    // CENTER_NAME and TIME_SYSTEM are Strings. OBJECT_TYPE has a fallback.
    // COV_ORDERING is a strict enum.
    let kvn = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nOBJECT_TYPE = PAYLOAD\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nCOV_START\nCOV_REF_FRAME = EME2000\nCOV_TYPE = POSITION_COVARIANCE\nCOV_ORDERING = INVALID_ORDERING\nCOV_STOP\n";
    let res = Ocm::from_kvn(kvn);
    assert!(res.is_err());
    let err = res.unwrap_err();

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
