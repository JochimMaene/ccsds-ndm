use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::{Ndm, Validate};

const BASE: &str = include_str!("../../data/kvn/ocm_g15.kvn");
const LINKED: &str = include_str!("../../data/kvn/ocm_g19.kvn");

#[test]
fn contact_keywords_round_trip_and_refuse_old_xml_without_output() {
    for key in [
        "OPERATOR_URL",
        "OPERATOR_EMAIL",
        "OPERATOR_PHONE",
        "OPERATOR_AUTOMATED_URL",
        "EMERGENCY_EMAIL",
        "EMERGENCY_PHONE",
        "EMERGENCY_AVAILABILITY",
        "SSA_EMAIL",
        "SSA_PHONE",
        "OWNER_URL",
    ] {
        let source = BASE.replace(
            "TIME_SYSTEM =",
            &format!("{key} = contact value\nTIME_SYSTEM ="),
        );
        let message = Ocm::from_kvn(&source).unwrap();
        assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
        assert!(message.to_xml().unwrap_err().to_string().contains(key));
        let mut output = Vec::new();
        assert!(message.write_xml_to(&mut output).is_err());
        assert!(output.is_empty());
    }
}

#[test]
fn covariance_links_are_required_resolved_and_unambiguous() {
    let message = Ocm::from_kvn(LINKED).unwrap();
    assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    assert!(message.to_xml().is_err());
    assert!(Ocm::from_kvn(include_str!(
        "../../data/invalid/ocm_g19_pre_corrigendum.kvn"
    ))
    .is_err());
    let mut edited = message.clone();
    edited.body.segment.data.cov[0].traj_id = None;
    assert!(edited.validate().is_err());
    edited.body.segment.data.cov[0].traj_id = Some("unknown".into());
    assert!(edited.validate().is_err());
    let trajectory = edited.body.segment.data.traj[0].clone();
    edited.body.segment.data.traj.push(trajectory);
    edited.body.segment.data.cov[0].traj_id = Some("G19_TRAJECTORY".into());
    assert!(
        edited.validate().is_err(),
        "duplicate trajectory identifiers"
    );
    edited.body.segment.data.traj[1].traj_id = Some("second".into());
    edited.validate().unwrap();
    edited.body.segment.data.traj[1].traj_id = None;
    assert!(edited.validate().is_err());
}

#[test]
fn sequences_preserve_rows_and_parameter_order_and_refuse_xml() {
    let source = format!("{BASE}\nUSER_START\nUSER_DEFINED_BEFORE = scalar\nUSER_DEFINED_COMPOSITION_GNSS = TIME_ABSOLUTE, PDOP\n2025-03-12T01:56:26.681583 2.1\n2025-03-12T01:56:46.267761 1.7\nUSER_DEFINED_AFTER = next\nUSER_DEFINED_COMPOSITION_EVENTS = EVENT, VALUE\n1 CODE=A\nUSER_EVENT 1\nUSER_STOP_EVENT 2\nCOMMENT opaque ICD row\nTRAJ_START\nUSER_STOP\n");
    let message = Ocm::from_kvn(&source).unwrap();
    let parameters = &message
        .body
        .segment
        .data
        .user
        .as_ref()
        .unwrap()
        .parameters()
        .collect::<Vec<_>>();
    assert_eq!(parameters.len(), 4);
    assert_eq!(parameters[1].records.len(), 2);
    assert_eq!(parameters[2].value, "next");
    assert_eq!(
        parameters[3].records,
        [
            "1 CODE=A",
            "USER_EVENT 1",
            "USER_STOP_EVENT 2",
            "COMMENT opaque ICD row",
            "TRAJ_START"
        ]
    );
    let generated = message.to_kvn().unwrap();
    assert_eq!(Ocm::from_kvn(&generated).unwrap(), message);
    let mut streamed = Vec::new();
    message.write_kvn_to(&mut streamed).unwrap();
    assert_eq!(streamed, generated.as_bytes());
    assert!(message.to_xml().is_err());
    let mut output = Vec::new();
    assert!(message.write_xml_to(&mut output).is_err());
    assert!(output.is_empty());
    let mut edited = message;
    let parameter = &mut edited
        .body
        .segment
        .data
        .user
        .as_mut()
        .unwrap()
        .parameters_mut()
        .nth(1)
        .unwrap();
    parameter.records[0] = "USER_STOP\nINJECTION = value".into();
    assert!(edited.to_kvn().is_err());
}

