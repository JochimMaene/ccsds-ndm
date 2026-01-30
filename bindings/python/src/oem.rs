// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{OdmHeader, StateVectorAcc};
use crate::types::parse_epoch;
use ccsds_ndm::messages::oem as core_oem;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::{
    Acc, Position, PositionCovariance, PositionVelocityCovariance, Velocity, VelocityCovariance,
};
use ccsds_ndm::MessageType;
use numpy::{PyArray, PyArrayMethods, PyReadonlyArray1, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;
use crate::common::{parse_reference_frame, parse_time_system};

use std::num::NonZeroU32;

/// Orbit Ephemeris Message (OEM).
///
/// An OEM specifies the position and velocity of a single object at multiple epochs contained
/// within a specified time range. The message recipient must have a means of interpolating
/// across these state vectors to obtain the state at an arbitrary time contained within the
/// span of the ephemeris.
///
/// The OEM is suited to exchanges that:
/// 1. Involve automated interaction (e.g., computer-to-computer communication).
/// 2. Require higher fidelity or higher precision dynamic modeling than is possible with the OPM.
///
/// Parameters
/// ----------
/// header : OdmHeader
///     The message header.
/// segments : list[OemSegment]
///     The list of data segments.
#[pyclass]
#[derive(Clone)]
pub struct Oem {
    pub inner: core_oem::Oem,
}

/// A single segment of the OEM.
///
/// Each segment contains metadata (context) and a list of ephemeris data points.
///
/// Parameters
/// ----------
/// metadata : OemMetadata
///     Segment metadata.
/// data : OemData
///     Segment data.
#[pyclass]
#[derive(Clone)]
pub struct OemSegment {
    pub inner: core_oem::OemSegment,
}

/// OEM Metadata Section.
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
/// time_system : str
///     Time system used for state vector, maneuver, and covariance data.
/// start_time : str
///     Start time of the total time span covered by the ephemeris data (ISO 8601).
/// stop_time : str
///     Stop time of the total time span covered by the ephemeris data (ISO 8601).
/// ref_frame_epoch : str, optional
///     Epoch of the reference frame, if not intrinsic to the definition (ISO 8601).
/// useable_start_time : str, optional
///     Start of the recommended time span for use of the ephemeris data (ISO 8601).
/// useable_stop_time : str, optional
///     End of the recommended time span for use of the ephemeris data (ISO 8601).
/// interpolation : str, optional
///     Recommended interpolation method for ephemeris data.
/// interpolation_degree : int, optional
///     Degree of the interpolation polynomial.
/// comment : list[str], optional
///     Comments.
#[pyclass]
#[derive(Clone)]
pub struct OemMetadata {
    pub inner: core_oem::OemMetadata,
}

/// OEM Data Section.
///
/// Parameters
/// ----------
///     state_vectors : list[StateVectorAcc]
///     List of state vectors.
///     comments : list[str], optional
///     Comments.
#[pyclass]
#[derive(Clone)]
pub struct OemData {
    pub inner: core_oem::OemData,
}

/// OEM Covariance Matrix.
///
/// Represents a 6x6 symmetric covariance matrix for position and velocity at a specific epoch.
/// The lower triangular portion is stored/transmitted.
///
/// Parameters
/// ----------
/// epoch : str
///     Epoch of the covariance matrix (ISO 8601).
///     values : numpy.ndarray
///     Flat NumPy array of length 21 containing the covariance values.
/// cov_ref_frame : str, optional
///     Reference frame for the covariance matrix.
/// comment : list[str], optional
///     Comments associated with this covariance matrix.
///
/// Attributes
/// ----------
/// epoch : str
///     Epoch of the covariance matrix.
/// cx_x : float
///     Position X covariance [1,1]. Units: km².
/// cy_x : float
///     Position X-Y covariance [2,1]. Units: km².
/// cy_y : float
///     Position Y covariance [2,2]. Units: km².
/// cz_x : float
///     Position X-Z covariance [3,1]. Units: km².
/// cz_y : float
///     Position Y-Z covariance [3,2]. Units: km².
/// cz_z : float
///     Position Z covariance [3,3]. Units: km².
/// cx_dot_x : float
///     Velocity X / Position X covariance [4,1]. Units: km²/s.
/// cx_dot_y : float
///     Velocity X / Position Y covariance [4,2]. Units: km²/s.
/// cx_dot_z : float
///     Velocity X / Position Z covariance [4,3]. Units: km²/s.
/// cx_dot_x_dot : float
///     Velocity X covariance [4,4]. Units: km²/s².
/// cy_dot_x : float
///     Velocity Y / Position X covariance [5,1]. Units: km²/s.
/// cy_dot_y : float
///     Velocity Y / Position Y covariance [5,2]. Units: km²/s.
/// cy_dot_z : float
///     Velocity Y / Position Z covariance [5,3]. Units: km²/s.
/// cy_dot_x_dot : float
///     Velocity Y / Velocity X covariance [5,4]. Units: km²/s².
/// cy_dot_y_dot : float
///     Velocity Y covariance [5,5]. Units: km²/s².
/// cz_dot_x : float
///     Velocity Z / Position X covariance [6,1]. Units: km²/s.
/// cz_dot_y : float
///     Velocity Z / Position Y covariance [6,2]. Units: km²/s.
/// cz_dot_z : float
///     Velocity Z / Position Z covariance [6,3]. Units: km²/s.
/// cz_dot_x_dot : float
///     Velocity Z / Velocity X covariance [6,4]. Units: km²/s².
/// cz_dot_y_dot : float
///     Velocity Z / Velocity Y covariance [6,5]. Units: km²/s².
/// cz_dot_z_dot : float
///     Velocity Z covariance [6,6]. Units: km²/s².
#[pyclass(name = "OemCovarianceMatrix")]
#[derive(Clone)]
pub struct OemCovarianceMatrix {
    pub inner: core_oem::OemCovarianceMatrix,
}

#[pymethods]
impl Oem {
    #[new]
    fn new(header: OdmHeader, segments: Vec<OemSegment>) -> Self {
        Self {
            inner: core_oem::Oem {
                header: header.inner,
                body: core_oem::OemBody {
                    segment: segments.into_iter().map(|s| s.inner).collect(),
                },
                id: Some("CCSDS_OEM_VERS".to_string()),
                version: "3.0".to_string(),
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Oem(object_name='{}', segment={})",
            self.inner
                .body
                .segment
                .first()
                .map(|s| s.metadata.object_name.clone())
                .unwrap_or_default(),
            self.inner.body.segment.len()
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

    /// The list of data segments.
    ///
    /// :type: list[OemSegment]
    #[getter]
    fn get_segments(&self) -> Vec<OemSegment> {
        self.inner
            .body
            .segment
            .iter()
            .map(|s| OemSegment { inner: s.clone() })
            .collect()
    }

    #[setter]
    fn set_segments(&mut self, segments: Vec<OemSegment>) {
        self.inner.body.segment = segments.into_iter().map(|s| s.inner).collect();
    }

    /// Create an OEM message from a string.
    ///
    /// Parameters
    /// ----------
    /// data : str
    ///     Input string/content.
    /// format : str, optional
    ///     Format ('kvn' or 'xml'). Auto-detected if None.
    ///     (Optional)
    ///
    /// Returns
    /// -------
    /// Oem
    ///     The parsed OEM object.
    #[staticmethod]
    fn from_str(data: &str, format: Option<&str>) -> PyResult<Self> {
        let inner =
            match format {
                Some("kvn") => core_oem::Oem::from_kvn(data)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
                Some("xml") => core_oem::Oem::from_xml(data)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
                Some(other) => {
                    return Err(PyValueError::new_err(format!(
                        "Unsupported format '{}'. Use 'kvn' or 'xml'",
                        other
                    )))
                }
                None => match ccsds_ndm::from_str(data) {
                    Ok(MessageType::Oem(oem)) => oem,
                    Ok(other) => {
                        return Err(PyValueError::new_err(format!(
                            "Parsed message is not OEM (got {:?})",
                            other
                        )))
                    }
                    Err(e) => return Err(PyValueError::new_err(e.to_string())),
                },
            };
        Ok(Self { inner })
    }

    /// Create an OEM message from a file.
    ///
    /// Parameters
    /// ----------
    /// path : str
    ///     Path to the input file.
    /// format : str, optional
    ///     Format ('kvn' or 'xml'). Auto-detected if None.
    ///     (Optional)
    ///
    /// Returns
    /// -------
    /// Oem
    ///     The parsed OEM object.
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

#[pymethods]
impl OemSegment {
    #[new]
    fn new(metadata: OemMetadata, data: OemData) -> Self {
        Self {
            inner: core_oem::OemSegment {
                metadata: metadata.inner,
                data: data.inner,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "OemSegment(object_name='{}',start_time='{}',stop_time='{}')",
            self.inner.metadata.object_name,
            self.inner.metadata.start_time.as_str(),
            self.inner.metadata.stop_time.as_str()
        )
    }

    /// A single segment of the OEM.
    ///
    /// Each segment contains metadata (context) and a list of ephemeris data points.
    ///
    /// :type: OemMetadata
    #[getter]
    fn get_metadata(&self) -> OemMetadata {
        OemMetadata {
            inner: self.inner.metadata.clone(),
        }
    }

    #[setter]
    fn set_metadata(&mut self, metadata: OemMetadata) {
        self.inner.metadata = metadata.inner;
    }

    /// Segment data.
    ///
    /// :type: OemData
    #[getter]
    fn get_data(&self) -> OemData {
        OemData {
            inner: self.inner.data.clone(),
        }
    }

    #[setter]
    fn set_data(&mut self, data: OemData) {
        self.inner.data = data.inner;
    }
}

#[pymethods]
impl OemMetadata {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        object_name,
        object_id,
        start_time,
        stop_time,
        center_name=String::from("EARTH"),
        ref_frame=None,
        time_system=None,
        ref_frame_epoch=None,
        useable_start_time=None,
        useable_stop_time=None,
        interpolation=None,
        interpolation_degree=None,
        comment=None
    ))]
    fn new(
        object_name: String,
        object_id: String,
        start_time: String,
        stop_time: String,
        center_name: String,
        ref_frame: Option<Bound<'_, PyAny>>,
        time_system: Option<Bound<'_, PyAny>>,
        ref_frame_epoch: Option<String>,
        useable_start_time: Option<String>,
        useable_stop_time: Option<String>,
        interpolation: Option<String>,
        interpolation_degree: Option<u32>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let ref_frame = match ref_frame {
            Some(ref ob) => parse_reference_frame(ob)?,
            None => "GCRF".to_string(),
        };
        let time_system = match time_system {
            Some(ref ob) => parse_time_system(ob)?,
            None => "UTC".to_string(),
        };

        Ok(Self {

            inner: core_oem::OemMetadata {
                object_name,
                object_id,
                center_name,
                ref_frame,
                time_system,
                start_time: parse_epoch(&start_time)?,
                stop_time: parse_epoch(&stop_time)?,
                comment: comment.unwrap_or_default(),
                ref_frame_epoch: ref_frame_epoch.map(|s| parse_epoch(&s)).transpose()?,
                useable_start_time: useable_start_time.map(|s| parse_epoch(&s)).transpose()?,
                useable_stop_time: useable_stop_time.map(|s| parse_epoch(&s)).transpose()?,
                interpolation,
                interpolation_degree: interpolation_degree.and_then(NonZeroU32::new),
            },
        })
    }

    fn __repr__(&self) -> String {
        format!("OemMetadata(object_name='{}')", self.inner.object_name)
    }

    /// Spacecraft name for which ephemeris data is provided. While there is no CCSDS-based
    /// restriction on the value for this keyword, it is recommended to use names from the UN
    /// Office of Outer Space Affairs designator index (reference `[3]`, which include Object name
    /// and international designator of the participant). If OBJECT_NAME is not listed in
    /// reference `[3]` or the content is either unknown or cannot be disclosed, the value should
    /// be set to UNKNOWN.
    ///
    /// Examples: EUTELSAT W1, MARS PATHFINDER, STS 106, NEAR, UNKNOWN
    ///
    /// :type: str
    #[getter]
    fn get_object_name(&self) -> String {
        self.inner.object_name.clone()
    }

    #[setter]
    fn set_object_name(&mut self, object_name: String) {
        self.inner.object_name = object_name;
    }

    /// Object identifier of the object for which ephemeris data is provided. While there is no
    /// CCSDS-based restriction on the value for this keyword, it is recommended to use the
    /// international spacecraft designator as published in the UN Office of Outer Space Affairs
    /// designator index. Recommended values have the format YYYY-NNNP{PP}, where: YYYY = Year
    /// of launch. NNN = Three-digit serial number of launch in year YYYY (with leading zeros).
    /// P{PP} = At least one capital letter for the identification of the part brought into
    /// space by the launch. If the asset is not listed, the UN Office of Outer Space Affairs
    /// designator index format is not used, or the content is either unknown or cannot be
    /// disclosed, the value should be set to UNKNOWN.
    ///
    /// Examples: 2000-052A, 1996-068A, 2000-053A, 1996-008A, UNKNOWN
    ///
    /// :type: str
    #[getter]
    fn get_object_id(&self) -> String {
        self.inner.object_id.clone()
    }

    #[setter]
    fn set_object_id(&mut self, object_id: String) {
        self.inner.object_id = object_id;
    }

    /// Origin of the OEM reference frame, which may be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the
    /// solar system barycenter, or another reference frame center (such as a spacecraft,
    /// formation flying reference ‘chief’ spacecraft, etc.). Natural bodies shall be selected
    /// from the accepted set of values indicated in annex B, subsection B2. For spacecraft, it
    /// is recommended to use either the OBJECT_ID or international designator of the
    /// participant as catalogued in the UN Office of Outer Space Affairs designator index
    /// (reference `[3]`).
    ///
    /// Examples: EARTH, EARTH BARYCENTER, MOON, SOLAR SYSTEM BARYCENTER, SUN,
    /// JUPITER BARYCENTER, STS 106, EROS
    ///
    /// :type: str
    #[getter]
    fn get_center_name(&self) -> String {
        self.inner.center_name.clone()
    }

    #[setter]
    fn set_center_name(&mut self, center_name: String) {
        self.inner.center_name = center_name;
    }

    /// Reference frame in which the ephemeris data are given. Use of values other than those in
    /// 3.2.3.3 should be documented in an ICD.
    ///
    /// Examples: ICRF, ITRF2000, EME2000, TEME
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame(&self) -> String {
        self.inner.ref_frame.clone()
    }

    #[setter]
    fn set_ref_frame(&mut self, ref_frame: String) {
        self.inner.ref_frame = ref_frame;
    }

    /// Time system used for ephemeris and covariance data. Use of values other than those in
    /// 3.2.3.2 should be documented in an ICD.
    ///
    /// Examples: UTC, TAI, TT, GPS, TDB, TCB
    ///
    /// :type: str
    #[getter]
    fn get_time_system(&self) -> String {
        self.inner.time_system.clone()
    }

    #[setter]
    fn set_time_system(&mut self, time_system: String) {
        self.inner.time_system = time_system;
    }

    /// Start of TOTAL time span covered by ephemeris data and covariance data immediately
    /// following this metadata block. (For format specification, see 7.5.10.)
    ///
    /// Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
    ///
    /// :type: str
    #[getter]
    fn get_start_time(&self) -> String {
        self.inner.start_time.as_str().to_string()
    }

    #[setter]
    fn set_start_time(&mut self, start_time: String) -> PyResult<()> {
        self.inner.start_time = parse_epoch(&start_time)?;
        Ok(())
    }

    /// End of TOTAL time span covered by ephemeris data and covariance data immediately
    /// following this metadata block. (For format specification, see 7.5.10.)
    ///
    /// Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
    ///
    /// :type: str
    #[getter]
    fn get_stop_time(&self) -> String {
        self.inner.stop_time.as_str().to_string()
    }

    #[setter]
    fn set_stop_time(&mut self, stop_time: String) -> PyResult<()> {
        self.inner.stop_time = parse_epoch(&stop_time)?;
        Ok(())
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
    fn set_ref_frame_epoch(&mut self, ref_frame_epoch: Option<String>) -> PyResult<()> {
        self.inner.ref_frame_epoch = ref_frame_epoch.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Start time of USEABLE time span covered by ephemeris data immediately following this
    /// metadata block. (For format specification, see 7.5.10.) This optional keyword allows the
    /// message creator to introduce fictitious (but numerically smooth) data nodes prior to the
    /// actual data time history to support interpolation methods requiring more than two nodes
    /// (e.g., pure higher-order Lagrange interpolation methods). The use of this keyword and
    /// introduction of fictitious node points are optional and may not be necessary.
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
    fn set_useable_start_time(&mut self, useable_start_time: Option<String>) -> PyResult<()> {
        self.inner.useable_start_time = useable_start_time.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Stop time of USEABLE time span covered by ephemeris data immediately following this
    /// metadata block. (For format specification, see 7.5.10.) This optional keyword allows the
    /// message creator to introduce fictitious (but numerically smooth) data nodes following
    /// the actual data time history to support interpolation methods requiring more than two
    /// nodes (e.g., pure higher-order Lagrange interpolation methods). The use of this keyword
    /// and introduction of fictitious node points are optional and may not be necessary.
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
    fn set_useable_stop_time(&mut self, useable_stop_time: Option<String>) -> PyResult<()> {
        self.inner.useable_stop_time = useable_stop_time.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// This keyword may be used to specify the recommended interpolation method for ephemeris
    /// data in the immediately following set of ephemeris lines.
    ///
    /// Examples: HERMITE, LINEAR, LAGRANGE
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_interpolation(&self) -> Option<String> {
        self.inner.interpolation.clone()
    }

    #[setter]
    fn set_interpolation(&mut self, interpolation: Option<String>) {
        self.inner.interpolation = interpolation;
    }

    /// Recommended interpolation degree for ephemeris data in the immediately following set of
    /// ephemeris lines. Must be an integer value. This keyword must be used if the
    /// ‘INTERPOLATION’ keyword is used.
    ///
    /// Examples: 5, 8
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_interpolation_degree(&self) -> Option<u32> {
        self.inner.interpolation_degree.map(|d| d.get())
    }

    #[setter]
    fn set_interpolation_degree(&mut self, interpolation_degree: Option<u32>) {
        self.inner.interpolation_degree = interpolation_degree.and_then(NonZeroU32::new);
    }

    /// Comments (see 7.8 for formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comment(&mut self, comments: Vec<String>) {
        self.inner.comment = comments;
    }
}

#[pymethods]
impl OemData {
    #[new]
    fn new(state_vectors: Vec<StateVectorAcc>, comments: Option<Vec<String>>) -> Self {
        Self {
            inner: core_oem::OemData {
                state_vector: state_vectors.into_iter().map(|s| s.inner).collect(),
                comment: comments.unwrap_or_default(),
                covariance_matrix: vec![],
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "OemData(state_vector={}, covariance_matrix={})",
            self.inner.state_vector.len(),
            self.inner.covariance_matrix.len()
        )
    }

    /// List of state vectors. Each vector contains position, velocity, and optional
    /// acceleration.
    ///
    /// Examples: 2020-01-01T00:00:00.000 1234.567 2345.678 3456.789 1.234 2.345 3.456
    ///
    /// Units: km, km/s, km/s²
    ///
    /// :type: list[StateVectorAcc]
    #[getter]
    fn get_state_vector(&self) -> Vec<StateVectorAcc> {
        self.inner
            .state_vector
            .iter()
            .map(|sv| StateVectorAcc { inner: sv.clone() })
            .collect()
    }

    #[setter]
    fn set_state_vector(&mut self, state_vectors: Vec<StateVectorAcc>) {
        self.inner.state_vector = state_vectors.into_iter().map(|sv| sv.inner).collect();
    }

    /// List of covariance matrices associated with the state vectors.
    ///
    /// Each 6x6 covariance matrix provides uncertainty information for position and velocity:
    /// - Position covariance in km²
    /// - Position-velocity cross-covariance in km²/s
    /// - Velocity covariance in km²/s²
    ///
    /// Matrices are given in lower triangular form in the covariance reference frame.
    ///
    /// :type: list[OemCovarianceMatrix]
    #[getter]
    fn get_covariance_matrix(&self) -> Vec<OemCovarianceMatrix> {
        self.inner
            .covariance_matrix
            .iter()
            .map(|cm| OemCovarianceMatrix { inner: cm.clone() })
            .collect()
    }

    #[setter]
    fn set_covariance_matrix(&mut self, covariance_matrices: Vec<OemCovarianceMatrix>) {
        self.inner.covariance_matrix = covariance_matrices.into_iter().map(|cm| cm.inner).collect();
    }

    /// Comments (see 7.8 for formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comment(&mut self, comments: Vec<String>) {
        self.inner.comment = comments;
    }

    /// State vectors as a tuple of epochs and a NumPy array.
    ///
    /// This method allows for efficient zero-copy access to state vector data
    /// compatible with scientific Python libraries.
    ///
    /// Returns
    /// -------
    /// tuple[list[str], numpy.ndarray]
    ///     A tuple containing:
    ///     - List of epoch strings (ISO 8601 format).
    ///     - 2D NumPy array of shape (N, 6) or (N, 9):
    ///       - N x 6: [X, Y, Z, X_DOT, Y_DOT, Z_DOT] if no accelerations.
    ///       - N x 9: [X, Y, Z, X_DOT, Y_DOT, Z_DOT, X_DDOT, Y_DDOT, Z_DDOT] if accelerations present.
    ///
    ///     Units:
    ///     - Position: km
    ///     - Velocity: km/s
    ///     - Acceleration: km/s²
    ///
    /// :type: tuple[list[str], numpy.ndarray]
    #[getter]
    fn get_state_vector_numpy<'py>(&self, py: Python<'py>) -> (Vec<String>, Py<PyAny>) {
        let epochs: Vec<String> = self
            .inner
            .state_vector
            .iter()
            .map(|sv| sv.epoch.as_str().to_string())
            .collect();

        let has_accel = self
            .inner
            .state_vector
            .iter()
            .any(|sv| sv.x_ddot.is_some() || sv.y_ddot.is_some() || sv.z_ddot.is_some());

        let num_cols = if has_accel { 9 } else { 6 };
        let mut data = Vec::with_capacity(self.inner.state_vector.len() * num_cols);

        for sv in &self.inner.state_vector {
            data.push(sv.x.value);
            data.push(sv.y.value);
            data.push(sv.z.value);
            data.push(sv.x_dot.value);
            data.push(sv.y_dot.value);
            data.push(sv.z_dot.value);
            if has_accel {
                data.push(sv.x_ddot.as_ref().map_or(f64::NAN, |a| a.value));
                data.push(sv.y_ddot.as_ref().map_or(f64::NAN, |a| a.value));
                data.push(sv.z_ddot.as_ref().map_or(f64::NAN, |a| a.value));
            }
        }
        let array = PyArray::from_vec(py, data)
            .reshape([self.inner.state_vector.len(), num_cols])
            .unwrap();
        (epochs, array.into())
    }

    #[setter]
    fn set_state_vector_numpy(
        &mut self,
        value: (Vec<String>, PyReadonlyArray2<f64>),
    ) -> PyResult<()> {
        let (epochs, array) = value;
        let shape = array.shape();
        if shape.len() != 2 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "NumPy array must be 2-dimensional",
            ));
        }
        if epochs.len() != shape[0] {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Number of epochs must match number of rows in NumPy array",
            ));
        }
        if shape[1] != 6 && shape[1] != 9 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "NumPy array must have 6 or 9 columns",
            ));
        }

        let has_accel = shape[1] == 9;
        let mut state_vectors = Vec::with_capacity(shape[0]);

        let array_view = array.as_array();
        for (i, epoch_str) in epochs.iter().enumerate() {
            let row = array_view.row(i);
            state_vectors.push(ccsds_ndm::common::StateVectorAcc {
                epoch: parse_epoch(epoch_str)?,
                x: Position {
                    value: row[0],
                    units: None,
                },
                y: Position {
                    value: row[1],
                    units: None,
                },
                z: Position {
                    value: row[2],
                    units: None,
                },
                x_dot: Velocity {
                    value: row[3],
                    units: None,
                },
                y_dot: Velocity {
                    value: row[4],
                    units: None,
                },
                z_dot: Velocity {
                    value: row[5],
                    units: None,
                },
                x_ddot: if has_accel && !row[6].is_nan() {
                    Some(Acc {
                        value: row[6],
                        units: None,
                    })
                } else {
                    None
                },
                y_ddot: if has_accel && !row[7].is_nan() {
                    Some(Acc {
                        value: row[7],
                        units: None,
                    })
                } else {
                    None
                },
                z_ddot: if has_accel && !row[8].is_nan() {
                    Some(Acc {
                        value: row[8],
                        units: None,
                    })
                } else {
                    None
                },
            });
        }
        self.inner.state_vector = state_vectors;
        Ok(())
    }

    /// Get covariance matrices as a tuple associated with a NumPy array.
    ///
    /// Returns:
    ///     tuple[list[str], np.ndarray]: (Epochs, 2D Array of size Nx21).
    ///
    /// :type: tuple[list[str], numpy.ndarray]
    #[getter]
    fn get_covariance_matrix_numpy<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Vec<String>, Py<PyAny>)> {
        let epochs: Vec<String> = self
            .inner
            .covariance_matrix
            .iter()
            .map(|cm| cm.epoch.as_str().to_string())
            .collect();

        let num_matrices = self.inner.covariance_matrix.len();
        let mut data = Vec::with_capacity(num_matrices * 21);

        for cm in &self.inner.covariance_matrix {
            data.extend_from_slice(&[
                cm.cx_x.value,
                cm.cy_x.value,
                cm.cy_y.value,
                cm.cz_x.value,
                cm.cz_y.value,
                cm.cz_z.value,
                cm.cx_dot_x.value,
                cm.cx_dot_y.value,
                cm.cx_dot_z.value,
                cm.cx_dot_x_dot.value,
                cm.cy_dot_x.value,
                cm.cy_dot_y.value,
                cm.cy_dot_z.value,
                cm.cy_dot_x_dot.value,
                cm.cy_dot_y_dot.value,
                cm.cz_dot_x.value,
                cm.cz_dot_y.value,
                cm.cz_dot_z.value,
                cm.cz_dot_x_dot.value,
                cm.cz_dot_y_dot.value,
                cm.cz_dot_z_dot.value,
            ]);
        }

        let array = PyArray::from_vec(py, data).reshape([num_matrices, 21])?;
        Ok((epochs, array.into()))
    }

    #[setter]
    fn set_covariance_matrix_numpy(
        &mut self,
        value: (Vec<String>, PyReadonlyArray2<f64>),
    ) -> PyResult<()> {
        let (epochs, array) = value;
        let shape = array.shape();
        if shape.len() != 2 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "NumPy array must be 2-dimensional",
            ));
        }
        if epochs.len() != shape[0] {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Number of epochs must match number of rows in NumPy array",
            ));
        }
        if shape[1] != 21 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "NumPy array must have 21 columns",
            ));
        }

        let mut covariance_matrices = Vec::with_capacity(shape[0]);
        let array_view = array.as_array();

        for (i, epoch_str) in epochs.iter().enumerate() {
            let row = array_view.row(i);
            covariance_matrices.push(core_oem::OemCovarianceMatrix {
                epoch: parse_epoch(epoch_str)?,
                cx_x: PositionCovariance {
                    value: row[0],
                    units: None,
                },
                cy_x: PositionCovariance {
                    value: row[1],
                    units: None,
                },
                cy_y: PositionCovariance {
                    value: row[2],
                    units: None,
                },
                cz_x: PositionCovariance {
                    value: row[3],
                    units: None,
                },
                cz_y: PositionCovariance {
                    value: row[4],
                    units: None,
                },
                cz_z: PositionCovariance {
                    value: row[5],
                    units: None,
                },
                cx_dot_x: PositionVelocityCovariance {
                    value: row[6],
                    units: None,
                },
                cx_dot_y: PositionVelocityCovariance {
                    value: row[7],
                    units: None,
                },
                cx_dot_z: PositionVelocityCovariance {
                    value: row[8],
                    units: None,
                },
                cx_dot_x_dot: VelocityCovariance {
                    value: row[9],
                    units: None,
                },
                cy_dot_x: PositionVelocityCovariance {
                    value: row[10],
                    units: None,
                },
                cy_dot_y: PositionVelocityCovariance {
                    value: row[11],
                    units: None,
                },
                cy_dot_z: PositionVelocityCovariance {
                    value: row[12],
                    units: None,
                },
                cy_dot_x_dot: VelocityCovariance {
                    value: row[13],
                    units: None,
                },
                cy_dot_y_dot: VelocityCovariance {
                    value: row[14],
                    units: None,
                },
                cz_dot_x: PositionVelocityCovariance {
                    value: row[15],
                    units: None,
                },
                cz_dot_y: PositionVelocityCovariance {
                    value: row[16],
                    units: None,
                },
                cz_dot_z: PositionVelocityCovariance {
                    value: row[17],
                    units: None,
                },
                cz_dot_x_dot: VelocityCovariance {
                    value: row[18],
                    units: None,
                },
                cz_dot_y_dot: VelocityCovariance {
                    value: row[19],
                    units: None,
                },
                cz_dot_z_dot: VelocityCovariance {
                    value: row[20],
                    units: None,
                },
                comment: vec![],
                cov_ref_frame: None,
            });
        }
        self.inner.covariance_matrix = covariance_matrices;
        Ok(())
    }
}

