// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::OdmHeader;
use crate::types::parse_epoch;
use ccsds_ndm::messages::ocm as core_ocm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::Duration;
use ccsds_ndm::MessageType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;
use crate::common::{parse_object_description, parse_time_system};


/// Orbit Comprehensive Message (OCM).
///
/// An OCM specifies position and velocity of either a single object or an en masse parent/child
/// deployment scenario stemming from a single object. The OCM aggregates and extends OPM, OEM,
/// and OMM content in a single comprehensive hybrid message.
///
/// Key features:
/// - Support for single object or parent/child deployment scenarios.
/// - Aggregation of OPM, OMM, and OEM content.
/// - Extensive optional content including physical properties, covariance, maneuvers, and
/// perturbations.
/// - Well-suited for exchanges involving automated interaction and large object catalogs.
///
/// Parameters
/// ----------
/// header : OdmHeader
///     The message header.
/// segment : OcmSegment
///     The OCM data segment.
#[pyclass]
#[derive(Clone)]
pub struct Ocm {
    pub inner: core_ocm::Ocm,
}

#[pymethods]
impl Ocm {
    /// Create an OCM message from a string.
    ///
    /// Parameters
    /// ----------
    /// data : str
    ///     Input string/content.
    /// format : str, optional
    ///     Format ('kvn' or 'xml'). Auto-detected if None.
    ///
    /// Returns
    /// -------
    /// Ocm
    ///     The parsed Ocm object.
    #[staticmethod]
    fn from_str(data: &str, format: Option<&str>) -> PyResult<Self> {
        let inner = match format {
            Some("kvn") => ccsds_ndm::messages::ocm::Ocm::from_kvn(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some("xml") => ccsds_ndm::messages::ocm::Ocm::from_xml(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some(other) => {
                return Err(PyValueError::new_err(format!(
                    "Unsupported format '{}'. Use 'kvn' or 'xml'",
                    other
                )))
            }
            None => match ccsds_ndm::from_str(data) {
                Ok(MessageType::Ocm(ocm)) => ocm,
                Ok(other) => {
                    return Err(PyValueError::new_err(format!(
                        "Parsed message is not OCM (got {:?})",
                        other
                    )))
                }
                Err(e) => return Err(PyValueError::new_err(e.to_string())),
            },
        };
        Ok(Self { inner })
    }

    /// Create an OCM message from a file.
    ///
    /// Parameters
    /// ----------
    /// path : str
    ///     Path to the input file.
    /// format : str, optional
    ///     Format ('kvn' or 'xml'). Auto-detected if None.
    ///
    /// Returns
    /// -------
    /// Ocm
    ///     The parsed OCM object.
    #[staticmethod]
    fn from_file(path: &str, format: Option<&str>) -> PyResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| PyValueError::new_err(format!("Failed to read file: {}", e)))?;
        Self::from_str(&content, format)
    }

    /// Create a new OCM message.
    #[new]
    fn new(header: OdmHeader, segment: OcmSegment) -> Self {
        Self {
            inner: core_ocm::Ocm {
                header: header.inner,
                body: core_ocm::OcmBody {
                    segment: Box::new(segment.inner),
                },
                id: Some("CCSDS_OCM_VERS".to_string()),
                version: "3.0".to_string(),
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Ocm(object_name='{}')",
            self.inner
                .body
                .segment
                .metadata
                .object_name
                .as_ref()
                .unwrap_or(&"N/A".to_string())
        )
    }

    /// Orbit Comprehensive Message (OCM).
    ///
    /// An OCM specifies position and velocity of either a single object or an en masse parent/child
    /// deployment scenario stemming from a single object. The OCM aggregates and extends OPM, OEM,
    /// and OMM content in a single comprehensive hybrid message.
    ///
    /// Key features:
    /// - Support for single object or parent/child deployment scenarios.
    /// - Aggregation of OPM, OMM, and OEM content.
    /// - Extensive optional content including physical properties, covariance, maneuvers, and
    /// perturbations.
    /// - Well-suited for exchanges involving automated interaction and large object catalogs.
    ///
    /// :type: OdmHeader
    #[getter]
    fn get_header(&self) -> OdmHeader {
        OdmHeader {
            inner: self.inner.header.clone(),
        }
    }

    #[setter]
    fn set_header(&mut self, header: OdmHeader) {
        self.inner.header = header.inner;
    }

    /// The OCM data segment.
    ///
    /// :type: OcmSegment
    #[getter]
    fn get_segment(&self) -> OcmSegment {
        OcmSegment {
            inner: *self.inner.body.segment.clone(),
        }
    }

    #[setter]
    fn set_segment(&mut self, segment: OcmSegment) {
        self.inner.body.segment = Box::new(segment.inner);
    }
    /// Serialize to string.
    ///
    /// Parameters
    /// ----------
    /// format : str
    ///     Output format ('kvn' or 'xml').
    ///
    /// Returns
    /// -------
    /// str
    ///     The serialized string.
    fn to_str(&self, format: &str) -> PyResult<String> {
        match format {
            "kvn" => self
                .inner
                .to_kvn()
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
            "xml" => self
                .inner
                .to_xml()
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
            other => Err(PyValueError::new_err(format!(
                "Unsupported format '{}'. Use 'kvn' or 'xml'",
                other
            ))),
        }
    }

    /// Write to file.
    ///
    /// Parameters
    /// ----------
    /// path : str
    ///     Output file path.
    /// format : str
    ///     Output format ('kvn' or 'xml').
    fn to_file(&self, path: &str, format: &str) -> PyResult<()> {
        let data = self.to_str(format)?;
        match fs::write(path, data) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(format!(
                "Failed to write file: {}",
                e
            ))),
        }
    }
}

/// A single segment of the OCM.
///
/// Contains metadata and data sections.
///
/// Parameters
/// ----------
/// metadata : OcmMetadata
///     Segment metadata.
/// data : OcmData
///     Segment data blocks.
#[pyclass]
#[derive(Clone)]
pub struct OcmSegment {
    pub inner: core_ocm::OcmSegment,
}

#[pymethods]
impl OcmSegment {
    /// Create a new OCM Segment.
    #[new]
    fn new(metadata: OcmMetadata, data: OcmData) -> Self {
        Self {
            inner: core_ocm::OcmSegment {
                metadata: metadata.inner,
                data: data.inner,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "OcmSegment(object_name='{}')",
            self.inner
                .metadata
                .object_name
                .as_ref()
                .unwrap_or(&"N/A".to_string())
        )
    }

    /// A single segment of the OCM.
    ///
    /// Contains metadata and data sections.
    ///
    /// :type: OcmMetadata
    #[getter]
    fn get_metadata(&self) -> OcmMetadata {
        OcmMetadata {
            inner: self.inner.metadata.clone(),
        }
    }

    #[setter]
    fn set_metadata(&mut self, metadata: OcmMetadata) {
        self.inner.metadata = metadata.inner;
    }

    /// Segment data blocks.
    ///
    /// :type: OcmData
    #[getter]
    fn get_data(&self) -> OcmData {
        OcmData {
            inner: self.inner.data.clone(),
        }
    }

    #[setter]
    fn set_data(&mut self, data: OcmData) {
        self.inner.data = data.inner;
    }
}

/// OCM Metadata Section.
///
/// Parameters
/// ----------
/// time_system : str
///     Time system that shall be used for all absolute time stamps in the message.
/// epoch_tzero : str
///     Epoch to which all relative times in the message are referenced (ISO 8601).
/// object_name : str, optional
///     Name of the space object that the message is associated with.
/// international_designator : str, optional
///     The COSPAR international designator of the space object.
/// catalog_name : str, optional
///     The name of the satellite catalog used for the space object identification.
/// object_designator : str, optional
///     The unique satellite identification designator used in the specified catalog.
/// alternate_names : str, optional
///     Alternate name(s) by which the space object is known.
/// originator_poc : str, optional
///     Originator Point-of-Contact.
/// originator_position : str, optional
///     Contact position of the originator PoC.
/// originator_phone : str, optional
///     Originator PoC phone number.
/// originator_email : str, optional
///     Originator PoC email address.
/// originator_address : str, optional
///     Originator's physical address.
/// tech_org : str, optional
///     Technical organization (creating agency or operator).
/// tech_poc : str, optional
///     Technical Point-of-Contact.
/// tech_position : str, optional
///     Contact position of the technical PoC.
/// tech_phone : str, optional
///     Technical PoC phone number.
/// tech_email : str, optional
///     Technical PoC email address.
/// tech_address : str, optional
///     Technical PoC physical address.
/// previous_message_id : str, optional
///     Identifier for the previous OCM message.
/// next_message_id : str, optional
///     Identifier for the anticipated next OCM message.
/// adm_msg_link : str, optional
///     Identifier of linked Attitude Data Message.
/// cdm_msg_link : str, optional
///     Identifier of linked Conjunction Data Message.
/// prm_msg_link : str, optional
///     Identifier of linked Pointing Request Message.
/// rdm_msg_link : str, optional
///     Identifier of linked Reentry Data Message.
/// tdm_msg_link : str, optional
///     Identifier of linked Tracking Data Message.
/// operator : str, optional
///     Operator of the space object.
/// owner : str, optional
///     Owner of the space object.
/// country : str, optional
///     Country of the owner or operator of the space object.
/// constellation : str, optional
///     Name of the constellation the space object belongs to.
/// object_type : str, optional
///     Type of object (PAYLOAD, ROCKET_BODY, DEBRIS, etc.).
/// ops_status : str, optional
///     Operational status of the space object.
/// orbit_category : str, optional
///     Orbit category (LEO, GEO, HEO, etc.).
/// ocm_data_elements : str, optional
///     List of data elements included in the OCM message.
/// sclk_offset_at_epoch : float, optional
///     Spacecraft clock offset at EPOCH_TZERO (s).
/// sclk_sec_per_si_sec : float, optional
///     Spacecraft clock scale factor (s/SI-s).
/// previous_message_epoch : str, optional
///     Epoch of the previous message (ISO 8601).
/// next_message_epoch : str, optional
///     Anticipated epoch of the next message (ISO 8601).
/// start_time : str, optional
///     Time of the earliest data in the message (ISO 8601).
/// stop_time : str, optional
///     Time of the latest data in the message (ISO 8601).
/// time_span : float, optional
///     Approximate time span covered by the data (d).
/// taimutc_at_tzero : float, optional
///     TAI minus UTC difference at EPOCH_TZERO (s).
/// next_leap_epoch : str, optional
///     Epoch of the next leap second (ISO 8601).
/// next_leap_taimutc : float, optional
///     TAI minus UTC difference at NEXT_LEAP_EPOCH (s).
/// ut1mutc_at_tzero : float, optional
///     UT1 minus UTC difference at EPOCH_TZERO (s).
/// eop_source : str, optional
///     Source of Earth Orientation Parameters.
/// interp_method_eop : str, optional
///     Interpolation method for EOP data.
/// celestial_source : str, optional
///     Source of celestial body ephemerides.
/// comment : list[str], optional
///     Comments for the metadata block.
///
/// Attributes
/// ----------
/// time_system : str
///     Time system.
/// epoch_tzero : str
///     Epoch T-Zero.
///     ... (see Parameters for full list)
#[pyclass]
#[derive(Clone)]
pub struct OcmMetadata {
    pub inner: core_ocm::OcmMetadata,
}

#[pymethods]
impl OcmMetadata {
    /// Create a new OcmMetadata object.
    #[new]
    #[pyo3(signature = (
        *,
        epoch_tzero,
        time_system=None,
        object_name=None,

        international_designator=None,
        catalog_name=None,
        object_designator=None,
        alternate_names=None,
        originator_poc=None,
        originator_position=None,
        originator_phone=None,
        originator_email=None,
        originator_address=None,
        tech_org=None,
        tech_poc=None,
        tech_position=None,
        tech_phone=None,
        tech_email=None,
        tech_address=None,
        previous_message_id=None,
        next_message_id=None,
        adm_msg_link=None,
        cdm_msg_link=None,
        prm_msg_link=None,
        rdm_msg_link=None,
        tdm_msg_link=None,
        operator=None,
        owner=None,
        country=None,
        constellation=None,
        object_type=None,
        ops_status=None,
        orbit_category=None,
        ocm_data_elements=None,
        sclk_offset_at_epoch=None,
        sclk_sec_per_si_sec=None,
        previous_message_epoch=None,
        next_message_epoch=None,
        start_time=None,
        stop_time=None,
        time_span=None,
        taimutc_at_tzero=None,
        next_leap_epoch=None,
        next_leap_taimutc=None,
        ut1mutc_at_tzero=None,
        eop_source=None,
        interp_method_eop=None,
        celestial_source=None,
        comment=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        epoch_tzero: String,
        time_system: Option<Bound<'_, PyAny>>,
        object_name: Option<String>,


        international_designator: Option<String>,
        catalog_name: Option<String>,
        object_designator: Option<String>,
        alternate_names: Option<String>,
        originator_poc: Option<String>,
        originator_position: Option<String>,
        originator_phone: Option<String>,
        originator_email: Option<String>,
        originator_address: Option<String>,
        tech_org: Option<String>,
        tech_poc: Option<String>,
        tech_position: Option<String>,
        tech_phone: Option<String>,
        tech_email: Option<String>,
        tech_address: Option<String>,
        previous_message_id: Option<String>,
        next_message_id: Option<String>,
        adm_msg_link: Option<String>,
        cdm_msg_link: Option<String>,
        prm_msg_link: Option<String>,
        rdm_msg_link: Option<String>,
        tdm_msg_link: Option<String>,
        operator: Option<String>,
        owner: Option<String>,
        country: Option<String>,
        constellation: Option<String>,
        object_type: Option<Bound<'_, PyAny>>,
        ops_status: Option<String>,
        orbit_category: Option<String>,
        ocm_data_elements: Option<String>,
        sclk_offset_at_epoch: Option<f64>,
        sclk_sec_per_si_sec: Option<f64>,
        previous_message_epoch: Option<String>,
        next_message_epoch: Option<String>,
        start_time: Option<String>,
        stop_time: Option<String>,
        time_span: Option<f64>,
        taimutc_at_tzero: Option<f64>,
        next_leap_epoch: Option<String>,
        next_leap_taimutc: Option<f64>,
        ut1mutc_at_tzero: Option<f64>,
        eop_source: Option<String>,
        interp_method_eop: Option<String>,
        celestial_source: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        use ccsds_ndm::types::{DayInterval, TimeOffset};

        let time_system = match time_system {
            Some(ref ob) => parse_time_system(ob)?,
            None => "UTC".to_string(),
        };

        let object_type_enum = match object_type {
            Some(ref ob) => Some(parse_object_description(ob)?),
            None => None,
        };

        Ok(Self {
            inner: core_ocm::OcmMetadata {
                comment: comment.unwrap_or_default(),
                object_name,
                international_designator,
                catalog_name,
                object_designator,
                alternate_names,
                originator_poc,
                originator_position,
                originator_phone,
                originator_email,
                originator_address,
                tech_org,
                tech_poc,
                tech_position,
                tech_phone,
                tech_email,
                tech_address,
                previous_message_id,
                next_message_id,
                adm_msg_link,
                cdm_msg_link,
                prm_msg_link,
                rdm_msg_link,
                tdm_msg_link,
                operator,
                owner,
                country,
                constellation,
                object_type: object_type_enum,
                time_system,
                epoch_tzero: parse_epoch(&epoch_tzero)?,
                ops_status,
                orbit_category,
                ocm_data_elements,
                sclk_offset_at_epoch: sclk_offset_at_epoch.map(|v| TimeOffset {
                    value: v,
                    units: None,
                }),
                sclk_sec_per_si_sec: sclk_sec_per_si_sec.map(|v| Duration {
                    value: v,
                    units: None,
                }),
                previous_message_epoch: previous_message_epoch
                    .map(|s| parse_epoch(&s))
                    .transpose()?,
                next_message_epoch: next_message_epoch.map(|s| parse_epoch(&s)).transpose()?,
                start_time: start_time.map(|s| parse_epoch(&s)).transpose()?,
                stop_time: stop_time.map(|s| parse_epoch(&s)).transpose()?,
                time_span: time_span.map(|v| DayInterval {
                    value: v,
                    units: None,
                }),
                taimutc_at_tzero: taimutc_at_tzero.map(|v| TimeOffset {
                    value: v,
                    units: None,
                }),
                next_leap_epoch: next_leap_epoch.map(|s| parse_epoch(&s)).transpose()?,
                next_leap_taimutc: next_leap_taimutc.map(|v| TimeOffset {
                    value: v,
                    units: None,
                }),
                ut1mutc_at_tzero: ut1mutc_at_tzero.map(|v| TimeOffset {
                    value: v,
                    units: None,
                }),
                eop_source,
                interp_method_eop,
                celestial_source,
            },
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "OcmMetadata(object_name='{}')",
            self.inner
                .object_name
                .as_ref()
                .unwrap_or(&"N/A".to_string())
        )
    }

    /// Time system for all absolute time stamps in this OCM including EPOCH_TZERO. Select from
    /// the accepted set of values indicated in annex B, subsection B3. This field is used by
    /// all OCM data blocks. If the SCLK timescale is selected, then 'EPOCH_TZERO' shall be
    /// interpreted as the spacecraft clock epoch and both SCLK_OFFSET_AT_EPOCH and
    /// SCLK_SEC_PER_SI_SEC shall be supplied.
    ///
    /// Examples: UTC
    ///
    /// :type: str
    #[getter]
    fn get_time_system(&self) -> String {
        self.inner.time_system.clone()
    }
    #[setter]
    fn set_time_system(&mut self, value: String) {
        self.inner.time_system = value;
    }

    /// Default epoch to which all relative times are referenced in data blocks (for format
    /// specification, see 7.5.10). The time scale of EPOCH_TZERO is controlled via the
    /// ‘TIME_SYSTEM' keyword, with the exception that for the SCLK timescale, EPOCH_TZERO
    /// shall be interpreted as being in the UTC timescale. This field is used by all OCM data
    /// blocks.
    ///
    /// Examples: 2001-11-06T11:17:33
    ///
    /// :type: str
    #[getter]
    fn get_epoch_tzero(&self) -> String {
        self.inner.epoch_tzero.as_str().to_string()
    }
    #[setter]
    fn set_epoch_tzero(&mut self, value: String) -> PyResult<()> {
        self.inner.epoch_tzero = parse_epoch(&value)?;
        Ok(())
    }

    /// Free-text field containing the name of the object. While there is no CCSDS-based
    /// restriction on the value for this keyword, it is recommended to use names from either
    /// the UN Office of Outer Space Affairs designator index (reference `[3]`, which include
    /// Object name and international designator of the participant), the spacecraft operator,
    /// or a State Actor or commercial Space Situational Awareness (SSA) provider maintaining
    /// the ‘CATALOG_NAME’ space catalog. If OBJECT_NAME is not listed in reference `[3]` or the
    /// content is either unknown (uncorrelated) or cannot be disclosed, the value should be
    /// set to UNKNOWN (or this keyword omitted).
    ///
    /// Examples: SPOT-7, ENVISAT, IRIDIUM NEXT-8, INTELSAT G-15, UNKNOWN
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_object_name(&self) -> Option<String> {
        self.inner.object_name.clone()
    }
    #[setter]
    fn set_object_name(&mut self, value: Option<String>) {
        self.inner.object_name = value;
    }

    /// Free-text field containing an international designator for the object as assigned by
    /// the UN Committee on Space Research (COSPAR). Such designator values shall have the
    /// following COSPAR format: YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three-digit
    /// serial number of launch in year YYYY (with leading zeros). P{PP} = At least one capital
    /// letter for the identification of the part brought into space by the launch. If the
    /// object has no international designator or the content is either unknown (uncorrelated)
    /// or cannot be disclosed, the value should be set to UNKNOWN (or this keyword omitted).
    /// NOTE—The international designator was typically specified by 'OBJECT_ID' in the OPM,
    /// OMM, and OEM.
    ///
    /// Examples: 2000-052A, 1996-068A, 2000-053A, 1996-008A, UNKNOWN
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_international_designator(&self) -> Option<String> {
        self.inner.international_designator.clone()
    }
    #[setter]
    fn set_international_designator(&mut self, value: Option<String>) {
        self.inner.international_designator = value;
    }

