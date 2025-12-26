# 1 Introduction

## 1.1 PURPOSE

1.1.1 This Attitude Data Message (ADM) Recommended Standard specifies three standard message formats for use in transferring spacecraft attitude information between space agencies and commercial or governmental spacecraft operators: the Attitude Parameter Message (APM), the Attitude Ephemeris Message (AEM), and the Attitude Comprehensive Message (ACM). Such exchanges are used for:

*   preflight planning for attitude estimation support;
*   scheduling attitude and data processing support;
*   carrying out attitude operations;
*   performing attitude comparisons;
*   carrying out attitude propagations and/or sensor predictions;
*   testing to initialize subsystem simulators (communications, power, etc.).

1.1.2 This Recommended Standard includes sets of requirements and criteria that the message formats have been designed to meet. For exchanges in which these requirements do not capture the needs of the participating agencies and operators, another mechanism can be selected.

## 1.2 SCOPE AND APPLICABILITY

1.2.1 This document contains three attitude data messages designed for applications involving data interchange in space data systems. The rationale behind the design of each message is described in annex E and can help the application engineer to select a suitable message. Applicability information specific to each Attitude Data Message format appears in sections 3 through 5 as well as in annex subsection E3. Definition of the attitude accuracy depends on the type of message: it is possible with the ACM, but not with the APM or AEM, in which case it can be specified via an Interface Control Document (ICD) between data exchange participants.

1.2.2 This Recommended Standard is applicable only to the message format and content, but not to its transmission. The transmission of the message between agencies and operators is outside the scope of this document.

1.2.3 Description of the message formats based on the use of ‘Keyword = Value’ Notation (KVN) is available (see section 6).

1.2.4 Description of the message formats based on the use of the eXtensible Markup Language (XML) is available (see section 7).

1.2.5 The format to be exchanged (KVN or XML) is subject to agreement between exchange partners.

## 1.3 CONVENTIONS AND DEFINITIONS

### 1.3.1 NOTATION

#### 1.3.1.1 Unit Notations

The following conventions for unit notations apply throughout this Recommended Standard. Units are drawn from the International System of Units (SI); units are either SI base units, SI derived units, or units outside the SI that are accepted for use with the SI. Except as noted, the units used within this document are as follows:

*   d: days, 86400 SI seconds;
*   kg: kilograms;
*   km: kilometers;
*   m: meters;
*   n/a: (units are not applicable);
*   %: percent;
*   s: SI seconds;
*   SFU: Solar Flux Units, equivalent to 10⁻²² W/(m²\*Hz);
*   W: watts.

#### 1.3.1.2 General

The following notational conventions are used in this document:

a) multiplication of units is denoted with a single asterisk ‘\*’ (e.g., ‘kg\*s’);
b) exponents of units are denoted with a double asterisk ‘\*\*’ (e.g., m² = m\*\*2);
c) square roots of units are denoted by the same exponent notation of a double asterisk ‘\*\*’ (e.g., √km = km\*\*0.5);
d) division of units is denoted with a single forward slash ‘/’ (e.g., m/s);
e) The usual order of operations ordering applies (e.g., exponents before multiplication).

### 1.3.2 NOMENCLATURE

The following conventions apply for the normative specifications in this Manual:

a) the words ‘shall’ and ‘must’ imply a binding and verifiable specification;
b) the word ‘should’ implies an optional, but desirable, specification;
c) the word ‘may’ implies an optional specification;
d) the words ‘is’, ‘are’, and ‘will’ imply statements of fact.

**NOTE** – These conventions do not imply constraints on diction in text that is clearly informative in nature.

### 1.3.3 DEFINITIONS

As in some attitude dynamics references, in this document the term ‘nutation’ is used to mean the motion of the spin axis of a body about an inertial axis. In many other references this motion is called ‘precession’.

## 1.4 STRUCTURE OF THIS DOCUMENT

1.4.1 Section 2 provides a brief overview of the CCSDS-recommended Attitude Data Message types, the APM, the AEM, and the ACM.

1.4.2 Section 3 provides details about the structure and content of the APM.

1.4.3 Section 4 provides details about the structure and content of the AEM.

1.4.4 Section 5 provides details about the structure and content of the ACM.

1.4.5 Section 6 provides details about ADM KVN syntax.

1.4.6 Section 7 provides details about constructing an ADM/XML instance.

1.4.7 Annex A provides the Implementation Conformance Statement (ICS) requirements list.

1.4.8 Annex B provides a list of approved values for selected keywords in the ADM Metadata and Data sections.

1.4.9 Annex C is relative to security, SANA, and patents considerations.

1.4.10 Annex D is a list of abbreviations and acronyms applicable to the ADM.

1.4.11 Annex E lists a set of requirements that were taken into consideration in the design of the APM, AEM, and ACM, along with tables and discussion regarding the applicability of the three message types to various attitude estimation tasks and functions.

1.4.12 Annex F details the conventions relative to ADM data used in this document.

1.4.13 Annex G shows examples of ADM messages.

1.4.14 Annex H is a list of informative references.

1.4.15 Annex I lists a number of items to cover in ICDs prior to exchanging ADMs on a regular basis. There are several statements throughout the document that refer to the desirability or necessity of such a document; this annex lists all the suggested ICD items in a single place in the document.

1.4.16 Annex J gives a summary of changes between ADM versions 1 and 2.

## 1.5 REFERENCES

The following documents contain provisions which, through reference in this text, constitute provisions of this Recommended Standard. At the time of publication, the editions indicated were valid. All documents are subject to revision, and users of this Recommended Standard are encouraged to investigate the possibility of applying the most recent editions of the documents indicated below. The CCSDS Secretariat maintains a register of currently valid CCSDS Recommended Standards.

[1] Information Technology—8-Bit Single-Byte Coded Graphic Character Sets—Part 1: Latin Alphabet No. 1. International Standard, ISO/IEC 8859-1:1998. Geneva: ISO, 1998.

[2] "Online Index of Objects Launched into Outer Space." United Nations Office for Outer Space Affairs (UNOOSA). http://www.unoosa.org/oosa/osoindex.

[3] Time Code Formats. Issue 4. Recommendation for Space Data System Standards (Blue Book), CCSDS 301.0-B-4. Washington, D.C.: CCSDS, November 2010.

[4] XML Specification for Navigation Data Messages. Issue 2. Recommendation for Space Data System Standards (Blue Book), CCSDS 505.0-B-2. Washington, D.C.: CCSDS, May 2021.

[5] IEEE Standard for Binary Floating-Point Arithmetic. IEEE Std 754-1985. New York: IEEE, 1985.

[6] Orbit Data Messages. Issue 3. Recommendation for Space Data System Standards (Blue Book), CCSDS 502.0-B-3. Washington, D.C.: CCSDS, April 2023.

[7] Henry S. Thompson, et al., eds. XML Schema Part 1: Structures. 2nd ed. W3C Recommendation. N.p.: W3C, October 2004.

[8] Paul V. Biron and Ashok Malhotra, eds. XML Schema Part 2: Datatypes. 2nd ed. W3C Recommendation. N.p.: W3C, October 2004.

[9] “CCSDS Navigation Standards Normative Annexes.” Space Assigned Numbers Authority. https://sanaregistry.org/r/navigation_standard_normative_annexes.


# 2 Overview

## 2.1 ATTITUDE DATA MESSAGE TYPES

2.1.1 Three CCSDS-recommended Attitude Data Messages are described in this Recommended Standard: the Attitude Parameter Message, the Attitude Ephemeris Message, and the Attitude Comprehensive Message.

2.1.2 The recommended format for attitude data messages is ASCII. While binary-based attitude data message formats are computer efficient and minimize overhead on uplinked/downlinked data streams, there are ground-segment applications for which an ASCII character-based message is more appropriate. For example, when files or data objects are created using text editors or word processors, ASCII character-based attitude data format representations are necessary. They are also useful in transferring text files between heterogeneous computing systems, because the ASCII character set is nearly universally used and is interpretable by all popular systems. In addition, direct human-readable downloads of text files or objects to displays or printers are possible without preprocessing. The penalty for this convenience is inefficiency.

2.1.3 As currently specified, an APM, AEM, or ACM file is to represent attitude data for a single vehicle.

## 2.2 ATTITUDE PARAMETER MESSAGE

2.2.1 An APM specifies the attitude state of a single object at a specified epoch. This message is suited to interagency exchanges that (1) involve automated interaction and/or human interaction, and (2) do not require high-fidelity dynamic modeling. (For high-fidelity dynamic modeling, see 2.3, Attitude Ephemeris Message and 2.4, Attitude Comprehensive Message.)

2.2.2 The APM requires the use of a propagation technique to determine the attitude state at times different from the specified epoch, leading to a higher level of effort for software implementation than for the AEM. When inertial frames are specified, the APM is fully self-contained and no additional information is required to specify the attitude; if local orbital frames are specified, then an APM must be accompanied by a corresponding Orbit Data Message (reference [6]).

2.2.3 The APM allows for modeling of any number of finite maneuvers.

2.2.4 The attributes of the APM also make it suitable for applications such as exchanges by email or even FAX or voice, or applications in which the message is to be frequently interpreted by humans.

## 2.3 ATTITUDE EPHEMERIS MESSAGE

2.3.1 An AEM specifies the attitude state of a single object at multiple epochs, contained within a specified time range. The AEM is suited to interagency exchanges that (1) involve automated interaction (e.g., computer-to-computer communication for which frequent, fast, automated time interpretation and processing are required), and (2) require higher fidelity or higher precision dynamic modeling than is possible with the APM (e.g., flexible structures, more complex attitude movement, etc.).

2.3.2 The AEM allows for dynamic modeling of any number of torques (solar pressure, atmospheric torques, magnetics, etc.). The AEM requires the use of an interpolation technique to interpret the attitude state at times different from the tabular epochs.

2.3.3 When inertial reference frames are specified, the AEM is fully self-contained and no additional information is required. If local orbital reference frames are specified, then an AEM must be used in conjunction with an Orbit Data Message (reference [6]).

## 2.4 ATTITUDE COMPREHENSIVE MESSAGE

2.4.1 An ACM specifies the attitude state of a single object at multiple epochs, contained within a specified time range. The ACM aggregates and extends APM and AEM content in a single comprehensive hybrid message and offers the following capabilities:

*   optional rate data elements;
*   optional spacecraft physical properties;
*   optional covariance elements;
*   optional maneuver parameters;
*   optional estimator information.

2.4.2 The ACM is well suited for interagency exchanges that (1) involve automated interaction (e.g., computer-to-computer communication for which frequent, fast, automated time interpretation and processing are required) and (2) require more detailed information such as estimator type, additional estimator states (e.g., gyro bias), sensor details, and covariance data.

2.4.3 When inertial reference frames are specified, the ACM is fully self-contained and no additional information is required. If local orbital reference frames are specified, then an ACM must be used in conjunction with an Orbit Data Message (reference [6]).

## 2.5 EXCHANGE OF MULTIPLE MESSAGES

2.5.1 For a given object, multiple APM, AEM, or ACM messages can be provided in a message exchange session to achieve attitude fidelity requirements. If attitude information for multiple objects is to be exchanged, then multiple APM, AEM, or ACM files are necessary.

## 2.6 DEFINITIONS

2.6.1 Definitions of time systems, reference frames, attitude estimation and prediction methods and models are provided in reference [9] and annex H ([H2], and [H3]).


# 3 ATTITUDE PARAMETER MESSAGE

## 3.1 OVERVIEW

3.1.1 Attitude information may be exchanged between two participants by sending the attitude state (see annex H, [H2] and [H3]) for a specified epoch using an APM. The message recipient must have an attitude propagator available that is able to propagate the APM state to compute the estimated attitude at other desired epochs. For this propagation, additional ancillary information (spacecraft properties such as inertia matrix, torque vectors, reaction wheel data, other data from momentum exchange devices, maneuver planning data, if applicable) shall be included with the message.

3.1.2 The use of the APM shall be applicable under the following conditions:

*   attitude states at specific times have to be exchanged (no propagation is required at the receiver's location);
*   attitude states at other times desired by the recipient have to be exchanged (in this case a propagator including a precise enough modeling of the dynamics has to be available at the receiver's location).

3.1.3 The APM shall be a text file consisting of attitude data for a single object.

3.1.4 The APM file-naming scheme should be mutually agreed between message exchange partners.

3.1.5 The method of exchanging APMs should be mutually agreed between message exchange partners.

**NOTE** – Example APMs are provided in annex G.

## 3.2 APM CONTENT

### 3.2.1 GENERAL

The APM shall be represented as a combination of the following:

a) a header;
b) metadata (data about the data);
c) optional comments (explanatory information); and
d) data.

### 3.2.2 APM HEADER

3.2.2.1 The header shall provide a CCSDS Attitude Data Message version number that identifies the format version.

**NOTE** – The version number is included to anticipate future changes.

3.2.2.1.1 The version keyword shall be `CCSDS_APM_VERS` and the value shall have the form of ‘x.y’, where ‘y’ shall be incremented for corrections and minor changes, and ‘x’ shall be incremented for major changes.

3.2.2.1.2 Version 1.0 shall be reserved for the initial version accepted by the CCSDS as an official Recommended Standard (‘Blue Book’). Version 2.0 shall be used for this Blue Book.

3.2.2.1.3 Testing shall be conducted using APM version numbers less than 1.0 (e.g., 0.x).

3.2.2.1.4 Participating agencies should mutually agree upon the specific APM version numbers they will support.

3.2.2.2 The header shall include the `CREATION_DATE` keyword with the value set to the Coordinated Universal Time (UTC) when the file was created (see 6.8.9 for formatting rules).

**NOTE** – A description of APM header keywords and values is provided in table 3-1.

3.2.2.3 The first header line shall be the first non-blank line in the file.

3.2.2.4 Table 3-1 specifies for each header item:

a) the keyword to be used;
b) a short description of the item;
c) examples of allowed values; and
d) whether the item is Mandatory (M), Optional (O), or Conditional (C); ‘Conditional’ indicates that the item is mandatory if specified conditions are met.

3.2.2.5 Only those keywords shown in table 3-1 shall be used in an APM header.

| Keyword | Description | Examples of Values | M/O/C |
| :--- | :--- | :--- | :--- |
| `CCSDS_APM_VERS` | Format version in the form of 'x.y', where 'y' is incremented for corrections and minor changes, and 'x' is incremented for major changes. | 2.0 | M |
| `COMMENT` | Comments (allowed at the beginning of the APM Header after the APM version number). Each comment line shall begin with this keyword. | `This is a comment.` | O |
| `CLASSIFICATION` | User-defined free-text message classification/caveats of this OPM. It is recommended that selected values be precoordinated between exchanging entities by mutual agreement. | `SBU` <br> 'Operator-proprietary data; secondary distribution not permitted' | O |
| `CREATION_DATE` | File creation date/time in UTC. (For format specification, see 6.8.9.) | `2001-11-06T11:17:33` <br> `2001-101T11:17:33` | M |
| `ORIGINATOR` | Creating agency or operator. Select from the accepted set of values indicated in annex B, subsection B1. If desired organization is not listed there, follow annex B procedures to request originator be added to SANA registry. | `CNES` <br> `ESOC` <br> `GSFC` <br> `GSOC` <br> `JPL` <br> `JAXA` <br> `Other agency` | M |
| `MESSAGE_ID` | ID that uniquely identifies a message from a given originator. The format and content of the message identifier value are at the discretion of the originator. | `201113719185` <br> `ABC-12` <br> `34` | O |

### 3.2.3 APM METADATA

3.2.3.1 Table 3-2 specifies for each metadata item:

a) the keyword to be used;
b) a short description of the item;
c) examples of allowed values; and
d) whether the item is Mandatory (M), Optional (O), or Conditional (C); ‘Conditional’ indicates that the item is mandatory if specified conditions are met.

3.2.3.2 Only those keywords shown in table 3-2 shall be used in APM metadata.

**NOTE** – For some keywords (`OBJECT_NAME`, `OBJECT_ID`, `CENTER_NAME`) there are no definitive lists of authorized values maintained by a control authority; the references listed in 1.5 (references [2] and [9]) are the best known sources for authorized values to date.

| Keyword | Description | Examples of Values | M/O/C |
| :--- | :--- | :--- | :--- |
| `COMMENT` | Comments (allowed only at the beginning of the APM Metadata before `OBJECT_NAME`). Each comment line shall begin with this keyword. | `This is a comment.` | O |
| `OBJECT_NAME` | Spacecraft name for which the attitude state is provided. While there is no CCSDS-based restriction on the value for this keyword, it is recommended to use names from the UN Office of Outer Space Affairs designator index (reference [2], which include object name and international designator). When `OBJECT_NAME` is not known or cannot be disclosed, the value should be set to `UNKNOWN`. | `EUTELSAT W1` <br> `MARS PATHFINDER` <br> `UNKNOWN` | M |
| `OBJECT_ID` | Spacecraft identifier of the object corresponding to the attitude data to be given. While there is no CCSDS-based restriction on the value for this keyword, it is recommended to use international designators from the UN Office of Outer Space Affairs (reference [2]). Recommended values have the format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three digit serial number of launch in year YYYY (with leading zeros). P{PP} = At least one letter for the identification of the part brought into space by the launch. In cases in which the asset is not listed in reference [2], the UN Office of Outer Space Affairs designator index format is not used, or the content cannot be disclosed, the value should be set to `UNKNOWN`. | `2000-052A` | M |
| `CENTER_NAME` | Celestial body orbited by the object, which may be a natural solar system body (planets, asteroids, comets, and natural satellites), including any planet barycenter or the solar system barycenter. The set of allowed values is described in annex B, subsection B8. | `EARTH` <br> `BARYCENTER` <br> `MOON` | O |
| `TIME_SYSTEM` | Time system used for attitude and maneuver data. The set of allowed values is described in annex B, subsection B2. | `UTC` <br> `TAI` | M |

### 3.2.4 APM DATA

3.2.4.1 Table 3-3 provides an overview of the six logical blocks in the APM Data section (attitude quaternion, attitude Euler angles, angular velocity data, spin data, spacecraft inertia parameters, maneuver parameters), and specifies for each data item:

a) the keyword to be used;
b) a short description of the item;
c) the data type (R: real, S: string; I: integer, E: epoch);
d) the units;
e) whether the item is Mandatory (M), Optional (O), or Conditional (C): an ‘M’ denotes mandatory keywords that must be included in this section if that particular Data section is included; a ‘C’ indicates that the item is mandatory if specified conditions are met (e.g., providing all nutation or momentum keywords if any are provided).

3.2.4.2 Only those keywords shown in table 3-3 shall be used in APM data.

**NOTE** – Remarks concerning the keywords in table 3-3 appear immediately after the table.

3.2.4.3 The APM message shall contain at least one logical block.

3.2.4.4 Any particular type of block may be repeated several times.

3.2.4.5 All data, except for the maneuver data, shall be relative to the same epoch.

3.2.4.6 The spin block shall contain either `NUTATION`, `NUTATION_PER`, `NUTATION_PHASE`, or `MOMENTUM_ALPHA`, `MOMENTUM_DELTA`, `NUTATION_VEL`.

**Table 3-3: APM Data**

| Keyword | Description | Type | Unit | M/O/C |
| :--- | :--- | :--- | :--- | :--- |
| `COMMENT` | One or more comment line(s). Each comment line shall begin with this keyword. | n/a | n/a | O |
| `EPOCH` | Epoch of the attitude elements and optional logical blocks. (For format specification, see 6.8.9.) | E | n/a | M |
| **Block: Attitude Quaternion** | All mandatory elements are to be provided if the block is present. (See annex F for conventions and further detail.) |
| `QUAT_START` | Indicator of start of data block | n/a | n/a | M |
| `COMMENT` | One or more comment line(s). Each comment line shall begin with this keyword. | n/a | n/a | O |
| `REF_FRAME_A` | Name of the reference frame that defines the starting point of the transformation. The set of allowed values is described in annex B, subsection B3. | S | n/a | M |
| `REF_FRAME_B` | Name of the reference frame that defines the end point of the transformation. The set of allowed values is described in annex B, subsection B3. | S | n/a | M |
| `Q1` | $e_1 * sin(φ/2)$ <br> φ = rotation angle, $e_1$ = 1st component of rotation axis | R | dimensionless | M |
| `Q2` | $e_2 * sin(φ/2)$ <br> φ = rotation angle, $e_2$ = 2nd component of rotation axis | R | dimensionless | M |
| `Q3` | $e_3 * sin(φ/2)$ <br> φ = rotation angle, $e_3$ = 3rd component of rotation axis | R | dimensionless | M |
| `QC` | $cos(φ/2)$ <br> φ = rotation angle | R | dimensionless | M |
| `Q1_DOT` | Time derivative of Q1 | R | 1/s | O |
| `Q2_DOT` | Time derivative of Q2 | R | 1/s | O |
| `Q3_DOT` | Time derivative of Q3 | R | 1/s | O |
| `QC_DOT` | Time derivative of Qc | R | 1/s | O |
| `QUAT_STOP` | Indicator of end of data block | n/a | n/a | M |
| **Block: Euler angle elements** | All mandatory elements of the logical block are to be provided if the block is present. (See annex F for conventions and further detail.) |
| `EULER_START` | Indicator of start of data block | n/a | n/a | M |
| `COMMENT` | One or more comment line(s). Each comment line shall begin with this keyword. | n/a | n/a | O |
| `REF_FRAME_A` | Name of the reference frame that defines the starting point of the transformation. The set of allowed values is described in annex B, subsection B3. | S | n/a | M |
| `REF_FRAME_B` | Name of the reference frame that defines the end point of the transformation. The set of allowed values is described in annex B, subsection B3. | S | n/a | M |
| `EULER_ROT_SEQ` | Rotation sequence that defines the `REF_FRAME_A` to `REF_FRAME_B` transformation. The order of the transformation is from left to right, where the leftmost letter (X, Y, or Z) represents the rotation axis of the first rotation, the second letter (X, Y, or Z) represents the rotation axis of the second rotation, and the third letter (X, Y, or Z) represents the rotation axis of the third rotation. | S | n/a | M |
| `ANGLE_1` | Angle of the first rotation | R | deg | M |
| `ANGLE_2` | Angle of the second rotation | R | deg | M |
| `ANGLE_3` | Angle of the third rotation | R | deg | M |
| `ANGLE_1_DOT` | Time derivative of angle of the first rotation | R | deg/s | O |
| `ANGLE_2_DOT` | Time derivative of angle of the second rotation | R | deg/s | O |
| `ANGLE_3_DOT` | Time derivative of angle of the third rotation | R | deg/s | O |
| `EULER_STOP` | Indicator of end of data block | n/a | n/a | M |
| **Block: Angular velocity vector** | All mandatory elements are to be provided if the block is present. (See annex F for conventions and further detail.) |
| `ANGVEL_START` | Indicator of start of data block | n/a | n/a | M |
| `COMMENT` | One or more comment line(s). Each comment line shall begin with this keyword. | n/a | n/a | O |
| `REF_FRAME_A` | Name of the reference frame that defines the starting point of the transformation. The set of allowed values is described in annex B, subsection B3. | S | n/a | M |
| `REF_FRAME_B` | Name of the reference frame that defines the end point of the transformation. The set of allowed values is described in annex B, subsection B3. | S | n/a | M |
| `ANGVEL_FRAME` | Reference frame in which the components of the angular velocity vector are given. The set of allowed values is described in annex B, subsection B3. | S | n/a | M |
| `ANGVEL_X` | Component of the angular velocity vector on the X axis | R | deg/s | M |
| `ANGVEL_Y` | Component of the angular velocity vector on the Y axis | R | deg/s | M |
| `ANGVEL_Z` | Component of the angular velocity vector on the Z axis | R | deg/s | M |
| `ANGVEL_STOP` | Indicator of end of data block | n/a | n/a | M |
| **Block: Spin** | All mandatory elements are to be provided if the block is present. (See annex F for conventions and further detail.) |
| `SPIN_START` | Indicator of start of data block | n/a | n/a | M |
| `COMMENT` | One or more comment line(s). Each comment line shall begin with this keyword. | n/a | n/a | O |
| `REF_FRAME_A` | Name of the reference frame that defines the starting point of the transformation. The set of allowed values is described in annex B, subsection B3. | S | n/a | M |
| `REF_FRAME_B` | Name of the reference frame that defines the end point of the transformation. The set of allowed values is described in annex B, subsection B3. | S | n/a | M |
| `SPIN_ALPHA` | Right ascension of spin axis vector in frame A | R | deg | M |
| `SPIN_DELTA` | Declination of the spin axis vector in frame A | R | deg | M |
| `SPIN_ANGLE` | Phase of the satellite about the spin axis | R | deg | M |
| `SPIN_ANGLE_VEL`| Angular velocity of satellite around spin axis | R | deg/s | M |
| `NUTATION` | Nutation angle of spin axis | R | deg | C |
| `NUTATION_PER` | Body nutation period of the spin axis | R | s | C |
| `NUTATION_PHASE`| Inertial nutation phase | R | deg | C |
| `MOMENTUM_ALPHA`| Right ascension of angular momentum vector in frame A | R | deg | C |
| `MOMENTUM_DELTA`| Declination of angular momentum vector in frame A | R | deg | C |
| `NUTATION_VEL` | Angular velocity of spin vector around the angular momentum vector | R | deg/s | C |
| `SPIN_STOP` | Indicator of end of data block | n/a | n/a | M |
| **Block: Inertia** | All mandatory elements are to be provided if the block is present. (See annex F for conventions and further detail.) |
| `INERTIA_START`| Indicator of start of data block | n/a | n/a | M |
| `COMMENT` | One or more comment line(s). Each comment line shall begin with this keyword. | n/a | n/a | O |
| `INERTIA_REF_FRAME` | Coordinate system for the inertia tensor. The set of allowed values is described in annex B, subsection B3. | S | n/a | M |
| `IXX` | Moment of Inertia about the X-axis | R | kg\*m\*\*2 | M |
| `IYY` | Moment of Inertia about the Y-axis | R | kg\*m\*\*2 | M |
| `IZZ` | Moment of Inertia about the Z-axis | R | kg\*m\*\*2 | M |
| `IXY` | Inertia Cross Product of the X and Y axes | R | kg\*m\*\*2 | M |
| `IXZ` | Inertia Cross Product of the X and Z axes | R | kg\*m\*\*2 | M |
| `IYZ` | Inertia Cross Product of the Y and Z axes | R | kg\*m\*\*2 | M |
| `INERTIA_STOP` | Indicator of end of data block | n/a | n/a | M |
| **Block: Maneuver Parameters** | All mandatory elements are to be provided if the block is present. (See annex F for conventions and further detail.) |
| `MAN_START` | Indicator of start of data block | n/a | n/a | M |
| `COMMENT` | One or more comment line(s). Each comment line shall begin with this keyword. | n/a | n/a | O |
| `MAN_EPOCH_START`| Epoch of start of maneuver. (For format specification, see 6.8.9.) | E | n/a | M |
| `MAN_DURATION` | Maneuver duration | R | s | M |
| `MAN_REF_FRAME`| Coordinate system for the torque vector. The set of allowed values is described in annex B, subsection B3. | S | n/a | M |
| `MAN_TOR_X` | 1st component of the torque vector | R | N\*m | M |
| `MAN_TOR_Y` | 2nd component of the torque vector | R | N\*m | M |
| `MAN_TOR_Z` | 3rd component of the torque vector | R | N\*m | M |
| `MAN_DELTA_MASS`| Mass change during maneuver (value is <= 0) | R | kg | O |
| `MAN_STOP` | Indicator of end of data block | n/a | n/a | M |

### 3.2.5 REMARKS

#### 3.2.5.1 Data Format

**NOTE** – Subsection 6.8.9 contains instructions about how to format the `EPOCH` and `MAN_EPOCH_START`.

In specifying the EPOCH of the message, care must be taken if UTC is used as the TIME_SYSTEM. If an APM message reports attitude during a time of leap seconds, the system making use of the message must be able to recognize 60 as a valid value for the seconds (e.g., `20xx-xx-xxT23:59:58.000` ... `20xx-xx-xxT23:59:59.000` ... `20xx-xx-xxT23:59:60.000` .. `20xx-xx-xxT00:00:00.000`).

#### 3.2.5.2 Technical

3.2.5.2.1 It could become necessary to utilize particular orbit information to process Euler angle elements or a local orbit frame (e.g., LVLH, QSW) properly. As an approach to this, the message may include a ‘COMMENT’ block specifying a particular OPM message (reference [6]) to use in conjunction with a particular APM.

3.2.5.2.2 Specification of Euler angle rotations around only one or two axes may be handled by entering the appropriate sequence for the desired one- or two-axis rotation and freely choosing the final axis of rotation and giving a value of zero for the rotation angle.


# 4 ATTITUDE EPHEMERIS MESSAGE

## 4.1 GENERAL

