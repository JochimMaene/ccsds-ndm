// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::MessageType;
use pyo3::prelude::*;
use pyo3::Py;

pub mod acm;
pub mod aem;
pub mod api;
pub mod apm;
pub mod attitude;
pub mod cdm;
pub mod common;
pub mod errors;
pub mod ndm;
pub mod ocm;
pub mod oem;
pub mod omm;
pub mod opm;
pub mod rdm;
pub mod tdm;
pub mod types;

use cdm::*;
use common::{
    AdmHeader, ControlledType, ObjectDescription, OdmHeader, ReferenceFrame, StateVector,
    StateVectorAcc, TimeSystem, YesNo,
};
use errors::{ccsds_error_to_pyerr, file_parse_error_to_pyerr};
use ndm::Ndm;
use oem::*;
use omm::*;
use opm::*;

pub(crate) fn message_to_py(py: Python<'_>, message: MessageType) -> PyResult<Py<PyAny>> {
    match message {
        MessageType::Oem(oem) => {
            let py_obj = Py::new(py, Oem::from_core(py, oem)?)?;
            Ok(py_obj.into_any())
        }
        MessageType::Cdm(cdm) => {
            let py_obj = Py::new(py, Cdm::from_core(py, cdm)?)?;
            Ok(py_obj.into_any())
        }
        MessageType::Opm(opm) => {
            let py_obj = Py::new(py, Opm::from_core(py, opm)?)?;
            Ok(py_obj.into_any())
        }
        MessageType::Omm(omm) => {
            let py_obj = Py::new(py, Omm::from_core(py, omm)?)?;
            Ok(py_obj.into_any())
        }
        MessageType::Ocm(ocm) => {
            let py_obj = Py::new(py, ocm::Ocm::from_core(py, ocm)?)?;
            Ok(py_obj.into_any())
        }
        MessageType::Rdm(rdm) => {
            let py_obj = Py::new(py, rdm::Rdm::from_core(py, rdm)?)?;
            Ok(py_obj.into_any())
        }
        MessageType::Tdm(tdm) => {
            let py_obj = Py::new(py, tdm::Tdm::from_core(py, tdm)?)?;
            Ok(py_obj.into_any())
        }
        MessageType::Ndm(ndm) => {
            let py_obj = Py::new(py, Ndm::from_core(py, ndm)?)?;
            Ok(py_obj.into_any())
        }
        MessageType::Aem(aem) => {
            let py_obj = Py::new(py, aem::Aem::from_core(py, aem)?)?;
            Ok(py_obj.into_any())
        }
        MessageType::Apm(apm) => {
            let py_obj = Py::new(py, apm::Apm::from_core(py, apm)?)?;
            Ok(py_obj.into_any())
        }
        MessageType::Acm(acm) => {
            let py_obj = Py::new(py, acm::Acm::from_core(py, acm)?)?;
            Ok(py_obj.into_any())
        }
    }
}

/// Parse a string containing KVN or XML.
///
/// Parameters
/// ----------
/// data : str
///     The content to parse.
/// format : str, optional
///     ``"kvn"`` or ``"xml"``. Detected automatically when omitted.
///
/// Returns
/// -------
/// Union[Oem, Cdm, Omm, Opm, Ocm, Tdm, Rdm, Ndm, Aem, Apm, Acm]
///     The parsed NDM object.
///
/// Raises
/// ------
/// ValueError
///     If the input is invalid or unsupported.
#[pyfunction]
#[pyo3(signature = (data, format=None, *, max_input_bytes=None, max_records=None))]
fn from_str(
    py: Python,
    data: &str,
    format: Option<&str>,
    max_input_bytes: Option<usize>,
    max_records: Option<usize>,
) -> PyResult<Py<PyAny>> {
    let options = api::parse_options(max_input_bytes, max_records);
    let message =
        ccsds_ndm::from_str_with_options(data, format.map(notation).transpose()?, &options)
            .map_err(ccsds_error_to_pyerr)?;
    message_to_py(py, message)
}

/// Parse from a file path (KVN or XML).
///
/// Parameters
/// ----------
/// path : str
///     Path to the file.
/// Returns
/// -------
/// Union[Oem, Cdm, Omm, Opm, Ocm, Tdm, Rdm, Ndm, Aem, Apm, Acm]
///     The parsed NDM object.
#[pyfunction]
#[pyo3(signature = (path, format=None, *, max_input_bytes=None, max_records=None))]
fn from_file(
    py: Python,
    path: &str,
    format: Option<&str>,
    max_input_bytes: Option<usize>,
    max_records: Option<usize>,
) -> PyResult<Py<PyAny>> {
    let options = api::parse_options(max_input_bytes, max_records);
    let notation = format.map(notation).transpose()?;
    let message = ccsds_ndm::from_file_with_options(path, notation, &options)
        .map_err(|error| file_parse_error_to_pyerr(error, notation, None))?;
    message_to_py(py, message)
}

