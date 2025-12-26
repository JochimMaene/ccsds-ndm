# AUTHORITY

**Issue:** Recommended Standard, Issue 1
**Date:** November 2019
**Location:** Washington, DC, USA

This document has been approved for publication by the Management Council of the Consultative Committee for Space Data Systems (CCSDS) and represents the consensus technical agreement of the participating CCSDS Member Agencies. The procedure for review and authorization of CCSDS documents is detailed in *Organization and Processes for the Consultative Committee for Space Data Systems* (CCSDS A02.1-Y-4), and the record of Agency participation in the authorization of this document can be obtained from the CCSDS Secretariat at the e-mail address below.

This document is published and maintained by:

CCSDS Secretariat
National Aeronautics and Space Administration
Washington, DC, USA
Email: secretariat@mailman.ccsds.org
# STATEMENT OF INTENT

The Consultative Committee for Space Data Systems (CCSDS) is an organization officially established by the management of its members. The Committee meets periodically to address data systems problems that are common to all participants, and to formulate sound technical solutions to these problems. Inasmuch as participation in the CCSDS is completely voluntary, the results of Committee actions are termed Recommended Standards and are not considered binding on any Agency.

This Recommended Standard is issued by, and represents the consensus of, the CCSDS members. Endorsement of this Recommendation is entirely voluntary. Endorsement, however, indicates the following understandings:

- Whenever a member establishes a CCSDS-related standard, this standard will be in accord with the relevant Recommended Standard. Establishing such a standard does not preclude other provisions which a member may develop.
- Whenever a member establishes a CCSDS-related standard, that member will provide other CCSDS members with the following information:
  - The standard itself.
  - The anticipated date of initial operational capability.
  - The anticipated duration of operational service.
- Specific service arrangements shall be made via memoranda of agreement. Neither this Recommended Standard nor any ensuing standard is a substitute for a memorandum of agreement.

No later than five years from its date of issuance, this Recommended Standard will be reviewed by the CCSDS to determine whether it should: (1) remain in effect without change; (2) be changed to reflect the impact of new technologies, new requirements, or new directions; or (3) be retired or canceled.

In those instances when a new version of a Recommended Standard is issued, existing CCSDS-related member standards and implementations are not negated or deemed to be non-CCSDS compatible. It is the responsibility of each member to determine when such standards or implementations are to be modified. Each member is, however, strongly encouraged to direct planning for its new standards and implementations towards the later version of the Recommended Standard.
# FOREWORD

This document is a CCSDS Recommended Standard for a Re-entry Data Message (RDM) and has been prepared by the CCSDS. The RDM described in this document is intended for re-entry (prediction) information exchange between interested parties.

Attention is drawn to the possibility that some of the elements of this document may be the subject of patent rights. CCSDS has processes for identifying patent issues and for securing from the patent holder agreement that all licensing policies are reasonable and non-discriminatory. However, CCSDS does not have a patent law staff, and CCSDS shall not be held responsible for identifying any or all such patent rights.

Through the process of normal evolution, it is expected that expansion, deletion, or modification of this document may occur. This Recommended Standard is therefore subject to CCSDS document management and change control procedures, which are defined in *Organization and Processes for the Consultative Committee for Space Data Systems* (CCSDS A02.1-Y-4). Current versions of CCSDS documents are maintained at the CCSDS Web site:

[http://www.ccsds.org/](http://www.ccsds.org/)

Questions relating to the contents or status of this document should be sent to the CCSDS Secretariat at the email address indicated on page i.
# Member and Observer Agencies

At time of publication, the active Member and Observer Agencies of the CCSDS were:

## Member Agencies

- Agenzia Spaziale Italiana (ASI)/Italy.
- Canadian Space Agency (CSA)/Canada.
- Centre National d’Etudes Spatiales (CNES)/France.
- China National Space Administration (CNSA)/People’s Republic of China.
- Deutsches Zentrum für Luft- und Raumfahrt (DLR)/Germany.
- European Space Agency (ESA)/Europe.
- Federal Space Agency (FSA)/Russian Federation.
- Instituto Nacional de Pesquisas Espaciais (INPE)/Brazil.
- Japan Aerospace Exploration Agency (JAXA)/Japan.
- National Aeronautics and Space Administration (NASA)/USA.
- UK Space Agency/United Kingdom.

## Observer Agencies

- Austrian Space Agency (ASA)/Austria.
- Belgian Federal Science Policy Office (BFSPO)/Belgium.
- Central Research Institute of Machine Building (TsNIIMash)/Russian Federation.
- China Satellite Launch and Tracking Control General, Beijing Institute of Tracking and Telecommunications Technology (CLTC/BITTT)/China.
- Chinese Academy of Sciences (CAS)/China.
- China Academy of Space Technology (CAST)/China.
- Commonwealth Scientific and Industrial Research Organization (CSIRO)/Australia.
- Danish National Space Center (DNSC)/Denmark.
- Departamento de Ciência e Tecnologia Aeroespacial (DCTA)/Brazil.
- Electronics and Telecommunications Research Institute (ETRI)/Korea.
- European Organization for the Exploitation of Meteorological Satellites (EUMETSAT)/Europe.
- European Telecommunications Satellite Organization (EUTELSAT)/Europe.
- Geo-Informatics and Space Technology Development Agency (GISTDA)/Thailand.
- Hellenic National Space Committee (HNSC)/Greece.
- Hellenic Space Agency (HSA)/Greece.
- Indian Space Research Organization (ISRO)/India.
- Institute of Space Research (IKI)/Russian Federation.
- Korea Aerospace Research Institute (KARI)/Korea.
- Ministry of Communications (MOC)/Israel.
- Mohammed Bin Rashid Space Centre (MBRSC)/United Arab Emirates.
- National Institute of Information and Communications Technology (NICT)/Japan.
- National Oceanic and Atmospheric Administration (NOAA)/USA.
- National Space Agency of the Republic of Kazakhstan (NSARK)/Kazakhstan.
- National Space Organization (NSPO)/Chinese Taipei.
- Naval Center for Space Technology (NCST)/USA.
- Research Institute for Particle & Nuclear Physics (KFKI)/Hungary.
- Scientific and Technological Research Council of Turkey (TUBITAK)/Turkey.
- South African National Space Agency (SANSA)/Republic of South Africa.
- Space and Upper Atmosphere Research Commission (SUPARCO)/Pakistan.
- Swedish Space Corporation (SSC)/Sweden.
- Swiss Space Office (SSO)/Switzerland.
- United States Geological Survey (USGS)/USA.
# DOCUMENT CONTROL

| Document | Title | Date | Status |
| :--- | :--- | :--- | :--- |
| CCSDS 508.1-B-1 | Re-entry Data Message, Recommended Standard, Issue 1 | November 2019 | Original issue |
| CCSDS 508.1-B-1 Cor. 1 | Technical Corrigendum 1 | October 2021 | Adds specifications and examples for messages with elements qualified with respect to a namespace. |
# 1 INTRODUCTION

## 1.1 PURPOSE AND SCOPE

The Re-entry Data Message (RDM) Recommended Standard (henceforth the RDM' or ‘this standard') specifies a standard message format to be used in the exchange of spacecraft re-entry information between Space Situational Awareness (SSA) or Space Surveillance and Tracking (SST)¹ data providers, satellite owners/operators, and other parties. This message can be used to inform spacecraft owners/operators of predicted re-entries or warn civil protection agencies about potential ground impacts.

The RDM will:

a) facilitate interoperability and enable consistent warning between data providers who supply re-entry prediction data and the entities that use it;
b) facilitate the automation of re-entry prediction processes; and
c) provide critical information to enable timely re-entry decisions (e.g., a change in the controlled re-entry strategy).

The document also includes informative descriptions of re-entry prediction methods and data.

## 1.2 APPLICABILITY

The RDM is applicable to Space Surveillance and Tracking activities, spacecraft operations, and other ‘ground' based activities (e.g., civil protection, civil and military aviation) where re-entering space objects are concerns. This Recommended Standard contains the specifications for an RDM designed for re-entry prediction information exchange (though it can be used post-facto as well) between originators of re-entry data and recipients. Re-entry data includes remaining orbital lifetime, start and end of the re-entry and impact windows, impact location, and object physical properties. Further information about the types of data exchanged can be found in section 3.

The RDM is suitable for both manual and automated interaction, and for machine-to-machine interfaces because of the large amount of data present. The RDM is self-contained, but it can be paired with other Navigation Data Messages to enhance its functionality. For example, an RDM could be paired with several Orbit Parameter Messages (OPM─reference [1], section 3) to exchange state vectors at critical epochs (last orbit determination, current, re-entry, etc.) or with one Orbit Ephemeris Message (OEM─reference [1], section 5) to give the trajectory for (most of) the re-entry. The presence of user defined keywords allows other information to be exchanged after being specified in an Interface Control Document (ICD); ICDs are not necessary for most RDM exchanges, but are expected in some cases (especially if ODMs are to be exchanged for object position). The CCSDS Navigation Working Group should be informed of new optional keywords for possible inclusion in future revisions of the standard.

¹ The term SSA is more commonly used in the US, while SST is preferred in Europe. This document uses SST hereafter.

It is desirable that RDM originators maintain consistency with respect to the optional keywords provided in their implementations; i.e., it is desirable that the composition of the RDMs provided not change on a frequent basis.

This standard is applicable only to the message format and content, not to its transmission or to the algorithms used to produce the data within. The method of transmitting the message between exchange partners is beyond the scope of this document and could be specified in an ICD. The methods used to predict re-entries and calculate the associated data (e.g., probability of ground impact and impact location) are also outside the scope of this standard.

## 1.3 DOCUMENT STRUCTURE

Section 2 provides a short overview of the CCSDS RDM.

Section 3 describes the structure and content of the 'keyword = value' (KVN) version of the RDM.

Section 4 describes the structure and content of the XML version of the RDM.

Section 5 describes the data and syntax of RDM messages, both the KVN and XML versions.

Annex A shows the Implementation Conformance Statement (ICS), listing all the requirements an RDM implementation should meet.

Annex B discusses security, SANA, and patent issues.

Annex C provides RDM examples.

Annex D lists the abbreviations and acronyms used in this document.

Annex E lists the requirements this standard was designed to meet.

Annex F presents a summary sheet that shows which optional metadata entries should be present for the data blocks in the message.

Annex G describes the types of information exchanged in an RDM.

Annex H provides a list of informative references.

## 1.4 NOMENCLATURE

### 1.4.1 NORMATIVE TEXT

The following conventions apply for the normative specifications in this Recommended Standard:

a) the words 'shall' and 'must' imply a binding and verifiable specification;
b) the word 'should' implies an optional, but desirable, specification;
c) the word 'may' implies an optional specification;
d) the words 'is', 'are', and 'will' imply statements of fact.

NOTE – These conventions do not imply constraints on diction in text that is clearly informative in nature.

### 1.4.2 INFORMATIVE TEXT

In the normative sections of this document, informative text is set off from the normative specifications either in notes or under one of the following subsection headings:

– Overview;
– Background;
– Rationale;
– Discussion.

## 1.5 CONVENTIONS AND DEFINITIONS

The following conventions for unit notations apply throughout this standard. To the extent possible, an effort has been made to use units that are part of the International System of Units (SI—defined in reference [2]); units are either SI base units, SI derived units, or units outside the SI that are accepted for use with the SI. The units used within this document are as follows:

– km: kilometers;
– m: meters;
– d: days (86400 s);
– s: seconds of time;
– kg: kilograms;
– deg: degrees of arc;
– %: percent.

For compound units the following conventions are used:

– multiplication is denoted with a single asterisk, '*' (e.g., 'kg*s');
– division is denoted with a single forward slash, '/' (e.g., 'km/s');
– exponents are denoted with a double asterisk, '**' (e.g., 'km**2').

The usual mathematical conventions for operation order (e.g., exponents before multiplication) apply.

The following conventions apply when referring to Earth re-entry prediction simulations:

a) short-term re-entry prediction refers to simulations covering the last few days (up to a few weeks) before re-entry (for a circular orbit altitude below 200 km);
b) medium-term re-entry prediction refers to simulations covering the last few weeks up to 11 years (one solar cycle) before re-entry; for the purposes of this standard it is grouped with long-term prediction;
c) long-term re-entry prediction refers to all re-entry simulations covering more than one solar cycle (over 11 years); grouped with medium-term prediction for this standard.

The term 'controlled re-entry' is used in this document for a re-entry event where:

a) the re-entry epoch is controlled; and
b) the impact of fragments on the central body's surface is confined to a predetermined area.

The term ‘ASCII' is generally used in this document to refer to the text character set defined in reference [3]. The term ‘n/a' is used to mean ‘not applicable' or ‘not available'.

## 1.6 REFERENCES

The following publications contain provisions which, through reference in this text, constitute provisions of this document. At the time of publication, the editions indicated were valid. All publications are subject to revision, and users of this standard are encouraged to investigate the possibility of applying the most recent editions of the publications indicated below. The CCSDS Secretariat maintains a register of currently valid CCSDS publications.