4.1.1 Attitude state information may be exchanged between participants by sending an ephemeris in the form of a series of attitude states using an AEM. The message recipient must have a suitable means of interpolating across these attitude states to obtain the attitude state at an arbitrary time contained within the span of the attitude ephemeris.

4.1.2 The AEM file-naming scheme should be mutually agreed between message exchange partners.

4.1.3 The method of exchanging AEMs should be mutually agreed between message exchange partners.

**NOTE** – Example AEMs are provided in annex G.

## 4.2 AEM CONTENT

### 4.2.1 GENERAL

The AEM shall be represented as a combination of the following:

a) a header;
b) metadata (data about data);
c) optional comments (explanatory information); and
d) attitude data.

**NOTES**

1.  The group composed of ‘metadata’, optional comments, and data is called a segment. The set of segments is called the ‘body’.
2.  Table 4-1 outlines the contents of an AEM.

| Item | | M/O/C |
| :--- | :--- | :--- |
| Header | | M |
| Body | Segment 1 | M |
| | Metadata 1 | |
| | Data 1 | |
| | Segment 2 | O |
| | Metadata 2 | |
| | Data 2 | |
| | ... | O |
| | Segment n | O |
| | Metadata n | |
| | Data n | |

### 4.2.2 AEM HEADER

4.2.2.1 The header shall provide a CCSDS Attitude Data Message version number that identifies the format version.

**NOTE** – The version number is included to anticipate future changes.

4.2.2.1.1 The version keyword shall be `CCSDS_AEM_VERS` and the value shall have the form of ‘x.y’, where ‘y’ is incremented for corrections and minor changes, and ‘x’ is incremented for major changes.

4.2.2.1.2 Version 1.0 shall be reserved for the initial version accepted by the CCSDS as an official Recommended Standard (‘Blue Book’). Version 2.0 shall be used for this blue book.

4.2.2.1.3 Testing shall be conducted using AEM version numbers less than 1.0 (e.g., 0.x).

4.2.2.1.4 Participating agencies should mutually agree upon the specific AEM version numbers they will support.

4.2.2.2 The header shall include the `CREATION_DATE` keyword with the value set to the UTC when the file was created (see 6.8.9 for formatting rules). A description of AEM header keywords and values is provided in table 4-2.

4.2.2.3 The first header line must be the first non-blank line in the file.

4.2.2.4 The AEM header assignments are shown in table 4-2, which specifies for each item:

a) the keyword to be used;
b) a short description of the item;
c) examples of allowed values; and
d) whether the item is Mandatory (M), Optional (O), or Conditional (C); ‘C’ indicates that the item is mandatory if specified conditions are met.

4.2.2.5 Only those keywords shown shall be used in an AEM header.

| Keyword | Description | Examples of Values | M/O/C |
| :--- | :--- | :--- | :--- |
| `CCSDS_AEM_VERS` | Format version in the form of 'x.y', where 'y' is incremented for corrections and minor changes, and 'x' is incremented for major changes. | 2.0 | M |
| `COMMENT` | One or more comment lines. Each comment line shall begin with this keyword. | `This is a comment.` | O |
| `CLASSIFICATION` | User-defined free-text message classification/caveats of this OPM. It is recommended that selected values be precoordinated between exchanging entities by mutual agreement. | `SBU` <br> 'Operator-proprietary data; secondary distribution not permitted' | O |
| `CREATION_DATE` | File creation date/time in UTC. (For format specification, see 6.8.9.) | `2001-11-06T11:17:33` | M |
| `ORIGINATOR` | Creating agency or operator. Select from the accepted set of values indicated in annex B, subsection B1. If desired organization is not listed there, follow annex B procedures to request originator be added to SANA registry. | `CNES` <br> `ESOC` <br> `GSFC` <br> `GSOC` <br> `JPL` <br> `JAXA` | M |
| `MESSAGE_ID` | ID that uniquely identifies a message from a given originator. The format and content of the message identifier value are at the discretion of the originator. | `201113719185` <br> `ABC-12_` <br> `34` | O |

### 4.2.3 AEM METADATA

4.2.3.1 A single METADATA group shall precede each attitude ephemeris data block. Multiple occurrences of a METADATA group followed by an attitude ephemeris data block may be used (e.g., METADATA, DATA, METADATA, DATA, etc.).

4.2.3.2 Before each METADATA group the string ‘META_START’ shall appear on a separate line and after each METADATA group (and before the associated `DATA_START` keyword) the string ‘META_STOP’ shall appear on a separate line.

4.2.3.3 The AEM metadata assignments are shown in table 4-3, which specifies for each item:

a) the keyword to be used;
b) a short description of the item;
c) examples of allowed values; and
d) whether the item is Mandatory (M), Optional (O), or Conditional (C). ‘Conditional’ indicates that the item is mandatory if specified conditions are met.

4.2.3.4 Only those keywords shown shall be used in AEM metadata. For some keywords (`OBJECT_NAME`, `OBJECT_ID`, `CENTER_NAME`) there are no definitive lists of authorized values maintained by a control authority; the references listed in 1.5 (references [2] and [9]) are the best known sources for authorized values to date.

| Keyword | Description | Examples of Values | M/O/C |
| :--- | :--- | :--- | :--- |
| `META_START` | The AEM message contains both metadata and attitude ephemeris data; this keyword is used to delineate the start of a metadata block within the message (metadata are provided in a block, surrounded by 'META_START' and 'META_STOP' markers to facilitate file parsing). This keyword must appear on a line by itself. | n/a | M |
| `COMMENT` | Comments allowed only at the beginning of the Metadata section. Each comment line shall begin with this keyword. | `This is a comment.` | O |
| `OBJECT_NAME` | Spacecraft name for which the attitude state is provided. While there is no CCSDS-based restriction on the value for this keyword, it is recommended to use names from the UN Office of Outer Space Affairs designator index (reference [2], which include Object name and international designator). When `OBJECT_NAME` is not known or cannot be disclosed, the value should be set to `UNKNOWN`. | `EUTELSAT W1` | M |
| `OBJECT_ID` | Spacecraft identifier of the object corresponding to the attitude data to be given. While there is no CCSDS-based restriction on the value for this keyword, it is recommended to use international designators from the UN Office of Outer Space Affairs (reference [2]). Recommended values have the format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three-digit serial number of launch in year YYYY (with leading zeros). P{PP} = At least one capital letter for the identification of the part brought into space by the launch. In cases in which the asset is not listed in reference [2], the UN Office of Outer Space Affairs designator index format is not used, or the content cannot be disclosed, the value should be set to `UNKNOWN`. | `2000-052A` | M |
| `CENTER_NAME` | Celestial body orbited by the object, which may be a natural solar system body (planets, asteroids, comets, and natural satellites), including any planet barycenter or the solar system barycenter. The set of allowed values is described in annex B, subsection B8. | `EARTH` <br> `STS 106` | O |
| `REF_FRAME_A` | Name of the reference frame that defines the starting point of the transformation. The set of allowed values is described in annex B, subsection B3. | `ICRF` <br> `SC_BODY_1` <br> `INSTRUMENT_A` | M |
| `REF_FRAME_B` | Name of the reference frame that defines the end point of the transformation. The set of allowed values is described in annex B, subsection B3. | `SC_BODY_1` <br> `INSTRUMENT_A` | M |
| `TIME_SYSTEM` | Time system used for both attitude ephemeris data and metadata. The set of allowed values is described in annex B, subsection B2. | `UTC` <br> `TAI` | M |
| `START_TIME` | Start of TOTAL time span covered by attitude ephemeris data immediately following this metadata block. (For format specification, see 6.8.9.) | `1996-12-18T14:28:15.11` | M |
| `USEABLE_START_TIME` | Optional start of USEABLE time span covered by attitude ephemeris data immediately following this metadata block. To allow for proper interpolation near the beginning/end of the attitude ephemeris data block, it may be necessary to utilize this keyword with values within the time span covered by the attitude ephemeris data records as denoted by the `START_TIME`/`STOP_TIME` time tags. The `USEABLE_START_TIME` time tag of a new block of ephemeris data must be greater than or equal to the `USEABLE_STOP_TIME` time tag of the previous block. (For format specification, see 6.8.9.) | `1996-12-18T14:28:15.11` | O |
| `USEABLE_STOP_TIME` | Optional stop of USEABLE time span covered by attitude ephemeris data immediately following this metadata block. (See also `USEABLE_START_TIME`.) (For format specification, see 6.8.9.) | `1996-12-18T14:28:15.11` | O |
| `STOP_TIME` | End of TOTAL time span covered by the attitude ephemeris data immediately following this metadata block. (For format specification, see 6.8.9.) | `1996-12-18T14:28:15.11` | M |
| `ATTITUDE_TYPE` | The type of information contained in the data lines. This keyword must have a value from the set specified at the right. (See table 4-4 for details of the data contained in each line.) | `QUATERNION` <br> `QUATERNION/DERIVATIVE` <br> `QUATERNION/ANGVEL` <br> `EULER_ANGLE` <br> `EULER_ANGLE/DERIVATIVE` <br> `EULER_ANGLE/ANGVEL` <br> `SPIN` <br> `SPIN/NUTATION` <br> `SPIN/NUTATION_MOM` | M |
| `EULER_ROT_SEQ` | Rotation sequence that defines the `REF_FRAME_A` to `REF_FRAME_B` transformation. The order of the transformation is from left to right, where the leftmost letter (X, Y, or Z) represents the rotation axis of the first rotation, the second letter (X, Y, or Z) represents the rotation axis of the second rotation, and the third letter (X, Y, or Z) represents the rotation axis of the third rotation. This keyword is applicable only if `ATTITUDE_TYPE` specifies the use of Euler angles. | `ZXZ` <br> `XYZ` | O |
| `ANGVEL_FRAME` | The frame of reference in which angular velocity data are specified. The set of allowed values is described in annex B, subsection B3. This keyword is applicable only if `ATTITUDE_TYPE` specifies the use of angular velocities in conjunction with either quaternions or Euler angles. | `ICRF` <br> `SC_BODY_1` | O |
| `INTERPOLATION_METHOD` | Recommended interpolation method for attitude ephemeris data in the block immediately following this metadata block. | `linear` <br> `HERMITE` <br> `LAGRANGE` | O |
| `INTERPOLATION_DEGREE` | Recommended interpolation degree for attitude ephemeris data in the block immediately following this metadata block. It must be an integer value. This keyword must be used if the `INTERPOLATION_METHOD` keyword is used. | `1` <br> `5` | O |
| `META_STOP` | The end of a metadata block within the message. The AEM message contains both metadata and attitude ephemeris data; this keyword is used to delineate the end of a metadata block within the message (metadata are provided in a block, surrounded by '`META_START`' and '`META_STOP`' markers to facilitate file parsing). This keyword must appear on a line by itself. | n/a | M |

### 4.2.4 AEM DATA

4.2.4.1 The Data section of the AEM shall be delineated by the '`DATA_START`' and ‘`DATA_STOP`' keywords.

**NOTE** – These keywords are intended to facilitate parsing and will also serve to advise the recipient that all the attitude data records associated with the immediately preceding AEM Metadata section have been received (the rationale for including this is that data volumes can be very large, so knowing when the data begins and ends is desirable). The AEM recipient might process the ‘`DATA_STOP`’ keyword as a ‘local’ end-of-file marker.

4.2.4.2 For AEMs, each set of attitude ephemeris data, including the time tag, must be provided on a single line.

4.2.4.3 Allowed combinations of data items shall be those in table 4-4, with each item following the same definition as given in table 3-3.

4.2.4.4 The order in which the data items are given shall be fixed as in table 4-4.

4.2.4.5 The choice of one of the formats in table 4-4 shall be specified via the `ATTITUDE_TYPE` keyword in the metadata. (See annex F for more information on the data.)

| Keyword | Value | Ephemeris Data Line |
| :--- | :--- | :--- |
| `ATTITUDE_TYPE` | `QUATERNION` | EPOCH, Q1, Q2, Q3, QC |
| | `QUATERNION/DERIVATIVE` | EPOCH, Q1, Q2, Q3, QC, Q1_DOT, Q2_DOT, Q3_DOT, QC_DOT |
| | `QUATERNION/ANGVEL` | EPOCH, Q1, Q2, Q3, QC, ANGVEL_X, ANGVEL_Y, ANGVEL_Z |
| `ATTITUDE_TYPE` | `EULER_ANGLE` | EPOCH, ANGLE_1, ANGLE_2, ANGLE_3 |
| | `EULER_ANGLE/DERIVATIVE` | EPOCH, ANGLE_1, ANGLE_2, ANGLE_3, ANGLE_1_DOT, ANGLE_2_DOT, ANGLE_3_DOT |
| | `EULER_ANGLE/ANGVEL` | EPOCH, ANGLE_1, ANGLE_2, ANGLE_3, ANGVEL_X, ANGVEL_Y, ANGVEL_Z |
| `ATTITUDE_TYPE` | `SPIN` | EPOCH, SPIN_ALPHA, SPIN_DELTA, SPIN_ANGLE, SPIN_ANGLE_VEL |
| | `SPIN/NUTATION` | EPOCH, SPIN_ALPHA, SPIN_DELTA, SPIN_ANGLE, SPIN_ANGLE_VEL, NUTATION, NUTATION_PER, NUTATION_PHASE |
| | `SPIN/NUTATION_MOM` | EPOCH, SPIN_ALPHA, SPIN_DELTA, SPIN_ANGLE, SPIN_ANGLE_VEL, MOMENTUM_ALPHA, MOMENTUM_DELTA, NUTATION_VEL |

**NOTE** – Addition information about the data is contained in 3.2.4.

4.2.4.6 The units used shall be the following:

*   dimensionless: EPOCH, Q1, Q2, Q3, QC;
*   1/s: Q1_DOT, Q2_DOT, Q3_DOT, QC_DOT;
*   deg: ANGLE_1, ANGLE_2, ANGLE_3, SPIN_ALPHA, SPIN_DELTA, SPIN_ANGLE, NUTATION, NUTATION_PHASE, MOMENTUM_ALPHA, MOMENTUM_DELTA;
*   deg/s: ANGLE_1_DOT, ANGLE_2_DOT, ANGLE_3_DOT, ANGVEL_X, ANGVEL_Y, ANGVEL_Z, SPIN_ANGLE_VEL, NUTATION_VEL;
*   s: NUTATION_PER.

**NOTE** – The units do not appear in the AEM data lines. The data lines only contain values.

#### 4.2.4.7 FORMAT

4.2.4.7.1 At least one space character must be used to separate the items in each attitude ephemeris data line.

4.2.4.7.2 Subsection 6.8.9 contains instructions about how to format the EPOCH. It should be noted that any epoch specified denotes spacecraft event time.

4.2.4.7.3 In specifying the EPOCH of the message, care must be taken if UTC is used as the TIME_SYSTEM. If an AEM message reports attitude during a time of leap seconds, the system making use of the message must be able to recognize 60 as a valid value for the seconds (e.g., `20xx-xx-xxT23:59:58.000` ... `20xx-xx-xxT23:59:59.000` ... `20xx-xx-xxT23:59:60.000` .. `20xx-xx-xxT00:00:00.000`).

#### 4.2.4.8 TECHNICAL

4.2.4.8.1 Attitude ephemeris data lines in a given data block must be ordered by increasing time, and time tags must not be repeated. The time step duration may vary within a given AEM.

4.2.4.8.2 The TIME_SYSTEM value must remain fixed within an AEM segment.

4.2.4.8.3 The occurrence of a second (or greater) metadata block after some attitude ephemeris data shall indicate that interpolation using succeeding attitude ephemeris data with attitude ephemeris data occurring prior to that metadata block shall not be done. This method may be used for proper modeling of propulsive maneuvers or any other source of a discontinuity such as eclipse entry or exit.

4.2.4.8.4 Details about the interpolation method should be specified using the `INTERPOLATION_METHOD` and `INTERPOLATION_DEGREE` keywords within the AEM. All data blocks must contain a sufficient number of attitude ephemeris data records to allow the recommended interpolation method to be carried out consistently throughout the AEM.

### 4.2.5 REMARKS

4.2.5.1 It could become necessary to utilize particular orbit information to process Euler angle elements or a local orbit frame (e.g., LVLH, QSW) properly. As an approach to this, the message may include a ‘`COMMENT`’ block specifying a particular OEM message (reference [6]) to use in conjunction with a particular AEM.

4.2.5.2 Specification of Euler angle rotations around only one or two axes may be handled by entering the appropriate sequence for the desired one- or two-axis rotation and freely choosing the final axis of rotation and giving a value of zero for the rotation angle.


# 5 ATTITUDE COMPREHENSIVE MESSAGE

## 5.1 OVERVIEW

Attitude information may be exchanged between two participants using an ACM. The ACM aggregates and extends APM and AEM content in a single hybrid message. The ACM simultaneously emphasizes flexibility and message conciseness by offering extensive optional standardized content while minimizing mandatory content.

## 5.2 REQUIREMENTS

5.2.1 The ACM shall be a plain text file consisting of attitude data for a single space object. It shall be easily readable by both humans and computers.

5.2.2 A sequence of ACMs for either a single object or multiple objects may be aggregated into a single Navigation Data Message (NDM) XML file as shown in section 7.

5.2.3 The ACM file-naming scheme should be mutually agreed between message exchange partners.

5.2.4 The method of exchanging ACMs should be mutually agreed between message exchange partners.

5.2.5 If attitude states are desired at arbitrary time(s) contained within the span of the attitude ephemeris, the message recipient may use a suitable interpolation or propagation method. For times outside of supplied attitude state time spans or if the step size between attitude states is too large to support interpolation or propagation, optional dynamic parameters should be included with this message and the recipient must have a suitably compatible attitude dynamics propagator.

**NOTES**

1.  Detailed syntax rules for the ACM are specified in section 6.
2.  Example ACMs are provided in annex G.

## 5.3 ACM CONTENT/STRUCTURE

### 5.3.1 GENERAL STRUCTURE

5.3.1.1 The ACM shall be represented as a combination of the following mandatory (M) and optional (O) data blocks as shown in table 5-1.

5.3.1.2 The ordering of these sections is mandatory. The order of occurrence of the ACM sections shall be fixed as shown in table 5-1:

a) one mandatory header;
b) a single mandatory Metadata section (data about data);
c) optional Data section(s), comprised of one or more data constituent types:
    1) one or more optional attitude state time histories;
    2) one optional space object physical characteristics section;
    3) one or more optional covariance time histories;
    4) one or more optional maneuver specification section(s);
    5) one optional attitude determination Data section;
    6) one optional, user-defined data and supplemental comments (explanatory information).

**Table 5-1: ACM Layout and Ordering Specification**

| Section | Content | Status M/O |
| :--- | :--- | :--- |
| Header | A single header of the message | M |
| Metadata | A single Metadata section (data about data) | M |
| Data | | |
| | **attitude data #1** | |
| | data description | One or more attitude state time histories (each consisting of one or more attitude states) | O |
| | data lines | |
| | ... | |
| | **attitude data #n** | |
| | data description | |
| | data lines | |
| | **physical properties** | A single space object physical characteristics section | O |
| | **covariance data #1** | |
| | data description | One or more covariance time histories (each consisting of one or more covariance matrix diagonals) | O |
| | data lines | |
| | ... | |
| | **covariance data #n** | |
| | data description | |
| | data lines | |
| | **maneuver data #1** | |
| | ... | One or more maneuver specification sections | O |
| | **maneuver data #n** | |
| | **attitude determination data** | A single attitude determination Data section | O |
| | **user-defined data** | A single user-defined Data section | O |

### 5.3.2 ACM HEADER

5.3.2.1 The header shall provide a CCSDS Attitude Data Message version number that identifies the format version.

**NOTE** – The version number is included to anticipate future changes.

5.3.2.1.1 The version keyword shall be `CCSDS_ACM_VERS` and the value shall have the form of ‘x.y’, where ‘y’ is incremented for corrections and minor changes, and ‘x’ is incremented for major changes.

5.3.2.1.2 Version 1.0 shall be reserved for the initial version accepted by the CCSDS as an official Recommended Standard (‘Blue Book’). Version 2.0 shall be used for this blue book.

5.3.2.1.3 Testing shall be conducted using ACM version numbers less than 1.0 (e.g., 0.x).

5.3.2.1.4 Participating agencies should mutually agree upon the specific ACM version numbers they will support.

5.3.2.2 The header shall include the `CREATION_DATE` keyword with the value set to the UTC when the file was created (see 6.8.9 for formatting rules).

**NOTE** – A description of ACM header keywords and values is provided in table 5-2.

5.3.2.3 The first header line must be the first non-blank line in the file.

5.3.2.4 Table 5-2 specifies the keywords for each header item, and whether they are mandatory (M), optional (O), or conditional (C). ‘Conditional’ indicates that the item is mandatory if specified conditions are met.

5.3.2.5 Only those keywords shown in table 5-2 shall be used in an ACM header.

**Table 5-2: ACM Header**

| Keyword | Description | Examples of Values | M/O/C |
| :--- | :--- | :--- | :--- |
| `CCSDS_ACM_VERS` | Format version in the form of 'x.y', where 'y' is incremented for corrections and minor changes, and 'x' is incremented for major changes. | 2.0 | M |
| `COMMENT` | Comments (allowed in the ACM Header only immediately after the ACM version number). | `This is a comment.` | O |
| `CLASSIFICATION` | User-defined free-text message classification/caveats of this OPM. It is recommended that selected values be precoordinated between exchanging entities by mutual agreement. | `SBU` <br> 'Operator-proprietary data; secondary distribution not permitted' | O |
| `CREATION_DATE` | File creation date/time in UTC. (For format specification, see 6.8.9.) | `2001-11-06T11:17:33` <br> `2002-204T15:56:23Z` | M |
| `ORIGINATOR` | Creating agency or operator. Select from the accepted set of values indicated in annex B, subsection B1. If desired organization is not listed there, follow annex B procedures to request originator be added to SANA registry. | `CNES` <br> `ESOC` <br> `GSFC` <br> `Other Agency` | M |
| `MESSAGE_ID` | ID that uniquely identifies a message from a given originator. The format and content of the message identifier value are at the discretion of the originator. | `201113719185` <br> `ABC-12_34` | O |

### 5.3.3 ACM METADATA

5.3.3.1 Table 5-3 specifies for each metadata item:

a) the keyword to be used;
b) a short description of the item;
c) examples of allowed values; and
d) whether the item is Mandatory (M), Optional (O), or Conditional (C). ‘Conditional’ indicates that the item is mandatory if specified conditions are met.

5.3.3.2 Only those keywords shown in table 5-3 shall be used in ACM metadata.

5.3.3.3 The Metadata section must begin with keyword `META_START` and end with keyword `META_STOP`.

5.3.3.4 The ACM shall only contain a single Metadata section in the entire scope of the message.

5.3.3.5 The order of occurrence of these ACM metadata keywords shall be fixed as shown in table 5-3.

**NOTES**

1.  For some keywords (`OBJECT_NAME`, `OBJECT_DESIGNATOR`) there are no definitive lists of authorized values maintained by a control authority. References [2] and [9] and the organizations provided on the SANA Registry (annex B, subsection B1) are the best known sources for authorized values to date.
2.  While specification of `OBJECT_NAME`, `OBJECT_DESIGNATOR`, and `INTERNATIONAL_DESIGNATOR` are individually optional, it is recommended that at least one of these three keywords be supplied.

**Table 5-3: ACM Metadata**

| Keyword | Description | Examples of Values | M/O/C |
| :--- | :--- | :--- | :--- |
| `META_START` | Start of the Metadata section | n/a | M |
| `COMMENT` | Comments (allowed only at the beginning of the ACM Metadata). Each comment line shall begin with this keyword. | `This is a comment.` | O |
| `CLASSIFICATION` | User-defined free-text message classification/caveats of this ACM. It is recommended that selected values be pre-coordinated between exchanging entities by mutual agreement. | `SBU` <br> `CUI` <br> 'Operator-proprietary data; secondary distribution not permitted' | O |
| `OBJECT_NAME` | Free-text field containing the name of the object. There is no CCSDS-based restriction on the value for this keyword, but it is recommended to use names from either the UN Office of Outer Space Affairs designator index (reference [2], which include Object name and international designator), the spacecraft operator, or a State Actor or commercial Space Situational Awareness (SSA) provider maintaining the 'CATALOG_NAME' space catalog. If the object name is not known (uncorrelated object), 'UNKNOWN' may be used (or this keyword omitted). | `SPOT` <br> `ENVISAT` <br> `IRIDIUM` <br> `INTELSAT` | M |
| `INTERNATIONAL_DESIGNATOR` | Free text field containing an international designator for the object as assigned by the UN Committee on Space Research (COSPAR) and the US National Space Science Data Center (NSSDC). Such designator values have the following COSPAR format: YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three-digit serial number of launch in year YYYY (with leading zeros). P{PP} = At least one capital letter for the identification of the part brought into space by the launch. In cases in which the object has no international designator, the value UNKNOWN may be used. <br> **NOTE** – The international designator is typically specified by 'OBJECT_ID' in the APM and AEM. | `2000-052A` <br> `1996-068A` <br> `2000-053A` <br> `1996-008A` <br> `UNKNOWN` | O |
| `CATALOG_NAME` | Free text field containing the satellite catalog source or the source agency or operator abbreviated name (see annex B, subsection B1). | `CSPOC` <br> `RFSA` <br> `ESA` <br> `COMSPOC` | O |
| `OBJECT_DESIGNATOR` | Free text field specification of the unique satellite identification designator for the object, as reflected in the catalog whose name is 'CATALOG_NAME'. If the ID is not known, ‘UNKNOWN’ may be used (or this keyword omitted). | `22444` <br> `18SPCS 18571` <br> `2147483648 04ae [...] d84c` <br> `UNKNOWN` | O |
| `ORIGINATOR_POC` | Free text field containing Programmatic or Technical Point-of-Contact (POC) for ACM. | `Ms. Rodgers` | O |
| `ORIGINATOR_POSITION` | Free text field containing contact position of the PoC. | `GNC Engineer` <br> `ACS Design Lead` | O |
| `ORIGINATOR_PHONE` | Free text field containing PoC phone number. | `+49615130312` | O |
| `ORIGINATOR_EMAIL` | Free-text field containing originator PoC email address. | `JOHN.DOE@SOMEWHERE.ORG` | O |
| `ORIGINATOR_ADDRESS` | Free text field containing Technical PoC information for ACM creator (suggest email, website, or physical address, etc.). | `JANE.DOE@SOMEWHERE.NET` | O |
| `ODM_MSG_LINK` | Free text field containing a unique identifier of Orbit Data Message(s) that are linked (relevant) to this Attitude Data Message. | `ODM_MSG_12345.txt` <br> `ORB_ID_0123` | O |
| `CENTER_NAME` | Celestial body orbited by the object, which may be a natural solar system body (planets, asteroids, comets, and natural satellites), including any planet barycenter or the solar system barycenter. The set of allowed values is described in annex B, subsection B8. | `EARTH BARYCENTER` <br> `MOON` | O |
| `TIME_SYSTEM` | Time system used for metadata, attitude data, covariance data. The set of allowed values is described in annex B, subsection B2. | `UTC` <br> `TAI` | M |
| `EPOCH_TZERO` | Epoch from which all ACM relative times are referenced. (For format specification, see 6.8.9.) The time scale for `EPOCH_TZERO` is the one specified by '`TIME_SYSTEM`' keyword in the Metadata section. | `2016-11-10T00:00:00` | M |
| `ACM_DATA_ELEMENTS` | Comma-delimited list of elements of information data blocks included in this message. The order shall be the same as the order of the data blocks in the message. Values shall be confined to the following list: `ATT`, `PHYS`, `COV`, `MAN`, `AD`, `USER`. If the ACM contains multiple `ATT`, `COV`, `MAN` data blocks (as allowed by table 5-1), the corresponding `ATT, COV, MAN` entry shall be duplicated to match. | `ATT, AD, USER` <br> `ATT, ATT, PHYS` <br> `ATT, COV, AD` | O |
| `START_TIME` | Time of the earliest data contained in the ACM, specified as either a relative or absolute time tag. | `100.0` <br> `2016-11-10T00:00:00` | O |
| `STOP_TIME` | Time of the latest data contained in the ACM, specified as either a relative or absolute time tag. | `1500.0` <br> `2016-11-11T00:00:00` | O |
| `TAIMUTC_AT_TZERO` | Difference (TAI – UTC) in seconds (i.e., total # leap seconds elapsed since 1958) as modeled by the message originator at epoch '`EPOCH_TZERO`'. | `36` | O |
| `NEXT_LEAP_EPOCH` | Epoch of next leap second, specified as an absolute time tag. | `2017-01-01T00:00:00` | O |
| `NEXT_LEAP_TAIMUTC` | Difference (TAI – UTC) in seconds (i.e., total number of leap seconds elapsed since 1958) incorporated by the message originator at epoch '`NEXT_LEAP_EPOCH`'. This keyword should be provided if `NEXT_LEAP_EPOCH` is supplied. | `37` | O |
| `META_STOP` | End of the Metadata section. | n/a | M |

### 5.3.4 ACM DATA: GENERAL REQUIREMENTS

**NOTE** – The following requirements apply to all ACM sections and content.

5.3.4.1 The order of occurrence of ACM keywords shall be fixed as listed in the keyword value tables in the ACM section descriptions.

5.3.4.2 Within each section, keywords labeled as mandatory (M) in the corresponding tables denotes those keywords that must be included in this section if that particular section is included. Keywords labeled as optional (O) may not be provided in the message. Keywords are labeled as conditional (C) if they are mandatory if this data block is included and certain conditions are met.

5.3.4.3 All time tags may be specified by either a (signed) double precision relative time (e.g., 20157.26) with respect to the specified epoch time (`EPOCH_TZERO`) or as an absolute time (e.g., 2018-11-13T11:13:20.5Z in CCSDS Time String A or B format, as specified in 6.8.9).

5.3.4.4 Duplicate time tags shall not be permitted.

5.3.4.5 Within an ACM data block, all time tags must adhere to either relative time or absolute time for the entirety of that data block. An ACM must not mix relative and absolute time within the same data block.

5.3.4.6 Time tags of information within ordered sequences of ACM sections may be separated by uniform or non-uniform step size(s).

5.3.4.7 Time tags of one ACM section may or may not match those of another ACM section.

### 5.3.5 ACM DATA: ATTITUDE STATE TIME HISTORY

5.3.5.1 Table 5-4 provides an overview of the ACM attitude state time history section. Only those keywords shown in table 5-4 shall be used in the ACM attitude state time history data specification.

5.3.5.2 Each attitude state time history data block must begin with keyword `ATT_START` and end with keyword `ATT_STOP`.

5.3.5.3 Each keyword shall appear on a line by itself.

5.3.5.4 Multiple attitude state time history blocks may appear in an ACM if they are delimited by separate `ATT_START` and `ATT_STOP` keywords.

5.3.5.5 Each attitude state data block should differ from all others in at least one of the following respects:

a) the selected attitude state set (`ATT_TYPE`) is unique;
b) the attitude state time history is based upon a unique attitude determination solution (`ATT_BASIS_ID`);
c) the transformations frames are unique (`REF_FRAME_A`, `REF_FRAME_B`);
d) the data interval timespan is unique (i.e., has no overlap with any other data interval).

