// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::MessageType;
use pyo3::prelude::*;
use pyo3::Py;
use std::fs;

pub mod cdm;
pub mod common;
pub mod ndm;
pub mod ocm;
pub mod oem;
pub mod omm;
pub mod opm;
pub mod rdm;
pub mod tdm;
pub mod types;
pub mod aem;
pub mod apm;
pub mod acm;
pub mod attitude;
pub mod errors;

use cdm::*;
use common::{OdmHeader, AdmHeader, StateVector, StateVectorAcc, ObjectDescription, YesNo, ControlledType, ReferenceFrame, TimeSystem};
use errors::ccsds_error_to_pyerr;
use ndm::Ndm;
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
    let message = ccsds_ndm::from_str(data).map_err(ccsds_error_to_pyerr)?;

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
        MessageType::Ndm(ndm) => {
            let py_obj = Py::new(py, Ndm { inner: ndm })?;
            Ok(py_obj.into_any())
        }
        MessageType::Aem(aem) => {
            let py_obj = Py::new(py, aem::Aem { inner: aem })?;
            Ok(py_obj.into_any())
        }
        MessageType::Apm(apm) => {
            let py_obj = Py::new(py, apm::Apm { inner: apm })?;
            Ok(py_obj.into_any())
        }
        MessageType::Acm(acm) => {
            let py_obj = Py::new(py, acm::Acm { inner: acm })?;
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
        .map_err(|e| errors::NdmIoError::new_err(e.to_string()))?;
    from_str(py, &content)
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
    m.add_class::<ManeuverParameters>()?;

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
    m.add_class::<apm::ManeuverParameters>()?;

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