[1] Orbit Data Messages. Issue 2. Recommendation for Space Data System Standards (Blue Book), CCSDS 502.0-B-2. Washington, D.C.: CCSDS, November 2009.
[2] The International System of Units (SI). 8th ed., 2006; updated in 2014. Sèvres, France: BIPM, 2006.
[3] Information Technology—8-Bit Single-Byte Coded Graphic Character Sets—Part 1: Latin Alphabet No. 1. International Standard, ISO/IEC 8859-1:1998. Geneva: ISO, 1998.
[4] Henry S. Thompson, et al., eds. XML Schema Part 1: Structures. 2nd ed. W3C Recommendation. N.p.: W3C, October 2004.
[5] Paul V. Biron and Ashok Malhotra, eds. XML Schema Part 2: Datatypes. 2nd ed. W3C Recommendation. N.p.: W3C, October 2004.
[6] “Organizations.” Space Assigned Numbers Authority (SANA). https://sanaregistry.org/r/organizations/.
[7] “United Nations Register of Objects Launched into Outer Space.” UNOOSA. http://www.unoosa.org/oosa/en/spaceobjectregister/.
[8] “Conjunction Data Message CATALOG_NAME.” Space Assigned Numbers Authority (SANA). https://sanaregistry.org/r/cdm_catalog.
[9] “Orbit Centers.” Space Assigned Numbers Authority. https://sanaregistry.org/r/orbit_centers.
[10] “Time Systems.” Space Assigned Numbers Authority. https://sanaregistry.org/r/time_systems.
[11] “Celestial Body Reference Frames.” Space Assigned Numbers Authority. https://sanaregistry.org/r/celestial_body_reference_frames.
[12] “Orbit-Relative Reference Frames." Space Assigned Numbers Authority. https://sanaregistry.org/r/orbit_relative_reference_frames.
[13] XML Specification for Navigation Data Messages. Issue 2. Recommendation for Space Data System Standards (Blue Book), CCSDS 505.0-B-2. Washington, D.C.: CCSDS, May 2021.
[14] Time Code Formats. Issue 4. Recommendation for Space Data System Standards (Blue Book), CCSDS 301.0-B-4. Washington, D.C.: CCSDS, November 2010.
# 2 OVERVIEW

## 2.1 OVERVIEW

This section provides a high-level overview of the Re-entry Data Message, a message format designed to help the exchange of re-entry data between originators of re-entry data, spacecraft operators, and other interested entities. The originators of re-entry data can be SST entities or even the spacecraft operators themselves (e.g., for a controlled re-entry).

## 2.2 RDM CONTENT

The RDM is an ASCII format, encoded either as plain text (usually referred to as KVN) or XML (references [3], [4], and [5]). This standard describes both.

The RDM contains information about a single re-entry event:

- information about the message itself (creation date, originator, etc.);
- identification of the re-entering object (name, ID);
- basic re-entry information (mandatory): remaining orbital lifetime, whether the re-entry is controlled or not, and which celestial body the object is orbiting;
- more complex re-entry information (optional): re-entry and impact windows, impact location and probabilities, state vector, object properties, the orbit determination (OD) process, and observations used to predict the re-entry.

The information is used by satellite operators, civil protection, or aviation authorities to assess the re-entry risk and plan any needed mitigation measures.

The RDM is not limited to man-made objects re-entering the Earth’s atmosphere. It could be used for any entry/impact event by specifying the appropriate CENTER_NAME, REF_FRAME, and OBJECT_TYPE values (e.g., a space probe landing on Venus).
# 3 RE-ENTRY DATA MESSAGE STRUCTURE AND CONTENT (KVN)

## 3.1 OVERVIEW

This section contains a description of the structure, content, and keywords allowed in a Re-entry Data Message.

## 3.2 GENERAL

3.2.1 The RDM shall be plain text consisting of re-entry (prediction) data for a single re-entry event.

3.2.2 The RDM in Keyword Value Notation (KVN) shall consist of digital data represented as ASCII text lines. The RDM shall contain the following:

a) a header;
b) a metadata section;
c) a data section.

NOTES
1. KVN messages contain one keyword per line (specified in 5.3.2.4).
2. The standard order of keywords in the KVN representation is fixed by this standard, as listed in tables 3-1, 3-2, and 3-3 (specified in 5.3.2.10).

3.2.3 The RDM file naming scheme should be agreed to on a case-by-case basis between the participating agencies, typically specified in an ICD. In general, the file name syntax and length must not violate computer constraints for those computing environments in use by Member Agencies for processing re-entry data.

3.2.4 The exchange method for RDMs should be determined on a case-by-case basis and should be documented in an ICD.

## 3.3 RDM HEADER

The RDM header shall only consist of the KVN elements defined in table 3-1, which specifies for each header item:

a) the keyword to be used;
b) a short description of the item;
c) examples of allowed values;
d) whether the item is mandatory (M) or optional (O).

### Table 3-1: RDM KVN Header
| Keyword | Description of values | Examples of values | M/O |
| :--- | :--- | :--- | :--- |
| `CCSDS_RDM_VERS` | Format version in the form of 'x.y', where 'y' is incremented for corrections and minor changes, and 'x' is incremented for major changes. | 1.0 | M |
| `COMMENT` | Comments (allowed in the RDM Header only immediately after the RDM version number). | `COMMENT This is a comment` | O |
| `CREATION_DATE` | File creation date and time in UTC. The format is defined in 5.3.3.5. | `2001-11-06T11:17:33`<br>`2002-204T15:56:23` | M |
| `ORIGINATOR` | Creating agency or entity; the value should be taken from the abbreviation column in the SANA organizations registry (reference [6]). The country of origin should also be provided where the originator is not a national space agency. | `DLR`<br>`ESA` | M |
| `MESSAGE_ID` | ID that uniquely identifies a message from a given originator. The format and content of the message identifier value are at the discretion of the originator. | `201113719185`<br>`ESA20190101-3345` | M |

## 3.4 RDM METADATA

3.4.1 The RDM metadata shall only consist of the KVN elements defined in table 3-2, which specifies for each metadata item:

a) the keyword;
b) a short description of the item;
c) normative values or examples of values (some examples wrap over multiple lines due to limited space, e.g., time strings; they would not wrap in an actual RDM);
d) whether the values are normative (N—all values allowed are in the list in the third column) or examples of values allowed (E);
e) whether the item is mandatory (M) or optional (O).

3.4.2 Mandatory items shall appear in every RDM metadata section. Items that are not mandatory may or may not appear, at the discretion of the data producer, based on the requirements of the data and its intended application (an RDM Summary Sheet that illustrates the relationships between data types and metadata can be found in annex F).

### Table 3-2: RDM KVN Metadata
| Keyword | Description | Examples of values | N/E | M/O |
| :--- | :--- | :--- | :--- | :--- |
| `COMMENT` | Comments (allowed only at the beginning of RDM metadata). | `COMMENT This is a comment` | E | O |
| `OBJECT_NAME` | Object name for which the orbit state is provided. There is no CCSDS-based restriction on the value for this keyword, but it is recommended to use names from the UNOOSA registry—reference [7], which includes object name and international designator of the participant (formatting rules specified in 5.2.3.3). For objects that are not in the UNOOSA registry, either a descriptive name (e.g., DEBRIS, if the object is identified as space debris) or UNKNOWN should be used. | `SENTINEL-1A`<br>`GOCE`<br>`ENVISAT`<br>`BRIZ R/B`<br>`DEBRIS`<br>`UNKNOWN` | E | M |
| `INTERNATIONAL_DESIGNATOR` | The full international designator (COSPAR ID) for the object. Values shall have the format YYYY-NNNP{PP}, where: YYYY = year of launch; NNN = three-digit serial number of launch (with leading zeros); P{PP} = at least one capital letter for the identification of the part brought into space by the launch. In cases where the object has no international designator, the value UNKNOWN should be used (formatting rules specified in 5.2.3.3). | `2010-012C`<br>`2016-001A`<br>`1985-067CD`<br>`UNKNOWN` | E | M |
| `CATALOG_NAME` | The satellite catalog used for the object (formatting rules specified in 5.2.3.3). The name should be taken from the appropriate SANA registry for catalog names, reference [8]. | `SATCAT`<br>`ESA SST` | E | O |
| `OBJECT_DESIGNATOR` | The CATALOG_NAME satellite catalog designator for the object (formatting rules specified in 5.2.3.3). | `37451`<br>`125387U` | E | O |
| `OBJECT_TYPE` | The object type. | `PAYLOAD`<br>`ROCKET BODY`<br>`DEBRIS`<br>`OTHER`<br>`UNKNOWN` | N | O |
| `OBJECT_OWNER` | Owner of the object (e.g., company, agency, or country owning the satellite). The value should be taken from the abbreviation column in the SANA organizations registry, reference [6]. | `DLR`<br>`INTELSAT`<br>`ESA`<br>`UNKNOWN` | E | O |
| `OBJECT_OPERATOR` | Operator of the object (e.g., company, agency, or country operating the satellite). The value should be taken from the abbreviation column in the SANA organizations registry, reference [6]. | `ESA`<br>`EUMETSAT` | E | O |
| `CONTROLLED_REENTRY` | Specification of whether the re-entry is controlled or not. | `YES`<br>`NO`<br>`UNKNOWN` | N | M |
| `CENTER_NAME` | Celestial body orbited by the object and origin of the reference frame, which may be a natural solar system body (planets, asteroids, comets, and natural satellites), including any planet barycenter or the solar system barycenter. The value should be taken from the orbit center column in the SANA orbit centers registry, reference [9]. | `EARTH`<br>`MOON`<br>`JUPITER` | E | M |
| `TIME_SYSTEM` | Time system for all data/metadata. The value should be taken from the name column in the SANA time systems registry, reference [10]. | `UTC`<br>`TAI` | E | M |
| `EPOCH_TZERO` | Epoch from which the ORBIT_LIFETIME is calculated (formatting rules specified in 5.3.3.5). | `2001-11-06T11:17:33`<br>`2002-204T15:56:23` | E | M |
| `REF_FRAME` | Reference frame in which the (optional) orbit information will be provided. The value should be taken from the keyword value name column in the SANA celestial body reference frames registry, reference [11]. The reference frame must be the same for all orbit data elements, with the exception of the covariance matrix, for which a different reference frame may be specified, and the ground impact data. This keyword becomes mandatory if state vectors are provided in the data section. | `ITRF-97`<br>`EME2000`<br>`ICRF` | E | O |
| `REF_FRAME_EPOCH` | Epoch of reference frame, if not intrinsic to the definition of the reference frame (formatting rules specified in 5.3.3.5). | `2001-11-06T11:17:33`<br>`2002-204T15:56:23` | E | O |
| `EPHEMERIS_NAME` | Unique identifier of an external ephemeris file used or NONE. | `NONE`<br>`EPHEMERIS`<br>`INTELSAT2` | E | O |
| `GRAVITY_MODEL` | The gravity model used in the simulation. The degree (D) and order (O) of the spherical harmonic coefficients applied should be given along with the name of the model. | `EGM-96: 36D 360`<br>`JGM-2: 41D 410` | E | O |
| `ATMOSPHERIC_MODEL` | The atmosphere model(s) used in the simulation. If more than one model is used they should be listed on the same line and separated by a comma. | `MSIS, JACCHIA 70`<br>`MSISE-90`<br>`NRLMSISE-00` | E | O |
| `SOLAR_FLUX_PREDICTION` | The method used to predict the solar flux and geomagnetic indices. | `STOCHASTIC`<br>`PREDICTED: MLLRT` | E | O |
| `N_BODY_PERTURBATIONS` | Comma separated list of other bodies used in the simulation. The names of the bodies should be taken from the SANA registry for orbit centers, reference [9]. If no other bodies are used in the simulation, the value should be NONE. | `MOON, SUN`<br>`JUPITER`<br>`NONE` | E | O |
| `SOLAR_RAD_PRESSURE` | Model used for the solar radiation pressure: either model name, or NO if solar radiation pressure was not modelled. | `GSPM04`<br>`NO` | E | O |
| `EARTH_TIDES` | Model used for solid Earth and ocean tides: either model name, or NO if tides were not modelled. | `ESR`<br>`NO` | E | O |
| `INTRACK_THRUST` | Indicator on whether in-track thrust modeling was used in the simulation. | `YES`<br>`NO` | N | O |
| `DRAG_PARAMETERS_SOURCE` | The method used to estimate the drag parameters of the object (DRAG_AREA, DRAG_COEFF, and/or BALLISTIC_COEFF). | `DESIGN`<br>`CFD: TOOL1`<br>`CFD DMSCFOAM`<br>`OD` | E | O |
| `DRAG_PARAMETERS_ALTITUDE` | The altitude (in km) at which the object drag parameters (DRAG_AREA, DRAG_COEFF, and/or BALLISTIC_COEFF) are valid. The units shall be kilometers, and the conventions specified in 5.2.4.1 and 5.3.4 must be followed. | `200 [km]`<br>`175 [km]` | E | O |
| `REENTRY_UNCERTAINTY_METHOD` | The method used to determine the orbit lifetime uncertainty or the re-entry windows. | `NONE`<br>`ANALYTICAL`<br>`STOCHASTIC`<br>`EMPIRICAL` | N | O |
| `REENTRY_DISINTEGRATION` | The aspects of disintegration during re-entry considered during simulations: none (the object was treated as a point mass), mass loss, break-ups (including explosion), or both. It is a coarse indication on whether the impact area in the data covers potential fragments as well. | `NONE`<br>`MASS-LOSS`<br>`BREAK-UP`<br>`MASS-LOSS + BREAK-UP` | N | O |
| `IMPACT_UNCERTAINTY_METHOD` | The method used to determine the impact location confidence interval(s). | `NONE`<br>`ANALYTICAL`<br>`STOCHASTIC`<br>`EMPIRICAL` | N | O |
| `PREVIOUS_MESSAGE_ID` | ID of the previous RDM issued for this object. | `ESA/2015-563892348` | E | O |
| `PREVIOUS_MESSAGE_EPOCH` | UTC Epoch of the previous RDM issued for this object (formatting rules specified in 5.3.3.5). | `2001-11-06T11:17:33` | E | O |
| `NEXT_MESSAGE_EPOCH` | Scheduled UTC epoch of the next RDM for the same object (formatting rules specified in 5.3.3.5); N/A if no other message is scheduled. | `2001-11-06T11:17:33`<br>`N/A` | E | O |