5.3.5.6 If the user includes attitude states at key mission event times, it is recommended that those mission event states be annotated as such by a descriptive comment line(s) immediately following the `ATT_START` keyword.

5.3.5.7 Time tags of consecutive attitude states within the ordered sequence may be separated by uniform or non-uniform step size(s).

5.3.5.8 Attitude state time tags may or may not match those of maneuver or covariance time histories.

5.3.5.9 At least one space character must be used to separate the items in each attitude data line.

**Table 5-4: ACM Data: Attitude State Time History**

| Keyword | Description | Units | Examples of Values | M/O/C |
| :--- | :--- | :--- | :--- | :--- |
| `ATT_START` | Start of an attitude state time history section | n/a | n/a | M |
| `COMMENT` | Comments allowed only immediately after the `ATT_START` keyword. | n/a | `This is a comment.` | O |
| `ATT_ID` | Optional alphanumeric free-text string containing the identification number for this attitude state time history. | n/a | `ATT_20160402_XYZ` | O |
| `ATT_PREV_ID` | Optional alphanumeric free-text string containing the identification number for the previous attitude time history block. <br> **NOTE** – If the message is not part of a sequence of attitude time histories or if this attitude time history is the first in a sequence of attitude time histories, then `ATT_PREV_ID` should be excluded from this message. | n/a | `ATT_20160401_XYZ` | O |
| `ATT_BASIS` | Basis of this attitude state time history data, this is a text field with the following suggested values: <br> a) 'PREDICTED'; <br> b) 'DETERMINED_GND' when estimated by post-processing attitude sensor data on the ground; <br> c) 'DETERMINED_OBC' when estimated on board using onboard sensor data; <br> d) 'SIMULATED' for future mission design or other testing purposes. | n/a | `PREDICTED` | O |
| `ATT_BASIS_ID` | Free-text field containing the identification number for the telemetry dataset, attitude determination, or simulation upon which this attitude state time history block is based. When a matching attitude determination block accompanies this attitude state time history, the `ATT_BASIS_ID` should match the corresponding `AD_ID` (see table 5-8). | n/a | `AD_1985` | O |
| `REF_FRAME_A` | Name of the reference frame that defines the starting point of the transformation. The set of allowed values is described in annex B, subsection B3. | n/a | `J2000` | M |
| `REF_FRAME_B` | Name of the reference frame that defines the end point of the transformation. The set of allowed values is described in annex B, subsection B3. | n/a | `SC_BODY_1` | M |
| `NUMBER_STATES` | Number of data states included. States to be included are attitude states and optional rate states. | n/a | `3` <br> `4` <br> `7` | M |
| `ATT_TYPE` | Type of attitude data, selected per annex B, subsection B4. Attitude data must always be listed before rate data. The units that shall be used are given in annex B, subsection B4. | n/a | `QUATERNION` <br> `EULER_ANGLES` <br> `DCM` | M |
| `EULER_ROT_SEQ` | Rotation sequence that defines the `REF_FRAME_A` to `REF_FRAME_B` transformation. The order of the transformation is from left to right, where the leftmost letter (X, Y, or Z) represents the rotation axis of the first rotation, the second letter (X, Y, or Z) represents the rotation axis of the second rotation, and the third letter (X, Y, or Z) represents the rotation axis of the third rotation. This keyword is applicable only if `ATT_TYPE` specifies the use of Euler angles. | n/a | `ZXZ` <br> `XYZ` | C |
| `RATE_TYPE` | Type of rate data, selected per annex B, subsection B4. If rate data are included, `NUMBER_STATES` must be at least 6 to include both attitude and rate data. The units that shall be used are given in annex B, subsection B4. If the value is `ANGVEL`, the reference frame used shall be `REF_FRAME_B`. | n/a | `ANGVEL` <br> `GYRO_BIAS` <br> `Q_DOT` <br> `NONE` | O |
| `<Insert attitude lines here>` | Data lines that consist of attitude data followed by rate data. (For the data units, see above [`ATT_TYPE` and `RATE_TYPE` keywords]). | | | M |
| `ATT_STOP` | End of an attitude state time history section | n/a | n/a | M |

### 5.3.6 ACM DATA: SPACE OBJECT PHYSICAL CHARACTERISTICS

5.3.6.1 Table 5-5 provides an overview of the ACM space object physical characteristics section. Only those keywords shown in table 5-5 shall be used in ACM space object physical characteristics data.

5.3.6.2 Keyword values shall be provided in the units specified in table 5-5.

5.3.6.3 Only one space object physical characteristics section shall appear in an ACM.

5.3.6.4 The space object physical characteristics Data section in the ACM shall be indicated by two keywords: `PHYS_START` and `PHYS_STOP`.

5.3.6.5 Further definition of Space Object Physical Characteristics parameters is provided in annex H, subsection [H2].

**Table 5-5: ACM Data: Space Object Physical Characteristics**

| Keyword | Description | Units | Examples of Values | M/O/C |
| :--- | :--- | :--- | :--- | :--- |
| `PHYS_START` | Start of a Space Object Physical Characteristics specification. | n/a | n/a | M |
| `COMMENT` | Comments allowed only immediately after the `PHYS_START` keyword. | n/a | `This is a comment.` | O |
| `DRAG_COEFF` | Drag coefficient. | n/a | 2 | O |
| `WET_MASS` | Space object total mass at the reference epoch '`EPOCH_TZERO`'. | kg | 750.0 | O |
| `DRY_MASS` | Space object dry mass (without propellant). | kg | 500.0 | O |
| `CP_REF_FRAME` | Coordinate system for the center of pressure vector. The set of allowed values is described in annex B, subsection B3. `CP_REF_FRAME` shall be present if `CP` is present. | n/a | `SC_BODY_1` | C |
| `CP` | Vector location of spacecraft center of pressure for determining solar pressure torque, measured from the spacecraft center of mass. The coordinate frame is defined by `CP_REF_FRAME`. `CP` contains 3 elements, one for each axis represented in `CP_REF_FRAME`. | m | `0.02 0.01 0.2` | O |
| `INERTIA_REF_FRAME` | Coordinate system for the inertia tensor. The set of allowed values is described in annex B, subsection B3. | n/a | `SC_BODY_1` | O |
| `IXX` | Moment of Inertia about the X axis of the spacecraft body frame defined by `INERTIA_REF_FRAME`. | kg\*m\*\*2 | 1000.0 | O |
| `IYY` | Moment of Inertia about the Y axis. | kg\*m\*\*2 | 800.0 | O |
| `IZZ` | Moment of Inertia about the Z axis. | kg\*m\*\*2 | 400.0 | O |
| `IXY` | Inertia Cross Product of the X & Y axes. | kg\*m\*\*2 | 20.0 | O |
| `IXZ` | Inertia Cross Product of the X & Z axes. | kg\*m\*\*2 | 40.0 | O |
| `IYZ` | Inertia Cross Product of the Y & Z axes. | kg\*m\*\*2 | 60.0 | O |
| `PHYS_STOP` | End of a Space Object Physical Characteristics specification. | n/a | n/a | M |

### 5.3.7 ACM DATA: ATTITUDE STATE COVARIANCE TIME HISTORY

5.3.7.1 Table 5-6 provides an overview of the ACM attitude state covariance time history section. Only those keywords shown in table 5-6 shall be used in ACM covariance time history data specification.

5.3.7.2 Each attitude state covariance time history data block must begin with keyword `COV_START` and end with keyword `COV_STOP`.

5.3.7.3 Multiple covariance data blocks may appear in an ACM only if they are delimited by separate `COV_START` and `COV_STOP` keywords.

5.3.7.4 Each attitude state covariance data block should differ from all others in at least one of the following respects:

a) the selected covariance composition (`COV_TYPE`);
b) the scenario basis (`COV_BASIS`);
c) the covariance time history is based upon a unique attitude determination solution or simulation (`COV_BASIS_ID`);
d) the data interval timespan is unique (i.e., has no overlap with any other data interval(s)).

5.3.7.5 Each attitude state covariance time history shall be time-ordered to be monotonically increasing.

**NOTE** – If the user includes attitude state covariances at key mission events or times, it is useful to provide times, names, and significance for such mission events in descriptive comment line(s) immediately following the `COV_START` keyword.

5.3.7.6 Values in the covariance matrix shall be only main diagonal elements provided on a single line directly following the time tag specification.

5.3.7.7 Off-diagonal elements may be defined in a user-defined block.

5.3.7.8 Values in the attitude state covariance matrix shall be expressed in the applicable reference frame specified via the ‘`COV_REF_FRAME`’ keyword.

**Table 5-6: ACM Data: Covariance Time History**

| Keyword | Description | Units | Examples of Values | M/O/C |
| :--- | :--- | :--- | :--- | :--- |
| `COV_START` | Start of a covariance time history section. | n/a | n/a | M |
| `COMMENT` | Comments allowed only immediately after the `COV_START` keyword. | n/a | `THIS is a comment.` | O |
| `COV_ID` | Optional alphanumeric free-text string containing the identification number for this attitude covariance time history block. | n/a | `COV_20160402_XYZ` | O |
| `COV_PREV_ID` | Optional alphanumeric free-text string containing the identification number for the previous covariance time history block. <br> **NOTE** – If the message is not part of a sequence of covariance time histories or if this covariance time history is the first in a sequence of covariance time histories, then `COV_PREV_ID` should be excluded from this message. | n/a | `COV_20160401_XYZ` | O |
| `COV_BASIS` | Basis of this covariance time history data, this is a text field with the following suggested values: <br> a) '`PREDICTED`'; <br> b) '`DETERMINED_GND`' when estimated by post-processing attitude sensor data on the ground; <br> c) '`DETERMINED_OBC`' when estimated on board using onboard sensor data; <br> d) '`SIMULATED`' for future mission design or other testing purposes. | n/a | `PREDICTED` | O |
| `COV_BASIS_ID`| Free-text field containing the identification number for the telemetry dataset, attitude determination, or simulation upon which this covariance time history block is based. When a matching attitude determination block accompanies this covariance time history, the `COV_BASIS_ID` should match the corresponding `AD_ID` (see table 5-8). | n/a | `AD_1985` | O |
| `COV_REF_FRAME`| Reference frame of the covariance time history. The full set of values is enumerated in annex B, subsection B3. | n/a | `SC_BODY_1` | O |
| `COV_TYPE` | Indicates covariance composition. Select from annex B, subsection B6. | n/a | `ANGLE` <br> `ANGLE_GYROBIAS` | M |
| `<Insert covariance data here>` | Covariance data lines (diagonal terms only). (For the data units, see annex B, subsection B6.) | | | M |
| `COV_STOP` | End of a covariance time history section. | n/a | n/a | M |

### 5.3.8 ACM DATA: MANEUVER SPECIFICATION

5.3.8.1 Table 5-7 provides an overview of the ACM maneuver specification section. Only those keywords shown in table 5-7 shall be used in the ACM maneuver specification.

5.3.8.2 Keyword values shall be provided in the units specified in the `Units` column of table 5-7.

5.3.8.3 Maneuver data in the ACM shall be indicated by two keywords: `MAN_START` and `MAN_STOP`.

5.3.8.4 Multiple maneuver data blocks shall appear in an ACM only when delimited by separate `MAN_START` and `MAN_STOP` keywords.

5.3.8.5 Each maneuver data block should differ from all other maneuver data blocks in at least one of the following respects:

a) the maneuver purpose (`MAN_PURPOSE`) is unique;
b) the data interval timespan is unique (i.e., has no overlap).

**Table 5-7: ACM Data: Maneuver Specification**

| Keyword | Description | Units | Examples of Values | M/O/C |
| :--- | :--- | :--- | :--- | :--- |
| `MAN_START` | Start of a maneuver data interval specification. | n/a | n/a | M |
| `COMMENT` | Comments allowed only immediately after the `MAN_START` keyword. | n/a | `This is a comment.` | O |
| `MAN_ID` | Optional alphanumeric free-text string containing the identification number for this maneuver. | n/a | `DH2018172` | O |
| `MAN_PREV_ID` | Optional alphanumeric free-text string containing the identification number for the previous maneuver block. If the message is not part of a sequence of maneuvers or if this maneuver is the first in a sequence of maneuvers, then `MAN_PREV_ID` should be excluded from this message. | n/a | `DH2018171` | O |
| `MAN_PURPOSE` | The user may specify the intention(s) of the maneuver. Multiple maneuver purposes may be provided as a comma-delimited list. While there is no CCSDS-based restriction on the value for this keyword, it is suggested to use: <br> - Attitude adjust (`ATT_ADJUST`); <br> - Momentum desaturation (`MOM_DESAT`); <br> - Pointing Request Message (`PRM_ID_xxxx`); <br> - Science objective (`SCI_OBJ`); <br> - Spin rate adjust (`SPIN_RATE_ADJUST`). | n/a | `ATT_ADJUST` | M |
| `MAN_BEGIN_TIME`| Start time of actual maneuver, measured as a relative time with respect to `EPOCH_TZERO`. | s | 100.0 | M |
| `MAN_END_TIME` | End time of actual maneuver, measured as a relative time with respect to `EPOCH_TZERO`. | s | 120.0 | C |
| `MAN_DURATION` | Length of maneuver, should only specify `MAN_END_TIME` or `MAN_DURATION`, not both. | s | 20.0 | C |
| `ACTUATOR_USED`| Specifies the type of actuator used for the maneuver. | n/a | `ATT-THRUSTER` <br> `RWA` | O |
| `TARGET_MOMENTUM`| If `MAN_PURPOSE=MOM_DESAT`, target momentum in `TARGET_MOM_FRAME`. Contains 3 elements, one for each axis. | N\*m\*s | `0 -10 0` | C |
| `TARGET_MOM_FRAME`| Reference frame of the `TARGET_MOMENTUM`. The full set of values is enumerated in annex B, subsection B3. `TARGET_MOM_FRAME` shall be present if `TARGET_MOMENTUM` is present. | n/a | `SC_BODY_1` | C |
| `TARGET_ATTITUDE`| If `MAN_PURPOSE=ATT_ADJUST`, target quaternion. Contains 4 elements in the order Q1, Q2, Q3, QC. | n/a | `0 0 0 1` | C |
| `TARGET_SPINRATE`| If `MAN_PURPOSE=SPIN_RATE_ADJUST`, target spin rate | deg/s | 0.31 | C |
| `MAN_STOP` | End maneuver data interval specification. | n/a | n/a | M |

### 5.3.9 ACM DATA: ATTITUDE DETERMINATION DATA

5.3.9.1 Table 5-8 provides an overview of the ACM attitude determination Data section. Only those keywords shown in table 5-8 shall be used in ACM attitude determination data specification.

5.3.9.2 At most, only one Attitude Determination Data section shall appear in an ACM.

5.3.9.3 Attitude determination data in the ACM shall be indicated by two keywords: `AD_START` and `AD_STOP`.

5.3.9.4 The attitude determination specification shall apply to all ACM attitude and covariance time history Data sections that are based upon 'determined' attitude solutions.

5.3.9.5 As many sensor blocks as there are sensors shall be included in the message.

5.3.9.6 Each sensor block shall be delimited by `SENSOR_START` and `SENSOR_STOP` and shall contain the necessary information, including the sensor number.

**NOTE** – The sensor number might not be sequential.

**Table 5-8: ACM Data: Attitude Determination Data**

| Keyword | Description | Units | Examples of Values | M/O/C |
| :--- | :--- | :--- | :--- | :--- |
| `AD_START` | Start of an attitude determination Data section. | n/a | n/a | M |
| `COMMENT` | Comments allowed only immediately after the `AD_START` keyword. | n/a | `This is a comment.` | O |
| `AD_ID` | Optional alphanumeric free-text string for this attitude determination. | n/a | `AD_20190101` | O |
| `AD_PREV_ID` | Optional alphanumeric free-text string containing the identification number for the previous attitude determination block. <br> **NOTE** – If the message is not part of a sequence of attitude determination blocks or if this attitude determination block is the first in a sequence of attitude determination blocks, then `AD_PREV_ID` should be excluded from this message. | n/a | `AD_20190100` | O |
| `AD_METHOD` | Type of attitude determination method used. (For further description, see annex B, subsection B5.) | n/a | `EKF` <br> `TRIAD` <br> `BATCH` | O |
| `ATTITUDE_SOURCE` | Source of attitude estimate, whether from a ground based estimator or onboard estimator. | n/a | `GND` <br> `OBC` | O |
| `NUMBER_STATES`| Number of states if `EKF`, `BATCH`, or `FILTER_SMOOTHER` is specified. | n/a | `3` <br> `6` <br> `7` | O |
| `ATTITUDE_STATES`| Type of attitude data, selected per annex B, subsection B4. Attitude states must always be listed before rate states. | n/a | `QUATERNION` | M |
| `EULER_ROT_SEQ`| Rotation sequence that defines the `REF_FRAME_A` to `REF_FRAME_B` transformation. The order of the transformation is from left to right, where the leftmost letter (X, Y, or Z) represents the rotation axis of the first rotation, the second letter (X, Y, or Z) represents the rotation axis of the second rotation, and the third letter (X, Y, or Z) represents the rotation axis of the third rotation. This keyword is applicable only if `ATTITUDE_STATES` specifies the use of Euler angles. | n/a | `ZXZ` <br> `XYZ` | C |
| `COV_TYPE` | Type of attitude error state included in the estimator. Select from annex B, subsection B6. | n/a | `ANGLE` <br> `ANGLE_GYROBIAS` <br> `NONE` | O |
| `REF_FRAME_A` | Name of the reference frame that defines the starting point of the transformation described by the attitude state in the estimator. The set of allowed values is described in annex B, subsection B3. | n/a | `J2000` | M |
| `REF_FRAME_B` | Name of the reference frame that defines the ending point of the transformation described by the attitude state in the estimator. The set of allowed values is described in annex B, subsection B3. | n/a | `SC_BODY_1` | M |
| `RATE_STATES` | Type of rate state included in the estimator. If rate states are included, `NUMBER_STATES` must be at least 6 to include both attitude states and rate states. | n/a | `ANGVEL` <br> `GYRO_BIAS` | O |
| `SIGMA_U` | Rate random walk if `RATE_STATES=GYRO_BIAS`. | deg/s\*\*1.5 | 3.7e-7 | O |
| `SIGMA_V` | Angle random walk if `RATE_STATES=GYRO_BIAS`. | deg/s\*\*0.5 | 1.3e-5 | O |
| `RATE_PROCESS_NOISE_STD_DEV`| Process noise standard deviation if `RATE_STATES=ANG_VEL`. | deg/s\*\*1.5 | 5.1E-06 | O |
| `SENSOR_START` | Start of Sensor data section. Multiple sections can be included. | n/a | n/a | O |
| `SENSOR_NUMBER`| Sensor number. Multiple sensors may be included, with each having a unique, ascending number. | n/a | `1` <br> `2` <br> `3` | O |
| `SENSOR_USED` | Type of sensor used in estimation. | n/a | `AST` <br> `DSS` <br> `GYRO` | O |
| `NUMBER_SENSOR_NOISE_COVARIANCE` | Number of noise elements for sensor. For example, noise along horizontal and vertical directions of a CCD, or noise along x, y, and z axes of a sensor. | n/a | `2` <br> `3` | O |
| `SENSOR_NOISE_STDDEV` | Standard deviation of sensor noise, size will be the same as `NUMBER_SENSOR_NOISE_COVARIANCE`. | deg | `0.0097 0.0097` | O |
| `SENSOR_FREQUENCY` | Frequency of sensor data. | Hz | 5 | O |
| `SENSOR_STOP` | End of sensor data section. | n/a | n/a | O |
| `AD_STOP` | End of an attitude determination Data section. | n/a | n/a | M |

### 5.3.10 ACM DATA: USER-DEFINED PARAMETERS

5.3.10.1 A single section of User-Defined Parameters may be provided if necessary.

**NOTES**

1.  In principle, this provides flexibility but also introduces complexity, non-standardization, potential ambiguity, and potential processing errors.
2.  Table 5-9 provides an overview of the ACM User-Defined Data section.

5.3.10.2 If a User-Defined Parameters section is used, the keywords and their meanings must be described in an ICD.

5.3.10.3 User-Defined Parameters, if included, should be used as sparingly as possible; their use is not encouraged.

5.3.10.4 At most, only one User-Defined Parameters section shall appear in an ACM.

5.3.10.5 Each User-Defined parameter line may be preceded by one or more comment lines.

5.3.10.6 Only those keywords shown in table 5-9 shall be used in ACM User-Defined data specification.

**Table 5-9: ACM Data: User-Defined Parameters**

| Keyword | Description | Units | Examples of Values | M/O/C |
| :--- | :--- | :--- | :--- | :--- |
| `USER_START` | Start of a User-Defined Parameters data block. | n/a | n/a | M |
| `COMMENT` | Comments allowed only immediately after the `USER_START` keyword. (See 6.10 for formatting rules.) | n/a | `This is a comment.` | O |
| `USER_DEFINED_x` | User-Defined Parameter, where 'x' is replaced by a variable length user specified character string. Any number of User-Defined Parameters may be included, if necessary to provide essential information that cannot be conveyed in standard ACM keywords or in COMMENT statements. | User-defined unit (as appropriate for variable) | `FINE_GUIDANCE_SENSOR` | M |
| `USER_STOP` | End of a User-Defined Parameters data block. | n/a | n/a | M |


# 6 ATTITUDE DATA MESSAGES KVN SYNTAX

## 6.1 OVERVIEW

This section details the syntactic requirements for attitude messages.

## 6.2 GENERAL

All APM, AEM, and ACM messages shall observe the syntax described in subsections 6.3 through 6.10.

## 6.3 APM

The APM shall be a plain text file, using keyword descriptions given in 3.2.2 through 3.2.4.

## 6.4 AEM

The AEM shall be a plain text file, using the keyword descriptions given in 4.2.2 through 4.2.4.

## 6.5 ACM

The ACM shall be a plain text file, using the keywords given in 5.3.2 through 5.3.10.

## 6.6 LINES

6.6.1 Each APM or AEM line must not exceed 254 ASCII characters and spaces, excluding line termination character(s).

6.6.2 ACM lines may be of arbitrary length. If exchange between the two parties requires a maximum line length, that limit should be negotiated and agreed.

6.6.3 Only printable ASCII characters and blanks shall be used. Control characters (such as TAB, etc.) shall not be used, except as indicated below for the termination of lines.

6.6.4 Blank lines may be used at any position within the file.

6.6.5 Comment lines shall be optional. (See 6.10.2 for details regarding the placement of comment lines in an APM, 6.10.3 for details regarding the placement of comment lines in an AEM, and 6.10.4 for details regarding the placement of comment lines in an ACM.)

6.6.6 APM, AEM, and ACM lines shall be terminated by a single Carriage Return or a single Line Feed, or a Carriage Return/Line Feed pair or a Line Feed/Carriage Return pair.

## 6.7 KEYWORDS

6.7.1 All header, metadata, and data lines, with exceptions identified in the note below, shall use ‘keyword = value’ notation, abbreviated as KVN.

6.7.2 Only a single ‘keyword = value’ assignment shall be made on a line.

6.7.3 Keywords must be uppercase and must not contain blanks.

6.7.4 Any white space immediately preceding or following the keyword shall not be significant.

6.7.5 Any white space immediately preceding or following the ‘equals’ sign shall not be significant.

6.7.6 Any white space immediately preceding the end of line shall not be significant.

6.7.7 Any white space immediately preceding or following the units shall not be significant.

6.7.8 The order of occurrence of mandatory and optional KVN assignments shall be fixed as shown in tables 3-1, 3-2, and 3-3 for the APM, tables 4-2 and 4-3 for the AEM, and tables 5-2 through 5-9 for the ACM.

**NOTE** – The keywords `COMMENT`, section delimiters `*_START` and `*_STOP`, AEM data lines, and some ACM data lines are exceptions to the KVN syntax.

## 6.8 VALUES

6.8.1 Angle measurements shall be given in degrees, with values between -360 and 360 degrees. If agencies wish to exchange using radians, this must be specified in an ICD because it is nominally outside the standard.

6.8.2 Blanks shall not appear within numeric values and time values.

6.8.3 Integer values shall consist of a sequence of decimal digits with an optional leading sign (‘+’ or ‘-’). If the sign is omitted, ‘+’ shall be assumed. Leading zeroes may be used. The range of values that may be expressed as an integer is:

` -2,147,483,648 ≤ x ≤ +2,147,483,647 (i.e., −2³¹ ≤ x ≤ 2³¹−1). `

**NOTE** – The commas in the range of values above are thousands separators and are used only for readability. They should not appear in an actual message.

6.8.4 Non-integer numeric values may be expressed in either fixed-point or floating-point notation. Both representations may be used within an APM, AEM, or ACM.

6.8.4.1 Non-integer numeric values expressed in fixed-point notation shall consist of a sequence of decimal digits separated by a period as a decimal point indicator, with an optional leading sign (‘+’ or ‘-’). If the sign is omitted, ‘+’ shall be assumed. Leading and trailing zeros may be used. At least 1 digit shall appear before and after a decimal point. The number of digits shall be 16 or fewer.

6.8.4.2 Non-integer numeric values expressed in floating-point notation shall consist of a sign, a mantissa, an alphabetic character indicating the division between the mantissa and exponent, and an exponent, constructed according to the following rules:

*   The sign may be ‘+’ or ‘-’. If the sign is omitted, ‘+’ shall be assumed.
*   The mantissa must be a string of no more than 16 decimal digits with a decimal point ‘.’ in the second position of the ASCII string, separating the integer portion of the mantissa from the fractional part of the mantissa.
*   The character used to denote exponentiation shall be ‘E’ or ‘e’. If the character indicating the exponent and the following exponent are omitted, an exponent value of zero shall be assumed (essentially yielding a fixed-point value).
*   The exponent must be an integer, and may have either a ‘+’ or ‘-’ sign (if the sign is omitted, then ‘+’ shall be assumed).

The maximum positive floating-point value is approximately 1.798E+308, with precision of 16 significant decimal digits. The minimum positive floating-point value is approximately 4.94E-324, with precision of 16 significant decimal digits.

6.8.5 These specifications for integer, fixed-point, and floating-point values conform to the XML specifications for the data types four-byte integer ‘xsd:int’, ‘xsd:decimal’ and ‘xsd:double’ respectively. The specifications for floating-point values conform to the IEEE double precision type (reference [5]). Floating-point numbers in IEEE extended-single or IEEE extended-double precision may be represented, but do require an ICD between participating agencies because of their implementation-specific attributes (reference [5]). It should be noted that NaN, +Inf, -Inf, and -0 are not supported values.

