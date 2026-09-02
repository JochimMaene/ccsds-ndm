// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::parse_time_system;
use crate::common::AdmHeader;
use crate::types::{parse_calendar_epoch, parse_epoch, parse_relative_time};
use ccsds_ndm::messages::acm as core_acm;
use ccsds_ndm::types::{AcmAttitudeType, AcmCovarianceLineType, AttBasisType, AttRateType, RotSeq};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::str::FromStr;

/// Attitude Comprehensive Message (ACM).
///
/// An ACM specifies the attitude state of a single object at multiple epochs, contained within a
/// specified time range. The ACM aggregates and extends APM and AEM content in a single
/// comprehensive hybrid message.
///
/// Capabilities include:
/// - Optional rate data elements
/// - Optional spacecraft physical properties
/// - Optional covariance elements
/// - Optional maneuver parameters
/// - Optional estimator information
#[pyclass]
pub struct Acm {
    id: Option<String>,
    version: String,
    header: Py<AdmHeader>,
    segment: Py<AcmSegment>,
}

impl Acm {
    pub(crate) fn from_core(py: Python<'_>, value: core_acm::Acm) -> PyResult<Self> {
        Ok(Self {
            id: value.id,
            version: value.version,
            header: Py::new(
                py,
                AdmHeader {
                    inner: value.header,
                },
            )?,
            segment: Py::new(py, AcmSegment::from_core(py, *value.body.segment)?)?,
        })
    }

    pub(crate) fn to_core(&self, py: Python<'_>) -> PyResult<core_acm::Acm> {
        Ok(core_acm::Acm {
            id: self.id.clone(),
            version: self.version.clone(),
            header: self.header.borrow(py).inner.clone(),
            body: core_acm::AcmBody {
                segment: Box::new(self.segment.borrow(py).to_core(py)?),
            },
        })
    }
}

#[pymethods]
impl Acm {
    #[new]
    fn new(header: Py<AdmHeader>, segment: Py<AcmSegment>) -> Self {
        Self {
            header,
            segment,
            id: Some("CCSDS_ACM_VERS".to_string()),
            version: "2.0".to_string(),
        }
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!(
            "Acm(object_name='{}')",
            self.segment
                .borrow(py)
                .metadata
                .borrow(py)
                .inner
                .object_name
        )
    }

    /// The message identifier.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    /// The message version.
    ///
    /// :type: str
    #[getter]
    fn get_version(&self) -> String {
        self.version.clone()
    }

    #[setter]
    fn set_version(&mut self, value: String) -> PyResult<()> {
        crate::common::validate_version(ccsds_ndm::validation::MessageKind::Acm, &value)?;
        self.version = value;
        Ok(())
    }

    /// Validate the message against CCSDS rules.
    ///
    fn validate(&self, py: Python<'_>) -> PyResult<()> {
        crate::api::validate_message(&self.to_core(py)?)
    }

    #[staticmethod]
    #[pyo3(signature = (data, format=None, *, max_input_bytes=None, max_records=None))]
    fn from_str(
        py: Python<'_>,
        data: &str,
        format: Option<&str>,
        max_input_bytes: Option<usize>,
        max_records: Option<usize>,
    ) -> PyResult<Self> {
        let options = crate::api::parse_options(max_input_bytes, max_records);
        let inner = crate::api::parse_typed_with_options(data, format, &options)?;
        Self::from_core(py, inner)
    }

    /// Serialize to validated KVN or XML.
    fn to_str(&self, py: Python<'_>, format: &str) -> PyResult<String> {
        crate::api::generate_string(&self.to_core(py)?, format)
    }

    /// Attitude Comprehensive Message (ACM).
    ///
    /// An ACM specifies the attitude state of a single object at multiple epochs, contained within a
    /// specified time range. The ACM aggregates and extends APM and AEM content in a single
    /// comprehensive hybrid message.
    ///
    /// Capabilities include:
    /// - Optional rate data elements
    /// - Optional spacecraft physical properties
    /// - Optional covariance elements
    /// - Optional maneuver parameters
    /// - Optional estimator information
    ///
    /// :type: AdmHeader
    #[getter]
    fn get_header(&self, py: Python<'_>) -> Py<AdmHeader> {
        self.header.clone_ref(py)
    }

    #[setter]
    fn set_header(&mut self, header: Py<AdmHeader>) {
        self.header = header;
    }

    /// ACM Segment.
    ///
    /// :type: AcmSegment
    #[getter]
    fn get_segment(&self, py: Python<'_>) -> Py<AcmSegment> {
        self.segment.clone_ref(py)
    }

    #[setter]
    fn set_segment(&mut self, value: Py<AcmSegment>) {
        self.segment = value;
    }
}

#[pyclass]
pub struct AcmSegment {
    metadata: Py<AcmMetadata>,
    data: Py<AcmData>,
}

impl AcmSegment {
    fn from_core(py: Python<'_>, value: core_acm::AcmSegment) -> PyResult<Self> {
        Ok(Self {
            metadata: Py::new(
                py,
                AcmMetadata {
                    inner: value.metadata,
                },
            )?,
            data: Py::new(py, AcmData::from_core(py, value.data)?)?,
        })
    }

    fn to_core(&self, py: Python<'_>) -> PyResult<core_acm::AcmSegment> {
        Ok(core_acm::AcmSegment {
            metadata: self.metadata.borrow(py).inner.clone(),
            data: self.data.borrow(py).to_core(py)?,
        })
    }
}

#[pymethods]
impl AcmSegment {
    #[new]
    fn new(metadata: Py<AcmMetadata>, data: Py<AcmData>) -> Self {
        Self { metadata, data }
    }

    /// ACM Metadata Section.
    ///
    /// :type: AcmMetadata
    #[getter]
    fn get_metadata(&self, py: Python<'_>) -> Py<AcmMetadata> {
        self.metadata.clone_ref(py)
    }

    #[setter]
    fn set_metadata(&mut self, value: Py<AcmMetadata>) {
        self.metadata = value;
    }

    /// ACM Data Section.
    ///
    /// :type: AcmData
    #[getter]
    fn get_data(&self, py: Python<'_>) -> Py<AcmData> {
        self.data.clone_ref(py)
    }

    #[setter]
    fn set_data(&mut self, value: Py<AcmData>) {
        self.data = value;
    }