## 3.5 RDM DATA

3.5.1 The RDM data section shall be formatted as logical blocks:
a) atmospheric re-entry;
b) ground impact data;
c) state vector (including epoch);
d) position/velocity covariance matrix (associated with the state vector and at the same epoch);
e) object physical parameters;
f) orbit determination parameters;
g) user defined parameters.

3.5.2 The logical blocks of the RDM data section shall consist of KVN lines, as defined in table 3-3, which specifies:
a) the keyword to be used;
b) a description of the content;
c) the units to be used;
d) whether the item is mandatory (M) or optional (O).

### Table 3-3: RDM KVN Data

| Keyword | Description of values | Units | M/O |
| :--- | :--- | :--- | :--- |
| **Atmospheric re-entry data** |
| `COMMENT` | Comments (allowed only at the beginning of each RDM data logical block). | n/a | O |
| `ORBIT_LIFETIME` | Time until re-entry: from the EPOCH_TZERO epoch in the metadata (days—double precision values allowed; integer values assumed to have .0 fractional part) to permanently crossing the altitude specified in REENTRY_ALTITUDE. If the NOMINAL_REENTRY_EPOCH keyword is present, the ORBIT_LIFETIME and NOMINAL_REENTRY_EPOCH should resolve to the same value. | d | M |
| `REENTRY_ALTITUDE` | Defined re-entry altitude over a spherical central body—once an object’s altitude permanently drops below this value, it is considered to be captured by the central body’s atmosphere. | km | M |
| `ORBIT_LIFETIME_WINDOW_START` | Start of the predicted orbital lifetime window from the EPOCH_TZERO epoch in the metadata (days—double precision values allowed; integer values assumed to have .0 fractional part). To be used for long-term predictions; REENTRY_WINDOW_START and _END should be used for accurate results. | d | O |
| `ORBIT_LIFETIME_WINDOW_END` | End of the predicted orbital lifetime window from the EPOCH_TZERO epoch in the metadata (days—double precision values allowed; integer values assumed to have .0 fractional part). To be used for long-term predictions; REENTRY_WINDOW_START and _END should be used for accurate results. | d | O |
| `NOMINAL_REENTRY_EPOCH` | Predicted epoch at which the object’s altitude permanently drops below NOMINAL_REENTRY_ALTITUDE (formatting rules specified in 5.3.3.5). | n/a | O |
| `REENTRY_WINDOW_START` | Start epoch of the predicted atmospheric re-entry window (formatting rules specified in 5.3.3.5). | n/a | O |
| `REENTRY_WINDOW_END` | End epoch of the predicted atmospheric re-entry window (formatting rules specified in 5.3.3.5). | n/a | O |
| `ORBIT_LIFETIME_CONFIDENCE_LEVEL` | Confidence level of the orbit lifetime or re-entry epoch being inside the window defined by ORBIT_LIFETIME_WINDOW_START and ORBIT_LIFETIME_WINDOW_END or REENTRY_WINDOW_START and REENTRY_WINDOW_END. | % | O |
| **Ground impact and burn-up data** |
| `COMMENT` | Comments (allowed only at the beginning of each RDM data logical block). | n/a | O |
| `PROBABILITY_OF_IMPACT` | Probability that any fragment will impact the Earth (either land or sea; 0 to 1). | n/a | O |
| `PROBABILITY_OF_BURN_UP` | Probability that the entire object and any fragments will burn up during atmospheric re-entry (0 to 1). | n/a | O |
| `PROBABILITY_OF_BREAK_UP` | Probability that the object will break up during re-entry (0 to 1). | n/a | O |
| `PROBABILITY_OF_LAND_IMPACT` | Probability that any fragment will impact solid ground (0 to 1). | n/a | O |
| `PROBABILITY_OF_CASUALTY` | Probability that the re-entry event will cause any casualties (severe injuries or deaths—0 to 1). | n/a | O |
| `NOMINAL_IMPACT_EPOCH` | Epoch of the predicted impact (formatting rules specified in 5.3.3.5). | n/a | O |
| `IMPACT_WINDOW_START` | Start epoch of the predicted impact window (formatting rules specified in 5.3.3.5). | n/a | O |
| `IMPACT_WINDOW_END` | End epoch of the predicted impact window (formatting rules specified in 5.3.3.5). | n/a | O |
| `IMPACT_REF_FRAME` | Reference frame of the impact location data. The value should be taken from the keyword value name column in the SANA celestial body reference frames registry, reference [11]. Only frames with the value 'Body-Fixed' in the Frame Type column shall be used. Mandatory if NOMINAL_IMPACT_LON and NOMINAL_IMPACT_LAT are present. | n/a | O |
| `NOMINAL_IMPACT_LON` | Longitude of the predicted impact location with respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in 3.5.14. | deg | O |
| `NOMINAL_IMPACT_LAT` | Latitude of the predicted impact location with respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in 3.5.15. | deg | O |
| `NOMINAL_IMPACT_ALT` | Altitude of the impact location with respect to the value of IMPACT_REF_FRAME. | m | O |
| `IMPACT_1_CONFIDENCE` | First (lowest) confidence interval for the impact location. | % | O |
| `IMPACT_1_START_LON` | Longitude of the start of the first confidence interval along the ground track with respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in 3.5.14. | deg | O |
| `IMPACT_1_START_LAT` | Latitude of the start of the first confidence interval along the ground track with respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in 3.5.15. | deg | O |
| `IMPACT_1_STOP_LON` | Longitude of the end of the first confidence interval along the ground track with respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in 3.5.14. | deg | O |
| `IMPACT_1_STOP_LAT` | Latitude of the end of the first confidence interval along the ground track with respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in 3.5.15. | deg | O |
| `IMPACT_1_CROSS_TRACK` | Cross-track size of the first confidence interval. | km | O |
| `IMPACT_2_CONFIDENCE` | Second confidence interval for the impact location. The IMPACT_1_* block must be present if IMPACT_2_* is used. | % | O |
| `IMPACT_2_START_LON` | Longitude of the start of the second confidence interval along the ground track with respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in 3.5.14. | deg | O |
| `IMPACT_2_START_LAT` | Latitude of the start of the second confidence interval along the ground track with respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in 3.5.15. | deg | O |
| `IMPACT_2_STOP_LON` | Longitude of the end of the second confidence interval along the ground track with respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in 3.5.14. | deg | O |
| `IMPACT_2_STOP_LAT` | Latitude of the end of the second confidence interval along the ground track with respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in 3.5.15. | deg | O |
| `IMPACT_2_CROSS_TRACK` | Cross-track size of the second confidence interval. | km | O |
| `IMPACT_3_CONFIDENCE` | Third (highest) confidence interval for the impact location. The IMPACT_2_* block must be present if IMPACT_3_* is used. | % | O |
| `IMPACT_3_START_LON` | Longitude of the start of the third confidence interval along the ground track with respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in 3.5.14. | deg | O |
| `IMPACT_3_START_LAT` | Latitude of the start of the third confidence interval along the ground track with respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in 3.5.15. | deg | O |
| `IMPACT_3_STOP_LON` | Longitude of the end of the third confidence interval along the ground track with respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in 3.5.14. | deg | O |
| `IMPACT_3_STOP_LAT` | Latitude of the end of the third confidence interval along the ground track with respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in 3.5.15. | deg | O |
| `IMPACT_3_CROSS_TRACK` | Cross-track size of the third confidence interval. | km | O |
| **State vector** |
| `COMMENT` | Comments (allowed only at the beginning of each RDM data logical block). | n/a | O |
| `EPOCH` | Epoch at which the state vector is given (formatting rules specified in 5.3.3.5). | n/a | O |
| `X` | x-component of the object state vector. | km | O |
| `Y` | y-component of the object state vector. | km | O |
| `Z` | z-component of the object state vector. | km | O |
| `X_DOT` | x'-component of the object state vector. | km/s | O |
| `Y_DOT` | y'-component of the object state vector. | km/s | O |
| `Z_DOT` | z'-component of the object state vector. | km/s | O |
| **Position/velocity covariance matrix** |
| `COMMENT` | Comments (allowed only at the beginning of each RDM data logical block). | n/a | O |
| `COV_REF_FRAME` | Reference frame for the covariance information, omitted if it is the same as REF_FRAME. The value should be taken from the keyword value name column in the SANA orbit-relative reference frames registry, reference [12]. | n/a | O |
| `CX_X` | Covariance matrix [1,1]. | km**2 | O |
| `CY_X` | Covariance matrix [2,1]. | km**2 | O |
| `CY_Y` | Covariance matrix [2,2]. | km**2 | O |
| `CZ_X` | Covariance matrix [3,1]. | km**2 | O |
| `CZ_Y` | Covariance matrix [3,2]. | km**2 | O |
| `CZ_Z` | Covariance matrix [3,3]. | km**2 | O |
| `CX_DOT_X` | Covariance matrix [4,1]. | km**2/s | O |
| `CX_DOT_Y` | Covariance matrix [4,2]. | km**2/s | O |
| `CX_DOT_Z` | Covariance matrix [4,3]. | km**2/s | O |
| `CX_DOT_X_DOT` | Covariance matrix [4,4]. | km**2/s**2 | O |
| `CY_DOT_X` | Covariance matrix [5,1]. | km**2/s | O |
| `CY_DOT_Y` | Covariance matrix [5,2]. | km**2/s | O |
| `CY_DOT_Z` | Covariance matrix [5,3]. | km**2/s | O |
| `CY_DOT_X_DOT` | Covariance matrix [5,4]. | km**2/s**2 | O |
| `CY_DOT_Y_DOT` | Covariance matrix [5,5]. | km**2/s**2 | O |
| `CZ_DOT_X` | Covariance matrix [6,1]. | km**2/s | O |
| `CZ_DOT_Y` | Covariance matrix [6,2]. | km**2/s | O |
| `CZ_DOT_Z` | Covariance matrix [6,3]. | km**2/s | O |
| `CZ_DOT_X_DOT` | Covariance matrix [6,4]. | km**2/s**2 | O |
| `CZ_DOT_Y_DOT` | Covariance matrix [6,5]. | km**2/s**2 | O |
| `CZ_DOT_Z_DOT` | Covariance matrix [6,6]. | km**2/s**2 | O |
| **Object physical parameters** |
| `COMMENT` | Comments (allowed only at the beginning of each RDM data logical block). | n/a | O |
| `WET_MASS` | Total object mass at EPOCH_TZERO. | kg | O |
| `DRY_MASS` | Object dry mass (without propellant). | kg | O |
| `HAZARDOUS_SUBSTANCES` | Comma separated list of hazardous substances contained by the object. | n/a | O |
| `SOLAR_RAD_AREA` | Object area exposed to Solar Radiation Pressure (SRP). | m**2 | O |
| `SOLAR_RAD_COEFF` | Object solar radiation coefficient. | n/a | O |
| `DRAG_AREA` | Object cross-sectional area. | m**2 | O |
| `DRAG_COEFF` | Object drag coefficient. | n/a | O |
| `RCS` | Object radar cross section. | m**2 | O |
| `BALLISTIC_COEFF` | Object ballistic coefficient. | kg/m**2 | O |
| `THRUST_ACCELERATION` | The object's acceleration due to in-track thrust used to propagate the state vector and covariance to NOMINAL_RENTRY_EPOCH (if a controlled re-entry). | m/s**2 | O |
| **Orbit determination (OD) parameters** |
| `COMMENT` | Comments (allowed only at the beginning of each RDM data logical block). | n/a | O |
| `TIME_LASTOB_START` | The start of a time interval (in the time system specified in the metadata) that contains the time of the last accepted observation. For an exact time, the time interval is of zero duration (i.e., same value as that of TIME_LASTOB_END). Formatting rules are specified in 5.3.3.5. | n/a | O |
| `TIME_LASTOB_END` | The end of a time interval (in the time system specified in the metadata) that contains the time of the last accepted observation. For an exact time, the time interval is of zero duration (i.e., same value as that of TIME_LASTOB_START). Formatting rules are specified in 5.3.3.5. | n/a | O |
| `RECOMMENDED_OD_SPAN` | The recommended OD time span calculated for the object (double precision). | d | O |
| `ACTUAL_OD_SPAN` | Based on the observations available and the RECOMMENDED_OD_SPAN, the actual time span used for the OD of the object (double precision). | d | O |
| `OBS_AVAILABLE` | Number of observations available for orbit determination of the object. | n/a | O |
| `OBS_USED` | Number of observations accepted by the OD system. | n/a | O |
| `TRACKS_AVAILABLE` | Number of sensor tracks available for OD of the object. | n/a | O |
| `TRACKS_USED` | Number of sensor tracks accepted by the OD system. | n/a | O |
| `RESIDUALS_ACCEPTED` | The percentage of residuals accepted in the OD of the object. | % | O |
| `WEIGHTED_RMS` | The weighted root mean square of the residuals from batch OD. | n/a | O |
| **User defined parameters** |
| `COMMENT` | Comments (allowed only at the beginning of each RDM data logical block). | n/a | O |
| `USER_DEFINED_X` | User defined parameter, where x is replaced by a variable length user specified character string. Any number of user defined parameters may be included, if necessary to provide essential information that cannot be conveyed in COMMENT statements. | n/a | O |