6.8.6 Text value fields must be constructed using only all uppercase or all lowercase.

6.8.7 A non-empty value field must be specified for each keyword provided, except as indicated in the note under 6.7.

6.8.8 In value fields that are text, an underscore shall be equivalent to a single blank. Individual blanks between non-blank characters shall be retained (shall be significant) but multiple blanks shall be equivalent to a single blank.

6.8.9 In value fields that represent a time tag or epoch, one of the following two formats shall be used:

`YYYY-MM-DDThh:mm:ss[.d→d][Z]`,

or

`YYYY-DDDThh:mm:ss[.d→d][Z]`.

where ‘YYYY’ is the year; ‘MM’ is the two-digit month; ‘DD’ is the two-digit day; ‘DDD’ is the three-digit day of year; ‘T’ is constant; ‘hh:mm:ss[.d→d]’ is the time in hours, minutes, seconds, and optional fractional seconds; and ‘Z’ is an optional time code terminator (the only permitted value is ‘Z’ for Zulu, i.e., UTC). As many ‘d’ characters to the right of the period may be used to obtain the required precision, up to the maximum allowed for a fixed-point number. All fields shall have leading zeros. (See reference [3], ASCII Time Code A and B.)

**NOTE** – During a leap second introduction, the value of the two-digit integer seconds (ss) field will be ‘60’ as specified on page 3-6 of reference [3].

6.8.10 A number of ACM keywords may be set to values containing more than one number. Examples include CP (see table 5-5) and `TARGET_MOMENTUM` (see table 5-7). Such vectors shall be space-delimited and provided serially on a single line following the equals ‘=’ sign, adhering to the requirements for numeric values provided in the previous sections.

## 6.9 UNITS

### 6.9.1 APM RESTRICTIONS

For clarity, units may be included as ASCII text after a value, but they must exactly match the units specified in table 3-3 (including case). If units are displayed, then:

a) there must be at least one blank character between the value and the units text;
b) the units must be enclosed within square brackets (e.g., ‘[deg]’);
c) multiplication of units shall be denoted with a single asterisk ‘\*’ (e.g., ‘[N\*m]’);
d) division of units shall be denoted with a forward slash ‘/’ (e.g., ‘[deg/s]’);
e) exponents of units shall be denoted with a double asterisk ‘\*\*’ (e.g., ‘[kg\*m\*\*2]’).

### 6.9.2 AEM RESTRICTIONS

Units shall not be displayed; the applicable units are determined by the value set for the `ATTITUDE_TYPE` keyword. (See 4.2.4.6.)

### 6.9.3 ACM RESTRICTIONS

6.9.3.1 Apart from attitude state and covariance, units of ACM keyword values shall correspond to the normative ‘Units’ column of the accompanying Keyword Value Tables (i.e., tables 5-4 through 5-9) for each section definition.

6.9.3.2 The units of attitude state time history data lines, when present, shall adhere to the specified units for attitude states as provided in annex B, subsection B4.

6.9.3.3 The units of covariance time history data lines, when present, shall adhere to the specified units for covariance data as provided in annex B, subsection B6.

6.9.3.4 For ACM keywords that are not used to convey multipartite attitude or rate state, covariance, or maneuver data lines, for documentation purposes and clarity only, units may be included as ASCII text after a value in the ACM. If units are displayed, then:

a) there must be at least one blank character between the value and the units text;
b) the units must be enclosed within square brackets (e.g., ‘[deg]’);
c) combinations of units shall adhere to requirements listed in 1.3.

6.9.3.5 Some of the items in the applicable tables are dimensionless. The table shows a unit value of ‘n/a’, which in this case means that there is no applicable units designator for these items (e.g., for `DRAG_COEFF`) and no units displayed.

## 6.10 COMMENTS

### 6.10.1 GENERAL

6.10.1.1 All comment lines shall begin with the ‘COMMENT’ keyword followed by at least one space. This keyword must appear on every comment line, not just the first such line. The remainder of the line shall be the comment value. White space shall be retained (shall be significant) in comment values.

6.10.1.2 Comments may be used to provide provenance information or to help describe dynamical events or other pertinent information associated with the data. This additional information is intended to aid in consistency checks and elaboration as needed, but shall not be required for successful processing of a file.

6.10.1.3 If accompanying descriptive text designed to clarify and/or remove ambiguities in provided ADM data does not fit well into the single comment line paradigm, it is recommended that the APM, AEM, or ACM producer convey key elements of that information in comments and use an ICD to provide further details.

6.10.1.4 Comments may be in any case desired by the user.

### 6.10.2 APM SPECIFIC

Comments are optional and may appear only at the beginning of the APM Header and APM Metadata sections, as shown in tables 3-1 and 3-2. In the APM Data section, comments shall appear only at the beginning of a logical block. Comments must not appear between the components of any logical block in the APM Data section. The logical blocks in the APM Data section are indicated in table 3-3.

### 6.10.3 AEM SPECIFIC

Comments are optional and may appear only after the specification of the keyword `CCSDS_AEM_VERS`, at the beginning of Metadata sections (only after `META_START` and before `OBJECT_NAME`), and immediately following the `DATA_START` keyword. Comments must not appear between attitude ephemeris data lines, nor after the `DATA_STOP` keyword. (See tables 4-2 and 4-3.)

### 6.10.4 ACM SPECIFIC

Comments are optional and may appear at the beginning of the ACM Header, ACM Metadata section, and after the start of each included ACM Data block. (See tables 5-2 through 5-9.)


# 7 CONSTRUCTING AN ADM/XML INSTANCE

## 7.1 OVERVIEW

This section provides detailed instructions for the user on how to create an XML message based on one of the KVN-formatted messages described in sections 3 through 5 of this document. This section applies only to the XML representation of the ADM messages.

The ADM/XML schemas are available on the SANA Web site. SANA is the registrar for the protocol registries created under CCSDS. The ADM/XML schemas explicitly define the permitted data elements and values acceptable for the XML versions of the ADM messages. The location of the ADM/XML schemas is:

*   https://sanaregistry.org/r/ndmxml_unqualified/\[schemaName] for schemas with the attribute ‘elementFormDefault="unqualified"’
*   https://sanaregistry.org/r/ndmxml_qualified/\[schemaName] for schemas with the attribute 'elementFormDefault="qualified"'
    *   APM: https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-4.0.0-apm-2.0.xsd
    *   APM: https://sanaregistry.org/r/ndmxml_qualified/ndmxml-4.0.0-apm-2.0.xsd
    *   AEM: https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-4.0.0-aem-2.0.xsd
    *   AEM: https://sanaregistry.org/r/ndmxml_qualified/ndmxml-4.0.0-aem-2.0.xsd
    *   ACM: https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-4.0.0-acm-2.0.xsd
    *   ACM: https://sanaregistry.org/r/ndmxml_qualified/ndmxml-4.0.0-acm-2.0.xsd

When possible, these schemas use simple types and complex types used by the constituent schemas that make up Navigation Data Messages (see reference [4]).

## 7.2 ADM/XML BASIC STRUCTURE

7.2.1 Each ADM shall consist of a `<header>` and a `<body>`.

7.2.2 The `<body>` shall consist of one or more `<segment>` constructs (one for the APM, one or more for the AEM, one for the ACM).

7.2.3 Each `<segment>` shall consist of one `<metadata>`/`<data>` pair, as shown in figure 7-1.

**NOTE** – An AEM may have more than one segment, in which case the metadata/data pair is repeated in each segment.

```xml
<header>
</header>
<body>
  <segment>
    <metadata>
    </metadata>
    <data>
    </data>
  </segment>
</body>
```

**Figure 7-1: ADM/XML Basic Structure**

## 7.3 ADM/XML TAGS

7.3.1 An ADM/XML tag shall be all uppercase if it corresponds directly to a KVN keyword from the Header, Metadata, or Data sections.

7.3.2 The '`CCSDS_A*M_VERS`' keyword and value shall appear as XML attributes of the root element rather than as XML elements. This is an exception for when there is not a strict correspondence between keywords in the KVN and tags in the XML implementations, specifically, the ‘`CCSDS_A*M_VERS`’ keywords from the Headers for the APM, AEM, and ACM respectively.

7.3.3 ADM/XML tags related to the XML message structure (i.e., that do not correspond directly to a KVN keyword) shall be in ‘lowerCamelCase’ (e.g., `<header>`, `<segment>`, `<metadata>`, `<attitudeStateType>`, etc.).

## 7.4 CONSTRUCTING AN ADM/XML INSTANCE

### 7.4.1 XML VERSION

7.4.1.1 The first line in the instantiation shall specify the XML version:

```xml
<?xml version="1.0" encoding="UTF-8"?>
```

7.4.1.2 This line must appear on the first line of each instantiation, exactly as shown.

### 7.4.2 BEGINNING THE INSTANTIATION: ROOT ELEMENT TAG

7.4.2.1 Each instantiation shall have a ‘root element tag’ that identifies the message type and other information such as where to find the applicable schema, required attributes, etc.

7.4.2.2 The root element tag in an ADM/XML instantiation shall be one of those listed in table 7-1.

**Table 7-1: ADM/XML Root Element Tags**

| Root Element Tag | Message Type |
| :--- | :--- |
| `<apm></apm>` | Attitude Parameter Message |
| `<aem></aem>` | Attitude Ephemeris Message |
| `<acm></acm>` | Attitude Comprehensive Message |

7.4.2.3 The XML Schema Instance namespace attribute must appear in the root element tag of all ADM/XML instantiations, exactly as shown:

`xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"`

7.4.2.5 If it is desired to validate an instantiation against the CCSDS Web-based schema, the `xsi:noNamespaceSchemaLocation` attribute must be coded as a single string of non-blank characters, with no line breaks, exactly as shown:

`xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-4.0.0-master-4.0.xsd"`

`xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_qualified/ndmxml-4.0.0-master-4.0.xsd"`

**NOTE** – The length of the value associated with the `xsi:noNamespaceSchemaLocation` attribute can cause the string to wrap to a new line; however, the string itself contains no breaks.

7.4.2.6 For use in a local operations environment, the schema set may be downloaded from the SANA website to a local server that meets local requirements for operations robustness.

7.4.2.7 If a local version is used, the value associated with the `xsi:noNamespaceSchemaLocation` attribute must be changed to a URL that is accessible to the local server.

7.4.2.8 Two attributes shall appear in the root element tag of an ADM/XML single message instantiation, specifically, the `CCSDS_xxx_VERS` keyword that is also part of the standard KVN header, and the Blue Book version number. The final attributes of the root element tag shall be ‘id’ and ‘version’.

7.4.2.9 The `CCSDS_xxx_VERS` keyword shall be supplied via the ‘id’ attribute of the root element tag. The ‘id’ attribute shall be ‘id="`CCSDS_xxx_VERS`"’, where xxx = AEM, APM, or ACM.

7.4.2.10 The version number of the Blue Book to which the schema applies shall be supplied via the ‘version’ attribute. The ‘version’ attribute shall be ‘version="2.0"’.

**NOTE** – The following example root element tags for an APM instantiation combine all the directions in the preceding several subsections:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<apm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-4.0.0-master-4.0.xsd"
   id="CCSDS_APM_VERS" version="2.0">
```

and

```xml
<?xml version="1.0" encoding="UTF-8"?>
<apm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_qualified/ndmxml-4.0.0-master-4.0.xsd"
   id="CCSDS_APM_VERS" version="2.0">
```

### 7.4.3 THE STANDARD ADM/XML HEADER SECTION

7.4.3.1 The ADMs (APM, AEM, ACM) shall share a standard header format, with tags `<header>` and `</header>` (see reference [4]).

7.4.3.2 Immediately following the `<header>` tag the message may have any number of `<COMMENT>` elements.

7.4.3.3 The standard ADM header may contain the `<CLASSIFICATION>` element.

7.4.3.4 The standard ADM header shall contain the `<CREATION_DATE>` and `<ORIGINATOR>` elements.

7.4.3.5 The standard ADM header may contain the `<MESSAGE_ID>` element.

**NOTE** – The rules for these keywords are specified in tables 3-1, 4-2, and 5-2. An example `<header>` section is shown immediately below:

```xml
<header>
  <COMMENT>This is the common ADM/XML Header.</COMMENT>
  <COMMENT>I can put as many comments here as I want,</COMMENT>
  <COMMENT>including none.</COMMENT>
  <CLASSIFICATION>NOT CLASSIFIED</CLASSIFICATION>
  <CREATION_DATE>2010-03-12T22:31:12.000</CREATION_DATE>
  <ORIGINATOR>AGENCYX</ORIGINATOR>
  <MESSAGE_ID>AGENCYX-1234</MESSAGE_ID>
</header>
```

### 7.4.4 THE ADM/XML BODY SECTION

7.4.4.1 After coding the `<header>`, the instantiation must include a `<body>` section.

7.4.4.2 Inside the `<body>` section must appear at least one `<segment>` section, depending on the particular ADM (APM, AEM, ACM).

7.4.4.3 Each `<segment>` must be made up of one `<metadata>` section and one `<data>` section.

### 7.4.5 THE ADM/XML METADATA SECTION

7.4.5.1 All ADMs must have at least one Metadata section.

7.4.5.2 The Metadata section shall be delimited by the `<metadata>` element.

7.4.5.3 Immediately following the `<metadata>` tag, the message may have any number of `<COMMENT></COMMENT>` tag pairs.

**NOTE** – The `<COMMENT></COMMENT>` placement is regulated by the XML schema. Standard XML comments, that is, of the form `<!-- comment content -->` may be placed anywhere in the Metadata section because they are ignored by the XML schema validator.

7.4.5.4 Between the `<metadata>` and `</metadata>` tags, the keywords shall be the same as those in the Metadata sections in sections 3 through 5 of this document, with exceptions as noted in the subsections that discuss creating instantiations of the specific messages.

### 7.4.6 THE ADM/XML DATA SECTION

7.4.6.1 All ADMs must have at least one Data section.

7.4.6.2 The Data Section shall follow the Metadata section and shall be delimited by the `<data>` element.

7.4.6.3 Immediately following the `<data>` tag, the message may have any number of `<COMMENT></COMMENT>` tag pairs.

**NOTE** – The `<COMMENT></COMMENT>` placement is regulated by the XML schema. Standard XML comments, that is, of the form `<!-- comment content -->` may be placed anywhere in the Data section because they are ignored by the XML schema validator.

7.4.6.4 Between the `<data>` and `</data>` tags, the keywords shall be the same as those in the Data sections in sections 3 through 5 of this document, with exceptions as noted in the subsections that discuss creating instantiations of the specific messages.

## 7.5 CREATING AN APM INSTANTIATION

7.5.1 An APM instantiation shall be delimited by the `<apm></apm>` root element tags using the standard attributes documented in 7.4.1 through 7.4.2.

**NOTE** – Annex G provides some example APM instantiations.

7.5.2 The final attributes of the `<apm>` tag shall be ‘id’ and ‘version’; the order in which these attributes are specified is not significant.

7.5.3 The ‘id’ attribute shall be ‘id="`CCSDS_APM_VERS`"’.

7.5.4 The ‘version’ attribute for the version of the APM described in section 3 shall be ‘version="2.0"’.

7.5.5 The standard ADM/XML header shall follow the `<apm>` tag (see 7.4.3).

7.5.6 The APM `<body>` shall consist of a single `<segment>`.

7.5.7 The `<segment>` shall consist of a `<metadata>` section and a `<data>` section.

7.5.8 The keywords in the `<metadata>` and `<data>` sections shall be those specified in 3.2. The rules for including any of the keyword tags in the APM/XML are the same as those specified for the APM/KVN.

7.5.9 Tags for keywords shall be all uppercase, as in 3.2.

7.5.10 Several of the APM/XML keywords may have a unit attribute, if desired by the APM producer, as illustrated in the following table:

**Table 7-2: APM XML Units**

| Keyword | Units | Example |
| :--- | :--- | :--- |
| `Q1_DOT` | 1/s | `<Q1_DOT units="1/s">numeric-value</Q1_DOT>` |
| `Q2_DOT` | 1/s | `<Q2_DOT units="1/s">numeric-value</Q2_DOT>` |
| `Q3_DOT` | 1/s | `<Q3_DOT units="1/s">numeric-value</Q3_DOT>` |
| `QC_DOT` | 1/s | `<QC_DOT units="1/s">numeric-value</QC_DOT>` |
| `ANGLE_1` | deg | `<ANGLE_1 units="deg">numeric-value</ANGLE_1>` |
| `ANGLE_2` | deg | `<ANGLE_2 units="deg">numeric-value</ANGLE_2>` |
| `ANGLE_3` | deg | `<ANGLE_3 units="deg">numeric-value</ANGLE_3>` |
| `ANGLE_1_DOT` | deg/s | `<ANGLE_1_DOT units="deg/s">numeric-value</ANGLE_1_DOT>` |
| `ANGLE_2_DOT` | deg/s | `<ANGLE_2_DOT units="deg/s">numeric-value</ANGLE_2_DOT>` |
| `ANGLE_3_DOT` | deg/s | `<ANGLE_3_DOT units="deg/s">numeric-value</ANGLE_3_DOT>` |
| `ANGVEL_X` | deg/s | `<ANGVEL_X units="deg/s">numeric-value</ANGVEL_X>` |
| `ANGVEL_Y` | deg/s | `<ANGVEL_Y units="deg/s">numeric-value</ANGVEL_Y>` |
| `ANGVEL_Z` | deg/s | `<ANGVEL_Z units="deg/s">numeric-value</ANGVEL_Z>` |
| `SPIN_ALPHA` | deg | `<SPIN_ALPHA units="deg">numeric-value</SPIN_ALPHA>` |
| `SPIN_DELTA` | deg | `<SPIN_DELTA units="deg">numeric-value</SPIN_DELTA>` |
| `SPIN_ANGLE` | deg | `<SPIN_ANGLE units="deg">numeric-value</SPIN_ANGLE>` |
| `SPIN_ANGLE_VEL` | deg/s | `<SPIN_ANGLE_VEL units="deg/s">numeric-value</SPIN_ANGLE_VEL>` |
| `NUTATION` | deg | `<NUTATION units="deg">numeric-value</NUTATION>` |
| `NUTATION_PER` | s | `<NUTATION_PER units="s">numeric-value</NUTATION_PER>` |
| `NUTATION_PHASE`| deg | `<NUTATION_PHASE units="deg">numeric-value</NUTATION_PHASE>` |
| `MOMENTUM_ALPHA`| deg | `<MOMENTUM_ALPHA units="deg">numeric-value</MOMENTUM_ALPHA>` |
| `MOMENTUM_DELTA`| deg | `<MOMENTUM_DELTA units="deg">numeric-value</MOMENTUM_DELTA>` |
| `NUTATION_VEL` | deg/s | `<NUTATION_VEL units="deg/s">numeric-value</NUTATION_VEL>` |
| `IXX` | kg\*m\*\*2 | `<IXX units="kg*m**2">numeric-value</IXX>` |
| `IYY` | kg\*m\*\*2 | `<IYY units="kg*m**2">numeric-value</IYY>` |
| `IZZ` | kg\*m\*\*2 | `<IZZ units="kg*m**2">numeric-value</IZZ>` |
| `IXY` | kg\*m\*\*2 | `<IXY units="kg*m**2">numeric-value</IXY>` |
| `IXZ` | kg\*m\*\*2 | `<IXZ units="kg*m**2">numeric-value</IXZ>` |
| `IYZ` | kg\*m\*\*2 | `<IYZ units="kg*m**2">numeric-value</IYZ>` |
| `MAN_DURATION` | s | `<MAN_DURATION units="s">numeric-value</MAN_DURATION>` |
| `MAN_TOR_X` | N\*m | `<MAN_TOR_X units="N*m">numeric-value</MAN_TOR_X>` |
| `MAN_TOR_Y` | N\*m | `<MAN_TOR_Y units="N*m">numeric-value</MAN_TOR_Y>` |
| `MAN_TOR_Z` | N\*m | `<MAN_TOR_Z units="N*m">numeric-value</MAN_TOR_Z>` |
| `MAN_DELTA_MASS`| kg | `<MAN_DELTA_MASS units="kg">numeric-value</MAN_DELTA_MASS>` |

### 7.5.11 SPECIAL TAGS IN THE APM/XML BODY

**NOTE** – In addition to the APM keywords specified in section 3, there are several special tags associated with the APM body as described in the next few subsections. The information content in the APM is separated into constructs referred to as ‘logical blocks’. Special tags in the APM are used to encapsulate the information in the logical blocks of the APM.

7.5.11.1 The APM/XML tags used to delimit the logical blocks of the APM shall be drawn from the following table:

**Table 7-3: Special Tags in the APM/XML Body**

| APM Logical Block | Associated APM/XML Tag |
| :--- | :--- |
| Attitude Quaternion | `<quaternionState>` <br> `<quaternion>` <br> `<quaternionDot>` <br> The `<quaternionState>` consists of the `<quaternion>` tag that contains the components of the quaternion itself, and the `<quaternionDot>` tag that contains the rate of change of the quaternion components |
| Euler Angle Elements | `<eulerAngleState>` |
| Angular Velocity Vector | `<angularVelocity>` |
| Spin | `<spin>` |
| Inertia | `<inertia>` |
| Maneuver Parameters | `<maneuverParameters>` |

7.5.11.2 Between the begin tag and end tag (e.g., between `<eulerAngleState>` and `</eulerAngleState>`), the user shall place the keywords required by the specific logical block as specified in section 3.

### 7.5.12 DISCUSSION

This non-normative subsection discusses and provides examples of the use of quaternion tags in the APM.

The XML representations of quaternions in the ADM constituent messages share a common quaternion definition. The following examples are meant to illustrate the standard for representing quaternions in the APM.

Here is an example APM quaternion construct:

```xml
<quaternionState>
  <REF_FRAME_A>ICRF</REF_FRAME_A>
  <REF_FRAME_B>ICRF</REF_FRAME_B>
  <quaternion>
    <Q1>0.00005</Q1>
    <Q2>0.87543</Q2>
    <Q3>0.40949</Q3>
    <QC>0.25678</QC>
  </quaternion>
</quaternionState>
```

Here is an example APM quaternion construct with the optional derivative:

```xml
<quaternionState>
  <REF_FRAME_A>ICRF</REF_FRAME_A>
  <REF_FRAME_B>ICRF</REF_FRAME_B>
  <quaternion>
    <Q1>0.00005</Q1>
    <Q2>0.87543</Q2>
    <Q3>0.40949</Q3>
    <QC>0.25678</QC>
  </quaternion>
  <quaternionDot>
    <Q1_DOT>0.002</Q1_DOT>
    <Q2_DOT>0.003</Q2_DOT>
    <Q3_DOT>0.004</Q3_DOT>
    <QC_DOT>0.001</QC_DOT>
  </quaternionDot>
</quaternionState>
```

The Spin Block is not required, but if it is present, there are three different ways in which to encode the last three keywords in the APM Spin Block. The first uses the three keywords `NUTATION`, `NUTATION_PER`, and `NUTATION_PHASE`. The second uses the three keywords `MOMENTUM_ALPHA`, `MOMENTUM_DELTA`, and `NUTATION_VEL`. Finally, in the third ‘degenerate’ encoding, neither of these two sets of three keywords is included. The APM schema enforces this structure.

```xml
<spin>
  <COMMENT>Round and round and round it goes...</COMMENT>
  <REF_FRAME_A>J2000</REF_FRAME_A>
  <REF_FRAME_B>SC_BODY_1</REF_FRAME_B>
  <SPIN_ALPHA units="deg">1.0</SPIN_ALPHA>
  <SPIN_DELTA units="deg">2.0</SPIN_DELTA>
  <SPIN_ANGLE units="deg">3.0</SPIN_ANGLE>
  <SPIN_ANGLE_VEL units="deg/s">10.0</SPIN_ANGLE_VEL>
  <NUTATION units="deg">2.0</NUTATION>
  <NUTATION_PER units="s">30.5</NUTATION_PER>
  <NUTATION_PHASE units="deg">92.7</NUTATION_PHASE>
</spin>
```

```xml
<spin>
  <COMMENT>Round and round and round it goes...</COMMENT>
  <REF_FRAME_A>J2000</REF_FRAME_A>
  <REF_FRAME_B>SC_BODY_1</REF_FRAME_B>
  <SPIN_ALPHA units="deg">1.0</SPIN_ALPHA>
  <SPIN_DELTA units="deg">2.0</SPIN_DELTA>
  <SPIN_ANGLE units="deg">3.0</SPIN_ANGLE>
  <SPIN_ANGLE_VEL units="deg/s">10.0</SPIN_ANGLE_VEL>
  <MOMENTUM_ALPHA units="deg">1.0</MOMENTUM_ALPHA>
  <MOMENTUM_DELTA units="deg">1.0</MOMENTUM_DELTA>
  <NUTATION_VEL units="deg/s">1.0</NUTATION_VEL>
</spin>
```

```xml
<spin>
  <COMMENT>Round and round and round it goes...</COMMENT>
  <REF_FRAME_A>J2000</REF_FRAME_A>
  <REF_FRAME_B>SC_BODY_1</REF_FRAME_B>
  <SPIN_ALPHA units="deg">1.0</SPIN_ALPHA>
  <SPIN_DELTA units="deg">2.0</SPIN_DELTA>
  <SPIN_ANGLE units="deg">3.0</SPIN_ANGLE>
  <SPIN_ANGLE_VEL units="deg/s">10.0</SPIN_ANGLE_VEL>
