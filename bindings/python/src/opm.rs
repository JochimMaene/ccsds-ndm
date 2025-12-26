// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{OdmHeader, StateVector};
use crate::types::parse_epoch;
use ccsds_ndm::messages::opm as core_opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::{Angle, Distance, Gm, Inclination, UserDefined as CoreUserDefined, UserDefinedParameter};
use ccsds_ndm::MessageType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;

/// Represents a CCSDS Orbit Parameter Message (OPM).
///
///
/// Parameters
/// ----------
/// header : OdmHeader
///     The message header.
/// segment : OpmSegment
///     The data segment.
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
/// Opm
///     The parsed OPM object.
#[pyclass]
#[derive(Clone)]
pub struct Opm {
    pub inner: core_opm::Opm,
}

#[pymethods]
impl Opm {
    #[new]
    fn new(header: OdmHeader, segment: OpmSegment) -> Self {
        Self {
            inner: core_opm::Opm {
                header: header.inner,
                body: core_opm::OpmBody {
                    segment: segment.inner,
                },
                id: Some("CCSDS_OPM_VERS".to_string()),
                version: "3.0".to_string(),
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Opm(object_name='{}')",
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
    /// :type: OpmSegment
    #[getter]
    fn get_segment(&self) -> OpmSegment {
        OpmSegment {
            inner: self.inner.body.segment.clone(),
        }
    }

    #[setter]
    fn set_segment(&mut self, segment: OpmSegment) {
        self.inner.body.segment = segment.inner;
    }

    /// Create an OPM message from a string.

    #[staticmethod]
    fn from_str(data: &str, format: Option<&str>) -> PyResult<Self> {
        let inner = match format {
            Some("kvn") => ccsds_ndm::messages::opm::Opm::from_kvn(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some("xml") => ccsds_ndm::messages::opm::Opm::from_xml(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some(other) => {
                return Err(PyValueError::new_err(format!(
                    "Unsupported format '{}'. Use 'kvn' or 'xml'",
                    other
                )))
            }
            None => match ccsds_ndm::from_str(data) {
                Ok(MessageType::Opm(opm)) => opm,
                Ok(other) => {
                    return Err(PyValueError::new_err(format!(
                        "Parsed message is not OPM (got {:?})",
                        other
                    )))
                }
                Err(e) => return Err(PyValueError::new_err(e.to_string())),
            },
        };
        Ok(Self { inner })
    }

    /// Create an OPM message from a file.
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
    /// Opm
    ///     The parsed OPM object.
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

/// Represents a single segment of an OPM.
#[pyclass]
#[derive(Clone)]
pub struct OpmSegment {
    pub inner: core_opm::OpmSegment,
}

#[pymethods]
impl OpmSegment {
    /// Create a new OPM Segment.
    ///
    /// Parameters
    /// ----------
    /// metadata : OpmMetadata
    ///     Segment metadata.
    /// data : OpmData
    ///     Segment data.
    #[new]
    fn new(metadata: OpmMetadata, data: OpmData) -> Self {
        Self {
            inner: core_opm::OpmSegment {
                metadata: metadata.inner,
                data: data.inner,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "OpmSegment(object_name='{}')",
            self.inner.metadata.object_name
        )
    }

    /// Segment metadata.
    ///
    /// :type: OpmMetadata
    #[getter]
    fn get_metadata(&self) -> OpmMetadata {
        OpmMetadata {
            inner: self.inner.metadata.clone(),
        }
    }

    #[setter]
    fn set_metadata(&mut self, metadata: OpmMetadata) {
        self.inner.metadata = metadata.inner;
    }

    /// Segment data.
    ///
    /// :type: OpmData
    #[getter]
    fn get_data(&self) -> OpmData {
        OpmData {
            inner: self.inner.data.clone(),
        }
    }

    #[setter]
    fn set_data(&mut self, data: OpmData) {
        self.inner.data = data.inner;
    }
}

/// Create a new OPM Metadata object.
///
/// Parameters
/// ----------
/// object_name : str
///     Spacecraft name for which orbit state data is provided.
/// object_id : str
///     Object identifier of the object for which orbit state data is provided.
/// center_name : str
///     Origin of the reference frame.
/// ref_frame : str
///     Reference frame in which state vector data is given.
///     Time system used for state vector, maneuver, and covariance data.
///     Allowed values: 'GMST', 'GPS', 'MET', 'MRT', 'SCLK', 'TAI', 'TCB', 'TDB', 'TCG', 'TT', 'UT1', 'UTC'.
/// ref_frame_epoch : str, optional
///     Epoch of the reference frame, if not intrinsic to the definition.
/// comment : list of str, optional
///     Comments.
#[pyclass]
#[derive(Clone)]
pub struct OpmMetadata {
    pub inner: core_opm::OpmMetadata,
}

#[pymethods]
impl OpmMetadata {
    #[new]
    fn new(
        object_name: String,
        object_id: String,
        center_name: String,
        ref_frame: String,
        time_system: String,
        ref_frame_epoch: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_opm::OpmMetadata {
                object_name,
                object_id,
                center_name,
                ref_frame,
                time_system,
                ref_frame_epoch: ref_frame_epoch.map(|s| parse_epoch(&s)).transpose()?,
                comment: comment.unwrap_or_default(),
            },
        })
    }

    fn __repr__(&self) -> String {
        format!("OpmMetadata(object_name='{}')", self.inner.object_name)
    }

    /// Spacecraft name for which orbit state data is provided.
    ///
    /// While there is no CCSDS-based restriction on the value for this keyword, it is recommended to use names from the UN Office of Outer Space Affairs designator index.
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

    /// Object identifier of the object for which orbit state data is provided.
    ///
    /// Recommended values have the format YYYY-NNNP{PP}.
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

    /// Origin of the OPM reference frame.
    ///
    /// Shall be a natural solar system body (planets, asteroids, comets, and natural satellites), including any planet barycenter or the solar system barycenter.
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

    /// Reference frame in which the state vector and optional Keplerian element data are given.
    ///
    /// Examples: 'ICRF', 'EME2000', 'ITRF2000', 'TEME'.
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

    /// Time system used for state vector, maneuver, and covariance data.
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

    /// Epoch of the reference frame, if not intrinsic to the definition.
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

    /// Comments.
    ///
    /// :type: List[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }
}

#[pyclass]
#[derive(Clone)]
pub struct KeplerianElements {
    pub inner: core_opm::KeplerianElements,
}

#[pymethods]
impl KeplerianElements {
    #[new]
    fn new(
        semi_major_axis: f64,
        eccentricity: f64,
        inclination: f64,
        ra_of_asc_node: f64,
        arg_of_pericenter: f64,
        gm: f64,
        true_anomaly: Option<f64>,
        mean_anomaly: Option<f64>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_opm::KeplerianElements {
                comment: vec![],
                semi_major_axis: Distance::new(semi_major_axis, None),
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
                gm: Gm::new(gm, None)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
                true_anomaly: true_anomaly
                    .map(|v| Angle::new(v, None))
                    .transpose()
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
                mean_anomaly: mean_anomaly
                    .map(|v| Angle::new(v, None))
                    .transpose()
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            },
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "KeplerianElements(semi_major_axis={:.3} km, eccentricity={:.6})",
            self.inner.semi_major_axis.value, self.inner.eccentricity
        )
    }

    /// Comments associated with the Keplerian elements.
    ///
    /// :type: List[str]
    #[getter]
    fn get_comments(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comments(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    /// Semi-major axis.
    ///
    /// Units: km
    ///
    /// :type: float
    #[getter]
    fn get_semi_major_axis(&self) -> f64 {
        self.inner.semi_major_axis.value
    }

    #[setter]
    fn set_semi_major_axis(&mut self, value: f64) {
        self.inner.semi_major_axis.value = value;
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

    /// Gravitational coefficient (GM).
    ///
    /// Units: km³/s²
    ///
    /// :type: float
    #[getter]
    fn get_gm(&self) -> f64 {
        self.inner.gm.value
    }

    #[setter]
    fn set_gm(&mut self, value: f64) -> PyResult<()> {
        self.inner.gm = Gm::new(value, None)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(())
    }

    /// True anomaly.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_true_anomaly(&self) -> Option<f64> {
        self.inner.true_anomaly.as_ref().map(|a| a.value)
    }

    #[setter]
    fn set_true_anomaly(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.true_anomaly = value
            .map(|v| Angle::new(v, None))
            .transpose()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(())
    }

    /// Mean anomaly.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_mean_anomaly(&self) -> Option<f64> {
        self.inner.mean_anomaly.as_ref().map(|a| a.value)
    }

    #[setter]
    fn set_mean_anomaly(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.mean_anomaly = value
            .map(|v| Angle::new(v, None))
            .transpose()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct OpmCovarianceMatrix {
    pub inner: ccsds_ndm::common::OpmCovarianceMatrix,
}

#[pymethods]
impl OpmCovarianceMatrix {
    #[new]
    fn new(
        cx_x: Option<f64>,
        cy_x: Option<f64>,
        cy_y: Option<f64>,
        cz_x: Option<f64>,
        cz_y: Option<f64>,
        cz_z: Option<f64>,
        cx_dot_x: Option<f64>,
        cx_dot_y: Option<f64>,
        cx_dot_z: Option<f64>,
        cy_dot_x: Option<f64>,
        cy_dot_y: Option<f64>,
        cy_dot_z: Option<f64>,
        cz_dot_x: Option<f64>,
        cz_dot_y: Option<f64>,
        cz_dot_z: Option<f64>,
        cx_dot_x_dot: Option<f64>,
        cy_dot_x_dot: Option<f64>,
        cy_dot_y_dot: Option<f64>,
        cz_dot_x_dot: Option<f64>,
        cz_dot_y_dot: Option<f64>,
        cz_dot_z_dot: Option<f64>,
        cov_ref_frame: Option<String>,
    ) -> Self {
        use ccsds_ndm::types::{
            PositionCovariance, PositionVelocityCovariance, VelocityCovariance,
        };
        Self {
            inner: ccsds_ndm::common::OpmCovarianceMatrix {
                comment: vec![],
                cov_ref_frame,
                cx_x: PositionCovariance::new(cx_x.unwrap_or(0.0), None),
                cy_x: PositionCovariance::new(cy_x.unwrap_or(0.0), None),
                cy_y: PositionCovariance::new(cy_y.unwrap_or(0.0), None),
                cz_x: PositionCovariance::new(cz_x.unwrap_or(0.0), None),
                cz_y: PositionCovariance::new(cz_y.unwrap_or(0.0), None),
                cz_z: PositionCovariance::new(cz_z.unwrap_or(0.0), None),
                cx_dot_x: PositionVelocityCovariance::new(cx_dot_x.unwrap_or(0.0), None),
                cx_dot_y: PositionVelocityCovariance::new(cx_dot_y.unwrap_or(0.0), None),
                cx_dot_z: PositionVelocityCovariance::new(cx_dot_z.unwrap_or(0.0), None),
                cy_dot_x: PositionVelocityCovariance::new(cy_dot_x.unwrap_or(0.0), None),
                cy_dot_y: PositionVelocityCovariance::new(cy_dot_y.unwrap_or(0.0), None),
                cy_dot_z: PositionVelocityCovariance::new(cy_dot_z.unwrap_or(0.0), None),
                cz_dot_x: PositionVelocityCovariance::new(cz_dot_x.unwrap_or(0.0), None),
                cz_dot_y: PositionVelocityCovariance::new(cz_dot_y.unwrap_or(0.0), None),
                cz_dot_z: PositionVelocityCovariance::new(cz_dot_z.unwrap_or(0.0), None),
                cx_dot_x_dot: VelocityCovariance::new(cx_dot_x_dot.unwrap_or(0.0), None),
                cy_dot_x_dot: VelocityCovariance::new(cy_dot_x_dot.unwrap_or(0.0), None),
                cy_dot_y_dot: VelocityCovariance::new(cy_dot_y_dot.unwrap_or(0.0), None),
                cz_dot_x_dot: VelocityCovariance::new(cz_dot_x_dot.unwrap_or(0.0), None),
                cz_dot_y_dot: VelocityCovariance::new(cz_dot_y_dot.unwrap_or(0.0), None),
                cz_dot_z_dot: VelocityCovariance::new(cz_dot_z_dot.unwrap_or(0.0), None),
            },
        }
    }

    fn __repr__(&self) -> String {
        "OpmCovarianceMatrix(...)".to_string()
    }

    /// Reference frame for the covariance matrix.
    ///
    /// If omitted, the covariance is expressed in the same reference frame
    /// as the state vector (REF_FRAME from metadata).
    ///
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_cov_ref_frame(&self) -> Option<String> {
        self.inner.cov_ref_frame.clone()
    }

    #[setter]
    fn set_cov_ref_frame(&mut self, value: Option<String>) {
        self.inner.cov_ref_frame = value;
    }

    /// Comments associated with this covariance matrix.
    ///
    ///
    /// :type: List[str]
    #[getter]
    fn get_comments(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comments(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    /// Position covariance [1,1] element (X-X variance).
    ///
    /// Units: km²
    ///
    ///
    /// :type: float
    #[getter]
    fn get_cx_x(&self) -> f64 {
        self.inner.cx_x.value
    }

    #[setter]
    fn set_cx_x(&mut self, value: f64) {
        self.inner.cx_x.value = value;
    }

    /// Position covariance [2,1] element (Y-X covariance).
    ///
    /// Units: km²
    ///
    /// :type: float
    #[getter]
    fn get_cy_x(&self) -> f64 {
        self.inner.cy_x.value
    }

    #[setter]
    fn set_cy_x(&mut self, value: f64) {
        self.inner.cy_x.value = value;
    }

    /// Position covariance [2,2] element (Y-Y variance).
    ///
    /// Units: km²
    ///
    /// :type: float
    #[getter]
    fn get_cy_y(&self) -> f64 {
        self.inner.cy_y.value
    }

    #[setter]
    fn set_cy_y(&mut self, value: f64) {
        self.inner.cy_y.value = value;
    }

    /// Position covariance [3,1] element (Z-X covariance).
    ///
    /// Units: km²
    ///
    /// :type: float
    #[getter]
    fn get_cz_x(&self) -> f64 {
        self.inner.cz_x.value
    }

    #[setter]
    fn set_cz_x(&mut self, value: f64) {
        self.inner.cz_x.value = value;
    }

    /// Position covariance [3,2] element (Z-Y covariance).
    ///
    /// Units: km²
    ///
    /// :type: float
    #[getter]
    fn get_cz_y(&self) -> f64 {
        self.inner.cz_y.value
    }

    #[setter]
    fn set_cz_y(&mut self, value: f64) {
        self.inner.cz_y.value = value;
    }

    /// Position covariance [3,3] element (Z-Z variance).
    ///
    /// Units: km²
    ///
    /// :type: float
    #[getter]
    fn get_cz_z(&self) -> f64 {
        self.inner.cz_z.value
    }

    #[setter]
    fn set_cz_z(&mut self, value: f64) {
        self.inner.cz_z.value = value;
    }

    /// Position-Velocity cross-covariance [4,1] element (X_DOT-X covariance).
    ///
    /// Units: km·km/s
    ///
    /// :type: float
    #[getter]
    fn get_cx_dot_x(&self) -> f64 {
        self.inner.cx_dot_x.value
    }

    #[setter]
    fn set_cx_dot_x(&mut self, value: f64) {
        self.inner.cx_dot_x.value = value;
    }

    /// Position-Velocity cross-covariance [4,2] element (X_DOT-Y covariance).
    ///
    /// Units: km·km/s
    ///
    /// :type: float
    #[getter]
    fn get_cx_dot_y(&self) -> f64 {
        self.inner.cx_dot_y.value
    }

    #[setter]
    fn set_cx_dot_y(&mut self, value: f64) {
        self.inner.cx_dot_y.value = value;
    }

    /// Position-Velocity cross-covariance [4,3] element (X_DOT-Z covariance).
    ///
    /// Units: km·km/s
    ///
    /// :type: float
    #[getter]
    fn get_cx_dot_z(&self) -> f64 {
        self.inner.cx_dot_z.value
    }

    #[setter]
    fn set_cx_dot_z(&mut self, value: f64) {
        self.inner.cx_dot_z.value = value;
    }

    /// Position-Velocity cross-covariance [5,1] element (Y_DOT-X covariance).
    ///
    /// Units: km·km/s
    ///
    /// :type: float
    #[getter]
    fn get_cy_dot_x(&self) -> f64 {
        self.inner.cy_dot_x.value
    }

    #[setter]
    fn set_cy_dot_x(&mut self, value: f64) {
        self.inner.cy_dot_x.value = value;
    }

    /// Position-Velocity cross-covariance [5,2] element (Y_DOT-Y covariance).
    ///
    /// Units: km·km/s
    ///
    /// :type: float
    #[getter]
    fn get_cy_dot_y(&self) -> f64 {
        self.inner.cy_dot_y.value
    }

    #[setter]
    fn set_cy_dot_y(&mut self, value: f64) {
        self.inner.cy_dot_y.value = value;
    }

    /// Position-Velocity cross-covariance [5,3] element (Y_DOT-Z covariance).
    ///
    /// Units: km·km/s
    ///
    /// :type: float
    #[getter]
    fn get_cy_dot_z(&self) -> f64 {
        self.inner.cy_dot_z.value
    }

    #[setter]
    fn set_cy_dot_z(&mut self, value: f64) {
        self.inner.cy_dot_z.value = value;
    }

    /// Position-Velocity cross-covariance [6,1] element (Z_DOT-X covariance).
    ///
    /// Units: km·km/s
    ///
    /// :type: float
    #[getter]
    fn get_cz_dot_x(&self) -> f64 {
        self.inner.cz_dot_x.value
    }

    #[setter]
    fn set_cz_dot_x(&mut self, value: f64) {
        self.inner.cz_dot_x.value = value;
    }

    /// Position-Velocity cross-covariance [6,2] element (Z_DOT-Y covariance).
    ///
    /// Units: km·km/s
    ///
    /// :type: float
    #[getter]
    fn get_cz_dot_y(&self) -> f64 {
        self.inner.cz_dot_y.value
    }

    #[setter]
    fn set_cz_dot_y(&mut self, value: f64) {
        self.inner.cz_dot_y.value = value;
    }

    /// Position-Velocity cross-covariance [6,3] element (Z_DOT-Z covariance).
    ///
    /// Units: km·km/s
    ///
    /// :type: float
    #[getter]
    fn get_cz_dot_z(&self) -> f64 {
        self.inner.cz_dot_z.value
    }

    #[setter]
    fn set_cz_dot_z(&mut self, value: f64) {
        self.inner.cz_dot_z.value = value;
    }

    /// Velocity covariance [4,4] element (X_DOT-X_DOT variance).
    ///
    /// Units: km²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cx_dot_x_dot(&self) -> f64 {
        self.inner.cx_dot_x_dot.value
    }

    #[setter]
    fn set_cx_dot_x_dot(&mut self, value: f64) {
        self.inner.cx_dot_x_dot.value = value;
    }

    /// Velocity covariance [5,4] element (Y_DOT-X_DOT covariance).
    ///
    /// Units: km²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cy_dot_x_dot(&self) -> f64 {
        self.inner.cy_dot_x_dot.value
    }

    #[setter]
    fn set_cy_dot_x_dot(&mut self, value: f64) {
        self.inner.cy_dot_x_dot.value = value;
    }

    /// Velocity covariance [5,5] element (Y_DOT-Y_DOT variance).
    ///
    /// Units: km²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cy_dot_y_dot(&self) -> f64 {
        self.inner.cy_dot_y_dot.value
    }

    #[setter]
    fn set_cy_dot_y_dot(&mut self, value: f64) {
        self.inner.cy_dot_y_dot.value = value;
    }

    /// Velocity covariance [6,4] element (Z_DOT-X_DOT covariance).
    ///
    /// Units: km²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cz_dot_x_dot(&self) -> f64 {
        self.inner.cz_dot_x_dot.value
    }

    #[setter]
    fn set_cz_dot_x_dot(&mut self, value: f64) {
        self.inner.cz_dot_x_dot.value = value;
    }

    /// Velocity covariance [6,5] element (Z_DOT-Y_DOT covariance).
    ///
    /// Units: km²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cz_dot_y_dot(&self) -> f64 {
        self.inner.cz_dot_y_dot.value
    }

    #[setter]
    fn set_cz_dot_y_dot(&mut self, value: f64) {
        self.inner.cz_dot_y_dot.value = value;
    }

    /// Velocity covariance [6,6] element (Z_DOT-Z_DOT variance).
    ///
    /// Units: km²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cz_dot_z_dot(&self) -> f64 {
        self.inner.cz_dot_z_dot.value
    }

    #[setter]
    fn set_cz_dot_z_dot(&mut self, value: f64) {
        self.inner.cz_dot_z_dot.value = value;
    }
}

/// Create a new OPM Data object.
///
/// Parameters
/// ----------
/// state_vector : StateVector
///     State vector.
#[pyclass]
#[derive(Clone)]
pub struct OpmData {
    pub inner: core_opm::OpmData,
}

#[pymethods]
impl OpmData {
    #[new]
    fn new(state_vector: StateVector) -> Self {
        Self {
            inner: core_opm::OpmData {
                comment: vec![],
                state_vector: state_vector.inner,
                keplerian_elements: None,
                spacecraft_parameters: None,
                covariance_matrix: None,
                maneuver_parameters: vec![],
                user_defined_parameters: None,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "OpmData(epoch='{}')",
            self.inner.state_vector.epoch.as_str()
        )
    }

    /// State vector.
    ///
    /// :type: StateVector
    #[getter]
    fn get_state_vector(&self) -> StateVector {
        StateVector {
            inner: self.inner.state_vector.clone(),
        }
    }

    #[setter]
    fn set_state_vector(&mut self, value: StateVector) {
        self.inner.state_vector = value.inner;
    }

    /// Keplerian elements.
    ///
    /// :type: Optional[KeplerianElements]
    #[getter]
    fn get_keplerian_elements(&self) -> Option<KeplerianElements> {
        self.inner
            .keplerian_elements
            .as_ref()
            .map(|k| KeplerianElements { inner: k.clone() })
    }

    #[setter]
    fn set_keplerian_elements(&mut self, value: Option<KeplerianElements>) {
        self.inner.keplerian_elements = value.map(|k| k.inner);
    }

    /// Spacecraft parameters.
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

    /// Covariance matrix.
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

    /// Maneuver parameters.
    ///
    /// :type: List[ManeuverParameters]
    #[getter]
    fn get_maneuver_parameters(&self) -> Vec<ManeuverParameters> {
        self.inner
            .maneuver_parameters
            .iter()
            .map(|m| ManeuverParameters { inner: m.clone() })
            .collect()
    }

    #[setter]
    fn set_maneuver_parameters(&mut self, value: Vec<ManeuverParameters>) {
        self.inner.maneuver_parameters = value.into_iter().map(|m| m.inner).collect();
    }

    /// User defined parameters.
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

/// Maneuver parameters.
///
/// Parameters
/// ----------
/// man_epoch_ignition : str
///     Epoch of ignition.
/// man_duration : float
///     Duration of maneuver (s).
/// man_delta_mass : float
///     Mass change during maneuver (kg).
/// man_ref_frame : str
///     Reference frame for velocity change.
/// man_dv_1 : float
///     Velocity change in 1st axis (km/s).
/// man_dv_2 : float
///     Velocity change in 2nd axis (km/s).
/// man_dv_3 : float
///     Velocity change in 3rd axis (km/s).
#[pyclass]
#[derive(Clone)]
pub struct ManeuverParameters {
    pub inner: core_opm::ManeuverParameters,
}

#[pymethods]
impl ManeuverParameters {
    #[new]
    fn new(
        man_epoch_ignition: String,
        man_duration: f64,
        man_delta_mass: f64,
        man_ref_frame: String,
        man_dv_1: f64,
        man_dv_2: f64,
        man_dv_3: f64,
    ) -> PyResult<Self> {
        use ccsds_ndm::types::{DeltaMass, Duration, Velocity};
        Ok(Self {
            inner: core_opm::ManeuverParameters {
                comment: vec![],
                man_epoch_ignition: parse_epoch(&man_epoch_ignition)?,
                man_duration: Duration::new(man_duration, None)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
                man_delta_mass: DeltaMass::new(man_delta_mass, None)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
                man_ref_frame,
                man_dv_1: Velocity::new(man_dv_1, None),
                man_dv_2: Velocity::new(man_dv_2, None),
                man_dv_3: Velocity::new(man_dv_3, None),
            },
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "ManeuverParameters(epoch='{}', duration={:.3} s)",
            self.inner.man_epoch_ignition.as_str(),
            self.inner.man_duration.value
        )
    }

    /// Comments.
    ///
    /// :type: List[str]
    #[getter]
    fn get_comments(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comments(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    /// Epoch of ignition.
    ///
    /// :type: str
    #[getter]
    fn get_man_epoch_ignition(&self) -> String {
        self.inner.man_epoch_ignition.as_str().to_string()
    }

    #[setter]
    fn set_man_epoch_ignition(&mut self, value: String) -> PyResult<()> {
        self.inner.man_epoch_ignition = parse_epoch(&value)?;
        Ok(())
    }

    /// Duration of maneuver.
    ///
    /// If 0, impulsive maneuver.
    ///
    /// Units: s
    ///
    /// :type: float
    #[getter]
    fn get_man_duration(&self) -> f64 {
        self.inner.man_duration.value
    }

    #[setter]
    fn set_man_duration(&mut self, value: f64) {
        self.inner.man_duration.value = value;
    }

    /// Mass change during maneuver.
    ///
    /// Value is < 0.
    ///
    /// Units: kg
    ///
    /// :type: float
    #[getter]
    fn get_man_delta_mass(&self) -> f64 {
        self.inner.man_delta_mass.value
    }

    #[setter]
    fn set_man_delta_mass(&mut self, value: f64) {
        self.inner.man_delta_mass.value = value;
    }

    /// Reference frame for velocity change.
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

    /// Velocity change in 1st axis.
    ///
    /// Units: km/s
    ///
    /// :type: float
    #[getter]
    fn get_man_dv_1(&self) -> f64 {
        self.inner.man_dv_1.value
    }

    #[setter]
    fn set_man_dv_1(&mut self, value: f64) {
        self.inner.man_dv_1.value = value;
    }

    /// Velocity change in 2nd axis.
    ///
    /// Units: km/s
    ///
    /// :type: float
    #[getter]
    fn get_man_dv_2(&self) -> f64 {
        self.inner.man_dv_2.value
    }

    #[setter]
    fn set_man_dv_2(&mut self, value: f64) {
        self.inner.man_dv_2.value = value;
    }

    /// Velocity change in 3rd axis.
    ///
    /// Units: km/s
    ///
    /// :type: float
    #[getter]
    fn get_man_dv_3(&self) -> f64 {
        self.inner.man_dv_3.value
    }

    #[setter]
    fn set_man_dv_3(&mut self, value: f64) {
        self.inner.man_dv_3.value = value;
    }
}

/// User defined parameters.
///
/// Parameters
/// ----------
/// parameters : dict
///     Dictionary of user defined parameters. Keys should generally start with 'USER_DEFINED_'.
#[pyclass]
#[derive(Clone)]
pub struct UserDefined {
    pub inner: CoreUserDefined,
}

#[pymethods]
impl UserDefined {
    #[new]
    fn new(parameters: std::collections::HashMap<String, String>) -> Self {
        let user_defined = parameters
            .into_iter()
            .map(|(k, v)| UserDefinedParameter {
                parameter: k,
                value: v,
            })
            .collect();
        Self {
            inner: CoreUserDefined {
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
    /// :type: List[str]
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
            .map(|(k, v)| UserDefinedParameter {
                parameter: k,
                value: v,
            })
            .collect();
    }
}