fn notation(value: &str) -> PyResult<ccsds_ndm::Notation> {
    match value {
        "kvn" => Ok(ccsds_ndm::Notation::Kvn),
        "xml" => Ok(ccsds_ndm::Notation::Xml),
        other => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Unsupported format '{other}'. Use 'kvn' or 'xml'"
        ))),
    }
}

/// Convert any recognized NDM message between KVN and XML through the shared generation gate.
#[pyfunction]
#[pyo3(signature = (data, to_format, *, max_input_bytes=None, max_records=None, max_output_bytes=None, version=None))]
fn convert(
    data: &str,
    to_format: &str,
    max_input_bytes: Option<usize>,
    max_records: Option<usize>,
    max_output_bytes: Option<usize>,
    version: Option<&str>,
) -> PyResult<String> {
    let parse = api::parse_options(max_input_bytes, max_records);
    let mut generate = api::generate_options(version);
    generate.max_output_bytes = max_output_bytes;
    ccsds_ndm::convert_with_options(data, notation(to_format)?, &parse, &generate)
        .map_err(ccsds_error_to_pyerr)
}

/// Convert any recognized NDM file and atomically replace the destination on success.
#[pyfunction]
#[pyo3(signature = (source_path, destination_path, to_format, *, max_input_bytes=None, max_records=None, max_output_bytes=None, version=None))]
fn convert_file(
    source_path: &str,
    destination_path: &str,
    to_format: &str,
    max_input_bytes: Option<usize>,
    max_records: Option<usize>,
    max_output_bytes: Option<usize>,
    version: Option<&str>,
) -> PyResult<()> {
    let parse = api::parse_options(max_input_bytes, max_records);
    let mut generate = api::generate_options(version);
    generate.max_output_bytes = max_output_bytes;
    ccsds_ndm::convert_file_with_options(
        source_path,
        destination_path,
        notation(to_format)?,
        &parse,
        &generate,
    )
    .map_err(ccsds_error_to_pyerr)
}

