# SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
#
# SPDX-License-Identifier: MPL-2.0

from typing import List, Literal, Optional, Tuple, Union

import numpy

# -----------------------------------------------------------------------------
# Module-level API
# -----------------------------------------------------------------------------
def from_str(data: str) -> Union["Oem", "Cdm", "Omm", "Opm", "Ocm", "Tdm", "Rdm"]: ...
def from_file(path: str) -> Union["Oem", "Cdm", "Omm", "Opm", "Ocm", "Tdm", "Rdm"]: ...

# -----------------------------------------------------------------------------
# Common types
# -----------------------------------------------------------------------------
class OdmHeader:
    def __init__(
        self,
        *,
        creation_date: str,
        originator: str,
        classification: Optional[str] = None,
        message_id: Optional[str] = None,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    @property
    def comments(self) -> List[str]: ...
    @property
    def creation_date(self) -> str: ...
    @property
    def originator(self) -> str: ...
    @property
    def message_id(self) -> Optional[str]: ...
    @property
    def classification(self) -> Optional[str]: ...

class StateVector:
    """Position components in km; velocity components in km/s."""
    def __init__(
        self,
        *,
        epoch: str,
        x: float,
        y: float,
        z: float,
        x_dot: float,
        y_dot: float,
        z_dot: float,
    ) -> None: ...
    @property
    def epoch(self) -> str: ...
    @property
    def x(self) -> float:
        """X position [km]"""
        ...
    @property
    def y(self) -> float:
        """Y position [km]"""
        ...
    @property
    def z(self) -> float:
        """Z position [km]"""
        ...
    @property
    def x_dot(self) -> float:
        """X velocity [km/s]"""
        ...
    @property
    def y_dot(self) -> float:
        """Y velocity [km/s]"""
        ...
    @property
    def z_dot(self) -> float:
        """Z velocity [km/s]"""
        ...

class StateVectorAcc:
    """Position in km, velocity in km/s, acceleration in km/s^2."""
    def __init__(
        self,
        *,
        epoch: str,
        x: float,
        y: float,
        z: float,
        x_dot: float,
        y_dot: float,
        z_dot: float,
        x_ddot: Optional[float] = None,
        y_ddot: Optional[float] = None,
        z_ddot: Optional[float] = None,
    ) -> None: ...
    @property
    def epoch(self) -> str: ...
    @property
    def x(self) -> float:
        """X position [km]"""
        ...
    @property
    def y(self) -> float:
        """Y position [km]"""
        ...
    @property
    def z(self) -> float:
        """Z position [km]"""
        ...
    @property
    def x_dot(self) -> float:
        """X velocity [km/s]"""
        ...
    @property
    def y_dot(self) -> float:
        """Y velocity [km/s]"""
        ...
    @property
    def z_dot(self) -> float:
        """Z velocity [km/s]"""
        ...
    @property
    def x_ddot(self) -> Optional[float]:
        """X acceleration [km/s^2]"""
        ...
    @property
    def y_ddot(self) -> Optional[float]:
        """Y acceleration [km/s^2]"""
        ...
    @property
    def z_ddot(self) -> Optional[float]:
        """Z acceleration [km/s^2]"""
        ...

# -----------------------------------------------------------------------------
# OEM
# -----------------------------------------------------------------------------
class Oem:
    def __init__(self, *, header: OdmHeader, segments: List["OemSegment"]) -> None: ...
    @property
    def header(self) -> OdmHeader: ...
    @property
    def segments(self) -> List["OemSegment"]: ...
    @property
    def id(self) -> Optional[str]: ...
    @property
    def version(self) -> str: ...
    @staticmethod
    def from_str(data: str, format: Optional[Literal["kvn", "xml"]] = ...) -> "Oem": ...
    @staticmethod
    def from_file(
        path: str, format: Optional[Literal["kvn", "xml"]] = ...
    ) -> "Oem": ...
    def to_str(self, format: Literal["kvn", "xml"]) -> str: ...
    def to_file(self, path: str, format: Literal["kvn", "xml"]) -> None: ...

class OemSegment:
    def __init__(self, *, metadata: "OemMetadata", data: "OemData") -> None: ...
    @property
    def metadata(self) -> "OemMetadata": ...
    @property
    def data(self) -> "OemData": ...

class OemMetadata:
    def __init__(
        self,
        *,
        object_name: str,
        object_id: str,
        center_name: str,
        ref_frame: str,
        time_system: str,
        start_time: str,
        stop_time: str,
        ref_frame_epoch: Optional[str] = None,
        useable_start_time: Optional[str] = None,
        useable_stop_time: Optional[str] = None,
        interpolation: Optional[str] = None,
        interpolation_degree: Optional[int] = None,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    @property
    def object_name(self) -> str: ...
    @property
    def object_id(self) -> str: ...
    @property
    def center_name(self) -> str: ...
    @property
    def ref_frame(self) -> str: ...
    @property
    def time_system(self) -> str: ...
    @property
    def start_time(self) -> str: ...
    @property
    def stop_time(self) -> str: ...
    @property
    def ref_frame_epoch(self) -> Optional[str]: ...
    @property
    def useable_start_time(self) -> Optional[str]: ...
    @property
    def useable_stop_time(self) -> Optional[str]: ...
    @property
    def interpolation(self) -> Optional[str]: ...
    @property
    def interpolation_degree(self) -> Optional[int]: ...
    @property
    def comment(self) -> List[str]: ...

class OemData:
    def __init__(self, state_vectors: List[StateVectorAcc]) -> None: ...
    @property
    def state_vectors(self) -> List[StateVectorAcc]: ...
    @property
    def covariance_matrices(self) -> List["OemCovarianceMatrix"]: ...
    @property
    def comments(self) -> List[str]: ...
    @property
    def state_vectors_numpy(self) -> Tuple[List[str], numpy.ndarray]:
        """Epochs plus array with columns [X,Y,Z (km), X_DOT,Y_DOT,Z_DOT (km/s), optional accelerations km/s^2]."""
        ...
    @state_vectors_numpy.setter
    def state_vectors_numpy(self, value: Tuple[List[str], numpy.ndarray]) -> None: ...
    @property
    def covariance_matrices_numpy(self) -> Tuple[List[str], numpy.ndarray]:
        """Epochs plus array of 21 covariance elements: km^2, km^2/s, km^2/s^2."""
        ...
    @covariance_matrices_numpy.setter
    def covariance_matrices_numpy(
        self, value: Tuple[List[str], numpy.ndarray]
    ) -> None: ...

class OemCovarianceMatrix:
    """Covariance units: positions km^2, pos/vel km^2/s, velocities km^2/s^2."""
    def __init__(
        self,
        *,
        epoch: str,
        cov_ref_frame: Optional[str] = None,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    @property
    def epoch(self) -> str: ...
    @property
    def cov_ref_frame(self) -> Optional[str]: ...
    @property
    def comment(self) -> List[str]: ...
    @property
    def cx_x(self) -> float:
        """CX_X [km^2]"""
        ...
    @property
    def cy_x(self) -> float:
        """CY_X [km^2]"""
        ...
    @property
    def cy_y(self) -> float:
        """CY_Y [km^2]"""
        ...
    @property
    def cz_x(self) -> float:
        """CZ_X [km^2]"""
        ...
    @property
    def cz_y(self) -> float:
        """CZ_Y [km^2]"""
        ...
    @property
    def cz_z(self) -> float:
        """CZ_Z [km^2]"""
        ...
    @property
    def cx_dot_x(self) -> float:
        """CX_DOT_X [km^2/s]"""
        ...
    @property
    def cx_dot_y(self) -> float:
        """CX_DOT_Y [km^2/s]"""
        ...
    @property
    def cx_dot_z(self) -> float:
        """CX_DOT_Z [km^2/s]"""
        ...
    @property
    def cy_dot_x(self) -> float:
        """CY_DOT_X [km^2/s]"""
        ...
    @property
    def cy_dot_y(self) -> float:
        """CY_DOT_Y [km^2/s]"""
        ...
    @property
    def cy_dot_z(self) -> float:
        """CY_DOT_Z [km^2/s]"""
        ...
    @property
    def cz_dot_x(self) -> float:
        """CZ_DOT_X [km^2/s]"""
        ...
    @property
    def cz_dot_y(self) -> float:
        """CZ_DOT_Y [km^2/s]"""
        ...
    @property
    def cz_dot_z(self) -> float:
        """CZ_DOT_Z [km^2/s]"""
        ...
    @property
    def cx_dot_x_dot(self) -> float:
        """CX_DOT_X_DOT [km^2/s^2]"""
        ...
    @property
    def cy_dot_x_dot(self) -> float:
        """CY_DOT_X_DOT [km^2/s^2]"""
        ...
    @property
    def cy_dot_y_dot(self) -> float:
        """CY_DOT_Y_DOT [km^2/s^2]"""
        ...
    @property
    def cz_dot_x_dot(self) -> float:
        """CZ_DOT_X_DOT [km^2/s^2]"""
        ...
    @property
    def cz_dot_y_dot(self) -> float:
        """CZ_DOT_Y_DOT [km^2/s^2]"""
        ...
    @property
    def cz_dot_z_dot(self) -> float:
        """CZ_DOT_Z_DOT [km^2/s^2]"""
        ...

# -----------------------------------------------------------------------------
# OPM
# -----------------------------------------------------------------------------
class Opm:
    def __init__(self, *, header: OdmHeader, segment: "OpmSegment") -> None: ...
    header: OdmHeader
    segment: "OpmSegment"
    id: Optional[str]
    version: str
    @staticmethod
    def from_str(data: str, format: Optional[Literal["kvn", "xml"]] = ...) -> "Opm": ...
    @staticmethod
    def from_file(
        path: str, format: Optional[Literal["kvn", "xml"]] = ...
    ) -> "Opm": ...
    def to_str(self, format: Literal["kvn", "xml"]) -> str: ...
    def to_file(self, path: str, format: Literal["kvn", "xml"]) -> None: ...

class OpmSegment:
    def __init__(self, *, metadata: "OpmMetadata", data: "OpmData") -> None: ...
    metadata: "OpmMetadata"
    data: "OpmData"

class OpmMetadata:
    def __init__(
        self,
        *,
        object_name: str,
        object_id: str,
        center_name: str,
        ref_frame: str,
        time_system: str,
        ref_frame_epoch: Optional[str] = None,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    object_name: str
    object_id: str
    center_name: str
    ref_frame: str
    time_system: str
    ref_frame_epoch: Optional[str]
    comment: List[str]

class KeplerianElements:
    """Units: semi_major_axis km; inclination/RAAN/arg/true/mean anomaly degrees; gm km^3/s^2."""
    def __init__(
        self,
        *,
        semi_major_axis: float,
        eccentricity: float,
        inclination: float,
        ra_of_asc_node: float,
        arg_of_pericenter: float,
        gm: float,
        true_anomaly: Optional[float] = None,
        mean_anomaly: Optional[float] = None,
    ) -> None: ...
    semi_major_axis: float
    """Semi-major axis [km]"""
    eccentricity: float
    """Eccentricity [unitless]"""
    inclination: float
    """Inclination [deg]"""
    ra_of_asc_node: float
    """Right ascension of ascending node [deg]"""
    arg_of_pericenter: float
    """Argument of pericenter [deg]"""
    gm: float
    """Gravitational parameter [km^3/s^2]"""
    true_anomaly: Optional[float]
    """True anomaly [deg]"""
    mean_anomaly: Optional[float]
    """Mean anomaly [deg]"""

class OpmCovarianceMatrix:
    """Covariance units: positions km^2, pos/vel km^2/s, velocities km^2/s^2."""
    def __init__(self) -> None: ...
    cov_ref_frame: Optional[str]
    cx_x: float
    """CX_X [km^2]"""
    cy_x: float
    """CY_X [km^2]"""
    cy_y: float
    """CY_Y [km^2]"""
    cz_x: float
    """CZ_X [km^2]"""
    cz_y: float
    """CZ_Y [km^2]"""
    cz_z: float
    """CZ_Z [km^2]"""
    cx_dot_x: float
    """CX_DOT_X [km^2/s]"""
    cx_dot_y: float
    """CX_DOT_Y [km^2/s]"""
    cx_dot_z: float
    """CX_DOT_Z [km^2/s]"""
    cy_dot_x: float
    """CY_DOT_X [km^2/s]"""
    cy_dot_y: float
    """CY_DOT_Y [km^2/s]"""
    cy_dot_z: float
    """CY_DOT_Z [km^2/s]"""
    cz_dot_x: float
    """CZ_DOT_X [km^2/s]"""
    cz_dot_y: float
    """CZ_DOT_Y [km^2/s]"""
    cz_dot_z: float
    """CZ_DOT_Z [km^2/s]"""
    cx_dot_x_dot: float
    """CX_DOT_X_DOT [km^2/s^2]"""
    cy_dot_x_dot: float
    """CY_DOT_X_DOT [km^2/s^2]"""
    cy_dot_y_dot: float
    """CY_DOT_Y_DOT [km^2/s^2]"""
    cz_dot_x_dot: float
    """CZ_DOT_X_DOT [km^2/s^2]"""
    cz_dot_y_dot: float
    """CZ_DOT_Y_DOT [km^2/s^2]"""
    cz_dot_z_dot: float
    """CZ_DOT_Z_DOT [km^2/s^2]"""

class OpmData:
    """State vector units: km and km/s; Keplerian a km, angles deg, gm km^3/s^2."""
    def __init__(self, *, state_vector: StateVector) -> None: ...
    state_vector: StateVector
    """State vector values in km and km/s."""
    keplerian_elements: Optional[KeplerianElements]
    covariance_matrix: Optional[OpmCovarianceMatrix]
    """Covariance in km^2 / km^2/s / km^2/s^2."""

class Omm:
    def __init__(self, *, header: OdmHeader, segment: "OmmSegment") -> None: ...
    header: OdmHeader
    segment: "OmmSegment"
    id: Optional[str]
    version: str
    @staticmethod
    def from_str(data: str, format: Optional[Literal["kvn", "xml"]] = ...) -> "Omm": ...
    @staticmethod
    def from_file(
        path: str, format: Optional[Literal["kvn", "xml"]] = ...
    ) -> "Omm": ...
    def to_str(self, format: Literal["kvn", "xml"]) -> str: ...
    def to_file(self, path: str, format: Literal["kvn", "xml"]) -> None: ...

class OmmSegment:
    def __init__(self, *, metadata: "OmmMetadata", data: "OmmData") -> None: ...
    metadata: "OmmMetadata"
    data: "OmmData"

class OmmMetadata:
    def __init__(
        self,
        *,
        object_name: str,
        object_id: str,
        center_name: str,
        ref_frame: str,
        time_system: str,
        mean_element_theory: str,
        ref_frame_epoch: Optional[str] = None,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    object_name: str
    object_id: str
    center_name: str
    ref_frame: str
    time_system: str
    mean_element_theory: str
    ref_frame_epoch: Optional[str]
    comment: List[str]

class MeanElements:
    """Units: angles deg; semi_major_axis km; mean_motion rev/day; gm km^3/s^2."""
    def __init__(
        self,
        *,
        epoch: str,
        eccentricity: float,
        inclination: float,
        ra_of_asc_node: float,
        arg_of_pericenter: float,
        mean_anomaly: float,
        semi_major_axis: Optional[float] = None,
        mean_motion: Optional[float] = None,
        gm: Optional[float] = None,
    ) -> None: ...
    epoch: str
    eccentricity: float
    """Eccentricity [unitless]"""
    inclination: float
    """Inclination [deg]"""
    ra_of_asc_node: float
    """Right ascension of ascending node [deg]"""
    arg_of_pericenter: float
    """Argument of pericenter [deg]"""
    mean_anomaly: float
    """Mean anomaly [deg]"""
    semi_major_axis: Optional[float]
    """Semi-major axis [km]"""
    mean_motion: Optional[float]
    """Mean motion [rev/day]"""
    gm: Optional[float]
    """Gravitational parameter [km^3/s^2]"""

class OmmData:
    """Mean elements units as in MeanElements; covariance matches OPM covariance units."""
    def __init__(self, *, mean_elements: MeanElements) -> None: ...
    mean_elements: MeanElements
    covariance_matrix: Optional[OpmCovarianceMatrix]

# -----------------------------------------------------------------------------
# OCM
# -----------------------------------------------------------------------------
class Ocm:
    def __init__(self, *, header: OdmHeader, segment: "OcmSegment") -> None: ...
    header: OdmHeader
    segment: "OcmSegment"
    id: Optional[str]
    version: str
    @staticmethod
    def from_str(data: str, format: Optional[Literal["kvn", "xml"]] = ...) -> "Ocm": ...
    @staticmethod
    def from_file(
        path: str, format: Optional[Literal["kvn", "xml"]] = ...
    ) -> "Ocm": ...
    def to_str(self, format: Literal["kvn", "xml"]) -> str: ...
    def to_file(self, path: str, format: Literal["kvn", "xml"]) -> None: ...

class OcmSegment:
    def __init__(self, *, metadata: "OcmMetadata", data: "OcmData") -> None: ...
    metadata: "OcmMetadata"
    data: "OcmData"

class OcmMetadata:
    def __init__(
        self,
        *,
        time_system: str,
        epoch_tzero: str,
        object_name: Optional[str] = None,
        international_designator: Optional[str] = None,
        catalog_name: Optional[str] = None,
        object_designator: Optional[str] = None,
        alternate_names: Optional[str] = None,
        originator_poc: Optional[str] = None,
        originator_position: Optional[str] = None,
        originator_phone: Optional[str] = None,
        originator_email: Optional[str] = None,
        originator_address: Optional[str] = None,
        tech_org: Optional[str] = None,
        tech_poc: Optional[str] = None,
        tech_position: Optional[str] = None,
        tech_phone: Optional[str] = None,
        tech_email: Optional[str] = None,
        tech_address: Optional[str] = None,
        previous_message_id: Optional[str] = None,
        next_message_id: Optional[str] = None,
        adm_msg_link: Optional[str] = None,
        cdm_msg_link: Optional[str] = None,
        prm_msg_link: Optional[str] = None,
        rdm_msg_link: Optional[str] = None,
        tdm_msg_link: Optional[str] = None,
        operator: Optional[str] = None,
        owner: Optional[str] = None,
        country: Optional[str] = None,
        constellation: Optional[str] = None,
        object_type: Optional[str] = None,
        ops_status: Optional[str] = None,
        orbit_category: Optional[str] = None,
        ocm_data_elements: Optional[str] = None,
        sclk_offset_at_epoch: Optional[float] = None,
        sclk_sec_per_si_sec: Optional[float] = None,
        previous_message_epoch: Optional[str] = None,
        next_message_epoch: Optional[str] = None,
        start_time: Optional[str] = None,
        stop_time: Optional[str] = None,
        time_span: Optional[float] = None,
        taimutc_at_tzero: Optional[float] = None,
        next_leap_epoch: Optional[str] = None,
        next_leap_taimutc: Optional[float] = None,
        ut1mutc_at_tzero: Optional[float] = None,
        eop_source: Optional[str] = None,
        interp_method_eop: Optional[str] = None,
        celestial_source: Optional[str] = None,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    time_system: str
    epoch_tzero: str
    object_name: Optional[str]
    international_designator: Optional[str]
    catalog_name: Optional[str]
    object_designator: Optional[str]
    alternate_names: Optional[str]
    originator_poc: Optional[str]
    originator_position: Optional[str]
    originator_phone: Optional[str]
    originator_email: Optional[str]
    originator_address: Optional[str]
    tech_org: Optional[str]
    tech_poc: Optional[str]
    tech_position: Optional[str]
    tech_phone: Optional[str]
    tech_email: Optional[str]
    tech_address: Optional[str]
    previous_message_id: Optional[str]
    next_message_id: Optional[str]
    adm_msg_link: Optional[str]
    cdm_msg_link: Optional[str]
    prm_msg_link: Optional[str]
    rdm_msg_link: Optional[str]
    tdm_msg_link: Optional[str]
    operator: Optional[str]
    owner: Optional[str]
    country: Optional[str]
    constellation: Optional[str]
    object_type: Optional[str]
    ops_status: Optional[str]
    orbit_category: Optional[str]
    ocm_data_elements: Optional[str]
    sclk_offset_at_epoch: Optional[float]
    sclk_sec_per_si_sec: Optional[float]
    previous_message_epoch: Optional[str]
    next_message_epoch: Optional[str]
    start_time: Optional[str]
    stop_time: Optional[str]
    time_span: Optional[float]
    taimutc_at_tzero: Optional[float]
    next_leap_epoch: Optional[str]
    next_leap_taimutc: Optional[float]
    ut1mutc_at_tzero: Optional[float]
    eop_source: Optional[str]
    interp_method_eop: Optional[str]
    celestial_source: Optional[str]
    comment: List[str]

class OcmData:
    def __init__(self) -> None: ...
    traj: List["OcmTrajState"]
    phys: Optional["OcmPhysicalDescription"]
    def get_traj_count(self) -> int: ...
    def get_cov_count(self) -> int: ...
    def get_man_count(self) -> int: ...
    def has_phys(self) -> bool: ...
    def has_pert(self) -> bool: ...
    def has_od(self) -> bool: ...

class OcmTrajState:
    """Trajectory lines use traj_units (commonly km/ km/s depending on traj_type)."""
    def __init__(
        self,
        *,
        center_name: str,
        traj_ref_frame: str,
        traj_type: str,
        traj_lines: List["TrajLine"],
        traj_id: Optional[str] = None,
        traj_prev_id: Optional[str] = None,
        traj_next_id: Optional[str] = None,
        traj_basis: Optional[str] = None,
        traj_basis_id: Optional[str] = None,
        interpolation: Optional[str] = None,
        interpolation_degree: Optional[int] = None,
        propagator: Optional[str] = None,
        traj_frame_epoch: Optional[str] = None,
        useable_start_time: Optional[str] = None,
        useable_stop_time: Optional[str] = None,
        orb_revnum: Optional[float] = None,
        orb_revnum_basis: Optional[str] = None,
        orb_averaging: Optional[str] = None,
        traj_units: Optional[str] = None,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    center_name: str
    traj_ref_frame: str
    traj_type: str
    traj_lines: List["TrajLine"]
    comment: List[str]

class TrajLine:
    def __init__(self, *, epoch: str, values: List[float]) -> None: ...
    epoch: str
    values: List[float]

class OcmPhysicalDescription:
    def __init__(
        self, *, manufacturer: Optional[str] = None, comment: Optional[List[str]] = None
    ) -> None: ...
    manufacturer: Optional[str]
    comment: List[str]

# -----------------------------------------------------------------------------
# TDM
# -----------------------------------------------------------------------------
class Tdm:
    def __init__(self, *, header: "TdmHeader", body: "TdmBody") -> None: ...
    header: "TdmHeader"
    body: "TdmBody"
    id: Optional[str]
    version: str
    @property
    def segments(self) -> List["TdmSegment"]: ...
    @staticmethod
    def from_str(data: str, format: Optional[Literal["kvn", "xml"]] = ...) -> "Tdm": ...
    @staticmethod
    def from_file(
        path: str, format: Optional[Literal["kvn", "xml"]] = ...
    ) -> "Tdm": ...
    def to_str(self, format: Literal["kvn", "xml"]) -> str: ...
    def to_file(self, path: str, format: Literal["kvn", "xml"]) -> None: ...

class TdmHeader:
    def __init__(
        self,
        *,
        originator: str,
        creation_date: str,
        message_id: Optional[str] = None,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    originator: str
    creation_date: str
    message_id: Optional[str]
    comment: List[str]

class TdmBody:
    def __init__(self, *, segments: List["TdmSegment"]) -> None: ...
    segments: List["TdmSegment"]

class TdmSegment:
    def __init__(self, *, metadata: "TdmMetadata", data: "TdmData") -> None: ...
    metadata: "TdmMetadata"
    data: "TdmData"

class TdmMetadata:
    def __init__(
        self,
        *,
        time_system: str,
        participant_1: str,
        track_id: Optional[str] = None,
        data_types: Optional[str] = None,
        start_time: Optional[str] = None,
        stop_time: Optional[str] = None,
        participant_2: Optional[str] = None,
        participant_3: Optional[str] = None,
        participant_4: Optional[str] = None,
        participant_5: Optional[str] = None,
        mode: Optional[str] = None,
        path: Optional[str] = None,
        path_1: Optional[str] = None,
        path_2: Optional[str] = None,
        transmit_band: Optional[str] = None,
        receive_band: Optional[str] = None,
        turnaround_numerator: Optional[int] = None,
        turnaround_denominator: Optional[int] = None,
        timetag_ref: Optional[str] = None,
        integration_interval: Optional[float] = None,
        integration_ref: Optional[str] = None,
        freq_offset: Optional[float] = None,
        range_mode: Optional[str] = None,
        range_modulus: Optional[float] = None,
        range_units: Optional[str] = None,
        angle_type: Optional[str] = None,
        reference_frame: Optional[str] = None,
        interpolation: Optional[str] = None,
        interpolation_degree: Optional[int] = None,
        doppler_count_bias: Optional[float] = None,
        doppler_count_scale: Optional[float] = None,
        doppler_count_rollover: Optional[str] = None,
        transmit_delay_1: Optional[float] = None,
        transmit_delay_2: Optional[float] = None,
        transmit_delay_3: Optional[float] = None,
        transmit_delay_4: Optional[float] = None,
        transmit_delay_5: Optional[float] = None,
        receive_delay_1: Optional[float] = None,
        receive_delay_2: Optional[float] = None,
        receive_delay_3: Optional[float] = None,
        receive_delay_4: Optional[float] = None,
        receive_delay_5: Optional[float] = None,
        data_quality: Optional[str] = None,
        correction_angle_1: Optional[float] = None,
        correction_angle_2: Optional[float] = None,
        correction_doppler: Optional[float] = None,
        correction_mag: Optional[float] = None,
        correction_range: Optional[float] = None,
        correction_rcs: Optional[float] = None,
        correction_receive: Optional[float] = None,
        correction_transmit: Optional[float] = None,
        correction_aberration_yearly: Optional[float] = None,
        correction_aberration_diurnal: Optional[float] = None,
        corrections_applied: Optional[str] = None,
        ephemeris_name_1: Optional[str] = None,
        ephemeris_name_2: Optional[str] = None,
        ephemeris_name_3: Optional[str] = None,
        ephemeris_name_4: Optional[str] = None,
        ephemeris_name_5: Optional[str] = None,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    time_system: str
    participant_1: str
    track_id: Optional[str]
    data_types: Optional[str]
    start_time: Optional[str]
    stop_time: Optional[str]
    participant_2: Optional[str]
    participant_3: Optional[str]
    participant_4: Optional[str]
    participant_5: Optional[str]
    mode: Optional[str]
    path: Optional[str]
    path_1: Optional[str]
    path_2: Optional[str]
    transmit_band: Optional[str]
    receive_band: Optional[str]
    turnaround_numerator: Optional[int]
    turnaround_denominator: Optional[int]
    timetag_ref: Optional[str]
    integration_interval: Optional[float]
    integration_ref: Optional[str]
    freq_offset: Optional[float]
    range_mode: Optional[str]
    range_modulus: Optional[float]
    range_units: Optional[str]
    angle_type: Optional[str]
    reference_frame: Optional[str]
    interpolation: Optional[str]
    interpolation_degree: Optional[int]
    doppler_count_bias: Optional[float]
    doppler_count_scale: Optional[float]
    doppler_count_rollover: Optional[str]
    transmit_delay_1: Optional[float]
    transmit_delay_2: Optional[float]
    transmit_delay_3: Optional[float]
    transmit_delay_4: Optional[float]
    transmit_delay_5: Optional[float]
    receive_delay_1: Optional[float]
    receive_delay_2: Optional[float]
    receive_delay_3: Optional[float]
    receive_delay_4: Optional[float]
    receive_delay_5: Optional[float]
    data_quality: Optional[str]
    correction_angle_1: Optional[float]
    correction_angle_2: Optional[float]
    correction_doppler: Optional[float]
    correction_mag: Optional[float]
    correction_range: Optional[float]
    correction_rcs: Optional[float]
    correction_receive: Optional[float]
    correction_transmit: Optional[float]
    correction_aberration_yearly: Optional[float]
    correction_aberration_diurnal: Optional[float]
    corrections_applied: Optional[str]
    ephemeris_name_1: Optional[str]
    ephemeris_name_2: Optional[str]
    ephemeris_name_3: Optional[str]
    ephemeris_name_4: Optional[str]
    ephemeris_name_5: Optional[str]
    comment: List[str]

class TdmData:
    def __init__(
        self,
        *,
        observations: Optional[List["TdmObservation"]] = None,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    observations: List["TdmObservation"]
    comment: List[str]
    def get_observation_count(self) -> int: ...

class TdmObservation:
    """Units depend on keyword (e.g., RANGE meters, DOPPLER Hz, ANGLE degrees)."""
    def __init__(self, *, epoch: str, keyword: str, value: float) -> None: ...
    epoch: str
    keyword: str
    def get_value(self) -> Optional[float]: ...
    def get_value_str(self) -> str: ...

# -----------------------------------------------------------------------------
# RDM
# -----------------------------------------------------------------------------
class Rdm:
    def __init__(self, *, header: "RdmHeader", segment: "RdmSegment") -> None: ...
    header: "RdmHeader"
    segment: "RdmSegment"
    id: Optional[str]
    version: str
    @staticmethod
    def from_str(data: str, format: Optional[Literal["kvn", "xml"]] = ...) -> "Rdm": ...
    @staticmethod
    def from_file(
        path: str, format: Optional[Literal["kvn", "xml"]] = ...
    ) -> "Rdm": ...
    def to_str(self, format: Literal["kvn", "xml"]) -> str: ...
    def to_file(self, path: str, format: Literal["kvn", "xml"]) -> None: ...

class RdmHeader:
    def __init__(
        self,
        *,
        originator: str,
        creation_date: str,
        message_id: str,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    originator: str
    creation_date: str
    message_id: str
    comment: List[str]

class RdmSegment:
    def __init__(self, *, metadata: "RdmMetadata", data: "RdmData") -> None: ...
    metadata: "RdmMetadata"
    data: "RdmData"

class RdmMetadata:
    def __init__(
        self,
        *,
        object_name: str,
        international_designator: str,
        controlled_reentry: str,
        center_name: str,
        time_system: str,
        epoch_tzero: str,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    object_name: str
    international_designator: str
    controlled_reentry: str
    center_name: str
    time_system: str
    epoch_tzero: str
    comment: List[str]

class RdmData:
    def __init__(
        self,
        *,
        atmospheric_reentry_parameters: "AtmosphericReentryParameters",
        ground_impact_parameters: Optional["GroundImpactParameters"] = None,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    atmospheric_reentry_parameters: "AtmosphericReentryParameters"
    ground_impact_parameters: Optional["GroundImpactParameters"]
    comment: List[str]
    state_vector: Optional[StateVector]
    covariance_matrix: Optional[OpmCovarianceMatrix]
    spacecraft_parameters: Optional["RdmSpacecraftParameters"]

class AtmosphericReentryParameters:
    """orbit_lifetime in days; reentry_altitude in meters."""
    def __init__(
        self,
        *,
        orbit_lifetime: float,
        reentry_altitude: float,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    orbit_lifetime: float
    reentry_altitude: float
    comment: List[str]

class GroundImpactParameters:
    """Probabilities are fractional (0-1)."""
    def __init__(
        self,
        *,
        probability_of_impact: Optional[float] = None,
        probability_of_burn_up: Optional[float] = None,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    probability_of_impact: Optional[float]
    probability_of_burn_up: Optional[float]
    comment: List[str]

class RdmSpacecraftParameters:
    """Mass values in kilograms."""
    def __init__(
        self,
        *,
        wet_mass: Optional[float] = None,
        dry_mass: Optional[float] = None,
        comment: Optional[List[str]] = None,
    ) -> None: ...
    wet_mass: Optional[float]
    dry_mass: Optional[float]
    comment: List[str]

# -----------------------------------------------------------------------------
# CDM
# -----------------------------------------------------------------------------
class Cdm:
    def __init__(
        self,
        header: "CdmHeader",
        body: "CdmBody",
        id: Optional[str] = None,
        version: str = "1.0",
    ) -> None: ...
    header: "CdmHeader"
    body: "CdmBody"
    id: Optional[str]
    version: str
    @staticmethod
    def from_str(data: str, format: Optional[Literal["kvn", "xml"]] = ...) -> "Cdm": ...
    @staticmethod
    def from_file(
        path: str, format: Optional[Literal["kvn", "xml"]] = ...
    ) -> "Cdm": ...
    def to_str(self, format: Literal["kvn", "xml"]) -> str: ...
    def to_file(self, path: str, format: Literal["kvn", "xml"]) -> None: ...

class CdmHeader:
    def __init__(
        self,
        creation_date: str,
        originator: str,
        message_id: str,
        message_for: Optional[str] = None,
        comment: List[str] = ...,
    ) -> None: ...
    creation_date: str
    originator: str
    message_id: str
    message_for: Optional[str]
    comment: List[str]

class CdmBody:
    def __init__(
        self,
        relative_metadata_data: "RelativeMetadataData",
        segments: List["CdmSegment"],
    ) -> None: ...
    relative_metadata_data: "RelativeMetadataData"
    segments: List["CdmSegment"]

class CdmSegment:
    def __init__(self, metadata: "CdmMetadata", data: "CdmData") -> None: ...
    metadata: "CdmMetadata"
    data: "CdmData"

class CdmMetadata:
    def __init__(
        self,
        object: "CdmObjectType",
        object_designator: str,
        catalog_name: str,
        object_name: str,
        international_designator: str,
        ephemeris_name: str,
        covariance_method: "CovarianceMethodType",
        maneuverable: "ManeuverableType",
        ref_frame: "ReferenceFrameType",
        object_type: Optional["ObjectDescription"] = None,
        operator_contact_position: Optional[str] = None,
        operator_organization: Optional[str] = None,
        operator_phone: Optional[str] = None,
        operator_email: Optional[str] = None,
        orbit_center: Optional[str] = None,
        gravity_model: Optional[str] = None,
        atmospheric_model: Optional[str] = None,
        n_body_perturbations: Optional[str] = None,
        solar_rad_pressure: Optional[bool] = None,
        earth_tides: Optional[bool] = None,
        intrack_thrust: Optional[bool] = None,
        comment: List[str] = ...,
    ) -> None: ...
    object_name: str
    object_designator: str
    catalog_name: str
    international_designator: str
    ephemeris_name: str
    covariance_method: "CovarianceMethodType"
    maneuverable: "ManeuverableType"
    ref_frame: "ReferenceFrameType"
    object_type: Optional["ObjectDescription"]
    operator_contact_position: Optional[str]
    operator_organization: Optional[str]
    operator_phone: Optional[str]
    operator_email: Optional[str]
    orbit_center: Optional[str]
    gravity_model: Optional[str]
    atmospheric_model: Optional[str]
    n_body_perturbations: Optional[str]
    solar_rad_pressure: Optional[bool]
    earth_tides: Optional[bool]
    intrack_thrust: Optional[bool]
    comment: List[str]

class CdmData:
    def __init__(
        self, state_vector: "CdmStateVector", covariance_matrix: "CdmCovarianceMatrix"
    ) -> None: ...
    state_vector: "CdmStateVector"
    covariance_matrix: "CdmCovarianceMatrix"
    comment: List[str]

class CdmStateVector:
    """Position km, velocity km/s."""
    def __init__(
        self, x: float, y: float, z: float, x_dot: float, y_dot: float, z_dot: float
    ) -> None: ...
    x: float
    """X position [km]"""
    y: float
    """Y position [km]"""
    z: float
    """Z position [km]"""
    x_dot: float
    """X velocity [km/s]"""
    y_dot: float
    """Y velocity [km/s]"""
    z_dot: float
    """Z velocity [km/s]"""

class CdmCovarianceMatrix:
    def to_numpy(self) -> numpy.ndarray: ...

class RelativeMetadataData:
    """miss_distance meters; relative_speed m/s; relative_position/velocity RTN components in meters and m/s; screen sizes meters."""
    def __init__(
        self,
        tca: str,
        miss_distance: float,
        relative_speed: Optional[float] = None,
        relative_position: Optional[Tuple[float, float, float]] = None,
        relative_velocity: Optional[Tuple[float, float, float]] = None,
        start_screen_period: Optional[str] = None,
        stop_screen_period: Optional[str] = None,
        screen_volume_frame: Optional["ScreenVolumeFrameType"] = None,
        screen_volume_shape: Optional["ScreenVolumeShapeType"] = None,
        screen_volume_x: Optional[float] = None,
        screen_volume_y: Optional[float] = None,
        screen_volume_z: Optional[float] = None,
        screen_entry_time: Optional[str] = None,
        screen_exit_time: Optional[str] = None,
        collision_probability: Optional[float] = None,
        collision_probability_method: Optional[str] = None,
        comment: List[str] = ...,
        miss_distance_unit: Optional[str] = None,
    ) -> None: ...
    tca: str
    miss_distance: float
    relative_speed: Optional[float]
    relative_position: Optional[Tuple[float, float, float]]
    relative_velocity: Optional[Tuple[float, float, float]]
    screen_volume_frame: Optional["ScreenVolumeFrameType"]
    screen_volume_shape: Optional["ScreenVolumeShapeType"]
    screen_volume_x: Optional[float]
    screen_volume_y: Optional[float]
    screen_volume_z: Optional[float]
    screen_entry_time: Optional[str]
    screen_exit_time: Optional[str]
    collision_probability: Optional[float]
    collision_probability_method: Optional[str]
    comment: List[str]

# CDM enums
class CdmObjectType:
    Object1: "CdmObjectType"
    Object2: "CdmObjectType"

class ScreenVolumeFrameType:
    Rtn: "ScreenVolumeFrameType"
    Tvn: "ScreenVolumeFrameType"

class ScreenVolumeShapeType:
    Ellipsoid: "ScreenVolumeShapeType"
    Box: "ScreenVolumeShapeType"

class ReferenceFrameType:
    Eme2000: "ReferenceFrameType"
    Gcrf: "ReferenceFrameType"
    Itrf: "ReferenceFrameType"

class CovarianceMethodType:
    Calculated: "CovarianceMethodType"
    Default: "CovarianceMethodType"

class ManeuverableType:
    Yes: "ManeuverableType"
    No: "ManeuverableType"
    NA: "ManeuverableType"

class ObjectDescription:
    Payload: "ObjectDescription"
    RocketBody: "ObjectDescription"
    Debris: "ObjectDescription"
    Unknown: "ObjectDescription"
    Other: "ObjectDescription"