    /// Free-text field containing the satellite catalog source (or source agency or operator,
    /// value to be drawn from the SANA registry list of Space Object Catalogs at
    /// <https://sanaregistry.org/r/space_object_catalog>, or alternatively, from the list of
    /// organizations listed in the 'Abbreviation' column of the SANA Organizations registry at
    /// <https://www.sanaregistry.org/r/organizations>) from which 'OBJECT_DESIGNATOR' was
    /// obtained.
    ///
    /// Examples: CSPOC, RFSA, ESA, COMSPOC
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_catalog_name(&self) -> Option<String> {
        self.inner.catalog_name.clone()
    }
    #[setter]
    fn set_catalog_name(&mut self, value: Option<String>) {
        self.inner.catalog_name = value;
    }

    /// Free-text field specification of the unique satellite identification designator for the
    /// object, as reflected in the catalog whose name is 'CATALOG_NAME'. If the ID is not known
    /// (uncorrelated object) or cannot be disclosed, 'UNKNOWN' may be used (or this keyword
    /// omitted).
    ///
    /// Examples: 22444, 18SPCS 18571, 2147483648_04ae[...]d84c, UNKNOWN
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_object_designator(&self) -> Option<String> {
        self.inner.object_designator.clone()
    }
    #[setter]
    fn set_object_designator(&mut self, value: Option<String>) {
        self.inner.object_designator = value;
    }

    /// Free-text comma-delimited field containing alternate name(s) of this space object,
    /// including assigned names used by spacecraft operator, State Actors, commercial SSA
    /// providers, and/or media.
    ///
    /// Examples: SV08, IN8
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_alternate_names(&self) -> Option<String> {
        self.inner.alternate_names.clone()
    }
    #[setter]
    fn set_alternate_names(&mut self, value: Option<String>) {
        self.inner.alternate_names = value;
    }

    /// Free-text field containing originator or programmatic Point-of-Contact (POC) for OCM.
    ///
    /// Examples: Mr. Rodgers
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_originator_poc(&self) -> Option<String> {
        self.inner.originator_poc.clone()
    }
    #[setter]
    fn set_originator_poc(&mut self, value: Option<String>) {
        self.inner.originator_poc = value;
    }

    /// Free-text field containing contact position of the originator PoC.
    ///
    /// Examples: Flight Dynamics, Mission Design Lead
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_originator_position(&self) -> Option<String> {
        self.inner.originator_position.clone()
    }
    #[setter]
    fn set_originator_position(&mut self, value: Option<String>) {
        self.inner.originator_position = value;
    }

    /// Free-text field containing originator PoC phone number.
    ///
    /// Examples: +12345678901
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_originator_phone(&self) -> Option<String> {
        self.inner.originator_phone.clone()
    }
    #[setter]
    fn set_originator_phone(&mut self, value: Option<String>) {
        self.inner.originator_phone = value;
    }

    /// Free-text field containing originator PoC email address.
    ///
    /// Examples: JOHN.DOE@SOMEWHERE.ORG
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_originator_email(&self) -> Option<String> {
        self.inner.originator_email.clone()
    }
    #[setter]
    fn set_originator_email(&mut self, value: Option<String>) {
        self.inner.originator_email = value;
    }

    /// Free-text field containing originator's physical address information for OCM creator
    /// (suggest comma-delimited address lines).
    ///
    /// Examples: 5040 Spaceflight Ave., Cocoa Beach, FL, USA, 12345
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_originator_address(&self) -> Option<String> {
        self.inner.originator_address.clone()
    }
    #[setter]
    fn set_originator_address(&mut self, value: Option<String>) {
        self.inner.originator_address = value;
    }

    /// Free-text field containing the creating agency or operator (value should be drawn from
    /// the 'Abbreviation' column of the SANA Organizations registry at
    /// <https://www.sanaregistry.org/r/organizations>).
    ///
    /// Examples: NASA, ESA, JAXA
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_tech_org(&self) -> Option<String> {
        self.inner.tech_org.clone()
    }
    #[setter]
    fn set_tech_org(&mut self, value: Option<String>) {
        self.inner.tech_org = value;
    }

    /// Free-text field containing technical PoC for OCM.
    ///
    /// Examples: Maxwell Smart
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_tech_poc(&self) -> Option<String> {
        self.inner.tech_poc.clone()
    }
    #[setter]
    fn set_tech_poc(&mut self, value: Option<String>) {
        self.inner.tech_poc = value;
    }

    /// Free-text field containing contact position of the technical PoC.
    ///
    /// Examples: Flight Dynamics, Mission Design Lead
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_tech_position(&self) -> Option<String> {
        self.inner.tech_position.clone()
    }
    #[setter]
    fn set_tech_position(&mut self, value: Option<String>) {
        self.inner.tech_position = value;
    }

    /// Free-text field containing technical PoC phone number.
    ///
    /// Examples: +49615130312
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_tech_phone(&self) -> Option<String> {
        self.inner.tech_phone.clone()
    }
    #[setter]
    fn set_tech_phone(&mut self, value: Option<String>) {
        self.inner.tech_phone = value;
    }

    /// Free-text field containing technical PoC email address.
    ///
    /// Examples: JOHN.DOE@SOMEWHERE.ORG
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_tech_email(&self) -> Option<String> {
        self.inner.tech_email.clone()
    }
    #[setter]
    fn set_tech_email(&mut self, value: Option<String>) {
        self.inner.tech_email = value;
    }

    /// Free-text field containing technical PoC physical address information for OCM creator
    /// (suggest comma-delimited address lines).
    ///
    /// Examples: 5040 Spaceflight Ave., Cocoa Beach, FL, USA, 12345
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_tech_address(&self) -> Option<String> {
        self.inner.tech_address.clone()
    }
    #[setter]
    fn set_tech_address(&mut self, value: Option<String>) {
        self.inner.tech_address = value;
    }

    /// Comments (a contiguous set of one or more comment lines may be provided in the OCM
    /// Metadata section; see 7.8 for comment formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    // === Message Linking Fields ===
    /// Free-text field containing an ID that uniquely identifies the previous message from
    /// this message originator for this space object. The format and content of the message
    /// identifier value are at the discretion of the originator. NOTE—One may provide the
    /// previous message ID without supplying the 'PREVIOUS_MESSAGE_EPOCH' keyword, and vice
    /// versa.
    ///
    /// Examples: OCM 201113719184, ABC-12_33
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_previous_message_id(&self) -> Option<String> {
        self.inner.previous_message_id.clone()
    }
    #[setter]
    fn set_previous_message_id(&mut self, value: Option<String>) {
        self.inner.previous_message_id = value;
    }

    /// Free-text field containing an ID that uniquely identifies the next message from this
    /// message originator for this space object. The format and content of the message
    /// identifier value are at the discretion of the originator. NOTE—One may provide the next
    /// message ID without supplying the ‘NEXT_MESSAGE_EPOCH' keyword, and vice versa.
    ///
    /// Examples: OCM 201113719186, ABC-12_35
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_next_message_id(&self) -> Option<String> {
        self.inner.next_message_id.clone()
    }
    #[setter]
    fn set_next_message_id(&mut self, value: Option<String>) {
        self.inner.next_message_id = value;
    }

    /// Free-text field containing a unique identifier of Attitude Data Message (ADM)
    /// (reference `[10]`) that are linked (relevant) to this Orbit Data Message.
    ///
    /// Examples: ADM_MSG_35132.txt, ADM_ID_0572
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_adm_msg_link(&self) -> Option<String> {
        self.inner.adm_msg_link.clone()
    }
    #[setter]
    fn set_adm_msg_link(&mut self, value: Option<String>) {
        self.inner.adm_msg_link = value;
    }

    /// Free-text field containing a unique identifier of Conjunction Data Message (CDM)
    /// (reference `[14]`) that are linked (relevant) to this Orbit Data Message.
    ///
    /// Examples: CDM_MSG_35132.txt, CDM_ID_8257
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_cdm_msg_link(&self) -> Option<String> {
        self.inner.cdm_msg_link.clone()
    }
    #[setter]
    fn set_cdm_msg_link(&mut self, value: Option<String>) {
        self.inner.cdm_msg_link = value;
    }

    /// Free-text field containing a unique identifier of Pointing Request Message (PRM)
    /// (reference `[13]`) that are linked (relevant) to this Orbit Data Message.
    ///
    /// Examples: PRM_MSG_35132.txt, PRM_ID_6897
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_prm_msg_link(&self) -> Option<String> {
        self.inner.prm_msg_link.clone()
    }
    #[setter]
    fn set_prm_msg_link(&mut self, value: Option<String>) {
        self.inner.prm_msg_link = value;
    }

    /// Free-text field containing a unique identifier of Reentry Data Message (RDM)
    /// (reference `[12]`) that are linked (relevant) to this Orbit Data Message.
    ///
    /// Examples: RDM_MSG_35132.txt, RDM_ID_1839
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_rdm_msg_link(&self) -> Option<String> {
        self.inner.rdm_msg_link.clone()
    }
    #[setter]
    fn set_rdm_msg_link(&mut self, value: Option<String>) {
        self.inner.rdm_msg_link = value;
    }

    /// Free-text string containing a comma-separated list of file name(s) and/or associated
    /// identification number(s) of Tracking Data Message (TDM) (reference `[9]`) observations
    /// upon which this OD is based.
    ///
    /// Examples: TDM_MSG_37.txt, TDM_835, TDM_836
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_tdm_msg_link(&self) -> Option<String> {
        self.inner.tdm_msg_link.clone()
    }
    #[setter]
    fn set_tdm_msg_link(&mut self, value: Option<String>) {
        self.inner.tdm_msg_link = value;
    }

    // === Object Information Fields ===
    /// Free-text field containing the operator of the space object.
    ///
    /// Examples: INTELSAT
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_operator(&self) -> Option<String> {
        self.inner.operator.clone()
    }
    #[setter]
    fn set_operator(&mut self, value: Option<String>) {
        self.inner.operator = value;
    }

    /// Free-text field containing the owner of the space object.
    ///
    /// Examples: SIRIUS
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_owner(&self) -> Option<String> {
        self.inner.owner.clone()
    }
    #[setter]
    fn set_owner(&mut self, value: Option<String>) {
        self.inner.owner = value;
    }

    /// Free-text field containing the name of the country, country code, or country
    /// abbreviation where the space object owner is based.
    ///
    /// Examples: US, SPAIN
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_country(&self) -> Option<String> {
        self.inner.country.clone()
    }
    #[setter]
    fn set_country(&mut self, value: Option<String>) {
        self.inner.country = value;
    }

    /// Free-text field containing the name of the constellation to which this space object
    /// belongs.
    ///
    /// Examples: SPIRE
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_constellation(&self) -> Option<String> {
        self.inner.constellation.clone()
    }
    #[setter]
    fn set_constellation(&mut self, value: Option<String>) {
        self.inner.constellation = value;
    }

    /// Specification of the type of object. Select from the accepted set of values indicated
    /// in annex B, subsection B11.
    ///
    /// Examples: PAYLOAD, ROCKET BODY, DEBRIS, UNKNOWN, OTHER
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_object_type(&self) -> Option<String> {
        self.inner.object_type.as_ref().map(|t| format!("{:?}", t))
    }
    #[setter]
    fn set_object_type(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.object_type = value
            .map(|s| s.parse())
            .transpose()
            .map_err(|e: ccsds_ndm::error::EnumParseError| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Specification of the operational status of the space object. Select from the accepted
    /// set of values indicated in annex B, subsection B12.
    ///
    /// Examples: OPERATIONAL
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_ops_status(&self) -> Option<String> {
        self.inner.ops_status.clone()
    }
    #[setter]
    fn set_ops_status(&mut self, value: Option<String>) {
        self.inner.ops_status = value;
    }

    /// Specification of the type of orbit. Select from the accepted set of values indicated in
    /// annex B, subsection B14.
    ///
    /// Examples: GEO, LEO
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_orbit_category(&self) -> Option<String> {
        self.inner.orbit_category.clone()
    }
    #[setter]
    fn set_orbit_category(&mut self, value: Option<String>) {
        self.inner.orbit_category = value;
    }

    /// Comma-delimited list of elements of information data blocks included in this message.
    /// The order shall be the same as the order of the data blocks in the message. Values shall
    /// be confined to the following list: ORB, PHYS, COV, MAN, PERT, OD, and USER. If the OCM
    /// contains multiple ORB, COV, or MAN data blocks (as allowed by table 6-1), the
    /// corresponding ORB, COV, or MAN entry shall be duplicated to match.
    ///
    /// Examples: ORB, ORB, PHYS, COV, MAN, MAN, PERT, OD, USER
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_ocm_data_elements(&self) -> Option<String> {
        self.inner.ocm_data_elements.clone()
    }
    #[setter]
    fn set_ocm_data_elements(&mut self, value: Option<String>) {
        self.inner.ocm_data_elements = value;
    }

    // === Time-Related Fields ===
    /// Defines the number of spacecraft clock counts existing at EPOCH_TZERO. This is only
    /// used if the SCLK timescale is employed by the user.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_sclk_offset_at_epoch(&self) -> Option<f64> {
        self.inner.sclk_offset_at_epoch.as_ref().map(|t| t.value)
    }
    #[setter]
    fn set_sclk_offset_at_epoch(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::TimeOffset;
        self.inner.sclk_offset_at_epoch = value.map(|v| TimeOffset {
            value: v,
            units: None,
        });
    }
    /// Defines the current number of clock seconds occurring during one SI second. It should be
    /// noted that this clock rate may vary with time and is the current approximate value.
    /// This is only used if the SCLK timescale is employed by the user.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_sclk_sec_per_si_sec(&self) -> Option<f64> {
        self.inner.sclk_sec_per_si_sec.as_ref().map(|d| d.value)
    }
    #[setter]
    fn set_sclk_sec_per_si_sec(&mut self, value: Option<f64>) {
        self.inner.sclk_sec_per_si_sec = value.map(|v| Duration {
            value: v,
            units: None,
        });
    }
    /// Creation epoch of the previous message from this originator for this space object. (For
    /// format specification, see 7.5.10.) NOTE—One may provide the previous message epoch
    /// without supplying the PREVIOUS_MESSAGE_ID, and vice versa.
    ///
    /// Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_previous_message_epoch(&self) -> Option<String> {
        self.inner
            .previous_message_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_previous_message_epoch(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.previous_message_epoch = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }
    /// Anticipated (or actual) epoch of the next message from this originator for this space
    /// object. (For format specification, see 7.5.10.) NOTE—One may provide the next message
    /// epoch without supplying the NEXT_MESSAGE_ID, and vice versa.
    ///
    /// Examples: 2001-11-07T11:17:33
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_next_message_epoch(&self) -> Option<String> {
        self.inner
            .next_message_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_next_message_epoch(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.next_message_epoch = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }
    /// Time of the earliest data contained in the OCM, specified as either a relative or
    /// absolute time tag.
    ///
    /// Examples: 2001-11-06T00:00:00
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_start_time(&self) -> Option<String> {
        self.inner
            .start_time
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_start_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.start_time = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }
    /// Time of the latest data contained in the OCM, specified as either a relative or absolute
    /// time tag.
    ///
    /// Examples: 2001-11-08T00:00:00
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_stop_time(&self) -> Option<String> {
        self.inner
            .stop_time
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_stop_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.stop_time = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }
    /// Span of time that the OCM covers, measured in days. TIME_SPAN is defined as
    /// (STOP_TIME-START_TIME), measured in days, irrespective of whether START_TIME or
    /// STOP_TIME are provided by the message creator.
    ///
    /// Units: d
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_time_span(&self) -> Option<f64> {
        self.inner.time_span.as_ref().map(|t| t.value)
    }
    #[setter]
    fn set_time_span(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::DayInterval;
        self.inner.time_span = value.map(|v| DayInterval {
            value: v,
            units: None,
        });
    }
    /// Difference (TAI – UTC) in seconds (i.e., total number of leap seconds elapsed since
    /// 1958) as modeled by the message originator at epoch 'EPOCH_TZERO'.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_taimutc_at_tzero(&self) -> Option<f64> {
        self.inner.taimutc_at_tzero.as_ref().map(|t| t.value)
    }
    #[setter]
    fn set_taimutc_at_tzero(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::TimeOffset;
        self.inner.taimutc_at_tzero = value.map(|v| TimeOffset {
            value: v,
            units: None,
        });
    }
    /// Epoch of next leap second, specified as an absolute time tag.
    ///
    /// Examples: 2016-12-31T23:59:60
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_next_leap_epoch(&self) -> Option<String> {
        self.inner
            .next_leap_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_next_leap_epoch(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.next_leap_epoch = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }
    /// Difference (TAI – UTC) in seconds (i.e., total number of leap seconds elapsed since
    /// 1958) incorporated by the message originator at epoch 'NEXT_LEAP_EPOCH'. This keyword
    /// should be provided if NEXT_LEAP_EPOCH is supplied.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_next_leap_taimutc(&self) -> Option<f64> {
        self.inner.next_leap_taimutc.as_ref().map(|t| t.value)
    }
    #[setter]
    fn set_next_leap_taimutc(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::TimeOffset;
        self.inner.next_leap_taimutc = value.map(|v| TimeOffset {
            value: v,
            units: None,
        });
    }

    /// Difference (UT1 – UTC) in seconds, as modeled by the originator at epoch 'EPOCH_TZERO'.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_ut1mutc_at_tzero(&self) -> Option<f64> {
        self.inner.ut1mutc_at_tzero.as_ref().map(|t| t.value)
    }
    #[setter]
    fn set_ut1mutc_at_tzero(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::TimeOffset;
        self.inner.ut1mutc_at_tzero = value.map(|v| TimeOffset {
            value: v,
            units: None,
        });
    }
    /// Free-text field specifying the source and version of the message originator's Earth
    /// Orientation Parameters (EOP) used in the creation of this message, including leap
    /// seconds, TAI – UT1, etc.
    ///
    /// Examples: CELESTRAK_20201028
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_eop_source(&self) -> Option<String> {
        self.inner.eop_source.clone()
    }
    #[setter]
    fn set_eop_source(&mut self, value: Option<String>) {
        self.inner.eop_source = value;
    }

    /// Free-text field specifying the method used to select or interpolate sequential EOP data.
    ///
    /// Examples: PRECEDING_VALUE, NEAREST_NEIGHBOR, LINEAR, LAGRANGE_ORDER_5
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_interp_method_eop(&self) -> Option<String> {
        self.inner.interp_method_eop.clone()
    }
    #[setter]
    fn set_interp_method_eop(&mut self, value: Option<String>) {
        self.inner.interp_method_eop = value;
    }

    /// Free-text field specifying the source and version of the message originator's celestial
    /// body (e.g., Sun/Earth/Planetary) ephemeris data used in the creation of this message.
    ///
    /// Examples: JPL_DE_FILES
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_celestial_source(&self) -> Option<String> {
        self.inner.celestial_source.clone()
    }
    #[setter]
    fn set_celestial_source(&mut self, value: Option<String>) {
        self.inner.celestial_source = value;
    }
}

/// OCM Data Section.
///
/// This struct is the primary data container for the OCM. It holds all the
/// different data blocks, such as trajectory, physical properties, covariance,
/// maneuvers, and other related information.
#[pyclass]
#[derive(Clone)]
pub struct OcmData {
    pub inner: core_ocm::OcmData,
}