3.5.3 Table 3-3 contains seven logical blocks, each of which has a descriptive heading. These descriptive headings shall not be included in an RDM, unless they appear in a properly formatted COMMENT statement (KVN comments specified in 5.3.5, XML in 5.4.4).

3.5.4 COMMENT lines may be used at the beginning of each logical block of the data section (comment placing is further specified in 5.2.5.2).

3.5.5 The ORBIT_LIFETIME keyword is mandatory and its value shall be used to convey both:
a) whether it is short-term or medium and long-term re-entry prediction (defined in 1.5);
b) the remaining orbital lifetime.

3.5.6 For short-term re-entry predictions the NOMINAL_REENTRY_EPOCH, REENTRY_WINDOW_START, and REENTRY_WINDOW_END keywords should be used.

3.5.7 If the NOMINAL_REENTRY_EPOCH, REENTRY_WINDOW_START, and REENTRY_WINDOW_END keywords are present, their values should be used in computations, rather than those of the ORBIT_LIFETIME, ORBIT_LIFETIME_WINDOW_START, and ORBIT_LIFETIME_WINDOW_END keywords.

3.5.8 If the NOMINAL_REENTRY_EPOCH keyword is present, the ORBIT_LIFETIME and NOMINAL_REENTRY_EPOCH keywords should resolve to the same value.

3.5.9 If both the ORBIT_LIFETIME_WINDOW_START and REENTRY_WINDOW_START keywords are present, they should both resolve to the same value. The same applies for ORBIT_LIFETIME_WINDOW_END and REENTRY_WINDOW_END.

3.5.10 If a ground impact location is given, the IMPACT_REF_FRAME keyword shall be mandatory and at least NOMINAL_IMPACT_LON and NOMINAL_IMPACT_LAT shall be present. NOMINAL_IMPACT_ALT may be given as well.

3.5.11 Values for all longitude keywords shall be between -180.0 and 180.0, with positive values for eastward longitudes and negative values for westward longitudes.

3.5.12 Values for all latitude keywords shall be between -90.0 and 90.0, with positive values for northern latitudes and negative values for southern latitudes.

3.5.13 If one confidence ‘n' interval is present then all the values associated with interval 'n' shall be present: IMPACT_n_CONFIDENCE, IMPACT_n_START_LON, IMPACT_n_START_LAT, IMPACT_n_STOP_LON, IMPACT_n_STOP_LAT, and IMPACT_n_CROSS_TRACK.

3.5.14 If only one confidence interval is present then the IMPACT_1_* keywords shall be used.

3.5.15 If two confidence intervals are present then the IMPACT_1_* and IMPACT_2_* keywords shall be used.

3.5.16 If more than one confidence interval is present, then IMPACT_1_CONFIDENCE shall be smaller than IMPACT_2_CONFIDENCE, which in turn shall be smaller than IMPACT_3_CONFIDENCE, if it is present.

3.5.17 If the REENTRY_DISINTEGRATION keyword in the metadata indicates that break-up was simulated, then the ground impact location keywords shall refer to all potential fragments related to the event; i.e., the NOMINAL_IMPACT_LAT and _LON will correspond to the highest probability that any fragment will impact there, the confidence intervals will apply to all fragments, etc.

3.5.18 The probability of impact should be within five percent at the following four points for each confidence interval given: the start and end of the confidence interval in the along-track direction, and the nominal impact location ± the cross-track confidence interval.

3.5.19 The state vector and covariance data are at the epoch specified by the EPOCH keyword in the data section. There are no restrictions on which epoch this is supposed to be (orbit determination epoch, message creation epoch, re-entry epoch, etc.). If the covariance block is present then the state vector shall be present as well.

3.5.20 If the state vector block is present then all elements in the block shall be present, with the exception of the comment line. No partial state vectors shall be present in an RDM. State vector values shall be expressed in standard double precision as related in 5.2.3.2.

3.5.21 Values in the covariance matrix shall be expressed in the applicable reference frame (COV_REF_FRAME keyword if used, or REF_FRAME keyword if not), and shall be presented sequentially from upper left [1,1] to lower right [6,6], lower triangular form, row by row, left to right. If the covariance block is present in the message, all covariance matrix elements shall be present. Variance and covariance values shall be expressed in standard double precision as related in 5.2.3.2.

3.5.22 Since re-entry prediction services are still in their infancy and some RDM originators might need to provide information that is not foreseen by this standard, a section of User Defined Parameters may be included at the end of the data section. In principle, User Defined Parameters provide flexibility, but also introduce complexity, non-standardization, potential ambiguity, and potential processing errors. Accordingly, if used, the keywords and their meanings must be described in an ICD. User Defined Parameters, if included in an RDM, should be used as sparingly as possible; their use is not encouraged.
# 4 RE-ENTRY DATA MESSAGE STRUCTURE & CONTENT (XML)

## 4.1 OVERVIEW—THE RDM/XML SCHEMA

This section applies only to the XML version.

The RDM/XML schema is available on the SANA Web site. SANA is the registrar for the protocol registries created under CCSDS.

The RDM XML schema explicitly defines the permitted data elements and values acceptable for the XML version of the RDM message. The location of the RDM/XML schema is:

- https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-2.0.0-rdm-1.0.xsd for messages with elements not qualified with respect to a namespace;
- https://sanaregistry.org/r/ndmxml_qualified/ndmxml-2.0.0-rdm-1.0.xsd for messages with elements qualified with respect to a namespace.

NOTE – Reference [13] subsection 4.3 has more information regarding messages with elements qualified with respect to a namespace.

Where possible this schema uses simple types and complex types used by the constituent schemas that make up Navigation Data Messages (specified in reference [13]).

## 4.2 RDM/XML BASIC STRUCTURE

4.2.1 Each RDM shall consist of a <header> and a <body>.

4.2.2 The RDM <body> shall consist of a single <segment> construct.

4.2.3 The RDM <segment> shall consist of a <metadata>/<data> pair, as shown in figure 4-1.

<figure>
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
<figcaption>Figure 4-1: RDM XML Basic Structure</figcaption>
</figure>

## 4.3 RDM/XML TAGS

4.3.1 An RDM XML tag shall be all uppercase if it corresponds directly to a KVN keyword from the header, metadata, or data sections.

4.3.2 There is an exception where there is not a strict correspondence between keywords in the KVN and tags in the XML implementations, specifically, the ‘CCSDS_RDM_VERS’ keyword from the RDM header. The ‘CCSDS_RDM_VERS’ keyword and its value shall appear as XML attributes rather than as an XML element.

4.3.3 RDM XML tags related to the XML message structure (i.e., that do not correspond directly to a KVN keyword) shall be in ‘lowerCamelCase’ (e.g., <header>, <segment>, <metadata>).

## 4.4 CONSTRUCTING AN RDM/XML INSTANCE

### 4.4.1 OVERVIEW

This subsection provides more detailed instructions for the user on how to create an XML message based on the ASCII-text KVN-formatted message described in 3.3 through 3.5.

### 4.4.2 XML VERSION

4.4.2.1 The first line in the instantiation shall specify the XML version:
`<?xml version="1.0" encoding="UTF-8"?>`

4.4.2.2 This line must appear on the first line of each instantiation, exactly as shown.

### 4.4.3 BEGINNING THE INSTANTIATION: ROOT DATA ELEMENT

4.4.3.1 An RDM instantiation shall be delimited with the <rdm></rdm> root element tags using the standard attributes documented in reference [13], subsection 4.3.

4.4.3.2 The XML Schema Instance namespace attribute must appear in the root element tag of all RDM/XML instantiations, exactly as shown:
`xmlns:xsi = "http://www.w3.org/2001/XMLSchema-instance"`

4.4.3.3 For messages with elements qualified with respect to a namespace, the NDM/XML namespace must next be coded, exactly as shown:
`xmlns:ndm="urn:ccsds:schema:ndmxml"`
The value that follows the ‘xmlns:’ in the NDM/XML name space (‘ndm’ in this case) is a prefix that must be used on every XML tag.

NOTE – This xmlns:ndm setting is only necessary for messages with elements qualified with respect to a namespace, but it does not hurt anything for it to appear on any NDM/XML instantiation.

4.4.3.4 If it is desired to validate an instantiation against the CCSDS Web-based schema, the xsi:noNamespaceSchemaLocation attribute must be coded as a single string of non-blank characters, with no line breaks, exactly as shown:
- `xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-2.0.0-master-2.0.xsd"` for messages with elements not qualified with respect to a namespace.
- `xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_qualified/ndmxml-2.0.0-master-2.0.xsd"` for messages with elements qualified with respect to a namespace.

NOTE – The length of the value associated with the xsi:noNamespaceSchemaLocation attribute can cause the string to wrap to a new line; however, the string itself contains no breaks.

4.4.3.5 The final attributes of the <rdm> tag shall be ‘id’ and ‘version’.
4.4.3.6 The ‘id’ attribute shall be ‘id="CCSDS_RDM_VERS"’. The ‘version’ attribute shall be ‘version="1.0"’.

### 4.4.4 THE RDM/XML HEADER SECTION

4.4.4.1 The RDM header shall have a standard header format, with tags <header> and </header>.

4.4.4.2 Immediately following the <header> tag the message may have any number of <COMMENT></COMMENT> tag pairs.

4.4.4.3 The standard RDM header shall contain the following element tags:
a) <CREATION_DATE>;
b) <ORIGINATOR>;
c) <MESSAGE_ID>.

NOTE – The rules for these keywords are specified in table 3-1. The header would look like this:
```xml
<header>
  <COMMENT>Some comment string.</COMMENT>
  <CREATION_DATE>2010-03-12T22:31:12.000</CREATION_DATE>
  <ORIGINATOR>AGENCYX</ORIGINATOR>
  <MESSAGE_ID>AGENCYX-1234</MESSAGE_ID>
</header>
```

### 4.4.5 THE RDM/XML BODY SECTION

4.4.5.1 After coding the <header>, the instantiation must include a <body></body> tag pair.
4.4.5.2 Inside the <body></body> tag pair must appear one <segment></segment> tag pair.
NOTE – In essence, the segment tag in the RDM XML implementation is not strictly necessary; however, it is necessary for structural symmetry with the overall NDM/XML paradigm (specified in reference [13], subsection 3.2).
4.4.5.3 The <segment> must be made up of one <metadata></metadata> tag pair and one <data></data> tag pair.

### 4.4.6 THE RDM/XML METADATA SECTION

4.4.6.1 The metadata section shall be set off by the <metadata></metadata> tag combination.
4.4.6.2 Between the <metadata> and </metadata> tags, the keywords shall be those specified in table 3-2.
4.4.6.3 Immediately following the <metadata> tag, the message may have any number of <COMMENT></COMMENT> tag pairs.

### 4.4.7 THE RDM DATA SECTION

4.4.7.1 The data section shall follow the metadata section and shall be set off by the <data></data> tag combination.
4.4.7.2 Between the <data> and </data> tags, the keywords shall be those specified in table 3-3 and the tags specified in table 4-1 (as specified in 4.4.8).
4.4.7.3 Immediately following the <data> tag, the message may have any number of <COMMENT></COMMENT> tag pairs.
4.4.7.4 Immediately following the special tags listed in table 4-1, the message may have any number of <COMMENT></COMMENT> tag pairs.

### 4.4.8 SPECIAL RDM/XML TAGS

Special tags that are not necessary in the KVN implementation shall be used to encapsulate the information in the XML implementation of the RDM. The special tags indicating data section logical block divisions shall be those defined in table 4-1.

### Table 4-1: Special RDM/XML Tags
| Special Tag | Definition |
| :--- | :--- |
| `<atmosphericReentryParameters>` | Delineates the logical block for atmospheric re-entry parameters. |
| `<groundImpactParameters>` | Delineates the logical block for ground impact and burn-up data parameters, if they are used. |
| `<stateVector>` | Delineates the state vector components in the coordinate system specified in the metadata, if it is used. |
| `<covarianceMatrix>` | Delineates the 6x6 position/velocity covariance matrix, if it is used. |
| `<spacecraftParameters>` | Delineates the logical block containing object physical parameters, if they are used. |
| `<odParameters>` | Delineates the logical block containing orbit determination parameters, if they are used. |
| `<userDefinedParameters>` | Delineates the logical block containing user defined parameters, if they are used. NOTE The use of <userDefinedParameters> is defined in reference [13], subsection 4.16. |

### 4.4.9 UNITS IN THE RDM/XML

The units in the RDM/XML shall be the same units used in the KVN-formatted RDM described in 3.4 and 3.5. XML attributes shall be used to explicitly define the units or other important information associated with the given data element (examples in annex C).

## 4.5 LOCAL OPERATIONS

4.5.1 For use in a local operations environment, the NDM/XML schema set (which includes the RDM schema) may be downloaded from the SANA Web site to a local server that meets local requirements for operations robustness.

4.5.2 If a local version is used, the value associated with the xsi:noNamespaceSchemaLocation attribute must be changed to a URL that is accessible to the local server.
# 5 RE-ENTRY DATA MESSAGE DATA AND SYNTAX

## 5.1 OVERVIEW

This section details the syntax requirements for the RDM using both the KVN and XML formats.

## 5.2 COMMON RDM SYNTAX

### 5.2.1 OVERVIEW

This subsection details the syntax requirements that are common to both KVN and XML formats.

### 5.2.2 RDM LINES

5.2.2.1 Each RDM line must not exceed 254 ASCII characters and spaces (excluding line termination character[s]).

