use crate::common::{mutated, mutated_once};
use crate::{sample_cdm_kvn, KVN, XML};
use ccsds_ndm::error::{FormatError, ValidationError};
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::types::{
    CdmObjectType, ManeuverableType, ObjectDescription, ReferenceFrameType, ScreenVolumeFrameType,
    ScreenVolumeShapeType, YesNo,
};
use ccsds_ndm::Ndm;
const DRAG_COVARIANCE_ROW: &str = "\
CDRG_R = 0.001 [m**3/kg]
CDRG_T = 0.002 [m**3/kg]
CDRG_N = 0.003 [m**3/kg]
CDRG_RDOT = 0.0001 [m**3/(kg*s)]
CDRG_TDOT = 0.0002 [m**3/(kg*s)]
CDRG_NDOT = 0.0003 [m**3/(kg*s)]
CDRG_DRG = 0.00001 [m**4/kg**2]";

const SRP_COVARIANCE_ROW: &str = "\
CSRP_R = 0.001 [m**3/kg]
CSRP_T = 0.002 [m**3/kg]
CSRP_N = 0.003 [m**3/kg]
CSRP_RDOT = 0.0001 [m**3/(kg*s)]
CSRP_TDOT = 0.0002 [m**3/(kg*s)]
CSRP_NDOT = 0.0003 [m**3/(kg*s)]
CSRP_DRG = 0.00001 [m**4/kg**2]
CSRP_SRP = 0.00002 [m**4/kg**2]";

const THRUST_COVARIANCE_ROW: &str = "\
CTHR_R = 0.001 [m**2/s**2]
CTHR_T = 0.002 [m**2/s**2]
CTHR_N = 0.003 [m**2/s**2]
CTHR_RDOT = 0.0001 [m**2/s**3]
CTHR_TDOT = 0.0002 [m**2/s**3]
CTHR_NDOT = 0.0003 [m**2/s**3]
CTHR_DRG = 0.00001 [m**3/(kg*s**2)]
CTHR_SRP = 0.00002 [m**3/(kg*s**2)]
CTHR_THR = 0.000001 [m**2/s**4]";

fn with_optional_covariance_rows(kvn: &str, rows: &str) -> String {
    kvn.replace(
        "CNDOT_NDOT = 1.0 [m**2/s**2]",
        &format!("CNDOT_NDOT = 1.0 [m**2/s**2]\n{rows}"),
    )
}

#[test]
fn parse_cdm_kvn_success() {
    let kvn = sample_cdm_kvn();
    let cdm = Cdm::from_kvn(&kvn).expect("CDM should parse");
    assert_eq!(cdm.version, "1.0");
    assert_eq!(cdm.header.originator, "TEST");
    assert_eq!(cdm.header.message_for.as_deref(), Some("OPERATOR"));
    assert_eq!(cdm.body.segments[0].metadata.object, CdmObjectType::Object1);
    assert_eq!(cdm.body.segments[1].metadata.object, CdmObjectType::Object2);
    assert_eq!(cdm.body.segments.len(), 2);
    assert_eq!(cdm.body.segments[0].metadata.object_name, "OBJ1");
    assert_eq!(cdm.body.segments[1].metadata.object_name, "OBJ2");
    assert_eq!(cdm.body.relative_metadata_data.miss_distance.value, 100.0);
    assert!(cdm
        .body
        .relative_metadata_data
        .relative_state_vector
        .is_some());
    assert_eq!(
        cdm.body.relative_metadata_data.screen_volume_frame,
        Some(ScreenVolumeFrameType::Rtn)
    );
    assert_eq!(
        cdm.body.relative_metadata_data.screen_volume_shape,
        Some(ScreenVolumeShapeType::Box)
    );
}