</spin>
```

## 7.6 CREATING AN AEM INSTANTIATION

7.6.1 An AEM instantiation shall be delimited with the `<aem></aem>` root element tags using the standard attributes documented in 7.4.1 through 7.4.2.

**NOTE** – Annex G provides some example AEM instantiations.

7.6.2 The final attributes of the `<aem>` tag shall be ‘id’ and ‘version’; the order in which these attributes are specified is not significant.

7.6.3 The ‘id’ attribute shall be ‘id="`CCSDS_AEM_VERS`"’.

7.6.4 The ‘version’ attribute for the version of the AEM described in section 4 shall be ‘version="2.0"’.

7.6.5 The standard ADM/XML header shall follow the `<aem>` tag (see 7.4.3).

7.6.6 The AEM `<body>` shall consist of one or more `<segment>` constructs (see reference [4], subsection 3.4).

7.6.7 Each `<segment>` shall consist of a `<metadata>` section and a `<data>` section.

7.6.8 The keywords in the `<metadata>` and `<data>` sections shall be those specified in 4.2. The rules for including any of the keyword tags in the AEM/XML are the same as those specified for the AEM/KVN.

7.6.9 Tags for keywords shall be all uppercase as in 4.2.

7.6.10 Although units are not specified in the KVN representation of the AEM, several of the AEM/XML keywords may have a units attribute, if desired by the AEM producer, as illustrated in the following table:

**Table 7-4: AEM XML Units**

| Keyword | Units | Example |
| :--- | :--- | :--- |
| `Q1_DOT` | 1/s | `<Q1_DOT units="1/s">numeric-value</Q1_DOT>` |
| `Q2_DOT` | 1/s | `<Q2_DOT units="1/s">numeric-value</Q2_DOT>` |
| `Q3_DOT` | 1/s | `<Q3_DOT units="1/s">numeric-value</Q3_DOT>` |
| `QC_DOT` | 1/s | `<QC_DOT units="1/s">numeric-value</QC_DOT>` |
| `ANGLE_1` | deg | `<ANGLE_1 units="deg">numeric-value</ANGLE_1>` |
| `ANGLE_2` | deg | `<ANGLE_2 units="deg">numeric-value</ANGLE_2>` |
| `ANGLE_3` | deg | `<ANGLE_3 units="deg">numeric-value</ANGLE_3>` |
| `ANGLE_1_DOT` | deg/s | `<ANGLE_1_DOT units="deg/s">numeric-value</ANGLE_1_DOT>` |
| `ANGLE_2_DOT` | deg/s | `<ANGLE_2_DOT units="deg/s">numeric-value</ANGLE_2_DOT>` |
| `ANGLE_3_DOT` | deg/s | `<ANGLE_3_DOT units="deg/s">numeric-value</ANGLE_3_DOT>` |
| `ANGVEL_X` | deg/s | `<ANGVEL_X units="deg/s">numeric-value</ANGVEL_X>` |
| `ANGVEL_Y` | deg/s | `<ANGVEL_Y units="deg/s">numeric-value</ANGVEL_Y>` |
| `ANGVEL_Z` | deg/s | `<ANGVEL_Z units="deg/s">numeric-value</ANGVEL_Z>` |
| `SPIN_ALPHA` | deg | `<SPIN_ALPHA units="deg">numeric-value</SPIN_ALPHA>` |
| `SPIN_DELTA` | deg | `<SPIN_DELTA units="deg">numeric-value</SPIN_DELTA>` |
| `SPIN_ANGLE` | deg | `<SPIN_ANGLE units="deg">numeric-value</SPIN_ANGLE>` |
| `SPIN_ANGLE_VEL`| deg/s | `<SPIN_ANGLE_VEL units="deg/s">numeric-value</SPIN_ANGLE_VEL>` |
| `NUTATION` | deg | `<NUTATION units="deg">numeric-value</NUTATION>` |
| `NUTATION_PER` | s | `<NUTATION_PER units="s">numeric-value</NUTATION_PER>` |
| `NUTATION_PHASE`| deg | `<NUTATION_PHASE units="deg">numeric-value</NUTATION_PHASE>` |
| `MOMENTUM_ALPHA`| deg | `<MOMENTUM_ALPHA units="deg">numeric-value</MOMENTUM_ALPHA>` |
| `MOMENTUM_DELTA`| deg | `<MOMENTUM_DELTA units="deg">numeric-value</MOMENTUM_DELTA>` |
| `NUTATION_VEL` | deg/s | `<NUTATION_VEL units="deg/s">numeric-value</NUTATION_VEL>` |

### 7.6.11 SPECIAL TAGS IN THE AEM/XML BODY

**NOTE** – In addition to the AEM keywords specified in 4.2, there are several special tags associated with the AEM body as described in the next few subsections.

7.6.11.1 The `<attitudeState>` tag shall be used to encapsulate the keywords associated with the structure of one of the attitude ephemeris data line types.

7.6.11.2 The AEM/XML tags used within the `<attitudeState>` structure shall be drawn from the following table:

**Table 7-5: Special Tags in the AEM/XML Body**

| AEM 'ATTITUDE_TYPE' Metadata Value | Associated ADM/XML Tags in the <attitudeState> |
| :--- | :--- |
| `QUATERNION` | `<quaternionEphemeris>`, `<quaternion>` |
| `QUATERNION/DERIVATIVE` | `<quaternionDerivative>`, `<quaternion>`, `<quaternionDot>` |
| `QUATERNION/ANGVEL` | `<quaternionAngVel>`, `<quaternion>`, `<angVel>` |
| `EULER_ANGLE` | `<eulerAngle>` |
| `EULER_ANGLE/DERIVATIVE` | `<eulerAngleDerivative>` |
| `EULER_ANGLE/ANGVEL` | `<eulerAngleAngVel>` |
| `SPIN` | `<spin>` |
| `SPIN/NUTATION` | `<spinNutation>` |
| `SPIN/NUTATION_MOM` | `<spinNutationMom>` |

7.6.11.3 Between the begin tag and end tag (e.g., between `<quaternionEphemeris>` and `</quaternionEphemeris>`), the user shall place the values required by the specific ephemeris data line type as specified in 4.2.4, table 4-4.

7.6.11.4 In the XML representation of the AEM, the components of the `<attitudeState>` ephemeris data line must be represented with keywords (i.e., a tag).

7.6.11.5 The `<attitudeState>` keywords shall be the same as those defined for the same construct in the APM.

**NOTE** – In the KVN representations of the ephemeris data lines, keywords are not used. Rather, the components of the ephemeris data line appear in an order defined by the specific ephemeris data line type. In the XML representation, the tags described are fundamental to the format.

7.6.11.6 Figure G-13 shows an example with each `<attitudeState>` type showing the use of the special tags.

## 7.7 CREATING AN ACM INSTANTIATION

7.7.1 An ACM instantiation shall be delimited with the `<acm></acm>` root element tags using the standard attributes documented in 7.4.1 through 7.4.2.

**NOTE** – Annex G provides some example ACM instantiations.

7.7.2 The final attributes of the `<acm>` tag shall be ‘id’ and ‘version’.

7.7.3 The ‘id’ attribute shall be ‘id="`CCSDS_ACM_VERS`"’.

7.7.4 The ‘version’ attribute for the version of the ACM described in section 5 shall be ‘version="2.0"’.

7.7.5 The standard NDM header shall follow the `<acm>` tag (see 7.4.3).

7.7.6 The ACM `<body>` shall consist of a single `<segment>` construct.

7.7.7 The `<segment>` shall consist of a `<metadata>` section and a `<data>` section.

7.7.8 The `<metadata>` and `<data>` sections shall be those specified in section 5. The rules for including any of the keyword tags in the ACM/XML are the same as those specified for the ACM/KVN.

7.7.9 Tags for keywords specified in section 5 shall be all uppercase.

7.7.10 Several of the ACM/XML keywords may have the unit attribute.

7.7.11 In all cases, the units shall match those defined in the tables in section 5.

7.7.12 Table 7-6 lists examples of the use of units in the ACM/XML.

**Table 7-6: ACM XML Units**

| Keyword | Units | Example |
| :--- | :--- | :--- |
| `SENSOR_NOISE_STDDEV` | deg | `<SENSOR_NOISE_STDDEV units="deg">numeric-value</SENSOR_NOISE_STDDEV>` |
| `TARGET_SPINRATE` | deg/s | `<TARGET_SPINRATE units="deg/s">numeric-value</TARGET_SPINRATE>` |
| `SIGMA_V` | deg/s\*\*0.5 | `<SIGMA_V units="deg/s**0.5">numeric-value</SIGMA_V>` |
| `SIGMA_U` | deg/s\*\*1.5 | `<SIGMA_U units="deg/s**1.5">numeric-value</SIGMA_U>` |
| `SENSOR_FREQUENCY` | Hz | `<SENSOR_FREQUENCY units="Hz">numeric-value</SENSOR_FREQUENCY>` |
| `WET_MASS` | kg | `<WET_MASS units="kg">numeric-value</WET_MASS>` |
| `IXX` | kg\*m\*\*2 | `<IXX units="kg*m**2">numeric-value</IXX>` |
| `CP` | m | `<CP units="m">vector-value</CP>` |
| `TARGET_MOMENTUM` | N\*m\*s | `<TARGET_MOMENTUM units="N*m*s">numeric-value</TARGET_MOMENTUM>` |
| `TAIMUTC_TZERO` | s | `<TAIMUTC_TZERO units="s">numeric-value</TAIMUTC_TZERO>` |

### 7.7.13 SPECIAL TAGS IN THE ACM/XML BODY

7.7.13.1 In addition to the ACM keywords specified in section 5, there are some special tags associated with the ACM/XML body as described in the next sections and given in table 7-7.

**Table 7-7: ACM Blocks and Tags**

| ACM Logical Block | ADM/XML ACM Section Tags | Data Line Tag |
| :--- | :--- | :--- |
| Attitude Data | `<att>` | `<attLine>` |
| Space Object Physical Characteristics | `<phys>` | N/A |
| Covariance Data | `<cov>` | `<covLine>` |
| Maneuver Data | `<man>` | N/A |
| Attitude Determination Data | `<ad>` | N/A |
| User-Defined Parameters | `<user>` | N/A |

7.7.13.2 Between the begin tag and end tag (e.g., between `<att>` and `</att>`), the user must place the keywords required by the specific ACM section as specified in section 5.

7.7.13.3 The data type of the `<attLine>` and `<covLine>` elements is ‘xsd:string’; that is, there is no validation of the contents, and the line must be parsed by the ACM recipient to access the individual components of the attitude or covariance line.

7.7.13.4 The number of individual components in the multipartite `<attLine>` shall be determined by the number of components in the value for the `ATT_TYPE` keyword, plus one for the time tag.

7.7.13.5 The number of individual components in the multipartite `<covLine>` shall be determined by the number of components for the `COV_TYPE` keywords, plus one for the time tag.

### 7.7.14 DISCUSSION

This non-normative subsection discusses and provides an example of the use of the `sensorData` special tag in the ACM. If this tag is used, it should be repeated as many times as there are sensors.

The following XML code provides an example of the section of table 5-8 covering sensor data, rendered in XML. The number of sensors used is 2; the two sections that follow characterize each sensor in a block that contains `<SENSOR_USED>`, `<NUMBER_SENSOR_NOISE_COVARIANCE>`, `<SENSOR_NOISE_STDDEV>`, and `<SENSOR_FREQUENCY>`.

```xml
<sensorData>
  <SENSOR_NUMBER>1</SENSOR_NUMBER>
  <SENSOR_USED>AST</SENSOR_USED>
  <NUMBER_SENSOR_NOISE_COVARIANCE>3</NUMBER_SENSOR_NOISE_COVARIANCE>
  <SENSOR_NOISE_STDDEV>0.003 0.003 0.010</SENSOR_NOISE_STDDEV>
</sensorData>
<sensorData>
  <SENSOR_NUMBER>2</SENSOR_NUMBER>
  <SENSOR_USED>DSS</SENSOR_USED>
  <NUMBER_SENSOR_NOISE_COVARIANCE>2</NUMBER_SENSOR_NOISE_COVARIANCE>
  <SENSOR_NOISE_STDDEV units="deg">0.01 0.01 </SENSOR_NOISE_STDDEV>
  <SENSOR_FREQUENCY units="Hz">5</SENSOR_FREQUENCY>
</sensorData>
```

## 7.8 CREATING A COMBINED INSTANTIATION

7.8.1 An ADM user may create an XML instance that incorporates any number of messages from sections 3 through 5 of this document in a logical suite called an ‘NDM combined instantiation’. Such combined instantiations may be useful for some situations; for example:

a) a scenario in which both a ‘with maneuver’ message and a ‘without maneuver’ message are combined in a single message;
b) a constellation scenario in which states (APM) and/or ephemeris data (AEM, ACM) for all the spacecraft in the constellation are combined in a single XML message;
c) a full AEM ephemeris with details on important states reflected in some number of APMs; the AEM and the multiple APMs can be conveniently conveyed in a single NDM.

7.8.2 An NDM combined instantiation shall be delimited with the `<ndm></ndm>` root element tags instead of one of the individual message tags described in table 7-1.

7.8.3 The standard attributes documented in 7.4.1 through 7.4.2 shall be used with the `<ndm>` tag, with the exception that neither ‘id’ nor ‘version’ attributes are associated with the `<ndm>` tag.

7.8.4 In the NDM combined instantiation, the only attributes that shall appear on the constituent message tags (i.e., the tags listed in table 7-1) are the ‘id’ and ‘version’ attributes.

7.8.5 Between the `<ndm></ndm>` tags, the desired messages described in 7.5 through 7.7 may be combined as needed to meet user requirements.

7.8.6 Any combination of constituent ADM message types may be used in an NDM combined instantiation.

7.8.7 Figures 7-2 and 7-3 illustrate the basic structure of an NDM combined instantiation. All detail has been removed from figure 7-2 in order to contrast the single message ADM with an NDM combined instantiation. As shown in figure 7-3, in an NDM combined instantiation, the individual message tags still have the ‘id’ and ‘version’ attributes, but the namespace attributes and schema location attributes are associated only with the `<ndm>` root element.

**Figure 7-2: Comparison of Single Message APM with NDM Combined Instantiation**

| Single Message APM | NDM Combined Instantiation |
| :--- | :--- |
| `<apm>` <br> `<header>` <br> `</header>` <br> `<body>` <br> `</body>` <br> `</apm>` | `<ndm>` <br> `<apm>` <br> `<header>` <br> `</header>` <br> `<body>` <br> `</body>` <br> `</apm>` <br> ... <br> `<apm>` <br> `<header>` <br> `</header>` <br> `<body>` <br> `</body>` <br> `</apm>` <br> `</ndm>` |

The APMs shown in the right-hand column of figure 7-2 may be replaced with any number of AEM or ACM messages in any combination as needed to meet user requirements, as shown in figure 7-3 below.

```xml
<?xml version="1.0" encoding="UTF-8"?>
<ndm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-4.0.0-master-4.0.xsd">
  <COMMENT>This figure combines multiple ADM/XML messages into a single message</COMMENT>
  <COMMENT>Message detail is deleted in order to focus on the message structure</COMMENT>
  <COMMENT>Note use of "<ndm>" root element, and ADM message/version attributes</COMMENT>
  <apm id="CCSDS_APM_VERS" version="2.0">
    <header>
    </header>
    <body>
    </body>
  </apm>
  <aem id="CCSDS_AEM_VERS" version="2.0">
    <header>
    </header>
    <body>
    </body>
  </aem>
  <apm id="CCSDS_APM_VERS" version="2.0">
    <header>
    </header>
    <body>
    </body>
  </apm>
  <aem id="CCSDS_AEM_VERS" version="2.0">
    <header>
    </header>
    <body>
    </body>
  </aem>
  <acm id="CCSDS_ACM_VERS" version="2.0">
    <header>
    </header>
    <body>
    </body>
  </acm>