#[pymethods]
impl OemCovarianceMatrix {
    #[new]
    fn new(
        epoch: String,
        values: PyReadonlyArray1<f64>,
        cov_ref_frame: Option<String>,
        comment: Vec<String>,
    ) -> PyResult<Self> {
        let shape = values.shape();
        if shape != [21] {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Covariance values must be a flat NumPy array of length 21.",
            ));
        }
        let v = values.as_array();

        Ok(Self {
            inner: core_oem::OemCovarianceMatrix {
                epoch: parse_epoch(&epoch)?,
                cov_ref_frame,
                comment,
                cx_x: PositionCovariance {
                    value: v[0],
                    units: None,
                },
                cy_x: PositionCovariance {
                    value: v[1],
                    units: None,
                },
                cy_y: PositionCovariance {
                    value: v[2],
                    units: None,
                },
                cz_x: PositionCovariance {
                    value: v[3],
                    units: None,
                },
                cz_y: PositionCovariance {
                    value: v[4],
                    units: None,
                },
                cz_z: PositionCovariance {
                    value: v[5],
                    units: None,
                },
                cx_dot_x: PositionVelocityCovariance {
                    value: v[6],
                    units: None,
                },
                cx_dot_y: PositionVelocityCovariance {
                    value: v[7],
                    units: None,
                },
                cx_dot_z: PositionVelocityCovariance {
                    value: v[8],
                    units: None,
                },
                cx_dot_x_dot: VelocityCovariance {
                    value: v[9],
                    units: None,
                },
                cy_dot_x: PositionVelocityCovariance {
                    value: v[10],
                    units: None,
                },
                cy_dot_y: PositionVelocityCovariance {
                    value: v[11],
                    units: None,
                },
                cy_dot_z: PositionVelocityCovariance {
                    value: v[12],
                    units: None,
                },
                cy_dot_x_dot: VelocityCovariance {
                    value: v[13],
                    units: None,
                },
                cy_dot_y_dot: VelocityCovariance {
                    value: v[14],
                    units: None,
                },
                cz_dot_x: PositionVelocityCovariance {
                    value: v[15],
                    units: None,
                },
                cz_dot_y: PositionVelocityCovariance {
                    value: v[16],
                    units: None,
                },
                cz_dot_z: PositionVelocityCovariance {
                    value: v[17],
                    units: None,
                },
                cz_dot_x_dot: VelocityCovariance {
                    value: v[18],
                    units: None,
                },
                cz_dot_y_dot: VelocityCovariance {
                    value: v[19],
                    units: None,
                },
                cz_dot_z_dot: VelocityCovariance {
                    value: v[20],
                    units: None,
                },
            },
        })
    }

    fn __repr__(&self) -> String {
        format!("OemCovarianceMatrix(epoch='{}')", self.inner.epoch.as_str())
    }

    /// Epoch of covariance matrix. (See 7.5.10 for formatting rules.)
    ///
    /// Examples: 2000-01-01T12:00:00Z
    ///
    /// :type: str
    #[getter]
    fn get_epoch(&self) -> String {
        self.inner.epoch.as_str().to_string()
    }

    #[setter]
    fn set_epoch(&mut self, epoch: String) -> PyResult<()> {
        self.inner.epoch = parse_epoch(&epoch)?;
        Ok(())
    }

    /// Reference frame in which the covariance data are given. Select from the accepted set of
    /// values indicated in 3.2.3.3 or 3.2.4.11.
    ///
    /// Examples: ICRF, EME2000
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_cov_ref_frame(&self) -> Option<String> {
        self.inner.cov_ref_frame.clone()
    }

    #[setter]
    fn set_cov_ref_frame(&mut self, cov_ref_frame: Option<String>) {
        self.inner.cov_ref_frame = cov_ref_frame;
    }

    /// Comments (see 7.8 for formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comment(&mut self, comments: Vec<String>) {
        self.inner.comment = comments;
    }

    /// Covariance matrix `[1,1]`
    ///
    /// Units: km²
    ///
    /// :type: float
    #[getter]
    fn get_cx_x(&self) -> f64 {
        self.inner.cx_x.value
    }

    #[setter]
    fn set_cx_x(&mut self, val: f64) {
        self.inner.cx_x.value = val;
    }

    /// Covariance matrix `[2,1]`
    ///
    /// Units: km²
    ///
    /// :type: float
    #[getter]
    fn get_cy_x(&self) -> f64 {
        self.inner.cy_x.value
    }

    #[setter]
    fn set_cy_x(&mut self, val: f64) {
        self.inner.cy_x.value = val;
    }

    /// Covariance matrix `[2,2]`
    ///
    /// Units: km²
    ///
    /// :type: float
    #[getter]
    fn get_cy_y(&self) -> f64 {
        self.inner.cy_y.value
    }

    #[setter]
    fn set_cy_y(&mut self, val: f64) {
        self.inner.cy_y.value = val;
    }

    /// Covariance matrix `[3,1]`
    ///
    /// Units: km²
    ///
    /// :type: float
    #[getter]
    fn get_cz_x(&self) -> f64 {
        self.inner.cz_x.value
    }

    #[setter]
    fn set_cz_x(&mut self, val: f64) {
        self.inner.cz_x.value = val;
    }

    /// Covariance matrix `[3,2]`
    ///
    /// Units: km²
    ///
    /// :type: float
    #[getter]
    fn get_cz_y(&self) -> f64 {
        self.inner.cz_y.value
    }

    #[setter]
    fn set_cz_y(&mut self, val: f64) {
        self.inner.cz_y.value = val;
    }

    /// Covariance matrix `[3,3]`
    ///
    /// Units: km²
    ///
    /// :type: float
    #[getter]
    fn get_cz_z(&self) -> f64 {
        self.inner.cz_z.value
    }

    #[setter]
    fn set_cz_z(&mut self, val: f64) {
        self.inner.cz_z.value = val;
    }

    /// Covariance matrix `[4,1]`
    ///
    /// Units: km²/s
    ///
    /// :type: float
    #[getter]
    fn get_cx_dot_x(&self) -> f64 {
        self.inner.cx_dot_x.value
    }

    #[setter]
    fn set_cx_dot_x(&mut self, val: f64) {
        self.inner.cx_dot_x.value = val;
    }

    /// Covariance matrix `[4,2]`
    ///
    /// Units: km²/s
    ///
    /// :type: float
    #[getter]
    fn get_cx_dot_y(&self) -> f64 {
        self.inner.cx_dot_y.value
    }

    #[setter]
    fn set_cx_dot_y(&mut self, val: f64) {
        self.inner.cx_dot_y.value = val;
    }

    /// Covariance matrix `[4,3]`
    ///
    /// Units: km²/s
    ///
    /// :type: float
    #[getter]
    fn get_cx_dot_z(&self) -> f64 {
        self.inner.cx_dot_z.value
    }

    #[setter]
    fn set_cx_dot_z(&mut self, val: f64) {
        self.inner.cx_dot_z.value = val;
    }

    /// Covariance matrix `[4,4]`
    ///
    /// Units: km²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cx_dot_x_dot(&self) -> f64 {
        self.inner.cx_dot_x_dot.value
    }

    #[setter]
    fn set_cx_dot_x_dot(&mut self, val: f64) {
        self.inner.cx_dot_x_dot.value = val;
    }

    /// Covariance matrix `[5,1]`
    ///
    /// Units: km²/s
    ///
    /// :type: float
    #[getter]
    fn get_cy_dot_x(&self) -> f64 {
        self.inner.cy_dot_x.value
    }

    #[setter]
    fn set_cy_dot_x(&mut self, val: f64) {
        self.inner.cy_dot_x.value = val;
    }

    /// Covariance matrix `[5,2]`
    ///
    /// Units: km²/s
    ///
    /// :type: float
    #[getter]
    fn get_cy_dot_y(&self) -> f64 {
        self.inner.cy_dot_y.value
    }

    #[setter]
    fn set_cy_dot_y(&mut self, val: f64) {
        self.inner.cy_dot_y.value = val;
    }

    /// Covariance matrix `[5,3]`
    ///
    /// Units: km²/s
    ///
    /// :type: float
    #[getter]
    fn get_cy_dot_z(&self) -> f64 {
        self.inner.cy_dot_z.value
    }

    #[setter]
    fn set_cy_dot_z(&mut self, val: f64) {
        self.inner.cy_dot_z.value = val;
    }

    /// Covariance matrix `[5,4]`
    ///
    /// Units: km²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cy_dot_x_dot(&self) -> f64 {
        self.inner.cy_dot_x_dot.value
    }

    #[setter]
    fn set_cy_dot_x_dot(&mut self, val: f64) {
        self.inner.cy_dot_x_dot.value = val;
    }

    /// Covariance matrix `[5,5]`
    ///
    /// Units: km²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cy_dot_y_dot(&self) -> f64 {
        self.inner.cy_dot_y_dot.value
    }

    #[setter]
    fn set_cy_dot_y_dot(&mut self, val: f64) {
        self.inner.cy_dot_y_dot.value = val;
    }

    /// Covariance matrix `[6,1]`
    ///
    /// Units: km²/s
    ///
    /// :type: float
    #[getter]
    fn get_cz_dot_x(&self) -> f64 {
        self.inner.cz_dot_x.value
    }

    #[setter]
    fn set_cz_dot_x(&mut self, val: f64) {
        self.inner.cz_dot_x.value = val;
    }

    /// Covariance matrix `[6,2]`
    ///
    /// Units: km²/s
    ///
    /// :type: float
    #[getter]
    fn get_cz_dot_y(&self) -> f64 {
        self.inner.cz_dot_y.value
    }

    #[setter]
    fn set_cz_dot_y(&mut self, val: f64) {
        self.inner.cz_dot_y.value = val;
    }

    /// Covariance matrix `[6,3]`
    ///
    /// Units: km²/s
    ///
    /// :type: float
    #[getter]
    fn get_cz_dot_z(&self) -> f64 {
        self.inner.cz_dot_z.value
    }

    #[setter]
    fn set_cz_dot_z(&mut self, val: f64) {
        self.inner.cz_dot_z.value = val;
    }

    /// Covariance matrix `[6,4]`
    ///
    /// Units: km²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cz_dot_x_dot(&self) -> f64 {
        self.inner.cz_dot_x_dot.value
    }

    #[setter]
    fn set_cz_dot_x_dot(&mut self, val: f64) {
        self.inner.cz_dot_x_dot.value = val;
    }

    /// Covariance matrix `[6,5]`
    ///
    /// Units: km²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cz_dot_y_dot(&self) -> f64 {
        self.inner.cz_dot_y_dot.value
    }

    #[setter]
    fn set_cz_dot_y_dot(&mut self, val: f64) {
        self.inner.cz_dot_y_dot.value = val;
    }

    /// Covariance matrix `[6,6]`
    ///
    /// Units: km²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cz_dot_z_dot(&self) -> f64 {
        self.inner.cz_dot_z_dot.value
    }

    #[setter]
    fn set_cz_dot_z_dot(&mut self, val: f64) {
        self.inner.cz_dot_z_dot.value = val;
    }
}
