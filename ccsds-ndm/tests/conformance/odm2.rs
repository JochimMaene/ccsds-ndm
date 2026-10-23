// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{fixtures, mutated_once};

use ccsds_ndm::messages::{oem::Oem, omm::Omm, opm::Opm};
use ccsds_ndm::{Ndm, Validate};
use std::path::Path;
use std::process::Command;

fn validate_with_odm_2_xsd(family: &str, xml: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/xsd");
    let family_schema = root.join(format!("ndmxml-2.0.0-{family}-2.0.xsd"));
    let wrapper = tempfile::NamedTempFile::new().expect("temporary wrapper schema");
    std::fs::write(
        wrapper.path(),
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema"
            xmlns:ndm="urn:ccsds:schema:ndmxml">
  <xsd:import namespace="urn:ccsds:schema:ndmxml" schemaLocation="{}"/>
  <xsd:element name="{family}" type="ndm:{family}Type"/>
</xsd:schema>
"#,
            family_schema.display()
        ),
    )
    .expect("write wrapper schema");
    let generated = tempfile::NamedTempFile::new().expect("temporary generated XML");
    std::fs::write(generated.path(), xml).expect("write generated XML");
    let output = Command::new("xmllint")
        .args(["--noout", "--schema"])
        .arg(wrapper.path())
        .arg(generated.path())
        .output()
        .unwrap_or_else(|error| panic!("xmllint is required for conformance tests: {error}"));
    assert!(
        output.status.success(),
        "generated {family} 2.0 XML failed the official schema:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn edition_2(input: &str) -> String {
    mutated_once(
        &input
            .lines()
            .filter(|line| !line.starts_with("MESSAGE_ID"))
            .map(|line| format!("{line}\n"))
            .collect::<String>(),
        "3.0",
        "2.0",
    )
}

#[test]
fn opm_2_generation_is_schema_valid_and_round_trips() {
    for (name, source) in fixtures("opm_", "kvn") {
        let opm = Opm::from_kvn(&edition_2(&source)).unwrap();
        let xml = opm.to_xml().unwrap();
        validate_with_odm_2_xsd("opm", &xml);
        assert_eq!(Opm::from_xml(&xml).unwrap(), opm, "{name} XML");
        assert_eq!(
            Opm::from_kvn(&opm.to_kvn().unwrap()).unwrap(),
            opm,
            "{name} KVN"
        );
    }
}

/// Every shipped OEM example as OEM 2.0. Annex J of 502.0-B-3 and the 2.0 schema differ from
/// 3.0 only in the header, so the 3.0 examples without MESSAGE_ID are valid 2.0 content.
fn oem_2_examples() -> Vec<Oem> {
    let mut messages: Vec<Oem> = [
        include_str!("../../data/kvn/oem_g11.kvn"),
        include_str!("../../data/kvn/oem_g12.kvn"),
        include_str!("../../data/kvn/oem_g13.kvn"),
    ]
    .into_iter()
    .map(|kvn| Oem::from_kvn(&edition_2(kvn)).unwrap())
    .collect();
    let mut xml = Oem::from_xml(include_str!("../../data/xml/oem_g14.xml")).unwrap();
    xml.version = "2.0".into();
    xml.header.message_id = None;
    messages.push(xml);
    messages
}

#[test]
fn oem_2_generation_is_schema_valid_and_round_trips() {
    for oem in oem_2_examples() {
        let xml = oem.to_xml().unwrap();
        validate_with_odm_2_xsd("oem", &xml);
        let parsed = Oem::from_xml(&xml).unwrap();
        assert_eq!(parsed.version, "2.0");
        assert_eq!(parsed, oem);
        let kvn = oem.to_kvn().unwrap();
        let first = kvn.lines().next().unwrap();
        assert!(
            first.starts_with("CCSDS_OEM_VERS") && first.ends_with("= 2.0"),
            "{first}"
        );
        assert_eq!(Oem::from_kvn(&kvn).unwrap(), oem);
    }
}

