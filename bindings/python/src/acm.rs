// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::AdmHeader;
use crate::types::parse_epoch;
use ccsds_ndm::messages::acm as core_acm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::MessageType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;
use crate::common::{parse_time_system};


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
#[derive(Clone)]
pub struct Acm {
    pub inner: core_acm::Acm,
}

#[pymethods]
impl Acm {
    #[new]
    fn new(header: AdmHeader, segment: AcmSegment) -> Self {
        Self {
            inner: core_acm::Acm {
                header: header.inner,
                body: core_acm::AcmBody {
                    segment: Box::new(segment.inner),
                },
                id: None,
                version: "2.0".to_string(),
            },
        }
    }

    #[staticmethod]
    fn from_str(data: &str, format: Option<&str>) -> PyResult<Self> {
        let inner = match format {
            Some("kvn") => ccsds_ndm::messages::acm::Acm::from_kvn(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some("xml") => ccsds_ndm::messages::acm::Acm::from_xml(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some(other) => {
                return Err(PyValueError::new_err(format!(
                    "Unsupported format '{}'. Use 'kvn' or 'xml'",
                    other
                )))
            }
            None => match ccsds_ndm::from_str(data) {
                Ok(MessageType::Acm(acm)) => acm,
                Ok(other) => {
                    return Err(PyValueError::new_err(format!(
                        "Parsed message is not ACM (got {:?})",
                        other
                    )))
                }
                Err(e) => return Err(PyValueError::new_err(e.to_string())),
            },
        };
        Ok(Self { inner })
    }

    #[staticmethod]
    fn from_file(path: &str, format: Option<&str>) -> PyResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| PyValueError::new_err(format!("Failed to read file: {}", e)))?;
        Self::from_str(&content, format)
    }

    fn to_str(&self, format: &str) -> PyResult<String> {
        match format {
            "kvn" => self.inner.to_kvn().map_err(|e| PyValueError::new_err(e.to_string())),
            "xml" => self.inner.to_xml().map_err(|e| PyValueError::new_err(e.to_string())),
            other => Err(PyValueError::new_err(format!("Unsupported format '{}'", other))),
        }
    }

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
    fn get_header(&self) -> AdmHeader {
        AdmHeader {
            inner: self.inner.header.clone(),
        }
    }

    /// ACM Segment.
    ///
    /// :type: AcmSegment
    #[getter]
    fn get_segment(&self) -> AcmSegment {
        AcmSegment {
            inner: (*self.inner.body.segment).clone(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct AcmSegment {
    pub inner: core_acm::AcmSegment,
}

#[pymethods]
impl AcmSegment {
    #[new]
    fn new(metadata: AcmMetadata, data: AcmData) -> Self {
        Self {
            inner: core_acm::AcmSegment {
                metadata: metadata.inner,
                data: data.inner,
            },
        }
    }

    /// ACM Metadata Section.
    ///
    /// :type: AcmMetadata
    #[getter]
    fn get_metadata(&self) -> AcmMetadata {
        AcmMetadata {
            inner: self.inner.metadata.clone(),
        }
    }

    /// ACM Data Section.
    ///
    /// :type: AcmData
    #[getter]
    fn get_data(&self) -> AcmData {
        AcmData {
            inner: self.inner.data.clone(),
        }
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
                epoch_tzero: parse_epoch(&epoch_tzero)?,
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
}

/// ACM Data Section.
#[pyclass]
#[derive(Clone)]
pub struct AcmData {
    pub inner: core_acm::AcmData,
}

#[pymethods]
impl AcmData {
    #[new]
    fn new(
        att: Option<Vec<AcmAttitudeState>>,
        phys: Option<AcmPhysicalDescription>,
        cov: Option<Vec<AcmCovarianceMatrix>>,
        man: Option<Vec<AcmManeuverParameters>>,
        ad: Option<AcmAttitudeDetermination>,
        user: Option<crate::types::UserDefined>,
    ) -> Self {
        Self {
            inner: core_acm::AcmData {
                att: att.unwrap_or_default().into_iter().map(|s| s.inner).collect(),
                phys: phys.map(|p| p.inner),
                cov: cov.unwrap_or_default().into_iter().map(|c| c.inner).collect(),
                man: man.unwrap_or_default().into_iter().map(|m| m.inner).collect(),
                ad: ad.map(|a| a.inner),
                user: user.map(|u| u.inner),
            },
        }
    }

    /// A single user-defined Data section.
    ///
    /// :type: UserDefined | None
    #[getter]
    fn get_user(&self) -> Option<crate::types::UserDefined> {
        self.inner.user.as_ref().map(|u| crate::types::UserDefined { inner: u.clone() })
    }

    #[setter]
    fn set_user(&mut self, user: Option<crate::types::UserDefined>) {
        self.inner.user = user.map(|u| u.inner);
    }

    /// One or more optional attitude state time histories (each consisting of one or more attitude
    /// states).
    ///
    /// :type: list[AcmAttitudeState]
    #[getter]
    fn get_att(&self) -> Vec<AcmAttitudeState> {
        self.inner.att.iter().map(|s| AcmAttitudeState { inner: s.clone() }).collect()
    }

    /// A single space object physical characteristics section.
    ///
    /// :type: AcmPhysicalDescription | None
    #[getter]
    fn get_phys(&self) -> Option<AcmPhysicalDescription> {
        self.inner.phys.as_ref().map(|p| AcmPhysicalDescription { inner: p.clone() })
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
    fn new(
        ref_frame_a: String,
        ref_frame_b: String,
        att_type: String,
        att_lines: Vec<Vec<f64>>,
        comment: Option<Vec<String>>,
    ) -> Self {
        Self {
            inner: core_acm::AcmAttitudeState {
                comment: comment.unwrap_or_default(),
                ref_frame_a,
                ref_frame_b,
                number_states: att_lines.len() as u32,
                att_type,
                att_lines: att_lines.into_iter().map(|values| core_acm::AttLine { values }).collect(),
                att_id: None,
                att_prev_id: None,
                att_basis: None,
                att_basis_id: None,
                rate_type: None,
                euler_rot_seq: None,
            },
        }
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
    fn new(
        cov_basis: String,
        cov_ref_frame: String,
        cov_type: String,
        cov_lines: Vec<Vec<f64>>,
        comment: Option<Vec<String>>,
    ) -> Self {
        Self {
            inner: core_acm::AcmCovarianceMatrix {
                comment: comment.unwrap_or_default(),
                cov_basis,
                cov_ref_frame,
                cov_type,
                cov_lines: cov_lines.into_iter().map(|values| core_acm::CovLine { values }).collect(),
                cov_confidence: None,
            },
        }
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
            },
        }
    }
}

/// ACM Data: Attitude Determination Data Section.
#[pyclass]
#[derive(Clone)]
pub struct AcmAttitudeDetermination {
    pub inner: core_acm::AcmAttitudeDetermination,
}

#[pymethods]
impl AcmAttitudeDetermination {
    #[new]
    fn new(ad_id: Option<String>, comment: Option<Vec<String>>) -> Self {
        Self {
            inner: core_acm::AcmAttitudeDetermination {
                comment: comment.unwrap_or_default(),
                ad_id,
                ad_prev_id: None,
                ad_method: None,
                attitude_source: None,
                number_states: None,
                attitude_states: None,
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
        }
    }
}