#[test]
fn sequences_reject_missing_id_rows_and_composition_columns() {
    for block in [
        "USER_DEFINED_COMPOSITION_ = TIME, VALUE\n0 1\n",
        "USER_DEFINED_COMPOSITION_TEST = TIME, VALUE\n",
        "USER_DEFINED_COMPOSITION_TEST = TIME,,VALUE\n0 1\n",
        "0 1\n",
    ] {
        assert!(
            Ocm::from_kvn(&format!("{BASE}\nUSER_START\n{block}USER_STOP\n")).is_err(),
            "{block}"
        );
    }
}

#[test]
fn covariance_dimensions_orderings_and_positive_definiteness() {
    use ccsds_ndm::messages::ocm::{CovLine, OcmCovarianceMatrix};
    use ccsds_ndm::types::CovOrder;
    for (kind, dimension) in [
        ("CARTP", 3),
        ("CARTPV", 6),
        ("TCARTPV", 7),
        ("TCARTPVA", 10),
    ] {
        for ordering in [
            CovOrder::Ltm,
            CovOrder::Utm,
            CovOrder::Full,
            CovOrder::LtmWcc,
            CovOrder::UtmWcc,
        ] {
            let mut values = Vec::new();
            for i in 0..dimension {
                for j in 0..dimension {
                    if (matches!(ordering, CovOrder::Ltm) && j > i)
                        || (matches!(ordering, CovOrder::Utm) && j < i)
                    {
                        continue;
                    }
                    values.push(if i == j { 1.0 } else { 0.0 });
                }
            }
            let covariance = OcmCovarianceMatrix::builder()
                .cov_ref_frame("GCRF")
                .cov_type(kind)
                .cov_ordering(ordering)
                .cov_lines(vec![CovLine {
                    epoch: "0".parse().unwrap(),
                    values,
                }])
                .build();
            let mut message = Ocm::from_kvn(BASE).unwrap();
            message.body.segment.data.cov = vec![covariance];
            message.validate().unwrap();
            assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
            let mut short = message.clone();
            short.body.segment.data.cov[0].cov_lines[0].values.pop();
            assert!(short.validate().is_err());
            message.body.segment.data.cov[0].cov_lines[0].values[0] = 0.0;
            assert!(message.validate().is_err(), "singular {kind}");
        }
    }
    let mut message = Ocm::from_kvn(BASE).unwrap();
    let covariance = OcmCovarianceMatrix::builder()
        .cov_ref_frame("GCRF")
        .cov_type("CARTP")
        .cov_lines(vec![CovLine {
            epoch: "0".parse().unwrap(),
            values: vec![1., 2., 1., 0., 0., 1.],
        }])
        .build();
    message.body.segment.data.cov = vec![covariance];
    assert!(
        message.validate().is_err(),
        "positive diagonal alone does not establish positive definiteness"
    );
    message.body.segment.data.traj[0].traj_lines[0].values.pop();
    assert!(message.validate().is_err(), "trajectory dimensions");
}