#[test]
fn oem_2_rejects_odm_3_header_fields_when_reading_and_writing() {
    // The 2.0 schema's ndmHeader has neither CLASSIFICATION nor MESSAGE_ID; annex J lists
    // MESSAGE_ID as new in 3.0.
    let base = oem_2_examples().remove(2);
    for (field, path) in [
        ("CLASSIFICATION", "header.classification"),
        ("MESSAGE_ID", "header.message_id"),
    ] {
        let mut message = base.clone();
        match field {
            "CLASSIFICATION" => message.header.classification = Some("SBU".into()),
            _ => message.header.message_id = Some("OEM-3-ONLY".into()),
        }
        for error in [message.to_kvn().unwrap_err(), message.to_xml().unwrap_err()] {
            assert_eq!(
                error.field_path().as_deref(),
                Some(path),
                "{field}: {error}"
            );
        }
        let mut as_three = message.clone();
        as_three.version = "3.0".into();
        let kvn = as_three.to_kvn().unwrap().replace("= 3.0", "= 2.0");
        let error = Oem::from_kvn(&kvn).unwrap_err();
        assert_eq!(
            error.field_path().as_deref(),
            Some(path),
            "KVN {field}: {error}"
        );
        let xml = as_three
            .to_xml()
            .unwrap()
            .replace("version=\"3.0\"", "version=\"2.0\"");
        let error = Oem::from_xml(&xml).unwrap_err();
        assert_eq!(
            error.field_path().as_deref(),
            Some(path),
            "XML {field}: {error}"
        );
    }
}

#[test]
fn oem_2_useable_start_may_not_precede_the_previous_useable_stop() {
    // 502.0-B-2 Cor. 1, table 5-3: "The USEABLE_START_TIME time tag at a new block of ephemeris
    // data must be greater than or equal to the USEABLE_STOP_TIME time tag of the previous
    // block." 3.0 dropped the sentence, so the same content stays valid there.
    let epoch = |value: &str| value.parse::<ccsds_ndm::types::Epoch>().unwrap();
    let path = "body.segment[1].metadata.useable_start_time";
    let assert_rejected = |message: &Oem| {
        crate::common::assert_rejects(message, "USEABLE_START_TIME");
        assert_eq!(
            message.validate().unwrap_err().field_path().as_deref(),
            Some(path)
        );
        let mut as_three = message.clone();
        as_three.version = "3.0".into();
        let kvn = as_three.to_kvn().unwrap().replace("= 3.0", "= 2.0");
        let error = Oem::from_kvn(&kvn).unwrap_err();
        assert_eq!(error.field_path().as_deref(), Some(path), "KVN: {error}");
        let xml = as_three
            .to_xml()
            .unwrap()
            .replace("version=\"3.0\"", "version=\"2.0\"");
        let error = Oem::from_xml(&xml).unwrap_err();
        assert_eq!(error.field_path().as_deref(), Some(path), "XML: {error}");
    };
    let base = oem_2_examples().remove(0);
    assert_eq!(base.body.segment.len(), 2);

    // Disjoint useable spans in reverse time order.
    let mut reversed = base.clone();
    reversed.body.segment.swap(0, 1);
    assert_rejected(&reversed);

    // The two values the rule names suffice; the other bounds may be absent.
    let mut partial = base.clone();
    let previous = &mut partial.body.segment[0].metadata;
    previous.useable_start_time = None;
    previous.stop_time = epoch("2019-12-29T00:00:00");
    previous.useable_stop_time = Some(epoch("2019-12-29T00:00:00"));
    partial.body.segment[1].metadata.useable_stop_time = None;
    assert_rejected(&partial);

    // A shared endpoint satisfies "greater than or equal to".
    let mut shared = base.clone();
    shared.body.segment[0].metadata.stop_time = epoch("2019-12-28T22:08:02.5");
    shared.body.segment[0].metadata.useable_stop_time = Some(epoch("2019-12-28T22:08:02.5"));
    shared.validate().expect("shared endpoint");

    // Without the previous USEABLE_STOP_TIME or the new USEABLE_START_TIME the rule is silent.
    let mut unnamed = reversed.clone();
    unnamed.body.segment[0].metadata.useable_stop_time = None;
    unnamed.validate().expect("no previous USEABLE_STOP_TIME");
    let mut unnamed = reversed.clone();
    unnamed.body.segment[1].metadata.useable_start_time = None;
    unnamed.validate().expect("no new USEABLE_START_TIME");

    for mut message in [reversed, partial] {
        message.version = "3.0".into();
        message.validate().expect("3.0 has no ordering rule");
    }
}

