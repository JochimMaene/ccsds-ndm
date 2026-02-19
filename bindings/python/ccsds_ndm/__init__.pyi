# Generated content DO NOT EDIT
from typing import Optional, Union
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

class Acm:
    """
    Attitude Comprehensive Message (ACM).

    An ACM specifies the attitude state of a single object at multiple epochs, contained within a
    specified time range. The ACM aggregates and extends APM and AEM content in a single
    comprehensive hybrid message.

    Capabilities include:
    - Optional rate data elements
    - Optional spacecraft physical properties
    - Optional covariance elements
    - Optional maneuver parameters
    - Optional estimator information
    """
    def __init__(header, segment) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @staticmethod
    def from_file(path, format):
        """ """
        ...

    @staticmethod
    def from_str(data, format):
        """ """
        ...

    @property
    def header(self) -> AdmHeader:
        """
        Attitude Comprehensive Message (ACM).

        An ACM specifies the attitude state of a single object at multiple epochs, contained within a
        specified time range. The ACM aggregates and extends APM and AEM content in a single
        comprehensive hybrid message.

        Capabilities include:
        - Optional rate data elements
        - Optional spacecraft physical properties
        - Optional covariance elements
        - Optional maneuver parameters
        - Optional estimator information
        """
        ...

    @header.setter
    def header(self, value: AdmHeader) -> None: ...
    @property
    def id(self) -> Optional[str]:
        """
        The message identifier.
        """
        ...

    @property
    def segment(self) -> AcmSegment:
        """
        ACM Segment.
        """
        ...

    @segment.setter
    def segment(self, value: AcmSegment) -> None: ...
    def to_file(self, path, format, validate=True):
        """ """
        ...

    def to_str(self, format, validate=True):
        """ """
        ...

    def validate(self, strict=True):
        """
        Validate the message against CCSDS rules.

        Parameters
        ----------
        strict : bool, optional
            If True (default), raises ValueError on the first error found.
            If False, returns a list of validation error messages (or None if valid).
        """
        ...

    @property
    def version(self) -> str:
        """
        The message version.
        """
        ...

    @version.setter
    def version(self, value: str) -> None: ...

class AcmAttitudeDetermination:
    """
    ACM Data: Attitude Determination Data Section.
    """
    def __init__(ad_id=None, comment=None) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def ad_epoch(self) -> str | None:
        """
        Attitude determination epoch.
        """
        ...

    @ad_epoch.setter
    def ad_epoch(self, value: str | None) -> None: ...
    @property
    def ad_id(self) -> str | None:
        """
        Attitude determination block identifier.
        """
        ...

    @ad_id.setter
    def ad_id(self, value: str | None) -> None: ...
    @property
    def ad_method(self) -> str | None:
        """
        Attitude determination method.
        """
        ...

    @ad_method.setter
    def ad_method(self, value: str | None) -> None: ...
    @property
    def ad_prev_id(self) -> str | None:
        """
        Previous attitude determination block identifier.
        """
        ...

    @ad_prev_id.setter
    def ad_prev_id(self, value: str | None) -> None: ...
    @property
    def attitude_source(self) -> str | None:
        """
        Source of attitude estimate.
        """
        ...

    @attitude_source.setter
    def attitude_source(self, value: str | None) -> None: ...
    @property
    def attitude_states(self) -> str | None:
        """
        Attitude state type for estimator.
        """
        ...

    @attitude_states.setter
    def attitude_states(self, value: str | None) -> None: ...
    @property
    def attitude_type(self) -> str | None:
        """
        Attitude type keyword.
        """
        ...

    @attitude_type.setter
    def attitude_type(self, value: str | None) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments allowed only immediately after the AD_START keyword.

        Examples: This is a comment.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def cov_type(self) -> str | None:
        """
        Covariance type for estimator.
        """
        ...

    @cov_type.setter
    def cov_type(self, value: str | None) -> None: ...
    @property
    def number_states(self) -> int | None:
        """
        Number of estimator states.
        """
        ...

    @number_states.setter
    def number_states(self, value: int | None) -> None: ...
    @property
    def rate_process_noise_stddev(self) -> float | None:
        """
        Rate process noise standard deviation.
        """
        ...

    @rate_process_noise_stddev.setter
    def rate_process_noise_stddev(self, value: float | None) -> None: ...
    @property
    def rate_states(self) -> str | None:
        """
        Rate states type.
        """
        ...

    @rate_states.setter
    def rate_states(self, value: str | None) -> None: ...
    @property
    def ref_frame_a(self) -> str | None:
        """
        Source reference frame.
        """
        ...

    @ref_frame_a.setter
    def ref_frame_a(self, value: str | None) -> None: ...
    @property
    def ref_frame_b(self) -> str | None:
        """
        Destination reference frame.
        """
        ...

    @ref_frame_b.setter
    def ref_frame_b(self, value: str | None) -> None: ...
    @property
    def sensors(self) -> list[AcmSensor]:
        """
        Sensor data blocks.
        """
        ...

    @sensors.setter
    def sensors(self, value: list[AcmSensor]) -> None: ...
    @property
    def sigma_u(self) -> float | None:
        """
        Rate random walk sigma_u.
        """
        ...

    @sigma_u.setter
    def sigma_u(self, value: float | None) -> None: ...
    @property
    def sigma_v(self) -> float | None:
        """
        Angle random walk sigma_v.
        """
        ...

    @sigma_v.setter
    def sigma_v(self, value: float | None) -> None: ...