#[test]
fn ellipsoid_screen_volume_does_not_require_a_frame() {
    let kvn = sample_cdm_kvn()
        .lines()
        .filter(|line| {
            !line.starts_with("SCREEN_VOLUME_") || line.starts_with("SCREEN_VOLUME_SHAPE")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let kvn = mutated(
        &kvn,
        "SCREEN_VOLUME_SHAPE = BOX",
        "SCREEN_VOLUME_SHAPE = ELLIPSOID",
    );
    let cdm = Cdm::from_kvn(&kvn).unwrap();
    assert_eq!(
        cdm.body.relative_metadata_data.screen_volume_shape,
        Some(ScreenVolumeShapeType::Ellipsoid)
    );
    assert!(cdm
        .body
        .relative_metadata_data
        .screen_volume_frame
        .is_none());
}

#[test]
fn header_missing_fields_error() {
    let kvn = r###"CCSDS_CDM_VERS = 1.0
ORIGINATOR = TEST
MESSAGE_FOR = SAT
MESSAGE_ID = MSG-001
"###;
    let err = Cdm::from_kvn(kvn).unwrap_err();
    assert!(
        matches!(err.as_validation_error(), Some(ValidationError::MissingRequiredField { field, .. }) if field == "CREATION_DATE"),
        "{err}"
    );
}

/// A CDM describes exactly two objects, so a message truncated before the second one must be
/// refused. Truncating a shipped fixture keeps the first segment byte-for-byte valid, leaving
/// the missing second segment as the only reason the parse can fail.
#[test]
fn validate_exactly_two_segments() {
    const FIXTURE: &str = include_str!("../../data/kvn/cdm_362.kvn");
    let first_segment_only: Vec<&str> = FIXTURE
        .lines()
        .take_while(|line| !line.starts_with("OBJECT = OBJECT2"))
        .collect();

    Cdm::from_kvn(FIXTURE).expect("fixture with both segments must parse");
    let error = Cdm::from_kvn(&format!("{}\n", first_segment_only.join("\n")))
        .expect_err("single-segment CDM accepted");
    assert!(
        matches!(
            error.as_validation_error(),
            Some(ValidationError::MissingRequiredField { field, .. }) if field == "OBJECT"
        ),
        "single-segment CDM was refused for an unrelated reason: {error}"
    );
}

#[test]
fn relative_state_vector_must_be_complete() {
    let base = sample_cdm_kvn();
    Cdm::from_kvn(&base).unwrap();
    for missing in [
        "RELATIVE_VELOCITY_N",
        "RELATIVE_POSITION_T",
        "RELATIVE_POSITION_R",
    ] {
        let input = base
            .lines()
            .filter(|line| {
                if missing == "RELATIVE_POSITION_R" {
                    !line.starts_with("RELATIVE_POSITION_")
                } else {
                    !line.starts_with(missing)
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        let error = Cdm::from_kvn(&input).unwrap_err();
        assert_eq!(error.code(), Some("validation.missing_required_field"));
        crate::common::assert_validation_field(&error, missing);
    }
}

#[test]
fn covariance_missing_required_field() {
    // Remove CR_R from first segment to trigger error
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("CR_R = 1.0 [m**2]", "");
    let err = Cdm::from_kvn(&kvn).unwrap_err();
    assert!(
        matches!(err.as_validation_error(), Some(ValidationError::MissingRequiredField { block, field, .. }) if block == "Covariance Matrix" && field == "CR_R"),
        "{err}"
    );
}

#[test]
fn version_must_be_first() {
    let kvn = r#"CREATION_DATE = 2025-01-01T00:00:00
CCSDS_CDM_VERS = 1.0
ORIGINATOR = TEST
"#;
    let err = Cdm::from_kvn(kvn).unwrap_err();
    let err = crate::common::kvn_parse_error(&err).expect("KVN syntax diagnostic");
    assert_eq!(err.message, "expected CCSDS_CDM_VERS as the first record");
}

#[test]
fn empty_file_error() {
    let error = Cdm::from_kvn("").unwrap_err();
    let parse = crate::common::kvn_parse_error(&error)
        .unwrap_or_else(|| panic!("expected a KVN parse error, got {error:?}"));
    assert_eq!(parse.message, "missing CCSDS_CDM_VERS");
    assert_eq!((parse.line, parse.column), (1, 1));
}

#[test]
fn relative_metadata_with_screen_periods() {
    let mut kvn = sample_cdm_kvn();
    // Insert screen period fields
    kvn = kvn.replace(
        "SCREEN_VOLUME_FRAME = RTN",
        "START_SCREEN_PERIOD = 2025-01-02T11:00:00\nSTOP_SCREEN_PERIOD = 2025-01-02T13:00:00\nSCREEN_VOLUME_FRAME = RTN",
    );
    kvn = kvn.replace(
        "COLLISION_PROBABILITY = 0.001",
        "SCREEN_ENTRY_TIME = 2025-01-02T11:30:00\nSCREEN_EXIT_TIME = 2025-01-02T12:30:00\nCOLLISION_PROBABILITY = 0.001\nCOLLISION_PROBABILITY_METHOD = FOSTER-1992",
    );

    let cdm = Cdm::from_kvn(&kvn).expect("should parse with screen period");
    assert!(cdm
        .body
        .relative_metadata_data
        .start_screen_period
        .is_some());
    assert!(cdm.body.relative_metadata_data.stop_screen_period.is_some());
    assert!(cdm.body.relative_metadata_data.screen_entry_time.is_some());
    assert!(cdm.body.relative_metadata_data.screen_exit_time.is_some());
    assert_eq!(
        cdm.body.relative_metadata_data.collision_probability_method,
        Some("FOSTER-1992".to_string())
    );
}

#[test]
fn relative_metadata_collision_probability_parse_error() {
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace(
        "COLLISION_PROBABILITY = 0.001",
        "COLLISION_PROBABILITY = INVALID",
    );

    let err = Cdm::from_kvn(&kvn).unwrap_err();
    assert!(crate::common::kvn_parse_error(&err).is_some(), "{err}");
}

#[test]
fn unknown_keywords_are_rejected_in_relative_metadata_metadata_and_covariance() {
    for (record, unknown) in [
        ("COLLISION_PROBABILITY = 0.001", "UNKNOWN_FIELD = VALUE"),
        ("REF_FRAME = EME2000", "UNKNOWN_META_KEY = VALUE"),
        ("CNDOT_NDOT = 1.0 [m**2/s**2]", "UNKNOWN_COV = 0.001"),
    ] {
        let input = mutated(&sample_cdm_kvn(), record, &format!("{record}\n{unknown}"));
        let error = Cdm::from_kvn(&input).unwrap_err();
        assert_eq!(error.code(), Some("parse.kvn.syntax"));
        assert!(crate::common::kvn_parse_error(&error)
            .unwrap()
            .message
            .contains("unknown CDM keyword"));
    }
}

#[test]
fn metadata_with_optional_fields() {
    let mut kvn = mutated(
        &sample_cdm_kvn(),
        "CCSDS_CDM_VERS = 1.0",
        "CCSDS_CDM_VERS = 1.0\nCOMMENT Header comment",
    );
    // Add optional metadata fields
    kvn = kvn.replace(
        "OBJECT_TYPE = PAYLOAD\nEPHEMERIS_NAME = EPH1\nCOVARIANCE_METHOD = CALCULATED\nMANEUVERABLE = YES\nREF_FRAME = EME2000",
        "OBJECT_TYPE = PAYLOAD\nOPERATOR_CONTACT_POSITION = Flight Director\nOPERATOR_ORGANIZATION = NASA\nOPERATOR_PHONE = +1-555-1234\nOPERATOR_EMAIL = contact@nasa.gov\nEPHEMERIS_NAME = EPH1\nCOVARIANCE_METHOD = CALCULATED\nMANEUVERABLE = YES\nORBIT_CENTER = EARTH\nREF_FRAME = EME2000\nGRAVITY_MODEL = EGM-96\nATMOSPHERIC_MODEL = JACCHIA 70 DCA\nN_BODY_PERTURBATIONS = MOON, SUN\nSOLAR_RAD_PRESSURE = YES\nEARTH_TIDES = YES\nINTRACK_THRUST = YES",
    );

    let cdm = Cdm::from_kvn(&kvn).expect("should parse with optional metadata");
    let seg1 = &cdm.body.segments[0];
    assert_eq!(seg1.metadata.object_type, Some(ObjectDescription::Payload));
    assert_eq!(
        seg1.metadata.operator_contact_position,
        Some("Flight Director".to_string())
    );
    assert_eq!(
        seg1.metadata.operator_organization,
        Some("NASA".to_string())
    );
    assert_eq!(
        seg1.metadata.operator_phone,
        Some("+1-555-1234".to_string())
    );
    assert_eq!(
        seg1.metadata.operator_email,
        Some("contact@nasa.gov".to_string())
    );
    assert_eq!(seg1.metadata.orbit_center, Some("EARTH".to_string()));
    assert_eq!(seg1.metadata.gravity_model, Some("EGM-96".to_string()));
    assert_eq!(
        seg1.metadata.atmospheric_model,
        Some("JACCHIA 70 DCA".to_string())
    );
    assert_eq!(
        seg1.metadata.n_body_perturbations,
        Some("MOON, SUN".to_string())
    );
    assert_eq!(seg1.metadata.solar_rad_pressure, Some(YesNo::Yes));
    assert_eq!(seg1.metadata.earth_tides, Some(YesNo::Yes));
    assert_eq!(seg1.metadata.intrack_thrust, Some(YesNo::Yes));
    assert_eq!(Cdm::from_kvn(&cdm.to_kvn().unwrap()).unwrap(), cdm);
    assert_eq!(Cdm::from_xml(&cdm.to_xml().unwrap()).unwrap(), cdm);
}

#[test]
fn closed_metadata_vocabularies_reject_unknown_tokens() {
    for (from, to) in [
        ("SCREEN_VOLUME_FRAME = RTN", "SCREEN_VOLUME_FRAME = BAD"),
        ("SCREEN_VOLUME_SHAPE = BOX", "SCREEN_VOLUME_SHAPE = BALL"),
        ("OBJECT = OBJECT1", "OBJECT = OBJECT3"),
        (
            "COVARIANCE_METHOD = CALCULATED",
            "COVARIANCE_METHOD = INVALID",
        ),
        ("MANEUVERABLE = YES", "MANEUVERABLE = MAYBE"),
        ("REF_FRAME = EME2000", "REF_FRAME = INVALID"),
        ("OBJECT_TYPE = PAYLOAD", "OBJECT_TYPE = SATELLITE"),
    ] {
        let kvn = mutated(&sample_cdm_kvn(), from, to);
        let error = Cdm::from_kvn(&kvn).unwrap_err();
        assert!(
            matches!(error.as_format_error(), Some(FormatError::Enum(_))),
            "{to}: {error}"
        );
    }
}

#[test]
fn metadata_object_types() {
    for (token, expected) in [
        ("ROCKET BODY", ObjectDescription::RocketBody),
        ("DEBRIS", ObjectDescription::Debris),
        ("UNKNOWN", ObjectDescription::Unknown),
        ("OTHER", ObjectDescription::Other),
    ] {
        let kvn = mutated(
            &sample_cdm_kvn(),
            "OBJECT_TYPE = PAYLOAD",
            &format!("OBJECT_TYPE = {token}"),
        );
        let cdm = Cdm::from_kvn(&kvn).unwrap();
        assert_eq!(
            cdm.body.segments[0].metadata.object_type,
            Some(expected),
            "{token}"
        );
    }
}

#[test]
fn metadata_maneuverable_na() {
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("MANEUVERABLE = YES", "MANEUVERABLE = N/A");
    let cdm = Cdm::from_kvn(&kvn).expect("parse");
    assert_eq!(
        cdm.body.segments[0].metadata.maneuverable,
        ManeuverableType::NA
    );
}

#[test]
fn metadata_ref_frames() {
    for (token, expected) in [
        ("GCRF", ReferenceFrameType::Gcrf),
        ("ITRF", ReferenceFrameType::Itrf),
    ] {
        let kvn = mutated(
            &sample_cdm_kvn(),
            "REF_FRAME = EME2000",
            &format!("REF_FRAME = {token}"),
        );
        let cdm = Cdm::from_kvn(&kvn).unwrap();
        assert_eq!(cdm.body.segments[0].metadata.ref_frame, expected);
    }
}

#[test]
fn data_with_additional_parameters() {
    let mut kvn = sample_cdm_kvn();
    // Insert additional parameters
    kvn = kvn.replace(
        "X = 1.0 [km]",
        "AREA_PC = 10.0 [m**2]\nAREA_DRG = 12.0 [m**2]\nAREA_SRP = 15.0 [m**2]\nMASS = 1000.0 [kg]\nCD_AREA_OVER_MASS = 0.012 [m**2/kg]\nCR_AREA_OVER_MASS = 0.015 [m**2/kg]\nTHRUST_ACCELERATION = 0.001 [m/s**2]\nSEDR = 0.05 [W/kg]\nX = 1.0 [km]",
    );

    let cdm = Cdm::from_kvn(&kvn).expect("should parse with additional parameters");
    let ap = cdm.body.segments[0]
        .data
        .additional_parameters
        .as_ref()
        .unwrap();
    assert!(ap.area_pc.is_some());
    assert!(ap.area_drg.is_some());
    assert!(ap.area_srp.is_some());
    assert!(ap.mass.is_some());
    assert!(ap.cd_area_over_mass.is_some());
    assert!(ap.cr_area_over_mass.is_some());
    assert!(ap.thrust_acceleration.is_some());
    assert!(ap.sedr.is_some());
    assert_eq!(Cdm::from_kvn(&cdm.to_kvn().unwrap()).unwrap(), cdm);
    assert_eq!(Cdm::from_xml(&cdm.to_xml().unwrap()).unwrap(), cdm);
}

#[test]
fn covariance_with_drag_fields() {
    let kvn = with_optional_covariance_rows(&sample_cdm_kvn(), DRAG_COVARIANCE_ROW);

    let cdm = Cdm::from_kvn(&kvn).expect("should parse with CDRG fields");
    let cov = &cdm.body.segments[0]
        .data
        .covariance_matrix
        .as_ref()
        .expect("Covariance matrix missing");
    assert!(cov.cdrg_r.is_some());
    assert!(cov.cdrg_t.is_some());
    assert!(cov.cdrg_n.is_some());
    assert!(cov.cdrg_rdot.is_some());
    assert!(cov.cdrg_tdot.is_some());
    assert!(cov.cdrg_ndot.is_some());
    assert!(cov.cdrg_drg.is_some());
}

#[test]
fn covariance_with_srp_fields() {
    let rows = format!("{DRAG_COVARIANCE_ROW}\n{SRP_COVARIANCE_ROW}");
    let kvn = with_optional_covariance_rows(&sample_cdm_kvn(), &rows);

    let cdm = Cdm::from_kvn(&kvn).expect("should parse with CSRP fields");
    let cov = &cdm.body.segments[0]
        .data
        .covariance_matrix
        .as_ref()
        .expect("Covariance matrix missing");
    assert!(cov.csrp_r.is_some());
    assert!(cov.csrp_t.is_some());
    assert!(cov.csrp_n.is_some());
    assert!(cov.csrp_rdot.is_some());
    assert!(cov.csrp_tdot.is_some());
    assert!(cov.csrp_ndot.is_some());
    assert!(cov.csrp_drg.is_some());
    assert!(cov.csrp_srp.is_some());
}

#[test]
fn covariance_with_thrust_fields() {
    let rows = format!("{DRAG_COVARIANCE_ROW}\n{SRP_COVARIANCE_ROW}\n{THRUST_COVARIANCE_ROW}");
    let kvn = with_optional_covariance_rows(&sample_cdm_kvn(), &rows);

    let cdm = Cdm::from_kvn(&kvn).expect("should parse with CTHR fields");
    let cov = &cdm.body.segments[0]
        .data
        .covariance_matrix
        .as_ref()
        .expect("Covariance matrix missing");
    assert!(cov.cthr_r.is_some());
    assert!(cov.cthr_t.is_some());
    assert!(cov.cthr_n.is_some());
    assert!(cov.cthr_rdot.is_some());
    assert!(cov.cthr_tdot.is_some());
    assert!(cov.cthr_ndot.is_some());
    assert!(cov.cthr_drg.is_some());
    assert!(cov.cthr_srp.is_some());
    assert!(cov.cthr_thr.is_some());
    assert_eq!(Cdm::from_kvn(&cdm.to_kvn().unwrap()).unwrap(), cdm);
    assert_eq!(Cdm::from_xml(&cdm.to_xml().unwrap()).unwrap(), cdm);
}

#[test]
fn covariance_rejects_optional_rows_without_preceding_rows() {
    for (label, rows, missing) in [
        ("SRP without drag", SRP_COVARIANCE_ROW, "CDRG_R"),
        (
            "thrust without drag or SRP",
            THRUST_COVARIANCE_ROW,
            "CDRG_R",
        ),
    ] {
        let kvn = with_optional_covariance_rows(&sample_cdm_kvn(), rows);
        let error = Cdm::from_kvn(&kvn).expect_err(label);
        assert_eq!(error.code(), Some("validation.missing_required_field"));
        assert!(error.to_string().contains(missing), "{label}: {error}");
    }
}

#[test]
fn unexpected_segment_start() {
    let kvn = r###"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
META_START
"###;
    let error = Cdm::from_kvn(kvn).unwrap_err();
    let parse = crate::common::kvn_parse_error(&error)
        .unwrap_or_else(|| panic!("expected a KVN parse error, got {error:?}"));
    assert_eq!(parse.message, "expected exactly one CDM assignment");
    assert_eq!(parse.line, 8, "diagnostic did not point at META_START");
}

/// A message that ends inside the relative metadata block. The parser does not report this as
/// an end-of-input condition — `UnexpectedEof` is raised only by format detection and by the
/// combined-NDM reader — it reports the first mandatory field the truncated block never
/// supplied, which is the diagnostic a producer can act on.
#[test]
fn truncated_relative_metadata_names_the_missing_field() {
    let kvn = r###"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
TCA = 2025-01-02T12:00:00
"###;
    let error = Cdm::from_kvn(kvn).unwrap_err();
    let validation = error
        .as_validation_error()
        .unwrap_or_else(|| panic!("expected a validation error, got {error:?}"));
    assert!(
        matches!(
            validation,
            ValidationError::MissingRequiredField { block, field, .. }
                if block == "Relative Metadata" && field == "MISS_DISTANCE"
        ),
        "unexpected diagnostic: {validation:?}"
    );
}

#[test]
fn test_parse_cdm_missing_covariance() {
    let base = sample_cdm_kvn();
    let cdm_no_cov = base
        .lines()
        .filter(|line| !line.contains("[m**2"))
        .collect::<Vec<_>>()
        .join("\n");
    let err = Cdm::from_kvn(&cdm_no_cov).unwrap_err();
    assert!(
        matches!(err.as_validation_error(), Some(ValidationError::MissingRequiredField { field, .. }) if field == "covarianceMatrix"),
        "{err}"
    );
}

#[test]
fn cdm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let tca = "TCA = 2010-03-13T22:37:52.618";
    let miss = "MISS_DISTANCE = 715 [m]";
    let object = "OBJECT = OBJECT1";
    for (label, source) in [
        (
            "duplicate relative keyword",
            mutated(KVN, miss, &format!("{miss}\n{miss}")),
        ),
        (
            "reordered relative keywords",
            mutated(KVN, &format!("{tca}\n{miss}"), &format!("{miss}\n{tca}")),
        ),
        (
            "comment inside relative block",
            mutated(KVN, miss, &format!("{miss}\nCOMMENT misplaced")),
        ),
        ("unknown marked block", mutated(KVN, object, "META_START")),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            mutated(KVN, object, "OBJECT = OBJECT1 €"),
        ),
    ] {
        let error = Cdm::from_kvn(&source).unwrap_err();
        assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
    }
}

#[test]
fn cdm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let tca = "<TCA>2010-03-13T22:37:52.618</TCA>";
    let miss = "<MISS_DISTANCE units=\"m\">715</MISS_DISTANCE>";
    let object = "<OBJECT>OBJECT1</OBJECT>";
    for (label, source) in [
        (
            "unknown data child",
            mutated(XML, "<data>", "<data><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown covariance child",
            mutated(
                XML,
                "<covarianceMatrix>",
                "<covarianceMatrix><UNKNOWN>1</UNKNOWN>",
            ),
        ),
        (
            "unknown container attribute",
            mutated(XML, "<segment>", "<segment unexpected=\"value\">"),
        ),
        (
            "unknown leaf attribute",
            mutated(
                XML,
                miss,
                "<MISS_DISTANCE units=\"m\" unexpected=\"value\">715</MISS_DISTANCE>",
            ),
        ),
        (
            "duplicate metadata child",
            mutated(XML, object, &format!("{object}{object}")),
        ),
        (
            "reordered relative children",
            mutated(XML, &format!("{tca}\n{miss}"), &format!("{miss}\n{tca}")),
        ),
        (
            "illegal nil attribute",
            mutated(XML, tca, "<TCA nil=\"true\"/>"),
        ),
    ] {
        let error = Cdm::from_xml(&source).unwrap_err();
        assert!(
            matches!(
                error.as_format_error(),
                Some(ccsds_ndm::error::FormatError::InvalidFormat(_))
            ),
            "{label}: {error}"
        );
    }
}

#[test]
fn cdm_kvn_rejects_a_state_vector_with_metre_units() {
    let source = include_str!("../../data/kvn/cdm_364.kvn");
    Cdm::from_kvn(source).expect("baseline fixture must parse");
    let input = mutated_once(
        source,
        "X = -41600.46272465 [km]",
        "X = -41600.46272465 [m]",
    );
    let error = Cdm::from_kvn(&input)
        .expect_err("CDM state-vector positions are kilometres, not arbitrary length units");
    assert!(
        matches!(error.as_format_error(), Some(FormatError::Enum(cause))
            if cause.field == "X" && cause.value == "m" && cause.expected == "\"km\""),
        "{error}"
    );
}

#[test]
fn xml_required_unit_attributes_are_not_inferred() {
    Cdm::from_xml(XML).expect("baseline fixture must parse");
    let input = mutated_once(XML, "<X units=\"km\">", "<X>");
    let error = Cdm::from_xml(&input).expect_err("CDM XML state-vector positions require units");
    let diagnostic = error.to_string();
    assert!(
        diagnostic.to_lowercase().contains("missing") && diagnostic.contains("units"),
        "{error}"
    );
}

#[test]
fn all_normative_line_endings_preserve_the_message() {
    let expected = Cdm::from_kvn(crate::KVN).unwrap();
    for ending in ["\n", "\r", "\r\n", "\n\r"] {
        let input = crate::KVN.replace('\n', ending);
        assert_eq!(Cdm::from_kvn(&input).unwrap(), expected, "{ending:?}");
    }
}

#[test]
fn xml_accepts_schema_listed_lowercase_categories() {
    let baseline = Cdm::from_xml(XML).unwrap();
    for (upper, lower) in [
        ("OBJECT1", "object1"),
        ("OBJECT2", "object2"),
        ("RTN", "rtn"),
        ("ELLIPSOID", "ellipsoid"),
        ("EME2000", "eme2000"),
        ("CALCULATED", "calculated"),
        ("YES", "yes"),
    ] {
        let xml = mutated(XML, &format!(">{upper}<"), &format!(">{lower}<"));
        crate::common::validate_xml("lowercase CDM", &xml);
        assert_eq!(Cdm::from_xml(&xml).unwrap(), baseline, "{lower}");
    }
    let invalid = mutated(XML, ">CALCULATED<", ">Calculated<");
    assert!(Cdm::from_xml(&invalid).is_err());
}

#[test]
fn namespace_qualified_xml_reads_as_the_unqualified_message() {
    // Cor. 2 (4.3.3.3) adds the `ndm:`-qualified form; generation stays unqualified.
    let qualified = XML.replace("</", "</ndm:").replace('<', "<ndm:");
    let qualified = qualified
        .replace("<ndm:?", "<?")
        .replace("<ndm:/ndm:", "</ndm:");
    let message = Cdm::from_xml(&qualified).unwrap();
    assert_eq!(message, Cdm::from_xml(XML).unwrap());
    assert!(message.to_xml().unwrap().contains("<cdm "));
}

#[test]
fn originator_accepts_the_books_esa_sst_example_and_optional_na_is_absent() {
    let mut message = Cdm::from_kvn(KVN).unwrap();
    message.header.originator = "ESA SST".into();
    let xml = message.to_xml().unwrap();
    assert_eq!(Cdm::from_xml(&xml).unwrap().header.originator, "ESA SST");
    let kvn = message.to_kvn().unwrap();
    assert_eq!(Cdm::from_kvn(&kvn).unwrap().header.originator, "ESA SST");
    let xml = xml.replace(
        "<ORIGINATOR>ESA SST</ORIGINATOR>",
        "<ORIGINATOR>N/A</ORIGINATOR>",
    );
    assert!(Cdm::from_xml(&xml).is_err());
}

#[test]
fn xml_na_is_cdm_specific_validates_units_and_refuses_lossy_output() {
    let source = Cdm::from_kvn(include_str!("../../data/kvn/cdm_362.kvn"))
        .unwrap()
        .to_xml()
        .unwrap();
    let source = source.replacen(
        "<stateVector>",
        "<additionalParameters><MASS units='kg'>1</MASS></additionalParameters><stateVector>",
        1,
    );
    let start = source.find("<MASS").unwrap();
    let end = start + source[start..].find("</MASS>").unwrap() + "</MASS>".len();
    for value in ["N/A", "<![CDATA[N/A]]>", "N&#47;A", "N<!--split-->/A"] {
        let xml = format!(
            "{}<MASS units='kg'>{value}</MASS>{}",
            &source[..start],
            &source[end..]
        );
        let message = Cdm::from_xml(&xml).unwrap();
        assert!(message.body.segments[0]
            .data
            .additional_parameters
            .as_ref()
            .is_none_or(|parameters| parameters.mass.is_none()));
        let invalid = xml.replace("units='kg'", "units='lb'");
        assert!(Cdm::from_xml(&invalid).is_err());
    }
    let mut message = Cdm::from_xml(&source).unwrap();
    message.header.message_for = Some("N/A".into());
    assert!(message
        .to_xml()
        .unwrap_err()
        .to_string()
        .contains("MESSAGE_FOR"));
    let mut output = Vec::new();
    assert!(message.write_xml_to(&mut output).is_err());
    assert!(output.is_empty());
    message.header.message_for = None;
    message.header.comment.push("N/A".into());
    assert_eq!(Cdm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
}

#[test]
fn cdm_originator_accepts_embedded_blanks_and_normalizes_xml_padding() {
    let message = Cdm::from_kvn(include_str!("../../data/kvn/cdm_362.kvn")).unwrap();
    let xml = message.to_xml().unwrap();
    let opening = xml.find("<ORIGINATOR>").unwrap() + "<ORIGINATOR>".len();
    let end = opening + xml[opening..].find('<').unwrap();
    let padded = format!("{} ESA SST {}", &xml[..opening], &xml[end..]);
    let mut message = Cdm::from_xml(&padded).unwrap();
    assert_eq!(message.header.originator, "ESA SST");
    message.to_kvn().unwrap();
    message.header.originator = " ESA SST ".into();
    let error = message.to_kvn().unwrap_err();
    assert!(error.to_string().contains("ORIGINATOR"));
}

#[test]
fn xml_unit_diagnostics_do_not_depend_on_comments_or_other_unavailable_values() {
    let source = Cdm::from_kvn(include_str!("../../data/kvn/cdm_362.kvn"))
        .unwrap()
        .to_xml()
        .unwrap();
    let source = source.replacen(
        "<stateVector>",
        "<additionalParameters><MASS units='lb'>1</MASS></additionalParameters><stateVector>",
        1,
    );
    let baseline = Cdm::from_xml(&source).unwrap_err();
    assert_eq!(baseline.code(), Some("parse.xml.syntax"));
    let baseline = baseline.diagnostic().unwrap();
    assert!(baseline.source_location.is_some());
    for extra in [
        "<!--unrelated-->",
        "<?test unrelated?>",
        "<COMMENT><![CDATA[unrelated]]></COMMENT>",
        "<COMMENT>N/A</COMMENT>",
    ] {
        let xml = source.replacen("</cdm>", &format!("{extra}</cdm>"), 1);
        // COMMENT elements belong in the header, unlike XML comments and PIs.
        let xml = if extra.starts_with("<COMMENT>") {
            source.replacen("<CREATION_DATE>", &format!("{extra}<CREATION_DATE>"), 1)
        } else {
            xml
        };
        let error = Cdm::from_xml(&xml).unwrap_err();
        assert_eq!(error.code(), Some("parse.xml.syntax"), "{extra}: {error}");
        assert!(
            error.diagnostic().unwrap().source_location.is_some(),
            "{extra}"
        );
    }
}

#[test]
fn unavailable_area_and_mass_keep_fixed_unit_checks_and_locations() {
    let source = Cdm::from_kvn(include_str!("../../data/kvn/cdm_362.kvn"))
        .unwrap()
        .to_xml()
        .unwrap();
    for (field, unit) in [
        ("MASS", "kg"),
        ("AREA_PC", "m**2"),
        ("CD_AREA_OVER_MASS", "m**2/kg"),
    ] {
        for extra in ["", "<!--unrelated-->"] {
            let xml = source.replacen("<stateVector>", &format!(
                "{extra}<additionalParameters><{field} units='{unit}'>N/A</{field}></additionalParameters><stateVector>"), 1);
            Cdm::from_xml(&xml).unwrap();
            let error =
                Cdm::from_xml(&xml.replace(&format!("units='{unit}'"), "units='lb'")).unwrap_err();
            assert_eq!(error.code(), Some("parse.xml.syntax"));
            assert!(error.diagnostic().unwrap().source_location.is_some());
        }
    }
}

#[test]
fn xml_identifier_padding_is_normalized_without_allowing_internal_blanks() {
    let message = Cdm::from_kvn(crate::KVN).unwrap();
    let mut xml = message.to_xml().unwrap();
    for field in [
        "ORIGINATOR",
        "MESSAGE_ID",
        "OBJECT_DESIGNATOR",
        "CATALOG_NAME",
        "INTERNATIONAL_DESIGNATOR",
    ] {
        xml = xml
            .replace(&format!("<{field}>"), &format!("<{field}> \t"))
            .replace(&format!("</{field}>"), &format!("\r\n </{field}>"));
    }
    assert_eq!(Cdm::from_xml(&xml).unwrap(), message);
    assert!(Cdm::from_xml(&xml.replace("<MESSAGE_ID> \t", "<MESSAGE_ID> A B")).is_err());
}

#[test]
fn xml_empty_units_attribute_is_an_absent_annotation() {
    // 18 SDS CDMExample_issue1458.xml writes `<RESIDUALS_ACCEPTED units=""/>`.
    let original = r#"<RESIDUALS_ACCEPTED units="%" >97.8</RESIDUALS_ACCEPTED>"#;
    let absent = crate::XML.replacen(original, r#"<RESIDUALS_ACCEPTED units=""/>"#, 1);
    let cdm = Cdm::from_xml(&absent).unwrap();
    let od = cdm.body.segments[0].data.od_parameters.as_ref().unwrap();
    assert!(od.residuals_accepted.is_none());

    let valued = crate::XML.replacen(
        original,
        r#"<RESIDUALS_ACCEPTED units="">97.8</RESIDUALS_ACCEPTED>"#,
        1,
    );
    let cdm = Cdm::from_xml(&valued).unwrap();
    let od = cdm.body.segments[0].data.od_parameters.as_ref().unwrap();
    assert_eq!(od.residuals_accepted.as_ref().unwrap().value, 97.8);
    let unannotated = mutated_once(&valued, r#" units="""#, "");
    assert_eq!(cdm, Cdm::from_xml(&unannotated).unwrap());
    let incompatible = mutated_once(&valued, r#"units="""#, r#"units="m""#);
    Cdm::from_xml(&incompatible).expect_err("valued incompatible units still fail");

    let wrong = crate::XML.replacen(original, r#"<RESIDUALS_ACCEPTED units="m"/>"#, 1);
    Cdm::from_xml(&wrong).expect_err("nonempty incompatible units still fail");
}
