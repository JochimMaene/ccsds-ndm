// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{
    extract_records, parse_interpolation_degree, update_records, visit_records, OdmHeader,
    StateVectorAcc,
};
use crate::types::parse_epoch;
use ccsds_ndm::messages::oem as core_oem;
use ccsds_ndm::types::{
    Acc, Position, PositionCovariance, PositionVelocityCovariance, Velocity, VelocityCovariance,
};
use numpy::ndarray::ArrayViewD;
use numpy::{PyArray, PyArrayMethods, PyReadonlyArray2, PyReadonlyArrayDyn, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// The `(row, column)` positions of the CCSDS covariance lower triangle, in transmission order:
/// row by row, and within a row from the first column to the diagonal (`[1,1]`, `[2,1]`,
/// `[2,2]`, `[3,1]`, ...). Indices are zero-based.
///
/// [`lower_triangle!`] lists the record fields in this same order.
const LOWER_TRIANGLE: [(usize, usize); 21] = {
    let mut positions = [(0, 0); 21];
    let mut index = 0;
    let mut row = 0;
    while row < 6 {
        let mut column = 0;
        while column <= row {
            positions[index] = (row, column);
            index += 1;
            column += 1;
        }
        row += 1;
    }
    positions
};

/// The 21 lower-triangle values of a covariance record, in [`LOWER_TRIANGLE`] order.
///
/// `lower_triangle!(matrix)` reads the values; `lower_triangle!(mut matrix)` yields the slots to
/// write them into. This is the only place the record fields are listed in covariance order.
macro_rules! lower_triangle {
    (@($($borrow:tt)*) $matrix:expr) => {
        [
            $($borrow)* $matrix.cx_x.value,
            $($borrow)* $matrix.cy_x.value,
            $($borrow)* $matrix.cy_y.value,
            $($borrow)* $matrix.cz_x.value,
            $($borrow)* $matrix.cz_y.value,
            $($borrow)* $matrix.cz_z.value,
            $($borrow)* $matrix.cx_dot_x.value,
            $($borrow)* $matrix.cx_dot_y.value,
            $($borrow)* $matrix.cx_dot_z.value,
            $($borrow)* $matrix.cx_dot_x_dot.value,
            $($borrow)* $matrix.cy_dot_x.value,
            $($borrow)* $matrix.cy_dot_y.value,
            $($borrow)* $matrix.cy_dot_z.value,
            $($borrow)* $matrix.cy_dot_x_dot.value,
            $($borrow)* $matrix.cy_dot_y_dot.value,
            $($borrow)* $matrix.cz_dot_x.value,
            $($borrow)* $matrix.cz_dot_y.value,
            $($borrow)* $matrix.cz_dot_z.value,
            $($borrow)* $matrix.cz_dot_x_dot.value,
            $($borrow)* $matrix.cz_dot_y_dot.value,
            $($borrow)* $matrix.cz_dot_z_dot.value,
        ]
    };
    (mut $matrix:expr) => {
        lower_triangle!(@(&mut) $matrix)
    };
    ($matrix:expr) => {
        lower_triangle!(@() $matrix)
    };
}

fn build_covariance_matrix(
    epoch: ccsds_ndm::types::Epoch,
    cov_ref_frame: Option<String>,
    comment: Vec<String>,
    values: [f64; 21],
) -> core_oem::OemCovarianceMatrix {
    let position = || PositionCovariance::new(0.0, None);
    let cross = || PositionVelocityCovariance::new(0.0, None);
    let velocity = || VelocityCovariance::new(0.0, None);
    let mut matrix = core_oem::OemCovarianceMatrix {
        epoch,
        cov_ref_frame,
        comment,
        cx_x: position(),
        cy_x: position(),
        cy_y: position(),
        cz_x: position(),
        cz_y: position(),
        cz_z: position(),
        cx_dot_x: cross(),
        cx_dot_y: cross(),
        cx_dot_z: cross(),
        cx_dot_x_dot: velocity(),
        cy_dot_x: cross(),
        cy_dot_y: cross(),
        cy_dot_z: cross(),
        cy_dot_x_dot: velocity(),
        cy_dot_y_dot: velocity(),
        cz_dot_x: cross(),
        cz_dot_y: cross(),
        cz_dot_z: cross(),
        cz_dot_x_dot: velocity(),
        cz_dot_y_dot: velocity(),
        cz_dot_z_dot: velocity(),
    };
    write_covariance(&mut matrix, values);
    matrix
}

fn write_covariance(matrix: &mut core_oem::OemCovarianceMatrix, values: [f64; 21]) {
    for (slot, value) in lower_triangle!(mut matrix).into_iter().zip(values) {
        *slot = value;
    }
}

/// How a caller's covariance array lays out its matrices.
#[derive(Clone, Copy)]
enum CovarianceLayout {
    /// `(N, 6, 6)`: full matrices.
    Stacked,
    /// `(N, 21)`: lower triangles.
    StackedLower,
    /// `(6, 6)`: one full matrix.
    Single,
    /// `(21,)`: one lower triangle.
    SingleLower,
}

impl CovarianceLayout {
    /// Classify an array shape, returning the layout and the number of matrices it holds.
    fn of(shape: &[usize]) -> PyResult<(Self, usize)> {
        match *shape {
            [count, 6, 6] => Ok((Self::Stacked, count)),
            [count, 21] => Ok((Self::StackedLower, count)),
            [6, 6] => Ok((Self::Single, 1)),
            [21] => Ok((Self::SingleLower, 1)),
            _ => Err(PyValueError::new_err(format!(
                "Covariance array must be shaped (N,6,6), (N,21), (6,6), or (21,); got {shape:?}"
            ))),
        }
    }

    /// Read matrix `matrix` from `values` in [`LOWER_TRIANGLE`] order.
    ///
    /// Indexing the view element by element keeps strided arrays working. A CCSDS covariance is
    /// lower-triangular, and the full matrices callers pass in come out of numerical filters where
    /// `P` is symmetric only to within rounding. Comparing the two triangles for equality would
    /// reject those, so the lower triangle is authoritative and the upper one is not read.
    fn read(self, values: &ArrayViewD<'_, f64>, matrix: usize) -> [f64; 21] {
        std::array::from_fn(|index| {
            let (row, column) = LOWER_TRIANGLE[index];
            match self {
                Self::Stacked => values[[matrix, row, column]],
                Self::StackedLower => values[[matrix, index]],
                Self::Single => values[[row, column]],
                Self::SingleLower => values[[index]],
            }
        })
    }
}

fn state_vector_columns(shape: &[usize]) -> PyResult<bool> {
    match shape[1] {
        6 => Ok(false),
        9 => Ok(true),
        _ => Err(PyValueError::new_err(
            "State vector array must have 6 or 9 columns",
        )),
    }
}

/// Write one six/nine-column caller row into a state record in place, preserving the NaN-gap
/// convention for absent accelerations.
///
/// The column reader is a closure over the ndarray row view rather than a contiguous slice: six or
/// nine columns do not imply contiguous storage (e.g. `array[:, ::2]`).
fn write_state_row(
    state: &mut ccsds_ndm::common::StateVectorAcc,
    value_at: impl Fn(usize) -> f64,
    has_accel: bool,
) {
    state.x.value = value_at(0);
    state.y.value = value_at(1);
    state.z.value = value_at(2);
    state.x_dot.value = value_at(3);
    state.y_dot.value = value_at(4);
    state.z_dot.value = value_at(5);
    let write_accel = |slot: &mut Option<Acc>, index: usize| {
        let value = has_accel
            .then(|| value_at(index))
            .filter(|value| !value.is_nan());
        match (slot.as_mut(), value) {
            (Some(acc), Some(value)) => acc.value = value,
            (_, value) => *slot = value.map(|value| Acc::new(value, None)),
        }
    };
    write_accel(&mut state.x_ddot, 6);
    write_accel(&mut state.y_ddot, 7);
    write_accel(&mut state.z_ddot, 8);
}

fn state_vector_from_row(
    epoch: ccsds_ndm::types::Epoch,
    value_at: impl Fn(usize) -> f64,
    has_accel: bool,
) -> ccsds_ndm::common::StateVectorAcc {
    let mut state = ccsds_ndm::common::StateVectorAcc {
        epoch,
        x: Position::new(0.0, None),
        y: Position::new(0.0, None),
        z: Position::new(0.0, None),
        x_dot: Velocity::new(0.0, None),
        y_dot: Velocity::new(0.0, None),
        z_dot: Velocity::new(0.0, None),
        x_ddot: None,
        y_ddot: None,
        z_ddot: None,
    };
    write_state_row(&mut state, value_at, has_accel);
    state
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
#[gen_stub_pyclass]
#[pyclass]
pub struct Oem {
    /// The message identifier.
    ///
    /// :type: Optional[str]
    #[pyo3(get)]
    id: Option<String>,

    /// The message version.
    ///
    /// :type: str
    #[pyo3(get)]
    version: String,

    /// The message header.
    ///
    /// :type: OdmHeader
    #[pyo3(get, set)]
    header: Py<OdmHeader>,

    segments: Py<PyList>,
}

impl Oem {
    pub(crate) fn from_core(py: Python<'_>, value: core_oem::Oem) -> PyResult<Self> {
        let segments = value
            .body
            .segment
            .into_iter()
            .map(|segment| Py::new(py, OemSegment::from_core(py, segment)?))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            id: value.id,
            version: value.version,
            header: Py::new(
                py,
                OdmHeader {
                    inner: value.header,
                },
            )?,
            segments: PyList::new(py, segments)?.unbind(),
        })
    }

    pub(crate) fn to_core(&self, py: Python<'_>) -> PyResult<core_oem::Oem> {
        let segment = extract_records(
            self.segments.bind(py),
            "segments",
            |segment: &OemSegment| segment.to_core(py),
        )?;
        Ok(core_oem::Oem {
            id: self.id.clone(),
            version: self.version.clone(),
            header: self.header.borrow(py).inner.clone(),
            body: core_oem::OemBody { segment },
        })
    }
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
#[gen_stub_pyclass]
#[pyclass]
pub struct OemSegment {
    /// Segment metadata.
    ///
    /// :type: OemMetadata
    #[pyo3(get, set)]
    metadata: Py<OemMetadata>,

    /// Segment data.
    ///
    /// :type: OemData
    #[pyo3(get, set)]
    data: Py<OemData>,
}

impl OemSegment {
    fn from_core(py: Python<'_>, value: core_oem::OemSegment) -> PyResult<Self> {
        Ok(Self {
            metadata: Py::new(
                py,
                OemMetadata {
                    inner: value.metadata,
                },
            )?,
            data: Py::new(py, OemData::from_core(py, value.data)?)?,
        })
    }

    fn to_core(&self, py: Python<'_>) -> PyResult<core_oem::OemSegment> {
        Ok(core_oem::OemSegment {
            metadata: self.metadata.borrow(py).inner.clone(),
            data: self.data.borrow(py).to_core(py)?,
        })
    }
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
#[gen_stub_pyclass]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct OemMetadata {
    pub inner: core_oem::OemMetadata,
}

/// OEM Data Section.
///
/// Parameters
/// ----------
/// state_vectors : list[StateVectorAcc]
///     List of state vectors.
/// covariance_matrices : list[OemCovarianceMatrix], optional
///     Covariance matrices.
/// comments : list[str], optional
///     Comments.
#[gen_stub_pyclass]
#[pyclass]
pub struct OemData {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// CCSDS Reference: 502.0-B-3, Section 5.2.4.
    ///
    /// :type: list[str]
    #[pyo3(get, set)]
    comment: Vec<String>,

    state_vector: Py<PyList>,
    covariance_matrix: Py<PyList>,
}

impl OemData {
    fn from_core(py: Python<'_>, value: core_oem::OemData) -> PyResult<Self> {
        let state_vectors = value
            .state_vector
            .into_iter()
            .map(|inner| Py::new(py, StateVectorAcc { inner }))
            .collect::<PyResult<Vec<_>>>()?;
        let covariance_matrices = value
            .covariance_matrix
            .into_iter()
            .map(|inner| Py::new(py, OemCovarianceMatrix { inner }))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            comment: value.comment,
            state_vector: PyList::new(py, state_vectors)?.unbind(),
            covariance_matrix: PyList::new(py, covariance_matrices)?.unbind(),
        })
    }

    fn to_core(&self, py: Python<'_>) -> PyResult<core_oem::OemData> {
        Ok(core_oem::OemData {
            comment: self.comment.clone(),
            state_vector: extract_records(
                self.state_vector.bind(py),
                "state_vector",
                |state: &StateVectorAcc| Ok(state.inner.clone()),
            )?,
            covariance_matrix: extract_records(
                self.covariance_matrix.bind(py),
                "covariance_matrix",
                |matrix: &OemCovarianceMatrix| Ok(matrix.inner.clone()),
            )?,
        })
    }
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
/// values : numpy.ndarray
///     NumPy array of shape (21,) containing the lower-triangular values, or (6,6) for
///     a full symmetric matrix. Only the lower triangle of a (6,6) input is read, so a
///     matrix that is symmetric only to within rounding is accepted as-is.
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
#[gen_stub_pyclass]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct OemCovarianceMatrix {
    pub inner: core_oem::OemCovarianceMatrix,
}