5.2.2.2 Only printable ASCII characters and spaces shall be used. Control characters (such as TAB, etc.) shall not be used, with the exception of the line termination characters specified below.

5.2.2.3 Blank lines may be used at any position within the file. Blank lines shall have no assignable meaning, and may be ignored.

5.2.2.4 All lines shall be terminated by a single Carriage Return or a single Line Feed, or a Carriage Return/Line Feed pair, or a Line Feed/Carriage Return pair.

### 5.2.3 RDM VALUES

5.2.3.1 A nonempty, valid value must be specified for each mandatory keyword.

5.2.3.2 Non-integer numeric values may be expressed in either fixed-point or floating-point notation.

5.2.3.3 Text value fields must be constructed using only all uppercase (‘_’, ‘-’, space, and digits are permitted as well). An exception is made for comment values (comments are specified in 5.2.5).

5.2.3.4 Blanks shall not be permitted within numeric values and time strings.

### 5.2.4 RDM UNITS

5.2.4.1 If units are applicable, as specified in the tables in section 3, they must be displayed and they must exactly match the units specified in each table (including case; units conventions and operations are described in 1.5).

5.2.4.2 The notation ‘[n/a]’ shall not appear in an RDM as a units designator.

NOTE – Some of the items in table 3-3 are dimensionless (e.g., DRAG_COEFF). For such items, the table shows a unit value of ‘n/a’, which in this case means that there is no applicable units designator for those items.

### 5.2.5 RDM COMMENTS

5.2.5.1 Comment lines shall be optional in the RDM.

5.2.5.2 Comments shall be placed as specified in the tables in section 3 describing the keywords. In the places where comments are allowed any number of comments may appear.

5.2.5.3 Comment text may be in any case desired by the user.

## 5.3 THE RDM IN KVN

### 5.3.1 OVERVIEW

KVN instantiations of an RDM shall observe the syntax described in 5.3.

### 5.3.2 RDM LINES IN KVN

5.3.2.1 Each RDM file shall consist of a set of RDM lines. Each RDM line shall be one of the following:
– Header line;
– Metadata line;
– Data line; or
– Blank line.

5.3.2.2 The first header line must be the first non-blank line in the file.

5.3.2.3 All header, metadata, and data lines shall use ‘keyword = value’ notation. For this purpose, only those keywords shown in tables 3-1, 3-2, and 3-3, shall be used in an RDM.

5.3.2.4 Only a single ‘keyword = value’ assignment shall be made on a line.

5.3.2.5 Keywords must be uppercase and must not contain blanks.

5.3.2.6 Any white space immediately preceding or following the keyword shall not be significant.

5.3.2.7 Any white space immediately preceding or following the ‘equals’ sign shall not be significant.

5.3.2.8 Any white space immediately preceding or following the units shall not be significant.

5.3.2.9 Any white space immediately preceding the end of line shall not be significant.

5.3.2.10 The order in which mandatory and optional KVN assignments appear shall be fixed as shown in the tables that describe the RDM keywords in section 3.

### 5.3.3 RDM VALUES IN KVN

5.3.3.1 Integer values shall consist of a sequence of decimal digits with an optional leading sign (‘+’ or ‘-’). If the sign is omitted, ‘+’ shall be assumed. Leading zeroes may be used. The range of values that may be expressed as an integer is:
-2 147 483 648 ≤ x ≤ +2 147 483 647 (i.e., -2³¹ ≤ x ≤ 2³¹ - 1)

5.3.3.2 Non-integer numeric values expressed in fixed-point notation shall consist of a sequence of decimal digits separated by a period (‘.’) as a decimal point indicator, with an optional leading sign (‘+’ or ‘-’). If the sign is omitted, ‘+’ shall be assumed. Leading and trailing zeroes may be used. At least one digit shall appear before and after a decimal point. The number of digits shall be 16 or fewer.

5.3.3.3 Non-integer numeric values expressed in floating point notation shall consist of a sign, a mantissa, an alphabetic character indicating the division between the mantissa and exponent, and an exponent, constructed according to the following rules:
a) The sign may be ‘+’ or ‘-’. If the sign is omitted, ‘+’ shall be assumed.
b) The mantissa must be a string of no more than 16 decimal digits with a decimal point (‘.’) in the second position of the ASCII string, separating the integer portion of the mantissa from the fractional part of the mantissa.
c) The character used to denote exponentiation shall be ‘E’ or ‘e’. If the character indicating the exponent and the following exponent are omitted, an exponent value of zero shall be assumed (yielding a fixed point value).
d) The exponent must be an integer, and may have either a ‘+’ or ‘-’ sign; if the sign is omitted, then ‘+’ shall be assumed.
e) The maximum positive floating point value is approximately 1.798·10³⁰⁸, with 16 significant decimal digits precision. The minimum positive floating point value is approximately 4.94·10⁻³²⁴, with 16 significant decimal digits precision.

5.3.3.4 In value fields that are text, an underscore shall be equivalent to a single blank. Individual blanks shall be retained (shall be significant), but multiple contiguous blanks shall be equivalent to a single blank. This requirement does not apply to leading or trailing blanks, as per 5.3.2.7, 5.3.2.8, and 5.3.2.9.

5.3.3.5 In value fields representing a time tag or epoch, one of the following two formats shall be used:
`yyyy-mm-ddThh:mm:ss[.d→d][Z]`
or
`yyyy-dddThh:mm:ss[.d→d][Z]`
where ‘yyyy’ is the year, ‘mm’ is the two-digit month, ‘dd’ is the two-digit day of the month, and ‘ddd’ is the three-digit day of the year, separated by hyphens; ‘T’ is a fixed separator between the date and time portions of the string; and ‘hh:mm:ss[.d→d]’ is the time in hours, minutes, seconds, and fractional seconds, separated by colons. As many ‘d’ characters to the right of the period as required may be used to obtain the required precision, up to the maximum allowed for a fixed-point number. All fields require leading zeros. The format is specified in reference [14], subsection 3.5.

### 5.3.4 RDM UNITS IN KVN

When units are displayed:
a) there must be at least one blank character between the value and the units;
b) units shall be in the correct case (e.g., lowercase for km, uppercase for K or W; uppercase units are not used in the RDM), as indicated in table 3-3; and
c) the units must be enclosed within square brackets (e.g., ‘[kg]’).

### 5.3.5 RDM COMMENTS IN KVN

All comment lines shall begin with the ‘COMMENT’ keyword followed by at least one space. This keyword must appear on every comment line, not just the first such line. The remainder of the line shall be the comment value. White space shall be retained (shall be significant) in comment values.

## 5.4 THE RDM IN XML

### 5.4.1 OVERVIEW

XML instantiations of an RDM shall observe the syntax described in this subsection.

### 5.4.2 RDM LINES IN XML

Each RDM file shall consist of a set of RDM lines. Each RDM line shall be one of the following:
– XML version line;
– an XML-formatted line; or
– a blank line.

### 5.4.3 RDM VALUES IN XML

5.4.3.1 Each mandatory XML tag must be present and contain a valid value.
5.4.3.2 Integer values shall follow the conventions of the integer data type per reference [5], section 3.3.13. Additional restrictions on the allowable range of values permitted for any integer data element may also be defined in the RDM XML Schema.
NOTE – Examples of such restrictions may include a defined range (e.g., 0–100, 1–10), a set of enumerated values (e.g., 0, 1, 2, 4, 8), a pre-defined specific variation such as positiveInteger, or a user-defined data type variation.
5.4.3.3 Non-integer numeric values may be expressed in either fixed-point or floating-point notation. Numeric values shall follow the conventions of the double data type per reference [5], section 3.2.5. Additional restrictions on the allowable range of values permitted for any numeric data element may also be defined in the RDM XML Schema.
NOTE – Examples of such restrictions may include a defined range (e.g., 0.0–100.0), or a user-defined data type variation.
5.4.3.4 Text value data shall follow the conventions of the string data type per reference [5], section 3.2.1. Additional restrictions on the allowable range or values permitted for any data element may also be defined in the RDM XML Schema.
NOTE – Examples of such restrictions may include a set of enumerated values (e.g., ‘YES’/’NO’) or other user-defined data type variation.
5.4.3.5 Text values in RDM/XML instantiations (i.e., the values between the opening and closing tags), shall consist of either all uppercase or all lowercase characters; an exception is made for values between the <COMMENT> and </COMMENT> tags, which may be in any case desired by the user. Otherwise, mixing of uppercase and lowercase characters is prohibited.
5.4.3.6 In value fields that represent a time tag, values shall follow the conventions of the ndm:epochType data type used in all CCSDS NDM/XML schemas (as specified in 5.3.3.5).

### 5.4.4 RDM COMMENTS IN XML

Comments are optional and must be displayed as values between the <COMMENT> and </COMMENT> tags. Comments may be in any case desired by the user.
# ANNEX A

## IMPLEMENTATION CONFORMANCE STATEMENT PROFORMA (NORMATIVE)

### A1 INTRODUCTION

#### A1.1 OVERVIEW
This annex provides the Implementation Conformance Statement (ICS) Requirements List (RL) for an implementation of a Re-entry Data Message. The ICS for an implementation is generated by completing the RL in accordance with the instructions below. An implementation claiming conformance must satisfy the mandatory requirements referenced in the RL.

#### A1.2 ABBREVIATIONS AND CONVENTIONS
The RL consists of information in tabular form. The status of features is indicated using the abbreviations and conventions described below.

**Item Column**
The item column contains sequential numbers for items in the table.

**Feature Column**
The feature column contains a brief descriptive name for a feature. It implicitly means ‘Is this feature supported by the implementation?’.

**Status Column**
The status column uses the following notations:
- M: mandatory;
- O: optional;
- C: conditional;
- X: prohibited;
- I: out of scope;
- N/A: not applicable.

**Support Column Symbols**
The support column is to be used by the implementer to state whether a feature is supported by entering Y, N, or N/A, indicating:
- Y: Yes, supported by the implementation.
- N: No, not supported by the implementation.
- N/A: Not applicable.
The support column should also be used, when appropriate, to enter values supported for a given capability.

#### A1.3 INSTRUCTIONS FOR COMPLETING THE RL
An implementer shows the extent of compliance to the RDM standard by completing the RL; that is, the state of compliance with all mandatory requirements and the options supported are shown. The resulting completed RL is called an ICS. The implementer shall complete the RL by entering appropriate responses in the support or values supported column, using the notation described in A1.2. If a conditional requirement is inapplicable, N/A should be used. If a mandatory requirement is not satisfied, exception information must be supplied by entering a reference Xi, where i is a unique identifier, to an accompanying rationale for the noncompliance.

### A2 ICS PROFORMA FOR RDM

#### A2.1 GENERAL INFORMATION

##### A2.1.1 Identification of ICS
| | |
| :--- | :--- |
| Date of Statement (DD/MM/YYYY) | |
| ICS serial number | |
| System Conformance statement cross-reference | |

##### A2.1.2 Identification of Implementation Under Test
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
| Other information necessary for full identification, e.g., name(s) and version(s) for machines and/or operating systems; | |
| System Name(s) | |

##### A2.1.4 Identification of Specification
| | |
| :--- | :--- |
| CCSDS 508.1-B-1 | |
| Have any exceptions been required? | Yes [ ] No [ ] |
| NOTE – A YES answer means that the implementation does not conform to the Recommended Standard. Non-supported mandatory capabilities are to be identified in the ICS, with an explanation of why the implementation is non-conforming. | |

