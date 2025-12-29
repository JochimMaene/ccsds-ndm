# Generated content DO NOT EDIT
from typing import Optional
import numpy

@staticmethod
def from_file(path):
    """
    Parse from a file path (KVN or XML).

    Parameters
    ----------
    path : str
        Path to the file.

    Returns
    -------
    Union[Oem, Cdm, Omm, Opm, Ocm, Tdm, Rdm]
        The parsed NDM object.
    """
    ...

@staticmethod
def from_str(data):
    """
    Parse a string (KVN or XML) and return the corresponding NDM object.

    Parameters
    ----------
    data : str
        The content to parse.

    Returns
    -------
    Union[Oem, Cdm, Omm, Opm, Ocm, Tdm, Rdm]
        The parsed NDM object.

    Raises
    ------
    ValueError
        If parsing fails.
    """
    ...

class AtmosphericReentryParameters:
    """
    Atmospheric re-entry information.

    Parameters
    ----------
    orbit_lifetime : float
        Remaining time in orbit (days).
    reentry_altitude : float
        Defined re-entry altitude (km).
    """
    def __init__(
        *,
        orbit_lifetime,
        reentry_altitude,
        orbit_lifetime_window_start=None,
        orbit_lifetime_window_end=None,
        nominal_reentry_epoch=None,
        reentry_window_start=None,
        reentry_window_end=None,
        orbit_lifetime_confidence_level=None,
        comment=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def nominal_reentry_epoch(self) -> Optional[str]:
        """
        Nominal re-entry epoch.

        Format: ISO 8601
        """
        ...

    @nominal_reentry_epoch.setter
    def nominal_reentry_epoch(self, value: Optional[str]) -> None: ...
    @property
    def orbit_lifetime(self) -> float:
        """
        Remaining time in orbit (days).
        """
        ...

    @orbit_lifetime.setter
    def orbit_lifetime(self, value: float) -> None: ...
    @property
    def orbit_lifetime_confidence_level(self) -> Optional[float]:
        """
        Confidence level for orbit lifetime (percentage 0-100).
        """
        ...

    @orbit_lifetime_confidence_level.setter
    def orbit_lifetime_confidence_level(self, value: Optional[float]) -> None: ...
    @property
    def orbit_lifetime_window_end(self) -> Optional[float]:
        """
        End of the orbit lifetime window (days).
        """
        ...

    @orbit_lifetime_window_end.setter
    def orbit_lifetime_window_end(self, value: Optional[float]) -> None: ...
    @property
    def orbit_lifetime_window_start(self) -> Optional[float]:
        """
        Start of the orbit lifetime window (days).
        """
        ...

    @orbit_lifetime_window_start.setter
    def orbit_lifetime_window_start(self, value: Optional[float]) -> None: ...
    @property
    def reentry_altitude(self) -> float:
        """
        Defined re-entry altitude (km).
        """
        ...

    @reentry_altitude.setter
    def reentry_altitude(self, value: float) -> None: ...
    @property
    def reentry_window_end(self) -> Optional[str]:
        """
        End of the re-entry window.

        Format: ISO 8601
        """
        ...

    @reentry_window_end.setter
    def reentry_window_end(self, value: Optional[str]) -> None: ...
    @property
    def reentry_window_start(self) -> Optional[str]:
        """
        Start of the re-entry window.

        Format: ISO 8601
        """
        ...

    @reentry_window_start.setter
    def reentry_window_start(self, value: Optional[str]) -> None: ...

class Cdm:
    """
    Represents a CCSDS Conjunction Data Message (CDM).

    The CDM specifies a standard message format for use in exchanging spacecraft
    conjunction information between originators of Conjunction Assessments (CAs)
    and satellite owner/operators and other authorized parties.

    It contains information about a single conjunction between two objects,
    including their positions/velocities, covariances at TCA, and relative
    state data.
    """
    def __init__(header, body, id=None, version=...) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def body(self) -> CdmBody:
        """
        The message body containing relative metadata/data and object segments.
        """
        ...

    @body.setter
    def body(self, value: CdmBody) -> None: ...
    @staticmethod
    def from_file(path, format=None):
        """
        Parse a CDM from a file path with optional format.

        Parameters
        ----------
        path : str
            The path to the file.
        format : str, optional
            The format of the file ('kvn' or 'xml'). If None, it will be auto-detected.

        Returns
        -------
        Cdm
            The parsed CDM object.
        """
        ...

    @staticmethod
    def from_kvn(kvn):
        """
        Parse a CDM from a KVN formatted string.

        Parameters
        ----------
        kvn : str
            The KVN string to parse.

        Returns
        -------
        Cdm
            The parsed CDM object.
        """
        ...

    @staticmethod
    def from_str(data, format=None):
        """
        Parse a CDM from a string with optional format.

        Parameters
        ----------
        data : str
            The string content to parse.
        format : str, optional
            The format of the input ('kvn' or 'xml'). If None, it will be auto-detected.

        Returns
        -------
        Cdm
            The parsed CDM object.
        """
        ...

    @property
    def header(self) -> CdmHeader:
        """
        The message header.
        """
        ...

    @header.setter
    def header(self, value: CdmHeader) -> None: ...
    @property
    def id(self) -> Optional[str]:
        """
        Unique ID for this message.
        """
        ...

    @id.setter
    def id(self, value: Optional[str]) -> None: ...
    def to_file(self, path, format):
        """
        Write the CDM to a file.

        Parameters
        ----------
        path : str
            The output file path.
        format : str
            The output format ('kvn' or 'xml').
        """
        ...

    def to_str(self, format):
        """
        Serialize the CDM to a string.

        Parameters
        ----------
        format : str
            The output format ('kvn' or 'xml').

        Returns
        -------
        str
            The serialized CDM string.
        """
        ...

    @property
    def version(self) -> str:
        """
        The CDM version.
        """
        ...

    @version.setter
    def version(self, value: str) -> None: ...

class CdmBody:
    """
    The body of the CDM.

    Contains relative metadata/data between the two objects and the
    specific segments for each object.

    Parameters
    ----------
    relative_metadata_data : RelativeMetadataData
        Data describing the relative relationships between Object1 and Object2.
    segments : list of CdmSegment
        The segments containing specific data for each object.
    """
    def __init__(relative_metadata_data, segments) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def relative_metadata_data(self) -> RelativeMetadataData:
        """
        Data describing the relative relationships between Object1 and Object2.
        """
        ...

    @relative_metadata_data.setter
    def relative_metadata_data(self, value: RelativeMetadataData) -> None: ...
    @property
    def segments(self) -> list[CdmSegment]:
        """
        The segments containing specific data for each object.
        """
        ...

    @segments.setter
    def segments(self, value: list[CdmSegment]) -> None: ...

class CdmCovarianceMatrix:
    """
    Covariance Matrix at TCA.

    Provides uncertainty information for the state vector.
    Can be converted to a NumPy array using `to_numpy()`.
    """
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comments(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    def to_numpy(self):
        """
        Returns the full 9x9 covariance matrix as a NumPy array.
        If the optional 7,8,9 rows (Drag, SRP, Thrust) are missing, they are filled with 0.0.
        """
        ...

class CdmData:
    """
    Data Section for an object in a CDM.

    Contains logical blocks for OD parameters, Additional parameters,
    State Vector, and Covariance Matrix.

    Parameters
    ----------
    state_vector : CdmStateVector
        Object position and velocity at TCA.
    covariance_matrix : CdmCovarianceMatrix
        Object covariance at TCA.
    """
    def __init__(state_vector, covariance_matrix, comments) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def covariance_matrix(self) -> CdmCovarianceMatrix:
        """
        Object covariance at TCA.
        """
        ...

    @covariance_matrix.setter
    def covariance_matrix(self, value: CdmCovarianceMatrix) -> None: ...
    @property
    def state_vector(self) -> CdmStateVector:
        """
        Object position and velocity at TCA.
        """
        ...

    @state_vector.setter
    def state_vector(self, value: CdmStateVector) -> None: ...

class CdmHeader:
    """
    Header section of the CDM.

    Contains metadata about the message itself, such as creation date,
    originator, and unique identifiers.

    Parameters
    ----------
    creation_date : str
        Message creation date/time in UTC (ISO 8601).
    originator : str
        Creating agency or owner/operator.
    message_id : str
        ID that uniquely identifies a message from a given originator.
    message_for : str, optional
        Spacecraft name(s) for which the CDM is provided.
    comment : list of str, optional
        Explanatory comments.
    """
    def __init__(
        creation_date, originator, message_id, message_for=None, comment=...
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Explanatory comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def creation_date(self) -> str:
        """
        Message creation date/time in UTC.
        """
        ...

    @creation_date.setter
    def creation_date(self, value: str) -> None: ...
    @property
    def message_for(self) -> Optional[str]:
        """
        Spacecraft name(s) for which the CDM is provided.
        """
        ...

    @message_for.setter
    def message_for(self, value: Optional[str]) -> None: ...
    @property
    def message_id(self) -> str:
        """
        ID used to uniquely identify this message.
        """
        ...

    @message_id.setter
    def message_id(self, value: str) -> None: ...
    @property
    def originator(self) -> str:
        """
        Creating agency or owner/operator.
        """
        ...

    @originator.setter
    def originator(self, value: str) -> None: ...

class CdmMetadata:
    """
    Metadata Section for an object in a CDM.

    Contains identification, contact, and modeling information for either
    Object1 or Object2.

    Parameters
    ----------
    object : CdmObjectType
        The object identification (OBJECT1 or OBJECT2).
    object_designator : str
        The satellite catalog designator for the object.
    catalog_name : str
        The satellite catalog used for the object.
    object_name : str
        Spacecraft name for the object.
    international_designator : str
        The full international designator (YYYY-NNNP{PP}).
    ephemeris_name : str
        Unique name of the external ephemeris file or 'NONE'.
    covariance_method : CovarianceMethodType
        Method used to calculate the covariance (CALCULATED or DEFAULT).
    maneuverable : ManeuverableType
        The maneuver capacity of the object (YES, NO, or NA).
    ref_frame : ReferenceFrameType
        Reference frame for state vector data (GCRF, EME2000, or ITRF).
    object_type : ObjectDescription, optional
        The object type (PAYLOAD, ROCKET BODY, DEBRIS, etc.).
    operator_contact_position : str, optional
        Contact position of the owner/operator.
    operator_organization : str, optional
        Contact organization.
    operator_phone : str, optional
        Phone number of the contact.
    operator_email : str, optional
        Email address of the contact.
    orbit_center : str, optional
        The central body (e.g., EARTH, SUN).
    gravity_model : str, optional
        The gravity model used for the OD.
    atmospheric_model : str, optional
        The atmospheric density model used for the OD.
    n_body_perturbations : str, optional
        N-body gravitational perturbations used.
    solar_rad_pressure : bool, optional
        Whether solar radiation pressure was used.
    earth_tides : bool, optional
        Whether solid Earth and ocean tides were used.
    intrack_thrust : bool, optional
        Whether in-track thrust modeling was used.
    comment : list of str, optional
        Comments.
    """
    def __init__(
        object,
        object_designator,
        catalog_name,
        object_name,
        international_designator,
        ephemeris_name,
        covariance_method,
        maneuverable,
        ref_frame,
        object_type=None,
        operator_contact_position=None,
        operator_organization=None,
        operator_phone=None,
        operator_email=None,
        orbit_center=None,
        gravity_model=None,
        atmospheric_model=None,
        n_body_perturbations=None,
        solar_rad_pressure=None,
        earth_tides=None,
        intrack_thrust=None,
        comment=...,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def atmospheric_model(self) -> Optional[str]:
        """
        The atmospheric density model used for the OD.
        """
        ...

    @atmospheric_model.setter
    def atmospheric_model(self, value: Optional[str]) -> None: ...
    @property
    def catalog_name(self) -> str:
        """
        The satellite catalog used for the object.
        """
        ...

    @catalog_name.setter
    def catalog_name(self, value: str) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def ephemeris_name(self) -> str:
        """
        Unique name of the external ephemeris file or 'NONE'.
        """
        ...

    @ephemeris_name.setter
    def ephemeris_name(self, value: str) -> None: ...
    @property
    def gravity_model(self) -> Optional[str]:
        """
        The gravity model used for the OD.
        """
        ...

    @gravity_model.setter
    def gravity_model(self, value: Optional[str]) -> None: ...
    @property
    def international_designator(self) -> str:
        """
        The full international designator (YYYY-NNNP{PP}).
        """
        ...

    @international_designator.setter
    def international_designator(self, value: str) -> None: ...
    @property
    def n_body_perturbations(self) -> Optional[str]:
        """
        N-body gravitational perturbations used.
        """
        ...

    @n_body_perturbations.setter
    def n_body_perturbations(self, value: Optional[str]) -> None: ...
    @property
    def object_designator(self) -> str:
        """
        The satellite catalog designator for the object.
        """
        ...

    @object_designator.setter
    def object_designator(self, value: str) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Spacecraft name for the object.
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def operator_contact_position(self) -> Optional[str]:
        """
        Contact position of the owner/operator.
        """
        ...

    @operator_contact_position.setter
    def operator_contact_position(self, value: Optional[str]) -> None: ...
    @property
    def operator_email(self) -> Optional[str]:
        """
        Email address of the contact.
        """
        ...

    @operator_email.setter
    def operator_email(self, value: Optional[str]) -> None: ...
    @property
    def operator_organization(self) -> Optional[str]:
        """
        Contact organization.
        """
        ...

    @operator_organization.setter
    def operator_organization(self, value: Optional[str]) -> None: ...
    @property
    def operator_phone(self) -> Optional[str]:
        """
        Phone number of the contact.
        """
        ...

    @operator_phone.setter
    def operator_phone(self, value: Optional[str]) -> None: ...
    @property
    def orbit_center(self) -> Optional[str]:
        """
        The central body (e.g., EARTH, SUN).
        """
        ...

    @orbit_center.setter
    def orbit_center(self, value: Optional[str]) -> None: ...

class CdmObjectType:
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

class CdmSegment:
    """
    A CDM Segment, consisting of metadata and data for a specific object.
    """
    def __init__(metadata, data) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def data(self) -> CdmData:
        """
        Data section for the object.
        """
        ...

    @data.setter
    def data(self, value: CdmData) -> None: ...
    @property
    def metadata(self) -> CdmMetadata:
        """
        Metadata for the object.
        """
        ...

    @metadata.setter
    def metadata(self, value: CdmMetadata) -> None: ...

class CdmStateVector:
    """
    State Vector containing position and velocity at TCA

    Parameters
    ----------
    x : float
        Position X component. Units: km.
    y : float
        Position Y component. Units: km.
    z : float
        Position Z component. Units: km.
    x_dot : float
        Velocity X component. Units: km/s.
    y_dot : float
        Velocity Y component. Units: km/s.
    z_dot : float
        Velocity Z component. Units: km/s.
    """
    def __init__(x, y, z, x_dot, y_dot, z_dot) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def x(self) -> float:
        """
        Position X component.
        """
        ...

    @x.setter
    def x(self, value: float) -> None: ...
    @property
    def x_dot(self) -> float:
        """
        Velocity X component.
        """
        ...

    @x_dot.setter
    def x_dot(self, value: float) -> None: ...
    @property
    def y(self) -> float:
        """
        Position Y component.
        """
        ...

    @y.setter
    def y(self, value: float) -> None: ...
    @property
    def y_dot(self) -> float:
        """
        Velocity Y component.
        """
        ...

    @y_dot.setter
    def y_dot(self, value: float) -> None: ...
    @property
    def z(self) -> float:
        """
        Position Z component.
        """
        ...

    @z.setter
    def z(self, value: float) -> None: ...
    @property
    def z_dot(self) -> float:
        """
        Velocity Z component.
        """
        ...

    @z_dot.setter
    def z_dot(self, value: float) -> None: ...

class CovLine:
    """
    A single line in a covariance time history.

    Parameters
    ----------
    epoch : str
        Absolute or relative time tag.
    values : list of float
        Covariance matrix elements for this epoch.
    """
    def __init__(*, epoch, values) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def epoch(self) -> str:
        """
        Absolute or relative time tag.
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def values(self) -> list[float]:
        """
        Covariance matrix elements for this epoch.
        """
        ...

    @values.setter
    def values(self, value: list[float]) -> None: ...

class CovarianceMethodType:
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

class GroundImpactParameters:
    """
    Ground impact and burn-up data parameters.
    """
    def __init__(
        *, probability_of_impact=None, probability_of_burn_up=None, comment=None
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def impact_ref_frame(self) -> Optional[str]:
        """
        Impact reference frame.
        """
        ...

    @impact_ref_frame.setter
    def impact_ref_frame(self, value: Optional[str]) -> None: ...
    @property
    def impact_window_end(self) -> Optional[str]:
        """
        End of the impact window.
        """
        ...

    @impact_window_end.setter
    def impact_window_end(self, value: Optional[str]) -> None: ...
    @property
    def impact_window_start(self) -> Optional[str]:
        """
        Start of the impact window.
        """
        ...

    @impact_window_start.setter
    def impact_window_start(self, value: Optional[str]) -> None: ...
    @property
    def nominal_impact_alt(self) -> Optional[float]:
        """
        Nominal impact altitude (km).
        """
        ...

    @nominal_impact_alt.setter
    def nominal_impact_alt(self, value: Optional[float]) -> None: ...
    @property
    def nominal_impact_epoch(self) -> Optional[str]:
        """
        Nominal impact epoch.
        """
        ...

    @nominal_impact_epoch.setter
    def nominal_impact_epoch(self, value: Optional[str]) -> None: ...
    @property
    def nominal_impact_lat(self) -> Optional[float]:
        """
        Nominal impact latitude (degrees).
        """
        ...

    @nominal_impact_lat.setter
    def nominal_impact_lat(self, value: Optional[float]) -> None: ...
    @property
    def nominal_impact_lon(self) -> Optional[float]:
        """
        Nominal impact longitude (degrees).
        """
        ...

    @nominal_impact_lon.setter
    def nominal_impact_lon(self, value: Optional[float]) -> None: ...
    @property
    def probability_of_break_up(self) -> Optional[float]:
        """
        Probability of break-up.
        """
        ...

    @probability_of_break_up.setter
    def probability_of_break_up(self, value: Optional[float]) -> None: ...
    @property
    def probability_of_burn_up(self) -> Optional[float]:
        """
        Probability of burn-up.
        """
        ...

    @probability_of_burn_up.setter
    def probability_of_burn_up(self, value: Optional[float]) -> None: ...
    @property
    def probability_of_casualty(self) -> Optional[float]:
        """
        Probability of casualty.
        """
        ...

    @probability_of_casualty.setter
    def probability_of_casualty(self, value: Optional[float]) -> None: ...
    @property
    def probability_of_impact(self) -> Optional[float]:
        """
        Probability of impact.
        """
        ...

    @probability_of_impact.setter
    def probability_of_impact(self, value: Optional[float]) -> None: ...
    @property
    def probability_of_land_impact(self) -> Optional[float]:
        """
        Probability of land impact.
        """
        ...

    @probability_of_land_impact.setter
    def probability_of_land_impact(self, value: Optional[float]) -> None: ...

class KeplerianElements:
    def __init__(
        semi_major_axis,
        eccentricity,
        inclination,
        ra_of_asc_node,
        arg_of_pericenter,
        gm,
        true_anomaly,
        mean_anomaly,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def arg_of_pericenter(self) -> float:
        """
        Argument of pericenter.

        Units: deg
        """
        ...

    @arg_of_pericenter.setter
    def arg_of_pericenter(self, value: float) -> None: ...
    @property
    def comments(self) -> list[str]:
        """
        Comments associated with the Keplerian elements.
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    @property
    def eccentricity(self) -> float:
        """
        Eccentricity.
        """
        ...

    @eccentricity.setter
    def eccentricity(self, value: float) -> None: ...
    @property
    def gm(self) -> float:
        """
        Gravitational coefficient (GM).

        Units: km³/s²
        """
        ...

    @gm.setter
    def gm(self, value: float) -> None: ...
    @property
    def inclination(self) -> float:
        """
        Inclination.

        Units: deg
        """
        ...

    @inclination.setter
    def inclination(self, value: float) -> None: ...
    @property
    def mean_anomaly(self) -> Optional[float]:
        """
        Mean anomaly.

        Units: deg
        """
        ...

    @mean_anomaly.setter
    def mean_anomaly(self, value: Optional[float]) -> None: ...
    @property
    def ra_of_asc_node(self) -> float:
        """
        Right ascension of the ascending node.

        Units: deg
        """
        ...

    @ra_of_asc_node.setter
    def ra_of_asc_node(self, value: float) -> None: ...
    @property
    def semi_major_axis(self) -> float:
        """
        Semi-major axis.

        Units: km
        """
        ...

    @semi_major_axis.setter
    def semi_major_axis(self, value: float) -> None: ...
    @property
    def true_anomaly(self) -> Optional[float]:
        """
        True anomaly.

        Units: deg
        """
        ...

    @true_anomaly.setter
    def true_anomaly(self, value: Optional[float]) -> None: ...

class ManLine:
    """
    A single line in a maneuver time history.

    Parameters
    ----------
    epoch : str
        Ignition epoch.
    values : list of str
        Maneuver elements for this epoch.
    """
    def __init__(*, epoch, values) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def epoch(self) -> str:
        """
        Ignition epoch.
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def values(self) -> list[str]:
        """
        Maneuver elements for this epoch.
        """
        ...

    @values.setter
    def values(self, value: list[str]) -> None: ...

class ManeuverParameters:
    """
    Maneuver parameters.

    Parameters
    ----------
    man_epoch_ignition : str
        Epoch of ignition.
    man_duration : float
        Duration of maneuver (s).
    man_delta_mass : float
        Mass change during maneuver (kg).
    man_ref_frame : str
        Reference frame for velocity change.
    man_dv_1 : float
        Velocity change in 1st axis (km/s).
    man_dv_2 : float
        Velocity change in 2nd axis (km/s).
    man_dv_3 : float
        Velocity change in 3rd axis (km/s).
    """
    def __init__(
        man_epoch_ignition,
        man_duration,
        man_delta_mass,
        man_ref_frame,
        man_dv_1,
        man_dv_2,
        man_dv_3,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comments(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    @property
    def man_delta_mass(self) -> float:
        """
        Mass change during maneuver.

        Value is < 0.

        Units: kg
        """
        ...

    @man_delta_mass.setter
    def man_delta_mass(self, value: float) -> None: ...
    @property
    def man_duration(self) -> float:
        """
        Duration of maneuver.

        If 0, impulsive maneuver.

        Units: s
        """
        ...

    @man_duration.setter
    def man_duration(self, value: float) -> None: ...
    @property
    def man_dv_1(self) -> float:
        """
        Velocity change in 1st axis.

        Units: km/s
        """
        ...

    @man_dv_1.setter
    def man_dv_1(self, value: float) -> None: ...
    @property
    def man_dv_2(self) -> float:
        """
        Velocity change in 2nd axis.

        Units: km/s
        """
        ...

    @man_dv_2.setter
    def man_dv_2(self, value: float) -> None: ...
    @property
    def man_dv_3(self) -> float:
        """
        Velocity change in 3rd axis.

        Units: km/s
        """
        ...

    @man_dv_3.setter
    def man_dv_3(self, value: float) -> None: ...
    @property
    def man_epoch_ignition(self) -> str:
        """
        Epoch of ignition.
        """
        ...

    @man_epoch_ignition.setter
    def man_epoch_ignition(self, value: str) -> None: ...
    @property
    def man_ref_frame(self) -> str:
        """
        Reference frame for velocity change.
        """
        ...

    @man_ref_frame.setter
    def man_ref_frame(self, value: str) -> None: ...

class ManeuverableType:
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

class MeanElements:
    """
    Create a new MeanElements object.

    Parameters
    ----------
    epoch : str
        Epoch of the mean elements.
    eccentricity : float
        Eccentricity.
    inclination : float
        Inclination (deg).
    ra_of_asc_node : float
        Right ascension of the ascending node (deg).
    arg_of_pericenter : float
        Argument of pericenter (deg).
    mean_anomaly : float
        Mean anomaly (deg).
    semi_major_axis : float, optional
        Semi-major axis (km).
    mean_motion : float, optional
        Mean motion (rev/day).
    gm : float, optional
        Gravitational coefficient (km³/s²).
    """
    def __init__(
        epoch,
        eccentricity,
        inclination,
        ra_of_asc_node,
        arg_of_pericenter,
        mean_anomaly,
        semi_major_axis,
        mean_motion,
        gm,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def arg_of_pericenter(self) -> float:
        """
        Argument of pericenter.

        Units: deg
        """
        ...

    @arg_of_pericenter.setter
    def arg_of_pericenter(self, value: float) -> None: ...
    @property
    def comments(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    @property
    def eccentricity(self) -> float:
        """
        Eccentricity.
        """
        ...

    @eccentricity.setter
    def eccentricity(self, value: float) -> None: ...
    @property
    def epoch(self) -> str:
        """
        Epoch of the mean elements.
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def gm(self) -> Optional[float]:
        """
        Gravitational coefficient (GM).

        Units: km³/s²
        """
        ...

    @gm.setter
    def gm(self, value: Optional[float]) -> None: ...
    @property
    def inclination(self) -> float:
        """
        Inclination.

        Units: deg
        """
        ...

    @inclination.setter
    def inclination(self, value: float) -> None: ...
    @property
    def mean_anomaly(self) -> float:
        """
        Mean anomaly.

        Units: deg
        """
        ...

    @mean_anomaly.setter
    def mean_anomaly(self, value: float) -> None: ...
    @property
    def mean_motion(self) -> Optional[float]:
        """
        Mean motion.

        Units: rev/day
        """
        ...

    @mean_motion.setter
    def mean_motion(self, value: Optional[float]) -> None: ...
    @property
    def ra_of_asc_node(self) -> float:
        """
        Right ascension of the ascending node.

        Units: deg
        """
        ...

    @ra_of_asc_node.setter
    def ra_of_asc_node(self, value: float) -> None: ...
    @property
    def semi_major_axis(self) -> Optional[float]:
        """
        Semi-major axis.

        Units: km
        """
        ...

    @semi_major_axis.setter
    def semi_major_axis(self, value: Optional[float]) -> None: ...

class ObjectDescription:
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

class Ocm:
    """
    Represents a CCSDS Orbit Comprehensive Message (OCM).

    The OCM aggregates and extends OMM, OPM, and OEM content in a single hybrid message.
    It emphasizes flexibility and message conciseness by offering extensive optional
    standardized content while minimizing mandatory content.

    Parameters
    ----------
    header : OdmHeader
        The message header.
        (Mandatory)
    segment : OcmSegment
        The OCM data segment.
        (Mandatory)
    """
    def __init__(header, segment) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @staticmethod
    def from_file(path, format):
        """
        Create an OCM message from a file.

        Parameters
        ----------
        path : str
            Path to the input file.
        format : str, optional
            Format ('kvn' or 'xml'). Auto-detected if None.

        Returns
        -------
        Ocm
            The parsed OCM object.
        """
        ...

    @staticmethod
    def from_str(data, format):
        """
        Create an OCM message from a string.

        Parameters
        ----------
        data : str
            Input string/content.
        format : str, optional
            Format ('kvn' or 'xml'). Auto-detected if None.

        Returns
        -------
        Ocm
            The parsed OCM object.
        """
        ...

    @property
    def header(self) -> OdmHeader:
        """
        The message header.
        """
        ...

    @header.setter
    def header(self, value: OdmHeader) -> None: ...
    @property
    def segment(self) -> OcmSegment:
        """
        The OCM data segment.
        """
        ...

    @segment.setter
    def segment(self, value: OcmSegment) -> None: ...
    def to_file(self, path, format):
        """
        Write to file.

        Parameters
        ----------
        path : str
            Output file path.
        format : str
            Output format ('kvn' or 'xml').
        """
        ...

    def to_str(self, format):
        """
        Serialize to string.

        Parameters
        ----------
        format : str
            Output format ('kvn' or 'xml').

        Returns
        -------
        str
            The serialized string.
        """
        ...

class OcmCovarianceMatrix:
    """
    OCM Covariance Matrix Time History segment.

    This block contains covariance matrices for a single object, optionally
    spanning fictitious nodes for interpolation smoothness.

    Parameters
    ----------
    epoch : str
        Epoch of the covariance matrix.
        (Mandatory)
    cov_ref_frame : str
        Reference frame for the covariance matrix.
        (Mandatory)
    cov_type : str
        Specifies the covariance element set type.
        (Mandatory)
    cov_matrix : list of float
        Upper triangular part of the covariance matrix.
        (Mandatory)
    cov_id : str, optional
        Identification number for this covariance matrix time history block.
        (Optional)
    cov_prev_id : str, optional
        Identification number for the previous covariance matrix time history.
        (Optional)
    cov_next_id : str, optional
        Identification number for the next covariance matrix time history.
        (Optional)
    cov_basis : str, optional
        Basis of this covariance matrix time history data (PREDICTED, DETERMINED, etc.).
        (Optional)
    cov_basis_id : str, optional
        Identification number for the telemetry dataset, orbit determination, or simulation.
        (Optional)
    cov_confidence : float, optional
        The confidence level associated with the covariance [0-100].
        (Optional)
    cov_scale_factor : float, optional
        Scale factor to be applied to the covariance matrix.
        (Optional)
    cov_units : str, optional
        Comma-delimited set of SI unit designations for the covariance elements.
        (Optional)
    comment : list[str], optional
        Comments.
        (Optional)
    """
    def __init__(
        *,
        cov_ref_frame,
        cov_type,
        cov_ordering,
        cov_lines,
        cov_id=None,
        cov_prev_id=None,
        cov_next_id=None,
        cov_basis=None,
        cov_basis_id=None,
        cov_frame_epoch=None,
        cov_scale_min=None,
        cov_scale_max=None,
        cov_confidence=None,
        cov_units=None,
        comment=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def cov_basis(self) -> Optional[str]:
        """
        Covariance basis.
        """
        ...

    @cov_basis.setter
    def cov_basis(self, value: Optional[str]) -> None: ...
    @property
    def cov_basis_id(self) -> Optional[str]:
        """
        Covariance basis identifier.
        """
        ...

    @cov_basis_id.setter
    def cov_basis_id(self, value: Optional[str]) -> None: ...
    @property
    def cov_confidence(self) -> Optional[float]:
        """
        The confidence level associated with the covariance [0-100].
        """
        ...

    @cov_confidence.setter
    def cov_confidence(self, value: Optional[float]) -> None: ...
    @property
    def cov_frame_epoch(self) -> Optional[str]:
        """
        Epoch of the covariance reference frame, if not intrinsic to the definition.
        """
        ...

    @cov_frame_epoch.setter
    def cov_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def cov_id(self) -> Optional[str]:
        """
        Identification number for this covariance matrix time history block.
        """
        ...

    @cov_id.setter
    def cov_id(self, value: Optional[str]) -> None: ...
    @property
    def cov_lines(self) -> list[CovLine]:
        """
        A list of covariance data lines.
        """
        ...

    @cov_lines.setter
    def cov_lines(self, value: list[CovLine]) -> None: ...
    @property
    def cov_next_id(self) -> Optional[str]:
        """
        Identification number for the next covariance matrix time history.
        """
        ...

    @cov_next_id.setter
    def cov_next_id(self, value: Optional[str]) -> None: ...
    @property
    def cov_ordering(self) -> str:
        """
        The ordering of the covariance matrix elements.
        """
        ...

    @cov_ordering.setter
    def cov_ordering(self, value: str) -> None: ...
    @property
    def cov_prev_id(self) -> Optional[str]:
        """
        Identification number for the previous covariance matrix time history.
        """
        ...

    @cov_prev_id.setter
    def cov_prev_id(self, value: Optional[str]) -> None: ...
    @property
    def cov_ref_frame(self) -> str:
        """
        Reference frame for the covariance matrix.

        Standard values: "ICRF", "EME2000", "ITRF", "TEME", "GCRF"
        """
        ...

    @cov_ref_frame.setter
    def cov_ref_frame(self, value: str) -> None: ...
    @property
    def cov_scale_max(self) -> Optional[float]:
        """
        Scale factor maximum.
        """
        ...

    @cov_scale_max.setter
    def cov_scale_max(self, value: Optional[float]) -> None: ...
    @property
    def cov_scale_min(self) -> Optional[float]:
        """
        Scale factor minimum.
        """
        ...

    @cov_scale_min.setter
    def cov_scale_min(self, value: Optional[float]) -> None: ...
    @property
    def cov_type(self) -> str:
        """
        Specifies the covariance element set type.

        Standard values: "CARTPV", "KEPLERIAN", "EQUINOCTIAL"
        """
        ...

    @cov_type.setter
    def cov_type(self, value: str) -> None: ...
    @property
    def cov_units(self) -> Optional[str]:
        """
        Comma-delimited set of SI unit designations for the covariance elements.
        """
        ...

    @cov_units.setter
    def cov_units(self, value: Optional[str]) -> None: ...

class OcmData:
    """
    OCM Data blocks.

    This class holds the various data sections of an OCM, including trajectory states,
    physical properties, covariance matrices, maneuvers, perturbations, and
    orbit determination parameters.
    """
    def __init__() -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def cov(self) -> list[OcmCovarianceMatrix]:
        """
        List of covariance time history blocks.
        """
        ...

    @cov.setter
    def cov(self, value: list[OcmCovarianceMatrix]) -> None: ...
    @property
    def cov_count(self) -> int:
        """
        Number of covariance time history blocks.
        """
        ...

    @cov_count.setter
    def cov_count(self, value: int) -> None: ...
    @property
    def has_user(self) -> bool:
        """
        Whether user-defined parameters are present.
        """
        ...

    @has_user.setter
    def has_user(self, value: bool) -> None: ...
    @property
    def man(self) -> list[OcmManeuver]:
        """
        List of maneuver specifications.
        """
        ...

    @man.setter
    def man(self, value: list[OcmManeuver]) -> None: ...
    @property
    def od(self) -> Optional[OcmOdParameters]:
        """
        Orbit determination data.
        """
        ...

    @od.setter
    def od(self, value: Optional[OcmOdParameters]) -> None: ...
    @property
    def pert(self) -> Optional[OcmPerturbations]:
        """
        Perturbation parameters.
        """
        ...

    @pert.setter
    def pert(self, value: Optional[OcmPerturbations]) -> None: ...
    @property
    def phys(self) -> Optional[OcmPhysicalDescription]:
        """
        Space object physical characteristics.
        """
        ...

    @phys.setter
    def phys(self, value: Optional[OcmPhysicalDescription]) -> None: ...
    @property
    def traj(self) -> list[OcmTrajState]:
        """
        List of trajectory state time history blocks.
        """
        ...

    @traj.setter
    def traj(self, value: list[OcmTrajState]) -> None: ...
    @property
    def user(self) -> Optional[UserDefined]:
        """
        User-defined parameters.
        """
        ...

    @user.setter
    def user(self, value: Optional[UserDefined]) -> None: ...

class OcmManeuver:
    """
    Maneuver Parameters segment.

    Parameters
    ----------
    man_id : str
        Identifier for the maneuver block.
    man_device_id : str
        Identifier for the maneuver device (e.g., thruster name).
    man_composition : str
        Specifies the maneuver composition (e.g., 'VECTOR', 'SCALAR').
    man_ref_frame : str
        Reference frame for the maneuver data.
    man_lines : list of ManLine
        A list of maneuver data lines.
    man_prev_id : str, optional
        Identifier for the previous maneuver block for this space object.
    man_next_id : str, optional
        Identifier for the next maneuver block for this space object.
    man_basis : str, optional
        Basis of the maneuver data ('Observed', 'Predicted', etc.).
    man_basis_id : str, optional
        Identifier for the orbit determination or simulation basis.
    man_prev_epoch : str, optional
        Epoch of the previous maneuver.
    man_next_epoch : str, optional
        Epoch of the next maneuver.
    man_purpose : str, optional
        Purpose of the maneuver.
    man_pred_source : str, optional
        Source of the predicted maneuver data.
    man_frame_epoch : str, optional
        Epoch of the maneuver reference frame.
    grav_assist_name : str, optional
        Name of the gravity assist body.
    dc_type : str, optional
        Type of duty cycle ('Continuous', 'Impulsive', 'Duration').
    man_units : str, optional
        SI unit designations for the maneuver elements.
    comment : list of str, optional
        Comments for this maneuver block.
    """
    def __init__(
        *,
        man_id,
        man_device_id,
        man_composition,
        man_ref_frame,
        man_lines,
        man_prev_id=None,
        man_next_id=None,
        man_basis=None,
        man_basis_id=None,
        man_prev_epoch=None,
        man_next_epoch=None,
        man_purpose=None,
        man_pred_source=None,
        man_frame_epoch=None,
        grav_assist_name=None,
        dc_type=None,
        dc_win_open=None,
        dc_win_close=None,
        dc_min_cycles=None,
        dc_max_cycles=None,
        dc_exec_start=None,
        dc_exec_stop=None,
        dc_ref_time=None,
        dc_time_pulse_duration=None,
        dc_time_pulse_period=None,
        dc_ref_dir=None,
        dc_body_frame=None,
        dc_body_trigger=None,
        dc_pa_start_angle=None,
        dc_pa_stop_angle=None,
        man_units=None,
        comment=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments for this maneuver block.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def dc_body_frame(self) -> Optional[str]:
        """
        Body frame for duty cycle.
        """
        ...

    @dc_body_frame.setter
    def dc_body_frame(self, value: Optional[str]) -> None: ...
    @property
    def dc_body_trigger(self) -> Optional[list[float]]:
        """
        Body trigger for duty cycle.
        """
        ...

    @dc_body_trigger.setter
    def dc_body_trigger(self, value: Optional[list[float]]) -> None: ...
    @property
    def dc_exec_start(self) -> Optional[str]:
        """
        Start time of duty cycle execution.
        """
        ...

    @dc_exec_start.setter
    def dc_exec_start(self, value: Optional[str]) -> None: ...
    @property
    def dc_exec_stop(self) -> Optional[str]:
        """
        Stop time of duty cycle execution.
        """
        ...

    @dc_exec_stop.setter
    def dc_exec_stop(self, value: Optional[str]) -> None: ...
    @property
    def dc_max_cycles(self) -> Optional[int]:
        """
        Maximum number of duty cycles.
        """
        ...

    @dc_max_cycles.setter
    def dc_max_cycles(self, value: Optional[int]) -> None: ...
    @property
    def dc_min_cycles(self) -> Optional[int]:
        """
        Minimum number of duty cycles.
        """
        ...

    @dc_min_cycles.setter
    def dc_min_cycles(self, value: Optional[int]) -> None: ...
    @property
    def dc_pa_start_angle(self) -> Optional[float]:
        """
        Phase angle start for duty cycle.
        """
        ...

    @dc_pa_start_angle.setter
    def dc_pa_start_angle(self, value: Optional[float]) -> None: ...
    @property
    def dc_pa_stop_angle(self) -> Optional[float]:
        """
        Phase angle stop for duty cycle.
        """
        ...

    @dc_pa_stop_angle.setter
    def dc_pa_stop_angle(self, value: Optional[float]) -> None: ...
    @property
    def dc_ref_dir(self) -> Optional[list[float]]:
        """
        Reference direction for duty cycle.
        """
        ...

    @dc_ref_dir.setter
    def dc_ref_dir(self, value: Optional[list[float]]) -> None: ...
    @property
    def dc_ref_time(self) -> Optional[str]:
        """
        Reference time for duty cycle.
        """
        ...

    @dc_ref_time.setter
    def dc_ref_time(self, value: Optional[str]) -> None: ...
    @property
    def dc_time_pulse_duration(self) -> Optional[float]:
        """
        Duration of the duty cycle pulse.
        """
        ...

    @dc_time_pulse_duration.setter
    def dc_time_pulse_duration(self, value: Optional[float]) -> None: ...
    @property
    def dc_time_pulse_period(self) -> Optional[float]:
        """
        Period of the duty cycle pulse.
        """
        ...

    @dc_time_pulse_period.setter
    def dc_time_pulse_period(self, value: Optional[float]) -> None: ...
    @property
    def dc_type(self) -> str:
        """
        Type of duty cycle ('Continuous', 'Impulsive', 'Duration').
        """
        ...

    @dc_type.setter
    def dc_type(self, value: str) -> None: ...
    @property
    def dc_win_close(self) -> Optional[str]:
        """
        End of the duty cycle window.
        """
        ...

    @dc_win_close.setter
    def dc_win_close(self, value: Optional[str]) -> None: ...
    @property
    def dc_win_open(self) -> Optional[str]:
        """
        Start of the duty cycle window.
        """
        ...

    @dc_win_open.setter
    def dc_win_open(self, value: Optional[str]) -> None: ...
    @property
    def grav_assist_name(self) -> Optional[str]:
        """
        Name of the gravity assist body.
        """
        ...

    @grav_assist_name.setter
    def grav_assist_name(self, value: Optional[str]) -> None: ...
    @property
    def man_basis(self) -> Optional[str]:
        """
        Basis of the maneuver data ('Observed', 'Predicted', etc.).
        """
        ...

    @man_basis.setter
    def man_basis(self, value: Optional[str]) -> None: ...
    @property
    def man_basis_id(self) -> Optional[str]:
        """
        Identifier for the orbit determination or simulation basis.
        """
        ...

    @man_basis_id.setter
    def man_basis_id(self, value: Optional[str]) -> None: ...
    @property
    def man_composition(self) -> str:
        """
        Specifies the maneuver composition (e.g., 'VECTOR', 'SCALAR').
        """
        ...

    @man_composition.setter
    def man_composition(self, value: str) -> None: ...
    @property
    def man_device_id(self) -> str:
        """
        Identifier for the maneuver device (e.g., thruster name).
        """
        ...

    @man_device_id.setter
    def man_device_id(self, value: str) -> None: ...
    @property
    def man_frame_epoch(self) -> Optional[str]:
        """
        Epoch of the maneuver reference frame, if not intrinsic to the definition.
        """
        ...

    @man_frame_epoch.setter
    def man_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def man_id(self) -> str:
        """
        Identification number for the maneuver block.
        """
        ...

    @man_id.setter
    def man_id(self, value: str) -> None: ...
    @property
    def man_lines(self) -> list[ManLine]:
        """
        A list of maneuver data lines.
        """
        ...

    @man_lines.setter
    def man_lines(self, value: list[ManLine]) -> None: ...
    @property
    def man_next_epoch(self) -> Optional[str]:
        """
        Epoch of the next maneuver.
        """
        ...

    @man_next_epoch.setter
    def man_next_epoch(self, value: Optional[str]) -> None: ...
    @property
    def man_next_id(self) -> Optional[str]:
        """
        Identification number for the next maneuver block.
        """
        ...

    @man_next_id.setter
    def man_next_id(self, value: Optional[str]) -> None: ...
    @property
    def man_pred_source(self) -> Optional[str]:
        """
        Source of the predicted maneuver data.
        """
        ...

    @man_pred_source.setter
    def man_pred_source(self, value: Optional[str]) -> None: ...
    @property
    def man_prev_epoch(self) -> Optional[str]:
        """
        Epoch of the previous maneuver.
        """
        ...

    @man_prev_epoch.setter
    def man_prev_epoch(self, value: Optional[str]) -> None: ...
    @property
    def man_prev_id(self) -> Optional[str]:
        """
        Identification number for the previous maneuver block.
        """
        ...

    @man_prev_id.setter
    def man_prev_id(self, value: Optional[str]) -> None: ...
    @property
    def man_purpose(self) -> Optional[str]:
        """
        Purpose of the maneuver.
        """
        ...

    @man_purpose.setter
    def man_purpose(self, value: Optional[str]) -> None: ...
    @property
    def man_ref_frame(self) -> str:
        """
        Reference frame for the maneuver data.

        Standard values: "ICRF", "EME2000", "ITRF", "TEME", "GCRF", "RSW", "RTN", "TNW"
        """
        ...

    @man_ref_frame.setter
    def man_ref_frame(self, value: str) -> None: ...
    @property
    def man_units(self) -> Optional[str]:
        """
        SI unit designations for the maneuver elements.
        """
        ...

    @man_units.setter
    def man_units(self, value: Optional[str]) -> None: ...

class OcmMetadata:
    """
    OCM Metadata

    This section contains data about the object and the message itself,
    including time systems, object identifiers, and Point-of-Contact (POC) information.

    Parameters
    ----------
    time_system : str
        Time system that shall be used for all absolute time stamps in the message.
        (Mandatory)
    epoch_tzero : str
        Epoch to which all relative times in the message are referenced.
        (Mandatory)
    object_name : str, optional
        Name of the space object that the message is associated with.
        (Optional)
    international_designator : str, optional
        The COSPAR international designator of the space object.
        (Optional)
    catalog_name : str, optional
        The name of the satellite catalog used for the space object identification.
        (Optional)
    object_designator : str, optional
        The unique satellite identification designator used in the specified catalog.
        (Optional)
    alternate_names : str, optional
        Alternate name(s) by which the space object is known.
        (Optional)
    originator_poc : str, optional
        Originator Point-of-Contact.
        (Optional)
    originator_position : str, optional
        Contact position of the originator PoC.
        (Optional)
    originator_phone : str, optional
        Originator PoC phone number.
        (Optional)
    originator_email : str, optional
        Originator PoC email address.
        (Optional)
    originator_address : str, optional
        Originator's physical address.
        (Optional)
    tech_org : str, optional
        Technical organization (creating agency or operator).
        (Optional)
    tech_poc : str, optional
        Technical Point-of-Contact.
        (Optional)
    tech_position : str, optional
        Contact position of the technical PoC.
        (Optional)
    tech_phone : str, optional
        Technical PoC phone number.
        (Optional)
    tech_email : str, optional
        Technical PoC email address.
        (Optional)
    tech_address : str, optional
        Technical PoC physical address.
        (Optional)
    previous_message_id : str, optional
        Identifier for the previous OCM message.
        (Optional)
    next_message_id : str, optional
        Identifier for the anticipated next OCM message.
        (Optional)
    adm_msg_link : str, optional
        Identifier of linked Attitude Data Message.
        (Optional)
    cdm_msg_link : str, optional
        Identifier of linked Conjunction Data Message.
        (Optional)
    prm_msg_link : str, optional
        Identifier of linked Pointing Request Message.
        (Optional)
    rdm_msg_link : str, optional
        Identifier of linked Reentry Data Message.
        (Optional)
    tdm_msg_link : str, optional
        Identifier of linked Tracking Data Message.
        (Optional)
    operator : str, optional
        Operator of the space object.
        (Optional)
    owner : str, optional
        Owner of the space object.
        (Optional)
    country : str, optional
        Country of the owner or operator of the space object.
        (Optional)
    constellation : str, optional
        Name of the constellation the space object belongs to.
        (Optional)
    object_type : str, optional
        Type of object (PAYLOAD, ROCKET_BODY, DEBRIS, etc.).
        (Optional)
    ops_status : str, optional
        Operational status of the space object.
        (Optional)
    orbit_category : str, optional
        Orbit category (LEO, GEO, HEO, etc.).
        (Optional)
    ocm_data_elements : str, optional
        List of data elements included in the OCM message.
        (Optional)
    sclk_offset_at_epoch : float, optional
        Spacecraft clock offset at EPOCH_TZERO [s].
        (Conditional)
    sclk_sec_per_si_sec : float, optional
        Spacecraft clock scale factor [s/SI-s].
        (Conditional)
    previous_message_epoch : str, optional
        Epoch of the previous message.
        (Optional)
    next_message_epoch : str, optional
        Anticipated epoch of the next message.
        (Optional)
    start_time : str, optional
        Time of the earliest data in the message.
        (Optional)
    stop_time : str, optional
        Time of the latest data in the message.
        (Optional)
    time_span : float, optional
        Approximate time span covered by the data [d].
        (Optional)
    taimutc_at_tzero : float, optional
        TAI minus UTC difference at EPOCH_TZERO [s].
        (Optional)
    next_leap_epoch : str, optional
        Epoch of the next leap second.
        (Optional)
    next_leap_taimutc : float, optional
        TAI minus UTC difference at NEXT_LEAP_EPOCH [s].
        (Conditional)
    ut1mutc_at_tzero : float, optional
        UT1 minus UTC difference at EPOCH_TZERO [s].
        (Optional)
    eop_source : str, optional
        Source of Earth Orientation Parameters.
        (Optional)
    interp_method_eop : str, optional
        Interpolation method for EOP data.
        (Optional)
    celestial_source : str, optional
        Source of celestial body ephemerides.
        (Optional)
    comment : list of str, optional
        Comments for the metadata block.
        (Optional)
    """
    def __init__(
        *,
        time_system,
        epoch_tzero,
        object_name=None,
        international_designator=None,
        catalog_name=None,
        object_designator=None,
        alternate_names=None,
        originator_poc=None,
        originator_position=None,
        originator_phone=None,
        originator_email=None,
        originator_address=None,
        tech_org=None,
        tech_poc=None,
        tech_position=None,
        tech_phone=None,
        tech_email=None,
        tech_address=None,
        previous_message_id=None,
        next_message_id=None,
        adm_msg_link=None,
        cdm_msg_link=None,
        prm_msg_link=None,
        rdm_msg_link=None,
        tdm_msg_link=None,
        operator=None,
        owner=None,
        country=None,
        constellation=None,
        object_type=None,
        ops_status=None,
        orbit_category=None,
        ocm_data_elements=None,
        sclk_offset_at_epoch=None,
        sclk_sec_per_si_sec=None,
        previous_message_epoch=None,
        next_message_epoch=None,
        start_time=None,
        stop_time=None,
        time_span=None,
        taimutc_at_tzero=None,
        next_leap_epoch=None,
        next_leap_taimutc=None,
        ut1mutc_at_tzero=None,
        eop_source=None,
        interp_method_eop=None,
        celestial_source=None,
        comment=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def adm_msg_link(self) -> Optional[str]:
        """
        Unique identifier of an Attitude Data Message associated with the OCM.
        """
        ...

    @adm_msg_link.setter
    def adm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def alternate_names(self) -> Optional[str]:
        """
        Alternate name(s) by which the space object is known.

        Examples: "SV08", "IN8"
        """
        ...

    @alternate_names.setter
    def alternate_names(self, value: Optional[str]) -> None: ...
    @property
    def catalog_name(self) -> Optional[str]:
        """
        Satellite catalog source (or source agency or operator).

        Examples: "CSPOC", "RFSA", "ESA", "COMSPOC"
        """
        ...

    @catalog_name.setter
    def catalog_name(self, value: Optional[str]) -> None: ...
    @property
    def cdm_msg_link(self) -> Optional[str]:
        """
        Unique identifier of a Conjunction Data Message associated with the OCM.
        """
        ...

    @cdm_msg_link.setter
    def cdm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def celestial_source(self) -> Optional[str]:
        """
        Source of celestial body ephemeris data used in the OCM.
        """
        ...

    @celestial_source.setter
    def celestial_source(self, value: Optional[str]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def constellation(self) -> Optional[str]:
        """
        The name of the constellation that the space object belongs to.

        Example: "SPIRE"
        """
        ...

    @constellation.setter
    def constellation(self, value: Optional[str]) -> None: ...
    @property
    def country(self) -> Optional[str]:
        """
        The country/jurisdiction of the owner or operator of the space object.

        Examples: "US", "SPAIN"
        """
        ...

    @country.setter
    def country(self, value: Optional[str]) -> None: ...
    @property
    def eop_source(self) -> Optional[str]:
        """
        Source and version of the Earth Orientation Parameters used.

        Example: "CELESTRAK_20201028"
        """
        ...

    @eop_source.setter
    def eop_source(self, value: Optional[str]) -> None: ...
    @property
    def epoch_tzero(self) -> str:
        """
        Epoch to which all relative times in the message are referenced.

        Format: ISO 8601 (e.g., "2023-01-01T00:00:00" or "2023-001T00:00:00Z")
        """
        ...

    @epoch_tzero.setter
    def epoch_tzero(self, value: str) -> None: ...
    @property
    def international_designator(self) -> Optional[str]:
        """
        The COSPAR international designator of the space object.

        Format: YYYY-NNNP{PP}. If the object has no international designator or the content
        is either unknown or cannot be disclosed, use "UNKNOWN".

        Examples: "2000-052A", "1996-068A", "UNKNOWN"
        """
        ...

    @international_designator.setter
    def international_designator(self, value: Optional[str]) -> None: ...
    @property
    def interp_method_eop(self) -> Optional[str]:
        """
        Method used to interpolate the EOP data.
        """
        ...

    @interp_method_eop.setter
    def interp_method_eop(self, value: Optional[str]) -> None: ...
    @property
    def next_leap_epoch(self) -> Optional[str]:
        """
        Epoch of the next leap second.
        """
        ...

    @next_leap_epoch.setter
    def next_leap_epoch(self, value: Optional[str]) -> None: ...
    @property
    def next_leap_taimutc(self) -> Optional[float]:
        """
        Difference between TAI and UTC at NEXT_LEAP_EPOCH.

        :unit: s
        """
        ...

    @next_leap_taimutc.setter
    def next_leap_taimutc(self, value: Optional[float]) -> None: ...
    @property
    def next_message_epoch(self) -> Optional[str]:
        """
        The anticipated creation epoch of the next OCM message for this space object.
        """
        ...

    @next_message_epoch.setter
    def next_message_epoch(self, value: Optional[str]) -> None: ...
    @property
    def next_message_id(self) -> Optional[str]:
        """
        Unique identifier for the anticipated next OCM message for this space object.
        """
        ...

    @next_message_id.setter
    def next_message_id(self, value: Optional[str]) -> None: ...
    @property
    def object_designator(self) -> Optional[str]:
        """
        Unique satellite identification designator from the specified catalog.

        Examples: "22444", "18SPCS 18571", "UNKNOWN"
        """
        ...

    @object_designator.setter
    def object_designator(self, value: Optional[str]) -> None: ...
    @property
    def object_name(self) -> Optional[str]:
        """
        Name of the space object that the message is associated with.

        While there is no CCSDS-based restriction on the value, it is recommended to use names
        from the UN Office of Outer Space Affairs designator index. If not listed or unknown,
        use "UNKNOWN".

        Examples: "SPOT-7", "ENVISAT", "IRIDIUM NEXT-8", "INTELSAT G-15", "UNKNOWN"
        """
        ...

    @object_name.setter
    def object_name(self, value: Optional[str]) -> None: ...
    @property
    def object_type(self) -> Optional[str]:
        """
        The type of space object.

        Standard values: "PAYLOAD", "ROCKET_BODY", "DEBRIS", "UNKNOWN"
        """
        ...

    @object_type.setter
    def object_type(self, value: Optional[str]) -> None: ...
    @property
    def ocm_data_elements(self) -> Optional[str]:
        """
        Comma-delimited list of data elements included in the OCM message.

        Values: "ORB", "PHYS", "COV", "MAN", "PERT", "OD", "USER"
        """
        ...

    @ocm_data_elements.setter
    def ocm_data_elements(self, value: Optional[str]) -> None: ...
    @property
    def operator(self) -> Optional[str]:
        """
        The organization that conducts the operational control of the space object.

        Example: "INTELSAT"
        """
        ...

    @operator.setter
    def operator(self, value: Optional[str]) -> None: ...
    @property
    def ops_status(self) -> Optional[str]:
        """
        The operational status of the space object.

        Standard values: "OPERATIONAL", "NONOPERATIONAL", "PARTIALLY_OPERATIONAL", "UNKNOWN"
        """
        ...

    @ops_status.setter
    def ops_status(self, value: Optional[str]) -> None: ...
    @property
    def orbit_category(self) -> Optional[str]:
        """
        The category of orbit of the space object.

        Standard values: "GEO", "LEO", "MEO", "HEO", "NGO", "OTHER"
        """
        ...

    @orbit_category.setter
    def orbit_category(self, value: Optional[str]) -> None: ...
    @property
    def originator_address(self) -> Optional[str]:
        """
        Physical address of the Programmatic Point of Contact at the OCM originator's organization.
        """
        ...

    @originator_address.setter
    def originator_address(self, value: Optional[str]) -> None: ...
    @property
    def originator_email(self) -> Optional[str]:
        """
        Email address of the Programmatic Point of Contact at the OCM originator's organization.
        """
        ...

    @originator_email.setter
    def originator_email(self, value: Optional[str]) -> None: ...
    @property
    def originator_phone(self) -> Optional[str]:
        """
        Phone number of the Programmatic Point of Contact at the OCM originator's organization.
        """
        ...

    @originator_phone.setter
    def originator_phone(self, value: Optional[str]) -> None: ...
    @property
    def originator_poc(self) -> Optional[str]:
        """
        Programmatic Point of Contact at the OCM originator's organization.

        Example: "Mr. Rodgers"
        """
        ...

    @originator_poc.setter
    def originator_poc(self, value: Optional[str]) -> None: ...
    @property
    def originator_position(self) -> Optional[str]:
        """
        Position of the Programmatic Point of Contact at the OCM originator's organization.

        Example: "Flight Dynamics, Mission Design Lead"
        """
        ...

    @originator_position.setter
    def originator_position(self, value: Optional[str]) -> None: ...
    @property
    def owner(self) -> Optional[str]:
        """
        The organization that owns the space object.

        Example: "SIRIUS"
        """
        ...

    @owner.setter
    def owner(self, value: Optional[str]) -> None: ...
    @property
    def previous_message_epoch(self) -> Optional[str]:
        """
        The creation epoch of the previous OCM for this space object.
        """
        ...

    @previous_message_epoch.setter
    def previous_message_epoch(self, value: Optional[str]) -> None: ...
    @property
    def previous_message_id(self) -> Optional[str]:
        """
        Unique identifier for the previous OCM message for this space object.
        """
        ...

    @previous_message_id.setter
    def previous_message_id(self, value: Optional[str]) -> None: ...
    @property
    def prm_msg_link(self) -> Optional[str]:
        """
        Unique identifier of a Pointing Request Message associated with the OCM.
        """
        ...

    @prm_msg_link.setter
    def prm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def rdm_msg_link(self) -> Optional[str]:
        """
        Unique identifier of a Reentry Data Message associated with the OCM.
        """
        ...

    @rdm_msg_link.setter
    def rdm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def sclk_offset_at_epoch(self) -> Optional[float]:
        """
        Spacecraft clock count offset at EPOCH_TZERO.

        :unit: s
        """
        ...

    @sclk_offset_at_epoch.setter
    def sclk_offset_at_epoch(self, value: Optional[float]) -> None: ...
    @property
    def sclk_sec_per_si_sec(self) -> Optional[float]:
        """
        Number of spacecraft clock seconds per SI second.

        :unit: s/SI-s
        """
        ...

    @sclk_sec_per_si_sec.setter
    def sclk_sec_per_si_sec(self, value: Optional[float]) -> None: ...
    @property
    def start_time(self) -> Optional[str]:
        """
        The time of the earliest data in the message.
        """
        ...

    @start_time.setter
    def start_time(self, value: Optional[str]) -> None: ...
    @property
    def stop_time(self) -> Optional[str]:
        """
        The time of the latest data in the message.
        """
        ...

    @stop_time.setter
    def stop_time(self, value: Optional[str]) -> None: ...
    @property
    def taimutc_at_tzero(self) -> Optional[float]:
        """
        Difference between TAI and UTC at EPOCH_TZERO.

        :unit: s
        """
        ...

    @taimutc_at_tzero.setter
    def taimutc_at_tzero(self, value: Optional[float]) -> None: ...
    @property
    def tdm_msg_link(self) -> Optional[str]:
        """
        Unique identifier of a Tracking Data Message associated with the OCM.
        """
        ...

    @tdm_msg_link.setter
    def tdm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def tech_address(self) -> Optional[str]:
        """
        Physical address of the technical Point of Contact at the creating agency or operator.
        """
        ...

    @tech_address.setter
    def tech_address(self, value: Optional[str]) -> None: ...
    @property
    def tech_email(self) -> Optional[str]:
        """
        Email address of the technical Point of Contact at the creating agency or operator.
        """
        ...

    @tech_email.setter
    def tech_email(self, value: Optional[str]) -> None: ...
    @property
    def tech_org(self) -> Optional[str]:
        """
        Technical organization (creating agency or operator) that can answer questions about the message.

        Example: "NASA"
        """
        ...

    @tech_org.setter
    def tech_org(self, value: Optional[str]) -> None: ...
    @property
    def tech_phone(self) -> Optional[str]:
        """
        Phone number of the technical Point of Contact at the creating agency or operator.
        """
        ...

    @tech_phone.setter
    def tech_phone(self, value: Optional[str]) -> None: ...
    @property
    def tech_poc(self) -> Optional[str]:
        """
        Technical Point of Contact at the creating agency or operator.

        Example: "John Doe"
        """
        ...

    @tech_poc.setter
    def tech_poc(self, value: Optional[str]) -> None: ...
    @property
    def tech_position(self) -> Optional[str]:
        """
        Position of the technical Point of Contact at the creating agency or operator.
        """
        ...

    @tech_position.setter
    def tech_position(self, value: Optional[str]) -> None: ...
    @property
    def time_span(self) -> Optional[float]:
        """
        The approximate span of time covered by the data in the message [d].
        """
        ...

    @time_span.setter
    def time_span(self, value: Optional[float]) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system that shall be used for all absolute time stamps in the message.

        Standard values: "UTC", "TAI", "TT", "GPS", "TDB", "TCB", "MET", "MRT", "SCLK"
        Use of other values should be documented in an ICD.
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...
    @property
    def ut1mutc_at_tzero(self) -> Optional[float]:
        """
        Difference between UT1 and UTC at EPOCH_TZERO.

        :unit: s
        """
        ...

    @ut1mutc_at_tzero.setter
    def ut1mutc_at_tzero(self, value: Optional[float]) -> None: ...

class OcmOdParameters:
    """
    Orbit Determination Parameters.

    This block describes the orbit determination process and metadata.

    Parameters
    ----------
    od_id : str
        Identifier for the orbit determination parameters block.
        (Mandatory)
    od_method : str
        Specifies the method used for the orbit determination.
        (Mandatory)
    od_epoch : str
        Epoch of the orbit determination.
        (Mandatory)
    od_prev_id : str, optional
        Identification number for the previous orbit determination block.
        (Optional)
    comment : list[str], optional
        Comments.
        (Optional)
    """
    def __init__(
        *, od_id, od_method, od_epoch, od_prev_id=None, comment=None
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def actual_od_span(self) -> Optional[float]:
        """
        Actual time span of the observations used in the orbit determination.

        :unit: d
        """
        ...

    @actual_od_span.setter
    def actual_od_span(self, value: Optional[float]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments for the orbit determination parameters section.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def consider_n(self) -> Optional[int]:
        """
        Number of consider parameters.
        """
        ...

    @consider_n.setter
    def consider_n(self, value: Optional[int]) -> None: ...
    @property
    def consider_params(self) -> Optional[str]:
        """
        A list of the consider parameters.
        """
        ...

    @consider_params.setter
    def consider_params(self, value: Optional[str]) -> None: ...
    @property
    def data_types(self) -> Optional[str]:
        """
        A list of tracking data types used.
        """
        ...

    @data_types.setter
    def data_types(self, value: Optional[str]) -> None: ...
    @property
    def days_since_first_obs(self) -> Optional[float]:
        """
        Time elapsed since the first observation [d].
        """
        ...

    @days_since_first_obs.setter
    def days_since_first_obs(self, value: Optional[float]) -> None: ...
    @property
    def days_since_last_obs(self) -> Optional[float]:
        """
        Time elapsed since the last observation [d].
        """
        ...

    @days_since_last_obs.setter
    def days_since_last_obs(self, value: Optional[float]) -> None: ...
    @property
    def gdop(self) -> Optional[float]:
        """
        Geometric Dilution of Precision.
        """
        ...

    @gdop.setter
    def gdop(self, value: Optional[float]) -> None: ...
    @property
    def maximum_obs_gap(self) -> Optional[float]:
        """
        Maximum gap between observations.

        :unit: s
        """
        ...

    @maximum_obs_gap.setter
    def maximum_obs_gap(self, value: Optional[float]) -> None: ...
    @property
    def obs_available(self) -> Optional[int]:
        """
        Total number of individual observations available.
        """
        ...

    @obs_available.setter
    def obs_available(self, value: Optional[int]) -> None: ...
    @property
    def obs_used(self) -> Optional[int]:
        """
        Total number of individual observations used.
        """
        ...

    @obs_used.setter
    def obs_used(self, value: Optional[int]) -> None: ...
    @property
    def od_confidence(self) -> Optional[float]:
        """
        Confidence level of the orbit determination [%].

        :unit: %
        """
        ...

    @od_confidence.setter
    def od_confidence(self, value: Optional[float]) -> None: ...
    @property
    def od_epoch(self) -> str:
        """
        Epoch of the orbit determination.
        """
        ...

    @od_epoch.setter
    def od_epoch(self, value: str) -> None: ...
    @property
    def od_epoch_eigint(self) -> Optional[float]:
        """
        Intermediate axis of the OD epoch uncertainty ellipse.

        :unit: m
        """
        ...

    @od_epoch_eigint.setter
    def od_epoch_eigint(self, value: Optional[float]) -> None: ...
    @property
    def od_epoch_eigmaj(self) -> Optional[float]:
        """
        Semi-major axis of the OD epoch uncertainty ellipse.

        :unit: m
        """
        ...

    @od_epoch_eigmaj.setter
    def od_epoch_eigmaj(self, value: Optional[float]) -> None: ...
    @property
    def od_epoch_eigmin(self) -> Optional[float]:
        """
        Semi-minor axis of the OD epoch uncertainty ellipse.

        :unit: m
        """
        ...

    @od_epoch_eigmin.setter
    def od_epoch_eigmin(self, value: Optional[float]) -> None: ...
    @property
    def od_id(self) -> str:
        """
        Identifier for the orbit determination parameters block.
        """
        ...

    @od_id.setter
    def od_id(self, value: str) -> None: ...
    @property
    def od_max_pred_eigmaj(self) -> Optional[float]:
        """
        Maximum predicted semi-major axis of the error ellipsoid.

        :unit: m
        """
        ...

    @od_max_pred_eigmaj.setter
    def od_max_pred_eigmaj(self, value: Optional[float]) -> None: ...
    @property
    def od_method(self) -> str:
        """
        Specifies the method used for the orbit determination.
        """
        ...

    @od_method.setter
    def od_method(self, value: str) -> None: ...
    @property
    def od_min_pred_eigmin(self) -> Optional[float]:
        """
        Minimum predicted semi-minor axis of the error ellipsoid.

        :unit: m
        """
        ...

    @od_min_pred_eigmin.setter
    def od_min_pred_eigmin(self, value: Optional[float]) -> None: ...
    @property
    def od_prev_id(self) -> Optional[str]:
        """
        Identification number for the previous orbit determination block.
        """
        ...

    @od_prev_id.setter
    def od_prev_id(self, value: Optional[str]) -> None: ...
    @property
    def recommended_od_span(self) -> Optional[float]:
        """
        Recommended time span for the orbit determination.

        :unit: d
        """
        ...

    @recommended_od_span.setter
    def recommended_od_span(self, value: Optional[float]) -> None: ...
    @property
    def sedr(self) -> Optional[float]:
        """
        Standard Energy Dissipation Rate [W/kg].

        :unit: W/kg
        """
        ...

    @sedr.setter
    def sedr(self, value: Optional[float]) -> None: ...
    @property
    def sensors(self) -> Optional[str]:
        """
        A list of sensors used.
        """
        ...

    @sensors.setter
    def sensors(self, value: Optional[str]) -> None: ...
    @property
    def sensors_n(self) -> Optional[int]:
        """
        Number of tracking sensors used.
        """
        ...

    @sensors_n.setter
    def sensors_n(self, value: Optional[int]) -> None: ...
    @property
    def solve_n(self) -> Optional[int]:
        """
        Number of states solved for.
        """
        ...

    @solve_n.setter
    def solve_n(self, value: Optional[int]) -> None: ...
    @property
    def solve_states(self) -> Optional[str]:
        """
        A list of the states solved for.
        """
        ...

    @solve_states.setter
    def solve_states(self, value: Optional[str]) -> None: ...
    @property
    def tracks_available(self) -> Optional[int]:
        """
        Total number of observation tracks available.
        """
        ...

    @tracks_available.setter
    def tracks_available(self, value: Optional[int]) -> None: ...
    @property
    def tracks_used(self) -> Optional[int]:
        """
        Total number of observation tracks used.
        """
        ...

    @tracks_used.setter
    def tracks_used(self, value: Optional[int]) -> None: ...
    @property
    def weighted_rms(self) -> Optional[float]:
        """
        Weighted Root Mean Square error.
        """
        ...

    @weighted_rms.setter
    def weighted_rms(self, value: Optional[float]) -> None: ...

class OcmPerturbations:
    """
    Perturbation Model Specification.

    This block describes the force models used in the orbit propagation.

    Parameters
    ----------
    comment : list[str], optional
        Comments.
        (Optional)
    """
    def __init__() -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def albedo_grid_size(self) -> Optional[int]:
        """
        Size of the grid used for albedo calculations.
        """
        ...

    @albedo_grid_size.setter
    def albedo_grid_size(self, value: Optional[int]) -> None: ...
    @property
    def albedo_model(self) -> Optional[str]:
        """
        Method and version of albedo model used.
        """
        ...

    @albedo_model.setter
    def albedo_model(self, value: Optional[str]) -> None: ...
    @property
    def atmospheric_model(self) -> Optional[str]:
        """
        Specifies the atmospheric model used for drag.
        """
        ...

    @atmospheric_model.setter
    def atmospheric_model(self, value: Optional[str]) -> None: ...
    @property
    def central_body_rotation(self) -> Optional[float]:
        """
        Rotation rate of the central body.

        :unit: deg/s
        """
        ...

    @central_body_rotation.setter
    def central_body_rotation(self, value: Optional[float]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments for the perturbations section.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def equatorial_radius(self) -> Optional[float]:
        """
        Equatorial radius of the central body [km].

        :unit: km
        """
        ...

    @equatorial_radius.setter
    def equatorial_radius(self, value: Optional[float]) -> None: ...
    @property
    def fixed_f10p7(self) -> Optional[float]:
        """
        Fixed solar flux index F10.7.

        :unit: sfu
        """
        ...

    @fixed_f10p7.setter
    def fixed_f10p7(self, value: Optional[float]) -> None: ...
    @property
    def fixed_f10p7_mean(self) -> Optional[float]:
        """
        Fixed mean solar flux index F10.7.

        :unit: sfu
        """
        ...

    @fixed_f10p7_mean.setter
    def fixed_f10p7_mean(self, value: Optional[float]) -> None: ...
    @property
    def fixed_geomag_ap(self) -> Optional[float]:
        """
        Fixed geomagnetic index Ap.

        :unit: Ap
        """
        ...

    @fixed_geomag_ap.setter
    def fixed_geomag_ap(self, value: Optional[float]) -> None: ...
    @property
    def fixed_geomag_dst(self) -> Optional[float]:
        """
        Fixed geomagnetic index Dst.

        :unit: nT
        """
        ...

    @fixed_geomag_dst.setter
    def fixed_geomag_dst(self, value: Optional[float]) -> None: ...
    @property
    def fixed_geomag_kp(self) -> Optional[float]:
        """
        Fixed geomagnetic planetary index Kp.

        :unit: Kp
        """
        ...

    @fixed_geomag_kp.setter
    def fixed_geomag_kp(self, value: Optional[float]) -> None: ...
    @property
    def fixed_m10p7(self) -> Optional[float]:
        """
        Fixed solar flux index M10.7.

        :unit: sfu
        """
        ...

    @fixed_m10p7.setter
    def fixed_m10p7(self, value: Optional[float]) -> None: ...
    @property
    def fixed_m10p7_mean(self) -> Optional[float]:
        """
        Fixed mean solar flux index M10.7.

        :unit: sfu
        """
        ...

    @fixed_m10p7_mean.setter
    def fixed_m10p7_mean(self, value: Optional[float]) -> None: ...
    @property
    def fixed_s10p7(self) -> Optional[float]:
        """
        Fixed solar flux index S10.7.

        :unit: sfu
        """
        ...

    @fixed_s10p7.setter
    def fixed_s10p7(self, value: Optional[float]) -> None: ...
    @property
    def fixed_s10p7_mean(self) -> Optional[float]:
        """
        Fixed mean solar flux index S10.7.

        :unit: sfu
        """
        ...

    @fixed_s10p7_mean.setter
    def fixed_s10p7_mean(self, value: Optional[float]) -> None: ...
    @property
    def fixed_y10p7(self) -> Optional[float]:
        """
        Fixed solar flux index Y10.7.

        :unit: sfu
        """
        ...

    @fixed_y10p7.setter
    def fixed_y10p7(self, value: Optional[float]) -> None: ...
    @property
    def fixed_y10p7_mean(self) -> Optional[float]:
        """
        Fixed mean solar flux index Y10.7.

        :unit: sfu
        """
        ...

    @fixed_y10p7_mean.setter
    def fixed_y10p7_mean(self, value: Optional[float]) -> None: ...
    @property
    def gm(self) -> Optional[float]:
        """
        Mass of the central body times the gravitational constant [km**3/s**2].

        :unit: km³/s²
        """
        ...

    @gm.setter
    def gm(self, value: Optional[float]) -> None: ...
    @property
    def gravity_model(self) -> Optional[str]:
        """
        Specifies the gravity model used.
        """
        ...

    @gravity_model.setter
    def gravity_model(self, value: Optional[str]) -> None: ...
    @property
    def n_body_perturbations(self) -> Optional[str]:
        """
        Specifies the celestial bodies used for n-body perturbations.
        """
        ...

    @n_body_perturbations.setter
    def n_body_perturbations(self, value: Optional[str]) -> None: ...
    @property
    def oblate_flattening(self) -> Optional[float]:
        """
        Oblateness/flattening of the central body.
        """
        ...

    @oblate_flattening.setter
    def oblate_flattening(self, value: Optional[float]) -> None: ...
    @property
    def ocean_tides_model(self) -> Optional[str]:
        """
        Specifies the ocean tides model used.
        """
        ...

    @ocean_tides_model.setter
    def ocean_tides_model(self, value: Optional[str]) -> None: ...
    @property
    def reduction_theory(self) -> Optional[str]:
        """
        Specifies the IAU celestial body reduction theory used (e.g., IAU-2000).
        """
        ...

    @reduction_theory.setter
    def reduction_theory(self, value: Optional[str]) -> None: ...
    @property
    def shadow_bodies(self) -> Optional[str]:
        """
        List of celestial bodies that contribute to shadows.
        """
        ...

    @shadow_bodies.setter
    def shadow_bodies(self, value: Optional[str]) -> None: ...
    @property
    def shadow_model(self) -> Optional[str]:
        """
        Method and version of shadow model used.
        """
        ...

    @shadow_model.setter
    def shadow_model(self, value: Optional[str]) -> None: ...
    @property
    def solid_tides_model(self) -> Optional[str]:
        """
        Specifies the solid tides model used.
        """
        ...

    @solid_tides_model.setter
    def solid_tides_model(self, value: Optional[str]) -> None: ...
    @property
    def srp_model(self) -> Optional[str]:
        """
        Method and version of solar radiation pressure model used.
        """
        ...

    @srp_model.setter
    def srp_model(self, value: Optional[str]) -> None: ...
    @property
    def sw_data_epoch(self) -> Optional[str]:
        """
        Epoch of the space weather data.
        """
        ...

    @sw_data_epoch.setter
    def sw_data_epoch(self, value: Optional[str]) -> None: ...
    @property
    def sw_data_source(self) -> Optional[str]:
        """
        Source of space weather data used.
        """
        ...

    @sw_data_source.setter
    def sw_data_source(self, value: Optional[str]) -> None: ...
    @property
    def sw_interp_method(self) -> Optional[str]:
        """
        Method used to interpolate space weather data.
        """
        ...

    @sw_interp_method.setter
    def sw_interp_method(self, value: Optional[str]) -> None: ...

class OcmPhysicalDescription:
    """
    Space Object Physical Characteristics.

    This block describes mass, drag, solar radiation pressure,
    and other physical properties of the space object.

    Parameters
    ----------
    manufacturer : str, optional
        The manufacturer of the space object.
        (Optional)
    comment : list[str], optional
        Comments.
        (Optional)
    """
    def __init__(
        *,
        manufacturer=None,
        bus_model=None,
        docked_with=None,
        drag_const_area=None,
        drag_coeff_nom=None,
        drag_uncertainty=None,
        initial_wet_mass=None,
        wet_mass=None,
        dry_mass=None,
        oeb_parent_frame=None,
        oeb_parent_frame_epoch=None,
        oeb_q1=None,
        oeb_q2=None,
        oeb_q3=None,
        oeb_qc=None,
        oeb_max=None,
        oeb_int=None,
        oeb_min=None,
        area_along_oeb_max=None,
        area_along_oeb_int=None,
        area_along_oeb_min=None,
        area_min_for_pc=None,
        area_max_for_pc=None,
        area_typ_for_pc=None,
        rcs=None,
        rcs_min=None,
        rcs_max=None,
        srp_const_area=None,
        solar_rad_coeff=None,
        solar_rad_uncertainty=None,
        vm_absolute=None,
        vm_apparent_min=None,
        vm_apparent=None,
        vm_apparent_max=None,
        reflectance=None,
        att_control_mode=None,
        att_actuator_type=None,
        att_knowledge=None,
        att_control=None,
        att_pointing=None,
        avg_maneuver_freq=None,
        max_thrust=None,
        dv_bol=None,
        dv_remaining=None,
        ixx=None,
        iyy=None,
        izz=None,
        ixy=None,
        ixz=None,
        iyz=None,
        comment=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def area_along_oeb_int(self) -> Optional[float]:
        """
        The cross-sectional area along the intermediate OEB axis.

        :unit: m²
        """
        ...

    @area_along_oeb_int.setter
    def area_along_oeb_int(self, value: Optional[float]) -> None: ...
    @property
    def area_along_oeb_max(self) -> Optional[float]:
        """
        The cross-sectional area along the maximum OEB axis.

        :unit: m²
        """
        ...

    @area_along_oeb_max.setter
    def area_along_oeb_max(self, value: Optional[float]) -> None: ...
    @property
    def area_along_oeb_min(self) -> Optional[float]:
        """
        The cross-sectional area along the minimum OEB axis.

        :unit: m²
        """
        ...

    @area_along_oeb_min.setter
    def area_along_oeb_min(self, value: Optional[float]) -> None: ...
    @property
    def area_max_for_pc(self) -> Optional[float]:
        """
        The maximum cross-sectional area for probability of collision.

        :unit: m²
        """
        ...

    @area_max_for_pc.setter
    def area_max_for_pc(self, value: Optional[float]) -> None: ...
    @property
    def area_min_for_pc(self) -> Optional[float]:
        """
        The minimum cross-sectional area for probability of collision.

        :unit: m²
        """
        ...

    @area_min_for_pc.setter
    def area_min_for_pc(self, value: Optional[float]) -> None: ...
    @property
    def area_typ_for_pc(self) -> Optional[float]:
        """
        The typical cross-sectional area for probability of collision.

        :unit: m²
        """
        ...

    @area_typ_for_pc.setter
    def area_typ_for_pc(self, value: Optional[float]) -> None: ...
    @property
    def att_actuator_type(self) -> Optional[str]:
        """
        Type of actuator for attitude control.

        Examples: "ATT_THRUSTERS", "REACTION_WHEELS", "MAGNETIC_TORQUERS"
        """
        ...

    @att_actuator_type.setter
    def att_actuator_type(self, value: Optional[str]) -> None: ...
    @property
    def att_control(self) -> Optional[float]:
        """
        The accuracy of the attitude control.

        :unit: deg
        """
        ...

    @att_control.setter
    def att_control(self, value: Optional[float]) -> None: ...
    @property
    def att_control_mode(self) -> Optional[str]:
        """
        Primary mode of attitude control.

        Examples: "THREE_AXIS", "SPIN", "GRAVITY_GRADIENT"
        """
        ...

    @att_control_mode.setter
    def att_control_mode(self, value: Optional[str]) -> None: ...
    @property
    def att_knowledge(self) -> Optional[float]:
        """
        The accuracy of the attitude knowledge.

        :unit: deg
        """
        ...

    @att_knowledge.setter
    def att_knowledge(self, value: Optional[float]) -> None: ...
    @property
    def att_pointing(self) -> Optional[float]:
        """
        The accuracy of the attitude pointing.

        :unit: deg
        """
        ...

    @att_pointing.setter
    def att_pointing(self, value: Optional[float]) -> None: ...
    @property
    def avg_maneuver_freq(self) -> Optional[float]:
        """
        The average frequency of maneuvers [# / year].
        """
        ...

    @avg_maneuver_freq.setter
    def avg_maneuver_freq(self, value: Optional[float]) -> None: ...
    @property
    def bus_model(self) -> Optional[str]:
        """
        The model name of the space object bus.
        """
        ...

    @bus_model.setter
    def bus_model(self, value: Optional[str]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def docked_with(self) -> Optional[str]:
        """
        Identifier for another space object that is docked with the space object.
        """
        ...

    @docked_with.setter
    def docked_with(self, value: Optional[str]) -> None: ...
    @property
    def drag_coeff_nom(self) -> Optional[float]:
        """
        The nominal drag coefficient (CD).
        """
        ...

    @drag_coeff_nom.setter
    def drag_coeff_nom(self, value: Optional[float]) -> None: ...
    @property
    def drag_const_area(self) -> Optional[float]:
        """
        The constant (nominal) drag cross-sectional area.

        :unit: m²
        """
        ...

    @drag_const_area.setter
    def drag_const_area(self, value: Optional[float]) -> None: ...
    @property
    def drag_uncertainty(self) -> Optional[float]:
        """
        The uncertainty in the drag coefficient.

        :unit: %
        """
        ...

    @drag_uncertainty.setter
    def drag_uncertainty(self, value: Optional[float]) -> None: ...
    @property
    def dry_mass(self) -> Optional[float]:
        """
        The dry mass of the spacecraft [kg].
        """
        ...

    @dry_mass.setter
    def dry_mass(self, value: Optional[float]) -> None: ...
    @property
    def dv_bol(self) -> Optional[float]:
        """
        The total delta-v capability at Beginning-of-Life [km/s].
        """
        ...

    @dv_bol.setter
    def dv_bol(self, value: Optional[float]) -> None: ...
    @property
    def dv_remaining(self) -> Optional[float]:
        """
        The estimated delta-v remaining [km/s].
        """
        ...

    @dv_remaining.setter
    def dv_remaining(self, value: Optional[float]) -> None: ...
    @property
    def initial_wet_mass(self) -> Optional[float]:
        """
        The total spacecraft mass at the start of the message.

        :unit: kg
        """
        ...

    @initial_wet_mass.setter
    def initial_wet_mass(self, value: Optional[float]) -> None: ...
    @property
    def ixx(self) -> Optional[float]:
        """
        The moment of inertia Ixx.

        :unit: kg·m²
        """
        ...

    @ixx.setter
    def ixx(self, value: Optional[float]) -> None: ...
    @property
    def ixy(self) -> Optional[float]:
        """
        The product of inertia Ixy.

        :unit: kg·m²
        """
        ...

    @ixy.setter
    def ixy(self, value: Optional[float]) -> None: ...
    @property
    def ixz(self) -> Optional[float]:
        """
        The product of inertia Ixz.

        :unit: kg·m²
        """
        ...

    @ixz.setter
    def ixz(self, value: Optional[float]) -> None: ...
    @property
    def iyy(self) -> Optional[float]:
        """
        The moment of inertia Iyy.

        :unit: kg·m²
        """
        ...

    @iyy.setter
    def iyy(self, value: Optional[float]) -> None: ...
    @property
    def iyz(self) -> Optional[float]:
        """
        The product of inertia Iyz.

        :unit: kg·m²
        """
        ...

    @iyz.setter
    def iyz(self, value: Optional[float]) -> None: ...
    @property
    def izz(self) -> Optional[float]:
        """
        The moment of inertia Izz.

        :unit: kg·m²
        """
        ...

    @izz.setter
    def izz(self, value: Optional[float]) -> None: ...
    @property
    def manufacturer(self) -> Optional[str]:
        """
        The manufacturer of the space object.
        """
        ...

    @manufacturer.setter
    def manufacturer(self, value: Optional[str]) -> None: ...
    @property
    def max_thrust(self) -> Optional[float]:
        """
        The maximum thrust capability [N].
        """
        ...

    @max_thrust.setter
    def max_thrust(self, value: Optional[float]) -> None: ...
    @property
    def oeb_int(self) -> Optional[float]:
        """
        The intermediate dimension of the optimally enclosing box.

        :unit: m
        """
        ...

    @oeb_int.setter
    def oeb_int(self, value: Optional[float]) -> None: ...
    @property
    def oeb_max(self) -> Optional[float]:
        """
        The maximum dimension of the optimally enclosing box.

        :unit: m
        """
        ...

    @oeb_max.setter
    def oeb_max(self, value: Optional[float]) -> None: ...
    @property
    def oeb_min(self) -> Optional[float]:
        """
        The minimum dimension of the optimally enclosing box.

        :unit: m
        """
        ...

    @oeb_min.setter
    def oeb_min(self, value: Optional[float]) -> None: ...
    @property
    def oeb_parent_frame(self) -> Optional[str]:
        """
        The parent reference frame for the optimally enclosing box.
        """
        ...

    @oeb_parent_frame.setter
    def oeb_parent_frame(self, value: Optional[str]) -> None: ...
    @property
    def oeb_parent_frame_epoch(self) -> Optional[str]:
        """
        The epoch of the parent reference frame.
        """
        ...

    @oeb_parent_frame_epoch.setter
    def oeb_parent_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def oeb_q1(self) -> Optional[float]:
        """
        1st component of quaternion from parent frame to OEB frame.
        """
        ...

    @oeb_q1.setter
    def oeb_q1(self, value: Optional[float]) -> None: ...
    @property
    def oeb_q2(self) -> Optional[float]:
        """
        2nd component of quaternion from parent frame to OEB frame.
        """
        ...

    @oeb_q2.setter
    def oeb_q2(self, value: Optional[float]) -> None: ...
    @property
    def oeb_q3(self) -> Optional[float]:
        """
        3rd component of quaternion from parent frame to OEB frame.
        """
        ...

    @oeb_q3.setter
    def oeb_q3(self, value: Optional[float]) -> None: ...
    @property
    def oeb_qc(self) -> Optional[float]:
        """
        scalar component of quaternion from parent frame to OEB frame.
        """
        ...

    @oeb_qc.setter
    def oeb_qc(self, value: Optional[float]) -> None: ...
    @property
    def rcs(self) -> Optional[float]:
        """
        The typical Radar Cross Section.

        :unit: m²
        """
        ...

    @rcs.setter
    def rcs(self, value: Optional[float]) -> None: ...
    @property
    def rcs_max(self) -> Optional[float]:
        """
        The maximum Radar Cross Section observed [m**2].
        """
        ...

    @rcs_max.setter
    def rcs_max(self, value: Optional[float]) -> None: ...
    @property
    def rcs_min(self) -> Optional[float]:
        """
        The minimum Radar Cross Section observed [m**2].
        """
        ...

    @rcs_min.setter
    def rcs_min(self, value: Optional[float]) -> None: ...
    @property
    def reflectance(self) -> Optional[float]:
        """
        The reflectance of the space object.

        :range: 0 to 1
        """
        ...

    @reflectance.setter
    def reflectance(self, value: Optional[float]) -> None: ...
    @property
    def solar_rad_coeff(self) -> Optional[float]:
        """
        The nominal solar radiation pressure coefficient (CR).
        """
        ...

    @solar_rad_coeff.setter
    def solar_rad_coeff(self, value: Optional[float]) -> None: ...
    @property
    def solar_rad_uncertainty(self) -> Optional[float]:
        """
        The uncertainty in the solar radiation pressure coefficient.

        :unit: %
        """
        ...

    @solar_rad_uncertainty.setter
    def solar_rad_uncertainty(self, value: Optional[float]) -> None: ...
    @property
    def srp_const_area(self) -> Optional[float]:
        """
        The constant (nominal) solar radiation pressure cross-sectional area.

        :unit: m²
        """
        ...

    @srp_const_area.setter
    def srp_const_area(self, value: Optional[float]) -> None: ...
    @property
    def vm_absolute(self) -> Optional[float]:
        """
        The absolute Visual Magnitude.
        """
        ...

    @vm_absolute.setter
    def vm_absolute(self, value: Optional[float]) -> None: ...
    @property
    def vm_apparent(self) -> Optional[float]:
        """
        The apparent Visual Magnitude.
        """
        ...

    @vm_apparent.setter
    def vm_apparent(self, value: Optional[float]) -> None: ...
    @property
    def vm_apparent_max(self) -> Optional[float]:
        """
        The maximum apparent Visual Magnitude.
        """
        ...

    @vm_apparent_max.setter
    def vm_apparent_max(self, value: Optional[float]) -> None: ...
    @property
    def vm_apparent_min(self) -> Optional[float]:
        """
        The minimum apparent Visual Magnitude.
        """
        ...

    @vm_apparent_min.setter
    def vm_apparent_min(self, value: Optional[float]) -> None: ...
    @property
    def wet_mass(self) -> Optional[float]:
        """
        Total spacecraft mass at the current state epoch [kg].
        """
        ...

    @wet_mass.setter
    def wet_mass(self, value: Optional[float]) -> None: ...

class OcmSegment:
    """
    OCM Segment containing metadata and data.

    In OCM, a single segment is used to represent orbit data for a single space object.

    Parameters
    ----------
    metadata : OcmMetadata
        Segment metadata.
        (Mandatory)
    data : OcmData
        Segment data blocks.
        (Mandatory)
    """
    def __init__(metadata, data) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def data(self) -> OcmData:
        """
        Segment data blocks.
        """
        ...

    @data.setter
    def data(self, value: OcmData) -> None: ...
    @property
    def metadata(self) -> OcmMetadata:
        """
        Segment metadata.
        """
        ...

    @metadata.setter
    def metadata(self, value: OcmMetadata) -> None: ...

class OcmTrajState:
    """
    OCM Trajectory State Time History segment.

    This block contains orbit states for a single object, optionally
    spanning fictitious nodes for interpolation smoothness.

    Parameters
    ----------
    center_name : str
        Origin of the orbit reference frame.
        (Mandatory)
    traj_ref_frame : str
        Reference frame of the trajectory state time history.
        (Mandatory)
    traj_type : str
        Specifies the trajectory state element set type.
        (Mandatory)
    traj_lines : list of TrajLine
        Contiguous set of trajectory state data lines.
        (Mandatory)
    traj_id : str, optional
        Identification number for this trajectory state time history block.
        (Optional)
    traj_prev_id : str, optional
        Identification number for the previous trajectory state time history.
        (Optional)
    traj_next_id : str, optional
        Identification number for the next trajectory state time history.
        (Optional)
    traj_basis : str, optional
        The basis of this trajectory state time history data (PREDICTED, DETERMINED, etc.).
        (Optional)
    traj_basis_id : str, optional
        Identification number for the telemetry dataset, orbit determination, or simulation.
        (Optional)
    interpolation : str, optional
        Recommended interpolation method for the ephemeris data.
        (Optional)
    interpolation_degree : int, optional
        Recommended interpolation degree.
        (Conditional)
    propagator : str, optional
        Name of the orbit propagator used to create this trajectory state time history.
        (Optional)
    traj_frame_epoch : str, optional
        Epoch of the orbit data reference frame, if not intrinsic to the definition.
        (Conditional)
    useable_start_time : str, optional
        Start time of the useable time span covered by the ephemeris data.
        (Optional)
    useable_stop_time : str, optional
        Stop time of the useable time span covered by the ephemeris data.
        (Optional)
    orb_revnum : float, optional
        The integer orbit revolution number associated with the first trajectory state.
        (Optional)
    orb_revnum_basis : str, optional
        Specifies the message creator’s basis for their orbit revolution counter (0 or 1).
        (Conditional)
    orb_averaging : str, optional
        Specifies whether the orbit elements are osculating elements or mean elements.
        (Conditional)
    traj_units : str, optional
        Comma-delimited set of SI unit designations for the trajectory state elements.
        (Optional)
    comment : list[str], optional
        Comments.
        (Optional)
    """
    def __init__(
        *,
        center_name,
        traj_ref_frame,
        traj_type,
        traj_lines,
        traj_id=None,
        traj_prev_id=None,
        traj_next_id=None,
        traj_basis=None,
        traj_basis_id=None,
        interpolation=None,
        interpolation_degree=None,
        propagator=None,
        traj_frame_epoch=None,
        useable_start_time=None,
        useable_stop_time=None,
        orb_revnum=None,
        orb_revnum_basis=None,
        orb_averaging=None,
        traj_units=None,
        comment=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def center_name(self) -> str:
        """
        Origin of the orbit reference frame.

        Standard values include natural solar system bodies (e.g., "EARTH", "MOON", "SUN").
        """
        ...

    @center_name.setter
    def center_name(self, value: str) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments for this trajectory block.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def interpolation(self) -> Optional[str]:
        """
        Recommended interpolation method for the state elements.
        """
        ...

    @interpolation.setter
    def interpolation(self, value: Optional[str]) -> None: ...
    @property
    def interpolation_degree(self) -> Optional[int]:
        """
        Recommended interpolation degree for the state elements.
        """
        ...

    @interpolation_degree.setter
    def interpolation_degree(self, value: Optional[int]) -> None: ...
    @property
    def orb_averaging(self) -> Optional[str]:
        """
        Specifies the averaging method for orbital elements.
        """
        ...

    @orb_averaging.setter
    def orb_averaging(self, value: Optional[str]) -> None: ...
    @property
    def orb_revnum(self) -> Optional[float]:
        """
        Integer orbit revolution number at the epoch of the first trajectory data line.
        """
        ...

    @orb_revnum.setter
    def orb_revnum(self, value: Optional[float]) -> None: ...
    @property
    def orb_revnum_basis(self) -> Optional[str]:
        """
        Basis for the orbit revolution counter (0 or 1).
        """
        ...

    @orb_revnum_basis.setter
    def orb_revnum_basis(self, value: Optional[str]) -> None: ...
    @property
    def propagator(self) -> Optional[str]:
        """
        The name of the propagator used in the creation of the trajectory state data.
        """
        ...

    @propagator.setter
    def propagator(self, value: Optional[str]) -> None: ...
    @property
    def traj_basis(self) -> Optional[str]:
        """
        The basis of this trajectory state time history data.

        Standard values: "PREDICTED", "DETERMINED", "SIMULATED", "OTHER"
        """
        ...

    @traj_basis.setter
    def traj_basis(self, value: Optional[str]) -> None: ...
    @property
    def traj_basis_id(self) -> Optional[str]:
        """
        Identification number for the telemetry dataset, orbit determination, or simulation.
        """
        ...

    @traj_basis_id.setter
    def traj_basis_id(self, value: Optional[str]) -> None: ...
    @property
    def traj_frame_epoch(self) -> Optional[str]:
        """
        Epoch of the orbit data reference frame, if not intrinsic to the definition.
        """
        ...

    @traj_frame_epoch.setter
    def traj_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def traj_id(self) -> Optional[str]:
        """
        Identification number for this trajectory state time history block.
        """
        ...

    @traj_id.setter
    def traj_id(self, value: Optional[str]) -> None: ...
    @property
    def traj_lines(self) -> list[TrajLine]:
        """
        Contiguous set of trajectory state data lines.
        """
        ...

    @traj_lines.setter
    def traj_lines(self, value: list[TrajLine]) -> None: ...
    @property
    def traj_next_id(self) -> Optional[str]:
        """
        Identification number for the next trajectory state time history.
        """
        ...

    @traj_next_id.setter
    def traj_next_id(self, value: Optional[str]) -> None: ...
    @property
    def traj_prev_id(self) -> Optional[str]:
        """
        Identification number for the previous trajectory state time history.
        """
        ...

    @traj_prev_id.setter
    def traj_prev_id(self, value: Optional[str]) -> None: ...
    @property
    def traj_ref_frame(self) -> str:
        """
        Reference frame of the trajectory state time history.

        Standard values: "ICRF", "EME2000", "ITRF", "TEME", "GCRF"
        """
        ...

    @traj_ref_frame.setter
    def traj_ref_frame(self, value: str) -> None: ...
    @property
    def traj_type(self) -> str:
        """
        Specifies the trajectory state element set type.

        Standard values: "CARTPV", "KEPLERIAN", "EQUINOCTIAL", "TLE"
        """
        ...

    @traj_type.setter
    def traj_type(self, value: str) -> None: ...
    @property
    def traj_units(self) -> Optional[str]:
        """
        SI unit designations for the state elements.
        """
        ...

    @traj_units.setter
    def traj_units(self, value: Optional[str]) -> None: ...
    @property
    def useable_start_time(self) -> Optional[str]:
        """
        Start time of the useable time span covered by the ephemeris data.
        """
        ...

    @useable_start_time.setter
    def useable_start_time(self, value: Optional[str]) -> None: ...
    @property
    def useable_stop_time(self) -> Optional[str]:
        """
        Stop time of the useable time span covered by the ephemeris data.
        """
        ...

    @useable_stop_time.setter
    def useable_stop_time(self, value: Optional[str]) -> None: ...

class OdParameters:
    """
    Orbit Determination Parameters.
    """
    def __init__(comment) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def actual_od_span(self) -> Optional[float]:
        """
        Actual orbit determination span (days).
        """
        ...

    @actual_od_span.setter
    def actual_od_span(self, value: Optional[float]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def obs_available(self) -> Optional[int]:
        """
        Number of observations available.
        """
        ...

    @obs_available.setter
    def obs_available(self, value: Optional[int]) -> None: ...
    @property
    def obs_used(self) -> Optional[int]:
        """
        Number of observations used.
        """
        ...

    @obs_used.setter
    def obs_used(self, value: Optional[int]) -> None: ...
    @property
    def recommended_od_span(self) -> Optional[float]:
        """
        Recommended orbit determination span (days).
        """
        ...

    @recommended_od_span.setter
    def recommended_od_span(self, value: Optional[float]) -> None: ...
    @property
    def residuals_accepted(self) -> Optional[float]:
        """
        Residuals accepted (percentage).
        """
        ...

    @residuals_accepted.setter
    def residuals_accepted(self, value: Optional[float]) -> None: ...
    @property
    def time_lastob_end(self) -> Optional[str]:
        """
        Time of last observation end.
        """
        ...

    @time_lastob_end.setter
    def time_lastob_end(self, value: Optional[str]) -> None: ...
    @property
    def time_lastob_start(self) -> Optional[str]:
        """
        Time of last observation start.
        """
        ...

    @time_lastob_start.setter
    def time_lastob_start(self, value: Optional[str]) -> None: ...
    @property
    def tracks_available(self) -> Optional[int]:
        """
        Number of tracks available.
        """
        ...

    @tracks_available.setter
    def tracks_available(self, value: Optional[int]) -> None: ...
    @property
    def tracks_used(self) -> Optional[int]:
        """
        Number of tracks used.
        """
        ...

    @tracks_used.setter
    def tracks_used(self, value: Optional[int]) -> None: ...
    @property
    def weighted_rms(self) -> Optional[float]:
        """
        Weighted RMS.
        """
        ...

    @weighted_rms.setter
    def weighted_rms(self, value: Optional[float]) -> None: ...

class OdmHeader:
    """
    Represents the header of a CCSDS Orbit Data Message (ODM).

    The header contains metadata common to all ODM message types.

    Parameters
    ----------
    creation_date : str
        File creation date/time in UTC.
    originator : str
        Creating agency or operator.
    classification : str, optional
        User-defined free-text message classification/caveats.
    message_id : str, optional
        ID that uniquely identifies a message from a given originator.
    comment : list of str, optional
        Comments.
    """
    def __init__(
        creation_date, originator, classification, message_id, comment
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def classification(self) -> Optional[str]:
        """
        User-defined free-text message classification/caveats.
        """
        ...

    @classification.setter
    def classification(self, value: Optional[str]) -> None: ...
    @property
    def comments(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    @property
    def creation_date(self) -> str:
        """
        File creation date/time in UTC.
        """
        ...

    @creation_date.setter
    def creation_date(self, value: str) -> None: ...
    @property
    def message_id(self) -> Optional[str]:
        """
        ID that uniquely identifies a message from a given originator.
        """
        ...

    @message_id.setter
    def message_id(self, value: Optional[str]) -> None: ...
    @property
    def originator(self) -> str:
        """
        Creating agency or operator.
        """
        ...

    @originator.setter
    def originator(self, value: str) -> None: ...

class Oem:
    """
    Represents a CCSDS Orbit Ephemeris Message (OEM).

    This is the top-level object for an OEM file. It contains a header
    and a list of ephemeris data segments.

    Parameters
    ----------
    header : OdmHeader
        The message header.
        (Mandatory)
    segments : list of OemSegment
        The list of data segments.
        (Mandatory)
    """
    def __init__(header, segments) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @staticmethod
    def from_file(path, format):
        """
        Create an OEM message from a file.

        Parameters
        ----------
        path : str
            Path to the input file.
        format : str, optional
            Format ('kvn' or 'xml'). Auto-detected if None.
            (Optional)

        Returns
        -------
        Oem
            The parsed OEM object.
        """
        ...

    @staticmethod
    def from_str(data, format):
        """
        Create an OEM message from a string.

        Parameters
        ----------
        data : str
            Input string/content.
        format : str, optional
            Format ('kvn' or 'xml'). Auto-detected if None.
            (Optional)

        Returns
        -------
        Oem
            The parsed OEM object.
        """
        ...

    @property
    def header(self) -> OdmHeader:
        """
        The message header.
        """
        ...

    @header.setter
    def header(self, value: OdmHeader) -> None: ...
    @property
    def segments(self) -> list[OemSegment]:
        """
        The list of data segments.
        """
        ...

    @segments.setter
    def segments(self, value: list[OemSegment]) -> None: ...
    def to_file(self, path, format):
        """
        Write to file.

        Parameters
        ----------
        path : str
            Output file path.
        format : str
            Output format ('kvn' or 'xml').
        """
        ...

    def to_str(self, format):
        """
        Serialize to string.

        Parameters
        ----------
        format : str
            Output format ('kvn' or 'xml').

        Returns
        -------
        str
            The serialized string.
        """
        ...

class OemCovarianceMatrix:
    """
    Represents a covariance matrix at a specific epoch.

    The matrix provides uncertainty information for the state vector at the
    same epoch.

    Parameters
    ----------
    epoch : str
        Epoch of the covariance matrix.
    values : numpy.ndarray
        Flat NumPy array of length 21 containing the covariance values.
    cov_ref_frame : str, optional
        Reference frame for the covariance matrix.
        (Optional)
    comment : list[str], optional
        Comments associated with this covariance matrix.
        (Optional)
    """
    def __init__(epoch, values, cov_ref_frame, comment) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comments(self) -> list[str]:
        """
        Comments associated with this covariance matrix.
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    @property
    def cov_ref_frame(self) -> Optional[str]:
        """
        Reference frame for the covariance matrix.

        If omitted, the covariance is expressed in the same reference frame
        as the state vector (REF_FRAME from metadata).

        Examples: "GCRF", "EME2000", "ITRF2000", "RTN" (Radial-Tangential-Normal)
        """
        ...

    @cov_ref_frame.setter
    def cov_ref_frame(self, value: Optional[str]) -> None: ...
    @property
    def cx_dot_x(self) -> float:
        """
        Position-velocity covariance [4,1] element (X_DOT-X covariance).

        Units: km²/s
        """
        ...

    @cx_dot_x.setter
    def cx_dot_x(self, value: float) -> None: ...
    @property
    def cx_dot_x_dot(self) -> float:
        """
        Velocity covariance [4,4] element (X_DOT-X_DOT variance).

        Units: km²/s²
        """
        ...

    @cx_dot_x_dot.setter
    def cx_dot_x_dot(self, value: float) -> None: ...
    @property
    def cx_dot_y(self) -> float:
        """
        Position-velocity covariance [4,2] element (X_DOT-Y covariance).

        Units: km²/s
        """
        ...

    @cx_dot_y.setter
    def cx_dot_y(self, value: float) -> None: ...
    @property
    def cx_dot_z(self) -> float:
        """
        Position-velocity covariance [4,3] element (X_DOT-Z covariance).

        Units: km²/s
        """
        ...

    @cx_dot_z.setter
    def cx_dot_z(self, value: float) -> None: ...
    @property
    def cx_x(self) -> float:
        """
        Position covariance [1,1] element (X-X variance).

        Units: km²
        """
        ...

    @cx_x.setter
    def cx_x(self, value: float) -> None: ...
    @property
    def cy_dot_x(self) -> float:
        """
        Position-velocity covariance [5,1] element (Y_DOT-X covariance).

        Units: km²/s
        """
        ...

    @cy_dot_x.setter
    def cy_dot_x(self, value: float) -> None: ...
    @property
    def cy_dot_x_dot(self) -> float:
        """
        Velocity covariance [5,4] element (Y_DOT-X_DOT covariance).

        Units: km²/s²
        """
        ...

    @cy_dot_x_dot.setter
    def cy_dot_x_dot(self, value: float) -> None: ...
    @property
    def cy_dot_y(self) -> float:
        """
        Position-velocity covariance [5,2] element (Y_DOT-Y covariance).

        Units: km²/s
        """
        ...

    @cy_dot_y.setter
    def cy_dot_y(self, value: float) -> None: ...
    @property
    def cy_dot_y_dot(self) -> float:
        """
        Velocity covariance [5,5] element (Y_DOT-Y_DOT variance).

        Units: km²/s²
        """
        ...

    @cy_dot_y_dot.setter
    def cy_dot_y_dot(self, value: float) -> None: ...
    @property
    def cy_dot_z(self) -> float:
        """
        Position-velocity covariance [5,3] element (Y_DOT-Z covariance).

        Units: km²/s
        """
        ...

    @cy_dot_z.setter
    def cy_dot_z(self, value: float) -> None: ...
    @property
    def cy_x(self) -> float:
        """
        Position covariance [2,1] element (Y-X covariance).

        Units: km²
        """
        ...

    @cy_x.setter
    def cy_x(self, value: float) -> None: ...
    @property
    def cy_y(self) -> float:
        """
        Position covariance [2,2] element (Y-Y variance).

        Units: km²
        """
        ...

    @cy_y.setter
    def cy_y(self, value: float) -> None: ...
    @property
    def cz_dot_x(self) -> float:
        """
        Position-velocity covariance [6,1] element (Z_DOT-X covariance).

        Units: km²/s
        """
        ...

    @cz_dot_x.setter
    def cz_dot_x(self, value: float) -> None: ...
    @property
    def cz_dot_x_dot(self) -> float:
        """
        Velocity covariance [6,4] element (Z_DOT-X_DOT covariance).

        Units: km²/s²
        """
        ...

    @cz_dot_x_dot.setter
    def cz_dot_x_dot(self, value: float) -> None: ...
    @property
    def cz_dot_y(self) -> float:
        """
        Position-velocity covariance [6,2] element (Z_DOT-Y covariance).

        Units: km²/s
        """
        ...

    @cz_dot_y.setter
    def cz_dot_y(self, value: float) -> None: ...
    @property
    def cz_dot_y_dot(self) -> float:
        """
        Velocity covariance [6,5] element (Z_DOT-Y_DOT covariance).

        Units: km²/s²
        """
        ...

    @cz_dot_y_dot.setter
    def cz_dot_y_dot(self, value: float) -> None: ...
    @property
    def cz_dot_z(self) -> float:
        """
        Position-velocity covariance [6,3] element (Z_DOT-Z covariance).

        Units: km²/s
        """
        ...

    @cz_dot_z.setter
    def cz_dot_z(self, value: float) -> None: ...
    @property
    def cz_dot_z_dot(self) -> float:
        """
        Velocity covariance [6,6] element (Z_DOT-Z_DOT variance).

        Units: km²/s²
        """
        ...

    @cz_dot_z_dot.setter
    def cz_dot_z_dot(self, value: float) -> None: ...
    @property
    def cz_x(self) -> float:
        """
        Position covariance [3,1] element (Z-X covariance).

        Units: km²
        """
        ...

    @cz_x.setter
    def cz_x(self, value: float) -> None: ...
    @property
    def cz_y(self) -> float:
        """
        Position covariance [3,2] element (Z-Y covariance).

        Units: km²
        """
        ...

    @cz_y.setter
    def cz_y(self, value: float) -> None: ...
    @property
    def cz_z(self) -> float:
        """
        Position covariance [3,3] element (Z-Z variance).

        Units: km²
        """
        ...

    @cz_z.setter
    def cz_z(self, value: float) -> None: ...
    @property
    def epoch(self) -> str:
        """
        Epoch of the covariance matrix.

        Time at which the covariance matrix applies, given in the time system
        specified in the metadata.

        Format: ISO 8601 (e.g., "2023-01-01T00:00:00")
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...

class OemData:
    """
    Contains the ephemeris data for an OEM segment.

    This includes a list of state vectors and an optional list of
    corresponding covariance matrices.

    Parameters
    ----------
    state_vectors : list of StateVectorAcc
        List of state vectors.
    """
    def __init__(state_vectors, comments) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comments(self) -> list[str]:
        """
        List of comments in the data section.

        Comments can appear between state vectors and covariance matrices to provide
        additional context or annotations about the ephemeris data.
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    @property
    def covariance_matrices(self) -> list[OemCovarianceMatrix]:
        """
        List of covariance matrices associated with the state vectors.

        Each 6x6 covariance matrix provides uncertainty information for position and velocity:
        - Position covariance in km²
        - Position-velocity cross-covariance in km²/s
        - Velocity covariance in km²/s²

        Matrices are given in lower triangular form in the covariance reference frame.
        """
        ...

    @covariance_matrices.setter
    def covariance_matrices(self, value: list[OemCovarianceMatrix]) -> None: ...
    @property
    def covariance_matrices_numpy(self) -> tuple[list[str], numpy.ndarray]:
        """
        Get covariance matrices as a tuple associated with a NumPy array.

        Returns:
            tuple[list[str], np.ndarray]: (Epochs, 2D Array of size Nx21).
        """
        ...

    @covariance_matrices_numpy.setter
    def covariance_matrices_numpy(
        self, value: tuple[list[str], numpy.ndarray]
    ) -> None: ...
    @property
    def state_vectors(self) -> list[StateVectorAcc]:
        """
        List of state vectors (position, velocity, and optionally acceleration).

        Each state vector includes:
        - Position components (X, Y, Z) in km
        - Velocity components (X_DOT, Y_DOT, Z_DOT) in km/s
        - Optional acceleration components (X_DDOT, Y_DDOT, Z_DDOT) in km/s²

        All components are expressed in the reference frame specified in the metadata.
        """
        ...

    @state_vectors.setter
    def state_vectors(self, value: list[StateVectorAcc]) -> None: ...
    @property
    def state_vectors_numpy(self) -> tuple[list[str], numpy.ndarray]:
        """
        State vectors as a tuple of epochs and NumPy array.

        Returns
        -------
        tuple[list[str], numpy.ndarray]
            A tuple containing:
            - List of epoch strings (ISO 8601 format)
            - 2D NumPy array of shape (N, 6) or (N, 9):
              - N x 6: [X, Y, Z, X_DOT, Y_DOT, Z_DOT] if no accelerations
              - N x 9: [X, Y, Z, X_DOT, Y_DOT, Z_DOT, X_DDOT, Y_DDOT, Z_DDOT] if accelerations present

            Units: position in km, velocity in km/s, acceleration in km/s²
        """
        ...

    @state_vectors_numpy.setter
    def state_vectors_numpy(self, value: tuple[list[str], numpy.ndarray]) -> None: ...

class OemMetadata:
    """
    Represents the metadata for a single OEM segment.

    This metadata provides context for the ephemeris data in the segment,
    such as the object being described, the reference frame, and the time system.

    Parameters
    ----------
    object_name : str
        Spacecraft name for which orbit state data is provided.
    object_id : str
        Object identifier of the object for which orbit state data is provided.
    center_name : str
        Origin of the reference frame.
    ref_frame : str
        Reference frame in which state vector data is given.
    time_system : str
        Time system used for state vector, maneuver, and covariance data.
    start_time : str
        Start time of the total time span covered by the ephemeris data.
    stop_time : str
        Stop time of the total time span covered by the ephemeris data.
    ref_frame_epoch : str, optional
        Epoch of the reference frame, if not intrinsic to the definition.
        (Optional)
    useable_start_time : str, optional
        Start of the recommended time span for use of the ephemeris data.
        (Optional)
    useable_stop_time : str, optional
        End of the recommended time span for use of the ephemeris data.
        (Optional)
    interpolation : str, optional
        Recommended interpolation method for ephemeris data.
        (Optional)
    interpolation_degree : int, optional
        Degree of the interpolation polynomial.
        (Optional)
    comment : list[str], optional
        Comments.
        (Optional)
    """
    def __init__(
        object_name,
        object_id,
        center_name,
        ref_frame,
        time_system,
        start_time,
        stop_time,
        ref_frame_epoch,
        useable_start_time,
        useable_stop_time,
        interpolation,
        interpolation_degree,
        comment,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def center_name(self) -> str:
        """
        Origin of the reference frame.

        Must be a natural solar system body (planets, asteroids, comets, and natural satellites),
        including any planet barycenter or the solar system barycenter.

        Examples: "EARTH", "EARTH_BARYCENTER", "MOON", "SOLAR_SYSTEM_BARYCENTER", "SUN", "MARS", "JUPITER_BARYCENTER"
        """
        ...

    @center_name.setter
    def center_name(self, value: str) -> None: ...
    @property
    def interpolation(self) -> Optional[str]:
        """
        Recommended interpolation method for ephemeris data.

        Specifies the interpolation technique to be used when estimating state vectors
        at times between the provided data points.

        Common values: "HERMITE", "LAGRANGE", "LINEAR", "CHEBYSHEV"
        """
        ...

    @interpolation.setter
    def interpolation(self, value: Optional[str]) -> None: ...
    @property
    def interpolation_degree(self) -> Optional[int]:
        """
        Degree of the interpolation polynomial.

        Specifies the polynomial degree to use for interpolation methods that require it
        (e.g., HERMITE, LAGRANGE). Must be a positive integer.
        """
        ...

    @interpolation_degree.setter
    def interpolation_degree(self, value: Optional[int]) -> None: ...
    @property
    def object_id(self) -> str:
        """
        Object identifier of the object for which orbit state data is provided.

        It is recommended to use the international spacecraft designator as published in the
        UN Office of Outer Space Affairs designator index. Format: YYYY-NNNP{PP}.
        If not listed or unknown, use "UNKNOWN".

        Examples: "1998-067A", "2011-037A", "UNKNOWN"
        """
        ...

    @object_id.setter
    def object_id(self, value: str) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Spacecraft name for which orbit state data is provided.

        While there is no CCSDS-based restriction on the value, it is recommended to use names
        from the UN Office of Outer Space Affairs designator index. If not listed or unknown,
        use "UNKNOWN".

        Examples: "EUTELSAT W1", "MARS PATHFINDER", "STS_106", "NEAR", "UNKNOWN"
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def ref_frame(self) -> str:
        """
        Reference frame in which state vector data is given.

        Standard values: "ICRF", "EME2000", "ITRF2000", "TEME", "GCRF"
        Use of other values should be documented in an ICD.
        """
        ...

    @ref_frame.setter
    def ref_frame(self, value: str) -> None: ...
    @property
    def ref_frame_epoch(self) -> Optional[str]:
        """
        Epoch of the reference frame, if not intrinsic to the definition.

        This field is conditional and only required if the reference frame definition
        requires an explicit epoch (e.g., for precessing frames).

        Format: ISO 8601 (e.g., "2000-01-01T00:00:00")
        """
        ...

    @ref_frame_epoch.setter
    def ref_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def start_time(self) -> str:
        """
        Start time of the total time span covered by the ephemeris data.

        Format: ISO 8601 (e.g., "2023-01-01T00:00:00" or "2023-001T00:00:00Z")
        """
        ...

    @start_time.setter
    def start_time(self, value: str) -> None: ...
    @property
    def stop_time(self) -> str:
        """
        Stop time of the total time span covered by the ephemeris data.

        Format: ISO 8601 (e.g., "2023-01-02T00:00:00" or "2023-002T00:00:00Z")
        """
        ...

    @stop_time.setter
    def stop_time(self, value: str) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system used for state vector, maneuver, and covariance data.

        Standard values: "UTC", "TAI", "TT", "GPS", "TDB", "TCB", "MET", "MRT", "SCLK"
        Use of other values should be documented in an ICD.
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...
    @property
    def useable_start_time(self) -> Optional[str]:
        """
        Start of the recommended time span for use of the ephemeris data.

        Indicates the earliest time at which the ephemeris data is recommended for use.
        The data may be less accurate outside the useable time range.

        Format: ISO 8601 (e.g., "2023-01-01T01:00:00")
        """
        ...

    @useable_start_time.setter
    def useable_start_time(self, value: Optional[str]) -> None: ...
    @property
    def useable_stop_time(self) -> Optional[str]:
        """
        End of the recommended time span for use of the ephemeris data.

        Indicates the latest time at which the ephemeris data is recommended for use.
        The data may be less accurate outside the useable time range.

        Format: ISO 8601 (e.g., "2023-01-01T23:00:00")
        """
        ...

    @useable_stop_time.setter
    def useable_stop_time(self, value: Optional[str]) -> None: ...

class OemSegment:
    """
    Represents a single segment of an OEM.

    An OEM file can contain multiple segments, each with its own metadata
    and ephemeris data (state vectors and optional covariance matrices).

    Parameters
    ----------
    metadata : OemMetadata
        Segment metadata.
    data : OemData
        Segment data.
    """
    def __init__(metadata, data) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def data(self) -> OemData:
        """
        Segment data.
        """
        ...

    @data.setter
    def data(self, value: OemData) -> None: ...
    @property
    def metadata(self) -> OemMetadata:
        """
        Segment metadata.
        """
        ...

    @metadata.setter
    def metadata(self, value: OemMetadata) -> None: ...

class Omm:
    """
    Create a new OMM message.

    Parameters
    ----------
    header : OdmHeader
        The message header.
    segment : OmmSegment
        The data segment.
    """
    def __init__(header, segment) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @staticmethod
    def from_file(path, format):
        """
        Create an OMM message from a file.

        Parameters
        ----------
        path : str
            Path to the input file.
        format : str, optional
            Format ('kvn' or 'xml'). Auto-detected if None.

        Returns
        -------
        Omm
            The parsed OMM object.
        """
        ...

    @staticmethod
    def from_str(data, format):
        """ """
        ...

    @property
    def header(self) -> OdmHeader:
        """
        The message header.
        """
        ...

    @header.setter
    def header(self, value: OdmHeader) -> None: ...
    @property
    def segment(self) -> OmmSegment:
        """
        The data segment.
        """
        ...

    @segment.setter
    def segment(self, value: OmmSegment) -> None: ...
    def to_file(self, path, format):
        """
        Write to file.

        Parameters
        ----------
        path : str
            Output file path.
        format : str
            Output format ('kvn' or 'xml').
        """
        ...

    def to_str(self, format):
        """
        Serialize to string.

        Parameters
        ----------
        format : str
            Output format ('kvn' or 'xml').
            (Mandatory)

        Returns
        -------
        str
            The serialized string.
        """
        ...

class OmmData:
    def __init__(mean_elements, comments) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comments(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    @property
    def covariance_matrix(self) -> Optional[OpmCovarianceMatrix]:
        """
        Covariance matrix.
        """
        ...

    @covariance_matrix.setter
    def covariance_matrix(self, value: Optional[OpmCovarianceMatrix]) -> None: ...
    @property
    def mean_elements(self) -> MeanElements:
        """
        Mean elements.
        """
        ...

    @mean_elements.setter
    def mean_elements(self, value: MeanElements) -> None: ...
    @property
    def spacecraft_parameters(self) -> Optional[SpacecraftParameters]:
        """
        Spacecraft parameters.
        """
        ...

    @spacecraft_parameters.setter
    def spacecraft_parameters(self, value: Optional[SpacecraftParameters]) -> None: ...
    @property
    def tle_parameters(self) -> Optional[TleParameters]:
        """
        TLE parameters.
        """
        ...

    @tle_parameters.setter
    def tle_parameters(self, value: Optional[TleParameters]) -> None: ...
    @property
    def user_defined_parameters(self) -> Optional[UserDefined]:
        """
        User defined parameters.
        """
        ...

    @user_defined_parameters.setter
    def user_defined_parameters(self, value: Optional[UserDefined]) -> None: ...

class OmmMetadata:
    """
    Create a new OMM Metadata object.

    Parameters
    ----------
    object_name : str
        Spacecraft name.
    object_id : str
        Object identifier.
    center_name : str
        Origin of the reference frame.
    ref_frame : str
        Reference frame.
    time_system : str
        Time system.
    mean_element_theory : str
        Description of the Mean Element Theory.
    ref_frame_epoch : str, optional
        Epoch of the reference frame.
    comment : list of str, optional
        Comments.
    """
    def __init__(
        object_name,
        object_id,
        center_name,
        ref_frame,
        time_system,
        mean_element_theory,
        ref_frame_epoch,
        comment,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def center_name(self) -> str:
        """
        Origin of the reference frame.
        """
        ...

    @center_name.setter
    def center_name(self, value: str) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def mean_element_theory(self) -> str:
        """
        Description of the Mean Element Theory.
        """
        ...

    @mean_element_theory.setter
    def mean_element_theory(self, value: str) -> None: ...
    @property
    def object_id(self) -> str:
        """
        Object identifier.
        """
        ...

    @object_id.setter
    def object_id(self, value: str) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Spacecraft name.
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def ref_frame(self) -> str:
        """
        Reference frame.
        """
        ...

    @ref_frame.setter
    def ref_frame(self, value: str) -> None: ...
    @property
    def ref_frame_epoch(self) -> Optional[str]:
        """
        Epoch of the reference frame.
        """
        ...

    @ref_frame_epoch.setter
    def ref_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system.
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...

class OmmSegment:
    """
    Create a new OMM Segment.

    Parameters
    ----------
    metadata : OmmMetadata
        Segment metadata.
    data : OmmData
        Segment data.
    """
    def __init__(metadata, data) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def data(self) -> OmmData:
        """
        Segment data.
        """
        ...

    @data.setter
    def data(self, value: OmmData) -> None: ...
    @property
    def metadata(self) -> OmmMetadata:
        """
        Segment metadata.
        """
        ...

    @metadata.setter
    def metadata(self, value: OmmMetadata) -> None: ...

class Opm:
    """
    Represents a CCSDS Orbit Parameter Message (OPM).


    Parameters
    ----------
    header : OdmHeader
        The message header.
    segment : OpmSegment
        The data segment.

    Parameters
    ----------
    data : str
        Input string/content.
    format : str, optional
        Format ('kvn' or 'xml'). Auto-detected if None.

    Returns
    -------
    Opm
        The parsed OPM object.
    """
    def __init__(header, segment) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @staticmethod
    def from_file(path, format):
        """
        Create an OPM message from a file.

        Parameters
        ----------
        path : str
            Path to the input file.
        format : str, optional
            Format ('kvn' or 'xml'). Auto-detected if None.

        Returns
        -------
        Opm
            The parsed OPM object.
        """
        ...

    @staticmethod
    def from_str(data, format):
        """
        Create an OPM message from a string.
        """
        ...

    @property
    def header(self) -> OdmHeader:
        """
        The message header.
        """
        ...

    @header.setter
    def header(self, value: OdmHeader) -> None: ...
    @property
    def segment(self) -> OpmSegment:
        """
        The data segment.
        """
        ...

    @segment.setter
    def segment(self, value: OpmSegment) -> None: ...
    def to_file(self, path, format):
        """
        Write to file.

        Parameters
        ----------
        path : str
            Output file path.
        format : str
            Output format ('kvn' or 'xml').
        """
        ...

    def to_str(self, format):
        """
        Serialize to string.

        Parameters
        ----------
        format : str
            Output format ('kvn' or 'xml').

        Returns
        -------
        str
            The serialized string.
        """
        ...

class OpmCovarianceMatrix:
    def __init__(
        cx_x,
        cy_x,
        cy_y,
        cz_x,
        cz_y,
        cz_z,
        cx_dot_x,
        cx_dot_y,
        cx_dot_z,
        cy_dot_x,
        cy_dot_y,
        cy_dot_z,
        cz_dot_x,
        cz_dot_y,
        cz_dot_z,
        cx_dot_x_dot,
        cy_dot_x_dot,
        cy_dot_y_dot,
        cz_dot_x_dot,
        cz_dot_y_dot,
        cz_dot_z_dot,
        cov_ref_frame,
        comments,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comments(self) -> list[str]:
        """
        Comments associated with this covariance matrix.
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    @property
    def cov_ref_frame(self) -> Optional[str]:
        """
        Reference frame for the covariance matrix.

        If omitted, the covariance is expressed in the same reference frame
        as the state vector (REF_FRAME from metadata).
        """
        ...

    @cov_ref_frame.setter
    def cov_ref_frame(self, value: Optional[str]) -> None: ...
    @property
    def cx_dot_x(self) -> float:
        """
        Position-Velocity cross-covariance [4,1] element (X_DOT-X covariance).

        Units: km·km/s
        """
        ...

    @cx_dot_x.setter
    def cx_dot_x(self, value: float) -> None: ...
    @property
    def cx_dot_x_dot(self) -> float:
        """
        Velocity covariance [4,4] element (X_DOT-X_DOT variance).

        Units: km²/s²
        """
        ...

    @cx_dot_x_dot.setter
    def cx_dot_x_dot(self, value: float) -> None: ...
    @property
    def cx_dot_y(self) -> float:
        """
        Position-Velocity cross-covariance [4,2] element (X_DOT-Y covariance).

        Units: km·km/s
        """
        ...

    @cx_dot_y.setter
    def cx_dot_y(self, value: float) -> None: ...
    @property
    def cx_dot_z(self) -> float:
        """
        Position-Velocity cross-covariance [4,3] element (X_DOT-Z covariance).

        Units: km·km/s
        """
        ...

    @cx_dot_z.setter
    def cx_dot_z(self, value: float) -> None: ...
    @property
    def cx_x(self) -> float:
        """
        Position covariance [1,1] element (X-X variance).

        Units: km²
        """
        ...

    @cx_x.setter
    def cx_x(self, value: float) -> None: ...
    @property
    def cy_dot_x(self) -> float:
        """
        Position-Velocity cross-covariance [5,1] element (Y_DOT-X covariance).

        Units: km·km/s
        """
        ...

    @cy_dot_x.setter
    def cy_dot_x(self, value: float) -> None: ...
    @property
    def cy_dot_x_dot(self) -> float:
        """
        Velocity covariance [5,4] element (Y_DOT-X_DOT covariance).

        Units: km²/s²
        """
        ...

    @cy_dot_x_dot.setter
    def cy_dot_x_dot(self, value: float) -> None: ...
    @property
    def cy_dot_y(self) -> float:
        """
        Position-Velocity cross-covariance [5,2] element (Y_DOT-Y covariance).

        Units: km·km/s
        """
        ...

    @cy_dot_y.setter
    def cy_dot_y(self, value: float) -> None: ...
    @property
    def cy_dot_y_dot(self) -> float:
        """
        Velocity covariance [5,5] element (Y_DOT-Y_DOT variance).

        Units: km²/s²
        """
        ...

    @cy_dot_y_dot.setter
    def cy_dot_y_dot(self, value: float) -> None: ...
    @property
    def cy_dot_z(self) -> float:
        """
        Position-Velocity cross-covariance [5,3] element (Y_DOT-Z covariance).

        Units: km·km/s
        """
        ...

    @cy_dot_z.setter
    def cy_dot_z(self, value: float) -> None: ...
    @property
    def cy_x(self) -> float:
        """
        Position covariance [2,1] element (Y-X covariance).

        Units: km²
        """
        ...

    @cy_x.setter
    def cy_x(self, value: float) -> None: ...
    @property
    def cy_y(self) -> float:
        """
        Position covariance [2,2] element (Y-Y variance).

        Units: km²
        """
        ...

    @cy_y.setter
    def cy_y(self, value: float) -> None: ...
    @property
    def cz_dot_x(self) -> float:
        """
        Position-Velocity cross-covariance [6,1] element (Z_DOT-X covariance).

        Units: km·km/s
        """
        ...

    @cz_dot_x.setter
    def cz_dot_x(self, value: float) -> None: ...
    @property
    def cz_dot_x_dot(self) -> float:
        """
        Velocity covariance [6,4] element (Z_DOT-X_DOT covariance).

        Units: km²/s²
        """
        ...

    @cz_dot_x_dot.setter
    def cz_dot_x_dot(self, value: float) -> None: ...
    @property
    def cz_dot_y(self) -> float:
        """
        Position-Velocity cross-covariance [6,2] element (Z_DOT-Y covariance).

        Units: km·km/s
        """
        ...

    @cz_dot_y.setter
    def cz_dot_y(self, value: float) -> None: ...
    @property
    def cz_dot_y_dot(self) -> float:
        """
        Velocity covariance [6,5] element (Z_DOT-Y_DOT covariance).

        Units: km²/s²
        """
        ...

    @cz_dot_y_dot.setter
    def cz_dot_y_dot(self, value: float) -> None: ...
    @property
    def cz_dot_z(self) -> float:
        """
        Position-Velocity cross-covariance [6,3] element (Z_DOT-Z covariance).

        Units: km·km/s
        """
        ...

    @cz_dot_z.setter
    def cz_dot_z(self, value: float) -> None: ...
    @property
    def cz_dot_z_dot(self) -> float:
        """
        Velocity covariance [6,6] element (Z_DOT-Z_DOT variance).

        Units: km²/s²
        """
        ...

    @cz_dot_z_dot.setter
    def cz_dot_z_dot(self, value: float) -> None: ...
    @property
    def cz_x(self) -> float:
        """
        Position covariance [3,1] element (Z-X covariance).

        Units: km²
        """
        ...

    @cz_x.setter
    def cz_x(self, value: float) -> None: ...
    @property
    def cz_y(self) -> float:
        """
        Position covariance [3,2] element (Z-Y covariance).

        Units: km²
        """
        ...

    @cz_y.setter
    def cz_y(self, value: float) -> None: ...
    @property
    def cz_z(self) -> float:
        """
        Position covariance [3,3] element (Z-Z variance).

        Units: km²
        """
        ...

    @cz_z.setter
    def cz_z(self, value: float) -> None: ...

class OpmData:
    """
    Create a new OPM Data object.

    Parameters
    ----------
    state_vector : StateVector
        State vector.
    """
    def __init__(state_vector) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def covariance_matrix(self) -> Optional[OpmCovarianceMatrix]:
        """
        Covariance matrix.
        """
        ...

    @covariance_matrix.setter
    def covariance_matrix(self, value: Optional[OpmCovarianceMatrix]) -> None: ...
    @property
    def keplerian_elements(self) -> Optional[KeplerianElements]:
        """
        Keplerian elements.
        """
        ...

    @keplerian_elements.setter
    def keplerian_elements(self, value: Optional[KeplerianElements]) -> None: ...
    @property
    def maneuver_parameters(self) -> list[ManeuverParameters]:
        """
        Maneuver parameters.
        """
        ...

    @maneuver_parameters.setter
    def maneuver_parameters(self, value: list[ManeuverParameters]) -> None: ...
    @property
    def spacecraft_parameters(self) -> Optional[SpacecraftParameters]:
        """
        Spacecraft parameters.
        """
        ...

    @spacecraft_parameters.setter
    def spacecraft_parameters(self, value: Optional[SpacecraftParameters]) -> None: ...
    @property
    def state_vector(self) -> StateVector:
        """
        State vector.
        """
        ...

    @state_vector.setter
    def state_vector(self, value: StateVector) -> None: ...
    @property
    def user_defined_parameters(self) -> Optional[UserDefined]:
        """
        User defined parameters.
        """
        ...

    @user_defined_parameters.setter
    def user_defined_parameters(self, value: Optional[UserDefined]) -> None: ...

class OpmMetadata:
    """
    Create a new OPM Metadata object.

    Parameters
    ----------
    object_name : str
        Spacecraft name for which orbit state data is provided.
    object_id : str
        Object identifier of the object for which orbit state data is provided.
    center_name : str
        Origin of the reference frame.
    ref_frame : str
        Reference frame in which state vector data is given.
        Time system used for state vector, maneuver, and covariance data.
        Allowed values: 'GMST', 'GPS', 'MET', 'MRT', 'SCLK', 'TAI', 'TCB', 'TDB', 'TCG', 'TT', 'UT1', 'UTC'.
    ref_frame_epoch : str, optional
        Epoch of the reference frame, if not intrinsic to the definition.
    comment : list of str, optional
        Comments.
    """
    def __init__(
        object_name,
        object_id,
        center_name,
        ref_frame,
        time_system,
        ref_frame_epoch,
        comment,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def center_name(self) -> str:
        """
        Origin of the OPM reference frame.

        Shall be a natural solar system body (planets, asteroids, comets, and natural satellites), including any planet barycenter or the solar system barycenter.
        """
        ...

    @center_name.setter
    def center_name(self, value: str) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def object_id(self) -> str:
        """
        Object identifier of the object for which orbit state data is provided.

        Recommended values have the format YYYY-NNNP{PP}.
        """
        ...

    @object_id.setter
    def object_id(self, value: str) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Spacecraft name for which orbit state data is provided.

        While there is no CCSDS-based restriction on the value for this keyword, it is recommended to use names from the UN Office of Outer Space Affairs designator index.
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def ref_frame(self) -> str:
        """
        Reference frame in which the state vector and optional Keplerian element data are given.

        Examples: 'ICRF', 'EME2000', 'ITRF2000', 'TEME'.
        """
        ...

    @ref_frame.setter
    def ref_frame(self, value: str) -> None: ...
    @property
    def ref_frame_epoch(self) -> Optional[str]:
        """
        Epoch of the reference frame, if not intrinsic to the definition.
        """
        ...

    @ref_frame_epoch.setter
    def ref_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system used for state vector, maneuver, and covariance data.
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...

class OpmSegment:
    """
    Represents a single segment of an OPM.
    """
    def __init__(metadata, data) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def data(self) -> OpmData:
        """
        Segment data.
        """
        ...

    @data.setter
    def data(self, value: OpmData) -> None: ...
    @property
    def metadata(self) -> OpmMetadata:
        """
        Segment metadata.
        """
        ...

    @metadata.setter
    def metadata(self, value: OpmMetadata) -> None: ...

class Rdm:
    """
    Represents a CCSDS Re-entry Data Message (RDM).

    The RDM specifies a standard message format to be used in the exchange of
    spacecraft re-entry information between Space Situational Awareness (SSA)
    or Space Surveillance and Tracking (SST) data providers, satellite
    owners/operators, and other parties.

    Parameters
    ----------
    header : RdmHeader
        The message header.
        (Mandatory)
    segment : RdmSegment
        The message segment containing metadata and data.
        (Mandatory)
    """
    def __init__(*, header, segment) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @staticmethod
    def from_file(path, format=None):
        """
        Create an RDM message from a file.

        Parameters
        ----------
        path : str
            Path to the input file.
        format : str, optional
            Format ('kvn' or 'xml'). Auto-detected if None.
            (Optional)

        Returns
        -------
        Rdm
            The parsed RDM object.
        """
        ...

    @staticmethod
    def from_str(data, format=None):
        """
        Create an RDM message from a string.

        Parameters
        ----------
        data : str
            Input string/content.
        format : str, optional
            Format ('kvn' or 'xml'). Auto-detected if None.
            (Optional)

        Returns
        -------
        Rdm
            The parsed RDM object.
        """
        ...

    @property
    def header(self) -> RdmHeader:
        """
        The message header.
        """
        ...

    @header.setter
    def header(self, value: RdmHeader) -> None: ...
    @property
    def segment(self) -> RdmSegment:
        """
        The message segment.
        """
        ...

    @segment.setter
    def segment(self, value: RdmSegment) -> None: ...
    def to_file(self, path, format):
        """
        Write to a file.

        Parameters
        ----------
        path : str
            Output file path.
        format : str
            Format ('kvn' or 'xml').
        """
        ...

    def to_kvn(self):
        """
        Serialize to KVN string.

        Returns
        -------
        str
            The serialized KVN string.
        """
        ...

    def to_str(self, format):
        """
        Serialize to string (generic).

        Parameters
        ----------
        format : str
            Format ('kvn' or 'xml').

        Returns
        -------
        str
            The serialized string.
        """
        ...

    def to_xml(self):
        """
        Serialize to XML string.

        Returns
        -------
        str
            The serialized XML string.
        """
        ...

class RdmCovarianceMatrix:
    """
    Position/velocity covariance matrix.
    """
    def __init__(cov_ref_frame, comment) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments for the covariance matrix.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def comments(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    @property
    def cov_ref_frame(self) -> Optional[str]:
        """
        Covariance reference frame.
        """
        ...

    @cov_ref_frame.setter
    def cov_ref_frame(self, value: Optional[str]) -> None: ...
    @property
    def cx_dot_x(self) -> float:
        """
        CX_DOT_X covariance element.
        """
        ...

    @cx_dot_x.setter
    def cx_dot_x(self, value: float) -> None: ...
    @property
    def cx_dot_x_dot(self) -> float:
        """
        CX_DOT_X_DOT covariance element.
        """
        ...

    @cx_dot_x_dot.setter
    def cx_dot_x_dot(self, value: float) -> None: ...
    @property
    def cx_dot_y(self) -> float:
        """
        CX_DOT_Y covariance element.
        """
        ...

    @cx_dot_y.setter
    def cx_dot_y(self, value: float) -> None: ...
    @property
    def cx_dot_z(self) -> float:
        """
        CX_DOT_Z covariance element.
        """
        ...

    @cx_dot_z.setter
    def cx_dot_z(self, value: float) -> None: ...
    @property
    def cx_x(self) -> float:
        """
        CX_X covariance element.
        """
        ...

    @cx_x.setter
    def cx_x(self, value: float) -> None: ...
    @property
    def cy_dot_x(self) -> float:
        """
        CY_DOT_X covariance element.
        """
        ...

    @cy_dot_x.setter
    def cy_dot_x(self, value: float) -> None: ...
    @property
    def cy_dot_x_dot(self) -> float:
        """
        CY_DOT_X_DOT covariance element.
        """
        ...

    @cy_dot_x_dot.setter
    def cy_dot_x_dot(self, value: float) -> None: ...
    @property
    def cy_dot_y(self) -> float:
        """
        CY_DOT_Y covariance element.
        """
        ...

    @cy_dot_y.setter
    def cy_dot_y(self, value: float) -> None: ...
    @property
    def cy_dot_y_dot(self) -> float:
        """
        CY_DOT_Y_DOT covariance element.
        """
        ...

    @cy_dot_y_dot.setter
    def cy_dot_y_dot(self, value: float) -> None: ...
    @property
    def cy_dot_z(self) -> float:
        """
        CY_DOT_Z covariance element.
        """
        ...

    @cy_dot_z.setter
    def cy_dot_z(self, value: float) -> None: ...
    @property
    def cy_x(self) -> float:
        """
        CY_X covariance element.
        """
        ...

    @cy_x.setter
    def cy_x(self, value: float) -> None: ...
    @property
    def cy_y(self) -> float:
        """
        CY_Y covariance element.
        """
        ...

    @cy_y.setter
    def cy_y(self, value: float) -> None: ...
    @property
    def cz_dot_x(self) -> float:
        """
        CZ_DOT_X covariance element.
        """
        ...

    @cz_dot_x.setter
    def cz_dot_x(self, value: float) -> None: ...
    @property
    def cz_dot_x_dot(self) -> float:
        """
        CZ_DOT_X_DOT covariance element.
        """
        ...

    @cz_dot_x_dot.setter
    def cz_dot_x_dot(self, value: float) -> None: ...
    @property
    def cz_dot_y(self) -> float:
        """
        CZ_DOT_Y covariance element.
        """
        ...

    @cz_dot_y.setter
    def cz_dot_y(self, value: float) -> None: ...
    @property
    def cz_dot_y_dot(self) -> float:
        """
        CZ_DOT_Y_DOT covariance element.
        """
        ...

    @cz_dot_y_dot.setter
    def cz_dot_y_dot(self, value: float) -> None: ...
    @property
    def cz_dot_z(self) -> float:
        """
        CZ_DOT_Z covariance element.
        """
        ...

    @cz_dot_z.setter
    def cz_dot_z(self, value: float) -> None: ...
    @property
    def cz_dot_z_dot(self) -> float:
        """
        CZ_DOT_Z_DOT covariance element.
        """
        ...

    @cz_dot_z_dot.setter
    def cz_dot_z_dot(self, value: float) -> None: ...
    @property
    def cz_x(self) -> float:
        """
        CZ_X covariance element.
        """
        ...

    @cz_x.setter
    def cz_x(self, value: float) -> None: ...
    @property
    def cz_y(self) -> float:
        """
        CZ_Y covariance element.
        """
        ...

    @cz_y.setter
    def cz_y(self, value: float) -> None: ...
    @property
    def cz_z(self) -> float:
        """
        CZ_Z covariance element.
        """
        ...

    @cz_z.setter
    def cz_z(self, value: float) -> None: ...

class RdmData:
    """
    Represents the Data Section of an RDM Segment.

    Contains logical blocks for atmospheric re-entry, ground impact, state vector,
    covariance, object parameters, OD parameters, and user-defined parameters.

    Parameters
    ----------
    atmospheric_reentry_parameters : AtmosphericReentryParameters
        Mandatory atmospheric re-entry data.
    ground_impact_parameters : GroundImpactParameters, optional
        Ground impact and burn-up data.
    state_vector : RdmStateVector, optional
        Spacecraft state vector.
    covariance_matrix : RdmCovarianceMatrix, optional
        Position/velocity covariance matrix.
    spacecraft_parameters : RdmSpacecraftParameters, optional
        Object physical parameters.
    od_parameters : OdParameters, optional
        Orbit determination parameters.
    user_defined_parameters : list[tuple[str, str]], optional
        User defined parameters as key-value pairs.
    comment : list[str], optional
        Comments.
    """
    def __init__(
        *,
        atmospheric_reentry_parameters,
        ground_impact_parameters=None,
        state_vector=None,
        covariance_matrix=None,
        spacecraft_parameters=None,
        od_parameters=None,
        user_defined_parameters=None,
        comment=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def atmospheric_reentry_parameters(self) -> AtmosphericReentryParameters:
        """
        Atmospheric re-entry parameters.
        """
        ...

    @atmospheric_reentry_parameters.setter
    def atmospheric_reentry_parameters(
        self, value: AtmosphericReentryParameters
    ) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def covariance_matrix(self) -> Optional[RdmCovarianceMatrix]:
        """
        Covariance matrix.
        """
        ...

    @covariance_matrix.setter
    def covariance_matrix(self, value: Optional[RdmCovarianceMatrix]) -> None: ...
    @property
    def ground_impact_parameters(self) -> Optional[GroundImpactParameters]:
        """
        Ground impact parameters.
        """
        ...

    @ground_impact_parameters.setter
    def ground_impact_parameters(
        self, value: Optional[GroundImpactParameters]
    ) -> None: ...
    @property
    def od_parameters(self) -> Optional[OdParameters]:
        """
        Orbit determination parameters.
        """
        ...

    @od_parameters.setter
    def od_parameters(self, value: Optional[OdParameters]) -> None: ...
    @property
    def spacecraft_parameters(self) -> Optional[RdmSpacecraftParameters]:
        """
        Spacecraft parameters.
        """
        ...

    @spacecraft_parameters.setter
    def spacecraft_parameters(
        self, value: Optional[RdmSpacecraftParameters]
    ) -> None: ...
    @property
    def state_vector(self) -> Optional[RdmStateVector]:
        """
        State vector.
        """
        ...

    @state_vector.setter
    def state_vector(self, value: Optional[RdmStateVector]) -> None: ...
    @property
    def user_defined_parameters(self) -> list[tuple[str, str]]:
        """
        User defined parameters.
        """
        ...

    @user_defined_parameters.setter
    def user_defined_parameters(self, value: list[tuple[str, str]]) -> None: ...

class RdmHeader:
    """
    Represents the Header of a Re-entry Data Message.

    Parameters
    ----------
    originator : str
        Creating agency or entity.
        (Mandatory)
    creation_date : str
        File creation date and time in UTC.
        (Mandatory)
    message_id : str
        ID that uniquely identifies a message from a given originator.
        (Mandatory)
    comment : list[str], optional
        Comments.
        (Optional)
    """
    def __init__(*, originator, creation_date, message_id, comment=None) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def creation_date(self) -> str:
        """
        File creation date and time in UTC.

        Format: ISO 8601.
        """
        ...

    @creation_date.setter
    def creation_date(self, value: str) -> None: ...
    @property
    def message_id(self) -> str:
        """
        ID that uniquely identifies a message from a given originator.
        """
        ...

    @message_id.setter
    def message_id(self, value: str) -> None: ...
    @property
    def originator(self) -> str:
        """
        Creating agency or entity.

        Value should be an entry from the SANA Organizations Registry.
        """
        ...

    @originator.setter
    def originator(self, value: str) -> None: ...

class RdmMetadata:
    """
    Represents the Metadata Section of an RDM Segment.

    Contains configuration details and object identification for the re-entry data.

    Mandatory Parameters
    --------------------
    object_name : str
        Object name for which the orbit state is provided.
    international_designator : str
        The full international designator (COSPAR ID) for the object.
    controlled_reentry : str
        Specification of whether the re-entry is controlled or not (YES, NO, UNKNOWN).
    center_name : str
        Celestial body orbited by the object.
    time_system : str
        Time system for all data/metadata (e.g., UTC, TAI).
    epoch_tzero : str
        Epoch from which the ORBIT_LIFETIME is calculated.

    Optional Parameters
    -------------------
    See the CCSDS RDM Blue Book for details on optional parameters.
    """
    def __init__(
        *,
        object_name,
        international_designator,
        controlled_reentry,
        center_name,
        time_system,
        epoch_tzero,
        catalog_name=None,
        object_designator=None,
        object_type=None,
        object_owner=None,
        object_operator=None,
        ref_frame=None,
        ref_frame_epoch=None,
        ephemeris_name=None,
        gravity_model=None,
        atmospheric_model=None,
        solar_flux_prediction=None,
        n_body_perturbations=None,
        solar_rad_pressure=None,
        earth_tides=None,
        intrack_thrust=None,
        drag_parameters_source=None,
        drag_parameters_altitude=None,
        reentry_uncertainty_method=None,
        reentry_disintegration=None,
        impact_uncertainty_method=None,
        previous_message_id=None,
        previous_message_epoch=None,
        next_message_epoch=None,
        comment=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def atmospheric_model(self) -> Optional[str]:
        """
        Atmospheric model.
        """
        ...

    @atmospheric_model.setter
    def atmospheric_model(self, value: Optional[str]) -> None: ...
    @property
    def catalog_name(self) -> Optional[str]:
        """
        Catalog name.
        """
        ...

    @catalog_name.setter
    def catalog_name(self, value: Optional[str]) -> None: ...
    @property
    def center_name(self) -> str:
        """
        Center name (e.g., EARTH).
        """
        ...

    @center_name.setter
    def center_name(self, value: str) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def controlled_reentry(self) -> str:
        """
        Controlled re-entry status.
        """
        ...

    @controlled_reentry.setter
    def controlled_reentry(self, value: str) -> None: ...
    @property
    def drag_parameters_altitude(self) -> Optional[float]:
        """
        Drag parameters altitude (km).
        """
        ...

    @drag_parameters_altitude.setter
    def drag_parameters_altitude(self, value: Optional[float]) -> None: ...
    @property
    def drag_parameters_source(self) -> Optional[str]:
        """
        Drag parameters source.
        """
        ...

    @drag_parameters_source.setter
    def drag_parameters_source(self, value: Optional[str]) -> None: ...
    @property
    def earth_tides(self) -> Optional[str]:
        """
        Earth tides model.
        """
        ...

    @earth_tides.setter
    def earth_tides(self, value: Optional[str]) -> None: ...
    @property
    def ephemeris_name(self) -> Optional[str]:
        """
        Ephemeris name.
        """
        ...

    @ephemeris_name.setter
    def ephemeris_name(self, value: Optional[str]) -> None: ...
    @property
    def epoch_tzero(self) -> str:
        """
        Epoch from which the orbit lifetime is calculated.
        """
        ...

    @epoch_tzero.setter
    def epoch_tzero(self, value: str) -> None: ...
    @property
    def gravity_model(self) -> Optional[str]:
        """
        Gravity model.
        """
        ...

    @gravity_model.setter
    def gravity_model(self, value: Optional[str]) -> None: ...
    @property
    def impact_uncertainty_method(self) -> Optional[str]:
        """
        Impact uncertainty method.
        """
        ...

    @impact_uncertainty_method.setter
    def impact_uncertainty_method(self, value: Optional[str]) -> None: ...
    @property
    def international_designator(self) -> str:
        """
        International designator (COSPAR ID).
        """
        ...

    @international_designator.setter
    def international_designator(self, value: str) -> None: ...
    @property
    def intrack_thrust(self) -> Optional[str]:
        """
        In-track thrust status.
        """
        ...

    @intrack_thrust.setter
    def intrack_thrust(self, value: Optional[str]) -> None: ...
    @property
    def n_body_perturbations(self) -> Optional[str]:
        """
        N-body perturbations.
        """
        ...

    @n_body_perturbations.setter
    def n_body_perturbations(self, value: Optional[str]) -> None: ...
    @property
    def next_message_epoch(self) -> Optional[str]:
        """
        Next message epoch.
        """
        ...

    @next_message_epoch.setter
    def next_message_epoch(self, value: Optional[str]) -> None: ...
    @property
    def object_designator(self) -> Optional[str]:
        """
        Object designator.
        """
        ...

    @object_designator.setter
    def object_designator(self, value: Optional[str]) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Object name.
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def object_operator(self) -> Optional[str]:
        """
        Object operator.
        """
        ...

    @object_operator.setter
    def object_operator(self, value: Optional[str]) -> None: ...
    @property
    def object_owner(self) -> Optional[str]:
        """
        Object owner.
        """
        ...

    @object_owner.setter
    def object_owner(self, value: Optional[str]) -> None: ...
    @property
    def object_type(self) -> Optional[str]:
        """
        Object type.
        """
        ...

    @object_type.setter
    def object_type(self, value: Optional[str]) -> None: ...
    @property
    def previous_message_epoch(self) -> Optional[str]:
        """
        Previous message epoch.
        """
        ...

    @previous_message_epoch.setter
    def previous_message_epoch(self, value: Optional[str]) -> None: ...
    @property
    def previous_message_id(self) -> Optional[str]:
        """
        Previous message ID.
        """
        ...

    @previous_message_id.setter
    def previous_message_id(self, value: Optional[str]) -> None: ...
    @property
    def reentry_disintegration(self) -> Optional[str]:
        """
        Reentry disintegration.
        """
        ...

    @reentry_disintegration.setter
    def reentry_disintegration(self, value: Optional[str]) -> None: ...
    @property
    def reentry_uncertainty_method(self) -> Optional[str]:
        """
        Reentry uncertainty method.
        """
        ...

    @reentry_uncertainty_method.setter
    def reentry_uncertainty_method(self, value: Optional[str]) -> None: ...
    @property
    def ref_frame(self) -> Optional[str]:
        """
        Reference frame.
        """
        ...

    @ref_frame.setter
    def ref_frame(self, value: Optional[str]) -> None: ...
    @property
    def ref_frame_epoch(self) -> Optional[str]:
        """
        Reference frame epoch.
        """
        ...

    @ref_frame_epoch.setter
    def ref_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def solar_flux_prediction(self) -> Optional[str]:
        """
        Solar flux prediction methodology.
        """
        ...

    @solar_flux_prediction.setter
    def solar_flux_prediction(self, value: Optional[str]) -> None: ...
    @property
    def solar_rad_pressure(self) -> Optional[str]:
        """
        Solar radiation pressure model.
        """
        ...

    @solar_rad_pressure.setter
    def solar_rad_pressure(self, value: Optional[str]) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system.
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...

class RdmSegment:
    """
    Represents a single segment of an RDM.

    An RDM segment consists of a Metadata Section and a Data Section.

    Parameters
    ----------
    metadata : RdmMetadata
        Segment metadata.
        (Mandatory)
    data : RdmData
        Segment data.
        (Mandatory)
    """
    def __init__(*, metadata, data) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def data(self) -> RdmData:
        """
        Segment data.
        """
        ...

    @data.setter
    def data(self, value: RdmData) -> None: ...
    @property
    def metadata(self) -> RdmMetadata:
        """
        Segment metadata.
        """
        ...

    @metadata.setter
    def metadata(self, value: RdmMetadata) -> None: ...

class RdmSpacecraftParameters:
    """
    Object physical parameters.
    """
    def __init__(*, wet_mass=None, dry_mass=None, comment=None) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def ballistic_coeff(self) -> Optional[float]:
        """
        Ballistic coefficient (kg/m^2).
        """
        ...

    @ballistic_coeff.setter
    def ballistic_coeff(self, value: Optional[float]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def drag_area(self) -> Optional[float]:
        """
        Drag area (m^2).
        """
        ...

    @drag_area.setter
    def drag_area(self, value: Optional[float]) -> None: ...
    @property
    def drag_coeff(self) -> Optional[float]:
        """
        Drag coefficient.
        """
        ...

    @drag_coeff.setter
    def drag_coeff(self, value: Optional[float]) -> None: ...
    @property
    def dry_mass(self) -> Optional[float]:
        """
        Spacecraft dry mass (kg).
        """
        ...

    @dry_mass.setter
    def dry_mass(self, value: Optional[float]) -> None: ...
    @property
    def hazardous_substances(self) -> Optional[str]:
        """
        Quantity of hazardous substances (kg).
        """
        ...

    @hazardous_substances.setter
    def hazardous_substances(self, value: Optional[str]) -> None: ...
    @property
    def rcs(self) -> Optional[float]:
        """
        Radar cross section (m^2).
        """
        ...

    @rcs.setter
    def rcs(self, value: Optional[float]) -> None: ...
    @property
    def solar_rad_area(self) -> Optional[float]:
        """
        Solar radiation pressure area (m^2).
        """
        ...

    @solar_rad_area.setter
    def solar_rad_area(self, value: Optional[float]) -> None: ...
    @property
    def solar_rad_coeff(self) -> Optional[float]:
        """
        Solar radiation pressure coefficient.
        """
        ...

    @solar_rad_coeff.setter
    def solar_rad_coeff(self, value: Optional[float]) -> None: ...
    @property
    def thrust_acceleration(self) -> Optional[float]:
        """
        Constant thrust acceleration (m/s^2).
        """
        ...

    @thrust_acceleration.setter
    def thrust_acceleration(self, value: Optional[float]) -> None: ...
    @property
    def wet_mass(self) -> Optional[float]:
        """
        Spacecraft wet mass (kg).
        """
        ...

    @wet_mass.setter
    def wet_mass(self, value: Optional[float]) -> None: ...

class RdmStateVector:
    """
    Spacecraft State Vector.
    """
    def __init__(*, epoch, x, y, z, x_dot, y_dot, z_dot, comment=None) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comments(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    @property
    def epoch(self) -> str:
        """
        Epoch.
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def x(self) -> float:
        """
        X position (km).
        """
        ...

    @x.setter
    def x(self, value: float) -> None: ...
    @property
    def x_dot(self) -> float:
        """
        X velocity (km/s).
        """
        ...

    @x_dot.setter
    def x_dot(self, value: float) -> None: ...
    @property
    def y(self) -> float:
        """
        Y position (km).
        """
        ...

    @y.setter
    def y(self, value: float) -> None: ...
    @property
    def y_dot(self) -> float:
        """
        Y velocity (km/s).
        """
        ...

    @y_dot.setter
    def y_dot(self, value: float) -> None: ...
    @property
    def z(self) -> float:
        """
        Z position (km).
        """
        ...

    @z.setter
    def z(self, value: float) -> None: ...
    @property
    def z_dot(self) -> float:
        """
        Z velocity (km/s).
        """
        ...

    @z_dot.setter
    def z_dot(self, value: float) -> None: ...

class ReferenceFrameType:
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

class RelativeMetadataData:
    """
    Metadata and data describing relative relationships between Object1 and Object2.

    This section includes Time of Closest Approach (TCA), miss distance,
    relative speed, and screening volume information.

    Parameters
    ----------
    tca : str
        The date and time in UTC of the closest approach (ISO 8601).
    miss_distance : float
        The norm of the relative position vector at TCA. Units: m.
    relative_speed : float, optional
        The norm of the relative velocity vector at TCA. Units: m/s.
    relative_position : list of float, optional
        The [R, T, N] components of Object2's position relative to Object1. Units: m.
    relative_velocity : list of float, optional
        The [R, T, N] components of Object2's velocity relative to Object1. Units: m/s.
    start_screen_period : str, optional
        The start time in UTC of the screening period.
    stop_screen_period : str, optional
        The stop time in UTC of the screening period.
    screen_volume_frame : ScreenVolumeFrameType, optional
        The reference frame for screening volume (RTN or TVN).
    screen_volume_shape : ScreenVolumeShapeType, optional
        The shape of the screening volume (ELLIPSOID or BOX).
    screen_volume_x : float, optional
        The X component size of the screening volume. Units: m.
    screen_volume_y : float, optional
        The Y component size of the screening volume. Units: m.
    screen_volume_z : float, optional
        The Z component size of the screening volume. Units: m.
    screen_entry_time : str, optional
        The time in UTC when Object2 enters the screening volume.
    screen_exit_time : str, optional
        The time in UTC when Object2 exits the screening volume.
    collision_probability : float, optional
        The probability that Object1 and Object2 will collide (0.0 to 1.0).
    collision_probability_method : str, optional
        The method used to calculate the collision probability.
    comment : list of str, optional
        Comments.
    miss_distance_unit : str, optional
        Optional unit string for validation (must be 'm').
    """
    def __init__(
        tca,
        miss_distance,
        relative_speed=None,
        relative_position=None,
        relative_velocity=None,
        start_screen_period=None,
        stop_screen_period=None,
        screen_volume_frame=None,
        screen_volume_shape=None,
        screen_volume_x=None,
        screen_volume_y=None,
        screen_volume_z=None,
        screen_entry_time=None,
        screen_exit_time=None,
        collision_probability=None,
        collision_probability_method=None,
        comment=...,
        miss_distance_unit=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def collision_probability(self) -> Optional[float]:
        """
        The probability (denoted 'p' where 0.0<=p<=1.0) that the two objects will collide.
        """
        ...

    @collision_probability.setter
    def collision_probability(self, value: Optional[float]) -> None: ...
    @property
    def collision_probability_method(self) -> Optional[str]:
        """
        The method used to calculate the collision probability.
        """
        ...

    @collision_probability_method.setter
    def collision_probability_method(self, value: Optional[str]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Explanatory comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def miss_distance(self) -> float:
        """
        The norm of the relative position vector at TCA.
        """
        ...

    @miss_distance.setter
    def miss_distance(self, value: float) -> None: ...
    @property
    def relative_speed(self) -> Optional[float]:
        """
        The norm of the relative velocity vector at TCA.
        """
        ...

    @relative_speed.setter
    def relative_speed(self, value: Optional[float]) -> None: ...
    @property
    def screen_entry_time(self) -> Optional[str]:
        """
        The time in UTC when Object2 enters the screening volume.
        """
        ...

    @screen_entry_time.setter
    def screen_entry_time(self, value: Optional[str]) -> None: ...
    @property
    def screen_exit_time(self) -> Optional[str]:
        """
        The time in UTC when Object2 exits the screening volume.
        """
        ...

    @screen_exit_time.setter
    def screen_exit_time(self, value: Optional[str]) -> None: ...
    @property
    def screen_volume_x(self) -> Optional[float]:
        """
        The R or T component size of the screening volume.
        """
        ...

    @screen_volume_x.setter
    def screen_volume_x(self, value: Optional[float]) -> None: ...
    @property
    def screen_volume_y(self) -> Optional[float]:
        """
        The T or V component size of the screening volume.
        """
        ...

    @screen_volume_y.setter
    def screen_volume_y(self, value: Optional[float]) -> None: ...
    @property
    def screen_volume_z(self) -> Optional[float]:
        """
        The N component size of the screening volume.
        """
        ...

    @screen_volume_z.setter
    def screen_volume_z(self, value: Optional[float]) -> None: ...
    @property
    def start_screen_period(self) -> Optional[str]:
        """
        The start time in UTC of the screening period for the conjunction assessment.
        """
        ...

    @start_screen_period.setter
    def start_screen_period(self, value: Optional[str]) -> None: ...
    @property
    def stop_screen_period(self) -> Optional[str]:
        """
        The stop time in UTC of the screening period for the conjunction assessment.
        """
        ...

    @stop_screen_period.setter
    def stop_screen_period(self, value: Optional[str]) -> None: ...
    @property
    def tca(self) -> str:
        """
        The date and time in UTC of the closest approach.
        """
        ...

    @tca.setter
    def tca(self, value: str) -> None: ...

class ScreenVolumeFrameType:
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

class ScreenVolumeShapeType:
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

class SpacecraftParameters:
    """
    Spacecraft parameters.

    Used in OPM and OMM.

    Parameters
    ----------
    mass : float, optional
        Spacecraft mass (kg).
    solar_rad_area : float, optional
        Solar radiation pressure area (m²).
    solar_rad_coeff : float, optional
        Solar radiation pressure coefficient.
    drag_area : float, optional
        Drag area (m²).
    drag_coeff : float, optional
        Drag coefficient.
    """
    def __init__(
        mass, solar_rad_area, solar_rad_coeff, drag_area, drag_coeff
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comments(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    @property
    def drag_area(self) -> Optional[float]:
        """
        Drag area.

        Units: m²
        """
        ...

    @drag_area.setter
    def drag_area(self, value: Optional[float]) -> None: ...
    @property
    def drag_coeff(self) -> Optional[float]:
        """
        Drag coefficient.
        """
        ...

    @drag_coeff.setter
    def drag_coeff(self, value: Optional[float]) -> None: ...
    @property
    def mass(self) -> Optional[float]:
        """
        Spacecraft mass.

        Units: kg
        """
        ...

    @mass.setter
    def mass(self, value: Optional[float]) -> None: ...
    @property
    def solar_rad_area(self) -> Optional[float]:
        """
        Solar radiation pressure area.

        Units: m²
        """
        ...

    @solar_rad_area.setter
    def solar_rad_area(self, value: Optional[float]) -> None: ...
    @property
    def solar_rad_coeff(self) -> Optional[float]:
        """
        Solar radiation pressure coefficient.
        """
        ...

    @solar_rad_coeff.setter
    def solar_rad_coeff(self, value: Optional[float]) -> None: ...

class StateVector:
    """
    Position and velocity at a specific epoch (without acceleration).

    Simplified version of StateVectorAcc used in OPM and other messages.

    Parameters
    ----------
    epoch : str
        Epoch of the state vector.
    x : float
        Position vector X-component (km).
    y : float
        Position vector Y-component (km).
    z : float
        Position vector Z-component (km).
    x_dot : float
        Velocity vector X-component (km/s).
    y_dot : float
        Velocity vector Y-component (km/s).
    z_dot : float
        Velocity vector Z-component (km/s).
    """
    def __init__(epoch, x, y, z, x_dot, y_dot, z_dot, comments) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comments(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    @property
    def epoch(self) -> str:
        """
        Epoch of the state vector.
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def x(self) -> float:
        """
        Position vector X-component (km).
        """
        ...

    @x.setter
    def x(self, value: float) -> None: ...
    @property
    def x_dot(self) -> float:
        """
        Velocity vector X-component (km/s).
        """
        ...

    @x_dot.setter
    def x_dot(self, value: float) -> None: ...
    @property
    def y(self) -> float:
        """
        Position vector Y-component (km).
        """
        ...

    @y.setter
    def y(self, value: float) -> None: ...
    @property
    def y_dot(self) -> float:
        """
        Velocity vector Y-component (km/s).
        """
        ...

    @y_dot.setter
    def y_dot(self, value: float) -> None: ...
    @property
    def z(self) -> float:
        """
        Position vector Z-component (km).
        """
        ...

    @z.setter
    def z(self, value: float) -> None: ...
    @property
    def z_dot(self) -> float:
        """
        Velocity vector Z-component (km/s).
        """
        ...

    @z_dot.setter
    def z_dot(self, value: float) -> None: ...

class StateVectorAcc:
    """
    Position and velocity (and optionally acceleration) at a specific epoch.

    Used in OEM to represent object state.
    Units: Position in km, Velocity in km/s, Acceleration in km/s² (by default)

    Parameters
    ----------
    epoch : str
        Epoch of the state vector.
    x : float
        Position vector X-component (km).
    y : float
        Position vector Y-component (km).
    z : float
        Position vector Z-component (km).
    x_dot : float
        Velocity vector X-component (km/s).
    y_dot : float
        Velocity vector Y-component (km/s).
    z_dot : float
        Velocity vector Z-component (km/s).
    x_ddot : float, optional
        Acceleration vector X-component (km/s²).
    y_ddot : float, optional
        Acceleration vector Y-component (km/s²).
    z_ddot : float, optional
        Acceleration vector Z-component (km/s²).
    """
    def __init__(
        epoch, x, y, z, x_dot, y_dot, z_dot, x_ddot, y_ddot, z_ddot
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def epoch(self) -> str:
        """
        Epoch of the state vector.
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def x(self) -> float:
        """
        Position vector X-component (km).
        """
        ...

    @x.setter
    def x(self, value: float) -> None: ...
    @property
    def x_ddot(self) -> Optional[float]:
        """
        Acceleration vector X-component (km/s²).
        """
        ...

    @x_ddot.setter
    def x_ddot(self, value: Optional[float]) -> None: ...
    @property
    def x_dot(self) -> float:
        """
        Velocity vector X-component (km/s).
        """
        ...

    @x_dot.setter
    def x_dot(self, value: float) -> None: ...
    @property
    def y(self) -> float:
        """
        Position vector Y-component (km).
        """
        ...

    @y.setter
    def y(self, value: float) -> None: ...
    @property
    def y_ddot(self) -> Optional[float]:
        """
        Acceleration vector Y-component (km/s²).
        """
        ...

    @y_ddot.setter
    def y_ddot(self, value: Optional[float]) -> None: ...
    @property
    def y_dot(self) -> float:
        """
        Velocity vector Y-component (km/s).
        """
        ...

    @y_dot.setter
    def y_dot(self, value: float) -> None: ...
    @property
    def z(self) -> float:
        """
        Position vector Z-component (km).
        """
        ...

    @z.setter
    def z(self, value: float) -> None: ...
    @property
    def z_ddot(self) -> Optional[float]:
        """
        Acceleration vector Z-component (km/s²).
        """
        ...

    @z_ddot.setter
    def z_ddot(self, value: Optional[float]) -> None: ...
    @property
    def z_dot(self) -> float:
        """
        Velocity vector Z-component (km/s).
        """
        ...

    @z_dot.setter
    def z_dot(self, value: float) -> None: ...

class Tdm:
    """
    Represents a CCSDS Tracking Data Message (TDM).

    The TDM is a standard message format for use in exchanging spacecraft tracking data
    between space agencies. Such exchanges are used for distributing tracking data output
    from routine interagency cross-supports in which spacecraft missions managed by one
    agency are tracked from a tracking station managed by a second agency.

    Parameters
    ----------
    header : TdmHeader
        The message header.
        (Mandatory)
    body : TdmBody
        The message body containing segments.
        (Mandatory)
    """
    def __init__(*, header, body) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def body(self) -> TdmBody:
        """
        The message body.
        """
        ...

    @body.setter
    def body(self, value: TdmBody) -> None: ...
    @staticmethod
    def from_file(path, format=None):
        """
        Create a TDM message from a file.

        Parameters
        ----------
        path : str
            Path to the input file.
        format : str, optional
            Format ('kvn' or 'xml'). Auto-detected if None.
            (Optional)

        Returns
        -------
        Tdm
            The parsed TDM object.
        """
        ...

    @staticmethod
    def from_str(data, format=None):
        """
        Create a TDM message from a string.

        Parameters
        ----------
        data : str
            Input string/content.
        format : str, optional
            Format ('kvn' or 'xml'). Auto-detected if None.
            (Optional)

        Returns
        -------
        Tdm
            The parsed TDM object.
        """
        ...

    @property
    def header(self) -> TdmHeader:
        """
        The message header.
        """
        ...

    @header.setter
    def header(self, value: TdmHeader) -> None: ...
    @property
    def segments(self) -> list[TdmSegment]:
        """
        Shortcut to access segments directly from the body.
        """
        ...

    @segments.setter
    def segments(self, value: list[TdmSegment]) -> None: ...
    def to_file(self, path, format):
        """
        Write to file.

        Parameters
        ----------
        path : str
            Output file path.
        format : str
            Output format ('kvn' or 'xml').
        """
        ...

    def to_str(self, format):
        """
        Serialize to string.

        Parameters
        ----------
        format : str
            Output format ('kvn' or 'xml').

        Returns
        -------
        str
            The serialized string.
        """
        ...

class TdmBody:
    """
    Represents the Body of a Tracking Data Message.

    The TDM Body consists of one or more TDM Segments.

    Parameters
    ----------
    segments : list[TdmSegment]
        List of data segments.
    """
    def __init__(*, segments) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def segments(self) -> list[TdmSegment]:
        """
        List of TDM segments.

        Each segment consists of a Metadata Section and a Data Section.
        """
        ...

    @segments.setter
    def segments(self, value: list[TdmSegment]) -> None: ...

class TdmData:
    """
    Represents the Data Section of a TDM Segment.

    Contains one or more Tracking Data Records.

    Parameters
    ----------
    observations : list[TdmObservation], optional
        List of tracking data records.
        (Optional)
    comment : list[str], optional
        Comments in the data section.
        (Optional)
    """
    def __init__(*, observations=None, comment=None) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def observation_count(self) -> int:
        """
        Count of observations.
        """
        ...

    @observation_count.setter
    def observation_count(self, value: int) -> None: ...
    @property
    def observations(self) -> list[TdmObservation]:
        """
        List of tracking observations.
        """
        ...

    @observations.setter
    def observations(self, value: list[TdmObservation]) -> None: ...

class TdmHeader:
    """
    Represents the Header of a Tracking Data Message.

    Parameters
    ----------
    originator : str
        Creating agency. Value should be an entry from the SANA Organizations Registry.
        (Mandatory)
    creation_date : str
        Data creation date/time in UTC.
        (Mandatory)
    message_id : str, optional
        ID that uniquely identifies a message from a given originator.
        (Optional)
    comment : list[str], optional
        Comments.
        (Optional)
    """
    def __init__(
        *, originator, creation_date, message_id=None, comment=None
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments in the Header.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def creation_date(self) -> str:
        """
        Data creation date/time in UTC.

        Format: ISO 8601 (e.g., "2001-11-06T11:17:33" or "2006-001T00:00:00Z")
        """
        ...

    @creation_date.setter
    def creation_date(self, value: str) -> None: ...
    @property
    def message_id(self) -> Optional[str]:
        """
        ID that uniquely identifies a message from a given originator.

        The format and content of the message identifier value are at the discretion of the originator.
        """
        ...

    @message_id.setter
    def message_id(self, value: Optional[str]) -> None: ...
    @property
    def originator(self) -> str:
        """
        Creating agency.

        Value should be an entry from the 'Abbreviation' column in the SANA Organizations Registry.
        Examples: CNES, ESA, GSFC, DLR, JPL, JAXA, etc.
        """
        ...

    @originator.setter
    def originator(self, value: str) -> None: ...

class TdmMetadata:
    """
    Represents the Metadata Section of a TDM Segment.

    Contains configuration details applicable to the Data Section in the same TDM Segment.

    Mandatory Parameters
    --------------------
    time_system : str
        Time system used for timetags (e.g., "UTC", "TAI").
    participant_1 : str
        First participant in the tracking session.

    Optional Parameters
    -------------------
    Many optional parameters are available to describe the tracking configuration,
    signal path, frequencies, and corrections. See CCSDS TDM Blue Book for full details.
    """
    def __init__(
        *,
        time_system,
        participant_1,
        track_id=None,
        data_types=None,
        start_time=None,
        stop_time=None,
        participant_2=None,
        participant_3=None,
        participant_4=None,
        participant_5=None,
        mode=None,
        path=None,
        path_1=None,
        path_2=None,
        transmit_band=None,
        receive_band=None,
        turnaround_numerator=None,
        turnaround_denominator=None,
        timetag_ref=None,
        integration_interval=None,
        integration_ref=None,
        freq_offset=None,
        range_mode=None,
        range_modulus=None,
        range_units=None,
        angle_type=None,
        reference_frame=None,
        interpolation=None,
        interpolation_degree=None,
        doppler_count_bias=None,
        doppler_count_scale=None,
        doppler_count_rollover=None,
        transmit_delay_1=None,
        transmit_delay_2=None,
        transmit_delay_3=None,
        transmit_delay_4=None,
        transmit_delay_5=None,
        receive_delay_1=None,
        receive_delay_2=None,
        receive_delay_3=None,
        receive_delay_4=None,
        receive_delay_5=None,
        data_quality=None,
        correction_angle_1=None,
        correction_angle_2=None,
        correction_doppler=None,
        correction_mag=None,
        correction_range=None,
        correction_rcs=None,
        correction_receive=None,
        correction_transmit=None,
        correction_aberration_yearly=None,
        correction_aberration_diurnal=None,
        corrections_applied=None,
        ephemeris_name_1=None,
        ephemeris_name_2=None,
        ephemeris_name_3=None,
        ephemeris_name_4=None,
        ephemeris_name_5=None,
        comment=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def angle_type(self) -> Optional[str]:
        """
        Type of antenna geometry (e.g., AZEL, RADEC).
        """
        ...

    @angle_type.setter
    def angle_type(self, value: Optional[str]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def correction_aberration_diurnal(self) -> Optional[float]:
        """
        Diurnal aberration correction (deg).
        """
        ...

    @correction_aberration_diurnal.setter
    def correction_aberration_diurnal(self, value: Optional[float]) -> None: ...
    @property
    def correction_aberration_yearly(self) -> Optional[float]:
        """
        Yearly aberration correction (deg).
        """
        ...

    @correction_aberration_yearly.setter
    def correction_aberration_yearly(self, value: Optional[float]) -> None: ...
    @property
    def correction_angle_1(self) -> Optional[float]:
        """
        Angle 1 correction (degrees).
        """
        ...

    @correction_angle_1.setter
    def correction_angle_1(self, value: Optional[float]) -> None: ...
    @property
    def correction_angle_2(self) -> Optional[float]:
        """
        Angle 2 correction (degrees).
        """
        ...

    @correction_angle_2.setter
    def correction_angle_2(self, value: Optional[float]) -> None: ...
    @property
    def correction_doppler(self) -> Optional[float]:
        """
        Doppler correction (km/s).
        """
        ...

    @correction_doppler.setter
    def correction_doppler(self, value: Optional[float]) -> None: ...
    @property
    def correction_mag(self) -> Optional[float]:
        """
        Magnitude correction.
        """
        ...

    @correction_mag.setter
    def correction_mag(self, value: Optional[float]) -> None: ...
    @property
    def correction_range(self) -> Optional[float]:
        """
        Range correction (Range Units, km, or s).
        """
        ...

    @correction_range.setter
    def correction_range(self, value: Optional[float]) -> None: ...
    @property
    def correction_rcs(self) -> Optional[float]:
        """
        RCS correction (m²).
        """
        ...

    @correction_rcs.setter
    def correction_rcs(self, value: Optional[float]) -> None: ...
    @property
    def correction_receive(self) -> Optional[float]:
        """
        Receive correction (Hz).
        """
        ...

    @correction_receive.setter
    def correction_receive(self, value: Optional[float]) -> None: ...
    @property
    def correction_transmit(self) -> Optional[float]:
        """
        Transmit correction (Hz).
        """
        ...

    @correction_transmit.setter
    def correction_transmit(self, value: Optional[float]) -> None: ...
    @property
    def corrections_applied(self) -> Optional[str]:
        """
        Indication if corrections have been applied (YES, NO).
        """
        ...

    @corrections_applied.setter
    def corrections_applied(self, value: Optional[str]) -> None: ...
    @property
    def data_quality(self) -> Optional[str]:
        """
        Data quality (RAW, VALIDATED, DEGRADED).
        """
        ...

    @data_quality.setter
    def data_quality(self, value: Optional[str]) -> None: ...
    @property
    def data_types(self) -> Optional[str]:
        """
        Comma-separated list of data types in the Data Section.
        """
        ...

    @data_types.setter
    def data_types(self, value: Optional[str]) -> None: ...
    @property
    def doppler_count_bias(self) -> Optional[float]:
        """
        Bias for Doppler count (Hz).
        """
        ...

    @doppler_count_bias.setter
    def doppler_count_bias(self, value: Optional[float]) -> None: ...
    @property
    def doppler_count_rollover(self) -> Optional[str]:
        """
        Indicator of Doppler count rollover (YES, NO).
        """
        ...

    @doppler_count_rollover.setter
    def doppler_count_rollover(self, value: Optional[str]) -> None: ...
    @property
    def doppler_count_scale(self) -> Optional[int]:
        """
        Scale factor for Doppler count.
        """
        ...

    @doppler_count_scale.setter
    def doppler_count_scale(self, value: Optional[int]) -> None: ...
    @property
    def ephemeris_name_1(self) -> Optional[str]:
        """
        Ephemeris name for Participant 1.
        """
        ...

    @ephemeris_name_1.setter
    def ephemeris_name_1(self, value: Optional[str]) -> None: ...
    @property
    def ephemeris_name_2(self) -> Optional[str]:
        """
        Ephemeris name for Participant 2.
        """
        ...

    @ephemeris_name_2.setter
    def ephemeris_name_2(self, value: Optional[str]) -> None: ...
    @property
    def ephemeris_name_3(self) -> Optional[str]:
        """
        Ephemeris name for Participant 3.
        """
        ...

    @ephemeris_name_3.setter
    def ephemeris_name_3(self, value: Optional[str]) -> None: ...
    @property
    def ephemeris_name_4(self) -> Optional[str]:
        """
        Ephemeris name for Participant 4.
        """
        ...

    @ephemeris_name_4.setter
    def ephemeris_name_4(self, value: Optional[str]) -> None: ...
    @property
    def ephemeris_name_5(self) -> Optional[str]:
        """
        Ephemeris name for Participant 5.
        """
        ...

    @ephemeris_name_5.setter
    def ephemeris_name_5(self, value: Optional[str]) -> None: ...
    @property
    def freq_offset(self) -> Optional[float]:
        """
        Frequency offset in Hz.
        """
        ...

    @freq_offset.setter
    def freq_offset(self, value: Optional[float]) -> None: ...
    @property
    def integration_interval(self) -> Optional[float]:
        """
        Doppler count time in seconds.
        """
        ...

    @integration_interval.setter
    def integration_interval(self, value: Optional[float]) -> None: ...
    @property
    def integration_ref(self) -> Optional[str]:
        """
        Relationship between INTEGRATION_INTERVAL and timetag (START, MIDDLE, END).
        """
        ...

    @integration_ref.setter
    def integration_ref(self, value: Optional[str]) -> None: ...
    @property
    def interpolation(self) -> Optional[str]:
        """
        Interpolation method (e.g., HERMITE, LAGRANGE).
        """
        ...

    @interpolation.setter
    def interpolation(self, value: Optional[str]) -> None: ...
    @property
    def interpolation_degree(self) -> Optional[int]:
        """
        Degree of interpolation polynomial.
        """
        ...

    @interpolation_degree.setter
    def interpolation_degree(self, value: Optional[int]) -> None: ...
    @property
    def mode(self) -> Optional[str]:
        """
        Tracking mode (e.g., SEQUENTIAL, SINGLE_DIFF).
        """
        ...

    @mode.setter
    def mode(self, value: Optional[str]) -> None: ...
    @property
    def participant_1(self) -> str:
        """
        First participant in the tracking session (Mandatory).
        """
        ...

    @participant_1.setter
    def participant_1(self, value: str) -> None: ...
    @property
    def participant_2(self) -> Optional[str]:
        """
        Second participant.
        """
        ...

    @participant_2.setter
    def participant_2(self, value: Optional[str]) -> None: ...
    @property
    def participant_3(self) -> Optional[str]:
        """
        Third participant.
        """
        ...

    @participant_3.setter
    def participant_3(self, value: Optional[str]) -> None: ...
    @property
    def participant_4(self) -> Optional[str]:
        """
        Fourth participant.
        """
        ...

    @participant_4.setter
    def participant_4(self, value: Optional[str]) -> None: ...
    @property
    def participant_5(self) -> Optional[str]:
        """
        Fifth participant.
        """
        ...

    @participant_5.setter
    def participant_5(self, value: Optional[str]) -> None: ...
    @property
    def path(self) -> Optional[str]:
        """
        Signal path (comma-separated participant indices). Used with SEQUENTIAL mode.
        """
        ...

    @path.setter
    def path(self, value: Optional[str]) -> None: ...
    @property
    def path_1(self) -> Optional[str]:
        """
        First signal path for SINGLE_DIFF mode.
        """
        ...

    @path_1.setter
    def path_1(self, value: Optional[str]) -> None: ...
    @property
    def path_2(self) -> Optional[str]:
        """
        Second signal path for SINGLE_DIFF mode.
        """
        ...

    @path_2.setter
    def path_2(self, value: Optional[str]) -> None: ...
    @property
    def range_mode(self) -> Optional[str]:
        """
        Range mode (COHERENT, CONSTANT, ONE_WAY).
        """
        ...

    @range_mode.setter
    def range_mode(self, value: Optional[str]) -> None: ...
    @property
    def range_modulus(self) -> Optional[float]:
        """
        Modulus of the range observable.
        """
        ...

    @range_modulus.setter
    def range_modulus(self, value: Optional[float]) -> None: ...
    @property
    def range_units(self) -> Optional[str]:
        """
        Units for the range observable (km, s, RU).
        """
        ...

    @range_units.setter
    def range_units(self, value: Optional[str]) -> None: ...
    @property
    def receive_band(self) -> Optional[str]:
        """
        Frequency band for received frequencies.
        """
        ...

    @receive_band.setter
    def receive_band(self, value: Optional[str]) -> None: ...
    @property
    def receive_delay_1(self) -> Optional[float]:
        """
        Receive delay for Participant 1 (seconds).
        """
        ...

    @receive_delay_1.setter
    def receive_delay_1(self, value: Optional[float]) -> None: ...
    @property
    def receive_delay_2(self) -> Optional[float]:
        """
        Receive delay for Participant 2 (seconds).
        """
        ...

    @receive_delay_2.setter
    def receive_delay_2(self, value: Optional[float]) -> None: ...
    @property
    def receive_delay_3(self) -> Optional[float]:
        """
        Receive delay for Participant 3 (seconds).
        """
        ...

    @receive_delay_3.setter
    def receive_delay_3(self, value: Optional[float]) -> None: ...
    @property
    def receive_delay_4(self) -> Optional[float]:
        """
        Receive delay for Participant 4 (seconds).
        """
        ...

    @receive_delay_4.setter
    def receive_delay_4(self, value: Optional[float]) -> None: ...
    @property
    def receive_delay_5(self) -> Optional[float]:
        """
        Receive delay for Participant 5 (seconds).
        """
        ...

    @receive_delay_5.setter
    def receive_delay_5(self, value: Optional[float]) -> None: ...
    @property
    def reference_frame(self) -> Optional[str]:
        """
        Inertial reference frame (e.g., EME2000). Applies if ANGLE_TYPE is RADEC.
        """
        ...

    @reference_frame.setter
    def reference_frame(self, value: Optional[str]) -> None: ...
    @property
    def start_time(self) -> Optional[str]:
        """
        Start time of the tracking data.
        """
        ...

    @start_time.setter
    def start_time(self, value: Optional[str]) -> None: ...
    @property
    def stop_time(self) -> Optional[str]:
        """
        Stop time of the tracking data.
        """
        ...

    @stop_time.setter
    def stop_time(self, value: Optional[str]) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system used for timetags (e.g., UTC, TAI, GPS, SCLK).
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...
    @property
    def timetag_ref(self) -> Optional[str]:
        """
        Reference for time tags (TRANSMIT or RECEIVE).
        """
        ...

    @timetag_ref.setter
    def timetag_ref(self, value: Optional[str]) -> None: ...
    @property
    def track_id(self) -> Optional[str]:
        """
        Unique identifier for the tracking data.
        """
        ...

    @track_id.setter
    def track_id(self, value: Optional[str]) -> None: ...
    @property
    def transmit_band(self) -> Optional[str]:
        """
        Frequency band for transmitted frequencies (e.g., S, X, Ka).
        """
        ...

    @transmit_band.setter
    def transmit_band(self, value: Optional[str]) -> None: ...
    @property
    def transmit_delay_1(self) -> Optional[float]:
        """
        Transmit delay for Participant 1 (seconds).
        """
        ...

    @transmit_delay_1.setter
    def transmit_delay_1(self, value: Optional[float]) -> None: ...
    @property
    def transmit_delay_2(self) -> Optional[float]:
        """
        Transmit delay for Participant 2 (seconds).
        """
        ...

    @transmit_delay_2.setter
    def transmit_delay_2(self, value: Optional[float]) -> None: ...
    @property
    def transmit_delay_3(self) -> Optional[float]:
        """
        Transmit delay for Participant 3 (seconds).
        """
        ...

    @transmit_delay_3.setter
    def transmit_delay_3(self, value: Optional[float]) -> None: ...
    @property
    def transmit_delay_4(self) -> Optional[float]:
        """
        Transmit delay for Participant 4 (seconds).
        """
        ...

    @transmit_delay_4.setter
    def transmit_delay_4(self, value: Optional[float]) -> None: ...
    @property
    def transmit_delay_5(self) -> Optional[float]:
        """
        Transmit delay for Participant 5 (seconds).
        """
        ...

    @transmit_delay_5.setter
    def transmit_delay_5(self, value: Optional[float]) -> None: ...
    @property
    def turnaround_denominator(self) -> Optional[int]:
        """
        Denominator of the turnaround ratio.
        """
        ...

    @turnaround_denominator.setter
    def turnaround_denominator(self, value: Optional[int]) -> None: ...
    @property
    def turnaround_numerator(self) -> Optional[int]:
        """
        Numerator of the turnaround ratio.
        """
        ...

    @turnaround_numerator.setter
    def turnaround_numerator(self, value: Optional[int]) -> None: ...

class TdmObservation:
    """
    Represents a single Tracking Data Record (TDR).

    A TDR consists of a keyword (data type), a timetag (epoch), and a measurement.

    Parameters
    ----------
    epoch : str
        Time associated with the tracking observable.
    keyword : str
        Data type keyword (e.g., "RANGE", "RECEIVE_FREQ").
    value : float
        Tracking observable value. Note: For phase counts that require full precision strings,
        use internal representation handling (this constructor takes float for simplicity,
        but the object can hold string representations internally).
    """
    def __init__(*, epoch, keyword, value) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def epoch(self) -> str:
        """
        Epoch of the observation.
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def keyword(self) -> str:
        """
        Keyword of the observation (e.g., "RANGE").
        """
        ...

    @keyword.setter
    def keyword(self, value: str) -> None: ...
    @property
    def value(self) -> Optional[float]:
        """
        Measurement value as float.

        Returns None if the value is not representable as a float (unlikely for TDM).
        """
        ...

    @value.setter
    def value(self, value: Optional[float]) -> None: ...
    @property
    def value_str(self) -> str:
        """
        Measurement value as string.

        Useful for phase counts which may require high precision.
        """
        ...

    @value_str.setter
    def value_str(self, value: str) -> None: ...

class TdmSegment:
    """
    Represents a single segment of a TDM.

    A segment consists of a Metadata Section (configuration details) and a
    Data Section (tracking data records).

    Parameters
    ----------
    metadata : TdmMetadata
        Segment metadata.
        (Mandatory)
    data : TdmData
        Segment data.
        (Mandatory)
    """
    def __init__(*, metadata, data) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def data(self) -> TdmData:
        """
        Segment data.

        Contains the tracking data records.
        """
        ...

    @data.setter
    def data(self, value: TdmData) -> None: ...
    @property
    def metadata(self) -> TdmMetadata:
        """
        Segment metadata.

        Contains configuration details applicable to the Data Section.
        """
        ...

    @metadata.setter
    def metadata(self, value: TdmMetadata) -> None: ...

class TleParameters:
    """
    Create a new TleParameters object.

    Parameters
    ----------
    ephemeris_type : int, optional
        Ephemeris type.
        (Optional)
    classification_type : str, optional
        Classification type.
        (Optional)
    norad_cat_id : int, optional
        NORAD catalog ID.
        (Optional)
    element_set_no : int, optional
        Element set number.
        (Optional)
    rev_at_epoch : int, optional
        Revolution number at epoch.
        (Optional)
    bstar : float, optional
        B* drag term (1/ER).
        (Optional)
    bterm : float, optional
        Ballistic coefficient (m²/kg).
        (Optional)
    mean_motion_dot : float, optional
        First derivative of mean motion (rev/day²).
        (Optional)
    mean_motion_ddot : float, optional
        Second derivative of mean motion (rev/day³).
        (Optional)
    agom : float, optional
        Solar radiation pressure area to mass ratio (m²/kg).
        (Optional)
    """
    def __init__(
        ephemeris_type: Optional[int] = None,
        classification_type: Optional[str] = None,
        norad_cat_id: Optional[int] = None,
        element_set_no: Optional[int] = None,
        rev_at_epoch: Optional[int] = None,
        bstar: Optional[float] = None,
        bterm: Optional[float] = None,
        mean_motion_dot: Optional[float] = None,
        mean_motion_ddot: Optional[float] = None,
        agom: Optional[float] = None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def agom(self) -> Optional[float]:
        """
        Solar radiation pressure area to mass ratio.

        Units: m²/kg
        """
        ...

    @agom.setter
    def agom(self, value: Optional[float]) -> None: ...
    @property
    def bstar(self) -> Optional[float]:
        """
        B* drag term.

        Units: 1/ER
        """
        ...

    @bstar.setter
    def bstar(self, value: Optional[float]) -> None: ...
    @property
    def bterm(self) -> Optional[float]:
        """
        Ballistic coefficient.

        Units: m²/kg
        """
        ...

    @bterm.setter
    def bterm(self, value: Optional[float]) -> None: ...
    @property
    def classification_type(self) -> Optional[str]:
        """
        Classification type.
        """
        ...

    @classification_type.setter
    def classification_type(self, value: Optional[str]) -> None: ...
    @property
    def comments(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    @property
    def element_set_no(self) -> Optional[int]:
        """
        Element set number.
        """
        ...

    @element_set_no.setter
    def element_set_no(self, value: Optional[int]) -> None: ...
    @property
    def ephemeris_type(self) -> Optional[int]:
        """
        Ephemeris type.
        """
        ...

    @ephemeris_type.setter
    def ephemeris_type(self, value: Optional[int]) -> None: ...
    @property
    def mean_motion_ddot(self) -> Optional[float]:
        """
        Second derivative of mean motion.

        Units: rev/day³
        """
        ...

    @mean_motion_ddot.setter
    def mean_motion_ddot(self, value: Optional[float]) -> None: ...
    @property
    def mean_motion_dot(self) -> Optional[float]:
        """
        First derivative of mean motion.

        Units: rev/day²
        """
        ...

    @mean_motion_dot.setter
    def mean_motion_dot(self, value: Optional[float]) -> None: ...
    @property
    def norad_cat_id(self) -> Optional[int]:
        """
        NORAD catalog ID.
        """
        ...

    @norad_cat_id.setter
    def norad_cat_id(self, value: Optional[int]) -> None: ...
    @property
    def rev_at_epoch(self) -> Optional[int]:
        """
        Revolution number at epoch.
        """
        ...

    @rev_at_epoch.setter
    def rev_at_epoch(self, value: Optional[int]) -> None: ...

class TrajLine:
    """
    A single line in a trajectory state time history.

    Parameters
    ----------
    epoch : str
        Absolute or relative time tag.
        (Mandatory)
    values : list of float
        Trajectory state elements for this epoch.
        (Mandatory)
    """
    def __init__(*, epoch, values) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def epoch(self) -> str:
        """
        Absolute or relative time tag.
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def values(self) -> list[float]:
        """
        Trajectory state elements for this epoch.
        """
        ...

    @values.setter
    def values(self, value: list[float]) -> None: ...

class UserDefined:
    """
    User-Defined Parameters.

    This section contains any user-defined metadata and data that cannot be
    accommodated in other sections.

    Parameters
    ----------
    params : dict[str, str]
        A dictionary of user-defined parameters and their values.
        (Mandatory)
    comment : list[str], optional
        Comments.
        (Optional)
    """
    def __init__() -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments for the user-defined section.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def parameters(self) -> dict[str, str]:
        """
        User-defined parameters.
        """
        ...

    @parameters.setter
    def parameters(self, value: dict[str, str]) -> None: ...
