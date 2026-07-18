use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;

const KVN: &str = include_str!("../../data/kvn/opm_g1.kvn");

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
fn strict_kvn_accepts_lf_and_crlf_without_changing_meaning() {
    let lf = Opm::from_kvn(KVN).expect("LF fixture should parse");
    let crlf_source = KVN.replace('\n', "\r\n");
    let crlf = Opm::from_kvn(&crlf_source).expect("CRLF fixture should parse");
    assert_eq!(crlf, lf);
}