#[pymethods]
impl OcmData {
    #[new]
    fn new() -> Self {
        Self {
            inner: core_ocm::OcmData::default(),
        }
    }

    fn __repr__(&self) -> String {
        format!("OcmData(traj_states={})", self.inner.traj.len())
    }

    /// List of trajectory state time history blocks.
    ///
    /// :type: list[OcmTrajState]
    #[getter]
    fn get_traj(&self) -> Vec<OcmTrajState> {
        self.inner
            .traj
            .iter()
            .map(|t| OcmTrajState { inner: t.clone() })
            .collect()
    }

    #[setter]
    fn set_traj(&mut self, value: Vec<OcmTrajState>) {
        self.inner.traj = value.into_iter().map(|t| t.inner).collect();
    }

    /// Space object physical characteristics.
    ///
    /// :type: Optional[OcmPhysicalDescription]
    #[getter]
    fn get_phys(&self) -> Option<OcmPhysicalDescription> {
        self.inner
            .phys
            .as_ref()
            .map(|p| OcmPhysicalDescription { inner: p.clone() })
    }

    #[setter]
    fn set_phys(&mut self, value: Option<OcmPhysicalDescription>) {
        self.inner.phys = value.map(|p| p.inner);
    }

    /// List of maneuver specifications.
    ///
    /// :type: list[OcmManeuverParameters]
    #[getter]
    fn get_man(&self) -> Vec<OcmManeuverParameters> {
        self.inner
            .man
            .iter()
            .map(|m| OcmManeuverParameters { inner: m.clone() })
            .collect()
    }
    #[setter]
    fn set_man(&mut self, value: Vec<OcmManeuverParameters>) {
        self.inner.man = value.into_iter().map(|m| m.inner).collect();
    }

    /// List of covariance time history blocks.
    ///
    /// :type: list[OcmCovarianceMatrix]
    #[getter]
    fn get_cov(&self) -> Vec<OcmCovarianceMatrix> {
        self.inner
            .cov
            .iter()
            .map(|c| OcmCovarianceMatrix { inner: c.clone() })
            .collect()
    }
    #[setter]
    fn set_cov(&mut self, value: Vec<OcmCovarianceMatrix>) {
        self.inner.cov = value.into_iter().map(|c| c.inner).collect();
    }

    /// Perturbation parameters.
    ///
    /// :type: Optional[OcmPerturbations]
    #[getter]
    fn get_pert(&self) -> Option<OcmPerturbations> {
        self.inner
            .pert
            .as_ref()
            .map(|p| OcmPerturbations { inner: p.clone() })
    }
    #[setter]
    fn set_pert(&mut self, value: Option<OcmPerturbations>) {
        self.inner.pert = value.map(|p| p.inner);
    }

    /// Orbit determination data.
    ///
    /// :type: Optional[OcmOdParameters]
    #[getter]
    fn get_od(&self) -> Option<OcmOdParameters> {
        self.inner
            .od
            .as_ref()
            .map(|o| OcmOdParameters { inner: o.clone() })
    }
    #[setter]
    fn set_od(&mut self, value: Option<OcmOdParameters>) {
        self.inner.od = value.map(|o| o.inner);
    }

    /// User-defined parameters.
    ///
    /// :type: UserDefined | None
    #[getter]
    fn get_user(&self) -> Option<crate::types::UserDefined> {
        self.inner
            .user
            .as_ref()
            .map(|u| crate::types::UserDefined { inner: u.clone() })
    }
    #[setter]
    fn set_user(&mut self, value: Option<crate::types::UserDefined>) {
        self.inner.user = value.map(|u| u.inner);
    }
}

/// A block of trajectory state data, which can be a time history of states.
///
/// Parameters
/// ----------
/// center_name : str
///     Origin of the orbit reference frame.
/// traj_ref_frame : str
///     Reference frame of the trajectory state time history.
/// traj_type : str
///     Specifies the trajectory state element set type.
/// traj_lines : list[TrajLine]
///     Contiguous set of trajectory state data lines.
/// traj_id : str, optional
///     Identification number for this trajectory state time history block.
/// traj_prev_id : str, optional
///     Identification number for the previous trajectory state time history.
/// traj_next_id : str, optional
///     Identification number for the next trajectory state time history.
/// traj_basis : str, optional
///     The basis of this trajectory state time history data (PREDICTED, DETERMINED, etc.).
/// traj_basis_id : str, optional
///     Identification number for the telemetry dataset, orbit determination, or simulation.
/// interpolation : str, optional
///     Recommended interpolation method for the ephemeris data.
/// interpolation_degree : int, optional
///     Recommended interpolation degree.
/// propagator : str, optional
///     Name of the orbit propagator used to create this trajectory state time history.
/// traj_frame_epoch : str, optional
///     Epoch of the orbit data reference frame, if not intrinsic to the definition.
/// useable_start_time : str, optional
///     Start time of the useable time span covered by the ephemeris data.
/// useable_stop_time : str, optional
///     Stop time of the useable time span covered by the ephemeris data.
/// orb_revnum : float, optional
///     The integer orbit revolution number associated with the first trajectory state.
/// orb_revnum_basis : str, optional
///     Specifies the message creator’s basis for their orbit revolution counter (0 or 1).
/// orb_averaging : str, optional
///     Specifies whether the orbit elements are osculating elements or mean elements.
/// traj_units : str, optional
///     Comma-delimited set of SI unit designations for the trajectory state elements.
/// comment : list[str], optional
///     Comments.
#[pyclass]
#[derive(Clone)]
pub struct OcmTrajState {
    pub inner: core_ocm::OcmTrajState,
}

#[pymethods]
impl OcmTrajState {
    /// Create a new OcmTrajState object.
    #[new]
    #[pyo3(signature = (
        *,
        center_name,
        traj_ref_frame,
        traj_type,
        traj_lines,
        traj_id=None,
        traj_prev_id=None,
        traj_next_id=None,
        traj_basis=None,
        traj_basis_id=None,
        interpolation=None,
        interpolation_degree=None,
        propagator=None,
        traj_frame_epoch=None,
        useable_start_time=None,
        useable_stop_time=None,
        orb_revnum=None,
        orb_revnum_basis=None,
        orb_averaging=None,
        traj_units=None,
        comment=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        center_name: String,
        traj_ref_frame: String,
        traj_type: String,
        traj_lines: Vec<TrajLine>,
        traj_id: Option<String>,
        traj_prev_id: Option<String>,
        traj_next_id: Option<String>,
        traj_basis: Option<String>,
        traj_basis_id: Option<String>,
        interpolation: Option<String>,
        interpolation_degree: Option<u32>,
        propagator: Option<String>,
        traj_frame_epoch: Option<String>,
        useable_start_time: Option<String>,
        useable_stop_time: Option<String>,
        orb_revnum: Option<f64>,
        orb_revnum_basis: Option<String>,
        orb_averaging: Option<String>,
        traj_units: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        // Note: traj_basis and orb_revnum_basis don't implement FromStr, so we ignore the input for now
        // The user can access these via inner if needed
        let _ = traj_basis; // Suppress unused warning
        let _ = orb_revnum_basis; // Suppress unused warning

        Ok(Self {
            inner: core_ocm::OcmTrajState {
                comment: comment.unwrap_or_default(),
                traj_id,
                traj_prev_id,
                traj_next_id,
                traj_basis: None, // TrajBasis enum doesn't implement FromStr
                traj_basis_id,
                interpolation,
                interpolation_degree,
                propagator,
                center_name,
                traj_ref_frame,
                traj_frame_epoch: traj_frame_epoch.map(|s| parse_epoch(&s)).transpose()?,
                useable_start_time: useable_start_time.map(|s| parse_epoch(&s)).transpose()?,
                useable_stop_time: useable_stop_time.map(|s| parse_epoch(&s)).transpose()?,
                orb_revnum,
                orb_revnum_basis: None, // RevNumBasis enum doesn't implement FromStr
                traj_type,
                orb_averaging,
                traj_units,
                traj_lines: traj_lines.into_iter().map(|t| t.inner).collect(),
            },
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "OcmTrajState(traj_type='{}', lines={})",
            self.inner.traj_type,
            self.inner.traj_lines.len()
        )
    }

    /// Origin of the orbit reference frame, which may be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter, or another reference frame center (such as a spacecraft, formation
    /// flying reference 'chief' spacecraft, etc.). Natural bodies shall be selected from the
    /// accepted set of values indicated in annex B, subsection B2. For spacecraft, it is
    /// recommended to use either the 'OBJECT_NAME' or 'INTERNATIONAL_DESIGNATOR' of the
    /// participant as catalogued in the UN Office of Outer Space Affairs designator index
    /// (reference `[3]`). Alternately, the 'OBJECT_DESIGNATOR' may be used. For other reference
    /// frame origins, this field is a free-text descriptor which may draw upon other naming
    /// conventions and sources.
    ///
    /// Examples: EARTH, MOON, ISS, EROS
    ///
    /// :type: str
    #[getter]
    fn get_center_name(&self) -> String {
        self.inner.center_name.clone()
    }
    #[setter]
    fn set_center_name(&mut self, value: String) {
        self.inner.center_name = value;
    }

    /// Reference frame of the trajectory state time history. Select from the accepted set of
    /// values indicated in annex B, subsection B4.
    ///
    /// Examples: ICRF3, J2000
    ///
    /// :type: str
    #[getter]
    fn get_traj_ref_frame(&self) -> String {
        self.inner.traj_ref_frame.clone()
    }
    #[setter]
    fn set_traj_ref_frame(&mut self, value: String) {
        self.inner.traj_ref_frame = value;
    }

    /// Specifies the trajectory state type; selected per annex B, subsection B7.
    ///
    /// Examples: CARTP, CARTPV
    ///
    /// :type: str
    #[getter]
    fn get_traj_type(&self) -> String {
        self.inner.traj_type.clone()
    }
    #[setter]
    fn set_traj_type(&mut self, value: String) {
        self.inner.traj_type = value;
    }

    /// Contiguous set of trajectory state data lines.
    ///
    /// :type: list[TrajLine]
    #[getter]
    fn get_traj_lines(&self) -> Vec<TrajLine> {
        self.inner
            .traj_lines
            .iter()
            .map(|t| TrajLine { inner: t.clone() })
            .collect()
    }
    #[setter]
    fn set_traj_lines(&mut self, value: Vec<TrajLine>) {
        self.inner.traj_lines = value.into_iter().map(|t| t.inner).collect();
    }

    /// Comments (a contiguous set of one or more comment lines may be provided in the
    /// Trajectory State Time History section only immediately after the TRAJ_START keyword;
    /// see 7.8 for comment formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    // === Trajectory ID Fields ===
    /// Free-text field containing the identification number for this trajectory state time
    /// history block.
    ///
    /// Examples: TRAJ_20160402_XYZ
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_traj_id(&self) -> Option<String> {
        self.inner.traj_id.clone()
    }
    #[setter]
    fn set_traj_id(&mut self, value: Option<String>) {
        self.inner.traj_id = value;
    }

    /// Free-text field containing the identification number for the previous trajectory state
    /// time history, contained either within this message or presented in a previous OCM.
    /// NOTE—If this message is not part of a sequence of orbit time histories or if this
    /// trajectory state time history is the first in a sequence of orbit time histories, then
    /// TRAJ_PREV_ID should be excluded from this message.
    ///
    /// Examples: ORB20160305A
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_traj_prev_id(&self) -> Option<String> {
        self.inner.traj_prev_id.clone()
    }
    #[setter]
    fn set_traj_prev_id(&mut self, value: Option<String>) {
        self.inner.traj_prev_id = value;
    }

    /// Free-text field containing the identification number for the next trajectory state
    /// time history, contained either within this message, or presented in a future OCM.
    /// NOTE—If this message is not part of a sequence of orbit time histories or if this
    /// trajectory state time history is the last in a sequence of orbit time histories, then
    /// TRAJ_NEXT_ID should be excluded from this message.
    ///
    /// Examples: ORB20160305C
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_traj_next_id(&self) -> Option<String> {
        self.inner.traj_next_id.clone()
    }
    #[setter]
    fn set_traj_next_id(&mut self, value: Option<String>) {
        self.inner.traj_next_id = value;
    }