#[test]
fn covariance_reconstructs_triangle_directions_and_checks_correlations() {
    use ccsds_ndm::messages::ocm::{CovLine, OcmCovarianceMatrix};
    use ccsds_ndm::types::CovOrder;
    for (ordering, values) in [
        (CovOrder::Ltm, vec![4., 1., 16., 0., 0., 9.]),
        (CovOrder::Utm, vec![4., 1., 0., 16., 0., 9.]),
        (CovOrder::Full, vec![4., 1., 0., 1., 16., 0., 0., 0., 9.]),
        (
            CovOrder::LtmWcc,
            vec![4., 0.125, 0., 1., 16., 0., 0., 0., 9.],
        ),
        (
            CovOrder::UtmWcc,
            vec![4., 1., 0., 0.125, 16., 0., 0., 0., 9.],
        ),
    ] {
        let mut message = Ocm::from_kvn(BASE).unwrap();
        message.body.segment.data.cov = vec![OcmCovarianceMatrix::builder()
            .cov_ref_frame("GCRF")
            .cov_type("CARTP")
            .cov_ordering(ordering.clone())
            .cov_lines(vec![CovLine {
                epoch: "0".parse().unwrap(),
                values,
            }])
            .build()];
        message.validate().unwrap();
        assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
        if matches!(
            ordering,
            CovOrder::LtmWcc | CovOrder::UtmWcc | CovOrder::Full
        ) {
            let index = if matches!(ordering, CovOrder::UtmWcc) {
                3
            } else {
                1
            };
            message.body.segment.data.cov[0].cov_lines[0].values[index] = 0.5;
            assert!(message.validate().is_err());
        }
    }
}

#[test]
fn covariance_eigenvectors_units_and_scalar_domains() {
    use ccsds_ndm::messages::ocm::{CovLine, OcmCovarianceMatrix};
    for kind in ["SIG3EIGVEC3", "TSIG3EIGVEC3"] {
        let mut values = vec![3., 2., 1., 1., 0., 0., 0., 1., 0., 0., 0., 1.];
        if kind.starts_with('T') {
            values.insert(0, 1.);
        }
        let mut message = Ocm::from_kvn(BASE).unwrap();
        message.body.segment.data.cov = vec![OcmCovarianceMatrix::builder()
            .cov_ref_frame("GCRF")
            .cov_type(kind)
            .cov_lines(vec![CovLine {
                epoch: "0".parse().unwrap(),
                values,
            }])
            .build()];
        message.validate().unwrap();
        let last = message.body.segment.data.cov[0].cov_lines[0]
            .values
            .last_mut()
            .unwrap();
        *last = 0.;
        assert!(message.validate().is_err(), "degenerate eigenvector basis");
    }
    let mut message = Ocm::from_kvn(LINKED).unwrap();
    message.body.segment.data.cov[0].cov_scale_min = Some(f64::NAN);
    assert!(message.validate().is_err());
    message.body.segment.data.cov[0].cov_scale_min = None;
    message.body.segment.data.cov[1].cov_units = Some("[km, km]".into());
    assert!(message.validate().is_err());
    message.body.segment.data.cov[1].cov_units = Some("[m, m, m]".into());
    assert!(message.validate().is_err());
    message.body.segment.data.cov[1].cov_units = Some("[km, km, km]".into());
    message.body.segment.data.traj[0].traj_units = Some("[km]".into());
    assert!(message.validate().is_err());
}

#[test]
fn published_g15_extra_acceleration_requires_the_correct_composition() {
    assert!(Ocm::from_kvn(include_str!("../../data/invalid/ocm_g15_original.kvn")).is_err());
    Ocm::from_kvn(BASE).unwrap();
}