#### A2.2 REQUIREMENTS LIST
| Feature | Keyword | Reference | Status | Support |
| :--- | :--- | :--- | :--- | :--- |
| 1 RDM Header | N/A | table 3-1 | M | |
| 2 RDM version | `CCSDS_RDM_VERS` | table 3-1 | M | |
| 3 Comment | `COMMENT` | table 3-1 | O | |
| 4 Message creation date/time | `CREATION_DATE` | table 3-1 | M | |
| 5 Message originator | `ORIGINATOR` | table 3-1 | M | |
| 6 Unique message identifier | `MESSAGE_ID` | table 3-1 | M | |
| 7 RDM Metadata | N/A | table 3-2 | M | |
| 8 Comment | `COMMENT` | table 3-2 | O | |
| 9 General information about the re-entering object | N/A | table 3-2 | M | |
| 10 Spacecraft name | `OBJECT_NAME` | table 3-2 | M | |
| 11 Spacecraft international designator | `INTERNATIONAL_DESIGNATOR` | table 3-2 | M | |
| 12 Object catalogue used | `CATALOG_NAME` | table 3-2 | O | |
| 13 Spacecraft ID in the catalogue | `OBJECT_DESIGNATOR` | table 3-2 | O | |
| 14 Spacecraft type | `OBJECT_TYPE` | table 3-2 | O | |
| 15 Spacecraft owner | `OBJECT_OWNER` | table 3-2 | O | |
| 16 Spacecraft operator | `OBJECT_OPERATOR` | table 3-2 | O | |
| 17 Controlled re-entry | `CONTROLLED_REENTRY` | table 3-2 | M | |
| 18 Celestial body orbited by the object | `CENTER_NAME` | table 3-2 | M | |
| 19 Time system used for the data in the metadata and data sections | `TIME_SYSTEM` | table 3-2 | M | |
| 20 Reference epoch to which orbit lifetime is computed | `EPOCH_TZERO` | table 3-2 | M | |
| 21 Information about any provided orbit data for the object/spacecraft | N/A | table 3-2 | O | |
| 22 Reference frame | `REF_FRAME` | table 3-2 | O | |
| 23 Reference frame epoch | `REF_FRAME_EPOCH` | table 3-2 | O | |
| 24 External ephemeris file identifier | `EPHEMERIS_NAME` | table 3-2 | O | |
| 25 Information about the orbit propagator used for re-entry prediction and orbit determination | N/A | table 3-2 | O | |
| 26 gravity model used | `GRAVITY_MODEL` | table 3-2 | O | |
| 27 atmospheric model used | `ATMOSPHERIC_MODEL` | table 3-2 | O | |
| 28 method used to estimate the solar flux indices for the atmospheric model | `SOLAR_FLUX_PREDICTION` | table 3-2 | O | |
| 29 n-body perturbations considered | `N_BODY_PERTURBATIONS` | table 3-2 | O | |
| 30 solar radiation pressure | `SOLAR_RAD_PRESSURE` | table 3-2 | O | |
| 31 Earth tides | `EARTH_TIDES` | table 3-2 | O | |
| 32 any thrust from the spacecraft | `INTRACK_THRUST` | table 3-2 | O | |
| 33 the source of the object drag parameters | `DRAG_PARAMETERS_SOURCE` | table 3-2 | O | |
| 34 the altitude at which the drag parameters apply | `DRAG_PARAMETERS_ALTITUDE` | table 3-2 | O | |
| 35 the method used to estimate the uncertainty in the re-entry date | `REENTRY_UNCERTAINTY_METHOD` | table 3-2 | O | |
| 36 any accounting for break-up and demise | `REENTRY_DISINTEGRATION` | table 3-2 | O | |
| 37 method used to determine the uncertainty in the ground impact location | `IMPACT_UNCERTAINTY_METHOD` | table 3-2 | O | |
| 38 Previous and next related RDM | N/A | table 3-2 | O | |
| 39 Identifier of the previous RDM issued | `PREVIOUS_MESSAGE_ID` | table 3-2 | O | |
| 40 Time at which the previous RDM was issued | `PREVIOUS_MESSAGE_EPOCH` | table 3-2 | O | |
| 41 Time at which the next RDM will be issued (predicted) | `NEXT_MESSAGE_EPOCH` | table 3-2 | O | |
| 42 RDM Data | N/A | table 3-3 | M | |
| 43 Atmospheric re-entry information | N/A | table 3-3 | M | |
| 44 Comment | `COMMENT` | table 3-3 | O | |
| 45 Remaining orbit lifetime | `ORBIT_LIFETIME` | table 3-3 | M | |
| 46 Defined re-entry altitude | `REENTRY_ALTITUDE` | table 3-3 | M | |
| 47 Orbit lifetime window | `ORBIT_LIFETIME_WINDOW_START` | table 3-3 | O | |
| 48 | `ORBIT_LIFETIME_WINDOW_END` | table 3-3 | O | |
| 49 Predicted re-entry epoch | `NOMINAL_REENTRY_EPOCH` | table 3-3 | O | |
| 50 Re-entry window | `REENTRY_WINDOW_START` | table 3-3 | O | |
| 51 | `REENTRY_WINDOW_END` | table 3-3 | O | |
| 52 Orbit lifetime confidence level | `ORBIT_LIFETIME_CONFIDENCE_LEVEL` | table 3-3 | O | |
| 53 Ground impact information | N/A | table 3-3 | O | |
| 54 Comment | `COMMENT` | table 3-3 | O | |
| 55 Probability any re-entry fragments reach the Earth surface | `PROBABILITY_OF_IMPACT` | table 3-3 | O | |
| 56 Probability all re-entry fragments suffer total demise | `PROBABILITY_OF_BURN_UP` | table 3-3 | O | |
| 57 Probability fragments are generated during re-entry | `PROBABILITY_OF_BREAK_UP` | table 3-3 | O | |
| 58 Land impact probability | `PROBABILITY_OF_LAND_IMPACT` | table 3-3 | O | |
| 59 Probabilities of casualties resulting from the re-entry | `PROBABILITY_OF_CASUALTY` | table 3-3 | O | |
| 60 Predicted (ground) impact epoch | `NOMINAL_IMPACT_EPOCH` | table 3-3 | O | |
| 61 Predicted (ground) impact window | `IMPACT_WINDOW_START` | table 3-3 | O | |
| 62 | `IMPACT_WINDOW_END` | table 3-3 | O | |
| 63 Reference frame of (ground) impact location | `IMPACT_REF_FRAME` | table 3-3 | O | |
| 64 Impact location | `NOMINAL_IMPACT_LON` | table 3-3 | O | |
| 65 | `NOMINAL_IMPACT_LAT` | table 3-3 | O | |
| 66 | `NOMINAL_IMPACT_ALT` | table 3-3 | O | |
| 67 Impact dispersion | `IMPACT_1_CONFIDENCE` | table 3-3 | O | |
| 68 | `IMPACT_1_START_LON` | table 3-3 | O | |
| 69 | `IMPACT_1_START_LAT` | table 3-3 | O | |
| 70 | `IMPACT_1_STOP_LON` | table 3-3 | O | |
| 71 | `IMPACT_1_STOP_LAT` | table 3-3 | O | |
| 72 | `IMPACT_1_CROSS_TRACK` | table 3-3 | O | |
| 73 | `IMPACT_2_CONFIDENCE` | table 3-3 | O | |
| 74 | `IMPACT_2_START_LON` | table 3-3 | O | |
| 75 | `IMPACT_2_START_LAT` | table 3-3 | O | |
| 76 | `IMPACT_2_STOP_LON` | table 3-3 | O | |
| 77 | `IMPACT_2_STOP_LAT` | table 3-3 | O | |
| 78 | `IMPACT_2_CROSS_TRACK` | table 3-3 | O | |
| 79 | `IMPACT_3_CONFIDENCE` | table 3-3 | O | |
| 80 | `IMPACT_3_START_LON` | table 3-3 | O | |
| 81 | `IMPACT_3_START_LAT` | table 3-3 | O | |
| 82 | `IMPACT_3_STOP_LON` | table 3-3 | O | |
| 83 | `IMPACT_3_STOP_LAT` | table 3-3 | O | |
| 84 | `IMPACT_3_CROSS_TRACK` | table 3-3 | O | |
| 85 Spacecraft state vector section | N/A | table 3-3 | O | |
| 86 Comment | `COMMENT` | table 3-3 | O | |
| 87 State vector epoch | `EPOCH` | table 3-3 | O | |
| 88 Position and velocity components | `X` | table 3-3 | O | |
| 89 | `Y` | table 3-3 | O | |
| 90 | `Z` | table 3-3 | O | |
| 91 | `X_DOT` | table 3-3 | O | |
| 92 | `Y_DOT` | table 3-3 | O | |
| 93 | `Z_DOT` | table 3-3 | O | |
| 94 Spacecraft state vector covariance information | N/A | table 3-3 | O | |
| 95 Comment | `COMMENT` | table 3-3 | O | |
| 96 Covariance reference frame | `COV_REF_FRAME` | table 3-3 | O | |
| 97 6x6 position/velocity covariance matrix elements | `CX_X` | table 3-3 | O | |
| 98 | `CY_X` | table 3-3 | O | |
| 99 | `CY_Y` | table 3-3 | O | |
| 100 | `CZ_X` | table 3-3 | O | |
| 101 | `CZ_Y` | table 3-3 | O | |
| 102 | `CZ_Z` | table 3-3 | O | |
| 103 | `CX_DOT_X` | table 3-3 | O | |
| 104 | `CX_DOT_Y` | table 3-3 | O | |
| 105 | `CX_DOT_Z` | table 3-3 | O | |
| 106 | `CX_DOT_X_DOT` | table 3-3 | O | |
| 107 | `CY_DOT_X` | table 3-3 | O | |
| 108 | `CY_DOT_Y` | table 3-3 | O | |
| 109 | `CY_DOT_Z` | table 3-3 | O | |
| 110 | `CY_DOT_X_DOT` | table 3-3 | O | |
| 111 | `CY_DOT_Y_DOT` | table 3-3 | O | |
| 112 | `CZ_DOT_X` | table 3-3 | O | |
| 113 | `CZ_DOT_Y` | table 3-3 | O | |
| 114 | `CZ_DOT_Z` | table 3-3 | O | |
| 115 | `CZ_DOT_X_DOT` | table 3-3 | O | |
| 116 | `CZ_DOT_Y_DOT` | table 3-3 | O | |
| 117 | `CZ_DOT_Z_DOT` | table 3-3 | O | |
| 118 Spacecraft properties | N/A | table 3-3 | O | |
| 119 Comment | `COMMENT` | table 3-3 | O | |
| 120 Total (wet) mass | `WET_MASS` | table 3-3 | O | |
| 121 Dry mass | `DRY_MASS` | table 3-3 | O | |
| 122 List of hazardous substances on board | `HAZARDOUS_SUBSTANCES` | table 3-3 | O | |
| 123 Solar radiation area | `SOLAR_RAD_AREA` | table 3-3 | O | |
| 124 Solar radiation coefficient | `SOLAR_RAD_COEFF` | table 3-3 | O | |
| 125 Drag area | `DRAG_AREA` | table 3-3 | O | |
| 126 Coefficient of drag | `DRAG_COEFF` | table 3-3 | O | |
| 127 Radar Cross-Section | `RCS` | table 3-3 | O | |
| 128 Ballistic coefficient | `BALLISTIC_COEFF` | table 3-3 | O | |
| 129 Object’s in-track acceleration used in OD and propagation | `THRUST_ACCELERATION` | table 3-3 | O | |
| 130 Orbit determination information | N/A | table 3-3 | O | |
| 131 Comment | `COMMENT` | table 3-3 | O | |
| 132 Interval during which the last accepted observation occurred | `TIME_LASTOB_START` | table 3-3 | O | |
| 133 | `TIME_LASTOB_END` | table 3-3 | O | |
| 134 Recommend timespan for OD and actual timespan used | `RECOMMENDED_OD_SPAN` | table 3-3 | O | |
| 135 | `ACTUAL_OD_SPAN` | table 3-3 | O | |
| 136 Total number of observations available and used | `OBS_AVAILABLE` | table 3-3 | O | |
| 137 | `OBS_USED` | table 3-3 | O | |
| 138 Total number of sensor tracks available and used | `TRACKS_AVAILABLE` | table 3-3 | O | |
| 139 | `TRACKS_USED` | table 3-3 | O | |
| 140 Percentage of residuals accepted in OD | `RESIDUALS_ACCEPTED` | table 3-3 | O | |
| 141 Weighted RMS of the OD residuals | `WEIGHTED_RMS` | table 3-3 | O | |
| 142 User defined parameters | `USER_DEFINED_X` | table 3-3 | O | |
| | Any supported user defined parameters should be listed here. | | | |
# ANNEX B

## SECURITY, SANA, AND PATENT CONSIDERATIONS (INFORMATIVE)

### B1 SECURITY CONSIDERATIONS

This subsection presents the results of an analysis of security considerations applied to the technologies specified in this Recommended Standard.

#### B1.1 SECURITY CONCERNS WITH RESPECT TO THE CCSDS DOCUMENT

##### B1.1.1 Data Privacy
Privacy of data formatted in compliance with the specifications of this Recommended Standard should be assured by the systems and networks on which this Recommended Standard is implemented.

##### B1.1.2 Data Integrity
Integrity of data formatted in compliance with the specifications of this Recommended Standard should be assured by the systems and networks on which this Recommended Standard is implemented.

##### B1.1.3 Authentication of Communicating Entities
Authentication of communicating entities involved in the transport of data which complies with the specifications of this Recommended Standard should be provided by the systems and networks on which this Recommended Standard is implemented.

##### B1.1.4 Control of Access to Resources
Control of access to resources should be managed by the systems upon which originator formatting and recipient processing are performed.

##### B1.1.5 Auditing of Resource Usage
Auditing of resource usage should be handled by the management of systems and networks on which this Recommended Standard is implemented.

#### B1.2 POTENTIAL THREATS AND ATTACK SCENARIOS
Potential threats or attack scenarios include, but are not limited to, (a) unauthorized access to the programs/processes that generate and interpret the messages, (b) unauthorized access to the messages during transmission between exchange partners, and (c) modification of the messages between partners. Protection from unauthorized access during transmission is especially important if the RDM provider and users utilize open ground networks, such as the Internet, to exchange data formatted in compliance with this Recommended Standard. It is strongly recommended that potential threats or attack scenarios applicable to the systems and networks on which this Recommended Standard is implemented be addressed by the management of those systems and networks.

#### B1.3 CONSEQUENCES OF NOT APPLYING SECURITY TO THE TECHNOLOGY
The consequences of not applying security to the systems and networks on which this Recommended Standard is implemented could include potential loss, corruption, and theft of data. The orbit data included in the RDM could allow an unintended recipient to track a re-entering spacecraft, and the ground impact data could allow them to collect any debris in an unauthorized fashion.

### B2 SANA CONSIDERATIONS
The following RDM-related items have been be registered with the SANA Operator:
- The RDM XML schemas in Navigation Data Messages XML Schema.

The following RDM elements should be taken from the appropriate SANA registry:
- The RDM originator, object owner, and object operator should be taken from the SANA Organizations registry;
- The catalog name should be taken from the SANA CDM catalog name registry;
- The center name should be taken from the SANA Orbit Centers registry;
- The time system should be taken from the SANA Time Systems registry;
- The reference frame and impact reference frame should be taken from the SANA Celestial Body Reference Frames registry;
- The covariance reference frame should be taken from the SANA Orbit-relative Reference Frames registry.

The registration rule for new entries in any of the registries listed above is the approval of new requests by the CCSDS Area or Working Group responsible for the maintenance of the RDM or of the respective registry at the time of the request. New requests for these registries should be sent to SANA (info@sanaregistry.org).