</ndm>
```

**Figure 7-3: NDM Combined Instantiation Showing Mix of ADMs and Use of Attributes**

**NOTE** – Figure G-12 provides a full example of one use case combining multiple ADMs in a single NDM/XML message. (For instructions on creating a combined instantiation that incorporates ADM/XML messages combined with other navigation related messages, see reference [4].)


# ANNEX A

## IMPLEMENTATION CONFORMANCE STATEMENT PROFORMA (NORMATIVE)

### A1 INTRODUCTION

#### A1.1 OVERVIEW

This annex provides the ICS Requirements List (RL) for an implementation of the Attitude Data Messages (CCSDS 504.0). The ICS for an implementation is generated by completing the RL in accordance with the instructions below. An implementation claiming conformance must satisfy the mandatory requirements referenced in the RL.

The RL in this annex is blank. An implementation’s completed RL is called the ICS. The ICS states which capabilities and options have been implemented. The following can use the ICS:

*   the implementer, as a checklist to reduce the risk of failure to conform to the standard through oversight;
*   a supplier or potential acquirer of the implementation, as a detailed indication of the capabilities of the implementation, stated relative to the common basis for understanding provided by the standard ICS proforma;
*   a user or potential user of the implementation, as a basis for initially checking the possibility of interworking with another implementation (it should be noted that, while interworking can never be guaranteed, failure to interwork can often be predicted from incompatible ICS lists);
*   a tester, as the basis for selecting appropriate tests against which to assess the claim for conformance of the implementation.

#### A1.2 ABBREVIATIONS AND CONVENTIONS

The RL consists of information in tabular form. The status of features is indicated using the abbreviations and conventions described below.

**Item Column**

The item column contains sequential numbers for items in the table.

**Feature Column**

The feature column contains a brief descriptive name for a feature. It implicitly means ‘Is this feature supported by the implementation?’

**Status Column**

The status column uses the following notations:

*   M mandatory;
*   O optional;
*   C conditional;
*   X prohibited;
*   I out of scope;
*   N/A not applicable.

**Support Column Symbols**

The support column is to be used by the implementer to state whether a feature is supported by entering Y, N, or N/A, indicating:

*   Y Yes, supported by the implementation.
*   N No, not supported by the implementation.
*   N/A Not applicable.

The support column should also be used, when appropriate, to enter values supported for a given capability.

#### A1.3 INSTRUCTIONS FOR COMPLETING THE RL

An implementer shows the extent of compliance to the Recommended Standard by completing the RL; that is, the state of compliance with all mandatory requirements and the options supported are shown. The resulting completed RL is called an ICS. The implementer shall complete the RL by entering appropriate responses in the support or values supported column, using the notation described in A1.2. If a conditional requirement is inapplicable, N/A should be used. If a mandatory requirement is not satisfied, exception information must be supplied by entering a reference Xi, where i is a unique identifier, to an accompanying rationale for the noncompliance.

### A2 ICS PROFORMA FOR THE ATTITUDE DATA MESSAGE

#### A2.1 GENERAL INFORMATION

##### A2.1.1 Identification of ICS

| | |
| :--- | :--- |
| Date of Statement (DD/MM/YYYY) | |
| ICS serial number | |
| System Conformance statement cross reference | |

##### A2.1.2 Identification of Implementation Under Test (IUT)

| | |
| :--- | :--- |
| Implementation Name | |
| Implementation Version | |
| Special Configuration | |
| Other Information | |

##### A2.1.3 Identification of Supplier

| | |
| :--- | :--- |
| Supplier | |
| Contact Point for Queries | |
| Implementation Name(s) and Versions | |
| Other information necessary for full identification, for example, name(s) and version(s) for machines and/or operating systems, system name(s) | |

##### A2.1.4 Identification of Specification

| | |
| :--- | :--- |
| 504.0-B-2 | |
| Have any exceptions been required? | Yes \[ ] No \[ ] |
| **NOTE** – A YES answer means that the implementation does not conform to the Recommended Standard. Non-supported mandatory capabilities are to be identified in the ICS, with an explanation of why the implementation is nonconforming. | |

#### A2.2 REQUIREMENTS LIST

Annex H ([H1]) provides additional information.

**NOTE** – In the following sections, the nomenclature is the following for the ‘Status’ column:

*   M: Mandatory;
*   O: Optional;
*   C: Conditional (optional, but some conditions apply).

##### A2.2.1 Attitude Parameter Message Requirements list

| Item | Feature | Keyword | Reference | Status M/O/C | Support |
| :--- | :--- | :--- | :--- | :--- | :--- |
| | **APM Header** | N/A | Table 3-1 | N/A | |
| 1 | APM Version | `CCSDS_APM_VERS` | Table 3-1 | M | |
| 2 | Comment | `COMMENT` | Table 3-1 | O | |
| 3 | Message classification/caveats | `CLASSIFICATION` | Table 3-1 | O | |
| 4 | Message creation date and time | `CREATION_DATE` | Table 3-1 | M | |
| 5 | Message originator | `ORIGINATOR` | Table 3-1 | M | |
| 6 | Unique message identifier | `MESSAGE_ID` | Table 3-1 | O | |
| | **APM Metadata** | N/A | Table 3-2 | N/A | |
| 7 | Comment | `COMMENT` | Table 3-2 | O | |
| 8 | Name of space object | `OBJECT_NAME` | Table 3-2 | M | |
| 9 | Identifier of space object | `OBJECT_ID` | Table 3-2 | M | |
| 10 | Orbit center | `CENTER_NAME` | Table 3-2 | O | |
| 11 | Time system applicable to data | `TIME_SYSTEM` | Table 3-2 | M | |
| | **APM Data** | N/A | Table 3-3 | N/A | |
| 12 | Comment | `COMMENT` | Table 3-3 | O | |
| 13 | Epoch of the state vector | `EPOCH` | Table 3-3 | M | |
| | **Quaternion block** | N/A | Table 3-3 | N/A | |
| 14 | Start of block | `QUAT_START` | Table 3-3 | M | |
| 15 | Comment | `COMMENT` | Table 3-3 | O | |
| 16 | Reference frame starting point | `REF_FRAME_A` | Table 3-3 | M | |
| 17 | Reference frame end point | `REF_FRAME_B` | Table 3-3 | M | |
| 18 | Quaternion component 1 | `Q1` | Table 3-3 | M | |
| 19 | Quaternion component 2 | `Q2` | Table 3-3 | M | |
| 20 | Quaternion component 3 | `Q3` | Table 3-3 | M | |
| 21 | Quaternion component 4 (real part) | `QC` | Table 3-3 | M | |
| 22 | Quaternion derivative component 1 | `Q1_DOT` | Table 3-3 | O | |
| 23 | Quaternion derivative component 2 | `Q2_DOT` | Table 3-3 | O | |
| 24 | Quaternion derivative component 3 | `Q3_DOT` | Table 3-3 | O | |
| 25 | Quaternion derivative component 4 (real part) | `QC_DOT` | Table 3-3 | O | |
| 26 | End of block | `QUAT_STOP` | Table 3-3 | M | |
| | **Euler block** | N/A | Table 3-3 | N/A | |
| 27 | Start of block | `EULER_START` | Table 3-3 | M | |
| 28 | Comment | `COMMENT` | Table 3-3 | O | |
| 29 | Reference frame starting point | `REF_FRAME_A` | Table 3-3 | M | |
| 30 | Reference frame end point | `REF_FRAME_B` | Table 3-3 | M | |
| 31 | Rotation sequence | `EULER_ROT_SEQ` | Table 3-3 | M | |
| 32 | Rotation angle 1 | `ANGLE_1` | Table 3-3 | M | |
| 33 | Rotation angle 2 | `ANGLE_2` | Table 3-3 | M | |
| 34 | Rotation angle 3 | `ANGLE_3` | Table 3-3 | M | |
| 35 | Rotation angle 1 derivative | `ANGLE_1_DOT` | Table 3-3 | O | |
| 36 | Rotation angle 2 derivative | `ANGLE_2_DOT` | Table 3-3 | O | |
| 37 | Rotation angle 3 derivative | `ANGLE_3_DOT` | Table 3-3 | O | |
| 38 | End of block | `EULER_STOP` | Table 3-3 | M | |
| | **Angular velocity block** | N/A | Table 3-3 | N/A | |
| 39 | Start of block | `ANGVEL_START` | Table 3-3 | M | |
| 40 | Comment | `COMMENT` | Table 3-3 | O | |
| 41 | Reference frame starting point | `REF_FRAME_A` | Table 3-3 | M | |
| 42 | Reference frame end point | `REF_FRAME_B` | Table 3-3 | M | |
| 43 | Reference frame | `ANGVEL_FRAME` | Table 3-3 | M | |
| 44 | Angular velocity X coordinate | `ANGVEL_X` | Table 3-3 | M | |
| 45 | Angular velocity Y coordinate | `ANGVEL_Y` | Table 3-3 | M | |
| 46 | Angular velocity Z coordinate | `ANGVEL_Z` | Table 3-3 | M | |
| 47 | End of block | `ANGVEL_STOP` | Table 3-3 | M | |
| | **Spin block** | N/A | Table 3-3 | N/A | |
| 48 | Start of block | `SPIN_START` | Table 3-3 | M | |
| 49 | Comment | `COMMENT` | Table 3-3 | O | |
| 50 | Reference frame starting point | `REF_FRAME_A` | Table 3-3 | M | |
| 51 | Reference frame end point | `REF_FRAME_B` | Table 3-3 | M | |
| 52 | Right ascension | `SPIN_ALPHA` | Table 3-3 | M | |
| 53 | Declination | `SPIN_DELTA` | Table 3-3 | M | |
| 54 | Phase | `SPIN_ANGLE` | Table 3-3 | M | |
| 55 | Angular velocity | `SPIN_ANGLE_VEL` | Table 3-3 | M | |
| 56 | Nutation angle | `NUTATION` | Table 3-3 | C | |
| 57 | Nutation period | `NUTATION_PER` | Table 3-3 | C | |
| 58 | Nutation phase | `NUTATION_PHASE` | Table 3-3 | C | |
| 59 | Right ascension of ang. momentum vector | `MOMENTUM_ALPHA` | Table 3-3 | C | |
| 60 | Declination of ang. momentum vector | `MOMENTUM_DELTA` | Table 3-3 | C | |
| 61 | Angular velocity around momentum | `NUTATION_VEL` | Table 3-3 | C | |
| 62 | End of block | `SPIN_STOP` | Table 3-3 | M | |
| | **Inertia block** | N/A | Table 3-3 | N/A | |
| 63 | Start of block | `INERTIA_START` | Table 3-3 | M | |
| 64 | Comment | `COMMENT` | Table 3-3 | O | |
| 65 | Reference frame | `INERTIA_REF_FRAME` | Table 3-3 | M | |
| 66 | Moment about X | `IXX` | Table 3-3 | M | |
| 67 | Moment about Y | `IYY` | Table 3-3 | M | |
| 68 | Moment about Z | `IZZ` | Table 3-3 | M | |
| 69 | Cross product X-Y | `IXY` | Table 3-3 | M | |
| 70 | Cross product X-Z | `IXZ` | Table 3-3 | M | |
| 71 | Cross product Y-Z | `IYZ` | Table 3-3 | M | |
| 72 | End of block | `INERTIA_STOP` | Table 3-3 | M | |
| | **Maneuver block** | N/A | Table 3-3 | N/A | |
| 73 | Start of block | `MAN_START` | Table 3-3 | M | |
| 74 | Comment | `COMMENT` | Table 3-3 | O | |
| 75 | Epoch of maneuver | `MAN_EPOCH_START` | Table 3-3 | M | |
| 76 | Maneuver duration | `MAN_DURATION` | Table 3-3 | M | |
| 77 | Reference frame | `MAN_REF_FRAME` | Table 3-3 | M | |
| 78 | Torque - X coordinate | `MAN_TOR_X` | Table 3-3 | M | |
| 79 | Torque - Y coordinate | `MAN_TOR_Y` | Table 3-3 | M | |
| 80 | Torque - Z coordinate | `MAN_TOR_Z` | Table 3-3 | M | |
| 81 | Mass variation | `MAN_DELTA_MASS` | Table 3-3 | O | |
| 82 | End of block | `MAN_STOP` | Table 3-3 | M | |

##### A2.2.2 Attitude Ephemeris Message Requirements list

| Item | Feature | Keyword | Reference | Status | Support |
| :--- | :--- | :--- | :--- | :--- | :--- |
| | **AEM Header** | N/A | Table 4-2 | N/A | |
| 1 | AEM Version | `CCSDS_AEM_VERS` | Table 4-2 | M | |
| 2 | Comment | `COMMENT` | Table 4-2 | O | |
| 3 | Message classification/caveats | `CLASSIFICATION` | Table 4-2 | O | |
| 4 | Message creation date and time | `CREATION_DATE` | Table 4-2 | M | |
| 5 | Message originator | `ORIGINATOR` | Table 4-2 | M | |
| 6 | Unique message identifier | `MESSAGE_ID` | Table 4-2 | O | |
| | **Metadata logical block** | N/A | Table 4-3 | N/A | |
| 7 | Start of AEM Metadata | `META_START` | Table 4-3 | M | |
| 8 | Comment | `COMMENT` | Table 4-3 | O | |
| 9 | Name of space object | `OBJECT_NAME` | Table 4-3 | M | |
| 10 | Identifier of space object | `OBJECT_ID` | Table 4-3 | M | |
| 11 | Orbit center | `CENTER_NAME` | Table 4-3 | O | |
| 12 | Reference frame starting point | `REF_FRAME_A` | Table 4-3 | M | |
| 13 | Reference frame end point | `REF_FRAME_B` | Table 4-3 | M | |
| 14 | Time system applicable to data | `TIME_SYSTEM` | Table 4-3 | M | |
| 15 | Start of total time span covered by data | `START_TIME` | Table 4-3 | M | |
| 16 | Start of useable time span | `USEABLE_START_TIME` | Table 4-3 | O | |
| 17 | End of useable time span | `USEABLE_STOP_TIME` | Table 4-3 | O | |
| 18 | End of total time span covered by data | `STOP_TIME` | Table 4-3 | M | |
| 19 | Type of attitude data lines | `ATTITUDE_TYPE` | Table 4-3 | M | |
| 20 | Rotation sequence | `EULER_ROT_SEQ` | Table 4-3 | O | |
| 21 | Reference frame for angular velocity vectors | `ANGVEL_FRAME` | Table 4-3 | O | |
| 22 | Recommended interpolation method | `INTERPOLATION_METHOD` | Table 4-3 | O | |
| 23 | Recommended interpolation degree | `INTERPOLATION_DEGREE` | Table 4-3 | O | |
| 24 | End of OEM Metadata | `META_STOP` | Table 4-3 | M | |
| | **Data logical block** | N/A | Table 4-3 | N/A | |
| 25 | Ephemeris lines | `<ephemeris data lines here>` | Table 4-4 | M | |

##### A2.2.3 Attitude Comprehensive Message Requirements list

| Item | Feature | Keyword | Reference | Status | Support |
| :--- | :--- | :--- | :--- | :--- | :--- |
| | **ACM Header** | N/A | Table 5-2 | N/A | |
| 1 | ACM Version | `CCSDS_ACM_VERS` | Table 5-2 | M | |
| 2 | Comment | `COMMENT` | Table 5-2 | O | |
| 3 | Message classification/caveats | `CLASSIFICATION` | Table 5-2 | O | |
| 4 | Message creation date and time | `CREATION_DATE` | Table 5-2 | M | |
| 5 | Message originator | `ORIGINATOR` | Table 5-2 | M | |
| 6 | Unique message identifier | `MESSAGE_ID` | Table 5-2 | O | |
| | **Metadata logical block** | N/A | Table 5-3 | N/A | |
| 7 | ACM Metadata Start | `META_START` | Table 5-3 | M | |
| 8 | Comment | `COMMENT` | Table 5-3 | O | |
| 9 | Classification | `CLASSIFICATION` | Table 5-3 | O | |
| 10 | Spacecraft name for the object | `OBJECT_NAME` | Table 5-3 | M | |
| 11 | International designator for the object | `INTERNATIONAL_DESIGNATOR` | Table 5-3 | O | |
| 12 | Satellite catalog source | `CATALOG_NAME` | Table 5-3 | O | |
| 13 | Unique satellite identification designator | `OBJECT_DESIGNATOR` | Table 5-3 | O | |
| 14 | Message originator or programmatic Point-of-Contact | `ORIGINATOR_POC` | Table 5-3 | O | |
| 15 | Contact position of the originator PoC | `ORIGINATOR_POSITION` | Table 5-3 | O | |
| 16 | Originator PoC phone number | `ORIGINATOR_PHONE` | Table 5-3 | O | |
| 17 | Originator PoC email | `ORIGINATOR_EMAIL` | Table 5-3 | O | |
| 18 | Originator PoC address | `ORIGINATOR_ADDRESS` | Table 5-3 | O | |
| 19 | Unique identifier of linked Orbit Data Message(s) | `ODM_MSG_LINK` | Table 5-3 | O | |
| 20 | Name of orbited object | `CENTER_NAME` | Table 5-3 | O | |
| 21 | Time system used for the data | `TIME_SYSTEM` | Table 5-3 | M | |
| 22 | Default epoch to which all relative times are referenced | `EPOCH_TZERO` | Table 5-3 | M | |
| 23 | Message contents | `ACM_DATA_ELEMENTS` | Table 5-3 | O | |
| 24 | Time of the earliest data | `START_TIME` | Table 5-3 | O | |
| 25 | Time of the latest data | `STOP_TIME` | Table 5-3 | O | |
| 26 | Difference (TAI – UTC) in seconds | `TAIMUTC_AT_TZERO` | Table 5-3 | O | |
| 27 | Epoch of next leap second | `NEXT_LEAP_EPOCH` | Table 5-3 | O | |
| 28 | Difference (TAI – UTC) after next leap second(s) are introduced | `NEXT_LEAP_TAIMUTC` | Table 5-3 | O | |
| 29 | Metadata Stop | `META_STOP` | Table 5-3 | M | |
| | **Attitude state time history logical block** | N/A | Table 5-4 | N/A | |
| 30 | Attitude state time history start | `ATT_START` | Table 5-4 | M | |
| 31 | Comment | `COMMENT` | Table 5-4 | O | |
| 32 | Identification number for this attitude state time history | `ATT_ID` | Table 5-4 | O | |
| 33 | Identification number for the previous attitude time history | `ATT_PREV_ID` | Table 5-4 | O | |
| 34 | Basis of this Attitude State time history data | `ATT_BASIS` | Table 5-4 | O | |
| 35 | Identification number for the telemetry dataset, attitude determination, or simulation | `ATT_BASIS_ID` | Table 5-4 | O | |
| 36 | Reference frame starting point | `REF_FRAME_A` | Table 5-4 | M | |
| 37 | Reference frame end point | `REF_FRAME_B` | Table 5-4 | M | |
| 38 | Number of states | `NUMBER_STATES` | Table 5-4 | M | |
| 39 | Type of attitude data | `ATT_TYPE` | Table 5-4 | M | |
| 40 | Rotation sequence | `EULER_ROT_SEQ` | Table 5-4 | C | |
| 41 | Type of rate data | `RATE_TYPE` | Table 5-4 | O | |
| 42 | ACM attitude state time history | `<attitude state time history here>` | Table 5-4 | M | |
| 43 | Attitude state time history end | `ATT_STOP` | Table 5-4 | M | |
| | **Space Object Physical Characteristics logical block** | N/A | Table 5-5 | N/A | |
| 44 | Start of a Space Object Physical Characteristics specification | `PHYS_START` | Table 5-5 | M | |
| 45 | Comment | `COMMENT` | Table 5-5 | O | |
| 46 | Drag Coefficient | `DRAG_COEFF` | Table 5-5 | O | |
| 47 | Space object total mass | `WET_MASS` | Table 5-5 | O | |
| 48 | Space object dry mass | `DRY_MASS` | Table 5-5 | O | |
| 49 | Coordinate system for the center of pressure vector. | `CP_REF_FRAME` | Table 5-5 | C1 | |
| 50 | Vector location of spacecraft center of pressure | `CP` | Table 5-5 | O | |
| 51 | Coordinate system for the inertia tensor. | `INERTIA_REF_FRAME` | Table 5-5 | O | |
| 52 | Moment about X. | `IXX` | Table 5-5 | O | |
| 53 | Moment about Y. | `IYY` | Table 5-5 | O | |
| 54 | Moment about Z. | `IZZ` | Table 5-5 | O | |
| 55 | Cross Product X-Y | `IXY` | Table 5-5 | O | |
| 56 | Cross Product X-Z | `IXZ` | Table 5-5 | O | |
| 57 | Cross Product Y-Z | `IYZ` | Table 5-5 | O | |
| 58 | End of the Space Object Physical Characteristics specification | `PHYS_STOP` | Table 5-5 | M | |
| | **Covariance time history logical block** | N/A | Table 5-6 | N/A | |
| 59 | Start of a covariance time history section | `COV_START` | Table 5-6 | M | |
| 60 | Comment | `COMMENT` | Table 5-6 | O | |
| 61 | Identification number for this covariance time history block | `COV_ID` | Table 5-6 | O | |
| 62 | Identification number for the previous covariance time history | `COV_PREV_ID` | Table 5-6 | O | |
| 63 | Basis of this covariance time history | `COV_BASIS` | Table 5-6 | O | |
| 64 | Identification number for the telemetry dataset, attitude determination, or simulation | `COV_BASIS_ID` | Table 5-6 | O | |
| 65 | Reference frame of the covariance time history | `COV_REF_FRAME` | Table 5-6 | O | |
| 66 | Covariance composition | `COV_TYPE` | Table 5-6 | M | |
| 67 | Covariance data | `<covariance data here>` | Table 5-6 | M | |
| 68 | End of a covariance time history section | `COV_STOP` | Table 5-6 | M | |
| | **Maneuver time history logical block** | N/A | Table 5-7 | N/A | |
| 69 | Start of a maneuver data block | `MAN_START` | Table 5-7 | M | |
| 70 | Comment | `COMMENT` | Table 5-7 | O | |
| 71 | Identification number for this maneuver | `MAN_ID` | Table 5-7 | O | |
| 72 | Identification number for the previous maneuver | `MAN_PREV_ID` | Table 5-7 | O | |
| 73 | Specifies the purpose of the maneuver | `MAN_PURPOSE` | Table 5-7 | M | |
| 74 | Start time of maneuver | `MAN_BEGIN_TIME` | Table 5-7 | M | |
| 75 | End time of maneuver | `MAN_END_TIME` | Table 5-7 | C | |
| 76 | Length of maneuver | `MAN_DURATION` | Table 5-7 | C | |
| 77 | Actuator used for the maneuver | `ACTUATOR_USED` | Table 5-7 | O | |
| 78 | Target momentum components | `TARGET_MOMENTUM` | Table 5-7 | C | |
| 79 | Target momentum frame | `TARGET_MOM_FRAME` | Table 5-7 | C2 | |
| 80 | Target quaternion components | `TARGET_ATTITUDE` | Table 5-7 | C | |
| 81 | Target spin rate | `TARGET_SPINRATE` | Table 5-7 | C | |
| 82 | End maneuver data block | `MAN_STOP` | Table 5-7 | M | |
| | **Attitude determination logical block** | N/A | Table 5-8 | N/A | |
| 83 | Start of an attitude determination section | `AD_START` | Table 5-8 | M | |
| 84 | Comment | `COMMENT` | Table 5-8 | O | |
| 85 | Identification number for this attitude determination block | `AD_ID` | Table 5-8 | O | |
| 86 | Identification number for the previous attitude determination block | `AD_PREV_ID` | Table 5-8 | O | |
| 87 | Type of attitude determination method | `AD_METHOD` | Table 5-8 | O | |
| 88 | Source of attitude estimate | `ATTITUDE_SOURCE` | Table 5-8 | O | |
| 89 | Number of states | `NUMBER_STATES` | Table 5-8 | O | |
| 90 | Type of attitude data. | `ATTITUDE_STATES` | Table 5-8 | M | |
| 91 | Rotation sequence | `EULER_ROT_SEQ` | Table 5-8 | C | |
| 92 | | `COV_TYPE` | Table 5-8 | O | |
| 93 | | `REF_FRAME_A` | Table 5-8 | M | |
| 94 | | `REF_FRAME_B` | Table 5-8 | M | |
| 95 | | `RATE_STATES` | Table 5-8 | O | |
| 96 | | `SIGMA_U` | Table 5-8 | O | |
| 97 | | `SIGMA_V` | Table 5-8 | O | |
| 98 | | `RATE_PROCESS_NOISE_STDDEV` | Table 5-8 | O | |
| 99 | | `SENSOR_START` | Table 5-8 | O | |
| 100| | `SENSOR_NUMBER` | Table 5-8 | O | |
| 101| | `SENSOR_USED` | Table 5-8 | O | |
| 102| | `NUMBER_SENSOR_NOISE_COVARIANCE` | Table 5-8 | O | |
| 103| | `SENSOR_NOISE_STDDEV` | Table 5-8 | O | |
| 104| | `SENSOR_FREQUENCY` | Table 5-8 | O | |
| 105| | `SENSOR_STOP` | Table 5-8 | O | |
| 106| | `AD_STOP` | Table 5-8 | M | |
| | | N/A | Table 5-9 | N/A | |
| 107| | `USER_START` | Table 5-9 | M | |
| 108| | `COMMENT` | Table 5-9 | O | |
| 109| As defined by user, 'essential information that cannot be conveyed in comment statements' | `USER_DEFINED_x` | Table 5-9 | M | |
| 110| User-defined parameters block end | `USER_STOP` | Table 5-9 | M | |

C1: If 50 then M.
C2: If 78 then M.


# ANNEX B

## VALUES FOR SELECTED KEYWORDS (NORMATIVE)

The values in this annex represent the recommended values for selected keywords. Each keyword’s values delineated here are present in either an APM, AEM, or ACM message. For details and descriptions of the keyword interpretations, the reader is directed to annex H ([H2]). If exchange partners wish to use different settings, they should be documented in an ICD.

These values are stored on the SANA Registry, globally accessible on the CCSDS SANA registry website (see reference [9]).

It should be noted that the message creator or recipient may wish to automate processing of SANA registry normative content, which can be done by ingesting and processing of such content in electronic format. These formats can be accessed via the ‘Actions’ link on each registry; for example, for the Time Systems registry, a Comma Separated Value (CSV) format can be exported at: https://www.sanaregistry.org/r/time_systems?_export=csv and a JavaScript Object Notation (JSON) format at: https://www.sanaregistry.org/r/time_systems?_export=json.

Exchange partners may submit additional (new) keyword values for consideration of future inclusion into the SANA registry by submitting a detailed email request (mailto:info@sanaregistry.org). The CCSDS Area or Working Group responsible for the maintenance of the ADM at the time of the request is the approval authority. Until a submitted value is included in the SANA registry, exchange partners may define and use such values if mutually agreed between message exchange partners.

### B1 MESSAGE ORIGINATORS

The set of recommended values for the `ORIGINATOR` keyword is enumerated in the SANA Registry of Organizations, located at:

https://sanaregistry.org/r/organizations.

The preferred value is the ‘Abbreviation’ of the Agency Name.

### B2 TIME_SYSTEM METADATA KEYWORD

The value associated with this keyword should be selected from the full set of allowed values enumerated in the SANA Registry:

https://sanaregistry.org/r/time_systems.

### B3 REF_FRAME KEYWORD VALUES

This section describes the allowable keywords for reference frames that can be used by ADM messages. They are valid for keywords `REF_FRAME_*` in the APM, AEM, and ACM messages, where ‘\*’ denotes ‘A’ or ‘B’ and for the keywords `ANGVEL_FRAME`, `INERTIA_REF_FRAME`, and `MAN_REF_FRAME`.

The value associated with these keywords should be selected from the full set of allowed values enumerated in one of the following SANA Registries:

*   https://sanaregistry.org/r/celestial_body_reference_frames;
*   https://sanaregistry.org/r/orbit_relative_reference_frames;
*   https://sanaregistry.org/r/spacecraft_body_reference_frames.

### B4 ATTITUDE AND RATE TYPES

The following table enumerates the allowed values for the keywords associated with `ATT_TYPE` and `RATE_TYPE` in the ACM.

| Keyword Value | Meaning/Description |
| :--- | :--- |
| `QUATERNION` | Coordinate transformation represented as a quaternion, with 4 elements. The scalar element is always last. Units are ‘dimensionless’. |
| `EULER_ANGLES` | Coordinate transformation represented with 3 successive rotations. Units are ‘deg’. |
| `DCM` | Coordinate transformation represented as a 3 × 3 matrix. Included as 9 elements listed by columns. First 3 numbers are column one; second 3 are column two; third 3 are column three. Units are ‘dimensionless’. |
| `ANGVEL` | Angular velocity vector. Contains 3 elements. Units are ‘deg/s’. |
| `Q_DOT` | Rate of change of the quaternion. Contains 4 elements. Units are ‘1/s’. |
| `EULER_RATE` | Time derivative of the Euler angles. Contains 3 elements. Units are ‘deg/s’. |
| `GYRO_BIAS` | Correction to gyro estimated angular velocity. Contains 3 elements. Units are ‘deg/s’. |

### B5 ESTIMATOR TYPES

The following table enumerates the allowed values for the keyword `AD_METHOD` in the ACM:

| Keyword Value | Meaning/Description |
| :--- | :--- |
| `EKF` | Extended Kalman Filter, a sequential estimation algorithm applied to spacecraft attitude determination. Often additional state vector components are included, such as gyro biases or angular velocity. |
| `TRIAD` | TRIAxial Attitude Determination, an algebraic method for determination of spacecraft attitude from a set of two vector observations. |
| `QUEST` | QUaternion ESTimator, an efficient, deterministic algorithm to estimate a spacecraft attitude quaternion. |
| `BATCH` | A batch least squares algorithm to estimate spacecraft attitude, and optionally additional sensor parameters such as alignments, biases, scale factors. |
| `Q_METHOD` | Considered the best deterministic algorithm to estimate a spacecraft attitude quaternion. Requires use of an eigenvalue decomposition algorithm. (See reference [H3].) |
| `FILTER_SMOOTHER` | A method to smooth noisy processes. Several smoothing approaches exist such as fixed-point, fixed-lag, and fixed-interval. Used in ground applications to produce fine attitude estimates for post-processing applications. |

### B6 COVARIANCE MATRIX TYPES

This section describes the allowable keywords for covariance matrix types that can be used by ACM messages.

| Keyword Value | Meaning/Description |
| :--- | :--- |
| `ANGLE` | The diagonal elements of a 3 × 3 matrix containing angular errors about each spacecraft axis. Units are deg². |
| `ANGLE_GYROBIAS` | The diagonal elements of a 6 × 6 matrix containing angular errors about each spacecraft axis and gyro bias errors. Units are deg² for the angular errors and (deg/s)² for the gyro bias errors. |
| `ANGLE_ANGVEL` | The diagonal elements of a 6 × 6 matrix containing angular errors about each spacecraft axis and angular velocity errors. Units are deg² for the angular errors and (deg/s)² for the angular velocity errors. |
| `QUATERNION` | The diagonal elements of a 4 × 4 matrix containing quaternion errors. Units are ‘dimensionless’ for the quaternion errors. Quaternion order is as specified in the APM (table 3-3). |
| `QUATERNION_GYROBIAS` | The diagonal elements of a 7 × 7 matrix containing quaternion errors and gyro bias errors. Units are ‘dimensionless’ for the quaternion errors and (deg/s)² for the gyro bias errors. Quaternion order is as specified in the APM (table 3-3). |
| `QUATERNION_ANGVEL` | The diagonal elements of a 7 × 7 matrix containing quaternion errors and angular velocity errors. Units are ‘dimensionless’ for the quaternion errors and (deg/s)² for the angular velocity errors. Quaternion order is as specified in the APM (table 3-3). |

### B7 NORMATIVE REFERENCES FOR ATTITUDE AND SPACECRAFT CONVENTIONS

Attitude and Spacecraft Conventions are defined in the following SANA registry:

https://sanaregistry.org/r/attitude_and_spacecraft_conventions.

### B8 ORBIT CENTER KEYWORD VALUES

A set of allowed values for the reference frame center keywords (`CENTER_NAME` for APM, AEM, and ACM) is enumerated in the SANA Registry of Orbit Centers, located at:

https://sanaregistry.org/r/orbit_centers.


# ANNEX C

## SECURITY, SANA, AND PATENT CONSIDERATIONS (INFORMATIVE)

### C1 SECURITY CONSIDERATIONS

#### C1.1 ANALYSIS OF SECURITY CONSIDERATIONS

This subsection presents the results of an analysis of security considerations applied to the technologies specified in this Recommended Standard.

#### C1.2 CONSEQUENCES OF NOT APPLYING SECURITY TO THE TECHNOLOGY

The consequences of not applying security to the systems and networks on which this Recommended Standard is implemented could include potential loss, corruption, and theft of data. Because these messages are used in spacecraft attitude analyses and potential maneuvers, the consequences of not applying security to the systems and networks on which this Recommended Standard is implemented could include compromise or loss of the mission if malicious tampering of a particularly severe nature occurs.

#### C1.3 POTENTIAL THREATS AND ATTACK SCENARIOS

Potential threats or attack scenarios include, but are not limited to, (a) unauthorized access to the programs/processes that generate and interpret the messages and (b) unauthorized access to the messages during transmission between exchange partners. Protection from unauthorized access during transmission is especially important if the mission utilizes open ground networks, such as the Internet, to provide ground-station connectivity for the exchange of data formatted in compliance with this Recommended Standard. It is strongly recommended that potential threats or attack scenarios applicable to the systems and networks on which this Recommended Standard is implemented be addressed by the management of those systems and networks.

#### C1.4 DATA PRIVACY

Privacy of data formatted in compliance with the specifications of this Recommended Standard should be assured by the systems and networks on which this Recommended Standard is implemented.

#### C1.5 DATA INTEGRITY

Integrity of data formatted in compliance with the specifications of this Recommended Standard should be assured by the systems and networks on which this Recommended Standard is implemented.

#### C1.6 AUTHENTICATION OF COMMUNICATING ENTITIES

Authentication of communicating entities involved in the transport of data which complies with the specifications of this Recommended Standard should be provided by the systems and networks on which this Recommended Standard is implemented.

#### C1.7 DATA TRANSFER BETWEEN COMMUNICATING ENTITIES

The transfer of data formatted in compliance with this Recommended Standard between communicating entities should be accomplished via secure mechanisms approved by the Information Technology Security functionaries of exchange participants.

#### C1.8 CONTROL OF ACCESS TO RESOURCES

Control of access to resources should be managed by the systems upon which originator formatting and recipient processing are performed.

#### C1.9 AUDITING OF RESOURCE USAGE

Auditing of resource usage should be handled by the management of systems and networks on which this Recommended Standard is implemented.

#### C1.10 UNAUTHORIZED ACCESS

Unauthorized access to the programs/processes that generate and interpret the messages should be prohibited in order to minimize potential threats and attack scenarios.

#### C1.11 DATA SECURITY IMPLEMENTATION SPECIFICS

Specific information-security interoperability provisions that may apply between agencies and other independent users involved in an exchange of data formatted in compliance with this Recommended Standard could be specified in an ICD.

### C2 SANA CONSIDERATIONS

The following ADM related items will be registered with the SANA Operator. The registration rule for new entries in the registry is the approval of new requests by the CCSDS Area or Working Group responsible for maintenance of the ADM at the time of the request. New requests for this registry should be sent to SANA (mailto:info@sanaregistry.org).

*   The ADM XML schema.
*   Values for various keywords or conventions from the following SANA registries:
    *   https://sanaregistry.org/r/time_systems;
    *   https://sanaregistry.org/r/orbit_centers;
    *   https://sanaregistry.org/r/celestial_body_reference_frames;
    *   https://sanaregistry.org/r/orbit_relative_reference_frames;
    *   https://sanaregistry.org/r/spacecraft_body_reference_frames;
    *   https://sanaregistry.org/r/attitude_and_spacecraft_conventions;
    *   https://sanaregistry.org/r/organzations.

### C3 PATENT CONSIDERATIONS

The recommendations of this document have no patent issues.


# ANNEX D

## ABBREVIATIONS AND ACRONYMS (INFORMATIVE)

| | |
| :--- | :--- |
| ASCII | American Standard Code for Information Interchange |
| ACM | Attitude Comprehensive Message |
| ADM | Attitude Data Message |
| AEM | Attitude Ephemeris Message |
| APM | Attitude Parameter Message |
| AST | autonomous star tracker |
| CCSDS | Consultative Committee for Space Data Systems |
| COSPAR | Committee for Space Research |
| CP | center of pressure |
| CSS | coarse sun sensor |
| CUI | controlled unclassified information |
| DCM | direction cosine matrix |
| DSS | digital sun sensor |
| EKF | extended Kalman filter |
| EME2000 | Earth Mean Equator and Equinox of J2000 (Julian Date 2000) |
| GPS | Global Positioning System |
| IAU | International Astronomical Union |
| ICD | interface control document |
| ICS | implementation conformance statement |
| ICRF | International Celestial Reference Frame |
| IEC | International Electrotechnical Commission |
| IEEE | Institute of Electrical and Electronics Engineers |
| IMU | inertial measurement unit |
| ISO | International Organization for Standardization |
| ITRF | International Terrestrial Reference Frame |
| KVN | ‘keyword = value’ notation |
| LVLH | local vertical local horizontal |
| NDM | Navigation Data Message |
| OCM | Orbit Comprehensive Message |
| ODM | Orbit Data Message |
| OEM | Orbit Ephemeris Message |
| OPM | Orbit Parameter Message |
| POC | point of contact |
| QSW | same as RTN |
| RL | requirements list |
| RTN | radial, tangential, normal |
| RWA | reaction wheel assembly |
| SBU | sensitive but unclassified |
| TAI | International Atomic Time |
| URL | uniform resource locator |
| UTC | Coordinated Universal Time |
| XML | eXtensible Markup Language |


# ANNEX E

## RATIONALE FOR THIS STANDARD (INFORMATIVE)

### E1 OVERVIEW

This annex presents the rationale behind the design of each message. It may help the application engineer to select a suitable message. Corrections and/or additions to these requirements are expected during future updates.

A specification of requirements agreed to by all parties is essential to focus design and to ensure the product meets the needs of the Member Agencies. There are many ways of organizing requirements, but the categorization of requirements is not as important as the agreement to a sufficiently comprehensive set. In this annex, the requirements are organized into three categories:

a) Primary Requirements: These are the most elementary and necessary requirements. They would exist no matter the context in which the CCSDS is operating, that is, regardless of pre-existing conditions within the CCSDS or its Member Agencies.
b) Heritage Requirements: These are additional requirements that derive from pre-existing Member Agency requirements, conditions, or needs. Ultimately these carry the same weight as the Primary Requirements. This Recommended Standard reflects heritage requirements pertaining to some of the technical participants’ home institutions collected during the preparation of the document; it does not speculate on heritage requirements that could arise from other Member Agencies.
c) Desirable Characteristics: These are not requirements, but they are felt to be important or useful features of the Recommended Standard.

### E2 REQUIREMENTS ACCEPTED BY THE ATTITUDE DATA MESSAGES

**Table E-1: Primary Requirements**

| | Requirement | Accepted for APM? | Accepted for AEM? | Accepted for ACM? |
| :--- | :--- | :--- | :--- | :--- |
| PR-1 | Data must be provided in digital form (computer file). | Y | Y | Y |
| PR-2 | The file specification must not require of the receiving agency the separate application of, or modeling of, spacecraft dynamics or gravitational force models, or integration or propagation. | N | Y | Y |
| PR-3 | The interface must facilitate the receiver of the message to generate an attitude state at any required epoch. | Y | Y | Y |
| PR-4 | Attitude state information must be provided in a reference frame that is clearly identified and unambiguous. | Y | Y | Y |
| PR-5 | Identification of the object must be clearly identified and unambiguous. | Y | Y | Y |
| PR-6 | The time bounds of the attitude ephemeris must be unambiguously specified. | Y | Y | Y |
| PR-7 | The standard must provide for clear specification of units of measure. | Y | Y | Y |
| PR-8 | Files must be readily ported between, and useable within, all Member Agency computational environments that could be used to exchange Attitude Data Messages. | Y | Y | Y |
| PR-9 | Files must have means of being uniquely identified and clearly annotated. The file name alone is considered insufficient for this purpose. | Y | Y | Y |
| PR-10 | File name syntax and length must not violate computer constraints for those Member Agency computing environments that could be used to exchange Attitude Data Messages. | Y | Y | Y |

**Table E-2: Heritage Requirements**

| | Requirement | Accepted for APM? | Accepted for AEM? | Accepted for ACM? |
| :--- | :--- | :--- | :--- | :--- |
| HR-1 | A complete attitude ephemeris, not subject to integration or propagation by the customer, must be provided. | N | Y | Y |
| HR-2 | The standard is, or includes, an ASCII format. | Y | Y | Y |
| HR-3 | The standard does not require software supplied by other agencies. | Y | Y | Y |

**Table E-3: Desirable Characteristics**

| | Requirement | Accepted for APM? | Accepted for AEM? | Accepted for ACM? |
| :--- | :--- | :--- | :--- | :--- |
| DC-1 | The standard applies to non-traditional objects, such as landers, rovers, balloons, and natural bodies (asteroids, comets). | Y | Y | Y |
| DC-2 | The standard allows attitude states to be provided in other than the traditional EME2000 inertial reference frame; one example is the International Astronomical Union (IAU) Mars body-fixed frame. (In such a case, provision or ready availability of supplemental information needed to transform data into a standard frame must be arranged.) | Y | Y | Y |
| DC-3 | The standard is extensible with no disruption to existing users or uses. | Y | Y | Y |
| DC-4 | The standard is as consistent as reasonable with any related CCSDS attitude standards used for Earth-to-spacecraft or spacecraft-to-spacecraft applications. | Y | Y | Y |
| DC-5 | The standard allows for the specification of the accuracy of the attitude solution. | N | N | Y |

### E3 APPLICABILITY OF CRITERIA TO MESSAGE OPTIONS

The selection of one particular message will depend on the optimization criteria in the given application. Table E-4 compares the three recommended messages in terms of the relevant selection criteria identified by the CCSDS:

**Table E-4: Applicability of the Criteria to Attitude Data Messages**

| Criteria | Definition | Applicable to APM? | Applicable to AEM? | Applicable to ACM? |
| :--- | :--- | :--- | :--- | :--- |
| Modeling Fidelity | Permits modeling of any dynamic perturbation to the attitude. | N | Y | Y |
| Human Readability | Provides easily readable message corresponding to widely used attitude representations. | Y | Y | Y |
| Remote Body Extensibility | Permits use for assets on remote solar system bodies. | Y | Y | Y |
| Lander/Rover Compatibility | Permits exchange of non-orbit attitudes. | Y | Y | Y |

### E4 SERVICES RELATED TO THE DIFFERENT ATTITUDE DATA MESSAGE FORMATS

The different Attitude Data Messages have been distinguished by their self-interpretability. All Attitude Data Messages provide for recognizing the boundaries of the attitude data fields and thus can transfer each field, as a block, to another location. The different services that can be achieved without special arrangements between users of the CCSDS Attitude Data Messages are listed in table E-5.

**Table E-5: Services Available with Attitude Data Messages**

| Service | Definition | Applicable to APM? | Applicable to AEM? | Applicable to ACM? |
| :--- | :--- | :--- | :--- | :--- |
| Absolute Attitude Interpretation | State availability at specific times for use in additional computations (geometry, event detection, etc.). | Y | Y | Y |
| Relative Attitude Interpretation | Trajectory comparison and differencing for events based on the same time source. | Only at time specified at Epoch | Y | Y |


# ANNEX F

## TECHNICAL MATERIAL AND CONVENTIONS (INFORMATIVE)

### F1 OVERVIEW

This annex details the conventions used in this document for the definition of attitude data.

### F2 QUATERNIONS

#### F2.1 DESCRIPTION

The quaternion called ‘from frame A to frame B’ is defined as the quaternion of the rotation that transforms the basis vectors of frame A into the basis vectors of frame B. That is to say that the basis vectors of frame B are the respective images of the basis vectors of frame A by the rotation.

The quaternion is defined by four components:

$Q_1 = sin(φ/2) * e_1$;
$Q_2 = sin(φ/2) * e_2$;
$Q_3 = sin(φ/2) * e_3$;
$Q_c = cos(φ/2)$;

where:

*   φ is the rotation angle; and
*   $e_1, e_2$, and $e_3$ are the coordinates of the rotation axis in either frame A or frame B.

The quaternion is related to the frame transformation matrix in the following way:

With $X_A$ being the coordinates of some vector in frame A, and $X_B$ being the coordinates of the same vector in frame B, the frame transformation matrix $M_{BA}$ that transforms coordinates in frame A to coordinates in frame B is defined by

$X_B = M_{BA} * X_A$,

where $M_{BA}$ is a function of the quaternion components

$$ M_{BA} = 
\begin{bmatrix}
Q_1^2 - Q_2^2 - Q_3^2 + Q_c^2 & 2(Q_1 Q_2 + Q_3 Q_c) & 2(Q_1 Q_3 - Q_2 Q_c) \\
2(Q_1 Q_2 - Q_3 Q_c) & -Q_1^2 + Q_2^2 - Q_3^2 + Q_c^2 & 2(Q_2 Q_3 + Q_1 Q_c) \\
2(Q_1 Q_3 + Q_2 Q_c) & 2(Q_2 Q_3 - Q_1 Q_c) & -Q_1^2 - Q_2^2 + Q_3^2 + Q_c^2
\end{bmatrix}
$$ 

#### F2.2 EXAMPLE

With Frame B being obtained from frame A by a rotation of axis Z and angle +90 degrees, so that the X axis of frame B has coordinates 0, 1, 0 in frame A, the quaternion ‘from A to B’ is then:

$Q_1 = 0, Q_2 = 0, Q_3 = 0.7071, Q_c = 0.7071$;

$M_{BA}$ is equal to:

$$ M_{BA} =
\begin{bmatrix}
0 & 1 & 0 \\
-1 & 0 & 0 \\
0 & 0 & 1
\end{bmatrix}
$$ 

the vector of coordinates $X_A = [1, 0, 0]$ in frame A has coordinates $X_B = [0, -1, 0]$ in frame B.

### F3 EULER ANGLES

#### F3.1 DESCRIPTION

The Euler angles called ‘from frame A to frame B’ are the rotation angles of the 3 successive intrinsic rotations that transform frame A into frame B.

With $θ_1, θ_2, θ_3$ being the three rotation angles, and $a_1, a_2, a_3$ being the respective rotation axes (X axis, Y axis, or Z axis), the images of the basis vectors of frame A by the three successive rotations of angle $θ_1$ and axis $a_1$, angle $θ_2$ and axis $a_2$, and angle $θ_3$ and axis $a_3$ are the respective basis vectors of frame B.

#### F3.2 EXAMPLE

With the three successive rotations around axes $a_1 = X, a_2 = Y, a_3 = Z$ of respective angles $θ_1, θ_2, θ_3$, the frame transformation matrix M can be defined such that

$X_B = M_{BA} * X_A$,

where $X_A$ denotes the coordinates of some vector in frame A, and $X_B$ the coordinates of the same vector in frame B.

That results in

$$ M_{BA} = 
\begin{bmatrix}
cosθ_3 & sinθ_3 & 0 \\
-sinθ_3 & cosθ_3 & 0 \\
0 & 0 & 1
\end{bmatrix}
\begin{bmatrix}
cosθ_2 & 0 & -sinθ_2 \\
0 & 1 & 0 \\
sinθ_2 & 0 & cosθ_2
\end{bmatrix}
\begin{bmatrix}
1 & 0 & 0 \\
0 & cosθ_1 & sinθ_1 \\
0 & -sinθ_1 & cosθ_1
\end{bmatrix}
$$ 

### F4 ANGULAR VELOCITY VECTOR

The angular velocity vector from frame A to frame B represents the angular velocity vector of frame B with respect to frame A.

The components can be defined either in frame A or frame B.

### F5 SPIN DATA

#### F5.1 GENERAL

The spin data enable the user of the message to propagate the attitude of an object using a simple model. In this model, the body frame rotates at a constant rate about the spin axis, and the spin axis rotates at a constant rate about the angular momentum vector.

Frame B represents the body frame, and frame A represents the reference frame with respect to which the orientation of the body frame is defined.

In addition, the spin axis is the Z axis of frame B and is a principal axis.

The spin data enable determination of the attitude at some reference epoch and description of how the attitude will evolve in time.

#### F5.2 ATTITUDE AT REFERENCE EPOCH

The attitude at the reference epoch is described by $α_o, δ_o$, and $σ_o$, with the correspondence:

| Notation | Corresponds to (APM/AEM data) |
| :--- | :--- |
| $α_o$ | SPIN_ALPHA |
| $δ_o$ | SPIN_DELTA |
| $σ_o$ | SPIN_ANGLE |

The attitude (orientation of frame B) at the reference epoch is defined as the result of 3 successive rotations of respective angles $α_o + π/2, π/2 – δ_o$, and $σ_o$ around the successive axes Z, X, Z starting from frame A.

The Z axis of frame B (spin axis) has then the following coordinates in frame A:

*   $cos(δ_o) cos(α_o)$ along X axis;
*   $cos(δ_o) sin(α_o)$ along Y axis;
*   $sin(δ_o)$ along Z axis.

$α_o$ is the angle in the X-Y plane from the X axis, and $δ_o$ is the angle with the X-Y plane (usual spherical coordinates).

#### F5.3 ATTITUDE PROPAGATION

The attitude as given in F5.2 can be propagated using additional data. Two sets of data can be used:

a) `MOMENTUM_ALPHA`, `MOMENTUM_DELTA`, `NUTATION_VEL`:
   *   `MOMENTUM_ALPHA` is the right ascension of the angular momentum vector in frame A.
   *   `MOMENTUM_DELTA` is the declination of the angular momentum vector in frame A.
   *   `NUTATION_VEL` is the angular velocity of the spin axis around the angular momentum vector.
b) `NUTATION`, `NUTATION_PHASE`, `NUTATION_PERIOD`
   *   `NUTATION` is the angle between the principal axis (spin axis) and the angular momentum vector.
   *   `NUTATION_PHASE` describes the initial orientation of the spin axis in its motion around the angular momentum vector (see below).
   *   `NUTATION_PERIOD` gives the period of the motion of the spin axis around the angular momentum vector.

`NUTATION_PHASE` is defined such that the coordinates of the angular momentum vector (unit vector) at the reference epoch in frame B are:

*   $X_B = sin(NUTATION) * cos(NUTATION_PHASE)$ along the X axis of frame B;
*   $Y_B = -sin(NUTATION) * sin(NUTATION_PHASE)$ along the Y axis of frame B;
*   $Z_B = cos(NUTATION)$ along the Z axis of frame B;

which gives:

`NUTATION = 90 – asin(ZB)`
`NUTATION_PHASE = -atan(YB / XB)`

where ‘asin’ and ‘atan’ are supposed to return a value in degrees.

For `NUTATION_PERIOD` = 360 / `NUTATION_VEL` (assuming `NUTATION_VEL` is in deg/s), the 2 possibilities are equivalent as `NUTATION` and `NUTATION_PHASE` can be computed knowing the coordinates of the angular momentum vector in frame A (hence B) at the reference epoch.

To propagate the attitude to any epoch (reference epoch + Δt), the standard way is the following:

a) an intermediate inertial reference frame (F) is determined, fixed with respect to frame A, whose Z axis is in the same direction as the angular momentum vector;
b) the three angles $φ_o, θ$ (nutation angle) and $ψ_o$ are determined such that the attitude at the reference epoch is obtained by the following successive rotations, starting from frame F:
   *   rotation of angle $φ_o$ around Z,
   *   rotation of angle θ around X (the spin axis is then the Z axis of the current frame),
   *   rotation of angle $ψ_o$ around Z;
c) the attitude at any time can then be obtained with the three successive rotations starting from frame F:
   *   rotation of angle $φ_o + φ * Δt$ around Z,
   *   rotation of angle θ around X (the spin axis is then the Z axis of the current frame),
   *   rotation of angle $ψ_o + ψ * Δt$ around Z;

using the correspondence:

| Notation | Corresponds to (APM/AEM data) |
| :--- | :--- |
| φ | NUTATION_VEL |
| ψ | SPIN_ANGLE_VEL |

#### F5.4 EXAMPLE

The spin data are defined as follows:

| Keyword | Description | Value | Unit |
| :--- | :--- | :--- | :--- |
| `SPIN_ALPHA` | Right ascension of spin axis vector in frame A | 0 | deg |
| `SPIN_DELTA` | Declination of the spin axis vector in frame A | 80 | deg |
| `SPIN_ANGLE` | Phase of the satellite about the spin axis | 45 | deg |
| `SPIN_ANGLE_VEL`| Angular velocity of satellite around spin axis | 1 | deg/s |
| `MOMENTUM_ALPHA`| Right ascension of angular momentum vector in frame A | 0 | deg |
| `MOMENTUM_DELTA`| Declination of angular momentum vector in frame A | 70 | deg |
| `NUTATION_VEL` | Angular velocity of spin vector around the angular momentum vector | 0.01 | deg/s |

The following results should be obtained:

The quaternion at the reference epoch is given (using `SPIN_ALPHA`, `SPIN_DELTA` and `SPIN_ANGLE`) by:

$Q1 = 0.0805, Q2 = 0.0334, Q3 = 0.9204, QC = 0.3812$

The spin axis (which is the Z axis of frame B) at the reference epoch has coordinates in frame A:

$X = 0.1736, Y = 0, Z = 0.9848$

The angular momentum (unit vector) has coordinates in frame A:

$X = 0.3420, Y = 0, Z = 0.9397$

In this particular case, it is possible to choose the intermediate frame (‘F’) such that its Z axis has the same direction as the angular momentum vector, and its Y axis is the same as the Y axis of frame A (step 1).

The quaternion from frame A to frame F is then:

$Q1 = 0.0000, Q2 = 0.1736, Q3 = 0.0000, QC = 0.9848$

And the quaternion from frame F to frame B at initial epoch is:

$Q1 = -0.0805, Q2 = -0.0334, Q3 = 0.9204, QC = 0.3812$

Using the quaternion from frame F to frame B at initial epoch, the following quantities can be computed (step 2):

*   $φ_o = -90$ deg;
*   $θ = 10$ deg (nutation angle, which is constant);
*   $ψ_o = -135$ deg.

The quaternion from frame A to frame B at time t = reference epoch + 300 s is then given by (step 3):

$Q1 = 0.0584, Q2 = 0.0650, Q3 = 0.6263, QC = 0.7747$

The spin axis at time t has coordinates in frame A:

$X = 0.1739, Y = -0.0091, Z = 0.9847$

### F6 INERTIA DATA

Inertia data consist of:

*   moments of inertia (diagonal terms);
*   inertial cross products (off diagonal terms);

$$ I = 
\begin{bmatrix}
I_{xx} & -I_{xy} & -I_{xz} \\
-I_{xy} & I_{yy} & -I_{yz} \\
-I_{xz} & -I_{yz} & I_{zz}
\end{bmatrix}
$$ 

The cross-product terms are negative.

The inertia matrix is defined relative to a particular frame (defined by its axis and origin).


# ANNEX G

## EXAMPLES (INFORMATIVE)

### G1 APM EXAMPLES

This section contains examples of Attitude Parameter Messages.

Figure G-1 is a simple example with one quaternion.

```
CCSDS_APM_VERS = 2.0
CREATION_DATE = 2003-09-30T19:23:57
ORIGINATOR = GSFC
MESSAGE_ID = A701521

