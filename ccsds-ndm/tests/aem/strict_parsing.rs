use crate::common::{mutated, mutated_once};
use crate::{KVN, SPIN_KVN, XML};
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::Ndm;

#[test]
fn aem_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object = "OBJECT_NAME = MARS GLOBAL SURVEYOR";
    let object_id = "OBJECT_ID = 1996-062A";
    let first_state = "1996-11-28T21:29:07.2555 0.56748 0.03146 0.45689 0.68427";
    for (label, source) in [
        (
            "duplicate metadata keyword",
            mutated(KVN, object, &format!("{object}\n{object}")),
        ),
        (
            "reordered metadata",
            mutated(
                KVN,
                &format!("{object}\n{object_id}"),
                &format!("{object_id}\n{object}"),
            ),
        ),
        (
            "unknown metadata",
            mutated(KVN, object, &format!("{object}\nUNKNOWN = value")),
        ),
        (
            "comment after history begins",
            mutated(
                KVN,
                first_state,
                &format!("{first_state}\nCOMMENT misplaced"),
            ),
        ),
        ("mismatched block", mutated(KVN, "META_STOP", "DATA_STOP")),
        ("unknown block", mutated(KVN, "DATA_START", "UNKNOWN_START")),
        (
            "XML-only metadata keyword in KVN",
            mutated(
                KVN,
                "ATTITUDE_TYPE = QUATERNION",
                "ATTITUDE_TYPE = QUATERNION\nANGVEL_FRAME = A",
            ),
        ),
        (
            "zero interpolation degree",
            mutated(
                SPIN_KVN,
                "ATTITUDE_TYPE = SPIN",
                "ATTITUDE_TYPE = SPIN\nINTERPOLATION_DEGREE = 0",
            ),
        ),
        (
            "missing interpolation degree",
            mutated(KVN, "INTERPOLATION_DEGREE = 7\n", ""),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
    ] {
        assert!(Aem::from_kvn(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn aem_kvn_rejects_non_ccsds_history_number_spellings() {
    for value in [".5", "1e0", "1.2345678901234567"] {
        let source = mutated_once(SPIN_KVN, "2.6862511e+002", value);
        let error = Aem::from_kvn(&source).unwrap_err();
        assert_eq!(error.code(), Some("parse.kvn.syntax"), "{value}: {error}");
    }
}

#[test]
fn aem_xml_rejects_unknown_choice_content_attributes_and_ordering_errors() {
    let object = "<OBJECT_NAME>TEST</OBJECT_NAME>";
    let object_id = "<OBJECT_ID>2000-999Z</OBJECT_ID>";
    let epoch = "<EPOCH>2000-100T00:00:00.000</EPOCH>";
    for (label, source) in [
        (
            "unknown attitude state choice",
            mutated(XML, "<attitudeState>", "<attitudeState><UNKNOWN/>"),
        ),
        (
            "two attitude state choices",
            mutated(XML, "</quaternionEphemeris>",
                "</quaternionEphemeris><spin><EPOCH>2000-100T00:00:00.000</EPOCH><SPIN_ALPHA>1</SPIN_ALPHA><SPIN_DELTA>2</SPIN_DELTA><SPIN_ANGLE>3</SPIN_ANGLE><SPIN_ANGLE_VEL>4</SPIN_ANGLE_VEL></spin>",
            ),
        ),
        (
            "unknown container attribute",
            mutated(XML, "<attitudeState>", "<attitudeState unexpected=\"value\">"),
        ),
        (
            "illegal quaternion units",
            mutated(XML, "<Q1>-0.005068</Q1>", "<Q1 units=\"1\">-0.005068</Q1>"),
        ),
        (
            "missing Euler rotation sequence",
            mutated(XML, "<EULER_ROT_SEQ>XYZ</EULER_ROT_SEQ>", ""),
        ),
        (
            "missing angular velocity frame",
            mutated(XML, "<ANGVEL_FRAME>REF_FRAME_B</ANGVEL_FRAME>", ""),
        ),
        (
            "duplicate epoch",
            mutated(XML, epoch, &format!("{epoch}{epoch}")),
        ),
        (
            "reordered metadata",
            mutated(XML, &format!("{object}\n{object_id}"),
                &format!("{object_id}\n{object}"),
            ),
        ),
    ] {
        assert!(Aem::from_xml(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn aem_kvn_accepts_carriage_return_only_line_endings() {
    let cr_only = mutated(&KVN.replace("\r\n", "\n"), "\n", "\r");
    assert!(!cr_only.contains('\n'), "fixture still holds line feeds");

    let from_cr = Aem::from_kvn(&cr_only).unwrap();
    let from_lf = Aem::from_kvn(KVN).unwrap();
    assert_eq!(from_cr, from_lf);
}
