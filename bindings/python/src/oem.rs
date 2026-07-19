// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{parse_reference_frame, parse_time_system};
use crate::common::{OdmHeader, StateVectorAcc};
use crate::types::{parse_calendar_epoch, parse_epoch};
use ccsds_ndm::messages::oem as core_oem;
use ccsds_ndm::types::{
    Acc, InterpolationDegree, Position, PositionCovariance, PositionVelocityCovariance, Velocity,
    VelocityCovariance,
};
use ccsds_ndm::ParseOptions;
use numpy::{PyArray, PyArrayMethods, PyReadonlyArray2, PyReadonlyArrayDyn, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use std::num::NonZeroU32;

fn build_covariance_matrix(
    epoch: ccsds_ndm::types::Epoch,
    cov_ref_frame: Option<String>,
    comment: Vec<String>,
    v: [f64; 21],
) -> core_oem::OemCovarianceMatrix {
    core_oem::OemCovarianceMatrix {
        epoch,
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
    }
}

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
///     NumPy array of shape (21,) containing the lower-triangular values, or (6,6) for
///     a full symmetric matrix.
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

    /// The message identifier.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_id(&self) -> Option<String> {
        self.inner.id.clone()
    }

    /// The message version.
    ///
    /// :type: str
    #[getter]
    fn get_version(&self) -> String {
        self.inner.version.clone()
    }

    #[setter]
    fn set_version(&mut self, value: String) -> PyResult<()> {
        crate::common::validate_version(ccsds_ndm::validation::MessageKind::Oem, &value)?;
        self.inner.version = value;
        Ok(())
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

    /// Validate the message against CCSDS rules.
    ///
    /// Parameters
    /// ----------
    /// strict : bool, optional
    ///     If True (default), raises ValueError on the first error found.
    ///     If False, returns a list of validation error messages (or None if valid).
    #[pyo3(signature = (strict=true))]
    fn validate(&self, strict: bool) -> PyResult<Option<Vec<String>>> {
        crate::api::validate_message(&self.inner, strict)
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
    #[pyo3(signature = (data, format=None, max_input_bytes=None, max_xml_depth=None, max_records=None))]
    fn from_str(
        _py: Python<'_>,
        data: &str,
        format: Option<&str>,
        max_input_bytes: Option<usize>,
        max_xml_depth: Option<usize>,
        max_records: Option<usize>,
    ) -> PyResult<Self> {
        let mut options = ParseOptions {
            max_input_bytes,
            max_records,
            ..ParseOptions::default()
        };
        if let Some(depth) = max_xml_depth {
            options.max_xml_depth = depth;
        }
        let inner = crate::api::parse_typed_with_options(data, format, &options)?;
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
    #[pyo3(signature = (path, format=None, max_input_bytes=None, max_xml_depth=None, max_records=None))]
    fn from_file(
        _py: Python<'_>,
        path: &str,
        format: Option<&str>,
        max_input_bytes: Option<usize>,
        max_xml_depth: Option<usize>,
        max_records: Option<usize>,
    ) -> PyResult<Self> {
        let mut options = ParseOptions {
            max_input_bytes,
            max_records,
            ..ParseOptions::default()
        };
        if let Some(depth) = max_xml_depth {
            options.max_xml_depth = depth;
        }
        let inner = crate::api::parse_typed_file_with_options(path, format, &options)?;
        Ok(Self { inner })
    }

    /// Serialize to KVN, preserving the source version by default.
    ///
    /// Pass ``version="latest"`` or an exact supported version to override it.
    #[pyo3(signature = (version=None, max_output_bytes=None))]
    fn to_kvn(&self, version: Option<&str>, max_output_bytes: Option<usize>) -> PyResult<String> {
        crate::api::generate_string_with_limit(&self.inner, "kvn", version, max_output_bytes)
    }

    /// Serialize to XML, preserving the source version by default.
    #[pyo3(signature = (version=None, max_output_bytes=None))]
    fn to_xml(&self, version: Option<&str>, max_output_bytes: Option<usize>) -> PyResult<String> {
        crate::api::generate_string_with_limit(&self.inner, "xml", version, max_output_bytes)
    }

    /// Serialize to validated KVN or XML.
    #[pyo3(signature = (format, version=None, max_output_bytes=None))]
    fn to_str(
        &self,
        format: &str,
        version: Option<&str>,
        max_output_bytes: Option<usize>,
    ) -> PyResult<String> {
        crate::api::generate_string_with_limit(&self.inner, format, version, max_output_bytes)
    }

    /// Write directly to a KVN or XML file.
    ///
    /// Parameters
    /// ----------
    /// path : str
    ///     Output file path.
    /// format : str
    ///     Output format ('kvn' or 'xml').
    /// version : str, optional
    ///     Source version by default, ``"latest"``, or an exact supported version.
    #[pyo3(signature = (path, format, version=None, max_output_bytes=None))]
    fn to_file(
        &self,
        path: &str,
        format: &str,
        version: Option<&str>,
        max_output_bytes: Option<usize>,
    ) -> PyResult<()> {
        crate::api::generate_file_with_limit(&self.inner, path, format, version, max_output_bytes)
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

    /// Validate the segment against CCSDS rules.
    fn validate(&self) -> PyResult<()> {
        use ccsds_ndm::traits::Validate;
        self.inner
            .validate()
            .map_err(|e| PyValueError::new_err(e.to_string()))
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
                ref_frame_epoch: ref_frame_epoch
                    .map(|s| parse_calendar_epoch(&s))
                    .transpose()?,
                useable_start_time: useable_start_time.map(|s| parse_epoch(&s)).transpose()?,
                useable_stop_time: useable_stop_time.map(|s| parse_epoch(&s)).transpose()?,
                interpolation,
                interpolation_degree: interpolation_degree
                    .and_then(NonZeroU32::new)
                    .map(InterpolationDegree),
            },
        })
    }

    fn __repr__(&self) -> String {
        format!("OemMetadata(object_name='{}')", self.inner.object_name)
    }

    /// Validate the metadata against CCSDS rules.
    fn validate(&self) -> PyResult<()> {
        use ccsds_ndm::traits::Validate;
        self.inner
            .validate()
            .map_err(|e| PyValueError::new_err(e.to_string()))
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
        self.inner.ref_frame_epoch = ref_frame_epoch
            .map(|s| parse_calendar_epoch(&s))
            .transpose()?;
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
        self.inner.interpolation_degree.map(|d| d.0.get())
    }

    #[setter]
    fn set_interpolation_degree(&mut self, interpolation_degree: Option<u32>) {
        self.inner.interpolation_degree = interpolation_degree
            .and_then(NonZeroU32::new)
            .map(InterpolationDegree);
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
    #[pyo3(signature = (state_vectors, covariance_matrix=None, comments=None))]
    fn new(
        state_vectors: Vec<StateVectorAcc>,
        covariance_matrix: Option<Vec<OemCovarianceMatrix>>,
        comments: Option<Vec<String>>,
    ) -> Self {
        Self {
            inner: core_oem::OemData {
                state_vector: state_vectors.into_iter().map(|s| s.inner).collect(),
                comment: comments.unwrap_or_default(),
                covariance_matrix: covariance_matrix
                    .unwrap_or_default()
                    .into_iter()
                    .map(|cm| cm.inner)
                    .collect(),
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

    /// Validate the data section against CCSDS rules.
    fn validate(&self) -> PyResult<()> {
        use ccsds_ndm::traits::Validate;
        self.inner
            .validate()
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[staticmethod]
    #[pyo3(signature = (
        state_vector_epochs,
        state_vector_numpy,
        covariance_matrix_epochs=None,
        covariance_matrix_numpy=None,
        cov_ref_frames=None,
        cov_comments=None,
        comments=None
    ))]
    fn from_numpy(
        state_vector_epochs: Vec<String>,
        state_vector_numpy: PyReadonlyArray2<f64>,
        covariance_matrix_epochs: Option<Vec<String>>,
        covariance_matrix_numpy: Option<PyReadonlyArrayDyn<f64>>,
        cov_ref_frames: Option<Vec<Option<String>>>,
        cov_comments: Option<Vec<Vec<String>>>,
        comments: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let shape = state_vector_numpy.shape();
        if shape.len() != 2 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "State vector array must be 2-dimensional",
            ));
        }
        if shape[1] != 6 && shape[1] != 9 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "State vector array must have 6 or 9 columns",
            ));
        }
        if state_vector_epochs.len() != shape[0] {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Number of state epochs must match number of rows in the array",
            ));
        }

        let has_accel = shape[1] == 9;
        let array_view = state_vector_numpy.as_array();
        let mut state_vectors = Vec::with_capacity(shape[0]);
        for (i, epoch_str) in state_vector_epochs.iter().enumerate() {
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

        let mut covariance_matrices = Vec::new();
        if covariance_matrix_epochs.is_some() || covariance_matrix_numpy.is_some() {
            let cov_epochs = covariance_matrix_epochs.ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "covariance_matrix_epochs is required when covariance_matrix_numpy is provided",
                )
            })?;
            let cov_array = covariance_matrix_numpy.ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "covariance_matrix_numpy is required when covariance_matrix_epochs is provided",
                )
            })?;

            let num_matrices = cov_epochs.len();
            let cov_ref_frames = cov_ref_frames.unwrap_or_else(|| vec![None; num_matrices]);
            if cov_ref_frames.len() != num_matrices {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "cov_ref_frames length must match number of covariance epochs",
                ));
            }
            let cov_comments = cov_comments.unwrap_or_else(|| vec![Vec::new(); num_matrices]);
            if cov_comments.len() != num_matrices {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "cov_comments length must match number of covariance epochs",
                ));
            }

            let shape = cov_array.shape();
            let array_view = cov_array.as_array();
            let expected = match shape.len() {
                3 => {
                    if shape[1] != 6 || shape[2] != 6 {
                        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                            "Covariance matrices must be 6x6",
                        ));
                    }
                    shape[0]
                }
                2 => {
                    if shape[1] == 21 {
                        shape[0]
                    } else if shape[0] == 6 && shape[1] == 6 {
                        1
                    } else {
                        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                            "Covariance array must be shaped (N,6,6), (N,21), (6,6), or (21,)",
                        ));
                    }
                }
                1 => {
                    if shape[0] == 21 {
                        1
                    } else {
                        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                            "Covariance array must be shaped (N,6,6), (N,21), (6,6), or (21,)",
                        ));
                    }
                }
                _ => {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "Covariance array must be 1-, 2-, or 3-dimensional",
                    ));
                }
            };
            if expected != num_matrices {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Number of covariance epochs must match number of matrices in the array",
                ));
            }

            covariance_matrices = Vec::with_capacity(num_matrices);
            for i in 0..num_matrices {
                let v: [f64; 21] = if shape.len() == 3 {
                    [
                        array_view[[i, 0, 0]],
                        array_view[[i, 1, 0]],
                        array_view[[i, 1, 1]],
                        array_view[[i, 2, 0]],
                        array_view[[i, 2, 1]],
                        array_view[[i, 2, 2]],
                        array_view[[i, 3, 0]],
                        array_view[[i, 3, 1]],
                        array_view[[i, 3, 2]],
                        array_view[[i, 3, 3]],
                        array_view[[i, 4, 0]],
                        array_view[[i, 4, 1]],
                        array_view[[i, 4, 2]],
                        array_view[[i, 4, 3]],
                        array_view[[i, 4, 4]],
                        array_view[[i, 5, 0]],
                        array_view[[i, 5, 1]],
                        array_view[[i, 5, 2]],
                        array_view[[i, 5, 3]],
                        array_view[[i, 5, 4]],
                        array_view[[i, 5, 5]],
                    ]
                } else if shape.len() == 2 && shape[1] == 21 {
                    [
                        array_view[[i, 0]],
                        array_view[[i, 1]],
                        array_view[[i, 2]],
                        array_view[[i, 3]],
                        array_view[[i, 4]],
                        array_view[[i, 5]],
                        array_view[[i, 6]],
                        array_view[[i, 7]],
                        array_view[[i, 8]],
                        array_view[[i, 9]],
                        array_view[[i, 10]],
                        array_view[[i, 11]],
                        array_view[[i, 12]],
                        array_view[[i, 13]],
                        array_view[[i, 14]],
                        array_view[[i, 15]],
                        array_view[[i, 16]],
                        array_view[[i, 17]],
                        array_view[[i, 18]],
                        array_view[[i, 19]],
                        array_view[[i, 20]],
                    ]
                } else if shape.len() == 1 {
                    [
                        array_view[[0]],
                        array_view[[1]],
                        array_view[[2]],
                        array_view[[3]],
                        array_view[[4]],
                        array_view[[5]],
                        array_view[[6]],
                        array_view[[7]],
                        array_view[[8]],
                        array_view[[9]],
                        array_view[[10]],
                        array_view[[11]],
                        array_view[[12]],
                        array_view[[13]],
                        array_view[[14]],
                        array_view[[15]],
                        array_view[[16]],
                        array_view[[17]],
                        array_view[[18]],
                        array_view[[19]],
                        array_view[[20]],
                    ]
                } else {
                    [
                        array_view[[0, 0]],
                        array_view[[1, 0]],
                        array_view[[1, 1]],
                        array_view[[2, 0]],
                        array_view[[2, 1]],
                        array_view[[2, 2]],
                        array_view[[3, 0]],
                        array_view[[3, 1]],
                        array_view[[3, 2]],
                        array_view[[3, 3]],
                        array_view[[4, 0]],
                        array_view[[4, 1]],
                        array_view[[4, 2]],
                        array_view[[4, 3]],
                        array_view[[4, 4]],
                        array_view[[5, 0]],
                        array_view[[5, 1]],
                        array_view[[5, 2]],
                        array_view[[5, 3]],
                        array_view[[5, 4]],
                        array_view[[5, 5]],
                    ]
                };

                covariance_matrices.push(build_covariance_matrix(
                    parse_epoch(&cov_epochs[i])?,
                    cov_ref_frames[i].clone(),
                    cov_comments[i].clone(),
                    v,
                ));
            }
        }

        Ok(Self {
            inner: core_oem::OemData {
                state_vector: state_vectors,
                comment: comments.unwrap_or_default(),
                covariance_matrix: covariance_matrices,
            },
        })
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

    /// Epochs for state vectors (ISO 8601).
    ///
    /// :type: list[str]
    #[getter]
    fn get_state_vector_epochs(&self) -> Vec<String> {
        self.inner
            .state_vector
            .iter()
            .map(|sv| sv.epoch.as_str().to_string())
            .collect()
    }

    #[setter]
    fn set_state_vector_epochs(&mut self, epochs: Vec<String>) -> PyResult<()> {
        if self.inner.state_vector.is_empty() {
            let mut state_vectors = Vec::with_capacity(epochs.len());
            for epoch_str in epochs {
                state_vectors.push(ccsds_ndm::common::StateVectorAcc {
                    epoch: parse_epoch(&epoch_str)?,
                    x: Position {
                        value: 0.0,
                        units: None,
                    },
                    y: Position {
                        value: 0.0,
                        units: None,
                    },
                    z: Position {
                        value: 0.0,
                        units: None,
                    },
                    x_dot: Velocity {
                        value: 0.0,
                        units: None,
                    },
                    y_dot: Velocity {
                        value: 0.0,
                        units: None,
                    },
                    z_dot: Velocity {
                        value: 0.0,
                        units: None,
                    },
                    x_ddot: None,
                    y_ddot: None,
                    z_ddot: None,
                });
            }
            self.inner.state_vector = state_vectors;
            return Ok(());
        }

        if epochs.len() != self.inner.state_vector.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Number of epochs must match number of state vectors",
            ));
        }

        for (sv, epoch_str) in self.inner.state_vector.iter_mut().zip(epochs.iter()) {
            sv.epoch = parse_epoch(epoch_str)?;
        }
        Ok(())
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

    /// Epochs for covariance matrices (ISO 8601).
    ///
    /// :type: list[str]
    #[getter]
    fn get_covariance_matrix_epochs(&self) -> Vec<String> {
        self.inner
            .covariance_matrix
            .iter()
            .map(|cm| cm.epoch.as_str().to_string())
            .collect()
    }

    #[setter]
    fn set_covariance_matrix_epochs(&mut self, epochs: Vec<String>) -> PyResult<()> {
        if self.inner.covariance_matrix.is_empty() {
            let mut covariance_matrices = Vec::with_capacity(epochs.len());
            for epoch_str in epochs {
                covariance_matrices.push(build_covariance_matrix(
                    parse_epoch(&epoch_str)?,
                    None,
                    Vec::new(),
                    [0.0; 21],
                ));
            }
            self.inner.covariance_matrix = covariance_matrices;
            return Ok(());
        }

        if epochs.len() != self.inner.covariance_matrix.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Number of epochs must match number of covariance matrices",
            ));
        }

        for (cm, epoch_str) in self.inner.covariance_matrix.iter_mut().zip(epochs.iter()) {
            cm.epoch = parse_epoch(epoch_str)?;
        }
        Ok(())
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

    /// State vectors as a NumPy array.
    ///
    /// Use `state_vector_epochs` for the corresponding epochs.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     2D array of shape (N, 6) or (N, 9):
    ///     - N x 6: [X, Y, Z, X_DOT, Y_DOT, Z_DOT] if no accelerations.
    ///     - N x 9: [X, Y, Z, X_DOT, Y_DOT, Z_DOT, X_DDOT, Y_DDOT, Z_DDOT] if accelerations present.
    ///
    /// Units:
    /// - Position: km
    /// - Velocity: km/s
    /// - Acceleration: km/s²
    ///
    /// :type: numpy.ndarray
    #[getter]
    fn get_state_vector_numpy<'py>(&self, py: Python<'py>) -> Py<PyAny> {
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
        array.into()
    }

    #[setter]
    fn set_state_vector_numpy(&mut self, array: PyReadonlyArray2<f64>) -> PyResult<()> {
        let shape = array.shape();
        if shape.len() != 2 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "NumPy array must be 2-dimensional",
            ));
        }
        if shape[1] != 6 && shape[1] != 9 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "NumPy array must have 6 or 9 columns",
            ));
        }
        if self.inner.state_vector.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "State vector epochs are missing; set state_vector_epochs or use from_numpy",
            ));
        }
        if self.inner.state_vector.len() != shape[0] {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Number of rows must match number of state vectors",
            ));
        }

        let has_accel = shape[1] == 9;
        let array_view = array.as_array();
        let mut state_vectors = Vec::with_capacity(shape[0]);

        for (i, existing) in self.inner.state_vector.iter().enumerate() {
            let row = array_view.row(i);
            state_vectors.push(ccsds_ndm::common::StateVectorAcc {
                epoch: existing.epoch,
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

    /// Get covariance matrices as a NumPy array.
    ///
    /// Use `covariance_matrix_epochs` for the corresponding epochs.
    ///
    /// The returned array is a 3D tensor of shape (N, 6, 6), where N is the number of covariance
    /// matrices. Each 6x6 matrix is symmetric and constructed from the lower-triangular CCSDS data.
    ///
    /// Indices: 0=X, 1=Y, 2=Z, 3=X_DOT, 4=Y_DOT, 5=Z_DOT
    ///
    /// :type: numpy.ndarray
    #[getter]
    fn get_covariance_matrix_numpy<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
        let num_matrices = self.inner.covariance_matrix.len();
        // 6x6 matrix = 36 elements per epoch
        let mut data = Vec::with_capacity(num_matrices * 36);

        for cm in &self.inner.covariance_matrix {
            // Row 0 (X)
            data.push(cm.cx_x.value); // 0,0
            data.push(cm.cy_x.value); // 0,1
            data.push(cm.cz_x.value); // 0,2
            data.push(cm.cx_dot_x.value); // 0,3
            data.push(cm.cy_dot_x.value); // 0,4
            data.push(cm.cz_dot_x.value); // 0,5

            // Row 1 (Y)
            data.push(cm.cy_x.value); // 1,0 (Symmetric)
            data.push(cm.cy_y.value); // 1,1
            data.push(cm.cz_y.value); // 1,2
            data.push(cm.cx_dot_y.value); // 1,3
            data.push(cm.cy_dot_y.value); // 1,4
            data.push(cm.cz_dot_y.value); // 1,5

            // Row 2 (Z)
            data.push(cm.cz_x.value); // 2,0 (Symmetric)
            data.push(cm.cz_y.value); // 2,1 (Symmetric)
            data.push(cm.cz_z.value); // 2,2
            data.push(cm.cx_dot_z.value); // 2,3
            data.push(cm.cy_dot_z.value); // 2,4
            data.push(cm.cz_dot_z.value); // 2,5

            // Row 3 (X_DOT)
            data.push(cm.cx_dot_x.value); // 3,0 (Symmetric)
            data.push(cm.cx_dot_y.value); // 3,1 (Symmetric)
            data.push(cm.cx_dot_z.value); // 3,2 (Symmetric)
            data.push(cm.cx_dot_x_dot.value); // 3,3
            data.push(cm.cy_dot_x_dot.value); // 3,4
            data.push(cm.cz_dot_x_dot.value); // 3,5

            // Row 4 (Y_DOT)
            data.push(cm.cy_dot_x.value); // 4,0 (Symmetric)
            data.push(cm.cy_dot_y.value); // 4,1 (Symmetric)
            data.push(cm.cy_dot_z.value); // 4,2 (Symmetric)
            data.push(cm.cy_dot_x_dot.value); // 4,3 (Symmetric)
            data.push(cm.cy_dot_y_dot.value); // 4,4
            data.push(cm.cz_dot_y_dot.value); // 4,5

            // Row 5 (Z_DOT)
            data.push(cm.cz_dot_x.value); // 5,0 (Symmetric)
            data.push(cm.cz_dot_y.value); // 5,1 (Symmetric)
            data.push(cm.cz_dot_z.value); // 5,2 (Symmetric)
            data.push(cm.cz_dot_x_dot.value); // 5,3 (Symmetric)
            data.push(cm.cz_dot_y_dot.value); // 5,4 (Symmetric)
            data.push(cm.cz_dot_z_dot.value); // 5,5
        }

        let array = PyArray::from_vec(py, data).reshape([num_matrices, 6, 6])?;
        Ok(array.into())
    }

    #[setter]
    fn set_covariance_matrix_numpy(&mut self, array: PyReadonlyArrayDyn<f64>) -> PyResult<()> {
        let shape = array.shape();
        let num_matrices = match shape.len() {
            3 => {
                if shape[1] != 6 || shape[2] != 6 {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "Covariance matrices must be 6x6",
                    ));
                }
                shape[0]
            }
            2 => {
                if shape[1] == 21 {
                    shape[0]
                } else if shape[0] == 6 && shape[1] == 6 {
                    1
                } else {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "NumPy array must be shaped (N,6,6), (N,21), (6,6), or (21,)",
                    ));
                }
            }
            1 => {
                if shape[0] == 21 {
                    1
                } else {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "NumPy array must be shaped (N,6,6), (N,21), (6,6), or (21,)",
                    ));
                }
            }
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "NumPy array must be 1-, 2-, or 3-dimensional",
                ));
            }
        };

        if self.inner.covariance_matrix.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Covariance epochs are missing; set covariance_matrix_epochs or use from_numpy",
            ));
        }
        if self.inner.covariance_matrix.len() != num_matrices {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Number of matrices must match number of covariance epochs",
            ));
        }

        let existing = self.inner.covariance_matrix.clone();
        let array_view = array.as_array();
        let mut covariance_matrices = Vec::with_capacity(num_matrices);

        for i in 0..num_matrices {
            let v: [f64; 21] = if shape.len() == 3 {
                [
                    array_view[[i, 0, 0]],
                    array_view[[i, 1, 0]],
                    array_view[[i, 1, 1]],
                    array_view[[i, 2, 0]],
                    array_view[[i, 2, 1]],
                    array_view[[i, 2, 2]],
                    array_view[[i, 3, 0]],
                    array_view[[i, 3, 1]],
                    array_view[[i, 3, 2]],
                    array_view[[i, 3, 3]],
                    array_view[[i, 4, 0]],
                    array_view[[i, 4, 1]],
                    array_view[[i, 4, 2]],
                    array_view[[i, 4, 3]],
                    array_view[[i, 4, 4]],
                    array_view[[i, 5, 0]],
                    array_view[[i, 5, 1]],
                    array_view[[i, 5, 2]],
                    array_view[[i, 5, 3]],
                    array_view[[i, 5, 4]],
                    array_view[[i, 5, 5]],
                ]
            } else if shape.len() == 2 && shape[1] == 21 {
                [
                    array_view[[i, 0]],
                    array_view[[i, 1]],
                    array_view[[i, 2]],
                    array_view[[i, 3]],
                    array_view[[i, 4]],
                    array_view[[i, 5]],
                    array_view[[i, 6]],
                    array_view[[i, 7]],
                    array_view[[i, 8]],
                    array_view[[i, 9]],
                    array_view[[i, 10]],
                    array_view[[i, 11]],
                    array_view[[i, 12]],
                    array_view[[i, 13]],
                    array_view[[i, 14]],
                    array_view[[i, 15]],
                    array_view[[i, 16]],
                    array_view[[i, 17]],
                    array_view[[i, 18]],
                    array_view[[i, 19]],
                    array_view[[i, 20]],
                ]
            } else if shape.len() == 1 {
                [
                    array_view[[0]],
                    array_view[[1]],
                    array_view[[2]],
                    array_view[[3]],
                    array_view[[4]],
                    array_view[[5]],
                    array_view[[6]],
                    array_view[[7]],
                    array_view[[8]],
                    array_view[[9]],
                    array_view[[10]],
                    array_view[[11]],
                    array_view[[12]],
                    array_view[[13]],
                    array_view[[14]],
                    array_view[[15]],
                    array_view[[16]],
                    array_view[[17]],
                    array_view[[18]],
                    array_view[[19]],
                    array_view[[20]],
                ]
            } else {
                [
                    array_view[[0, 0]],
                    array_view[[1, 0]],
                    array_view[[1, 1]],
                    array_view[[2, 0]],
                    array_view[[2, 1]],
                    array_view[[2, 2]],
                    array_view[[3, 0]],
                    array_view[[3, 1]],
                    array_view[[3, 2]],
                    array_view[[3, 3]],
                    array_view[[4, 0]],
                    array_view[[4, 1]],
                    array_view[[4, 2]],
                    array_view[[4, 3]],
                    array_view[[4, 4]],
                    array_view[[5, 0]],
                    array_view[[5, 1]],
                    array_view[[5, 2]],
                    array_view[[5, 3]],
                    array_view[[5, 4]],
                    array_view[[5, 5]],
                ]
            };

            let current = &existing[i];
            covariance_matrices.push(build_covariance_matrix(
                current.epoch,
                current.cov_ref_frame.clone(),
                current.comment.clone(),
                v,
            ));
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
        values: PyReadonlyArrayDyn<f64>,
        cov_ref_frame: Option<String>,
        comment: Vec<String>,
    ) -> PyResult<Self> {
        let shape = values.shape();
        let v: [f64; 21] = if shape.len() == 1 && shape[0] == 21 {
            let v = values.as_array();
            [
                v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10], v[11], v[12],
                v[13], v[14], v[15], v[16], v[17], v[18], v[19], v[20],
            ]
        } else if shape.len() == 2 && shape[0] == 6 && shape[1] == 6 {
            let v = values.as_array();
            [
                v[[0, 0]],
                v[[1, 0]],
                v[[1, 1]],
                v[[2, 0]],
                v[[2, 1]],
                v[[2, 2]],
                v[[3, 0]],
                v[[3, 1]],
                v[[3, 2]],
                v[[3, 3]],
                v[[4, 0]],
                v[[4, 1]],
                v[[4, 2]],
                v[[4, 3]],
                v[[4, 4]],
                v[[5, 0]],
                v[[5, 1]],
                v[[5, 2]],
                v[[5, 3]],
                v[[5, 4]],
                v[[5, 5]],
            ]
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Covariance values must be shape (21,) or (6,6).",
            ));
        };

        Ok(Self {
            inner: build_covariance_matrix(parse_epoch(&epoch)?, cov_ref_frame, comment, v),
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