    /// The basis of this trajectory state time history data. This is a free-text field with the
    /// following suggested values: a) 'PREDICTED'. b) 'DETERMINED' when estimated from
    /// observation-based orbit determination, reconstruction, and/or calibration. For
    /// definitive OD performed onboard spacecraft whose solutions have been telemetered to the
    /// ground for inclusion in an OCM, the TRAJ_BASIS shall be DETERMINED. c) 'TELEMETRY' when
    /// the trajectory states are read directly from telemetry, for example, based on inertial
    /// navigation systems or GNSS data. d) 'SIMULATED' for generic simulations, future mission
    /// design studies, and optimization studies. e) 'OTHER' for other bases of this data.
    ///
    /// Examples: PREDICTED
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_traj_basis(&self) -> Option<String> {
        self.inner.traj_basis.as_ref().map(|b| format!("{:?}", b))
    }
    #[setter]
    fn set_traj_basis(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.traj_basis = value
            .map(|s| s.parse())
            .transpose()
            .map_err(|e: ccsds_ndm::error::EnumParseError| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Free-text field containing the identification number for the telemetry dataset, orbit
    /// determination, navigation solution, or simulation upon which this trajectory state time
    /// history block is based. When a matching orbit determination block accompanies this
    /// trajectory state time history, the TRAJ_BASIS_ID should match the corresponding OD_ID
    /// (see table 6-11).
    ///
    /// Examples: OD_5910
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_traj_basis_id(&self) -> Option<String> {
        self.inner.traj_basis_id.clone()
    }
    #[setter]
    fn set_traj_basis_id(&mut self, value: Option<String>) {
        self.inner.traj_basis_id = value;
    }

    // === Interpolation/Propagation Fields ===
    /// This keyword may be used to specify the recommended interpolation method for ephemeris
    /// data in the immediately following set of ephemeris lines. PROPAGATE indicates that orbit
    /// propagation is the preferred method to obtain states at intermediate times, via either
    /// a midpoint-switching or endpoint switching approach.
    ///
    /// Examples: HERMITE, LINEAR, LAGRANGE, PROPAGATE
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_interpolation(&self) -> Option<String> {
        self.inner.interpolation.clone()
    }
    #[setter]
    fn set_interpolation(&mut self, value: Option<String>) {
        self.inner.interpolation = value;
    }

    /// Recommended interpolation degree for ephemeris data in the immediately following set of
    /// ephemeris lines. Must be an integer value. This keyword must be provided if the
    /// 'INTERPOLATION' keyword is used and set to anything other than PROPAGATE.
    ///
    /// Examples: 5, 1
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_interpolation_degree(&self) -> Option<u32> {
        self.inner.interpolation_degree
    }
    #[setter]
    fn set_interpolation_degree(&mut self, value: Option<u32>) {
        self.inner.interpolation_degree = value;
    }

    /// Free-text field containing the name of the orbit propagator used to create this
    /// trajectory state time history.
    ///
    /// Examples: HPOP, SP, SGP4
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_propagator(&self) -> Option<String> {
        self.inner.propagator.clone()
    }
    #[setter]
    fn set_propagator(&mut self, value: Option<String>) {
        self.inner.propagator = value;
    }

    // === Frame/Time Fields ===
    /// Epoch of the orbit data reference frame, if not intrinsic to the definition of the
    /// reference frame. (See 7.5.10 for formatting rules.)
    ///
    /// Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_traj_frame_epoch(&self) -> Option<String> {
        self.inner
            .traj_frame_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_traj_frame_epoch(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.traj_frame_epoch = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Start time of USEABLE time span covered by ephemeris data immediately following this
    /// metadata block. (For format specification, see 7.5.10.) NOTES 1. This optional keyword
    /// allows the message creator to introduce fictitious (but numerically smooth) data nodes
    /// following the actual data time history to support interpolation methods requiring more
    /// than two nodes (e.g., pure higher-order Lagrange interpolation methods). The use of this
    /// keyword and introduction of fictitious node points are optional and may not be necessary.
    /// 2. If this keyword is not supplied, then all data shall be assumed to be valid.
    ///
    /// Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_useable_start_time(&self) -> Option<String> {
        self.inner
            .useable_start_time
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_useable_start_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.useable_start_time = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Stop time of USEABLE time span covered by ephemeris data immediately following this
    /// metadata block. (For format specification, see 7.5.10.) NOTES 1. This optional keyword
    /// allows the message creator to introduce fictitious (but numerically smooth) data nodes
    /// following the actual data time history to support interpolation methods requiring more
    /// than two nodes (e.g., pure higher-order Lagrange interpolation methods). The use of this
    /// keyword and introduction of fictitious node points are optional and may not be necessary.
    /// 2. If this keyword is not supplied, then all data shall be assumed to be valid.
    ///
    /// Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_useable_stop_time(&self) -> Option<String> {
        self.inner
            .useable_stop_time
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_useable_stop_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.useable_stop_time = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    // === Orbit Revolution Fields ===
    /// The integer orbit revolution number associated with the first trajectory state in this
    /// trajectory state time history block. NOTE—The first ascending node crossing that occurs
    /// AFTER launch or deployment is designated to be the beginning of orbit revolution number
    /// = one ('1').
    ///
    /// Examples: 1500, 30007
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_orb_revnum(&self) -> Option<f64> {
        self.inner.orb_revnum
    }
    #[setter]
    fn set_orb_revnum(&mut self, value: Option<f64>) {
        self.inner.orb_revnum = value;
    }

    /// Specifies the message creator's basis for their orbit revolution counter, with '0',
    /// designating that the first launch or deployment trajectory state corresponds to a
    /// revolution number of 0.XXXX, where XXXX represents the fraction of an orbit revolution
    /// measured from the equatorial plane, and orbit revolution 1.0 begins at the very next
    /// (subsequent) ascending node passage; '1', designating that the first launch or
    /// deployment trajectory state corresponds to a revolution number of 1.XXXX, and orbit
    /// revolution 2.0 begins at the very next ascending node passage. This keyword shall be
    /// provided if ORB_REVNUM is specified.
    ///
    /// Examples: 0, 1
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_orb_revnum_basis(&self) -> Option<String> {
        self.inner
            .orb_revnum_basis
            .as_ref()
            .map(|b| format!("{:?}", b))
    }
    #[setter]
    fn set_orb_revnum_basis(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.orb_revnum_basis = value
            .map(|s| s.parse())
            .transpose()
            .map_err(|e: ccsds_ndm::error::EnumParseError| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }
    /// If orbital elements are provided, specifies whether those elements are osculating
    /// elements or mean elements, and if mean elements, which mean element definition is
    /// employed. The intent of this field is to allow the user to correctly interpret how to
    /// use the provided orbit elements and know how to use them operationally. This field is
    /// not required if one of the orbital element types selected by the "TRAJ_TYPE" keyword is
    /// Cartesian (e.g., CARTP, CARTPV, or CARTPVA) or spherical elements (e.g., LDBARV, ADBARV,
    /// or GEODETIC). Values should be selected from the accepted set indicated in annex B,
    /// subsection B13. If an alternate single- or double-averaging formulation other than that
    /// provided is used, the user may name it as mutually agreed upon by message exchange
    /// participants.
    ///
    /// Examples: OSCULATING, BROUWER, KOZAI
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_orb_averaging(&self) -> Option<String> {
        self.inner.orb_averaging.clone()
    }
    #[setter]
    fn set_orb_averaging(&mut self, value: Option<String>) {
        self.inner.orb_averaging = value;
    }

    /// A comma-delimited set of SI unit designations for each element of the trajectory state
    /// time history following the trajectory state time tag solely for informational purposes,
    /// provided as a free-text field enclosed in square brackets. When provided, each
    /// trajectory state element shall have a corresponding units entry, with non-dimensional
    /// values (such as orbit eccentricity) denoted by 'n/a'. NOTE—The listing of units via the
    /// TRAJ_UNITS keyword does not override the mandatory units specified for the selected
    /// TRAJ_TYPE (links to the relevant SANA registries provided in annex B, subsection B7).
    ///
    /// Examples: [km,km,km,km/s,km/s,km/s], [km,n/a,deg, deg, deg, deg]
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_traj_units(&self) -> Option<String> {
        self.inner.traj_units.clone()
    }
    #[setter]
    fn set_traj_units(&mut self, value: Option<String>) {
        self.inner.traj_units = value;
    }
}

/// A single line in a trajectory state time history.
///
/// Parameters
/// ----------
/// epoch : str
///     Absolute or relative time tag.
///     (Mandatory)
/// values : list of float
///     Trajectory state elements for this epoch.
///     (Mandatory)
#[pyclass]
#[derive(Clone)]
pub struct TrajLine {
    pub inner: core_ocm::TrajLine,
}

#[pymethods]
impl TrajLine {
    /// Create a new TrajLine object.
    #[new]
    #[pyo3(signature = (*, epoch, values))]
    fn new(epoch: String, values: Vec<f64>) -> Self {
        Self {
            inner: core_ocm::TrajLine { epoch, values },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "TrajLine(epoch='{}', values={})",
            self.inner.epoch,
            self.inner.values.len()
        )
    }

    /// Absolute or relative time tag.
    ///
    /// :type: str
    #[getter]
    fn get_epoch(&self) -> String {
        self.inner.epoch.clone()
    }
    #[setter]
    fn set_epoch(&mut self, value: String) {
        self.inner.epoch = value;
    }

    /// Trajectory state elements for this epoch.
    ///
    /// :type: list[float]
    #[getter]
    fn get_values(&self) -> Vec<f64> {
        self.inner.values.clone()
    }
    #[setter]
    fn set_values(&mut self, value: Vec<f64>) {
        self.inner.values = value;
    }
}

/// Space Object Physical Characteristics.
///
/// Parameters
/// ----------
/// manufacturer : str, optional
///     The manufacturer of the space object.
///     (Optional)
/// comment : list[str], optional
///     Comments.
///     (Optional)
#[pyclass]
#[derive(Clone)]
pub struct OcmPhysicalDescription {
    pub inner: core_ocm::OcmPhysicalDescription,
}

#[pymethods]
impl OcmPhysicalDescription {
    /// Create a new OcmPhysicalDescription object.
    #[new]
    #[pyo3(signature = (
        *,
        manufacturer=None,
        bus_model=None,
        docked_with=None,
        drag_const_area=None,
        drag_coeff_nom=None,
        drag_uncertainty=None,
        initial_wet_mass=None,
        wet_mass=None,
        dry_mass=None,
        oeb_parent_frame=None,
        oeb_parent_frame_epoch=None,
        oeb_q1=None,
        oeb_q2=None,
        oeb_q3=None,
        oeb_qc=None,
        oeb_max=None,
        oeb_int=None,
        oeb_min=None,
        area_along_oeb_max=None,
        area_along_oeb_int=None,
        area_along_oeb_min=None,
        area_min_for_pc=None,
        area_max_for_pc=None,
        area_typ_for_pc=None,
        rcs=None,
        rcs_min=None,
        rcs_max=None,
        srp_const_area=None,
        solar_rad_coeff=None,
        solar_rad_uncertainty=None,
        vm_absolute=None,
        vm_apparent_min=None,
        vm_apparent=None,
        vm_apparent_max=None,
        reflectance=None,
        att_control_mode=None,
        att_actuator_type=None,
        att_knowledge=None,
        att_control=None,
        att_pointing=None,
        avg_maneuver_freq=None,
        max_thrust=None,
        dv_bol=None,
        dv_remaining=None,
        ixx=None,
        iyy=None,
        izz=None,
        ixy=None,
        ixz=None,
        iyz=None,
        comment=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        manufacturer: Option<String>,
        bus_model: Option<String>,
        docked_with: Option<String>,
        drag_const_area: Option<f64>,
        drag_coeff_nom: Option<f64>,
        drag_uncertainty: Option<f64>,
        initial_wet_mass: Option<f64>,
        wet_mass: Option<f64>,
        dry_mass: Option<f64>,
        oeb_parent_frame: Option<String>,
        oeb_parent_frame_epoch: Option<String>,
        oeb_q1: Option<f64>,
        oeb_q2: Option<f64>,
        oeb_q3: Option<f64>,
        oeb_qc: Option<f64>,
        oeb_max: Option<f64>,
        oeb_int: Option<f64>,
        oeb_min: Option<f64>,
        area_along_oeb_max: Option<f64>,
        area_along_oeb_int: Option<f64>,
        area_along_oeb_min: Option<f64>,
        area_min_for_pc: Option<f64>,
        area_max_for_pc: Option<f64>,
        area_typ_for_pc: Option<f64>,
        rcs: Option<f64>,
        rcs_min: Option<f64>,
        rcs_max: Option<f64>,
        srp_const_area: Option<f64>,
        solar_rad_coeff: Option<f64>,
        solar_rad_uncertainty: Option<f64>,
        vm_absolute: Option<f64>,
        vm_apparent_min: Option<f64>,
        vm_apparent: Option<f64>,
        vm_apparent_max: Option<f64>,
        reflectance: Option<f64>,
        att_control_mode: Option<String>,
        att_actuator_type: Option<String>,
        att_knowledge: Option<f64>,
        att_control: Option<f64>,
        att_pointing: Option<f64>,
        avg_maneuver_freq: Option<f64>,
        max_thrust: Option<f64>,
        dv_bol: Option<f64>,
        dv_remaining: Option<f64>,
        ixx: Option<f64>,
        iyy: Option<f64>,
        izz: Option<f64>,
        ixy: Option<f64>,
        ixz: Option<f64>,
        iyz: Option<f64>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        use ccsds_ndm::types::{
            Angle, Area, Length, ManeuverFreq, Mass, Moment, Percentage, Probability, Thrust,
            Velocity,
        };

        Ok(Self {
            inner: core_ocm::OcmPhysicalDescription {
                comment: comment.unwrap_or_default(),
                manufacturer,
                bus_model,
                docked_with,
                drag_const_area: drag_const_area.map(|v| Area {
                    value: v,
                    units: None,
                }),
                drag_coeff_nom,
                drag_uncertainty: drag_uncertainty.map(|v| Percentage {
                    value: v,
                    units: None,
                }),
                initial_wet_mass: initial_wet_mass.map(|v| Mass {
                    value: v,
                    units: None,
                }),
                wet_mass: wet_mass.map(|v| Mass {
                    value: v,
                    units: None,
                }),
                dry_mass: dry_mass.map(|v| Mass {
                    value: v,
                    units: None,
                }),
                oeb_parent_frame,
                oeb_parent_frame_epoch: oeb_parent_frame_epoch
                    .map(|s| parse_epoch(&s))
                    .transpose()?,
                oeb_q1,
                oeb_q2,
                oeb_q3,
                oeb_qc,
                oeb_max: oeb_max.map(|v| Length {
                    value: v,
                    units: None,
                }),
                oeb_int: oeb_int.map(|v| Length {
                    value: v,
                    units: None,
                }),
                oeb_min: oeb_min.map(|v| Length {
                    value: v,
                    units: None,
                }),
                area_along_oeb_max: area_along_oeb_max.map(|v| Area {
                    value: v,
                    units: None,
                }),
                area_along_oeb_int: area_along_oeb_int.map(|v| Area {
                    value: v,
                    units: None,
                }),
                area_along_oeb_min: area_along_oeb_min.map(|v| Area {
                    value: v,
                    units: None,
                }),
                area_min_for_pc: area_min_for_pc.map(|v| Area {
                    value: v,
                    units: None,
                }),
                area_max_for_pc: area_max_for_pc.map(|v| Area {
                    value: v,
                    units: None,
                }),
                area_typ_for_pc: area_typ_for_pc.map(|v| Area {
                    value: v,
                    units: None,
                }),
                rcs: rcs.map(|v| Area {
                    value: v,
                    units: None,
                }),
                rcs_min: rcs_min.map(|v| Area {
                    value: v,
                    units: None,
                }),
                rcs_max: rcs_max.map(|v| Area {
                    value: v,
                    units: None,
                }),
                srp_const_area: srp_const_area.map(|v| Area {
                    value: v,
                    units: None,
                }),
                solar_rad_coeff,
                solar_rad_uncertainty: solar_rad_uncertainty.map(|v| Percentage {
                    value: v,
                    units: None,
                }),
                vm_absolute,
                vm_apparent_min,
                vm_apparent,
                vm_apparent_max,
                reflectance: reflectance.map(|v| Probability { value: v }),
                att_control_mode,
                att_actuator_type,
                att_knowledge: att_knowledge.map(|v| Angle {
                    value: v,
                    units: None,
                }),
                att_control: att_control.map(|v| Angle {
                    value: v,
                    units: None,
                }),
                att_pointing: att_pointing.map(|v| Angle {
                    value: v,
                    units: None,
                }),
                avg_maneuver_freq: avg_maneuver_freq.map(|v| ManeuverFreq {
                    value: v,
                    units: None,
                }),
                max_thrust: max_thrust.map(|v| Thrust {
                    value: v,
                    units: None,
                }),
                dv_bol: dv_bol.map(|v| Velocity {
                    value: v,
                    units: None,
                }),
                dv_remaining: dv_remaining.map(|v| Velocity {
                    value: v,
                    units: None,
                }),
                ixx: ixx.map(|v| Moment {
                    value: v,
                    units: None,
                }),
                iyy: iyy.map(|v| Moment {
                    value: v,
                    units: None,
                }),
                izz: izz.map(|v| Moment {
                    value: v,
                    units: None,
                }),
                ixy: ixy.map(|v| Moment {
                    value: v,
                    units: None,
                }),
                ixz: ixz.map(|v| Moment {
                    value: v,
                    units: None,
                }),
                iyz: iyz.map(|v| Moment {
                    value: v,
                    units: None,
                }),
            },
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "OcmPhysicalDescription(manufacturer={:?})",
            self.inner.manufacturer
        )
    }

    /// Free-text field containing the satellite manufacturer's name.
    ///
    /// Examples: BOEING
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_manufacturer(&self) -> Option<String> {
        self.inner.manufacturer.clone()
    }
    #[setter]
    fn set_manufacturer(&mut self, value: Option<String>) {
        self.inner.manufacturer = value;
    }

    /// Comments (a contiguous set of one or more comment lines may be provided in the OCM Space
    /// Object Physical Characteristics only immediately after the PHYS_START keyword; see 7.8
    /// for comment formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    // === Bus Information ===
    /// Free-text field containing the satellite manufacturer's spacecraft bus model name.
    ///
    /// Examples: 702
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_bus_model(&self) -> Option<String> {
        self.inner.bus_model.clone()
    }
    #[setter]
    fn set_bus_model(&mut self, value: Option<String>) {
        self.inner.bus_model = value;
    }

    /// Free-text field containing a comma-separated list of other space objects that this
    /// object is docked to.
    ///
    /// Examples: ISS
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_docked_with(&self) -> Option<String> {
        self.inner.docked_with.clone()
    }
    #[setter]
    fn set_docked_with(&mut self, value: Option<String>) {
        self.inner.docked_with = value;
    }

    // === Drag Properties (Area in m**2) ===
    /// Attitude-independent drag cross-sectional area (AD) facing the relative wind vector,
    /// not already incorporated into the attitude-dependent 'AREA_ALONG_OEB' parameters.
    ///
    /// Examples: 2.5
    ///
    /// Units: m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_drag_const_area(&self) -> Option<f64> {
        self.inner.drag_const_area.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_drag_const_area(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Area;
        self.inner.drag_const_area = value.map(|v| Area {
            value: v,
            units: None,
        });
    }

    /// Nominal drag Coefficient (CD Nom). If the atmospheric drag coefficient, CD, is set to
    /// zero, no atmospheric drag shall be considered.
    ///
    /// Examples: 2.2
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_drag_coeff_nom(&self) -> Option<f64> {
        self.inner.drag_coeff_nom
    }
    #[setter]
    fn set_drag_coeff_nom(&mut self, value: Option<f64>) {
        self.inner.drag_coeff_nom = value;
    }

    /// Drag coefficient one sigma (1σ) percent uncertainty, where the actual range of drag
    /// coefficients to within 1σ shall be obtained from [1.0 ± DRAG_UNCERTAINTY/100.0] * (CD
    /// Nom). This factor is intended to allow operators to supply the nominal ballistic
    /// coefficient components while accommodating ballistic coefficient uncertainties.
    ///
    /// Examples: 10.0
    ///
    /// Units: %
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_drag_uncertainty(&self) -> Option<f64> {
        self.inner.drag_uncertainty.as_ref().map(|p| p.value)
    }
    #[setter]
    fn set_drag_uncertainty(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Percentage;
        self.inner.drag_uncertainty = value.map(|v| Percentage {
            value: v,
            units: None,
        });
    }

    // === SRP Properties ===
    /// Attitude-independent solar radiation pressure cross-sectional area (AR) facing the Sun,
    /// not already incorporated into the attitude-dependent ‘AREA_ALONG_OEB’ parameters.
    ///
    /// Units: m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_srp_const_area(&self) -> Option<f64> {
        self.inner.srp_const_area.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_srp_const_area(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Area;
        self.inner.srp_const_area = value.map(|v| Area {
            value: v,
            units: None,
        });
    }

    /// Nominal Solar Radiation Pressure Coefficient (CR NOM). If the solar radiation
    /// coefficient, CR, is set to zero, no solar radiation pressure shall be considered.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_solar_rad_coeff(&self) -> Option<f64> {
        self.inner.solar_rad_coeff
    }
    #[setter]
    fn set_solar_rad_coeff(&mut self, value: Option<f64>) {
        self.inner.solar_rad_coeff = value;
    }

    /// SRP one sigma (1σ) percent uncertainty, where the actual range of SRP coefficients to
    /// within 1σ shall be obtained from [1.0 ± 0.01*SRP_UNCERTAINTY] (CR NOM). This factor is
    /// intended to allow operators to supply the nominal ballistic coefficient components
    /// while accommodating ballistic coefficient uncertainties.
    ///
    /// Units: %
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_solar_rad_uncertainty(&self) -> Option<f64> {
        self.inner.solar_rad_uncertainty.as_ref().map(|p| p.value)
    }
    #[setter]
    fn set_solar_rad_uncertainty(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Percentage;
        self.inner.solar_rad_uncertainty = value.map(|v| Percentage {
            value: v,
            units: None,
        });
    }

    // === Mass Properties (kg) ===
    /// Space object total mass at beginning of life.
    ///
    /// Examples: 500
    ///
    /// Units: kg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_initial_wet_mass(&self) -> Option<f64> {
        self.inner.initial_wet_mass.as_ref().map(|m| m.value)
    }
    #[setter]
    fn set_initial_wet_mass(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Mass;
        self.inner.initial_wet_mass = value.map(|v| Mass {
            value: v,
            units: None,
        });
    }
    /// Space object total mass (including propellant, i.e., 'wet mass') at the current
    /// reference epoch 'EPOCH_TZERO'.
    ///
    /// Examples: 472.3
    ///
    /// Units: kg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_wet_mass(&self) -> Option<f64> {
        self.inner.wet_mass.as_ref().map(|m| m.value)
    }
    #[setter]
    fn set_wet_mass(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Mass;
        self.inner.wet_mass = value.map(|v| Mass {
            value: v,
            units: None,
        });
    }
    /// Space object dry mass (without propellant).
    ///
    /// Examples: 300
    ///
    /// Units: kg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_dry_mass(&self) -> Option<f64> {
        self.inner.dry_mass.as_ref().map(|m| m.value)
    }
    #[setter]
    fn set_dry_mass(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Mass;
        self.inner.dry_mass = value.map(|v| Mass {
            value: v,
            units: None,
        });
    }

    // === OEB (Optimally Enclosing Box) Fields ===
    /// Parent reference frame that maps to the OEB frame via the quaternion-based
    /// transformation defined in annex F, subsection F1. Select from the accepted set of
    /// values indicated in B, subsections B4 and B5. This keyword shall be provided if
    /// OEB_Q1,2,3,4 are specified.
    ///
    /// Examples: ITRF1997
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_oeb_parent_frame(&self) -> Option<String> {
        self.inner.oeb_parent_frame.clone()
    }
    #[setter]
    fn set_oeb_parent_frame(&mut self, value: Option<String>) {
        self.inner.oeb_parent_frame = value;
    }
    /// Epoch of the OEB parent frame, if OEB_PARENT_FRAME is provided and its epoch is not
    /// intrinsic to the definition of the reference frame. (See 7.5.10 for formatting rules.)
    ///
    /// Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_oeb_parent_frame_epoch(&self) -> Option<String> {
        self.inner
            .oeb_parent_frame_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_oeb_parent_frame_epoch(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.oeb_parent_frame_epoch = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }
    /// q1 = e1 * sin(φ/2), where per reference `[H1]`, φ = Euler rotation angle and e1 = 1st
    /// component of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME
    /// (defined above) to the frame aligned with the OEB (defined in annex F, subsection F1).
    /// A value of '-999' denotes a tumbling space object.
    ///
    /// Examples: -0.575131822
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_oeb_q1(&self) -> Option<f64> {
        self.inner.oeb_q1
    }
    #[setter]
    fn set_oeb_q1(&mut self, value: Option<f64>) {
        self.inner.oeb_q1 = value;
    }
    /// q2 = e2 * sin(φ/2), where per reference `[H1]`, φ = Euler rotation angle and e2 = 2nd
    /// component of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME
    /// (defined above) to the frame aligned with the Optimally Encompassing Box (defined in
    /// annex F, subsection F1). A value of '-999' denotes a tumbling space object.
    ///
    /// Examples: -0.280510532
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_oeb_q2(&self) -> Option<f64> {
        self.inner.oeb_q2
    }
    #[setter]
    fn set_oeb_q2(&mut self, value: Option<f64>) {
        self.inner.oeb_q2 = value;
    }
    /// q3 = e3 * sin(φ/2), where per reference `[H1]`, φ = Euler rotation angle and e3 = 3rd
    /// component of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME
    /// (defined above) to the frame aligned with the Optimally Encompassing Box (defined in
    /// annex F, subsection F1). A value of '-999' denotes a tumbling space object.
    ///
    /// Examples: -0.195634856
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_oeb_q3(&self) -> Option<f64> {
        self.inner.oeb_q3
    }
    #[setter]
    fn set_oeb_q3(&mut self, value: Option<f64>) {
        self.inner.oeb_q3 = value;
    }
    /// qc = cos(φ/2), where per reference `[H1]`, φ = the Euler rotation angle for the rotation
    /// that maps from the OEB_PARENT_FRAME (defined above) to the frame aligned with the
    /// Optimally Encompassing Box (annex F, subsection F1). qc shall be made non-negative by
    /// convention. A value of '-999' denotes a tumbling space object.
    ///
    /// Examples: 0.743144825
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_oeb_qc(&self) -> Option<f64> {
        self.inner.oeb_qc
    }
    #[setter]
    fn set_oeb_qc(&mut self, value: Option<f64>) {
        self.inner.oeb_qc = value;
    }
    /// Maximum physical dimension (along Xoeb) of the OEB.
    ///
    /// Examples: 1
    ///
    /// Units: m
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_oeb_max(&self) -> Option<f64> {
        self.inner.oeb_max.as_ref().map(|l| l.value)
    }
    #[setter]
    fn set_oeb_max(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Length;
        self.inner.oeb_max = value.map(|v| Length {
            value: v,
            units: None,
        });
    }
    /// Intermediate physical dimension (along Ŷoeb) of OEB normal to OEB_MAX direction.
    ///
    /// Examples: 0.5
    ///
    /// Units: m
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_oeb_int(&self) -> Option<f64> {
        self.inner.oeb_int.as_ref().map(|l| l.value)
    }
    #[setter]
    fn set_oeb_int(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Length;
        self.inner.oeb_int = value.map(|v| Length {
            value: v,
            units: None,
        });
    }
    /// Minimum physical dimension (along Ẑoeb) of OEB in direction normal to both OEB_MAX and
    /// OEB_INT directions.
    ///
    /// Examples: 0.3
    ///
    /// Units: m
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_oeb_min(&self) -> Option<f64> {
        self.inner.oeb_min.as_ref().map(|l| l.value)
    }
    #[setter]
    fn set_oeb_min(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Length;
        self.inner.oeb_min = value.map(|v| Length {
            value: v,
            units: None,
        });
    }
    /// Attitude-dependent cross-sectional area of space object (not already included in
    /// DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along max OEB (Xoeb) direction as
    /// defined in annex F.
    ///
    /// Examples: 0.15
    ///
    /// Units: m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_area_along_oeb_max(&self) -> Option<f64> {
        self.inner.area_along_oeb_max.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_area_along_oeb_max(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Area;
        self.inner.area_along_oeb_max = value.map(|v| Area {
            value: v,
            units: None,
        });
    }
    /// Attitude-dependent cross-sectional area of space object (not already included in
    /// DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along intermediate OEB (Ŷoeb) direction
    /// as defined in annex F.
    ///
    /// Examples: 0.3
    ///
    /// Units: m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_area_along_oeb_int(&self) -> Option<f64> {
        self.inner.area_along_oeb_int.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_area_along_oeb_int(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Area;
        self.inner.area_along_oeb_int = value.map(|v| Area {
            value: v,
            units: None,
        });
    }
    /// Attitude-dependent cross-sectional area of space object (not already included in
    /// DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along minimum OEB (Ẑoeb) direction as
    /// defined in annex F.
    ///
    /// Examples: 0.5
    ///
    /// Units: m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_area_along_oeb_min(&self) -> Option<f64> {
        self.inner.area_along_oeb_min.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_area_along_oeb_min(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Area;
        self.inner.area_along_oeb_min = value.map(|v| Area {
            value: v,
            units: None,
        });
    }