    /// Validate the segment against CCSDS rules.
    fn validate(&self, py: Python<'_>, header: AdmHeader) -> PyResult<()> {
        self.to_core(py)?
            .validate(&header.inner)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

/// ACM Metadata Section.
#[pyclass]
#[derive(Clone)]
pub struct AcmMetadata {
    pub inner: core_acm::AcmMetadata,
}

#[pymethods]
impl AcmMetadata {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        object_name,
        epoch_tzero,
        time_system=None,
        international_designator=None,
        comment=None
    ))]
    fn new(
        object_name: String,
        epoch_tzero: String,
        time_system: Option<Bound<'_, PyAny>>,
        international_designator: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let time_system = match time_system {
            Some(ref ob) => parse_time_system(ob)?,
            None => "UTC".to_string(),
        };

        Ok(Self {
            inner: core_acm::AcmMetadata {
                comment: comment.unwrap_or_default(),
                object_name,
                international_designator,
                time_system,
                epoch_tzero: parse_calendar_epoch(&epoch_tzero)?,
                catalog_name: None,
                object_designator: None,
                originator_poc: None,
                originator_position: None,
                originator_phone: None,
                originator_email: None,
                originator_address: None,
                odm_msg_link: None,
                center_name: None,
                acm_data_elements: None,
                start_time: None,
                stop_time: None,
                taimutc_at_tzero: None,
                next_leap_epoch: None,
                next_leap_taimutc: None,
            },
        })
    }

    /// Free-text field containing the name of the object. There is no CCSDS-based restriction on
    /// the value for this keyword, but it is recommended to use names from either the UN Office of
    /// Outer Space Affairs designator index (reference `[2]`), which include Object name and
    /// international designator), the spacecraft operator, or a State Actor or commercial Space
    /// Situational Awareness (SSA) provider maintaining the ‘CATALOG_NAME’ space catalog. If the
    /// object name is not known (uncorrelated object), ‘UNKNOWN’ may be used (or this keyword
    /// omitted).
    ///
    /// Examples: SPOT, ENVISAT, IRIDIUM, INTELSAT
    ///
    /// :type: str
    #[getter]
    fn get_object_name(&self) -> String {
        self.inner.object_name.clone()
    }

    #[setter]
    fn set_object_name(&mut self, value: String) {
        self.inner.object_name = value;
    }

    /// Free text field containing an international designator for the object as assigned by the UN
    /// Committee on Space Research (COSPAR) and the US National Space Science Data Center (NSSDC).
    /// Such designator values have the following COSPAR format: YYYY-NNNP{PP}, where: YYYY = Year
    /// of launch. NNN = Three-digit serial number of launch in year YYYY (with leading zeros).
    /// P{PP} = At least one capital letter for the identification of the part brought into space
    /// by the launch. In cases in which the object has no international designator, the value
    /// UNKNOWN may be used. NOTE – The international designator is typically specified by
    /// ‘OBJECT_ID’ in the APM and AEM.
    ///
    /// Examples: 2000-052A, 1996-068A, 2000-053A, 1996-008A, UNKNOWN
    ///
    /// :type: str | None
    #[getter]
    fn get_international_designator(&self) -> Option<String> {
        self.inner.international_designator.clone()
    }

    #[setter]
    fn set_international_designator(&mut self, value: Option<String>) {
        self.inner.international_designator = value;
    }

    /// Comments (allowed only at the beginning of the ACM Metadata). Each comment line shall begin
    /// with this keyword.
    ///
    /// Examples: This is a comment.
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

    /// Satellite catalog source.
    ///
    /// :type: str | None
    #[getter]
    fn get_catalog_name(&self) -> Option<String> {
        self.inner.catalog_name.clone()
    }

    #[setter]
    fn set_catalog_name(&mut self, value: Option<String>) {
        self.inner.catalog_name = value;
    }

    /// Unique object designator in the source catalog.
    ///
    /// :type: str | None
    #[getter]
    fn get_object_designator(&self) -> Option<String> {
        self.inner.object_designator.clone()
    }

    #[setter]
    fn set_object_designator(&mut self, value: Option<String>) {
        self.inner.object_designator = value;
    }

    /// Originator point-of-contact.
    ///
    /// :type: str | None
    #[getter]
    fn get_originator_poc(&self) -> Option<String> {
        self.inner.originator_poc.clone()
    }

    #[setter]
    fn set_originator_poc(&mut self, value: Option<String>) {
        self.inner.originator_poc = value;
    }

    /// Originator point-of-contact position.
    ///
    /// :type: str | None
    #[getter]
    fn get_originator_position(&self) -> Option<String> {
        self.inner.originator_position.clone()
    }

    #[setter]
    fn set_originator_position(&mut self, value: Option<String>) {
        self.inner.originator_position = value;
    }

    /// Originator point-of-contact phone.
    ///
    /// :type: str | None
    #[getter]
    fn get_originator_phone(&self) -> Option<String> {
        self.inner.originator_phone.clone()
    }

    #[setter]
    fn set_originator_phone(&mut self, value: Option<String>) {
        self.inner.originator_phone = value;
    }

    /// Originator point-of-contact email.
    ///
    /// :type: str | None
    #[getter]
    fn get_originator_email(&self) -> Option<String> {
        self.inner.originator_email.clone()
    }

    #[setter]
    fn set_originator_email(&mut self, value: Option<String>) {
        self.inner.originator_email = value;
    }

    /// Originator point-of-contact address.
    ///
    /// :type: str | None
    #[getter]
    fn get_originator_address(&self) -> Option<String> {
        self.inner.originator_address.clone()
    }

    #[setter]
    fn set_originator_address(&mut self, value: Option<String>) {
        self.inner.originator_address = value;
    }

    /// Linked Orbit Data Message identifier.
    ///
    /// :type: str | None
    #[getter]
    fn get_odm_msg_link(&self) -> Option<String> {
        self.inner.odm_msg_link.clone()
    }

    #[setter]
    fn set_odm_msg_link(&mut self, value: Option<String>) {
        self.inner.odm_msg_link = value;
    }

    /// Central body name.
    ///
    /// :type: str | None
    #[getter]
    fn get_center_name(&self) -> Option<String> {
        self.inner.center_name.clone()
    }

    #[setter]
    fn set_center_name(&mut self, value: Option<String>) {
        self.inner.center_name = value;
    }

    /// Time system used for metadata, attitude data, covariance data. The set of allowed values is
    /// described in annex B, subsection B2.
    ///
    /// Examples: UTC, TAI
    ///
    /// :type: str
    #[getter]
    fn get_time_system(&self) -> String {
        self.inner.time_system.clone()
    }

    #[setter]
    fn set_time_system(&mut self, value: Bound<'_, PyAny>) -> PyResult<()> {
        self.inner.time_system = parse_time_system(&value)?;
        Ok(())
    }

    /// Epoch from which all ACM relative times are referenced. (For format specification, see
    /// 6.8.9.) The time scale for EPOCH_TZERO is the one specified by ‘TIME_SYSTEM’ keyword in the
    /// Metadata section.
    ///
    /// Examples: 2016-11-10T00:00:00
    ///
    /// :type: str
    #[getter]
    fn get_epoch_tzero(&self) -> String {
        self.inner.epoch_tzero.as_str().to_string()
    }

    #[setter]
    fn set_epoch_tzero(&mut self, value: String) -> PyResult<()> {
        self.inner.epoch_tzero = parse_calendar_epoch(&value)?;
        Ok(())
    }

    /// Included ACM data block elements.
    ///
    /// :type: str | None
    #[getter]
    fn get_acm_data_elements(&self) -> Option<String> {
        self.inner.acm_data_elements.clone()
    }

    #[setter]
    fn set_acm_data_elements(&mut self, value: Option<String>) {
        self.inner.acm_data_elements = value;
    }

    /// Earliest data time in this ACM.
    ///
    /// :type: str | None
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

    /// Latest data time in this ACM.
    ///
    /// :type: str | None
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

    /// Difference (TAI - UTC) at EPOCH_TZERO, seconds.
    ///
    /// :type: float | None
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

    /// Epoch of the next leap second.
    ///
    /// :type: str | None
    #[getter]
    fn get_next_leap_epoch(&self) -> Option<String> {
        self.inner
            .next_leap_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }

    #[setter]
    fn set_next_leap_epoch(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.next_leap_epoch = value.map(|s| parse_calendar_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Difference (TAI - UTC) at NEXT_LEAP_EPOCH, seconds.
    ///
    /// :type: float | None
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

    /// Validate the metadata section against CCSDS rules.
    fn validate(&self) -> PyResult<()> {
        self.inner
            .validate()
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

/// ACM Data Section.
#[pyclass]
pub struct AcmData {
    att: Py<PyList>,
    phys: Option<Py<AcmPhysicalDescription>>,
    cov: Py<PyList>,
    man: Py<PyList>,
    ad: Option<Py<AcmAttitudeDetermination>>,
    user: Option<Py<crate::types::UserDefined>>,
}

impl AcmData {
    fn from_core(py: Python<'_>, value: core_acm::AcmData) -> PyResult<Self> {
        macro_rules! py_list {
            ($values:expr, $wrapper:ident) => {{
                let objects = $values
                    .into_iter()
                    .map(|inner| Py::new(py, $wrapper { inner }))
                    .collect::<PyResult<Vec<_>>>()?;
                PyList::new(py, objects)?.unbind()
            }};
        }
        Ok(Self {
            att: py_list!(value.att, AcmAttitudeState),
            phys: value
                .phys
                .map(|inner| Py::new(py, AcmPhysicalDescription { inner }))
                .transpose()?,
            cov: py_list!(value.cov, AcmCovarianceMatrix),
            man: py_list!(value.man, AcmManeuverParameters),
            ad: value
                .ad
                .map(|inner| Py::new(py, AcmAttitudeDetermination::from_core(py, inner)?))
                .transpose()?,
            user: value
                .user
                .map(|inner| Py::new(py, crate::types::UserDefined { inner }))
                .transpose()?,
        })
    }

    fn to_core(&self, py: Python<'_>) -> PyResult<core_acm::AcmData> {
        macro_rules! core_list {
            ($values:expr, $wrapper:ty, $name:literal) => {
                $values
                    .bind(py)
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        value
                            .extract::<PyRef<'_, $wrapper>>()
                            .map(|value| value.inner.clone())
                            .map_err(|_| {
                                PyValueError::new_err(format!(
                                    "{}[{index}] must be {}",
                                    $name,
                                    stringify!($wrapper)
                                ))
                            })
                    })
                    .collect::<PyResult<Vec<_>>>()?
            };
        }
        Ok(core_acm::AcmData {
            att: core_list!(self.att, AcmAttitudeState, "att"),
            phys: self
                .phys
                .as_ref()
                .map(|value| value.borrow(py).inner.clone()),
            cov: core_list!(self.cov, AcmCovarianceMatrix, "cov"),
            man: core_list!(self.man, AcmManeuverParameters, "man"),
            ad: self
                .ad
                .as_ref()
                .map(|value| value.borrow(py).to_core(py))
                .transpose()?,
            user: self
                .user
                .as_ref()
                .map(|value| value.borrow(py).inner.clone()),
        })
    }
}

#[pymethods]
impl AcmData {
    #[new]
    #[pyo3(signature = (att=None, phys=None, cov=None, man=None, ad=None, user=None))]
    fn new(
        py: Python<'_>,
        att: Option<Vec<Py<AcmAttitudeState>>>,
        phys: Option<Py<AcmPhysicalDescription>>,
        cov: Option<Vec<Py<AcmCovarianceMatrix>>>,
        man: Option<Vec<Py<AcmManeuverParameters>>>,
        ad: Option<Py<AcmAttitudeDetermination>>,
        user: Option<Py<crate::types::UserDefined>>,
    ) -> PyResult<Self> {
        Ok(Self {
            att: PyList::new(py, att.unwrap_or_default())?.unbind(),
            phys,
            cov: PyList::new(py, cov.unwrap_or_default())?.unbind(),
            man: PyList::new(py, man.unwrap_or_default())?.unbind(),
            ad,
            user,
        })
    }

    /// A single user-defined Data section.
    ///
    /// :type: UserDefined | None
    #[getter]
    fn get_user(&self, py: Python<'_>) -> Option<Py<crate::types::UserDefined>> {
        self.user.as_ref().map(|value| value.clone_ref(py))
    }

    #[setter]
    fn set_user(&mut self, user: Option<Py<crate::types::UserDefined>>) {
        self.user = user;
    }

    /// One or more optional attitude state time histories (each consisting of one or more attitude
    /// states).
    ///
    /// :type: list[AcmAttitudeState]
    #[getter]
    fn get_att(&self, py: Python<'_>) -> Py<PyList> {
        self.att.clone_ref(py)
    }

    #[setter]
    fn set_att(&mut self, py: Python<'_>, value: Vec<Py<AcmAttitudeState>>) -> PyResult<()> {
        self.att = PyList::new(py, value)?.unbind();
        Ok(())
    }

    /// A single space object physical characteristics section.
    ///
    /// :type: AcmPhysicalDescription | None
    #[getter]
    fn get_phys(&self, py: Python<'_>) -> Option<Py<AcmPhysicalDescription>> {
        self.phys.as_ref().map(|value| value.clone_ref(py))
    }

    #[setter]
    fn set_phys(&mut self, value: Option<Py<AcmPhysicalDescription>>) {
        self.phys = value;
    }

    /// One or more optional covariance time histories (each consisting of one or more covariance
    /// matrix diagonals).
    ///
    /// :type: list[AcmCovarianceMatrix]
    #[getter]
    fn get_cov(&self, py: Python<'_>) -> Py<PyList> {
        self.cov.clone_ref(py)
    }

    #[setter]
    fn set_cov(&mut self, py: Python<'_>, value: Vec<Py<AcmCovarianceMatrix>>) -> PyResult<()> {
        self.cov = PyList::new(py, value)?.unbind();
        Ok(())
    }

    /// One or more optional maneuver specification section(s).
    ///
    /// :type: list[AcmManeuverParameters]
    #[getter]
    fn get_man(&self, py: Python<'_>) -> Py<PyList> {
        self.man.clone_ref(py)
    }

    #[setter]
    fn set_man(&mut self, py: Python<'_>, value: Vec<Py<AcmManeuverParameters>>) -> PyResult<()> {
        self.man = PyList::new(py, value)?.unbind();
        Ok(())
    }

    /// A single attitude determination Data section.
    ///
    /// :type: AcmAttitudeDetermination | None
    #[getter]
    fn get_ad(&self, py: Python<'_>) -> Option<Py<AcmAttitudeDetermination>> {
        self.ad.as_ref().map(|value| value.clone_ref(py))
    }

    #[setter]
    fn set_ad(&mut self, value: Option<Py<AcmAttitudeDetermination>>) {
        self.ad = value;
    }

    /// Validate the data section against CCSDS rules.
    fn validate(&self, py: Python<'_>, metadata: AcmMetadata) -> PyResult<()> {
        self.to_core(py)?
            .validate_with_metadata(&metadata.inner)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

/// ACM Data: Attitude State Time History Section.
#[pyclass]
#[derive(Clone)]
pub struct AcmAttitudeState {
    pub inner: core_acm::AcmAttitudeState,
}

#[pymethods]
impl AcmAttitudeState {
    #[new]
    #[pyo3(signature = (ref_frame_a, ref_frame_b, att_type, att_lines, comment=None))]
    fn new(
        ref_frame_a: String,
        ref_frame_b: String,
        att_type: String,
        att_lines: Vec<Vec<f64>>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let att_type = AcmAttitudeType::from_str(&att_type)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let number_states = att_lines
            .first()
            .and_then(|line| line.len().checked_sub(1))
            .ok_or_else(|| {
                PyValueError::new_err(
                    "att_lines must contain relative time followed by at least one state",
                )
            })?;
        if att_lines.iter().any(|line| line.len() != number_states + 1) {
            return Err(PyValueError::new_err(
                "all att_lines must contain the same number of states",
            ));
        }
        let number_states = u32::try_from(number_states)
            .map_err(|_| PyValueError::new_err("attitude state count exceeds u32"))?;
        Ok(Self {
            inner: core_acm::AcmAttitudeState {
                comment: comment.unwrap_or_default(),
                ref_frame_a,
                ref_frame_b,
                number_states,
                att_type,
                att_lines: att_lines
                    .into_iter()
                    .map(|values| core_acm::AttLine { values })
                    .collect(),
                att_id: None,
                att_prev_id: None,
                att_basis: None,
                att_basis_id: None,
                rate_type: None,
                euler_rot_seq: None,
            },
        })
    }

    /// Comments allowed only immediately after the ATT_START keyword.
    ///
    /// Examples: This is a comment.
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

    /// Attitude state block identifier.
    ///
    /// :type: str | None
    #[getter]
    fn get_att_id(&self) -> Option<String> {
        self.inner.att_id.clone()
    }

    #[setter]
    fn set_att_id(&mut self, value: Option<String>) {
        self.inner.att_id = value;
    }

    /// Previous attitude state block identifier.
    ///
    /// :type: str | None
    #[getter]
    fn get_att_prev_id(&self) -> Option<String> {
        self.inner.att_prev_id.clone()
    }

    #[setter]
    fn set_att_prev_id(&mut self, value: Option<String>) {
        self.inner.att_prev_id = value;
    }

    /// Basis of this attitude state data.
    ///
    /// :type: str | None
    #[getter]
    fn get_att_basis(&self) -> Option<String> {
        self.inner.att_basis.as_ref().map(|v| v.to_string())
    }

    #[setter]
    fn set_att_basis(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.att_basis = value
            .map(|s| AttBasisType::from_str(&s))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Basis dataset identifier.
    ///
    /// :type: str | None
    #[getter]
    fn get_att_basis_id(&self) -> Option<String> {
        self.inner.att_basis_id.clone()
    }

    #[setter]
    fn set_att_basis_id(&mut self, value: Option<String>) {
        self.inner.att_basis_id = value;
    }

    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// Examples: J2000
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame_a(&self) -> String {
        self.inner.ref_frame_a.clone()
    }

    #[setter]
    fn set_ref_frame_a(&mut self, value: String) {
        self.inner.ref_frame_a = value;
    }

    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// Examples: SC_BODY_1
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame_b(&self) -> String {
        self.inner.ref_frame_b.clone()
    }

    #[setter]
    fn set_ref_frame_b(&mut self, value: String) {
        self.inner.ref_frame_b = value;
    }

    /// Number of data states included. States to be included are attitude states and optional rate
    /// states.
    ///
    /// Examples: 3, 4, 7
    ///
    /// :type: int
    #[getter]
    fn get_number_states(&self) -> u32 {
        self.inner.number_states
    }

    #[setter]
    fn set_number_states(&mut self, value: u32) {
        self.inner.number_states = value;
    }

    /// Type of attitude data, selected per annex B, subsection B4. Attitude data must always be
    /// listed before rate data. The units that shall be used are given in annex B, subsection B4.
    ///
    /// Examples: QUATERNION, EULER_ANGLES, DCM
    ///
    /// :type: str
    #[getter]
    fn get_att_type(&self) -> String {
        self.inner.att_type.to_string()
    }

    #[setter]
    fn set_att_type(&mut self, value: String) -> PyResult<()> {
        self.inner.att_type =
            AcmAttitudeType::from_str(&value).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Optional rate state type.
    ///
    /// :type: str | None
    #[getter]
    fn get_rate_type(&self) -> Option<String> {
        self.inner.rate_type.as_ref().map(|v| v.to_string())
    }

    #[setter]
    fn set_rate_type(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.rate_type = value
            .map(|s| AttRateType::from_str(&s))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Optional Euler rotation sequence.
    ///
    /// :type: str | None
    #[getter]
    fn get_euler_rot_seq(&self) -> Option<String> {
        self.inner.euler_rot_seq.as_ref().map(|v| v.to_string())
    }

    #[setter]
    fn set_euler_rot_seq(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.euler_rot_seq = value
            .map(|s| RotSeq::from_str(&s))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Data lines that consist of attitude data followed by rate data. (For the data units, see
    /// above [ATT_TYPE and RATE_TYPE keywords]).
    ///
    /// :type: list[list[float]]
    #[getter]
    fn get_att_lines(&self) -> Vec<Vec<f64>> {
        self.inner
            .att_lines
            .iter()
            .map(|l| l.values.clone())
            .collect()
    }

    #[setter]
    fn set_att_lines(&mut self, value: Vec<Vec<f64>>) {
        self.inner.att_lines = value
            .into_iter()
            .map(|values| core_acm::AttLine { values })
            .collect();
    }
}

/// ACM Data: Space Object Physical Characteristics Section.
#[pyclass]
#[derive(Clone)]
pub struct AcmPhysicalDescription {
    pub inner: core_acm::AcmPhysicalDescription,
}

#[pymethods]
impl AcmPhysicalDescription {
    #[new]
    #[pyo3(signature = (comment=None))]
    fn new(comment: Option<Vec<String>>) -> Self {
        Self {
            inner: core_acm::AcmPhysicalDescription {
                comment: comment.unwrap_or_default(),
                drag_coeff: None,
                wet_mass: None,
                dry_mass: None,
                cp_ref_frame: None,
                cp: None,
                inertia_ref_frame: None,
                ixx: None,
                iyy: None,
                izz: None,
                ixy: None,
                ixz: None,
                iyz: None,
            },
        }
    }

    /// Comments allowed only immediately after the PHYS_START keyword.
    ///
    /// Examples: This is a comment.
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

    /// Drag coefficient.
    ///
    /// :type: float | None
    #[getter]
    fn get_drag_coeff(&self) -> Option<f64> {
        self.inner.drag_coeff
    }

    #[setter]
    fn set_drag_coeff(&mut self, value: Option<f64>) {
        self.inner.drag_coeff = value;
    }

    /// Wet mass (kg).
    ///
    /// :type: float | None
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

    /// Dry mass (kg).
    ///
    /// :type: float | None
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

    /// Center-of-pressure reference frame.
    ///
    /// :type: str | None
    #[getter]
    fn get_cp_ref_frame(&self) -> Option<String> {
        self.inner.cp_ref_frame.clone()
    }

    #[setter]
    fn set_cp_ref_frame(&mut self, value: Option<String>) {
        self.inner.cp_ref_frame = value;
    }

    /// Center-of-pressure vector [x, y, z] in meters.
    ///
    /// :type: list[float] | None
    #[getter]
    fn get_cp(&self) -> Option<Vec<f64>> {
        self.inner.cp.as_ref().map(|v| v.elements.to_vec())
    }

    #[setter]
    fn set_cp(&mut self, value: Option<Vec<f64>>) -> PyResult<()> {
        self.inner.cp = if let Some(v) = value {
            if v.len() != 3 {
                return Err(PyValueError::new_err("cp must have exactly 3 elements"));
            }
            Some(ccsds_ndm::types::Vector3::new([v[0], v[1], v[2]], None))
        } else {
            None
        };
        Ok(())
    }

    /// Inertia reference frame.
    ///
    /// :type: str | None
    #[getter]
    fn get_inertia_ref_frame(&self) -> Option<String> {
        self.inner.inertia_ref_frame.clone()
    }

    #[setter]
    fn set_inertia_ref_frame(&mut self, value: Option<String>) {
        self.inner.inertia_ref_frame = value;
    }

    /// Moment of inertia IXX.
    ///
    /// :type: float | None
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

    /// Moment of inertia IYY.
    ///
    /// :type: float | None
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

    /// Moment of inertia IZZ.
    ///
    /// :type: float | None
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

    /// Product of inertia IXY.
    ///
    /// :type: float | None
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

    /// Product of inertia IXZ.
    ///
    /// :type: float | None
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

    /// Product of inertia IYZ.
    ///
    /// :type: float | None
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

/// ACM Data: Covariance Time History Section.
#[pyclass]
#[derive(Clone)]
pub struct AcmCovarianceMatrix {
    pub inner: core_acm::AcmCovarianceMatrix,
}

#[pymethods]
impl AcmCovarianceMatrix {
    #[new]
    #[pyo3(signature = (cov_basis, cov_ref_frame, cov_type, cov_lines, comment=None))]
    fn new(
        cov_basis: String,
        cov_ref_frame: String,
        cov_type: String,
        cov_lines: Vec<Vec<f64>>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let cov_type = AcmCovarianceLineType::from_str(&cov_type)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self {
            inner: core_acm::AcmCovarianceMatrix {
                comment: comment.unwrap_or_default(),
                cov_id: None,
                cov_prev_id: None,
                cov_basis: Some(
                    AttBasisType::from_str(&cov_basis)
                        .map_err(|e| PyValueError::new_err(e.to_string()))?,
                ),
                cov_basis_id: None,
                cov_ref_frame: Some(cov_ref_frame),
                cov_type,
                cov_lines: cov_lines
                    .into_iter()
                    .map(|values| core_acm::CovLine { values })
                    .collect(),
                cov_confidence: None,
            },
        })
    }

    /// Comments allowed only immediately after the COV_START keyword.
    ///
    /// Examples: THIS is a comment.
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

    /// Covariance history identifier.
    ///
    /// :type: str | None
    #[getter]
    fn get_cov_id(&self) -> Option<String> {
        self.inner.cov_id.clone()
    }

    #[setter]
    fn set_cov_id(&mut self, value: Option<String>) {
        self.inner.cov_id = value;
    }

    /// Previous covariance history identifier.
    ///
    /// :type: str | None
    #[getter]
    fn get_cov_prev_id(&self) -> Option<String> {
        self.inner.cov_prev_id.clone()
    }

    #[setter]
    fn set_cov_prev_id(&mut self, value: Option<String>) {
        self.inner.cov_prev_id = value;
    }

    /// Basis of this covariance time history data.
    ///
    /// Examples: PREDICTED, DETERMINED_GND, DETERMINED_OBC, SIMULATED
    ///
    /// :type: str | None
    #[getter]
    fn get_cov_basis(&self) -> Option<String> {
        self.inner.cov_basis.as_ref().map(ToString::to_string)
    }

    #[setter]
    fn set_cov_basis(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.cov_basis = value
            .map(|value| AttBasisType::from_str(&value))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Identifier for the covariance basis.
    ///
    /// :type: str | None
    #[getter]
    fn get_cov_basis_id(&self) -> Option<String> {
        self.inner.cov_basis_id.clone()
    }

    #[setter]
    fn set_cov_basis_id(&mut self, value: Option<String>) {
        self.inner.cov_basis_id = value;
    }

    /// Reference frame of the covariance time history. The full set of values is enumerated in
    /// annex B, subsection B3.
    ///
    /// Examples: SC_BODY_1
    ///
    /// :type: str | None
    #[getter]
    fn get_cov_ref_frame(&self) -> Option<String> {
        self.inner.cov_ref_frame.clone()
    }

    #[setter]
    fn set_cov_ref_frame(&mut self, value: Option<String>) {
        self.inner.cov_ref_frame = value;
    }

    /// Indicates covariance composition. Select from annex B, subsection B6.
    ///
    /// Examples: ANGLE, ANGLE_GYROBIAS
    ///
    /// :type: str
    #[getter]
    fn get_cov_type(&self) -> String {
        format!("{}", self.inner.cov_type)
    }

    #[setter]
    fn set_cov_type(&mut self, value: String) -> PyResult<()> {
        self.inner.cov_type = AcmCovarianceLineType::from_str(&value)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Optional confidence level of the covariance matrix.
    ///
    /// :type: float | None
    #[getter]
    fn get_cov_confidence(&self) -> Option<f64> {
        self.inner.cov_confidence
    }

    #[setter]
    fn set_cov_confidence(&mut self, value: Option<f64>) {
        self.inner.cov_confidence = value;
    }

    /// Covariance data lines (diagonal terms only). (For the data units, see annex B, subsection
    /// B6.)
    ///
    /// :type: list[list[float]]
    #[getter]
    fn get_cov_lines(&self) -> Vec<Vec<f64>> {
        self.inner
            .cov_lines
            .iter()
            .map(|l| l.values.clone())
            .collect()
    }

    #[setter]
    fn set_cov_lines(&mut self, value: Vec<Vec<f64>>) {
        self.inner.cov_lines = value
            .into_iter()
            .map(|values| core_acm::CovLine { values })
            .collect();
    }
}

/// ACM Data: Maneuver Specification Section.
#[pyclass]
#[derive(Clone)]
pub struct AcmManeuverParameters {
    pub inner: core_acm::AcmManeuverParameters,
}

#[pymethods]
impl AcmManeuverParameters {
    #[new]
    #[pyo3(signature = (man_id=None, comment=None))]
    fn new(man_id: Option<String>, comment: Option<Vec<String>>) -> Self {
        Self {
            inner: core_acm::AcmManeuverParameters {
                comment: comment.unwrap_or_default(),
                man_id,
                man_prev_id: None,
                man_purpose: None,
                man_begin_time: None,
                man_end_time: None,
                man_duration: None,
                actuator_used: None,
                target_momentum: None,
                target_mom_frame: None,
                target_attitude: None,
                target_spinrate: None,
            },
        }
    }

    /// Comments allowed only immediately after the MAN_START keyword.
    ///
    /// Examples: This is a comment.
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

    /// Maneuver block identifier.
    ///
    /// :type: str | None
    #[getter]
    fn get_man_id(&self) -> Option<String> {
        self.inner.man_id.clone()
    }

    #[setter]
    fn set_man_id(&mut self, value: Option<String>) {
        self.inner.man_id = value;
    }

    /// Previous maneuver block identifier.
    ///
    /// :type: str | None
    #[getter]
    fn get_man_prev_id(&self) -> Option<String> {
        self.inner.man_prev_id.clone()
    }

    #[setter]
    fn set_man_prev_id(&mut self, value: Option<String>) {
        self.inner.man_prev_id = value;
    }

    /// Maneuver purpose.
    ///
    /// :type: str | None
    #[getter]
    fn get_man_purpose(&self) -> Option<String> {
        self.inner.man_purpose.clone()
    }

    #[setter]
    fn set_man_purpose(&mut self, value: Option<String>) {
        self.inner.man_purpose = value;
    }

    /// Maneuver begin time in seconds relative to EPOCH_TZERO.
    ///
    /// :type: str | None
    #[getter]
    fn get_man_begin_time(&self) -> Option<String> {
        self.inner
            .man_begin_time
            .as_ref()
            .map(|e| e.as_str().to_string())
    }

    #[setter]
    fn set_man_begin_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.man_begin_time = value.map(|s| parse_relative_time(&s)).transpose()?;
        Ok(())
    }

    /// Maneuver end time in seconds relative to EPOCH_TZERO.
    ///
    /// :type: str | None
    #[getter]
    fn get_man_end_time(&self) -> Option<String> {
        self.inner
            .man_end_time
            .as_ref()
            .map(|e| e.as_str().to_string())
    }

    #[setter]
    fn set_man_end_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.man_end_time = value.map(|s| parse_relative_time(&s)).transpose()?;
        Ok(())
    }

    /// Maneuver duration in seconds.
    ///
    /// :type: float | None
    #[getter]
    fn get_man_duration(&self) -> Option<f64> {
        self.inner.man_duration.as_ref().map(|d| d.value)
    }

    #[setter]
    fn set_man_duration(&mut self, value: Option<f64>) {
        self.inner.man_duration = value.map(|v| ccsds_ndm::types::Duration {
            value: v,
            units: None,
        });
    }

    /// Actuator used for this maneuver.
    ///
    /// :type: str | None
    #[getter]
    fn get_actuator_used(&self) -> Option<String> {
        self.inner.actuator_used.clone()
    }

    #[setter]
    fn set_actuator_used(&mut self, value: Option<String>) {
        self.inner.actuator_used = value;
    }

    /// Target momentum vector [x, y, z].
    ///
    /// :type: list[float] | None
    #[getter]
    fn get_target_momentum(&self) -> Option<Vec<f64>> {
        self.inner
            .target_momentum
            .as_ref()
            .map(|v| v.elements.clone())
    }

    #[setter]
    fn set_target_momentum(&mut self, value: Option<Vec<f64>>) -> PyResult<()> {
        self.inner.target_momentum = if let Some(v) = value {
            if v.len() != 3 {
                return Err(PyValueError::new_err(
                    "target_momentum must have exactly 3 elements",
                ));
            }
            Some(ccsds_ndm::types::TargetMomentum::new(
                [v[0], v[1], v[2]],
                None,
            ))
        } else {
            None
        };
        Ok(())
    }

    /// Reference frame of target momentum.
    ///
    /// :type: str | None
    #[getter]
    fn get_target_mom_frame(&self) -> Option<String> {
        self.inner.target_mom_frame.clone()
    }

    #[setter]
    fn set_target_mom_frame(&mut self, value: Option<String>) {
        self.inner.target_mom_frame = value;
    }

    /// Target attitude quaternion-like 4-vector.
    ///
    /// :type: list[float] | None
    #[getter]
    fn get_target_attitude(&self) -> Option<Vec<f64>> {
        self.inner
            .target_attitude
            .as_ref()
            .map(|v| v.values.clone())
    }

    #[setter]
    fn set_target_attitude(&mut self, value: Option<Vec<f64>>) -> PyResult<()> {
        self.inner.target_attitude = if let Some(v) = value {
            if v.len() != 4 {
                return Err(PyValueError::new_err(
                    "target_attitude must have exactly 4 elements",
                ));
            }
            Some(ccsds_ndm::types::Vec4Double {
                values: vec![v[0], v[1], v[2], v[3]],
            })
        } else {
            None
        };
        Ok(())
    }

    /// Target spin rate (deg/s).
    ///
    /// :type: float | None
    #[getter]
    fn get_target_spinrate(&self) -> Option<f64> {
        self.inner.target_spinrate.as_ref().map(|r| r.value)
    }

    #[setter]
    fn set_target_spinrate(&mut self, value: Option<f64>) {
        self.inner.target_spinrate = value.map(|v| ccsds_ndm::types::AngleRate {
            value: v,
            units: None,
        });
    }
}

/// ACM Data: Sensor Data Section.
#[pyclass]
#[derive(Clone)]
pub struct AcmSensor {
    pub inner: core_acm::AcmSensor,
}

#[pymethods]
impl AcmSensor {
    #[new]
    #[pyo3(signature = (sensor_number=None, sensor_used=None, sensor_noise_stddev=None, sensor_frequency=None, comment=None))]
    fn new(
        sensor_number: Option<u32>,
        sensor_used: Option<String>,
        sensor_noise_stddev: Option<Vec<f64>>,
        sensor_frequency: Option<f64>,
        comment: Option<Vec<String>>,
    ) -> Self {
        Self {
            inner: core_acm::AcmSensor {
                comment: comment.unwrap_or_default(),
                sensor_number,
                sensor_used,
                number_sensor_noise_covariance: None,
                sensor_noise_stddev: sensor_noise_stddev.map(|values| {
                    ccsds_ndm::types::SensorNoise {
                        values,
                        units: None,
                    }
                }),
                sensor_frequency: sensor_frequency
                    .map(|value| ccsds_ndm::types::Frequency { value, units: None }),
            },
        }
    }

    /// Comments allowed only immediately after the SENSOR_START keyword.
    ///
    /// Examples: This is a comment.
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

    /// Sensor number. Multiple sensors may be included, with each having a unique, ascending
    /// number.
    ///
    /// Examples: 1, 2, 3
    ///
    /// :type: int | None
    #[getter]
    fn get_sensor_number(&self) -> Option<u32> {
        self.inner.sensor_number
    }

    #[setter]
    fn set_sensor_number(&mut self, value: Option<u32>) {
        self.inner.sensor_number = value;
    }

    /// Sensor type identifier.
    ///
    /// :type: str | None
    #[getter]
    fn get_sensor_used(&self) -> Option<String> {
        self.inner.sensor_used.clone()
    }

    #[setter]
    fn set_sensor_used(&mut self, value: Option<String>) {
        self.inner.sensor_used = value;
    }

    /// Number of sensor-noise covariance elements.
    ///
    /// :type: int | None
    #[getter]
    fn get_number_sensor_noise_covariance(&self) -> Option<u32> {
        self.inner.number_sensor_noise_covariance
    }

    #[setter]
    fn set_number_sensor_noise_covariance(&mut self, value: Option<u32>) {
        self.inner.number_sensor_noise_covariance = value;
    }

    /// Sensor noise standard deviation values.
    ///
    /// :type: list[float] | None
    #[getter]
    fn get_sensor_noise_stddev(&self) -> Option<Vec<f64>> {
        self.inner
            .sensor_noise_stddev
            .as_ref()
            .map(|v| v.values.clone())
    }

    #[setter]
    fn set_sensor_noise_stddev(&mut self, value: Option<Vec<f64>>) {
        self.inner.sensor_noise_stddev = value.map(|values| ccsds_ndm::types::SensorNoise {
            values,
            units: None,
        });
    }

    /// Sensor frequency in Hz.
    ///
    /// :type: float | None
    #[getter]
    fn get_sensor_frequency(&self) -> Option<f64> {
        self.inner
            .sensor_frequency
            .as_ref()
            .map(|value| value.value)
    }

    #[setter]
    fn set_sensor_frequency(&mut self, value: Option<f64>) {
        self.inner.sensor_frequency =
            value.map(|value| ccsds_ndm::types::Frequency { value, units: None });
    }
}

/// ACM Data: Attitude Determination Data Section.
#[pyclass]
pub struct AcmAttitudeDetermination {
    inner: core_acm::AcmAttitudeDetermination,
    sensors: Py<PyList>,
}

impl AcmAttitudeDetermination {
    fn from_core(py: Python<'_>, mut value: core_acm::AcmAttitudeDetermination) -> PyResult<Self> {
        let sensors = std::mem::take(&mut value.sensors)
            .into_iter()
            .map(|inner| Py::new(py, AcmSensor { inner }))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            inner: value,
            sensors: PyList::new(py, sensors)?.unbind(),
        })
    }

    fn to_core(&self, py: Python<'_>) -> PyResult<core_acm::AcmAttitudeDetermination> {
        let sensors = self
            .sensors
            .bind(py)
            .iter()
            .enumerate()
            .map(|(index, value)| {
                value
                    .extract::<PyRef<'_, AcmSensor>>()
                    .map(|value| value.inner.clone())
                    .map_err(|_| {
                        PyValueError::new_err(format!("sensors[{index}] must be AcmSensor"))
                    })
            })
            .collect::<PyResult<Vec<_>>>()?;
        let mut value = self.inner.clone();
        value.sensors = sensors;
        Ok(value)
    }
}

#[pymethods]
impl AcmAttitudeDetermination {
    #[new]
    #[pyo3(signature = (ad_id=None, comment=None))]
    fn new(py: Python<'_>, ad_id: Option<String>, comment: Option<Vec<String>>) -> PyResult<Self> {
        Ok(Self {
            inner: core_acm::AcmAttitudeDetermination {
                comment: comment.unwrap_or_default(),
                ad_id,
                ad_prev_id: None,
                ad_method: None,
                attitude_source: None,
                number_states: None,
                attitude_states: None,
                euler_rot_seq: None,
                cov_type: None,
                ad_epoch: None,
                ref_frame_a: None,
                ref_frame_b: None,
                attitude_type: None,
                rate_states: None,
                sigma_u: None,
                sigma_v: None,
                rate_process_noise_stddev: None,
                sensors: vec![],
            },
            sensors: PyList::empty(py).unbind(),
        })
    }

    /// Comments allowed only immediately after the AD_START keyword.
    ///
    /// Examples: This is a comment.
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

    /// Attitude determination block identifier.
    ///
    /// :type: str | None
    #[getter]
    fn get_ad_id(&self) -> Option<String> {
        self.inner.ad_id.clone()
    }

    #[setter]
    fn set_ad_id(&mut self, value: Option<String>) {
        self.inner.ad_id = value;
    }

    /// Previous attitude determination block identifier.
    ///
    /// :type: str | None
    #[getter]
    fn get_ad_prev_id(&self) -> Option<String> {
        self.inner.ad_prev_id.clone()
    }

    #[setter]
    fn set_ad_prev_id(&mut self, value: Option<String>) {
        self.inner.ad_prev_id = value;
    }

    /// Attitude determination method.
    ///
    /// :type: str | None
    #[getter]
    fn get_ad_method(&self) -> Option<String> {
        self.inner.ad_method.clone()
    }

    #[setter]
    fn set_ad_method(&mut self, value: Option<String>) {
        self.inner.ad_method = value;
    }

    /// Source of attitude estimate.
    ///
    /// :type: str | None
    #[getter]
    fn get_attitude_source(&self) -> Option<String> {
        self.inner.attitude_source.clone()
    }

    #[setter]
    fn set_attitude_source(&mut self, value: Option<String>) {
        self.inner.attitude_source = value;
    }

    /// Number of estimator states.
    ///
    /// :type: int | None
    #[getter]
    fn get_number_states(&self) -> Option<u32> {
        self.inner.number_states
    }

    #[setter]
    fn set_number_states(&mut self, value: Option<u32>) {
        self.inner.number_states = value;
    }

    /// Attitude state type for estimator.
    ///
    /// :type: str | None
    #[getter]
    fn get_attitude_states(&self) -> Option<String> {
        self.inner.attitude_states.as_ref().map(|v| v.to_string())
    }

    #[setter]
    fn set_attitude_states(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.attitude_states = value
            .map(|s| AcmAttitudeType::from_str(&s))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Euler rotation sequence used by Euler-angle attitude states.
    ///
    /// :type: str | None
    #[getter]
    fn get_euler_rot_seq(&self) -> Option<String> {
        self.inner.euler_rot_seq.as_ref().map(ToString::to_string)
    }

    #[setter]
    fn set_euler_rot_seq(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.euler_rot_seq = value
            .map(|value| RotSeq::from_str(&value))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Covariance type for estimator.
    ///
    /// :type: str | None
    #[getter]
    fn get_cov_type(&self) -> Option<String> {
        self.inner.cov_type.as_ref().map(|v| v.to_string())
    }

    #[setter]
    fn set_cov_type(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.cov_type = value
            .map(|s| AcmCovarianceLineType::from_str(&s))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Epoch of the attitude determination.
    ///
    /// :type: str | None
    #[getter]
    fn get_ad_epoch(&self) -> Option<String> {
        self.inner.ad_epoch.as_ref().map(|e| e.as_str().to_string())
    }

    #[setter]
    fn set_ad_epoch(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.ad_epoch = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Source reference frame.
    ///
    /// :type: str | None
    #[getter]
    fn get_ref_frame_a(&self) -> Option<String> {
        self.inner.ref_frame_a.clone()
    }

    #[setter]
    fn set_ref_frame_a(&mut self, value: Option<String>) {
        self.inner.ref_frame_a = value;
    }

    /// Destination reference frame.
    ///
    /// :type: str | None
    #[getter]
    fn get_ref_frame_b(&self) -> Option<String> {
        self.inner.ref_frame_b.clone()
    }

    #[setter]
    fn set_ref_frame_b(&mut self, value: Option<String>) {
        self.inner.ref_frame_b = value;
    }

    /// Type of attitude data, selected per annex B, subsection B4. Attitude states must always be
    /// listed before rate states.
    ///
    /// Examples: QUATERNION
    ///
    /// :type: str | None
    #[getter]
    fn get_attitude_type(&self) -> Option<String> {
        self.inner.attitude_type.clone()
    }

    #[setter]
    fn set_attitude_type(&mut self, value: Option<String>) {
        self.inner.attitude_type = value;
    }

    /// Rate states type.
    ///
    /// :type: str | None
    #[getter]
    fn get_rate_states(&self) -> Option<String> {
        self.inner.rate_states.as_ref().map(|v| v.to_string())
    }

    #[setter]
    fn set_rate_states(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.rate_states = value
            .map(|s| AttRateType::from_str(&s))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Rate random walk sigma_u.
    ///
    /// :type: float | None
    #[getter]
    fn get_sigma_u(&self) -> Option<f64> {
        self.inner.sigma_u.as_ref().map(|v| v.value)
    }

    #[setter]
    fn set_sigma_u(&mut self, value: Option<f64>) {
        self.inner.sigma_u = value.map(|v| ccsds_ndm::types::SigmaU {
            value: v,
            units: None,
        });
    }

    /// Angle random walk sigma_v.
    ///
    /// :type: float | None
    #[getter]
    fn get_sigma_v(&self) -> Option<f64> {
        self.inner.sigma_v.as_ref().map(|v| v.value)
    }

    #[setter]
    fn set_sigma_v(&mut self, value: Option<f64>) {
        self.inner.sigma_v = value.map(|v| ccsds_ndm::types::SigmaV {
            value: v,
            units: None,
        });
    }

    /// Rate process noise standard deviation.
    ///
    /// :type: float | None
    #[getter]
    fn get_rate_process_noise_stddev(&self) -> Option<f64> {
        self.inner
            .rate_process_noise_stddev
            .as_ref()
            .map(|v| v.value)
    }

    #[setter]
    fn set_rate_process_noise_stddev(&mut self, value: Option<f64>) {
        self.inner.rate_process_noise_stddev = value.map(|v| ccsds_ndm::types::SigmaU {
            value: v,
            units: None,
        });
    }

    /// Sensor data blocks.
    ///
    /// :type: list[AcmSensor]
    #[getter]
    fn get_sensors(&self, py: Python<'_>) -> Py<PyList> {
        self.sensors.clone_ref(py)
    }

    #[setter]
    fn set_sensors(&mut self, py: Python<'_>, value: Vec<Py<AcmSensor>>) -> PyResult<()> {
        self.sensors = PyList::new(py, value)?.unbind();
        Ok(())
    }
}