#[gen_stub_pymethods]
#[pymethods]
impl Oem {
    #[new]
    fn new(py: Python<'_>, header: Py<OdmHeader>, segments: Vec<Py<OemSegment>>) -> PyResult<Self> {
        Ok(Self {
            header,
            segments: PyList::new(py, segments)?.unbind(),
            id: Some("CCSDS_OEM_VERS".to_string()),
            version: "3.0".to_string(),
        })
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        let segments = self.segments.bind(py);
        let object_name = segments
            .get_item(0)
            .ok()
            .and_then(|segment| segment.extract::<PyRef<'_, OemSegment>>().ok())
            .map(|segment| segment.metadata.borrow(py).inner.object_name.clone())
            .unwrap_or_default();
        format!(
            "Oem(object_name='{}', segment={})",
            object_name,
            segments.len()
        )
    }

    #[setter]
    fn set_version(&mut self, value: String) -> PyResult<()> {
        crate::common::validate_version(ccsds_ndm::validation::MessageKind::Oem, &value)?;
        self.version = value;
        Ok(())
    }

    /// The list of data segments.
    ///
    /// :type: list[OemSegment]
    #[gen_stub(override_return_type(type_repr = "list[OemSegment]"))]
    #[getter]
    fn get_segments(&self, py: Python<'_>) -> Py<PyList> {
        self.segments.clone_ref(py)
    }

    #[setter]
    fn set_segments(&mut self, segments: Vec<Py<OemSegment>>) -> PyResult<()> {
        Python::attach(|py| {
            self.segments = PyList::new(py, segments)?.unbind();
            Ok(())
        })
    }

    /// Validate the message against CCSDS rules.
    ///
    fn validate(&self, py: Python<'_>) -> PyResult<()> {
        crate::api::validate_message(&self.to_core(py)?)
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
    #[pyo3(signature = (data, format=None))]
    fn from_str(
        py: Python<'_>,
        data: &str,
        #[gen_stub(override_type(type_repr="typing.Optional[typing.Literal[\"kvn\", \"xml\"]]", imports=("typing")))]
        format: Option<&str>,
    ) -> PyResult<Self> {
        let inner = crate::api::parse_typed(data, format)?;
        Self::from_core(py, inner)
    }

    /// Parse an OEM from a KVN or XML file.
    #[staticmethod]
    #[pyo3(signature = (path, format=None))]
    fn from_file(
        py: Python<'_>,
        #[gen_stub(override_type(type_repr="builtins.str | os.PathLike[builtins.str]", imports=("builtins", "os")))]
        path: std::path::PathBuf,
        #[gen_stub(override_type(type_repr="typing.Optional[typing.Literal[\"kvn\", \"xml\"]]", imports=("typing")))]
        format: Option<&str>,
    ) -> PyResult<Self> {
        let inner = crate::api::parse_typed_file(&path, format)?;
        Self::from_core(py, inner)
    }

    /// Atomically write this OEM as KVN or XML.
    fn to_file(
        &self,
        py: Python<'_>,
        #[gen_stub(override_type(type_repr="builtins.str | os.PathLike[builtins.str]", imports=("builtins", "os")))]
        path: std::path::PathBuf,
        #[gen_stub(override_type(type_repr="typing.Literal[\"kvn\", \"xml\"]", imports=("typing")))]
        format: &str,
    ) -> PyResult<()> {
        crate::api::generate_file(&ccsds_ndm::Message::Oem(self.to_core(py)?), &path, format)
    }

    /// Serialize to validated KVN or XML.
    fn to_str(
        &self,
        py: Python<'_>,
        #[gen_stub(override_type(type_repr="typing.Literal[\"kvn\", \"xml\"]", imports=("typing")))]
        format: &str,
    ) -> PyResult<String> {
        crate::api::generate_string(&self.to_core(py)?, format)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl OemSegment {
    #[new]
    fn new(metadata: Py<OemMetadata>, data: Py<OemData>) -> Self {
        Self { metadata, data }
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        let metadata = self.metadata.borrow(py);
        format!(
            "OemSegment(object_name='{}',start_time='{}',stop_time='{}')",
            metadata.inner.object_name,
            metadata.inner.start_time.as_str(),
            metadata.inner.stop_time.as_str()
        )
    }

    /// Validate the segment against CCSDS rules.
    fn validate(&self, py: Python<'_>) -> PyResult<()> {
        crate::api::validate_message(&self.to_core(py)?)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl OemMetadata {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        object_name,
        object_id,
        start_time,
        stop_time,
        center_name,
        ref_frame,
        time_system,
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
        ref_frame: String,
        time_system: String,
        ref_frame_epoch: Option<String>,
        useable_start_time: Option<String>,
        useable_stop_time: Option<String>,
        interpolation: Option<String>,
        interpolation_degree: Option<u32>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
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
                interpolation_degree: parse_interpolation_degree(interpolation_degree)?,
            },
        })
    }

    fn __repr__(&self) -> String {
        format!("OemMetadata(object_name='{}')", self.inner.object_name)
    }

    /// Validate the metadata against CCSDS rules.
    fn validate(&self) -> PyResult<()> {
        crate::api::validate_message(&self.inner)
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.3.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.3.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.3.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.3.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.3.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.3.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.3.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.3.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.3.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.3.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.3.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.3.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_interpolation_degree(&self) -> Option<u32> {
        self.inner.interpolation_degree.map(|d| d.0.get())
    }

    #[setter]
    fn set_interpolation_degree(&mut self, interpolation_degree: Option<u32>) -> PyResult<()> {
        self.inner.interpolation_degree = parse_interpolation_degree(interpolation_degree)?;
        Ok(())
    }

    /// Comments (see 7.8 for formatting rules).
    ///
    /// CCSDS Reference: 502.0-B-3, Section 5.2.3.
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

#[gen_stub_pymethods]
#[pymethods]
impl OemData {
    #[new]
    #[pyo3(signature = (state_vectors, covariance_matrices=None, comments=None))]
    fn new(
        py: Python<'_>,
        state_vectors: Vec<Py<StateVectorAcc>>,
        covariance_matrices: Option<Vec<Py<OemCovarianceMatrix>>>,
        comments: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            state_vector: PyList::new(py, state_vectors)?.unbind(),
            comment: comments.unwrap_or_default(),
            covariance_matrix: PyList::new(py, covariance_matrices.unwrap_or_default())?.unbind(),
        })
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!(
            "OemData(state_vector={}, covariance_matrix={})",
            self.state_vector.bind(py).len(),
            self.covariance_matrix.bind(py).len()
        )
    }

    /// Validate the data section against CCSDS rules.
    fn validate(&self, py: Python<'_>) -> PyResult<()> {
        crate::api::validate_message(&self.to_core(py)?)
    }

    /// Create a data section from NumPy arrays.
    ///
    /// Parameters
    /// ----------
    /// state_vector_epochs : list[str]
    ///     Epoch of each state vector (ISO 8601), one per row of `state_vector_numpy`.
    /// state_vector_numpy : numpy.ndarray
    ///     Array of shape (N, 6) with [X, Y, Z, X_DOT, Y_DOT, Z_DOT], or (N, 9) with
    ///     accelerations [X_DDOT, Y_DDOT, Z_DDOT] appended. A NaN acceleration omits that
    ///     acceleration for the row. Units: km, km/s, km/s².
    /// covariance_matrix_epochs : list[str], optional
    ///     Epoch of each covariance matrix (ISO 8601). Required when any covariance argument
    ///     is given.
    /// covariance_matrix_numpy : numpy.ndarray, optional
    ///     Covariance matrices shaped (N, 6, 6) or (N, 21), or a single matrix shaped (6, 6) or
    ///     (21,). Values of length 21 are the lower triangle in CCSDS order; for full matrices
    ///     only the lower triangle is read. Required when `covariance_matrix_epochs` is given.
    /// cov_ref_frames : list[Optional[str]], optional
    ///     Reference frame of each covariance matrix.
    /// cov_comments : list[list[str]], optional
    ///     Comments of each covariance matrix.
    /// comments : list[str], optional
    ///     Comments for the data section.
    ///
    /// Returns
    /// -------
    /// OemData
    ///     The new data section.
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
    #[allow(clippy::too_many_arguments)]
    fn from_numpy(
        py: Python<'_>,
        state_vector_epochs: Vec<String>,
        state_vector_numpy: PyReadonlyArray2<f64>,
        covariance_matrix_epochs: Option<Vec<String>>,
        covariance_matrix_numpy: Option<PyReadonlyArrayDyn<f64>>,
        cov_ref_frames: Option<Vec<Option<String>>>,
        cov_comments: Option<Vec<Vec<String>>>,
        comments: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let shape = state_vector_numpy.shape();
        let has_accel = state_vector_columns(shape)?;
        if state_vector_epochs.len() != shape[0] {
            return Err(PyValueError::new_err(
                "Number of state epochs must match number of rows in the array",
            ));
        }

        let array_view = state_vector_numpy.as_array();
        let mut state_vectors = Vec::with_capacity(shape[0]);
        for (i, epoch_str) in state_vector_epochs.iter().enumerate() {
            let row = array_view.row(i);
            state_vectors.push(state_vector_from_row(
                parse_epoch(epoch_str)?,
                |index| row[index],
                has_accel,
            ));
        }

        let mut covariance_matrices = Vec::new();
        if covariance_matrix_epochs.is_some()
            || covariance_matrix_numpy.is_some()
            || cov_ref_frames.is_some()
            || cov_comments.is_some()
        {
            let cov_epochs = covariance_matrix_epochs.ok_or_else(|| {
                PyValueError::new_err(
                    "covariance_matrix_epochs is required when covariance values or metadata are provided",
                )
            })?;
            let cov_array = covariance_matrix_numpy.ok_or_else(|| {
                PyValueError::new_err(
                    "covariance_matrix_numpy is required when covariance_matrix_epochs is provided",
                )
            })?;

            let num_matrices = cov_epochs.len();
            let cov_ref_frames = cov_ref_frames.unwrap_or_else(|| vec![None; num_matrices]);
            if cov_ref_frames.len() != num_matrices {
                return Err(PyValueError::new_err(
                    "cov_ref_frames length must match number of covariance epochs",
                ));
            }
            let cov_comments = cov_comments.unwrap_or_else(|| vec![Vec::new(); num_matrices]);
            if cov_comments.len() != num_matrices {
                return Err(PyValueError::new_err(
                    "cov_comments length must match number of covariance epochs",
                ));
            }

            let (layout, count) = CovarianceLayout::of(cov_array.shape())?;
            if count != num_matrices {
                return Err(PyValueError::new_err(
                    "Number of covariance epochs must match number of matrices in the array",
                ));
            }

            let array_view = cov_array.as_array();
            covariance_matrices = Vec::with_capacity(num_matrices);
            for (i, ((epoch, cov_ref_frame), comment)) in cov_epochs
                .iter()
                .zip(cov_ref_frames)
                .zip(cov_comments)
                .enumerate()
            {
                covariance_matrices.push(build_covariance_matrix(
                    parse_epoch(epoch)?,
                    cov_ref_frame,
                    comment,
                    layout.read(&array_view, i),
                ));
            }
        }

        Self::from_core(
            py,
            core_oem::OemData {
                state_vector: state_vectors,
                comment: comments.unwrap_or_default(),
                covariance_matrix: covariance_matrices,
            },
        )
    }

    /// List of state vectors. Each vector contains position, velocity, and optional
    /// acceleration.
    ///
    /// Examples: 2020-01-01T00:00:00.000 1234.567 2345.678 3456.789 1.234 2.345 3.456
    ///
    /// Units: km, km/s, km/s²
    ///
    /// CCSDS Reference: 502.0-B-3, Section 5.2.4.
    ///
    /// :type: list[StateVectorAcc]
    #[gen_stub(override_return_type(type_repr = "list[StateVectorAcc]"))]
    #[getter]
    fn get_state_vector(&self, py: Python<'_>) -> Py<PyList> {
        self.state_vector.clone_ref(py)
    }

    #[setter]
    fn set_state_vector(&mut self, state_vectors: Vec<Py<StateVectorAcc>>) -> PyResult<()> {
        Python::attach(|py| {
            self.state_vector = PyList::new(py, state_vectors)?.unbind();
            Ok(())
        })
    }

    /// Epochs for state vectors (ISO 8601).
    ///
    /// :type: list[str]
    #[getter]
    fn get_state_vector_epochs(&self, py: Python<'_>) -> PyResult<Vec<String>> {
        extract_records(
            self.state_vector.bind(py),
            "state_vector",
            |state: &StateVectorAcc| Ok(state.inner.epoch.as_str().to_string()),
        )
    }

    #[setter]
    fn set_state_vector_epochs(&mut self, epochs: Vec<String>) -> PyResult<()> {
        Python::attach(|py| {
            let state_vectors = self.state_vector.bind(py);
            if state_vectors.is_empty() {
                return Err(PyValueError::new_err(
                    "Cannot set epochs when no state vectors exist; create states first",
                ));
            }
            if epochs.len() != state_vectors.len() {
                return Err(PyValueError::new_err(
                    "Number of epochs must match number of state vectors",
                ));
            }

            let parsed = epochs
                .iter()
                .map(|epoch| parse_epoch(epoch))
                .collect::<PyResult<Vec<_>>>()?;
            update_records(
                state_vectors,
                "state_vector",
                |index, state: &mut StateVectorAcc| state.inner.epoch = parsed[index],
            )
        })
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
    ///
    /// :type: list[OemCovarianceMatrix]
    #[gen_stub(override_return_type(type_repr = "list[OemCovarianceMatrix]"))]
    #[getter]
    fn get_covariance_matrix(&self, py: Python<'_>) -> Py<PyList> {
        self.covariance_matrix.clone_ref(py)
    }

    #[setter]
    fn set_covariance_matrix(
        &mut self,
        covariance_matrices: Vec<Py<OemCovarianceMatrix>>,
    ) -> PyResult<()> {
        Python::attach(|py| {
            self.covariance_matrix = PyList::new(py, covariance_matrices)?.unbind();
            Ok(())
        })
    }

    /// Epochs for covariance matrices (ISO 8601).
    ///
    /// :type: list[str]
    #[getter]
    fn get_covariance_matrix_epochs(&self, py: Python<'_>) -> PyResult<Vec<String>> {
        extract_records(
            self.covariance_matrix.bind(py),
            "covariance_matrix",
            |matrix: &OemCovarianceMatrix| Ok(matrix.inner.epoch.as_str().to_string()),
        )
    }

    #[setter]
    fn set_covariance_matrix_epochs(&mut self, epochs: Vec<String>) -> PyResult<()> {
        Python::attach(|py| {
            let covariance_matrices = self.covariance_matrix.bind(py);
            if covariance_matrices.is_empty() {
                return Err(PyValueError::new_err(
                    "Cannot set epochs when no covariance matrices exist; create matrices first",
                ));
            }
            if epochs.len() != covariance_matrices.len() {
                return Err(PyValueError::new_err(
                    "Number of epochs must match number of covariance matrices",
                ));
            }

            let parsed = epochs
                .iter()
                .map(|epoch| parse_epoch(epoch))
                .collect::<PyResult<Vec<_>>>()?;
            update_records(
                covariance_matrices,
                "covariance_matrix",
                |index, matrix: &mut OemCovarianceMatrix| matrix.inner.epoch = parsed[index],
            )
        })
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
    fn get_state_vector_numpy<'py>(&self, py: Python<'py>) -> PyResult<Py<numpy::PyArray2<f64>>> {
        let count = self.state_vector.bind(py).len();
        // Whether the array is six or nine columns wide is only known once every
        // record has been seen, but walking the Python list is the expensive part
        // and is worth doing exactly once. Accelerations are therefore collected
        // alongside the always-present columns and merged in afterwards, so the
        // common six-column case hands NumPy an exactly sized buffer.
        //
        // The buffer's capacity matters as well as its length: `PyArray::from_vec`
        // takes ownership of the allocation rather than copying it, so any excess
        // capacity would stay resident for as long as the array is alive.
        let mut states = Vec::with_capacity(count * 6);
        let mut accelerations = Vec::with_capacity(count * 3);
        let mut has_accel = false;
        visit_records(
            self.state_vector.bind(py),
            "state_vector",
            |state: &StateVectorAcc| {
                let sv = &state.inner;
                states.push(sv.x.value);
                states.push(sv.y.value);
                states.push(sv.z.value);
                states.push(sv.x_dot.value);
                states.push(sv.y_dot.value);
                states.push(sv.z_dot.value);
                has_accel |= sv.x_ddot.is_some() || sv.y_ddot.is_some() || sv.z_ddot.is_some();
                accelerations.push(sv.x_ddot.as_ref().map_or(f64::NAN, |a| a.value));
                accelerations.push(sv.y_ddot.as_ref().map_or(f64::NAN, |a| a.value));
                accelerations.push(sv.z_ddot.as_ref().map_or(f64::NAN, |a| a.value));
                Ok(())
            },
        )?;

        if !has_accel {
            drop(accelerations);
            let array = PyArray::from_vec(py, states).reshape([count, 6])?;
            return Ok(array.into());
        }

        // Records without accelerations contribute NaN, so that the column layout
        // stays uniform across the history.
        let mut data = Vec::with_capacity(count * 9);
        for row in 0..count {
            data.extend_from_slice(&states[row * 6..row * 6 + 6]);
            data.extend_from_slice(&accelerations[row * 3..row * 3 + 3]);
        }

        let array = PyArray::from_vec(py, data).reshape([count, 9])?;
        Ok(array.into())
    }

    #[setter]
    fn set_state_vector_numpy(&mut self, array: PyReadonlyArray2<f64>) -> PyResult<()> {
        Python::attach(|py| {
            let shape = array.shape();
            let has_accel = state_vector_columns(shape)?;
            let state_vectors = self.state_vector.bind(py);
            if state_vectors.is_empty() {
                return Err(PyValueError::new_err(
                    "No state vectors exist; create states or use from_numpy",
                ));
            }
            if state_vectors.len() != shape[0] {
                return Err(PyValueError::new_err(
                    "Number of rows must match number of state vectors",
                ));
            }

            let array_view = array.as_array();
            update_records(
                state_vectors,
                "state_vector",
                |index, state: &mut StateVectorAcc| {
                    let row = array_view.row(index);
                    write_state_row(&mut state.inner, |column| row[column], has_accel);
                },
            )
        })
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
    fn get_covariance_matrix_numpy<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Py<numpy::PyArray3<f64>>> {
        let num_matrices = self.covariance_matrix.bind(py).len();
        let mut data = Vec::with_capacity(num_matrices * 36);

        visit_records(
            self.covariance_matrix.bind(py),
            "covariance_matrix",
            |matrix: &OemCovarianceMatrix| {
                let mut full = [0.0; 36];
                for (&(row, column), value) in
                    LOWER_TRIANGLE.iter().zip(lower_triangle!(matrix.inner))
                {
                    full[row * 6 + column] = value;
                    full[column * 6 + row] = value;
                }
                data.extend_from_slice(&full);
                Ok(())
            },
        )?;

        let array = PyArray::from_vec(py, data).reshape([num_matrices, 6, 6])?;
        Ok(array.into())
    }

    #[setter]
    fn set_covariance_matrix_numpy(&mut self, array: PyReadonlyArrayDyn<f64>) -> PyResult<()> {
        Python::attach(|py| {
            let (layout, num_matrices) = CovarianceLayout::of(array.shape())?;
            let covariance_matrices = self.covariance_matrix.bind(py);
            if covariance_matrices.is_empty() {
                return Err(PyValueError::new_err(
                    "No covariance matrices exist; create matrices or use from_numpy",
                ));
            }
            if covariance_matrices.len() != num_matrices {
                return Err(PyValueError::new_err(
                    "Number of matrices must match number of covariance epochs",
                ));
            }

            let array_view = array.as_array();
            update_records(
                covariance_matrices,
                "covariance_matrix",
                |index, matrix: &mut OemCovarianceMatrix| {
                    write_covariance(&mut matrix.inner, layout.read(&array_view, index));
                },
            )
        })
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl OemCovarianceMatrix {
    #[new]
    #[pyo3(signature = (epoch, values, cov_ref_frame=None, comment=None))]
    fn new(
        epoch: String,
        values: PyReadonlyArrayDyn<f64>,
        cov_ref_frame: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let layout = match CovarianceLayout::of(values.shape()) {
            Ok((layout @ (CovarianceLayout::Single | CovarianceLayout::SingleLower), _)) => layout,
            _ => {
                return Err(PyValueError::new_err(
                    "Covariance values must be shape (21,) or (6,6).",
                ))
            }
        };

        Ok(Self {
            inner: build_covariance_matrix(
                parse_epoch(&epoch)?,
                cov_ref_frame,
                comment.unwrap_or_default(),
                layout.read(&values.as_array(), 0),
            ),
        })
    }

    fn __repr__(&self) -> String {
        format!("OemCovarianceMatrix(epoch='{}')", self.inner.epoch.as_str())
    }

    /// Epoch of covariance matrix. (See 7.5.10 for formatting rules.)
    ///
    /// Examples: 2000-01-01T12:00:00Z
    ///
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
    /// CCSDS Reference: 502.0-B-3, Section 5.2.5.
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