class AcmAttitudeState:
    """
    ACM Data: Attitude State Time History Section.
    """
    def __init__(
        ref_frame_a, ref_frame_b, att_type, att_lines, comment=None
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def att_basis(self) -> str | None:
        """
        Basis of this attitude state data.
        """
        ...

    @att_basis.setter
    def att_basis(self, value: str | None) -> None: ...
    @property
    def att_basis_id(self) -> str | None:
        """
        Basis dataset identifier.
        """
        ...

    @att_basis_id.setter
    def att_basis_id(self, value: str | None) -> None: ...
    @property
    def att_id(self) -> str | None:
        """
        Attitude state block identifier.
        """
        ...

    @att_id.setter
    def att_id(self, value: str | None) -> None: ...
    @property
    def att_lines(self) -> list[list[float]]:
        """
        Data lines that consist of attitude data followed by rate data. (For the data units, see
        above [ATT_TYPE and RATE_TYPE keywords]).
        """
        ...

    @att_lines.setter
    def att_lines(self, value: list[list[float]]) -> None: ...
    @property
    def att_prev_id(self) -> str | None:
        """
        Previous attitude state block identifier.
        """
        ...

    @att_prev_id.setter
    def att_prev_id(self, value: str | None) -> None: ...
    @property
    def att_type(self) -> str:
        """
        Type of attitude data, selected per annex B, subsection B4. Attitude data must always be
        listed before rate data. The units that shall be used are given in annex B, subsection B4.

        Examples: QUATERNION, EULER_ANGLES, DCM
        """
        ...

    @att_type.setter
    def att_type(self, value: str) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments allowed only immediately after the ATT_START keyword.

        Examples: This is a comment.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def euler_rot_seq(self) -> str | None:
        """
        Optional Euler rotation sequence.
        """
        ...

    @euler_rot_seq.setter
    def euler_rot_seq(self, value: str | None) -> None: ...
    @property
    def number_states(self) -> int:
        """
        Number of data states included. States to be included are attitude states and optional rate
        states.

        Examples: 3, 4, 7
        """
        ...

    @number_states.setter
    def number_states(self, value: int) -> None: ...
    @property
    def rate_type(self) -> str | None:
        """
        Optional rate state type.
        """
        ...

    @rate_type.setter
    def rate_type(self, value: str | None) -> None: ...
    @property
    def ref_frame_a(self) -> str:
        """
        Name of the reference frame that defines the starting point of the transformation. The set
        of allowed values is described in annex B, subsection B3.

        Examples: J2000
        """
        ...

    @ref_frame_a.setter
    def ref_frame_a(self, value: str) -> None: ...
    @property
    def ref_frame_b(self) -> str:
        """
        Name of the reference frame that defines the end point of the transformation. The set of
        allowed values is described in annex B, subsection B3.

        Examples: SC_BODY_1
        """
        ...

    @ref_frame_b.setter
    def ref_frame_b(self, value: str) -> None: ...

class AcmCovarianceMatrix:
    """
    ACM Data: Covariance Time History Section.
    """
    def __init__(
        cov_basis, cov_ref_frame, cov_type, cov_lines, comment=None
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments allowed only immediately after the COV_START keyword.

        Examples: THIS is a comment.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def cov_basis(self) -> str:
        """
        Basis of this covariance time history data.

        Examples: PREDICTED, DETERMINED_GND, DETERMINED_OBC, SIMULATED
        """
        ...

    @cov_basis.setter
    def cov_basis(self, value: str) -> None: ...
    @property
    def cov_confidence(self) -> float | None:
        """
        Optional covariance confidence.
        """
        ...

    @cov_confidence.setter
    def cov_confidence(self, value: float | None) -> None: ...
    @property
    def cov_lines(self) -> list[list[float]]:
        """
        Covariance data lines (diagonal terms only). (For the data units, see annex B, subsection
        B6.)
        """
        ...

    @cov_lines.setter
    def cov_lines(self, value: list[list[float]]) -> None: ...
    @property
    def cov_ref_frame(self) -> str:
        """
        Reference frame of the covariance time history. The full set of values is enumerated in
        annex B, subsection B3.

        Examples: SC_BODY_1
        """
        ...

    @cov_ref_frame.setter
    def cov_ref_frame(self, value: str) -> None: ...
    @property
    def cov_type(self) -> str:
        """
        Indicates covariance composition. Select from annex B, subsection B6.

        Examples: ANGLE, ANGLE_GYROBIAS
        """
        ...

    @cov_type.setter
    def cov_type(self, value: str) -> None: ...

class AcmData:
    """
    ACM Data Section.
    """
    def __init__(
        att=None, phys=None, cov=None, man=None, ad=None, user=None
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def ad(self) -> AcmAttitudeDetermination | None:
        """
        A single optional attitude determination section.
        """
        ...

    @ad.setter
    def ad(self, value: AcmAttitudeDetermination | None) -> None: ...
    @property
    def att(self) -> list[AcmAttitudeState]:
        """
        One or more optional attitude state time histories (each consisting of one or more attitude
        states).
        """
        ...

    @att.setter
    def att(self, value: list[AcmAttitudeState]) -> None: ...
    @property
    def cov(self) -> list[AcmCovarianceMatrix]:
        """
        One or more optional covariance time histories (each consisting of one or more covariance
        matrix diagonals).
        """
        ...

    @cov.setter
    def cov(self, value: list[AcmCovarianceMatrix]) -> None: ...
    @property
    def man(self) -> list[AcmManeuverParameters]:
        """
        One or more optional maneuver specification section(s).
        """
        ...

    @man.setter
    def man(self, value: list[AcmManeuverParameters]) -> None: ...
    @property
    def phys(self) -> AcmPhysicalDescription | None:
        """
        A single space object physical characteristics section.
        """
        ...

    @phys.setter
    def phys(self, value: AcmPhysicalDescription | None) -> None: ...
    @property
    def user(self) -> UserDefined | None:
        """
        A single user-defined Data section.
        """
        ...

    @user.setter
    def user(self, value: UserDefined | None) -> None: ...
    def validate(self, metadata):
        """
        Validate the data section against CCSDS rules.
        """
        ...

class AcmManeuverParameters:
    """
    ACM Data: Maneuver Specification Section.
    """
    def __init__(man_id=None, comment=None) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def actuator_used(self) -> str | None:
        """
        Actuator used for this maneuver.
        """
        ...

    @actuator_used.setter
    def actuator_used(self, value: str | None) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments allowed only immediately after the MAN_START keyword.

        Examples: This is a comment.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def man_begin_time(self) -> str | None:
        """
        Maneuver begin time (relative or absolute epoch string).
        """
        ...

    @man_begin_time.setter
    def man_begin_time(self, value: str | None) -> None: ...
    @property
    def man_duration(self) -> float | None:
        """
        Maneuver duration in seconds.
        """
        ...

    @man_duration.setter
    def man_duration(self, value: float | None) -> None: ...
    @property
    def man_end_time(self) -> str | None:
        """
        Maneuver end time (relative or absolute epoch string).
        """
        ...

    @man_end_time.setter
    def man_end_time(self, value: str | None) -> None: ...
    @property
    def man_id(self) -> str | None:
        """
        Maneuver block identifier.
        """
        ...

    @man_id.setter
    def man_id(self, value: str | None) -> None: ...
    @property
    def man_prev_id(self) -> str | None:
        """
        Previous maneuver block identifier.
        """
        ...

    @man_prev_id.setter
    def man_prev_id(self, value: str | None) -> None: ...
    @property
    def man_purpose(self) -> str | None:
        """
        Maneuver purpose.
        """
        ...

    @man_purpose.setter
    def man_purpose(self, value: str | None) -> None: ...
    @property
    def target_attitude(self) -> list[float] | None:
        """
        Target attitude quaternion-like 4-vector.
        """
        ...

    @target_attitude.setter
    def target_attitude(self, value: list[float] | None) -> None: ...
    @property
    def target_mom_frame(self) -> str | None:
        """
        Reference frame of target momentum.
        """
        ...

    @target_mom_frame.setter
    def target_mom_frame(self, value: str | None) -> None: ...
    @property
    def target_momentum(self) -> list[float] | None:
        """
        Target momentum vector [x, y, z].
        """
        ...

    @target_momentum.setter
    def target_momentum(self, value: list[float] | None) -> None: ...
    @property
    def target_spinrate(self) -> float | None:
        """
        Target spin rate (deg/s).
        """
        ...

    @target_spinrate.setter
    def target_spinrate(self, value: float | None) -> None: ...

class AcmMetadata:
    """
    ACM Metadata Section.
    """
    def __init__(
        object_name,
        epoch_tzero,
        time_system=None,
        international_designator=None,
        comment=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def acm_data_elements(self) -> str | None:
        """
        Included ACM data block elements.
        """
        ...

    @acm_data_elements.setter
    def acm_data_elements(self, value: str | None) -> None: ...
    @property
    def catalog_name(self) -> str | None:
        """
        Satellite catalog source.
        """
        ...

    @catalog_name.setter
    def catalog_name(self, value: str | None) -> None: ...
    @property
    def center_name(self) -> str | None:
        """
        Central body name.
        """
        ...

    @center_name.setter
    def center_name(self, value: str | None) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (allowed only at the beginning of the ACM Metadata). Each comment line shall begin
        with this keyword.

        Examples: This is a comment.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def epoch_tzero(self) -> str:
        """
        Epoch from which all ACM relative times are referenced. (For format specification, see
        6.8.9.) The time scale for EPOCH_TZERO is the one specified by ‘TIME_SYSTEM’ keyword in the
        Metadata section.

        Examples: 2016-11-10T00:00:00
        """
        ...

    @epoch_tzero.setter
    def epoch_tzero(self, value: str) -> None: ...
    @property
    def international_designator(self) -> str | None:
        """
        Free text field containing an international designator for the object as assigned by the UN
        Committee on Space Research (COSPAR) and the US National Space Science Data Center (NSSDC).
        Such designator values have the following COSPAR format: YYYY-NNNP{PP}, where: YYYY = Year
        of launch. NNN = Three-digit serial number of launch in year YYYY (with leading zeros).
        P{PP} = At least one capital letter for the identification of the part brought into space
        by the launch. In cases in which the object has no international designator, the value
        UNKNOWN may be used. NOTE – The international designator is typically specified by
        ‘OBJECT_ID’ in the APM and AEM.

        Examples: 2000-052A, 1996-068A, 2000-053A, 1996-008A, UNKNOWN
        """
        ...

    @international_designator.setter
    def international_designator(self, value: str | None) -> None: ...
    @property
    def next_leap_epoch(self) -> str | None:
        """
        Epoch of the next leap second.
        """
        ...

    @next_leap_epoch.setter
    def next_leap_epoch(self, value: str | None) -> None: ...
    @property
    def next_leap_taimutc(self) -> float | None:
        """
        Difference (TAI - UTC) at NEXT_LEAP_EPOCH, seconds.
        """
        ...

    @next_leap_taimutc.setter
    def next_leap_taimutc(self, value: float | None) -> None: ...
    @property
    def object_designator(self) -> str | None:
        """
        Unique object designator in the source catalog.
        """
        ...

    @object_designator.setter
    def object_designator(self, value: str | None) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Free-text field containing the name of the object. There is no CCSDS-based restriction on
        the value for this keyword, but it is recommended to use names from either the UN Office of
        Outer Space Affairs designator index (reference `[2]`), which include Object name and
        international designator), the spacecraft operator, or a State Actor or commercial Space
        Situational Awareness (SSA) provider maintaining the ‘CATALOG_NAME’ space catalog. If the
        object name is not known (uncorrelated object), ‘UNKNOWN’ may be used (or this keyword
        omitted).

        Examples: SPOT, ENVISAT, IRIDIUM, INTELSAT
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def odm_msg_link(self) -> str | None:
        """
        Linked Orbit Data Message identifier.
        """
        ...

    @odm_msg_link.setter
    def odm_msg_link(self, value: str | None) -> None: ...
    @property
    def originator_address(self) -> str | None:
        """
        Originator point-of-contact address.
        """
        ...

    @originator_address.setter
    def originator_address(self, value: str | None) -> None: ...
    @property
    def originator_email(self) -> str | None:
        """
        Originator point-of-contact email.
        """
        ...

    @originator_email.setter
    def originator_email(self, value: str | None) -> None: ...
    @property
    def originator_phone(self) -> str | None:
        """
        Originator point-of-contact phone.
        """
        ...

    @originator_phone.setter
    def originator_phone(self, value: str | None) -> None: ...
    @property
    def originator_poc(self) -> str | None:
        """
        Originator point-of-contact.
        """
        ...

    @originator_poc.setter
    def originator_poc(self, value: str | None) -> None: ...
    @property
    def originator_position(self) -> str | None:
        """
        Originator point-of-contact position.
        """
        ...

    @originator_position.setter
    def originator_position(self, value: str | None) -> None: ...
    @property
    def start_time(self) -> str | None:
        """
        Earliest data time in this ACM.
        """
        ...

    @start_time.setter
    def start_time(self, value: str | None) -> None: ...
    @property
    def stop_time(self) -> str | None:
        """
        Latest data time in this ACM.
        """
        ...

    @stop_time.setter
    def stop_time(self, value: str | None) -> None: ...
    @property
    def taimutc_at_tzero(self) -> float | None:
        """
        Difference (TAI - UTC) at EPOCH_TZERO, seconds.
        """
        ...

    @taimutc_at_tzero.setter
    def taimutc_at_tzero(self, value: float | None) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system used for metadata, attitude data, covariance data. The set of allowed values is
        described in annex B, subsection B2.

        Examples: UTC, TAI
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...
    def validate(self):
        """
        Validate the metadata section against CCSDS rules.
        """
        ...

class AcmPhysicalDescription:
    """
    ACM Data: Space Object Physical Characteristics Section.
    """
    def __init__(comment=None) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments allowed only immediately after the PHYS_START keyword.

        Examples: This is a comment.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def cp(self) -> list[float] | None:
        """
        Center-of-pressure vector [x, y, z] in meters.
        """
        ...

    @cp.setter
    def cp(self, value: list[float] | None) -> None: ...
    @property
    def cp_ref_frame(self) -> str | None:
        """
        Center-of-pressure reference frame.
        """
        ...

    @cp_ref_frame.setter
    def cp_ref_frame(self, value: str | None) -> None: ...
    @property
    def drag_coeff(self) -> float | None:
        """
        Drag coefficient.
        """
        ...

    @drag_coeff.setter
    def drag_coeff(self, value: float | None) -> None: ...
    @property
    def dry_mass(self) -> float | None:
        """
        Dry mass (kg).
        """
        ...

    @dry_mass.setter
    def dry_mass(self, value: float | None) -> None: ...
    @property
    def inertia_ref_frame(self) -> str | None:
        """
        Inertia reference frame.
        """
        ...

    @inertia_ref_frame.setter
    def inertia_ref_frame(self, value: str | None) -> None: ...
    @property
    def ixx(self) -> float | None:
        """
        Moment of inertia IXX.
        """
        ...

    @ixx.setter
    def ixx(self, value: float | None) -> None: ...
    @property
    def ixy(self) -> float | None:
        """
        Product of inertia IXY.
        """
        ...

    @ixy.setter
    def ixy(self, value: float | None) -> None: ...
    @property
    def ixz(self) -> float | None:
        """
        Product of inertia IXZ.
        """
        ...

    @ixz.setter
    def ixz(self, value: float | None) -> None: ...
    @property
    def iyy(self) -> float | None:
        """
        Moment of inertia IYY.
        """
        ...

    @iyy.setter
    def iyy(self, value: float | None) -> None: ...
    @property
    def iyz(self) -> float | None:
        """
        Product of inertia IYZ.
        """
        ...

    @iyz.setter
    def iyz(self, value: float | None) -> None: ...
    @property
    def izz(self) -> float | None:
        """
        Moment of inertia IZZ.
        """
        ...

    @izz.setter
    def izz(self, value: float | None) -> None: ...
    @property
    def wet_mass(self) -> float | None:
        """
        Wet mass (kg).
        """
        ...

    @wet_mass.setter
    def wet_mass(self, value: float | None) -> None: ...

class AcmSegment:
    def __init__(metadata, data) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def data(self) -> AcmData:
        """
        ACM Data Section.
        """
        ...

    @data.setter
    def data(self, value: AcmData) -> None: ...
    @property
    def metadata(self) -> AcmMetadata:
        """
        ACM Metadata Section.
        """
        ...

    @metadata.setter
    def metadata(self, value: AcmMetadata) -> None: ...
    def validate(self, header):
        """
        Validate the segment against CCSDS rules.
        """
        ...

class AcmSensor:
    """
    ACM Data: Sensor Data Section.
    """
    def __init__(
        sensor_number,
        sensor_used=None,
        sensor_noise_stddev=None,
        sensor_frequency=None,
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
        Comments allowed only immediately after the SENSOR_START keyword.

        Examples: This is a comment.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def sensor_frequency(self) -> float | None:
        """
        Sensor frequency in Hz.
        """
        ...

    @sensor_frequency.setter
    def sensor_frequency(self, value: float | None) -> None: ...
    @property
    def sensor_noise_stddev(self) -> list[float] | None:
        """
        Sensor noise standard deviation values.
        """
        ...

    @sensor_noise_stddev.setter
    def sensor_noise_stddev(self, value: list[float] | None) -> None: ...
    @property
    def sensor_number(self) -> int:
        """
        Sensor number. Multiple sensors may be included, with each having a unique, ascending
        number.

        Examples: 1, 2, 3
        """
        ...

    @sensor_number.setter
    def sensor_number(self, value: int) -> None: ...
    @property
    def sensor_used(self) -> str | None:
        """
        Sensor type identifier.
        """
        ...

    @sensor_used.setter
    def sensor_used(self, value: str | None) -> None: ...

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
    def area_drg(self) -> Optional[float]:
        """
        The effective area of the object exposed to atmospheric drag. (See annex E for
        definition.)

        Units: m²
        """
        ...

    @area_drg.setter
    def area_drg(self, value: Optional[float]) -> None: ...
    @property
    def area_pc(self) -> Optional[float]:
        """
        The actual area of the object. (See annex E for definition.)

        Units: m²
        """
        ...

    @area_pc.setter
    def area_pc(self, value: Optional[float]) -> None: ...
    @property
    def area_srp(self) -> Optional[float]:
        """
        The effective area of the object exposed to solar radiation pressure. (See annex E for
        definition.)

        Units: m²
        """
        ...

    @area_srp.setter
    def area_srp(self, value: Optional[float]) -> None: ...
    @property
    def cd_area_over_mass(self) -> Optional[float]:
        """
        The object's CD•A/m used to propagate the state vector and covariance to TCA. (See
        annex E for definition.)

        Units: m²/kg
        """
        ...

    @cd_area_over_mass.setter
    def cd_area_over_mass(self, value: Optional[float]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 6.3.4 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def cr_area_over_mass(self) -> Optional[float]:
        """
        The object's CR•A/m used to propagate the state vector and covariance to TCA. (See
        annex E for definition.)

        Units: m²/kg
        """
        ...

    @cr_area_over_mass.setter
    def cr_area_over_mass(self, value: Optional[float]) -> None: ...
    @property
    def mass(self) -> Optional[float]:
        """
        The mass of the object.

        Units: kg
        """
        ...

    @mass.setter
    def mass(self, value: Optional[float]) -> None: ...
    @property
    def sedr(self) -> Optional[float]:
        """
        The amount of energy being removed from the object's orbit by atmospheric drag. This
        value is an average calculated during the OD.

        Units: W/kg
        """
        ...

    @sedr.setter
    def sedr(self, value: Optional[float]) -> None: ...
    @property
    def thrust_acceleration(self) -> Optional[float]:
        """
        The object's acceleration due to in-track thrust used to propagate the state vector and
        covariance to TCA. (See annex E for definition.)

        Units: m/s²
        """
        ...

    @thrust_acceleration.setter
    def thrust_acceleration(self, value: Optional[float]) -> None: ...

class AdmHeader:
    """
    Represents the `admHeader` complex type from the XSD.
    """
    def __init__(
        creation_date, originator, classification=None, message_id=None, comment=None
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def classification(self) -> Optional[str]:
        """
        User-defined free-text message classification/caveats of this ADM. It is recommended
        that selected values be pre-coordinated between exchanging entities by mutual agreement.

        Examples: SBU, ‘Operator-proprietary data; secondary distribution not permitted’
        """
        ...

    @classification.setter
    def classification(self, value: Optional[str]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        User-defined comments. (See 7.8 for formatting rules.)

        Examples: This is a comment
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def creation_date(self) -> str:
        """
        File creation date/time in UTC. (For format specification, see 6.8.9.)

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @creation_date.setter
    def creation_date(self, value: str) -> None: ...
    @property
    def message_id(self) -> Optional[str]:
        """
        ID that uniquely identifies a message from a given originator. The format and content of
        the message identifier value are at the discretion of the originator.

        Examples: APM_201113719185, ABC-12_34
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

class Aem:
    """
    Attitude Ephemeris Message (AEM).

    An AEM specifies the attitude state of a single object at multiple epochs, contained within a
    specified time range. The AEM is suited to interagency exchanges that involve automated
    interaction and require higher fidelity or higher precision dynamic modeling than is
    possible with the APM.

    The AEM allows for dynamic modeling of any number of torques (solar pressure, atmospheric
    torques, magnetics, etc.). It requires the use of an interpolation technique to interpret
    the attitude state at times different from the tabular epochs.
    """
    def __init__(header, segments) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @staticmethod
    def from_file(path, format):
        """ """
        ...

    @staticmethod
    def from_str(data, format):
        """ """
        ...

    @property
    def header(self) -> AdmHeader:
        """
        Attitude Ephemeris Message (AEM).

        An AEM specifies the attitude state of a single object at multiple epochs, contained within a
        specified time range. The AEM is suited to interagency exchanges that involve automated
        interaction and require higher fidelity or higher precision dynamic modeling than is
        possible with the APM.

        The AEM allows for dynamic modeling of any number of torques (solar pressure, atmospheric
        torques, magnetics, etc.). It requires the use of an interpolation technique to interpret
        the attitude state at times different from the tabular epochs.
        """
        ...

    @header.setter
    def header(self, value: AdmHeader) -> None: ...
    @property
    def id(self) -> Optional[str]:
        """
        The message identifier.
        """
        ...

    @id.setter
    def id(self, value: Optional[str]) -> None: ...
    @property
    def segments(self) -> list[AemSegment]:
        """
        AEM Segments.
        """
        ...

    @segments.setter
    def segments(self, value: list[AemSegment]) -> None: ...
    def to_file(self, path, format, validate=True):
        """
        Write to file.
        """
        ...

    def to_str(self, format, validate=True):
        """
        Serialize to string.
        """
        ...

    def validate(self, strict=True):
        """
        Validate the message against CCSDS rules.

        Parameters
        ----------
        strict : bool, optional
            If True (default), raises ValueError on the first error found.
            If False, returns a list of validation error messages (or None if valid).
        """
        ...

    @property
    def version(self) -> str:
        """
        The message version.
        """
        ...

    @version.setter
    def version(self, value: str) -> None: ...

class AemData:
    """
    AEM Data Section.
    """
    def __init__(attitude_states, attitude_type=None, comment=None) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def attitude_states(self) -> list[AttitudeState]:
        """
        Attitude ephemeris data lines.
        """
        ...

    @attitude_states.setter
    def attitude_states(self, value: list[AttitudeState]) -> None: ...
    @property
    def attitude_states_epochs(self) -> list[str]:
        """
        Epochs for attitude states (ISO 8601).
        """
        ...

    @attitude_states_epochs.setter
    def attitude_states_epochs(self, value: list[str]) -> None: ...
    @property
    def attitude_states_numpy(self) -> numpy.ndarray:
        """
        Get attitude states as a 2D NumPy array.

        Use `attitude_states_epochs` for the corresponding epochs.

        Supports all AEM attitude state types, but all rows must be of the same type.
        """
        ...

    @attitude_states_numpy.setter
    def attitude_states_numpy(self, value: numpy.ndarray) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments allowed only at the beginning of the Data section. Each comment line shall begin
        with this keyword.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @staticmethod
    def from_numpy(epochs, array, attitude_type=None, comment=None):
        """ """
        ...

    def validate(self, attitude_type):
        """
        Validate the data section against CCSDS rules.
        """
        ...

class AemMetadata:
    """
    AEM Metadata Section.
    """
    def __init__(
        object_name,
        object_id,
        ref_frame_a=None,
        ref_frame_b=None,
        start_time=None,
        stop_time=None,
        time_system=None,
        attitude_type=...,
        center_name=None,
        useable_start_time=None,
        useable_stop_time=None,
        euler_rot_seq=None,
        angvel_frame=None,
        interpolation_method=None,
        interpolation_degree=None,
        comment=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def angvel_frame(self) -> str | None:
        """
        The frame of reference in which angular velocity data are specified. The set of allowed
        values is described in annex B, subsection B3. This keyword is applicable only if
        ATTITUDE_TYPE specifies the use of angular velocities in conjunction with either
        quaternions or Euler angles.

        Examples: ICRF, SC_BODY_1
        """
        ...

    @angvel_frame.setter
    def angvel_frame(self, value: str | None) -> None: ...
    @property
    def attitude_type(self) -> str:
        """
        The type of information contained in the data lines. This keyword must have a value from the
        set specified at the right. (See table 4-4 for details of the data contained in each line.)

        Examples: QUATERNION, QUATERNION/DERIVATIVE, QUATERNION/ANGVEL, EULER_ANGLE,
        EULER_ANGLE/DERIVATIVE, EULER_ANGLE/ANGVEL, SPIN, SPIN/NUTATION, SPIN/NUTATION_MOM
        """
        ...

    @attitude_type.setter
    def attitude_type(self, value: str) -> None: ...
    @property
    def center_name(self) -> str | None:
        """
        Celestial body orbited by the object, which may be a natural solar system body (planets,
        asteroids, comets, and natural satellites), including any planet barycenter or the solar
        system barycenter. The set of allowed values is described in annex B, subsection B8.

        Examples: EARTH, STS-106
        """
        ...

    @center_name.setter
    def center_name(self, value: str | None) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments allowed only at the beginning of the Metadata section. Each comment line shall
        begin with this keyword.

        Examples: This is a comment.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def euler_rot_seq(self) -> str | None:
        """
        Rotation sequence that defines the REF_FRAME_A to REF_FRAME_B transformation. The order of
        the transformation is from left to right, where the leftmost letter (X, Y, or Z) represents
        the rotation axis of the first rotation, the second letter (X, Y, or Z) represents the
        rotation axis of the second rotation, and the third letter (X, Y, or Z) represents the
        rotation axis of the third rotation. This keyword is applicable only if ATTITUDE_TYPE
        specifies the use of Euler angles.

        Examples: ZXZ, XYZ
        """
        ...

    @euler_rot_seq.setter
    def euler_rot_seq(self, value: str | None) -> None: ...
    @property
    def interpolation_degree(self) -> int | None:
        """
        Recommended interpolation degree for attitude ephemeris data in the block immediately
        following this metadata block. It must be an integer value. This keyword must be used if
        the ‘INTERPOLATION_METHOD’ keyword is used.

        Examples: 1, 5
        """
        ...

    @interpolation_degree.setter
    def interpolation_degree(self, value: int | None) -> None: ...
    @property
    def interpolation_method(self) -> str | None:
        """
        Recommended interpolation method for attitude ephemeris data in the block immediately
        following this metadata block.

        Examples: LINEAR, HERMITE, LAGRANGE
        """
        ...

    @interpolation_method.setter
    def interpolation_method(self, value: str | None) -> None: ...
    @property
    def object_id(self) -> str:
        """
        Spacecraft identifier of the object corresponding to the attitude data to be given. While
        there is no CCSDS-based restriction on the value for this keyword, it is recommended to use
        international designators from the UN Office of Outer Space Affairs (reference [ADM-2]).
        Recommended values have the format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three-
        digit serial number of launch in year YYYY (with leading zeros). P{PP} = At least one
        capital letter for the identification of the part brought into space by the launch. In
        cases in which the asset is not listed in reference [ADM-2], the UN Office of Outer Space
        Affairs designator index format is not used, or the content cannot be disclosed, the value
        should be set to UNKNOWN.

        Examples: 2000-052A
        """
        ...

    @object_id.setter
    def object_id(self, value: str) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Spacecraft name for which the attitude state is provided. While there is no CCSDS-based
        restriction on the value for this keyword, it is recommended to use names from the UN
        Office of Outer Space Affairs designator index (reference [ADM-2], which include Object
        name and international designator). When OBJECT_NAME is not known or cannot be disclosed,
        the value should be set to UNKNOWN.

        Examples: EUTELSAT W1
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def ref_frame_a(self) -> str:
        """
        Name of the reference frame that defines the starting point of the transformation. The set
        of allowed values is described in annex B, subsection B3.

        Examples: ICRF, SC_BODY_1, INSTRUMENT_A
        """
        ...

    @ref_frame_a.setter
    def ref_frame_a(self, value: str) -> None: ...
    @property
    def ref_frame_b(self) -> str:
        """
        Name of the reference frame that defines the end point of the transformation. The set of
        allowed values is described in annex B, subsection B3.

        Examples: SC_BODY_1, INSTRUMENT_A
        """
        ...

    @ref_frame_b.setter
    def ref_frame_b(self, value: str) -> None: ...
    @property
    def start_time(self) -> str:
        """
        Start of TOTAL time span covered by attitude ephemeris data immediately following this
        metadata block.

        Examples: 1996-12-18T14:28:15.11
        """
        ...

    @start_time.setter
    def start_time(self, value: str) -> None: ...
    @property
    def stop_time(self) -> str:
        """
        End of TOTAL time span covered by the attitude ephemeris data immediately following this
        metadata block.

        Examples: 1996-12-18T14:28:15.11
        """
        ...

    @stop_time.setter
    def stop_time(self, value: str) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system used for both attitude ephemeris data and metadata. The set of allowed values
        is described in annex B, subsection B2.

        Examples: UTC, TAI
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...
    @property
    def useable_start_time(self) -> str | None:
        """
        Optional start of USEABLE time span covered by attitude ephemeris data immediately
        following this metadata block. To allow for proper interpolation near the beginning/end of
        the attitude ephemeris data block, it may be necessary to utilize this keyword with values
        within the time span covered by the attitude ephemeris data records as denoted by the
        START/STOP_TIME time tags. The USEABLE_START_TIME time tag of a new block of ephemeris data
        must be greater than or equal to the USEABLE_STOP_TIME time tag of the previous block.

        Examples: 1996-12-18T14:28:15.11
        """
        ...

    @useable_start_time.setter
    def useable_start_time(self, value: str | None) -> None: ...
    @property
    def useable_stop_time(self) -> str | None:
        """
        Optional stop of USEABLE time span covered by attitude ephemeris data immediately following
        this metadata block. (See also USEABLE_START_TIME.)

        Examples: 1996-12-18T14:28:15.11
        """
        ...

    @useable_stop_time.setter
    def useable_stop_time(self, value: str | None) -> None: ...
    def validate(self):
        """
        Validate the metadata section against CCSDS rules.
        """
        ...

class AemSegment:
    def __init__(metadata, data) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def data(self) -> AemData:
        """
        AEM Data Section.
        """
        ...

    @data.setter
    def data(self, value: AemData) -> None: ...
    @property
    def metadata(self) -> AemMetadata:
        """
        AEM Metadata Section.
        """
        ...

    @metadata.setter
    def metadata(self, value: AemMetadata) -> None: ...
    def validate(self):
        """
        Validate the segment against CCSDS rules.
        """
        ...

class AngVelState:
    """
    Angular velocity vector.

    All mandatory elements are to be provided if the block is present.
    (See annex F for conventions and further detail.)
    """
    def __init__(
        ref_frame_a, ref_frame_b, angvel_frame, angvel_x, angvel_y, angvel_z, comment
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def angvel_frame(self) -> str:
        """
        Reference frame in which the components of the angular velocity vector are given. The set
        of allowed values is described in annex B, subsection B3.
        """
        ...

    @angvel_frame.setter
    def angvel_frame(self, value: str) -> None: ...
    @property
    def angvel_x(self) -> float:
        """
        Component of the angular velocity vector on the X axis.

        Units: deg/s
        """
        ...

    @angvel_x.setter
    def angvel_x(self, value: float) -> None: ...
    @property
    def angvel_y(self) -> float:
        """
        Component of the angular velocity vector on the Y axis.

        Units: deg/s
        """
        ...

    @angvel_y.setter
    def angvel_y(self, value: float) -> None: ...
    @property
    def angvel_z(self) -> float:
        """
        Component of the angular velocity vector on the Z axis.

        Units: deg/s
        """
        ...

    @angvel_z.setter
    def angvel_z(self, value: float) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        One or more comment line(s). Each comment line shall begin with this keyword.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def ref_frame_a(self) -> str:
        """
        Name of the reference frame that defines the starting point of the transformation. The set
        of allowed values is described in annex B, subsection B3.
        """
        ...

    @ref_frame_a.setter
    def ref_frame_a(self, value: str) -> None: ...
    @property
    def ref_frame_b(self) -> str:
        """
        Name of the reference frame that defines the end point of the transformation. The set of
        allowed values is described in annex B, subsection B3.
        """
        ...

    @ref_frame_b.setter
    def ref_frame_b(self, value: str) -> None: ...

class Apm:
    """
    Attitude Parameter Message (APM).

    An APM specifies the attitude state of a single object at a specified epoch. This message
    is suited to interagency exchanges that involve automated interaction and/or human
    interaction, and/or human interaction, and do not require high-fidelity dynamic modeling.

    The APM requires the use of a propagation technique to determine the attitude state at
    times different from the specified epoch.
    """
    def __init__(header, segment) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @staticmethod
    def from_file(path, format):
        """ """
        ...

    @staticmethod
    def from_str(data, format):
        """ """
        ...

    @property
    def header(self) -> AdmHeader:
        """
        Attitude Parameter Message (APM).

        An APM specifies the attitude state of a single object at a specified epoch. This message
        is suited to interagency exchanges that involve automated interaction and/or human
        interaction, and/or human interaction, and do not require high-fidelity dynamic modeling.

        The APM requires the use of a propagation technique to determine the attitude state at
        times different from the specified epoch.
        """
        ...

    @header.setter
    def header(self, value: AdmHeader) -> None: ...
    @property
    def id(self) -> Optional[str]:
        """
        The message identifier.
        """
        ...

    @id.setter
    def id(self, value: Optional[str]) -> None: ...
    @property
    def segment(self) -> ApmSegment:
        """
        APM Segment.
        """
        ...

    @segment.setter
    def segment(self, value: ApmSegment) -> None: ...
    def to_file(self, path, format, validate=True):
        """
        Write to file.
        """
        ...

    def to_str(self, format, validate=True):
        """
        Serialize to string.
        """
        ...

    def validate(self, strict=True):
        """
        Validate the message against CCSDS rules.

        Parameters
        ----------
        strict : bool, optional
            If True (default), raises ValueError on the first error found.
            If False, returns a list of validation error messages (or None if valid).
        """
        ...

    @property
    def version(self) -> str:
        """
        The message version.
        """
        ...

    @version.setter
    def version(self, value: str) -> None: ...

class ApmData:
    """
    APM Data Section.
    """
    def __init__(
        epoch,
        quaternion_state=None,
        euler_angle_state=None,
        angular_velocity=None,
        spin=None,
        inertia=None,
        maneuver_parameters=None,
        comment=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def angular_velocity(self) -> list[AngVelState]:
        """
        Angular velocity vector.
        """
        ...

    @angular_velocity.setter
    def angular_velocity(self, value: list[AngVelState]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        One or more comment line(s). Each comment line shall begin with this keyword.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def epoch(self) -> str:
        """
        Epoch of the attitude elements and optional logical blocks.
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def euler_angle_state(self) -> list[EulerAngleState]:
        """
        Euler angle elements. All mandatory elements of the logical block are to be provided if the
        block is present. (See annex F for conventions and further detail.)
        """
        ...

    @euler_angle_state.setter
    def euler_angle_state(self, value: list[EulerAngleState]) -> None: ...
    @property
    def inertia(self) -> list[InertiaState]:
        """
        Inertia. All mandatory elements are to be provided if the block is present. (See annex F
        for conventions and further detail.)
        """
        ...

    @inertia.setter
    def inertia(self, value: list[InertiaState]) -> None: ...
    @property
    def maneuver_parameters(self) -> list[ManeuverParameters]:
        """
        Maneuver Parameters.
        """
        ...

    @maneuver_parameters.setter
    def maneuver_parameters(self, value: list[ManeuverParameters]) -> None: ...
    @property
    def quaternion_state(self) -> list[QuaternionState]:
        """
        Attitude quaternion. All mandatory elements are to be provided if the block is present.
        (See annex F for conventions and further detail.)
        """
        ...

    @quaternion_state.setter
    def quaternion_state(self, value: list[QuaternionState]) -> None: ...
    @property
    def spin(self) -> list[SpinState]:
        """
        Spin. All mandatory elements are to be provided if the block is present. (See annex F for
        conventions and further detail.)
        """
        ...

    @spin.setter
    def spin(self, value: list[SpinState]) -> None: ...

class ApmMetadata:
    """
    APM Metadata Section.
    """
    def __init__(
        object_name, object_id, time_system=None, center_name=None, comment=None
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def center_name(self) -> str | None:
        """
        Celestial body orbited by the object, which may be a natural solar system body (planets,
        asteroids, comets, and natural satellites), including any planet barycenter or the solar
        system barycenter. The set of allowed values is described in annex B, subsection B8.

        Examples: EARTH, BARYCENTER, MOON
        """
        ...

    @center_name.setter
    def center_name(self, value: str | None) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (allowed only at the beginning of the APM Metadata before OBJECT_NAME). Each
        comment line shall begin with this keyword.

        Examples: This is a comment.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def object_id(self) -> str:
        """
        Spacecraft identifier of the object corresponding to the attitude data to be given. While
        there is no CCSDS-based restriction on the value for this keyword, it is recommended to use
        international designators from the UN Office of Outer Space Affairs (reference [ADM-2]).
        Recommended values have the format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three
        digit serial number of launch in year YYYY (with leading zeros). P{PP} = At least one
        letter for the identification of the part brought into space by the launch. In cases in
        which the asset is not listed in reference [ADM-2], the UN Office of Outer Space Affairs
        designator index format is not used, or the content cannot be disclosed, the value should
        be set to UNKNOWN.

        Examples: 2000-052A
        """
        ...

    @object_id.setter
    def object_id(self, value: str) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Spacecraft name for which the attitude state is provided. While there is no CCSDS-based
        restriction on the value for this keyword, it is recommended to use names from the UN
        Office of Outer Space Affairs designator index (reference [ADM-2], which include object
        name and international designator). When OBJECT_NAME is not known or cannot be disclosed,
        the value should be set to UNKNOWN.

        Examples: EUTELSAT W1, MARS PATHFINDER, UNKNOWN
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system used for attitude and maneuver data. The set of allowed values is described in
        annex B, subsection B2.

        Examples: UTC, TAI
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...

class ApmSegment:
    def __init__(metadata, data) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def data(self) -> ApmData:
        """
        APM Data Section.
        """
        ...

    @data.setter
    def data(self, value: ApmData) -> None: ...
    @property
    def metadata(self) -> ApmMetadata:
        """
        APM Metadata Section.
        """
        ...

    @metadata.setter
    def metadata(self, value: ApmMetadata) -> None: ...

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
        Comments (allowed only at the beginning of each RDM data logical block).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def nominal_reentry_epoch(self) -> Optional[str]:
        """
        Predicted epoch at which the object’s altitude permanently drops below
        NOMINAL_REENTRY_ALTITUDE (formatting rules specified in 5.3.3.5).
        """
        ...

    @nominal_reentry_epoch.setter
    def nominal_reentry_epoch(self, value: Optional[str]) -> None: ...
    @property
    def orbit_lifetime(self) -> float:
        """
        Time until re-entry: from the EPOCH_TZERO epoch in the metadata (days—double precision
        values allowed; integer values assumed to have .0 fractional part) to permanently
        crossing the altitude specified in REENTRY_ALTITUDE. If the NOMINAL_REENTRY_EPOCH
        keyword is present, the ORBIT_LIFETIME and NOMINAL_REENTRY_EPOCH should resolve to the
        same value.

        Units: d
        """
        ...

    @orbit_lifetime.setter
    def orbit_lifetime(self, value: float) -> None: ...
    @property
    def orbit_lifetime_confidence_level(self) -> Optional[float]:
        """
        Confidence level of the orbit lifetime or re-entry epoch being inside the window
        defined by ORBIT_LIFETIME_WINDOW_START and ORBIT_LIFETIME_WINDOW_END or
        REENTRY_WINDOW_START and REENTRY_WINDOW_END.

        Units: %
        """
        ...

    @orbit_lifetime_confidence_level.setter
    def orbit_lifetime_confidence_level(self, value: Optional[float]) -> None: ...
    @property
    def orbit_lifetime_window_end(self) -> Optional[float]:
        """
        End of the predicted orbital lifetime window from the EPOCH_TZERO epoch in the metadata
        (days—double precision values allowed; integer values assumed to have .0 fractional
        part). To be used for long-term predictions; REENTRY_WINDOW_START and _END should be
        used for accurate results.

        Units: d
        """
        ...

    @orbit_lifetime_window_end.setter
    def orbit_lifetime_window_end(self, value: Optional[float]) -> None: ...
    @property
    def orbit_lifetime_window_start(self) -> Optional[float]:
        """
        Start of the predicted orbital lifetime window from the EPOCH_TZERO epoch in the
        metadata (days—double precision values allowed; integer values assumed to have .0
        fractional part). To be used for long-term predictions; REENTRY_WINDOW_START and _END
        should be used for accurate results.

        Units: d
        """
        ...

    @orbit_lifetime_window_start.setter
    def orbit_lifetime_window_start(self, value: Optional[float]) -> None: ...
    @property
    def reentry_altitude(self) -> float:
        """
        Defined re-entry altitude over a spherical central body—once an object’s altitude
        permanently drops below this value, it is considered to be captured by the central
        body’s atmosphere.

        Units: km
        """
        ...

    @reentry_altitude.setter
    def reentry_altitude(self, value: float) -> None: ...
    @property
    def reentry_window_end(self) -> Optional[str]:
        """
        End epoch of the predicted atmospheric re-entry window (formatting rules specified in
        5.3.3.5).
        """
        ...

    @reentry_window_end.setter
    def reentry_window_end(self, value: Optional[str]) -> None: ...
    @property
    def reentry_window_start(self) -> Optional[str]:
        """
        Start epoch of the predicted atmospheric re-entry window (formatting rules specified in
        5.3.3.5).
        """
        ...

    @reentry_window_start.setter
    def reentry_window_start(self, value: Optional[str]) -> None: ...

class AttitudeState:
    def __init__(epoch, values) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def epoch(self): ...
    @epoch.setter
    def epoch(self, value: object) -> None: ...
    @property
    def values(self): ...
    @values.setter
    def values(self, value: object) -> None: ...

class Cdm:
    """
    Conjunction Data Message (CDM).

    The CDM contains information about a single conjunction between a primary object (Object1)
    and a secondary object (Object2). It allows satellite operators to evaluate the risk of
    collision and plan avoidance maneuvers.

    The message includes:
    - Positions and velocities of both objects at Time of Closest Approach (TCA).
    - Covariance matrices for both objects at TCA.
    - Relative position and velocity of Object2 with respect to Object1.
    - Metadata describing how the data was determined (orbit determination settings).
    """
    def __init__(header, body) -> None: ...
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
        Conjunction Data Message (CDM).

        The CDM contains information about a single conjunction between a primary object (Object1)
        and a secondary object (Object2). It allows satellite operators to evaluate the risk of
        collision and plan avoidance maneuvers.

        The message includes:
        - Positions and velocities of both objects at Time of Closest Approach (TCA).
        - Covariance matrices for both objects at TCA.
        - Relative position and velocity of Object2 with respect to Object1.
        - Metadata describing how the data was determined (orbit determination settings).
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

    def to_file(self, path, format, validate=True):
        """
        Write the CDM to a file.

        Parameters
        ----------
        path : str
            The output file path.
        format : str
            The output format ('kvn' or 'xml').
        validate : bool, optional
            Whether to validate the message before writing (default: True).
        """
        ...

    def to_str(self, format, validate=True):
        """
        Serialize the CDM to a string.

        Parameters
        ----------
        format : str
            The output format ('kvn' or 'xml').
        validate : bool, optional
            Whether to validate the message before writing (default: True).

        Returns
        -------
        str
            The serialized CDM string.
        """
        ...

    def validate(self, strict=True):
        """
        Validate the message against CCSDS rules.

        Parameters
        ----------
        strict : bool, optional
            If True (default), raises ValueError on the first error found.
            If False, returns a list of validation error messages (or None if valid).
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
        cdrg_r=None,
        cdrg_t=None,
        cdrg_n=None,
        cdrg_rdot=None,
        cdrg_tdot=None,
        cdrg_ndot=None,
        cdrg_drg=None,
        csrp_r=None,
        csrp_t=None,
        csrp_n=None,
        csrp_rdot=None,
        csrp_tdot=None,
        csrp_ndot=None,
        csrp_drg=None,
        csrp_srp=None,
        cthr_r=None,
        cthr_t=None,
        cthr_n=None,
        cthr_rdot=None,
        cthr_tdot=None,
        cthr_ndot=None,
        cthr_drg=None,
        cthr_srp=None,
        cthr_thr=None,
        comment=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def cdrg_drg(self) -> Optional[float]:
        """
        Object covariance matrix `[7,7]`.

        Units: m⁴/kg²
        """
        ...

    @cdrg_drg.setter
    def cdrg_drg(self, value: Optional[float]) -> None: ...
    @property
    def cdrg_n(self) -> Optional[float]:
        """
        Object covariance matrix `[7,3]`.

        Units: m³/kg
        """
        ...

    @cdrg_n.setter
    def cdrg_n(self, value: Optional[float]) -> None: ...
    @property
    def cdrg_ndot(self) -> Optional[float]:
        """
        Object covariance matrix `[7,6]`.

        Units: m³/(kg*s)
        """
        ...

    @cdrg_ndot.setter
    def cdrg_ndot(self, value: Optional[float]) -> None: ...
    @property
    def cdrg_r(self) -> Optional[float]:
        """
        Object covariance matrix `[7,1]`.

        Units: m³/kg
        """
        ...

    @cdrg_r.setter
    def cdrg_r(self, value: Optional[float]) -> None: ...
    @property
    def cdrg_rdot(self) -> Optional[float]:
        """
        Object covariance matrix `[7,4]`.

        Units: m³/(kg*s)
        """
        ...

    @cdrg_rdot.setter
    def cdrg_rdot(self, value: Optional[float]) -> None: ...
    @property
    def cdrg_t(self) -> Optional[float]:
        """
        Object covariance matrix `[7,2]`.

        Units: m³/kg
        """
        ...

    @cdrg_t.setter
    def cdrg_t(self, value: Optional[float]) -> None: ...
    @property
    def cdrg_tdot(self) -> Optional[float]:
        """
        Object covariance matrix `[7,5]`.

        Units: m³/(kg*s)
        """
        ...

    @cdrg_tdot.setter
    def cdrg_tdot(self, value: Optional[float]) -> None: ...
    @property
    def cn_n(self) -> float:
        """
        Object covariance matrix `[3,3]`.

        Units: m²
        """
        ...

    @cn_n.setter
    def cn_n(self, value: float) -> None: ...
    @property
    def cn_r(self) -> float:
        """
        Object covariance matrix `[3,1]`.

        Units: m²
        """
        ...

    @cn_r.setter
    def cn_r(self, value: float) -> None: ...
    @property
    def cn_t(self) -> float:
        """
        Object covariance matrix `[3,2]`.

        Units: m²
        """
        ...

    @cn_t.setter
    def cn_t(self, value: float) -> None: ...
    @property
    def cndot_n(self) -> float:
        """
        Object covariance matrix `[6,3]`.

        Units: m²/s
        """
        ...

    @cndot_n.setter
    def cndot_n(self, value: float) -> None: ...
    @property
    def cndot_ndot(self) -> float:
        """
        Object covariance matrix `[6,6]`.

        Units: m²/s²
        """
        ...

    @cndot_ndot.setter
    def cndot_ndot(self, value: float) -> None: ...
    @property
    def cndot_r(self) -> float:
        """
        Object covariance matrix `[6,1]`.

        Units: m²/s
        """
        ...

    @cndot_r.setter
    def cndot_r(self, value: float) -> None: ...
    @property
    def cndot_rdot(self) -> float:
        """
        Object covariance matrix `[6,4]`.

        Units: m²/s²
        """
        ...

    @cndot_rdot.setter
    def cndot_rdot(self, value: float) -> None: ...
    @property
    def cndot_t(self) -> float:
        """
        Object covariance matrix `[6,2]`.

        Units: m²/s
        """
        ...

    @cndot_t.setter
    def cndot_t(self, value: float) -> None: ...
    @property
    def cndot_tdot(self) -> float:
        """
        Object covariance matrix `[6,5]`.

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
        Object covariance matrix `[1,1]`.

        Units: m²
        """
        ...

    @cr_r.setter
    def cr_r(self, value: float) -> None: ...
    @property
    def crdot_n(self) -> float:
        """
        Object covariance matrix `[4,3]`.

        Units: m²/s
        """
        ...

    @crdot_n.setter
    def crdot_n(self, value: float) -> None: ...
    @property
    def crdot_r(self) -> float:
        """
        Object covariance matrix `[4,1]`.

        Units: m²/s
        """
        ...

    @crdot_r.setter
    def crdot_r(self, value: float) -> None: ...
    @property
    def crdot_rdot(self) -> float:
        """
        Object covariance matrix `[4,4]`.

        Units: m²/s²
        """
        ...

    @crdot_rdot.setter
    def crdot_rdot(self, value: float) -> None: ...
    @property
    def crdot_t(self) -> float:
        """
        Object covariance matrix `[4,2]`.

        Units: m²/s
        """
        ...

    @crdot_t.setter
    def crdot_t(self, value: float) -> None: ...
    @property
    def csrp_drg(self) -> Optional[float]:
        """
        Object covariance matrix `[8,7]`.

        Units: m⁴/kg²
        """
        ...

    @csrp_drg.setter
    def csrp_drg(self, value: Optional[float]) -> None: ...
    @property
    def csrp_n(self) -> Optional[float]:
        """
        Object covariance matrix `[8,3]`.

        Units: m³/kg
        """
        ...

    @csrp_n.setter
    def csrp_n(self, value: Optional[float]) -> None: ...
    @property
    def csrp_ndot(self) -> Optional[float]:
        """
        Object covariance matrix `[8,6]`.

        Units: m³/(kg*s)
        """
        ...

    @csrp_ndot.setter
    def csrp_ndot(self, value: Optional[float]) -> None: ...
    @property
    def csrp_r(self) -> Optional[float]:
        """
        Object covariance matrix `[8,1]`.

        Units: m³/kg
        """
        ...

    @csrp_r.setter
    def csrp_r(self, value: Optional[float]) -> None: ...
    @property
    def csrp_rdot(self) -> Optional[float]:
        """
        Object covariance matrix `[8,4]`.

        Units: m³/(kg*s)
        """
        ...

    @csrp_rdot.setter
    def csrp_rdot(self, value: Optional[float]) -> None: ...
    @property
    def csrp_srp(self) -> Optional[float]:
        """
        Object covariance matrix `[8,8]`.

        Units: m⁴/kg²
        """
        ...

    @csrp_srp.setter
    def csrp_srp(self, value: Optional[float]) -> None: ...
    @property
    def csrp_t(self) -> Optional[float]:
        """
        Object covariance matrix `[8,2]`.

        Units: m³/kg
        """
        ...

    @csrp_t.setter
    def csrp_t(self, value: Optional[float]) -> None: ...
    @property
    def csrp_tdot(self) -> Optional[float]:
        """
        Object covariance matrix `[8,5]`.

        Units: m³/(kg*s)
        """
        ...

    @csrp_tdot.setter
    def csrp_tdot(self, value: Optional[float]) -> None: ...
    @property
    def ct_r(self) -> float:
        """
        Object covariance matrix `[2,1]`.

        Units: m²
        """
        ...

    @ct_r.setter
    def ct_r(self, value: float) -> None: ...
    @property
    def ct_t(self) -> float:
        """
        Object covariance matrix `[2,2]`.

        Units: m²
        """
        ...

    @ct_t.setter
    def ct_t(self, value: float) -> None: ...
    @property
    def ctdot_n(self) -> float:
        """
        Object covariance matrix `[5,3]`.

        Units: m²/s
        """
        ...

    @ctdot_n.setter
    def ctdot_n(self, value: float) -> None: ...
    @property
    def ctdot_r(self) -> float:
        """
        Object covariance matrix `[5,1]`.

        Units: m²/s
        """
        ...

    @ctdot_r.setter
    def ctdot_r(self, value: float) -> None: ...
    @property
    def ctdot_rdot(self) -> float:
        """
        Object covariance matrix `[5,4]`.

        Units: m²/s²
        """
        ...

    @ctdot_rdot.setter
    def ctdot_rdot(self, value: float) -> None: ...
    @property
    def ctdot_t(self) -> float:
        """
        Object covariance matrix `[5,2]`.

        Units: m²/s
        """
        ...

    @ctdot_t.setter
    def ctdot_t(self, value: float) -> None: ...
    @property
    def ctdot_tdot(self) -> float:
        """
        Object covariance matrix `[5,5]`.

        Units: m²/s²
        """
        ...

    @ctdot_tdot.setter
    def ctdot_tdot(self, value: float) -> None: ...
    @property
    def cthr_drg(self) -> Optional[float]:
        """
        Object covariance matrix `[9,7]`.

        Units: m³/(kg*s²)
        """
        ...

    @cthr_drg.setter
    def cthr_drg(self, value: Optional[float]) -> None: ...
    @property
    def cthr_n(self) -> Optional[float]:
        """
        Object covariance matrix `[9,3]`.

        Units: m²/s²
        """
        ...

    @cthr_n.setter
    def cthr_n(self, value: Optional[float]) -> None: ...
    @property
    def cthr_ndot(self) -> Optional[float]:
        """
        Object covariance matrix `[9,6]`.

        Units: m²/s³
        """
        ...

    @cthr_ndot.setter
    def cthr_ndot(self, value: Optional[float]) -> None: ...
    @property
    def cthr_r(self) -> Optional[float]:
        """
        Object covariance matrix `[9,1]`.

        Units: m²/s²
        """
        ...

    @cthr_r.setter
    def cthr_r(self, value: Optional[float]) -> None: ...
    @property
    def cthr_rdot(self) -> Optional[float]:
        """
        Object covariance matrix `[9,4]`.

        Units: m²/s³
        """
        ...

    @cthr_rdot.setter
    def cthr_rdot(self, value: Optional[float]) -> None: ...
    @property
    def cthr_srp(self) -> Optional[float]:
        """
        Object covariance matrix `[9,8]`.

        Units: m³/(kg*s²)
        """
        ...

    @cthr_srp.setter
    def cthr_srp(self, value: Optional[float]) -> None: ...
    @property
    def cthr_t(self) -> Optional[float]:
        """
        Object covariance matrix `[9,2]`.

        Units: m²/s²
        """
        ...

    @cthr_t.setter
    def cthr_t(self, value: Optional[float]) -> None: ...
    @property
    def cthr_tdot(self) -> Optional[float]:
        """
        Object covariance matrix `[9,5]`.

        Units: m²/s³
        """
        ...

    @cthr_tdot.setter
    def cthr_tdot(self, value: Optional[float]) -> None: ...
    @property
    def cthr_thr(self) -> Optional[float]:
        """
        Object covariance matrix `[9,9]`.

        Units: m²/s⁴
        """
        ...

    @cthr_thr.setter
    def cthr_thr(self, value: Optional[float]) -> None: ...
    @staticmethod
    def from_numpy(array, comment=None):
        """ """
        ...

    def to_numpy(self):
        """
        Returns the covariance matrix as a NumPy array.
        The size will be 6x6, 7x7, 8x8, or 9x9 depending on whether optional
        Drag, SRP, and Thrust parameters are provided, as per CCSDS 508.0-B-1.
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
    def __init__(
        state_vector,
        covariance_matrix=None,
        od_parameters=None,
        additional_parameters=None,
        comments=None,
    ) -> None: ...
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
    def covariance_matrix(self) -> Optional[CdmCovarianceMatrix]:
        """
        Covariance Matrix.
        """
        ...

    @covariance_matrix.setter
    def covariance_matrix(self, value: Optional[CdmCovarianceMatrix]) -> None: ...
    @property
    def covariance_matrix_numpy(self) -> numpy.ndarray:
        """
        Covariance matrix as a NumPy array (convenience method).

        Returns:
            numpy.ndarray: 9x9 covariance matrix.
        """
        ...

    @covariance_matrix_numpy.setter
    def covariance_matrix_numpy(self, value: numpy.ndarray) -> None: ...
    @staticmethod
    def from_numpy(
        state_vector,
        covariance_matrix=None,
        od_parameters=None,
        additional_parameters=None,
        comments=None,
    ):
        """ """
        ...

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
    @property
    def state_vector_numpy(self) -> numpy.ndarray:
        """
        State vector as a NumPy array (convenience method).

        Returns:
            numpy.ndarray: 1D array of shape (6,) containing [X, Y, Z, X_DOT, Y_DOT, Z_DOT].
            Units: [km, km, km, km/s, km/s, km/s]
        """
        ...

    @state_vector_numpy.setter
    def state_vector_numpy(self, value: numpy.ndarray) -> None: ...

class CdmHeader:
    """
    Represents the `cdmHeader` complex type.

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
        Comments (allowed in the CDM Header only immediately after the CDM version number).
        (See 6.3.4 for formatting rules.)

        Examples: This is a comment
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def creation_date(self) -> str:
        """
        Message creation date/time in Coordinated Universal Time (UTC). (See 6.3.2.6 for
        formatting rules.)

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
        ID that uniquely identifies a message from a given originator. The format and content
        of the message identifier value are at the discretion of the originator. (See 5.2.9
        for formatting rules.)

        Examples: 201113719185, ABC-12_34
        """
        ...

    @message_id.setter
    def message_id(self, value: str) -> None: ...
    @property
    def originator(self) -> str:
        """
        Creating agency or owner/operator. Value should be the 'Abbreviation' value from the
        SANA 'Organizations' registry (<https://sanaregistry.org/r/organizations>) for an
        organization that has the Role of 'Conjunction Data Message Originator'. (See 5.2.9
        for formatting rules.)

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
    object : Union[CdmObjectType, str]
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
    covariance_method : Union[CovarianceMethodType, str]
        Method used to calculate the covariance (CALCULATED or DEFAULT).
    maneuverable : Union[ManeuverableType, str]
        The maneuver capacity of the object (YES, NO, or NA).
    ref_frame : Union[ReferenceFrameType, str]
        Reference frame for state vector data (GCRF, EME2000, or ITRF).
    object_type : Union[ObjectDescription, str], optional
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
        ref_frame,
        ephemeris_name=...,
        covariance_method=None,
        maneuverable=None,
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
        The atmospheric density model used for the OD of the object. If 'NONE' is specified,
        then no atmospheric model was used.

        Examples: JACCHIA 70, MSIS, JACCHIA 70 DCA, NONE
        """
        ...

    @atmospheric_model.setter
    def atmospheric_model(self, value: Optional[str]) -> None: ...
    @property
    def catalog_name(self) -> str:
        """
        The satellite catalog used for the object. Value should be taken from the SANA
        'Conjunction Data Message CATALOG_NAME' registry
        (<https://sanaregistry.org/r/cdm_catalog>). (See 5.2.9 for formatting rules.)

        Examples: SATCAT
        """
        ...

    @catalog_name.setter
    def catalog_name(self, value: str) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 6.3.4 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def covariance_method(self) -> CovarianceMethodType:
        """
        Method used to calculate the covariance during the OD that produced the state vector, or
        whether an arbitrary, non-calculated default value was used. Caution should be used
        when using the default value for calculating collision probability.

        Examples: CALCULATED, DEFAULT
        """
        ...

    @covariance_method.setter
    def covariance_method(self, value: CovarianceMethodType) -> None: ...
    @property
    def earth_tides(self) -> Optional[bool]:
        """
        Indication of whether solid Earth and ocean tides were used for the OD of the object.

        Examples: YES, NO
        """
        ...

    @earth_tides.setter
    def earth_tides(self, value: Optional[bool]) -> None: ...
    @property
    def ephemeris_name(self) -> str:
        """
        Unique name of the external ephemeris file used for the object or NONE. This is used to
        indicate whether an external (i.e., Owner/Operator [O/O] provided) ephemeris file was
        used to calculate the CA. If 'NONE' is specified, then the output of the most current
        Orbit Determination (OD) of the CDM originator was used in the CA.

        Examples: EPHEMERIS SATELLITE A, NONE
        """
        ...

    @ephemeris_name.setter
    def ephemeris_name(self, value: str) -> None: ...
    @property
    def gravity_model(self) -> Optional[str]:
        """
        The gravity model used for the OD of the object. (See annex E under GRAVITY_MODEL for
        definition).

        Examples: EGM-96: 36D 360, WGS-84_GEOID: 24D 240, JGM-2: 41D 410
        """
        ...

    @gravity_model.setter
    def gravity_model(self, value: Optional[str]) -> None: ...
    @property
    def international_designator(self) -> str:
        """
        The full international designator for the object. Values shall have the format
        YYYY-NNNP{PP}, where: YYYY = year of launch; NNN = three-digit serial number of launch
        (with leading zeros); P{PP} = At least one capital letter for the identification of the
        part brought into space by the launch. In cases where the object has no international
        designator, the value UNKNOWN should be used. (See 5.2.9 for further formatting rules.)

        Examples: 2002-021A, UNKNOWN
        """
        ...

    @international_designator.setter
    def international_designator(self, value: str) -> None: ...
    @property
    def intrack_thrust(self) -> Optional[bool]:
        """
        Indication of whether in-track thrust modeling was used for the OD of the object.

        Examples: YES, NO
        """
        ...

    @intrack_thrust.setter
    def intrack_thrust(self, value: Optional[bool]) -> None: ...
    @property
    def maneuverable(self) -> ManeuverableType:
        """
        The maneuver capacity of the object. (See 1.4.3.1 for definition of 'N/A'.)

        Examples: YES, NO, N/A
        """
        ...

    @maneuverable.setter
    def maneuverable(self, value: ManeuverableType) -> None: ...
    @property
    def n_body_perturbations(self) -> Optional[str]:
        """
        The N-body gravitational perturbations used for the OD of the object. If 'NONE' is
        specified, then no third-body gravitational perturbations were used.

        Examples: MOON, SUN, JUPITER, NONE
        """
        ...

    @n_body_perturbations.setter
    def n_body_perturbations(self, value: Optional[str]) -> None: ...
    @property
    def object(self) -> CdmObjectType:
        """
        The object to which the metadata and data apply (Object1 or Object2).

        Examples: OBJECT1, OBJECT2
        """
        ...

    @object.setter
    def object(self, value: CdmObjectType) -> None: ...
    @property
    def object_designator(self) -> str:
        """
        The satellite catalog designator for the object. (See 5.2.9 for formatting rules.)

        Examples: 12345
        """
        ...

    @object_designator.setter
    def object_designator(self, value: str) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Spacecraft name for the object.

        Examples: SPOT, ENVISAT, IRIDIUM, INTELSAT
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

        Examples: ORBITAL SAFETY ANALYST (OSA), NETWORK CONTROLLER
        """
        ...

    @operator_contact_position.setter
    def operator_contact_position(self, value: Optional[str]) -> None: ...
    @property
    def operator_email(self) -> Optional[str]:
        """
        Email address of the contact position or organization of the object.

        Examples: JOHN.DOE@SOMEWHERE.NET
        """
        ...

    @operator_email.setter
    def operator_email(self, value: Optional[str]) -> None: ...
    @property
    def operator_organization(self) -> Optional[str]:
        """
        Contact organization of the object.

        Examples: EUMETSAT, ESA, INTELSAT, IRIDIUM
        """
        ...

    @operator_organization.setter
    def operator_organization(self, value: Optional[str]) -> None: ...
    @property
    def operator_phone(self) -> Optional[str]:
        """
        Phone number of the contact position or organization for the object.

        Examples: +49615130312
        """
        ...

    @operator_phone.setter
    def operator_phone(self, value: Optional[str]) -> None: ...
    @property
    def orbit_center(self) -> Optional[str]:
        """
        The central body about which Object1 and Object2 orbit. If not specified, the center is
        assumed to be Earth.

        Examples: EARTH, SUN, MOON, MARS
        """
        ...

    @orbit_center.setter
    def orbit_center(self, value: Optional[str]) -> None: ...
    @property
    def ref_frame(self) -> ReferenceFrameType:
        """
        Name of the reference frame in which the state vector data are given. Value must be
        selected from the list of values to the right (see reference `[F1]`) and be the same for
        both Object1 and Object2.

        Examples: GCRF, EME2000, ITRF
        """
        ...

    @ref_frame.setter
    def ref_frame(self, value: ReferenceFrameType) -> None: ...
    @property
    def solar_rad_pressure(self) -> Optional[bool]:
        """
        Indication of whether solar radiation pressure perturbations were used for the OD of the
        object.

        Examples: YES, NO
        """
        ...

    @solar_rad_pressure.setter
    def solar_rad_pressure(self, value: Optional[bool]) -> None: ...

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

    @staticmethod
    def from_numpy(array):
        """ """
        ...

    def to_numpy(self) -> numpy.ndarray:
        """
        Return the state vector as a NumPy array.

        Returns:
            numpy.ndarray: 1D array of shape (6,) containing [X, Y, Z, X_DOT, Y_DOT, Z_DOT].
            Units: [km, km, km, km/s, km/s, km/s]
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

class ControlledType:
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

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

class EulerAngleState:
    """
    Euler angle elements.

    All mandatory elements of the logical block are to be provided if the block is present.
    (See annex F for conventions and further detail.)
    """
    def __init__(
        ref_frame_a,
        ref_frame_b,
        euler_rot_seq,
        angle_1,
        angle_2,
        angle_3,
        angle_1_dot,
        angle_2_dot,
        angle_3_dot,
        comment,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def angle_1(self) -> float:
        """
        Angle of the first rotation.

        Units: deg
        """
        ...

    @angle_1.setter
    def angle_1(self, value: float) -> None: ...
    @property
    def angle_1_dot(self) -> Optional[float]:
        """
        Time derivative of angle of the first rotation.

        Units: deg/s
        """
        ...

    @angle_1_dot.setter
    def angle_1_dot(self, value: Optional[float]) -> None: ...
    @property
    def angle_2(self) -> float:
        """
        Angle of the second rotation.

        Units: deg
        """
        ...

    @angle_2.setter
    def angle_2(self, value: float) -> None: ...
    @property
    def angle_2_dot(self) -> Optional[float]:
        """
        Time derivative of angle of the second rotation.

        Units: deg/s
        """
        ...

    @angle_2_dot.setter
    def angle_2_dot(self, value: Optional[float]) -> None: ...
    @property
    def angle_3(self) -> float:
        """
        Angle of the third rotation.

        Units: deg
        """
        ...

    @angle_3.setter
    def angle_3(self, value: float) -> None: ...
    @property
    def angle_3_dot(self) -> Optional[float]:
        """
        Time derivative of angle of the third rotation.

        Units: deg/s
        """
        ...

    @angle_3_dot.setter
    def angle_3_dot(self, value: Optional[float]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        One or more comment line(s). Each comment line shall begin with this keyword.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def euler_rot_seq(self) -> str:
        """
        Rotation sequence that defines the REF_FRAME_A to REF_FRAME_B transformation. The order of
        the transformation is from left to right, where the leftmost letter (X, Y, or Z) represents
        the rotation axis of the first rotation, the second letter (X, Y, or Z) represents the
        rotation axis of the second rotation, and the third letter (X, Y, or Z) represents the
        rotation axis of the third rotation.
        """
        ...

    @euler_rot_seq.setter
    def euler_rot_seq(self, value: str) -> None: ...
    @property
    def ref_frame_a(self) -> str:
        """
        Name of the reference frame that defines the starting point of the transformation. The set
        of allowed values is described in annex B, subsection B3.
        """
        ...

    @ref_frame_a.setter
    def ref_frame_a(self, value: str) -> None: ...
    @property
    def ref_frame_b(self) -> str:
        """
        Name of the reference frame that defines the end point of the transformation. The set of
        allowed values is described in annex B, subsection B3.
        """
        ...

    @ref_frame_b.setter
    def ref_frame_b(self, value: str) -> None: ...

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
        *,
        probability_of_impact=None,
        probability_of_burn_up=None,
        probability_of_break_up=None,
        probability_of_land_impact=None,
        probability_of_casualty=None,
        nominal_impact_epoch=None,
        impact_window_start=None,
        impact_window_end=None,
        impact_ref_frame=None,
        nominal_impact_lon=None,
        nominal_impact_lat=None,
        nominal_impact_alt=None,
        impact_1_confidence=None,
        impact_1_start_lon=None,
        impact_1_start_lat=None,
        impact_1_stop_lon=None,
        impact_1_stop_lat=None,
        impact_1_cross_track=None,
        impact_2_confidence=None,
        impact_2_start_lon=None,
        impact_2_start_lat=None,
        impact_2_stop_lon=None,
        impact_2_stop_lat=None,
        impact_2_cross_track=None,
        impact_3_confidence=None,
        impact_3_start_lon=None,
        impact_3_start_lat=None,
        impact_3_stop_lon=None,
        impact_3_stop_lat=None,
        impact_3_cross_track=None,
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
        Comments (allowed only at the beginning of each RDM data logical block).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def impact_1_confidence(self) -> Optional[float]:
        """
        First (lowest) confidence interval for the impact location.

        Units: %
        """
        ...

    @impact_1_confidence.setter
    def impact_1_confidence(self, value: Optional[float]) -> None: ...
    @property
    def impact_1_cross_track(self) -> Optional[float]:
        """
        Cross-track size of the first confidence interval.

        Units: km
        """
        ...

    @impact_1_cross_track.setter
    def impact_1_cross_track(self, value: Optional[float]) -> None: ...
    @property
    def impact_1_start_lat(self) -> Optional[float]:
        """
        Latitude of the start of the first confidence interval along the ground track with
        respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
        the rules specified in 3.5.12.

        Units: deg
        """
        ...

    @impact_1_start_lat.setter
    def impact_1_start_lat(self, value: Optional[float]) -> None: ...
    @property
    def impact_1_start_lon(self) -> Optional[float]:
        """
        Longitude of the start of the first confidence interval along the ground track with
        respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
        the rules specified in 3.5.11.

        Units: deg
        """
        ...

    @impact_1_start_lon.setter
    def impact_1_start_lon(self, value: Optional[float]) -> None: ...
    @property
    def impact_1_stop_lat(self) -> Optional[float]:
        """
        Latitude of the end of the first confidence interval along the ground track with
        respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
        the rules specified in 3.5.12.

        Units: deg
        """
        ...

    @impact_1_stop_lat.setter
    def impact_1_stop_lat(self, value: Optional[float]) -> None: ...
    @property
    def impact_1_stop_lon(self) -> Optional[float]:
        """
        Longitude of the end of the first confidence interval along the ground track with
        respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
        the rules specified in 3.5.11.

        Units: deg
        """
        ...

    @impact_1_stop_lon.setter
    def impact_1_stop_lon(self, value: Optional[float]) -> None: ...
    @property
    def impact_2_confidence(self) -> Optional[float]:
        """
        Second confidence interval for the impact location. The IMPACT_1_* block must be
        present if IMPACT_2_* is used.

        Units: %
        """
        ...

    @impact_2_confidence.setter
    def impact_2_confidence(self, value: Optional[float]) -> None: ...
    @property
    def impact_2_cross_track(self) -> Optional[float]:
        """
        Cross-track size of the second confidence interval.

        Units: km
        """
        ...

    @impact_2_cross_track.setter
    def impact_2_cross_track(self, value: Optional[float]) -> None: ...
    @property
    def impact_2_start_lat(self) -> Optional[float]:
        """
        Latitude of the start of the second confidence interval along the ground track with
        respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
        the rules specified in 3.5.12.

        Units: deg
        """
        ...

    @impact_2_start_lat.setter
    def impact_2_start_lat(self, value: Optional[float]) -> None: ...
    @property
    def impact_2_start_lon(self) -> Optional[float]:
        """
        Longitude of the start of the second confidence interval along the ground track with
        respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
        the rules specified in 3.5.11.

        Units: deg
        """
        ...

    @impact_2_start_lon.setter
    def impact_2_start_lon(self, value: Optional[float]) -> None: ...
    @property
    def impact_2_stop_lat(self) -> Optional[float]:
        """
        Latitude of the end of the second confidence interval along the ground track with
        respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
        the rules specified in 3.5.12.

        Units: deg
        """
        ...

    @impact_2_stop_lat.setter
    def impact_2_stop_lat(self, value: Optional[float]) -> None: ...
    @property
    def impact_2_stop_lon(self) -> Optional[float]:
        """
        Longitude of the end of the second confidence interval along the ground track with
        respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
        the rules specified in 3.5.11.

        Units: deg
        """
        ...

    @impact_2_stop_lon.setter
    def impact_2_stop_lon(self, value: Optional[float]) -> None: ...
    @property
    def impact_3_confidence(self) -> Optional[float]:
        """
        Third (highest) confidence interval for the impact location. The IMPACT_2_* block must
        be present if IMPACT_3_* is used.

        Units: %
        """
        ...

    @impact_3_confidence.setter
    def impact_3_confidence(self, value: Optional[float]) -> None: ...
    @property
    def impact_3_cross_track(self) -> Optional[float]:
        """
        Cross-track size of the third confidence interval.

        Units: km
        """
        ...

    @impact_3_cross_track.setter
    def impact_3_cross_track(self, value: Optional[float]) -> None: ...
    @property
    def impact_3_start_lat(self) -> Optional[float]:
        """
        Latitude of the start of the third confidence interval along the ground track with
        respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
        the rules specified in 3.5.12.

        Units: deg
        """
        ...

    @impact_3_start_lat.setter
    def impact_3_start_lat(self, value: Optional[float]) -> None: ...
    @property
    def impact_3_start_lon(self) -> Optional[float]:
        """
        Longitude of the start of the third confidence interval along the ground track with
        respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
        the rules specified in 3.5.11.

        Units: deg
        """
        ...

    @impact_3_start_lon.setter
    def impact_3_start_lon(self, value: Optional[float]) -> None: ...
    @property
    def impact_3_stop_lat(self) -> Optional[float]:
        """
        Latitude of the end of the third confidence interval along the ground track with
        respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
        the rules specified in 3.5.12.

        Units: deg
        """
        ...

    @impact_3_stop_lat.setter
    def impact_3_stop_lat(self, value: Optional[float]) -> None: ...
    @property
    def impact_3_stop_lon(self) -> Optional[float]:
        """
        Longitude of the end of the third confidence interval along the ground track with
        respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
        the rules specified in 3.5.11.

        Units: deg
        """
        ...

    @impact_3_stop_lon.setter
    def impact_3_stop_lon(self, value: Optional[float]) -> None: ...
    @property
    def impact_ref_frame(self) -> Optional[str]:
        """
        Reference frame of the impact location data. The value should be taken from the keyword
        value name column in the SANA celestial body reference frames registry, reference `[11]`.
        Only frames with the value ‘Body-Fixed’ in the Frame Type column shall be used.
        Mandatory if NOMINAL_IMPACT_LON and NOMINAL_IMPACT_LAT are present.
        """
        ...

    @impact_ref_frame.setter
    def impact_ref_frame(self, value: Optional[str]) -> None: ...
    @property
    def impact_window_end(self) -> Optional[str]:
        """
        End epoch of the predicted impact window (formatting rules specified in 5.3.3.5).
        """
        ...

    @impact_window_end.setter
    def impact_window_end(self, value: Optional[str]) -> None: ...
    @property
    def impact_window_start(self) -> Optional[str]:
        """
        Start epoch of the predicted impact window (formatting rules specified in 5.3.3.5).
        """
        ...

    @impact_window_start.setter
    def impact_window_start(self, value: Optional[str]) -> None: ...
    @property
    def nominal_impact_alt(self) -> Optional[float]:
        """
        Altitude of the impact location with respect to the value of IMPACT_REF_FRAME.

        Units: m
        """
        ...

    @nominal_impact_alt.setter
    def nominal_impact_alt(self, value: Optional[float]) -> None: ...
    @property
    def nominal_impact_epoch(self) -> Optional[str]:
        """
        Epoch of the predicted impact (formatting rules specified in 5.3.3.5).
        """
        ...

    @nominal_impact_epoch.setter
    def nominal_impact_epoch(self, value: Optional[str]) -> None: ...
    @property
    def nominal_impact_lat(self) -> Optional[float]:
        """
        Latitude of the predicted impact location with respect to the value of
        IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in
        3.5.12.

        Units: deg
        """
        ...

    @nominal_impact_lat.setter
    def nominal_impact_lat(self, value: Optional[float]) -> None: ...
    @property
    def nominal_impact_lon(self) -> Optional[float]:
        """
        Longitude of the predicted impact location with respect to the value of
        IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in
        3.5.11.

        Units: deg
        """
        ...

    @nominal_impact_lon.setter
    def nominal_impact_lon(self, value: Optional[float]) -> None: ...
    @property
    def probability_of_break_up(self) -> Optional[float]:
        """
        Probability that the object will break up during re-entry (0 to 1).
        """
        ...

    @probability_of_break_up.setter
    def probability_of_break_up(self, value: Optional[float]) -> None: ...
    @property
    def probability_of_burn_up(self) -> Optional[float]:
        """
        Probability that the entire object and any fragments will burn up during atmospheric
        re-entry (0 to 1).
        """
        ...

    @probability_of_burn_up.setter
    def probability_of_burn_up(self, value: Optional[float]) -> None: ...
    @property
    def probability_of_casualty(self) -> Optional[float]:
        """
        Probability that the re-entry event will cause any casualties (severe injuries or
        deaths—0 to 1).
        """
        ...

    @probability_of_casualty.setter
    def probability_of_casualty(self, value: Optional[float]) -> None: ...
    @property
    def probability_of_impact(self) -> Optional[float]:
        """
        Probability that any fragment will impact the Earth (either land or sea; 0 to 1).
        """
        ...

    @probability_of_impact.setter
    def probability_of_impact(self, value: Optional[float]) -> None: ...
    @property
    def probability_of_land_impact(self) -> Optional[float]:
        """
        Probability that any fragment will impact solid ground (0 to 1).
        """
        ...

    @probability_of_land_impact.setter
    def probability_of_land_impact(self, value: Optional[float]) -> None: ...

class InertiaState:
    """
    Inertia block.

    All mandatory elements are to be provided if the block is present.
    (See annex F for conventions and further detail.)
    """
    def __init__(inertia_ref_frame, ixx, iyy, izz, ixy, ixz, iyz, comment) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        One or more comment line(s). Each comment line shall begin with this keyword.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def inertia_ref_frame(self) -> str:
        """
        Coordinate system for the inertia tensor. The set of allowed values is described in annex B,
        subsection B3.
        """
        ...

    @inertia_ref_frame.setter
    def inertia_ref_frame(self, value: str) -> None: ...
    @property
    def ixx(self) -> float:
        """
        Moment of Inertia about the X-axis.

        Units: kg*m²
        """
        ...

    @ixx.setter
    def ixx(self, value: float) -> None: ...
    @property
    def ixy(self) -> float:
        """
        Inertia Cross Product of the X and Y axes.

        Units: kg*m²
        """
        ...

    @ixy.setter
    def ixy(self, value: float) -> None: ...
    @property
    def ixz(self) -> float:
        """
        Inertia Cross Product of the X and Z axes.

        Units: kg*m²
        """
        ...

    @ixz.setter
    def ixz(self, value: float) -> None: ...
    @property
    def iyy(self) -> float:
        """
        Moment of Inertia about the Y-axis.

        Units: kg*m²
        """
        ...

    @iyy.setter
    def iyy(self, value: float) -> None: ...
    @property
    def iyz(self) -> float:
        """
        Inertia Cross Product of the Y and Z axes.

        Units: kg*m²
        """
        ...

    @iyz.setter
    def iyz(self, value: float) -> None: ...
    @property
    def izz(self) -> float:
        """
        Moment of Inertia about the Z-axis.

        Units: kg*m²
        """
        ...

    @izz.setter
    def izz(self, value: float) -> None: ...

class KeplerianElements:
    """
    Osculating Keplerian Elements in the Specified Reference Frame (none or all parameters of
    this block must be given).

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
        true_anomaly=None,
        mean_anomaly=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def arg_of_pericenter(self) -> float:
        """
        Argument of pericenter

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
        Eccentricity

        Units: n/a
        """
        ...

    @eccentricity.setter
    def eccentricity(self, value: float) -> None: ...
    @property
    def gm(self) -> float:
        """
        Gravitational Coefficient (Gravitational Constant × Central Mass)

        Units: km³/s²
        """
        ...

    @gm.setter
    def gm(self, value: float) -> None: ...
    @property
    def inclination(self) -> float:
        """
        Inclination

        Units: deg
        """
        ...

    @inclination.setter
    def inclination(self, value: float) -> None: ...
    @property
    def mean_anomaly(self) -> Optional[float]:
        """
        True anomaly or mean anomaly

        Units: deg
        """
        ...

    @mean_anomaly.setter
    def mean_anomaly(self, value: Optional[float]) -> None: ...
    @property
    def ra_of_asc_node(self) -> float:
        """
        Right ascension of ascending node

        Units: deg
        """
        ...

    @ra_of_asc_node.setter
    def ra_of_asc_node(self, value: float) -> None: ...
    @property
    def semi_major_axis(self) -> float:
        """
        Semi-major axis

        Units: km
        """
        ...

    @semi_major_axis.setter
    def semi_major_axis(self, value: float) -> None: ...
    @property
    def true_anomaly(self) -> Optional[float]:
        """
        True anomaly or mean anomaly

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
    Maneuver Parameters (Repeat for each maneuver).

    References:
    - CCSDS 502.0-B-3, Section 3.2.4 (OPM Data Section)
    """
    def __init__(
        man_epoch_start,
        man_duration,
        man_ref_frame,
        man_tor_1,
        man_tor_2,
        man_tor_3,
        man_delta_mass=None,
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
    def man_delta_mass(self) -> Optional[float]:
        """
        Mass change during maneuver (value is < 0)

        Units: kg


        **Note**: The CCSDS standard requires this value to be strictly negative (`< 0`).
        However, this implementation allows non-negative values to support non-standard use cases.
        """
        ...

    @man_delta_mass.setter
    def man_delta_mass(self, value: Optional[float]) -> None: ...
    @property
    def man_duration(self) -> float:
        """
        Maneuver duration (If = 0, impulsive maneuver)

        Units: s
        """
        ...

    @man_duration.setter
    def man_duration(self, value: float) -> None: ...
    @property
    def man_epoch_start(self) -> str:
        """
        Epoch of ignition (see 7.5.10 for formatting rules)
        """
        ...

    @man_epoch_start.setter
    def man_epoch_start(self, value: str) -> None: ...
    @property
    def man_ref_frame(self) -> str:
        """
        Reference frame in which the velocity increment vector data are given. The user must
        select from the accepted set of values indicated in 3.2.4.11.
        """
        ...

    @man_ref_frame.setter
    def man_ref_frame(self, value: str) -> None: ...
    @property
    def man_tor_x(self) -> float:
        """
        Torque X component.

        Units: N*m
        """
        ...

    @man_tor_x.setter
    def man_tor_x(self, value: float) -> None: ...
    @property
    def man_tor_y(self) -> float:
        """
        Torque Y component.

        Units: N*m
        """
        ...

    @man_tor_y.setter
    def man_tor_y(self, value: float) -> None: ...
    @property
    def man_tor_z(self) -> float:
        """
        Torque Z component.

        Units: N*m
        """
        ...

    @man_tor_z.setter
    def man_tor_z(self, value: float) -> None: ...

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
        semi_major_axis=None,
        mean_motion=None,
        gm=None,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def arg_of_pericenter(self) -> float:
        """
        Argument of pericenter

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
        Eccentricity

        Examples: 0.7303

        Units: n/a
        """
        ...

    @eccentricity.setter
    def eccentricity(self, value: float) -> None: ...
    @property
    def epoch(self) -> str:
        """
        Epoch of Mean Keplerian elements (see 7.5.10 for formatting rules)

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def gm(self) -> Optional[float]:
        """
        Gravitational Coefficient (Gravitational Constant × Central Mass)

        Examples: 398600.44

        Units: km³/s²
        """
        ...

    @gm.setter
    def gm(self, value: Optional[float]) -> None: ...
    @property
    def inclination(self) -> float:
        """
        Inclination

        Examples: 63.4

        Units: deg
        """
        ...

    @inclination.setter
    def inclination(self, value: float) -> None: ...
    @property
    def mean_anomaly(self) -> float:
        """
        Mean anomaly

        Examples: 130.0

        Units: deg
        """
        ...

    @mean_anomaly.setter
    def mean_anomaly(self, value: float) -> None: ...
    @property
    def mean_motion(self) -> Optional[float]:
        """
        Semi-major axis in kilometers (preferred), or, if MEAN_ELEMENT_THEORY = SGP/SGP4, the
        Keplerian Mean motion in revolutions per day

        Examples: 1.491325

        Units: km or rev/day
        """
        ...

    @mean_motion.setter
    def mean_motion(self, value: Optional[float]) -> None: ...
    @property
    def ra_of_asc_node(self) -> float:
        """
        Right ascension of ascending node

        Examples: 345.0

        Units: deg
        """
        ...

    @ra_of_asc_node.setter
    def ra_of_asc_node(self, value: float) -> None: ...
    @property
    def semi_major_axis(self) -> Optional[float]:
        """
        Semi-major axis in kilometers (preferred), or, if MEAN_ELEMENT_THEORY = SGP/SGP4, the
        Keplerian Mean motion in revolutions per day

        Examples: 28594.4

        Units: km or rev/day
        """
        ...

    @semi_major_axis.setter
    def semi_major_axis(self, value: Optional[float]) -> None: ...

class Ndm:
    """
    Combined Instantiation Navigation Data Message (NDM).

    It is possible to create an XML instance that incorporates any number of NDM messages in a
    logical suite called an ‘NDM combined instantiation’. Such combined instantiations may be
    useful for some situations, for example: (1) a constellation of spacecraft in which
    ephemeris data for all of the spacecraft is combined in a single XML message; (2) a
    spacecraft attitude that depends upon a particular orbital state (an APM and its
    associated OPM could be conveniently conveyed in a single NDM); (3) an ephemeris message
    with the set of tracking data messages used in the orbit determination.
    """
    def __init__(messages, id=None, comments=...) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comments(self) -> list[str]:
        """
        Comments (optional).
        """
        ...

    @comments.setter
    def comments(self, value: list[str]) -> None: ...
    @staticmethod
    def from_file(path, format=None):
        """
        Parse an NDM combined instantiation from a file.
        """
        ...

    @staticmethod
    def from_str(data, format=None):
        """
        Parse an NDM combined instantiation from a string.
        """
        ...

    @property
    def id(self) -> Optional[str]:
        """
        Message Identifier (optional).
        """
        ...

    @property
    def messages(self) -> list[Union[Oem, Cdm, Opm, Omm, Ocm, Rdm, Tdm, Ndm]]:
        """
        List of contained navigation messages.
        """
        ...

    @messages.setter
    def messages(
        self, value: list[Union[Oem, Cdm, Opm, Omm, Ocm, Rdm, Tdm, Ndm]]
    ) -> None: ...
    def to_file(self, path, format, validate=True):
        """
        Write to file.

        Parameters
        ----------
        path : str
            Output file path.
        format : str
            Output format ('kvn' or 'xml').
        validate : bool, optional
            Whether to validate the message before writing (default: True).
        """
        ...

    def to_str(self, format, validate=True):
        """
        Serialize to a string.
        """
        ...

    def validate(self, strict=True):
        """
        Validate the combined message against CCSDS rules.

        Parameters
        ----------
        strict : bool, optional
            If True (default), raises ValueError on the first error found.
            If False, returns a list of validation error messages (or None if valid).
        """
        ...

class ObjectDescription:
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

class Ocm:
    """
    Orbit Comprehensive Message (OCM).

    An OCM specifies position and velocity of either a single object or an en masse parent/child
    deployment scenario stemming from a single object. The OCM aggregates and extends OPM, OEM,
    and OMM content in a single comprehensive hybrid message.

    Key features:
    - Support for single object or parent/child deployment scenarios.
    - Aggregation of OPM, OMM, and OEM content.
    - Extensive optional content including physical properties, covariance, maneuvers, and
    perturbations.
    - Well-suited for exchanges involving automated interaction and large object catalogs.

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

        An OCM specifies position and velocity of either a single object or an en masse parent/child
        deployment scenario stemming from a single object. The OCM aggregates and extends OPM, OEM,
        and OMM content in a single comprehensive hybrid message.

        Key features:
        - Support for single object or parent/child deployment scenarios.
        - Aggregation of OPM, OMM, and OEM content.
        - Extensive optional content including physical properties, covariance, maneuvers, and
        perturbations.
        - Well-suited for exchanges involving automated interaction and large object catalogs.
        """
        ...

    @header.setter
    def header(self, value: OdmHeader) -> None: ...
    @property
    def id(self) -> Optional[str]:
        """
        The message identifier.
        """
        ...

    @property
    def segment(self) -> OcmSegment:
        """
        The OCM data segment.
        """
        ...

    @segment.setter
    def segment(self, value: OcmSegment) -> None: ...
    def to_file(self, path, format, validate=True):
        """
        Write to file.

        Parameters
        ----------
        path : str
            Output file path.
        format : str
            Output format ('kvn' or 'xml').
        validate : bool, optional
            Whether to validate the message before writing (default: True).
        """
        ...

    def to_str(self, format, validate=True):
        """
        Serialize to string.

        Parameters
        ----------
        format : str
            Output format ('kvn' or 'xml').
        validate : bool, optional
            Whether to validate the message before writing (default: True).

        Returns
        -------
        str
            The serialized string.
        """
        ...

    def validate(self, strict=True):
        """
        Validate the message against CCSDS rules.

        Parameters
        ----------
        strict : bool, optional
            If True (default), raises ValueError on the first error found.
            If False, returns a list of validation error messages (or None if valid).
        """
        ...

    @property
    def version(self) -> str:
        """
        The message version.
        """
        ...

    @version.setter
    def version(self, value: str) -> None: ...

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
        Comments (a contiguous set of one or more comment lines may be provided in the OCM
        covariance time history section only immediately after the COV_START keyword; see 7.8
        for comment formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def cov_basis(self) -> Optional[str]:
        """
        Basis of this covariance time history data. This is free-text field with the following
        suggested values: a) 'PREDICTED'. b) 'DETERMINED' when estimated from observation-based
        orbit determination, reconstruction and/or calibration. For definitive OD performed
        onboard whose solutions have been telemetered to the ground for inclusion in an OCM,
        the COV_BASIS shall be considered to be DETERMINED. c) EMPIRICAL (for empirically
        determined such as overlap analyses). d) SIMULATED for simulation-based (including
        Monte Carlo) estimations, future mission design studies, and optimization studies. e)
        'OTHER' for other bases of this data.

        Examples: PREDICTED, EMPIRICAL, DETERMINED, SIMULATED, OTHER
        """
        ...

    @cov_basis.setter
    def cov_basis(self, value: Optional[str]) -> None: ...
    @property
    def cov_basis_id(self) -> Optional[str]:
        """
        Free-text field containing the identification number for the orbit determination,
        navigation solution, or simulation upon which this covariance time history block is
        based. When a matching orbit determination block accompanies this covariance time
        history, the COV_BASIS_ID should match the corresponding OD_ID (see table 6-11).

        Examples: OD_5910
        """
        ...

    @cov_basis_id.setter
    def cov_basis_id(self, value: Optional[str]) -> None: ...
    @property
    def cov_confidence(self) -> Optional[float]:
        """
        A measure of the confidence in the covariance errors matching reality, as characterized
        via a Wald test, a Chi-squared test, the log of likelihood, or a numerical
        representation per mutual agreement.

        Examples: 50

        Units: %
        """
        ...

    @cov_confidence.setter
    def cov_confidence(self, value: Optional[float]) -> None: ...
    @property
    def cov_frame_epoch(self) -> Optional[str]:
        """
        Epoch of the covariance data reference frame, if not intrinsic to the definition of the
        reference frame. (See 7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @cov_frame_epoch.setter
    def cov_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def cov_id(self) -> Optional[str]:
        """
        Free-text field containing the identification number for this covariance time history
        block.

        Examples: COV_20160402_XYZ
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
        Free-text field containing the identification number for the next covariance time
        history, contained either within this message, or presented in a future OCM. NOTE—If
        this message is not part of a sequence of covariance time histories or if this
        covariance time history is the last in a sequence of covariance time histories, then
        COV_NEXT_ID should be excluded from this message.

        Examples: COV_20160305C
        """
        ...

    @cov_next_id.setter
    def cov_next_id(self, value: Optional[str]) -> None: ...
    @property
    def cov_ordering(self) -> str:
        """
        Indicates covariance ordering as being either LTM, UTM, Full covariance, LTM covariance
        with cross-correlation information provided in upper triangle off-diagonal terms
        (LTMWCC), or UTM covariance with cross-correlation information provided in lower
        triangle off-diagonal terms (UTMWCC).

        Examples: LTM, UTM, FULL, LTMWCC, UTMWCC
        """
        ...

    @cov_ordering.setter
    def cov_ordering(self, value: str) -> None: ...
    @property
    def cov_prev_id(self) -> Optional[str]:
        """
        Free-text field containing the identification number for the previous covariance time
        history, contained either within this message or presented in a previous OCM. NOTE—If
        this message is not part of a sequence of covariance time histories or if this
        covariance time history is the first in a sequence of covariance time histories, then
        COV_PREV_ID should be excluded from this message.

        Examples: COV_20160305a
        """
        ...

    @cov_prev_id.setter
    def cov_prev_id(self, value: Optional[str]) -> None: ...
    @property
    def cov_ref_frame(self) -> str:
        """
        Reference frame of the covariance time history. Select from the accepted set of values
        indicated in annex B, subsection B4 and B5.

        Examples: TNW_INERTIA, J2000
        """
        ...

    @cov_ref_frame.setter
    def cov_ref_frame(self, value: str) -> None: ...
    @property
    def cov_scale_max(self) -> Optional[float]:
        """
        Maximum scale factor to apply to this covariance data to achieve realism.

        Examples: 5.0
        """
        ...

    @cov_scale_max.setter
    def cov_scale_max(self, value: Optional[float]) -> None: ...
    @property
    def cov_scale_min(self) -> Optional[float]:
        """
        Minimum scale factor to apply to this covariance data to achieve realism.

        Examples: 0.5
        """
        ...

    @cov_scale_min.setter
    def cov_scale_min(self, value: Optional[float]) -> None: ...
    @property
    def cov_type(self) -> str:
        """
        Indicates covariance composition. Select from annex B, subsections B7 and B8.

        Examples: CARTP, CARTPV, ADBARV
        """
        ...

    @cov_type.setter
    def cov_type(self, value: str) -> None: ...
    @property
    def cov_units(self) -> Optional[str]:
        """
        A comma-delimited set of SI unit designations for each element of the covariance time
        history following the covariance time tag, solely for informational purposes, provided
        as a free-text field enclosed in square brackets. When provided, these units
        designations shall correspond to the units of the standard deviations (or square roots)
        of each of the covariance matrix diagonal elements (or variances), respectively, and
        all diagonal elements shall have a corresponding units entry, with non-dimensional
        values (such as dispersion in orbit eccentricity) denoted by 'n/a'. NOTE—The listing of
        units via the COV_UNITS keyword does not override the mandatory units specified for the
        selected COV_TYPE (links to the relevant SANA registries provided in annex B,
        subsections B7 and B8).

        Examples: [km,km,km,km/s,km/s,km/s]
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
    def user(self) -> UserDefined | None:
        """
        User-defined parameters.
        """
        ...

    @user.setter
    def user(self, value: UserDefined | None) -> None: ...

class OcmManeuverParameters:
    """
    OCM Maneuver Parameters.

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
        Comments (a contiguous set of one or more comment lines may be provided in the OCM
        Maneuver Specification only immediately after the MAN_START keyword; see 7.8 for
        comment formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def dc_body_frame(self) -> Optional[str]:
        """
        For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the body
        reference frame in which DC_BODY_TRIGGER will be specified. Select from the accepted
        set of values indicated in annex B, subsection B6. This keyword shall be set if
        DC_TYPE = 'TIME_AND_ANGLE'.
        """
        ...

    @dc_body_frame.setter
    def dc_body_frame(self, value: Optional[str]) -> None: ...
    @property
    def dc_body_trigger(self) -> Optional[list[float]]:
        """
        For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the body frame
        reference vector direction in the 'DC_BODY_FRAME' reference frame at which, when its
        projection onto the spin plane crosses the corresponding projection of DC_REF_DIR onto
        the spin plane, this angle-based duty cycle is initiated (see DC_PA_START_ANGLE for
        phasing). This tripartite value shall be provided if DC_TYPE = 'TIME_AND_ANGLE'.
        """
        ...

    @dc_body_trigger.setter
    def dc_body_trigger(self, value: Optional[list[float]]) -> None: ...
    @property
    def dc_exec_start(self) -> Optional[str]:
        """
        Start time of the initial duty cycle-based maneuver sequence execution. DC_EXEC_START
        is defined to occur on or prior to the first maneuver 'ON' portion within the duty
        cycle sequence. DC_EXEC_START must be scheduled to occur coincident with or after
        DC_WIN_OPEN. This keyword shall be set if DC_TYPE ≠ 'CONTINUOUS'.
        """
        ...

    @dc_exec_start.setter
    def dc_exec_start(self, value: Optional[str]) -> None: ...
    @property
    def dc_exec_stop(self) -> Optional[str]:
        """
        End time of the final duty cycle-based maneuver sequence execution. DC_EXEC_STOP
        typically occurs on or after the end of the final maneuver 'ON' portion within the duty
        cycle sequence. DC_EXEC_STOP must be scheduled to occur coincident with or prior to
        DC_WIN_CLOSE. This keyword shall be set if DC_TYPE ≠ 'CONTINUOUS'.
        """
        ...

    @dc_exec_stop.setter
    def dc_exec_stop(self, value: Optional[str]) -> None: ...
    @property
    def dc_max_cycles(self) -> Optional[int]:
        """
        Maximum number of 'ON' duty cycles (may override DC_EXEC_STOP). This value is optional
        even if DC_TYPE = 'CONTINUOUS'.
        """
        ...

    @dc_max_cycles.setter
    def dc_max_cycles(self, value: Optional[int]) -> None: ...
    @property
    def dc_min_cycles(self) -> Optional[int]:
        """
        Minimum number of 'ON' duty cycles (may override DC_EXEC_STOP). This value is optional
        even if DC_TYPE = 'CONTINUOUS'.
        """
        ...

    @dc_min_cycles.setter
    def dc_min_cycles(self, value: Optional[int]) -> None: ...
    @property
    def dc_pa_start_angle(self) -> Optional[float]:
        """
        For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the phase angle
        offset of thruster pulse start, measured with respect to the occurrence of a
        DC_BODY_TRIGGER crossing of the DC_REF_DIR direction when both are projected into the
        spin plane (normal to the body spin axis). This phase angle offset can be positive or
        negative to allow the duty cycle to begin prior to the next crossing of the
        DC_REF_DIR. As this angular direction is to be used in a modulo sense, there is no
        requirement for the magnitude of the phase angle offset to be less than 360 degrees.
        This keyword shall be set if DC_TYPE = 'TIME_AND_ANGLE'.

        Units: deg
        """
        ...

    @dc_pa_start_angle.setter
    def dc_pa_start_angle(self, value: Optional[float]) -> None: ...
    @property
    def dc_pa_stop_angle(self) -> Optional[float]:
        """
        For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the phase angle
        of thruster pulse stop, measured with respect to the DC_BODY_TRIGGER crossing of the
        DC_REF_DIR direction when both are projected into the spin plane. This phase angle
        offset can be positive or negative to allow the duty cycle to end after to the next
        crossing of the DC_REF_DIR. As this angular direction is to be used in a modulo sense,
        there is no requirement for the magnitude of the phase angle offset to be less than
        360 degrees. This keyword shall be set if DC_TYPE = 'TIME_AND_ANGLE'.

        Units: deg
        """
        ...

    @dc_pa_stop_angle.setter
    def dc_pa_stop_angle(self, value: Optional[float]) -> None: ...
    @property
    def dc_ref_dir(self) -> Optional[list[float]]:
        """
        For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the reference
        vector direction in the 'MAN_REF_FRAME' reference frame at which, when mapped into the
        space object's spin plane (normal to the spin axis), the duty cycle is triggered (see
        DC_PA_START_ANGLE for phasing). This (tripartite, or three-element vector) value shall
        be provided if DC_TYPE = 'TIME_AND_ANGLE'. This reference direction does not represent
        the duty cycle midpoint.
        """
        ...

    @dc_ref_dir.setter
    def dc_ref_dir(self, value: Optional[list[float]]) -> None: ...
    @property
    def dc_ref_time(self) -> Optional[str]:
        """
        Reference time for the THRUST duty cycle, specified as either time in seconds (relative
        to EPOCH_TZERO), or as an absolute '`<epoch>`' (see 7.5.10 for formatting rules).
        NOTE—Depending upon EPOCH_TZERO, DC_REF_TIME relative times may be negative. This
        keyword shall be set if DC_TYPE ≠ 'CONTINUOUS'.
        """
        ...

    @dc_ref_time.setter
    def dc_ref_time(self, value: Optional[str]) -> None: ...
    @property
    def dc_time_pulse_duration(self) -> Optional[float]:
        """
        Thruster pulse 'ON' duration, initiated at first satisfaction of the burn 'ON' time
        constraint or upon completion of the previous DC_TIME_PULSE_PERIOD cycle. This keyword
        shall be set if DC_TYPE ≠ 'CONTINUOUS'.

        Units: s
        """
        ...

    @dc_time_pulse_duration.setter
    def dc_time_pulse_duration(self, value: Optional[float]) -> None: ...
    @property
    def dc_time_pulse_period(self) -> Optional[float]:
        """
        Elapsed time between the start of one pulse and the start of the next. Must be greater
        than or equal to DC_TIME_PULSE_DURATION. This keyword shall be set if DC_TYPE ≠
        'CONTINUOUS'.

        Units: s
        """
        ...

    @dc_time_pulse_period.setter
    def dc_time_pulse_period(self, value: Optional[float]) -> None: ...
    @property
    def dc_type(self) -> str:
        """
        Duty cycle type to use for this maneuver time history section: CONTINUOUS denotes
        full/continuous thrust `<default>`; TIME denotes a time-based duty cycle driven by time
        past a reference time and the duty cycle ON and OFF durations; TIME_AND_ANGLE denotes a
        duty cycle driven by the phasing/clocking of a space object body frame 'trigger'
        direction past a reference direction.
        """
        ...

    @dc_type.setter
    def dc_type(self, value: str) -> None: ...
    @property
    def dc_win_close(self) -> Optional[str]:
        """
        End time of the duty cycle-based maneuver window that occurs on or after the actual
        maneuver execution end time. For example, this may identify the time at which the
        satellite is taken out of a special duty-cycle-based maneuver mode. This keyword shall
        be set if DC_TYPE ≠ 'CONTINUOUS'.
        """
        ...

    @dc_win_close.setter
    def dc_win_close(self, value: Optional[str]) -> None: ...
    @property
    def dc_win_open(self) -> Optional[str]:
        """
        Start time of the duty cycle-based maneuver window that occurs on or prior to the
        actual maneuver execution start time. For example, this may identify the time at which
        the satellite is first placed into a special duty-cycle-based maneuver mode. This
        keyword shall be set if DC_TYPE ≠ 'CONTINUOUS'.
        """
        ...

    @dc_win_open.setter
    def dc_win_open(self, value: Optional[str]) -> None: ...
    @property
    def grav_assist_name(self) -> Optional[str]:
        """
        Origin of maneuver gravitational assist body, which may be a natural solar system body
        (planets, asteroids, comets, and natural satellites), including any planet barycenter
        or the solar system barycenter. (See annex B, subsection B2, for acceptable
        GRAV_ASSIST_NAME values and the procedure to propose new values.)
        """
        ...

    @grav_assist_name.setter
    def grav_assist_name(self, value: Optional[str]) -> None: ...
    @property
    def man_basis(self) -> Optional[str]:
        """
        Basis of this maneuver time history data, which shall be selected from one of the
        following values: 'CANDIDATE' for a proposed operational or a hypothetical (i.e.,
        mission design and optimization studies) future maneuver, 'PLANNED' for a currently
        planned future maneuver, 'ANTICIPATED' for a non-cooperative future maneuver that is
        anticipated (i.e., likely) to occur (e.g., based upon patterns-of-life analysis),
        'TELEMETRY' when the maneuver is determined directly from telemetry (e.g., based on
        inertial navigation systems or accelerometers), 'DETERMINED' when a past maneuver is
        estimated from observation-based orbit determination reconstruction and/or
        calibration, 'SIMULATED' for generic maneuver simulations, future mission design
        studies, and optimization studies, 'OTHER' for other bases of this data.
        """
        ...

    @man_basis.setter
    def man_basis(self, value: Optional[str]) -> None: ...
    @property
    def man_basis_id(self) -> Optional[str]:
        """
        Free-text field containing the identification number for the orbit determination,
        navigation solution, or simulation upon which this maneuver time history block is
        based. Where a matching orbit determination block accompanies this maneuver time
        history, the MAN_BASIS_ID should match the corresponding OD_ID (see table 6-11).
        """
        ...

    @man_basis_id.setter
    def man_basis_id(self, value: Optional[str]) -> None: ...
    @property
    def man_composition(self) -> str:
        """
        The comma-delimited ordered set of maneuver elements of information contained on every
        maneuver time history line, with values selected from table 6-8. Within this maneuver
        data section, the maneuver composition shall include only one TIME specification
        (TIME_ABSOLUTE or TIME_RELATIVE).
        """
        ...

    @man_composition.setter
    def man_composition(self, value: str) -> None: ...
    @property
    def man_device_id(self) -> str:
        """
        Free-text field containing the maneuver device identifier used for this maneuver. 'ALL'
        indicates that this maneuver represents the summed acceleration, velocity increment,
        or thrust imparted by any/all thrusters utilized in the maneuver.
        """
        ...

    @man_device_id.setter
    def man_device_id(self, value: str) -> None: ...
    @property
    def man_frame_epoch(self) -> Optional[str]:
        """
        Epoch of the maneuver data reference frame, if not intrinsic to the definition of the
        reference frame. (See 7.5.10 for formatting rules.)
        """
        ...

    @man_frame_epoch.setter
    def man_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def man_id(self) -> str:
        """
        Free-text field containing the unique maneuver identification number for this maneuver.
        All supplied maneuver 'constituents' within the same MAN_BASIS and MAN_REF_FRAME
        categories shall be added together to represent the total composite maneuver
        description.
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
        Identifies the start time of the next maneuver for this MAN_BASIS.
        """
        ...

    @man_next_epoch.setter
    def man_next_epoch(self, value: Optional[str]) -> None: ...
    @property
    def man_next_id(self) -> Optional[str]:
        """
        Free-text field containing the identification number of the next maneuver for this
        MAN_BASIS, contained either within this message, or presented in a future OCM. If this
        message is not part of a sequence of maneuver messages or if this maneuver is the last
        in a sequence of maneuvers, then MAN_NEXT_ID should be excluded from this message.
        """
        ...

    @man_next_id.setter
    def man_next_id(self, value: Optional[str]) -> None: ...
    @property
    def man_pred_source(self) -> Optional[str]:
        """
        For future maneuvers, specifies the source of the orbit and/or attitude state(s) upon
        which the maneuver is based. While there is no CCSDS-based restriction on the value for
        this free-text keyword, it is suggested to consider using TRAJ_ID and OD_ID keywords
        as described in tables 6-4 and 6-11, respectively, or a combination thereof.
        """
        ...

    @man_pred_source.setter
    def man_pred_source(self, value: Optional[str]) -> None: ...
    @property
    def man_prev_epoch(self) -> Optional[str]:
        """
        Identifies the completion time of the previous maneuver for this MAN_BASIS.
        """
        ...

    @man_prev_epoch.setter
    def man_prev_epoch(self, value: Optional[str]) -> None: ...
    @property
    def man_prev_id(self) -> Optional[str]:
        """
        Free-text field containing the identification number of the previous maneuver for this
        MAN_BASIS, contained either within this message, or presented in a previous OCM. If
        this message is not part of a sequence of maneuver messages or if this maneuver is the
        first in a sequence of maneuvers, then MAN_PREV_ID should be excluded from this
        message.
        """
        ...

    @man_prev_id.setter
    def man_prev_id(self, value: Optional[str]) -> None: ...
    @property
    def man_purpose(self) -> Optional[str]:
        """
        A free-text field used to specify the intention(s) of the maneuver. Multiple maneuver
        purposes can be provided as a comma-delimited list.
        """
        ...

    @man_purpose.setter
    def man_purpose(self, value: Optional[str]) -> None: ...
    @property
    def man_ref_frame(self) -> str:
        """
        Reference frame in which all maneuver vector direction data is provided in this
        maneuver data block. Select from the accepted set of values indicated in annex B,
        subsections B4 and B5. The reference frame must be the same for all data elements
        within a given maneuver time history block.
        """
        ...

    @man_ref_frame.setter
    def man_ref_frame(self, value: str) -> None: ...
    @property
    def man_units(self) -> Optional[str]:
        """
        A comma-delimited set of SI unit designations for each and every element of the
        maneuver time history following the maneuver time tag(s), solely for informational
        purposes, provided as a free-text field enclosed in square brackets. When MAN_UNITS is
        provided, all elements of MAN_COMPOSITION AFTER the maneuver time tag(s) must have a
        corresponding units entry; percentages shall be denoted by '%', and control switches,
        non-dimensional values, and text strings shall be labelled as 'n/a'. NOTE—The listing
        of units via the MAN_UNITS keyword does not override the mandatory units for the
        selected MAN_COMPOSITION, as specified in table 6-8 or table 6-9.
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
        epoch_tzero,
        time_system=None,
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
        Free-text field containing a unique identifier of Attitude Data Message (ADM)
        (reference `[10]`) that are linked (relevant) to this Orbit Data Message.

        Examples: ADM_MSG_35132.txt, ADM_ID_0572
        """
        ...

    @adm_msg_link.setter
    def adm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def alternate_names(self) -> Optional[str]:
        """
        Free-text comma-delimited field containing alternate name(s) of this space object,
        including assigned names used by spacecraft operator, State Actors, commercial SSA
        providers, and/or media.

        Examples: SV08, IN8
        """
        ...

    @alternate_names.setter
    def alternate_names(self, value: Optional[str]) -> None: ...
    @property
    def catalog_name(self) -> Optional[str]:
        """
        Free-text field containing the satellite catalog source (or source agency or operator,
        value to be drawn from the SANA registry list of Space Object Catalogs at
        <https://sanaregistry.org/r/space_object_catalog>, or alternatively, from the list of
        organizations listed in the 'Abbreviation' column of the SANA Organizations registry at
        <https://www.sanaregistry.org/r/organizations>) from which 'OBJECT_DESIGNATOR' was
        obtained.

        Examples: CSPOC, RFSA, ESA, COMSPOC
        """
        ...

    @catalog_name.setter
    def catalog_name(self, value: Optional[str]) -> None: ...
    @property
    def cdm_msg_link(self) -> Optional[str]:
        """
        Free-text field containing a unique identifier of Conjunction Data Message (CDM)
        (reference `[14]`) that are linked (relevant) to this Orbit Data Message.

        Examples: CDM_MSG_35132.txt, CDM_ID_8257
        """
        ...

    @cdm_msg_link.setter
    def cdm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def celestial_source(self) -> Optional[str]:
        """
        Free-text field specifying the source and version of the message originator's celestial
        body (e.g., Sun/Earth/Planetary) ephemeris data used in the creation of this message.

        Examples: JPL_DE_FILES
        """
        ...

    @celestial_source.setter
    def celestial_source(self, value: Optional[str]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (a contiguous set of one or more comment lines may be provided in the OCM
        Metadata section; see 7.8 for comment formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def constellation(self) -> Optional[str]:
        """
        Free-text field containing the name of the constellation to which this space object
        belongs.

        Examples: SPIRE
        """
        ...

    @constellation.setter
    def constellation(self, value: Optional[str]) -> None: ...
    @property
    def country(self) -> Optional[str]:
        """
        Free-text field containing the name of the country, country code, or country
        abbreviation where the space object owner is based.

        Examples: US, SPAIN
        """
        ...

    @country.setter
    def country(self, value: Optional[str]) -> None: ...
    @property
    def eop_source(self) -> Optional[str]:
        """
        Free-text field specifying the source and version of the message originator's Earth
        Orientation Parameters (EOP) used in the creation of this message, including leap
        seconds, TAI – UT1, etc.

        Examples: CELESTRAK_20201028
        """
        ...

    @eop_source.setter
    def eop_source(self, value: Optional[str]) -> None: ...
    @property
    def epoch_tzero(self) -> str:
        """
        Default epoch to which all relative times are referenced in data blocks (for format
        specification, see 7.5.10). The time scale of EPOCH_TZERO is controlled via the
        ‘TIME_SYSTEM' keyword, with the exception that for the SCLK timescale, EPOCH_TZERO
        shall be interpreted as being in the UTC timescale. This field is used by all OCM data
        blocks.

        Examples: 2001-11-06T11:17:33
        """
        ...

    @epoch_tzero.setter
    def epoch_tzero(self, value: str) -> None: ...
    @property
    def international_designator(self) -> Optional[str]:
        """
        Free-text field containing an international designator for the object as assigned by
        the UN Committee on Space Research (COSPAR). Such designator values shall have the
        following COSPAR format: YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three-digit
        serial number of launch in year YYYY (with leading zeros). P{PP} = At least one capital
        letter for the identification of the part brought into space by the launch. If the
        object has no international designator or the content is either unknown (uncorrelated)
        or cannot be disclosed, the value should be set to UNKNOWN (or this keyword omitted).
        NOTE—The international designator was typically specified by 'OBJECT_ID' in the OPM,
        OMM, and OEM.

        Examples: 2000-052A, 1996-068A, 2000-053A, 1996-008A, UNKNOWN
        """
        ...

    @international_designator.setter
    def international_designator(self, value: Optional[str]) -> None: ...
    @property
    def interp_method_eop(self) -> Optional[str]:
        """
        Free-text field specifying the method used to select or interpolate sequential EOP data.

        Examples: PRECEDING_VALUE, NEAREST_NEIGHBOR, LINEAR, LAGRANGE_ORDER_5
        """
        ...

    @interp_method_eop.setter
    def interp_method_eop(self, value: Optional[str]) -> None: ...
    @property
    def next_leap_epoch(self) -> Optional[str]:
        """
        Epoch of next leap second, specified as an absolute time tag.

        Examples: 2016-12-31T23:59:60
        """
        ...

    @next_leap_epoch.setter
    def next_leap_epoch(self, value: Optional[str]) -> None: ...
    @property
    def next_leap_taimutc(self) -> Optional[float]:
        """
        Difference (TAI – UTC) in seconds (i.e., total number of leap seconds elapsed since
        1958) incorporated by the message originator at epoch 'NEXT_LEAP_EPOCH'. This keyword
        should be provided if NEXT_LEAP_EPOCH is supplied.

        Units: s
        """
        ...

    @next_leap_taimutc.setter
    def next_leap_taimutc(self, value: Optional[float]) -> None: ...
    @property
    def next_message_epoch(self) -> Optional[str]:
        """
        Anticipated (or actual) epoch of the next message from this originator for this space
        object. (For format specification, see 7.5.10.) NOTE—One may provide the next message
        epoch without supplying the NEXT_MESSAGE_ID, and vice versa.

        Examples: 2001-11-07T11:17:33
        """
        ...

    @next_message_epoch.setter
    def next_message_epoch(self, value: Optional[str]) -> None: ...
    @property
    def next_message_id(self) -> Optional[str]:
        """
        Free-text field containing an ID that uniquely identifies the next message from this
        message originator for this space object. The format and content of the message
        identifier value are at the discretion of the originator. NOTE—One may provide the next
        message ID without supplying the ‘NEXT_MESSAGE_EPOCH' keyword, and vice versa.

        Examples: OCM 201113719186, ABC-12_35
        """
        ...

    @next_message_id.setter
    def next_message_id(self, value: Optional[str]) -> None: ...
    @property
    def object_designator(self) -> Optional[str]:
        """
        Free-text field specification of the unique satellite identification designator for the
        object, as reflected in the catalog whose name is 'CATALOG_NAME'. If the ID is not known
        (uncorrelated object) or cannot be disclosed, 'UNKNOWN' may be used (or this keyword
        omitted).

        Examples: 22444, 18SPCS 18571, 2147483648_04ae[...]d84c, UNKNOWN
        """
        ...

    @object_designator.setter
    def object_designator(self, value: Optional[str]) -> None: ...
    @property
    def object_name(self) -> Optional[str]:
        """
        Free-text field containing the name of the object. While there is no CCSDS-based
        restriction on the value for this keyword, it is recommended to use names from either
        the UN Office of Outer Space Affairs designator index (reference `[3]`, which include
        Object name and international designator of the participant), the spacecraft operator,
        or a State Actor or commercial Space Situational Awareness (SSA) provider maintaining
        the ‘CATALOG_NAME’ space catalog. If OBJECT_NAME is not listed in reference `[3]` or the
        content is either unknown (uncorrelated) or cannot be disclosed, the value should be
        set to UNKNOWN (or this keyword omitted).

        Examples: SPOT-7, ENVISAT, IRIDIUM NEXT-8, INTELSAT G-15, UNKNOWN
        """
        ...

    @object_name.setter
    def object_name(self, value: Optional[str]) -> None: ...
    @property
    def object_type(self) -> Optional[str]:
        """
        Specification of the type of object. Select from the accepted set of values indicated
        in annex B, subsection B11.

        Examples: PAYLOAD, ROCKET BODY, DEBRIS, UNKNOWN, OTHER
        """
        ...

    @object_type.setter
    def object_type(self, value: Optional[str]) -> None: ...
    @property
    def ocm_data_elements(self) -> Optional[str]:
        """
        Comma-delimited list of elements of information data blocks included in this message.
        The order shall be the same as the order of the data blocks in the message. Values shall
        be confined to the following list: ORB, PHYS, COV, MAN, PERT, OD, and USER. If the OCM
        contains multiple ORB, COV, or MAN data blocks (as allowed by table 6-1), the
        corresponding ORB, COV, or MAN entry shall be duplicated to match.

        Examples: ORB, ORB, PHYS, COV, MAN, MAN, PERT, OD, USER
        """
        ...

    @ocm_data_elements.setter
    def ocm_data_elements(self, value: Optional[str]) -> None: ...
    @property
    def operator(self) -> Optional[str]:
        """
        Free-text field containing the operator of the space object.

        Examples: INTELSAT
        """
        ...

    @operator.setter
    def operator(self, value: Optional[str]) -> None: ...
    @property
    def ops_status(self) -> Optional[str]:
        """
        Specification of the operational status of the space object. Select from the accepted
        set of values indicated in annex B, subsection B12.

        Examples: OPERATIONAL
        """
        ...

    @ops_status.setter
    def ops_status(self, value: Optional[str]) -> None: ...
    @property
    def orbit_category(self) -> Optional[str]:
        """
        Specification of the type of orbit. Select from the accepted set of values indicated in
        annex B, subsection B14.

        Examples: GEO, LEO
        """
        ...

    @orbit_category.setter
    def orbit_category(self, value: Optional[str]) -> None: ...
    @property
    def originator_address(self) -> Optional[str]:
        """
        Free-text field containing originator's physical address information for OCM creator
        (suggest comma-delimited address lines).

        Examples: 5040 Spaceflight Ave., Cocoa Beach, FL, USA, 12345
        """
        ...

    @originator_address.setter
    def originator_address(self, value: Optional[str]) -> None: ...
    @property
    def originator_email(self) -> Optional[str]:
        """
        Free-text field containing originator PoC email address.

        Examples: JOHN.DOE@SOMEWHERE.ORG
        """
        ...

    @originator_email.setter
    def originator_email(self, value: Optional[str]) -> None: ...
    @property
    def originator_phone(self) -> Optional[str]:
        """
        Free-text field containing originator PoC phone number.

        Examples: +12345678901
        """
        ...

    @originator_phone.setter
    def originator_phone(self, value: Optional[str]) -> None: ...
    @property
    def originator_poc(self) -> Optional[str]:
        """
        Free-text field containing originator or programmatic Point-of-Contact (POC) for OCM.

        Examples: Mr. Rodgers
        """
        ...

    @originator_poc.setter
    def originator_poc(self, value: Optional[str]) -> None: ...
    @property
    def originator_position(self) -> Optional[str]:
        """
        Free-text field containing contact position of the originator PoC.

        Examples: Flight Dynamics, Mission Design Lead
        """
        ...

    @originator_position.setter
    def originator_position(self, value: Optional[str]) -> None: ...
    @property
    def owner(self) -> Optional[str]:
        """
        Free-text field containing the owner of the space object.

        Examples: SIRIUS
        """
        ...

    @owner.setter
    def owner(self, value: Optional[str]) -> None: ...
    @property
    def previous_message_epoch(self) -> Optional[str]:
        """
        Creation epoch of the previous message from this originator for this space object. (For
        format specification, see 7.5.10.) NOTE—One may provide the previous message epoch
        without supplying the PREVIOUS_MESSAGE_ID, and vice versa.

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @previous_message_epoch.setter
    def previous_message_epoch(self, value: Optional[str]) -> None: ...
    @property
    def previous_message_id(self) -> Optional[str]:
        """
        Free-text field containing an ID that uniquely identifies the previous message from
        this message originator for this space object. The format and content of the message
        identifier value are at the discretion of the originator. NOTE—One may provide the
        previous message ID without supplying the 'PREVIOUS_MESSAGE_EPOCH' keyword, and vice
        versa.

        Examples: OCM 201113719184, ABC-12_33
        """
        ...

    @previous_message_id.setter
    def previous_message_id(self, value: Optional[str]) -> None: ...
    @property
    def prm_msg_link(self) -> Optional[str]:
        """
        Free-text field containing a unique identifier of Pointing Request Message (PRM)
        (reference `[13]`) that are linked (relevant) to this Orbit Data Message.

        Examples: PRM_MSG_35132.txt, PRM_ID_6897
        """
        ...

    @prm_msg_link.setter
    def prm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def rdm_msg_link(self) -> Optional[str]:
        """
        Free-text field containing a unique identifier of Reentry Data Message (RDM)
        (reference `[12]`) that are linked (relevant) to this Orbit Data Message.

        Examples: RDM_MSG_35132.txt, RDM_ID_1839
        """
        ...

    @rdm_msg_link.setter
    def rdm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def sclk_offset_at_epoch(self) -> Optional[float]:
        """
        Defines the number of spacecraft clock counts existing at EPOCH_TZERO. This is only
        used if the SCLK timescale is employed by the user.

        Units: s
        """
        ...

    @sclk_offset_at_epoch.setter
    def sclk_offset_at_epoch(self, value: Optional[float]) -> None: ...
    @property
    def sclk_sec_per_si_sec(self) -> Optional[float]:
        """
        Defines the current number of clock seconds occurring during one SI second. It should be
        noted that this clock rate may vary with time and is the current approximate value.
        This is only used if the SCLK timescale is employed by the user.

        Units: s
        """
        ...

    @sclk_sec_per_si_sec.setter
    def sclk_sec_per_si_sec(self, value: Optional[float]) -> None: ...
    @property
    def start_time(self) -> Optional[str]:
        """
        Time of the earliest data contained in the OCM, specified as either a relative or
        absolute time tag.

        Examples: 2001-11-06T00:00:00
        """
        ...

    @start_time.setter
    def start_time(self, value: Optional[str]) -> None: ...
    @property
    def stop_time(self) -> Optional[str]:
        """
        Time of the latest data contained in the OCM, specified as either a relative or absolute
        time tag.

        Examples: 2001-11-08T00:00:00
        """
        ...

    @stop_time.setter
    def stop_time(self, value: Optional[str]) -> None: ...
    @property
    def taimutc_at_tzero(self) -> Optional[float]:
        """
        Difference (TAI – UTC) in seconds (i.e., total number of leap seconds elapsed since
        1958) as modeled by the message originator at epoch 'EPOCH_TZERO'.

        Units: s
        """
        ...

    @taimutc_at_tzero.setter
    def taimutc_at_tzero(self, value: Optional[float]) -> None: ...
    @property
    def tdm_msg_link(self) -> Optional[str]:
        """
        Free-text string containing a comma-separated list of file name(s) and/or associated
        identification number(s) of Tracking Data Message (TDM) (reference `[9]`) observations
        upon which this OD is based.

        Examples: TDM_MSG_37.txt, TDM_835, TDM_836
        """
        ...

    @tdm_msg_link.setter
    def tdm_msg_link(self, value: Optional[str]) -> None: ...
    @property
    def tech_address(self) -> Optional[str]:
        """
        Free-text field containing technical PoC physical address information for OCM creator
        (suggest comma-delimited address lines).

        Examples: 5040 Spaceflight Ave., Cocoa Beach, FL, USA, 12345
        """
        ...

    @tech_address.setter
    def tech_address(self, value: Optional[str]) -> None: ...
    @property
    def tech_email(self) -> Optional[str]:
        """
        Free-text field containing technical PoC email address.

        Examples: JOHN.DOE@SOMEWHERE.ORG
        """
        ...

    @tech_email.setter
    def tech_email(self, value: Optional[str]) -> None: ...
    @property
    def tech_org(self) -> Optional[str]:
        """
        Free-text field containing the creating agency or operator (value should be drawn from
        the 'Abbreviation' column of the SANA Organizations registry at
        <https://www.sanaregistry.org/r/organizations>).

        Examples: NASA, ESA, JAXA
        """
        ...

    @tech_org.setter
    def tech_org(self, value: Optional[str]) -> None: ...
    @property
    def tech_phone(self) -> Optional[str]:
        """
        Free-text field containing technical PoC phone number.

        Examples: +49615130312
        """
        ...

    @tech_phone.setter
    def tech_phone(self, value: Optional[str]) -> None: ...
    @property
    def tech_poc(self) -> Optional[str]:
        """
        Free-text field containing technical PoC for OCM.

        Examples: Maxwell Smart
        """
        ...

    @tech_poc.setter
    def tech_poc(self, value: Optional[str]) -> None: ...
    @property
    def tech_position(self) -> Optional[str]:
        """
        Free-text field containing contact position of the technical PoC.

        Examples: Flight Dynamics, Mission Design Lead
        """
        ...

    @tech_position.setter
    def tech_position(self, value: Optional[str]) -> None: ...
    @property
    def time_span(self) -> Optional[float]:
        """
        Span of time that the OCM covers, measured in days. TIME_SPAN is defined as
        (STOP_TIME-START_TIME), measured in days, irrespective of whether START_TIME or
        STOP_TIME are provided by the message creator.

        Units: d
        """
        ...

    @time_span.setter
    def time_span(self, value: Optional[float]) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system for all absolute time stamps in this OCM including EPOCH_TZERO. Select from
        the accepted set of values indicated in annex B, subsection B3. This field is used by
        all OCM data blocks. If the SCLK timescale is selected, then 'EPOCH_TZERO' shall be
        interpreted as the spacecraft clock epoch and both SCLK_OFFSET_AT_EPOCH and
        SCLK_SEC_PER_SI_SEC shall be supplied.

        Examples: UTC
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...
    @property
    def ut1mutc_at_tzero(self) -> Optional[float]:
        """
        Difference (UT1 – UTC) in seconds, as modeled by the originator at epoch 'EPOCH_TZERO'.

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
        Actual time span in days used for the OD of the object. NOTE—Should equal
        (DAYS_SINCE_FIRST_OBS - DAYS_SINCE_LAST_OBS).

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
        """
        ...

    @consider_n.setter
    def consider_n(self, value: Optional[int]) -> None: ...
    @property
    def consider_params(self) -> Optional[str]:
        """
        Free-text comma-delimited description of the consider parameters used in the orbit
        determination.
        """
        ...

    @consider_params.setter
    def consider_params(self, value: Optional[str]) -> None: ...
    @property
    def data_types(self) -> Optional[str]:
        """
        Comma-separated list of observation data types utilized in this orbit determination.
        Although this is a free-text field, it is recommended at a minimum to use data type
        descriptor(s) as provided in table 3-5 of the TDM standard (reference `[9]`) (excluding
        the DATA_START, DATA_STOP, and COMMENT keywords). Additional descriptors/detail is
        encouraged if the descriptors of table 3-5 are not sufficiently clear; for example, one
        could replace ANGLE_1 and ANGLE_2 with RADEC (e.g., from a telescope), AZEL (e.g., from
        a ground radar), RANGE (whether from radar or laser ranging), etc.
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

        Days elapsed between first accepted observation and OD_EPOCH. NOTE—May be positive or
        negative.

        Units: d
        """
        ...

    @days_since_first_obs.setter
    def days_since_first_obs(self, value: Optional[float]) -> None: ...
    @property
    def days_since_last_obs(self) -> Optional[float]:
        """
        Days elapsed between last accepted observation and OD_EPOCH. NOTE—May be positive or
        negative.

        Units: d
        """
        ...

    @days_since_last_obs.setter
    def days_since_last_obs(self, value: Optional[float]) -> None: ...
    @property
    def gdop(self) -> Optional[float]:
        """
        Generalized Dilution Of Precision for this orbit determination, based on the
        observability grammian as defined in references `[H15]` and `[H16]` and expressed in
        informative annex F, subsection F4. GDOP provides a rating metric of the observability
        of the element set from the OD. Alternate GDOP formations may be used as mutually
        defined by message exchange participants.
        """
        ...

    @gdop.setter
    def gdop(self, value: Optional[float]) -> None: ...
    @property
    def maximum_obs_gap(self) -> Optional[float]:
        """
        The maximum time between observations in the OD of the object.

        Units: d
        """
        ...

    @maximum_obs_gap.setter
    def maximum_obs_gap(self, value: Optional[float]) -> None: ...
    @property
    def obs_available(self) -> Optional[int]:
        """
        The number of observations available within the actual OD time span.
        """
        ...

    @obs_available.setter
    def obs_available(self, value: Optional[int]) -> None: ...
    @property
    def obs_used(self) -> Optional[int]:
        """
        The number of observations accepted within the actual OD time span.
        """
        ...

    @obs_used.setter
    def obs_used(self, value: Optional[int]) -> None: ...
    @property
    def od_confidence(self) -> Optional[float]:
        """
        OD confidence metric, which spans 0 to 100% (useful only for Filter-based OD systems).
        The OD confidence metric shall be as mutually defined by message exchange
        participants.

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

        Units: m
        """
        ...

    @od_epoch_eigint.setter
    def od_epoch_eigint(self, value: Optional[float]) -> None: ...
    @property
    def od_epoch_eigmaj(self) -> Optional[float]:
        """
        Positional error ellipsoid 1σ major eigenvalue at the epoch of the OD.

        Units: m
        """
        ...

    @od_epoch_eigmaj.setter
    def od_epoch_eigmaj(self, value: Optional[float]) -> None: ...
    @property
    def od_epoch_eigmin(self) -> Optional[float]:
        """
        Positional error ellipsoid 1σ minor eigenvalue at the epoch of the OD.

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
        The resulting maximum predicted major eigenvalue of the 1σ positional error ellipsoid
        over the entire TIME_SPAN of the OCM, stemming from this OD.

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
        The resulting minimum predicted minor eigenvalue of the 1σ positional error ellipsoid
        over the entire TIME_SPAN of the OCM, stemming from this OD.

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
        Number of days of observations recommended for the OD of the object (useful only for
        Batch OD systems).

        Units: d
        """
        ...

    @recommended_od_span.setter
    def recommended_od_span(self, value: Optional[float]) -> None: ...
    @property
    def sedr(self) -> Optional[float]:
        """
        The Specific Energy Dissipation Rate, which is the amount of energy being removed from
        the object's orbit by the non-conservative forces. This value is an average
        calculated during the OD. (See annex F, subsection F7 for definition.)

        Units: W/kg
        """
        ...

    @sedr.setter
    def sedr(self, value: Optional[float]) -> None: ...
    @property
    def sensors(self) -> Optional[str]:
        """
        Free-text comma-delimited description of the sensors used in the orbit determination.
        """
        ...

    @sensors.setter
    def sensors(self, value: Optional[str]) -> None: ...
    @property
    def sensors_n(self) -> Optional[int]:
        """
        The number of sensors used in the orbit determination.
        """
        ...

    @sensors_n.setter
    def sensors_n(self, value: Optional[int]) -> None: ...
    @property
    def solve_n(self) -> Optional[int]:
        """
        The number of solve-for states in the orbit determination.
        """
        ...

    @solve_n.setter
    def solve_n(self, value: Optional[int]) -> None: ...
    @property
    def solve_states(self) -> Optional[str]:
        """
        Free-text comma-delimited description of the state elements solved for in the orbit
        determination.
        """
        ...

    @solve_states.setter
    def solve_states(self, value: Optional[str]) -> None: ...
    @property
    def tracks_available(self) -> Optional[int]:
        """
        The number of sensor tracks available for the OD within the actual time span (see
        definition of 'tracks', 1.5.2).
        """
        ...

    @tracks_available.setter
    def tracks_available(self, value: Optional[int]) -> None: ...
    @property
    def tracks_used(self) -> Optional[int]:
        """
        The number of sensor tracks accepted for the OD within the actual time span (see
        definition of 'tracks', 1.5.2).
        """
        ...

    @tracks_used.setter
    def tracks_used(self, value: Optional[int]) -> None: ...
    @property
    def weighted_rms(self) -> Optional[float]:
        """
        (Useful/valid only for Batch OD systems.) The weighted RMS residual ratio, defined as:
        .. math:: \text{Weighted RMS} = \sqrt{\frac{\sum_{i=1}^{N} w_i(y_i - \hat{y}_i)^2}{N}}
        Where yi is the ith observation measurement, ŷi is the current estimate of yi, wi =
        1/σi² is the weight (sigma) associated with the measurement at the ith time and N is
        the number of observations. This is a value that can generally identify the quality of
        the most recent vector update and is used by the analyst in evaluating the OD process.
        A value of 1.00 is ideal.
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
        """
        ...

    @albedo_grid_size.setter
    def albedo_grid_size(self, value: Optional[int]) -> None: ...
    @property
    def albedo_model(self) -> Optional[str]:
        """
        Name of the albedo model.
        """
        ...

    @albedo_model.setter
    def albedo_model(self, value: Optional[str]) -> None: ...
    @property
    def atmospheric_model(self) -> Optional[str]:
        """
        Name of atmosphere model, which shall be selected from the accepted set of values
        indicated in annex B, subsection B9.

        Examples: MSISE90, NRLMSIS00, J70, J71, JROBERTS, DTM, JB2008
        """
        ...

    @atmospheric_model.setter
    def atmospheric_model(self, value: Optional[str]) -> None: ...
    @property
    def central_body_rotation(self) -> Optional[float]:
        """
        Central body angular rotation rate, measured about the major principal axis of the
        inertia tensor of the central body, relating inertial, and central-body-fixed
        reference frames. NOTE—The rotation axis may be slightly offset from the inertial
        frame Z-axis definition.

        Units: deg/s
        """
        ...

    @central_body_rotation.setter
    def central_body_rotation(self, value: Optional[float]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (a contiguous set of one or more comment lines may be provided in the OCM
        Perturbations Specification only immediately after the PERT_START keyword; see 7.8 for
        comment formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def equatorial_radius(self) -> Optional[float]:
        """
        Oblate spheroid equatorial radius of the central body used in the message, if
        different from the gravity model.

        Units: km
        """
        ...

    @equatorial_radius.setter
    def equatorial_radius(self, value: Optional[float]) -> None: ...
    @property
    def fixed_f10p7(self) -> Optional[float]:
        """
        Fixed F10.7 solar flux.

        Units: SFU
        """
        ...

    @fixed_f10p7.setter
    def fixed_f10p7(self, value: Optional[float]) -> None: ...
    @property
    def fixed_f10p7_mean(self) -> Optional[float]:
        """
        Fixed 81-day average F10.7 solar flux.

        Units: SFU
        """
        ...

    @fixed_f10p7_mean.setter
    def fixed_f10p7_mean(self, value: Optional[float]) -> None: ...
    @property
    def fixed_geomag_ap(self) -> Optional[float]:
        """
        Fixed geomagnetic Ap index.
        """
        ...

    @fixed_geomag_ap.setter
    def fixed_geomag_ap(self, value: Optional[float]) -> None: ...
    @property
    def fixed_geomag_dst(self) -> Optional[float]:
        """
        Fixed geomagnetic Dst index.
        """
        ...

    @fixed_geomag_dst.setter
    def fixed_geomag_dst(self, value: Optional[float]) -> None: ...
    @property
    def fixed_geomag_kp(self) -> Optional[float]:
        """
        Fixed geomagnetic Kp index.
        """
        ...

    @fixed_geomag_kp.setter
    def fixed_geomag_kp(self, value: Optional[float]) -> None: ...
    @property
    def fixed_m10p7(self) -> Optional[float]:
        """
        Fixed M10.7 solar flux.

        Units: SFU
        """
        ...

    @fixed_m10p7.setter
    def fixed_m10p7(self, value: Optional[float]) -> None: ...
    @property
    def fixed_m10p7_mean(self) -> Optional[float]:
        """
        Fixed 81-day average M10.7 solar flux.

        Units: SFU
        """
        ...

    @fixed_m10p7_mean.setter
    def fixed_m10p7_mean(self, value: Optional[float]) -> None: ...
    @property
    def fixed_s10p7(self) -> Optional[float]:
        """
        Fixed S10.7 solar flux.

        Units: SFU
        """
        ...

    @fixed_s10p7.setter
    def fixed_s10p7(self, value: Optional[float]) -> None: ...
    @property
    def fixed_s10p7_mean(self) -> Optional[float]:
        """
        Fixed 81-day average S10.7 solar flux.

        Units: SFU
        """
        ...

    @fixed_s10p7_mean.setter
    def fixed_s10p7_mean(self, value: Optional[float]) -> None: ...
    @property
    def fixed_y10p7(self) -> Optional[float]:
        """
        Fixed Y10.7 solar flux.

        Units: SFU
        """
        ...

    @fixed_y10p7.setter
    def fixed_y10p7(self, value: Optional[float]) -> None: ...
    @property
    def fixed_y10p7_mean(self) -> Optional[float]:
        """
        Fixed 81-day average Y10.7 solar flux.

        Units: SFU
        """
        ...

    @fixed_y10p7_mean.setter
    def fixed_y10p7_mean(self, value: Optional[float]) -> None: ...
    @property
    def gm(self) -> Optional[float]:
        """
        Gravitational coefficient of attracting body (Gravitational Constant × Central Mass),
        if different from the gravity model.

        Units: km³/s²
        """
        ...

    @gm.setter
    def gm(self, value: Optional[float]) -> None: ...
    @property
    def gravity_model(self) -> Optional[str]:
        """
        The gravity model (selected from the accepted set of gravity model names indicated in
        annex B, subsection B10), followed by the degree (D) and order (O) of the applied
        spherical harmonic coefficients used in the simulation. NOTE—Specifying a zero value
        for 'order' (e.g., 2D 0O) denotes zonals (J2 ... JD).

        Examples: EGM-96: 36D 36O, WGS-84: 8D 0O, GGM-01: 36D 36O, TEG-4: 36D 36O
        """
        ...

    @gravity_model.setter
    def gravity_model(self, value: Optional[str]) -> None: ...
    @property
    def n_body_perturbations(self) -> Optional[str]:
        """
        One OR MORE (N-body) gravitational perturbations bodies used. Values, listed serially
        in comma-delimited fashion, denote a natural solar or extra-solar system body (stars,
        planets, asteroids, comets, and natural satellites). NOTE—Only those entries specified
        under CENTER_NAME in annex B, subsection B2 are acceptable values.

        Examples: MOON, SUN, JUPITER
        """
        ...

    @n_body_perturbations.setter
    def n_body_perturbations(self, value: Optional[str]) -> None: ...
    @property
    def oblate_flattening(self) -> Optional[float]:
        """
        Central body's oblate spheroid oblateness for the polar-symmetric oblate central body
        model (e.g., for the Earth, it is approximately 1.0/298.257223563).
        """
        ...

    @oblate_flattening.setter
    def oblate_flattening(self, value: Optional[float]) -> None: ...
    @property
    def ocean_tides_model(self) -> Optional[str]:
        """
        Name of ocean tides model (optionally specify order or constituent effects, diurnal,
        semi-diurnal, etc.). This is a free-text field, so if the examples on the right are
        insufficient, others may be used.

        Examples: DIURNAL, SEMI-DIURNAL
        """
        ...

    @ocean_tides_model.setter
    def ocean_tides_model(self, value: Optional[str]) -> None: ...
    @property
    def reduction_theory(self) -> Optional[str]:
        """
        Specification of the reduction theory used for precession and nutation modeling. This
        is a free-text field, so if the examples on the right are insufficient, others may be
        used.

        Examples: IAU1976/FK5, IAU2010, IERS1996
        """
        ...

    @reduction_theory.setter
    def reduction_theory(self, value: Optional[str]) -> None: ...
    @property
    def shadow_bodies(self) -> Optional[str]:
        """
        List of bodies included in shadow calculations (value(s) to be drawn from the SANA
        registry list of Orbit Centers at <https://sanaregistry.org/r/orbit_centers>).

        Examples: EARTH, MOON
        """
        ...

    @shadow_bodies.setter
    def shadow_bodies(self, value: Optional[str]) -> None: ...
    @property
    def shadow_model(self) -> Optional[str]:
        """
        Shadow model used for Solar Radiation Pressure; dual cone uses both umbra/penumbra
        regions. Selected option should be one of ‘NONE’, ‘CYLINDRICAL’, ‘CONE’, or
        ‘DUAL_CONE’.

        Examples: NONE, CYLINDRICAL, CONE, DUAL_CONE
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
        Free-text field specifying the method used to select or interpolate any and all
        sequential space weather data (Kp, ap, Dst, F10.7, M10.7, S10.7, Y10.7, etc.). While
        not constrained to specific entries, it is anticipated that the utilized method would
        match methods detailed in numerical analysis textbooks.

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
        DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along intermediate OEB (Ŷoeb) direction
        as defined in annex F.

        Examples: 0.3

        Units: m²
        """
        ...

    @area_along_oeb_int.setter
    def area_along_oeb_int(self, value: Optional[float]) -> None: ...
    @property
    def area_along_oeb_max(self) -> Optional[float]:
        """
        Attitude-dependent cross-sectional area of space object (not already included in
        DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along max OEB (Xoeb) direction as
        defined in annex F.

        Examples: 0.15

        Units: m²
        """
        ...

    @area_along_oeb_max.setter
    def area_along_oeb_max(self, value: Optional[float]) -> None: ...
    @property
    def area_along_oeb_min(self) -> Optional[float]:
        """
        Attitude-dependent cross-sectional area of space object (not already included in
        DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along minimum OEB (Ẑoeb) direction as
        defined in annex F.

        Examples: 0.5

        Units: m²
        """
        ...

    @area_along_oeb_min.setter
    def area_along_oeb_min(self, value: Optional[float]) -> None: ...
    @property
    def area_max_for_pc(self) -> Optional[float]:
        """
        Maximum cross-sectional area for collision probability estimation purposes.

        Examples: 1.0

        Units: m²
        """
        ...

    @area_max_for_pc.setter
    def area_max_for_pc(self, value: Optional[float]) -> None: ...
    @property
    def area_min_for_pc(self) -> Optional[float]:
        """
        Minimum cross-sectional area for collision probability estimation purposes.

        Examples: 1.0

        Units: m²
        """
        ...

    @area_min_for_pc.setter
    def area_min_for_pc(self, value: Optional[float]) -> None: ...
    @property
    def area_typ_for_pc(self) -> Optional[float]:
        """
        Typical (50th percentile) cross-sectional area sampled over all space object
        orientations for collision probability estimation purposes.

        Units: m²
        """
        ...

    @area_typ_for_pc.setter
    def area_typ_for_pc(self, value: Optional[float]) -> None: ...
    @property
    def att_actuator_type(self) -> Optional[str]:
        """
        Free-text specification of type of actuator for attitude control. Suggested examples
        include: ATT_THRUSTERS, ACTIVE_MAG_TORQUE, PASSIVE_MAG_TORQUE, REACTION_WHEELS,
        MOMENTUM_WHEELS, CONTROL_MOMENT_GYROSCOPE, NONE, OTHER

        Examples: ATT_THRUSTERS
        """
        ...

    @att_actuator_type.setter
    def att_actuator_type(self, value: Optional[str]) -> None: ...
    @property
    def att_control(self) -> Optional[float]:
        """
        Accuracy of attitude control system (ACS) to maintain attitude, assuming attitude
        knowledge was perfect (i.e., deadbands).

        Examples: 2.0

        Units: deg
        """
        ...

    @att_control.setter
    def att_control(self, value: Optional[float]) -> None: ...
    @property
    def att_control_mode(self) -> Optional[str]:
        """
        Free-text specification of primary mode of attitude control for the space object.
        Suggested examples include: THREE_AXIS, SPIN, DUAL_SPIN, TUMBLING, GRAVITY_GRADIENT

        Examples: SPIN
        """
        ...

    @att_control_mode.setter
    def att_control_mode(self, value: Optional[str]) -> None: ...
    @property
    def att_knowledge(self) -> Optional[float]:
        """
        Accuracy of attitude knowledge.

        Examples: 0.3

        Units: deg
        """
        ...

    @att_knowledge.setter
    def att_knowledge(self, value: Optional[float]) -> None: ...
    @property
    def att_pointing(self) -> Optional[float]:
        """
        Overall accuracy of spacecraft to maintain attitude, including attitude knowledge
        errors and ACS operation.

        Examples: 2.3

        Units: deg
        """
        ...

    @att_pointing.setter
    def att_pointing(self, value: Optional[float]) -> None: ...
    @property
    def avg_maneuver_freq(self) -> Optional[float]:
        """
        Average maneuver frequency, measured in the number of orbit- or attitude-adjust
        maneuvers per year.

        Examples: 20.0

        Units: #/yr
        """
        ...

    @avg_maneuver_freq.setter
    def avg_maneuver_freq(self, value: Optional[float]) -> None: ...
    @property
    def bus_model(self) -> Optional[str]:
        """
        Free-text field containing the satellite manufacturer's spacecraft bus model name.

        Examples: 702
        """
        ...

    @bus_model.setter
    def bus_model(self, value: Optional[str]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (a contiguous set of one or more comment lines may be provided in the OCM Space
        Object Physical Characteristics only immediately after the PHYS_START keyword; see 7.8
        for comment formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def docked_with(self) -> Optional[str]:
        """
        Free-text field containing a comma-separated list of other space objects that this
        object is docked to.

        Examples: ISS
        """
        ...

    @docked_with.setter
    def docked_with(self, value: Optional[str]) -> None: ...
    @property
    def drag_coeff_nom(self) -> Optional[float]:
        """
        Nominal drag Coefficient (CD Nom). If the atmospheric drag coefficient, CD, is set to
        zero, no atmospheric drag shall be considered.

        Examples: 2.2
        """
        ...

    @drag_coeff_nom.setter
    def drag_coeff_nom(self, value: Optional[float]) -> None: ...
    @property
    def drag_const_area(self) -> Optional[float]:
        """
        Attitude-independent drag cross-sectional area (AD) facing the relative wind vector,
        not already incorporated into the attitude-dependent 'AREA_ALONG_OEB' parameters.

        Examples: 2.5

        Units: m²
        """
        ...

    @drag_const_area.setter
    def drag_const_area(self, value: Optional[float]) -> None: ...
    @property
    def drag_uncertainty(self) -> Optional[float]:
        """
        Drag coefficient one sigma (1σ) percent uncertainty, where the actual range of drag
        coefficients to within 1σ shall be obtained from [1.0 ± DRAG_UNCERTAINTY/100.0] * (CD
        Nom). This factor is intended to allow operators to supply the nominal ballistic
        coefficient components while accommodating ballistic coefficient uncertainties.

        Examples: 10.0

        Units: %
        """
        ...

    @drag_uncertainty.setter
    def drag_uncertainty(self, value: Optional[float]) -> None: ...
    @property
    def dry_mass(self) -> Optional[float]:
        """
        Space object dry mass (without propellant).

        Examples: 300

        Units: kg
        """
        ...

    @dry_mass.setter
    def dry_mass(self, value: Optional[float]) -> None: ...
    @property
    def dv_bol(self) -> Optional[float]:
        """
        Total ΔV capability of the spacecraft at beginning of life.

        Examples: 1.0

        Units: km/s
        """
        ...

    @dv_bol.setter
    def dv_bol(self, value: Optional[float]) -> None: ...
    @property
    def dv_remaining(self) -> Optional[float]:
        """
        Total ΔV remaining for the spacecraft.

        Examples: 0.2

        Units: km/s
        """
        ...

    @dv_remaining.setter
    def dv_remaining(self, value: Optional[float]) -> None: ...
    @property
    def initial_wet_mass(self) -> Optional[float]:
        """
        Space object total mass at beginning of life.

        Examples: 500

        Units: kg
        """
        ...

    @initial_wet_mass.setter
    def initial_wet_mass(self, value: Optional[float]) -> None: ...
    @property
    def ixx(self) -> Optional[float]:
        """
        Moment of Inertia about the X-axis of the space object's primary body frame (e.g.,
        SC_Body_1) (see reference `[H1]`).

        Examples: 1000.0

        Units: kg·m²
        """
        ...

    @ixx.setter
    def ixx(self, value: Optional[float]) -> None: ...
    @property
    def ixy(self) -> Optional[float]:
        """
        Inertia Cross Product of the X & Y axes.

        Examples: 20.0

        Units: kg·m²
        """
        ...

    @ixy.setter
    def ixy(self, value: Optional[float]) -> None: ...
    @property
    def ixz(self) -> Optional[float]:
        """
        Inertia Cross Product of the X & Z axes.

        Examples: 40.0

        Units: kg·m²
        """
        ...

    @ixz.setter
    def ixz(self, value: Optional[float]) -> None: ...
    @property
    def iyy(self) -> Optional[float]:
        """
        Moment of Inertia about the Y-axis.

        Examples: 800.0

        Units: kg·m²
        """
        ...

    @iyy.setter
    def iyy(self, value: Optional[float]) -> None: ...
    @property
    def iyz(self) -> Optional[float]:
        """
        Inertia Cross Product of the Y & Z axes.

        Examples: 60.0

        Units: kg·m²
        """
        ...

    @iyz.setter
    def iyz(self, value: Optional[float]) -> None: ...
    @property
    def izz(self) -> Optional[float]:
        """
        Moment of Inertia about the Z-axis.

        Examples: 400.0

        Units: kg·m²
        """
        ...

    @izz.setter
    def izz(self, value: Optional[float]) -> None: ...
    @property
    def manufacturer(self) -> Optional[str]:
        """
        Free-text field containing the satellite manufacturer's name.

        Examples: BOEING
        """
        ...

    @manufacturer.setter
    def manufacturer(self, value: Optional[str]) -> None: ...
    @property
    def max_thrust(self) -> Optional[float]:
        """
        Maximum composite thrust the spacecraft can accomplish in any single body-fixed
        direction.

        Examples: 1.0

        Units: N
        """
        ...

    @max_thrust.setter
    def max_thrust(self, value: Optional[float]) -> None: ...
    @property
    def oeb_int(self) -> Optional[float]:
        """
        Intermediate physical dimension (along Ŷoeb) of OEB normal to OEB_MAX direction.

        Examples: 0.5

        Units: m
        """
        ...

    @oeb_int.setter
    def oeb_int(self, value: Optional[float]) -> None: ...
    @property
    def oeb_max(self) -> Optional[float]:
        """
        Maximum physical dimension (along Xoeb) of the OEB.

        Examples: 1

        Units: m
        """
        ...

    @oeb_max.setter
    def oeb_max(self, value: Optional[float]) -> None: ...
    @property
    def oeb_min(self) -> Optional[float]:
        """
        Minimum physical dimension (along Ẑoeb) of OEB in direction normal to both OEB_MAX and
        OEB_INT directions.

        Examples: 0.3

        Units: m
        """
        ...

    @oeb_min.setter
    def oeb_min(self, value: Optional[float]) -> None: ...
    @property
    def oeb_parent_frame(self) -> Optional[str]:
        """
        Parent reference frame that maps to the OEB frame via the quaternion-based
        transformation defined in annex F, subsection F1. Select from the accepted set of
        values indicated in B, subsections B4 and B5. This keyword shall be provided if
        OEB_Q1,2,3,4 are specified.

        Examples: ITRF1997
        """
        ...

    @oeb_parent_frame.setter
    def oeb_parent_frame(self, value: Optional[str]) -> None: ...
    @property
    def oeb_parent_frame_epoch(self) -> Optional[str]:
        """
        Epoch of the OEB parent frame, if OEB_PARENT_FRAME is provided and its epoch is not
        intrinsic to the definition of the reference frame. (See 7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @oeb_parent_frame_epoch.setter
    def oeb_parent_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def oeb_q1(self) -> Optional[float]:
        """
        q1 = e1 * sin(φ/2), where per reference `[H1]`, φ = Euler rotation angle and e1 = 1st
        component of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME
        (defined above) to the frame aligned with the OEB (defined in annex F, subsection F1).
        A value of '-999' denotes a tumbling space object.

        Examples: -0.575131822
        """
        ...

    @oeb_q1.setter
    def oeb_q1(self, value: Optional[float]) -> None: ...
    @property
    def oeb_q2(self) -> Optional[float]:
        """
        q2 = e2 * sin(φ/2), where per reference `[H1]`, φ = Euler rotation angle and e2 = 2nd
        component of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME
        (defined above) to the frame aligned with the Optimally Encompassing Box (defined in
        annex F, subsection F1). A value of '-999' denotes a tumbling space object.

        Examples: -0.280510532
        """
        ...

    @oeb_q2.setter
    def oeb_q2(self, value: Optional[float]) -> None: ...
    @property
    def oeb_q3(self) -> Optional[float]:
        """
        q3 = e3 * sin(φ/2), where per reference `[H1]`, φ = Euler rotation angle and e3 = 3rd
        component of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME
        (defined above) to the frame aligned with the Optimally Encompassing Box (defined in
        annex F, subsection F1). A value of '-999' denotes a tumbling space object.

        Examples: -0.195634856
        """
        ...

    @oeb_q3.setter
    def oeb_q3(self, value: Optional[float]) -> None: ...
    @property
    def oeb_qc(self) -> Optional[float]:
        """
        qc = cos(φ/2), where per reference `[H1]`, φ = the Euler rotation angle for the rotation
        that maps from the OEB_PARENT_FRAME (defined above) to the frame aligned with the
        Optimally Encompassing Box (annex F, subsection F1). qc shall be made non-negative by
        convention. A value of '-999' denotes a tumbling space object.

        Examples: 0.743144825
        """
        ...

    @oeb_qc.setter
    def oeb_qc(self, value: Optional[float]) -> None: ...
    @property
    def rcs(self) -> Optional[float]:
        """
        Typical (50th percentile) effective Radar Cross Section of the space object sampled
        over all possible viewing angles.

        Units: m²
        """
        ...

    @rcs.setter
    def rcs(self, value: Optional[float]) -> None: ...
    @property
    def rcs_max(self) -> Optional[float]:
        """
        Maximum Radar Cross Section observed for this object.

        Units: m²
        """
        ...

    @rcs_max.setter
    def rcs_max(self, value: Optional[float]) -> None: ...
    @property
    def rcs_min(self) -> Optional[float]:
        """
        Minimum Radar Cross Section observed for this object.

        Units: m²
        """
        ...

    @rcs_min.setter
    def rcs_min(self, value: Optional[float]) -> None: ...
    @property
    def reflectance(self) -> Optional[float]:
        """
        Typical (50th percentile) coefficient of REFLECTANCE of the space object over all
        possible viewing angles, ranging from 0 (none) to 1 (perfect reflectance).

        Examples: 0.7
        """
        ...

    @reflectance.setter
    def reflectance(self, value: Optional[float]) -> None: ...
    @property
    def solar_rad_coeff(self) -> Optional[float]:
        """
        Nominal Solar Radiation Pressure Coefficient (CR NOM). If the solar radiation
        coefficient, CR, is set to zero, no solar radiation pressure shall be considered.
        """
        ...

    @solar_rad_coeff.setter
    def solar_rad_coeff(self, value: Optional[float]) -> None: ...
    @property
    def solar_rad_uncertainty(self) -> Optional[float]:
        """
        SRP one sigma (1σ) percent uncertainty, where the actual range of SRP coefficients to
        within 1σ shall be obtained from [1.0 ± 0.01*SRP_UNCERTAINTY] (CR NOM). This factor is
        intended to allow operators to supply the nominal ballistic coefficient components
        while accommodating ballistic coefficient uncertainties.

        Units: %
        """
        ...

    @solar_rad_uncertainty.setter
    def solar_rad_uncertainty(self, value: Optional[float]) -> None: ...
    @property
    def srp_const_area(self) -> Optional[float]:
        """
        Attitude-independent solar radiation pressure cross-sectional area (AR) facing the Sun,
        not already incorporated into the attitude-dependent ‘AREA_ALONG_OEB’ parameters.

        Units: m²
        """
        ...

    @srp_const_area.setter
    def srp_const_area(self, value: Optional[float]) -> None: ...
    @property
    def vm_absolute(self) -> Optional[float]:
        """
        Typical (50th percentile) Visual Magnitude of the space object sampled over all
        possible viewing angles and sampled over all possible viewing angles and ‘normalized’
        as specified in informative annex F, subsection F2 to a 1 AU Sun-to-target distance,
        a phase angle of 0°, and a 40,000 km target-to-sensor distance (equivalent of GEO
        satellite tracked at 15.6° above local horizon).

        Examples: 15.0
        """
        ...

    @vm_absolute.setter
    def vm_absolute(self, value: Optional[float]) -> None: ...
    @property
    def vm_apparent(self) -> Optional[float]:
        """
        Typical (50th percentile) apparent Visual Magnitude observed for this space object.

        Examples: 15.0
        """
        ...

    @vm_apparent.setter
    def vm_apparent(self, value: Optional[float]) -> None: ...
    @property
    def vm_apparent_max(self) -> Optional[float]:
        """
        Maximum apparent Visual Magnitude observed for this space object. NOTE—The 'MAX' value
        represents the brightest observation, which associates with a lower Vmag.

        Examples: 16.0
        """
        ...

    @vm_apparent_max.setter
    def vm_apparent_max(self, value: Optional[float]) -> None: ...
    @property
    def vm_apparent_min(self) -> Optional[float]:
        """
        Minimum apparent Visual Magnitude observed for this space object.

        Examples: 19.0
        """
        ...

    @vm_apparent_min.setter
    def vm_apparent_min(self, value: Optional[float]) -> None: ...
    @property
    def wet_mass(self) -> Optional[float]:
        """
        Space object total mass (including propellant, i.e., 'wet mass') at the current
        reference epoch 'EPOCH_TZERO'.

        Examples: 472.3

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
        Origin of the orbit reference frame, which may be a natural solar system body (planets,
        asteroids, comets, and natural satellites), including any planet barycenter or the solar
        system barycenter, or another reference frame center (such as a spacecraft, formation
        flying reference 'chief' spacecraft, etc.). Natural bodies shall be selected from the
        accepted set of values indicated in annex B, subsection B2. For spacecraft, it is
        recommended to use either the 'OBJECT_NAME' or 'INTERNATIONAL_DESIGNATOR' of the
        participant as catalogued in the UN Office of Outer Space Affairs designator index
        (reference `[3]`). Alternately, the 'OBJECT_DESIGNATOR' may be used. For other reference
        frame origins, this field is a free-text descriptor which may draw upon other naming
        conventions and sources.

        Examples: EARTH, MOON, ISS, EROS
        """
        ...

    @center_name.setter
    def center_name(self, value: str) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (a contiguous set of one or more comment lines may be provided in the
        Trajectory State Time History section only immediately after the TRAJ_START keyword;
        see 7.8 for comment formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def interpolation(self) -> Optional[str]:
        """
        This keyword may be used to specify the recommended interpolation method for ephemeris
        data in the immediately following set of ephemeris lines. PROPAGATE indicates that orbit
        propagation is the preferred method to obtain states at intermediate times, via either
        a midpoint-switching or endpoint switching approach.

        Examples: HERMITE, LINEAR, LAGRANGE, PROPAGATE
        """
        ...

    @interpolation.setter
    def interpolation(self, value: Optional[str]) -> None: ...
    @property
    def interpolation_degree(self) -> Optional[int]:
        """
        Recommended interpolation degree for ephemeris data in the immediately following set of
        ephemeris lines. Must be an integer value. This keyword must be provided if the
        'INTERPOLATION' keyword is used and set to anything other than PROPAGATE.

        Examples: 5, 1
        """
        ...

    @interpolation_degree.setter
    def interpolation_degree(self, value: Optional[int]) -> None: ...
    @property
    def orb_averaging(self) -> Optional[str]:
        """
        If orbital elements are provided, specifies whether those elements are osculating
        elements or mean elements, and if mean elements, which mean element definition is
        employed. The intent of this field is to allow the user to correctly interpret how to
        use the provided orbit elements and know how to use them operationally. This field is
        not required if one of the orbital element types selected by the "TRAJ_TYPE" keyword is
        Cartesian (e.g., CARTP, CARTPV, or CARTPVA) or spherical elements (e.g., LDBARV, ADBARV,
        or GEODETIC). Values should be selected from the accepted set indicated in annex B,
        subsection B13. If an alternate single- or double-averaging formulation other than that
        provided is used, the user may name it as mutually agreed upon by message exchange
        participants.

        Examples: OSCULATING, BROUWER, KOZAI
        """
        ...

    @orb_averaging.setter
    def orb_averaging(self, value: Optional[str]) -> None: ...
    @property
    def orb_revnum(self) -> Optional[float]:
        """
        The integer orbit revolution number associated with the first trajectory state in this
        trajectory state time history block. NOTE—The first ascending node crossing that occurs
        AFTER launch or deployment is designated to be the beginning of orbit revolution number
        = one ('1').

        Examples: 1500, 30007
        """
        ...

    @orb_revnum.setter
    def orb_revnum(self, value: Optional[float]) -> None: ...
    @property
    def orb_revnum_basis(self) -> Optional[str]:
        """
        Specifies the message creator's basis for their orbit revolution counter, with '0',
        designating that the first launch or deployment trajectory state corresponds to a
        revolution number of 0.XXXX, where XXXX represents the fraction of an orbit revolution
        measured from the equatorial plane, and orbit revolution 1.0 begins at the very next
        (subsequent) ascending node passage; '1', designating that the first launch or
        deployment trajectory state corresponds to a revolution number of 1.XXXX, and orbit
        revolution 2.0 begins at the very next ascending node passage. This keyword shall be
        provided if ORB_REVNUM is specified.

        Examples: 0, 1
        """
        ...

    @orb_revnum_basis.setter
    def orb_revnum_basis(self, value: Optional[str]) -> None: ...
    @property
    def propagator(self) -> Optional[str]:
        """
        Free-text field containing the name of the orbit propagator used to create this
        trajectory state time history.

        Examples: HPOP, SP, SGP4
        """
        ...

    @propagator.setter
    def propagator(self, value: Optional[str]) -> None: ...
    @property
    def traj_basis(self) -> Optional[str]:
        """
        The basis of this trajectory state time history data. This is a free-text field with the
        following suggested values: a) 'PREDICTED'. b) 'DETERMINED' when estimated from
        observation-based orbit determination, reconstruction, and/or calibration. For
        definitive OD performed onboard spacecraft whose solutions have been telemetered to the
        ground for inclusion in an OCM, the TRAJ_BASIS shall be DETERMINED. c) 'TELEMETRY' when
        the trajectory states are read directly from telemetry, for example, based on inertial
        navigation systems or GNSS data. d) 'SIMULATED' for generic simulations, future mission
        design studies, and optimization studies. e) 'OTHER' for other bases of this data.

        Examples: PREDICTED
        """
        ...

    @traj_basis.setter
    def traj_basis(self, value: Optional[str]) -> None: ...
    @property
    def traj_basis_id(self) -> Optional[str]:
        """
        Free-text field containing the identification number for the telemetry dataset, orbit
        determination, navigation solution, or simulation upon which this trajectory state time
        history block is based. When a matching orbit determination block accompanies this
        trajectory state time history, the TRAJ_BASIS_ID should match the corresponding OD_ID
        (see table 6-11).

        Examples: OD_5910
        """
        ...

    @traj_basis_id.setter
    def traj_basis_id(self, value: Optional[str]) -> None: ...
    @property
    def traj_frame_epoch(self) -> Optional[str]:
        """
        Epoch of the orbit data reference frame, if not intrinsic to the definition of the
        reference frame. (See 7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @traj_frame_epoch.setter
    def traj_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def traj_id(self) -> Optional[str]:
        """
        Free-text field containing the identification number for this trajectory state time
        history block.

        Examples: TRAJ_20160402_XYZ
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
        Free-text field containing the identification number for the next trajectory state
        time history, contained either within this message, or presented in a future OCM.
        NOTE—If this message is not part of a sequence of orbit time histories or if this
        trajectory state time history is the last in a sequence of orbit time histories, then
        TRAJ_NEXT_ID should be excluded from this message.

        Examples: ORB20160305C
        """
        ...

    @traj_next_id.setter
    def traj_next_id(self, value: Optional[str]) -> None: ...
    @property
    def traj_prev_id(self) -> Optional[str]:
        """
        Free-text field containing the identification number for the previous trajectory state
        time history, contained either within this message or presented in a previous OCM.
        NOTE—If this message is not part of a sequence of orbit time histories or if this
        trajectory state time history is the first in a sequence of orbit time histories, then
        TRAJ_PREV_ID should be excluded from this message.

        Examples: ORB20160305A
        """
        ...

    @traj_prev_id.setter
    def traj_prev_id(self, value: Optional[str]) -> None: ...
    @property
    def traj_ref_frame(self) -> str:
        """
        Reference frame of the trajectory state time history. Select from the accepted set of
        values indicated in annex B, subsection B4.

        Examples: ICRF3, J2000
        """
        ...

    @traj_ref_frame.setter
    def traj_ref_frame(self, value: str) -> None: ...
    @property
    def traj_type(self) -> str:
        """
        Specifies the trajectory state type; selected per annex B, subsection B7.

        Examples: CARTP, CARTPV
        """
        ...

    @traj_type.setter
    def traj_type(self, value: str) -> None: ...
    @property
    def traj_units(self) -> Optional[str]:
        """
        A comma-delimited set of SI unit designations for each element of the trajectory state
        time history following the trajectory state time tag solely for informational purposes,
        provided as a free-text field enclosed in square brackets. When provided, each
        trajectory state element shall have a corresponding units entry, with non-dimensional
        values (such as orbit eccentricity) denoted by 'n/a'. NOTE—The listing of units via the
        TRAJ_UNITS keyword does not override the mandatory units specified for the selected
        TRAJ_TYPE (links to the relevant SANA registries provided in annex B, subsection B7).

        Examples: [km,km,km,km/s,km/s,km/s], [km,n/a,deg, deg, deg, deg]
        """
        ...

    @traj_units.setter
    def traj_units(self, value: Optional[str]) -> None: ...
    @property
    def useable_start_time(self) -> Optional[str]:
        """
        Start time of USEABLE time span covered by ephemeris data immediately following this
        metadata block. (For format specification, see 7.5.10.) NOTES 1. This optional keyword
        allows the message creator to introduce fictitious (but numerically smooth) data nodes
        following the actual data time history to support interpolation methods requiring more
        than two nodes (e.g., pure higher-order Lagrange interpolation methods). The use of this
        keyword and introduction of fictitious node points are optional and may not be necessary.
        2. If this keyword is not supplied, then all data shall be assumed to be valid.

        Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
        """
        ...

    @useable_start_time.setter
    def useable_start_time(self, value: Optional[str]) -> None: ...
    @property
    def useable_stop_time(self) -> Optional[str]:
        """
        Stop time of USEABLE time span covered by ephemeris data immediately following this
        metadata block. (For format specification, see 7.5.10.) NOTES 1. This optional keyword
        allows the message creator to introduce fictitious (but numerically smooth) data nodes
        following the actual data time history to support interpolation methods requiring more
        than two nodes (e.g., pure higher-order Lagrange interpolation methods). The use of this
        keyword and introduction of fictitious node points are optional and may not be necessary.
        2. If this keyword is not supplied, then all data shall be assumed to be valid.

        Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
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
        time_lastob_start=None,
        time_lastob_end=None,
        recommended_od_span=None,
        actual_od_span=None,
        obs_available=None,
        obs_used=None,
        tracks_available=None,
        tracks_used=None,
        residuals_accepted=None,
        weighted_rms=None,
        comment=...,
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

        Units: %
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
        creation_date, originator, classification=None, message_id=None, comment=None
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
        Comments (allowed in the ODM Header only immediately after the ODM version number).
        (See 7.8 for formatting rules.)

        Examples: This is a comment
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
        ID that uniquely identifies a message from a given originator. The format and content of
        the message identifier value are at the discretion of the originator.

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

    An OEM specifies the position and velocity of a single object at multiple epochs contained
    within a specified time range. The message recipient must have a means of interpolating
    across these state vectors to obtain the state at an arbitrary time contained within the
    span of the ephemeris.

    The OEM is suited to exchanges that:
    1. Involve automated interaction (e.g., computer-to-computer communication).
    2. Require higher fidelity or higher precision dynamic modeling than is possible with the OPM.

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
    def id(self) -> Optional[str]:
        """
        The message identifier.
        """
        ...

    @property
    def segments(self) -> list[OemSegment]:
        """
        The list of data segments.
        """
        ...

    @segments.setter
    def segments(self, value: list[OemSegment]) -> None: ...
    def to_file(self, path, format, validate=True):
        """
        Write to file.

        Parameters
        ----------
        path : str
            Output file path.
        format : str
            Output format ('kvn' or 'xml').
        Write to file.

        Parameters
        ----------
        path : str
            Output file path.
        format : str
            Output format ('kvn' or 'xml').
        validate : bool, optional
            Whether to validate the message before writing (default: True).
        """
        ...

    def to_str(self, format, validate=True):
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
        Serialize to string.

        Parameters
        ----------
        format : str
            Output format ('kvn' or 'xml').
        validate : bool, optional
            Whether to validate the message before writing (default: True).

        Returns
        -------
        str
            The serialized string.
        """
        ...

    def validate(self, strict=True):
        """
        Validate the message against CCSDS rules.

        Parameters
        ----------
        strict : bool, optional
            If True (default), raises ValueError on the first error found.
            If False, returns a list of validation error messages (or None if valid).
        """
        ...

    @property
    def version(self) -> str:
        """
        The message version.
        """
        ...

    @version.setter
    def version(self, value: str) -> None: ...

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
        NumPy array of shape (21,) containing the lower-triangular values, or (6,6) for
        a full symmetric matrix.
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
        Covariance matrix `[4,1]`

        Units: km²/s
        """
        ...

    @cx_dot_x.setter
    def cx_dot_x(self, value: float) -> None: ...
    @property
    def cx_dot_x_dot(self) -> float:
        """
        Covariance matrix `[4,4]`

        Units: km²/s²
        """
        ...

    @cx_dot_x_dot.setter
    def cx_dot_x_dot(self, value: float) -> None: ...
    @property
    def cx_dot_y(self) -> float:
        """
        Covariance matrix `[4,2]`

        Units: km²/s
        """
        ...

    @cx_dot_y.setter
    def cx_dot_y(self, value: float) -> None: ...
    @property
    def cx_dot_z(self) -> float:
        """
        Covariance matrix `[4,3]`

        Units: km²/s
        """
        ...

    @cx_dot_z.setter
    def cx_dot_z(self, value: float) -> None: ...
    @property
    def cx_x(self) -> float:
        """
        Covariance matrix `[1,1]`

        Units: km²
        """
        ...

    @cx_x.setter
    def cx_x(self, value: float) -> None: ...
    @property
    def cy_dot_x(self) -> float:
        """
        Covariance matrix `[5,1]`

        Units: km²/s
        """
        ...

    @cy_dot_x.setter
    def cy_dot_x(self, value: float) -> None: ...
    @property
    def cy_dot_x_dot(self) -> float:
        """
        Covariance matrix `[5,4]`

        Units: km²/s²
        """
        ...

    @cy_dot_x_dot.setter
    def cy_dot_x_dot(self, value: float) -> None: ...
    @property
    def cy_dot_y(self) -> float:
        """
        Covariance matrix `[5,2]`

        Units: km²/s
        """
        ...

    @cy_dot_y.setter
    def cy_dot_y(self, value: float) -> None: ...
    @property
    def cy_dot_y_dot(self) -> float:
        """
        Covariance matrix `[5,5]`

        Units: km²/s²
        """
        ...

    @cy_dot_y_dot.setter
    def cy_dot_y_dot(self, value: float) -> None: ...
    @property
    def cy_dot_z(self) -> float:
        """
        Covariance matrix `[5,3]`

        Units: km²/s
        """
        ...

    @cy_dot_z.setter
    def cy_dot_z(self, value: float) -> None: ...
    @property
    def cy_x(self) -> float:
        """
        Covariance matrix `[2,1]`

        Units: km²
        """
        ...

    @cy_x.setter
    def cy_x(self, value: float) -> None: ...
    @property
    def cy_y(self) -> float:
        """
        Covariance matrix `[2,2]`

        Units: km²
        """
        ...

    @cy_y.setter
    def cy_y(self, value: float) -> None: ...
    @property
    def cz_dot_x(self) -> float:
        """
        Covariance matrix `[6,1]`

        Units: km²/s
        """
        ...

    @cz_dot_x.setter
    def cz_dot_x(self, value: float) -> None: ...
    @property
    def cz_dot_x_dot(self) -> float:
        """
        Covariance matrix `[6,4]`

        Units: km²/s²
        """
        ...

    @cz_dot_x_dot.setter
    def cz_dot_x_dot(self, value: float) -> None: ...
    @property
    def cz_dot_y(self) -> float:
        """
        Covariance matrix `[6,2]`

        Units: km²/s
        """
        ...

    @cz_dot_y.setter
    def cz_dot_y(self, value: float) -> None: ...
    @property
    def cz_dot_y_dot(self) -> float:
        """
        Covariance matrix `[6,5]`

        Units: km²/s²
        """
        ...

    @cz_dot_y_dot.setter
    def cz_dot_y_dot(self, value: float) -> None: ...
    @property
    def cz_dot_z(self) -> float:
        """
        Covariance matrix `[6,3]`

        Units: km²/s
        """
        ...

    @cz_dot_z.setter
    def cz_dot_z(self, value: float) -> None: ...
    @property
    def cz_dot_z_dot(self) -> float:
        """
        Covariance matrix `[6,6]`

        Units: km²/s²
        """
        ...

    @cz_dot_z_dot.setter
    def cz_dot_z_dot(self, value: float) -> None: ...
    @property
    def cz_x(self) -> float:
        """
        Covariance matrix `[3,1]`

        Units: km²
        """
        ...

    @cz_x.setter
    def cz_x(self, value: float) -> None: ...
    @property
    def cz_y(self) -> float:
        """
        Covariance matrix `[3,2]`

        Units: km²
        """
        ...

    @cz_y.setter
    def cz_y(self, value: float) -> None: ...
    @property
    def cz_z(self) -> float:
        """
        Covariance matrix `[3,3]`

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
    def __init__(state_vectors, covariance_matrix=None, comments=None) -> None: ...
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
    def covariance_matrix_epochs(self) -> list[str]:
        """
        Epochs for covariance matrices (ISO 8601).
        """
        ...

    @covariance_matrix_epochs.setter
    def covariance_matrix_epochs(self, value: list[str]) -> None: ...
    @property
    def covariance_matrix_numpy(self) -> numpy.ndarray:
        """
        Get covariance matrices as a NumPy array.

        Use `covariance_matrix_epochs` for the corresponding epochs.

        The returned array is a 3D tensor of shape (N, 6, 6), where N is the number of covariance
        matrices. Each 6x6 matrix is symmetric and constructed from the lower-triangular CCSDS data.

        Indices: 0=X, 1=Y, 2=Z, 3=X_DOT, 4=Y_DOT, 5=Z_DOT
        """
        ...

    @covariance_matrix_numpy.setter
    def covariance_matrix_numpy(self, value: numpy.ndarray) -> None: ...
    @staticmethod
    def from_numpy(
        state_vector_epochs,
        state_vector_numpy,
        covariance_matrix_epochs=None,
        covariance_matrix_numpy=None,
        cov_ref_frames=None,
        cov_comments=None,
        comments=None,
    ):
        """ """
        ...

    @property
    def state_vector(self) -> list[StateVectorAcc]:
        """
        List of state vectors. Each vector contains position, velocity, and optional
        acceleration.

        Examples: 2020-01-01T00:00:00.000 1234.567 2345.678 3456.789 1.234 2.345 3.456

        Units: km, km/s, km/s²
        """
        ...

    @state_vector.setter
    def state_vector(self, value: list[StateVectorAcc]) -> None: ...
    @property
    def state_vector_epochs(self) -> list[str]:
        """
        Epochs for state vectors (ISO 8601).
        """
        ...

    @state_vector_epochs.setter
    def state_vector_epochs(self, value: list[str]) -> None: ...
    @property
    def state_vector_numpy(self) -> numpy.ndarray:
        """
        State vectors as a NumPy array.

        Use `state_vector_epochs` for the corresponding epochs.

        Returns
        -------
        numpy.ndarray
            2D array of shape (N, 6) or (N, 9):
            - N x 6: [X, Y, Z, X_DOT, Y_DOT, Z_DOT] if no accelerations.
            - N x 9: [X, Y, Z, X_DOT, Y_DOT, Z_DOT, X_DDOT, Y_DDOT, Z_DDOT] if accelerations present.

        Units:
        - Position: km
        - Velocity: km/s
        - Acceleration: km/s²
        """
        ...

    @state_vector_numpy.setter
    def state_vector_numpy(self, value: numpy.ndarray) -> None: ...
    def validate(self):
        """
        Validate the data section against CCSDS rules.
        """
        ...

class OemMetadata:
    """
    OEM Metadata Section.

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
        start_time,
        stop_time,
        center_name=...,
        ref_frame=None,
        time_system=None,
        ref_frame_epoch=None,
        useable_start_time=None,
        useable_stop_time=None,
        interpolation=None,
        interpolation_degree=None,
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
        Origin of the OEM reference frame, which may be a natural solar system body (planets,
        asteroids, comets, and natural satellites), including any planet barycenter or the
        solar system barycenter, or another reference frame center (such as a spacecraft,
        formation flying reference ‘chief’ spacecraft, etc.). Natural bodies shall be selected
        from the accepted set of values indicated in annex B, subsection B2. For spacecraft, it
        is recommended to use either the OBJECT_ID or international designator of the
        participant as catalogued in the UN Office of Outer Space Affairs designator index
        (reference `[3]`).

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
        This keyword may be used to specify the recommended interpolation method for ephemeris
        data in the immediately following set of ephemeris lines.

        Examples: HERMITE, LINEAR, LAGRANGE
        """
        ...

    @interpolation.setter
    def interpolation(self, value: Optional[str]) -> None: ...
    @property
    def interpolation_degree(self) -> Optional[int]:
        """
        Recommended interpolation degree for ephemeris data in the immediately following set of
        ephemeris lines. Must be an integer value. This keyword must be used if the
        ‘INTERPOLATION’ keyword is used.

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
        designator index. Recommended values have the format YYYY-NNNP{PP}, where: YYYY = Year
        of launch. NNN = Three-digit serial number of launch in year YYYY (with leading zeros).
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
        restriction on the value for this keyword, it is recommended to use names from the UN
        Office of Outer Space Affairs designator index (reference `[3]`, which include Object name
        and international designator of the participant). If OBJECT_NAME is not listed in
        reference `[3]` or the content is either unknown or cannot be disclosed, the value should
        be set to UNKNOWN.

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
        Epoch of reference frame, if not intrinsic to the definition of the reference frame.
        (See 7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @ref_frame_epoch.setter
    def ref_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def start_time(self) -> str:
        """
        Start of TOTAL time span covered by ephemeris data and covariance data immediately
        following this metadata block. (For format specification, see 7.5.10.)

        Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
        """
        ...

    @start_time.setter
    def start_time(self, value: str) -> None: ...
    @property
    def stop_time(self) -> str:
        """
        End of TOTAL time span covered by ephemeris data and covariance data immediately
        following this metadata block. (For format specification, see 7.5.10.)

        Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
        """
        ...

    @stop_time.setter
    def stop_time(self, value: str) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system used for ephemeris and covariance data. Use of values other than those in
        3.2.3.2 should be documented in an ICD.

        Examples: UTC, TAI, TT, GPS, TDB, TCB
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...
    @property
    def useable_start_time(self) -> Optional[str]:
        """
        Start time of USEABLE time span covered by ephemeris data immediately following this
        metadata block. (For format specification, see 7.5.10.) This optional keyword allows the
        message creator to introduce fictitious (but numerically smooth) data nodes prior to the
        actual data time history to support interpolation methods requiring more than two nodes
        (e.g., pure higher-order Lagrange interpolation methods). The use of this keyword and
        introduction of fictitious node points are optional and may not be necessary.

        Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
        """
        ...

    @useable_start_time.setter
    def useable_start_time(self, value: Optional[str]) -> None: ...
    @property
    def useable_stop_time(self) -> Optional[str]:
        """
        Stop time of USEABLE time span covered by ephemeris data immediately following this
        metadata block. (For format specification, see 7.5.10.) This optional keyword allows the
        message creator to introduce fictitious (but numerically smooth) data nodes following
        the actual data time history to support interpolation methods requiring more than two
        nodes (e.g., pure higher-order Lagrange interpolation methods). The use of this keyword
        and introduction of fictitious node points are optional and may not be necessary.

        Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
        """
        ...

    @useable_stop_time.setter
    def useable_stop_time(self, value: Optional[str]) -> None: ...
    def validate(self):
        """
        Validate the metadata against CCSDS rules.
        """
        ...

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
    def validate(self):
        """
        Validate the segment against CCSDS rules.
        """
        ...

class Omm:
    """
    Orbit Mean-Elements Message (OMM).

    The OMM contains the orbital characteristics of a single object at a specified epoch,
    expressed in mean Keplerian elements: mean motion, eccentricity, inclination, right
    ascension of ascending node, argument of perigee, and mean anomaly.

    These elements are adequate for providing the initial mean state of analytical and
    semi-analytical orbit models (e.g., SGP4). The OMM includes keywords and values that may
    be used to generate canonical NORAD Two Line Element (TLE) sets to accommodate the needs
    of heritage users.

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

    @staticmethod
    def from_tle_lines(
        line1,
        line2,
        object_name=None,
        object_id=None,
        originator=None,
        message_id=None,
        creation_date=None,
    ):
        """
        Build a minimal OMM from canonical NORAD TLE line 1 and line 2.

        Parameters
        ----------
        line1 : str
            TLE line 1 (69 chars including checksum).
        line2 : str
            TLE line 2 (69 chars including checksum).
        object_name : str, optional
            Metadata OBJECT_NAME override (default: "UNKNOWN").
        object_id : str, optional
            Metadata OBJECT_ID override (default: derived from TLE launch designator).
        originator : str, optional
            Header ORIGINATOR override (default: "UNKNOWN").
        message_id : str, optional
            Header MESSAGE_ID override.
        creation_date : str, optional
            Header CREATION_DATE override in CCSDS epoch format.
        """
        ...

    @property
    def header(self) -> OdmHeader:
        """
        Orbit Mean-Elements Message (OMM).

        The OMM contains the orbital characteristics of a single object at a specified epoch,
        expressed in mean Keplerian elements: mean motion, eccentricity, inclination, right
        ascension of ascending node, argument of perigee, and mean anomaly.

        These elements are adequate for providing the initial mean state of analytical and
        semi-analytical orbit models (e.g., SGP4). The OMM includes keywords and values that may
        be used to generate canonical NORAD Two Line Element (TLE) sets to accommodate the needs
        of heritage users.
        """
        ...

    @header.setter
    def header(self, value: OdmHeader) -> None: ...
    @property
    def id(self) -> Optional[str]:
        """
        The message identifier.
        """
        ...

    @property
    def segment(self) -> OmmSegment:
        """
        The data segment.
        """
        ...

    @segment.setter
    def segment(self, value: OmmSegment) -> None: ...
    def to_file(self, path, format, validate=True):
        """
        Write to file.

        Parameters
        ----------
        path : str
            Output file path.
        format : str
            Output format ('kvn' or 'xml').
        validate : bool, optional
            Whether to validate the message before writing (default: True).
        """
        ...

    def to_str(self, format, validate=True):
        """
        Serialize to string.

        Parameters
        ----------
        format : str
            Output format ('kvn' or 'xml').
            (Mandatory)
        validate : bool, optional
            Whether to validate the message before writing (default: True).

        Returns
        -------
        str
            The serialized string.
        """
        ...

    def to_tle_lines(self):
        """
        Generate canonical NORAD TLE lines (line 1 and line 2) from this OMM.

        Returns
        -------
        tuple[str, str]
            `(line1, line2)` without a line 0.
        """
        ...

    def validate(self, strict=True):
        """
        Validate the message against CCSDS rules.

        Parameters
        ----------
        strict : bool, optional
            If True (default), raises ValueError on the first error found.
            If False, returns a list of validation error messages (or None if valid).
        """
        ...

    @property
    def version(self) -> str:
        """
        The message version.
        """
        ...

    @version.setter
    def version(self, value: str) -> None: ...

class OmmData:
    """
    OMM Data section.
    """
    def __init__(mean_elements, comments=None) -> None: ...
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
    def user_defined_parameters(self) -> UserDefined | None:
        """
        User-Defined Parameters.
        """
        ...

    @user_defined_parameters.setter
    def user_defined_parameters(self, value: UserDefined | None) -> None: ...

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
        center_name=...,
        ref_frame=None,
        time_system=None,
        mean_element_theory=...,
        ref_frame_epoch=None,
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

        Examples: This is a comment
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def mean_element_theory(self) -> str:
        """
        Description of the Mean Element Theory. Indicates the proper method to employ to
        propagate the state.

        Examples: SGP, SGP4, SGP4-XP, DSST, USM
        """
        ...

    @mean_element_theory.setter
    def mean_element_theory(self, value: str) -> None: ...
    @property
    def object_id(self) -> str:
        """
        Object identifier of the object for which mean element orbit state data is provided.
        While there is no CCSDS-based restriction on the value for this keyword, it is
        recommended to use the international spacecraft designator as published in the UN Office
        of Outer Space Affairs designator index (reference `[3]`). Recommended values have the
        format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three-digit serial number of
        launch in year YYYY (with leading zeros). P{PP} = At least one capital letter for the
        identification of the part brought into space by the launch. If the asset is not listed
        in reference `[3]`, the UN Office of Outer Space Affairs designator index format is not
        used, or the content is either unknown or cannot be disclosed, the value should be set
        to UNKNOWN.

        Examples: 2005-046A, 2005-046B, 2003-022A, UNKNOWN
        """
        ...

    @object_id.setter
    def object_id(self, value: str) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Spacecraft name for which mean element orbit state data is provided. While there is no
        CCSDS-based restriction on the value for this keyword, it is recommended to use names
        from the UN Office of Outer Space Affairs designator index (reference `[3]`, which include
        Object name and international designator of the participant). If OBJECT_NAME is not
        listed in reference `[3]` or the content is either unknown or cannot be disclosed, the
        value should be set to UNKNOWN.

        Examples: Telkom 2, Spaceway 2, INMARSAT 4-F2, UNKNOWN
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def ref_frame(self) -> str:
        """
        Reference frame in which the Keplerian element data are given. Use of values other than
        those in 3.2.3.3 should be documented in an ICD. NOTE—NORAD Two Line Element Sets and
        corresponding Simplified General Perturbations (SGP) orbit propagator ephemeris outputs
        are explicitly defined to be in the True Equator Mean Equinox of Date (TEME of Date)
        reference frame. Therefore, TEME of date shall be used for OMMs based on NORAD Two Line
        Element sets, rather than the almost imperceptibly different TEME of Epoch (see
        reference `[H2]` or `[H3]` for further details).

        Examples: ICRF, ITRF2000, EME2000, TEME
        """
        ...

    @ref_frame.setter
    def ref_frame(self, value: str) -> None: ...
    @property
    def ref_frame_epoch(self) -> Optional[str]:
        """
        Epoch of reference frame, if not intrinsic to the definition of the reference frame.
        (See 7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @ref_frame_epoch.setter
    def ref_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system used for Keplerian elements and covariance data. Use of values other than
        those in 3.2.3.2 should be documented in an ICD.

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
    def id(self) -> Optional[str]:
        """
        The message identifier.
        """
        ...

    @property
    def segment(self) -> OpmSegment:
        """
        The data segment.
        """
        ...

    @segment.setter
    def segment(self, value: OpmSegment) -> None: ...
    def to_file(self, path, format, validate=True):
        """
        Write to file.

        Parameters
        ----------
        path : str
            Output file path.
        format : str
            Output format ('kvn' or 'xml').
        validate : bool, optional
            Whether to validate the message before writing (default: True).
        """
        ...

    def to_str(self, format, validate=True):
        """
        Serialize to string.

        Parameters
        ----------
        format : str
            Output format ('kvn' or 'xml').
        validate : bool, optional
            Whether to validate the message before writing (default: True).

        Returns
        -------
        str
            The serialized string.
        """
        ...

    def validate(self, strict=True):
        """
        Validate the message against CCSDS rules.

        Parameters
        ----------
        strict : bool, optional
            If True (default), raises ValueError on the first error found.
            If False, returns a list of validation error messages (or None if valid).
        """
        ...

    @property
    def version(self) -> str:
        """
        The message version.
        """
        ...

    @version.setter
    def version(self, value: str) -> None: ...

class OpmCovarianceMatrix:
    """
    Position/Velocity Covariance Matrix (6x6 Lower Triangular Form. None or all parameters of the
    matrix must be given. COV_REF_FRAME may be omitted if it is the same as REF_FRAME.)

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
        cx_x=None,
        cy_x=None,
        cy_y=None,
        cz_x=None,
        cz_y=None,
        cz_z=None,
        cx_dot_x=None,
        cx_dot_y=None,
        cx_dot_z=None,
        cy_dot_x=None,
        cy_dot_y=None,
        cy_dot_z=None,
        cz_dot_x=None,
        cz_dot_y=None,
        cz_dot_z=None,
        cx_dot_x_dot=None,
        cy_dot_x_dot=None,
        cy_dot_y_dot=None,
        cz_dot_x_dot=None,
        cz_dot_y_dot=None,
        cz_dot_z_dot=None,
        cov_ref_frame=None,
        comments=None,
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
        Covariance matrix `[4,1]`

        Units: km²/s
        """
        ...

    @cx_dot_x.setter
    def cx_dot_x(self, value: float) -> None: ...
    @property
    def cx_dot_x_dot(self) -> float:
        """
        Covariance matrix `[4,4]`

        Units: km²/s²
        """
        ...

    @cx_dot_x_dot.setter
    def cx_dot_x_dot(self, value: float) -> None: ...
    @property
    def cx_dot_y(self) -> float:
        """
        Covariance matrix `[4,2]`

        Units: km²/s
        """
        ...

    @cx_dot_y.setter
    def cx_dot_y(self, value: float) -> None: ...
    @property
    def cx_dot_z(self) -> float:
        """
        Covariance matrix `[4,3]`

        Units: km²/s
        """
        ...

    @cx_dot_z.setter
    def cx_dot_z(self, value: float) -> None: ...
    @property
    def cx_x(self) -> float:
        """
        Covariance matrix `[1,1]`

        Units: km²
        """
        ...

    @cx_x.setter
    def cx_x(self, value: float) -> None: ...
    @property
    def cy_dot_x(self) -> float:
        """
        Covariance matrix `[5,1]`

        Units: km²/s
        """
        ...

    @cy_dot_x.setter
    def cy_dot_x(self, value: float) -> None: ...
    @property
    def cy_dot_x_dot(self) -> float:
        """
        Covariance matrix `[5,4]`

        Units: km²/s²
        """
        ...

    @cy_dot_x_dot.setter
    def cy_dot_x_dot(self, value: float) -> None: ...
    @property
    def cy_dot_y(self) -> float:
        """
        Covariance matrix `[5,2]`

        Units: km²/s
        """
        ...

    @cy_dot_y.setter
    def cy_dot_y(self, value: float) -> None: ...
    @property
    def cy_dot_y_dot(self) -> float:
        """
        Covariance matrix `[5,5]`

        Units: km²/s²
        """
        ...

    @cy_dot_y_dot.setter
    def cy_dot_y_dot(self, value: float) -> None: ...
    @property
    def cy_dot_z(self) -> float:
        """
        Covariance matrix `[5,3]`

        Units: km²/s
        """
        ...

    @cy_dot_z.setter
    def cy_dot_z(self, value: float) -> None: ...
    @property
    def cy_x(self) -> float:
        """
        Covariance matrix `[2,1]`

        Units: km²
        """
        ...

    @cy_x.setter
    def cy_x(self, value: float) -> None: ...
    @property
    def cy_y(self) -> float:
        """
        Covariance matrix `[2,2]`

        Units: km²
        """
        ...

    @cy_y.setter
    def cy_y(self, value: float) -> None: ...
    @property
    def cz_dot_x(self) -> float:
        """
        Covariance matrix `[6,1]`

        Units: km²/s
        """
        ...

    @cz_dot_x.setter
    def cz_dot_x(self, value: float) -> None: ...
    @property
    def cz_dot_x_dot(self) -> float:
        """
        Covariance matrix `[6,4]`

        Units: km²/s²
        """
        ...

    @cz_dot_x_dot.setter
    def cz_dot_x_dot(self, value: float) -> None: ...
    @property
    def cz_dot_y(self) -> float:
        """
        Covariance matrix `[6,2]`

        Units: km²/s
        """
        ...

    @cz_dot_y.setter
    def cz_dot_y(self, value: float) -> None: ...
    @property
    def cz_dot_y_dot(self) -> float:
        """
        Covariance matrix `[6,5]`

        Units: km²/s²
        """
        ...

    @cz_dot_y_dot.setter
    def cz_dot_y_dot(self, value: float) -> None: ...
    @property
    def cz_dot_z(self) -> float:
        """
        Covariance matrix `[6,3]`

        Units: km²/s
        """
        ...

    @cz_dot_z.setter
    def cz_dot_z(self, value: float) -> None: ...
    @property
    def cz_dot_z_dot(self) -> float:
        """
        Covariance matrix `[6,6]`

        Units: km²/s²
        """
        ...

    @cz_dot_z_dot.setter
    def cz_dot_z_dot(self, value: float) -> None: ...
    @property
    def cz_x(self) -> float:
        """
        Covariance matrix `[3,1]`

        Units: km²
        """
        ...

    @cz_x.setter
    def cz_x(self, value: float) -> None: ...
    @property
    def cz_y(self) -> float:
        """
        Covariance matrix `[3,2]`

        Units: km²
        """
        ...

    @cz_y.setter
    def cz_y(self, value: float) -> None: ...
    @property
    def cz_z(self) -> float:
        """
        Covariance matrix `[3,3]`

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
    def __init__(state_vector, comment=None) -> None: ...
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
    def user_defined_parameters(self) -> UserDefined | None:
        """
        User defined parameters.
        """
        ...

    @user_defined_parameters.setter
    def user_defined_parameters(self, value: UserDefined | None) -> None: ...

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
        center_name=...,
        ref_frame=None,
        time_system=None,
        ref_frame_epoch=None,
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
        Origin of the OPM reference frame, which shall be a natural solar system body (planets,
        asteroids, comets, and natural satellites), including any planet barycenter or the solar
        system barycenter. Natural bodies shall be selected from the accepted set of values
        indicated in annex B, subsection B2.

        Examples: EARTH EARTH BARYCENTER MOON SOLAR SYSTEM BARYCENTER SUN JUPITER BARYCENTER
        STS 106 EROS
        """
        ...

    @center_name.setter
    def center_name(self, value: str) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (allowed at the beginning of the OPM Metadata). (See 7.8 for formatting rules.)

        Examples: This is a comment
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
        designator index (reference ``[3]``). Recommended values have the format YYYY-NNNP{PP}, where:
        YYYY = Year of launch. NNN = Three-digit serial number of launch in year YYYY (with leading
        zeros). P{PP} = At least one capital letter for the identification of the part brought into
        space by the launch. If the asset is not listed in reference ``[3]``, the UN Office of Outer
        Space Affairs designator index format is not used, or the content is either unknown or
        cannot be disclosed, the value should be set to UNKNOWN.

        Examples: 2000-052A 1996-068A 2000-053A 1996-008A UNKNOWN
        """
        ...

    @object_id.setter
    def object_id(self, value: str) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Spacecraft name for which orbit state data is provided. While there is no CCSDS-based
        restriction on the value for this keyword, it is recommended to use names from the UN
        Office of Outer Space Affairs designator index (reference ``[3]``, which include Object name
        and international designator of the participant). If OBJECT_NAME is not listed in reference
        `[3]` or the content is either unknown or cannot be disclosed, the value should be set to
        UNKNOWN.

        Examples: EUTELSAT W1 MARS PATHFINDER STS 106 NEAR UNKNOWN
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def ref_frame(self) -> str:
        """
        Reference frame in which the state vector and optional Keplerian element data are given.
        Use of values other than those in 3.2.3.3 should be documented in an ICD.

        Examples: ICRF EME2000 ITRF2000 TEME
        """
        ...

    @ref_frame.setter
    def ref_frame(self, value: str) -> None: ...
    @property
    def ref_frame_epoch(self) -> Optional[str]:
        """
        Epoch of reference frame, if not intrinsic to the definition of the reference frame. (See
        7.5.10 for formatting rules.)

        Examples: 2001-11-06T11:17:33 2002-204T15:56:23Z
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

class QuaternionState:
    """
    Attitude quaternion.

    All mandatory elements are to be provided if the block is present.
    (See annex F for conventions and further detail.)
    """
    def __init__(
        ref_frame_a,
        ref_frame_b,
        q1,
        q2,
        q3,
        qc,
        q1_dot,
        q2_dot,
        q3_dot,
        qc_dot,
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
        One or more comment line(s). Each comment line shall begin with this keyword.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def q1(self) -> float:
        """
        Quaternion components Q1, Q2, Q3, QC.

        Units: dimensionless
        """
        ...

    @q1.setter
    def q1(self, value: float) -> None: ...
    @property
    def q2(self) -> float:
        """
        Quaternion components Q1, Q2, Q3, QC.

        Units: dimensionless
        """
        ...

    @q2.setter
    def q2(self, value: float) -> None: ...
    @property
    def q3(self) -> float:
        """
        Quaternion components Q1, Q2, Q3, QC.

        Units: dimensionless
        """
        ...

    @q3.setter
    def q3(self, value: float) -> None: ...
    @property
    def qc(self) -> float:
        """
        Quaternion components Q1, Q2, Q3, QC.

        Units: dimensionless
        """
        ...

    @qc.setter
    def qc(self, value: float) -> None: ...
    @property
    def quaternion_dot(self) -> list[float] | None:
        """
        Quaternion derivative components [Q1_DOT, Q2_DOT, Q3_DOT, QC_DOT].

        Units: 1/s
        """
        ...

    @quaternion_dot.setter
    def quaternion_dot(self, value: list[float] | None) -> None: ...
    @property
    def ref_frame_a(self) -> str:
        """
        Name of the reference frame that defines the starting point of the transformation. The set
        of allowed values is described in annex B, subsection B3.
        """
        ...

    @ref_frame_a.setter
    def ref_frame_a(self, value: str) -> None: ...
    @property
    def ref_frame_b(self) -> str:
        """
        Name of the reference frame that defines the end point of the transformation. The set of
        allowed values is described in annex B, subsection B3.
        """
        ...

    @ref_frame_b.setter
    def ref_frame_b(self, value: str) -> None: ...

class Rdm:
    """
    Re-entry Data Message (RDM).

    The RDM specifies a standard message format to be used in the exchange of spacecraft
    re-entry information between Space Situational Awareness (SSA) or Space Surveillance and
    Tracking (SST) data providers, satellite owners/operators, and other parties.

    It includes data such as:
    - Remaining orbital lifetime
    - Start and end of the re-entry and impact windows
    - Impact location and probabilities
    - Object physical properties

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

        The RDM specifies a standard message format to be used in the exchange of spacecraft
        re-entry information between Space Situational Awareness (SSA) or Space Surveillance and
        Tracking (SST) data providers, satellite owners/operators, and other parties.

        It includes data such as:
        - Remaining orbital lifetime
        - Start and end of the re-entry and impact windows
        - Impact location and probabilities
        - Object physical properties
        """
        ...

    @header.setter
    def header(self, value: RdmHeader) -> None: ...
    @property
    def id(self) -> Optional[str]:
        """
        The message identifier.
        """
        ...

    @property
    def segment(self) -> RdmSegment:
        """
        The RDM Body consists of a single segment.
        """
        ...

    @segment.setter
    def segment(self, value: RdmSegment) -> None: ...
    def to_file(self, path, format, validate=True):
        """
        Write to a file.

        Parameters
        ----------
        path : str
            Output file path.
        format : str
            Format ('kvn' or 'xml').
        validate : bool, optional
            Whether to validate the message before writing (default: True).
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

    def to_str(self, format, validate=True):
        """
        Serialize to string (generic).

        Parameters
        ----------
        format : str
            Format ('kvn' or 'xml').
        validate : bool, optional
            Whether to validate the message before writing (default: True).

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

    def validate(self, strict=True):
        """
        Validate the message against CCSDS rules.

        Parameters
        ----------
        strict : bool, optional
            If True (default), raises ValueError on the first error found.
            If False, returns a list of validation error messages (or None if valid).
        """
        ...

    @property
    def version(self) -> str:
        """
        The message version.
        """
        ...

    @version.setter
    def version(self, value: str) -> None: ...

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
    user_defined_parameters : UserDefined, optional
        User defined parameters.
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
    def user_defined_parameters(self) -> UserDefined | None:
        """
        User defined parameters.
        """
        ...

    @user_defined_parameters.setter
    def user_defined_parameters(self, value: UserDefined | None) -> None: ...

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
        epoch_tzero,
        controlled_reentry=None,
        center_name=...,
        time_system=None,
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
        The atmosphere model(s) used in the simulation. If more than one model is used they
        should be listed on the same line and separated by a comma.

        Examples: MSIS, JACCHIA 70, MSISE-90, NRLMSISE-00
        """
        ...

    @atmospheric_model.setter
    def atmospheric_model(self, value: Optional[str]) -> None: ...
    @property
    def catalog_name(self) -> Optional[str]:
        """
        The satellite catalog used for the object (formatting rules specified in 5.2.3.3). The
        name should be taken from the appropriate SANA registry for catalog names, reference
        `[8]`.

        Examples: SATCAT, ESA SST
        """
        ...

    @catalog_name.setter
    def catalog_name(self, value: Optional[str]) -> None: ...
    @property
    def center_name(self) -> str:
        """
        Celestial body orbited by the object and origin of the reference frame, which may be a
        natural solar system body (planets, asteroids, comets, and natural satellites),
        including any planet barycenter or the solar system barycenter. The value should be
        taken from the orbit center column in the SANA orbit centers registry, reference `[9]`.

        Examples: EARTH, MOON, JUPITER
        """
        ...

    @center_name.setter
    def center_name(self, value: str) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (allowed only at the beginning of RDM metadata).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def controlled_reentry(self) -> str:
        """
        Specification of whether the re-entry is controlled or not.

        Examples: YES, NO, UNKNOWN
        """
        ...

    @controlled_reentry.setter
    def controlled_reentry(self, value: str) -> None: ...
    @property
    def drag_parameters_altitude(self) -> Optional[float]:
        """
        The altitude (in km) at which the object drag parameters (DRAG_AREA, DRAG_COEFF, and/or
        BALLISTIC_COEFF) are valid. The units shall be kilometers, and the conventions
        specified in 5.2.4.1 and 5.3.4 must be followed.

        Examples: 200 `[km]`, 175 `[km]`

        Units: km
        """
        ...

    @drag_parameters_altitude.setter
    def drag_parameters_altitude(self, value: Optional[float]) -> None: ...
    @property
    def drag_parameters_source(self) -> Optional[str]:
        """
        The method used to estimate the drag parameters of the object (DRAG_AREA, DRAG_COEFF,
        and/or BALLISTIC_COEFF).

        Examples: DESIGN, CFD: TOOL1, CFD DMSCFOAM, OD
        """
        ...

    @drag_parameters_source.setter
    def drag_parameters_source(self, value: Optional[str]) -> None: ...
    @property
    def earth_tides(self) -> Optional[str]:
        """
        Model used for solid Earth and ocean tides: either model name, or NO if tides were not
        modelled.

        Examples: ESR, NO
        """
        ...

    @earth_tides.setter
    def earth_tides(self, value: Optional[str]) -> None: ...
    @property
    def ephemeris_name(self) -> Optional[str]:
        """
        Unique identifier of an external ephemeris file used or NONE.

        Examples: NONE, EPHEMERIS, INTELSAT2
        """
        ...

    @ephemeris_name.setter
    def ephemeris_name(self, value: Optional[str]) -> None: ...
    @property
    def epoch_tzero(self) -> str:
        """
        Epoch from which the ORBIT_LIFETIME is calculated (formatting rules specified in
        5.3.3.5).

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23
        """
        ...

    @epoch_tzero.setter
    def epoch_tzero(self, value: str) -> None: ...
    @property
    def gravity_model(self) -> Optional[str]:
        """
        The gravity model used in the simulation. The degree (D) and order (O) of the spherical
        harmonic coefficients applied should be given along with the name of the model.

        Examples: EGM-96: 36D 36O, JGM-2: 41D 41O
        """
        ...

    @gravity_model.setter
    def gravity_model(self, value: Optional[str]) -> None: ...
    @property
    def impact_uncertainty_method(self) -> Optional[str]:
        """
        The method used to determine the impact location confidence interval(s).

        Examples: NONE, ANALYTICAL, STOCHASTIC, EMPIRICAL
        """
        ...

    @impact_uncertainty_method.setter
    def impact_uncertainty_method(self, value: Optional[str]) -> None: ...
    @property
    def international_designator(self) -> str:
        """
        The full international designator (COSPAR ID) for the object. Values shall have the
        format YYYY-NNNP{PP}, where: YYYY = year of launch; NNN = three-digit serial number of
        launch (with leading zeros); P{PP} = at least one capital letter for the identification
        of the part brought into space by the launch. In cases where the object has no
        international designator, the value UNKNOWN should be used (formatting rules specified
        in 5.2.3.3).

        Examples: 2010-012C, 2016-001A, 1985-067CD, UNKNOWN
        """
        ...

    @international_designator.setter
    def international_designator(self, value: str) -> None: ...
    @property
    def intrack_thrust(self) -> Optional[str]:
        """
        Indicator on whether in-track thrust modeling was used in the simulation.

        Examples: YES, NO
        """
        ...

    @intrack_thrust.setter
    def intrack_thrust(self, value: Optional[str]) -> None: ...
    @property
    def n_body_perturbations(self) -> Optional[str]:
        """
        Comma separated list of other bodies used in the simulation. The names of the bodies
        should be taken from the SANA registry for orbit centers, reference `[9]`. If no other
        bodies are used in the simulation, the value should be NONE.

        Examples: MOON, SUN, JUPITER, NONE
        """
        ...

    @n_body_perturbations.setter
    def n_body_perturbations(self, value: Optional[str]) -> None: ...
    @property
    def next_message_epoch(self) -> Optional[str]:
        """
        Scheduled UTC epoch of the next RDM for the same object (formatting rules specified in
        5.3.3.5); N/A if no other message is scheduled.

        Examples: 2001-11-06T11:17:33, N/A
        """
        ...

    @next_message_epoch.setter
    def next_message_epoch(self, value: Optional[str]) -> None: ...
    @property
    def object_designator(self) -> Optional[str]:
        """
        The CATALOG_NAME satellite catalog designator for the object (formatting rules
        specified in 5.2.3.3).

        Examples: 37451, 125387U
        """
        ...

    @object_designator.setter
    def object_designator(self, value: Optional[str]) -> None: ...
    @property
    def object_name(self) -> str:
        """
        Object name for which the orbit state is provided. There is no CCSDS-based restriction
        on the value for this keyword, but it is recommended to use names from the UNOOSA
        registry—reference `[7]`, which includes object name and international designator of the
        participant (formatting rules specified in 5.2.3.3). For objects that are not in the
        UNOOSA registry, either a descriptive name (e.g., DEBRIS, if the object is identified as
        space debris) or UNKNOWN should be used.

        Examples: SENTINEL-1A, GOCE, ENVISAT, BRIZ R/B, DEBRIS, UNKNOWN
        """
        ...

    @object_name.setter
    def object_name(self, value: str) -> None: ...
    @property
    def object_operator(self) -> Optional[str]:
        """
        Operator of the object (e.g., company, agency, or country operating the satellite).
        The value should be taken from the abbreviation column in the SANA organizations
        registry, reference `[6]`.

        Examples: ESA, EUMETSAT
        """
        ...

    @object_operator.setter
    def object_operator(self, value: Optional[str]) -> None: ...
    @property
    def object_owner(self) -> Optional[str]:
        """
        Owner of the object (e.g., company, agency, or country owning the satellite). The value
        should be taken from the abbreviation column in the SANA organizations registry,
        reference `[6]`.

        Examples: DLR, INTELSAT, ESA, UNKNOWN
        """
        ...

    @object_owner.setter
    def object_owner(self, value: Optional[str]) -> None: ...
    @property
    def object_type(self) -> Optional[str]:
        """
        The object type.

        Examples: PAYLOAD, ROCKET BODY, DEBRIS, OTHER, UNKNOWN
        """
        ...

    @object_type.setter
    def object_type(self, value: Optional[str]) -> None: ...
    @property
    def previous_message_epoch(self) -> Optional[str]:
        """
        UTC Epoch of the previous RDM issued for this object (formatting rules specified in
        5.3.3.5).

        Examples: 2001-11-06T11:17:33
        """
        ...

    @previous_message_epoch.setter
    def previous_message_epoch(self, value: Optional[str]) -> None: ...
    @property
    def previous_message_id(self) -> Optional[str]:
        """
        ID of the previous RDM issued for this object.

        Examples: ESA/2015-563892348
        """
        ...

    @previous_message_id.setter
    def previous_message_id(self, value: Optional[str]) -> None: ...
    @property
    def reentry_disintegration(self) -> Optional[str]:
        """
        The aspects of disintegration during re-entry considered during simulations: none (the
        object was treated as a point mass), mass loss, break-ups (including explosion), or
        both. It is a coarse indication on whether the impact area in the data covers potential
        fragments as well.

        Examples: NONE, MASS-LOSS, BREAK-UP, MASS-LOSS + BREAK-UP
        """
        ...

    @reentry_disintegration.setter
    def reentry_disintegration(self, value: Optional[str]) -> None: ...
    @property
    def reentry_uncertainty_method(self) -> Optional[str]:
        """
        The method used to determine the orbit lifetime uncertainty or the re-entry windows.

        Examples: NONE, ANALYTICAL, STOCHASTIC, EMPIRICAL
        """
        ...

    @reentry_uncertainty_method.setter
    def reentry_uncertainty_method(self, value: Optional[str]) -> None: ...
    @property
    def ref_frame(self) -> Optional[str]:
        """
        Reference frame in which the (optional) orbit information will be provided. The value
        should be taken from the keyword value name column in the SANA celestial body reference
        frames registry, reference `[11]`. The reference frame must be the same for all orbit
        data elements, with the exception of the covariance matrix, for which a different
        reference frame may be specified, and the ground impact data. This keyword becomes
        mandatory if state vectors are provided in the data section.

        Examples: ITRF-97, EME2000, ICRF
        """
        ...

    @ref_frame.setter
    def ref_frame(self, value: Optional[str]) -> None: ...
    @property
    def ref_frame_epoch(self) -> Optional[str]:
        """
        Epoch of reference frame, if not intrinsic to the definition of the reference frame
        (formatting rules specified in 5.3.3.5).

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
        """
        ...

    @ref_frame_epoch.setter
    def ref_frame_epoch(self, value: Optional[str]) -> None: ...
    @property
    def solar_flux_prediction(self) -> Optional[str]:
        """
        The method used to predict the solar flux and geomagnetic indices.

        Examples: STOCHASTIC, PREDICTED: MLLRT
        """
        ...

    @solar_flux_prediction.setter
    def solar_flux_prediction(self, value: Optional[str]) -> None: ...
    @property
    def solar_rad_pressure(self) -> Optional[str]:
        """
        Model used for the solar radiation pressure: either model name, or NO if solar
        radiation pressure was not modelled.

        Examples: GSPM04, NO
        """
        ...

    @solar_rad_pressure.setter
    def solar_rad_pressure(self, value: Optional[str]) -> None: ...
    @property
    def time_system(self) -> str:
        """
        Time system for all data/metadata. The value should be taken from the name column in
        the SANA time systems registry, reference `[10]`.

        Examples: UTC, TAI
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
        Object ballistic coefficient.

        Units: kg/m²
        """
        ...

    @ballistic_coeff.setter
    def ballistic_coeff(self, value: Optional[float]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (allowed only at the beginning of each RDM data logical block).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def drag_area(self) -> Optional[float]:
        """
        Object cross-sectional area.

        Units: m²
        """
        ...

    @drag_area.setter
    def drag_area(self, value: Optional[float]) -> None: ...
    @property
    def drag_coeff(self) -> Optional[float]:
        """
        Object drag coefficient.
        """
        ...

    @drag_coeff.setter
    def drag_coeff(self, value: Optional[float]) -> None: ...
    @property
    def dry_mass(self) -> Optional[float]:
        """
        Object dry mass (without propellant).

        Units: kg
        """
        ...

    @dry_mass.setter
    def dry_mass(self, value: Optional[float]) -> None: ...
    @property
    def hazardous_substances(self) -> Optional[str]:
        """
        Comma separated list of hazardous substances contained by the object.
        """
        ...

    @hazardous_substances.setter
    def hazardous_substances(self, value: Optional[str]) -> None: ...
    @property
    def rcs(self) -> Optional[float]:
        """
        Object radar cross section.

        Units: m²
        """
        ...

    @rcs.setter
    def rcs(self, value: Optional[float]) -> None: ...
    @property
    def solar_rad_area(self) -> Optional[float]:
        """
        Object area exposed to Solar Radiation Pressure (SRP).

        Units: m²
        """
        ...

    @solar_rad_area.setter
    def solar_rad_area(self, value: Optional[float]) -> None: ...
    @property
    def solar_rad_coeff(self) -> Optional[float]:
        """
        Object solar radiation coefficient.
        """
        ...

    @solar_rad_coeff.setter
    def solar_rad_coeff(self, value: Optional[float]) -> None: ...
    @property
    def thrust_acceleration(self) -> Optional[float]:
        """
        The object’s acceleration due to in-track thrust used to propagate the state vector and
        covariance to NOMINAL_RENTRY_EPOCH (if a controlled re-entry).

        Units: m/s²
        """
        ...

    @thrust_acceleration.setter
    def thrust_acceleration(self, value: Optional[float]) -> None: ...
    @property
    def wet_mass(self) -> Optional[float]:
        """
        Total object mass at EPOCH_TZERO.

        Units: kg
        """
        ...

    @wet_mass.setter
    def wet_mass(self, value: Optional[float]) -> None: ...

class ReferenceFrame:
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

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
    screen_volume_frame : Union[ScreenVolumeFrameType, str], optional
        The reference frame for screening volume (RTN or TVN).
    screen_volume_shape : Union[ScreenVolumeShapeType, str], optional
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
        The probability (denoted 'p' where 0.0<=p<=1.0), that Object1 and Object2 will collide.
        Data type = double.
        """
        ...

    @collision_probability.setter
    def collision_probability(self, value: Optional[float]) -> None: ...
    @property
    def collision_probability_method(self) -> Optional[str]:
        """
        The method that was used to calculate the collision probability. (See annex E for
        definition.)
        """
        ...

    @collision_probability_method.setter
    def collision_probability_method(self, value: Optional[str]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 6.3.4 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def miss_distance(self) -> float:
        """
        The norm of the relative position vector. It indicates how close the two objects are at
        TCA. Data type = double.

        Units: m
        """
        ...

    @miss_distance.setter
    def miss_distance(self, value: float) -> None: ...
    @property
    def relative_speed(self) -> Optional[float]:
        """
        The norm of the relative velocity vector. It indicates how fast the two objects are
        moving relative to each other at TCA. Data type = double.

        Units: m/s
        """
        ...

    @relative_speed.setter
    def relative_speed(self, value: Optional[float]) -> None: ...
    @property
    def relative_state_vector(self) -> Optional[RelativeStateVector]:
        """
        Relative state vector [R, T, N, VR, VT, VN] (combined position and velocity).
        """
        ...

    @relative_state_vector.setter
    def relative_state_vector(self, value: Optional[RelativeStateVector]) -> None: ...
    @property
    def screen_entry_time(self) -> Optional[str]:
        """
        The time in UTC when Object2 enters the screening volume. (See 6.3.2.6 for formatting
        rules.)
        """
        ...

    @screen_entry_time.setter
    def screen_entry_time(self, value: Optional[str]) -> None: ...
    @property
    def screen_exit_time(self) -> Optional[str]:
        """
        The time in UTC when Object2 exits the screening volume. (See 6.3.2.6 for formatting
        rules.)
        """
        ...

    @screen_exit_time.setter
    def screen_exit_time(self, value: Optional[str]) -> None: ...
    @property
    def screen_volume_frame(self) -> Optional[ScreenVolumeFrameType]:
        """
        Name of the Object1 centered reference frame in which the screening volume data are
        given. Available options are RTN and Transverse, Velocity, and Normal (TVN). (See annex
        E for definition.)
        """
        ...

    @screen_volume_frame.setter
    def screen_volume_frame(self, value: Optional[ScreenVolumeFrameType]) -> None: ...
    @property
    def screen_volume_shape(self) -> Optional[ScreenVolumeShapeType]:
        """
        Shape of the screening volume: ELLIPSOID or BOX.
        """
        ...

    @screen_volume_shape.setter
    def screen_volume_shape(self, value: Optional[ScreenVolumeShapeType]) -> None: ...
    @property
    def screen_volume_x(self) -> Optional[float]:
        """
        The R or T (depending on if RTN or TVN is selected) component size of the screening
        volume in the SCREEN_VOLUME_FRAME. Data type = double.

        Units: m
        """
        ...

    @screen_volume_x.setter
    def screen_volume_x(self, value: Optional[float]) -> None: ...
    @property
    def screen_volume_y(self) -> Optional[float]:
        """
        The T or V (depending on if RTN or TVN is selected) component size of the screening
        volume in the SCREEN_VOLUME_FRAME. Data type = double.

        Units: m
        """
        ...

    @screen_volume_y.setter
    def screen_volume_y(self, value: Optional[float]) -> None: ...
    @property
    def screen_volume_z(self) -> Optional[float]:
        """
        The N component size of the screening volume in the SCREEN_VOLUME_FRAME. Data type =
        double.

        Units: m
        """
        ...

    @screen_volume_z.setter
    def screen_volume_z(self, value: Optional[float]) -> None: ...
    @property
    def start_screen_period(self) -> Optional[str]:
        """
        The start time in UTC of the screening period for the conjunction assessment. (See
        6.3.2.6 for formatting rules.)
        """
        ...

    @start_screen_period.setter
    def start_screen_period(self, value: Optional[str]) -> None: ...
    @property
    def stop_screen_period(self) -> Optional[str]:
        """
        The stop time in UTC of the screening period for the conjunction assessment. (See
        6.3.2.6 for formatting rules.)
        """
        ...

    @stop_screen_period.setter
    def stop_screen_period(self, value: Optional[str]) -> None: ...
    @property
    def tca(self) -> str:
        """
        The date and time in UTC of the closest approach. (See 6.3.2.6 for formatting rules.)
        """
        ...

    @tca.setter
    def tca(self, value: str) -> None: ...

class RelativeStateVector:
    """
    Relative State Vector containing relative position and velocity.

    Parameters
    ----------
    relative_position_r : float
        Relative position R component. Units: m.
    relative_position_t : float
        Relative position T component. Units: m.
    relative_position_n : float
        Relative position N component. Units: m.
    relative_velocity_r : float
        Relative velocity R component. Units: m/s.
    relative_velocity_t : float
        Relative velocity T component. Units: m/s.
    relative_velocity_n : float
        Relative velocity N component. Units: m/s.
    """
    def __init__(
        relative_position_r,
        relative_position_t,
        relative_position_n,
        relative_velocity_r,
        relative_velocity_t,
        relative_velocity_n,
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def relative_position_n(self) -> float:
        """
        Relative position N component.

        Units: m
        """
        ...

    @relative_position_n.setter
    def relative_position_n(self, value: float) -> None: ...
    @property
    def relative_position_r(self) -> float:
        """
        Relative position R component.

        Units: m
        """
        ...

    @relative_position_r.setter
    def relative_position_r(self, value: float) -> None: ...
    @property
    def relative_position_t(self) -> float:
        """
        Relative position T component.

        Units: m
        """
        ...

    @relative_position_t.setter
    def relative_position_t(self, value: float) -> None: ...
    @property
    def relative_velocity_n(self) -> float:
        """
        Relative velocity N component.

        Units: m/s
        """
        ...

    @relative_velocity_n.setter
    def relative_velocity_n(self, value: float) -> None: ...
    @property
    def relative_velocity_r(self) -> float:
        """
        Relative velocity R component.

        Units: m/s
        """
        ...

    @relative_velocity_r.setter
    def relative_velocity_r(self, value: float) -> None: ...
    @property
    def relative_velocity_t(self) -> float:
        """
        Relative velocity T component.

        Units: m/s
        """
        ...

    @relative_velocity_t.setter
    def relative_velocity_t(self, value: float) -> None: ...
    def to_numpy(self) -> numpy.ndarray:
        """
        Return the relative state vector as a NumPy array.

        Returns:
            numpy.ndarray: 1D array of shape (6,) containing [R, T, N, VR, VT, VN].
            Units: [m, m, m, m/s, m/s, m/s]
        """
        ...

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
    Spacecraft Parameters (if maneuver is specified, then mass must be provided).

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
        mass=None,
        solar_rad_area=None,
        solar_rad_coeff=None,
        drag_area=None,
        drag_coeff=None,
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

        Units: n/a
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

        Units: n/a
        """
        ...

    @solar_rad_coeff.setter
    def solar_rad_coeff(self, value: Optional[float]) -> None: ...

class SpinState:
    """
    Spin block.

    All mandatory elements are to be provided if the block is present.
    (See annex F for conventions and further detail.)
    """
    def __init__(
        ref_frame_a,
        ref_frame_b,
        spin_alpha,
        spin_delta,
        spin_angle,
        spin_angle_vel,
        nutation,
        nutation_per,
        nutation_phase,
        momentum_alpha,
        momentum_delta,
        nutation_vel,
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
        One or more comment line(s). Each comment line shall begin with this keyword.
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def momentum_alpha(self) -> Optional[float]:
        """
        Right ascension of angular momentum vector in frame A.

        Units: deg
        """
        ...

    @momentum_alpha.setter
    def momentum_alpha(self, value: Optional[float]) -> None: ...
    @property
    def momentum_delta(self) -> Optional[float]:
        """
        Declination of angular momentum vector in frame A.

        Units: deg
        """
        ...

    @momentum_delta.setter
    def momentum_delta(self, value: Optional[float]) -> None: ...
    @property
    def nutation(self) -> Optional[float]:
        """
        Nutation angle of spin axis.

        Units: deg
        """
        ...

    @nutation.setter
    def nutation(self, value: Optional[float]) -> None: ...
    @property
    def nutation_per(self) -> Optional[float]:
        """
        Body nutation period of the spin axis.

        Units: s
        """
        ...

    @nutation_per.setter
    def nutation_per(self, value: Optional[float]) -> None: ...
    @property
    def nutation_phase(self) -> Optional[float]:
        """
        Inertial nutation phase.

        Units: deg
        """
        ...

    @nutation_phase.setter
    def nutation_phase(self, value: Optional[float]) -> None: ...
    @property
    def nutation_vel(self) -> Optional[float]:
        """
        Angular velocity of spin vector around the angular momentum vector.

        Units: deg/s
        """
        ...

    @nutation_vel.setter
    def nutation_vel(self, value: Optional[float]) -> None: ...
    @property
    def ref_frame_a(self) -> str:
        """
        Name of the reference frame that defines the starting point of the transformation. The set
        of allowed values is described in annex B, subsection B3.
        """
        ...

    @ref_frame_a.setter
    def ref_frame_a(self, value: str) -> None: ...
    @property
    def ref_frame_b(self) -> str:
        """
        Name of the reference frame that defines the end point of the transformation. The set of
        allowed values is described in annex B, subsection B3.
        """
        ...

    @ref_frame_b.setter
    def ref_frame_b(self, value: str) -> None: ...
    @property
    def spin_alpha(self) -> float:
        """
        Right ascension of spin axis vector in frame A.

        Units: deg
        """
        ...

    @spin_alpha.setter
    def spin_alpha(self, value: float) -> None: ...
    @property
    def spin_angle(self) -> float:
        """
        Phase of the satellite about the spin axis.

        Units: deg
        """
        ...

    @spin_angle.setter
    def spin_angle(self, value: float) -> None: ...
    @property
    def spin_angle_vel(self) -> float:
        """
        Angular velocity of satellite around spin axis.

        Units: deg/s
        """
        ...

    @spin_angle_vel.setter
    def spin_angle_vel(self, value: float) -> None: ...
    @property
    def spin_delta(self) -> float:
        """
        Declination of the spin axis vector in frame A.

        Units: deg
        """
        ...

    @spin_delta.setter
    def spin_delta(self, value: float) -> None: ...

class StateVector:
    """
    State Vector Components in the Specified Coordinate System.

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
    def __init__(epoch, x, y, z, x_dot, y_dot, z_dot, comments=None) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def comment(self) -> list[str]:
        """
        Comments (allowed at the beginning of the OPM Metadata). (See 7.8 for formatting rules.)
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def epoch(self) -> str:
        """
        Epoch of state vector & optional Keplerian elements (see 7.5.10 for formatting rules).
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def x(self) -> float:
        """
        Position vector X-component.

        Units: km
        """
        ...

    @x.setter
    def x(self, value: float) -> None: ...
    @property
    def x_dot(self) -> float:
        """
        Velocity vector X-component.

        Units: km/s
        """
        ...

    @x_dot.setter
    def x_dot(self, value: float) -> None: ...
    @property
    def y(self) -> float:
        """
        Position vector Y-component.

        Units: km
        """
        ...

    @y.setter
    def y(self, value: float) -> None: ...
    @property
    def y_dot(self) -> float:
        """
        Velocity vector Y-component.

        Units: km/s
        """
        ...

    @y_dot.setter
    def y_dot(self, value: float) -> None: ...
    @property
    def z(self) -> float:
        """
        Position vector Z-component.

        Units: km
        """
        ...

    @z.setter
    def z(self, value: float) -> None: ...
    @property
    def z_dot(self) -> float:
        """
        Velocity vector Z-component.

        Units: km/s
        """
        ...

    @z_dot.setter
    def z_dot(self, value: float) -> None: ...

class StateVectorAcc:
    """
    State Vector Components in the Specified Coordinate System.

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
        epoch, x, y, z, x_dot, y_dot, z_dot, x_ddot=None, y_ddot=None, z_ddot=None
    ) -> None: ...
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    @property
    def epoch(self) -> str:
        """
        Epoch of state vector & optional Keplerian elements (see 7.5.10 for formatting rules).
        """
        ...

    @epoch.setter
    def epoch(self, value: str) -> None: ...
    @property
    def x(self) -> float:
        """
        Position vector X-component.

        Units: km
        """
        ...

    @x.setter
    def x(self, value: float) -> None: ...
    @property
    def x_ddot(self) -> Optional[float]:
        """
        Acceleration vector X-component.

        Units: km/s²
        """
        ...

    @x_ddot.setter
    def x_ddot(self, value: Optional[float]) -> None: ...
    @property
    def x_dot(self) -> float:
        """
        Velocity vector X-component.

        Units: km/s
        """
        ...

    @x_dot.setter
    def x_dot(self, value: float) -> None: ...
    @property
    def y(self) -> float:
        """
        Position vector Y-component.

        Units: km
        """
        ...

    @y.setter
    def y(self, value: float) -> None: ...
    @property
    def y_ddot(self) -> Optional[float]:
        """
        Acceleration vector Y-component.

        Units: km/s²
        """
        ...

    @y_ddot.setter
    def y_ddot(self, value: Optional[float]) -> None: ...
    @property
    def y_dot(self) -> float:
        """
        Velocity vector Y-component.

        Units: km/s
        """
        ...

    @y_dot.setter
    def y_dot(self, value: float) -> None: ...
    @property
    def z(self) -> float:
        """
        Position vector Z-component.

        Units: km
        """
        ...

    @z.setter
    def z(self, value: float) -> None: ...
    @property
    def z_ddot(self) -> Optional[float]:
        """
        Acceleration vector Z-component.

        Units: km/s²
        """
        ...

    @z_ddot.setter
    def z_ddot(self, value: Optional[float]) -> None: ...
    @property
    def z_dot(self) -> float:
        """
        Velocity vector Z-component.

        Units: km/s
        """
        ...

    @z_dot.setter
    def z_dot(self, value: float) -> None: ...

class Tdm:
    """
    Tracking Data Message (TDM).

    The TDM specifies a standard message format for use in exchanging spacecraft tracking data
    between space agencies. Such exchanges are used for distributing tracking data output from
    routine interagency cross-supports.

    Tracking data includes data types such as:
    - Doppler
    - Transmit/Received frequencies
    - Range
    - Angles
    - Delta-DOR
    - Media correction (ionosphere, troposphere)
    - Meteorological data

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

        The TDM specifies a standard message format for use in exchanging spacecraft tracking data
        between space agencies. Such exchanges are used for distributing tracking data output from
        routine interagency cross-supports.

        Tracking data includes data types such as:
        - Doppler
        - Transmit/Received frequencies
        - Range
        - Angles
        - Delta-DOR
        - Media correction (ionosphere, troposphere)
        - Meteorological data
        """
        ...

    @header.setter
    def header(self, value: TdmHeader) -> None: ...
    @property
    def id(self) -> Optional[str]:
        """
        The message identifier.
        """
        ...

    @property
    def segments(self) -> list[TdmSegment]:
        """
        Shortcut to access segments directly from the body.
        """
        ...

    def to_file(self, path, format, validate=True):
        """
        Write to file.

        Parameters
        ----------
        path : str
            Output file path.
        format : str
            Output format ('kvn' or 'xml').
        validate : bool, optional
            Whether to validate the message before writing (default: True).
        """
        ...

    def to_str(self, format, validate=True):
        """
        Serialize to string.

        Parameters
        ----------
        format : str
            Output format ('kvn' or 'xml').
        validate : bool, optional
            Whether to validate the message before writing (default: True).

        Returns
        -------
        str
            The serialized string.
        """
        ...

    def validate(self, strict=True):
        """
        Validate the message against CCSDS rules.

        Parameters
        ----------
        strict : bool, optional
            If True (default), raises ValueError on the first error found.
            If False, returns a list of validation error messages (or None if valid).
        """
        ...

    @property
    def version(self) -> str:
        """
        The message version.
        """
        ...

    @version.setter
    def version(self, value: str) -> None: ...

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
    def __init__(observations=None, comment=None) -> None: ...
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
    Represents the `tdmHeader` complex type.

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
        Comments (allowed in the TDM Header only immediately after the TDM version number).
        (See 4.5 for formatting rules.)

        Examples: This is a comment
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def creation_date(self) -> str:
        """
        Data creation date/time in UTC. (For format specification, see 4.3.9.)

        Examples: 2001-11-06T11:17:33, 2002-204T15:56:23.4, 2006-001T00:00:00Z
        """
        ...

    @creation_date.setter
    def creation_date(self, value: str) -> None: ...
    @property
    def message_id(self) -> Optional[str]:
        """
        ID that uniquely identifies a message from a given originator. The format and content
        of the message identifier value are at the discretion of the originator.

        Examples: 201113719185
        """
        ...

    @message_id.setter
    def message_id(self, value: Optional[str]) -> None: ...
    @property
    def originator(self) -> str:
        """
        Creating agency. Value should be an entry from the ‘Abbreviation’ column in the SANA
        Organizations Registry, <https://sanaregistry.org/r/organizations/organizations.html>
        (reference `[11]`).

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
        participant_1,
        time_system=None,
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
        The ANGLE_TYPE keyword shall indicate the type of antenna geometry represented in the
        angle data (ANGLE_1 and ANGLE_2 keywords).

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
        The set of CORRECTION_* keywords may be used to reflect the values of corrections that
        have been added to the data or should be added to the data (e.g., ranging station delay
        calibration, etc.).

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
        This keyword is used to indicate whether or not the values associated with the
        CORRECTION_* keywords have been applied to the tracking data. Required if any of the
        CORRECTION_* keywords is used.

        Examples: YES, NO
        """
        ...

    @corrections_applied.setter
    def corrections_applied(self, value: Optional[str]) -> None: ...
    @property
    def data_quality(self) -> Optional[str]:
        """
        Provides an estimate of the quality of the data, based on indicators from the producers
        of the data (e.g., bad time synchronization flags, marginal lock status indicators,
        etc.). The default value shall be ‘RAW’.

        Examples: RAW, VALIDATED, DEGRADED
        """
        ...

    @data_quality.setter
    def data_quality(self, value: Optional[str]) -> None: ...
    @property
    def data_types(self) -> Optional[str]:
        """
        Comma-separated list of data types in the Data Section. The elements of the list shall
        be selected from the data types shown in table 3-5, with the exception of the
        DATA_START, DATA_STOP, and COMMENT keywords.

        Examples: RANGE, TRANSMIT_FREQ_n, RECEIVE_FREQ
        """
        ...

    @data_types.setter
    def data_types(self, value: Optional[str]) -> None: ...
    @property
    def doppler_count_bias(self) -> Optional[float]:
        """
        Doppler counts are generally biased so as to accommodate negative Doppler within an
        accumulator. In order to reconstruct the measurement, the bias shall be subtracted from
        the DOPPLER_COUNT data value.

        Examples: 2.4e6, 240000000.0

        Units: Hz
        """
        ...

    @doppler_count_bias.setter
    def doppler_count_bias(self, value: Optional[float]) -> None: ...
    @property
    def doppler_count_rollover(self) -> Optional[str]:
        """
        Doppler counts may overflow the accumulator and roll over in cases where the track is
        of long duration or very high Doppler shift. This flag indicates whether or not a
        counter rollover has occurred during the track.

        Examples: YES, NO
        """
        ...

    @doppler_count_rollover.setter
    def doppler_count_rollover(self, value: Optional[str]) -> None: ...
    @property
    def doppler_count_scale(self) -> Optional[int]:
        """
        Doppler counts are generally scaled so as to capture partial cycles in an integer
        count. In order to reconstruct the measurement, the DOPPLER_COUNT data value shall be
        divided by the scale factor. The default shall be 1.

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
        The FREQ_OFFSET keyword represents a frequency in Hz that must be added to every
        RECEIVE_FREQ to reconstruct it. One use is if a Doppler shift frequency observable is
        transferred instead of the actual received frequency. The default shall be 0.0.

        Examples: 0.0, 8415000000.0

        Units: Hz
        """
        ...

    @freq_offset.setter
    def freq_offset(self, value: Optional[float]) -> None: ...
    @property
    def integration_interval(self) -> Optional[float]:
        """
        The INTEGRATION_INTERVAL keyword shall provide the Doppler count time in seconds for
        Doppler data or for the creation of normal points.

        Examples: 60.0, 0.1, 1.0

        Units: s
        """
        ...

    @integration_interval.setter
    def integration_interval(self, value: Optional[float]) -> None: ...
    @property
    def integration_ref(self) -> Optional[str]:
        """
        Indicates the relationship between the INTEGRATION_INTERVAL and the timetag on the
        data, i.e., whether the timetag represents the start, middle, or end of the integration
        period.

        Examples: START, MIDDLE, END
        """
        ...

    @integration_ref.setter
    def integration_ref(self, value: Optional[str]) -> None: ...
    @property
    def interpolation(self) -> Optional[str]:
        """
        The INTERPOLATION keyword shall specify the interpolation method to be used to calculate
        a transmit phase count at an arbitrary time in tracking data where the uplink frequency
        is not constant.

        Examples: HERMITE, LAGRANGE, LINEAR
        """
        ...

    @interpolation.setter
    def interpolation(self, value: Optional[str]) -> None: ...
    @property
    def interpolation_degree(self) -> Optional[int]:
        """
        The INTERPOLATION_DEGREE keyword shall specify the recommended degree of the
        interpolating polynomial used to calculate a transmit phase count at an arbitrary time
        in tracking data where the uplink frequency is not constant.

        Examples: 3, 5, 7, 11
        """
        ...

    @interpolation_degree.setter
    def interpolation_degree(self, value: Optional[int]) -> None: ...
    @property
    def mode(self) -> Optional[str]:
        """
        The MODE keyword shall reflect the tracking mode associated with the Data Section of
        the segment. The value ‘SEQUENTIAL’ applies for most sequential signal paths; the name
        implies a sequential signal path between tracking participants. The value
        ‘SINGLE_DIFF’ applies only for differenced data.

        Examples: SEQUENTIAL, SINGLE_DIFF
        """
        ...

    @mode.setter
    def mode(self, value: Optional[str]) -> None: ...
    @property
    def participant_1(self) -> str:
        """
        The PARTICIPANT_n keyword shall represent the participants (see 1.3.4.1) in a tracking
        data session. It is indexed to allow unambiguous reference to other data in the TDM
        (max index is 5). At least two participants must be specified for most sessions; for
        some special TDMs such as tropospheric media, only one participant need be listed.

        Examples: DSS-63-S400K, ROSETTA, `<Quasar catalog name>`, 1997-061A, UNKNOWN
        """
        ...

    @participant_1.setter
    def participant_1(self, value: str) -> None: ...
    @property
    def participant_2(self) -> Optional[str]:
        """
        The second participant in a tracking data session.
        """
        ...

    @participant_2.setter
    def participant_2(self, value: Optional[str]) -> None: ...
    @property
    def participant_3(self) -> Optional[str]:
        """
        The third participant in a tracking data session.
        """
        ...

    @participant_3.setter
    def participant_3(self, value: Optional[str]) -> None: ...
    @property
    def participant_4(self) -> Optional[str]:
        """
        The fourth participant in a tracking data session.
        """
        ...

    @participant_4.setter
    def participant_4(self, value: Optional[str]) -> None: ...
    @property
    def participant_5(self) -> Optional[str]:
        """
        The fifth participant in a tracking data session.
        """
        ...

    @participant_5.setter
    def participant_5(self, value: Optional[str]) -> None: ...
    @property
    def path(self) -> Optional[str]:
        """
        The PATH keywords shall reflect the signal path by listing the index of each participant
        in order, separated by commas, with no inserted white space. Correlated with the
        indices of the PARTICIPANT_n keywords. The first entry in the PATH shall be the
        transmit participant.

        Examples: PATH = 1,2,1, PATH_1 = 1,2,1, PATH_2 = 3,1
        """
        ...

    @path.setter
    def path(self, value: Optional[str]) -> None: ...
    @property
    def path_1(self) -> Optional[str]:
        """
        The first signal path where the MODE is 'SINGLE_DIFF'.
        """
        ...

    @path_1.setter
    def path_1(self, value: Optional[str]) -> None: ...
    @property
    def path_2(self) -> Optional[str]:
        """
        The second signal path where the MODE is 'SINGLE_DIFF'.
        """
        ...

    @path_2.setter
    def path_2(self, value: Optional[str]) -> None: ...
    @property
    def range_mode(self) -> Optional[str]:
        """
        The value of the RANGE_MODE keyword shall be ‘COHERENT’, in which case the range tones
        are coherent with the uplink carrier; ‘CONSTANT’, in which case the range tones have a
        constant frequency; or ‘ONE_WAY’ (used in Delta-DOR).

        Examples: COHERENT, CONSTANT, ONE_WAY
        """
        ...

    @range_mode.setter
    def range_mode(self, value: Optional[str]) -> None: ...
    @property
    def range_modulus(self) -> Optional[float]:
        """
        The value associated with the RANGE_MODULUS keyword shall be the modulus of the range
        observable in the units as specified by the RANGE_UNITS keyword; that is, the actual
        (unambiguous) range is an integer k times the modulus, plus the observable value. The
        default value shall be 0.0.

        Examples: 32768.0, 2.0e+23, 0.0, 161.6484
        """
        ...

    @range_modulus.setter
    def range_modulus(self, value: Optional[float]) -> None: ...
    @property
    def range_units(self) -> Optional[str]:
        """
        The RANGE_UNITS keyword specifies the units for the range observable. ‘km’ shall be
        used if the range is measured in kilometers. ‘s’ shall be used if the range is measured
        in seconds. ‘RU’, for ‘range units’, shall be used where the transmit frequency is
        changing. The default value shall be ‘km’.

        Examples: km, s, RU
        """
        ...

    @range_units.setter
    def range_units(self, value: Optional[str]) -> None: ...
    @property
    def receive_band(self) -> Optional[str]:
        """
        The RECEIVE_BAND keyword shall indicate the frequency band for received frequencies.
        Although not required in general, the RECEIVE_BAND must be present if the MODE is
        SINGLE_DIFF and differenced frequencies or differenced range are provided in order to
        allow proper frequency dependent corrections to be applied.

        Examples: S, X, Ka, L, UHF, GREEN
        """
        ...

    @receive_band.setter
    def receive_band(self, value: Optional[str]) -> None: ...
    @property
    def receive_delay_1(self) -> Optional[float]:
        """
        The RECEIVE_DELAY_n keyword shall specify a fixed interval of time, in seconds,
        required for the signal to travel from the tracking point to the receiving electronics.
        The default value shall be 0.0.

        Examples: 1.23, 0.0326, 0.00777

        Units: s
        """
        ...

    @receive_delay_1.setter
    def receive_delay_1(self, value: Optional[float]) -> None: ...
    @property
    def receive_delay_2(self) -> Optional[float]:
        """
        Fixed interval of time, in seconds, required for the signal to travel from the tracking
        point to the receiving electronics for participant 2.

        Units: s
        """
        ...

    @receive_delay_2.setter
    def receive_delay_2(self, value: Optional[float]) -> None: ...
    @property
    def receive_delay_3(self) -> Optional[float]:
        """
        Fixed interval of time, in seconds, required for the signal to travel from the tracking
        point to the receiving electronics for participant 3.

        Units: s
        """
        ...

    @receive_delay_3.setter
    def receive_delay_3(self, value: Optional[float]) -> None: ...
    @property
    def receive_delay_4(self) -> Optional[float]:
        """
        Fixed interval of time, in seconds, required for the signal to travel from the tracking
        point to the receiving electronics for participant 4.

        Units: s
        """
        ...

    @receive_delay_4.setter
    def receive_delay_4(self, value: Optional[float]) -> None: ...
    @property
    def receive_delay_5(self) -> Optional[float]:
        """
        Fixed interval of time, in seconds, required for the signal to travel from the tracking
        point to the receiving electronics for participant 5.

        Units: s
        """
        ...

    @receive_delay_5.setter
    def receive_delay_5(self, value: Optional[float]) -> None: ...
    @property
    def reference_frame(self) -> Optional[str]:
        """
        The REFERENCE_FRAME keyword shall be used in conjunction with the ‘ANGLE_TYPE=RADEC’
        keyword/value combination, indicating the inertial reference frame to which the antenna
        frame is referenced.

        Examples: EME2000, ICRF, ITRF1993, ITRF2000, TOD_EARTH
        """
        ...

    @reference_frame.setter
    def reference_frame(self, value: Optional[str]) -> None: ...
    @property
    def start_time(self) -> Optional[str]:
        """
        The START_TIME keyword shall specify the UTC start time of the total time span covered
        by the tracking data immediately following this Metadata Section. (For format
        specification, see 4.3.9.)

        Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54, 2006-001T00:00:00Z
        """
        ...

    @start_time.setter
    def start_time(self, value: Optional[str]) -> None: ...
    @property
    def stop_time(self) -> Optional[str]:
        """
        The STOP_TIME keyword shall specify the UTC stop time of the total time span covered by
        the tracking data immediately following this Metadata Section. (For format
        specification, see 4.3.9.)

        Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54, 2006-001T00:00:00Z
        """
        ...

    @stop_time.setter
    def stop_time(self, value: Optional[str]) -> None: ...
    @property
    def time_system(self) -> str:
        """
        The TIME_SYSTEM keyword shall specify the time system used for timetags in the
        associated Data Section. This should be UTC for ground-based data. The value associated
        with this keyword must be selected from the full set of allowed values enumerated in
        the SANA Time Systems Registry <https://sanaregistry.org/r/time_systems> (reference `[12]`).
        (See annex B.)

        Examples: UTC, TAI, GPS, SCLK
        """
        ...

    @time_system.setter
    def time_system(self, value: str) -> None: ...
    @property
    def timetag_ref(self) -> Optional[str]:
        """
        The TIMETAG_REF keyword shall provide a reference for time tags in the tracking data.
        This keyword indicates whether the timetag associated with the data is the transmit
        time or the receive time.

        Examples: TRANSMIT, RECEIVE
        """
        ...

    @timetag_ref.setter
    def timetag_ref(self, value: Optional[str]) -> None: ...
    @property
    def track_id(self) -> Optional[str]:
        """
        The TRACK_ID keyword specifies a unique identifier for the tracking data in the
        associated data section. The value may be a freely selected string of characters and
        numbers, only required to be unique for each track of the corresponding sensor. For
        example, the value may be constructed from the measurement date and time and a counter
        to distinguish simultaneously tracked objects.

        Examples: 20190918_1200135-0001
        """
        ...

    @track_id.setter
    def track_id(self, value: Optional[str]) -> None: ...
    @property
    def transmit_band(self) -> Optional[str]:
        """
        The TRANSMIT_BAND keyword shall indicate the frequency band for transmitted
        frequencies. The frequency ranges associated with each band should be specified in the
        ICD.

        Examples: S, X, Ka, L, UHF, GREEN
        """
        ...

    @transmit_band.setter
    def transmit_band(self, value: Optional[str]) -> None: ...
    @property
    def transmit_delay_1(self) -> Optional[float]:
        """
        The TRANSMIT_DELAY_n keyword shall specify a fixed interval of time, in seconds,
        required for the signal to travel from the transmitting electronics to the transmit
        point. The default value shall be 0.0.

        Examples: 1.23, 0.0326, 0.00077

        Units: s
        """
        ...

    @transmit_delay_1.setter
    def transmit_delay_1(self, value: Optional[float]) -> None: ...
    @property
    def transmit_delay_2(self) -> Optional[float]:
        """
        Fixed interval of time, in seconds, required for the signal to travel from the
        transmitting electronics to the transmit point for participant 2.

        Units: s
        """
        ...

    @transmit_delay_2.setter
    def transmit_delay_2(self, value: Optional[float]) -> None: ...
    @property
    def transmit_delay_3(self) -> Optional[float]:
        """
        Fixed interval of time, in seconds, required for the signal to travel from the
        transmitting electronics to the transmit point for participant 3.

        Units: s
        """
        ...

    @transmit_delay_3.setter
    def transmit_delay_3(self, value: Optional[float]) -> None: ...
    @property
    def transmit_delay_4(self) -> Optional[float]:
        """
        Fixed interval of time, in seconds, required for the signal to travel from the
        transmitting electronics to the transmit point for participant 4.

        Units: s
        """
        ...

    @transmit_delay_4.setter
    def transmit_delay_4(self, value: Optional[float]) -> None: ...
    @property
    def transmit_delay_5(self) -> Optional[float]:
        """
        Fixed interval of time, in seconds, required for the signal to travel from the
        transmitting electronics to the transmit point for participant 5.

        Units: s
        """
        ...

    @transmit_delay_5.setter
    def transmit_delay_5(self, value: Optional[float]) -> None: ...
    @property
    def turnaround_denominator(self) -> Optional[int]:
        """
        The TURNAROUND_DENOMINATOR keyword shall indicate the denominator of the turnaround
        ratio that is necessary to calculate the coherent downlink from the uplink frequency.

        Examples: 221, 749
        """
        ...

    @turnaround_denominator.setter
    def turnaround_denominator(self, value: Optional[int]) -> None: ...
    @property
    def turnaround_numerator(self) -> Optional[int]:
        """
        The TURNAROUND_NUMERATOR keyword shall indicate the numerator of the turnaround ratio
        that is necessary to calculate the coherent downlink from the uplink frequency.

        Examples: 240, 880
        """
        ...

    @turnaround_numerator.setter
    def turnaround_numerator(self, value: Optional[int]) -> None: ...

class TdmMode:
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

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

    @property
    def value(self) -> Optional[float]:
        """
        Measurement value as float.

        Returns None if the value is not representable as a float (unlikely for TDM).
        """
        ...

    @property
    def value_str(self) -> str:
        """
        Measurement value as string.

        Useful for phase counts which may require high precision.
        """
        ...

class TdmPath:
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

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

class TimeSystem:
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

class TleParameters:
    """
    TLE Related Parameters (This section is only required if MEAN_ELEMENT_THEORY=SGP/SGP4).

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
        mean_motion_dot: float,
        ephemeris_type: Optional[int] = None,
        classification_type: Optional[str] = None,
        norad_cat_id: Optional[int] = None,
        element_set_no: Optional[int] = None,
        rev_at_epoch: Optional[int] = None,
        bstar: Optional[float] = None,
        bterm: Optional[float] = None,
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
        Solar radiation pressure coefficient AY/m, where y = reflectivity, A = average
        cross-sectional area, m = mass. Example values AGOM = 0.01 (rocket body) and 0.001
        (payload); average value spanning 20,000 catalog objects = 0.0143 m2/kg. Required
        when MEAN_ELEMENT_THEORY= SGP4-XP.

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

        Units: 1/[Earth radii]
        """
        ...

    @bstar.setter
    def bstar(self, value: Optional[float]) -> None: ...
    @property
    def bterm(self) -> Optional[float]:
        """
        Drag-like ballistic coefficient, required for SGP4 and SGP4-XP mean element models:
        MEAN_ELEMENT_THEORY= SGP4-XP (BTERM ballistic coefficient CDA/m, where CD = drag
        coefficient, A = average cross-sectional area, m = mass. Example values for BTERM =
        0.02 (rocket body), 0.0015 (payload); average value spanning 20,000 catalog objects =
        0.0286.

        Units: m²/kg
        """
        ...

    @bterm.setter
    def bterm(self, value: Optional[float]) -> None: ...
    @property
    def classification_type(self) -> Optional[str]:
        """
        Classification type. Default value = U. (See 4.2.4.7.)
        """
        ...

    @classification_type.setter
    def classification_type(self, value: Optional[str]) -> None: ...
    @property
    def comment(self) -> list[str]:
        """
        Comments (see 7.8 for formatting rules).
        """
        ...

    @comment.setter
    def comment(self, value: list[str]) -> None: ...
    @property
    def element_set_no(self) -> Optional[int]:
        """
        Element set number for this satellite. Normally incremented sequentially but may be out
        of sync if it is generated from a backup source. Used to distinguish different TLEs,
        and therefore only meaningful if TLE-based data is being exchanged (i.e.,
        MEAN_ELEMENT_THEORY = SGP/SGP4).
        """
        ...

    @element_set_no.setter
    def element_set_no(self, value: Optional[int]) -> None: ...
    @property
    def ephemeris_type(self) -> Optional[int]:
        """
        Ephemeris type. Default value = 0. (See 4.2.4.7.)
        """
        ...

    @ephemeris_type.setter
    def ephemeris_type(self, value: Optional[int]) -> None: ...
    @property
    def mean_motion_ddot(self) -> Optional[float]:
        """
        Second Time Derivative of Mean Motion (i.e., a drag term). (See 4.2.4.7 for important
        details). Required when MEAN_ELEMENT_THEORY= SGP or PPT3.

        Units: rev/day³
        """
        ...

    @mean_motion_ddot.setter
    def mean_motion_ddot(self, value: Optional[float]) -> None: ...
    @property
    def mean_motion_dot(self) -> float:
        """
        First Time Derivative of the Mean Motion (i.e., a drag term, required when
        MEAN_ELEMENT_THEORY = SGP or PPT3). (See 4.2.4.7 for important details).

        Units: rev/day²
        """
        ...

    @mean_motion_dot.setter
    def mean_motion_dot(self, value: float) -> None: ...
    @property
    def norad_cat_id(self) -> Optional[int]:
        """
        NORAD Catalog Number (‘Satellite Number’) an integer of up to nine digits. This keyword
        is only required if MEAN_ELEMENT_THEORY=SGP/SGP4.
        """
        ...

    @norad_cat_id.setter
    def norad_cat_id(self, value: Optional[int]) -> None: ...
    @property
    def rev_at_epoch(self) -> Optional[int]:
        """
        Revolution Number
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
        parameters : dict[str, str], optional
        A dictionary of user-defined parameters and their values.
    comment : list[str], optional
        Comments.
    """
    def __init__(parameters=None, comment=None) -> None: ...
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

class YesNo:
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

class NdmError(Exception):
    """
    Base exception for all CCSDS NDM errors.
    """
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    def __setstate__():
        """ """
        ...

    @property
    def args(self): ...

class NdmEpochError(ValueError):
    """
    Error parsing a CCSDS epoch string.
    """
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    def __setstate__():
        """ """
        ...

    @property
    def args(self): ...

class NdmFormatError(ValueError):
    """
    Error during parsing of NDM data (KVN or XML).
    """
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    def __setstate__():
        """ """
        ...

    @property
    def args(self): ...

class NdmIoError(OSError):
    """
    I/O error during file operations.
    """
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    def __setstate__():
        """ """
        ...

    @property
    def args(self): ...
    @property
    def characters_written(self): ...

class NdmUnsupportedMessageError(NdmError):
    """
    Unsupported CCSDS message type.
    """
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    def __setstate__():
        """ """
        ...

    @property
    def args(self): ...

class NdmValidationError(NdmError):
    """
    Validation error against CCSDS rules.
    """
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    def __setstate__():
        """ """
        ...

    @property
    def args(self): ...

class NdmKvnParseError(NdmFormatError):
    """
    Error during KVN parsing.
    """
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    def __setstate__():
        """ """
        ...

    @property
    def args(self): ...

class NdmXmlError(NdmFormatError):
    """
    Error during XML parsing or serialization.
    """
    def __getstate__(self, /):
        """
        Helper for pickle.
        """
        ...

    def __setstate__():
        """ """
        ...

    @property
    def args(self): ...