    // === Collision Properties ===
    /// Minimum cross-sectional area for collision probability estimation purposes.
    ///
    /// Examples: 1.0
    ///
    /// Units: m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_area_min_for_pc(&self) -> Option<f64> {
        self.inner.area_min_for_pc.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_area_min_for_pc(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Area;
        self.inner.area_min_for_pc = value.map(|v| Area {
            value: v,
            units: None,
        });
    }
    /// Maximum cross-sectional area for collision probability estimation purposes.
    ///
    /// Examples: 1.0
    ///
    /// Units: m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_area_max_for_pc(&self) -> Option<f64> {
        self.inner.area_max_for_pc.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_area_max_for_pc(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Area;
        self.inner.area_max_for_pc = value.map(|v| Area {
            value: v,
            units: None,
        });
    }
    /// Typical (50th percentile) cross-sectional area sampled over all space object
    /// orientations for collision probability estimation purposes.
    ///
    /// Units: m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_area_typ_for_pc(&self) -> Option<f64> {
        self.inner.area_typ_for_pc.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_area_typ_for_pc(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Area;
        self.inner.area_typ_for_pc = value.map(|v| Area {
            value: v,
            units: None,
        });
    }

    /// Typical (50th percentile) effective Radar Cross Section of the space object sampled
    /// over all possible viewing angles.
    ///
    /// Units: m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_rcs(&self) -> Option<f64> {
        self.inner.rcs.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_rcs(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Area;
        self.inner.rcs = value.map(|v| Area {
            value: v,
            units: None,
        });
    }
    /// Minimum Radar Cross Section observed for this object.
    ///
    /// Units: m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_rcs_min(&self) -> Option<f64> {
        self.inner.rcs_min.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_rcs_min(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Area;
        self.inner.rcs_min = value.map(|v| Area {
            value: v,
            units: None,
        });
    }
    /// Maximum Radar Cross Section observed for this object.
    ///
    /// Units: m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_rcs_max(&self) -> Option<f64> {
        self.inner.rcs_max.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_rcs_max(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Area;
        self.inner.rcs_max = value.map(|v| Area {
            value: v,
            units: None,
        });
    }

    // === Visual Magnitude ===
    /// Typical (50th percentile) Visual Magnitude of the space object sampled over all
    /// possible viewing angles and sampled over all possible viewing angles and ‘normalized’
    /// as specified in informative annex F, subsection F2 to a 1 AU Sun-to-target distance,
    /// a phase angle of 0°, and a 40,000 km target-to-sensor distance (equivalent of GEO
    /// satellite tracked at 15.6° above local horizon).
    ///
    /// Examples: 15.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_vm_absolute(&self) -> Option<f64> {
        self.inner.vm_absolute
    }
    #[setter]
    fn set_vm_absolute(&mut self, value: Option<f64>) {
        self.inner.vm_absolute = value;
    }

    /// Typical (50th percentile) apparent Visual Magnitude observed for this space object.
    ///
    /// Examples: 15.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_vm_apparent(&self) -> Option<f64> {
        self.inner.vm_apparent
    }
    #[setter]
    fn set_vm_apparent(&mut self, value: Option<f64>) {
        self.inner.vm_apparent = value;
    }

    /// Minimum apparent Visual Magnitude observed for this space object.
    ///
    /// Examples: 19.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_vm_apparent_min(&self) -> Option<f64> {
        self.inner.vm_apparent_min
    }
    #[setter]
    fn set_vm_apparent_min(&mut self, value: Option<f64>) {
        self.inner.vm_apparent_min = value;
    }

    /// Maximum apparent Visual Magnitude observed for this space object. NOTE—The 'MAX' value
    /// represents the brightest observation, which associates with a lower Vmag.
    ///
    /// Examples: 16.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_vm_apparent_max(&self) -> Option<f64> {
        self.inner.vm_apparent_max
    }
    #[setter]
    fn set_vm_apparent_max(&mut self, value: Option<f64>) {
        self.inner.vm_apparent_max = value;
    }

    /// Typical (50th percentile) coefficient of REFLECTANCE of the space object over all
    /// possible viewing angles, ranging from 0 (none) to 1 (perfect reflectance).
    ///
    /// Examples: 0.7
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_reflectance(&self) -> Option<f64> {
        self.inner.reflectance.as_ref().map(|p| p.value)
    }
    #[setter]
    fn set_reflectance(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Probability;
        self.inner.reflectance = value.map(|v| Probability { value: v });
    }

    // === Attitude Control ===
    /// Free-text specification of primary mode of attitude control for the space object.
    /// Suggested examples include: THREE_AXIS, SPIN, DUAL_SPIN, TUMBLING, GRAVITY_GRADIENT
    ///
    /// Examples: SPIN
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_att_control_mode(&self) -> Option<String> {
        self.inner.att_control_mode.clone()
    }
    #[setter]
    fn set_att_control_mode(&mut self, value: Option<String>) {
        self.inner.att_control_mode = value;
    }

    /// Free-text specification of type of actuator for attitude control. Suggested examples
    /// include: ATT_THRUSTERS, ACTIVE_MAG_TORQUE, PASSIVE_MAG_TORQUE, REACTION_WHEELS,
    /// MOMENTUM_WHEELS, CONTROL_MOMENT_GYROSCOPE, NONE, OTHER
    ///
    /// Examples: ATT_THRUSTERS
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_att_actuator_type(&self) -> Option<String> {
        self.inner.att_actuator_type.clone()
    }
    #[setter]
    fn set_att_actuator_type(&mut self, value: Option<String>) {
        self.inner.att_actuator_type = value;
    }

    /// Accuracy of attitude knowledge.
    ///
    /// Examples: 0.3
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_att_knowledge(&self) -> Option<f64> {
        self.inner.att_knowledge.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_att_knowledge(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Angle;
        self.inner.att_knowledge = value.map(|v| Angle {
            value: v,
            units: None,
        });
    }

    /// Accuracy of attitude control system (ACS) to maintain attitude, assuming attitude
    /// knowledge was perfect (i.e., deadbands).
    ///
    /// Examples: 2.0
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_att_control(&self) -> Option<f64> {
        self.inner.att_control.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_att_control(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Angle;
        self.inner.att_control = value.map(|v| Angle {
            value: v,
            units: None,
        });
    }

    /// Overall accuracy of spacecraft to maintain attitude, including attitude knowledge
    /// errors and ACS operation.
    ///
    /// Examples: 2.3
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_att_pointing(&self) -> Option<f64> {
        self.inner.att_pointing.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_att_pointing(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Angle;
        self.inner.att_pointing = value.map(|v| Angle {
            value: v,
            units: None,
        });
    }

    // === Maneuver Capabilities ===
    /// Average maneuver frequency, measured in the number of orbit- or attitude-adjust
    /// maneuvers per year.
    ///
    /// Examples: 20.0
    ///
    /// Units: #/yr
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_avg_maneuver_freq(&self) -> Option<f64> {
        self.inner.avg_maneuver_freq.as_ref().map(|f| f.value)
    }
    #[setter]
    fn set_avg_maneuver_freq(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::ManeuverFreq;
        self.inner.avg_maneuver_freq = value.map(|v| ManeuverFreq {
            value: v,
            units: None,
        });
    }
    /// Maximum composite thrust the spacecraft can accomplish in any single body-fixed
    /// direction.
    ///
    /// Examples: 1.0
    ///
    /// Units: N
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_max_thrust(&self) -> Option<f64> {
        self.inner.max_thrust.as_ref().map(|t| t.value)
    }
    #[setter]
    fn set_max_thrust(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Thrust;
        self.inner.max_thrust = value.map(|v| Thrust {
            value: v,
            units: None,
        });
    }
    /// Total ΔV capability of the spacecraft at beginning of life.
    ///
    /// Examples: 1.0
    ///
    /// Units: km/s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_dv_bol(&self) -> Option<f64> {
        self.inner.dv_bol.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_dv_bol(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Velocity;
        self.inner.dv_bol = value.map(|v| Velocity {
            value: v,
            units: None,
        });
    }
    /// Total ΔV remaining for the spacecraft.
    ///
    /// Examples: 0.2
    ///
    /// Units: km/s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_dv_remaining(&self) -> Option<f64> {
        self.inner.dv_remaining.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_dv_remaining(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Velocity;
        self.inner.dv_remaining = value.map(|v| Velocity {
            value: v,
            units: None,
        });
    }

    // === Moments of Inertia ===
    /// Moment of Inertia about the X-axis of the space object's primary body frame (e.g.,
    /// SC_Body_1) (see reference `[H1]`).
    ///
    /// Examples: 1000.0
    ///
    /// Units: kg·m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_ixx(&self) -> Option<f64> {
        self.inner.ixx.as_ref().map(|m| m.value)
    }
    #[setter]
    fn set_ixx(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Moment;
        self.inner.ixx = value.map(|v| Moment {
            value: v,
            units: None,
        });
    }

    /// Moment of Inertia about the Y-axis.
    ///
    /// Examples: 800.0
    ///
    /// Units: kg·m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_iyy(&self) -> Option<f64> {
        self.inner.iyy.as_ref().map(|m| m.value)
    }
    #[setter]
    fn set_iyy(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Moment;
        self.inner.iyy = value.map(|v| Moment {
            value: v,
            units: None,
        });
    }

    /// Moment of Inertia about the Z-axis.
    ///
    /// Examples: 400.0
    ///
    /// Units: kg·m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_izz(&self) -> Option<f64> {
        self.inner.izz.as_ref().map(|m| m.value)
    }
    #[setter]
    fn set_izz(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Moment;
        self.inner.izz = value.map(|v| Moment {
            value: v,
            units: None,
        });
    }

    /// Inertia Cross Product of the X & Y axes.
    ///
    /// Examples: 20.0
    ///
    /// Units: kg·m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_ixy(&self) -> Option<f64> {
        self.inner.ixy.as_ref().map(|m| m.value)
    }
    #[setter]
    fn set_ixy(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Moment;
        self.inner.ixy = value.map(|v| Moment {
            value: v,
            units: None,
        });
    }

    /// Inertia Cross Product of the X & Z axes.
    ///
    /// Examples: 40.0
    ///
    /// Units: kg·m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_ixz(&self) -> Option<f64> {
        self.inner.ixz.as_ref().map(|m| m.value)
    }
    #[setter]
    fn set_ixz(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Moment;
        self.inner.ixz = value.map(|v| Moment {
            value: v,
            units: None,
        });
    }

    /// Inertia Cross Product of the Y & Z axes.
    ///
    /// Examples: 60.0
    ///
    /// Units: kg·m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_iyz(&self) -> Option<f64> {
        self.inner.iyz.as_ref().map(|m| m.value)
    }
    #[setter]
    fn set_iyz(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Moment;
        self.inner.iyz = value.map(|v| Moment {
            value: v,
            units: None,
        });
    }
}

// ============================================================================
// OcmCovarianceMatrix - Covariance Time History
// ============================================================================

/// OCM Covariance Matrix.
///
/// Parameters
/// ----------
///     epoch : str
///     Epoch of the covariance matrix.
///     (Mandatory)
/// cov_ref_frame : str
///     Reference frame for the covariance matrix.
///     (Mandatory)
/// cov_type : str
///     Specifies the covariance element set type.
///     (Mandatory)
///     cov_matrix : list of float
///     Upper triangular part of the covariance matrix.
///     (Mandatory)
/// cov_id : str, optional
///     Identification number for this covariance matrix time history block.
///     (Optional)
/// cov_prev_id : str, optional
///     Identification number for the previous covariance matrix time history.
///     (Optional)
/// cov_next_id : str, optional
///     Identification number for the next covariance matrix time history.
///     (Optional)
/// cov_basis : str, optional
///     Basis of this covariance matrix time history data (PREDICTED, DETERMINED, etc.).
///     (Optional)
/// cov_basis_id : str, optional
///     Identification number for the telemetry dataset, orbit determination, or simulation.
///     (Optional)
/// cov_confidence : float, optional
///     The confidence level associated with the covariance [0-100].
///     (Optional)
///     cov_scale_factor : float, optional
///     Scale factor to be applied to the covariance matrix.
///     (Optional)
/// cov_units : str, optional
///     Comma-delimited set of SI unit designations for the covariance elements.
///     (Optional)
/// comment : list[str], optional
///     Comments.
///     (Optional)
#[pyclass]
#[derive(Clone)]
pub struct OcmCovarianceMatrix {
    pub inner: core_ocm::OcmCovarianceMatrix,
}

#[pymethods]
impl OcmCovarianceMatrix {
    #[new]
    #[pyo3(signature = (*, cov_ref_frame, cov_type, cov_ordering, cov_lines, cov_id=None, cov_prev_id=None, cov_next_id=None, cov_basis=None, cov_basis_id=None, cov_frame_epoch=None, cov_scale_min=None, cov_scale_max=None, cov_confidence=None, cov_units=None, comment=None))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        cov_ref_frame: String,
        cov_type: String,
        cov_ordering: String,
        cov_lines: Vec<CovLine>,
        cov_id: Option<String>,
        cov_prev_id: Option<String>,
        cov_next_id: Option<String>,
        cov_basis: Option<String>,
        cov_basis_id: Option<String>,
        cov_frame_epoch: Option<String>,
        cov_scale_min: Option<f64>,
        cov_scale_max: Option<f64>,
        cov_confidence: Option<f64>,
        cov_units: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        use ccsds_ndm::types::Percentage;
        Ok(Self {
            inner: core_ocm::OcmCovarianceMatrix {
                comment: comment.unwrap_or_default(),
                cov_id,
                cov_prev_id,
                cov_next_id,
                cov_basis: cov_basis.map(|s| s.parse()).transpose().map_err(
                    |e: ccsds_ndm::error::EnumParseError| PyValueError::new_err(e.to_string()),
                )?,
                cov_basis_id,
                cov_ref_frame,
                cov_frame_epoch: cov_frame_epoch.map(|s| parse_epoch(&s)).transpose()?,
                cov_scale_min,
                cov_scale_max,
                cov_confidence: cov_confidence.map(|v| Percentage {
                    value: v,
                    units: None,
                }),
                cov_type,
                cov_ordering: cov_ordering.parse().map_err(
                    |e: ccsds_ndm::error::EnumParseError| PyValueError::new_err(e.to_string()),
                )?,
                cov_units,
                cov_lines: cov_lines.into_iter().map(|c| c.inner).collect(),
            },
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "OcmCovarianceMatrix(cov_type='{}', lines={})",
            self.inner.cov_type,
            self.inner.cov_lines.len()
        )
    }

    /// Free-text field containing the identification number for this covariance time history
    /// block.
    ///
    /// Examples: COV_20160402_XYZ
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_cov_id(&self) -> Option<String> {
        self.inner.cov_id.clone()
    }
    #[setter]
    fn set_cov_id(&mut self, value: Option<String>) {
        self.inner.cov_id = value;
    }

    /// Free-text field containing the identification number for the previous covariance time
    /// history, contained either within this message or presented in a previous OCM. NOTE—If
    /// this message is not part of a sequence of covariance time histories or if this
    /// covariance time history is the first in a sequence of covariance time histories, then
    /// COV_PREV_ID should be excluded from this message.
    ///
    /// Examples: COV_20160305a
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_cov_prev_id(&self) -> Option<String> {
        self.inner.cov_prev_id.clone()
    }
    #[setter]
    fn set_cov_prev_id(&mut self, value: Option<String>) {
        self.inner.cov_prev_id = value;
    }

    /// Free-text field containing the identification number for the next covariance time
    /// history, contained either within this message, or presented in a future OCM. NOTE—If
    /// this message is not part of a sequence of covariance time histories or if this
    /// covariance time history is the last in a sequence of covariance time histories, then
    /// COV_NEXT_ID should be excluded from this message.
    ///
    /// Examples: COV_20160305C
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_cov_next_id(&self) -> Option<String> {
        self.inner.cov_next_id.clone()
    }
    #[setter]
    fn set_cov_next_id(&mut self, value: Option<String>) {
        self.inner.cov_next_id = value;
    }
    /// Basis of this covariance time history data. This is free-text field with the following
    /// suggested values: a) 'PREDICTED'. b) 'DETERMINED' when estimated from observation-based
    /// orbit determination, reconstruction and/or calibration. For definitive OD performed
    /// onboard whose solutions have been telemetered to the ground for inclusion in an OCM,
    /// the COV_BASIS shall be considered to be DETERMINED. c) EMPIRICAL (for empirically
    /// determined such as overlap analyses). d) SIMULATED for simulation-based (including
    /// Monte Carlo) estimations, future mission design studies, and optimization studies. e)
    /// 'OTHER' for other bases of this data.
    ///
    /// Examples: PREDICTED, EMPIRICAL, DETERMINED, SIMULATED, OTHER
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_cov_basis(&self) -> Option<String> {
        self.inner.cov_basis.as_ref().map(|b| format!("{:?}", b))
    }
    /// Free-text field containing the identification number for the orbit determination,
    /// navigation solution, or simulation upon which this covariance time history block is
    /// based. When a matching orbit determination block accompanies this covariance time
    /// history, the COV_BASIS_ID should match the corresponding OD_ID (see table 6-11).
    ///
    /// Examples: OD_5910
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_cov_basis_id(&self) -> Option<String> {
        self.inner.cov_basis_id.clone()
    }
    #[setter]
    fn set_cov_basis_id(&mut self, value: Option<String>) {
        self.inner.cov_basis_id = value;
    }
    /// Reference frame of the covariance time history. Select from the accepted set of values
    /// indicated in annex B, subsection B4 and B5.
    ///
    /// Examples: TNW_INERTIA, J2000
    ///
    /// :type: str
    #[getter]
    fn get_cov_ref_frame(&self) -> String {
        self.inner.cov_ref_frame.clone()
    }
    #[setter]
    fn set_cov_ref_frame(&mut self, value: String) {
        self.inner.cov_ref_frame = value;
    }

