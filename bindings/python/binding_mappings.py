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
    # AEM - Attitude Ephemeris Message
    "Aem": {
        "segments": "body.segment",
    },
    # APM - Attitude Parameter Message
    "Apm": {
        "segment": "body.segment",
    },
    # ACM - Attitude Comprehensive Message
    "Acm": {
        "segment": "body.segment",
    },
    "RelativeMetadataData": {
        "relative_state_vector": "relative_state_vector",
    },
    "ManeuverParameters": {
        "man_epoch_start": "man_epoch_ignition",
        "man_tor_x": "man_tor_1",
        "man_tor_y": "man_tor_2",
        "man_tor_z": "man_tor_3",
    },
    "QuaternionState": {
        "q1": "quaternion",
        "q2": "quaternion",
        "q3": "quaternion",
        "qc": "quaternion",
    },
}

# ---------------------------------------------------------------------------
# Python Helper Classes
# ---------------------------------------------------------------------------
# Classes that exist in Python bindings but do NOT have a direct 1:1 matching
# Rust struct in the core library. These are typically convenience wrappers,
# projections, or merged types.
# The audit tool will not look for a matching Rust struct for these.

PYTHON_HELPER_CLASSES: set[str] = {
    "AttitudeState",
    "CovLine",
    "AttLine",
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
        "state_vector_epochs",
        "covariance_matrix_numpy",
        "covariance_matrix_epochs",
        "from_numpy",
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
        "from_numpy",
    ],
    "CdmStateVector": [
        "from_numpy",
    ],
    "CdmData": [
        "state_vector_numpy",
        "covariance_matrix_numpy",
        "from_numpy",
    ],
    "AemData": [
        "attitude_states",
        "attitude_states_numpy",
        "attitude_states_epochs",
        "from_numpy",
    ],
    # Add other NumPy accessors as needed
}

# ---------------------------------------------------------------------------
# Read-Only Python Fields
# ---------------------------------------------------------------------------
# Fields intentionally exposed as getter-only in Python bindings.
# Values are short reason tags used by audit reporting.
#
# Common reason tags:
# - protocol_managed: Message-level/system-managed value
# - derived_projection: Computed/derived projection of internal representation
# - immutable_view: Immutable wrapper/view by design
# - partial_binding: Temporarily read-only due partial implementation

READ_ONLY_FIELDS: dict[str, dict[str, str]] = {
    "Acm": {
        "id": "protocol_managed",
        "segment": "partial_binding",
    },
    "AcmSegment": {
        "metadata": "partial_binding",
        "data": "partial_binding",
    },
    "AcmMetadata": {
        "object_name": "partial_binding",
        "international_designator": "partial_binding",
    },
    "AcmData": {
        "att": "partial_binding",
        "phys": "partial_binding",
    },
    "AttitudeState": {
        "epoch": "immutable_view",
        "values": "immutable_view",
    },
    "Cdm": {
        "id": "protocol_managed",
        "header": "immutable_view",
        "body": "immutable_view",
    },
    "CdmBody": {
        "relative_metadata_data": "immutable_view",
        "segments": "immutable_view",
    },
    "CdmSegment": {
        "metadata": "immutable_view",
        "data": "immutable_view",
    },
    "CdmData": {
        "state_vector": "immutable_view",
        "covariance_matrix": "immutable_view",
    },
    "Ndm": {
        "id": "protocol_managed",
        "messages": "immutable_view",
        "comments": "immutable_view",
    },
    "Oem": {
        "id": "protocol_managed",
    },
    "Omm": {
        "id": "protocol_managed",
    },
    "Opm": {
        "id": "protocol_managed",
    },
    "Ocm": {
        "id": "protocol_managed",
    },
    "Rdm": {
        "id": "protocol_managed",
    },
    "Tdm": {
        "id": "protocol_managed",
        "segments": "derived_projection",
    },
    "TdmObservation": {
        "keyword": "derived_projection",
        "value": "derived_projection",
        "value_str": "derived_projection",
    },
    "OcmOdParameters": {
        "days_since_first_obs": "derived_projection",
        "days_since_last_obs": "derived_projection",
    },
    "OcmCovarianceMatrix": {
        "cov_basis": "partial_binding",
        "cov_confidence": "partial_binding",
        "cov_ordering": "partial_binding",
    },
    "OcmManeuverParameters": {
        "man_basis": "partial_binding",
    },
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
    "Aem": ["id", "version", "body"],
    "Apm": ["id", "version", "body"],
    "Acm": ["id", "version", "body", "header"],
    "TdmObservation": ["data"],
    # Internal body wrappers - skip entirely
    "OemBody": ["*"],
    "OmmBody": ["*"],
    "OpmBody": ["*"],
    "OcmBody": ["*"],
    "TdmBody": ["*"],
    "RdmBody": ["*"],
    "AemBody": ["*"],
    "ApmBody": ["*"],
    "AcmBody": ["*"],
    # Partially implemented or legacy
    "AcmAttitudeDetermination": ["*"],
    "AcmAttitudeState": ["*"],
    "AcmCovarianceMatrix": ["*"],
    "AcmData": ["*"],
    "AcmManeuverParameters": ["*"],
    "AcmMetadata": ["*"],
    "AcmPhysicalDescription": ["*"],
    "AcmSegment": ["*"],
    "QuaternionState": ["quaternion_dot"],
}

# ---------------------------------------------------------------------------
# Struct Name Mappings
# ---------------------------------------------------------------------------
# Maps Python class names to Rust struct names when they differ.
# Usually not needed as names are kept consistent.

STRUCT_MAPPINGS: dict[str, str] = {
    "Ndm": "CombinedNdm",
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


def is_python_helper_class(python_class: str) -> bool:
    """Check if a class is a Python-only helper (no Rust struct equivalent)."""
    return python_class in PYTHON_HELPER_CLASSES


def should_skip_rust_field(rust_struct: str, rust_field: str) -> bool:
    """Check if a Rust field should be skipped from Python binding validation."""
    skip_list = RUST_SKIP_FIELDS.get(rust_struct, [])
    return "*" in skip_list or rust_field in skip_list


def get_rust_struct_name(python_class: str) -> str:
    """Get the Rust struct name for a Python class."""
    return STRUCT_MAPPINGS.get(python_class, python_class)


def get_read_only_reason(python_class: str, python_field: str) -> str | None:
    """Get reason tag for an intentionally read-only Python field."""
    return READ_ONLY_FIELDS.get(python_class, {}).get(python_field)
