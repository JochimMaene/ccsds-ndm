use crate::common::{data_dir, mutated_once};
use ccsds_ndm::from_str;

/// `time_tag` with its fractional seconds zero-padded to `digits`, which keeps its value.
fn padded(time_tag: &str, digits: usize) -> String {
    let (whole, fraction) = time_tag.split_once('.').unwrap_or((time_tag, ""));
    format!("{whole}.{fraction:0<digits$}")
}

#[test]
fn fractional_seconds_stop_at_the_fixed_point_maximum_except_in_tdm() {
    // ODM 7.5.10 and the ADM, CDM, and RDM books limit fractions to a fixed-point number's
    // 16 digits; the TDM book (4.3.9) states no limit.
    for (fixture, context, time_tag) in [
        ("opm_g1.kvn", "EPOCH = ", "2022-12-18T14:28:15.1172"),
        ("omm_g7.kvn", "EPOCH = ", "2020-064T10:34:41.4264"),
        ("ocm_g15.kvn", "EPOCH_TZERO = ", "2022-12-18T14:28:15.1172"),
        ("oem_g11.kvn", "\n", "2019-12-18T12:00:00.331"),
        ("cdm_362.kvn", "TCA = ", "2010-03-13T22:37:52.618"),
        ("rdm_c1.kvn", "EPOCH_TZERO = ", "2018-04-22T00:00:00.00"),
        ("aem_g4.kvn", "\n", "1996-11-28T21:29:07.2555"),
        ("apm_g1.kvn", "EPOCH = ", "2003-09-30T14:28:15.1172"),
        ("acm_g6.kvn", "EPOCH_TZERO = ", "1998-12-18T14:28:15.1172"),
        ("tdm_e1.kvn", "RECEIVE_FREQ_1 = ", "2005-159T17:41:00"),
    ] {
        let input = std::fs::read_to_string(data_dir().join("kvn").join(fixture)).unwrap();
        // A leading newline as context selects a data line rather than a keyword value.
        let with = |digits| {
            mutated_once(
                &input,
                &format!("{context}{time_tag}"),
                &format!("{context}{}", padded(time_tag, digits)),
            )
        };
        from_str(&with(16)).unwrap_or_else(|error| panic!("{fixture}: 16 digits: {error}"));
        let result = from_str(&with(17));
        if fixture.starts_with("tdm_") {
            assert!(result.is_ok(), "{fixture}: TDM has no fraction limit");
        } else {
            let error = result.expect_err(&format!("{fixture}: 17 digits accepted"));
            assert!(
                error.to_string().contains("16 fractional"),
                "{fixture}: {error}"
            );
        }
    }
}

#[test]
fn optional_xml_values_keep_their_schema_types_in_every_family() {
    // An optional element is either absent or holds a value of its schema type: an empty or
    // `n/a` epoch, number or enumeration is invalid, while xsd:string values keep their text.
    for (fixture, element, value) in [
        ("rdm_c4.xml", "NEXT_MESSAGE_EPOCH", ""),
        ("rdm_c4.xml", "NEXT_MESSAGE_EPOCH", "n/a"),
        ("rdm_c4.xml", "INTRACK_THRUST", ""),
        ("rdm_c4.xml", "ORBIT_LIFETIME", "n/a"),
        ("opm_g5.xml", "SOLAR_RAD_AREA", ""),
    ] {
        let input =
            std::fs::read_to_string(crate::common::data_dir().join("xml").join(fixture)).unwrap();
        let start = input.find(&format!("<{element}")).unwrap();
        let end = start + input[start..].find(&format!("</{element}>")).unwrap();
        let open_end = start + input[start..].find('>').unwrap() + 1;
        let invalid = format!("{}{value}{}", &input[..open_end], &input[end..]);
        assert!(
            from_str(&invalid).is_err(),
            "{fixture}: accepted {element}={value:?}"
        );
    }
    for (fixture, element) in [
        ("rdm_c4.xml", "GRAVITY_MODEL"),
        ("aem_g11.xml", "CENTER_NAME"),
    ] {
        let input =
            std::fs::read_to_string(crate::common::data_dir().join("xml").join(fixture)).unwrap();
        let start = input.find(&format!("<{element}>")).unwrap() + element.len() + 2;
        let end = start + input[start..].find('<').unwrap();
        let literal = format!("{}n/a{}", &input[..start], &input[end..]);
        let xml = from_str(&literal).unwrap().to_xml().unwrap();
        assert!(
            xml.contains(&format!("<{element}>n/a</{element}>")),
            "{fixture}: {element} lost its literal text"
        );
    }
}

#[test]
fn section_validators_apply_the_fraction_limit_themselves() {
    use ccsds_ndm::messages::{aem::Aem, ocm::Ocm};
    use ccsds_ndm::{Ndm, Validate};

    // Every entry point agrees: the section, its segment, and the message.
    let mut aem =
        Aem::from_kvn(&std::fs::read_to_string(data_dir().join("kvn/aem_g4.kvn")).unwrap())
            .unwrap();
    let segment = &mut aem.body.segment[0];
    segment.metadata.start_time = padded(segment.metadata.start_time.as_str(), 17)
        .parse()
        .unwrap();
    for error in [
        segment.metadata.validate().unwrap_err(),
        segment.validate().unwrap_err(),
        aem.validate().unwrap_err(),
    ] {
        crate::common::assert_validation_field(&error, "START_TIME");
    }

    // OCM data lines hold their epochs inside a text line in XML.
    let mut ocm =
        Ocm::from_kvn(&std::fs::read_to_string(data_dir().join("kvn/ocm_g15.kvn")).unwrap())
            .unwrap();
    let data = &mut ocm.body.segment.data;
    data.traj[0].traj_lines[0].epoch = padded("2022-12-18T14:28:15.1172", 17).parse().unwrap();
    crate::common::assert_validation_field(&Validate::validate(&*data).unwrap_err(), "EPOCH");
}