COMMENT GEOCENTRIC, CARTESIAN, EARTH FIXED
COMMENT OBJECT ID: 1997-074A
COMMENT $ITIM = 1997 NOV 21 22:26:18.40000000, $ original launch time
OBJECT_NAME = TRMM
OBJECT_ID = 1997-074A
CENTER_NAME = EARTH
TIME_SYSTEM = UTC

COMMENT Current attitude for orbit 335
COMMENT Attitude state quaternion
COMMENT Accuracy of this attitude is 0.02 deg RSS.

EPOCH = 2003-09-30T14:28:15.1172

QUAT_START
REF_FRAME_A = SC_BODY_1
REF_FRAME_B = ITRF1997
Q1 = 0.00005
Q2 = 0.87543
Q3 = 0.40949
QC = 0.25678
QUAT_STOP
```

**Figure G-1: APM Example with Quaternion**

Figure G-2 is a simple example with Euler angles.

```
CCSDS_APM_VERS = 2.0
CREATION_DATE = 2006-03-13T13:13:33
ORIGINATOR = GSFC
MESSAGE_ID = A701522

OBJECT_NAME = GOES-P
OBJECT_ID = 2006-003A
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
COMMENT GEOSYNCHRONOUS, CARTESIAN, EARTH FIXED

COMMENT OBJECT ID: 2006-003A
COMMENT $ITIM = 2006 FEB 5 03:23:45.60000000, $ original launch time
COMMENT Attitude given by Euler angles

EPOCH = 2006-03-12T09:56:39.4987

EULER_START
COMMENT Euler angles
REF_FRAME_A = BODY_FRAME_A
REF_FRAME_B = ITRF1997
EULER_ROT_SEQ = YXY
ANGLE_1 = -26.78 [deg]
ANGLE_2 = 46.26 [deg]
ANGLE_3 = 144.10 [deg]
EULER_STOP
```

**Figure G-2: APM File Example with Euler Angles**

Figure G-3 is a more complex example with several data blocks.

```
CCSDS_APM_VERS = 2.0
CREATION_DATE = 2004-02-14T19:23:57
ORIGINATOR = JPL
MESSAGE_ID = 900018

OBJECT_NAME = MARS_SPIRIT
OBJECT_ID = 2004-003A
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
COMMENT GEOCENTRIC, CARTESIAN, EARTH FIXED

COMMENT OBJECT ID: 2004-003
COMMENT $ITIM = 2004 JAN 14 22:26:18.400000, $ original launch time 14:36
COMMENT Generated by JPL
COMMENT Current attitude for orbit 20 and attitude maneuver
COMMENT planning data.

EPOCH = 2004-02-14T14:28:15.1172

QUAT_START
COMMENT Attitude state quaternion (ref frame = ITRF1997)
REF_FRAME_A = ITRF1997
REF_FRAME_B = INSTRUMENT_A
Q1 = 0.03123
Q2 = 0.78543
Q3 = 0.39158
QC = 0.47832
QUAT_STOP

QUAT_START
COMMENT Attitude state quaternion (ref frame = ICRF)
REF_FRAME_A = ICRF
REF_FRAME_B = INSTRUMENT_A
Q1 = 0.02478
Q2 = 0.78576
Q3 = 0.39552
QC = 0.47491
QUAT_STOP

INERTIA_START
COMMENT Spacecraft Inertia Parameters
INERTIA_REF_FRAME = SC_BODY_1
IXX = 6080.0 [kg*m**2]
IYY = 5245.5 [kg*m**2]
IZZ = 8067.3 [kg*m**2]
IXY = -135.9 [kg*m**2]
IXZ = 89.3 [kg*m**2]
IYZ = -90.7 [kg*m**2]
INERTIA_STOP

MAN_START
COMMENT Data follows for 1 planned maneuver.
COMMENT First attitude maneuver for: MARS_SPIRIT
COMMENT Impulsive, torque direction fixed in body frame
MAN_EPOCH_START = 2004-02-14T14:29:00.5098
MAN_DURATION = 3 [s]
MAN_REF_FRAME = ICRF
MAN_TOR_X = -1.25 [N*m]
MAN_TOR_Y = -0.5 [N*m]
MAN_TOR_Z = 0.5 [N*m]
MAN_STOP
```

**Figure G-3: APM File Example with Various Contents**

### G2 AEM EXAMPLES

This annex subsection contains examples of Attitude Ephemeris Messages.

Figure G-4 is an example of an AEM. It should be noted that some attitude ephemeris lines were omitted.

```
CCSDS_AEM_VERS = 2.0
CREATION_DATE = 2002-11-04T17:22:31
ORIGINATOR = NASA/JPL
MESSAGE_ID = A701523

META_START
COMMENT This file was produced by M.R. Somebody, MSOO NAV/JPL.
COMMENT It is to be used for attitude reconstruction only. The relative accuracy of these
COMMENT attitudes is 0.1 degrees per axis.
OBJECT_NAME = MARS_GLOBAL_SURVEYOR
OBJECT_ID = 1996-062A
CENTER_NAME = MARS BARYCENTER
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
TIME_SYSTEM = UTC
START_TIME = 1996-11-28T21:29:07.2555
USEABLE_START_TIME = 1996-11-28T22:08:02.5555
USEABLE_STOP_TIME = 1996-11-30T01:18:02.5555
STOP_TIME = 1996-11-30T01:28:02.5555
ATTITUDE_TYPE = QUATERNION
INTERPOLATION_METHOD = hermite
INTERPOLATION_DEGREE = 7
META_STOP

DATA_START
1996-11-28T21:29:07.2555 0.56748 0.03146 0.45689 0.68427
1996-11-28T22:08:03.5555 0.42319 -0.45697 0.23784 0.74533
1996-11-28T22:08:04.5555 -0.84532 0.26974 -0.06532 0.45652
...
1996-11-30T01:28:02.5555 0.74563 -0.45375 0.36875 0.31964
DATA_STOP

META_START
COMMENT This block begins after trajectory correction maneuver TCM-3.
OBJECT_NAME = mars_global_surveyor
OBJECT_ID = 1996-062A
CENTER_NAME = MARS_BARYCENTER
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
TIME_SYSTEM = UTC
START_TIME = 1996-12-18T12:05:00.5555
USEABLE_START_TIME = 1996-12-18T12:10:00.5555
USEABLE_STOP_TIME = 1996-12-28T21:23:00.5555
STOP_TIME = 1996-12-28T21:28:00.5555
ATTITUDE_TYPE = QUATERNION
META_STOP

DATA_START
1996-12-18T12:05:00.5555 -0.64585 0.018542 -0.43475 0.62501
1996-12-18T12:10:05.5555 0.87451 -0.23854 0.13458 0.16767
1996-12-18T12:10:10.5555 0.03125 -0.65874 0.23458 0.71418
...
1996-12-28T21:28:00.5555 -0.25485 0.58745 -0.36845 0.67394
DATA_STOP
```

**Figure G-4: AEM Example**

Figure G-5 is an example of an AEM describing a spinning spacecraft. It should be noted that some attitude ephemeris lines were omitted.

```
CCSDS_AEM_VERS = 2.0
CREATION_DATE = 2008-071T17:09:49
ORIGINATOR = GSFC
MESSAGE_ID = 7077456

META_START
OBJECT_NAME = ST5-224
OBJECT_ID = 2006-224A
CENTER_NAME = EARTH
REF_FRAME_A = J2000
REF_FRAME_B = SC_BODY_1
TIME_SYSTEM = UTC
START_TIME = 2006-090T05:00:00.071
USEABLE_START_TIME = 2006-090T05:00:00.071
USEABLE_STOP_TIME = 2006-090T05:00:00.946
STOP_TIME = 2006-090T05:00:00.946
ATTITUDE_TYPE = SPIN
META_STOP

DATA_START
COMMENT Spin KF ground solution, SPINKF rates
2006-090T05:00:00.071 2.6862511e+002 6.8448486e+001 1.5969509e+002 -1.0996528e+002
2006-090T05:00:00.196 2.6863990e+002 6.8432197e+001 1.4593720e+002 -1.0996493e+002
2006-090T05:00:00.321 2.6864591e+002 6.8412960e+001 1.3218766e+002 -1.0996455e+002
2006-090T05:00:00.446 2.6863697e+002 6.8392049e+001 1.1845280e+002 -1.0996402e+002
2006-090T05:00:00.571 2.6861072e+002 6.8371266e+001 1.0473305e+002 -1.0996370e+002
2006-090T05:00:00.696 2.6856625e+002 6.8353279e+001 9.1030304e+001 -1.0996339e+002
2006-090T05:00:00.821 2.6850631e+002 6.8340398e+001 7.7341548e+001 -1.0996317e+002
2006-090T05:00:00.946 2.6843571e+002 6.8332398e+001 6.3662262e+001 -1.0996304e+002
DATA_STOP
```

**Figure G-5: AEM Spinner Example**

### G3 ACM EXAMPLES

This annex subsection contains examples of Attitude Comprehensive Messages.

Figure G-6 shows an example with a time history of attitude states; it constitutes a minimal content ACM.

```
CCSDS_ACM_VERS = 2.0
CREATION_DATE = 1998-11-06T09:23:57
ORIGINATOR = JAXA
MESSAGE_ID = A701524

META_START
OBJECT_NAME = EUROBIRD-4A
INTERNATIONAL_DESIGNATOR = 2000-052A
TIME_SYSTEM = UTC
EPOCH_TZERO = 1998-12-18T14:28:15.1172
META_STOP

ATT_START
REF_FRAME_A = J2000
REF_FRAME_B = SC_BODY_1
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0.0 0.73566 -0.50547 0.41309 0.180707
0.25 0.73529 -0.50531 0.41375 0.181158
0.50 0.73492 -0.50515 0.41441 0.181610
< additional data records omitted here >
ATT_STOP
```

**Figure G-6: Simple/Succinct ACM File Example**

Figure G-7 is an example of ACM which includes a maneuver with associated attitude history.

```
CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2017-12-01T00:00:00
ORIGINATOR = NASA
MESSAGE_ID = A701525

META_START
OBJECT_NAME = SDO
INTERNATIONAL_DESIGNATOR = 2010-005A
TIME_SYSTEM = UTC
EPOCH_TZERO = 2017-12-26T19:40:00.000
META_STOP

ATT_START
COMMENT OBC Attitude and Bias during momentum management maneuver
REF_FRAME_A = J2000
REF_FRAME_B = SC_BODY_1
NUMBER_STATES = 7
ATT_TYPE = QUATERNION
RATE_TYPE = GYRO_BIAS
0.000000 0.1153 -0.1424 0.8704 0.4571 2.271e-06 -4.405e-06 -3.785e-06
2.000000 0.1153 -0.1424 0.8704 0.4571 2.271e-06 -4.405e-06 -3.785e-06
< intervening data records omitted here >
99.80183 0.1017 -0.1332 0.8806 0.4433 2.587e-06 8.769e-06 5.436e-06
< intervening data records omitted here >
599.80275 0.1152 -0.1423 0.8704 0.4571 2.48e-06 -4.350e-06 -3.779e-06
ATT_STOP

MAN_START
COMMENT Momentum management maneuver
MAN_PURPOSE = MOM_DESAT
MAN_BEGIN_TIME = 100.0
MAN_DURATION = 450.0
ACTUATOR_USED = ATT-THRUSTER
TARGET_MOMENTUM = 1.30 -16.400 -11.350
TARGET_MOM_FRAME = J2000
MAN_STOP

AD_START
COMMENT SDO Onboard Filter
AD_METHOD = EKF
ATTITUDE_SOURCE = OBC
ATTITUDE_STATES = QUATERNION
REF_FRAME_A = J2000
REF_FRAME_B = SC_BODY_1
SENSOR_START
SENSOR_NUMBER = 1
SENSOR_USED = AST
SENSOR_STOP
SENSOR_START
SENSOR_NUMBER = 2
SENSOR_USED = AST
SENSOR_STOP
SENSOR_START
SENSOR_NUMBER = 5
SENSOR_USED = DSS
SENSOR_STOP
SENSOR_START
SENSOR_NUMBER = 6
SENSOR_USED = IMU
SENSOR_STOP
AD_STOP
```

**Figure G-7: ACM Example with Attitude State Time History, Maneuver Specification, and Attitude Determination Data**

Figure G-8 is an example of ACM which includes object’s physical characteristics.

```
CCSDS_ACM_VERS = 2.0
CREATION_DATE = 1998-11-06T09:23:57
ORIGINATOR = JAXA
MESSAGE_ID = A701526

META_START
OBJECT_NAME = TEST_SAT
ORIGINATOR_POC = Ms. Rodgers, (719)555-5555, email@email.XXX
TIME_SYSTEM = TAI
EPOCH_TZERO = 1998-12-18T14:28:15.1172
TAIMUTC_AT_TZERO = 36 [s]
META_STOP

PHYS_START
COMMENT Spacecraft Physical Parameters
WET_MASS = 1916 [kg]
CP_REF_FRAME = SC_BODY_1
CP = 0.04 -0.78 -0.023 [m]
IXX = 752 [kg*m**2]
IYY = 1305 [kg*m**2]
IZZ = 1490 [kg*m**2]
IXY = 81.1 [kg*m**2]
IXZ = -25.7 [kg*m**2]
IYZ = 74.1 [kg*m**2]
PHYS_STOP
```

**Figure G-8: Example Space Object Physical Characteristics**

Figure G-9 is an example with attitude state covariance time history and attitude determination data.

```
CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2017-12-30T00:00:00
ORIGINATOR = NASA
MESSAGE_ID = A701527

META_START
OBJECT_NAME = LRO
INTERNATIONAL_DESIGNATOR = 2009-031A
TIME_SYSTEM = UTC
EPOCH_TZERO = 2017-12-30T00:00:00.0
ACM_DATA_ELEMENTS = COV, AD
META_STOP

COV_START
COMMENT Diagonal Covariance for LRO Onboard Kalman Filter
COV_BASIS = DETERMINED_OBC
COV_REF_FRAME = SC_BODY_1
COV_TYPE = ANGLE_GYROBIAS
0.0 6.74E-11 8.10E-11 9.22E-11 1.11E-15 1.11E-15 1.12E-15
1.096694 6.74E-11 8.10E-11 9.22E-11 1.11E-15 1.11E-15 1.12E-15
< intervening data records omitted here >
59.896697 6.74E-11 8.10E-11 9.22E-11 1.11E-15 1.11E-15 1.12E-15
COV_STOP

AD_START
COMMENT LRO Onboard Filter, A Multiplicative Extended Kalman Filter
AD_METHOD = EKF
ATTITUDE_SOURCE = OBC
NUMBER_STATES = 7
ATTITUDE_STATES = QUATERNION
COV_TYPE = ANGLE_GYROBIAS
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
RATE_STATES = GYRO_BIAS
SENSOR_START
SENSOR_NUMBER = 2
SENSOR_USED = AST
SENSOR_STOP
SENSOR_START
SENSOR_NUMBER = 4
SENSOR_USED = AST
SENSOR_STOP
SENSOR_START
SENSOR_NUMBER = 7
SENSOR_USED = IMU
SENSOR_STOP
AD_STOP
```

**Figure G-9: ACM Example with Attitude State Covariance Time History and Attitude Determination Data**

### G4 EXAMPLES IN XML (APM, AEM, ACM)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<apm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-3.0.0-master-3.0.xsd"
   xmlns:ndm="urn:ccsds:schema:ndmxml"
   id="CCSDS_APM_VERS" version="2.0">
  <header>
    <CREATION_DATE>2003-09-30T19:23:57</CREATION_DATE>
    <ORIGINATOR>GSFC</ORIGINATOR>
    <MESSAGE_ID>A701521</MESSAGE_ID>
  </header>
  <body>
    <segment>
      <metadata>
        <COMMENT>GEOCENTRIC, CARTESIAN, EARTH FIXED</COMMENT>
        <COMMENT>OBJECT ID: 1997-074A</COMMENT>
        <COMMENT>$ITIM = 1997 NOV 21 22:26:18.40000000, $ original launch time</COMMENT>
        <OBJECT_NAME>TRMM</OBJECT_NAME>
        <OBJECT_ID>1997-074A</OBJECT_ID>
        <CENTER_NAME>EARTH</CENTER_NAME>
        <TIME_SYSTEM>UTC</TIME_SYSTEM>
      </metadata>
      <data>
        <COMMENT>Current attitude for orbit 335</COMMENT>
        <COMMENT>Attitude state quaternion</COMMENT>
        <COMMENT>Accuracy of this attitude is 0.02 deg RSS.</COMMENT>
        <EPOCH>2003-09-30T14:28:15.1172</EPOCH>
        <quaternionState>
          <COMMENT>Attitude state vector quaternion</COMMENT>
          <REF_FRAME_A>SC_BODY_1</REF_FRAME_A>
          <REF_FRAME_B>ITRF1997</REF_FRAME_B>
          <quaternion>
            <Q1>0.00005</Q1>
            <Q2>0.87543</Q2>
            <Q3>0.40949</Q3>
            <QC>0.25678</QC>
          </quaternion>
        </quaternionState>
      </data>
    </segment>
  </body>
</apm>
```

**Figure G-10: APM Example**