### B3 PATENT CONSIDERATIONS
The recommendations of this Recommended Standard have no patent issues.
# ANNEX C

## RE-ENTRY DATA MESSAGE EXAMPLES

(INFORMATIVE)

Figures C-1 and C-2 show examples of RDM messages in KVN. The former only includes mandatory keywords, while the latter uses optional keywords. Figure C-1 presents a very basic RDM, the kind a Member Agency (or any other group) would make available to the interested public (e.g., amateur astronomers wishing to schedule the observation of a re-entry) for long-term re-entry prediction and containing only the remaining orbital lifetime. Figure C-2 shows a more complex message, the kind of information two Member Agencies might exchange with each other for short-term re-entry predictions and containing the state vector, position/velocity covariance matrix.

MISSING IMAGE (Figure C-1: Sample RDM in KVN Using Only Mandatory Keywords)
MISSING IMAGE (Figure C-2: Sample RDM in KVN Using Optional Keywords)
MISSING IMAGE (Figure C-2: Sample RDM in KVN Using Optional Keywords (Continued))

Figures C-3 and C-4 show examples of RDM messages in the XML format. Some of the lines wrap in this representation, but would not wrap in an actual RDM file given the 254 character line length.

MISSING IMAGE (Figure C-3: Sample RDM in XML Using Only Mandatory Keywords)
MISSING IMAGE (Figure C-4: Sample RDM in XML Using Optional Keywords)
MISSING IMAGE (Figure C-4: Sample RDM in XML Using Optional Keywords (Continued))
MISSING IMAGE (Figure C-4: Sample RDM in XML Using Optional Keywords (Continued))
# ANNEX D

## ABBREVIATIONS AND ACRONYMS

(INFORMATIVE)

| | |
| :--- | :--- |
| ASCII | American Standard Code for Information Interchange |
| CCSDS | Consultative Committee for Space Data Systems |
| CDM | Conjunction Data Message |
| COSPAR | Committee on Space Research |
| DOF | degree of freedom |
| IADC | Inter-Agency Debris Coordination Committee |
| ICD | interface control document |
| ICS | implementation conformance statement |
| ID | identifier |
| ISO | International Organization for Standardization |
| KVN | keyword value notation |
| NDM | Navigation Data Message |
| OD | orbit determination |
| ODM | Orbit Data Message |
| OEM | Orbit Ephemeris Message |
| OPM | Orbit Parameter Message |
| PDF | probability distribution function |
| RDM | Re-entry Data Message |
| RL | requirements list |
| RMS | root mean square |
| SANA | Space Assigned Numbers Authority |
| SI | International System of Units (Système international d’unités) |
| SRP | solar radiation pressure |
| SSA | space situational awareness |
| SST | space surveillance and tracking |
| UNOOSA | United Nations Office for Outer Space Affairs |
| URL | uniform resource locator |
| UTC | Coordinated Universal Time |
| XML | Extensible Markup Language |
# ANNEX E

## RATIONALE AND REQUIREMENTS FOR RE-ENTRY DATA MESSAGES (INFORMATIVE)

This annex presents the rationale and requirements behind the design of the Re-entry Data Message. Table E-1 shows the RDM requirements':
- requirement identifier;
- requirement text;
- rationale behind the requirement;
- traceability to sections of this standard; and
- whether the requirement is mandatory (M) (shall) or optional (O), but desirable (should).

### Table E-1: RDM Requirements and Rationale
| # | Requirement | Rationale | Trace | M/O |
| :--- | :--- | :--- | :--- | :--- |
| RDM-0010 | The RDM shall be provided in digital form. | To facilitate automated interaction. | 3.2.2 | M |
| RDM-0020 | The RDM shall be provided in platform-independent data structures. | The CCSDS objective of promoting interoperability is best met by avoiding proprietary data formats. | section 3 | M |
| RDM-0025 | The RDM shall be readable by both humans and computers. | To facilitate both automatic interaction and analysis of the data by human operators. | table 3-1<br>table 3-2<br>table 3-3 | M |
| RDM-0030 | The RDM shall provide a means of uniquely identifying each message. | The file name is not sufficient for these purposes and a unique ID facilitates exchange. | table 3-1 | M |
| RDM-0040 | The RDM shall clearly identify the object predicted to (re-)enter. | Any ambiguity in the object reduces the usefulness of the message. | table 3-2 | M |
| RDM-0050 | The RDM shall provide the remaining orbit lifetime of the object. | This information is needed to determine the timeliness of any needed mitigation measures. | table 3-3 | M |
| RDM-0055 | The RDM shall specify whether the re-entry is controlled, uncontrolled, or unknown. | To determine if/what mitigation measures are needed. Unknown is a valid value as well. | table 3-2 | M |
| RDM-0060 | The RDM shall provide time measurements in the accepted CCSDS timecode formats. | To fulfill the CCSDS’ objective of promoting interoperability and to make them easier to read and understand by humans. | 5.3.3.5 | M |
| RDM-0070 | The RDM shall provide the units used for all the data. | To promote interoperability and eliminate ambiguity. | table 3-2<br>table 3-3 | M |
| RDM-0080 | The RDM shall allow the exchange of (re-)entry information for objects orbiting an arbitrary body. | Allows the use of the standard for Moon/Mars/Jupiter atmospheric entry besides Earth, further promoting the use of this standard. | table 3-2 | M |
| RDM-0090 | The information in the RDM shall be usable without the need to model any (re-)entry spacecraft dynamics by the receiving entity. | There are several situations in which this is useful, such as remaining orbit lifetime is too short for modeling or the receiving entity does not have those capabilities. | table 3-3 | M |
| RDM-0100 | The RDM should be extensible, without disrupting existing uses. | Some users might need other parameters and they should be able to use them with user defined lines specified in an ICD. | table 3-3 | O |
| RDM-0110 | The RDM should be consistent with the other CCSDS messages. | To facilitate use and improve readability. | table 3-1<br>table 3-2<br>table 3-3 | O |
| RDM-0120 | The RDM shall allow for the object owner/operator to be specified. | Can help with mitigation measures. | table 3-2 | M |
| RDM-0130 | The RDM should allow the exchange of one spacecraft position/velocity state vector and associated information. | Allows the message to be self-contained and not rely on ODMs (which might require an ICD). | table 3-3 | O |
| RDM-0140 | The RDM should allow the exchange of covariance information for the position/velocity state vector. | To allow the uncertainty in the state vector to be exchanged as well. | table 3-3 | O |
| RDM-0150 | The RDM should allow the modeling used to determine the data to be specified. | Necessary if the data is to be verified by the received agency. | table 3-2 | O |
| RDM-0160 | The RDM shall contain information about the previous and next messages for the same spacecraft. | Because of modeling uncertainties it is expected that RDMs for the same spacecraft would be published regularly. | table 3-2 | M |
| RDM-0170 | The RDM shall specify the predicted ground impact location and epoch for the object. | Important information if ground impact is predicted. | table 3-3 | M |
| RDM-0180 | The RDM should contain the spacecraft parameters used in OD and re-entry prediction. | Allows verification and independent orbit propagation. | table 3-3 | O |
| RDM-0190 | The RDM shall contain information about the observations used in OD. | Allows the receiving agency to evaluate the reliability of the data. | table 3-3 | M |
# ANNEX F

## RDM SUMMARY SHEET (INFORMATIVE)

The provision of optional keywords is at the discretion of the data provider. This annex is intended to provide a helpful guide (table F-1) in associating optional metadata keywords (from table 3-2) and data categories (from table 3-3). There are only a few required metadata and data keywords, but many more that are applicable to one or more data categories. Some data categories can also provide extra information for other categories. Additionally, there are some keywords that are only applicable in certain restricted situations. Finally, there are some metadata keywords that are completely optional. This summary may assist the user in constructing an RDM that properly describes a specific re-entry event.

The terms 'required' and 'recommended' in table F-1 relate only to the context set by the data categories and are not normative for the RDM.

### Table F-1: Relationship between RDM Data Categories and Optional Metadata Keywords

| Data category | Relevant metadata optional keywords | Relevant data blocks |
| :--- | :--- | :--- |
| **long-term atmospheric re-entry** | required: none (only mandatory keywords) <br> recommended: <br> `GRAVITY_MODEL` <br> `ATMOSPHERIC_MODEL` <br> `SOLAR_FLUX_PREDICTION` <br> `N_BODY_PERTURBATIONS` | required: none (only mandatory keywords) <br> recommended: <br> object physical parameters |
| **short-term atmospheric re-entry** | required: none (only mandatory keywords) <br> recommended: <br> `GRAVITY_MODEL` <br> `ATMOSPHERIC_MODEL` <br> `SOLAR_FLUX_PREDICTION` <br> `N_BODY_PERTURBATIONS` | required: none (only mandatory keywords) <br> recommended: <br> `NOMINAL_REENTRY_EPOCH` <br> `REENTRY_WINDOW_START` <br> `REENTRY_WINDOW_END` <br> object physical parameters |
| **ground impact data** | required: <br> `REENTRY_DISINTEGRATION` <br> recommended: <br> same as short-term atmospheric re-entry, and: <br> `IMPACT_UNCERTAINTY_METHOD` | required: ground impact and burn-up data <br> recommended: <br> same as short-term atmospheric re-entry |
| **object state vector** | required: <br> `REF_FRAME` | required: object state vector <br> recommended: <br> object physical parameters |
| **position/velocity covariance matrix** | required: none (only mandatory keywords) | required: position/velocity covariance matrix, object state vector |
| **object physical parameters** | required: none (only mandatory keywords) | required: object physical parameters |
| **orbit determination parameters** | required: <br> `GRAVITY_MODEL` <br> `ATMOSPHERIC_MODEL` <br> `SOLAR_FLUX_PREDICTION` <br> `N_BODY_PERTURBATIONS` <br> `SOLAR_RAD_PRESSURE` <br> `EARTH_TIDES` <br> `INTRACK_THRUST` | required: object physical parameters |
# ANNEX G

## RE-ENTRY INFORMATION DESCRIPTION

(INFORMATIVE)

### G1 RE-ENTRY PREDICTION

#### G1.1 OVERVIEW

As of April 2017, slightly more than 24000, or 56 percent of all catalogued objects, have decayed and re-entered the atmosphere and several more re-enter every month. Re-entries can pose risk to ground based infrastructure and population, either due to the large mass of the spacecraft (e.g., Mir, Skylab, or the Salyut space stations), or due to hazardous materials (e.g., Cosmos 954).

Predicting and simulating re-entries covers:

- long and medium-term re-entry predictions (several years to a few weeks before re-entry);
- short-term re-entry predictions (last days/weeks before re-entry; altitude below 200 km);
- break-up and survival (out of scope of the RDM); and
- ground impact and on-ground risk (partially out of scope of the RDM, due to limitations in break-up and survival estimations).

The following subsections describe re-entry simulations and modeling for the Earth, but similar considerations apply to other celestial bodies. General information on navigation data can be found in reference [H1]. Re-entry simulation details can be found in chapter 9 of [H2], which describes re-entry simulations and how they applied to the Mir re-entry. Details on the European Space Agency’s contributions to IADC (Inter-Agency Debris Coordination committee) re-entry campaigns can be found in references [H3] and [H4].

The estimation of the orbit lifetime is covered by an ISO standard, ISO 27852:2016 (reference [H5]). The standard focuses on long- and medium-term predictions and contains useful information to determine which mathematical models (e.g., gravity and atmospheric models) are appropriate in most situations. ISO is also working on a standard on atmospheric modelling, but as of the writing of this document, it has not been completed.

#### G1.2 NOMENCLATURE

The following terms will be used in equations in the following subsections:

- $a$: semi-major axis
- $A$: area
- $B$: ballistic coefficient
- $C_d$: drag coefficient
- $C_p$: specific heat at constant pressure
- $g_0$: sea-level gravitational acceleration
- $h_m$: melting enthalpy
- $m$: mass
- $Q$: heat
- $t$: time
- $T$: temperature
- $T_m$: melting temperature
- $v$: velocity
- $V$: volume
- $v_a$: velocity relative to the atmosphere
- $V_{\text{impact}}$: impact velocity
- $\epsilon$: radiation emission coefficient
- $\rho$: density
- $\rho_0$: Earth sea level atmospheric density
- $\sigma$: Stefan-Boltzmann constant

### G1.3 LONG- AND MEDIUM-TERM RE-ENTRY PREDICTIONS

In the context of re-entry predictions, long- and medium-term means several years to a few weeks of orbital lifetime remaining. The current typical approach for these simulations is to use singly averaged perturbation equations integrated over a time step between 0.1 and 5 orbits. The perturbations accounted for by existing modeling software are: non-spherical Earth gravity, dynamic Earth atmosphere, luni-solar gravity perturbations, and solar radiation pressure combined with an oblate cylindrical Earth shadow. The end goal of these simulations is to estimate the remaining orbital lifetime.

Long- and medium-term re-entry predictions require simulating the decrease in the orbit’s semi-major axis. For nearly circular orbits the rate of change of the semi-major axis with respect to time can be approximated by:

$$ \frac{da}{dt} = -\frac{2}{a} \rho v_a B $$ 

$$ B = \frac{m}{C_d A} $$ 

where $v_a$ is the velocity with respect to the atmosphere. While $v_a$ and the atmospheric density can be estimated using Earth atmosphere models and orbit dynamics, the ballistic coefficient cannot. Uncontrolled objects tumble, so their angle of attack, and hence aerodynamic cross-section and drag coefficient are not constant. One approach to this issue is to retro-fit the simulated semi-major axis time history over observed values over a period of time by adjusting the ballistic coefficient.