    /// Epoch of the covariance data reference frame, if not intrinsic to the definition of the
    /// reference frame. (See 7.5.10 for formatting rules.)
    ///
    /// Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_cov_frame_epoch(&self) -> Option<String> {
        self.inner
            .cov_frame_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_cov_frame_epoch(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.cov_frame_epoch = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Minimum scale factor to apply to this covariance data to achieve realism.
    ///
    /// Examples: 0.5
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_cov_scale_min(&self) -> Option<f64> {
        self.inner.cov_scale_min
    }
    #[setter]
    fn set_cov_scale_min(&mut self, value: Option<f64>) {
        self.inner.cov_scale_min = value;
    }

    /// Maximum scale factor to apply to this covariance data to achieve realism.
    ///
    /// Examples: 5.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_cov_scale_max(&self) -> Option<f64> {
        self.inner.cov_scale_max
    }
    #[setter]
    fn set_cov_scale_max(&mut self, value: Option<f64>) {
        self.inner.cov_scale_max = value;
    }

    /// A measure of the confidence in the covariance errors matching reality, as characterized
    /// via a Wald test, a Chi-squared test, the log of likelihood, or a numerical
    /// representation per mutual agreement.
    ///
    /// Examples: 50
    ///
    /// Units: %
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_cov_confidence(&self) -> Option<f64> {
        self.inner.cov_confidence.as_ref().map(|p| p.value)
    }
    /// Indicates covariance composition. Select from annex B, subsections B7 and B8.
    ///
    /// Examples: CARTP, CARTPV, ADBARV
    ///
    /// :type: str
    #[getter]
    fn get_cov_type(&self) -> String {
        self.inner.cov_type.clone()
    }
    #[setter]
    fn set_cov_type(&mut self, value: String) {
        self.inner.cov_type = value;
    }

    /// Indicates covariance ordering as being either LTM, UTM, Full covariance, LTM covariance
    /// with cross-correlation information provided in upper triangle off-diagonal terms
    /// (LTMWCC), or UTM covariance with cross-correlation information provided in lower
    /// triangle off-diagonal terms (UTMWCC).
    ///
    /// Examples: LTM, UTM, FULL, LTMWCC, UTMWCC
    ///
    /// :type: str
    #[getter]
    fn get_cov_ordering(&self) -> String {
        format!("{:?}", self.inner.cov_ordering)
    }

    /// A comma-delimited set of SI unit designations for each element of the covariance time
    /// history following the covariance time tag, solely for informational purposes, provided
    /// as a free-text field enclosed in square brackets. When provided, these units
    /// designations shall correspond to the units of the standard deviations (or square roots)
    /// of each of the covariance matrix diagonal elements (or variances), respectively, and
    /// all diagonal elements shall have a corresponding units entry, with non-dimensional
    /// values (such as dispersion in orbit eccentricity) denoted by 'n/a'. NOTE—The listing of
    /// units via the COV_UNITS keyword does not override the mandatory units specified for the
    /// selected COV_TYPE (links to the relevant SANA registries provided in annex B,
    /// subsections B7 and B8).
    ///
    /// Examples: [km,km,km,km/s,km/s,km/s]
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_cov_units(&self) -> Option<String> {
        self.inner.cov_units.clone()
    }
    #[setter]
    fn set_cov_units(&mut self, value: Option<String>) {
        self.inner.cov_units = value;
    }

    /// Contiguous set of covariance matrix data lines.
    ///
    /// :type: list[CovLine]
    #[getter]
    fn get_cov_lines(&self) -> Vec<CovLine> {
        self.inner
            .cov_lines
            .iter()
            .map(|c| CovLine { inner: c.clone() })
            .collect()
    }
    #[setter]
    fn set_cov_lines(&mut self, value: Vec<CovLine>) {
        self.inner.cov_lines = value.into_iter().map(|c| c.inner).collect();
    }

    /// Comments (a contiguous set of one or more comment lines may be provided in the OCM
    /// covariance time history section only immediately after the COV_START keyword; see 7.8
    /// for comment formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }
}

/// A single line in a covariance time history.
///
/// Parameters
/// ----------
/// epoch : str
///     Absolute or relative time tag.
/// values : list of float
///     Covariance matrix elements for this epoch.
#[pyclass]
#[derive(Clone)]
pub struct CovLine {
    pub inner: core_ocm::CovLine,
}

#[pymethods]
impl CovLine {
    /// Create a new CovLine object.
    #[new]
    #[pyo3(signature = (*, epoch, values))]
    fn new(epoch: String, values: Vec<f64>) -> Self {
        Self {
            inner: core_ocm::CovLine { epoch, values },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "CovLine(epoch='{}', values={})",
            self.inner.epoch,
            self.inner.values.len()
        )
    }

    /// Absolute or relative time tag.
    ///
    /// :type: str
    #[getter]
    fn get_epoch(&self) -> String {
        self.inner.epoch.clone()
    }
    #[setter]
    fn set_epoch(&mut self, value: String) {
        self.inner.epoch = value;
    }

    /// Covariance matrix elements for this epoch.
    ///
    /// :type: list[float]
    #[getter]
    fn get_values(&self) -> Vec<f64> {
        self.inner.values.clone()
    }
    #[setter]
    fn set_values(&mut self, value: Vec<f64>) {
        self.inner.values = value;
    }
}

// ============================================================================
// OcmManeuverParameters - Maneuver Parameters
// ============================================================================

/// OCM Maneuver Parameters.
///
/// Parameters
/// ----------
/// man_id : str
///     Identifier for the maneuver block.
/// man_device_id : str
///     Identifier for the maneuver device (e.g., thruster name).
/// man_composition : str
///     Specifies the maneuver composition (e.g., 'VECTOR', 'SCALAR').
/// man_ref_frame : str
///     Reference frame for the maneuver data.
/// man_lines : list of ManLine
///     A list of maneuver data lines.
/// man_prev_id : str, optional
///     Identifier for the previous maneuver block for this space object.
/// man_next_id : str, optional
///     Identifier for the next maneuver block for this space object.
/// man_basis : str, optional
///     Basis of the maneuver data ('Observed', 'Predicted', etc.).
/// man_basis_id : str, optional
///     Identifier for the orbit determination or simulation basis.
/// man_prev_epoch : str, optional
///     Epoch of the previous maneuver.
/// man_next_epoch : str, optional
///     Epoch of the next maneuver.
/// man_purpose : str, optional
///     Purpose of the maneuver.
/// man_pred_source : str, optional
///     Source of the predicted maneuver data.
/// man_frame_epoch : str, optional
///     Epoch of the maneuver reference frame.
/// grav_assist_name : str, optional
///     Name of the gravity assist body.
/// dc_type : str, optional
///     Type of duty cycle ('Continuous', 'Impulsive', 'Duration').
/// man_units : str, optional
///     SI unit designations for the maneuver elements.
/// comment : list of str, optional
///     Comments for this maneuver block.
#[pyclass]
#[derive(Clone)]
pub struct OcmManeuverParameters {
    pub inner: core_ocm::OcmManeuverParameters,
}

#[pymethods]
impl OcmManeuverParameters {
    /// Create a new OcmManeuverParameters object.
    #[new]
    #[pyo3(signature = (
        *,
        man_id,
        man_device_id,
        man_composition,
        man_ref_frame,
        man_lines,
        man_prev_id=None,
        man_next_id=None,
        man_basis=None,
        man_basis_id=None,
        man_prev_epoch=None,
        man_next_epoch=None,
        man_purpose=None,
        man_pred_source=None,
        man_frame_epoch=None,
        grav_assist_name=None,
        dc_type=None,
        dc_win_open=None,
        dc_win_close=None,
        dc_min_cycles=None,
        dc_max_cycles=None,
        dc_exec_start=None,
        dc_exec_stop=None,
        dc_ref_time=None,
        dc_time_pulse_duration=None,
        dc_time_pulse_period=None,
        dc_ref_dir=None,
        dc_body_frame=None,
        dc_body_trigger=None,
        dc_pa_start_angle=None,
        dc_pa_stop_angle=None,
        man_units=None,
        comment=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        man_id: String,
        man_device_id: String,
        man_composition: String,
        man_ref_frame: String,
        man_lines: Vec<ManLine>,
        man_prev_id: Option<String>,
        man_next_id: Option<String>,
        man_basis: Option<String>,
        man_basis_id: Option<String>,
        man_prev_epoch: Option<String>,
        man_next_epoch: Option<String>,
        man_purpose: Option<String>,
        man_pred_source: Option<String>,
        man_frame_epoch: Option<String>,
        grav_assist_name: Option<String>,
        dc_type: Option<String>,
        dc_win_open: Option<String>,
        dc_win_close: Option<String>,
        dc_min_cycles: Option<u64>,
        dc_max_cycles: Option<u64>,
        dc_exec_start: Option<String>,
        dc_exec_stop: Option<String>,
        dc_ref_time: Option<String>,
        dc_time_pulse_duration: Option<f64>,
        dc_time_pulse_period: Option<f64>,
        dc_ref_dir: Option<Vec<f64>>,
        dc_body_frame: Option<String>,
        dc_body_trigger: Option<Vec<f64>>,
        dc_pa_start_angle: Option<f64>,
        dc_pa_stop_angle: Option<f64>,
        man_units: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        use ccsds_ndm::types::{Angle, Duration, Vec3Double};

        let dc_ref_dir = if let Some(v) = dc_ref_dir {
            if v.len() != 3 {
                return Err(PyValueError::new_err(
                    "dc_ref_dir must have exactly 3 elements",
                ));
            }
            Some(Vec3Double {
                x: v[0],
                y: v[1],
                z: v[2],
            })
        } else {
            None
        };

        let dc_body_trigger = if let Some(v) = dc_body_trigger {
            if v.len() != 3 {
                return Err(PyValueError::new_err(
                    "dc_body_trigger must have exactly 3 elements",
                ));
            }
            Some(Vec3Double {
                x: v[0],
                y: v[1],
                z: v[2],
            })
        } else {
            None
        };

        Ok(Self {
            inner: core_ocm::OcmManeuverParameters {
                comment: comment.unwrap_or_default(),
                man_id,
                man_prev_id,
                man_next_id,
                man_basis: man_basis.map(|s| s.parse()).transpose().map_err(
                    |e: ccsds_ndm::error::EnumParseError| PyValueError::new_err(e.to_string()),
                )?,
                man_basis_id,
                man_device_id,
                man_prev_epoch: man_prev_epoch.map(|s| parse_epoch(&s)).transpose()?,
                man_next_epoch: man_next_epoch.map(|s| parse_epoch(&s)).transpose()?,
                man_purpose,
                man_pred_source,
                man_ref_frame,
                man_frame_epoch: man_frame_epoch.map(|s| parse_epoch(&s)).transpose()?,
                grav_assist_name,
                dc_type: dc_type
                    .map(|s| s.parse())
                    .transpose()
                    .map_err(|e: ccsds_ndm::error::EnumParseError| {
                        PyValueError::new_err(e.to_string())
                    })?
                    .unwrap_or(ccsds_ndm::types::ManDc::Continuous),
                dc_win_open: dc_win_open.map(|s| parse_epoch(&s)).transpose()?,
                dc_win_close: dc_win_close.map(|s| parse_epoch(&s)).transpose()?,
                dc_min_cycles,
                dc_max_cycles,
                dc_exec_start: dc_exec_start.map(|s| parse_epoch(&s)).transpose()?,
                dc_exec_stop: dc_exec_stop.map(|s| parse_epoch(&s)).transpose()?,
                dc_ref_time: dc_ref_time.map(|s| parse_epoch(&s)).transpose()?,
                dc_time_pulse_duration: dc_time_pulse_duration.map(|v| Duration {
                    value: v,
                    units: None,
                }),
                dc_time_pulse_period: dc_time_pulse_period.map(|v| Duration {
                    value: v,
                    units: None,
                }),
                dc_ref_dir,
                dc_body_frame,
                dc_body_trigger,
                dc_pa_start_angle: dc_pa_start_angle.map(|v| Angle {
                    value: v,
                    units: None,
                }),
                dc_pa_stop_angle: dc_pa_stop_angle.map(|v| Angle {
                    value: v,
                    units: None,
                }),
                man_composition,
                man_units,
                man_lines: man_lines.into_iter().map(|l| l.inner).collect(),
            },
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "OcmManeuverParameters(man_id='{}', device='{}')",
            self.inner.man_id, self.inner.man_device_id
        )
    }

    /// Free-text field containing the unique maneuver identification number for this maneuver.
    /// All supplied maneuver 'constituents' within the same MAN_BASIS and MAN_REF_FRAME
    /// categories shall be added together to represent the total composite maneuver
    /// description.
    ///
    /// :type: str
    #[getter]
    fn get_man_id(&self) -> String {
        self.inner.man_id.clone()
    }
    #[setter]
    fn set_man_id(&mut self, value: String) {
        self.inner.man_id = value;
    }

