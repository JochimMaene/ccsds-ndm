"""
Mapping configuration for Python binding auditor.

Defines intentional differences between Rust core API and Python API,
fields that are Python-only additions, and fields to skip from validation.
"""

from __future__ import annotations

# ---------------------------------------------------------------------------
# Field Path Mappings
# ---------------------------------------------------------------------------
# Maps Python class fields to their corresponding Rust paths when they differ.
# Format: "PythonClass": {"python_field": "rust_path"}
#
# Example: Python `oem.segments` maps to Rust `oem.body.segment`

FIELD_MAPPINGS: dict[str, dict[str, str]] = {
    # OEM - Orbit Ephemeris Message
    "Oem": {
        "segments": "body.segment",  # Python flattens body.segment to segments
    },
    # OMM - Orbit Mean-Elements Message
    "Omm": {
        "segment": "body.segment",  # Single segment, not wrapped in body
    },
    # OPM - Orbit Parameter Message
    "Opm": {
        "segment": "body.segment",
    },
    # CDM - Conjunction Data Message (exposes body directly, no flattening)
    # OCM - Orbit Comprehensive Message
    "Ocm": {
        "segment": "body.segment",
    },
    # TDM - Tracking Data Message
    "Tdm": {
        "segment": "body.segment",
    },
    # RDM - Re-entry Data Message
    "Rdm": {
        "segment": "body.segment",
    },
    "RelativeMetadataData": {
        "relative_position": "relative_state_vector",
        "relative_velocity": "relative_state_vector",
    },
}

# ---------------------------------------------------------------------------
# Python-Only Fields
# ---------------------------------------------------------------------------
# Fields that exist in Python bindings but NOT in Rust core.
# These are typically performance optimizations (NumPy accessors) or
# convenience methods. The audit tool will not expect Rust docstrings for these.

PYTHON_ONLY_FIELDS: dict[str, list[str]] = {
    # OEM NumPy accessors
    "OemData": [
        "state_vector_numpy",
        "covariance_matrix_numpy",
    ],
    # OCM NumPy accessors
    "OcmTrajectoryStateHistory": [
        "state_numpy",
    ],
    "OcmCovarianceHistory": [
        "covariance_numpy",
    ],
    # CDM NumPy accessors
    "CdmCovarianceMatrix": [
        "to_numpy",
    ],
    # Add other NumPy accessors as needed
}

# ---------------------------------------------------------------------------
# Rust Fields to Skip
# ---------------------------------------------------------------------------
# Fields that exist in Rust but are intentionally NOT exposed in Python.
# Use "*" to skip entire struct (e.g., internal wrapper structs like OemBody).
#
# Common reasons to skip:
# - Internal wrappers (OemBody, OmmBody) - flattened in Python API
# - XML/serde attributes (@id, @version) - exposed via dedicated getters
# - Implementation details

RUST_SKIP_FIELDS: dict[str, list[str]] = {
    # Top-level message structs - skip internal fields
    "Oem": ["id", "version", "body"],  # body is flattened
    "Omm": ["id", "version", "body"],
    "Opm": ["id", "version", "body"],
    "Ocm": ["id", "version", "body"],
    "Cdm": [],  # CDM exposes body directly
    "Tdm": ["id", "version", "body"],
    "Rdm": ["id", "version", "body"],
    "TdmObservation": ["data"],
    # Internal body wrappers - skip entirely
    "OemBody": ["*"],
    "OmmBody": ["*"],
    "OpmBody": ["*"],
    "OcmBody": ["*"],
    "TdmBody": ["*"],
    "RdmBody": ["*"],
}

# ---------------------------------------------------------------------------
# Struct Name Mappings
# ---------------------------------------------------------------------------
# Maps Python class names to Rust struct names when they differ.
# Usually not needed as names are kept consistent.

STRUCT_MAPPINGS: dict[str, str] = {
    # "PythonClassName": "RustStructName"
}

# ---------------------------------------------------------------------------
# Known Compound Types
# ---------------------------------------------------------------------------
# Types that wrap a value with units (e.g., Position, Velocity).
# The audit tool uses this to understand type conversions.

TYPED_VALUE_WRAPPERS: set[str] = {
    "Position",
    "Velocity",
    "Acc",
    "PositionCovariance",
    "VelocityCovariance",
    "PositionVelocityCovariance",
    "Mass",
    "Area",
    "Angle",
    "AngularVelocity",
    "Epoch",
    "Duration",
    # Add more as needed
}

# ---------------------------------------------------------------------------
# Helper Functions
# ---------------------------------------------------------------------------


def get_rust_path(python_class: str, python_field: str) -> str:
    """Get the Rust path for a Python field, applying any mappings."""
    mappings = FIELD_MAPPINGS.get(python_class, {})
    return mappings.get(python_field, python_field)


def is_python_only(python_class: str, python_field: str) -> bool:
    """Check if a field is Python-only (no Rust equivalent expected)."""
    return python_field in PYTHON_ONLY_FIELDS.get(python_class, [])


def should_skip_rust_field(rust_struct: str, rust_field: str) -> bool:
    """Check if a Rust field should be skipped from Python binding validation."""
    skip_list = RUST_SKIP_FIELDS.get(rust_struct, [])
    return "*" in skip_list or rust_field in skip_list


def get_rust_struct_name(python_class: str) -> str:
    """Get the Rust struct name for a Python class."""
    return STRUCT_MAPPINGS.get(python_class, python_class)
