// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::errors::{ccsds_error_to_pyerr, NdmParseWarning};
use ccsds_ndm::generation::VersionedNdm;
use ccsds_ndm::options::{GenerateOptions, ParseMode};
use ccsds_ndm::traits::{Ndm, Validate};
use pyo3::exceptions::{PyOSError, PyValueError};
use pyo3::prelude::*;
use std::path::Path;

pub fn parse_mode(strict: bool) -> ParseMode {
    if strict {
        ParseMode::Strict
    } else {
        ParseMode::Permissive
    }
}

pub fn parse_typed<T: Ndm>(
    py: Python<'_>,
    data: &str,
    format: Option<&str>,
    strict: bool,
) -> PyResult<T> {
    let mode = parse_mode(strict);
    let report = match format {
        Some("kvn") => T::from_kvn_with_mode(data, mode),
        Some("xml") => T::from_xml_with_mode(data, mode),
        Some(other) => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unsupported format '{other}'. Use 'kvn' or 'xml'",
            )))
        }
        None if data.trim_start().starts_with('<') => T::from_xml_with_mode(data, mode),
        None => T::from_kvn_with_mode(data, mode),
    }
    .map_err(ccsds_error_to_pyerr)?;

    emit_diagnostics(py, &report.diagnostics)?;
    Ok(report.message)
}

pub fn emit_diagnostics(
    py: Python<'_>,
    diagnostics: &[ccsds_ndm::validation::ValidationIssue],
) -> PyResult<()> {
    if diagnostics.is_empty() {
        return Ok(());
    }

    let warnings = py.import("warnings")?;
    let category = py.get_type::<NdmParseWarning>();
    for diagnostic in diagnostics {
        warnings.call_method1("warn", (diagnostic.error.to_string(), &category))?;
    }
    Ok(())
}

pub fn validate_message<T: Validate>(
    message: &T,
    strict: bool,
) -> PyResult<Option<Vec<String>>> {
    if strict {
        message
            .validate()
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        return Ok(None);
    }

    let errors = message
        .validation_errors()
        .map_err(ccsds_error_to_pyerr)?;
    if errors.is_empty() {
        Ok(None)
    } else {
        Ok(Some(
            errors
                .into_iter()
                .map(|error| error.to_string())
                .collect(),
        ))
    }
}

pub fn generate_options(version: Option<&str>) -> GenerateOptions {
    match version {
        None => GenerateOptions::source(),
        Some("latest") => GenerateOptions::latest(),
        Some(version) => GenerateOptions::version(version),
    }
}

pub fn generate_string<T: VersionedNdm>(
    message: &T,
    format: &str,
    version: Option<&str>,
) -> PyResult<String> {
    let options = generate_options(version);
    match format {
        "kvn" => message.to_kvn_with(&options).map_err(ccsds_error_to_pyerr),
        "xml" => message.to_xml_with(&options).map_err(ccsds_error_to_pyerr),
        other => Err(unsupported_format(other)),
    }
}

pub fn generate_file<T: VersionedNdm>(
    message: &T,
    path: &str,
    format: &str,
    version: Option<&str>,
) -> PyResult<()> {
    let options = generate_options(version);
    if format != "kvn" && format != "xml" {
        return Err(unsupported_format(format));
    }

    atomic_write(path, |output| {
        match format {
            "kvn" => message.write_kvn_to(output, &options),
            "xml" => message.write_xml_to(output, &options),
            _ => unreachable!(),
        }
        .map_err(ccsds_error_to_pyerr)
    })
}

pub fn atomic_write(
    path: &str,
    write: impl FnOnce(&mut std::fs::File) -> PyResult<()>,
) -> PyResult<()> {
    let destination = Path::new(path);
    let parent = destination
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut builder = tempfile::Builder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let permissions = destination
            .metadata()
            .map(|metadata| metadata.permissions())
            .unwrap_or_else(|_| std::fs::Permissions::from_mode(0o666));
        builder.permissions(permissions);
    }
    let mut output = builder
        .tempfile_in(parent)
        .map_err(|error| PyOSError::new_err(error.to_string()))?;

    write(output.as_file_mut())?;

    output
        .as_file_mut()
        .sync_all()
        .map_err(|error| PyOSError::new_err(error.to_string()))?;
    output
        .persist(destination)
        .map_err(|error| PyOSError::new_err(error.error.to_string()))?;
    Ok(())
}

pub fn require_checked_generation(validate: bool) -> PyResult<()> {
    if validate {
        Ok(())
    } else {
        Err(PyValueError::new_err(
            "unchecked generation is not supported; generated messages must be CCSDS-compliant",
        ))
    }
}

pub fn unsupported_format(format: &str) -> PyErr {
    PyValueError::new_err(format!("Unsupported format '{format}'. Use 'kvn' or 'xml'",))
}