#[test]
fn rounded_covariance_must_retain_record_constraints_before_kvn_output() {
    use ccsds_ndm::messages::ocm::{CovLine, OcmCovarianceMatrix};
    let mut message = Ocm::from_kvn(BASE).unwrap();
    message.body.segment.data.cov = vec![OcmCovarianceMatrix::builder()
        .cov_ref_frame("GCRF")
        .cov_type("CARTP")
        .cov_lines(vec![CovLine {
            epoch: "0".parse().unwrap(),
            values: vec![1.0000000000000004, 1., 1., 0., 0., 1.],
        }])
        .build()];
    message.validate().unwrap();
    let xml = message.to_xml().unwrap();
    assert_eq!(Ocm::from_xml(&xml).unwrap(), message);
    assert!(message.to_kvn().is_err());
    let mut output = Vec::new();
    assert!(message.write_kvn_to(&mut output).is_err());
    assert!(output.is_empty());
    // Ordinary rounding is still available when it retains a valid covariance.
    message.body.segment.data.cov[0].cov_lines[0].values[1] = 0.3;
    let kvn = message.to_kvn().unwrap();
    Ocm::from_kvn(&kvn).unwrap();

    // The same preflight applies to eigenvector records close to their existing norm boundary.
    for kind in ["SIG3EIGVEC3", "TSIG3EIGVEC3"] {
        let covariance = &mut message.body.segment.data.cov[0];
        covariance.cov_type = kind.into();
        covariance.cov_lines[0].values = vec![
            1.,
            2.,
            3.,
            1.0000000000004998,
            0.,
            0.,
            0.,
            1.,
            0.,
            0.,
            0.,
            1.,
        ];
        if kind.starts_with('T') {
            covariance.cov_lines[0].values.insert(0, 1.);
        }
        message.validate().unwrap();
        assert_eq!(Ocm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
        assert!(message.to_kvn().is_err());
        let mut output = Vec::new();
        assert!(message.write_kvn_to(&mut output).is_err());
        assert!(output.is_empty());
    }
}

#[test]
fn known_compositions_are_case_insensitive_without_rewriting_producer_tokens() {
    use ccsds_ndm::messages::ocm::{CovLine, OcmCovarianceMatrix};
    let mut message = Ocm::from_kvn(BASE).unwrap();
    message.body.segment.data.traj[0].traj_type = "cartpva".into();
    message.validate().unwrap();
    assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    message.body.segment.data.traj[0].traj_lines[0].values.pop();
    assert!(message.validate().is_err());
    message.body.segment.data.traj.clear();
    message.body.segment.data.cov = vec![OcmCovarianceMatrix::builder()
        .cov_ref_frame("GCRF")
        .cov_type("cartp")
        .cov_lines(vec![CovLine {
            epoch: "0".parse().unwrap(),
            values: vec![1., 0., 1., 0., 0., 1.],
        }])
        .build()];
    message.validate().unwrap();
    assert_eq!(Ocm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
    message.body.segment.data.cov[0].cov_lines[0].values[0] = 0.;
    assert!(message.validate().is_err());
}

#[test]
fn eigenvector_units_describe_diagonal_standard_deviations_in_any_case() {
    use ccsds_ndm::messages::ocm::{CovLine, OcmCovarianceMatrix};
    for kind in ["sig3eigvec3", "tsig3eigvec3"] {
        let timed = kind.starts_with('t');
        let mut values = vec![3., 2., 1., 1., 0., 0., 0., 1., 0., 0., 0., 1.];
        if timed {
            values.insert(0, 1.);
        }
        let mut message = Ocm::from_kvn(BASE).unwrap();
        message.body.segment.data.traj.clear();
        message.body.segment.data.cov = vec![OcmCovarianceMatrix::builder()
            .cov_ref_frame("GCRF")
            .cov_type(kind)
            .cov_units(if timed { "[s,km,km,km]" } else { "[km,km,km]" })
            .cov_lines(vec![CovLine {
                epoch: "0".parse().unwrap(),
                values,
            }])
            .build()];
        message.validate().unwrap();
        assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
        assert_eq!(Ocm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
        message.body.segment.data.cov[0].cov_units = Some("[m,m,m]".into());
        assert!(message.validate().is_err());
        message.body.segment.data.cov[0].cov_units = None;
        *message.body.segment.data.cov[0].cov_lines[0]
            .values
            .last_mut()
            .unwrap() = 0.;
        assert!(message.validate().is_err());
    }
}

#[test]
fn published_g17_typo_is_preserved_separately_from_the_corrected_fixture() {
    let original = include_str!("../../data/invalid/ocm_g17_original.kvn");
    let error = Ocm::from_kvn(original).unwrap_err();
    crate::common::assert_validation_field(&error, "MAN_COMPOSITION");
    let corrected = include_str!("../../data/kvn/ocm_g17.kvn");
    let message = Ocm::from_kvn(corrected).unwrap();
    assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    assert_eq!(Ocm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
    assert_eq!(
        message.body.segment.data.man[1].man_units.as_deref(),
        Some("[s, N, N, N, n/a, n/a, s, %]")
    );
}

#[test]
fn tc1_keywords_allow_alternate_header_positions_but_not_duplicates() {
    for before in ["TIME_SYSTEM", "EPOCH_TZERO", "META_STOP"] {
        let source = BASE.replacen(
            before,
            &format!("OWNER_URL = https://example.org\n{before}"),
            1,
        );
        Ocm::from_kvn(&source).unwrap();
        let duplicate = source.replacen(
            "META_STOP",
            "OWNER_URL = https://duplicate.org\nMETA_STOP",
            1,
        );
        assert!(Ocm::from_kvn(&duplicate)
            .unwrap_err()
            .to_string()
            .contains("duplicate"));
    }
    let start = LINKED.find("COV_START").unwrap();
    let source = format!(
        "{}{}",
        &LINKED[..start],
        LINKED[start..].replacen("TRAJ_ID = G19_TRAJECTORY\n", "", 1)
    );
    for before in ["COV_BASIS", "COV_TYPE", "COV_UNITS"] {
        let source = source.replacen(before, &format!("TRAJ_ID = G19_TRAJECTORY\n{before}"), 1);
        Ocm::from_kvn(&source).unwrap();
        let duplicate = source.replacen("COV_BASIS", "TRAJ_ID = G19_TRAJECTORY\nCOV_BASIS", 1);
        assert!(Ocm::from_kvn(&duplicate)
            .unwrap_err()
            .to_string()
            .contains("duplicate"));
    }
}

#[test]
fn tc1_composition_continuations_preserve_columns_and_refuse_missing_continuations() {
    let source = format!("{BASE}\nUSER_START\nUSER_DEFINED_COMPOSITION_TEST = TIME_RELATIVE,\n VALUE\n0 1\nUSER_STOP\n");
    let message = Ocm::from_kvn(&source).unwrap();
    let parameter = message
        .body
        .segment
        .data
        .user
        .as_ref()
        .unwrap()
        .parameters()
        .next()
        .unwrap();
    assert_eq!(parameter.value, "TIME_RELATIVE, VALUE");
    assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    for continuation in ["", "USER_DEFINED_OTHER = value", "USER_STOP"] {
        assert!(Ocm::from_kvn(&source.replace(" VALUE\n", &format!("{continuation}\n"))).is_err());
    }
}

#[test]
fn maneuver_units_and_deploy_device_are_checked_on_input_and_generation() {
    let source = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nMAN_START\nMAN_ID = M1\nMAN_DEVICE_ID = THRUSTER\nMAN_REF_FRAME = GCRF\nMAN_COMPOSITION = TIME_RELATIVE, DV_X\nMAN_UNITS = km/s\n0 1\nMAN_STOP\n";
    let message = Ocm::from_kvn(source).unwrap();
    let xml = message.to_xml().unwrap();
    for (key, value) in [("MAN_UNITS", "kg"), ("MAN_DEVICE_ID", "DEPLOY")] {
        let old = if key == "MAN_UNITS" {
            "km/s"
        } else {
            "THRUSTER"
        };
        let invalid = source.replace(&format!("{key} = {old}"), &format!("{key} = {value}"));
        assert!(Ocm::from_kvn(&invalid)
            .unwrap_err()
            .to_string()
            .contains(key));
        let invalid = xml.replace(
            &format!("<{key}>{old}</{key}>"),
            &format!("<{key}>{value}</{key}>"),
        );
        assert!(Ocm::from_xml(&invalid)
            .unwrap_err()
            .to_string()
            .contains(key));
        let mut edited = message.clone();
        if key == "MAN_UNITS" {
            edited.body.segment.data.man[0].man_units = Some(value.into());
        } else {
            edited.body.segment.data.man[0].man_device_id = value.into();
        }
        crate::common::assert_rejects(&edited, key);
    }
    let lower = source
        .replace("TIME_RELATIVE, DV_X", "time_relative, dv_x")
        .replace("THRUSTER", "deploy");
    assert!(Ocm::from_kvn(&lower)
        .unwrap_err()
        .to_string()
        .contains("MAN_DEVICE_ID"));
    let mut edited = message;
    edited.body.segment.data.man[0].man_device_id = "deploy".into();
    edited.body.segment.data.man[0].man_composition = "time_relative, dv_x".into();
    crate::common::assert_rejects(&edited, "MAN_DEVICE_ID");
}

#[test]
fn reserved_ocm_values_follow_blank_underscore_equivalence() {
    // ODM 7.5.9: blanks and underscores are equivalent in reserved values; custom text is kept.
    let source = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nMAN_START\nMAN_ID = M1\nMAN_DEVICE_ID = THRUSTER\nMAN_REF_FRAME = GCRF\nMAN_COMPOSITION = TIME_RELATIVE, DV_X\nMAN_UNITS = km/s\n0 1\nMAN_STOP\n";
    let message = Ocm::from_kvn(source).unwrap();

    // Complete comma-delimited column aliases are one column each.
    for composition in [
        "TIME RELATIVE, DV_X",
        "TIME_RELATIVE, DV X",
        "time relative, dv  x",
    ] {
        let aliased = Ocm::from_kvn(&source.replace("TIME_RELATIVE, DV_X", composition)).unwrap();
        assert!(aliased
            .to_kvn()
            .unwrap()
            .contains("MAN_COMPOSITION      = TIME_RELATIVE, DV_X"));
    }

    // A padded reserved DEPLOY device is still the reserved value, in the model and in XML.
    let mut padded = message.clone();
    padded.body.segment.data.man[0].man_device_id = " DEPLOY ".into();
    crate::common::assert_rejects(&padded, "MAN_DEVICE_ID");
    let xml = message.to_xml().unwrap().replace(
        "<MAN_DEVICE_ID>THRUSTER</MAN_DEVICE_ID>",
        "<MAN_DEVICE_ID> DEPLOY </MAN_DEVICE_ID>",
    );
    assert!(Ocm::from_xml(&xml)
        .unwrap_err()
        .to_string()
        .contains("MAN_DEVICE_ID"));

    // A blank-spelled known covariance type keeps its 28-value matrix check.
    let covariance = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nCOV_START\nCOV_REF_FRAME = GCRF\nCOV_TYPE = TEQUINOCTIAL_P\nCOV_ORDERING = LTM\n0 1\nCOV_STOP\n";
    // Control: a genuinely unknown registry token stays opaque, so the rejection is the matrix check.
    Ocm::from_kvn(&covariance.replace("TEQUINOCTIAL_P", "CUSTOM_TYPE")).unwrap();
    for spelling in ["TEQUINOCTIAL_P", "TEQUINOCTIAL P", "tequinoctialmod n"] {
        assert!(
            Ocm::from_kvn(&covariance.replace("TEQUINOCTIAL_P", spelling)).is_err(),
            "{spelling}"
        );
    }
}

#[test]
fn xml_scalar_composition_prefix_error_explains_tc1_reservation() {
    use ccsds_ndm::messages::ocm::{OcmUserDefined, OcmUserDefinedEntry, OcmUserDefinedParameter};
    let mut message = Ocm::from_kvn(BASE).unwrap();
    message.body.segment.data.user = Some(OcmUserDefined {
        entries: vec![OcmUserDefinedEntry::Parameter(OcmUserDefinedParameter {
            parameter: "MODE".into(),
            value: "AWGN".into(),
            records: Vec::new(),
        })],
    });
    let xml = message
        .to_xml()
        .unwrap()
        .replace("parameter=\"MODE\"", "parameter=\"COMPOSITION_MODE\"");
    let error = Ocm::from_xml(&xml).unwrap_err().to_string();
    assert!(
        error.contains("COMPOSITION_MODE") && error.contains("TC1") && error.contains("XML 4.0"),
        "{error}"
    );
}

#[test]
fn owner_country_and_owner_url_accept_adjacent_metadata_orders() {
    for entries in [
        "OWNER = OWNER
COUNTRY = COUNTRY
OWNER_URL = https://example.org
",
        "OWNER_URL = https://example.org
OWNER = OWNER
COUNTRY = COUNTRY
",
        "OWNER = OWNER
OWNER_URL = https://example.org
COUNTRY = COUNTRY
",
    ] {
        let source = BASE.replacen("TIME_SYSTEM", &format!("{entries}TIME_SYSTEM"), 1);
        let message = Ocm::from_kvn(&source).unwrap();
        assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    }
}