    /// Free-text field containing the identification number of the previous maneuver for this
    /// MAN_BASIS, contained either within this message, or presented in a previous OCM. If
    /// this message is not part of a sequence of maneuver messages or if this maneuver is the
    /// first in a sequence of maneuvers, then MAN_PREV_ID should be excluded from this
    /// message.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_man_prev_id(&self) -> Option<String> {
        self.inner.man_prev_id.clone()
    }
    #[setter]
    fn set_man_prev_id(&mut self, value: Option<String>) {
        self.inner.man_prev_id = value;
    }

    /// Free-text field containing the identification number of the next maneuver for this
    /// MAN_BASIS, contained either within this message, or presented in a future OCM. If this
    /// message is not part of a sequence of maneuver messages or if this maneuver is the last
    /// in a sequence of maneuvers, then MAN_NEXT_ID should be excluded from this message.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_man_next_id(&self) -> Option<String> {
        self.inner.man_next_id.clone()
    }
    #[setter]
    fn set_man_next_id(&mut self, value: Option<String>) {
        self.inner.man_next_id = value;
    }

    /// Basis of this maneuver time history data, which shall be selected from one of the
    /// following values: 'CANDIDATE' for a proposed operational or a hypothetical (i.e.,
    /// mission design and optimization studies) future maneuver, 'PLANNED' for a currently
    /// planned future maneuver, 'ANTICIPATED' for a non-cooperative future maneuver that is
    /// anticipated (i.e., likely) to occur (e.g., based upon patterns-of-life analysis),
    /// 'TELEMETRY' when the maneuver is determined directly from telemetry (e.g., based on
    /// inertial navigation systems or accelerometers), 'DETERMINED' when a past maneuver is
    /// estimated from observation-based orbit determination reconstruction and/or
    /// calibration, 'SIMULATED' for generic maneuver simulations, future mission design
    /// studies, and optimization studies, 'OTHER' for other bases of this data.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_man_basis(&self) -> Option<String> {
        self.inner.man_basis.as_ref().map(|b| format!("{:?}", b))
    }

    /// Free-text field containing the identification number for the orbit determination,
    /// navigation solution, or simulation upon which this maneuver time history block is
    /// based. Where a matching orbit determination block accompanies this maneuver time
    /// history, the MAN_BASIS_ID should match the corresponding OD_ID (see table 6-11).
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_man_basis_id(&self) -> Option<String> {
        self.inner.man_basis_id.clone()
    }
    #[setter]
    fn set_man_basis_id(&mut self, value: Option<String>) {
        self.inner.man_basis_id = value;
    }

    /// Free-text field containing the maneuver device identifier used for this maneuver. 'ALL'
    /// indicates that this maneuver represents the summed acceleration, velocity increment,
    /// or thrust imparted by any/all thrusters utilized in the maneuver.
    ///
    /// :type: str
    #[getter]
    fn get_man_device_id(&self) -> String {
        self.inner.man_device_id.clone()
    }
    #[setter]
    fn set_man_device_id(&mut self, value: String) {
        self.inner.man_device_id = value;
    }
    /// Identifies the completion time of the previous maneuver for this MAN_BASIS.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_man_prev_epoch(&self) -> Option<String> {
        self.inner
            .man_prev_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_man_prev_epoch(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.man_prev_epoch = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Identifies the start time of the next maneuver for this MAN_BASIS.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_man_next_epoch(&self) -> Option<String> {
        self.inner
            .man_next_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_man_next_epoch(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.man_next_epoch = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// A free-text field used to specify the intention(s) of the maneuver. Multiple maneuver
    /// purposes can be provided as a comma-delimited list.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_man_purpose(&self) -> Option<String> {
        self.inner.man_purpose.clone()
    }
    #[setter]
    fn set_man_purpose(&mut self, value: Option<String>) {
        self.inner.man_purpose = value;
    }

    /// For future maneuvers, specifies the source of the orbit and/or attitude state(s) upon
    /// which the maneuver is based. While there is no CCSDS-based restriction on the value for
    /// this free-text keyword, it is suggested to consider using TRAJ_ID and OD_ID keywords
    /// as described in tables 6-4 and 6-11, respectively, or a combination thereof.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_man_pred_source(&self) -> Option<String> {
        self.inner.man_pred_source.clone()
    }
    #[setter]
    fn set_man_pred_source(&mut self, value: Option<String>) {
        self.inner.man_pred_source = value;
    }
    /// Reference frame in which all maneuver vector direction data is provided in this
    /// maneuver data block. Select from the accepted set of values indicated in annex B,
    /// subsections B4 and B5. The reference frame must be the same for all data elements
    /// within a given maneuver time history block.
    ///
    /// :type: str
    #[getter]
    fn get_man_ref_frame(&self) -> String {
        self.inner.man_ref_frame.clone()
    }
    #[setter]
    fn set_man_ref_frame(&mut self, value: String) {
        self.inner.man_ref_frame = value;
    }

    /// Epoch of the maneuver data reference frame, if not intrinsic to the definition of the
    /// reference frame. (See 7.5.10 for formatting rules.)
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_man_frame_epoch(&self) -> Option<String> {
        self.inner
            .man_frame_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_man_frame_epoch(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.man_frame_epoch = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Origin of maneuver gravitational assist body, which may be a natural solar system body
    /// (planets, asteroids, comets, and natural satellites), including any planet barycenter
    /// or the solar system barycenter. (See annex B, subsection B2, for acceptable
    /// GRAV_ASSIST_NAME values and the procedure to propose new values.)
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_grav_assist_name(&self) -> Option<String> {
        self.inner.grav_assist_name.clone()
    }
    #[setter]
    fn set_grav_assist_name(&mut self, value: Option<String>) {
        self.inner.grav_assist_name = value;
    }

    /// Duty cycle type to use for this maneuver time history section: CONTINUOUS denotes
    /// full/continuous thrust `<default>`; TIME denotes a time-based duty cycle driven by time
    /// past a reference time and the duty cycle ON and OFF durations; TIME_AND_ANGLE denotes a
    /// duty cycle driven by the phasing/clocking of a space object body frame 'trigger'
    /// direction past a reference direction.
    ///
    /// :type: str
    #[getter]
    fn get_dc_type(&self) -> String {
        format!("{:?}", self.inner.dc_type)
    }
    #[setter]
    fn set_dc_type(&mut self, value: String) -> PyResult<()> {
        self.inner.dc_type = value
            .parse()
            .map_err(|e: ccsds_ndm::error::EnumParseError| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Start time of the duty cycle-based maneuver window that occurs on or prior to the
    /// actual maneuver execution start time. For example, this may identify the time at which
    /// the satellite is first placed into a special duty-cycle-based maneuver mode. This
    /// keyword shall be set if DC_TYPE ≠ 'CONTINUOUS'.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_dc_win_open(&self) -> Option<String> {
        self.inner
            .dc_win_open
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_dc_win_open(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.dc_win_open = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// End time of the duty cycle-based maneuver window that occurs on or after the actual
    /// maneuver execution end time. For example, this may identify the time at which the
    /// satellite is taken out of a special duty-cycle-based maneuver mode. This keyword shall
    /// be set if DC_TYPE ≠ 'CONTINUOUS'.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_dc_win_close(&self) -> Option<String> {
        self.inner
            .dc_win_close
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_dc_win_close(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.dc_win_close = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Minimum number of 'ON' duty cycles (may override DC_EXEC_STOP). This value is optional
    /// even if DC_TYPE = 'CONTINUOUS'.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_dc_min_cycles(&self) -> Option<u64> {
        self.inner.dc_min_cycles
    }
    #[setter]
    fn set_dc_min_cycles(&mut self, value: Option<u64>) {
        self.inner.dc_min_cycles = value;
    }

    /// Maximum number of 'ON' duty cycles (may override DC_EXEC_STOP). This value is optional
    /// even if DC_TYPE = 'CONTINUOUS'.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_dc_max_cycles(&self) -> Option<u64> {
        self.inner.dc_max_cycles
    }
    #[setter]
    fn set_dc_max_cycles(&mut self, value: Option<u64>) {
        self.inner.dc_max_cycles = value;
    }

    /// Start time of the initial duty cycle-based maneuver sequence execution. DC_EXEC_START
    /// is defined to occur on or prior to the first maneuver 'ON' portion within the duty
    /// cycle sequence. DC_EXEC_START must be scheduled to occur coincident with or after
    /// DC_WIN_OPEN. This keyword shall be set if DC_TYPE ≠ 'CONTINUOUS'.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_dc_exec_start(&self) -> Option<String> {
        self.inner
            .dc_exec_start
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_dc_exec_start(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.dc_exec_start = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// End time of the final duty cycle-based maneuver sequence execution. DC_EXEC_STOP
    /// typically occurs on or after the end of the final maneuver 'ON' portion within the duty
    /// cycle sequence. DC_EXEC_STOP must be scheduled to occur coincident with or prior to
    /// DC_WIN_CLOSE. This keyword shall be set if DC_TYPE ≠ 'CONTINUOUS'.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_dc_exec_stop(&self) -> Option<String> {
        self.inner
            .dc_exec_stop
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_dc_exec_stop(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.dc_exec_stop = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Reference time for the THRUST duty cycle, specified as either time in seconds (relative
    /// to EPOCH_TZERO), or as an absolute '`<epoch>`' (see 7.5.10 for formatting rules).
    /// NOTE—Depending upon EPOCH_TZERO, DC_REF_TIME relative times may be negative. This
    /// keyword shall be set if DC_TYPE ≠ 'CONTINUOUS'.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_dc_ref_time(&self) -> Option<String> {
        self.inner
            .dc_ref_time
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_dc_ref_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.dc_ref_time = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Thruster pulse 'ON' duration, initiated at first satisfaction of the burn 'ON' time
    /// constraint or upon completion of the previous DC_TIME_PULSE_PERIOD cycle. This keyword
    /// shall be set if DC_TYPE ≠ 'CONTINUOUS'.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_dc_time_pulse_duration(&self) -> Option<f64> {
        self.inner.dc_time_pulse_duration.as_ref().map(|d| d.value)
    }
    #[setter]
    fn set_dc_time_pulse_duration(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Duration;
        self.inner.dc_time_pulse_duration = value.map(|v| Duration {
            value: v,
            units: None,
        });
    }

    /// Elapsed time between the start of one pulse and the start of the next. Must be greater
    /// than or equal to DC_TIME_PULSE_DURATION. This keyword shall be set if DC_TYPE ≠
    /// 'CONTINUOUS'.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_dc_time_pulse_period(&self) -> Option<f64> {
        self.inner.dc_time_pulse_period.as_ref().map(|d| d.value)
    }
    #[setter]
    fn set_dc_time_pulse_period(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Duration;
        self.inner.dc_time_pulse_period = value.map(|v| Duration {
            value: v,
            units: None,
        });
    }

    /// For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the reference
    /// vector direction in the 'MAN_REF_FRAME' reference frame at which, when mapped into the
    /// space object's spin plane (normal to the spin axis), the duty cycle is triggered (see
    /// DC_PA_START_ANGLE for phasing). This (tripartite, or three-element vector) value shall
    /// be provided if DC_TYPE = 'TIME_AND_ANGLE'. This reference direction does not represent
    /// the duty cycle midpoint.
    ///
    /// :type: Optional[list[float]]
    #[getter]
    fn get_dc_ref_dir(&self) -> Option<Vec<f64>> {
        self.inner.dc_ref_dir.as_ref().map(|v| vec![v.x, v.y, v.z])
    }
    #[setter]
    fn set_dc_ref_dir(&mut self, value: Option<Vec<f64>>) -> PyResult<()> {
        use ccsds_ndm::types::Vec3Double;
        if let Some(v) = value {
            if v.len() != 3 {
                return Err(PyValueError::new_err(
                    "dc_ref_dir must have exactly 3 elements",
                ));
            }
            self.inner.dc_ref_dir = Some(Vec3Double {
                x: v[0],
                y: v[1],
                z: v[2],
            });
        } else {
            self.inner.dc_ref_dir = None;
        }
        Ok(())
    }

    /// For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the body
    /// reference frame in which DC_BODY_TRIGGER will be specified. Select from the accepted
    /// set of values indicated in annex B, subsection B6. This keyword shall be set if
    /// DC_TYPE = 'TIME_AND_ANGLE'.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_dc_body_frame(&self) -> Option<String> {
        self.inner.dc_body_frame.clone()
    }
    #[setter]
    fn set_dc_body_frame(&mut self, value: Option<String>) {
        self.inner.dc_body_frame = value;
    }

    /// For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the body frame
    /// reference vector direction in the 'DC_BODY_FRAME' reference frame at which, when its
    /// projection onto the spin plane crosses the corresponding projection of DC_REF_DIR onto
    /// the spin plane, this angle-based duty cycle is initiated (see DC_PA_START_ANGLE for
    /// phasing). This tripartite value shall be provided if DC_TYPE = 'TIME_AND_ANGLE'.
    ///
    /// :type: Optional[list[float]]
    #[getter]
    fn get_dc_body_trigger(&self) -> Option<Vec<f64>> {
        self.inner
            .dc_body_trigger
            .as_ref()
            .map(|v| vec![v.x, v.y, v.z])
    }
    #[setter]
    fn set_dc_body_trigger(&mut self, value: Option<Vec<f64>>) -> PyResult<()> {
        use ccsds_ndm::types::Vec3Double;
        if let Some(v) = value {
            if v.len() != 3 {
                return Err(PyValueError::new_err(
                    "dc_body_trigger must have exactly 3 elements",
                ));
            }
            self.inner.dc_body_trigger = Some(Vec3Double {
                x: v[0],
                y: v[1],
                z: v[2],
            });
        } else {
            self.inner.dc_body_trigger = None;
        }
        Ok(())
    }

    /// For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the phase angle
    /// offset of thruster pulse start, measured with respect to the occurrence of a
    /// DC_BODY_TRIGGER crossing of the DC_REF_DIR direction when both are projected into the
    /// spin plane (normal to the body spin axis). This phase angle offset can be positive or
    /// negative to allow the duty cycle to begin prior to the next crossing of the
    /// DC_REF_DIR. As this angular direction is to be used in a modulo sense, there is no
    /// requirement for the magnitude of the phase angle offset to be less than 360 degrees.
    /// This keyword shall be set if DC_TYPE = 'TIME_AND_ANGLE'.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_dc_pa_start_angle(&self) -> Option<f64> {
        self.inner.dc_pa_start_angle.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_dc_pa_start_angle(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Angle;
        self.inner.dc_pa_start_angle = value.map(|v| Angle {
            value: v,
            units: None,
        });
    }

    /// For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the phase angle
    /// of thruster pulse stop, measured with respect to the DC_BODY_TRIGGER crossing of the
    /// DC_REF_DIR direction when both are projected into the spin plane. This phase angle
    /// offset can be positive or negative to allow the duty cycle to end after to the next
    /// crossing of the DC_REF_DIR. As this angular direction is to be used in a modulo sense,
    /// there is no requirement for the magnitude of the phase angle offset to be less than
    /// 360 degrees. This keyword shall be set if DC_TYPE = 'TIME_AND_ANGLE'.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_dc_pa_stop_angle(&self) -> Option<f64> {
        self.inner.dc_pa_stop_angle.as_ref().map(|a| a.value)
    }
    #[setter]
    fn set_dc_pa_stop_angle(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Angle;
        self.inner.dc_pa_stop_angle = value.map(|v| Angle {
            value: v,
            units: None,
        });
    }

    /// The comma-delimited ordered set of maneuver elements of information contained on every
    /// maneuver time history line, with values selected from table 6-8. Within this maneuver
    /// data section, the maneuver composition shall include only one TIME specification
    /// (TIME_ABSOLUTE or TIME_RELATIVE).
    ///
    /// :type: str
    #[getter]
    fn get_man_composition(&self) -> String {
        self.inner.man_composition.clone()
    }
    #[setter]
    fn set_man_composition(&mut self, value: String) {
        self.inner.man_composition = value;
    }
    /// A comma-delimited set of SI unit designations for each and every element of the
    /// maneuver time history following the maneuver time tag(s), solely for informational
    /// purposes, provided as a free-text field enclosed in square brackets. When MAN_UNITS is
    /// provided, all elements of MAN_COMPOSITION AFTER the maneuver time tag(s) must have a
    /// corresponding units entry; percentages shall be denoted by '%', and control switches,
    /// non-dimensional values, and text strings shall be labelled as 'n/a'. NOTE—The listing
    /// of units via the MAN_UNITS keyword does not override the mandatory units for the
    /// selected MAN_COMPOSITION, as specified in table 6-8 or table 6-9.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_man_units(&self) -> Option<String> {
        self.inner.man_units.clone()
    }
    #[setter]
    fn set_man_units(&mut self, value: Option<String>) {
        self.inner.man_units = value;
    }
    /// Maneuver time history data lines.
    ///
    /// :type: list[ManLine]
    #[getter]
    fn get_man_lines(&self) -> Vec<ManLine> {
        self.inner
            .man_lines
            .iter()
            .map(|l| ManLine { inner: l.clone() })
            .collect()
    }
    #[setter]
    fn set_man_lines(&mut self, value: Vec<ManLine>) {
        self.inner.man_lines = value.into_iter().map(|l| l.inner).collect();
    }

    /// Comments (a contiguous set of one or more comment lines may be provided in the OCM
    /// Maneuver Specification only immediately after the MAN_START keyword; see 7.8 for
    /// comment formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }
}

/// A single line in a maneuver time history.
///
/// Parameters
/// ----------
/// epoch : str
///     Ignition epoch.
/// values : list of str
///     Maneuver elements for this epoch.
#[pyclass]
#[derive(Clone)]
pub struct ManLine {
    pub inner: core_ocm::ManLine,
}

#[pymethods]
impl ManLine {
    /// Create a new ManLine object.
    #[new]
    #[pyo3(signature = (*, epoch, values))]
    fn new(epoch: String, values: Vec<String>) -> Self {
        Self {
            inner: core_ocm::ManLine { epoch, values },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "ManLine(epoch='{}', values={})",
            self.inner.epoch,
            self.inner.values.len()
        )
    }

    /// Ignition epoch.
    ///
    /// :type: str
    #[getter]
    fn get_epoch(&self) -> String {
        self.inner.epoch.clone()
    }
    #[setter]
    fn set_epoch(&mut self, value: String) {
        self.inner.epoch = value;
    }

    /// Maneuver elements for this epoch.
    ///
    /// :type: list[str]
    #[getter]
    fn get_values(&self) -> Vec<String> {
        self.inner.values.clone()
    }
    #[setter]
    fn set_values(&mut self, value: Vec<String>) {
        self.inner.values = value;
    }
}

// ============================================================================
// OcmPerturbations - Perturbation Model Specification
// ============================================================================

/// OCM Perturbations Parameters.
///
/// Parameters
/// ----------
/// comment : list[str], optional
///     Comments.
///     (Optional)
#[pyclass]
#[derive(Clone)]
pub struct OcmPerturbations {
    pub inner: core_ocm::OcmPerturbations,
}

#[pymethods]
impl OcmPerturbations {
    /// Create a new OcmPerturbations object.
    #[new]
    fn new() -> Self {
        Self {
            inner: core_ocm::OcmPerturbations::default(),
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "OcmPerturbations(gravity_model={:?})",
            self.inner.gravity_model
        )
    }

    /// Comments (a contiguous set of one or more comment lines may be provided in the OCM
    /// Perturbations Specification only immediately after the PERT_START keyword; see 7.8 for
    /// comment formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }
    /// Name of atmosphere model, which shall be selected from the accepted set of values
    /// indicated in annex B, subsection B9.
    ///
    /// Examples: MSISE90, NRLMSIS00, J70, J71, JROBERTS, DTM, JB2008
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_atmospheric_model(&self) -> Option<String> {
        self.inner.atmospheric_model.clone()
    }
    #[setter]
    fn set_atmospheric_model(&mut self, value: Option<String>) {
        self.inner.atmospheric_model = value;
    }
    /// The gravity model (selected from the accepted set of gravity model names indicated in
    /// annex B, subsection B10), followed by the degree (D) and order (O) of the applied
    /// spherical harmonic coefficients used in the simulation. NOTE—Specifying a zero value
    /// for 'order' (e.g., 2D 0O) denotes zonals (J2 ... JD).
    ///
    /// Examples: EGM-96: 36D 36O, WGS-84: 8D 0O, GGM-01: 36D 36O, TEG-4: 36D 36O
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_gravity_model(&self) -> Option<String> {
        self.inner.gravity_model.clone()
    }
    #[setter]
    fn set_gravity_model(&mut self, value: Option<String>) {
        self.inner.gravity_model = value;
    }
    /// Oblate spheroid equatorial radius of the central body used in the message, if
    /// different from the gravity model.
    ///
    /// Units: km
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_equatorial_radius(&self) -> Option<f64> {
        self.inner.equatorial_radius.as_ref().map(|p| p.value)
    }
    #[setter]
    fn set_equatorial_radius(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Position;
        self.inner.equatorial_radius = value.map(|v| Position {
            value: v,
            units: None,
        });
    }
    /// Gravitational coefficient of attracting body (Gravitational Constant × Central Mass),
    /// if different from the gravity model.
    ///
    /// Units: km³/s²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_gm(&self) -> Option<f64> {
        self.inner.gm.as_ref().map(|g| g.value)
    }
    #[setter]
    fn set_gm(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Gm;
        self.inner.gm = value.map(|v| Gm::new(v, None).unwrap());
    }
    /// One OR MORE (N-body) gravitational perturbations bodies used. Values, listed serially
    /// in comma-delimited fashion, denote a natural solar or extra-solar system body (stars,
    /// planets, asteroids, comets, and natural satellites). NOTE—Only those entries specified
    /// under CENTER_NAME in annex B, subsection B2 are acceptable values.
    ///
    /// Examples: MOON, SUN, JUPITER
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_n_body_perturbations(&self) -> Option<String> {
        self.inner.n_body_perturbations.clone()
    }
    #[setter]
    fn set_n_body_perturbations(&mut self, value: Option<String>) {
        self.inner.n_body_perturbations = value;
    }
    /// Central body angular rotation rate, measured about the major principal axis of the
    /// inertia tensor of the central body, relating inertial, and central-body-fixed
    /// reference frames. NOTE—The rotation axis may be slightly offset from the inertial
    /// frame Z-axis definition.
    ///
    /// Units: deg/s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_central_body_rotation(&self) -> Option<f64> {
        self.inner.central_body_rotation.as_ref().map(|r| r.value)
    }
    #[setter]
    fn set_central_body_rotation(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::AngleRate;
        self.inner.central_body_rotation = value.map(|v| AngleRate {
            value: v,
            units: None,
        });
    }
    /// Central body's oblate spheroid oblateness for the polar-symmetric oblate central body
    /// model (e.g., for the Earth, it is approximately 1.0/298.257223563).
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_oblate_flattening(&self) -> Option<f64> {
        self.inner.oblate_flattening
    }
    #[setter]
    fn set_oblate_flattening(&mut self, value: Option<f64>) {
        self.inner.oblate_flattening = value;
    }
    /// Name of ocean tides model (optionally specify order or constituent effects, diurnal,
    /// semi-diurnal, etc.). This is a free-text field, so if the examples on the right are
    /// insufficient, others may be used.
    ///
    /// Examples: DIURNAL, SEMI-DIURNAL
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_ocean_tides_model(&self) -> Option<String> {
        self.inner.ocean_tides_model.clone()
    }
    #[setter]
    fn set_ocean_tides_model(&mut self, value: Option<String>) {
        self.inner.ocean_tides_model = value;
    }
    /// Name of solid tides model (optionally specify order or constituent effects, diurnal,
    /// semi-diurnal, etc.).
    ///
    /// Examples: DIURNAL, SEMI-DIURNAL
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_solid_tides_model(&self) -> Option<String> {
        self.inner.solid_tides_model.clone()
    }
    #[setter]
    fn set_solid_tides_model(&mut self, value: Option<String>) {
        self.inner.solid_tides_model = value;
    }
    /// Specification of the reduction theory used for precession and nutation modeling. This
    /// is a free-text field, so if the examples on the right are insufficient, others may be
    /// used.
    ///
    /// Examples: IAU1976/FK5, IAU2010, IERS1996
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_reduction_theory(&self) -> Option<String> {
        self.inner.reduction_theory.clone()
    }
    #[setter]
    fn set_reduction_theory(&mut self, value: Option<String>) {
        self.inner.reduction_theory = value;
    }
    /// Name of the albedo model.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_albedo_model(&self) -> Option<String> {
        self.inner.albedo_model.clone()
    }
    #[setter]
    fn set_albedo_model(&mut self, value: Option<String>) {
        self.inner.albedo_model = value;
    }
    /// Size of the albedo grid.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_albedo_grid_size(&self) -> Option<u64> {
        self.inner.albedo_grid_size
    }
    #[setter]
    fn set_albedo_grid_size(&mut self, value: Option<u64>) {
        self.inner.albedo_grid_size = value;
    }
    /// Shadow model used for Solar Radiation Pressure; dual cone uses both umbra/penumbra
    /// regions. Selected option should be one of ‘NONE’, ‘CYLINDRICAL’, ‘CONE’, or
    /// ‘DUAL_CONE’.
    ///
    /// Examples: NONE, CYLINDRICAL, CONE, DUAL_CONE
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_shadow_model(&self) -> Option<String> {
        self.inner.shadow_model.clone()
    }
    #[setter]
    fn set_shadow_model(&mut self, value: Option<String>) {
        self.inner.shadow_model = value;
    }
    /// List of bodies included in shadow calculations (value(s) to be drawn from the SANA
    /// registry list of Orbit Centers at <https://sanaregistry.org/r/orbit_centers>).
    ///
    /// Examples: EARTH, MOON
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_shadow_bodies(&self) -> Option<String> {
        self.inner.shadow_bodies.clone()
    }
    #[setter]
    fn set_shadow_bodies(&mut self, value: Option<String>) {
        self.inner.shadow_bodies = value;
    }
    /// Name of the Solar Radiation Pressure (SRP) model.
    ///
    /// Examples: CANNONBALL, FLAT_PLATE, BOX_WING
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_srp_model(&self) -> Option<String> {
        self.inner.srp_model.clone()
    }
    #[setter]
    fn set_srp_model(&mut self, value: Option<String>) {
        self.inner.srp_model = value;
    }
    /// Space weather data source.
    ///
    /// Examples: NOAA
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_sw_data_source(&self) -> Option<String> {
        self.inner.sw_data_source.clone()
    }
    #[setter]
    fn set_sw_data_source(&mut self, value: Option<String>) {
        self.inner.sw_data_source = value;
    }
    /// Epoch of the space weather data.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_sw_data_epoch(&self) -> Option<String> {
        self.inner
            .sw_data_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_sw_data_epoch(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.sw_data_epoch = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }
    /// Free-text field specifying the method used to select or interpolate any and all
    /// sequential space weather data (Kp, ap, Dst, F10.7, M10.7, S10.7, Y10.7, etc.). While
    /// not constrained to specific entries, it is anticipated that the utilized method would
    /// match methods detailed in numerical analysis textbooks.
    ///
    /// Examples: PRECEDING_VALUE, NEAREST_NEIGHBOR, LINEAR, LAGRANGE_ORDER_5
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_sw_interp_method(&self) -> Option<String> {
        self.inner.sw_interp_method.clone()
    }
    #[setter]
    fn set_sw_interp_method(&mut self, value: Option<String>) {
        self.inner.sw_interp_method = value;
    }
    /// Fixed geomagnetic Kp index.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_fixed_geomag_kp(&self) -> Option<f64> {
        self.inner.fixed_geomag_kp.as_ref().map(|g| g.value)
    }
    #[setter]
    fn set_fixed_geomag_kp(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Geomag;
        self.inner.fixed_geomag_kp = value.map(|v| Geomag {
            value: v,
            units: None,
        });
    }
    /// Fixed geomagnetic Ap index.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_fixed_geomag_ap(&self) -> Option<f64> {
        self.inner.fixed_geomag_ap.as_ref().map(|g| g.value)
    }
    #[setter]
    fn set_fixed_geomag_ap(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Geomag;
        self.inner.fixed_geomag_ap = value.map(|v| Geomag {
            value: v,
            units: None,
        });
    }
    /// Fixed geomagnetic Dst index.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_fixed_geomag_dst(&self) -> Option<f64> {
        self.inner.fixed_geomag_dst.as_ref().map(|g| g.value)
    }
    #[setter]
    fn set_fixed_geomag_dst(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Geomag;
        self.inner.fixed_geomag_dst = value.map(|v| Geomag {
            value: v,
            units: None,
        });
    }
    /// Fixed F10.7 solar flux.
    ///
    /// Units: SFU
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_fixed_f10p7(&self) -> Option<f64> {
        self.inner.fixed_f10p7.as_ref().map(|f| f.value)
    }
    #[setter]
    fn set_fixed_f10p7(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::SolarFlux;
        self.inner.fixed_f10p7 = value.map(|v| SolarFlux {
            value: v,
            units: None,
        });
    }
    /// Fixed 81-day average F10.7 solar flux.
    ///
    /// Units: SFU
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_fixed_f10p7_mean(&self) -> Option<f64> {
        self.inner.fixed_f10p7_mean.as_ref().map(|f| f.value)
    }
    #[setter]
    fn set_fixed_f10p7_mean(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::SolarFlux;
        self.inner.fixed_f10p7_mean = value.map(|v| SolarFlux {
            value: v,
            units: None,
        });
    }
    /// Fixed M10.7 solar flux.
    ///
    /// Units: SFU
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_fixed_m10p7(&self) -> Option<f64> {
        self.inner.fixed_m10p7.as_ref().map(|f| f.value)
    }
    #[setter]
    fn set_fixed_m10p7(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::SolarFlux;
        self.inner.fixed_m10p7 = value.map(|v| SolarFlux {
            value: v,
            units: None,
        });
    }
    /// Fixed 81-day average M10.7 solar flux.
    ///
    /// Units: SFU
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_fixed_m10p7_mean(&self) -> Option<f64> {
        self.inner.fixed_m10p7_mean.as_ref().map(|f| f.value)
    }
    #[setter]
    fn set_fixed_m10p7_mean(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::SolarFlux;
        self.inner.fixed_m10p7_mean = value.map(|v| SolarFlux {
            value: v,
            units: None,
        });
    }
    /// Fixed S10.7 solar flux.
    ///
    /// Units: SFU
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_fixed_s10p7(&self) -> Option<f64> {
        self.inner.fixed_s10p7.as_ref().map(|f| f.value)
    }
    #[setter]
    fn set_fixed_s10p7(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::SolarFlux;
        self.inner.fixed_s10p7 = value.map(|v| SolarFlux {
            value: v,
            units: None,
        });
    }
    /// Fixed 81-day average S10.7 solar flux.
    ///
    /// Units: SFU
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_fixed_s10p7_mean(&self) -> Option<f64> {
        self.inner.fixed_s10p7_mean.as_ref().map(|f| f.value)
    }
    #[setter]
    fn set_fixed_s10p7_mean(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::SolarFlux;
        self.inner.fixed_s10p7_mean = value.map(|v| SolarFlux {
            value: v,
            units: None,
        });
    }
    /// Fixed Y10.7 solar flux.
    ///
    /// Units: SFU
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_fixed_y10p7(&self) -> Option<f64> {
        self.inner.fixed_y10p7.as_ref().map(|f| f.value)
    }
    #[setter]
    fn set_fixed_y10p7(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::SolarFlux;
        self.inner.fixed_y10p7 = value.map(|v| SolarFlux {
            value: v,
            units: None,
        });
    }
    /// Fixed 81-day average Y10.7 solar flux.
    ///
    /// Units: SFU
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_fixed_y10p7_mean(&self) -> Option<f64> {
        self.inner.fixed_y10p7_mean.as_ref().map(|f| f.value)
    }
    #[setter]
    fn set_fixed_y10p7_mean(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::SolarFlux;
        self.inner.fixed_y10p7_mean = value.map(|v| SolarFlux {
            value: v,
            units: None,
        });
    }
}

