use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::traits::Ndm;

#[test]
fn opm_preserves_comment_before_user_defined_after_optional_maneuvers() {
    let source = include_str!("../../data/kvn/opm_g4.kvn").replace(
        "USER_DEFINED_EARTH_MODEL",
        "COMMENT belongs to user parameters\nUSER_DEFINED_EARTH_MODEL",
    );
    let message = Opm::from_kvn(&source).unwrap();
    assert_eq!(
        message
            .body
            .segment
            .data
            .user_defined_parameters
            .as_ref()
            .unwrap()
            .comment,
        ["belongs to user parameters"]
    );
}

#[test]
fn opm_rejects_maneuver_fields_without_ignition_epoch() {
    let source = include_str!("../../data/kvn/opm_g4.kvn").replace(
        "USER_DEFINED_EARTH_MODEL",
        "MAN_DURATION = 10.0 [s]\n\
         MAN_DELTA_MASS = -1.0 [kg]\n\
         MAN_REF_FRAME = TNW\n\
         MAN_DV_1 = 0.0 [km/s]\n\
         MAN_DV_2 = 0.0 [km/s]\n\
         MAN_DV_3 = 0.0 [km/s]\n\
         USER_DEFINED_EARTH_MODEL",
    );
    assert!(Opm::from_kvn(&source).is_err());
}

#[test]
fn oem_preserves_the_first_covariance_comment() {
    let source = include_str!("../../data/kvn/oem_g13.kvn").replace(
        "COVARIANCE_START\n",
        "COVARIANCE_START\nCOMMENT belongs to first covariance\n",
    );
    let message = Oem::from_kvn(&source).unwrap();
    assert_eq!(
        message.body.segment[0].data.covariance_matrix[0].comment,
        ["belongs to first covariance"]
    );
}

#[test]
fn oem_enforces_integer_range_but_accepts_large_fixed_point_numbers() {
    let source = include_str!("../../data/kvn/oem_g13.kvn");
    let integer = source.replacen("-2432.166", "3000000000", 1);
    let decimal = source.replacen("-2432.166", "3000000000.0", 1);
    Oem::from_kvn(&integer).expect_err("ODM integer-form values are limited to signed 32-bit");
    let decimal_message = Oem::from_kvn(&decimal).unwrap();
    assert_eq!(
        decimal_message.body.segment[0].data.state_vector[0].x.value,
        3_000_000_000.0
    );
}

#[test]
fn user_defined_values_may_contain_assignment_delimiters() {
    let omm_source = include_str!("../../data/kvn/omm_g9.kvn").replace(
        "USER_DEFINED_EARTH_MODEL = WGS-84",
        "USER_DEFINED_EARTH_MODEL = a=b",
    );
    let omm = Omm::from_kvn(&omm_source).unwrap();
    assert_eq!(
        omm.body
            .segment
            .data
            .user_defined_parameters
            .as_ref()
            .unwrap()
            .user_defined[0]
            .value,
        "a=b"
    );

    let rdm_source = format!(
        "{}\nUSER_DEFINED_EQUATION = a=b\n",
        include_str!("../../data/kvn/rdm_c1.kvn")
    );
    let rdm = Rdm::from_kvn(&rdm_source).unwrap();
    assert_eq!(
        rdm.body
            .segment
            .data
            .user_defined_parameters
            .as_ref()
            .unwrap()
            .user_defined[0]
            .value,
        "a=b"
    );
}