Besides the ballistic coefficient, other sources of uncertainty in the results are: atmospheric density modeling (both the models themselves and unexpected space weather events), attitude, and the orbit state. A standard deviation of $\pm 20$ percent for the remaining orbital lifetime is a good estimate for all the above in nominal conditions. More complex uncertainty modeling is also possible, e.g., accounting for orbit covariance and ballistic coefficient fit residuals.

#### G1.4 SHORT-TERM RE-ENTRY PREDICTIONS AND IMPACT

When the orbit altitude drops below $\sim 200$ km, the assumptions used in long-term predictions (perturbation effects are small, no cross-coupling effects) no longer hold and more accurate modeling is needed. Numerical solutions to the perturbed equations of motion are used, typically accounting for: Earth gravity, luni-solar attraction, solar radiation pressure with umbra and penumbra transits, and atmospheric drag. Some success has been demonstrated for controlled re-entry with 6DOF numerical integration of the satellite states and attitude.

Below $\sim 120$ km atmospheric drag becomes the dominating force, and properly accounting for the change in drag coefficient between free-molecular flow, transitional, and continuum conditions is critical. For example, the drag coefficient of the Mir space station decreased from 2.21 in the free-molecular flow regime to 1.18 in the continuum subsonic regime. Atmospheric drag steepens the flight path, making the orbit more eccentric. At some point the re-entering object will achieve equilibrium free fall, leading to an impact velocity:

$$ V_{\text{impact}} = \sqrt{\frac{2mg_0}{C_d A \rho_0}} $$ 

Dispersion in the impact location of a re-entry survivor object is driven by uncertainties in the area-to-mass ratio, aerodynamic coefficients (drag, lift, and side-force), air density, and initial orbit and attitude state and it is given in the along- and cross-track directions. Monte Carlo simulations are needed to determine a geographic probability distribution. The probability density function in a given area can be computed as the fraction of impact in said area over total number of simulations. The probability distribution function (PDF) in the along-track direction is a distorted 2D Gaussian, further complicating matters.

Simpler approaches are sometimes used. Analytical formulae allow the computation of dispersion due to aerodynamic forces, based on lift, drag, and side-force coefficients and the ratio of lift and side-force, but they can be difficult to use and do not account for the main sources of uncertainty. An alternative is to assume a standard Gaussian distribution, compute the along-track dispersion by varying the aerodynamic drag term ($p-B$) and use an empirical value for the cross-track dispersion (e.g., a 1-sigma value of 20 km).

### G1.5 SURVIVAL AND BREAK-UP DURING RE-ENTRY

During re-entry aerodynamic braking and aerothermal heating can lead to:

- disintegration—the object breaks up into multiple fragments or loses external parts (e.g., solar panels, thermal shielding);
- mass loss—heating leads to melting and ablative mass loss; or
- total demise—the object burns up completely and there is no impact.

The heat flux due to aerothermal heating depends on the flow conditions (free-molecular, transitional, continuum supersonic, and continuum subsonic), hence on object velocity and atmospheric density. How to estimate this flux is beyond the scope of this informative annex. The heat flux into the object leads to an increase in internal temperature, melting (once temperature is high enough), and radiating heat away from the object. Assuming a uniform composition leads to the following equations:

$$ Q = \rho C_p V \frac{dT}{dt} + \epsilon \sigma A T^4, \quad T < T_m $$ 

$$ Q = - \rho h_m \frac{dV}{dt} + \epsilon \sigma A T^4, \quad T = T_m $$ 

Until the object’s surface reaches the melting temperature, the heat flux in is equal to the heat absorbed by increasing internal temperature and the heat radiated away. Once the surface reaches the melting temperature, the heat flux is equal to the heat lost by mass ablation and the heat radiated away. This leads to two ways in which an object can survive re-entry: absorb the re-entry heat without reaching the melting temperature or radiate the heat away efficiently. If the analysis is restricted to solid metallic spheres, very small objects (smaller than $\sim 1$ cm), can do the latter, while very large objects (larger than $\sim 10$ cm) can do the former. Objects in between (e.g., most nuts and bolts used in spacecraft assembly) will typically, but not always, suffer total demise.

Simulating disintegration and mass loss for complex objects is a multi-disciplinary process, since spacecraft have complex shapes and contain a multitude of materials. The SCARAB tool, for example, can simulate disintegration and mass loss by dividing the spacecraft into basic elements, each with consistent material properties and each element into multiple voxels (volume elements). The simulation accounts for the forces acting on each element/voxel (as in G1.4) and computes heat fluxes, temperatures, and normal and shear stresses for a thermal and structural analysis. This determines break-ups (when stresses exceed the ultimate tensile or shear stress in the structure) and ablative mass loss.

Exchanging the information needed to accurately replicate these types of simulations is beyond the scope of the RDM. Furthermore, the tools needed to replicate the results are not widely available, which restricts usefulness. Break-ups during re-entry tend to happen at around 80 km altitude, when the combined aerodynamic and thermal loads are highest, but without knowing the structure of the object nothing more can be said.

### G2 ATMOSPHERIC RE-ENTRY AND GROUND IMPACT DATA KEYWORDS

`ORBIT_LIFETIME`: The remaining time in orbit of the object, in days, from the EPOCH_TZERO keyword in the metadata. This is intended to be a coarse estimation, useful for long- and medium-term (more than a few days) re-entry prediction. The orbit lifetime is considered to end when the altitude permanently drops below the `REENTRY_ALTITUDE`. This keyword has two roles:

- Specify whether long/medium or short-term re-entry simulations were used for the object (e.g., a high value means a long-term simulation was performed); and
- Give an estimate of the remaining orbit lifetime, especially for long-term predictions, with appropriate accuracy.

`REENTRY_ALTITUDE`: The altitude at which the orbit lifetime ends and at which re-entry is defined. Depending on the particular re-entry event, different values can be specified, e.g., 150 km for 'object captured by the Earth’s atmosphere', 70 km for ‘object likely to break-up at this altitude', or simply the altitude limit at which the re-entry simulation stops.

`ORBIT_LIFETIME_WINDOW_START` and `ORBIT_LIFETIME_WINDOW_END`: The uncertainty time window associated with the `ORBIT_LIFETIME` keyword and meant for long-term re-entry predictions. $\pm 20\%$ from the orbit lifetime provides an acceptable empirical estimate for nominal conditions, accounting for uncertainties in the ballistic coefficient, atmospheric density, and orbit parameters.

MISSING IMAGE (Figure G-1: Example of Medium-Term Re-entry Prediction Data Use)

`NOMINAL_REENTRY_EPOCH`: Predicted re-entry epoch, providing much greater precision for short-term (last couple of days before re-entry, when altitude is below 200 km) re-entry predictions.

`REENTRY_WINDOW_START` and `REENTRY_WINDOW_END`: Allow the RDM originator to specify a re-entry time window, which does not have to be symmetric around the nominal re-entry epoch. They can account for uncertainties in the modeling (atmospheric density, drag coefficient, attitude influence) or for under/over performance of thrusting maneuvers in a controlled re-entry.

`ORBIT_LIFETIME_CONFIDENCE_LEVEL`: The (estimated) confidence level of the re-entry occurring between `ORBIT_LIFETIME_WINDOW_START` and `ORBIT_LIFETIME_WINDOW_END` or between `REENTRY_WINDOW_START` and `REENTRY_WINDOW_END`.

`PROBABILITY_OF_IMPACT`: Probability of the object or any resulting fragments impacting the Earth (land or sea).

`PROBABILITY_OF_BURN_UP`: Total demise probability for the object and/or any resulting fragments.

`PROBABILITY_OF_BREAK_UP`: Probability of fragments being generated during re-entry, both 'shedding' of parts (e.g., solar panels, heat shields) and fragmentation of the main structure.

`PROBABILITY_OF_LAND_IMPACT`: Probability of the object or any resulting fragments impacting solid land. This is the first step in assessing any casualty probability, in conjunction with population density mapping.

`NOMINAL_IMPACT_EPOCH`: The nominal (predicted) epoch at which the object hits the surface of the Earth (either solid land or ocean).

`IMPACT_WINDOW_START` and `IMPACT_WINDOW_END`: The same as `REENTRY_WINDOW_START` and `_END`, but for the impact epoch.

`IMPACT_REF_FRAME`, `NOMINAL_IMPACT_LON`, `_LAT`, `_ALT`: Allow the specification of the nominal (predicted) impact location in geodetic coordinates.

`IMPACT_n_CONFIDENCE`, `IMPACT_n_START_LON`, `IMPACT_n_START_LAT`, `IMPACT_n_STOP_LON`, `IMPACT_n_STOP_LAT`, `IMPACT_n_CROSS_TRACK`: Uncertainties in the ground impact location are expressed in the along-track and cross-track directions, which means the (predicted) ground track needs to be known. Since the probability distribution is skewed, the method chosen to deal with these two issues is to:

- give the coordinates corresponding to the maximum probability, with the `NOMINAL_IMPACT_LON` and `_LAT` keywords;
- define up to three confidence intervals;
- give the coordinates of the two points on the ground track corresponding to the start and end of each of the (up to) three confidence intervals.

As long as at least three points are provided the ground track can be estimated as one or more segments of a great circle. The cross-track confidence intervals (in km) can be used to determine the swaths of impact probability, which can then be used together with population density mapping to determine the casualty probability.

The ground impact location and uncertainties are supposed to cover all eventual re-entry fragments (fragments generated during re-entry, not before). For example, if the re-entry data is obtained through Monte Carlo simulations, `NOMINAL_IMPACT_LON`, `_LAT` would correspond to the bin with the most impacts, the first swath would cover `IMPACT_1_CONFIDENCE` of the impacts, and so on. If analytical formulae are used for the 1-sigma confidence interval, `IMPACT_1_CONFIDENCE` should be 47 percent (for rectangular, rather than elliptical areas), the `IMPACT_1_START/STOP_LAT/LON` values can be determined from the formulae and the ground track, and `IMPACT_1_CROSS_TRACK` from the formula used.

### G3 ORBIT DETERMINATION KEYWORDS

To promote consistency and ease implementation, the orbit determination keywords are the same as in the Conjunction Data Message (reference [H6]). These keywords are present to give the consumer of the message an idea of the quality of the data provided and when the re-entering object was last observed.

`Observation`: Unique measurement of a satellite’s location from a single sensor at a single time (e.g., azimuth from a single sensor at a single time).

`TIME_LASTOB_START` and `TIME_LASTOB_END`: The start and end of a time interval that contains the time of the last accepted observation. For an exact time, the time interval is of zero duration (i.e., `TIME_LASTOB_START = TIME_LASTOB_END`).

`RECOMMENDED_OD_SPAN`: How many days of observations were recommended for the OD of the object.

`ACTUAL_OD_SPAN`: The actual time span used for the OD of the object based on the observations available and the `RECOMMENDED_OD_SPAN`.

`OBS_AVAILABLE`: The number of observations, for the recommended time span, that were available for the OD.

`OBS_USED`: The number of observations, for the recommended time span, that were accepted for the OD.

`Sensor Track`: A set of at least three observations for the same object, observed by the same sensor, where each observation is within a specified number of minutes (which is dependent on the orbit regime of the object) of the other observations in the track.

`TRACKS_AVAILABLE`: The number of sensor tracks, for the recommended time span, that were available for the OD. This provides information about the independence of the observational data used in the OD.

`TRACKS_USED`: The number of sensor tracks, for the recommended time span, that were accepted for the OD. This provides information about the independence of the observational data used in the OD.

`WEIGHTED_RMS`:

$$ \text{Weighted RMS} = \sqrt{\frac{1}{N} \sum (y_i - y_{i, \text{estimated}})^2 } $$ 

Where:

- $y_i$ is the $i$-th observation;
- $y_{i, \text{estimated}}$ is the estimated value (from the resulting orbit) of $y_i$;
- $w_i$ is the weighting of the observation; and
- $N$ is the total number of observations.

This is a value that can generally identify the quality of the most recent orbit update, and is used by the analyst in evaluating the OD process.
# ANNEX H

## INFORMATIVE REFERENCES

(INFORMATIVE)

[H1] Navigation Data—Definitions and Conventions. Issue 3. Report Concerning Space Data System Standards (Green Book), CCSDS 500.0-G-3. Washington, D.C.: CCSDS, May 2010.
[H2] Heiner Klinkrad. Space Debris: Models and Risk Analysis. Springer Praxis Books. Berlin, Heidelberg, New York: Springer, 2006.
[H3] Benjamin Bastida Virgili, et al. “GOCE Re-entry Campaign.” In Proceedings of the 5th GOCE User Workshop (25–28 November 2014, UNESCO, Paris, France). Oakville, Ontario: ESA Communications, 2015.
[H4] Benjamin Bastida Virgili, et al. “Practicalities of Re-entry Predictions—The VEGA-01 AVUM Case.” In Proceedings of the 7th European Conference on Space Debris (18–21 April 2017, Darmstadt, Germany). Edited by T. Flohrer and F. Schmitz. Darmstadt, Germany: ESA Space Debris Office, 2017.
[H5] Space Systems—Estimation of Orbit Lifetime. 2nd ed. International Standard, ISO 27852:2016. Geneva: ISO, 2016.
[H6] Conjunction Data Message. Issue 1. Recommendation for Space Data System Standards (Blue Book), CCSDS 508.0-B-1. Washington, D.C.: CCSDS, June 2013.
