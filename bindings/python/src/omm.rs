// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::OdmHeader;
use crate::types::parse_epoch;
use ccsds_ndm::messages::omm as core_omm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::{Angle, Distance, Gm, Inclination};
use ccsds_ndm::MessageType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;
use crate::common::{parse_reference_frame, parse_time_system};


// Import OpmCovarianceMatrix from opm module (shared type)
use crate::opm::OpmCovarianceMatrix;

/// Orbit Mean-Elements Message (OMM).
///
/// The OMM contains the orbital characteristics of a single object at a specified epoch,
/// expressed in mean Keplerian elements: mean motion, eccentricity, inclination, right
/// ascension of ascending node, argument of perigee, and mean anomaly.
///
/// These elements are adequate for providing the initial mean state of analytical and
/// semi-analytical orbit models (e.g., SGP4). The OMM includes keywords and values that may
/// be used to generate canonical NORAD Two Line Element (TLE) sets to accommodate the needs
/// of heritage users.
///
/// Parameters
/// ----------
/// header : OdmHeader
///     The message header.
/// segment : OmmSegment
///     The data segment.
#[pyclass]
#[derive(Clone)]
pub struct Omm {
    pub inner: core_omm::Omm,
}

#[pymethods]
impl Omm {
    #[new]
    fn new(header: OdmHeader, segment: OmmSegment) -> Self {
        Self {
            inner: core_omm::Omm {
                header: header.inner,
                body: core_omm::OmmBody {
                    segment: segment.inner,
                },
                id: Some("CCSDS_OMM_VERS".to_string()),
                version: "2.0".to_string(),
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Omm(object_name='{}')",
            self.inner.body.segment.metadata.object_name
        )
    }

    /// Orbit Mean-Elements Message (OMM).
    ///
    /// The OMM contains the orbital characteristics of a single object at a specified epoch,
    /// expressed in mean Keplerian elements: mean motion, eccentricity, inclination, right
    /// ascension of ascending node, argument of perigee, and mean anomaly.
    ///
    /// These elements are adequate for providing the initial mean state of analytical and
    /// semi-analytical orbit models (e.g., SGP4). The OMM includes keywords and values that may
    /// be used to generate canonical NORAD Two Line Element (TLE) sets to accommodate the needs
    /// of heritage users.
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

    /// The data segment.
    ///
    /// :type: OmmSegment
    #[getter]
    fn get_segment(&self) -> OmmSegment {
        OmmSegment {
            inner: self.inner.body.segment.clone(),
        }
    }

    #[setter]
    fn set_segment(&mut self, segment: OmmSegment) {
        self.inner.body.segment = segment.inner;
    }

    #[staticmethod]
    fn from_str(data: &str, format: Option<&str>) -> PyResult<Self> {
        let inner = match format {
            Some("kvn") => ccsds_ndm::messages::omm::Omm::from_kvn(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some("xml") => ccsds_ndm::messages::omm::Omm::from_xml(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some(other) => {
                return Err(PyValueError::new_err(format!(
                    "Unsupported format '{}'. Use 'kvn' or 'xml'",
                    other
                )))
            }
            None => match ccsds_ndm::from_str(data) {
                Ok(MessageType::Omm(omm)) => omm,
                Ok(other) => {
                    return Err(PyValueError::new_err(format!(
                        "Parsed message is not OMM (got {:?})",
                        other
                    )))
                }
                Err(e) => return Err(PyValueError::new_err(e.to_string())),
            },
        };
        Ok(Self { inner })
    }

    /// Create an OMM message from a file.
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
    /// Omm
    ///     The parsed OMM object.
    #[staticmethod]
    fn from_file(path: &str, format: Option<&str>) -> PyResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| PyValueError::new_err(format!("Failed to read file: {}", e)))?;
        Self::from_str(&content, format)
    }

    /// Serialize to string.
    ///
    /// Parameters
    /// ----------
    /// format : str
    ///     Output format ('kvn' or 'xml').
    ///     (Mandatory)
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

/// Create a new OMM Segment.
///
/// Parameters
/// ----------
/// metadata : OmmMetadata
///     Segment metadata.
/// data : OmmData
///     Segment data.
#[pyclass]
#[derive(Clone)]
pub struct OmmSegment {
    pub inner: core_omm::OmmSegment,
}

#[pymethods]
impl OmmSegment {
    #[new]
    fn new(metadata: OmmMetadata, data: OmmData) -> Self {
        Self {
            inner: core_omm::OmmSegment {
                metadata: metadata.inner,
                data: data.inner,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "OmmSegment(object_name='{}')",
            self.inner.metadata.object_name
        )
    }

    /// Segment metadata.
    ///
    /// :type: OmmMetadata
    #[getter]
    fn get_metadata(&self) -> OmmMetadata {
        OmmMetadata {
            inner: self.inner.metadata.clone(),
        }
    }

    #[setter]
    fn set_metadata(&mut self, metadata: OmmMetadata) {
        self.inner.metadata = metadata.inner;
    }

    /// Segment data.
    ///
    /// :type: OmmData
    #[getter]
    fn get_data(&self) -> OmmData {
        OmmData {
            inner: self.inner.data.clone(),
        }
    }

    #[setter]
    fn set_data(&mut self, data: OmmData) {
        self.inner.data = data.inner;
    }
}

/// Metadata for the OMM.
///
/// Parameters
/// ----------
/// object_name : str
///     Spacecraft name for which mean element orbit state data is provided.
/// object_id : str
///     Object identifier of the object for which mean element orbit state data is provided.
/// center_name : str
///     Origin of the OMM reference frame.
/// ref_frame : str
///     Reference frame in which the Keplerian element data are given.
/// time_system : str
///     Time system used for Keplerian elements and covariance data.
/// mean_element_theory : str
///     Description of the Mean Element Theory. Indicates the proper method to employ to propagate the state.
/// ref_frame_epoch : str, optional
///     Epoch of reference frame, if not intrinsic to the definition of the reference frame.
/// comment : list of str, optional
///     Comments (allowed at the beginning of the OMM Metadata).
#[pyclass]
#[derive(Clone)]
pub struct OmmMetadata {
    pub inner: core_omm::OmmMetadata,
}

#[pymethods]
impl OmmMetadata {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        object_name,
        object_id,
        center_name=String::from("EARTH"),
        ref_frame=None,
        time_system=None,
        mean_element_theory=String::from("SGP4"),
        ref_frame_epoch=None,
        comment=None
    ))]
    fn new(
        object_name: String,
        object_id: String,
        center_name: String,
        ref_frame: Option<Bound<'_, PyAny>>,
        time_system: Option<Bound<'_, PyAny>>,
        mean_element_theory: String,
        ref_frame_epoch: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let ref_frame = match ref_frame {
            Some(ref ob) => parse_reference_frame(ob)?,
            None => "TEME".to_string(),
        };
        let time_system = match time_system {
            Some(ref ob) => parse_time_system(ob)?,
            None => "UTC".to_string(),
        };

        Ok(Self {
            inner: core_omm::OmmMetadata {
                object_name,
                object_id,
                center_name,
                ref_frame,
                time_system,
                mean_element_theory,
                ref_frame_epoch: ref_frame_epoch.map(|s| parse_epoch(&s)).transpose()?,
                comment: comment.unwrap_or_default(),
            },
        })
    }