#[test]
fn omm_2_generation_is_schema_valid_and_round_trips() {
    for (name, source) in fixtures("omm_", "kvn") {
        let omm = Omm::from_kvn(&edition_2(&source)).unwrap();
        let xml = omm.to_xml().unwrap();
        validate_with_odm_2_xsd("omm", &xml);
        assert_eq!(Omm::from_xml(&xml).unwrap(), omm, "{name} XML");
        assert_eq!(
            Omm::from_kvn(&omm.to_kvn().unwrap()).unwrap(),
            omm,
            "{name} KVN"
        );
    }
}

#[test]
fn odm_2_generation_rejects_odm_3_only_fields() {
    let mut opm = Opm::from_kvn(&edition_2(include_str!("../../data/kvn/opm_g1.kvn"))).unwrap();
    opm.header.message_id = Some("OPM-3-ONLY".into());
    let error = opm.to_xml().unwrap_err();
    assert_eq!(error.field_path().as_deref(), Some("header.message_id"));
}

#[test]
fn opm_and_omm_2_reject_new_header_fields_in_both_notations() {
    for source in [
        include_str!("../../data/kvn/opm_g1.kvn"),
        include_str!("../../data/kvn/omm_g7.kvn"),
    ] {
        for (keyword, path) in [
            ("CLASSIFICATION", "header.classification"),
            ("MESSAGE_ID", "header.message_id"),
        ] {
            let mut message = ccsds_ndm::from_str(&edition_2(source)).unwrap();
            let header = match &mut message {
                ccsds_ndm::Message::Opm(message) => &mut message.header,
                ccsds_ndm::Message::Omm(message) => &mut message.header,
                _ => unreachable!(),
            };
            match keyword {
                "CLASSIFICATION" => header.classification = Some("U".into()),
                _ => header.message_id = Some("new-only".into()),
            }
            for error in [message.to_kvn().unwrap_err(), message.to_xml().unwrap_err()] {
                assert_eq!(error.field_path().as_deref(), Some(path));
            }
            match &mut message {
                ccsds_ndm::Message::Opm(message) => message.version = "3.0".into(),
                ccsds_ndm::Message::Omm(message) => message.version = "3.0".into(),
                _ => unreachable!(),
            }
            for source in [
                message.to_kvn().unwrap().replacen("= 3.0", "= 2.0", 1),
                message
                    .to_xml()
                    .unwrap()
                    .replace("version=\"3.0\"", "version=\"2.0\""),
            ] {
                let error = ccsds_ndm::from_str(&source).unwrap_err();
                assert_eq!(error.field_path().as_deref(), Some(path));
            }
        }
    }
}