// ============================================================================
// OcmOdParameters - Orbit Determination Parameters
// ============================================================================

/// OCM Orbit Determination Parameters.
///
/// Parameters
/// ----------
/// od_id : str
///     Identifier for the orbit determination parameters block.
///     (Mandatory)
/// od_method : str
///     Specifies the method used for the orbit determination.
///     (Mandatory)
/// od_epoch : str
///     Epoch of the orbit determination.
///     (Mandatory)
/// od_prev_id : str, optional
///     Identification number for the previous orbit determination block.
///     (Optional)
/// comment : list[str], optional
///     Comments.
///     (Optional)
#[pyclass]
#[derive(Clone)]
pub struct OcmOdParameters {
    pub inner: core_ocm::OcmOdParameters,
}

#[pymethods]
impl OcmOdParameters {
    #[new]
    #[pyo3(signature = (*, od_id, od_method, od_epoch, od_prev_id=None, comment=None))]
    fn new(
        od_id: String,
        od_method: String,
        od_epoch: String,
        od_prev_id: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_ocm::OcmOdParameters {
                comment: comment.unwrap_or_default(),
                od_id,
                od_prev_id,
                od_method,
                od_epoch: parse_epoch(&od_epoch)?,
                days_since_first_obs: None,
                days_since_last_obs: None,
                recommended_od_span: None,
                actual_od_span: None,
                obs_available: None,
                obs_used: None,
                tracks_available: None,
                tracks_used: None,
                maximum_obs_gap: None,
                od_epoch_eigmaj: None,
                od_epoch_eigint: None,
                od_epoch_eigmin: None,
                od_max_pred_eigmaj: None,
                od_min_pred_eigmin: None,
                od_confidence: None,
                gdop: None,
                solve_n: None,
                solve_states: None,
                consider_n: None,
                consider_params: None,
                sedr: None,
                sensors_n: None,
                sensors: None,
                weighted_rms: None,
                data_types: None,
            },
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "OcmOdParameters(od_id='{}', od_method='{}')",
            self.inner.od_id, self.inner.od_method
        )
    }

    /// Comments (see 7.8 for formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }
    /// Identification number for this orbit determination.
    ///
    /// Examples: 1
    ///
    /// :type: str
    #[getter]
    fn get_od_id(&self) -> String {
        self.inner.od_id.clone()
    }
    #[setter]
    fn set_od_id(&mut self, value: String) {
        self.inner.od_id = value;
    }

    /// Optional identification number for the previous orbit determination.
    ///
    /// Examples: 0
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_od_prev_id(&self) -> Option<String> {
        self.inner.od_prev_id.clone()
    }
    #[setter]
    fn set_od_prev_id(&mut self, value: Option<String>) {
        self.inner.od_prev_id = value;
    }

    /// Type of orbit determination method used to produce the orbit estimate.
    ///
    /// Examples: LEAST_SQUARES, KALMAN_FILTER
    ///
    /// :type: str
    #[getter]
    fn get_od_method(&self) -> String {
        self.inner.od_method.clone()
    }
    #[setter]
    fn set_od_method(&mut self, value: String) {
        self.inner.od_method = value;
    }

    /// Relative or absolute time tag of the orbit determination solved-for state in the selected OCM
    /// time system recorded by the TIME_SYSTEM keyword.
    ///
    /// Examples: 2000-01-01T12:00:00Z
    ///
    /// :type: str
    #[getter]
    fn get_od_epoch(&self) -> String {
        self.inner.od_epoch.as_str().to_string()
    }
    #[setter]
    fn set_od_epoch(&mut self, value: String) -> PyResult<()> {
        self.inner.od_epoch = parse_epoch(&value)?;
        Ok(())
    }
    /// Days elapsed between first accepted observation and OD_EPOCH.
    ///
    /// Examples: 1.5
    ///
    /// Units: d
    ///
    /// Days elapsed between first accepted observation and OD_EPOCH. NOTE—May be positive or
    /// negative.
    ///
    /// Units: d
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_days_since_first_obs(&self) -> Option<f64> {
        self.inner.days_since_first_obs.as_ref().map(|d| d.value)
    }
    /// Days elapsed between last accepted observation and OD_EPOCH. NOTE—May be positive or
    /// negative.
    ///
    /// Units: d
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_days_since_last_obs(&self) -> Option<f64> {
        self.inner.days_since_last_obs.as_ref().map(|d| d.value)
    }
    /// Number of days of observations recommended for the OD of the object (useful only for
    /// Batch OD systems).
    ///
    /// Units: d
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_recommended_od_span(&self) -> Option<f64> {
        self.inner.recommended_od_span.as_ref().map(|d| d.value)
    }
    #[setter]
    fn set_recommended_od_span(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::{DayInterval, DayIntervalUnits};
        self.inner.recommended_od_span = value.map(|v| DayInterval {
            value: v,
            units: Some(DayIntervalUnits::D),
        });
    }
    /// Actual time span in days used for the OD of the object. NOTE—Should equal
    /// (DAYS_SINCE_FIRST_OBS - DAYS_SINCE_LAST_OBS).
    ///
    /// Units: d
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_actual_od_span(&self) -> Option<f64> {
        self.inner.actual_od_span.as_ref().map(|d| d.value)
    }
    #[setter]
    fn set_actual_od_span(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::{DayInterval, DayIntervalUnits};
        self.inner.actual_od_span = value.map(|v| DayInterval {
            value: v,
            units: Some(DayIntervalUnits::D),
        });
    }
    /// The number of observations available within the actual OD time span.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_obs_available(&self) -> Option<u64> {
        self.inner.obs_available
    }
    #[setter]
    fn set_obs_available(&mut self, value: Option<u64>) {
        self.inner.obs_available = value;
    }
    /// The number of observations accepted within the actual OD time span.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_obs_used(&self) -> Option<u64> {
        self.inner.obs_used
    }
    #[setter]
    fn set_obs_used(&mut self, value: Option<u64>) {
        self.inner.obs_used = value;
    }
    /// The number of sensor tracks available for the OD within the actual time span (see
    /// definition of 'tracks', 1.5.2).
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_tracks_available(&self) -> Option<u64> {
        self.inner.tracks_available
    }
    #[setter]
    fn set_tracks_available(&mut self, value: Option<u64>) {
        self.inner.tracks_available = value;
    }
    /// The number of sensor tracks accepted for the OD within the actual time span (see
    /// definition of 'tracks', 1.5.2).
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_tracks_used(&self) -> Option<u64> {
        self.inner.tracks_used
    }
    #[setter]
    fn set_tracks_used(&mut self, value: Option<u64>) {
        self.inner.tracks_used = value;
    }
    /// The maximum time between observations in the OD of the object.
    ///
    /// Units: d
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_maximum_obs_gap(&self) -> Option<f64> {
        self.inner.maximum_obs_gap.as_ref().map(|d| d.value)
    }
    #[setter]
    fn set_maximum_obs_gap(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::{DayInterval, DayIntervalUnits};
        self.inner.maximum_obs_gap = value.map(|v| DayInterval {
            value: v,
            units: Some(DayIntervalUnits::D),
        });
    }
    /// Positional error ellipsoid 1σ major eigenvalue at the epoch of the OD.
    ///
    /// Units: m
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_od_epoch_eigmaj(&self) -> Option<f64> {
        self.inner.od_epoch_eigmaj.as_ref().map(|d| d.value)
    }
    #[setter]
    fn set_od_epoch_eigmaj(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Length;
        self.inner.od_epoch_eigmaj = value.map(|v| Length {
            value: v,
            units: None,
        });
    }
    /// Positional error ellipsoid 1σ intermediate eigenvalue at the epoch of the OD.
    ///
    /// Units: m
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_od_epoch_eigint(&self) -> Option<f64> {
        self.inner.od_epoch_eigint.as_ref().map(|d| d.value)
    }
    #[setter]
    fn set_od_epoch_eigint(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Length;
        self.inner.od_epoch_eigint = value.map(|v| Length {
            value: v,
            units: None,
        });
    }
    /// Positional error ellipsoid 1σ minor eigenvalue at the epoch of the OD.
    ///
    /// Units: m
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_od_epoch_eigmin(&self) -> Option<f64> {
        self.inner.od_epoch_eigmin.as_ref().map(|d| d.value)
    }
    #[setter]
    fn set_od_epoch_eigmin(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Length;
        self.inner.od_epoch_eigmin = value.map(|v| Length {
            value: v,
            units: None,
        });
    }
    /// The resulting maximum predicted major eigenvalue of the 1σ positional error ellipsoid
    /// over the entire TIME_SPAN of the OCM, stemming from this OD.
    ///
    /// Units: m
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_od_max_pred_eigmaj(&self) -> Option<f64> {
        self.inner.od_max_pred_eigmaj.as_ref().map(|d| d.value)
    }
    #[setter]
    fn set_od_max_pred_eigmaj(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Length;
        self.inner.od_max_pred_eigmaj = value.map(|v| Length {
            value: v,
            units: None,
        });
    }

    /// The resulting minimum predicted minor eigenvalue of the 1σ positional error ellipsoid
    /// over the entire TIME_SPAN of the OCM, stemming from this OD.
    ///
    /// Units: m
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_od_min_pred_eigmin(&self) -> Option<f64> {
        self.inner.od_min_pred_eigmin.as_ref().map(|d| d.value)
    }
    #[setter]
    fn set_od_min_pred_eigmin(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Length;
        self.inner.od_min_pred_eigmin = value.map(|v| Length {
            value: v,
            units: None,
        });
    }
    /// OD confidence metric, which spans 0 to 100% (useful only for Filter-based OD systems).
    /// The OD confidence metric shall be as mutually defined by message exchange
    /// participants.
    ///
    /// Units: %
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_od_confidence(&self) -> Option<f64> {
        self.inner.od_confidence.as_ref().map(|p| p.value)
    }
    #[setter]
    fn set_od_confidence(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Percentage;
        self.inner.od_confidence = value.map(|v| Percentage {
            value: v,
            units: None,
        });
    }
    /// Generalized Dilution Of Precision for this orbit determination, based on the
    /// observability grammian as defined in references `[H15]` and `[H16]` and expressed in
    /// informative annex F, subsection F4. GDOP provides a rating metric of the observability
    /// of the element set from the OD. Alternate GDOP formations may be used as mutually
    /// defined by message exchange participants.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_gdop(&self) -> Option<f64> {
        self.inner.gdop
    }
    #[setter]
    fn set_gdop(&mut self, value: Option<f64>) {
        self.inner.gdop = value;
    }
    /// The number of solve-for states in the orbit determination.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_solve_n(&self) -> Option<u64> {
        self.inner.solve_n
    }
    #[setter]
    fn set_solve_n(&mut self, value: Option<u64>) {
        self.inner.solve_n = value;
    }
    /// Free-text comma-delimited description of the state elements solved for in the orbit
    /// determination.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_solve_states(&self) -> Option<String> {
        self.inner.solve_states.clone()
    }
    #[setter]
    fn set_solve_states(&mut self, value: Option<String>) {
        self.inner.solve_states = value;
    }
    /// The number of consider parameters used in the orbit determination.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_consider_n(&self) -> Option<u64> {
        self.inner.consider_n
    }
    #[setter]
    fn set_consider_n(&mut self, value: Option<u64>) {
        self.inner.consider_n = value;
    }
    /// Free-text comma-delimited description of the consider parameters used in the orbit
    /// determination.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_consider_params(&self) -> Option<String> {
        self.inner.consider_params.clone()
    }
    #[setter]
    fn set_consider_params(&mut self, value: Option<String>) {
        self.inner.consider_params = value;
    }
    /// The Specific Energy Dissipation Rate, which is the amount of energy being removed from
    /// the object's orbit by the non-conservative forces. This value is an average
    /// calculated during the OD. (See annex F, subsection F7 for definition.)
    ///
    /// Units: W/kg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_sedr(&self) -> Option<f64> {
        self.inner.sedr.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_sedr(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Wkg;
        self.inner.sedr = value.map(Wkg::new);
    }
    /// The number of sensors used in the orbit determination.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_sensors_n(&self) -> Option<u64> {
        self.inner.sensors_n
    }
    #[setter]
    fn set_sensors_n(&mut self, value: Option<u64>) {
        self.inner.sensors_n = value;
    }
    /// Free-text comma-delimited description of the sensors used in the orbit determination.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_sensors(&self) -> Option<String> {
        self.inner.sensors.clone()
    }
    #[setter]
    fn set_sensors(&mut self, value: Option<String>) {
        self.inner.sensors = value;
    }
    /// (Useful/valid only for Batch OD systems.) The weighted RMS residual ratio, defined as:
    /// .. math:: \text{Weighted RMS} = \sqrt{\frac{\sum_{i=1}^{N} w_i(y_i - \hat{y}_i)^2}{N}}
    /// Where yi is the ith observation measurement, ŷi is the current estimate of yi, wi =
    /// 1/σi² is the weight (sigma) associated with the measurement at the ith time and N is
    /// the number of observations. This is a value that can generally identify the quality of
    /// the most recent vector update and is used by the analyst in evaluating the OD process.
    /// A value of 1.00 is ideal.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_weighted_rms(&self) -> Option<f64> {
        self.inner.weighted_rms.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_weighted_rms(&mut self, value: Option<f64>) {
        self.inner.weighted_rms = value.map(|value| ccsds_ndm::types::NonNegativeDouble { value });
    }
    /// Comma-separated list of observation data types utilized in this orbit determination.
    /// Although this is a free-text field, it is recommended at a minimum to use data type
    /// descriptor(s) as provided in table 3-5 of the TDM standard (reference `[9]`) (excluding
    /// the DATA_START, DATA_STOP, and COMMENT keywords). Additional descriptors/detail is
    /// encouraged if the descriptors of table 3-5 are not sufficiently clear; for example, one
    /// could replace ANGLE_1 and ANGLE_2 with RADEC (e.g., from a telescope), AZEL (e.g., from
    /// a ground radar), RANGE (whether from radar or laser ranging), etc.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_data_types(&self) -> Option<String> {
        self.inner.data_types.clone()
    }
    #[setter]
    fn set_data_types(&mut self, value: Option<String>) {
        self.inner.data_types = value;
    }
}
