use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;

const KVN: &str = include_str!("../data/kvn/opm_g1.kvn");

fn assert_rejected(label: &str, kvn: String) {
    assert!(
        Opm::from_kvn(&kvn).is_err(),
        "{label} was silently accepted"
    );
}

#[test]
fn strict_kvn_rejects_unknown_duplicate_reordered_malformed_and_lossy_content() {
    let object_name = KVN
        .lines()
        .find(|line| line.starts_with("OBJECT_NAME"))
        .expect("fixture should contain OBJECT_NAME");
    assert_rejected(
        "duplicate fixed keyword",
        KVN.replace(object_name, &format!("{object_name}\n{object_name}")),
    );

    let object_id = KVN
        .lines()
        .find(|line| line.starts_with("OBJECT_ID"))
        .expect("fixture should contain OBJECT_ID");
    assert_rejected(
        "reordered fixed keywords",
        KVN.replace(
            &format!("{object_name}\n{object_id}"),
            &format!("{object_id}\n{object_name}"),
        ),
    );
    assert_rejected(
        "unknown keyword",
        KVN.replace(object_name, &format!("{object_name}\nUNKNOWN_KEY = value")),
    );
    assert_rejected(
        "comment in the middle of a logical block",
        KVN.replace(object_name, &format!("{object_name}\nCOMMENT misplaced")),
    );
    assert_rejected(
        "leading comment that cannot be represented",
        format!("COMMENT lost\n{KVN}"),
    );
    assert_rejected(
        "non-printable character",
        KVN.replace(object_name, &format!("{object_name}\u{1}")),
    );

    let version = KVN.lines().next().expect("fixture should have a version");
    assert_rejected(
        "line longer than 254 characters",
        KVN.replace(version, &format!("{version}\nCOMMENT {}", "x".repeat(250))),
    );
    assert_rejected(
        "lone carriage return",
        KVN.replace("OBJECT_NAME", "OBJECT\r_NAME"),
    );
}

#[test]
fn strict_kvn_accepts_every_normative_line_ending_without_changing_meaning() {
    let lf = Opm::from_kvn(KVN).expect("LF fixture should parse");
    for (name, ending) in [("CR", "\r"), ("CRLF", "\r\n"), ("LFCR", "\n\r")] {
        let source = KVN.replace('\n', ending);
        let parsed = Opm::from_kvn(&source)
            .unwrap_or_else(|error| panic!("{name} fixture should parse: {error}"));
        assert_eq!(parsed, lf, "{name} changed the parsed model");
    }
}

/// `TRUE_ANOMALY` and `MEAN_ANOMALY` share an ordering rank so either may fill the single
/// anomaly slot. That allowance must not extend to repeating one of them: `parse_block!` keeps
/// the last assignment, so a repeat that reached the parser would silently discard a value.
#[test]
fn strict_kvn_separates_the_anomaly_choice_from_a_repeated_anomaly() {
    let keplerian = "SEMI_MAJOR_AXIS = 6800.0
ECCENTRICITY = 0.0005
INCLINATION = 51.6
RA_OF_ASC_NODE = 10.0
ARG_OF_PERICENTER = 20.0
";
    let with_anomalies = |anomalies: &str| {
        let mut kvn = KVN
            .lines()
            .take_while(|line| !line.starts_with("MASS"))
            .collect::<Vec<_>>()
            .join("\n");
        kvn.push('\n');
        kvn.push_str(keplerian);
        kvn.push_str(anomalies);
        kvn.push_str("GM = 398600.4418\n");
        kvn
    };

    Opm::from_kvn(&with_anomalies("TRUE_ANOMALY = 30.0\n"))
        .expect("a single TRUE_ANOMALY fills the choice");
    Opm::from_kvn(&with_anomalies("MEAN_ANOMALY = 30.0\n"))
        .expect("a single MEAN_ANOMALY fills the choice");

    assert_rejected(
        "repeated TRUE_ANOMALY",
        with_anomalies("TRUE_ANOMALY = 30.0\nTRUE_ANOMALY = 40.0\n"),
    );
    assert_rejected(
        "repeated MEAN_ANOMALY",
        with_anomalies("MEAN_ANOMALY = 30.0\nMEAN_ANOMALY = 40.0\n"),
    );

    // Both alternatives present remains a semantic choice violation, not an ordering error, so
    // the diagnostic still names the pair.
    for order in [
        "TRUE_ANOMALY = 30.0\nMEAN_ANOMALY = 40.0\n",
        "MEAN_ANOMALY = 30.0\nTRUE_ANOMALY = 40.0\n",
    ] {
        let error =
            Opm::from_kvn(&with_anomalies(order)).expect_err("only one anomaly may be present");
        assert_eq!(error.code(), Some("validation.invalid_choice"));
    }
}