```xml
<?xml version="1.0" encoding="UTF-8"?>
<aem xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-3.0.0-master-3.0.xsd"
   xmlns:ndm="urn:ccsds:schema:ndmxml"
   id="CCSDS_AEM_VERS" version="2.0">
  <header>
    <CREATION_DATE>2008-071T17:09:49</CREATION_DATE>
    <ORIGINATOR>GSFC/FDF</ORIGINATOR>
    <MESSAGE_ID>7077456</MESSAGE_ID>
  </header>
  <body>
    <segment>
      <metadata>
        <OBJECT_NAME>ST5-224</OBJECT_NAME>
        <OBJECT_ID>2006-224A</OBJECT_ID>
        <CENTER_NAME>EARTH</CENTER_NAME>
        <REF_FRAME_A>J2000</REF_FRAME_A>
        <REF_FRAME_B>SC_BODY_1</REF_FRAME_B>
        <TIME_SYSTEM>UTC</TIME_SYSTEM>
        <START_TIME>2006-090T05:00:00.071</START_TIME>
        <USEABLE_START_TIME>2006-090T05:00:00.071</USEABLE_START_TIME>
        <USEABLE_STOP_TIME>2006-090T05:00:00.946</USEABLE_STOP_TIME>
        <STOP_TIME>2006-090T05:00:00.946</STOP_TIME>
        <ATTITUDE_TYPE>SPIN</ATTITUDE_TYPE>
      </metadata>
      <data>
        <COMMENT>Spin KF ground solution, SPINKF rates</COMMENT>
        <attitudeState>
          <spin>
            <EPOCH>2006-090T05:00:00.071</EPOCH>
            <SPIN_ALPHA>2.6862511e+002</SPIN_ALPHA>
            <SPIN_DELTA>6.8448486e+001</SPIN_DELTA>
            <SPIN_ANGLE>1.5969509e+002</SPIN_ANGLE>
            <SPIN_ANGLE_VEL>-1.0996528e+002</SPIN_ANGLE_VEL>
          </spin>
        </attitudeState>
        <attitudeState>
          <spin>
            <EPOCH>2006-090T05:00:00.196</EPOCH>
            <SPIN_ALPHA>2.6863990e+002</SPIN_ALPHA>
            <SPIN_DELTA>6.8432197e+001</SPIN_DELTA>
            <SPIN_ANGLE>1.4593720e+002</SPIN_ANGLE>
            <SPIN_ANGLE_VEL>-1.0996493e+002</SPIN_ANGLE_VEL>
          </spin>
        </attitudeState>
        <attitudeState>
          <spin>
            <EPOCH>2006-090T05:00:00.321</EPOCH>
            <SPIN_ALPHA>2.6864591e+002</SPIN_ALPHA>
            <SPIN_DELTA>6.8412960e+001</SPIN_DELTA>
            <SPIN_ANGLE>1.3218766e+002</SPIN_ANGLE>
            <SPIN_ANGLE_VEL>-1.0996455e+002</SPIN_ANGLE_VEL>
          </spin>
        </attitudeState>
        <attitudeState>
          <spin>
            <EPOCH>2006-090T05:00:00.446</EPOCH>
            <SPIN_ALPHA>2.6863697e+002</SPIN_ALPHA>
            <SPIN_DELTA>6.8392049e+001</SPIN_DELTA>
            <SPIN_ANGLE>1.1845280e+002</SPIN_ANGLE>
            <SPIN_ANGLE_VEL>-1.0996402e+002</SPIN_ANGLE_VEL>
          </spin>
        </attitudeState>
        <attitudeState>
          <spin>
            <EPOCH>2006-090T05:00:00.571</EPOCH>
            <SPIN_ALPHA>2.6861072e+002</SPIN_ALPHA>
            <SPIN_DELTA>6.8371266e+001</SPIN_DELTA>
            <SPIN_ANGLE>1.0473305e+002</SPIN_ANGLE>
            <SPIN_ANGLE_VEL>-1.0996370e+002</SPIN_ANGLE_VEL>
          </spin>
        </attitudeState>
        <attitudeState>
          <spin>
            <EPOCH>2006-090T05:00:00.696</EPOCH>
            <SPIN_ALPHA>2.6856625e+002</SPIN_ALPHA>
            <SPIN_DELTA>6.8353279e+001</SPIN_DELTA>
            <SPIN_ANGLE>9.1030304e+001</SPIN_ANGLE>
            <SPIN_ANGLE_VEL>-1.0996339E+002</SPIN_ANGLE_VEL>
          </spin>
        </attitudeState>
        <attitudeState>
          <spin>
            <EPOCH>2006-090T05:00:00.821</EPOCH>
            <SPIN_ALPHA>2.6850631e+002</SPIN_ALPHA>
            <SPIN_DELTA>6.8340398e+001</SPIN_DELTA>
            <SPIN_ANGLE>7.7341548e+001</SPIN_ANGLE>
            <SPIN_ANGLE_VEL>-1.0996317e+002</SPIN_ANGLE_VEL>
          </spin>
        </attitudeState>
        <attitudeState>
          <spin>
            <EPOCH>2006-090T05:00:00.946</EPOCH>
            <SPIN_ALPHA>2.6843571e+002</SPIN_ALPHA>
            <SPIN_DELTA>6.8332398e+001</SPIN_DELTA>
            <SPIN_ANGLE>6.3662262e+001</SPIN_ANGLE>
            <SPIN_ANGLE_VEL>-1.0996304e+002</SPIN_ANGLE_VEL>
          </spin>
        </attitudeState>
      </data>
    </segment>
  </body>
</aem>
```

**Figure G-11: AEM Example (continued)**

```xml
<?xml version="1.0" encoding="UTF-8"?>
<ndm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-3.0.0-master-3.0.xsd"
   xmlns:ndm="urn:ccsds:schema:ndmxml">
  <COMMENT>Example: 1 each APM, AEM, ACM in combined instantiation</COMMENT>
  <apm id="CCSDS_APM_VERS" version="2.0">
    <header>
      <CREATION_DATE>2007-11-10T15:23:57</CREATION_DATE>
      <ORIGINATOR>CNES</ORIGINATOR>
    </header>
    <body>
      <segment>
        <metadata>
          <COMMENT>APM Implementation Test</COMMENT>
          <OBJECT_NAME>TEST</OBJECT_NAME>
          <OBJECT_ID>2007-011</OBJECT_ID>
          <CENTER_NAME>EARTH</CENTER_NAME>
          <TIME_SYSTEM>UTC</TIME_SYSTEM>
        </metadata>
        <data>
          <EPOCH>2007-10-01T00:02:00.000</EPOCH>
          <eulerAngleState>
            <COMMENT>Attitude specified as Euler elements</COMMENT>
            <REF_FRAME_A>SC_BODY_1</REF_FRAME_A>
            <REF_FRAME_B>J2000</REF_FRAME_B>
            <EULER_ROT_SEQ>ZXZ</EULER_ROT_SEQ>
            <ANGLE_1 units="deg">90.</ANGLE_1>
            <ANGLE_2 units="deg">130.</ANGLE_2>
            <ANGLE_3 units="deg">270.</ANGLE_3>
            <ANGLE_1_DOT units="deg/s">0.</ANGLE_1_DOT>
            <ANGLE_2_DOT units="deg/s">0.</ANGLE_2_DOT>
            <ANGLE_3_DOT units="deg/s">6.</ANGLE_3_DOT>
          </eulerAngleState>
        </data>
      </segment>
    </body>
  </apm>
  <aem id="CCSDS_AEM_VERS" version="2.0">
    <header>
      <COMMENT>Note that data is NOT necessarily realistic; just shows form</COMMENT>
      <CREATION_DATE>2000-100T01:00:00</CREATION_DATE>
      <ORIGINATOR>NASA/JPL</ORIGINATOR>
      <MESSAGE_ID>AEM12345678</MESSAGE_ID>
    </header>
    <body>
      <segment>
        <metadata>
          <COMMENT>Attitude State Type = Quaternion</COMMENT>
          <OBJECT_NAME>TEST</OBJECT_NAME>
          <OBJECT_ID>2000-999Z</OBJECT_ID>
          <REF_FRAME_A>SC_BODY_1</REF_FRAME_A>
          <REF_FRAME_B>J2000</REF_FRAME_B>
          <TIME_SYSTEM>TDB</TIME_SYSTEM>
          <START_TIME>2000-100T00:00:00.000</START_TIME>
          <STOP_TIME>2000-100T00:00:00.000</STOP_TIME>
          <ATTITUDE_TYPE>QUATERNION</ATTITUDE_TYPE>
        </metadata>
        <data>
          <attitudeState>
            <quaternionEphemeris>
              <EPOCH>2000-100T00:00:00.000</EPOCH>
              <quaternion>
                <Q1>-0.005068</Q1>
                <Q2>0.906506</Q2>
                <Q3>0.002360</Q3>
                <QC>0.422157</QC>
              </quaternion>
            </quaternionEphemeris>
          </attitudeState>
        </data>
      </segment>
    </body>
  </aem>
  <acm id="CCSDS_ACM_VERS" version="2.0">
    <header>
      <CREATION_DATE>1998-11-06T09:23:57</CREATION_DATE>
      <ORIGINATOR>JAXA</ORIGINATOR>
      <MESSAGE_ID>A701524</MESSAGE_ID>
    </header>
    <body>
      <segment>
        <metadata>
          <OBJECT_NAME>EUROBIRD-4A</OBJECT_NAME>
          <INTERNATIONAL_DESIGNATOR>2000-052A</INTERNATIONAL_DESIGNATOR>
          <TIME_SYSTEM>UTC</TIME_SYSTEM>
          <EPOCH_TZERO>1998-12-18T14:28:15.1172</EPOCH_TZERO>
        </metadata>
        <data>
          <att>
            <REF_FRAME_A>J2000</REF_FRAME_A>
            <REF_FRAME_B>SC_BODY_1</REF_FRAME_B>
            <NUMBER_STATES>4</NUMBER_STATES>
            <ATT_TYPE>QUATERNION</ATT_TYPE>
            <attLine>0.0 0.73566 -0.50547 0.41390 0.180707</attLine>
            <attLine>0.25 0.73529 -0.50531 0.41375 0.181158</attLine>
            <attLine>0.50 0.73492 -0.50515 0.41441 0.181610</attLine>
          </att>
        </data>
      </segment>
    </body>
  </acm>
</ndm>
```

**Figure G-12: Combined Instantiation with One Each APM, AEM, ACM (continued)**

```xml
<?xml version="1.0" encoding="UTF-8"?>
<aem xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-3.0.0-master-3.0.xsd"
   xmlns:ndm="urn:ccsds:schema:ndmxml"
   id="CCSDS_AEM_VERS" version="2.0">
  <header>
    <COMMENT>Example AEM illustrating all Attitude State Types</COMMENT>
    <COMMENT>Note that data is NOT necessarily realistic; just shows form</COMMENT>
    <CREATION_DATE>2000-100T01:00:00</CREATION_DATE>
    <ORIGINATOR>NASA/JPL</ORIGINATOR>
    <MESSAGE_ID>AEM12345678</MESSAGE_ID>
  </header>
  <body>
    <segment>
      <metadata>
        <COMMENT>Attitude State Type = Quaternion</COMMENT>
        <OBJECT_NAME>TEST</OBJECT_NAME>
        <OBJECT_ID>2000-999Z</OBJECT_ID>
        <REF_FRAME_A>SC_BODY_1</REF_FRAME_A>
        <REF_FRAME_B>J2000</REF_FRAME_B>
        <TIME_SYSTEM>TDB</TIME_SYSTEM>
        <START_TIME>2000-100T00:00:00.000</START_TIME>
        <STOP_TIME>2000-100T00:00:00.000</STOP_TIME>
        <ATTITUDE_TYPE>QUATERNION</ATTITUDE_TYPE>
      </metadata>
      <data>
        <attitudeState>
          <quaternionEphemeris>
            <EPOCH>2000-100T00:00:00.000</EPOCH>
            <quaternion>
              <Q1>-0.005068</Q1>
              <Q2>0.906506</Q2>
              <Q3>0.002360</Q3>
              <QC>0.422157</QC>
            </quaternion>
          </quaternionEphemeris>
        </attitudeState>
      </data>
    </segment>
    <segment>
      <metadata>
        <COMMENT>Attitude State Type = Quaternion/Derivative</COMMENT>
        <OBJECT_NAME>TEST</OBJECT_NAME>
        <OBJECT_ID>2000-999Z</OBJECT_ID>
        <REF_FRAME_A>SC_BODY_1</REF_FRAME_A>
        <REF_FRAME_B>J2000</REF_FRAME_B>
        <TIME_SYSTEM>TDB</TIME_SYSTEM>
        <START_TIME>2000-100T00:00:00.000</START_TIME>
        <STOP_TIME>2000-100T00:00:00.000</STOP_TIME>
        <ATTITUDE_TYPE>QUATERNION/DERIVATIVE</ATTITUDE_TYPE>
      </metadata>
      <data>
        <attitudeState>
          <quaternionDerivative>
            <EPOCH>2000-100T00:00:00.000</EPOCH>
            <quaternion>
              <Q1>-0.005068</Q1>
              <Q2>0.906506</Q2>
              <Q3>0.002360</Q3>
              <QC>0.422157</QC>
            </quaternion>
            <quaternionDot>
              <Q1_DOT>-0.047454</Q1_DOT>
              <Q2_DOT>0.0000</Q2_DOT>
              <Q3_DOT>-0.022128</Q3_DOT>
              <QC_DOT>0.000</QC_DOT>
            </quaternionDot>
          </quaternionDerivative>
        </attitudeState>
      </data>
    </segment>
    <segment>
      <metadata>
        <COMMENT>Attitude State Type = Quaternion/AngVel</COMMENT>
        <OBJECT_NAME>TEST</OBJECT_NAME>
        <OBJECT_ID>2000-999Z</OBJECT_ID>
        <REF_FRAME_A>SC_BODY_1</REF_FRAME_A>
        <REF_FRAME_B>J2000</REF_FRAME_B>
        <TIME_SYSTEM>TDB</TIME_SYSTEM>
        <START_TIME>2000-100T00:00:00.000</START_TIME>
        <STOP_TIME>2000-100T00:00:00.000</STOP_TIME>
        <ATTITUDE_TYPE>QUATERNION/ANGVEL</ATTITUDE_TYPE>
        <ANGVEL_FRAME>REF_FRAME_B</ANGVEL_FRAME>
      </metadata>
      <data>
        <attitudeState>
          <quaternionAngVel>
            <EPOCH>2000-100T00:00:00.000</EPOCH>
            <quaternion>
              <Q1>-0.005068</Q1>
              <Q2>0.906506</Q2>
              <Q3>0.002360</Q3>
              <QC>0.422157</QC>
            </quaternion>
            <angVel>
              <ANGVEL_X>0.000</ANGVEL_X>
              <ANGVEL_Y>-0.047454</ANGVEL_Y>
              <ANGVEL_Z>0.0000</ANGVEL_Z>
            </angVel>
          </quaternionAngVel>
        </attitudeState>
      </data>
    </segment>
    <segment>
      <metadata>
        <COMMENT>Attitude State Type = Euler Angle</COMMENT>
        <OBJECT_NAME>TEST</OBJECT_NAME>
        <OBJECT_ID>2000-999Z</OBJECT_ID>
        <REF_FRAME_A>SC_BODY_1</REF_FRAME_A>
        <REF_FRAME_B>J2000</REF_FRAME_B>
        <TIME_SYSTEM>TDB</TIME_SYSTEM>
        <START_TIME>2000-100T00:00:00.000</START_TIME>
        <STOP_TIME>2000-100T00:00:00.000</STOP_TIME>
        <ATTITUDE_TYPE>EULER_ANGLE</ATTITUDE_TYPE>
        <EULER_ROT_SEQ>XYZ</EULER_ROT_SEQ>
      </metadata>
      <data>
        <attitudeState>
          <eulerAngle>
            <EPOCH>2000-100T00:00:00.000</EPOCH>
            <ANGLE_1>2.6862511e+002</ANGLE_1>
            <ANGLE_2>6.8448486e+001</ANGLE_2>
            <ANGLE_3>1.5969509e+002</ANGLE_3>
          </eulerAngle>
        </attitudeState>
      </data>
    </segment>
    <segment>
      <metadata>
        <COMMENT>Attitude State Type = Euler Angle/Derivative</COMMENT>
        <OBJECT_NAME>TEST</OBJECT_NAME>
        <OBJECT_ID>2000-999Z</OBJECT_ID>
        <REF_FRAME_A>SC_BODY_1</REF_FRAME_A>
        <REF_FRAME_B>J2000</REF_FRAME_B>
        <TIME_SYSTEM>TDB</TIME_SYSTEM>
        <START_TIME>2000-100T00:00:00.000</START_TIME>
        <STOP_TIME>2000-100T00:00:00.000</STOP_TIME>
        <ATTITUDE_TYPE>EULER_ANGLE/DERIVATIVE</ATTITUDE_TYPE>
        <EULER_ROT_SEQ>XYZ</EULER_ROT_SEQ>
      </metadata>
      <data>
        <attitudeState>
          <eulerAngleDerivative>
            <EPOCH>2000-100T00:00:00.000</EPOCH>
            <ANGLE_1>2.6862511e+002</ANGLE_1>
            <ANGLE_2>6.8448486e+001</ANGLE_2>
            <ANGLE_3>1.5969509e+002</ANGLE_3>
            <ANGLE_1_DOT>1.000</ANGLE_1_DOT>
            <ANGLE_2_DOT>1.000</ANGLE_2_DOT>
            <ANGLE_3_DOT>1.000</ANGLE_3_DOT>
          </eulerAngleDerivative>
        </attitudeState>
      </data>
    </segment>
    <segment>
      <metadata>
        <COMMENT>Attitude State Type = Euler Angle/Angvel</COMMENT>
        <OBJECT_NAME>TEST</OBJECT_NAME>
        <OBJECT_ID>2000-999Z</OBJECT_ID>
        <REF_FRAME_A>SC_BODY_1</REF_FRAME_A>
        <REF_FRAME_B>J2000</REF_FRAME_B>
        <TIME_SYSTEM>TDB</TIME_SYSTEM>
        <START_TIME>2000-100T00:00:00.000</START_TIME>
        <STOP_TIME>2000-100T00:00:00.000</STOP_TIME>
        <ATTITUDE_TYPE>EULER_ANGLE/ANGVEL</ATTITUDE_TYPE>
        <EULER_ROT_SEQ>XYZ</EULER_ROT_SEQ>
        <ANGVEL_FRAME>REF_FRAME_B</ANGVEL_FRAME>
      </metadata>
      <data>
        <attitudeState>
          <eulerAngleAngVel>
            <EPOCH>2000-100T00:00:00.000</EPOCH>
            <ANGLE_1>2.6862511e+002</ANGLE_1>
            <ANGLE_2>6.8448486e+001</ANGLE_2>
            <ANGLE_3>1.5969509e+002</ANGLE_3>
            <ANGVEL_X>0.000</ANGVEL_X>
            <ANGVEL_Y>-0.047454</ANGVEL_Y>
            <ANGVEL_Z>0.0000</ANGVEL_Z>
          </eulerAngleAngVel>
        </attitudeState>
      </data>
    </segment>
    <segment>
      <metadata>
        <COMMENT>Attitude State Type = Spin</COMMENT>
        <OBJECT_NAME>TEST</OBJECT_NAME>
        <OBJECT_ID>2000-999Z</OBJECT_ID>
        <REF_FRAME_A>SC_BODY_1</REF_FRAME_A>
        <REF_FRAME_B>J2000</REF_FRAME_B>
        <TIME_SYSTEM>TDB</TIME_SYSTEM>
        <START_TIME>2000-100T00:00:00.000</START_TIME>
        <STOP_TIME>2000-100T00:00:00.000</STOP_TIME>
        <ATTITUDE_TYPE>SPIN</ATTITUDE_TYPE>
      </metadata>
      <data>
        <attitudeState>
          <spin>
            <EPOCH>2000-100T00:00:00.000</EPOCH>
            <SPIN_ALPHA>2.6862511e+002</SPIN_ALPHA>
            <SPIN_DELTA>6.8448486e+001</SPIN_DELTA>
            <SPIN_ANGLE>1.5969509e+002</SPIN_ANGLE>
            <SPIN_ANGLE_VEL>-1.0996528e+002</SPIN_ANGLE_VEL>
          </spin>
        </attitudeState>
      </data>
    </segment>
    <segment>
      <metadata>
        <COMMENT>Attitude State Type = Spin/Nutation</COMMENT>
        <OBJECT_NAME>TEST</OBJECT_NAME>
        <OBJECT_ID>2000-999Z</OBJECT_ID>
        <REF_FRAME_A>SC_BODY_1</REF_FRAME_A>
        <REF_FRAME_B>J2000</REF_FRAME_B>
        <TIME_SYSTEM>TDB</TIME_SYSTEM>
        <START_TIME>2000-100T00:00:00.000</START_TIME>
        <STOP_TIME>2000-100T00:00:00.000</STOP_TIME>
        <ATTITUDE_TYPE>SPIN/NUTATION</ATTITUDE_TYPE>
      </metadata>
      <data>
        <attitudeState>
          <spinNutation>
            <EPOCH>2000-100T00:00:00.000</EPOCH>
            <SPIN_ALPHA>2.6862511e+002</SPIN_ALPHA>
            <SPIN_DELTA>6.8448486e+001</SPIN_DELTA>
            <SPIN_ANGLE>1.5969509e+002</SPIN_ANGLE>
            <SPIN_ANGLE_VEL>-1.0996528e+002</SPIN_ANGLE_VEL>
            <NUTATION units="deg">2.0</NUTATION>
            <NUTATION_PER units="s">30.5</NUTATION_PER>
            <NUTATION_PHASE units="deg">92.7</NUTATION_PHASE>
          </spinNutation>
        </attitudeState>
      </data>
    </segment>
    <segment>
      <metadata>
        <COMMENT>Attitude State Type = Spin/Nutation/Momentum</COMMENT>
        <OBJECT_NAME>TEST</OBJECT_NAME>
        <OBJECT_ID>2000-999Z</OBJECT_ID>
        <REF_FRAME_A>SC_BODY_1</REF_FRAME_A>
        <REF_FRAME_B>J2000</REF_FRAME_B>
        <TIME_SYSTEM>TDB</TIME_SYSTEM>
        <START_TIME>2000-100T00:00:00.000</START_TIME>
        <STOP_TIME>2000-100T00:00:00.000</STOP_TIME>
        <ATTITUDE_TYPE>SPIN/NUTATION_MOM</ATTITUDE_TYPE>
      </metadata>
      <data>
        <attitudeState>
          <spinNutationMom>
            <EPOCH>2000-100T00:00:00.000</EPOCH>
            <SPIN_ALPHA>2.6862511e+002</SPIN_ALPHA>
            <SPIN_DELTA>6.8448486e+001</SPIN_DELTA>
            <SPIN_ANGLE>1.5969509e+002</SPIN_ANGLE>
            <SPIN_ANGLE_VEL>-1.0996528e+002</SPIN_ANGLE_VEL>
            <MOMENTUM_ALPHA units="deg">2.0</MOMENTUM_ALPHA>
            <MOMENTUM_DELTA units="deg">3.5</MOMENTUM_DELTA>
            <NUTATION_VEL units="deg/s">2.7</NUTATION_VEL>
          </spinNutationMom>
        </attitudeState>
      </data>
    </segment>
  </body>
</aem>
```

**Figure G-13: AEM Ephemeris Types Illustrating Special Tags (continued)**


# ANNEX H

## INFORMATIVE REFERENCES (INFORMATIVE)

[H1] Organization and Processes for the Consultative Committee for Space Data Systems. Issue 4. CCSDS Record (Yellow Book), CCSDS A02.1-Y-4. Washington, D.C.: CCSDS, April 2014.

[H2] Navigation Data—Definitions and Conventions. Issue 4. Report Concerning Space Data System Standards (Green Book), CCSDS 500.0-G-4. Washington, D.C.: CCSDS, November 2019.

[H3] F. Landis Markley and John L. Crassidis. Fundamentals of Spacecraft Attitude Determination and Control. Space Technology Library. New York: Springer, 2014.

**NOTE** – Normative references are provided in 1.5.


# ANNEX I

## ITEMS FOR AN INTERFACE CONTROL DOCUMENT (INFORMATIVE)

In several places in this document, there are references to items that should be specified in an ICD between agencies participating in an exchange of attitude data. The ICD should be jointly produced by both agencies participating in a cross-support activity involving the transfer of attitude data. This annex compiles those recommendations into a single list.

**NOTE** – The greater the amount of material specified via ICD, the lesser the utility/benefit of the ADM (custom programming will be required to tailor software for each ICD).

**Table I-1: Items Recommended for an ICD**

| ICD Item | Section Trace |
| :--- | :--- |
| 1. Definition of attitude accuracy requirements pertaining to data in an ADM as well as attitude dynamics modeling. | 1.2.2 |
| 2. Description of User-Defined Parameters. | 5.3.10.1 |
| 3. If the chosen angle units are radians (which is outside the standard). | 6.8.1 |
| 4. If floating-point numbers in extended-single or extended-double precision are to be used, then discussion of implementation-specific attributes is required. | 6.8.5 |
| 5. Information which must appear in comments for any given ADM exchange. | 6.10.1.3 |
| 6. Keyword value settings that are different from those given in annex B. | annex B |
| 7. Data security implementation specifics. | annex C, C1.11 |


# ANNEX J

## CHANGES VERSUS PREVIOUS VERSION (INFORMATIVE)

The present section gives the main changes between ADM 1.0 and ADM 2.0.

**Changes relative to all messages:**

*   Keywords in version 1.0 could be ‘Obligatory’ or ‘Optional’. These words have been replaced by ‘Mandatory’ and ‘Optional’ because the Implementation Conformance Specification uses that wording, which is set by the CCSDS.
*   The XML formatting is included in this volume instead of a completely separate volume.

**Changes relative to APM:**

| Number | Description | Rationale for change | Subsection |
| :--- | :--- | :--- | :--- |
| 1 | The quaternion block is now optional. | To enable more flexibility if other data needs to be exchanged. | 3.2.4 |
| 2 | Any block can now be present as many times as necessary. | To increase flexibility. | 3.2.4, 3.2.4.4 |
| 3 | The meanings of quaternion, Euler angles, spin data, etc., are now clearly defined by the standard. | To avoid misuse of exchange data. | 3.2.4, annex F |
| 4 | The order for quaternion components (real part first or last) is now imposed by the standard. | For the sake of simplicity of the standard. | 3.2.4 |
| 5 | Euler rotation sequences ('`EULER_ROT_SEQ`' keyword) are specified by letter (X, Y, Z) instead of number, for example, XYX instead of 121. | This is an improvement, as version 1 led to repeated keyword as X_ANGLE, Y_ANGLE, X_ANGLE. | 3.2.4 |
| 6 | The logical block ‘Euler angles’ now contains angle derivatives rather than components of the angular velocity vector. | To improve the design of the standard. | 3.2.4 |
| 7 | A new block for the angular velocity vector has been added: ANGVEL. | To improve the design of the standard. | 3.2.4 |
| 8 | The keywords for the moments of inertia have changed: `IXY` instead of `I12`, etc. | To make consistent with other changes. | 3.2.4 |
| 9 | Data block delimiters have been added. Data types such as quaternion and Euler angles are explicitly enclosed between `QUAT_START` ... `QUAT_STOP`, `EULER_START` ... `EULER_STOP`, etc., delimiters. | To make the data easier to process and the standard easier to extend in the future. | 3.2.4 |
| 10 | A new keyword: '`MESSAGE_ID`' has been added. | To make consistent with other standards. | 3.2.2 |
| 11 | Frame related keywords have changed in APM version 2: keywords in version 1 such as `Q_FRAME_*`, `SPIN_FRAME_*`, etc. (where * denotes 'A' or 'B') have been removed. The keywords in version 2 are `REF_FRAME_*`. | Increased simplicity. | 3.2.4 |
| 12 | The keywords defining attitude direction (`Q_DIR`, `EULER_DIR`, `SPIN_DIR`) have been removed. The direction is always from A to B. | Simplicity of the standard. | 3.2.4 |
| 13 | New keywords have been added in spin block (`MOMENTUM_ALPHA`, `MOMENTUM_DELTA`, `NUTATION_VEL`). | Simplicity of the standard. | 3.2.4 |

**Changes relative to AEM:**

| Number | Description | Rationale for change | Subsection |
| :--- | :--- | :--- | :--- |
| 1 | Euler rotation sequences ('`EULER_ROT_SEQ`' keyword) are specified by letter (X, Y, Z) instead of number, for example, XYX instead of 121. | Consistency with APM. | 4.2.3 |
| 2 | A new value has been introduced for the '`ATTITUDE_TYPE`' keyword: `EULER_ANGLE/DERIVATIVE` | Consistency with APM. | 4.2.3, 4.2.4 |
| 3 | Values for the ‘`ATTITUDE_TYPE`’ keyword have changed: `QUATERNION/RATE` and `EULER_ANGLE/RATE` have been removed. `QUATERNION/ANGVEL` and `EULER_ANGLE/ANGVEL` have been added. | More consistency between APM and AEM (naming conventions). | 4.2.3, 4.2.4 |
| 4 | The order for quaternion components (real part first or last) is now imposed by the standard. | Simplicity of the standard. | 4.2.4 |
| 5 | The `ATTITUDE_DIR` keyword has been removed. | Consistency with APM. | 4.2.3 |
| 6 | A new keyword: '`MESSAGE_ID`' has been added. | Consistency with APM/other standards. | 4.2.2 |
| 7 | The keyword ‘`QUATERNION_TYPE`’ has been removed. The order in the AEM is the same as in the APM: Q1, Q2, Q3, QC by convention. This change also makes the KVN and XML versions more consistent. | More consistency between APM and AEM, and simplicity of the standard. | 4.2.3, 4.2.4 |
| 8 | New way to describe spin data (`SPIN/NUTATION_MOM`). | Simplicity of the standard and consistency with APM. | 4.2.4 |

**Changes relative to ACM:**

The ACM was added to provide symmetry with the Orbit Comprehensive Message (OCM) being added to the Orbit Data Messages standard. (See section 5.)


