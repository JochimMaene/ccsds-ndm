// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::traits::Ndm;

const AEM_XML: &str = include_str!("../data/xml/aem_g11.xml");

#[test]
fn xml_parsing_rejects_non_calendar_adm_creation_date() {
    let invalid = AEM_XML.replace(
        "<CREATION_DATE>2008-071T17:09:49</CREATION_DATE>",
        "<CREATION_DATE>12345</CREATION_DATE>",
    );
    assert!(Aem::from_xml(&invalid).is_err());
}
