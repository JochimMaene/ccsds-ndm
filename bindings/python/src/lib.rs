// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::MessageType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::Py;
use std::fs;

pub mod cdm;
pub mod common;
pub mod ocm;
pub mod oem;
pub mod omm;
pub mod opm;
pub mod rdm;
pub mod tdm;
pub mod types;

use cdm::*;
use common::{OdmHeader, StateVector, StateVectorAcc};
use oem::*;
use omm::*;
use opm::*;

/// Parse a string (KVN or XML) and return the corresponding NDM object.
///
/// Parameters
/// ----------
/// data : str
///     The content to parse.
///
/// Returns
/// -------
/// Union[Oem, Cdm, Omm, Opm, Ocm, Tdm, Rdm]
///     The parsed NDM object.
///
/// Raises
/// ------
/// ValueError
///     If parsing fails.
#[pyfunction]
fn from_str(py: Python, data: &str) -> PyResult<Py<PyAny>> {
    // Call the core library's auto-detection function
    let message = ccsds_ndm::from_str(data)
        .map_err(|e| PyValueError::new_err(format!("Parsing failed: {}", e)))?;

    match message {
        MessageType::Oem(oem) => {
            let py_obj = Py::new(py, Oem { inner: oem })?;
            Ok(py_obj.into_any())
        }
        MessageType::Cdm(cdm) => {
            let py_obj = Py::new(py, Cdm { inner: cdm })?;
            Ok(py_obj.into_any())
        }
        MessageType::Opm(opm) => {
            let py_obj = Py::new(py, Opm { inner: opm })?;
            Ok(py_obj.into_any())
        }
        MessageType::Omm(omm) => {
            let py_obj = Py::new(py, Omm { inner: omm })?;
            Ok(py_obj.into_any())
        }
        MessageType::Ocm(ocm) => {
            let py_obj = Py::new(py, ocm::Ocm { inner: ocm })?;
            Ok(py_obj.into_any())
        }
        MessageType::Rdm(rdm) => {
            let py_obj = Py::new(py, rdm::Rdm { inner: rdm })?;
            Ok(py_obj.into_any())
        }
        MessageType::Tdm(tdm) => {
            let py_obj = Py::new(py, tdm::Tdm { inner: tdm })?;
            Ok(py_obj.into_any())
        }
    }
}

/// Parse from a file path (KVN or XML).
///
/// Parameters
/// ----------
/// path : str
///     Path to the file.
///
/// Returns
/// -------
/// Union[Oem, Cdm, Omm, Opm, Ocm, Tdm, Rdm]
///     The parsed NDM object.
#[pyfunction]
fn from_file(py: Python, path: &str) -> PyResult<Py<PyAny>> {
    let content = fs::read_to_string(path)
        .map_err(|e| PyValueError::new_err(format!("Failed to read file: {}", e)))?;
    from_str(py, &content)
}

/// The Python module definition.
#[pymodule]
#[pyo3(name = "ccsds_ndm")]
fn ccsds_ndm_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // High-level API aligned with Rust core
    m.add_function(wrap_pyfunction!(from_str, m)?)?;
    m.add_function(wrap_pyfunction!(from_file, m)?)?;

    // Common types shared across message types
    m.add_class::<OdmHeader>()?;
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

    // Register OPM wrapper classes
    m.add_class::<Opm>()?;
    m.add_class::<OpmSegment>()?;
    m.add_class::<OpmMetadata>()?;
    m.add_class::<KeplerianElements>()?;
    m.add_class::<OpmCovarianceMatrix>()?;
    m.add_class::<OpmData>()?;

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
    m.add_class::<ocm::OcmManeuver>()?;
    m.add_class::<ocm::ManLine>()?;
    m.add_class::<ocm::OcmPerturbations>()?;
    m.add_class::<ocm::OcmOdParameters>()?;
    m.add_class::<ocm::UserDefined>()?;

    // Register TDM wrapper classes
    m.add_class::<tdm::Tdm>()?;
    m.add_class::<tdm::TdmHeader>()?;
    m.add_class::<tdm::TdmBody>()?;
    m.add_class::<tdm::TdmSegment>()?;
    m.add_class::<tdm::TdmMetadata>()?;
    m.add_class::<tdm::TdmData>()?;
    m.add_class::<tdm::TdmObservation>()?;

    // Register RDM wrapper classes
    m.add_class::<rdm::Rdm>()?;
    m.add_class::<rdm::RdmHeader>()?;
    m.add_class::<rdm::RdmSegment>()?;
    m.add_class::<rdm::RdmMetadata>()?;
    m.add_class::<rdm::RdmData>()?;
    m.add_class::<rdm::AtmosphericReentryParameters>()?;
    m.add_class::<rdm::GroundImpactParameters>()?;
    m.add_class::<rdm::RdmSpacecraftParameters>()?;

    // Register CDM wrapper classes
    // CDM Classes
    m.add_class::<Cdm>()?;
    m.add_class::<CdmHeader>()?;
    m.add_class::<CdmBody>()?;
    m.add_class::<CdmSegment>()?;
    m.add_class::<CdmMetadata>()?;
    m.add_class::<CdmData>()?;
    m.add_class::<RelativeMetadataData>()?;
    m.add_class::<CdmStateVector>()?;
    m.add_class::<CdmCovarianceMatrix>()?;

    // CDM Enums
    m.add_class::<CdmObjectType>()?;
    m.add_class::<ScreenVolumeFrameType>()?;
    m.add_class::<ScreenVolumeShapeType>()?;
    m.add_class::<ReferenceFrameType>()?;
    m.add_class::<CovarianceMethodType>()?;
    m.add_class::<ManeuverableType>()?;
    m.add_class::<ObjectDescription>()?;

    Ok(())
}