    fn __repr__(&self) -> String {
        format!("OmmMetadata(object_name='{}')", self.inner.object_name)
    }

    /// Spacecraft name for which mean element orbit state data is provided. While there is no
    /// CCSDS-based restriction on the value for this keyword, it is recommended to use names
    /// from the UN Office of Outer Space Affairs designator index (reference `[3]`, which include
    /// Object name and international designator of the participant). If OBJECT_NAME is not
    /// listed in reference `[3]` or the content is either unknown or cannot be disclosed, the
    /// value should be set to UNKNOWN.
    ///
    /// Examples: Telkom 2, Spaceway 2, INMARSAT 4-F2, UNKNOWN
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

    /// Object identifier of the object for which mean element orbit state data is provided.
    /// While there is no CCSDS-based restriction on the value for this keyword, it is
    /// recommended to use the international spacecraft designator as published in the UN Office
    /// of Outer Space Affairs designator index (reference `[3]`). Recommended values have the
    /// format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three-digit serial number of
    /// launch in year YYYY (with leading zeros). P{PP} = At least one capital letter for the
    /// identification of the part brought into space by the launch. If the asset is not listed
    /// in reference `[3]`, the UN Office of Outer Space Affairs designator index format is not
    /// used, or the content is either unknown or cannot be disclosed, the value should be set
    /// to UNKNOWN.
    ///
    /// Examples: 2005-046A, 2005-046B, 2003-022A, UNKNOWN
    ///
    /// :type: str
    #[getter]
    fn get_object_id(&self) -> String {
        self.inner.object_id.clone()
    }

