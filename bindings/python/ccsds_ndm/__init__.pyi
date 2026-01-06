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

class AdditionalParameters:
    """
    Additional Parameters.

    Parameters
    ----------
    area_pc : float, optional
        Projected area. Units: m^2
    area_drg : float, optional
        Drag area. Units: m^2
    area_srp : float, optional
        SRP area. Units: m^2
    mass : float, optional
        Mass. Units: kg
    cd_area_over_mass : float, optional
        Drag coefficient * Area / Mass. Units: m^2/kg
    cr_area_over_mass : float, optional
        Reflectivity coefficient * Area / Mass. Units: m^2/kg
    thrust_acceleration : float, optional
        Thrust acceleration. Units: m/s^2
    sedr : float, optional
        Solar energy dissipation rate. Units: W/kg
    comment : list of str, optional
        Comments.
    """
    def __init__(
        area_pc,
        area_drg,
        area_srp,
        mass,
        cd_area_over_mass,
        cr_area_over_mass,
        thrust_acceleration,
        sedr,
        comment,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def area_drg(self) -> float:
        """
        The effective area of the object exposed to atmospheric drag. (See annex E for
        definition.)

        Units: m²
        """
        ...

    @area_drg.setter
    def area_drg(self, value: float) -> None: ...
    @property
    def area_pc(self) -> float:
        """
        The actual area of the object. (See annex E for definition.)

        Units: m²
        """
        ...

    @area_pc.setter
    def area_pc(self, value: float) -> None: ...
    @property
    def area_srp(self) -> float:
        """
        The effective area of the object exposed to solar radiation pressure. (See annex E for
        definition.)

        Units: m²
        """
        ...

    @area_srp.setter
    def area_srp(self, value: float) -> None: ...
    @property
    def cd_area_over_mass(self) -> float:
        """
        The object's CD•A/m used to propagate the state vector and covariance to TCA. (See
        annex E for definition.)

        Units: m²/kg
        """
        ...

    @cd_area_over_mass.setter
    def cd_area_over_mass(self, value: float) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 6.3.4 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def cr_area_over_mass(self) -> float:
        """
        The object's CR•A/m used to propagate the state vector and covariance to TCA. (See
        annex E for definition.)

        Units: m²/kg
        """
        ...

    @cr_area_over_mass.setter
    def cr_area_over_mass(self, value: float) -> None: ...
    @property
    def mass(self) -> float:
        """
        The mass of the object.

        Units: kg
        """
        ...

    @mass.setter
    def mass(self, value: float) -> None: ...
    @property
    def sedr(self) -> float:
        """
        The amount of energy being removed from the object's orbit by atmospheric drag. This
        value is an average calculated during the OD.

        Units: W/kg
        """
        ...

    @sedr.setter
    def sedr(self, value: float) -> None: ...
    @property
    def thrust_acceleration(self) -> float:
        """
        The object's acceleration due to in-track thrust used to propagate the state vector and
        covariance to TCA. (See annex E for definition.)

        Units: m/s²
        """
        ...

    @thrust_acceleration.setter
    def thrust_acceleration(self, value: float) -> None: ...

class AtmosphericReentryParameters:
    """
    Atmospheric reentry parameters (atmosphericReentryParametersType, RDM).

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
    Covariance Matrix.

    Parameters
    ----------
    cr_r : float
        Radial position variance. Units: m^2
    ct_r : float
        Transverse-Radial position covariance. Units: m^2
    ct_t : float
        Transverse position variance. Units: m^2
    cn_r : float
        Normal-Radial position covariance. Units: m^2
    cn_t : float
        Normal-Transverse position covariance. Units: m^2
    cn_n : float
        Normal position variance. Units: m^2
    crdot_r : float
        Radial velocity - Radial position covariance. Units: m^2/s
    crdot_t : float
        Radial velocity - Transverse position covariance. Units: m^2/s
    crdot_n : float
        Radial velocity - Normal position covariance. Units: m^2/s
    crdot_rdot : float
        Radial velocity variance. Units: m^2/s^2
    ctdot_r : float
        Transverse velocity - Radial position covariance. Units: m^2/s
    ctdot_t : float
        Transverse velocity - Transverse position covariance. Units: m^2/s
    ctdot_n : float
        Transverse velocity - Normal position covariance. Units: m^2/s
    ctdot_rdot : float
        Transverse velocity - Radial velocity covariance. Units: m^2/s^2
    ctdot_tdot : float
        Transverse velocity variance. Units: m^2/s^2
    cndot_r : float
        Normal velocity - Radial position covariance. Units: m^2/s
    cndot_t : float
        Normal velocity - Transverse position covariance. Units: m^2/s
    cndot_n : float
        Normal velocity - Normal position covariance. Units: m^2/s
    cndot_rdot : float
        Normal velocity - Radial velocity covariance. Units: m^2/s^2
    cndot_tdot : float
        Normal velocity - Transverse velocity covariance. Units: m^2/s^2
    cndot_ndot : float
        Normal velocity variance. Units: m^2/s^2
    cdrg_r : float
        Drag coeff - Radial position covariance.
    cdrg_t : float
        Drag coeff - Transverse position covariance.
    cdrg_n : float
        Drag coeff - Normal position covariance.
    cdrg_rdot : float
        Drag coeff - Radial velocity covariance.
    cdrg_tdot : float
        Drag coeff - Transverse velocity covariance.
    cdrg_ndot : float
        Drag coeff - Normal velocity covariance.
    cdrg_drg : float
        Drag coeff variance.
    csrp_r : float
        SRP coeff - Radial position covariance.
    csrp_t : float
        SRP coeff - Transverse position covariance.
    csrp_n : float
        SRP coeff - Normal position covariance.
    csrp_rdot : float
        SRP coeff - Radial velocity covariance.
    csrp_tdot : float
        SRP coeff - Transverse velocity covariance.
    csrp_ndot : float
        SRP coeff - Normal velocity covariance.
    csrp_drg : float
        SRP coeff - Drag coeff covariance.
    csrp_srp : float
        SRP coeff variance.
    cthr_r : float
        Thrust - Radial position covariance.
    cthr_t : float
        Thrust - Transverse position covariance.
    cthr_n : float
        Thrust - Normal position covariance.
    cthr_rdot : float
        Thrust - Radial velocity covariance.
    cthr_tdot : float
        Thrust - Transverse velocity covariance.
    cthr_ndot : float
        Thrust - Normal velocity covariance.
    cthr_drg : float
        Thrust - Drag coeff covariance.
    cthr_srp : float
        Thrust - SRP coeff covariance.
    cthr_thr : float
        Thrust variance.
    comment : list of str, optional
        Comments.
    """
    def __init__(
        cr_r,
        ct_r,
        ct_t,
        cn_r,
        cn_t,
        cn_n,
        crdot_r,
        crdot_t,
        crdot_n,
        crdot_rdot,
        ctdot_r,
        ctdot_t,
        ctdot_n,
        ctdot_rdot,
        ctdot_tdot,
        cndot_r,
        cndot_t,
        cndot_n,
        cndot_rdot,
        cndot_tdot,
        cndot_ndot,
        cdrg_r,
        cdrg_t,
        cdrg_n,
        cdrg_rdot,
        cdrg_tdot,
        cdrg_ndot,
        cdrg_drg,
        csrp_r,
        csrp_t,
        csrp_n,
        csrp_rdot,
        csrp_tdot,
        csrp_ndot,
        csrp_drg,
        csrp_srp,
        cthr_r,
        cthr_t,
        cthr_n,
        cthr_rdot,
        cthr_tdot,
        cthr_ndot,
        cthr_drg,
        cthr_srp,
        cthr_thr,
        comment,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def cdrg_drg(self) -> float:
        """
        Object covariance matrix [7,7].

        Units: m⁴/kg²
        """
        ...

    @cdrg_drg.setter
    def cdrg_drg(self, value: float) -> None: ...
    @property
    def cdrg_n(self) -> float:
        """
        Object covariance matrix [7,3].

        Units: m³/kg
        """
        ...

    @cdrg_n.setter
    def cdrg_n(self, value: float) -> None: ...
    @property
    def cdrg_ndot(self) -> float:
        """
        Object covariance matrix [7,6].

        Units: m³/(kg*s)
        """
        ...

    @cdrg_ndot.setter
    def cdrg_ndot(self, value: float) -> None: ...
    @property
    def cdrg_r(self) -> float:
        """
        Object covariance matrix [7,1].

        Units: m³/kg
        """
        ...

    @cdrg_r.setter
    def cdrg_r(self, value: float) -> None: ...
    @property
    def cdrg_rdot(self) -> float:
        """
        Object covariance matrix [7,4].

        Units: m³/(kg*s)
        """
        ...

    @cdrg_rdot.setter
    def cdrg_rdot(self, value: float) -> None: ...
    @property
    def cdrg_t(self) -> float:
        """
        Object covariance matrix [7,2].

        Units: m³/kg
        """
        ...

    @cdrg_t.setter
    def cdrg_t(self, value: float) -> None: ...
    @property
    def cdrg_tdot(self) -> float:
        """
        Object covariance matrix [7,5].

        Units: m³/(kg*s)
        """
        ...

    @cdrg_tdot.setter
    def cdrg_tdot(self, value: float) -> None: ...
    @property
    def cn_n(self) -> float:
        """
        Object covariance matrix [3,3].

        Units: m²
        """
        ...

    @cn_n.setter
    def cn_n(self, value: float) -> None: ...
    @property
    def cn_r(self) -> float:
        """
        Object covariance matrix [3,1].

        Units: m²
        """
        ...

    @cn_r.setter
    def cn_r(self, value: float) -> None: ...
    @property
    def cn_t(self) -> float:
        """
        Object covariance matrix [3,2].

        Units: m²
        """
        ...

    @cn_t.setter
    def cn_t(self, value: float) -> None: ...
    @property
    def cndot_n(self) -> float:
        """
        Object covariance matrix [6,3].

        Units: m²/s
        """
        ...

    @cndot_n.setter
    def cndot_n(self, value: float) -> None: ...
    @property
    def cndot_ndot(self) -> float:
        """
        Object covariance matrix [6,6].

        Units: m²/s²
        """
        ...

    @cndot_ndot.setter
    def cndot_ndot(self, value: float) -> None: ...
    @property
    def cndot_r(self) -> float:
        """
        Object covariance matrix [6,1].

        Units: m²/s
        """
        ...

    @cndot_r.setter
    def cndot_r(self, value: float) -> None: ...
    @property
    def cndot_rdot(self) -> float:
        """
        Object covariance matrix [6,4].

        Units: m²/s²
        """
        ...

    @cndot_rdot.setter
    def cndot_rdot(self, value: float) -> None: ...
    @property
    def cndot_t(self) -> float:
        """
        Object covariance matrix [6,2].

        Units: m²/s
        """
        ...

    @cndot_t.setter
    def cndot_t(self, value: float) -> None: ...
    @property
    def cndot_tdot(self) -> float:
        """
        Object covariance matrix [6,5].

        Units: m²/s²
        """
        ...

    @cndot_tdot.setter
    def cndot_tdot(self, value: float) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def cr_r(self) -> float:
        """
        Object covariance matrix [1,1].

        Units: m²
        """
        ...

    @cr_r.setter
    def cr_r(self, value: float) -> None: ...
    @property
    def crdot_n(self) -> float:
        """
        Object covariance matrix [4,3].

        Units: m²/s
        """
        ...

    @crdot_n.setter
    def crdot_n(self, value: float) -> None: ...
    @property
    def crdot_r(self) -> float:
        """
        Object covariance matrix [4,1].

        Units: m²/s
        """
        ...

    @crdot_r.setter
    def crdot_r(self, value: float) -> None: ...
    @property
    def crdot_rdot(self) -> float:
        """
        Object covariance matrix [4,4].

        Units: m²/s²
        """
        ...

    @crdot_rdot.setter
    def crdot_rdot(self, value: float) -> None: ...
    @property
    def crdot_t(self) -> float:
        """
        Object covariance matrix [4,2].

        Units: m²/s
        """
        ...

    @crdot_t.setter
    def crdot_t(self, value: float) -> None: ...
    @property
    def csrp_drg(self) -> float:
        """
        Object covariance matrix [8,7].

        Units: m⁴/kg²
        """
        ...

    @csrp_drg.setter
    def csrp_drg(self, value: float) -> None: ...
    @property
    def csrp_n(self) -> float:
        """
        Object covariance matrix [8,3].

        Units: m³/kg
        """
        ...

    @csrp_n.setter
    def csrp_n(self, value: float) -> None: ...
    @property
    def csrp_ndot(self) -> float:
        """
        Object covariance matrix [8,6].

        Units: m³/(kg*s)
        """
        ...

    @csrp_ndot.setter
    def csrp_ndot(self, value: float) -> None: ...
    @property
    def csrp_r(self) -> float:
        """
        Object covariance matrix [8,1].

        Units: m³/kg
        """
        ...

    @csrp_r.setter
    def csrp_r(self, value: float) -> None: ...
    @property
    def csrp_rdot(self) -> float:
        """
        Object covariance matrix [8,4].

        Units: m³/(kg*s)
        """
        ...

    @csrp_rdot.setter
    def csrp_rdot(self, value: float) -> None: ...
    @property
    def csrp_srp(self) -> float:
        """
        Object covariance matrix [8,8].

        Units: m⁴/kg²
        """
        ...

    @csrp_srp.setter
    def csrp_srp(self, value: float) -> None: ...
    @property
    def csrp_t(self) -> float:
        """
        Object covariance matrix [8,2].

        Units: m³/kg
        """
        ...

    @csrp_t.setter
    def csrp_t(self, value: float) -> None: ...
    @property
    def csrp_tdot(self) -> float:
        """
        Object covariance matrix [8,5].

        Units: m³/(kg*s)
        """
        ...

    @csrp_tdot.setter
    def csrp_tdot(self, value: float) -> None: ...
    @property
    def ct_r(self) -> float:
        """
        Object covariance matrix [2,1].

        Units: m²
        """
        ...

    @ct_r.setter
    def ct_r(self, value: float) -> None: ...
    @property
    def ct_t(self) -> float:
        """
        Object covariance matrix [2,2].

        Units: m²
        """
        ...

    @ct_t.setter
    def ct_t(self, value: float) -> None: ...
    @property
    def ctdot_n(self) -> float:
        """
        Object covariance matrix [5,3].

        Units: m²/s
        """
        ...

    @ctdot_n.setter
    def ctdot_n(self, value: float) -> None: ...
    @property
    def ctdot_r(self) -> float:
        """
        Object covariance matrix [5,1].

        Units: m²/s
        """
        ...

    @ctdot_r.setter
    def ctdot_r(self, value: float) -> None: ...
    @property
    def ctdot_rdot(self) -> float:
        """
        Object covariance matrix [5,4].

        Units: m²/s²
        """
        ...

    @ctdot_rdot.setter
    def ctdot_rdot(self, value: float) -> None: ...
    @property
    def ctdot_t(self) -> float:
        """
        Object covariance matrix [5,2].

        Units: m²/s
        """
        ...

    @ctdot_t.setter
    def ctdot_t(self, value: float) -> None: ...
    @property
    def ctdot_tdot(self) -> float:
        """
        Object covariance matrix [5,5].

        Units: m²/s²
        """
        ...

    @ctdot_tdot.setter
    def ctdot_tdot(self, value: float) -> None: ...
    @property
    def cthr_drg(self) -> float:
        """
        Object covariance matrix [9,7].

        Units: m³/(kg*s²)
        """
        ...

    @cthr_drg.setter
    def cthr_drg(self, value: float) -> None: ...
    @property
    def cthr_n(self) -> float:
        """
        Object covariance matrix [9,3].

        Units: m²/s²
        """
        ...

    @cthr_n.setter
    def cthr_n(self, value: float) -> None: ...
    @property
    def cthr_ndot(self) -> float:
        """
        Object covariance matrix [9,6].

        Units: m²/s³
        """
        ...

    @cthr_ndot.setter
    def cthr_ndot(self, value: float) -> None: ...
    @property
    def cthr_r(self) -> float:
        """
        Object covariance matrix [9,1].

        Units: m²/s²
        """
        ...

    @cthr_r.setter
    def cthr_r(self, value: float) -> None: ...
    @property
    def cthr_rdot(self) -> float:
        """
        Object covariance matrix [9,4].

        Units: m²/s³
        """
        ...

    @cthr_rdot.setter
    def cthr_rdot(self, value: float) -> None: ...
    @property
    def cthr_srp(self) -> float:
        """
        Object covariance matrix [9,8].

        Units: m³/(kg*s²)
        """
        ...

    @cthr_srp.setter
    def cthr_srp(self, value: float) -> None: ...
    @property
    def cthr_t(self) -> float:
        """
        Object covariance matrix [9,2].

        Units: m²/s²
        """
        ...

    @cthr_t.setter
    def cthr_t(self, value: float) -> None: ...
    @property
    def cthr_tdot(self) -> float:
        """
        Object covariance matrix [9,5].

        Units: m²/s³
        """
        ...

    @cthr_tdot.setter
    def cthr_tdot(self, value: float) -> None: ...
    @property
    def cthr_thr(self) -> float:
        """
        Object covariance matrix [9,9].

        Units: m²/s⁴
        """
        ...

    @cthr_thr.setter
    def cthr_thr(self, value: float) -> None: ...
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
    def additional_parameters(self) -> Optional[AdditionalParameters]:
        """
        Additional Parameters.
        """
        ...

    @additional_parameters.setter
    def additional_parameters(self, value: Optional[AdditionalParameters]) -> None: ...
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
        Covariance Matrix.
        """
        ...

    @covariance_matrix.setter
    def covariance_matrix(self, value: CdmCovarianceMatrix) -> None: ...
    @property
    def od_parameters(self) -> Optional[OdParameters]:
        """
        Orbit Determination Parameters.
        """
        ...

    @od_parameters.setter
    def od_parameters(self, value: Optional[OdParameters]) -> None: ...
    @property
    def state_vector(self) -> CdmStateVector:
        """
        State Vector.
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
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def creation_date(self) -> str:
        """
        Message creation date/time in Coordinated Universal Time (UTC).

        Examples: 2010-03-12T22:31:12.000, 2010-071T22:31:12.000
        """
        ...

    @creation_date.setter
    def creation_date(self, value: str) -> None: ...
    @property
    def message_for(self) -> Optional[str]:
        """
        Spacecraft name(s) for which the CDM is provided.

        Examples: SPOT, ENVISAT, IRIDIUM, INTELSAT
        """
        ...

    @message_for.setter
    def message_for(self, value: Optional[str]) -> None: ...
    @property
    def message_id(self) -> str:
        """
        ID that uniquely identifies a message from a given originator.

        Examples: 201113719185, ABC-12_34
        """
        ...

    @message_id.setter
    def message_id(self, value: str) -> None: ...
    @property
    def originator(self) -> str:
        """
        Creating agency or owner/operator.

        Examples: JSPOC, ESA SST, CAESAR, JPL, SDC
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
        The atmospheric density model used for the OD of the object.
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
    def covariance_method(self) -> CovarianceMethodType:
        """
        Method used to calculate the covariance.
        """
        ...

    @covariance_method.setter
    def covariance_method(self, value: CovarianceMethodType) -> None: ...
    @property
    def earth_tides(self) -> Optional[str]:
        """
        Indication of whether solid Earth and ocean tides were used.
        """
        ...

    @earth_tides.setter
    def earth_tides(self, value: Optional[str]) -> None: ...
    @property
    def ephemeris_name(self) -> str:
        """
        Unique name of the external ephemeris file used for the object or NONE.
        """
        ...

    @ephemeris_name.setter
    def ephemeris_name(self, value: str) -> None: ...
    @property
    def gravity_model(self) -> Optional[str]:
        """
        The gravity model used for the OD of the object.
        """
        ...

    @gravity_model.setter
    def gravity_model(self, value: Optional[str]) -> None: ...
    @property
    def international_designator(self) -> str:
        """
        The full international designator for the object.
        """
        ...

    @international_designator.setter
    def international_designator(self, value: str) -> None: ...
    @property
    def intrack_thrust(self) -> Optional[str]:
        """
        Indication of whether in-track thrust modeling was used.
        """
        ...

    @intrack_thrust.setter
    def intrack_thrust(self, value: Optional[str]) -> None: ...
    @property
    def maneuverable(self) -> ManeuverableType:
        """
        The maneuver capacity of the object.
        """
        ...

    @maneuverable.setter
    def maneuverable(self, value: ManeuverableType) -> None: ...
    @property
    def n_body_perturbations(self) -> Optional[str]:
        """
        The N-body gravitational perturbations used for the OD of the object.
        """
        ...

    @n_body_perturbations.setter
    def n_body_perturbations(self, value: Optional[str]) -> None: ...
    @property
    def object(self) -> CdmObjectType:
        """
        The object to which the metadata and data apply.

        Examples: OBJECT1, OBJECT2
        """
        ...

    @object.setter
    def object(self, value: CdmObjectType) -> None: ...
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
    def object_type(self) -> Optional[ObjectDescription]:
        """
        The object type.

        Examples: PAYLOAD, ROCKET BODY, DEBRIS, UNKNOWN, OTHER
        """
        ...

    @object_type.setter
    def object_type(self, value: Optional[ObjectDescription]) -> None: ...
    @property
    def operator_contact_position(self) -> Optional[str]:
        """
        Contact position of the owner/operator of the object.
        """
        ...

    @operator_contact_position.setter
    def operator_contact_position(self, value: Optional[str]) -> None: ...
    @property
    def operator_email(self) -> Optional[str]:
        """
        Email address of the contact position or organization of the object.
        """
        ...

    @operator_email.setter
    def operator_email(self, value: Optional[str]) -> None: ...
    @property
    def operator_organization(self) -> Optional[str]:
        """
        Contact organization of the object.
        """
        ...

    @operator_organization.setter
    def operator_organization(self, value: Optional[str]) -> None: ...
    @property
    def operator_phone(self) -> Optional[str]:
        """
        Phone number of the contact position or organization for the object.
        """
        ...

    @operator_phone.setter
    def operator_phone(self, value: Optional[str]) -> None: ...
    @property
    def orbit_center(self) -> Optional[str]:
        """
        The central body about which Object1 and Object2 orbit.
        """
        ...

    @orbit_center.setter
    def orbit_center(self, value: Optional[str]) -> None: ...
    @property
    def ref_frame(self) -> ReferenceFrameType:
        """
        Name of the reference frame in which the state vector data are given.
        """
        ...

    @ref_frame.setter
    def ref_frame(self, value: ReferenceFrameType) -> None: ...
    @property
    def solar_rad_pressure(self) -> Optional[str]:
        """
        Indication of whether solar radiation pressure perturbations were used.
        """
        ...

    @solar_rad_pressure.setter
    def solar_rad_pressure(self, value: Optional[str]) -> None: ...

class CdmObjectType:
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
        Object Position Vector X component.

        Units: km
        """
        ...

    @x.setter
    def x(self, value: float) -> None: ...
    @property
    def x_dot(self) -> float:
        """
        Object Velocity Vector X component.

        Units: km/s
        """
        ...

    @x_dot.setter
    def x_dot(self, value: float) -> None: ...
    @property
    def y(self) -> float:
        """
        Object Position Vector Y component.

        Units: km
        """
        ...

    @y.setter
    def y(self, value: float) -> None: ...
    @property
    def y_dot(self) -> float:
        """
        Object Velocity Vector Y component.

        Units: km/s
        """
        ...

    @y_dot.setter
    def y_dot(self, value: float) -> None: ...
    @property
    def z(self) -> float:
        """
        Object Position Vector Z component.

        Units: km
        """
        ...

    @z.setter
    def z(self, value: float) -> None: ...
    @property
    def z_dot(self) -> float:
        """
        Object Velocity Vector Z component.

        Units: km/s
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
    Ground impact parameters (groundImpactParametersType, RDM).

    Parameters
    ----------
    probability_of_impact : float, optional
        Probability of impact.
    probability_of_burn_up : float, optional
        Probability of burn up.
    probability_of_break_up : float, optional
        Probability of break up.
    probability_of_land_impact : float, optional
        Probability of land impact.
    probability_of_casualty : float, optional
        Probability of casualty.
    nominal_impact_epoch : str, optional
        Nominal impact epoch.
    impact_window_start : str, optional
        Impact window start.
    impact_window_end : str, optional
        Impact window end.
    impact_ref_frame : str, optional
        Impact reference frame.
    nominal_impact_lon : float, optional
        Nominal impact longitude. Units: deg
    nominal_impact_lat : float, optional
        Nominal impact latitude. Units: deg
    nominal_impact_alt : float, optional
        Nominal impact altitude. Units: km
    impact_1_confidence : float, optional
        Impact 1 confidence. Units: %
    impact_1_start_lon : float, optional
        Impact 1 start longitude. Units: deg
    impact_1_start_lat : float, optional
        Impact 1 start latitude. Units: deg
    impact_1_stop_lon : float, optional
        Impact 1 stop longitude. Units: deg
    impact_1_stop_lat : float, optional
        Impact 1 stop latitude. Units: deg
    impact_1_cross_track : float, optional
        Impact 1 cross track. Units: km
    impact_2_confidence : float, optional
        Impact 2 confidence. Units: %
    impact_2_start_lon : float, optional
        Impact 2 start longitude. Units: deg
    impact_2_start_lat : float, optional
        Impact 2 start latitude. Units: deg
    impact_2_stop_lon : float, optional
        Impact 2 stop longitude. Units: deg
    impact_2_stop_lat : float, optional
        Impact 2 stop latitude. Units: deg
    impact_2_cross_track : float, optional
        Impact 2 cross track. Units: km
    impact_3_confidence : float, optional
        Impact 3 confidence. Units: %
    impact_3_start_lon : float, optional
        Impact 3 start longitude. Units: deg
    impact_3_start_lat : float, optional
        Impact 3 start latitude. Units: deg
    impact_3_stop_lon : float, optional
        Impact 3 stop longitude. Units: deg
    impact_3_stop_lat : float, optional
        Impact 3 stop latitude. Units: deg
    impact_3_cross_track : float, optional
        Impact 3 cross track. Units: km
    comment : list of str, optional
        Comments.
    """
    def __init__(
        probability_of_impact,
        probability_of_burn_up,
        probability_of_break_up,
        probability_of_land_impact,
        probability_of_casualty,
        nominal_impact_epoch,
        impact_window_start,
        impact_window_end,
        impact_ref_frame,
        nominal_impact_lon,
        nominal_impact_lat,
        nominal_impact_alt,
        impact_1_confidence,
        impact_1_start_lon,
        impact_1_start_lat,
        impact_1_stop_lon,
        impact_1_stop_lat,
        impact_1_cross_track,
        impact_2_confidence,
        impact_2_start_lon,
        impact_2_start_lat,
        impact_2_stop_lon,
        impact_2_stop_lat,
        impact_2_cross_track,
        impact_3_confidence,
        impact_3_start_lon,
        impact_3_start_lat,
        impact_3_stop_lon,
        impact_3_stop_lat,
        impact_3_cross_track,
        comment,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def impact_1_confidence(self) -> Optional[float]:
        """
        Confidence of impact prediction 1.
        """
        ...

    @impact_1_confidence.setter
    def impact_1_confidence(self, value: Optional[float]) -> None: ...
    @property
    def impact_1_cross_track(self) -> Optional[float]:
        """
        Impact 1 cross track distance.

        Units: km
        """
        ...

    @impact_1_cross_track.setter
    def impact_1_cross_track(self, value: Optional[float]) -> None: ...
    @property
    def impact_1_start_lat(self) -> Optional[float]:
        """
        Impact 1 start latitude.

        Units: deg
        """
        ...

    @impact_1_start_lat.setter
    def impact_1_start_lat(self, value: Optional[float]) -> None: ...
    @property
    def impact_1_start_lon(self) -> Optional[float]:
        """
        Impact 1 start longitude.

        Units: deg
        """
        ...

    @impact_1_start_lon.setter
    def impact_1_start_lon(self, value: Optional[float]) -> None: ...
    @property
    def impact_1_stop_lat(self) -> Optional[float]:
        """
        Impact stops latitude.

        Units: deg
        """
        ...

    @impact_1_stop_lat.setter
    def impact_1_stop_lat(self, value: Optional[float]) -> None: ...
    @property
    def impact_1_stop_lon(self) -> Optional[float]:
        """
        Impact 1 stop longitude.

        Units: deg
        """
        ...

    @impact_1_stop_lon.setter
    def impact_1_stop_lon(self, value: Optional[float]) -> None: ...
    @property
    def impact_2_confidence(self) -> Optional[float]:
        """
        Confidence of impact prediction 2.
        """
        ...

    @impact_2_confidence.setter
    def impact_2_confidence(self, value: Optional[float]) -> None: ...
    @property
    def impact_2_cross_track(self) -> Optional[float]:
        """
        Impact 2 cross track distance.

        Units: km
        """
        ...

    @impact_2_cross_track.setter
    def impact_2_cross_track(self, value: Optional[float]) -> None: ...
    @property
    def impact_2_start_lat(self) -> Optional[float]:
        """
        Impact 2 start latitude.

        Units: deg
        """
        ...

    @impact_2_start_lat.setter
    def impact_2_start_lat(self, value: Optional[float]) -> None: ...
    @property
    def impact_2_start_lon(self) -> Optional[float]:
        """
        Impact 2 start longitude.

        Units: deg
        """
        ...

    @impact_2_start_lon.setter
    def impact_2_start_lon(self, value: Optional[float]) -> None: ...
    @property
    def impact_2_stop_lat(self) -> Optional[float]:
        """
        Impact 2 stop latitude.

        Units: deg
        """
        ...

    @impact_2_stop_lat.setter
    def impact_2_stop_lat(self, value: Optional[float]) -> None: ...
    @property
    def impact_2_stop_lon(self) -> Optional[float]:
        """
        Impact 2 stop longitude.

        Units: deg
        """
        ...

    @impact_2_stop_lon.setter
    def impact_2_stop_lon(self, value: Optional[float]) -> None: ...
    @property
    def impact_3_confidence(self) -> Optional[float]:
        """
        Confidence of impact prediction 3.
        """
        ...

    @impact_3_confidence.setter
    def impact_3_confidence(self, value: Optional[float]) -> None: ...
    @property
    def impact_3_cross_track(self) -> Optional[float]:
        """
        Impact 3 cross track distance.

        Units: km
        """
        ...

    @impact_3_cross_track.setter
    def impact_3_cross_track(self, value: Optional[float]) -> None: ...
    @property
    def impact_3_start_lat(self) -> Optional[float]:
        """
        Impact 3 start latitude.

        Units: deg
        """
        ...

    @impact_3_start_lat.setter
    def impact_3_start_lat(self, value: Optional[float]) -> None: ...
    @property
    def impact_3_start_lon(self) -> Optional[float]:
        """
        Impact 3 start longitude.

        Units: deg
        """
        ...

    @impact_3_start_lon.setter
    def impact_3_start_lon(self, value: Optional[float]) -> None: ...
    @property
    def impact_3_stop_lat(self) -> Optional[float]:
        """
        Impact 3 stop latitude.

        Units: deg
        """
        ...

    @impact_3_stop_lat.setter
    def impact_3_stop_lat(self, value: Optional[float]) -> None: ...
    @property
    def impact_3_stop_lon(self) -> Optional[float]:
        """
        Impact 3 stop longitude.

        Units: deg
        """
        ...

    @impact_3_stop_lon.setter
    def impact_3_stop_lon(self, value: Optional[float]) -> None: ...
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
        Impact window end epoch.
        """
        ...

    @impact_window_end.setter
    def impact_window_end(self, value: Optional[str]) -> None: ...
    @property
    def impact_window_start(self) -> Optional[str]:
        """
        Impact window start epoch.
        """
        ...

    @impact_window_start.setter
    def impact_window_start(self, value: Optional[str]) -> None: ...
    @property
    def nominal_impact_alt(self) -> Optional[float]:
        """
        Nominal impact altitude.

        Units: km
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
        Nominal impact latitude.

        Units: deg
        """
        ...

    @nominal_impact_lat.setter
    def nominal_impact_lat(self, value: Optional[float]) -> None: ...
    @property
    def nominal_impact_lon(self) -> Optional[float]:
        """
        Nominal impact longitude.

        Units: deg
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
    """
    Osculating Keplerian Elements.

    References:
    - CCSDS 502.0-B-3, Section 3.2.4 (OPM Data Section)

    Parameters
    ----------
    semi_major_axis : float
        Semi-major axis (km).
    eccentricity : float
        Eccentricity (dimensionless).
    inclination : float
        Inclination (deg).
    ra_of_asc_node : float
        Right ascension of the ascending node (deg).
    arg_of_pericenter : float
        Argument of pericenter (deg).
    gm : float
        Gravitational coefficient (km³/s²).
    true_anomaly : float, optional
        True anomaly (deg).
    mean_anomaly : float, optional
        Mean anomaly (deg).

    Attributes
    ----------
    semi_major_axis : float
        Semi-major axis. Units: km.
    eccentricity : float
        Eccentricity. Units: dimensionless.
    inclination : float
        Inclination. Units: deg.
    ra_of_asc_node : float
        Right ascension of the ascending node. Units: deg.
    arg_of_pericenter : float
        Argument of pericenter. Units: deg.
    gm : float
        Gravitational coefficient (GM). Units: km³/s².
    true_anomaly : float or None
        True anomaly. Units: deg.
    mean_anomaly : float or None
        Mean anomaly. Units: deg.
    """
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

        Examples: 45.6

        Units: deg
        """
        ...

    @arg_of_pericenter.setter
    def arg_of_pericenter(self, value: float) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def eccentricity(self) -> float:
        """
        Eccentricity.

        Examples: 0.001
        """
        ...

    @eccentricity.setter
    def eccentricity(self, value: float) -> None: ...
    @property
    def gm(self) -> float:
        """
        Gravitational Coefficient (Gravitational Constant × Central Mass).

        Examples: 398600.4418

        Units: km³/s²
        """
        ...

    @gm.setter
    def gm(self, value: float) -> None: ...
    @property
    def inclination(self) -> float:
        """
        Inclination.

        Examples: 51.6

        Units: deg
        """
        ...

    @inclination.setter
    def inclination(self, value: float) -> None: ...
    @property
    def mean_anomaly(self) -> Optional[float]:
        """
        Mean anomaly.

        Examples: 0.0

        Units: deg
        """
        ...

    @mean_anomaly.setter
    def mean_anomaly(self, value: Optional[float]) -> None: ...
    @property
    def ra_of_asc_node(self) -> float:
        """
        Right ascension of ascending node.

        Examples: 123.4

        Units: deg
        """
        ...

    @ra_of_asc_node.setter
    def ra_of_asc_node(self, value: float) -> None: ...
    @property
    def semi_major_axis(self) -> float:
        """
        Semi-major axis.

        Examples: 6653.148

        Units: km
        """
        ...

    @semi_major_axis.setter
    def semi_major_axis(self, value: float) -> None: ...
    @property
    def true_anomaly(self) -> Optional[float]:
        """
        True anomaly.

        Examples: 0.0

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
    Maneuver Parameters.

    References:
    - CCSDS 502.0-B-3, Section 3.2.4 (OPM Data Section)

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
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def man_delta_mass(self) -> float:
        """
        Mass change during maneuver (value is < 0).

        Units: kg
        """
        ...

    @man_delta_mass.setter
    def man_delta_mass(self, value: float) -> None: ...
    @property
    def man_duration(self) -> float:
        """
        Maneuver duration (If = 0, impulsive maneuver).

        Units: s
        """
        ...

    @man_duration.setter
    def man_duration(self, value: float) -> None: ...
    @property
    def man_dv_1(self) -> float:
        """
        1st component of the velocity increment.

        Units: km/s
        """
        ...

    @man_dv_1.setter
    def man_dv_1(self, value: float) -> None: ...
    @property
    def man_dv_2(self) -> float:
        """
        2nd component of the velocity increment.

        Units: km/s
        """
        ...

    @man_dv_2.setter
    def man_dv_2(self, value: float) -> None: ...
    @property
    def man_dv_3(self) -> float:
        """
        3rd component of the velocity increment.

        Units: km/s
        """
        ...

    @man_dv_3.setter
    def man_dv_3(self, value: float) -> None: ...
    @property
    def man_epoch_ignition(self) -> str:
        """
        Epoch of ignition (see 7.5.10 for formatting rules).
        """
        ...

    @man_epoch_ignition.setter
    def man_epoch_ignition(self, value: str) -> None: ...
    @property
    def man_ref_frame(self) -> str:
        """
        Reference frame in which the velocity increment vector data are given. The user must select
        from the accepted set of values indicated in 3.2.4.11.
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
    Mean Keplerian Elements in the Specified Reference Frame.

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
        Semi-major axis in kilometers. Preferred over MEAN_MOTION.
    mean_motion : float, optional
        Keplerian Mean motion in revolutions per day. Required if MEAN_ELEMENT_THEORY = SGP/SGP4.
    gm : float, optional
        Gravitational Coefficient (Gravitational Constant × Central Mass) in km³/s².
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

        Examples: 270.0

        Units: deg
        """
        ...

    @arg_of_pericenter.setter
    def arg_of_pericenter(self, value: float) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def eccentricity(self) -> float:
        """
        Eccentricity.

        Examples: 0.7303
        """
        ...

    @eccentricity.setter
    def eccentricity(self, value: float) -> None: ...
    @property
    def epoch(self) -> str:
        """
        Epoch of Mean Keplerian elements. (See 7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def gm(self) -> Optional[float]:
        """
        Gravitational Coefficient (Gravitational Constant × Central Mass).

        Examples: 398600.44

        Units: km³/s²
        """
        ...

    @gm.setter
    def gm(self, value: Optional[float]) -> None: ...
    @property
    def inclination(self) -> float:
        """
        Inclination.

        Examples: 63.4

        Units: deg
        """
        ...

    @inclination.setter
    def inclination(self, value: float) -> None: ...
    @property
    def mean_anomaly(self) -> float:
        """
        Mean anomaly.

        Examples: 130.0

        Units: deg
        """
        ...

    @mean_anomaly.setter
    def mean_anomaly(self, value: float) -> None: ...
    @property
    def mean_motion(self) -> Optional[float]:
        """
        Keplerian Mean motion.

        Required if MEAN_ELEMENT_THEORY = SGP/SGP4.

        Examples: 1.491325

        Units: rev/day
        """
        ...

    @mean_motion.setter
    def mean_motion(self, value: Optional[float]) -> None: ...
    @property
    def ra_of_asc_node(self) -> float:
        """
        Right ascension of ascending node.

        Examples: 345.0

        Units: deg
        """
        ...

    @ra_of_asc_node.setter
    def ra_of_asc_node(self, value: float) -> None: ...
    @property
    def semi_major_axis(self) -> Optional[float]:
        """
        Semi-major axis. Preferred over MEAN_MOTION.

        Examples: 28594.4

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
    Orbit Comprehensive Message (OCM).

    An OCM aggregates and extends OMM, OPM, and OEM content in a single hybrid message.
    It emphasizes flexibility and message conciseness by offering extensive optional
    standardized content while minimizing mandatory content.

    References:
    - CCSDS 502.0-B-3, Section 5 (OCM)

    Parameters
    ----------
    header : OdmHeader
        The message header.
    segment : OcmSegment
        The OCM data segment.
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
            The parsed Ocm object.
        """
        ...

    @property
    def header(self) -> OdmHeader:
        """
        Orbit Comprehensive Message (OCM).

        An OCM aggregates and extends OMM, OPM, and OEM content in a single hybrid message.
        It emphasizes flexibility and message conciseness by offering extensive optional
        standardized content while minimizing mandatory content.

        References:
        - CCSDS 502.0-B-3, Section 5 (OCM)
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
    OCM Covariance Matrix.

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
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def cov_basis(self) -> Optional[str]:
        """
        Basis of this covariance time history data (e.g., PREDICTED, DETERMINED).

        Examples: PREDICTED
        """
        ...

    @cov_basis.setter
    def cov_basis(self, value: Optional[str]) -> None: ...
    @property
    def cov_basis_id(self) -> Optional[str]:
        """
        Identification number for the telemetry dataset, orbit determination, or simulation upon
        which the COV_BASIS is based.

        Examples: OD-123
        """
        ...

    @cov_basis_id.setter
    def cov_basis_id(self, value: Optional[str]) -> None: ...
    @property
    def cov_confidence(self) -> Optional[float]:
        """
        A measure of the confidence in the covariance errors matching reality.

        Examples: 95.0

        Units: %
        """
        ...

    @cov_confidence.setter
    def cov_confidence(self, value: Optional[float]) -> None: ...
    @property
    def cov_frame_epoch(self) -> Optional[str]:
        """
        Epoch of the covariance data reference frame, if not intrinsic to its definition.

        Examples: 2000-01-01T12:00:00Z
        """
        ...

    @cov_frame_epoch.setter
    def cov_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def cov_id(self) -> Optional[str]:
        """
        Identification number for this covariance time history block.

        Examples: 1
        """
        ...

    @cov_id.setter
    def cov_id(self, value: Optional[str]) -> None: ...
    @property
    def cov_lines(self) -> list[CovLine]:
        """
        Contiguous set of covariance matrix data lines.
        """
        ...

    @cov_lines.setter
    def cov_lines(self, value: list[CovLine]) -> None: ...
    @property
    def cov_next_id(self) -> Optional[str]:
        """
        Identification number for the next covariance time history.

        Examples: 2
        """
        ...

    @cov_next_id.setter
    def cov_next_id(self, value: Optional[str]) -> None: ...
    @property
    def cov_ordering(self) -> str:
        """
        Indicates covariance ordering (LTM or UTM).

        Examples: LTM
        """
        ...

    @cov_ordering.setter
    def cov_ordering(self, value: str) -> None: ...
    @property
    def cov_prev_id(self) -> Optional[str]:
        """
        Identification number for the previous covariance time history.

        Examples: 0
        """
        ...

    @cov_prev_id.setter
    def cov_prev_id(self, value: Optional[str]) -> None: ...
    @property
    def cov_ref_frame(self) -> str:
        """
        Reference frame of the covariance time history (value to be drawn from the SANA registry list
        of Reference Frames at <https://sanaregistry.org/r/celestial_body_reference_frames> or
        <https://sanaregistry.org/r/orbit_relative_reference_frames>).

        Examples: ICRF, EME2000
        """
        ...

    @cov_ref_frame.setter
    def cov_ref_frame(self, value: str) -> None: ...
    @property
    def cov_scale_max(self) -> Optional[float]:
        """
        Maximum scale factor to apply to this covariance data to achieve realism.

        Examples: 1.1
        """
        ...

    @cov_scale_max.setter
    def cov_scale_max(self, value: Optional[float]) -> None: ...
    @property
    def cov_scale_min(self) -> Optional[float]:
        """
        Minimum scale factor to apply to this covariance data to achieve realism.

        Examples: 0.9
        """
        ...

    @cov_scale_min.setter
    def cov_scale_min(self, value: Optional[float]) -> None: ...
    @property
    def cov_type(self) -> str:
        """
        Specification of the covariance element set type (value to be drawn from the SANA registry
        list of Covariance Types at <https://sanaregistry.org/r/orbital_covariance_matrix_types>).

        Examples: CARTESIAN
        """
        ...

    @cov_type.setter
    def cov_type(self, value: str) -> None: ...
    @property
    def cov_units(self) -> Optional[str]:
        """
        SI unit designations for the covariance elements.

        Examples: km**2, km**2/s
        """
        ...

    @cov_units.setter
    def cov_units(self, value: Optional[str]) -> None: ...

class OcmData:
    """
    OCM Data Section.

    This struct is the primary data container for the OCM. It holds all the
    different data blocks, such as trajectory, physical properties, covariance,
    maneuvers, and other related information.
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
    def man(self) -> list[OcmManeuverParameters]:
        """
        List of maneuver specifications.
        """
        ...

    @man.setter
    def man(self, value: list[OcmManeuverParameters]) -> None: ...
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

class OcmManeuverParameters:
    """
    OCM Maneuver Parameters.

    References:
    - CCSDS 502.0-B-3, Section 4.5.5 (OCM Maneuver Section)

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
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def dc_body_frame(self) -> Optional[str]:
        """
        Body reference frame in which DC_BODY_TRIGGER will be specified.

        Examples: SC_BODY
        """
        ...

    @dc_body_frame.setter
    def dc_body_frame(self, value: Optional[str]) -> None: ...
    @property
    def dc_body_trigger(self) -> Optional[list[float]]:
        """
        Body frame reference vector direction for angle-based duty cycle initiation.

        Examples: 0.0 1.0 0.0
        """
        ...

    @dc_body_trigger.setter
    def dc_body_trigger(self, value: Optional[list[float]]) -> None: ...
    @property
    def dc_exec_start(self) -> Optional[str]:
        """
        Start time of the initial duty cycle-based maneuver sequence execution.

        Examples: 2000-01-01T12:05:00Z
        """
        ...

    @dc_exec_start.setter
    def dc_exec_start(self, value: Optional[str]) -> None: ...
    @property
    def dc_exec_stop(self) -> Optional[str]:
        """
        End time of the final duty cycle-based maneuver sequence execution.

        Examples: 2000-01-01T12:55:00Z
        """
        ...

    @dc_exec_stop.setter
    def dc_exec_stop(self, value: Optional[str]) -> None: ...
    @property
    def dc_max_cycles(self) -> Optional[int]:
        """
        Maximum number of ‘ON’ duty cycles.

        Examples: 10
        """
        ...

    @dc_max_cycles.setter
    def dc_max_cycles(self, value: Optional[int]) -> None: ...
    @property
    def dc_min_cycles(self) -> Optional[int]:
        """
        Minimum number of ‘ON’ duty cycles.

        Examples: 1
        """
        ...

    @dc_min_cycles.setter
    def dc_min_cycles(self, value: Optional[int]) -> None: ...
    @property
    def dc_pa_start_angle(self) -> Optional[float]:
        """
        Phase angle offset of thruster pulse start.

        Examples: 10.0

        Units: deg
        """
        ...

    @dc_pa_start_angle.setter
    def dc_pa_start_angle(self, value: Optional[float]) -> None: ...
    @property
    def dc_pa_stop_angle(self) -> Optional[float]:
        """
        Phase angle of thruster pulse stop.

        Examples: 20.0

        Units: deg
        """
        ...

    @dc_pa_stop_angle.setter
    def dc_pa_stop_angle(self, value: Optional[float]) -> None: ...
    @property
    def dc_ref_dir(self) -> Optional[list[float]]:
        """
        Reference vector direction in the body frame for angle-initiated thruster duty cycles.

        Examples: 1.0 0.0 0.0
        """
        ...

    @dc_ref_dir.setter
    def dc_ref_dir(self, value: Optional[list[float]]) -> None: ...
    @property
    def dc_ref_time(self) -> Optional[str]:
        """
        Reference time for the THRUST duty cycle.

        Examples: 2000-01-01T12:00:00Z
        """
        ...

    @dc_ref_time.setter
    def dc_ref_time(self, value: Optional[str]) -> None: ...
    @property
    def dc_time_pulse_duration(self) -> Optional[float]:
        """
        Thruster pulse ‘ON’ duration.

        Examples: 10.0

        Units: s
        """
        ...

    @dc_time_pulse_duration.setter
    def dc_time_pulse_duration(self, value: Optional[float]) -> None: ...
    @property
    def dc_time_pulse_period(self) -> Optional[float]:
        """
        Elapsed time between the start of one pulse and the start of the next.

        Examples: 100.0

        Units: s
        """
        ...

    @dc_time_pulse_period.setter
    def dc_time_pulse_period(self, value: Optional[float]) -> None: ...
    @property
    def dc_type(self) -> str:
        """
        Duty cycle type to use for this maneuver time history section.

        Examples: LUSTRE
        """
        ...

    @dc_type.setter
    def dc_type(self, value: str) -> None: ...
    @property
    def dc_win_close(self) -> Optional[str]:
        """
        End time of the duty cycle-based maneuver window.

        Examples: 2000-01-01T13:00:00Z
        """
        ...

    @dc_win_close.setter
    def dc_win_close(self, value: Optional[str]) -> None: ...
    @property
    def dc_win_open(self) -> Optional[str]:
        """
        Start time of the duty cycle-based maneuver window.

        Examples: 2000-01-01T12:00:00Z
        """
        ...

    @dc_win_open.setter
    def dc_win_open(self, value: Optional[str]) -> None: ...
    @property
    def grav_assist_name(self) -> Optional[str]:
        """
        Identification of a gravitational body that would be used for an assist maneuver (value to be
        drawn from the SANA registry list of Common Central Body Names at
        <https://sanaregistry.org/r/central_body_name>).

        Examples: EARTH, JUPITER
        """
        ...

    @grav_assist_name.setter
    def grav_assist_name(self, value: Optional[str]) -> None: ...
    @property
    def man_basis(self) -> Optional[str]:
        """
        Basis of this maneuver data (e.g., PREDICTED, DETERMINED, SIMULATED).

        Examples: PREDICTED
        """
        ...

    @man_basis.setter
    def man_basis(self, value: Optional[str]) -> None: ...
    @property
    def man_basis_id(self) -> Optional[str]:
        """
        Identification number for the telemetry dataset, orbit determination, or simulation upon
        which the MAN_BASIS is based.

        Examples: OD-123
        """
        ...

    @man_basis_id.setter
    def man_basis_id(self, value: Optional[str]) -> None: ...
    @property
    def man_composition(self) -> str:
        """
        Specification of the maneuver element set type (value to be drawn from the SANA registry list
        of Maneuver Types at https://sanaregistry.org/r/maneuver_type).

        Examples: ΔV_CARTESIAN, ΔV_SPHERICAL, THRUST_CARTESIAN
        """
        ...

    @man_composition.setter
    def man_composition(self, value: str) -> None: ...
    @property
    def man_device_id(self) -> str:
        """
        Identification name of the maneuver device (e.g., ‘THRUSTER-1’).

        Examples: THRUSTER-1
        """
        ...

    @man_device_id.setter
    def man_device_id(self, value: str) -> None: ...
    @property
    def man_frame_epoch(self) -> Optional[str]:
        """
        Epoch of the maneuver reference frame, if not intrinsic to its definition.

        Examples: 2000-01-01T12:00:00Z
        """
        ...

    @man_frame_epoch.setter
    def man_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def man_id(self) -> str:
        """
        Unique maneuver identification number for this maneuver block.

        Examples: 1
        """
        ...

    @man_id.setter
    def man_id(self, value: str) -> None: ...
    @property
    def man_lines(self) -> list[ManLine]:
        """
        Maneuver time history data lines.
        """
        ...

    @man_lines.setter
    def man_lines(self, value: list[ManLine]) -> None: ...
    @property
    def man_next_epoch(self) -> Optional[str]:
        """
        Start time of the next maneuver for this MAN_BASIS.

        Examples: 2000-01-02T12:00:00Z
        """
        ...

    @man_next_epoch.setter
    def man_next_epoch(self, value: Optional[str]) -> None: ...
    @property
    def man_next_id(self) -> Optional[str]:
        """
        Identification number for the next maneuver.

        Examples: 2
        """
        ...

    @man_next_id.setter
    def man_next_id(self, value: Optional[str]) -> None: ...
    @property
    def man_pred_source(self) -> Optional[str]:
        """
        Identification (e.g., message or file) of the predicted maneuver parameters upon which this
        maneuver is based.

        Examples: MAN-PRED-456
        """
        ...

    @man_pred_source.setter
    def man_pred_source(self, value: Optional[str]) -> None: ...
    @property
    def man_prev_epoch(self) -> Optional[str]:
        """
        Completion time of the previous maneuver for this MAN_BASIS.

        Examples: 2000-01-01T12:00:00Z
        """
        ...

    @man_prev_epoch.setter
    def man_prev_epoch(self, value: Optional[str]) -> None: ...
    @property
    def man_prev_id(self) -> Optional[str]:
        """
        Identification number for the previous maneuver.

        Examples: 0
        """
        ...

    @man_prev_id.setter
    def man_prev_id(self, value: Optional[str]) -> None: ...
    @property
    def man_purpose(self) -> Optional[str]:
        """
        Purpose of the maneuver (e.g., ‘WHEEL-DESAT’, ‘STATION-KEEPING’).

        Examples: STATION-KEEPING
        """
        ...

    @man_purpose.setter
    def man_purpose(self, value: Optional[str]) -> None: ...
    @property
    def man_ref_frame(self) -> str:
        """
        Reference frame for the maneuver thrust vector (value to be drawn from the SANA registry list
        of Reference Frames at <https://sanaregistry.org/r/orbit_relative_reference_frames>).

        Examples: TNW, RSW
        """
        ...

    @man_ref_frame.setter
    def man_ref_frame(self, value: str) -> None: ...
    @property
    def man_units(self) -> Optional[str]:
        """
        SI unit designations for the maneuver parameters.

        Examples: km/s, N
        """
        ...

    @man_units.setter
    def man_units(self, value: Optional[str]) -> None: ...

class OcmMetadata:
    """
    OCM Metadata Section.

    Parameters
    ----------
    time_system : str
        Time system that shall be used for all absolute time stamps in the message.
    epoch_tzero : str
        Epoch to which all relative times in the message are referenced (ISO 8601).
    object_name : str, optional
        Name of the space object that the message is associated with.
    international_designator : str, optional
        The COSPAR international designator of the space object.
    catalog_name : str, optional
        The name of the satellite catalog used for the space object identification.
    object_designator : str, optional
        The unique satellite identification designator used in the specified catalog.
    alternate_names : str, optional
        Alternate name(s) by which the space object is known.
    originator_poc : str, optional
        Originator Point-of-Contact.
    originator_position : str, optional
        Contact position of the originator PoC.
    originator_phone : str, optional
        Originator PoC phone number.
    originator_email : str, optional
        Originator PoC email address.
    originator_address : str, optional
        Originator's physical address.
    tech_org : str, optional
        Technical organization (creating agency or operator).
    tech_poc : str, optional
        Technical Point-of-Contact.
    tech_position : str, optional
        Contact position of the technical PoC.
    tech_phone : str, optional
        Technical PoC phone number.
    tech_email : str, optional
        Technical PoC email address.
    tech_address : str, optional
        Technical PoC physical address.
    previous_message_id : str, optional
        Identifier for the previous OCM message.
    next_message_id : str, optional
        Identifier for the anticipated next OCM message.
    adm_msg_link : str, optional
        Identifier of linked Attitude Data Message.
    cdm_msg_link : str, optional
        Identifier of linked Conjunction Data Message.
    prm_msg_link : str, optional
        Identifier of linked Pointing Request Message.
    rdm_msg_link : str, optional
        Identifier of linked Reentry Data Message.
    tdm_msg_link : str, optional
        Identifier of linked Tracking Data Message.
    operator : str, optional
        Operator of the space object.
    owner : str, optional
        Owner of the space object.
    country : str, optional
        Country of the owner or operator of the space object.
    constellation : str, optional
        Name of the constellation the space object belongs to.
    object_type : str, optional
        Type of object (PAYLOAD, ROCKET_BODY, DEBRIS, etc.).
    ops_status : str, optional
        Operational status of the space object.
    orbit_category : str, optional
        Orbit category (LEO, GEO, HEO, etc.).
    ocm_data_elements : str, optional
        List of data elements included in the OCM message.
    sclk_offset_at_epoch : float, optional
        Spacecraft clock offset at EPOCH_TZERO (s).
    sclk_sec_per_si_sec : float, optional
        Spacecraft clock scale factor (s/SI-s).
    previous_message_epoch : str, optional
        Epoch of the previous message (ISO 8601).
    next_message_epoch : str, optional
        Anticipated epoch of the next message (ISO 8601).
    start_time : str, optional
        Time of the earliest data in the message (ISO 8601).
    stop_time : str, optional
        Time of the latest data in the message (ISO 8601).
    time_span : float, optional
        Approximate time span covered by the data (d).
    taimutc_at_tzero : float, optional
        TAI minus UTC difference at EPOCH_TZERO (s).
    next_leap_epoch : str, optional
        Epoch of the next leap second (ISO 8601).
    next_leap_taimutc : float, optional
        TAI minus UTC difference at NEXT_LEAP_EPOCH (s).
    ut1mutc_at_tzero : float, optional
        UT1 minus UTC difference at EPOCH_TZERO (s).
    eop_source : str, optional
        Source of Earth Orientation Parameters.
    interp_method_eop : str, optional
        Interpolation method for EOP data.
    celestial_source : str, optional
        Source of celestial body ephemerides.
    comment : list[str], optional
        Comments for the metadata block.

    Attributes
    ----------
    time_system : str
        Time system.
    epoch_tzero : str
        Epoch T-Zero.
        ... (see Parameters for full list)
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
        Link(s) to relevant Attitude Data Message(s).

        Examples: ADM-2023-001
        """
        ...

    @adm_msg_link.setter
    def adm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def alternate_names(self) -> Optional[str]:
        """
        Alternate name(s) of this space object, including assigned names used by spacecraft operator,
        State Actors, commercial SSA providers, and/or media.

        Examples: CALIPSO, 2006-016B
        """
        ...

    @alternate_names.setter
    def alternate_names(self, value: Optional[str]) -> None: ...
    @property
    def catalog_name(self) -> Optional[str]:
        """
        Satellite catalog source (or source agency or operator, value to be drawn from the SANA
        registry list of Space Object Catalogs at <https://sanaregistry.org/r/space_object_catalog>).

        Examples: NORAD, SATCAT
        """
        ...

    @catalog_name.setter
    def catalog_name(self, value: Optional[str]) -> None: ...
    @property
    def cdm_msg_link(self) -> Optional[str]:
        """
        Link(s) to relevant Conjunction Data Message(s).

        Examples: CDM-2023-042
        """
        ...

    @cdm_msg_link.setter
    def cdm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def celestial_source(self) -> Optional[str]:
        """
        Source of celestial body ephemerides.

        Examples: JPL_DE430
        """
        ...

    @celestial_source.setter
    def celestial_source(self, value: Optional[str]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def constellation(self) -> Optional[str]:
        """
        Constellation to which this space object belongs.

        Examples: GALILEO, STARLINK
        """
        ...

    @constellation.setter
    def constellation(self, value: Optional[str]) -> None: ...
    @property
    def country(self) -> Optional[str]:
        """
        Country or country code where the owner is based.

        Examples: FR, USA, JP
        """
        ...

    @country.setter
    def country(self, value: Optional[str]) -> None: ...
    @property
    def eop_source(self) -> Optional[str]:
        """
        Source of Earth Orientation Parameters.

        Examples: IERS_A
        """
        ...

    @eop_source.setter
    def eop_source(self, value: Optional[str]) -> None: ...
    @property
    def epoch_tzero(self) -> str:
        """
        Epoch to which all relative times in the message are referenced. (For format specification,
        see 7.5.10.)

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @epoch_tzero.setter
    def epoch_tzero(self, value: str) -> None: ...
    @property
    def international_designator(self) -> Optional[str]:
        """
        COSPAR international designator for the object. Such designator values shall have the
        following COSPAR format: YYYY-NNNP{PP}, where: YYYY = Year of launch; NNN = Three-digit serial
        number of launch in year YYYY (with leading zeros); P{PP} = At least one capital letter for
        the identification of the part brought into space by the launch. If the object has no
        international designator or the content is either unknown (uncorrelated) or cannot be
        disclosed, the value should be set to UNKNOWN (or this keyword omitted).

        Examples: 2000-052A, 1996-068A, 2000-053A, 1996-008A, UNKNOWN
        """
        ...

    @international_designator.setter
    def international_designator(self, value: Optional[str]) -> None: ...
    @property
    def interp_method_eop(self) -> Optional[str]:
        """
        Interpolation method for EOP data.

        Examples: HERMITE, LINEAR
        """
        ...

    @interp_method_eop.setter
    def interp_method_eop(self, value: Optional[str]) -> None: ...
    @property
    def next_leap_epoch(self) -> Optional[str]:
        """
        Epoch of the next leap second. (See 7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33
        """
        ...

    @next_leap_epoch.setter
    def next_leap_epoch(self, value: Optional[str]) -> None: ...
    @property
    def next_leap_taimutc(self) -> Optional[float]:
        """
        TAI minus UTC difference at NEXT_LEAP_EPOCH.

        Examples: 38.0

        Units: s
        """
        ...

    @next_leap_taimutc.setter
    def next_leap_taimutc(self, value: Optional[float]) -> None: ...
    @property
    def next_message_epoch(self) -> Optional[str]:
        """
        Anticipated epoch of the next message. (See 7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33
        """
        ...

    @next_message_epoch.setter
    def next_message_epoch(self, value: Optional[str]) -> None: ...
    @property
    def next_message_id(self) -> Optional[str]:
        """
        Message ID of the next message from this message originator for this space object.

        Examples: MSG-12346
        """
        ...

    @next_message_id.setter
    def next_message_id(self, value: Optional[str]) -> None: ...
    @property
    def object_designator(self) -> Optional[str]:
        """
        Unique satellite identification designator for the object, as reflected in the catalog whose
        name is ‘CATALOG_NAME’. If the ID is not known (uncorrelated object) or cannot be disclosed,
        ‘UNKNOWN’ may be used (or this keyword omitted).

        Examples: 28893
        """
        ...

    @object_designator.setter
    def object_designator(self, value: Optional[str]) -> None: ...
    @property
    def object_name(self) -> Optional[str]:
        """
        Spacecraft name for which OCM data is provided. While there is no CCSDS-based restriction on
        the value for this keyword, it is recommended to use names from either the UN Office of Outer
        Space Affairs designator index (reference \[3\]), the spacecraft operator, or a State Actor or
        commercial Space Situational Awareness (SSA) provider maintaining the ‘CATALOG_NAME’ space
        catalog. If OBJECT_NAME is not listed in reference \[3\] or the content is either unknown
        (uncorrelated) or cannot be disclosed, the value should be set to UNKNOWN (or this keyword
        omitted).

        Examples: EUTELSAT W1, MARS PATHFINDER, STS 106, NEAR, UNKNOWN
        """
        ...

    @object_name.setter
    def object_name(self, value: Optional[str]) -> None: ...
    @property
    def object_type(self) -> Optional[str]:
        """
        Type of object (value to be drawn from the SANA registry list of Object Descriptions at
        <https://sanaregistry.org/r/object_types>).

        Examples: PAYLOAD, ROCKET BODY, DEBRIS, OTHER
        """
        ...

    @object_type.setter
    def object_type(self, value: Optional[str]) -> None: ...
    @property
    def ocm_data_elements(self) -> Optional[str]:
        """
        List of data elements included in the OCM message.

        Examples: TRAJ, PHYS, COV, MAN, PERT, OD, USER
        """
        ...

    @ocm_data_elements.setter
    def ocm_data_elements(self, value: Optional[str]) -> None: ...
    @property
    def operator(self) -> Optional[str]:
        """
        Spacecraft operator of the space object.

        Examples: SES, INTELSAT
        """
        ...

    @operator.setter
    def operator(self, value: Optional[str]) -> None: ...
    @property
    def ops_status(self) -> Optional[str]:
        """
        Operational status of the space object (value to be drawn from the SANA registry list of
        Operational Status at <https://sanaregistry.org/r/operational_status>).

        Examples: OPERATIONAL, NON-OPERATIONAL
        """
        ...

    @ops_status.setter
    def ops_status(self, value: Optional[str]) -> None: ...
    @property
    def orbit_category(self) -> Optional[str]:
        """
        Orbit category of the space object (value to be drawn from the SANA registry list of Orbit
        Categories at <https://sanaregistry.org/r/orbit_categories>).

        Examples: GEO, LEO
        """
        ...

    @orbit_category.setter
    def orbit_category(self, value: Optional[str]) -> None: ...
    @property
    def originator_address(self) -> Optional[str]:
        """
        Originator’s physical address.

        Examples: 123 Main St, Anytown, USA
        """
        ...

    @originator_address.setter
    def originator_address(self, value: Optional[str]) -> None: ...
    @property
    def originator_email(self) -> Optional[str]:
        """
        Originator PoC email address.

        Examples: john.doe@example.com
        """
        ...

    @originator_email.setter
    def originator_email(self, value: Optional[str]) -> None: ...
    @property
    def originator_phone(self) -> Optional[str]:
        """
        Originator PoC phone number.

        Examples: +1 123-456-7890
        """
        ...

    @originator_phone.setter
    def originator_phone(self, value: Optional[str]) -> None: ...
    @property
    def originator_poc(self) -> Optional[str]:
        """
        Point-of-Contact (PoC) for OCM.

        Examples: John Doe
        """
        ...

    @originator_poc.setter
    def originator_poc(self, value: Optional[str]) -> None: ...
    @property
    def originator_position(self) -> Optional[str]:
        """
        Contact position of the originator PoC.

        Examples: Analyst
        """
        ...

    @originator_position.setter
    def originator_position(self, value: Optional[str]) -> None: ...
    @property
    def owner(self) -> Optional[str]:
        """
        Owner of the space object.

        Examples: Government of France
        """
        ...

    @owner.setter
    def owner(self, value: Optional[str]) -> None: ...
    @property
    def previous_message_epoch(self) -> Optional[str]:
        """
        Epoch of the previous message. (See 7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33
        """
        ...

    @previous_message_epoch.setter
    def previous_message_epoch(self, value: Optional[str]) -> None: ...
    @property
    def previous_message_id(self) -> Optional[str]:
        """
        Message ID of the previous message from this message originator for this space object.

        Examples: MSG-12344
        """
        ...

    @previous_message_id.setter
    def previous_message_id(self, value: Optional[str]) -> None: ...
    @property
    def prm_msg_link(self) -> Optional[str]:
        """
        Link(s) to relevant Pointing Request Message(s).

        Examples: PRM-2023-005
        """
        ...

    @prm_msg_link.setter
    def prm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def rdm_msg_link(self) -> Optional[str]:
        """
        Link(s) to relevant Reentry Data Message(s).

        Examples: RDM-2023-010
        """
        ...

    @rdm_msg_link.setter
    def rdm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def sclk_offset_at_epoch(self) -> Optional[float]:
        """
        Spacecraft clock offset at EPOCH_TZERO.

        Examples: 0.0

        Units: s
        """
        ...

    @sclk_offset_at_epoch.setter
    def sclk_offset_at_epoch(self, value: Optional[float]) -> None: ...
    @property
    def sclk_sec_per_si_sec(self) -> Optional[float]:
        """
        Spacecraft clock scale factor.

        Examples: 1.0

        Units: s/SI-s
        """
        ...

    @sclk_sec_per_si_sec.setter
    def sclk_sec_per_si_sec(self, value: Optional[float]) -> None: ...
    @property
    def start_time(self) -> Optional[str]:
        """
        Time of the earliest data in the message. (See 7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33
        """
        ...

    @start_time.setter
    def start_time(self, value: Optional[str]) -> None: ...
    @property
    def stop_time(self) -> Optional[str]:
        """
        Time of the latest data in the message. (See 7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33
        """
        ...

    @stop_time.setter
    def stop_time(self, value: Optional[str]) -> None: ...
    @property
    def taimutc_at_tzero(self) -> Optional[float]:
        """
        TAI minus UTC difference at EPOCH_TZERO.

        Examples: 37.0

        Units: s
        """
        ...

    @taimutc_at_tzero.setter
    def taimutc_at_tzero(self, value: Optional[float]) -> None: ...
    @property
    def tdm_msg_link(self) -> Optional[str]:
        """
        Link(s) to relevant Tracking Data Message(s).

        Examples: TDM-2023-111
        """
        ...

    @tdm_msg_link.setter
    def tdm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def tech_address(self) -> Optional[str]:
        """
        Physical address information for OCM creator.

        Examples: 456 Tech Park, Sometown, USA
        """
        ...

    @tech_address.setter
    def tech_address(self, value: Optional[str]) -> None: ...
    @property
    def tech_email(self) -> Optional[str]:
        """
        Technical PoC email address.

        Examples: jane.smith@example.com
        """
        ...

    @tech_email.setter
    def tech_email(self, value: Optional[str]) -> None: ...
    @property
    def tech_org(self) -> Optional[str]:
        """
        Creating agency or operator (value should be drawn from the ‘Abbreviation’ column of the SANA
        Organizations registry at <https://www.sanaregistry.org/r/organizations>).

        Examples: NASA, ESA, JAXA
        """
        ...

    @tech_org.setter
    def tech_org(self, value: Optional[str]) -> None: ...
    @property
    def tech_phone(self) -> Optional[str]:
        """
        Technical PoC phone number.

        Examples: +1 987-654-3210
        """
        ...

    @tech_phone.setter
    def tech_phone(self, value: Optional[str]) -> None: ...
    @property
    def tech_poc(self) -> Optional[str]:
        """
        Technical PoC for OCM.

        Examples: Jane Smith
        """
        ...

    @tech_poc.setter
    def tech_poc(self, value: Optional[str]) -> None: ...
    @property
    def tech_position(self) -> Optional[str]:
        """
        Contact position of the technical PoC.

        Examples: Engineer
        """
        ...

    @tech_position.setter
    def tech_position(self, value: Optional[str]) -> None: ...
    @property
    def time_span(self) -> Optional[float]:
        """
        Approximate time span covered by the data in the message.

        Examples: 0.1

        Units: d
        """
        ...

    @time_span.setter
    def time_span(self, value: Optional[float]) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system used for all absolute time stamps in the message (e.g., UTC, TAI).

        Examples: UTC, TAI
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...
    @property
    def ut1mutc_at_tzero(self) -> Optional[float]:
        """
        UT1 minus UTC difference at EPOCH_TZERO.

        Examples: 0.3

        Units: s
        """
        ...

    @ut1mutc_at_tzero.setter
    def ut1mutc_at_tzero(self, value: Optional[float]) -> None: ...

class OcmOdParameters:
    """
    OCM Orbit Determination Parameters.

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
        Actual time span in days used for the OD of the object.

        Examples: 4.8

        Units: d
        """
        ...

    @actual_od_span.setter
    def actual_od_span(self, value: Optional[float]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def consider_n(self) -> Optional[int]:
        """
        The number of consider parameters used in the orbit determination.

        Examples: 3
        """
        ...

    @consider_n.setter
    def consider_n(self, value: Optional[int]) -> None: ...
    @property
    def consider_params(self) -> Optional[str]:
        """
        Free-text comma-delimited description of the consider parameters used in the orbit
        determination.

        Examples: DRAG_COEFF, SRP_COEFF
        """
        ...

    @consider_params.setter
    def consider_params(self, value: Optional[str]) -> None: ...
    @property
    def data_types(self) -> Optional[str]:
        """
        Comma-separated list of observation data types utilized in this orbit determination.

        Examples: RANGE, DOPPLER
        """
        ...

    @data_types.setter
    def data_types(self, value: Optional[str]) -> None: ...
    @property
    def days_since_first_obs(self) -> Optional[float]:
        """
        Days elapsed between first accepted observation and OD_EPOCH.

        Examples: 1.5

        Units: d
        """
        ...

    @days_since_first_obs.setter
    def days_since_first_obs(self, value: Optional[float]) -> None: ...
    @property
    def days_since_last_obs(self) -> Optional[float]:
        """
        Days elapsed between last accepted observation and OD_EPOCH.

        Examples: 0.1

        Units: d
        """
        ...

    @days_since_last_obs.setter
    def days_since_last_obs(self, value: Optional[float]) -> None: ...
    @property
    def gdop(self) -> Optional[float]:
        """
        Generalized Dilution Of Precision for this orbit determination.

        Examples: 1.5
        """
        ...

    @gdop.setter
    def gdop(self, value: Optional[float]) -> None: ...
    @property
    def maximum_obs_gap(self) -> Optional[float]:
        """
        The maximum time between observations in the OD of the object.

        Examples: 0.5

        Units: d
        """
        ...

    @maximum_obs_gap.setter
    def maximum_obs_gap(self, value: Optional[float]) -> None: ...
    @property
    def obs_available(self) -> Optional[int]:
        """
        The number of observations available within the actual OD time span.

        Examples: 100
        """
        ...

    @obs_available.setter
    def obs_available(self, value: Optional[int]) -> None: ...
    @property
    def obs_used(self) -> Optional[int]:
        """
        The number of observations accepted within the actual OD time span.

        Examples: 95
        """
        ...

    @obs_used.setter
    def obs_used(self, value: Optional[int]) -> None: ...
    @property
    def od_confidence(self) -> Optional[float]:
        """
        OD confidence metric, which spans 0 to 100% (useful only for Filter-based OD systems).

        Examples: 99.0

        Units: %
        """
        ...

    @od_confidence.setter
    def od_confidence(self, value: Optional[float]) -> None: ...
    @property
    def od_epoch(self) -> str:
        """
        Relative or absolute time tag of the orbit determination solved-for state in the selected OCM
        time system recorded by the TIME_SYSTEM keyword.

        Examples: 2000-01-01T12:00:00Z
        """
        ...

    @od_epoch.setter
    def od_epoch(self, value: str) -> None: ...
    @property
    def od_epoch_eigint(self) -> Optional[float]:
        """
        Positional error ellipsoid 1σ intermediate eigenvalue at the epoch of the OD.

        Examples: 50.0

        Units: m
        """
        ...

    @od_epoch_eigint.setter
    def od_epoch_eigint(self, value: Optional[float]) -> None: ...
    @property
    def od_epoch_eigmaj(self) -> Optional[float]:
        """
        Positional error ellipsoid 1 sigma (1σ) major eigenvalue at the epoch of the OD.

        Examples: 100.0

        Units: m
        """
        ...

    @od_epoch_eigmaj.setter
    def od_epoch_eigmaj(self, value: Optional[float]) -> None: ...
    @property
    def od_epoch_eigmin(self) -> Optional[float]:
        """
        Positional error ellipsoid 1σ minor eigenvalue at the epoch of the OD.

        Examples: 20.0

        Units: m
        """
        ...

    @od_epoch_eigmin.setter
    def od_epoch_eigmin(self, value: Optional[float]) -> None: ...
    @property
    def od_id(self) -> str:
        """
        Identification number for this orbit determination.

        Examples: 1
        """
        ...

    @od_id.setter
    def od_id(self, value: str) -> None: ...
    @property
    def od_max_pred_eigmaj(self) -> Optional[float]:
        """
        The resulting maximum predicted major eigenvalue of the 1σ positional error ellipsoid over
        the entire TIME_SPAN of the OCM, stemming from this OD.

        Examples: 500.0

        Units: m
        """
        ...

    @od_max_pred_eigmaj.setter
    def od_max_pred_eigmaj(self, value: Optional[float]) -> None: ...
    @property
    def od_method(self) -> str:
        """
        Type of orbit determination method used to produce the orbit estimate.

        Examples: LEAST_SQUARES, KALMAN_FILTER
        """
        ...

    @od_method.setter
    def od_method(self, value: str) -> None: ...
    @property
    def od_min_pred_eigmin(self) -> Optional[float]:
        """
        The resulting minimum predicted minor eigenvalue of the 1σ positional error ellipsoid over
        the entire TIME_SPAN of the OCM, stemming from this OD.

        Examples: 10.0

        Units: m
        """
        ...

    @od_min_pred_eigmin.setter
    def od_min_pred_eigmin(self, value: Optional[float]) -> None: ...
    @property
    def od_prev_id(self) -> Optional[str]:
        """
        Optional identification number for the previous orbit determination.

        Examples: 0
        """
        ...

    @od_prev_id.setter
    def od_prev_id(self, value: Optional[str]) -> None: ...
    @property
    def recommended_od_span(self) -> Optional[float]:
        """
        Number of days of observations recommended for the OD of the object (useful only for Batch OD
        systems).

        Examples: 5.0

        Units: d
        """
        ...

    @recommended_od_span.setter
    def recommended_od_span(self, value: Optional[float]) -> None: ...
    @property
    def sedr(self) -> Optional[float]:
        """
        The Specific Energy Dissipation Rate, which is the amount of energy being removed from the
        object's orbit by the non-conservative forces.

        Examples: 1.25e-7

        Units: W/kg
        """
        ...

    @sedr.setter
    def sedr(self, value: Optional[float]) -> None: ...
    @property
    def sensors(self) -> Optional[str]:
        """
        Free-text comma-delimited description of the sensors used in the orbit determination.

        Examples: SENSOR1, SENSOR2
        """
        ...

    @sensors.setter
    def sensors(self, value: Optional[str]) -> None: ...
    @property
    def sensors_n(self) -> Optional[int]:
        """
        The number of sensors used in the orbit determination.

        Examples: 5
        """
        ...

    @sensors_n.setter
    def sensors_n(self, value: Optional[int]) -> None: ...
    @property
    def solve_n(self) -> Optional[int]:
        """
        The number of solve-for states in the orbit determination.

        Examples: 6
        """
        ...

    @solve_n.setter
    def solve_n(self, value: Optional[int]) -> None: ...
    @property
    def solve_states(self) -> Optional[str]:
        """
        Free-text comma-delimited description of the state elements solved for in the orbit
        determination.

        Examples: X, Y, Z, X_DOT, Y_DOT, Z_DOT
        """
        ...

    @solve_states.setter
    def solve_states(self, value: Optional[str]) -> None: ...
    @property
    def tracks_available(self) -> Optional[int]:
        """
        The number of sensor tracks available for the OD within the actual time span (see definition
        of ‘tracks’, 1.5.2).

        Examples: 10
        """
        ...

    @tracks_available.setter
    def tracks_available(self, value: Optional[int]) -> None: ...
    @property
    def tracks_used(self) -> Optional[int]:
        """
        The number of sensor tracks accepted for the OD within the actual time span (see definition of
        ‘tracks’, 1.5.2).

        Examples: 9
        """
        ...

    @tracks_used.setter
    def tracks_used(self, value: Optional[int]) -> None: ...
    @property
    def weighted_rms(self) -> Optional[float]:
        """
        (Useful/valid only for Batch OD systems.) The weighted RMS residual ratio.

        Examples: 0.95
        """
        ...

    @weighted_rms.setter
    def weighted_rms(self, value: Optional[float]) -> None: ...

class OcmPerturbations:
    """
    OCM Perturbations Parameters.

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
        Size of the albedo grid.

        Examples: 10
        """
        ...

    @albedo_grid_size.setter
    def albedo_grid_size(self, value: Optional[int]) -> None: ...
    @property
    def albedo_model(self) -> Optional[str]:
        """
        Name of the albedo model.

        Examples: EARTH_ALBEDO
        """
        ...

    @albedo_model.setter
    def albedo_model(self, value: Optional[str]) -> None: ...
    @property
    def atmospheric_model(self) -> Optional[str]:
        """
        Name of the atmospheric model (value to be drawn from the SANA registry list of Atmospheric
        Models at https://sanaregistry.org/r/atmospheric_model).

        Examples: JB2008, MSISE00
        """
        ...

    @atmospheric_model.setter
    def atmospheric_model(self, value: Optional[str]) -> None: ...
    @property
    def central_body_rotation(self) -> Optional[float]:
        """
        Central body angular rotation rate.

        Examples: 0.00417807462

        Units: deg/s
        """
        ...

    @central_body_rotation.setter
    def central_body_rotation(self, value: Optional[float]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def equatorial_radius(self) -> Optional[float]:
        """
        Equatorial radius of the central body.

        Examples: 6378137.0

        Units: m
        """
        ...

    @equatorial_radius.setter
    def equatorial_radius(self, value: Optional[float]) -> None: ...
    @property
    def fixed_f10p7(self) -> Optional[float]:
        """
        Fixed F10.7 solar flux.

        Examples: 150.0

        Units: SFU
        """
        ...

    @fixed_f10p7.setter
    def fixed_f10p7(self, value: Optional[float]) -> None: ...
    @property
    def fixed_f10p7_mean(self) -> Optional[float]:
        """
        Fixed 81-day average F10.7 solar flux.

        Examples: 140.0

        Units: SFU
        """
        ...

    @fixed_f10p7_mean.setter
    def fixed_f10p7_mean(self, value: Optional[float]) -> None: ...
    @property
    def fixed_geomag_ap(self) -> Optional[float]:
        """
        Fixed geomagnetic Ap index.

        Examples: 15.0
        """
        ...

    @fixed_geomag_ap.setter
    def fixed_geomag_ap(self, value: Optional[float]) -> None: ...
    @property
    def fixed_geomag_dst(self) -> Optional[float]:
        """
        Fixed geomagnetic Dst index.

        Examples: -20.0
        """
        ...

    @fixed_geomag_dst.setter
    def fixed_geomag_dst(self, value: Optional[float]) -> None: ...
    @property
    def fixed_geomag_kp(self) -> Optional[float]:
        """
        Fixed geomagnetic Kp index.

        Examples: 3.0
        """
        ...

    @fixed_geomag_kp.setter
    def fixed_geomag_kp(self, value: Optional[float]) -> None: ...
    @property
    def fixed_m10p7(self) -> Optional[float]:
        """
        Fixed M10.7 solar flux.

        Examples: 130.0

        Units: SFU
        """
        ...

    @fixed_m10p7.setter
    def fixed_m10p7(self, value: Optional[float]) -> None: ...
    @property
    def fixed_m10p7_mean(self) -> Optional[float]:
        """
        Fixed 81-day average M10.7 solar flux.

        Examples: 120.0

        Units: SFU
        """
        ...

    @fixed_m10p7_mean.setter
    def fixed_m10p7_mean(self, value: Optional[float]) -> None: ...
    @property
    def fixed_s10p7(self) -> Optional[float]:
        """
        Fixed S10.7 solar flux.

        Examples: 110.0

        Units: SFU
        """
        ...

    @fixed_s10p7.setter
    def fixed_s10p7(self, value: Optional[float]) -> None: ...
    @property
    def fixed_s10p7_mean(self) -> Optional[float]:
        """
        Fixed 81-day average S10.7 solar flux.

        Examples: 100.0

        Units: SFU
        """
        ...

    @fixed_s10p7_mean.setter
    def fixed_s10p7_mean(self, value: Optional[float]) -> None: ...
    @property
    def fixed_y10p7(self) -> Optional[float]:
        """
        Fixed Y10.7 solar flux.

        Examples: 90.0

        Units: SFU
        """
        ...

    @fixed_y10p7.setter
    def fixed_y10p7(self, value: Optional[float]) -> None: ...
    @property
    def fixed_y10p7_mean(self) -> Optional[float]:
        """
        Fixed 81-day average Y10.7 solar flux.

        Examples: 85.0

        Units: SFU
        """
        ...

    @fixed_y10p7_mean.setter
    def fixed_y10p7_mean(self, value: Optional[float]) -> None: ...
    @property
    def gm(self) -> Optional[float]:
        """
        Gravitational coefficient of the central body.

        Examples: 398600.4418

        Units: km³/s²
        """
        ...

    @gm.setter
    def gm(self, value: Optional[float]) -> None: ...
    @property
    def gravity_model(self) -> Optional[str]:
        """
        Name of the gravity model (value to be drawn from the SANA registry list of Gravitational
        Models at https://sanaregistry.org/r/gravity_model).

        Examples: EGM96, EGM2008
        """
        ...

    @gravity_model.setter
    def gravity_model(self, value: Optional[str]) -> None: ...
    @property
    def n_body_perturbations(self) -> Optional[str]:
        """
        List of N-body perturbations included (value(s) to be drawn from the SANA registry list of
        Common Central Body Names at https://sanaregistry.org/r/central_body_name).

        Examples: MOON, SUN
        """
        ...

    @n_body_perturbations.setter
    def n_body_perturbations(self, value: Optional[str]) -> None: ...
    @property
    def oblate_flattening(self) -> Optional[float]:
        """
        Oblate flattening of the central body.

        Examples: 0.00335281
        """
        ...

    @oblate_flattening.setter
    def oblate_flattening(self, value: Optional[float]) -> None: ...
    @property
    def ocean_tides_model(self) -> Optional[str]:
        """
        Name of the ocean tides model (value to be drawn from the SANA registry list of Ocean Tides
        Models at https://sanaregistry.org/r/ocean_tides_model).

        Examples: FES2004
        """
        ...

    @ocean_tides_model.setter
    def ocean_tides_model(self, value: Optional[str]) -> None: ...
    @property
    def reduction_theory(self) -> Optional[str]:
        """
        Specification of the reduction theory used for precession and nutation modeling. This is a
        free-text field, so if the examples on the right are insufficient, others may be used.

        Examples: IAU1976/FK5, IAU2010
        """
        ...

    @reduction_theory.setter
    def reduction_theory(self, value: Optional[str]) -> None: ...
    @property
    def shadow_bodies(self) -> Optional[str]:
        """
        List of bodies included in shadow calculations (value(s) to be drawn from the SANA registry
        list of Orbit Centers at <https://sanaregistry.org/r/orbit_centers>).

        Examples: EARTH, MOON
        """
        ...

    @shadow_bodies.setter
    def shadow_bodies(self, value: Optional[str]) -> None: ...
    @property
    def shadow_model(self) -> Optional[str]:
        """
        Examples: NONE, CONE, DUAL_CONE, CYLINDRICAL
        """
        ...

    @shadow_model.setter
    def shadow_model(self, value: Optional[str]) -> None: ...
    @property
    def solid_tides_model(self) -> Optional[str]:
        """
        Name of solid tides model (optionally specify order or constituent effects, diurnal,
        semi-diurnal, etc.).

        Examples: DIURNAL, SEMI-DIURNAL
        """
        ...

    @solid_tides_model.setter
    def solid_tides_model(self, value: Optional[str]) -> None: ...
    @property
    def srp_model(self) -> Optional[str]:
        """
        Name of the Solar Radiation Pressure (SRP) model.

        Examples: CANNONBALL, FLAT_PLATE, BOX_WING
        """
        ...

    @srp_model.setter
    def srp_model(self, value: Optional[str]) -> None: ...
    @property
    def sw_data_epoch(self) -> Optional[str]:
        """
        Epoch of the space weather data.

        Examples: 2000-01-01T12:00:00Z
        """
        ...

    @sw_data_epoch.setter
    def sw_data_epoch(self, value: Optional[str]) -> None: ...
    @property
    def sw_data_source(self) -> Optional[str]:
        """
        Space weather data source.

        Examples: NOAA
        """
        ...

    @sw_data_source.setter
    def sw_data_source(self, value: Optional[str]) -> None: ...
    @property
    def sw_interp_method(self) -> Optional[str]:
        """
        Free-text field specifying the method used to select or interpolate any and all sequential
        space weather data (Kp, ap, Dst, F10.7, M10.7, S10.7, Y10.7, etc.). While not constrained to
        specific entries, it is anticipated that the utilized method would match methods detailed in
        numerical analysis textbooks.

        Examples: PRECEDING_VALUE, NEAREST_NEIGHBOR, LINEAR, LAGRANGE_ORDER_5
        """
        ...

    @sw_interp_method.setter
    def sw_interp_method(self, value: Optional[str]) -> None: ...

class OcmPhysicalDescription:
    """
    Space Object Physical Characteristics.

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
        Attitude-dependent cross-sectional area of space object (not already included in
        DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along intermediate OEB (Ŷoeb) direction as
        defined in annex F.

        Examples: 20.0

        Units: m²
        """
        ...

    @area_along_oeb_int.setter
    def area_along_oeb_int(self, value: Optional[float]) -> None: ...
    @property
    def area_along_oeb_max(self) -> Optional[float]:
        """
        Attitude-dependent cross-sectional area of space object (not already included in
        DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along max OEB (Xoeb) direction as defined in
        annex F.

        Examples: 10.0

        Units: m²
        """
        ...

    @area_along_oeb_max.setter
    def area_along_oeb_max(self, value: Optional[float]) -> None: ...
    @property
    def area_along_oeb_min(self) -> Optional[float]:
        """
        Attitude-dependent cross-sectional area of space object (not already included in
        DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along minimum OEB (Ẑoeb) direction as defined
        in annex F.

        Examples: 50.0

        Units: m²
        """
        ...

    @area_along_oeb_min.setter
    def area_along_oeb_min(self, value: Optional[float]) -> None: ...
    @property
    def area_max_for_pc(self) -> Optional[float]:
        """
        Maximum cross-sectional area for collision probability estimation purposes.

        Examples: 50.0

        Units: m²
        """
        ...

    @area_max_for_pc.setter
    def area_max_for_pc(self, value: Optional[float]) -> None: ...
    @property
    def area_min_for_pc(self) -> Optional[float]:
        """
        Minimum cross-sectional area for collision probability estimation purposes.

        Examples: 5.0

        Units: m²
        """
        ...

    @area_min_for_pc.setter
    def area_min_for_pc(self, value: Optional[float]) -> None: ...
    @property
    def area_typ_for_pc(self) -> Optional[float]:
        """
        Typical (50th percentile) cross-sectional area sampled over all space object orientations for
        collision probability estimation purposes.

        Examples: 15.0

        Units: m²
        """
        ...

    @area_typ_for_pc.setter
    def area_typ_for_pc(self, value: Optional[float]) -> None: ...
    @property
    def att_actuator_type(self) -> Optional[str]:
        """
        Free-text specification of type of actuator for attitude control.

        Examples: ATT_THRUSTERS, ACTIVE_MAG_TORQUE, PASSIVE_MAG_TORQUE, REACTION_WHEELS,
        MOMENTUM_WHEELS, CONTROL_MOMENT_GYROSCOPE, NONE, OTHER
        """
        ...

    @att_actuator_type.setter
    def att_actuator_type(self, value: Optional[str]) -> None: ...
    @property
    def att_control(self) -> Optional[float]:
        """
        Accuracy of attitude control system (ACS) to maintain attitude, assuming attitude knowledge
        was perfect (i.e., deadbands).

        Examples: 0.1

        Units: deg
        """
        ...

    @att_control.setter
    def att_control(self, value: Optional[float]) -> None: ...
    @property
    def att_control_mode(self) -> Optional[str]:
        """
        Free-text specification of primary mode of attitude control for the space object.

        Examples: THREE_AXIS, SPIN, DUAL_SPIN, TUMBLING, GRAVITY_GRADIENT
        """
        ...

    @att_control_mode.setter
    def att_control_mode(self, value: Optional[str]) -> None: ...
    @property
    def att_knowledge(self) -> Optional[float]:
        """
        Accuracy of attitude knowledge.

        Examples: 0.01

        Units: deg
        """
        ...

    @att_knowledge.setter
    def att_knowledge(self, value: Optional[float]) -> None: ...
    @property
    def att_pointing(self) -> Optional[float]:
        """
        Overall accuracy of spacecraft to maintain attitude, including attitude knowledge errors and
        ACS operation.

        Examples: 0.5

        Units: deg
        """
        ...

    @att_pointing.setter
    def att_pointing(self, value: Optional[float]) -> None: ...
    @property
    def avg_maneuver_freq(self) -> Optional[float]:
        """
        Average maneuver frequency, measured in the number of orbit- or attitude-adjust maneuvers per
        year.

        Examples: 52.0

        Units: #/yr
        """
        ...

    @avg_maneuver_freq.setter
    def avg_maneuver_freq(self, value: Optional[float]) -> None: ...
    @property
    def bus_model(self) -> Optional[str]:
        """
        Free-text field containing the satellite manufacturer’s spacecraft bus model name.

        Examples: LS-1300, A2100
        """
        ...

    @bus_model.setter
    def bus_model(self, value: Optional[str]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def docked_with(self) -> Optional[str]:
        """
        Free-text field containing a comma-separated list of other space objects that this object is
        docked to.

        Examples: 2021-098A, 2021-098B
        """
        ...

    @docked_with.setter
    def docked_with(self, value: Optional[str]) -> None: ...
    @property
    def drag_coeff_nom(self) -> Optional[float]:
        """
        Nominal drag Coefficient (CD NOM). If the atmospheric drag coefficient, CD, is set to zero, no
        atmospheric drag shall be considered.

        Examples: 2.2
        """
        ...

    @drag_coeff_nom.setter
    def drag_coeff_nom(self, value: Optional[float]) -> None: ...
    @property
    def drag_const_area(self) -> Optional[float]:
        """
        Attitude-independent drag cross-sectional area (AD) facing the relative wind vector, not
        already incorporated into the attitude-dependent ‘AREA_ALONG_OEB’ parameters.

        Examples: 2.0

        Units: m²
        """
        ...

    @drag_const_area.setter
    def drag_const_area(self, value: Optional[float]) -> None: ...
    @property
    def drag_uncertainty(self) -> Optional[float]:
        """
        Drag coefficient one sigma (1σ) percent uncertainty, where the actual range of drag
        coefficients to within 1σ shall be obtained from \[1.0 ± 0.01*DRAG_UNCERTAINTY\] (CD NOM). This
        factor is intended to allow operators to supply the nominal ballistic coefficient components
        while accommodating ballistic coefficient uncertainties.

        Examples: 5.0

        Units: %
        """
        ...

    @drag_uncertainty.setter
    def drag_uncertainty(self, value: Optional[float]) -> None: ...
    @property
    def dry_mass(self) -> Optional[float]:
        """
        Space object dry mass (without propellant).

        Examples: 500.0

        Units: kg
        """
        ...

    @dry_mass.setter
    def dry_mass(self, value: Optional[float]) -> None: ...
    @property
    def dv_bol(self) -> Optional[float]:
        """
        Total ΔV capability of the spacecraft at beginning of life.

        Examples: 2.0

        Units: km/s
        """
        ...

    @dv_bol.setter
    def dv_bol(self, value: Optional[float]) -> None: ...
    @property
    def dv_remaining(self) -> Optional[float]:
        """
        Total ΔV remaining for the spacecraft.

        Examples: 1.5

        Units: km/s
        """
        ...

    @dv_remaining.setter
    def dv_remaining(self, value: Optional[float]) -> None: ...
    @property
    def initial_wet_mass(self) -> Optional[float]:
        """
        Space object total mass at beginning of life.

        Examples: 1000.0

        Units: kg
        """
        ...

    @initial_wet_mass.setter
    def initial_wet_mass(self, value: Optional[float]) -> None: ...
    @property
    def ixx(self) -> Optional[float]:
        """
        Moment of Inertia about the X-axis of the space object’s primary body frame.

        Examples: 100.0

        Units: kg·m²
        """
        ...

    @ixx.setter
    def ixx(self, value: Optional[float]) -> None: ...
    @property
    def ixy(self) -> Optional[float]:
        """
        Inertia Cross Product of the X & Y axes.

        Examples: 1.0

        Units: kg·m²
        """
        ...

    @ixy.setter
    def ixy(self, value: Optional[float]) -> None: ...
    @property
    def ixz(self) -> Optional[float]:
        """
        Inertia Cross Product of the X & Z axes.

        Examples: 2.0

        Units: kg·m²
        """
        ...

    @ixz.setter
    def ixz(self, value: Optional[float]) -> None: ...
    @property
    def iyy(self) -> Optional[float]:
        """
        Moment of Inertia about the Y-axis.

        Examples: 200.0

        Units: kg·m²
        """
        ...

    @iyy.setter
    def iyy(self, value: Optional[float]) -> None: ...
    @property
    def iyz(self) -> Optional[float]:
        """
        Inertia Cross Product of the Y & Z axes.

        Examples: 3.0

        Units: kg·m²
        """
        ...

    @iyz.setter
    def iyz(self, value: Optional[float]) -> None: ...
    @property
    def izz(self) -> Optional[float]:
        """
        Moment of Inertia about the Z-axis.

        Examples: 300.0

        Units: kg·m²
        """
        ...

    @izz.setter
    def izz(self, value: Optional[float]) -> None: ...
    @property
    def manufacturer(self) -> Optional[str]:
        """
        Free-text field containing the satellite manufacturer’s name.

        Examples: Boeing, Lockheed Martin
        """
        ...

    @manufacturer.setter
    def manufacturer(self, value: Optional[str]) -> None: ...
    @property
    def max_thrust(self) -> Optional[float]:
        """
        Maximum composite thrust the spacecraft can accomplish in any single body-fixed direction.

        Examples: 100.0

        Units: N
        """
        ...

    @max_thrust.setter
    def max_thrust(self, value: Optional[float]) -> None: ...
    @property
    def oeb_int(self) -> Optional[float]:
        """
        Intermediate physical dimension (along Ŷoeb) of OEB normal to OEB_MAX direction.

        Examples: 5.0

        Units: m
        """
        ...

    @oeb_int.setter
    def oeb_int(self, value: Optional[float]) -> None: ...
    @property
    def oeb_max(self) -> Optional[float]:
        """
        Maximum physical dimension (along Xoeb) of the OEB.

        Examples: 10.0

        Units: m
        """
        ...

    @oeb_max.setter
    def oeb_max(self, value: Optional[float]) -> None: ...
    @property
    def oeb_min(self) -> Optional[float]:
        """
        Minimum physical dimension (along Ẑoeb) of OEB in direction normal to both OEB_MAX and OEB_INT
        directions.

        Examples: 2.0

        Units: m
        """
        ...

    @oeb_min.setter
    def oeb_min(self, value: Optional[float]) -> None: ...
    @property
    def oeb_parent_frame(self) -> Optional[str]:
        """
        Parent reference frame that maps to the OEB frame via the quaternion-based transformation
        defined in annex F, subsection F1. Select from the accepted set of values indicated in annex
        B, subsections B4 and B5. This keyword shall be provided if OEB_Q1,2,3,qc are specified.

        Examples: ICRF, EME2000
        """
        ...

    @oeb_parent_frame.setter
    def oeb_parent_frame(self, value: Optional[str]) -> None: ...
    @property
    def oeb_parent_frame_epoch(self) -> Optional[str]:
        """
        Epoch of the OEB parent frame, if OEB_PARENT_FRAME is provided and its epoch is not intrinsic
        to the definition of the reference frame.

        Examples: 2000-01-01T12:00:00Z
        """
        ...

    @oeb_parent_frame_epoch.setter
    def oeb_parent_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def oeb_q1(self) -> Optional[float]:
        """
        q1 = e1 * sin(φ/2), where per reference [H1], φ = Euler rotation angle and e1 = 1st component
        of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME (defined above) to
        the frame aligned with the OEB (defined in annex F, subsection F1). A value of ‘-999’ denotes
        a tumbling space object.

        Examples: 0.0
        """
        ...

    @oeb_q1.setter
    def oeb_q1(self, value: Optional[float]) -> None: ...
    @property
    def oeb_q2(self) -> Optional[float]:
        """
        q2 = e2 * sin(φ/2), where per reference [H1], φ = Euler rotation angle and e2 = 2nd component
        of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME (defined above) to
        the frame aligned with the Optimally Encompassing Box (defined in annex F, subsection F1). A
        value of ‘-999’ denotes a tumbling space object.

        Examples: 0.0
        """
        ...

    @oeb_q2.setter
    def oeb_q2(self, value: Optional[float]) -> None: ...
    @property
    def oeb_q3(self) -> Optional[float]:
        """
        q3 = e3 * sin(φ/2), where per reference [H1], φ = Euler rotation angle and e3 = 3rd component
        of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME (defined above) to
        the frame aligned with the Optimally Encompassing Box (defined in annex F, subsection F1). A
        value of ‘-999’ denotes a tumbling space object.

        Examples: 0.0
        """
        ...

    @oeb_q3.setter
    def oeb_q3(self, value: Optional[float]) -> None: ...
    @property
    def oeb_qc(self) -> Optional[float]:
        """
        qc = cos(φ/2), where per reference [H1], φ = the Euler rotation angle for the rotation that
        maps from the OEB_PARENT_FRAME (defined above) to the frame aligned with the Optimally
        Encompassing Box (annex F, subsection F1). qc shall be made non-negative by convention. A
        value of ‘-999’ denotes a tumbling space object.

        Examples: 1.0
        """
        ...

    @oeb_qc.setter
    def oeb_qc(self, value: Optional[float]) -> None: ...
    @property
    def rcs(self) -> Optional[float]:
        """
        Typical (50th percentile) effective Radar Cross Section of the space object sampled over all
        possible viewing angles.

        Examples: 10.0

        Units: m²
        """
        ...

    @rcs.setter
    def rcs(self, value: Optional[float]) -> None: ...
    @property
    def rcs_max(self) -> Optional[float]:
        """
        Maximum Radar Cross Section observed for this object.

        Examples: 100.0

        Units: m²
        """
        ...

    @rcs_max.setter
    def rcs_max(self, value: Optional[float]) -> None: ...
    @property
    def rcs_min(self) -> Optional[float]:
        """
        Minimum Radar Cross Section observed for this object.

        Examples: 1.0

        Units: m²
        """
        ...

    @rcs_min.setter
    def rcs_min(self, value: Optional[float]) -> None: ...
    @property
    def reflectance(self) -> Optional[float]:
        """
        Typical (50th percentile) coefficient of REFLECTANCE of the space object over all possible
        viewing angles, ranging from 0 (none) to 1 (perfect reflectance).

        Examples: 0.2
        """
        ...

    @reflectance.setter
    def reflectance(self, value: Optional[float]) -> None: ...
    @property
    def solar_rad_coeff(self) -> Optional[float]:
        """
        Nominal Solar Radiation Pressure Coefficient (CR NOM). If the solar radiation coefficient, CR,
        is set to zero, no solar radiation pressure shall be considered.

        Examples: 1.2
        """
        ...

    @solar_rad_coeff.setter
    def solar_rad_coeff(self, value: Optional[float]) -> None: ...
    @property
    def solar_rad_uncertainty(self) -> Optional[float]:
        """
        SRP one sigma (1σ) percent uncertainty, where the actual range of SRP coefficients to within
        1σ shall be obtained from \[1.0 ± 0.01*SRP_UNCERTAINTY\] (CR NOM). This factor is intended to
        allow operators to supply the nominal ballistic coefficient components while accommodating
        ballistic coefficient uncertainties.

        Examples: 10.0

        Units: %
        """
        ...

    @solar_rad_uncertainty.setter
    def solar_rad_uncertainty(self, value: Optional[float]) -> None: ...
    @property
    def srp_const_area(self) -> Optional[float]:
        """
        Attitude-independent solar radiation pressure cross-sectional area (AR) facing the Sun, not
        already incorporated into the attitude-dependent ‘AREA_ALONG_OEB’ parameters.

        Examples: 5.0

        Units: m²
        """
        ...

    @srp_const_area.setter
    def srp_const_area(self, value: Optional[float]) -> None: ...
    @property
    def vm_absolute(self) -> Optional[float]:
        """
        Typical (50th percentile) absolute Visual Magnitude of the space object sampled over all
        possible viewing angles and ‘normalized’ as specified in informative annex F, subsection F2 to
        a 1 AU Sun-to-target distance, a phase angle of 0°, and a 40,000 km target-to-sensor distance.

        Examples: 4.5
        """
        ...

    @vm_absolute.setter
    def vm_absolute(self, value: Optional[float]) -> None: ...
    @property
    def vm_apparent(self) -> Optional[float]:
        """
        Typical (50th percentile) apparent Visual Magnitude observed for this space object.

        Examples: 12.0
        """
        ...

    @vm_apparent.setter
    def vm_apparent(self, value: Optional[float]) -> None: ...
    @property
    def vm_apparent_max(self) -> Optional[float]:
        """
        Maximum apparent Visual Magnitude observed for this space object. The ‘MAX’ value represents
        the dimmest observation, which associates with a higher Vmag.

        Examples: 18.0
        """
        ...

    @vm_apparent_max.setter
    def vm_apparent_max(self, value: Optional[float]) -> None: ...
    @property
    def vm_apparent_min(self) -> Optional[float]:
        """
        Minimum apparent Visual Magnitude observed for this space object. The ‘MIN’ value represents
        the brightest observation, which associates with a lower Vmag.

        Examples: 3.0
        """
        ...

    @vm_apparent_min.setter
    def vm_apparent_min(self, value: Optional[float]) -> None: ...
    @property
    def wet_mass(self) -> Optional[float]:
        """
        Space object total mass (including propellant, i.e., ‘wet mass’) at the current reference epoch
        ‘EPOCH_TZERO’.

        Examples: 950.0

        Units: kg
        """
        ...

    @wet_mass.setter
    def wet_mass(self, value: Optional[float]) -> None: ...

class OcmSegment:
    """
    A single segment of the OCM.

    Contains metadata and data sections.

    Parameters
    ----------
    metadata : OcmMetadata
        Segment metadata.
    data : OcmData
        Segment data blocks.
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
        A single segment of the OCM.

        Contains metadata and data sections.
        """
        ...

    @metadata.setter
    def metadata(self, value: OcmMetadata) -> None: ...

class OcmTrajState:
    """
    A block of trajectory state data, which can be a time history of states.

    References:
    - CCSDS 502.0-B-3, Section 4.5.2 (OCM Trajectory State Section)

    Parameters
    ----------
    center_name : str
        Origin of the orbit reference frame.
    traj_ref_frame : str
        Reference frame of the trajectory state time history.
    traj_type : str
        Specifies the trajectory state element set type.
    traj_lines : list[TrajLine]
        Contiguous set of trajectory state data lines.
    traj_id : str, optional
        Identification number for this trajectory state time history block.
    traj_prev_id : str, optional
        Identification number for the previous trajectory state time history.
    traj_next_id : str, optional
        Identification number for the next trajectory state time history.
    traj_basis : str, optional
        The basis of this trajectory state time history data (PREDICTED, DETERMINED, etc.).
    traj_basis_id : str, optional
        Identification number for the telemetry dataset, orbit determination, or simulation.
    interpolation : str, optional
        Recommended interpolation method for the ephemeris data.
    interpolation_degree : int, optional
        Recommended interpolation degree.
    propagator : str, optional
        Name of the orbit propagator used to create this trajectory state time history.
    traj_frame_epoch : str, optional
        Epoch of the orbit data reference frame, if not intrinsic to the definition.
    useable_start_time : str, optional
        Start time of the useable time span covered by the ephemeris data.
    useable_stop_time : str, optional
        Stop time of the useable time span covered by the ephemeris data.
    orb_revnum : float, optional
        The integer orbit revolution number associated with the first trajectory state.
    orb_revnum_basis : str, optional
        Specifies the message creator’s basis for their orbit revolution counter (0 or 1).
    orb_averaging : str, optional
        Specifies whether the orbit elements are osculating elements or mean elements.
    traj_units : str, optional
        Comma-delimited set of SI unit designations for the trajectory state elements.
    comment : list[str], optional
        Comments.
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
        Name of the central body (value to be drawn from the SANA registry list of Common Central Body
        Names at <https://sanaregistry.org/r/central_body_name>).

        Examples: EARTH, MOON
        """
        ...

    @center_name.setter
    def center_name(self, value: str) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def interpolation(self) -> Optional[str]:
        """
        Recommended interpolation method for the state elements (value to be drawn from the SANA
        registry list of Interpolation Methods at <https://sanaregistry.org/r/interpolation_methods>).

        Examples: HERMITE, LINEAR
        """
        ...

    @interpolation.setter
    def interpolation(self, value: Optional[str]) -> None: ...
    @property
    def interpolation_degree(self) -> Optional[int]:
        """
        Recommended interpolation degree for the state elements.

        Examples: 5
        """
        ...

    @interpolation_degree.setter
    def interpolation_degree(self, value: Optional[int]) -> None: ...
    @property
    def orb_averaging(self) -> Optional[str]:
        """
        Method used for orbit averaging if TRAJ_TYPE is not osculating (value to be drawn from the SANA
        registry list of Orbit Averaging Methods at <https://sanaregistry.org/r/orbit_averaging>).

        Examples: BROUWER-LYDDANE
        """
        ...

    @orb_averaging.setter
    def orb_averaging(self, value: Optional[str]) -> None: ...
    @property
    def orb_revnum(self) -> Optional[float]:
        """
        Integer orbit revolution number at the epoch of the first trajectory data line.

        Examples: 1234.0
        """
        ...

    @orb_revnum.setter
    def orb_revnum(self, value: Optional[float]) -> None: ...
    @property
    def orb_revnum_basis(self) -> Optional[str]:
        """
        Basis for the orbit revolution counter (0 or 1).

        Examples: 1
        """
        ...

    @orb_revnum_basis.setter
    def orb_revnum_basis(self, value: Optional[str]) -> None: ...
    @property
    def propagator(self) -> Optional[str]:
        """
        Name of the propagator used in the creation of the trajectory state data.

        Examples: GMAT, STK
        """
        ...

    @propagator.setter
    def propagator(self, value: Optional[str]) -> None: ...
    @property
    def traj_basis(self) -> Optional[str]:
        """
        Basis of this trajectory state time history data (e.g., PREDICTED, DETERMINED, SIMULATED).

        Examples: PREDICTED
        """
        ...

    @traj_basis.setter
    def traj_basis(self, value: Optional[str]) -> None: ...
    @property
    def traj_basis_id(self) -> Optional[str]:
        """
        Identification number for the telemetry dataset, orbit determination, or simulation upon
        which the TRAJ_BASIS is based.

        Examples: OD-123
        """
        ...

    @traj_basis_id.setter
    def traj_basis_id(self, value: Optional[str]) -> None: ...
    @property
    def traj_frame_epoch(self) -> Optional[str]:
        """
        Epoch of the orbit reference frame, if TRAJ_REF_FRAME is provided and its epoch is not
        intrinsic to the definition of the reference frame.

        Examples: 2000-01-01T12:00:00Z
        """
        ...

    @traj_frame_epoch.setter
    def traj_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def traj_id(self) -> Optional[str]:
        """
        Identification number for this trajectory state time history block.

        Examples: 1
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

        Examples: 2
        """
        ...

    @traj_next_id.setter
    def traj_next_id(self, value: Optional[str]) -> None: ...
    @property
    def traj_prev_id(self) -> Optional[str]:
        """
        Identification number for the previous trajectory state time history.

        Examples: 0
        """
        ...

    @traj_prev_id.setter
    def traj_prev_id(self, value: Optional[str]) -> None: ...
    @property
    def traj_ref_frame(self) -> str:
        """
        Orbit reference frame (value to be drawn from the SANA registry list of Reference Frames at
        <https://sanaregistry.org/r/orbit_relative_reference_frames>).

        Examples: ICRF, EME2000
        """
        ...

    @traj_ref_frame.setter
    def traj_ref_frame(self, value: str) -> None: ...
    @property
    def traj_type(self) -> str:
        """
        Specification of the trajectory state element set type (value to be drawn from the SANA
        registry list of Trajectory State Types at <https://sanaregistry.org/r/orbital_elements>).

        Examples: CARTESIAN
        """
        ...

    @traj_type.setter
    def traj_type(self, value: str) -> None: ...
    @property
    def traj_units(self) -> Optional[str]:
        """
        SI unit designations for the state elements.

        Examples: km, km/s
        """
        ...

    @traj_units.setter
    def traj_units(self, value: Optional[str]) -> None: ...
    @property
    def useable_start_time(self) -> Optional[str]:
        """
        Start time of the useable time span covered by the ephemeris data.

        Examples: 2000-01-01T12:00:00Z
        """
        ...

    @useable_start_time.setter
    def useable_start_time(self, value: Optional[str]) -> None: ...
    @property
    def useable_stop_time(self) -> Optional[str]:
        """
        Stop time of the useable time span covered by the ephemeris data.

        Examples: 2000-01-02T12:00:00Z
        """
        ...

    @useable_stop_time.setter
    def useable_stop_time(self, value: Optional[str]) -> None: ...

class OdParameters:
    """
    Orbit Determination Parameters.

    Parameters
    ----------
    time_lastob_start : str, optional
        Time of last observation start.
    time_lastob_end : str, optional
        Time of last observation end.
    recommended_od_span : float, optional
        Recommended OD span. Units: d
    actual_od_span : float, optional
        Actual OD span. Units: d
    obs_available : int, optional
        Observations available.
    obs_used : int, optional
        Observations used.
    tracks_available : int, optional
        Tracks available.
    tracks_used : int, optional
        Tracks used.
    residuals_accepted : float, optional
        Residuals accepted. Units: %
    weighted_rms : float, optional
        Weighted RMS.
    comment : list of str, optional
        Comments.
    """
    def __init__(
        time_lastob_start,
        time_lastob_end,
        recommended_od_span,
        actual_od_span,
        obs_available,
        obs_used,
        tracks_available,
        tracks_used,
        residuals_accepted,
        weighted_rms,
        comment,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def actual_od_span(self) -> Optional[float]:
        """
        Based on the observations available and the RECOMMENDED_OD_SPAN, the actual
        time span used for the OD of the object. (See annex E for definition.)

        Examples: 14, 20.0

        Units: days
        """
        ...

    @actual_od_span.setter
    def actual_od_span(self, value: Optional[float]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 6.3.4 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def obs_available(self) -> Optional[int]:
        """
        The total number of observations available for orbit determination.
        """
        ...

    @obs_available.setter
    def obs_available(self, value: Optional[int]) -> None: ...
    @property
    def obs_used(self) -> Optional[int]:
        """
        The number of observations used in the orbit determination.
        """
        ...

    @obs_used.setter
    def obs_used(self, value: Optional[int]) -> None: ...
    @property
    def recommended_od_span(self) -> Optional[float]:
        """
        The recommended OD time span calculated for the object.

        Examples: 14, 20.0

        Units: days
        """
        ...

    @recommended_od_span.setter
    def recommended_od_span(self, value: Optional[float]) -> None: ...
    @property
    def residuals_accepted(self) -> Optional[float]:
        """
        The percentage of residuals accepted during orbit determination.
        """
        ...

    @residuals_accepted.setter
    def residuals_accepted(self, value: Optional[float]) -> None: ...
    @property
    def time_lastob_end(self) -> Optional[str]:
        """
        The end of a time interval (UTC) that contains the time of the last accepted
        observation. (See 6.3.2.6 for formatting rules.) For an exact time, the time interval is
        of zero duration (i.e., same value as that of TIME_LASTOB_START).
        """
        ...

    @time_lastob_end.setter
    def time_lastob_end(self, value: Optional[str]) -> None: ...
    @property
    def time_lastob_start(self) -> Optional[str]:
        """
        The start of a time interval (UTC) that contains the time of the last accepted
        observation. (See 6.3.2.6 for formatting rules.) For an exact time, the time interval is
        of zero duration (i.e., same value as that of TIME_LASTOB_END).
        """
        ...

    @time_lastob_start.setter
    def time_lastob_start(self, value: Optional[str]) -> None: ...
    @property
    def tracks_available(self) -> Optional[int]:
        """
        The total number of tracks available for orbit determination.
        """
        ...

    @tracks_available.setter
    def tracks_available(self, value: Optional[int]) -> None: ...
    @property
    def tracks_used(self) -> Optional[int]:
        """
        The number of tracks used in the orbit determination.
        """
        ...

    @tracks_used.setter
    def tracks_used(self, value: Optional[int]) -> None: ...
    @property
    def weighted_rms(self) -> Optional[float]:
        """
        The weighted root mean square (RMS) of the residuals.
        """
        ...

    @weighted_rms.setter
    def weighted_rms(self, value: Optional[float]) -> None: ...

class OdmHeader:
    """
    Represents the `odmHeader` complex type.

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
        User-defined free-text message classification/caveats of this ODM. It is recommended
        that selected values be pre-coordinated between exchanging entities by mutual agreement.

        Examples: SBU, ‘Operator-proprietary data; secondary distribution not permitted’
        """
        ...

    @classification.setter
    def classification(self, value: Optional[str]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def creation_date(self) -> str:
        """
        File creation date/time in UTC. (For format specification, see 7.5.10.)

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @creation_date.setter
    def creation_date(self, value: str) -> None: ...
    @property
    def message_id(self) -> Optional[str]:
        """
        ID that uniquely identifies a message from a given originator. The format and content of the
        message identifier value are at the discretion of the originator.

        Examples: OPM_201113719185, ABC-12_34
        """
        ...

    @message_id.setter
    def message_id(self, value: Optional[str]) -> None: ...
    @property
    def originator(self) -> str:
        """
        Creating agency or operator. Select from the accepted set of values indicated in annex B,
        subsection B1 from the ‘Abbreviation’ column (when present), or the ‘Name’ column when an
        Abbreviation column is not populated. If desired organization is not listed there, follow
        procedures to request that originator be added to SANA registry.

        Examples: CNES, ESOC, GSFC, GSOC, JPL, JAXA, INTELSAT, USAF, INMARSAT
        """
        ...

    @originator.setter
    def originator(self, value: str) -> None: ...

class Oem:
    """
    Orbit Ephemeris Message (OEM).

    Ephemeris information may be exchanged between two participants by sending a state vector (see
    reference \[1\]) for multiple epochs using an Orbit Ephemeris Message (OEM). The OEM also contains
    an optional covariance matrix that reflects the uncertainty of the orbit solution used to
    generate states in the ephemeris.

    Parameters
    ----------
    header : OdmHeader
        The message header.
    segments : list[OemSegment]
        The list of data segments.
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
    OEM Covariance Matrix.

    Represents a 6x6 symmetric covariance matrix for position and velocity at a specific epoch.
    The lower triangular portion is stored/transmitted.

    Parameters
    ----------
    epoch : str
        Epoch of the covariance matrix (ISO 8601).
        values : numpy.ndarray
        Flat NumPy array of length 21 containing the covariance values.
    cov_ref_frame : str, optional
        Reference frame for the covariance matrix.
    comment : list[str], optional
        Comments associated with this covariance matrix.

    Attributes
    ----------
    epoch : str
        Epoch of the covariance matrix.
    cx_x : float
        Position X covariance [1,1]. Units: km².
    cy_x : float
        Position X-Y covariance [2,1]. Units: km².
    cy_y : float
        Position Y covariance [2,2]. Units: km².
    cz_x : float
        Position X-Z covariance [3,1]. Units: km².
    cz_y : float
        Position Y-Z covariance [3,2]. Units: km².
    cz_z : float
        Position Z covariance [3,3]. Units: km².
    cx_dot_x : float
        Velocity X / Position X covariance [4,1]. Units: km²/s.
    cx_dot_y : float
        Velocity X / Position Y covariance [4,2]. Units: km²/s.
    cx_dot_z : float
        Velocity X / Position Z covariance [4,3]. Units: km²/s.
    cx_dot_x_dot : float
        Velocity X covariance [4,4]. Units: km²/s².
    cy_dot_x : float
        Velocity Y / Position X covariance [5,1]. Units: km²/s.
    cy_dot_y : float
        Velocity Y / Position Y covariance [5,2]. Units: km²/s.
    cy_dot_z : float
        Velocity Y / Position Z covariance [5,3]. Units: km²/s.
    cy_dot_x_dot : float
        Velocity Y / Velocity X covariance [5,4]. Units: km²/s².
    cy_dot_y_dot : float
        Velocity Y covariance [5,5]. Units: km²/s².
    cz_dot_x : float
        Velocity Z / Position X covariance [6,1]. Units: km²/s.
    cz_dot_y : float
        Velocity Z / Position Y covariance [6,2]. Units: km²/s.
    cz_dot_z : float
        Velocity Z / Position Z covariance [6,3]. Units: km²/s.
    cz_dot_x_dot : float
        Velocity Z / Velocity X covariance [6,4]. Units: km²/s².
    cz_dot_y_dot : float
        Velocity Z / Velocity Y covariance [6,5]. Units: km²/s².
    cz_dot_z_dot : float
        Velocity Z covariance [6,6]. Units: km²/s².
    """
    def __init__(epoch, values, cov_ref_frame, comment) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def cov_ref_frame(self) -> Optional[str]:
        """
        Reference frame in which the covariance data are given. Select from the accepted set of
        values indicated in 3.2.3.3 or 3.2.4.11.

        Examples: ICRF, EME2000
        """
        ...

    @cov_ref_frame.setter
    def cov_ref_frame(self, value: Optional[str]) -> None: ...
    @property
    def cx_dot_x(self) -> float:
        """
        Velocity X / Position X covariance [4,1].

        Examples: 0.001

        Units: km²/s
        """
        ...

    @cx_dot_x.setter
    def cx_dot_x(self, value: float) -> None: ...
    @property
    def cx_dot_x_dot(self) -> float:
        """
        Velocity X covariance [4,4].

        Examples: 0.00001

        Units: km²/s²
        """
        ...

    @cx_dot_x_dot.setter
    def cx_dot_x_dot(self, value: float) -> None: ...
    @property
    def cx_dot_y(self) -> float:
        """
        Velocity X / Position Y covariance [4,2].

        Examples: 0.0001

        Units: km²/s
        """
        ...

    @cx_dot_y.setter
    def cx_dot_y(self, value: float) -> None: ...
    @property
    def cx_dot_z(self) -> float:
        """
        Velocity X / Position Z covariance [4,3].

        Examples: 0.0001

        Units: km²/s
        """
        ...

    @cx_dot_z.setter
    def cx_dot_z(self, value: float) -> None: ...
    @property
    def cx_x(self) -> float:
        """
        Position X covariance [1,1].

        Examples: 1.0

        Units: km²
        """
        ...

    @cx_x.setter
    def cx_x(self, value: float) -> None: ...
    @property
    def cy_dot_x(self) -> float:
        """
        Velocity Y / Position X covariance [5,1].

        Examples: 0.0001

        Units: km²/s
        """
        ...

    @cy_dot_x.setter
    def cy_dot_x(self, value: float) -> None: ...
    @property
    def cy_dot_x_dot(self) -> float:
        """
        Velocity Y / Velocity X covariance [5,4].

        Examples: 0.00001

        Units: km²/s²
        """
        ...

    @cy_dot_x_dot.setter
    def cy_dot_x_dot(self, value: float) -> None: ...
    @property
    def cy_dot_y(self) -> float:
        """
        Velocity Y / Position Y covariance [5,2].

        Examples: 0.001

        Units: km²/s
        """
        ...

    @cy_dot_y.setter
    def cy_dot_y(self, value: float) -> None: ...
    @property
    def cy_dot_y_dot(self) -> float:
        """
        Velocity Y covariance [5,5].

        Examples: 0.00001

        Units: km²/s²
        """
        ...

    @cy_dot_y_dot.setter
    def cy_dot_y_dot(self, value: float) -> None: ...
    @property
    def cy_dot_z(self) -> float:
        """
        Velocity Y / Position Z covariance [5,3].

        Examples: 0.0001

        Units: km²/s
        """
        ...

    @cy_dot_z.setter
    def cy_dot_z(self, value: float) -> None: ...
    @property
    def cy_x(self) -> float:
        """
        Position X-Y covariance [2,1].

        Examples: 0.1

        Units: km²
        """
        ...

    @cy_x.setter
    def cy_x(self, value: float) -> None: ...
    @property
    def cy_y(self) -> float:
        """
        Position Y covariance [2,2].

        Examples: 1.0

        Units: km²
        """
        ...

    @cy_y.setter
    def cy_y(self, value: float) -> None: ...
    @property
    def cz_dot_x(self) -> float:
        """
        Velocity Z / Position X covariance [6,1].

        Examples: 0.0001

        Units: km²/s
        """
        ...

    @cz_dot_x.setter
    def cz_dot_x(self, value: float) -> None: ...
    @property
    def cz_dot_x_dot(self) -> float:
        """
        Velocity Z / Velocity X covariance [6,4].

        Examples: 0.00001

        Units: km²/s²
        """
        ...

    @cz_dot_x_dot.setter
    def cz_dot_x_dot(self, value: float) -> None: ...
    @property
    def cz_dot_y(self) -> float:
        """
        Velocity Z / Position Y covariance [6,2].

        Examples: 0.0001

        Units: km²/s
        """
        ...

    @cz_dot_y.setter
    def cz_dot_y(self, value: float) -> None: ...
    @property
    def cz_dot_y_dot(self) -> float:
        """
        Velocity Z / Velocity Y covariance [6,5].

        Examples: 0.00001

        Units: km²/s²
        """
        ...

    @cz_dot_y_dot.setter
    def cz_dot_y_dot(self, value: float) -> None: ...
    @property
    def cz_dot_z(self) -> float:
        """
        Velocity Z / Position Z covariance [6,3].

        Examples: 0.001

        Units: km²/s
        """
        ...

    @cz_dot_z.setter
    def cz_dot_z(self, value: float) -> None: ...
    @property
    def cz_dot_z_dot(self) -> float:
        """
        Velocity Z covariance [6,6].

        Examples: 0.00001

        Units: km²/s²
        """
        ...

    @cz_dot_z_dot.setter
    def cz_dot_z_dot(self, value: float) -> None: ...
    @property
    def cz_x(self) -> float:
        """
        Position X-Z covariance [3,1].

        Examples: 0.1

        Units: km²
        """
        ...

    @cz_x.setter
    def cz_x(self, value: float) -> None: ...
    @property
    def cz_y(self) -> float:
        """
        Position Y-Z covariance [3,2].

        Examples: 0.1

        Units: km²
        """
        ...

    @cz_y.setter
    def cz_y(self, value: float) -> None: ...
    @property
    def cz_z(self) -> float:
        """
        Position Z covariance [3,3].

        Examples: 1.0

        Units: km²
        """
        ...

    @cz_z.setter
    def cz_z(self, value: float) -> None: ...
    @property
    def epoch(self) -> str:
        """
        Epoch of covariance matrix. (See 7.5.10 for formatting rules.)

        Examples: 2000-01-01T12:00:00Z
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...

class OemData:
    """
    OEM Data Section.

    Parameters
    ----------
        state_vectors : list[StateVectorAcc]
        List of state vectors.
        comments : list[str], optional
        Comments.
    """
    def __init__(state_vectors, comments) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def covariance_matrix(self) -> list[OemCovarianceMatrix]:
        """
        List of covariance matrices associated with the state vectors.

        Each 6x6 covariance matrix provides uncertainty information for position and velocity:
        - Position covariance in km²
        - Position-velocity cross-covariance in km²/s
        - Velocity covariance in km²/s²

        Matrices are given in lower triangular form in the covariance reference frame.
        """
        ...

    @covariance_matrix.setter
    def covariance_matrix(self, value: list[OemCovarianceMatrix]) -> None: ...
    @property
    def covariance_matrix_numpy(self) -> tuple[list[str], numpy.ndarray]:
        """
        Get covariance matrices as a tuple associated with a NumPy array.

        Returns:
            tuple[list[str], np.ndarray]: (Epochs, 2D Array of size Nx21).
        """
        ...

    @covariance_matrix_numpy.setter
    def covariance_matrix_numpy(
        self, value: tuple[list[str], numpy.ndarray]
    ) -> None: ...
    @property
    def state_vector(self) -> list[StateVectorAcc]:
        """
        List of state vectors. Each vector contains position, velocity, and optional acceleration.

        Examples: 2020-01-01T00:00:00.000 1234.567 2345.678 3456.789 1.234 2.345 3.456

        Units: km, km/s, km/s²
        """
        ...

    @state_vector.setter
    def state_vector(self, value: list[StateVectorAcc]) -> None: ...
    @property
    def state_vector_numpy(self) -> tuple[list[str], numpy.ndarray]:
        """
        State vectors as a tuple of epochs and a NumPy array.

        This method allows for efficient zero-copy access to state vector data
        compatible with scientific Python libraries.

        Returns
        -------
        tuple[list[str], numpy.ndarray]
            A tuple containing:
            - List of epoch strings (ISO 8601 format).
            - 2D NumPy array of shape (N, 6) or (N, 9):
              - N x 6: [X, Y, Z, X_DOT, Y_DOT, Z_DOT] if no accelerations.
              - N x 9: [X, Y, Z, X_DOT, Y_DOT, Z_DOT, X_DDOT, Y_DDOT, Z_DDOT] if accelerations present.

            Units:
            - Position: km
            - Velocity: km/s
            - Acceleration: km/s²
        """
        ...

    @state_vector_numpy.setter
    def state_vector_numpy(self, value: tuple[list[str], numpy.ndarray]) -> None: ...

class OemMetadata:
    """
    OEM Metadata Section.

    This section contains descriptive information about the object and the ephemeris data,
    such as reference frames, time systems, and validation intervals.

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
        Start time of the total time span covered by the ephemeris data (ISO 8601).
    stop_time : str
        Stop time of the total time span covered by the ephemeris data (ISO 8601).
    ref_frame_epoch : str, optional
        Epoch of the reference frame, if not intrinsic to the definition (ISO 8601).
    useable_start_time : str, optional
        Start of the recommended time span for use of the ephemeris data (ISO 8601).
    useable_stop_time : str, optional
        End of the recommended time span for use of the ephemeris data (ISO 8601).
    interpolation : str, optional
        Recommended interpolation method for ephemeris data.
    interpolation_degree : int, optional
        Degree of the interpolation polynomial.
    comment : list[str], optional
        Comments.
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
        Origin of the OEM reference frame, which may be a natural solar system body (planets,
        asteroids, comets, and natural satellites), including any planet barycenter or the solar
        system barycenter, or another reference frame center (such as a spacecraft, formation flying
        reference ‘chief’ spacecraft, etc.). Natural bodies shall be selected from the accepted set of
        values indicated in annex B, subsection B2. For spacecraft, it is recommended to use either
        the OBJECT_ID or international designator of the participant as catalogued in the UN Office of
        Outer Space Affairs designator index (reference \[3\]).

        Examples: EARTH, EARTH BARYCENTER, MOON, SOLAR SYSTEM BARYCENTER, SUN,
        JUPITER BARYCENTER, STS 106, EROS
        """
        ...

    @center_name.setter
    def center_name(self, value: str) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def interpolation(self) -> Optional[str]:
        """
        This keyword may be used to specify the recommended interpolation method for ephemeris data in
        the immediately following set of ephemeris lines.

        Examples: HERMITE, LINEAR, LAGRANGE
        """
        ...

    @interpolation.setter
    def interpolation(self, value: Optional[str]) -> None: ...
    @property
    def interpolation_degree(self) -> Optional[int]:
        """
        Recommended interpolation degree for ephemeris data in the immediately following set of
        ephemeris lines. Must be an integer value. This keyword must be used if the ‘INTERPOLATION’
        keyword is used.

        Examples: 5, 8
        """
        ...

    @interpolation_degree.setter
    def interpolation_degree(self, value: Optional[int]) -> None: ...
    @property
    def object_id(self) -> str:
        """
        Object identifier of the object for which ephemeris data is provided. While there is no
        CCSDS-based restriction on the value for this keyword, it is recommended to use the
        international spacecraft designator as published in the UN Office of Outer Space Affairs
        designator index. Recommended values have the format YYYY-NNNP{PP}, where:
        YYYY = Year of launch.
        NNN = Three-digit serial number of launch in year YYYY (with leading zeros).
        P{PP} = At least one capital letter for the identification of the part brought into
        space by the launch. If the asset is not listed, the UN Office of Outer Space Affairs
        designator index format is not used, or the content is either unknown or cannot be
        disclosed, the value should be set to UNKNOWN.

        Examples: 2000-052A, 1996-068A, 2000-053A, 1996-008A, UNKNOWN
        """
        ...

    @object_id.setter
    def object_id(self, value: str) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Spacecraft name for which ephemeris data is provided. While there is no CCSDS-based
        restriction on the value for this keyword, it is recommended to use names from the UN Office
        of Outer Space Affairs designator index (reference \[3\], which include Object name and
        international designator of the participant). If OBJECT_NAME is not listed in reference \[3\]
        or the content is either unknown or cannot be disclosed, the value should be set to UNKNOWN.

        Examples: EUTELSAT W1, MARS PATHFINDER, STS 106, NEAR, UNKNOWN
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def ref_frame(self) -> str:
        """
        Reference frame in which the ephemeris data are given. Use of values other than those in
        3.2.3.3 should be documented in an ICD.

        Examples: ICRF, ITRF2000, EME2000, TEME
        """
        ...

    @ref_frame.setter
    def ref_frame(self, value: str) -> None: ...
    @property
    def ref_frame_epoch(self) -> Optional[str]:
        """
        Epoch of reference frame, if not intrinsic to the definition of the reference frame. (See
        7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @ref_frame_epoch.setter
    def ref_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def start_time(self) -> str:
        """
        Start of TOTAL time span covered by ephemeris data and covariance data immediately following
        this metadata block. (For format specification, see 7.5.10.)

        Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
        """
        ...

    @start_time.setter
    def start_time(self, value: str) -> None: ...
    @property
    def stop_time(self) -> str:
        """
        End of TOTAL time span covered by ephemeris data and covariance data immediately following
        this metadata block. (For format specification, see 7.5.10.)

        Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
        """
        ...

    @stop_time.setter
    def stop_time(self, value: str) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system used for ephemeris and covariance data. Use of values other than those in 3.2.3.2
        should be documented in an ICD.

        Examples: UTC, TAI, TT, GPS, TDB, TCB
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...
    @property
    def useable_start_time(self) -> Optional[str]:
        """
        Start time of USEABLE time span covered by ephemeris data immediately following this metadata
        block. (For format specification, see 7.5.10.) This optional keyword allows the message
        creator to introduce fictitious (but numerically smooth) data nodes prior to the actual data
        time history to support interpolation methods requiring more than two nodes (e.g., pure
        higher-order Lagrange interpolation methods). The use of this keyword and introduction of
        fictitious node points are optional and may not be necessary.

        Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
        """
        ...

    @useable_start_time.setter
    def useable_start_time(self, value: Optional[str]) -> None: ...
    @property
    def useable_stop_time(self) -> Optional[str]:
        """
        Stop time of USEABLE time span covered by ephemeris data immediately following this metadata
        block. (For format specification, see 7.5.10.) This optional keyword allows the message
        creator to introduce fictitious (but numerically smooth) data nodes following the actual data
        time history to support interpolation methods requiring more than two nodes (e.g., pure
        higher-order Lagrange interpolation methods). The use of this keyword and introduction of
        fictitious node points are optional and may not be necessary.

        Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
        """
        ...

    @useable_stop_time.setter
    def useable_stop_time(self, value: Optional[str]) -> None: ...

class OemSegment:
    """
    A single segment of the OEM.

    Each segment contains metadata (context) and a list of ephemeris data points.

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
        A single segment of the OEM.

        Each segment contains metadata (context) and a list of ephemeris data points.
        """
        ...

    @metadata.setter
    def metadata(self, value: OemMetadata) -> None: ...

class Omm:
    """
    Orbit Mean-Elements Message (OMM).

    The OMM contains the orbital characteristics of a single object at a specified epoch,
    expressed in mean Keplerian elements.

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
        Orbit Mean-Elements Message (OMM).

        The OMM contains the orbital characteristics of a single object at a specified epoch,
        expressed in mean Keplerian elements.
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
    """
    OMM Data section.
    """
    def __init__(mean_elements, comments) -> None: ...
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
    def covariance_matrix(self) -> Optional[OpmCovarianceMatrix]:
        """
        Position/Velocity Covariance Matrix (6x6 Lower Triangular Form).
        """
        ...

    @covariance_matrix.setter
    def covariance_matrix(self, value: Optional[OpmCovarianceMatrix]) -> None: ...
    @property
    def mean_elements(self) -> MeanElements:
        """
        Mean Keplerian Elements in the Specified Reference Frame.
        """
        ...

    @mean_elements.setter
    def mean_elements(self, value: MeanElements) -> None: ...
    @property
    def spacecraft_parameters(self) -> Optional[SpacecraftParameters]:
        """
        Spacecraft Parameters.
        """
        ...

    @spacecraft_parameters.setter
    def spacecraft_parameters(self, value: Optional[SpacecraftParameters]) -> None: ...
    @property
    def tle_parameters(self) -> Optional[TleParameters]:
        """
        TLE Related Parameters (Only required if MEAN_ELEMENT_THEORY=SGP/SGP4).
        """
        ...

    @tle_parameters.setter
    def tle_parameters(self, value: Optional[TleParameters]) -> None: ...
    @property
    def user_defined_parameters(self) -> Optional[UserDefined]:
        """
        User-Defined Parameters.
        """
        ...

    @user_defined_parameters.setter
    def user_defined_parameters(self, value: Optional[UserDefined]) -> None: ...

class OmmMetadata:
    """
    Metadata for the OMM.

    Parameters
    ----------
    object_name : str
        Spacecraft name for which mean element orbit state data is provided.
    object_id : str
        Object identifier of the object for which mean element orbit state data is provided.
    center_name : str
        Origin of the OMM reference frame.
    ref_frame : str
        Reference frame in which the Keplerian element data are given.
    time_system : str
        Time system used for Keplerian elements and covariance data.
    mean_element_theory : str
        Description of the Mean Element Theory. Indicates the proper method to employ to propagate the state.
    ref_frame_epoch : str, optional
        Epoch of reference frame, if not intrinsic to the definition of the reference frame.
    comment : list of str, optional
        Comments (allowed at the beginning of the OMM Metadata).
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
        Origin of the OMM reference frame, which shall be a natural solar system body (planets,
        asteroids, comets, and natural satellites), including any planet barycenter or the solar
        system barycenter. Natural bodies shall be selected from the accepted set of values
        indicated in annex B, subsection B2.

        Examples: EARTH, MARS, MOON
        """
        ...

    @center_name.setter
    def center_name(self, value: str) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (allowed at the beginning of the OMM Metadata). (See 7.8 for formatting rules.)
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def mean_element_theory(self) -> str:
        """
        Description of the Mean Element Theory. Indicates the proper method to employ to propagate the
        state.

        Examples: SGP, SGP4, SGP4-XP, DSST, USM
        """
        ...

    @mean_element_theory.setter
    def mean_element_theory(self, value: str) -> None: ...
    @property
    def object_id(self) -> str:
        """
        Object identifier of the object for which mean element orbit state data is provided. While
        there is no CCSDS-based restriction on the value for this keyword, it is recommended to use
        the international spacecraft designator as published in the UN Office of Outer Space Affairs
        designator index (reference \[3\]). Recommended values have the format YYYY-NNNP{PP}, where:
        YYYY = Year of launch. NNN = Three-digit serial number of launch in year YYYY (with leading
        zeros). P{PP} = At least one capital letter for the identification of the part brought into
        space by the launch. If the asset is not listed in reference \[3\], the UN Office of Outer
        Space Affairs designator index format is not used, or the content is either unknown or cannot
        be disclosed, the value should be set to UNKNOWN.

        Examples: 2005-046A, 2005-046B, 2003-022A, UNKNOWN
        """
        ...

    @object_id.setter
    def object_id(self, value: str) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Spacecraft name for which mean element orbit state data is provided. While there is no
        CCSDS-based restriction on the value for this keyword, it is recommended to use names from the
        UN Office of Outer Space Affairs designator index (reference \[3\], which include Object name
        and international designator of the participant). If OBJECT_NAME is not listed in reference
        \[3\] or the content is either unknown or cannot be disclosed, the value should be set to UNKNOWN.

        Examples: Telkom 2, Spaceway 2, INMARSAT 4-F2, UNKNOWN
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def ref_frame(self) -> str:
        """
        Reference frame in which the Keplerian element data are given. Use of values other than those
        in 3.2.3.3 should be documented in an ICD. NOTE—NORAD Two Line Element Sets and corresponding
        Simplified General Perturbations (SGP) orbit propagator ephemeris outputs are explicitly
        defined to be in the True Equator Mean Equinox of Date (TEME of Date) reference frame.
        Therefore, TEME of date shall be used for OMMs based on NORAD Two Line Element sets, rather
        than the almost imperceptibly different TEME of Epoch (see reference \[H2\] or \[H3\] for
        further details).

        Examples: ICRF, ITRF2000, EME2000, TEME
        """
        ...

    @ref_frame.setter
    def ref_frame(self, value: str) -> None: ...
    @property
    def ref_frame_epoch(self) -> Optional[str]:
        """
        Epoch of reference frame, if not intrinsic to the definition of the reference frame. (See
        7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @ref_frame_epoch.setter
    def ref_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system used for Keplerian elements and covariance data. Use of values other than those
        in 3.2.3.2 should be documented in an ICD.

        Examples: UTC
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
    Orbit Parameter Message (OPM).

    Orbit information may be exchanged between two participants by sending a state vector (see
    reference \[H1\]) for a specified epoch using an OPM. The message recipient must have an orbit
    propagator available that is able to propagate the OPM state vector to compute the orbit at other
    desired epochs. For this propagation, additional ancillary information (spacecraft properties
    such as mass, area, and maneuver planning data, if applicable) may be included with the message.

    Parameters
    ----------
    header : OdmHeader
        The message header.
    segment : OpmSegment
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
        Orbit Parameter Message (OPM).

        Orbit information may be exchanged between two participants by sending a state vector (see
        reference \[H1\]) for a specified epoch using an OPM. The message recipient must have an orbit
        propagator available that is able to propagate the OPM state vector to compute the orbit at other
        desired epochs. For this propagation, additional ancillary information (spacecraft properties
        such as mass, area, and maneuver planning data, if applicable) may be included with the message.
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
    """
    OPM covariance matrix block (opmCovarianceMatrixType).

    Parameters
    ----------
    cx_x : float, optional
        Position X covariance [1,1]. Units: km².
    cy_x : float, optional
        Position X-Y covariance [2,1]. Units: km².
    cy_y : float, optional
        Position Y covariance [2,2]. Units: km².
    cz_x : float, optional
        Position X-Z covariance [3,1]. Units: km².
    cz_y : float, optional
        Position Y-Z covariance [3,2]. Units: km².
    cz_z : float, optional
        Position Z covariance [3,3]. Units: km².
    cx_dot_x : float, optional
        Velocity X / Position X covariance [4,1]. Units: km²/s.
    cx_dot_y : float, optional
        Velocity X / Position Y covariance [4,2]. Units: km²/s.
    cx_dot_z : float, optional
        Velocity X / Position Z covariance [4,3]. Units: km²/s.
    cx_dot_x_dot : float, optional
        Velocity X covariance [4,4]. Units: km²/s².
    cy_dot_x : float, optional
        Velocity Y / Position X covariance [5,1]. Units: km²/s.
    cy_dot_y : float, optional
        Velocity Y / Position Y covariance [5,2]. Units: km²/s.
    cy_dot_z : float, optional
        Velocity Y / Position Z covariance [5,3]. Units: km²/s.
    cy_dot_x_dot : float, optional
        Velocity Y / Velocity X covariance [5,4]. Units: km²/s².
    cy_dot_y_dot : float, optional
        Velocity Y covariance [5,5]. Units: km²/s².
    cz_dot_x : float, optional
        Velocity Z / Position X covariance [6,1]. Units: km²/s.
    cz_dot_y : float, optional
        Velocity Z / Position Y covariance [6,2]. Units: km²/s.
    cz_dot_z : float, optional
        Velocity Z / Position Z covariance [6,3]. Units: km²/s.
    cz_dot_x_dot : float, optional
        Velocity Z / Velocity X covariance [6,4]. Units: km²/s².
    cz_dot_y_dot : float, optional
        Velocity Z / Velocity Y covariance [6,5]. Units: km²/s².
    cz_dot_z_dot : float, optional
        Velocity Z covariance [6,6]. Units: km²/s².
    cov_ref_frame : str, optional
        Reference frame for the covariance matrix.
        comments : list[str], optional
        Comments.

    Attributes
    ----------
    cx_x : float
        Position X covariance [1,1]. Units: km².
        ... (see Parameters for full list of attributes with units)
    """
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
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def cov_ref_frame(self) -> Optional[str]:
        """
        Reference frame in which the covariance data are given. Select from the accepted set of
        values indicated in 3.2.4.11.
        """
        ...

    @cov_ref_frame.setter
    def cov_ref_frame(self, value: Optional[str]) -> None: ...
    @property
    def cx_dot_x(self) -> float:
        """
        Velocity X / Position X covariance \[4,1\].

        Units: km²/s
        """
        ...

    @cx_dot_x.setter
    def cx_dot_x(self, value: float) -> None: ...
    @property
    def cx_dot_x_dot(self) -> float:
        """
        Velocity X covariance \[4,4\].

        Units: km²/s²
        """
        ...

    @cx_dot_x_dot.setter
    def cx_dot_x_dot(self, value: float) -> None: ...
    @property
    def cx_dot_y(self) -> float:
        """
        Velocity X / Position Y covariance \[4,2\].

        Units: km²/s
        """
        ...

    @cx_dot_y.setter
    def cx_dot_y(self, value: float) -> None: ...
    @property
    def cx_dot_z(self) -> float:
        """
        Velocity X / Position Z covariance \[4,3\].

        Units: km²/s
        """
        ...

    @cx_dot_z.setter
    def cx_dot_z(self, value: float) -> None: ...
    @property
    def cx_x(self) -> float:
        """
        Position X covariance \[1,1\].

        Units: km²
        """
        ...

    @cx_x.setter
    def cx_x(self, value: float) -> None: ...
    @property
    def cy_dot_x(self) -> float:
        """
        Velocity Y / Position X covariance \[5,1\].

        Units: km²/s
        """
        ...

    @cy_dot_x.setter
    def cy_dot_x(self, value: float) -> None: ...
    @property
    def cy_dot_x_dot(self) -> float:
        """
        Velocity Y / Velocity X covariance \[5,4\].

        Units: km²/s²
        """
        ...

    @cy_dot_x_dot.setter
    def cy_dot_x_dot(self, value: float) -> None: ...
    @property
    def cy_dot_y(self) -> float:
        """
        Velocity Y / Position Y covariance \[5,2\].

        Units: km²/s
        """
        ...

    @cy_dot_y.setter
    def cy_dot_y(self, value: float) -> None: ...
    @property
    def cy_dot_y_dot(self) -> float:
        """
        Velocity Y covariance \[5,5\].

        Units: km²/s²
        """
        ...

    @cy_dot_y_dot.setter
    def cy_dot_y_dot(self, value: float) -> None: ...
    @property
    def cy_dot_z(self) -> float:
        """
        Velocity Y / Position Z covariance \[5,3\].

        Units: km²/s
        """
        ...

    @cy_dot_z.setter
    def cy_dot_z(self, value: float) -> None: ...
    @property
    def cy_x(self) -> float:
        """
        Position Y / Position X covariance \[2,1\].

        Units: km²
        """
        ...

    @cy_x.setter
    def cy_x(self, value: float) -> None: ...
    @property
    def cy_y(self) -> float:
        """
        Position Y covariance \[2,2\].

        Units: km²
        """
        ...

    @cy_y.setter
    def cy_y(self, value: float) -> None: ...
    @property
    def cz_dot_x(self) -> float:
        """
        Velocity Z / Position X covariance \[6,1\].

        Units: km²/s
        """
        ...

    @cz_dot_x.setter
    def cz_dot_x(self, value: float) -> None: ...
    @property
    def cz_dot_x_dot(self) -> float:
        """
        Velocity Z / Velocity X covariance \[6,4\].

        Units: km²/s²
        """
        ...

    @cz_dot_x_dot.setter
    def cz_dot_x_dot(self, value: float) -> None: ...
    @property
    def cz_dot_y(self) -> float:
        """
        Velocity Z / Position Y covariance \[6,2\].

        Units: km²/s
        """
        ...

    @cz_dot_y.setter
    def cz_dot_y(self, value: float) -> None: ...
    @property
    def cz_dot_y_dot(self) -> float:
        """
        Velocity Z / Velocity Y covariance \[6,5\].

        Units: km²/s²
        """
        ...

    @cz_dot_y_dot.setter
    def cz_dot_y_dot(self, value: float) -> None: ...
    @property
    def cz_dot_z(self) -> float:
        """
        Velocity Z / Position Z covariance \[6,3\].

        Units: km²/s
        """
        ...

    @cz_dot_z.setter
    def cz_dot_z(self, value: float) -> None: ...
    @property
    def cz_dot_z_dot(self) -> float:
        """
        Velocity Z covariance \[6,6\].

        Units: km²/s²
        """
        ...

    @cz_dot_z_dot.setter
    def cz_dot_z_dot(self, value: float) -> None: ...
    @property
    def cz_x(self) -> float:
        """
        Position Z / Position X covariance \[3,1\].

        Units: km²
        """
        ...

    @cz_x.setter
    def cz_x(self, value: float) -> None: ...
    @property
    def cz_y(self) -> float:
        """
        Position Z / Position Y covariance \[3,2\].

        Units: km²
        """
        ...

    @cz_y.setter
    def cz_y(self, value: float) -> None: ...
    @property
    def cz_z(self) -> float:
        """
        Position Z covariance \[3,3\].

        Units: km²
        """
        ...

    @cz_z.setter
    def cz_z(self, value: float) -> None: ...

class OpmData:
    """
    OPM Data Section.

    Parameters
    ----------
    state_vector : StateVector
        State vector.
    """
    def __init__(state_vector, comment) -> None: ...
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
        State vector components (position and velocity).
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
    OPM Metadata Section.

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
    ref_frame_epoch : str, optional
        Epoch of the reference frame, if not intrinsic to the definition (ISO 8601).
    comment : list[str], optional
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
        Origin of the OPM reference frame, which shall be a natural solar system body (planets,
        asteroids, comets, and natural satellites), including any planet barycenter or the solar
        system barycenter. Natural bodies shall be selected from the accepted set of values
        indicated in annex B, subsection B2.

        Examples: EARTH, EARTH BARYCENTER, MOON, SOLAR SYSTEM BARYCENTER, SUN,
        JUPITER BARYCENTER, STS 106, EROS
        """
        ...

    @center_name.setter
    def center_name(self, value: str) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (allowed at the beginning of the OPM Metadata). (See 7.8 for formatting rules.)
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def object_id(self) -> str:
        """
        Object identifier of the object for which orbit state data is provided. While there is no
        CCSDS-based restriction on the value for this keyword, it is recommended to use the
        international spacecraft designator as published in the UN Office of Outer Space Affairs
        designator index (reference \[3\]). Recommended values have the format YYYY-NNNP{PP}, where:
        YYYY = Year of launch. NNN = Three-digit serial number of launch in year YYYY (with leading
        zeros). P{PP} = At least one capital letter for the identification of the part brought into
        space by the launch. If the asset is not listed in reference \[3\], the UN Office of Outer
        Space Affairs designator index format is not used, or the content is either unknown or cannot
        be disclosed, the value should be set to UNKNOWN.

        Examples: 2000-052A, 1996-068A, 2000-053A, 1996-008A, UNKNOWN
        """
        ...

    @object_id.setter
    def object_id(self, value: str) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Spacecraft name for which orbit state data is provided. While there is no CCSDS-based
        restriction on the value for this keyword, it is recommended to use names from the UN Office
        of Outer Space Affairs designator index (reference \[3\], which include Object name and
        international designator of the participant). If OBJECT_NAME is not listed in reference \[3\]
        or the content is either unknown or cannot be disclosed, the value should be set to UNKNOWN.

        Examples: EUTELSAT W1, MARS PATHFINDER, STS 106, NEAR, UNKNOWN
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def ref_frame(self) -> str:
        """
        Reference frame in which the state vector and optional Keplerian element data are given. Use
        of values other than those in 3.2.3.3 should be documented in an ICD.

        Examples: ICRF, EME2000, ITRF2000, TEME
        """
        ...

    @ref_frame.setter
    def ref_frame(self, value: str) -> None: ...
    @property
    def ref_frame_epoch(self) -> Optional[str]:
        """
        Epoch of reference frame, if not intrinsic to the definition of the reference frame. (See
        7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @ref_frame_epoch.setter
    def ref_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system used for state vector, maneuver, and covariance data. Use of values other than
        those in 3.2.3.2 should be documented in an ICD.

        Examples: UTC, TAI, TT, GPS, TDB, TCB
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...

class OpmSegment:
    """
    A single segment of the OPM.

    Contains metadata and data sections.

    Parameters
    ----------
    metadata : OpmMetadata
        Segment metadata.
    data : OpmData
        Segment data.
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
        A single segment of the OPM.

        Contains metadata and data sections.
        """
        ...

    @metadata.setter
    def metadata(self, value: OpmMetadata) -> None: ...

class Rdm:
    """
    Re-entry Data Message (RDM).

    A message format for use in exchanging spacecraft re-entry information.

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
        Re-entry Data Message (RDM).

        A message format for use in exchanging spacecraft re-entry information.
        """
        ...

    @header.setter
    def header(self, value: RdmHeader) -> None: ...
    @property
    def segment(self) -> RdmSegment:
        """
        The RDM Body consists of a single segment.
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

class RdmData:
    """
    The RDM Data section.

    Parameters
    ----------
    atmospheric_reentry_parameters : AtmosphericReentryParameters
        Mandatory atmospheric re-entry data.
    ground_impact_parameters : GroundImpactParameters, optional
        Ground impact and burn-up data.
    state_vector : StateVector, optional
        Spacecraft state vector.
    covariance_matrix : OpmCovarianceMatrix, optional
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
    def covariance_matrix(self) -> Optional[OpmCovarianceMatrix]:
        """
        Covariance matrix.
        """
        ...

    @covariance_matrix.setter
    def covariance_matrix(self, value: Optional[OpmCovarianceMatrix]) -> None: ...
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
    def state_vector(self) -> Optional[StateVector]:
        """
        State vector.
        """
        ...

    @state_vector.setter
    def state_vector(self, value: Optional[StateVector]) -> None: ...
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
    The RDM Header provides information about the message.

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

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23
        """
        ...

    @creation_date.setter
    def creation_date(self, value: str) -> None: ...
    @property
    def message_id(self) -> str:
        """
        ID that uniquely identifies a message from a given originator.

        Examples: 201113719185, ESA20190101-3345
        """
        ...

    @message_id.setter
    def message_id(self, value: str) -> None: ...
    @property
    def originator(self) -> str:
        """
        Creating agency or entity.

        Examples: DLR, ESA
        """
        ...

    @originator.setter
    def originator(self, value: str) -> None: ...

class RdmMetadata:
    """
    The RDM Metadata provides information about the re-entry event.

    Parameters
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

        Optional
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
        The atmospheric model used.

        Examples: Jacchia 70, MSIS-86
        """
        ...

    @atmospheric_model.setter
    def atmospheric_model(self, value: Optional[str]) -> None: ...
    @property
    def catalog_name(self) -> Optional[str]:
        """
        The catalog name for the object.

        Examples: SATCAT, SPCS, MCN
        """
        ...

    @catalog_name.setter
    def catalog_name(self, value: Optional[str]) -> None: ...
    @property
    def center_name(self) -> str:
        """
        The celestial body the object is orbiting.

        Examples: EARTH, MOON, MARS
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
        Whether the re-entry is controlled or not.

        Examples: YES, NO, UNKNOWN
        """
        ...

    @controlled_reentry.setter
    def controlled_reentry(self, value: str) -> None: ...
    @property
    def drag_parameters_altitude(self) -> Optional[float]:
        """
        The altitude at which the drag parameters were estimated.

        Units: km

        Examples: 200.0 [km]
        """
        ...

    @drag_parameters_altitude.setter
    def drag_parameters_altitude(self, value: Optional[float]) -> None: ...
    @property
    def drag_parameters_source(self) -> Optional[str]:
        """
        The source of the drag parameters.

        Examples: OD, DATABASE, DEFAULT
        """
        ...

    @drag_parameters_source.setter
    def drag_parameters_source(self, value: Optional[str]) -> None: ...
    @property
    def earth_tides(self) -> Optional[str]:
        """
        The Earth tides model used.

        Examples: ERS, IERS
        """
        ...

    @earth_tides.setter
    def earth_tides(self, value: Optional[str]) -> None: ...
    @property
    def ephemeris_name(self) -> Optional[str]:
        """
        The name of the ephemeris used.

        Examples: DE430, JPLEPH.405
        """
        ...

    @ephemeris_name.setter
    def ephemeris_name(self, value: Optional[str]) -> None: ...
    @property
    def epoch_tzero(self) -> str:
        """
        The reference epoch for the message.

        Examples: 2018-04-22T09:00:00.00
        """
        ...

    @epoch_tzero.setter
    def epoch_tzero(self, value: str) -> None: ...
    @property
    def gravity_model(self) -> Optional[str]:
        """
        The gravity model used.

        Examples: EGM-96, JGM-3
        """
        ...

    @gravity_model.setter
    def gravity_model(self, value: Optional[str]) -> None: ...
    @property
    def impact_uncertainty_method(self) -> Optional[str]:
        """
        The method used to compute impact uncertainty.

        Examples: MONTE-CARLO, ANALYTICAL
        """
        ...

    @impact_uncertainty_method.setter
    def impact_uncertainty_method(self, value: Optional[str]) -> None: ...
    @property
    def international_designator(self) -> str:
        """
        The international designator of the object.

        Examples: 1999-025A, 1991-063B, 2011-053A
        """
        ...

    @international_designator.setter
    def international_designator(self, value: str) -> None: ...
    @property
    def intrack_thrust(self) -> Optional[str]:
        """
        Whether there was any intrack thrust.

        Examples: YES, NO
        """
        ...

    @intrack_thrust.setter
    def intrack_thrust(self, value: Optional[str]) -> None: ...
    @property
    def n_body_perturbations(self) -> Optional[str]:
        """
        The n-body perturbations used.

        Examples: MOON, SUN, JUPITER
        """
        ...

    @n_body_perturbations.setter
    def n_body_perturbations(self, value: Optional[str]) -> None: ...
    @property
    def next_message_epoch(self) -> Optional[str]:
        """
        The epoch of the next message for this object.

        Examples: 2018-04-23T09:00:00
        """
        ...

    @next_message_epoch.setter
    def next_message_epoch(self, value: Optional[str]) -> None: ...
    @property
    def object_designator(self) -> Optional[str]:
        """
        The object designator in the catalog.

        Examples: 25730, 21574, 37820
        """
        ...

    @object_designator.setter
    def object_designator(self, value: Optional[str]) -> None: ...
    @property
    def object_name(self) -> str:
        """
        The name of the object.

        Examples: FENGYUN 1C, UARS, Tiangong-1
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def object_operator(self) -> Optional[str]:
        """
        The operator of the object.

        Examples: EUMETSAT, SES
        """
        ...

    @object_operator.setter
    def object_operator(self, value: Optional[str]) -> None: ...
    @property
    def object_owner(self) -> Optional[str]:
        """
        The owner of the object.

        Examples: China, USA, France
        """
        ...

    @object_owner.setter
    def object_owner(self, value: Optional[str]) -> None: ...
    @property
    def object_type(self) -> Optional[str]:
        """
        The type of the object.

        Examples: PAYLOAD, ROCKET BODY, DEBRIS, UNKNOWN, OTHER
        """
        ...

    @object_type.setter
    def object_type(self, value: Optional[str]) -> None: ...
    @property
    def previous_message_epoch(self) -> Optional[str]:
        """
        The epoch of the previous message for this object.

        Examples: 2018-04-21T09:00:00.00
        """
        ...

    @previous_message_epoch.setter
    def previous_message_epoch(self, value: Optional[str]) -> None: ...
    @property
    def previous_message_id(self) -> Optional[str]:
        """
        The ID of the previous message for this object.

        Examples: ESA/20180421-007
        """
        ...

    @previous_message_id.setter
    def previous_message_id(self, value: Optional[str]) -> None: ...
    @property
    def reentry_disintegration(self) -> Optional[str]:
        """
        The method used to model the object’s disintegration.

        Examples: MASS-LOSS, BREAK-UP, NONE
        """
        ...

    @reentry_disintegration.setter
    def reentry_disintegration(self, value: Optional[str]) -> None: ...
    @property
    def reentry_uncertainty_method(self) -> Optional[str]:
        """
        The method used to compute re-entry uncertainty.

        Examples: MONTE-CARLO, ANALYTICAL
        """
        ...

    @reentry_uncertainty_method.setter
    def reentry_uncertainty_method(self, value: Optional[str]) -> None: ...
    @property
    def ref_frame(self) -> Optional[str]:
        """
        The reference frame of the state vector and covariance matrix.

        Examples: EME2000, GCRF, ICRF, ITRF2000, TDR
        """
        ...

    @ref_frame.setter
    def ref_frame(self, value: Optional[str]) -> None: ...
    @property
    def ref_frame_epoch(self) -> Optional[str]:
        """
        The epoch of the reference frame.

        Examples: 2000-01-01T00:00:00.000
        """
        ...

    @ref_frame_epoch.setter
    def ref_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def solar_flux_prediction(self) -> Optional[str]:
        """
        The solar flux and geomagnetic activity data used.

        Examples: F10.7_MEAN_81_CYCLE, SCHATTEN_ADJUSTED
        """
        ...

    @solar_flux_prediction.setter
    def solar_flux_prediction(self, value: Optional[str]) -> None: ...
    @property
    def solar_rad_pressure(self) -> Optional[str]:
        """
        Whether solar radiation pressure was used.

        Examples: YES, NO
        """
        ...

    @solar_rad_pressure.setter
    def solar_rad_pressure(self, value: Optional[str]) -> None: ...
    @property
    def time_system(self) -> str:
        """
        The time system used for the message.

        Examples: UTC, TAI, TDB
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
        The data for this RDM segment.
        """
        ...

    @data.setter
    def data(self, value: RdmData) -> None: ...
    @property
    def metadata(self) -> RdmMetadata:
        """
        The metadata for this RDM segment.
        """
        ...

    @metadata.setter
    def metadata(self, value: RdmMetadata) -> None: ...

class RdmSpacecraftParameters:
    """
    RDM spacecraft parameters (rdmSpacecraftParametersType).
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
        The probability that Object1 and Object2 will collide.
        """
        ...

    @collision_probability.setter
    def collision_probability(self, value: Optional[float]) -> None: ...
    @property
    def collision_probability_method(self) -> Optional[str]:
        """
        The method that was used to calculate the collision probability.
        """
        ...

    @collision_probability_method.setter
    def collision_probability_method(self, value: Optional[str]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def miss_distance(self) -> float:
        """
        The norm of the relative position vector.

        Units: m
        """
        ...

    @miss_distance.setter
    def miss_distance(self, value: float) -> None: ...
    @property
    def relative_position(self) -> Optional[list[float]]:
        """
        Relative position [R, T, N].
        """
        ...

    @relative_position.setter
    def relative_position(self, value: Optional[list[float]]) -> None: ...
    @property
    def relative_speed(self) -> Optional[float]:
        """
        The norm of the relative velocity vector.

        Units: m/s
        """
        ...

    @relative_speed.setter
    def relative_speed(self, value: Optional[float]) -> None: ...
    @property
    def relative_velocity(self) -> Optional[list[float]]:
        """
        Relative velocity [R, T, N].
        """
        ...

    @relative_velocity.setter
    def relative_velocity(self, value: Optional[list[float]]) -> None: ...
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
    def screen_volume_frame(self) -> Optional[ScreenVolumeFrameType]:
        """
        Name of the Object1 centered reference frame in which the screening volume data are given.
        """
        ...

    @screen_volume_frame.setter
    def screen_volume_frame(self, value: Optional[ScreenVolumeFrameType]) -> None: ...
    @property
    def screen_volume_shape(self) -> Optional[ScreenVolumeShapeType]:
        """
        Shape of the screening volume.
        """
        ...

    @screen_volume_shape.setter
    def screen_volume_shape(self, value: Optional[ScreenVolumeShapeType]) -> None: ...
    @property
    def screen_volume_x(self) -> Optional[float]:
        """
        The R or T component size of the screening volume.

        Units: m
        """
        ...

    @screen_volume_x.setter
    def screen_volume_x(self, value: Optional[float]) -> None: ...
    @property
    def screen_volume_y(self) -> Optional[float]:
        """
        The T or V component size of the screening volume.

        Units: m
        """
        ...

    @screen_volume_y.setter
    def screen_volume_y(self, value: Optional[float]) -> None: ...
    @property
    def screen_volume_z(self) -> Optional[float]:
        """
        The N component size of the screening volume.

        Units: m
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
    Spacecraft physical parameters (mass, area, coefficients).

    References:
    - CCSDS 502.0-B-3, Section 3.2.4 (OPM Data Section)

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
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def drag_area(self) -> Optional[float]:
        """
        Drag Area (AD).

        Examples: 14, 20.0

        Units: m²
        """
        ...

    @drag_area.setter
    def drag_area(self, value: Optional[float]) -> None: ...
    @property
    def drag_coeff(self) -> Optional[float]:
        """
        Drag Coefficient (CD).

        Examples: 2, 2.1
        """
        ...

    @drag_coeff.setter
    def drag_coeff(self, value: Optional[float]) -> None: ...
    @property
    def mass(self) -> Optional[float]:
        """
        Spacecraft mass.

        Examples: 1850.2, 3352.0

        Units: kg
        """
        ...

    @mass.setter
    def mass(self, value: Optional[float]) -> None: ...
    @property
    def solar_rad_area(self) -> Optional[float]:
        """
        Solar Radiation Pressure Area (AR).

        Examples: 14, 20.0

        Units: m²
        """
        ...

    @solar_rad_area.setter
    def solar_rad_area(self, value: Optional[float]) -> None: ...
    @property
    def solar_rad_coeff(self) -> Optional[float]:
        """
        Solar Radiation Pressure Coefficient (CR).

        Examples: 1, 1.34
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
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def epoch(self) -> str:
        """
        Epoch of state vector (see 7.5.10 for formatting rules).

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def x(self) -> float:
        """
        Position vector X-component.

        Examples: 6653.148

        Units: km
        """
        ...

    @x.setter
    def x(self, value: float) -> None: ...
    @property
    def x_dot(self) -> float:
        """
        Velocity vector X-component.

        Examples: 0.0

        Units: km/s
        """
        ...

    @x_dot.setter
    def x_dot(self, value: float) -> None: ...
    @property
    def y(self) -> float:
        """
        Position vector Y-component.

        Examples: -20.0

        Units: km
        """
        ...

    @y.setter
    def y(self, value: float) -> None: ...
    @property
    def y_dot(self) -> float:
        """
        Velocity vector Y-component.

        Examples: 7.7

        Units: km/s
        """
        ...

    @y_dot.setter
    def y_dot(self, value: float) -> None: ...
    @property
    def z(self) -> float:
        """
        Position vector Z-component.

        Examples: 0.0

        Units: km
        """
        ...

    @z.setter
    def z(self, value: float) -> None: ...
    @property
    def z_dot(self) -> float:
        """
        Velocity vector Z-component.

        Examples: 0.0

        Units: km/s
        """
        ...

    @z_dot.setter
    def z_dot(self, value: float) -> None: ...

class StateVectorAcc:
    """
    Represents the `stateVectorType` and `stateVectorAccType` from the XSD.

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
        Epoch of state vector (see 7.5.10 for formatting rules).

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def x(self) -> float:
        """
        Position vector X-component.

        Examples: 6653.148

        Units: km
        """
        ...

    @x.setter
    def x(self, value: float) -> None: ...
    @property
    def x_ddot(self) -> Optional[float]:
        """
        Acceleration vector X-component.

        Examples: 0.001

        Units: km/s²
        """
        ...

    @x_ddot.setter
    def x_ddot(self, value: Optional[float]) -> None: ...
    @property
    def x_dot(self) -> float:
        """
        Velocity vector X-component.

        Examples: 0.0

        Units: km/s
        """
        ...

    @x_dot.setter
    def x_dot(self, value: float) -> None: ...
    @property
    def y(self) -> float:
        """
        Position vector Y-component.

        Examples: -20.0

        Units: km
        """
        ...

    @y.setter
    def y(self, value: float) -> None: ...
    @property
    def y_ddot(self) -> Optional[float]:
        """
        Acceleration vector Y-component.

        Examples: 0.0

        Units: km/s²
        """
        ...

    @y_ddot.setter
    def y_ddot(self, value: Optional[float]) -> None: ...
    @property
    def y_dot(self) -> float:
        """
        Velocity vector Y-component.

        Examples: 7.7

        Units: km/s
        """
        ...

    @y_dot.setter
    def y_dot(self, value: float) -> None: ...
    @property
    def z(self) -> float:
        """
        Position vector Z-component.

        Examples: 0.0

        Units: km
        """
        ...

    @z.setter
    def z(self, value: float) -> None: ...
    @property
    def z_ddot(self) -> Optional[float]:
        """
        Acceleration vector Z-component.

        Examples: 0.0

        Units: km/s²
        """
        ...

    @z_ddot.setter
    def z_ddot(self, value: Optional[float]) -> None: ...
    @property
    def z_dot(self) -> float:
        """
        Velocity vector Z-component.

        Examples: 0.0

        Units: km/s
        """
        ...

    @z_dot.setter
    def z_dot(self, value: float) -> None: ...

class Tdm:
    """
    Tracking Data Message (TDM).

    The TDM specifies a standard message format for use in exchanging tracking data.

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
        Tracking Data Message (TDM).

        The TDM specifies a standard message format for use in exchanging tracking data.
        """
        ...

    @header.setter
    def header(self, value: TdmHeader) -> None: ...
    @property
    def segment(self) -> list[TdmSegment]:
        """
        Shortcut to access segments directly from the body.
        """
        ...

    @segment.setter
    def segment(self, value: list[TdmSegment]) -> None: ...
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
    def segment(self) -> list[TdmSegment]:
        """
        List of TDM segments.

        Each segment consists of a Metadata Section and a Data Section.
        """
        ...

    @segment.setter
    def segment(self, value: list[TdmSegment]) -> None: ...

class TdmData:
    """
    The Data Section of the TDM Segment consists of one or more Tracking Data Records.

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
    def comment(self) -> list[TdmObservation]:
        """
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[TdmObservation]) -> None: ...
    @property
    def observations(self) -> list[TdmObservation]:
        """
        Tracking data records.
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
        Comments.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def creation_date(self) -> str:
        """
        Data creation date/time in UTC.

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23.4, 2006-001T00:00:00Z
        """
        ...

    @creation_date.setter
    def creation_date(self, value: str) -> None: ...
    @property
    def message_id(self) -> Optional[str]:
        """
        ID that uniquely identifies a message from a given originator.

        Examples: 201113719185
        """
        ...

    @message_id.setter
    def message_id(self, value: Optional[str]) -> None: ...
    @property
    def originator(self) -> str:
        """
        Creating agency.

        Examples: CNES, ESA, GSFC, DLR, JPL, JAXA
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
        The type of antenna geometry represented in the angle data.

        Examples: AZEL, RADEC, XEYN, XSYE
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
        A correction value for diurnal aberration.

        Examples: -1.35, 0.23, -3.0e-1, 150000.0
        """
        ...

    @correction_aberration_diurnal.setter
    def correction_aberration_diurnal(self, value: Optional[float]) -> None: ...
    @property
    def correction_aberration_yearly(self) -> Optional[float]:
        """
        A correction value for yearly aberration.

        Examples: -1.35, 0.23, -3.0e-1, 150000.0
        """
        ...

    @correction_aberration_yearly.setter
    def correction_aberration_yearly(self, value: Optional[float]) -> None: ...
    @property
    def correction_angle_1(self) -> Optional[float]:
        """
        A correction value to be added to the ANGLE_1 data.

        Examples: -1.35, 0.23, -3.0e-1, 150000.0
        """
        ...

    @correction_angle_1.setter
    def correction_angle_1(self, value: Optional[float]) -> None: ...
    @property
    def correction_angle_2(self) -> Optional[float]:
        """
        A correction value to be added to the ANGLE_2 data.

        Examples: -1.35, 0.23, -3.0e-1, 150000.0
        """
        ...

    @correction_angle_2.setter
    def correction_angle_2(self, value: Optional[float]) -> None: ...
    @property
    def correction_doppler(self) -> Optional[float]:
        """
        A correction value to be added to the Doppler data.

        Examples: -1.35, 0.23, -3.0e-1, 150000.0
        """
        ...

    @correction_doppler.setter
    def correction_doppler(self, value: Optional[float]) -> None: ...
    @property
    def correction_mag(self) -> Optional[float]:
        """
        A correction value to be added to the magnitude data.

        Examples: -1.35, 0.23, -3.0e-1, 150000.0
        """
        ...

    @correction_mag.setter
    def correction_mag(self, value: Optional[float]) -> None: ...
    @property
    def correction_range(self) -> Optional[float]:
        """
        A correction value to be added to the range data.

        Examples: -1.35, 0.23, -3.0e-1, 150000.0
        """
        ...

    @correction_range.setter
    def correction_range(self, value: Optional[float]) -> None: ...
    @property
    def correction_rcs(self) -> Optional[float]:
        """
        A correction value to be added to the RCS data.

        Examples: -1.35, 0.23, -3.0e-1, 150000.0
        """
        ...

    @correction_rcs.setter
    def correction_rcs(self, value: Optional[float]) -> None: ...
    @property
    def correction_receive(self) -> Optional[float]:
        """
        A correction value to be added to the received frequency or phase count data.

        Examples: -1.35, 0.23, -3.0e-1, 150000.0
        """
        ...

    @correction_receive.setter
    def correction_receive(self, value: Optional[float]) -> None: ...
    @property
    def correction_transmit(self) -> Optional[float]:
        """
        A correction value to be added to the transmitted frequency or phase count data.

        Examples: -1.35, 0.23, -3.0e-1, 150000.0
        """
        ...

    @correction_transmit.setter
    def correction_transmit(self, value: Optional[float]) -> None: ...
    @property
    def corrections_applied(self) -> Optional[str]:
        """
        Indicates whether or not the correction values have been applied to the tracking data.

        Examples: YES, NO
        """
        ...

    @corrections_applied.setter
    def corrections_applied(self, value: Optional[str]) -> None: ...
    @property
    def data_quality(self) -> Optional[str]:
        """
        An estimate of the quality of the data.

        Examples: RAW, VALIDATED, DEGRADED
        """
        ...

    @data_quality.setter
    def data_quality(self, value: Optional[str]) -> None: ...
    @property
    def data_types(self) -> Optional[str]:
        """
        Comma-separated list of data types in the Data Section.

        Examples: RANGE, TRANSMIT_FREQ_n, RECEIVE_FREQ
        """
        ...

    @data_types.setter
    def data_types(self, value: Optional[str]) -> None: ...
    @property
    def doppler_count_bias(self) -> Optional[float]:
        """
        A bias that shall be subtracted from the DOPPLER_COUNT data value.

        Units: Hz

        Examples: 2.4e6, 240000000.0
        """
        ...

    @doppler_count_bias.setter
    def doppler_count_bias(self, value: Optional[float]) -> None: ...
    @property
    def doppler_count_rollover(self) -> Optional[str]:
        """
        Flag indicating whether or not a Doppler counter rollover has occurred.

        Examples: YES, NO
        """
        ...

    @doppler_count_rollover.setter
    def doppler_count_rollover(self, value: Optional[str]) -> None: ...
    @property
    def doppler_count_scale(self) -> Optional[int]:
        """
        A scale factor that the DOPPLER_COUNT data value shall be divided by.

        Examples: 1000, 1
        """
        ...

    @doppler_count_scale.setter
    def doppler_count_scale(self, value: Optional[int]) -> None: ...
    @property
    def ephemeris_name_1(self) -> Optional[str]:
        """
        Unique name of the external ephemeris file used for participant 1.

        Examples: SATELLITE_A_EPHEM27
        """
        ...

    @ephemeris_name_1.setter
    def ephemeris_name_1(self, value: Optional[str]) -> None: ...
    @property
    def ephemeris_name_2(self) -> Optional[str]:
        """
        Unique name of the external ephemeris file used for participant 2.

        Examples: SATELLITE_A_EPHEM27
        """
        ...

    @ephemeris_name_2.setter
    def ephemeris_name_2(self, value: Optional[str]) -> None: ...
    @property
    def ephemeris_name_3(self) -> Optional[str]:
        """
        Unique name of the external ephemeris file used for participant 3.

        Examples: SATELLITE_A_EPHEMERIS
        """
        ...

    @ephemeris_name_3.setter
    def ephemeris_name_3(self, value: Optional[str]) -> None: ...
    @property
    def ephemeris_name_4(self) -> Optional[str]:
        """
        Unique name of the external ephemeris file used for participant 4.

        Examples: SATELLITE_A_EPHEMERIS
        """
        ...

    @ephemeris_name_4.setter
    def ephemeris_name_4(self, value: Optional[str]) -> None: ...
    @property
    def ephemeris_name_5(self) -> Optional[str]:
        """
        Unique name of the external ephemeris file used for participant 5.

        Examples: SATELLITE_A_EPHEMERIS
        """
        ...

    @ephemeris_name_5.setter
    def ephemeris_name_5(self, value: Optional[str]) -> None: ...
    @property
    def freq_offset(self) -> Optional[float]:
        """
        A frequency in Hz that must be added to every RECEIVE_FREQ to reconstruct it.

        Examples: 0.0, 8415000000.0
        """
        ...

    @freq_offset.setter
    def freq_offset(self, value: Optional[float]) -> None: ...
    @property
    def integration_interval(self) -> Optional[float]:
        """
        The Doppler count time in seconds for Doppler data.

        Units: s

        Examples: 60.0, 0.1, 1.0
        """
        ...

    @integration_interval.setter
    def integration_interval(self, value: Optional[float]) -> None: ...
    @property
    def integration_ref(self) -> Optional[str]:
        """
        Indicates the relationship between the INTEGRATION_INTERVAL and the timetag.

        Examples: START, MIDDLE, END
        """
        ...

    @integration_ref.setter
    def integration_ref(self, value: Optional[str]) -> None: ...
    @property
    def interpolation(self) -> Optional[str]:
        """
        The interpolation method to be used to calculate a transmit phase count.

        Examples: HERMITE, LAGRANGE, LINEAR
        """
        ...

    @interpolation.setter
    def interpolation(self, value: Optional[str]) -> None: ...
    @property
    def interpolation_degree(self) -> Optional[int]:
        """
        The recommended degree of the interpolating polynomial for phase count data.

        Examples: 3, 5, 7, 11
        """
        ...

    @interpolation_degree.setter
    def interpolation_degree(self, value: Optional[int]) -> None: ...
    @property
    def mode(self) -> Optional[str]:
        """
        The tracking mode associated with the Data Section of the segment.

        Examples: SEQUENTIAL, SINGLE_DIFF
        """
        ...

    @mode.setter
    def mode(self, value: Optional[str]) -> None: ...
    @property
    def participant_1(self) -> str:
        """
        The first participant in a tracking data session.

        Examples: DSS-63-S400K, ROSETTA, \<Quasar catalog name>, 1997-061A, UNKNOWN
        """
        ...

    @participant_1.setter
    def participant_1(self, value: str) -> None: ...
    @property
    def participant_2(self) -> Optional[str]:
        """
        The second participant in a tracking data session.

        Examples: DSS-63-S400K, ROSETTA, \<Quasar catalog name\>, 1997-061A, UNKNOWN
        """
        ...

    @participant_2.setter
    def participant_2(self, value: Optional[str]) -> None: ...
    @property
    def participant_3(self) -> Optional[str]:
        """
        The third participant in a tracking data session.

        Examples: DSS-63-S400K, ROSETTA, \<Quasar catalog name\>, 1997-061A, UNKNOWN
        """
        ...

    @participant_3.setter
    def participant_3(self, value: Optional[str]) -> None: ...
    @property
    def participant_4(self) -> Optional[str]:
        """
        The fourth participant in a tracking data session.

        Examples: DSS-63-S400K, ROSETTA, \<Quasar catalog name\>, 1997-061A, UNKNOWN
        """
        ...

    @participant_4.setter
    def participant_4(self, value: Optional[str]) -> None: ...
    @property
    def participant_5(self) -> Optional[str]:
        """
        The fifth participant in a tracking data session.

        Examples: DSS-63-S400K, ROSETTA, \<Quasar catalog name\>, 1997-061A, UNKNOWN
        """
        ...

    @participant_5.setter
    def participant_5(self, value: Optional[str]) -> None: ...
    @property
    def path(self) -> Optional[str]:
        """
        The signal path by listing the index of each participant in order, separated by commas.

        Examples: 1,2,1
        """
        ...

    @path.setter
    def path(self, value: Optional[str]) -> None: ...
    @property
    def path_1(self) -> Optional[str]:
        """
        The first signal path where the MODE is 'SINGLE_DIFF'.

        Examples: 1,2,1
        """
        ...

    @path_1.setter
    def path_1(self, value: Optional[str]) -> None: ...
    @property
    def path_2(self) -> Optional[str]:
        """
        The second signal path where the MODE is 'SINGLE_DIFF'.

        Examples: 3,1
        """
        ...

    @path_2.setter
    def path_2(self, value: Optional[str]) -> None: ...
    @property
    def range_mode(self) -> Optional[str]:
        """
        The range observable mode.

        Examples: COHERENT, CONSTANT, ONE_WAY
        """
        ...

    @range_mode.setter
    def range_mode(self, value: Optional[str]) -> None: ...
    @property
    def range_modulus(self) -> Optional[float]:
        """
        The modulus of the range observable.

        Examples: 32768.0, 2.0e+23, 0.0, 161.6484
        """
        ...

    @range_modulus.setter
    def range_modulus(self, value: Optional[float]) -> None: ...
    @property
    def range_units(self) -> Optional[str]:
        """
        The units for the range observable.

        Examples: km, s, RU
        """
        ...

    @range_units.setter
    def range_units(self, value: Optional[str]) -> None: ...
    @property
    def receive_band(self) -> Optional[str]:
        """
        The frequency band for received frequencies.

        Examples: S, X, Ka, L, UHF, GREEN
        """
        ...

    @receive_band.setter
    def receive_band(self, value: Optional[str]) -> None: ...
    @property
    def receive_delay_1(self) -> Optional[float]:
        """
        A fixed interval of time, in seconds, required for the signal to travel from the tracking
        point to the receiving electronics for participant 1.

        Units: s

        Examples: 1.23, 0.0326, 0.00777
        """
        ...

    @receive_delay_1.setter
    def receive_delay_1(self, value: Optional[float]) -> None: ...
    @property
    def receive_delay_2(self) -> Optional[float]:
        """
        A fixed interval of time, in seconds, required for the signal to travel from the tracking
        point to the receiving electronics for participant 2.

        Units: s

        Examples: 1.23, 0.0326, 0.00777
        """
        ...

    @receive_delay_2.setter
    def receive_delay_2(self, value: Optional[float]) -> None: ...
    @property
    def receive_delay_3(self) -> Optional[float]:
        """
        A fixed interval of time, in seconds, required for the signal to travel from the tracking
        point to the receiving electronics for participant 3.

        Units: s

        Examples: 1.23, 0.0326, 0.00777
        """
        ...

    @receive_delay_3.setter
    def receive_delay_3(self, value: Optional[float]) -> None: ...
    @property
    def receive_delay_4(self) -> Optional[float]:
        """
        A fixed interval of time, in seconds, required for the signal to travel from the tracking
        point to the receiving electronics for participant 4.

        Units: s

        Examples: 1.23, 0.0326, 0.00777
        """
        ...

    @receive_delay_4.setter
    def receive_delay_4(self, value: Optional[float]) -> None: ...
    @property
    def receive_delay_5(self) -> Optional[float]:
        """
        A fixed interval of time, in seconds, required for the signal to travel from the tracking
        point to the receiving electronics for participant 5.

        Units: s

        Examples: 1.23, 0.0326, 0.00777
        """
        ...

    @receive_delay_5.setter
    def receive_delay_5(self, value: Optional[float]) -> None: ...
    @property
    def reference_frame(self) -> Optional[str]:
        """
        The inertial reference frame to which the antenna frame is referenced.

        Examples: EME2000, ICRF, ITRF1993, ITRF2000, TOD_EARTH
        """
        ...

    @reference_frame.setter
    def reference_frame(self, value: Optional[str]) -> None: ...
    @property
    def start_time(self) -> Optional[str]:
        """
        The UTC start time of the total time span covered by the tracking data.

        Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54, 2006-001T00:00:00Z
        """
        ...

    @start_time.setter
    def start_time(self, value: Optional[str]) -> None: ...
    @property
    def stop_time(self) -> Optional[str]:
        """
        The UTC stop time of the total time span covered by the tracking data.

        Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54, 2006-001T00:00:00Z
        """
        ...

    @stop_time.setter
    def stop_time(self, value: Optional[str]) -> None: ...
    @property
    def time_system(self) -> str:
        """
        The time system used for timetags in the associated Data Section.

        Examples: UTC, TAI, GPS, SCLK
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...
    @property
    def timetag_ref(self) -> Optional[str]:
        """
        A reference for time tags in the tracking data.

        Examples: TRANSMIT, RECEIVE
        """
        ...

    @timetag_ref.setter
    def timetag_ref(self, value: Optional[str]) -> None: ...
    @property
    def track_id(self) -> Optional[str]:
        """
        Unique identifier for the tracking data in the associated data section.

        Examples: 20190918_1200135-0001
        """
        ...

    @track_id.setter
    def track_id(self, value: Optional[str]) -> None: ...
    @property
    def transmit_band(self) -> Optional[str]:
        """
        The frequency band for transmitted frequencies.

        Examples: S, X, Ka, L, UHF, GREEN
        """
        ...

    @transmit_band.setter
    def transmit_band(self, value: Optional[str]) -> None: ...
    @property
    def transmit_delay_1(self) -> Optional[float]:
        """
        A fixed interval of time, in seconds, required for the signal to travel from the
        transmitting electronics to the transmit point for participant 1.

        Units: s

        Examples: 1.23, 0.0326, 0.00077
        """
        ...

    @transmit_delay_1.setter
    def transmit_delay_1(self, value: Optional[float]) -> None: ...
    @property
    def transmit_delay_2(self) -> Optional[float]:
        """
        A fixed interval of time, in seconds, required for the signal to travel from the
        transmitting electronics to the transmit point for participant 2.

        Units: s

        Examples: 1.23, 0.0326, 0.00077
        """
        ...

    @transmit_delay_2.setter
    def transmit_delay_2(self, value: Optional[float]) -> None: ...
    @property
    def transmit_delay_3(self) -> Optional[float]:
        """
        A fixed interval of time, in seconds, required for the signal to travel from the
        transmitting electronics to the transmit point for participant 3.

        Units: s

        Examples: 1.23, 0.0326, 0.00077
        """
        ...

    @transmit_delay_3.setter
    def transmit_delay_3(self, value: Optional[float]) -> None: ...
    @property
    def transmit_delay_4(self) -> Optional[float]:
        """
        A fixed interval of time, in seconds, required for the signal to travel from the
        transmitting electronics to the transmit point for participant 4.

        Units: s

        Examples: 1.23, 0.0326, 0.00077
        """
        ...

    @transmit_delay_4.setter
    def transmit_delay_4(self, value: Optional[float]) -> None: ...
    @property
    def transmit_delay_5(self) -> Optional[float]:
        """
        A fixed interval of time, in seconds, required for the signal to travel from the
        transmitting electronics to the transmit point for participant 5.

        Units: s

        Examples: 1.23, 0.0326, 0.00077
        """
        ...

    @transmit_delay_5.setter
    def transmit_delay_5(self, value: Optional[float]) -> None: ...
    @property
    def turnaround_denominator(self) -> Optional[int]:
        """
        The denominator of the turnaround ratio.

        Examples: 221, 749
        """
        ...

    @turnaround_denominator.setter
    def turnaround_denominator(self, value: Optional[int]) -> None: ...
    @property
    def turnaround_numerator(self) -> Optional[int]:
        """
        The numerator of the turnaround ratio.

        Examples: 240, 880
        """
        ...

    @turnaround_numerator.setter
    def turnaround_numerator(self, value: Optional[int]) -> None: ...

class TdmObservation:
    """
    A single tracking data record consisting of a timetag and a measurement.

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
        Time associated with the tracking observable.
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
        Data section for this TDM segment.
        """
        ...

    @data.setter
    def data(self, value: TdmData) -> None: ...
    @property
    def metadata(self) -> TdmMetadata:
        """
        Metadata section for this TDM segment.
        """
        ...

    @metadata.setter
    def metadata(self, value: TdmMetadata) -> None: ...

class TleParameters:
    """
    TLE Related Parameters.

    This section is only required if MEAN_ELEMENT_THEORY=SGP/SGP4.

    Parameters
    ----------
    ephemeris_type : int, optional
        Ephemeris Type, default value = 0.
    classification_type : str, optional
        Classification Type, default value = U.
    norad_cat_id : int, optional
        NORAD Catalog Number ('Satellite Number').
    element_set_no : int, optional
        Element set number for this satellite.
    rev_at_epoch : int, optional
        Revolution Number.
    bstar : float, optional
        B* drag term in 1/ER (Inverse Earth Radii). Required for SGP4.
    bterm : float, optional
        Ballistic coefficient (m²/kg). Required for SGP4-XP.
    mean_motion_dot : float, optional
        First derivative of mean motion (rev/day²). Required when MEAN_ELEMENT_THEORY = SGP or PPT3.
    mean_motion_ddot : float, optional
        Second derivative of mean motion (rev/day³). Required when MEAN_ELEMENT_THEORY = SGP or PPT3.
    agom : float, optional
        Solar radiation pressure coefficient (m²/kg). Required for SGP4-XP.
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
        MEAN_ELEMENT_THEORY= SGP4-XP: Solar radiation pressure coefficient AY/m, where y =
        reflectivity, A = average cross-sectional area, m = mass. Example values AGOM = 0.01 (rocket
        body) and 0.001 (payload); average value spanning 20,00 catalog objects = 0.0143 m2/kg.

        Examples: 0.01

        Units: m²/kg
        """
        ...

    @agom.setter
    def agom(self, value: Optional[float]) -> None: ...
    @property
    def bstar(self) -> Optional[float]:
        """
        Drag-like ballistic coefficient, required for SGP4 and SGP4-XP mean element models:
        MEAN_ELEMENT_THEORY= SGP4 (BSTAR = drag parameter for SGP4).

        Examples: 0.0001

        Units: 1/ER
        """
        ...

    @bstar.setter
    def bstar(self, value: Optional[float]) -> None: ...
    @property
    def bterm(self) -> Optional[float]:
        """
        Drag-like ballistic coefficient, required for SGP4 and SGP4-XP mean element models:
        MEAN_ELEMENT_THEORY= SGP4-XP (BTERM ballistic coefficient CDA/m, where CD = drag coefficient,
        A = average cross-sectional area, m = mass. Example values for BTERM = 0.02 (rocket body),
        0.0015 (payload); average value spanning 20,00 catalog objects = 0.0286.

        Examples: 0.02

        Units: m²/kg
        """
        ...

    @bterm.setter
    def bterm(self, value: Optional[float]) -> None: ...
    @property
    def classification_type(self) -> Optional[str]:
        """
        Classification Type, default value = U. Some sources suggest the following coding for
        the CLASSIFICATION_TYPE keyword: U=unclassified, S=secret

        Examples: U
        """
        ...

    @classification_type.setter
    def classification_type(self, value: Optional[str]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules.)
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def element_set_no(self) -> Optional[int]:
        """
        Element set number for this satellite. Normally incremented sequentially but may be out of
        sync if it is generated from a backup source. Used to distinguish different TLEs, and
        therefore only meaningful if TLE-based data is being exchanged (i.e., MEAN_ELEMENT_THEORY =
        SGP/SGP4).

        Examples: 999
        """
        ...

    @element_set_no.setter
    def element_set_no(self, value: Optional[int]) -> None: ...
    @property
    def ephemeris_type(self) -> Optional[int]:
        """
        Ephemeris type. Indicates what type of propagator was used to transform the native state to
        the SGP/SGP4 ephemeris state. The default is 0. (See 4.2.4.7 for numeric definitions.)

        - 0 = SGP
        - 2 = SGP4
        - 3 = PPT3
        - 4 = SGP4-XP
        - 6 = Special Perturbations

        Examples: 0
        """
        ...

    @ephemeris_type.setter
    def ephemeris_type(self, value: Optional[int]) -> None: ...
    @property
    def mean_motion_ddot(self) -> Optional[float]:
        """
        MEAN_ELEMENT_THEORY= SGP or PPT3: Second Time Derivative of Mean Motion (i.e., a drag term).
        (See 4.2.4.7 for important details).

        Examples: 0.0

        Units: rev/day³
        """
        ...

    @mean_motion_ddot.setter
    def mean_motion_ddot(self, value: Optional[float]) -> None: ...
    @property
    def mean_motion_dot(self) -> Optional[float]:
        """
        First Time Derivative of the Mean Motion (i.e., a drag term, required when MEAN_ELEMENT_THEORY
        = SGP or PPT3). (See 4.2.4.7 for important details).

        Examples: 0.000001

        Units: rev/day²
        """
        ...

    @mean_motion_dot.setter
    def mean_motion_dot(self, value: Optional[float]) -> None: ...
    @property
    def norad_cat_id(self) -> Optional[int]:
        """
        NORAD Catalog Number (‘Satellite Number’) an integer of up to nine digits. This keyword is
        only required if MEAN_ELEMENT_THEORY=SGP/SGP4.

        Examples: 28893
        """
        ...

    @norad_cat_id.setter
    def norad_cat_id(self, value: Optional[int]) -> None: ...
    @property
    def rev_at_epoch(self) -> Optional[int]:
        """
        Number of revolutions at epoch.

        Examples: 120
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
    USER DEFINED PARAMETERS block (`userDefinedType`).
    User-defined parameters.

    Allow for the exchange of any desired orbital data not already provided in the message.

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
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def user_defined(self) -> dict[str, str]:
        """
        User-defined parameters.
        """
        ...

    @user_defined.setter
    def user_defined(self, value: dict[str, str]) -> None: ...