#[test]
fn omm_2_tle_schema_shape_is_xml_only() {
    let mut base = Omm::from_kvn(&edition_2(include_str!("../../data/kvn/omm_g7.kvn"))).unwrap();
    // A neutral theory isolates the schema shape from the 2.0 book's theory conditions.
    base.body.segment.metadata.mean_element_theory = "ICD_THEORY".into();
    type Edit = fn(&mut ccsds_ndm::messages::omm::TleParameters);
    let mutations: &[(&str, Edit)] = &[
        ("norad_cat_id", |tle| tle.norad_cat_id = None),
        ("element_set_no", |tle| tle.element_set_no = None),
        ("rev_at_epoch", |tle| tle.rev_at_epoch = None),
        ("bstar", |tle| tle.bstar = None),
        ("mean_motion_ddot", |tle| tle.mean_motion_ddot = None),
        ("mean_motion_dot", |tle| tle.mean_motion_dot = None),
        ("bterm", |tle| {
            tle.bstar = None;
            tle.bterm = Some(ccsds_ndm::types::M2kg::new(0.01, None));
        }),
        ("agom", |tle| {
            tle.mean_motion_ddot = None;
            tle.agom = Some(ccsds_ndm::types::M2kg::new(0.01, None));
        }),
    ];
    for (field, mutate) in mutations {
        let mut message = base.clone();
        mutate(message.body.segment.data.tle_parameters.as_mut().unwrap());
        if matches!(*field, "bterm" | "agom") {
            assert!(message.validate().is_err(), "{field}");
            assert!(message.to_kvn().is_err(), "{field}");
        } else {
            message.validate().unwrap();
            let kvn = message.to_kvn().unwrap();
            assert_eq!(Omm::from_kvn(&kvn).unwrap(), message);
        }
        assert!(message.to_xml().is_err(), "{field}");
        // A neutral theory permits KVN omissions in 3.0; XML still requires each choice.
        message.version = "3.0".into();
        let source = message.to_kvn().unwrap().replacen("= 3.0", "= 2.0", 1);
        if matches!(*field, "bterm" | "agom") {
            assert!(ccsds_ndm::from_str(&source).is_err(), "{field}");
        } else {
            ccsds_ndm::from_str(&source).unwrap();
        }
        if matches!(*field, "bstar" | "mean_motion_ddot" | "mean_motion_dot") {
            assert!(message.to_xml().is_err(), "{field}");
        } else {
            let source = message
                .to_xml()
                .unwrap()
                .replace("version=\"3.0\"", "version=\"2.0\"");
            assert!(ccsds_ndm::from_str(&source).is_err(), "{field}");
        }
    }
}

#[test]
fn opm_2_requires_negative_maneuver_delta_mass_in_every_route() {
    let mut message = Opm::from_kvn(&edition_2(include_str!("../../data/kvn/opm_g2.kvn"))).unwrap();
    message.body.segment.data.maneuver_parameters[0]
        .man_delta_mass
        .value = 0.0;
    let path = "body.segment.data.maneuver_parameters[0].man_delta_mass";
    assert_eq!(
        message.validate().unwrap_err().field_path().as_deref(),
        Some(path)
    );
    assert_eq!(
        message.to_kvn().unwrap_err().field_path().as_deref(),
        Some(path)
    );
    assert_eq!(
        message.to_xml().unwrap_err().field_path().as_deref(),
        Some(path)
    );
    message.version = "3.0".into();
    for source in [
        message.to_kvn().unwrap().replacen("= 3.0", "= 2.0", 1),
        message
            .to_xml()
            .unwrap()
            .replace("version=\"3.0\"", "version=\"2.0\""),
    ] {
        assert_eq!(
            ccsds_ndm::from_str(&source)
                .unwrap_err()
                .field_path()
                .as_deref(),
            Some(path)
        );
    }
}

#[test]
fn odm_2_partial_spacecraft_is_kvn_valid_but_not_xml_representable() {
    let mut opm = Opm::from_kvn(&edition_2(include_str!("../../data/kvn/opm_g2.kvn"))).unwrap();
    let mut omm = Omm::from_kvn(&edition_2(include_str!("../../data/kvn/omm_g9.kvn"))).unwrap();
    let parameters = ccsds_ndm::common::SpacecraftParameters {
        mass: Some(ccsds_ndm::types::Mass::new(100.0, None).unwrap()),
        ..Default::default()
    };
    opm.body.segment.data.spacecraft_parameters = Some(parameters.clone());
    omm.body.segment.data.spacecraft_parameters = Some(parameters);
    opm.validate().unwrap();
    omm.validate().unwrap();
    Opm::from_kvn(&opm.to_kvn().unwrap()).unwrap();
    Omm::from_kvn(&omm.to_kvn().unwrap()).unwrap();
    assert!(opm.to_xml().is_err());
    assert!(omm.to_xml().is_err());
    for (family, mut source) in [
        ("opm", {
            opm.version = "3.0".into();
            opm.to_xml().unwrap()
        }),
        ("omm", {
            omm.version = "3.0".into();
            omm.to_xml().unwrap()
        }),
    ] {
        source = source.replace("version=\"3.0\"", "version=\"2.0\"");
        assert!(ccsds_ndm::from_str(&source).is_err(), "{family}");
    }
}