    #[setter]
    fn set_object_id(&mut self, value: String) {
        self.inner.object_id = value;
    }

    /// Origin of the OMM reference frame, which shall be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter. Natural bodies shall be selected from the accepted set of values
    /// indicated in annex B, subsection B2.
    ///
    /// Examples: EARTH, MARS, MOON
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

    /// Reference frame in which the Keplerian element data are given. Use of values other than
    /// those in 3.2.3.3 should be documented in an ICD. NOTE—NORAD Two Line Element Sets and
    /// corresponding Simplified General Perturbations (SGP) orbit propagator ephemeris outputs
    /// are explicitly defined to be in the True Equator Mean Equinox of Date (TEME of Date)
    /// reference frame. Therefore, TEME of date shall be used for OMMs based on NORAD Two Line
    /// Element sets, rather than the almost imperceptibly different TEME of Epoch (see
    /// reference `[H2]` or `[H3]` for further details).
    ///
    /// Examples: ICRF, ITRF2000, EME2000, TEME
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame(&self) -> String {
        self.inner.ref_frame.clone()
    }

    #[setter]
    fn set_ref_frame(&mut self, value: String) {
        self.inner.ref_frame = value;
    }

    /// Time system used for Keplerian elements and covariance data. Use of values other than
    /// those in 3.2.3.2 should be documented in an ICD.
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

    /// Description of the Mean Element Theory. Indicates the proper method to employ to
    /// propagate the state.
    ///
    /// Examples: SGP, SGP4, SGP4-XP, DSST, USM
    ///
    /// :type: str
    #[getter]
    fn get_mean_element_theory(&self) -> String {
        self.inner.mean_element_theory.clone()
    }

    #[setter]
    fn set_mean_element_theory(&mut self, value: String) {
        self.inner.mean_element_theory = value;
    }