/// The Python module definition.
#[pymodule]
#[pyo3(name = "ccsds_ndm")]
fn ccsds_ndm_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register exception types
    errors::register_exceptions(m)?;

    // High-level API aligned with Rust core
    m.add_function(wrap_pyfunction!(from_str, m)?)?;
    m.add_function(wrap_pyfunction!(from_file, m)?)?;
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    m.add_function(wrap_pyfunction!(convert_file, m)?)?;

    // Common types shared across message types
    m.add_class::<OdmHeader>()?;
    m.add_class::<AdmHeader>()?;
    m.add_class::<StateVector>()?;
    m.add_class::<StateVectorAcc>()?;

    // Register wrapper classes
    m.add_class::<Oem>()?;
    m.add_class::<OemSegment>()?;
    m.add_class::<OemMetadata>()?;
    m.add_class::<OemData>()?;
    m.add_class::<OemCovarianceMatrix>()?;

    // Register OMM wrapper classes
    m.add_class::<Omm>()?;
    m.add_class::<OmmSegment>()?;
    m.add_class::<OmmMetadata>()?;
    m.add_class::<MeanElements>()?;
    m.add_class::<OmmData>()?;
    m.add_class::<omm::TleParameters>()?;
    m.add_class::<common::SpacecraftParameters>()?;

    // Register OPM wrapper classes
    m.add_class::<Opm>()?;
    m.add_class::<OpmSegment>()?;
    m.add_class::<OpmMetadata>()?;
    m.add_class::<KeplerianElements>()?;
    m.add_class::<OpmCovarianceMatrix>()?;
    m.add_class::<OpmData>()?;
    m.add_class::<OpmManeuverParameters>()?;

    // Register OCM wrapper classes
    m.add_class::<ocm::Ocm>()?;
    m.add_class::<ocm::OcmSegment>()?;
    m.add_class::<ocm::OcmMetadata>()?;
    m.add_class::<ocm::OcmData>()?;
    m.add_class::<ocm::OcmTrajState>()?;
    m.add_class::<ocm::TrajLine>()?;
    m.add_class::<ocm::OcmPhysicalDescription>()?;
    m.add_class::<ocm::OcmCovarianceMatrix>()?;
    m.add_class::<ocm::CovLine>()?;
    m.add_class::<ocm::OcmManeuverParameters>()?;
    m.add_class::<ocm::ManLine>()?;
    m.add_class::<ocm::OcmPerturbations>()?;
    m.add_class::<ocm::OcmOdParameters>()?;
    m.add_class::<types::UserDefined>()?;

    // Register TDM wrapper classes
    m.add_class::<tdm::Tdm>()?;
    m.add_class::<tdm::TdmHeader>()?;
    m.add_class::<tdm::TdmBody>()?;
    m.add_class::<tdm::TdmSegment>()?;
    m.add_class::<tdm::TdmMetadata>()?;
    m.add_class::<tdm::TdmData>()?;
    m.add_class::<tdm::TdmObservation>()?;
    m.add_class::<tdm::TdmMode>()?;
    m.add_class::<tdm::TdmPath>()?;

    // Register RDM wrapper classes
    m.add_class::<rdm::Rdm>()?;
    m.add_class::<rdm::RdmHeader>()?;
    m.add_class::<rdm::RdmSegment>()?;
    m.add_class::<rdm::RdmMetadata>()?;
    m.add_class::<rdm::RdmData>()?;
    m.add_class::<rdm::AtmosphericReentryParameters>()?;
    m.add_class::<common::GroundImpactParameters>()?;
    m.add_class::<rdm::RdmSpacecraftParameters>()?;
    m.add_class::<common::OdParameters>()?;

    // Register NDM wrapper classes
    m.add_class::<Ndm>()?;

    // Register AEM wrapper classes
    m.add_class::<aem::Aem>()?;
    m.add_class::<aem::AemSegment>()?;
    m.add_class::<aem::AemMetadata>()?;
    m.add_class::<aem::AemData>()?;
    m.add_class::<aem::AttitudeState>()?;

    // Register APM wrapper classes
    m.add_class::<apm::Apm>()?;
    m.add_class::<apm::ApmSegment>()?;
    m.add_class::<apm::ApmMetadata>()?;
    m.add_class::<apm::ApmData>()?;
    m.add_class::<apm::ApmManeuverParameters>()?;

    // Register shared attitude states
    m.add_class::<attitude::QuaternionState>()?;
    m.add_class::<attitude::EulerAngleState>()?;
    m.add_class::<attitude::AngVelState>()?;
    m.add_class::<attitude::SpinState>()?;
    m.add_class::<attitude::InertiaState>()?;

    // Register ACM wrapper classes
    m.add_class::<acm::Acm>()?;
    m.add_class::<acm::AcmSegment>()?;
    m.add_class::<acm::AcmMetadata>()?;
    m.add_class::<acm::AcmData>()?;
    m.add_class::<acm::AcmAttitudeState>()?;
    m.add_class::<acm::AcmPhysicalDescription>()?;
    m.add_class::<acm::AcmCovarianceMatrix>()?;
    m.add_class::<acm::AcmManeuverParameters>()?;
    m.add_class::<acm::AcmSensor>()?;
    m.add_class::<acm::AcmAttitudeDetermination>()?;

    // Register CDM wrapper classes
    // CDM Classes
    m.add_class::<Cdm>()?;
    m.add_class::<CdmHeader>()?;
    m.add_class::<CdmBody>()?;
    m.add_class::<CdmSegment>()?;
    m.add_class::<CdmMetadata>()?;
    m.add_class::<CdmData>()?;
    m.add_class::<RelativeMetadataData>()?;
    m.add_class::<RelativeStateVector>()?;
    m.add_class::<CdmStateVector>()?;
    m.add_class::<CdmCovarianceMatrix>()?;
    m.add_class::<AdditionalParameters>()?;

    // CDM Enums
    m.add_class::<CdmObjectType>()?;
    m.add_class::<ScreenVolumeFrameType>()?;
    m.add_class::<ScreenVolumeShapeType>()?;
    m.add_class::<ReferenceFrameType>()?;
    m.add_class::<CovarianceMethodType>()?;
    m.add_class::<ManeuverableType>()?;
    m.add_class::<ObjectDescription>()?;
    m.add_class::<YesNo>()?;
    m.add_class::<ControlledType>()?;
    m.add_class::<ReferenceFrame>()?;
    m.add_class::<TimeSystem>()?;

    Ok(())
}