/// ODM 2.0 table 4-3 states its own theory conditions: NORAD_CAT_ID and BSTAR for SGP/SGP4,
/// both derivatives for SGP. PPT3 and SGP4-XP are 3.0 theories and impose nothing in 2.0.
#[test]
fn omm_2_applies_its_own_theory_table() {
    let base = Omm::from_kvn(&edition_2(include_str!("../../data/kvn/omm_g7.kvn"))).unwrap();
    for (theory, field) in [
        ("SGP/SGP4", "NORAD_CAT_ID"),
        ("SGP4", "BSTAR"),
        ("SGP", "MEAN_MOTION_DOT"),
    ] {
        let mut message = base.clone();
        message.body.segment.metadata.mean_element_theory = theory.into();
        let tle = message.body.segment.data.tle_parameters.as_mut().unwrap();
        match field {
            "NORAD_CAT_ID" => tle.norad_cat_id = None,
            "BSTAR" => tle.bstar = None,
            _ => tle.mean_motion_dot = None,
        }
        crate::common::assert_validation_field(&message.validate().unwrap_err(), field);
        // The same omission is valid in 3.0 except where the 3.0 table also requires it.
        message.version = "3.0".into();
        assert_eq!(
            message.validate().is_ok(),
            field == "NORAD_CAT_ID",
            "{theory}"
        );
    }
    for theory in ["PPT3", "SGP4-XP"] {
        let mut message = base.clone();
        message.body.segment.metadata.mean_element_theory = theory.into();
        message.body.segment.data.tle_parameters = None;
        message.validate().unwrap();
        message.version = "3.0".into();
        crate::common::assert_validation_field(&message.validate().unwrap_err(), "TLE_PARAMETERS");
    }
}

/// The ODM 2.0 common schema's angleRange is [-180, 360); 3.0 allows [-360, 360).
#[test]
fn odm_2_angles_use_the_2_0_range() {
    let mut omm = Omm::from_kvn(&edition_2(include_str!("../../data/kvn/omm_g7.kvn"))).unwrap();
    omm.body.segment.data.mean_elements.mean_anomaly.value = -200.0;
    let error = omm.validate().unwrap_err();
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment.data.mean_elements.mean_anomaly")
    );
    assert!(omm.to_kvn().is_err());
    omm.version = "3.0".into();
    omm.header.message_id = None;
    omm.validate().unwrap();

    let mut opm = Opm::from_kvn(&edition_2(include_str!("../../data/kvn/opm_g2.kvn"))).unwrap();
    let elements = opm.body.segment.data.keplerian_elements.as_mut().unwrap();
    elements.ra_of_asc_node.value = -180.5;
    let error = opm.validate().unwrap_err();
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment.data.keplerian_elements.ra_of_asc_node")
    );
    // KVN input applies the same 2.0 bound; the 3.0 original still reads it.
    let source = mutated_once(
        include_str!("../../data/kvn/opm_g2.kvn"),
        "RA_OF_ASC_NODE = 17.604721",
        "RA_OF_ASC_NODE = -180.5",
    );
    Opm::from_kvn(&source).unwrap();
    crate::common::assert_validation_field(
        &Opm::from_kvn(&edition_2(&source)).unwrap_err(),
        "RA_OF_ASC_NODE",
    );
}