    /// Epoch of reference frame, if not intrinsic to the definition of the reference frame.
    /// (See 7.5.10 for formatting rules.)
    ///
    /// Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_ref_frame_epoch(&self) -> Option<String> {
        self.inner
            .ref_frame_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }

    #[setter]
    fn set_ref_frame_epoch(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.ref_frame_epoch = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Comments (allowed at the beginning of the OMM Metadata). (See 7.8 for formatting rules.)
    ///
    /// Examples: This is a comment
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

/// Mean Keplerian Elements in the Specified Reference Frame.
///
/// Parameters
/// ----------
/// epoch : str
///     Epoch of the mean elements.
/// eccentricity : float
///     Eccentricity.
/// inclination : float
///     Inclination (deg).
/// ra_of_asc_node : float
///     Right ascension of the ascending node (deg).
/// arg_of_pericenter : float
///     Argument of pericenter (deg).
/// mean_anomaly : float
///     Mean anomaly (deg).
/// semi_major_axis : float, optional
///     Semi-major axis in kilometers. Preferred over MEAN_MOTION.
/// mean_motion : float, optional
///     Keplerian Mean motion in revolutions per day. Required if MEAN_ELEMENT_THEORY = SGP/SGP4.
/// gm : float, optional
///     Gravitational Coefficient (Gravitational Constant × Central Mass) in km³/s².
#[pyclass]
#[derive(Clone)]
pub struct MeanElements {
    pub inner: core_omm::MeanElements,
}

#[pymethods]
impl MeanElements {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        epoch: String,
        eccentricity: f64,
        inclination: f64,
        ra_of_asc_node: f64,
        arg_of_pericenter: f64,
        mean_anomaly: f64,
        semi_major_axis: Option<f64>,
        mean_motion: Option<f64>,
        gm: Option<f64>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_omm::MeanElements {
                comment: vec![],
                epoch: parse_epoch(&epoch)?,
                eccentricity: ccsds_ndm::types::NonNegativeDouble { value: eccentricity },
                inclination: Inclination {
                    angle: Angle::new(inclination, None).map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
                    })?,
                },
                ra_of_asc_node: Angle::new(ra_of_asc_node, None)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
                arg_of_pericenter: Angle::new(arg_of_pericenter, None)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
                mean_anomaly: Angle::new(mean_anomaly, None)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
                semi_major_axis: semi_major_axis.map(|v| Distance::new(v, None)),
                mean_motion: mean_motion.map(|v| core_omm::MeanMotion::new(v, None)),
                gm: gm
                    .map(|v| Gm::new(v, None))
                    .transpose()
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            },
        })
    }

    fn __repr__(&self) -> String {
        format!("MeanElements(epoch='{}')", self.inner.epoch.as_str())
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

    /// Epoch of Mean Keplerian elements (see 7.5.10 for formatting rules)
    ///
    /// Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// :type: str
    #[getter]
    fn get_epoch(&self) -> String {
        self.inner.epoch.as_str().to_string()
    }

    #[setter]
    fn set_epoch(&mut self, value: String) -> PyResult<()> {
        self.inner.epoch = parse_epoch(&value)?;
        Ok(())
    }

    /// Eccentricity
    ///
    /// Examples: 0.7303
    ///
    /// Units: n/a
    ///
    /// :type: float
    #[getter]
    fn get_eccentricity(&self) -> f64 {
        self.inner.eccentricity.value
    }

    #[setter]
    fn set_eccentricity(&mut self, value: f64) {
        self.inner.eccentricity = ccsds_ndm::types::NonNegativeDouble { value };
    }

    /// Inclination
    ///
    /// Examples: 63.4
    ///
    /// Units: deg
    ///
    /// :type: float
    #[getter]
    fn get_inclination(&self) -> f64 {
        self.inner.inclination.angle.value
    }

    #[setter]
    fn set_inclination(&mut self, value: f64) -> PyResult<()> {
        self.inner.inclination = Inclination {
            angle: Angle::new(value, None)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
        };
        Ok(())
    }

    /// Right ascension of ascending node
    ///
    /// Examples: 345.0
    ///
    /// Units: deg
    ///
    /// :type: float
    #[getter]
    fn get_ra_of_asc_node(&self) -> f64 {
        self.inner.ra_of_asc_node.value
    }

    #[setter]
    fn set_ra_of_asc_node(&mut self, value: f64) -> PyResult<()> {
        self.inner.ra_of_asc_node = Angle::new(value, None)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(())
    }

    /// Argument of pericenter
    ///
    /// Examples: 270.0
    ///
    /// Units: deg
    ///
    /// :type: float
    #[getter]
    fn get_arg_of_pericenter(&self) -> f64 {
        self.inner.arg_of_pericenter.value
    }

    #[setter]
    fn set_arg_of_pericenter(&mut self, value: f64) -> PyResult<()> {
        self.inner.arg_of_pericenter = Angle::new(value, None)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(())
    }

    /// Mean anomaly
    ///
    /// Examples: 130.0
    ///
    /// Units: deg
    ///
    /// :type: float
    #[getter]
    fn get_mean_anomaly(&self) -> f64 {
        self.inner.mean_anomaly.value
    }

    #[setter]
    fn set_mean_anomaly(&mut self, value: f64) -> PyResult<()> {
        self.inner.mean_anomaly = Angle::new(value, None)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(())
    }

    /// Semi-major axis in kilometers (preferred), or, if MEAN_ELEMENT_THEORY = SGP/SGP4, the
    /// Keplerian Mean motion in revolutions per day
    ///
    /// Examples: 28594.4
    ///
    /// Units: km or rev/day
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_semi_major_axis(&self) -> Option<f64> {
        self.inner.semi_major_axis.as_ref().map(|v| v.value)
    }

    #[setter]
    fn set_semi_major_axis(&mut self, value: Option<f64>) {
        self.inner.semi_major_axis = value.map(|v| Distance::new(v, None));
    }

    /// Semi-major axis in kilometers (preferred), or, if MEAN_ELEMENT_THEORY = SGP/SGP4, the
    /// Keplerian Mean motion in revolutions per day
    ///
    /// Examples: 1.491325
    ///
    /// Units: km or rev/day
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_mean_motion(&self) -> Option<f64> {
        self.inner.mean_motion.as_ref().map(|v| v.value)
    }

    #[setter]
    fn set_mean_motion(&mut self, value: Option<f64>) {
        self.inner.mean_motion = value.map(|v| core_omm::MeanMotion::new(v, None));
    }

    /// Gravitational Coefficient (Gravitational Constant × Central Mass)
    ///
    /// Examples: 398600.44
    ///
    /// Units: km³/s²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_gm(&self) -> Option<f64> {
        self.inner.gm.as_ref().map(|v| v.value)
    }

    #[setter]
    fn set_gm(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.gm = value
            .map(|v| Gm::new(v, None))
            .transpose()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(())
    }
}

