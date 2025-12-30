// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
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

// Import OpmCovarianceMatrix from opm module (shared type)
use crate::opm::OpmCovarianceMatrix;

/// Orbit Mean-Elements Message (OMM).
///
/// The OMM contains the orbital characteristics of a single object at a specified epoch,
/// expressed in mean Keplerian elements.
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

    /// The message header.
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
    fn new(
        object_name: String,
        object_id: String,
        center_name: String,
        ref_frame: String,
        time_system: String,
        mean_element_theory: String,
        ref_frame_epoch: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
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

    /// Spacecraft name for which mean element orbit state data is provided.
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

    /// Origin of the OMM reference frame.
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

    /// Reference frame in which the Keplerian element data are given.
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

    /// Time system used for Keplerian elements and covariance data.
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

    /// Description of the Mean Element Theory. Indicates the proper method to employ to propagate the state.
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

    /// Comments (allowed at the beginning of the OMM Metadata).
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
                eccentricity,
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

    /// Comments.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comments(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comments(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    /// Epoch of Mean Keplerian elements.
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

    /// Eccentricity.
    ///
    /// :type: float
    #[getter]
    fn get_eccentricity(&self) -> f64 {
        self.inner.eccentricity
    }

    #[setter]
    fn set_eccentricity(&mut self, value: f64) {
        self.inner.eccentricity = value;
    }

    /// Inclination.
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

    /// Right ascension of the ascending node.
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

    /// Argument of pericenter.
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

    /// Mean anomaly.
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

    /// Semi-major axis.
    ///
    /// Preferred over MEAN_MOTION.
    ///
    /// Units: km
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

    /// Mean motion.
    ///
    /// Required if MEAN_ELEMENT_THEORY = SGP/SGP4.
    ///
    /// Units: rev/day
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

    /// Gravitational coefficient (GM).
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
    fn get_comments(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comments(&mut self, value: Vec<String>) {
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
    /// :type: Optional[UserDefined]
    #[getter]
    fn get_user_defined_parameters(&self) -> Option<UserDefined> {
        self.inner
            .user_defined_parameters
            .as_ref()
            .map(|u| UserDefined { inner: u.clone() })
    }

    #[setter]
    fn set_user_defined_parameters(&mut self, value: Option<UserDefined>) {
        self.inner.user_defined_parameters = value.map(|u| u.inner);
    }
}

/// TLE Related Parameters.
///
/// This section is only required if MEAN_ELEMENT_THEORY=SGP/SGP4.
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
        signature = (*, ephemeris_type=None, classification_type=None, norad_cat_id=None, element_set_no=None, rev_at_epoch=None, bstar=None, bterm=None, mean_motion_dot=None, mean_motion_ddot=None, agom=None),
        text_signature = "(ephemeris_type: Optional[int] = None, classification_type: Optional[str] = None, norad_cat_id: Optional[int] = None, element_set_no: Optional[int] = None, rev_at_epoch: Optional[int] = None, bstar: Optional[float] = None, bterm: Optional[float] = None, mean_motion_dot: Optional[float] = None, mean_motion_ddot: Optional[float] = None, agom: Optional[float] = None)"
    )]
    #[allow(clippy::too_many_arguments)]
    fn new(
        ephemeris_type: Option<i32>,
        classification_type: Option<String>,
        norad_cat_id: Option<u32>,
        element_set_no: Option<u32>,
        rev_at_epoch: Option<u32>,
        bstar: Option<f64>,
        bterm: Option<f64>,
        mean_motion_dot: Option<f64>,
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
                element_set_no,
                rev_at_epoch,
                bstar: bstar.map(|v| BStar::new(v, Default::default())),
                bterm: bterm.map(|v| M2kg::new(v, Default::default())),
                mean_motion_dot: mean_motion_dot.map(|v| MeanMotionDot::new(v, Default::default())),
                mean_motion_ddot: mean_motion_ddot
                    .map(|v| MeanMotionDDot::new(v, Default::default())),
                agom: agom.map(|v| M2kg::new(v, Default::default())),
            },
        })
    }

    fn __repr__(&self) -> String {
        "TleParameters(...)".to_string()
    }

    /// Comments.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comments(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comments(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    /// Ephemeris Type, default value = 0.
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

    /// Classification Type, default value = U.
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

    /// NORAD Catalog Number ('Satellite Number').
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

    /// Element set number for this satellite.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_element_set_no(&self) -> Option<u32> {
        self.inner.element_set_no
    }

    #[setter]
    fn set_element_set_no(&mut self, value: Option<u32>) {
        self.inner.element_set_no = value;
    }

    /// Revolution Number.
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

    /// B* drag term.
    ///
    /// Units: 1/ER
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

    /// Ballistic coefficient.
    ///
    /// Required for SGP4-XP.
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

    /// First Time Derivative of the Mean Motion.
    ///
    /// Units: rev/day²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_mean_motion_dot(&self) -> Option<f64> {
        self.inner.mean_motion_dot.as_ref().map(|v| v.value)
    }

    #[setter]
    fn set_mean_motion_dot(&mut self, value: Option<f64>) {
        use ccsds_ndm::messages::omm::MeanMotionDot;
        self.inner.mean_motion_dot = value.map(|v| MeanMotionDot::new(v, Default::default()));
    }

    /// Second Time Derivative of Mean Motion.
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

    /// Solar radiation pressure area to mass ratio.
    ///
    /// Required for SGP4-XP.
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

/// UserDefined object.
///
/// Parameters
/// ----------
/// parameters : dict
///     Dictionary of user defined parameters.
#[pyclass]
#[derive(Clone)]
pub struct UserDefined {
    pub inner: ccsds_ndm::types::UserDefined,
}

#[pymethods]
impl UserDefined {
    #[new]
    #[pyo3(text_signature = "(parameters: Dict[str, str])")]
    fn new(parameters: std::collections::HashMap<String, String>) -> Self {
        let user_defined = parameters
            .into_iter()
            .map(|(k, v)| ccsds_ndm::types::UserDefinedParameter {
                parameter: k,
                value: v,
            })
            .collect();
        Self {
            inner: ccsds_ndm::types::UserDefined {
                comment: vec![],
                user_defined,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!("UserDefined(count={})", self.inner.user_defined.len())
    }

    /// Comments.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comments(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comments(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    /// User defined parameters as a dictionary.
    ///
    /// :type: Dict[str, str]
    #[getter]
    fn get_parameters(&self) -> std::collections::HashMap<String, String> {
        self.inner
            .user_defined
            .iter()
            .map(|p| (p.parameter.clone(), p.value.clone()))
            .collect()
    }

    #[setter]
    fn set_parameters(&mut self, value: std::collections::HashMap<String, String>) {
        self.inner.user_defined = value
            .into_iter()
            .map(|(k, v)| ccsds_ndm::types::UserDefinedParameter {
                parameter: k,
                value: v,
            })
            .collect();
    }
}