/// OMM Data section.
#[pyclass]
#[derive(Clone)]
pub struct OmmData {
    pub inner: core_omm::OmmData,
}

#[pymethods]
impl OmmData {
    /// Create a new OMM Data object.
    ///
    /// Parameters
    /// ----------
    /// mean_elements : MeanElements
    ///     Mean elements.
    #[new]
    fn new(mean_elements: MeanElements, comments: Option<Vec<String>>) -> Self {
        Self {
            inner: core_omm::OmmData {
                comment: comments.unwrap_or_default(),
                mean_elements: mean_elements.inner,
                spacecraft_parameters: None,
                tle_parameters: None,
                covariance_matrix: None,
                user_defined_parameters: None,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "OmmData(epoch='{}')",
            self.inner.mean_elements.epoch.as_str()
        )
    }

    /// Mean Keplerian Elements in the Specified Reference Frame.
    ///
    /// :type: MeanElements
    #[getter]
    fn get_mean_elements(&self) -> MeanElements {
        MeanElements {
            inner: self.inner.mean_elements.clone(),
        }
    }

    #[setter]
    fn set_mean_elements(&mut self, value: MeanElements) {
        self.inner.mean_elements = value.inner;
    }

    /// Comments.
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

    /// Spacecraft Parameters.
    ///
    /// :type: Optional[SpacecraftParameters]
    #[getter]
    fn get_spacecraft_parameters(&self) -> Option<crate::common::SpacecraftParameters> {
        self.inner
            .spacecraft_parameters
            .as_ref()
            .map(|s| crate::common::SpacecraftParameters { inner: s.clone() })
    }

    #[setter]
    fn set_spacecraft_parameters(&mut self, value: Option<crate::common::SpacecraftParameters>) {
        self.inner.spacecraft_parameters = value.map(|s| s.inner);
    }

    /// TLE Related Parameters (Only required if MEAN_ELEMENT_THEORY=SGP/SGP4).
    ///
    /// :type: Optional[TleParameters]
    #[getter]
    fn get_tle_parameters(&self) -> Option<TleParameters> {
        self.inner
            .tle_parameters
            .as_ref()
            .map(|t| TleParameters { inner: t.clone() })
    }

    #[setter]
    fn set_tle_parameters(&mut self, value: Option<TleParameters>) {
        self.inner.tle_parameters = value.map(|t| t.inner);
    }

    /// Position/Velocity Covariance Matrix (6x6 Lower Triangular Form).
    ///
    /// :type: Optional[OpmCovarianceMatrix]
    #[getter]
    fn get_covariance_matrix(&self) -> Option<OpmCovarianceMatrix> {
        self.inner
            .covariance_matrix
            .as_ref()
            .map(|c| OpmCovarianceMatrix { inner: c.clone() })
    }

    #[setter]
    fn set_covariance_matrix(&mut self, value: Option<OpmCovarianceMatrix>) {
        self.inner.covariance_matrix = value.map(|c| c.inner);
    }

    /// User-Defined Parameters.
    ///
    /// :type: UserDefined | None
    #[getter]
    fn get_user_defined_parameters(&self) -> Option<crate::types::UserDefined> {
        self.inner
            .user_defined_parameters
            .as_ref()
            .map(|u| crate::types::UserDefined { inner: u.clone() })
    }

    #[setter]
    fn set_user_defined_parameters(&mut self, value: Option<crate::types::UserDefined>) {
        self.inner.user_defined_parameters = value.map(|u| u.inner);
    }
}

/// TLE Related Parameters (This section is only required if MEAN_ELEMENT_THEORY=SGP/SGP4).
///
/// Parameters
/// ----------
/// ephemeris_type : int, optional
///     Ephemeris Type, default value = 0.
/// classification_type : str, optional
///     Classification Type, default value = U.
/// norad_cat_id : int, optional
///     NORAD Catalog Number ('Satellite Number').
/// element_set_no : int, optional
///     Element set number for this satellite.
/// rev_at_epoch : int, optional
///     Revolution Number.
/// bstar : float, optional
///     B* drag term in 1/ER (Inverse Earth Radii). Required for SGP4.
/// bterm : float, optional
///     Ballistic coefficient (m²/kg). Required for SGP4-XP.
/// mean_motion_dot : float, optional
///     First derivative of mean motion (rev/day²). Required when MEAN_ELEMENT_THEORY = SGP or PPT3.
/// mean_motion_ddot : float, optional
///     Second derivative of mean motion (rev/day³). Required when MEAN_ELEMENT_THEORY = SGP or PPT3.
/// agom : float, optional
///     Solar radiation pressure coefficient (m²/kg). Required for SGP4-XP.
#[pyclass]
#[derive(Clone)]
pub struct TleParameters {
    pub inner: core_omm::TleParameters,
}

#[pymethods]
impl TleParameters {
    #[new]
    #[pyo3(
        signature = (*, mean_motion_dot, ephemeris_type=None, classification_type=None, norad_cat_id=None, element_set_no=None, rev_at_epoch=None, bstar=None, bterm=None, mean_motion_ddot=None, agom=None),
        text_signature = "(mean_motion_dot: float, ephemeris_type: Optional[int] = None, classification_type: Optional[str] = None, norad_cat_id: Optional[int] = None, element_set_no: Optional[int] = None, rev_at_epoch: Optional[int] = None, bstar: Optional[float] = None, bterm: Optional[float] = None, mean_motion_ddot: Optional[float] = None, agom: Optional[float] = None)"
    )]
    #[allow(clippy::too_many_arguments)]
    fn new(
        mean_motion_dot: f64,
        ephemeris_type: Option<i32>,
        classification_type: Option<String>,
        norad_cat_id: Option<u32>,
        element_set_no: Option<u32>,
        rev_at_epoch: Option<u32>,
        bstar: Option<f64>,
        bterm: Option<f64>,
        mean_motion_ddot: Option<f64>,
        agom: Option<f64>,
    ) -> PyResult<Self> {
        use ccsds_ndm::messages::omm::{BStar, MeanMotionDDot, MeanMotionDot};
        use ccsds_ndm::types::M2kg;

        Ok(Self {
            inner: core_omm::TleParameters {
                comment: vec![],
                ephemeris_type,
                classification_type,
                norad_cat_id,
                element_set_no: element_set_no.map(|value| ccsds_ndm::types::ElementSetNo { value }),
                rev_at_epoch,
                bstar: bstar.map(|v| BStar::new(v, Default::default())),
                bterm: bterm.map(|v| M2kg::new(v, Default::default())),
                mean_motion_dot: MeanMotionDot::new(mean_motion_dot, Default::default()),
                mean_motion_ddot: mean_motion_ddot
                    .map(|v| MeanMotionDDot::new(v, Default::default())),
                agom: agom.map(|v| M2kg::new(v, Default::default())),
            },
        })
    }

    fn __repr__(&self) -> String {
        "TleParameters(...)".to_string()
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

    /// Ephemeris type. Default value = 0. (See 4.2.4.7.)
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_ephemeris_type(&self) -> Option<i32> {
        self.inner.ephemeris_type
    }

    #[setter]
    fn set_ephemeris_type(&mut self, value: Option<i32>) {
        self.inner.ephemeris_type = value;
    }

    /// Classification type. Default value = U. (See 4.2.4.7.)
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_classification_type(&self) -> Option<String> {
        self.inner.classification_type.clone()
    }

    #[setter]
    fn set_classification_type(&mut self, value: Option<String>) {
        self.inner.classification_type = value;
    }

    /// NORAD Catalog Number (‘Satellite Number’) an integer of up to nine digits. This keyword
    /// is only required if MEAN_ELEMENT_THEORY=SGP/SGP4.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_norad_cat_id(&self) -> Option<u32> {
        self.inner.norad_cat_id
    }

    #[setter]
    fn set_norad_cat_id(&mut self, value: Option<u32>) {
        self.inner.norad_cat_id = value;
    }

    /// Element set number for this satellite. Normally incremented sequentially but may be out
    /// of sync if it is generated from a backup source. Used to distinguish different TLEs,
    /// and therefore only meaningful if TLE-based data is being exchanged (i.e.,
    /// MEAN_ELEMENT_THEORY = SGP/SGP4).
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_element_set_no(&self) -> Option<u32> {
        self.inner.element_set_no.as_ref().map(|v| v.value)
    }

    #[setter]
    fn set_element_set_no(&mut self, value: Option<u32>) {
        self.inner.element_set_no = value.map(|value| ccsds_ndm::types::ElementSetNo { value });
    }

    /// Revolution Number
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_rev_at_epoch(&self) -> Option<u32> {
        self.inner.rev_at_epoch
    }

    #[setter]
    fn set_rev_at_epoch(&mut self, value: Option<u32>) {
        self.inner.rev_at_epoch = value;
    }

    /// Drag-like ballistic coefficient, required for SGP4 and SGP4-XP mean element models:
    /// MEAN_ELEMENT_THEORY= SGP4 (BSTAR = drag parameter for SGP4).
    ///
    /// Units: 1/[Earth radii]
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_bstar(&self) -> Option<f64> {
        self.inner.bstar.as_ref().map(|v| v.value)
    }

    #[setter]
    fn set_bstar(&mut self, value: Option<f64>) {
        use ccsds_ndm::messages::omm::BStar;
        self.inner.bstar = value.map(|v| BStar::new(v, Default::default()));
    }

    /// Drag-like ballistic coefficient, required for SGP4 and SGP4-XP mean element models:
    /// MEAN_ELEMENT_THEORY= SGP4-XP (BTERM ballistic coefficient CDA/m, where CD = drag
    /// coefficient, A = average cross-sectional area, m = mass. Example values for BTERM =
    /// 0.02 (rocket body), 0.0015 (payload); average value spanning 20,000 catalog objects =
    /// 0.0286.
    ///
    /// Units: m²/kg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_bterm(&self) -> Option<f64> {
        self.inner.bterm.as_ref().map(|v| v.value)
    }

    #[setter]
    fn set_bterm(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::M2kg;
        self.inner.bterm = value.map(|v| M2kg::new(v, Default::default()));
    }

    /// First Time Derivative of the Mean Motion (i.e., a drag term, required when
    /// MEAN_ELEMENT_THEORY = SGP or PPT3). (See 4.2.4.7 for important details).
    ///
    /// Units: rev/day²
    ///
    /// :type: float
    #[getter]
    fn get_mean_motion_dot(&self) -> f64 {
        self.inner.mean_motion_dot.value
    }

    #[setter]
    fn set_mean_motion_dot(&mut self, value: f64) {
        use ccsds_ndm::messages::omm::MeanMotionDot;
        self.inner.mean_motion_dot = MeanMotionDot::new(value, Default::default());
    }

    /// Second Time Derivative of Mean Motion (i.e., a drag term). (See 4.2.4.7 for important
    /// details). Required when MEAN_ELEMENT_THEORY= SGP or PPT3.
    ///
    /// Units: rev/day³
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_mean_motion_ddot(&self) -> Option<f64> {
        self.inner.mean_motion_ddot.as_ref().map(|v| v.value)
    }

    #[setter]
    fn set_mean_motion_ddot(&mut self, value: Option<f64>) {
        use ccsds_ndm::messages::omm::MeanMotionDDot;
        self.inner.mean_motion_ddot = value.map(|v| MeanMotionDDot::new(v, Default::default()));
    }

    /// Solar radiation pressure coefficient AY/m, where y = reflectivity, A = average
    /// cross-sectional area, m = mass. Example values AGOM = 0.01 (rocket body) and 0.001
    /// (payload); average value spanning 20,000 catalog objects = 0.0143 m2/kg. Required
    /// when MEAN_ELEMENT_THEORY= SGP4-XP.
    ///
    /// Units: m²/kg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_agom(&self) -> Option<f64> {
        self.inner.agom.as_ref().map(|v| v.value)
    }

    #[setter]
    fn set_agom(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::M2kg;
        self.inner.agom = value.map(|v| M2kg::new(v, Default::default()));
    }
}
