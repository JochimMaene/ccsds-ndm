.. title:: TRACKING DATA MESSAGE

.. rubric:: RECOMMENDED STANDARD

.. centered:: CCSDS 503.0-B-2

.. centered:: BLUE BOOK
.. centered:: June 2020

.. centered::
   Note:
   This current
   issue includes
   all updates through
   Technical Corrigendum 1,
   dated October 2021

AUTHORITY
=========

.. table::
   :align: center

   +----------+--------------------------------+
   | Issue:   | Recommended Standard, Issue 2  |
   +----------+--------------------------------+
   | Date:    | June 2020                      |
   +----------+--------------------------------+
   | Location:| Washington, DC, USA            |
   +----------+--------------------------------+

This document has been approved for publication by the Management Council of the
Consultative Committee for Space Data Systems (CCSDS) and represents the consensus
technical agreement of the participating CCSDS Member Agencies. The procedure for
review and authorization of CCSDS documents is detailed in *Organization and Processes for
the Consultative Committee for Space Data Systems* (CCSDS A02.1-Y-4), and the record of
Agency participation in the authorization of this document can be obtained from the CCSDS
Secretariat at the e-mail address below.

This document is published and maintained by:

    CCSDS Secretariat
    National Aeronautics and Space Administration
    Washington, DC, USA
    Email: secretariat@mailman.ccsds.org

STATEMENT OF INTENT
===================

The Consultative Committee for Space Data Systems (CCSDS) is an organization officially
established by the management of its members. The Committee meets periodically to address
data systems problems that are common to all participants, and to formulate sound technical
solutions to these problems. Inasmuch as participation in the CCSDS is completely
voluntary, the results of Committee actions are termed **Recommended Standards** and are
not considered binding on any Agency.

This **Recommended Standard** is issued by, and represents the consensus of, the CCSDS
members. Endorsement of this **Recommendation** is entirely voluntary. Endorsement,
however, indicates the following understandings:

-  Whenever a member establishes a CCSDS-related **standard**, this **standard** will be in
   accord with the relevant **Recommended Standard**. Establishing such a **standard**
   does not preclude other provisions which a member may develop.

-  Whenever a member establishes a CCSDS-related **standard**, that member will
   provide other CCSDS members with the following information:

   -- The **standard** itself.

   -- The anticipated date of initial operational capability.

   -- The anticipated duration of operational service.

-  Specific service arrangements shall be made via memoranda of agreement. Neither
   this **Recommended Standard** nor any ensuing **standard** is a substitute for a
   memorandum of agreement.

No later than five years from its date of issuance, this **Recommended Standard** will be
reviewed by the CCSDS to determine whether it should: (1) remain in effect without change;
(2) be changed to reflect the impact of new technologies, new requirements, or new
directions; or (3) be retired or canceled.

In those instances when a new version of a **Recommended Standard** is issued, existing
CCSDS-related member standards and implementations are not negated or deemed to be
non-CCSDS compatible. It is the responsibility of each member to determine when such
standards or implementations are to be modified. Each member is, however, strongly
encouraged to direct planning for its new standards and implementations towards the later
version of the Recommended Standard.

FOREWORD
========

This document is a Recommended Standard for tracking data messages and has been
prepared by the Consultative Committee for Space Data Systems (CCSDS). The tracking
data message described in this Recommended Standard is the baseline concept for tracking
data interchange applications that are cross-supported between Agencies of the CCSDS.

This Recommended Standard establishes a common framework and provides a common
basis for the format of tracking data exchange between space agencies. It allows
implementing organizations within each Agency to proceed coherently with the development
of compatible derived standards for the flight and ground systems that are within their
cognizance. Derived Agency standards may implement only a subset of the optional features
allowed by the Recommended Standard and may incorporate features not addressed by this
Recommended Standard.

Attention is drawn to the possibility that some of the elements of this document may be the
subject of patent rights. CCSDS has processes for identifying patent issues and for securing
from the patent holder agreement that all licensing policies are reasonable and non-
discriminatory. However, CCSDS does not have a patent law staff, and CCSDS shall not be
held responsible for identifying any or all such patent rights.

Through the process of normal evolution, it is expected that expansion, deletion, or
modification of this document may occur. This Recommended Standard is therefore subject
to CCSDS document management and change control procedures, which are defined in
*Organization and Processes for the Consultative Committee for Space Data Systems*
(CCSDS A02.1-Y-4). Current versions of CCSDS documents are maintained at the CCSDS
Web site:

http://www.ccsds.org/

Questions relating to the contents or status of this document should be sent to the CCSDS
Secretariat at the email address indicated on page i.

At time of publication, the active Member and Observer Agencies of the CCSDS were:

Member Agencies
---------------

-  Agenzia Spaziale Italiana (ASI)/Italy.
-  Canadian Space Agency (CSA)/Canada.
-  Centre National d'Etudes Spatiales (CNES)/France.
-  China National Space Administration (CNSA)/People's Republic of China.
-  Deutsches Zentrum für Luft- und Raumfahrt (DLR)/Germany.
-  European Space Agency (ESA)/Europe.
-  Federal Space Agency (FSA)/Russian Federation.
-  Instituto Nacional de Pesquisas Espaciais (INPE)/Brazil.
-  Japan Aerospace Exploration Agency (JAXA)/Japan.
-  National Aeronautics and Space Administration (NASA)/USA.
-  UK Space Agency/United Kingdom.

Observer Agencies
-----------------

-  Austrian Space Agency (ASA)/Austria.
-  Belgian Federal Science Policy Office (BFSPO)/Belgium.
-  Central Research Institute of Machine Building (TsNIIMash)/Russian Federation.
-  China Satellite Launch and Tracking Control General, Beijing Institute of Tracking and
   Telecommunications Technology (CLTC/BITTT)/China.
-  Chinese Academy of Sciences (CAS)/China.
-  China Academy of Space Technology (CAST)/China.
-  Commonwealth Scientific and Industrial Research Organization (CSIRO)/Australia.
-  Danish National Space Center (DNSC)/Denmark.
-  Departamento de Ciência e Tecnologia Aeroespacial (DCTA)/Brazil.
-  Electronics and Telecommunications Research Institute (ETRI)/Korea.
-  European Organization for the Exploitation of Meteorological Satellites (EUMETSAT)/Europe.
-  European Telecommunications Satellite Organization (EUTELSAT)/Europe.
-  Geo-Informatics and Space Technology Development Agency (GISTDA)/Thailand.
-  Hellenic National Space Committee (HNSC)/Greece.
-  Hellenic Space Agency (HSA)/Greece.
-  Indian Space Research Organization (ISRO)/India.
-  Institute of Space Research (IKI)/Russian Federation.
-  Korea Aerospace Research Institute (KARI)/Korea.
-  Ministry of Communications (MOC)/Israel.
-  Mohammed Bin Rashid Space Centre (MBRSC)/United Arab Emirates.
-  National Institute of Information and Communications Technology (NICT)/Japan.
-  National Oceanic and Atmospheric Administration (NOAA)/USA.
-  National Space Agency of the Republic of Kazakhstan (NSARK)/Kazakhstan.
-  National Space Organization (NSPO)/Chinese Taipei.
-  Naval Center for Space Technology (NCST)/USA.
-  Research Institute for Particle & Nuclear Physics (KFKI)/Hungary.
-  Scientific and Technological Research Council of Turkey (TUBITAK)/Turkey.
-  South African National Space Agency (SANSA)/Republic of South Africa.
-  Space and Upper Atmosphere Research Commission (SUPARCO)/Pakistan.
-  Swedish Space Corporation (SSC)/Sweden.
-  Swiss Space Office (SSO)/Switzerland.
-  United States Geological Survey (USGS)/USA.

DOCUMENT CONTROL
================

.. list-table::
   :widths: 20 30 15 35
   :header-rows: 1

   * - Document
     - Title
     - Date
     - Status
   * - CCSDS 503.0-B-1
     - Tracking Data Message, Recommended Standard, Issue 1
     - November 2007
     - Original issue, superseded
   * - CCSDS 503.0-B-2
     - Tracking Data Message, Recommended Standard, Issue 2
     - June 2020
     - Current issue: Substantive changes from the original issue are enumerated in 1.2.6.
   * - CCSDS 503.0-B-2 Cor. 1
     - Technical Corrigendum 1
     - October 2021
     - Adds specifications and examples for messages with elements qualified with respect to a namespace.
.. _introduction:

1 INTRODUCTION
==============

.. _purpose:

1.1 PURPOSE
-----------

1.1.1 This Tracking Data Message (TDM) Recommended Standard specifies a standard
message format for use in exchanging spacecraft tracking data between space agencies. Such
exchanges are used for distributing tracking data output from routine interagency cross-
supports in which spacecraft missions managed by one agency are tracked from a tracking
station managed by a second agency. The standardization of tracking data formats facilitates
space agency allocation of tracking sessions to alternate tracking resources.

1.1.2 This document includes requirements and criteria that the message format has been
designed to meet. For exchanges where these requirements do not capture the needs of the
participating Agencies another mechanism may be selected.

.. _scope_and_applicability:

1.2 SCOPE AND APPLICABILITY
---------------------------

1.2.1 This Recommended Standard contains the specification for a Tracking Data Message
designed for applications involving tracking data interchange between space data systems.
Tracking data includes data types such as Doppler, transmit/received frequencies, range,
angles, Delta-DOR, DORIS, PRARE, media correction, weather, etc. The rationale behind the
design of the message is described in annex G and may help the application engineer construct
a suitable message. It is acknowledged that this version of the Recommended Standard may
not apply to every single tracking session or data type; however, it is desired to focus on
covering most common tracking scenarios, and to expand the coverage in future versions as
necessary.

1.2.2 This message is suited to inter-agency exchanges that involve automated interaction.
The attributes of a TDM make it primarily suitable for use in computer-to-computer
communication because of the large amount of data typically present. The TDM is generally
intended to be used in conjunction with an Interface Control Document (ICD) written jointly
by the service provider and customer agency. The ICD outlines TDM options that have been
exercised in the specific implementation.

1.2.3 Definition of the accuracy pertaining to any particular TDM is outside the scope of
this Recommended Standard and should be specified via an ICD between data exchange
participants.

1.2.4 This Recommended Standard is applicable only to the message format and content, but
not to its transmission. The method of transmitting the message between exchange partners is
beyond the scope of this document and should be specified in the ICD. Message transmission
could be based on a CCSDS data transfer protocol, file based transfer protocol such as Secure
File Transfer Protocol (SFTP), stream-oriented media, or other secure transmission mechanism.
In general, the transmission mechanism must not place constraints on the technical data content
of a TDM.

1.2.5 There are some specific exclusions to the TDM, as listed below:

a) Satellite Laser Ranging (SLR) ‘Fullrate’ and ‘Normal Points’ format (sometimes
   referred to as ‘Quicklook’), which are already transferred via a standardized format
   documented at https://ilrs.cddis.eosdis.nasa.gov/; however, such data could
   conceivably be transferred via TDM with a ‘RANGE’ keyword (see 3.5.2.7);

b) exchanges of raw Global Navigation Satellite System (GNSS) data, which is
   standardized via the RINEX format (http://www.igs.org);

c) Global Positioning Satellite (GPS) navigation point solutions, which are standardized
   via the SP3 format (https://www.ngs.noaa.gov/orbits/); [1]_

d) optical data from navigation cameras (pixel-based, row-column, etc.);

e) LIDAR data (which may include a laser range finder); however, such data could
   conceivably be transferred via TDM with a ‘RANGE’ keyword (see 3.5.2.7); and

f) altimeter data; however, such data could conceivably be transferred via TDM with a
   ‘RANGE’ keyword (see 3.5.2.7).

1.2.6 Changes in Version 2 of the Tracking Data Message include the following:

1.2.6.1 Description of the message format based on the use of eXtensible Markup
Language (XML) is now detailed in section 5 of this document.

1.2.6.2 References, including inline references to various Web sites, have been updated as
applicable.

1.2.6.3 The labeling of several annexes has changed, primarily in order to respond to
changing CCSDS document requirements, for example, the Implementation Conformance
Statement (ICS) (annex A) was added, causing several prior annex labels to shift; and the
Security section was converted from a main document section (5) to an annex (annex C).

1.2.6.4 The Space Assigned Numbers Authority (SANA) Registry is now a source of
values for some keywords, as noted in the relevant tables.

1.2.6.5 The word ‘obligatory’ is no longer used; ‘mandatory’ is substituted based on the
requirements of the ICS.

1.2.6.6 There are several new Data Section keywords added based on
suggestions/recommendations by TDM version 1 users. These include transmit/receive
phase; optical magnitude and radar cross section based on space situational awareness
applications; and Doppler counts. For each of these new data types there are one or more
related Metadata Section keywords.

.. [1] It has been suggested that the statement regarding navigation solutions’ being standardized by SP3 is not
   correct, because SP3 prescribes equidistant data (ephemerides), which are in general not provided by each
   GPS/GNSS receiver. It was proposed that the navigation solution data (epoch, x, y, z, vx, vy, vz) should be
   provided in the TDM, with the velocities as optional values. However, this would require major changes to the
   TDM that are contrary to its intended purpose. As an alternative, the CCSDS Orbit Data Messages Orbit
   Ephemeris Message (OEM) (see reference [4]) could be used to convey the navigation solution if all position
   and velocity components are transferred. The OEM is already set up to convey all the required values and can
   be used to convey orbit reconstructions as well as orbit predictions.

.. _conventions_and_definitions:

1.3 CONVENTIONS AND DEFINITIONS
-------------------------------

1.3.1 GENERAL
^^^^^^^^^^^^^

Conventions and definitions of navigation concepts such as reference frames, time systems,
etc., are provided in reference [F7]. (Also see SANA Registries specified in annex C.)

1.3.2 NORMATIVE TEXT
^^^^^^^^^^^^^^^^^^^^

The following conventions apply for the normative specifications in this Recommended
Standard:

a) the words ‘shall’ and ‘must’ imply a binding and verifiable specification;

b) the word ‘should’ implies an optional, but desirable, specification;

c) the word ‘may’ implies an optional specification;

d) the words ‘is’, ‘are’, and ‘will’ imply statements of fact.

NOTE – These conventions do not imply constraints on diction in text that is clearly
informative in nature.

1.3.3 INFORMATIVE TEXT
^^^^^^^^^^^^^^^^^^^^^^

In the normative sections of this document (sections 3–5), informative text is set off from the
normative specifications either in notes or under one of the following subsection headings:

– Overview;

– Background;

– Rationale;

– Discussion.

1.3.4 DEFINITIONS
^^^^^^^^^^^^^^^^^

1.3.4.1 Terms
"""""""""""""

**participant:** An entity that has the ability to acquire, broadcast, or reflect navigation
messages and/or electromagnetic frequencies, for example, a spacecraft, a quasar, a tracking
station, a tracking instrument, or an agency center, as discussed in reference [F7]. Thus there
may exist Tracking Data Messages for which there is no applicable spacecraft.

**agency:** An exchange partner.

NOTE – This usage results from the history of the CCSDS, which was formed as a
coalition of the world’s space agencies. Over time, as the space industry and the
CCSDS have evolved, there is a wider group of organizations (e.g., military,
commercial) that could utilize CCSDS standards. In this document, the term
‘agency’ is meant to encompass any and all of these exchange partners.

**n/a, N/A:** Not applicable or not available.

1.3.4.2 Unit Notations
""""""""""""""""""""""

The following conventions for unit notations apply throughout this Recommended Standard.
Insofar as possible, an effort has been made to use units that are part of the International
System of Units (SI Units); units are either SI base units, SI derived units, or units outside
the SI that are accepted for use with the SI (see reference [7]). There are a small number of
specific cases where units that are more widely used in the navigation community are
specified, but every effort has been made to minimize these departures from the SI.

%: percent
dBHz: decibels referenced to one Hz
dBW: decibels referenced to one Watt
deg: degrees of plane angle
hPa: hectoPascal
Hz: Hertz
K: Kelvin
km: kilometers
m: meters
m**2: square meters
RU: range units
s: seconds
TECU: Total Electron Count Units

.. _structure_of_this_document:

1.4 STRUCTURE OF THIS DOCUMENT
------------------------------

1.4.1 Section 2 provides a brief overview of the CCSDS-recommended Tracking Data
Message (TDM).

1.4.2 Section 3 provides details about the structure and content of the TDM.

1.4.3 Section 4 provides details about the syntax used in the TDM in Keyword-Value
Notation (KVN) format.

1.4.4 Section 5 discusses a CCSDS XML schema for the TDM and how to create an XML
instantiation of a TDM.

1.4.5 Annex A provides an ICS for the TDM.

1.4.6 Annex B discusses values for selected TDM Metadata Section keywords.

1.4.7 Annex C discusses security, SANA, and patent considerations with respect to the
TDM.

1.4.8 Annex D lists a number of items that should be covered in interagency ICDs prior to
exchanging TDMs on a regular basis. There are several statements throughout the document
that refer to the desirability or necessity of such a document; this annex consolidates all the
suggested ICD items in a single list.

1.4.9 Annex E shows how various tracking scenarios can be accommodated using the
TDM, via several examples.

1.4.10 Annex F contains a list of informative references.

1.4.11 Annex G lists a set of requirements and desirable characteristics that were taken into
consideration in the design of the TDM.

1.4.12 Annex H is a list of abbreviations and acronyms applicable to the TDM.

1.4.13 Annex I provides a TDM Summary Sheet, or ‘Quick Reference’.

.. _references:

1.5 REFERENCES
--------------

The following publications contain provisions which, through reference in this text,
constitute provisions of this document. At the time of publication, the editions indicated
were valid. All publications are subject to revision, and users of this document are
encouraged to investigate the possibility of applying the most recent editions of the
publications indicated below. The CCSDS Secretariat maintains a register of currently valid
CCSDS publications.

[1] “Outer Space Objects Index.” United Nations Office for Outer Space Affairs.
    http://www.unoosa.org/oosa/osoindex/index.jspx.

[2] Information Technology—8-Bit Single-Byte Coded Graphic Character Sets—Part 1:
    Latin Alphabet No. 1. International Standard, ISO/IEC 8859-1:1998. Geneva: ISO,
    1998.

[3] Time Code Formats. Issue 4. Recommendation for Space Data System Standards (Blue
    Book), CCSDS 301.0-B-4. Washington, D.C.: CCSDS, November 2010.

[4] Orbit Data Messages. Issue 2. Recommendation for Space Data System Standards
    (Blue Book), CCSDS 502.0-B-2. Washington, D.C.: CCSDS, November 2009.

[5] Paul V. Biron and Ashok Malhotra, eds. XML Schema Part 2: Datatypes. 2nd ed. W3C
    Recommendation. W3C, October 2004. http://www.w3.org/TR/2004/REC-xmlschema-2-20041028/

[6] IEEE Standard for Floating-Point Arithmetic. 2nd ed. IEEE Std. 754-2008. New York:
    IEEE, 2008.

[7] The International System of Units (SI). 9th ed. Sèvres, France: BIPM, 2009.

[8] Attitude Data Messages. Issue 1. Recommendation for Space Data System Standards
    (Blue Book), CCSDS 504.0-B-1. Washington, D.C.: CCSDS, May 2008.

[9] XML Specification for Navigation Data Messages. Issue 2. Recommendation for Space
    Data System Standards (Blue Book), CCSDS 505.0-B-2. Washington, D.C.: CCSDS,
    May 2021.

[10] Henry S. Thompson, et al., eds. “XML Schema Part 1: Structures.” W3C
     Recommendation. 2nd ed., 28 October 2004. The World Wide Web Consortium
     (W3C). http://www.w3.org/TR/2004/REC-xmlschema-1-20041028/.

[11] “Organizations.” Space Assigned Numbers Authority.
     https://sanaregistry.org/r/organizations.

[12] “Time Systems.” Space Assigned Numbers Authority.
     https://sanaregistry.org/r/time_systems.

[13] “Celestial Body Reference Frames.” Space Assigned Numbers Authority.
     https://sanaregistry.org/r/celestial_body_reference_frames.

NOTE – Informative references are provided in annex F.
.. _overview:

2 OVERVIEW
==========

.. _overview_general:

2.1 GENERAL
-----------

This section provides a high-level overview of the CCSDS recommended Tracking Data
Message, a message format designed to facilitate standardized exchange of spacecraft
tracking data between space agencies.

.. _tdm_basic_content:

2.2 THE TRACKING DATA MESSAGE (TDM) BASIC CONTENT
-------------------------------------------------

2.2.1 The TDM is realized as a sequence of ASCII text lines (reference [2]), which may be
in either a file format or a real-time stream. The content is separated into three basic types of
computer data structure as described in section 3. The TDM architecture takes into account
that some aspects of tracking data change on a measurement-by-measurement basis (data);
some aspects change less frequently, but perhaps several times per track (metadata); and
other aspects change only rarely, for example, once per track or perhaps less frequently
(header). The TDM makes it possible to convey a variety of tracking data used in the orbit
determination process in a single data message (e.g., standard Doppler and range
radiometrics in a variety of tracking modes, transmit/receive frequencies, VLBI data, antenna
pointing angles, etc.). To aid in precision trajectory modeling, additional ancillary
information may be included within a TDM if it is desired and/or available (e.g., media
corrections, meteorological data, clock data, and other ancillary data). Facilities for
documenting comments are provided.

2.2.2 The Tracking Data Message in this version of the Recommended Standard is ASCII
text formatted. While binary-based tracking data message formats are computer efficient and
minimize overhead during data transfer, there are ground-segment applications for which an
ASCII character-based message is more appropriate. For example, ASCII format character-
based tracking data representations are useful in transferring data between heterogeneous
computing systems, because the ASCII character set is nearly universally used and is
interpretable by all popular systems. In addition, direct human-readable dumps of text to
displays, emails, documents, or printers are possible without preprocessing. The penalty for
this convenience is some measure of inefficiency (based on early tests, such penalty would
be greatly reduced if the data is compressed for transmission).

2.2.3 The ASCII text in a TDM can be exchanged in either of two formats: a KVN format
or an XML format. The KVN formatted TDM and XML formatted TDM are described in
this document. Further information on XML is detailed in an integrated XML schema
document for all Navigation Data Messages (reference [9]). It is recommended that
exchange participants specify in the ICD which TDM ASCII format will be exchanged, the
KVN or the XML format.

2.2.4 Normally a TDM will contain tracking data for a single spacecraft participant, unless
the tracking session is spacecraft-to-spacecraft in nature. If a tracking operation involves
information from multiple spacecraft participants tracked from the ground, the data may be
included in a single TDM by using multiple segments (see 3.1); or multiple TDMs may be
used, one per spacecraft participant.

2.2.5 For a given spacecraft participant, multiple tracking data messages could be provided
in a message exchange session to achieve the tracking data requirements of the participating
agencies (e.g., launch supports with periodically delivered TDMs, or other critical events
such as maneuvers, encounters, etc.).

2.2.6 Provisions for the frequency of exchange and special types of exchanges should be
specified in an ICD.
.. _tdm_structure_and_content:

3 TRACKING DATA MESSAGE STRUCTURE AND CONTENT
=============================================

.. _structure_general:

3.1 GENERAL
-----------

3.1.1 The TDM shall consist of digital data represented as ASCII text lines (see reference [2])
in KVN format (see section 4) or XML format (see section 5). The lines constituting a TDM
shall be represented as a combination of:

a) a Header (see 3.2);

b) a Metadata Section (data about data) (see 3.3); and

c) a Data Section (tracking data represented as ‘Tracking Data Records’) (see 3.4, 3.5).

Optional comments may appear in specified locations in the Header, Metadata, and Data
Sections (see 4.5).

3.1.2 Taken together, the Metadata Section and its associated Data Section shall be called a
TDM Segment.

3.1.3 Each TDM shall have a Header and a Body. The TDM Body shall consist of one or
more TDM Segments. There shall be no limit to the number of Segments in a given TDM
Body, beyond practical constraints, as shown in table 3-1. Each Segment shall consist of a
Metadata Section and a Data Section that consists of a minimum of one Tracking Data
Record. Therefore the overall structure of the TDM shall be:

– TDM = Header + Body;

– Body = Segment [+ Segment + … + Segment];

– Segment = Metadata Section + Data Section;

– Data Section = Tracking Data Record (TDR) [ + TDR + TDR … + TDR].

.. _table_3-1:

.. list-table:: Table 3-1: TDM Structure
   :widths: 20 20 30 30
   :header-rows: 1

   * - Item
     -
     -
     - Mandatory?
   * - Header
     -
     -
     - Yes
   * - Body
     - Segment 1
     - Metadata 1
     - Yes
   * -
     -
     - Data 1
     -
   * -
     - Segment 2
     - Metadata 2
     - No
   * -
     -
     - Data 2
     -
   * -
     - .
     - .
     - .
   * -
     - .
     - .
     - .
   * -
     - .
     - .
     - .
   * -
     - Segment n
     - Metadata n
     - No
   * -
     -
     - Data n
     -

3.1.4 The TDM shall consist of tracking data for one or more tracking participants (see
1.3.4.1) at multiple epochs contained within a specified time range. Generally, but not
necessarily, the time range of a TDM may correspond to a ‘tracking pass’.

3.1.5 It shall be possible to exchange a TDM either as a real-time stream or as a file.

3.1.6 The TDM file naming scheme should be agreed to on a case-by-case basis between
the participating agencies, and should be specified in an ICD.

3.1.7 The method of exchanging TDMs shall be decided on a case-by-case basis by the
participating agencies and should be documented in an ICD. The exchange method shall not
constrain the tracking data content.

.. _tdm_header:

3.2 TDM HEADER
--------------

3.2.1 The TDM shall include a Header that consists of information that identifies the basic
parameters of the message. The first Header line must be the first non-blank line in the
message.

3.2.2 A description of TDM Header items and values is provided in table 3-2, which
specifies for each item:

– the keyword to be used;

– a short description of the item;

– examples of allowed values; and

– whether the item is mandatory or optional.

3.2.3 Only those keywords shown in table 3-2 shall be used in a TDM Header. The order of
occurrence of the mandatory and optional KVN assignments shall be fixed as shown in
table 3-2.

.. _table_3-2:

.. list-table:: Table 3-2: TDM Header
   :widths: 20 40 25 15
   :header-rows: 1

   * - Keyword
     - Description
     - Examples
     - Mandatory
   * - CCSDS_TDM_VERS
     - Format version in the form of ‘x.y’, where ‘y’ shall be incremented for corrections and minor changes, and ‘x’ shall be incremented for major changes.
     - 0.12 (for testing)
       1.0 (2007 version)
       2.0 (this version)
     - Yes
   * - COMMENT
     - (See 4.5.)
     - COMMENT This is a comment
     - No
   * - CREATION_DATE
     - Data creation date/time in UTC. (For format specification, see 4.3.9.)
     - 2001-11-06T11:17:33
       2002-204T15:56:23.4
       2006-001T00:00:00Z
     - Yes
   * - ORIGINATOR
     - Creating agency. Value should be an entry from the ‘Abbreviation’ column in the SANA Organizations Registry, https://sanaregistry.org/r/organizations/organizations.html (reference [11]).
     - CNES, ESA, GSFC, DLR, JPL, JAXA, etc.
     - Yes
   * - MESSAGE_ID
     - ID that uniquely identifies a message from a given originator. The format and content of the message identifier value are at the discretion of the originator.
     - 201113719185
     - No

3.2.4 Each line in the TDM Header, with the exception of COMMENTs, shall have the
following generic format:

   keyword = value

3.2.5 The TDM Header shall provide a CCSDS Tracking Data Message version number
that identifies the format version; this is included to anticipate future changes and to provide
the ability to extend the standard with no disruption to existing users. The version keyword is
CCSDS_TDM_VERS and the value shall have the form of x.y where y is incremented for
corrections and minor changes, and x is incremented for major changes. Versions x.0, where
x≥1, shall be reserved for versions accepted by the CCSDS as an official Recommended
Standard (‘Blue Book’). Interagency testing of TDMs shall be conducted using version
numbers less than 1.0 (e.g., ‘0.y’). Specific TDM versions that will be exchanged between
agencies should be documented via the ICD.

3.2.6 The TDM Header shall include the CREATION_DATE keyword with the value set to
the Coordinated Universal Time (UTC) when the data was created (file creation time if in file
format, or first data point in stream), as specified in 4.3.9.

.. _tdm_metadata:

3.3 TDM METADATA
----------------

3.3.1 GENERAL
^^^^^^^^^^^^^

3.3.1.1 The TDM shall include at least one Metadata Section that contains configuration
details (metadata) applicable to the Data Section in the same TDM Segment. The
information in the Metadata Section aligns with the tracking data to provide descriptive
information (typically, the metadata is the type of information that does not change
frequently during a tracking session).

3.3.1.2 Each line in the TDM Metadata Section, with the exception of COMMENTs, shall
have the following generic format:

   keyword = value

3.3.1.3 A single TDM Metadata Section shall precede each Data Section.

3.3.1.4 When there are changes in the values assigned to any of the keywords in the
Metadata Section, a new Segment must be started (e.g., mode change from one-way to two-
way tracking).

3.3.1.5 The first and last lines of a TDM Metadata Section shall consist of the
META_START and META_STOP keywords, respectively. These keywords are used to
facilitate parsing.

3.3.1.6 Table 3-3 specifies for each Metadata item:

– the keyword to be used;

– a short description of the item;

– a list of required values or examples of allowed values; and

– whether the item is mandatory or optional.

The column marked ‘N/E’ will contain an ‘N’ if the column marked ‘Normative Values /
Examples’ contains normative values, and will contain an ‘E’ if the column contains example
values that are non-normative. For normative values, a fully enumerated set of values is
provided.

3.3.1.7 Only those keywords shown in table 3-3 shall be used in a TDM Metadata Section.
Mandatory items shall appear in every TDM Metadata Section. Items that are optional may
or may not appear in any given TDM Metadata Section, at the discretion of the data
producer, based on the requirements of the data and its intended application (see annex I for
a TDM Summary Sheet that illustrates the relationships between data types and metadata).
For most metadata keywords there is no default value; where there is a default value, it is
specified at the end of the ‘Description’ section for the given keyword. If a keyword is not
present in a TDM, and a default value is defined, the default shall be assumed.

3.3.1.8 The order of occurrence of the mandatory and optional KVN assignments shall be
fixed as shown in table 3-3.

3.3.1.9 The Metadata Section shall describe the participants in a tracking session using the
keyword ‘PARTICIPANT_n’. There may be several participants associated with a tracking
data session (the number of participants is always greater than or equal to one, and generally
greater than or equal to two). The ‘n’ in the keyword is an indexer. The indexer shall not be
the same for any two participants in a given Metadata Section.

3.3.1.10 The value associated with any given PARTICIPANT_n keyword may be a ground
tracking station, a spacecraft, a quasar catalog name; or may include non-traditional objects,
such as landers, rovers, balloons, etc. The list of eligible names that is used to specify
participants should be documented in the ICD. Subsections 3.3.2 through 3.3.2.7 identify the
relationships between the MODE, the PATH, and PARTICIPANT_n keywords for typical
tracking sessions. Participants may generally be listed in any order.

3.3.1.11 In this version of the TDM, the maximum number of participants per segment shall
be five. If more than five participants are defined (i.e., PARTICIPANT_6 +), then special
arrangements between exchange participants are necessary. These arrangements should be
documented in an ICD. It should be noted that although the restriction to five participants
may appear to be a constraint it is probably not, because of other aspects of the TDM
structure. Five participants easily allow the user to describe the great majority of tracking
passes. In some cases there may be ‘critical event’ tracking sessions in which a single
spacecraft is tracked by a large number of antennas, such that the total number of participants
appears to be six or more. However, because of the nature of the ‘PATH’ keyword, several
TDM Segments with five or fewer participants would be required to describe the full set of
tracking data. For the critical event example scenario just given, one TDM Segment would
be used to describe the two-way connection, and one additional segment would be required
for each three-way connection; it would not be possible to provide a single ‘PATH’
statement that would convey the multiple signal paths.

.. _table_3-3:

.. list-table:: Table 3-3: TDM Metadata Section
   :widths: 20 40 25 5 10
   :header-rows: 1

   * - Keyword
     - Description
     - Normative Values / Examples
     - N/E
     - Mandatory
   * - META_START
     - The META_START keyword shall delineate the start of the TDM Metadata Section within the message. It must appear on a line by itself; that is, it shall have no parameters, timetags or values.
     - N/A
     - -----
     - Yes
   * - COMMENT
     - (See 4.5.) It should be noted that if comments are used in the metadata, they shall only appear at the beginning of the Metadata Section.
     - COMMENT file = tdm.dat
     - E
     - No
   * - TRACK_ID
     - The TRACK_ID keyword specifies a unique identifier for the tracking data in the associated data section. The value may be a freely selected string of characters and numbers, only required to be unique for each track of the corresponding sensor. For example, the value may be constructed from the measurement date and time and a counter to distinguish simultaneously tracked objects.
     - 20190918_1200135-0001
     - E
     - No
   * - DATA_TYPES
     - Comma-separated list of data types in the Data Section. The elements of the list shall be selected from the data types shown in table 3-5, with the exception of the DATA_START, DATA_STOP, and COMMENT keywords.
     - RANGE, TRANSMIT_FREQ_n, RECEIVE_FREQ
     - E
     - No
   * - TIME_SYSTEM
     - The TIME_SYSTEM keyword shall specify the time system used for timetags in the associated Data Section. This should be UTC for ground-based data. The value associated with this keyword must be selected from the full set of allowed values enumerated in the SANA Time Systems Registry https://sanaregistry.org/r/time_systems (reference [12]). (See annex B.)
     - UTC, TAI, GPS, SCLK
     - E
     - Yes
   * - START_TIME
     - The START_TIME keyword shall specify the UTC start time of the total time span covered by the tracking data immediately following this Metadata Section. (For format specification, see 4.3.9.)
     - 1996-12-18T14:28:15.1172
       1996-277T07:22:54
       2006-001T00:00:00Z
     - E
     - No
   * - STOP_TIME
     - The STOP_TIME keyword shall specify the UTC stop time of the total time span covered by the tracking data immediately following this Metadata Section. (For format specification, see 4.3.9.)
     - 1996-12-18T14:28:15.1172
       1996-277T07:22:54
       2006-001T00:00:00Z
     - E
     - No
   * - PARTICIPANT_n, n = {1, 2, 3, 4, 5}
     - The PARTICIPANT_n keyword shall represent the participants (see 1.3.4.1) in a tracking data session. It is indexed to allow unambiguous reference to other data in the TDM (max index is 5). At least two participants must be specified for most sessions; for some special TDMs such as tropospheric media, only one participant need be listed. Participants represent the classical transmitting parties, transponding parties, and receiving parties, while allowing for flexibility to consider tracking sessions that go beyond the familiar one-way spacecraft-to-ground, two-way ground-spacecraft-ground, etc. Participants may be listed in any order, and the PATH keywords specify the signal paths. For spacecraft identifiers, there is no CCSDS-based restriction on the value for this keyword, but names could be drawn from the United Nations Outer Space Objects Index (reference [1]), which includes Object name and international designator of the participant. The list of eligible names that is used to specify participants should be documented in the ICD. The PARTICIPANT_n may be ‘UNKNOWN’, for example, in initial space surveillance object detection.
     - DSS-63-S400K, ROSETTA, <Quasar catalog name>, 1997-061A, UNKNOWN
     - E
     - Yes (at least one)
   * - MODE
     - The MODE keyword shall reflect the tracking mode associated with the Data Section of the segment. The value ‘SEQUENTIAL’ applies for frequencies, phase, range, Doppler, carrier power, carrier-power-to-noise spectral density, ranging-power-to-noise spectral density, optical, angles, and line-of-sight ionosphere calibrations; the name implies a sequential signal path between tracking participants. The value ‘SINGLE_DIFF’ applies only for differenced data. In other cases, such as troposphere, weather, clocks, etc., use of the MODE keyword does not apply.
     - SEQUENTIAL, SINGLE_DIFF
     - N
     - No
   * - PATH, PATH_1, PATH_2
     - The PATH keywords shall reflect the signal path by listing the index of each participant in order, separated by commas, with no inserted white space. The integers 1, 2, 3, 4, 5 used to specify the signal path are correlated with the indices of the PARTICIPANT_n keywords. The first entry in the PATH shall be the transmit participant. The non-indexed ‘PATH’ keyword shall be used if the MODE is SEQUENTIAL (i.e., MODE=SEQUENTIAL is specified). The indexed ‘PATH_1’ and ‘PATH_2’ keywords shall be used where the MODE is ‘SINGLE_DIFF’. Examples: 1,2 = one-way; 2,1,2 = two-way; 3,2,1 = three-way; 1,2,3,4 = four-way.
     - PATH = 1,2,1, PATH_1 = 1,2,1, PATH_2 = 3,1
     - E
     - No
   * - EPHEMERIS_NAME_n, n = {1, 2, 3, 4, 5}
     - Unique name of the external ephemeris file used for tracking one of the n PARTICIPANTs. The ‘n’ corresponds to the ‘n’ associated with the PARTICIPANT_n keyword (e.g., EPHEMERIS_NAME_1, if present, applies to PARTICIPANT_1).
     - SATELLITE_A_EPHEM27
     - E
     - No
   * - TRANSMIT_BAND
     - The TRANSMIT_BAND keyword shall indicate the frequency band for transmitted frequencies. The frequency ranges associated with each band should be specified in the ICD.
     - S, X, Ka, L, UHF, GREEN
     - E
     - No
   * - RECEIVE_BAND
     - The RECEIVE_BAND keyword shall indicate the frequency band for received frequencies. Although not required in general, the RECEIVE_BAND must be present if the MODE is SINGLE_DIFF and differenced frequencies or differenced range are provided in order to allow proper frequency dependent corrections to be applied. The frequency ranges associated with each band should be specified in the ICD.
     - S, X, Ka, L, UHF, GREEN
     - E
     - No
   * - TURNAROUND_NUMERATOR
     - The TURNAROUND_NUMERATOR keyword shall indicate the numerator of the turnaround ratio that is necessary to calculate the coherent downlink from the uplink frequency. The value shall be an integer. Also may be specified in the ICD if the value is always constant.
     - 240, 880
     - E
     - No
   * - TURNAROUND_DENOMINATOR
     - The TURNAROUND_DENOMINATOR keyword shall indicate the denominator of the turnaround ratio that is necessary to calculate the coherent downlink from the uplink frequency. The value shall be an integer. Also may be specified in the ICD if the value is always constant.
     - 221, 749
     - E
     - No
   * - TIMETAG_REF
     - The TIMETAG_REF keyword shall provide a reference for time tags in the tracking data. This keyword indicates whether the timetag associated with the data is the transmit time or the receive time. This keyword is provided specifically to accommodate two special cases: (1) systems where a received range data point has been timetagged with the time that the range tone signal was transmitted (i.e., TIMETAG_REF=TRANSMIT), and (2) for quasar DOR, where the transmit frequency is the interferometer reference frequency at receive time (i.e., TIMETAG_REF=RECEIVE). It is anticipated otherwise that transmit-related data will generally be timetagged with the time of transmission, and that receive-related data will generally be timetagged with the time of receipt; in these two standard cases, it is not necessary to specify the TIMETAG_REF keyword.
     - TRANSMIT, RECEIVE
     - N
     - No
   * - INTEGRATION_INTERVAL
     - The INTEGRATION_INTERVAL keyword shall provide the Doppler count time in seconds for Doppler data or for the creation of normal points (also applicable for differenced Doppler; also sometimes known as ‘compression time’, ‘condensation interval’, etc.). The data type shall be positive double precision.
     - 60.0, 0.1, 1.0
     - E
     - No
   * - INTEGRATION_REF
     - The INTEGRATION_REF keyword shall be used in conjunction with the INTEGRATION_INTERVAL and TIMETAG_REF keywords. This keyword indicates the relationship between the INTEGRATION_INTERVAL and the timetag on the data, i.e., whether the timetag represents the start, middle, or end of the integration period.
     - START, MIDDLE, END
     - N
     - No
   * - FREQ_OFFSET
     - The FREQ_OFFSET keyword represents a frequency in Hz that must be added to every RECEIVE_FREQ (see 3.5.2.8) to reconstruct it. One use is if a Doppler shift frequency observable is transferred instead of the actual received frequency. The data type shall be double precision, and may be negative, zero, or positive. Examples are shown in the ‘Normative Values / Examples’ column. The default shall be 0.0 (zero).
     - 0.0, 8415000000.0
     - E
     - No
   * - RANGE_MODE
     - The value of the RANGE_MODE keyword shall be ‘COHERENT’, in which case the range tones are coherent with the uplink carrier, and the range unit must be defined in an ICD; ‘CONSTANT’, in which case the range tones have a constant frequency; or ‘ONE_WAY’ (used in Delta-DOR). NOTE – It cannot be determined in advance whether the range mode is coherent or non-coherent. For ESA and JAXA, it is important for the two/three-way Doppler to be coherent, but not the RANGE. This keyword may not be applicable for differenced range data.
     - COHERENT, CONSTANT, ONE_WAY
     - N
     - No
   * - RANGE_MODULUS
     - The value associated with the RANGE_MODULUS keyword shall be the modulus of the range observable in the units as specified by the RANGE_UNITS keyword; that is, the actual (unambiguous) range is an integer k times the modulus, plus the observable value. RANGE_MODULUS shall be a non-negative double precision value. For measurements that are not ambiguous range, the MODULUS setting shall be 0 to indicate an essentially infinite modulus. The default value shall be 0.0. NOTE – The range modulus is sometimes also called the ‘range ambiguity’.
     - 32768.0, 2.0e+23, 0.0, 161.6484
     - E
     - No
   * - RANGE_UNITS
     - The RANGE_UNITS keyword specifies the units for the range observable. ‘km’ shall be used if the range is measured in kilometers. ‘s’ shall be used if the range is measured in seconds. ‘RU’, for ‘range units’, shall be used where the transmit frequency is changing, and the method of computing the range unit should be described in the ICD. The default (preferred) value shall be ‘km’.
     - km, s, RU
     - N
     - No
   * - ANGLE_TYPE
     - The ANGLE_TYPE keyword shall indicate the type of antenna geometry represented in the angle data (ANGLE_1 and ANGLE_2 keywords). The value shall be one of the values: AZEL for azimuth, elevation (local horizontal); RADEC for right ascension, declination or hour angle, declination (must be referenced to an inertial frame); XEYN for x-east, y-north; XSYE for x-south, y-east. Other values are possible, but must be defined in an ICD.
     - AZEL, RADEC, XEYN, XSYE
     - N
     - No
   * - REFERENCE_FRAME
     - The REFERENCE_FRAME keyword shall be used in conjunction with the ‘ANGLE_TYPE=RADEC’ keyword/value combination, indicating the inertial reference frame to which the antenna frame is referenced. The origin (center) of the reference frame is assumed to be at the antenna reference point. Applies only to ANGLE_TYPE = RADEC. The value associated with this keyword must be selected from the full set of allowed values enumerated in the SANA Celestial Body Reference Frames Registry https://sanaregistry.org/r/celestial_body_reference_frames (reference [13]). (See annex B.)
     - EME2000, ICRF, ITRF1993, ITRF2000, TOD_EARTH
     - E
     - No
   * - INTERPOLATION
     - The INTERPOLATION keyword shall specify the interpolation method to be used to calculate a transmit phase count at an arbitrary time in tracking data where the uplink frequency is not constant.
     - HERMITE, LAGRANGE, LINEAR
     - E
     - No
   * - INTERPOLATION_DEGREE
     - The INTERPOLATION_DEGREE keyword shall specify the recommended degree of the interpolating polynomial used to calculate a transmit phase count at an arbitrary time in tracking data where the uplink frequency is not constant. The value must be an integer and must be used if the INTERPOLATION keyword is used.
     - 3, 5, 7, 11
     - E
     - No
   * - DOPPLER_COUNT_BIAS
     - Doppler counts are generally biased so as to accommodate negative Doppler within an accumulator. In order to reconstruct the measurement, the bias shall be subtracted from the DOPPLER_COUNT data value. The data type shall be double precision, and shall be positive. Examples are shown in the ‘Normative Values / Examples’ column. Units are Hz.
     - 2.4e6, 240000000.0
     - E
     - No
   * - DOPPLER_COUNT_SCALE
     - Doppler counts are generally scaled so as to capture partial cycles in an integer count. In order to reconstruct the measurement, the DOPPLER_COUNT data value shall be divided by the scale factor. The data type shall be integer, and shall be positive. Examples are shown in the ‘Normative Values / Examples’ column. The default shall be 1 (one).
     - 1000, 1
     - E
     - No
   * - DOPPLER_COUNT_ROLLOVER
     - Doppler counts may overflow the accumulator and roll over in cases where the track is of long duration or very high Doppler shift. This flag indicates whether or not a counter rollover has occurred during the track.
     - YES, NO
     - N
     - No
   * - TRANSMIT_DELAY_n, n = {1, 2, 3, 4, 5}
     - The TRANSMIT_DELAY_n keyword shall specify a fixed interval of time, in seconds, required for the signal to travel from the transmitting electronics to the transmit point. This may be used to account for gross factors that do not change from pass to pass, such as antennas with remote electronics, arraying delays, or spacecraft transponder delays. The ‘n’ corresponds to the ‘n’ associated with the PARTICIPANT_n keyword (e.g., TRANSMIT_DELAY_1, if present, applies to timetags for PARTICIPANT_1). Delays associated with uplink antenna arraying should be indicated with this keyword. If the user wishes to convey a ranging transponder delay, then one half of the transponder delay should be specified via the TRANSMIT_DELAY_n keyword. The TRANSMIT_DELAY should generally not be included in ground corrections applied to the tracking data. The TRANSMIT_DELAY shall be a non-negative double precision value. The default value shall be 0.0. NOTE – This value should not be used to convey clock bias information. (See the ‘CLOCK_BIAS’ keyword in the Data Section keywords.)
     - 1.23, 0.0326, 0.00077
     - E
     - No
   * - RECEIVE_DELAY_n, n = {1, 2, 3, 4, 5}
     - The RECEIVE_DELAY_n keyword shall specify a fixed interval of time, in seconds, required for the signal to travel from the tracking point to the receiving electronics. This may be used to account for gross factors that do not change from pass to pass, such as antennas with remote electronics, arraying delays, or spacecraft transponder delays. The ‘n’ corresponds to the ‘n’ associated with the PARTICIPANT_n keyword (e.g., RECEIVE_DELAY_1, if present, applies to timetags for PARTICIPANT_1). Delays associated with downlink antenna arraying should be indicated with this keyword. If the user wishes to convey a ranging transponder delay, then one half of the transponder delay should be specified via the RECEIVE_DELAY_n keyword. The RECEIVE_DELAY should generally not be included in ground corrections applied to the tracking data. The RECEIVE_DELAY shall be a non-negative double precision value. The default value shall be 0.0. NOTE – This value should not be used to convey clock bias information. (See the ‘CLOCK_BIAS’ keyword in the Data Section keywords.)
     - 1.23, 0.0326, 0.00777
     - E
     - No
   * - DATA_QUALITY
     - The DATA_QUALITY keyword may be used to provide an estimate of the quality of the data, based on indicators from the producers of the data (e.g., bad time synchronization flags, marginal lock status indicators, etc.). A value of ‘RAW’ shall indicate that no quality check of the data has occurred (e.g., in a real-time broadcast or near–real-time automated file transfer). A value of ‘VALIDATED’ shall indicate that data quality has been checked, and passed tests. A value of ‘DEGRADED’ shall indicate that data quality has been checked and quality issues exist. ‘Checking’ may be via human intervention or automation. Specific definitions of ‘RAW’, ‘VALIDATED’, and ‘DEGRADED’ that may apply to a particular exchange should be listed in the ICD. If the value is ‘DEGRADED’, information on the nature of the degradation may be conveyed via the COMMENT mechanism. It should be noted that because of the nature of TDM metadata, if ‘DEGRADED’ is specified, it applies to all the data in the segment. Thus degraded data should be isolated in dedicated segments. The default value shall be ‘RAW’ (rationale: agencies often do not validate tracking data before export).
     - RAW, VALIDATED, DEGRADED
     - N
     - No
   * - CORRECTION_ANGLE_1, CORRECTION_ANGLE_2, CORRECTION_DOPPLER, CORRECTION_MAG, CORRECTION_RANGE, CORRECTION_RCS, CORRECTION_RECEIVE, CORRECTION_TRANSMIT, CORRECTION_ABERRATION_YEARLY, CORRECTION_ABERRATION_DIURNAL
     - The set of CORRECTION_* keywords may be used to reflect the values of corrections that have been added to the data or should be added to the data (e.g., ranging station delay calibration, etc.). This information may be provided to the user, so that the base measurement could be recreated if a different correction procedure is desired. Tracking data should be corrected for ground delays only. It should be noted that it may not be feasible to apply all ground corrections for a near–real-time transfer. Units for the correction shall be the same as those for the applicable observable. All corrections should be signed, double precision values. Examples are shown in the ‘Normative Values / Examples’ column.
     - -1.35, 0.23, -3.0e-1, 150000.0
     - E
     - No
   * - CORRECTIONS_APPLIED
     - This keyword is used to indicate whether or not the values associated with the CORRECTION_* keywords have been applied to the tracking data. This keyword is required if any of the CORRECTION_* keywords is used. Because of the nature of TDM metadata, the application of corrections applies to all of the data described by a given Metadata Section. Thus all of the data in a given segment must have corrections applied or corrections not applied. The value of this keyword thus applies to all the data related to a Metadata Section in which it is used.
     - YES, NO
     - N
     - No
   * - META_STOP
     - The META_STOP keyword shall delineate the end of the TDM Metadata Section within the message. It must appear on a line by itself; that is, it shall have no parameters, timetags, or values.
     - N/A
     - -----
     - Yes

3.3.2 MODE AND PATH SETTINGS FOR TYPICAL TRACKING SESSIONS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

NOTE – The following subsections discuss possible relationships between the ‘MODE’,
‘PATH’, and ‘PARTICIPANT_n’ keywords. This discussion is provided in
order to facilitate the implementation of TDM generation for typical tracking
sessions (e.g., one-way, two-way, three-way, etc.). Annex I supplies
recommendations of the metadata keywords that should be used to properly
describe the tracking data of various types depending on the settings of the
MODE and PATH keywords, with allowance for characteristics of the uplink
frequency (if applicable).

3.3.2.1 One-Way Data
""""""""""""""""""""

3.3.2.1.1 The setting of the ‘MODE’ keyword shall be ‘SEQUENTIAL’.

3.3.2.1.2 For one-way data, the signal path generally originates at the spacecraft
transmitter, so the spacecraft’s participant number shall be the first number in the value
assigned to the PATH keyword. The receiver, which may be a tracking station or another
spacecraft, shall be represented by the second number in the value of the PATH keyword.

EXAMPLES – ‘PATH=1,2’ indicates transmission from PARTICIPANT_1 to
PARTICIPANT_2; ‘PATH=2,1’ indicates transmission from
PARTICIPANT_2 to PARTICIPANT_1.

3.3.2.1.3 To facilitate generation of the one-way tracking observable, the nominal
spacecraft transmit frequency should be provided via a TRANSMIT_FREQ_n keyword in
TDMs that contain one-way receive frequency data. The transmit frequency data may be in
the same segment as the receive frequency data, or a separate segment, at the preference of
the TDM originator.

NOTE – Figures E-1 and E-2 are examples TDMs containing one-way tracking data.

3.3.2.2 Two-Way Data
""""""""""""""""""""

3.3.2.2.1 The setting of the ‘MODE’ keyword shall be ‘SEQUENTIAL’.

3.3.2.2.2 For two-way data, the signal path originates at a ground antenna (or a ‘first
spacecraft’), so the uplink (or crosslink) transmit participant number shall be the first number
in the value assigned to the PATH keyword. The participant number of the transponder
onboard the spacecraft to which the signal is being uplinked shall be the second number in
the value assigned to the PATH keyword. The third entry in the PATH keyword value shall
be the same as the first (two-way downlink is received at the same participant which
transmits the uplink/crosslink). Both PARTICIPANT_1 and PARTICIPANT_2 may be
spacecraft as in the case of a spacecraft-spacecraft exchange.

EXAMPLES – ‘PATH=1,2,1’ indicates transmission from PARTICIPANT_1 to
PARTICIPANT_2, with final reception at PARTICIPANT_1;
‘PATH=2,1,2’ indicates transmission from PARTICIPANT_2 to
PARTICIPANT_1, with final reception at PARTICIPANT_2.

NOTE – Figures E-3, E-4, E-9, E-18, E-19, and E-20 are example TDMs containing two-
way tracking data.

3.3.2.3 Three-Way Data
""""""""""""""""""""""

3.3.2.3.1 The setting of the ‘MODE’ keyword shall be ‘SEQUENTIAL’.

3.3.2.3.2 For three-way data, the signal path originates with a ground station (uplink
antenna), so the participant number of the uplink station shall be the first entry in the value
assigned to the PATH keyword. The participant number of the transponder onboard the
spacecraft to which the signal is being uplinked shall be the second number in the value
assigned to the PATH keyword. The participant number of the downlink antenna shall be the
third number in the value assigned to the PATH keyword.

3.3.2.3.3 For three-way data, the first and last numbers in the value assigned to the PATH
keyword must be different.

EXAMPLES – ‘PATH=1,2,3’ indicates transmission from PARTICIPANT_1 to
PARTICIPANT_2, with final reception at PARTICIPANT_3.

NOTE – Figure E-5 is an example TDM containing three-way tracking data.

3.3.2.4 N-Way Data
""""""""""""""""""

3.3.2.4.1 One-way, two-way, and three-way tracking cover the bulk of tracking sequences.
However, four-way and greater (n-way) scenarios are possible (e.g., via use of one or more
relay satellites). These may be accomplished via the sequence assigned to the PATH
keyword.

3.3.2.4.2 The setting of the ‘MODE’ keyword shall be ‘SEQUENTIAL’.

3.3.2.4.3 The value assigned to the PATH keyword shall convey the signal path among the
participants followed by the signal; for example, ‘PATH=1,2,3,2,1’ and ‘PATH=1,2,3,4’
represent two different four-way tracking signal paths.

3.3.2.4.4 In this version of the TDM, the maximum number of participants per segment
shall be five. If more than five participants are defined (i.e., PARTICIPANT_6 +), then
special arrangements shall be made by exchange participants; these should be specified in the
ICD.

NOTE – Figure E-6 is an example TDM containing four-way tracking data.

3.3.2.5 Differenced Modes and VLBI Data
"""""""""""""""""""""""""""""""""""""""

3.3.2.5.1 Differenced data and VLBI data may also be exchanged in a Tracking Data
Message. Differenced data may include differenced Doppler and differenced range (see
references [F3] and [F4]).

3.3.2.5.2 The setting of the ‘MODE’ keyword shall be ‘SINGLE_DIFF’.

3.3.2.5.3 When the MODE is ‘SINGLE_DIFF’, two path keywords, ‘PATH_1’ and
‘PATH_2’, shall be used to convey the signal paths that have been differenced.

3.3.2.5.4 When the MODE is ‘SINGLE_DIFF’, the observable shall be calculated by
subtracting the value achieved for the measurement using PATH_1 from the value achieved
using PATH_2, that is, PATH_2 – PATH_1. Only the final observable shall be communicated
via the TDM.

3.3.2.5.5 If the TDM contains differenced Doppler shift data, the ‘RECEIVE_FREQ’
keyword shall be used for the observable (the ‘RECEIVE_FREQ’ keyword is a Data Section
keyword not yet described in the text—see 3.5.2.8).

3.3.2.5.6 If the TDM contains two-way or three-way differenced Doppler data, then a
history of the uplink frequencies shall be provided with the TRANSMIT_FREQ_n keyword
in order to process the data correctly (the ‘TRANSMIT_FREQ_n’ keyword is a Data Section
keyword not yet described in the text—see 3.5.2.9).

3.3.2.5.7 If differenced range is provided, the ‘RANGE’ keyword shall be used for the
observable (the ‘RANGE’ keyword is a Data Section keyword not yet described in the text—
see 3.5.2.7).

3.3.2.5.8 If the TDM contains differenced data collected during a Delta-Differential One
Way Range (Delta-DOR) session with a spacecraft, then the DOR keyword shall be used for
the observable (the ‘DOR’ keyword is a Data Section keyword not yet described in the
text—see 3.5.3.2).

3.3.2.5.9 If the TDM contains differenced data collected during a VLBI session with a
quasar, then the VLBI_DELAY keyword shall be used for the observable (the
‘VLBI_DELAY’ keyword is a Data Section keyword not yet described in the text—see
3.5.3.3).

NOTE – Figures E-10 and E-11 are example TDMs containing single differenced tracking
data.

3.3.2.6 Angle Data
""""""""""""""""""

Angle data is applicable for any tracking scenario where MODE=SEQUENTIAL is
specified, but is based on pointing with respect to the two rightmost participants listed in the
PATH statement (e.g., spacecraft downlink to an antenna, direction of a participant measured
by a navigation camera, etc.).

NOTE – Figures E-8 and E-12 are example TDMs containing angle data.

3.3.2.7 Media, Weather, Ancillary Data
""""""""""""""""""""""""""""""""""""""

NOTE – Figures E-13 through E-15 are example TDMs containing tracking data related to
media, weather, and ancillary data.

3.3.2.7.1 When all the data in a TDM Segment is media related, weather related, or
ancillary-data related, then the use of the MODE keyword may or may not apply as discussed
below.

3.3.2.7.2 Data of this type may be relative to a reference location within the tracking
complex; in this case the methods used to extrapolate the measurements to other antennas
should be specified in the ICD. In the case where a reference location is used, there shall be
only one participant (PARTICIPANT_1), which is the reference antenna, and the MODE
keyword shall not be used. This case corresponds to tropospheric correction data, zenith
ionospheric correction data, and weather data.

3.3.2.7.3 When ionospheric charged particle delays are provided for a line-of-sight between
the antenna and a specific spacecraft, the participants include both the antenna and the
spacecraft, the MODE should be set to ‘SEQUENTIAL’, and a standard PATH statement
should be used.
.. _tdm_data_section:

3.4 TDM DATA SECTION (GENERAL SPECIFICATION)
--------------------------------------------

3.4.1 The Data Section of the TDM Segment shall consist of one or more Tracking Data
Records. Each Tracking Data Record shall have the following generic format:

   keyword = timetag measurement

NOTE – More detail on the generic format of a Tracking Data Record is shown in table 3-4.

.. _table_3-4:

.. list-table:: Table 3-4: Tracking Data Record Generic Format
   :widths: 25 45 15 15
   :header-rows: 1

   * - Element
     - Description
     - Examples
     - Mandatory
   * - <keyword>
     - Data type keyword from the list specified in 3.5.
     - (See annex E.)
     - Yes (at least one keyword must be used)
   * - =
     - Equals sign
     - =
     - Yes
   * - <timetag>
     - Time associated with the tracking observable according to the TIME_SYSTEM keyword. (For requirements on the timetag, see 3.4.8 through 3.4.12. For format specification, see 4.3.9. )
     - 2003-205T18:00:01.275
       2003-205T18:00:01Z
     - Yes
   * - <measurement>
     - Tracking observable (measurement or calculation) in units defined in the TDM.
     - (See 3.5.)
     - Yes

3.4.2 Each Tracking Data Record must be provided on a single line.

3.4.3 Each Tracking Data Record shall contain a value that depends upon the data type
keyword used. The value shall consist of two elements: a timetag and a tracking observable
(a measurement or calculation based on measurements); either without the other is useless
for tracking purposes. Hereafter, the term ‘measurement’ shall be understood to include
calculations based on measurements as noted above.

3.4.4 At least one blank character must be used to separate the timetag and the observable
in the value associated with each Tracking Data Record.

3.4.5 Applicable keywords and their associated characteristics are detailed in 3.5.

3.4.6 There shall be no mandatory keywords in the Data Section of the TDM Segment,
with the exception of ‘DATA_START’ and ‘DATA_STOP’, because the data presented in
any given TDM is dependent upon the characteristics of the data collection activity.

3.4.7 The Data Section of the TDM Segment shall be delineated by the ‘DATA_START’ and
‘DATA_STOP’ keywords. These keywords are intended to facilitate parsing, and will also
serve to advise the recipient that all the Tracking Data Records associated with the immediately
preceding TDM Metadata Section have been received. The TDM recipient may process the
‘DATA_STOP’ keyword as a ‘local’ end-of-file marker.

3.4.8 Tracking data shall be tagged according to the value of the ‘TIME_SYSTEM’
metadata keyword.

3.4.9 Interpretation of the timetag for transmitted data is straightforward; it is the transmit
time. Interpretation of the timetag for received data is determined by the values of the
‘TIMETAG_REF’, ‘INTEGRATION_REF’, and ‘INTEGRATION_INTERVAL’ keywords,
as applicable (see table 3-3 and 3.5.2.8). For other data types (e.g., meteorological, media,
clock bias/drift), the timetag represents the time the measurement was taken.

3.4.10 In general, no required ordering of Tracking Data Records shall be imposed, because
there are certain scenarios in which data are collected from multiple sources that are not
processed in strictly chronological order. Thus it may only be possible to generate data in
chronological order if it is sorted post-pass. However, there is one ordering requirement
placed on Tracking Data Records; specifically, in any given Data Section, the data for any
given keyword shall be in chronological order. Also, some TDM creators may wish to sort
tracking data by keyword rather than by timetag. Special sorting requirements should be
specified in the ICD.

3.4.11 Each keyword/timetag combination must be unique within a given Data Section (i.e.,
a given keyword/timetag combination shall not be repeated in the same set of Tracking Data
Records).

3.4.12 The time duration between timetags may be constant, or may vary, within any given
TDM.

3.4.13 Every tracking instrument shall have a defined reference location. This reference
location shall not depend on the observing geometry. The tracking instrument locations
should be conveyed via an ICD. The ICD information should include a complete description
of the station locations and characteristics, including the antenna coordinates with their
defining system, plate motion, and the relative geometry of the tracking point and cross axis
of the antenna mount, accommodations for antenna tilt to avoid keyhole problems, etc. The
station location could be provided via an OPM (reference [4]). Antenna geometry may be
necessary for exceptional cases, where the station location is not fixed during track, for
example.

3.4.14 The measurement shall be converted to an equipment-independent quantity; for
example, frequencies shall be reported at the ‘sky level’ (i.e., actual transmitted/received
frequencies, unless the FREQ_OFFSET keyword is used in the metadata). It should not be
necessary for the data recipient to have detailed information regarding the internal network
of the data producer.

3.4.15 Tracking data is normally subject to a number of corrections, as described in the
following paragraphs.

3.4.15.1 The tracking data measurements shall be corrected with the best estimate of all
known instrument calibrations, such as path delay calibrations between the reference point
and the tracking equipment, if applicable.

NOTE – These measures should reduce the requirement for consumers of tracking data to
have detailed knowledge of the underlying structure of the hardware/software
system that performed the measurements.

3.4.15.2 Tracking data should be corrected for ground delays only. The corrections that
have been applied may be specified to the message recipient via use of the optional
‘CORRECTION_*’ keywords in the metadata.

NOTE – The ‘TRANSMIT_DELAY’ and ‘RECEIVE_DELAY’ keywords do not
represent ‘ground corrections’ per se. They are meant to convey gross factors
that do not change from pass-to-pass (or contact-to-contact). However, if
exchange partners agree via the ICD, ‘TRANSMIT_DELAY’ and
‘RECEIVE_DELAY’ could be removed from the measurements. It is generally
operationally inconvenient for the producer to treat these values as corrections
because of the possible requirement to alter uplink timetags; thus these delays
are best handled in orbit determination post-processing. Modifying timetags to
account for these delays also complicates the use of differenced measurements.
It is thus more straightforward to allow the recipient to process these delays
rather than to correct the data prior to exchange.

3.4.15.3 If correction values are indicated via any of the ‘CORRECTION_*’ keywords, then
the TDM producer must indicate whether these correction values have or have not been applied
to the tracking data. This indication is accomplished via the use of the metadata keyword
‘CORRECTIONS_APPLIED’; this metadata item must have a value of ‘YES’ or ‘NO’.

3.4.15.4 Media corrections (ionosphere, troposphere) should not be applied by the TDM
producer; media corrections may be applied by the TDM recipient using the data conveyed in
the STEC, TROPO_WET, and TROPO_DRY Data Section keywords.

3.4.15.5 The party that will perform any applicable spin corrections should be specified in
the ICD (most appropriate party may be the party that operates the spacecraft).

3.4.15.6 Special correction algorithms that are more complex than a simple scalar value
should be specified in the ICD.

3.4.15.7 Any other corrections applied to the data should be agreed by the service provider
and the customer Agencies and specified in an ICD.

3.4.16 All data type keywords in the TDM Data Section must be from 3.5, which specifies
for each keyword:

– the keyword to be used;

– applicable units for the associated values;

– a reference to the text section where the keyword is described in detail.

NOTES

1 The standard tracking data types are extended to cover also some of the ancillary data
that may be required for precise orbit determination work. Subsection 3.5 identifies
the most frequently used data and ancillary types.

2 Annex E provides detailed usage examples.

3 Annex I supplies recommendations of the metadata keywords that should be used to
properly describe the tracking data of various types depending on the settings of the
MODE and PATH keywords, with allowance for characteristics of the uplink
frequency (if applicable).

4 The TDM structure allows a great deal of flexibility in terms of the content of a Data
Section, as shown in the examples in annex E. However, as a practical consideration
given the challenges of implementing generic TDM readers, early implementers of
the TDM have tended to minimize the number of data types represented in any given
TDM segment. For example, for a two-way tracking pass with ranging, the TDM
originator may provide three segments, one for transmit frequencies, one for received
frequencies, and one for range measurements.

.. _tdm_data_section_keywords:

3.5 TDM DATA SECTION KEYWORDS
-----------------------------

3.5.1 OVERVIEW
^^^^^^^^^^^^^^

This subsection describes each of the keywords that may be used in the Data Section of the
TDM Segment. In general, there is no required order in the Data Section of the TDM
Segment. Exceptions are the ‘DATA_START’ and ‘DATA_STOP’ keywords, which must
be the first and last keywords in the Data Section, respectively. For ease of reference,
table 3-5 containing all the keywords sorted in alphabetical order is shown immediately
below. Table 3-6 repeats the information from table 3-5 in category order. Descriptive
information about the keywords is shown starting in 3.5.2. The remainder of this subsection
is organized according to the category of data to which the keyword applies (e.g., all the
signal related keywords are together, all media related keywords are together, etc.).

.. _table_3-5:

.. list-table:: Table 3-5: Summary Table of TDM Data Section Keywords (Alpha Order)
   :widths: 60 20 20
   :header-rows: 1

   * - Keyword
     - Units
     - Text Link
   * - ANGLE_1
     - deg
     - 3.5.4.2
   * - ANGLE_2
     - deg
     - 3.5.4.3
   * - CARRIER_POWER
     - dBW
     - 3.5.2.1
   * - CLOCK_BIAS
     - s
     - 3.5.6.1
   * - CLOCK_DRIFT
     - s/s
     - 3.5.6.2
   * - COMMENT
     - n/a
     - 3.5.9.1
   * - DATA_START
     - n/a
     - 3.5.9.2
   * - DATA_STOP
     - n/a
     - 3.5.9.3
   * - DOPPLER_COUNT
     - n/a
     - 3.5.2.4
   * - DOPPLER_INSTANTANEOUS
     - km/s
     - 3.5.2.2
   * - DOPPLER_INTEGRATED
     - km/s
     - 3.5.2.3
   * - DOR
     - s
     - 3.5.3.2
   * - MAG
     - n/a
     - 3.5.5.1
   * - PC_N0
     - dBHz
     - 3.5.2.5
   * - PR_N0
     - dBHz
     - 3.5.2.6
   * - PRESSURE
     - hPa
     - 3.5.8.1
   * - RANGE
     - km, s, or RU
     - 3.5.2.7
   * - RCS
     - m**2
     - 3.5.5.2
   * - RECEIVE_FREQ_n (n = 1, 2, 3, 4, 5)
     - Hz
     - 3.5.2.8
   * - RECEIVE_FREQ
     - Hz
     - 3.5.2.8
   * - RECEIVE_PHASE_CT_n (n = 1, 2, 3, 4, 5)
     - n/a
     - 3.5.2.11
   * - RHUMIDITY
     - %
     - 3.5.8.2
   * - STEC
     - TECU
     - 3.5.7.1
   * - TEMPERATURE
     - K
     - 3.5.8.3
   * - TRANSMIT_FREQ_n (n = 1, 2, 3, 4, 5)
     - Hz
     - 3.5.2.9
   * - TRANSMIT_FREQ_RATE_n (n = 1, 2, 3, 4, 5)
     - Hz/s
     - 3.5.2.10
   * - TRANSMIT_PHASE_CT_n (n = 1, 2, 3, 4, 5)
     - n/a
     - 3.5.2.12
   * - TROPO_DRY
     - m
     - 3.5.7.2
   * - TROPO_WET
     - m
     - 3.5.7.3
   * - VLBI_DELAY
     - s
     - 3.5.3.3

.. _table_3-6:

.. list-table:: Table 3-6: Summary Table of TDM Data Section Keywords (Category Order)
   :widths: 60 20 20
   :header-rows: 1

   * - Keyword
     - Units
     - Text Link
   * - **Signal Related Keywords**
     -
     - **3.5.2**
   * - CARRIER_POWER
     - dBW
     - 3.5.2.1
   * - DOPPLER_COUNT
     - n/a
     - 3.5.2.4
   * - DOPPLER_INSTANTANEOUS
     - km/s
     - 3.5.2.2
   * - DOPPLER_INTEGRATED
     - km/s
     - 3.5.2.3
   * - PC_N0
     - dBHz
     - 3.5.2.5
   * - RECEIVE_PHASE_CT_n (n = 1, 2, 3, 4, 5)
     - n/a
     - 3.5.2.11
   * - TRANSMIT_PHASE_CT_n (n = 1, 2, 3, 4, 5)
     - n/a
     - 3.5.2.12
   * - PR_N0
     - dBHz
     - 3.5.2.6
   * - RANGE
     - km, s, or RU
     - 3.5.2.7
   * - RECEIVE_FREQ_n (n = 1, 2, 3, 4, 5)
     - Hz
     - 3.5.2.8
   * - RECEIVE_FREQ
     - Hz
     - 3.5.2.8
   * - TRANSMIT_FREQ_n (n = 1, 2, 3, 4, 5)
     - Hz
     - 3.5.2.9
   * - TRANSMIT_FREQ_RATE_n (n = 1, 2, 3, 4, 5)
     - Hz/s
     - 3.5.2.10
   * - **VLBI/Delta-DOR Related Keywords**
     -
     - **3.5.3**
   * - DOR
     - s
     - 3.5.3.2
   * - VLBI_DELAY
     - s
     - 3.5.3.3
   * - **Angle Related Keywords**
     -
     - **3.5.4**
   * - ANGLE_1
     - deg
     - 3.5.4.2
   * - ANGLE_2
     - deg
     - 3.5.4.3
   * - **Optical/Radar Related Keywords**
     -
     - **3.5.5**
   * - MAG
     - n/a
     - 3.5.5.1
   * - RCS
     - m**2
     - 3.5.5.2
   * - **Time Related Keywords**
     -
     - **3.5.6**
   * - CLOCK_BIAS
     - s
     - 3.5.6.1
   * - CLOCK_DRIFT
     - s/s
     - 3.5.6.2
   * - **Media Related Keywords**
     -
     - **3.5.7**
   * - STEC
     - TECU
     - 3.5.7.1
   * - TROPO_DRY
     - m
     - 3.5.7.2
   * - TROPO_WET
     - m
     - 3.5.7.3
   * - **Meteorological Related Keywords**
     -
     - **3.5.8**
   * - PRESSURE
     - hPa
     - 3.5.8.1
   * - RHUMIDITY
     - %
     - 3.5.8.2
   * - TEMPERATURE
     - K
     - 3.5.8.3
   * - **Miscellaneous Keywords**
     -
     - **3.5.9**
   * - COMMENT
     - n/a
     - 3.5.9.1
   * - DATA_START
     - n/a
     - 3.5.9.2
   * - DATA_STOP
     - n/a
     - 3.5.9.3

3.5.2 SIGNAL RELATED KEYWORDS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

3.5.2.1 CARRIER_POWER
"""""""""""""""""""""

The CARRIER_POWER keyword conveys the strength of the radio signal transmitted by the
spacecraft as received at the ground station or at another spacecraft (e.g., in formation flight).
This reports the strength of the signal received from the spacecraft, in decibels (referenced to
1 watt). The unit for the CARRIER_POWER keyword is dBW. The value shall be a double
precision value, and may be positive, zero, or negative. The value is based on the last leg of
the signal path (PATH keyword), for example, spacecraft downlink to an antenna.
Additional TDM Segments should be used for each participant if it is important to know the
carrier power at each participant in a PATH that involves more than one receiver.

3.5.2.2 DOPPLER_INSTANTANEOUS
"""""""""""""""""""""""""""""

The value associated with the DOPPLER_INSTANTANEOUS keyword represents the
instantaneous range rate of the spacecraft. The observable may be one-way, two-way, or
three-way. The value shall be a double precision value and may be negative, zero, or
positive. Units are km/s. In order to ensure that corrections due to the ionosphere and solar
plasma are accurately applied by the recipient, the transmit frequency and receive frequency
should be supplied when this data type is exchanged.

NOTE – The DOPPLER_INSTANTANEOUS assumes a fixed uplink frequency (or one
with small Round-Trip Light Time [RTLT] errors), and thus should not be used
in cases where there is a deep space ramped uplink (the TRANSMIT_FREQ and
RECEIVE_FREQ keywords should be used instead).

3.5.2.3 DOPPLER_INTEGRATED
""""""""""""""""""""""""""

The value associated with the DOPPLER_INTEGRATED keyword represents the mean
range rate of the spacecraft over the INTEGRATION_INTERVAL specified in the Metadata
Section. The timetag and the time bounds of the integration interval are determined by the
TIMETAG_REF and INTEGRATION_REF keywords. The observable may be one-way,
two-way, or three-way. For one-way data, the observable is the mean range rate of the
spacecraft over the INTEGRATION_INTERVAL. For two-way and three-way data, the
ICD should specify whether the observable is the calculated mean range rate, or half the
calculated mean range rate (due to the signal’s having traveled to the spacecraft and back to
the receiver). The value shall be a double precision value and may be negative, zero, or
positive. Units are km/s. In order to ensure that corrections due to the ionosphere and solar
plasma are accurately applied, the transmit frequency and receive frequency should be
supplied when this data type is exchanged.

NOTE – The DOPPLER_INTEGRATED assumes a fixed uplink frequency (or one with
small RTLT errors), and thus should not be used in cases where there is a deep
space ramped uplink (the TRANSMIT_FREQ and RECEIVE_FREQ keywords
should be used instead).

3.5.2.4 DOPPLER_COUNT
"""""""""""""""""""""

The value associated with the DOPPLER_COUNT keyword represents a count of the
number of times the phase of a received signal slips one cycle with respect to a transmitted
signal (or reference signal). The DOPPLER_COUNT keyword should be used in conjunction
with the DOPPLER_COUNT_BIAS, DOPPLER_COUNT_SCALE, and
DOPPLER_COUNT_ROLLOVER metadata. The value shall be an integer and should be
positive (though in unlikely cases it may be zero). Units are not applicable. It should be
noted that it may be necessary to process this data type in conjunction with a suitable Orbit
Data Message (ODM, reference [4]) in order to understand the velocity of the spacecraft
transmitter. The calculation to reconstruct the Doppler into units of Hertz is:

{[(DOPPLER_COUNT_n+1 – DOPPLER_COUNT_n)/(t_n+1 – t_n)] − DOPPLER_COUNT_BIAS}/DOPPLER_COUNT_SCALE

3.5.2.5 PC_N0
"""""""""""""

The value associated with the PC_N0 keyword shall be the carrier power to noise spectral
density ratio (Pc/N0). The units for PC_N0 shall be dBHz. The value shall be a double
precision value, and may be positive, zero, or negative.

3.5.2.6 PR_N0
"""""""""""""

The value associated with the PR_N0 keyword shall be the ranging power to noise spectral
density ratio (Pr/N0). The units for PR_N0 shall be dBHz. It shall be a double precision
value, and may be positive, zero, or negative.

3.5.2.7 RANGE
"""""""""""""

The value associated with the RANGE keyword is the range observable. The values
represent measurements from ambiguous ranging systems, differenced range, skin radar,
proximity radar, or similar radar. The units for RANGE shall be as determined by the
‘RANGE_UNITS’ metadata keyword (i.e., either ‘km’, ‘s’, or ‘RU’). The
‘RANGE_UNITS’ metadata keyword should always be specified, but if it is not, the default
(preferred) value shall be ‘km’. If different range units are used by the tracking agency (e.g.,
‘DSN range units’), the definition of the range unit should be described in the ICD. It should
be noted that for many applications, proper processing of the RANGE will require a time
history of the uplink frequencies. If ambiguous range is provided (i.e., the
RANGE_MODULUS is non-zero), then the RANGE does not represent the actual range to
the spacecraft; a calculation using the RANGE_MODULUS and the RANGE observable
must be performed. For two-way and three-way data, the ICD should specify whether the
observable is based upon the round trip light time, or half the round trip light time (due to the
signal’s having traveled to the spacecraft and back to the receiver). If differenced range is
provided (MODE = SINGLE_DIFF), the ‘RANGE’ keyword shall be used to convey the
difference in range. The value shall be a double precision value, and is generally positive
(exceptions to this could occur if the data is a differenced type, or if the observable is a one
way pseudorange).

NOTE – The TDM specifically excludes Satellite Laser Ranging (SLR), which is already
transferred via an internationally standardized format documented at
https://ilrs.cddis.eosdis.nasa.gov/.

3.5.2.8 RECEIVE_FREQ (and RECEIVE_FREQ_n)
"""""""""""""""""""""""""""""""""""""""""

3.5.2.8.1 The RECEIVE_FREQ keyword shall be used to indicate that the values represent
measurements of the received frequency. It is suitable for use with deep space ramped uplink if
the TRANSMIT_FREQ is also exchanged. The keyword is indexed to accommodate a
scenario in which multiple downlinks are used; it may also be used without an index where the
frequency cannot be associated with a particular participant (e.g., in the case of a differenced
Doppler shift measurement). The value associated with the RECEIVE_FREQ keyword shall
be the average frequency observable over the INTEGRATION_INTERVAL specified in the
metadata, at the measurement timetag. The interpretation of the timetag shall be determined by
the combined settings of the TIMETAG_REF, INTEGRATION_REF, and
INTEGRATION_INTERVAL keywords (see table 3-3 for a description of how the settings of
these values affect the interpretation of the timetag). Correlation between the
RECEIVE_FREQ and the associated TRANSMIT_FREQ may be determined via the use of an
a priori estimate and should be resolved via the orbit determination process. The units for
RECEIVE_FREQ shall be Hertz (Hz). The value shall be a double precision value (generally
positive, but could be negative or zero if used with the ‘FREQ_OFFSET’ metadata keyword).

3.5.2.8.2 Using the RECEIVE_FREQ, the instantaneous Doppler measurement in Hz is
calculated as follows:

.. math:: D_m = ((F_t * tr) - F_r)

where :math:`D_m` is the Doppler measurement, :math:`F_t` is the transmitted frequency, :math:`tr` is the
transponder ratio (tr=1 for one-way), and :math:`F_r` is the RECEIVE_FREQ.

For integrated Doppler, the Doppler measurement in Hz is calculated as follows, where :math:`t` is
the timetag, and :math:`\Delta t` is the value assigned to the INTEGRATION_INTERVAL keyword:

.. math:: D_m = \frac{1}{\Delta t} \int_{t + (\frac{-1}{2} + \alpha)\Delta t}^{t + (\frac{1}{2} + \alpha)\Delta t} ((F_t * tr) - F_r) dt

The limits of integration are determined by the INTEGRATION_REF keyword in the metadata;
the constant :math:`\alpha` in the equation has the value –½, 0, or ½ for the INTEGRATION_REF values of
‘END’, ‘MIDDLE’, or ‘START’, respectively (see reference [F4]).

.. list-table::
   :widths: 25 25 25 25
   :header-rows: 1

   * - INTEGRATION_REF
     - END
     - MIDDLE
     - START
   * - :math:`\alpha`
     - :math:`\alpha = -½`
     - :math:`\alpha = 0`
     - :math:`\alpha = ½`
   * - Upper Limit
     - :math:`t`
     - :math:`t + ½\Delta t`
     - :math:`t + \Delta t`
   * - Lower Limit
     - :math:`t - \Delta t`
     - :math:`t - ½\Delta t`
     - :math:`t`

3.5.2.8.3 If differenced Doppler is provided, the non-indexed ‘RECEIVE_FREQ’ keyword
shall be used to convey the difference in Hz.

3.5.2.8.4 The transponder ratios used for interagency exchanges should be specified in the
ICD if they are always constant. They may also be specified in the metadata by using the
TURNAROUND_NUMERATOR and TURNAROUND_DENOMINATOR keywords.

3.5.2.8.5 The equation for four-way Doppler, if it is to be exchanged, should be in the ICD
since the four-way connections tend to be implementation dependent.

3.5.2.9 TRANSMIT_FREQ_n
"""""""""""""""""""""""

The TRANSMIT_FREQ keyword shall be used to indicate that the values represent
measurements of a transmitted frequency, for example, from an uplink operation. The
TRANSMIT_FREQ keyword is indexed to accommodate scenarios in which multiple
transmitters are used. The value associated with the TRANSMIT_FREQ_n keyword shall be
the starting frequency observable at the timetag. The units for TRANSMIT_FREQ_n shall be
Hertz (Hz). The value shall be a positive double precision value. The turnaround ratios
necessary to calculate the predicted receive frequency may be specified using the
TURNAROUND_NUMERATOR and TURNAROUND_DENOMINATOR metadata
keywords, or may be specified in the ICD. In the case of software defined radios, the metadata
keywords may be preferable as the ratios can change with some regularity and it is necessary to
get the applicable ratio with the tracking data. Usage notes: when the data mode is one-way
(i.e., MODE=SEQUENTIAL, PATH=1,2 or PATH=2,1), the signal is at the beacon frequency
transmitted from the spacecraft. If a given spacecraft has more than one transponder, then there
should be unique names specified for each transponder (e.g., Cassini_S, Cassini_X,
Cassini_Ka). If a TDM is constructed with only transmit frequencies, then the MODE is
‘SEQUENTIAL’ and the PATH keyword defines the signal path. Generally the timetag for the
TRANSMIT_FREQ_n keywords should be the time that the signal was transmitted. For quasar
DOR, the TRANSMIT_FREQ_n is the interferometer reference frequency at the receive time
(thus TIMETAG_REF=RECEIVE for this case). If the transmit frequency varies in the TDM
segment, then the TRANSMIT_FREQ_RATE_n keyword should be used to convey the
frequency rate between transmit frequencies (see next section); otherwise, the frequency rate is
assumed to be zero and a step function results.

3.5.2.10 TRANSMIT_FREQ_RATE_n
"""""""""""""""""""""""""""""

The value associated with the TRANSMIT_FREQ_RATE_n keyword is the linear rate of
change of the frequency starting at the timetag and continuing until the next
TRANSMIT_FREQ_RATE timetag (or until the end of the data). The units for
TRANSMIT_FREQ_RATE_n shall be Hertz-per-second (Hz/s). The value shall be a double
precision value, and may be negative, zero, or positive. If the TRANSMIT_FREQ_RATE_n
is not specified, it is assumed to be zero (i.e., constant frequency).

3.5.2.11 RECEIVE_PHASE_CT_n
"""""""""""""""""""""""""""

The value associated with the RECEIVE_PHASE_CT keyword is the number of phase cycles
at the receiver. There are no applicable units for the RECEIVE_PHASE_CT. The keyword is
indexed to enable association with the PARTICIPANT_n. The value shall be a string
representing a real number that can be any number of digits required to convey the necessary
precision. If the received phase difference over a time interval is not based on the true
frequency but an intermediate frequency from which the true received frequency is calculated,
the FREQ_OFFSET metadata keyword should be specified to provide the intermediate
frequency.

3.5.2.12 TRANSMIT_PHASE_CT_n
""""""""""""""""""""""""""""

The value associated with the TRANSMIT_PHASE_CT keyword is the number of phase
cycles at the transmitter. The TRANSMIT_FREQ keyword is indexed to enable association
with the PARTICIPANT_n. There are no applicable units for the TRANSMIT_PHASE_CT.
The value shall be a string representing a real number that can be any number of digits
required to convey the necessary precision. If the transmit phase difference over a time
interval is not based on the true frequency but an intermediate frequency from which the true
transmit frequency is calculated, the FREQ_OFFSET metadata keyword should be used to
provide the intermediate frequency. If the uplink frequency is not constant then the
INTERPOLATION and INTERPOLATION_DEGREE metadata keywords shall be used to
characterize the uplink behavior.

3.5.3 VLBI AND DELTA-DOR RELATED KEYWORDS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

3.5.3.1 Overview
""""""""""""""""

In VLBI, a signal source is measured simultaneously using two receivers in different antenna
complexes, achieving a long baseline (up to thousands of kilometers). The signals recorded
at the two complexes are correlated and differenced to produce the observable, which may be
further processed by navigation software. ‘Delta-DOR’ sessions are a VLBI application in
which the antenna slews from a spacecraft source to a quasar source and back to the
spacecraft during the tracking pass. This sequence may occur multiple times. There are two
data keywords that relate to VLBI and Delta-DOR measurements, and several metadata
keyword settings are applicable (MODE=SINGLE_DIFF, PATH_1 and PATH_2).

3.5.3.2 DOR
"""""""""""

The observable associated with the DOR keyword represents the range measured via
PATH_2 minus the range measured via PATH_1. The timetag is the time of signal reception
via PATH_1. This data type is normally used for the spacecraft observable in a Delta-DOR
measurement. The range is either one-way, two-way, or three-way, depending on the values
of the PARTICIPANT_n and PATH keywords. TRANSMIT_FREQ_n shall provide the
spacecraft beacon frequency if one-way, or the transmit frequency at the uplink station if
two-way or three-way, at the signal transmission time. The DOR measurement shall be a
double precision value and may be negative or positive. Units shall be seconds.

3.5.3.3 VLBI_DELAY
""""""""""""""""""

The observable associated with the VLBI_DELAY keyword represents the time of signal
arrival via PATH_2 minus the time of signal arrival via PATH_1. The timetag is the time of
signal reception via PATH_1. This data type is normally used for the quasar observable in a
Delta-DOR measurement. TRANSMIT_FREQ_n shall provide the interferometer reference
frequency. The VLBI_DELAY measurement shall be a double precision value and may be
negative or positive. Units shall be seconds.

3.5.4 ANGLE DATA KEYWORDS
^^^^^^^^^^^^^^^^^^^^^^^^^

3.5.4.1 General
"""""""""""""""

Angle data is measured at the ground antenna, using downlink data only, regardless of the
mode of the tracking session. There shall be two angle keywords: ANGLE_1 and
ANGLE_2. The ANGLE_TYPE metadata keyword indicates how these two keywords
should be interpreted. Some TDM users may require that the ANGLE_1 keyword is
followed immediately by the corresponding ANGLE_2 keyword; however, this sort is not a
general TDM requirement. Special sorting requirements should be specified in the ICD.

3.5.4.2 ANGLE_1
"""""""""""""""

The value assigned to the ANGLE_1 keyword represents the azimuth, right ascension, or ‘X’
angle of the measurement, depending on the value of the ANGLE_TYPE keyword. The
angle measurement shall be a double precision value as follows: −180.0 ≤ ANGLE_1 <
360.0. Units shall be degrees.

3.5.4.3 ANGLE_2
"""""""""""""""

The value assigned to the ANGLE_2 keyword represents the elevation, declination, or ‘Y’
angle of the measurement, depending on the value of the ANGLE_TYPE keyword. The
angle measurement shall be a double precision value as follows: −180.0 ≤ ANGLE_2 <
360.0. Units shall be degrees.

3.5.5 OPTICAL/RADAR RELATED KEYWORDS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

3.5.5.1 MAG
"""""""""""

The value assigned to the MAG keyword shall represent the apparent visual magnitude of an
object when observed with an optical telescope. The apparent magnitude of an object is a
measure of its brightness as seen by an observer on Earth, adjusted to the value it would have
in the absence of the atmosphere. Units are not applicable. The MAG measurement shall be a
double precision value and may be positive, zero, or negative.

3.5.5.2 RCS
"""""""""""

The value assigned to the Radar Cross Section (RCS) keyword shall represent the radar cross
section of an object being tracked with a radar. The RCS shall be computed from radar
measurements to provide an indication of the detected object size, orientation, and surface
properties. It is the measure of a target’s ability to reflect radar signals in the direction of the
radar receiver. A larger RCS indicates that an object will be more easily detected. The RCS
measurement shall be a positive double precision value. Units shall be square meters (m**2).

3.5.6 TIME RELATED KEYWORDS
^^^^^^^^^^^^^^^^^^^^^^^^^^^

3.5.6.1 CLOCK_BIAS
""""""""""""""""""

In general, the timetags provided for the tracking data should be corrected, but when that is
not possible (e.g., for three-way data or differenced data types), then this data type may be
used. The CLOCK_BIAS keyword can be used by the message recipient to adjust timetag
measurements by a specified amount with respect to a common reference. For example, the
CLOCK_BIAS keyword may be used to show the difference between UTC and a station
clock by setting PARTICIPANT_1 to the name of the station clock and PARTICIPANT_2 to
‘UTC’. The observable should be calculated as clock#2 minus clock#1 (i.e., UTC – ST,
where ST is the station time), consistent with the TDM convention for differenced data. This
parameter may also be used to express the difference between two station clocks, for
example, for differenced data including Delta-DOR. If used for Delta-DOR, only a single
CLOCK_BIAS should be provided per daily VLBI session, with a time-tag strictly before the
first data point (e.g., one minute prior), and with the understanding that the clock will
continue to drift throughout the session. An exception could be made for the (rare) case
where a station clock is adjusted in the middle of a VLBI session, in which case a second
CLOCK_BIAS measurement may be provided. The clock bias is stated in the data, but the
timetags in the message have not been corrected by applying the bias; the message recipient
shall apply the bias to the measurement data. Normally the time related data such as
CLOCK_BIAS data and CLOCK_DRIFT data should appear in a dedicated TDM Segment,
that is, not mixed with signal data or other data types. The units for CLOCK_BIAS shall be
seconds. The value shall be a double precision value, and may be positive, zero, or negative.
The default value shall be 0.0.

3.5.6.2 CLOCK_DRIFT
"""""""""""""""""""

In general, ground-based clocks in tracking stations are sufficiently stable that a
measurement of the clock drift may not be necessary. However, for spacecraft-to-spacecraft
exchanges, there may be onboard clock drifts that are sufficiently significant that they should
be accounted for in the measurements and calculations. Drift in clocks may also be an
important factor when differenced data is being exchanged. The CLOCK_DRIFT keyword
should be used to adjust timetag measurements by an amount that is a function of time with
respect to a common reference, normally UTC (as opposed to the CLOCK_BIAS, which is
meant to be a constant adjustment). Thus CLOCK_DRIFT could be used to calculate an
interpolated CLOCK_BIAS between two timetags, by multiplying the CLOCK_DRIFT
measurement at the timetag by the number of seconds desired and adding it to the
CLOCK_BIAS. The drift should be calculated as a drift of clock#2 with respect to clock#1,
consistent with the TDM convention for differenced data. Normally the time related data
such as CLOCK_DRIFT data and CLOCK_BIAS data should appear in a dedicated TDM
Segment, that is, not mixed with signal data or other data types. The units for
CLOCK_DRIFT shall be seconds-per-second (s/s). The value shall be a double precision
value, and may be positive, zero, or negative. The default value shall be 0.0.

3.5.7 MEDIA RELATED KEYWORDS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

3.5.7.1 STEC
""""""""""""

The Slant Total Electron Count (STEC) keyword shall be used to convey the line of sight,
one way charged particle delay or total electron count (TEC) at the timetag associated with a
tracking measurement, which is calculated by integrating the electron density along the
propagation path (electrons/m\ :sup:`2`). The charged particles could have several sources, for
example, solar plasma, Earth ionosphere, or the Io plasma torus. The units for the STEC
keyword are Total Electron Count Units (TECU), where 1 TECU = 10\ :sup:`16` electrons/m\ :sup:`2` = 1.661
x 10\ :sup:`-8` mol/m\ :sup:`2` (SI Units). The value shall be a positive double precision value (the TEC
along the satellite line of sight may vary between 1 and 400 TECU; larger values may be
observed during periods of high solar activity). This keyword should appear in its own TDM
Segment with PARTICIPANTs being one spacecraft and one antenna, and a MODE setting
of ‘SEQUENTIAL’. Exchange partners who wish to distinguish between ionospheric and
interplanetary STEC should indicate so in the ICD, and the data must be provided in separate
TDM Segments.

3.5.7.2 TROPO_DRY
"""""""""""""""""

The value associated with the TROPO_DRY keyword shall be the dry zenith delay through
the troposphere measured at the timetag. There should be agreed upon elevation mappings
for the dry component specified in the ICD (e.g., the Niell mapping function developed for
VLBI applications). Tropospheric corrections should be applied by the recipient of the
TDM; the required correction is the value associated with this keyword at the timetag.
Recommended polynomial interpolations (if applicable) should be specified in the ICD. The
units for TROPO_DRY shall be meters (m). The value shall be a non-negative double
precision value (0.0 ≤ TROPO_DRY).

3.5.7.3 TROPO_WET
"""""""""""""""""

The value associated with the TROPO_WET keyword shall be the wet zenith delay through
the troposphere measured at the timetag. There should be agreed upon elevation mappings
for the wet component specified in the ICD (e.g., the Niell mapping function developed for
VLBI applications). Tropospheric corrections should be applied by the recipient of the
TDM; the required correction is the value associated with this keyword at the timetag.
Recommended polynomial interpolations (if applicable) should be specified in the ICD. The
units for TROPO_WET shall be meters (m). The value shall be a non-negative double
precision value (0.0 ≤ TROPO_WET).

3.5.8 METEOROLOGICAL RELATED KEYWORDS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

3.5.8.1 PRESSURE
""""""""""""""""

The value associated with the PRESSURE keyword shall be the atmospheric pressure
observable as measured at the tracking participant, specified in hectopascal (1 hectopascal
(hPa) = 1 millibar). The PRESSURE shall be a double precision value; practically speaking
it is always positive.

3.5.8.2 RHUMIDITY
"""""""""""""""""

The value associated with the RHUMIDITY keyword shall be the relative humidity
observable as measured at the tracking participant, specified in percent. RHUMIDITY shall
be a double precision type value, 0.0 ≤ RHUMIDITY ≤ 100.0.

3.5.8.3 TEMPERATURE
"""""""""""""""""""

The value associated with the TEMPERATURE keyword shall be the temperature
observable as measured at the tracking participant, specified in Kelvin (K). The
TEMPERATURE shall be a positive double precision type value.

3.5.9 MISCELLANEOUS KEYWORDS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

3.5.9.1 COMMENT
"""""""""""""""

The COMMENT keyword is not required. Subsection 4.5 provides full details on usage of
the COMMENT keyword.

3.5.9.2 DATA_START
""""""""""""""""""

The ‘DATA_START’ keyword must be the first keyword in the Data Section of the TDM
Segment, which serves to delimit the Data Section. The keyword shall appear on a line by
itself with no timetags or values. Example: ‘DATA_START’.

3.5.9.3 DATA_STOP
"""""""""""""""""

The ‘DATA_STOP’ keyword must be the last keyword in the Data Section of the TDM
Segment, which serves to delimit the Data Section. The keyword shall appear on a line by
itself with no timetags or values. Example: ‘DATA_STOP’.
.. _kvn_syntax:

4 TRACKING DATA MESSAGE SYNTAX IN KVN
=====================================

.. _kvn_general:

4.1 GENERAL
-----------

The TDM represented in ‘keyword = value’ syntax, abbreviated as KVN, shall observe the
syntax described in 4.2 through 4.5.

.. _tdm_lines:

4.2 TDM LINES
-------------

4.2.1 The TDM shall consist of a set of TDM lines. The TDM line must contain only
printable ASCII characters and blanks. ASCII control characters (such as TAB, etc.) must
not be used, except as indicated below for the termination of the TDM line. A TDM line
must not exceed 254 ASCII characters and spaces (excluding line termination character[s]).

4.2.2 Each TDM line shall be one of the following:

– Header line;

– Metadata Section line;

– Data Section line;

– blank line.

4.2.3 All Header, Metadata Section, and Data Section lines, with exceptions as noted
below, shall use KVN.

4.2.4 Only a single ‘keyword = value’ assignment shall be made on a TDM line.

4.2.5 The following distinctions in KVN syntax shall apply for TDM lines:

a) TDM lines in the Header and Metadata Section shall consist of a keyword, followed
   by an equals sign ‘=’, followed by a single value assignment. Before and after the
   equals sign, blank characters (white space) may be added, but shall not be required.

b) TDM lines in the Data Section shall consist of a keyword, followed by an equals sign
   ‘=’, followed by a value that consists of two primary elements (essentially an ordered
   pair): a timetag and the measurement or calculation associated with that timetag
   (either without the other is unusable for tracking purposes). Before and after the
   equals sign, blank characters (white space) may be added. The timetag and
   measurement/calculation in the value must be separated by at least one blank
   character (white space).

c) The keywords COMMENT, META_START, META_STOP, DATA_START, and
   DATA_STOP are exceptions to the KVN syntax.

4.2.6 Keywords must be uppercase and must not contain blanks.

4.2.7 Any white space immediately preceding or following the keyword shall not be
significant.

4.2.8 Any white space immediately preceding or following the equals sign ‘=’ shall not be
significant.

4.2.9 Any white space immediately preceding the end of line shall not be significant.

4.2.10 Blank lines may be used at any position within the TDM.

4.2.11 TDM lines shall be terminated by a single Carriage Return or a single Line Feed or a
Carriage Return/Line Feed pair or a Line Feed/Carriage Return pair.

.. _tdm_values:

4.3 TDM VALUES
--------------

4.3.1 A non-empty value field must be specified for each keyword provided.

4.3.2 Integer values shall consist of a sequence of decimal digits with an optional leading
sign (‘+’ or ‘-’). If the sign is omitted, ‘+’ shall be assumed. Leading zeros may be used.
The range of values that may be expressed as an integer is:

−2147483648 ≤ x ≤ +2147483647 (i.e., :math:`-2^{31} \le x \le 2^{31}-1`).

4.3.3 Non-integer numeric values may be expressed in either fixed-point or floating-point
notation. Both representations may be used within a TDM.

4.3.4 Non-integer numeric values expressed in fixed-point notation shall consist of a
sequence of decimal digits separated by a period as a decimal point indicator, with an
optional leading sign (‘+’ or ‘-’). If the sign is omitted, ‘+’ shall be assumed. Leading and
trailing zeros may be used. At least one digit shall be used before and after a decimal point.
The number of digits shall be 16 or fewer.

4.3.5 Non-integer numeric values expressed in floating-point notation shall consist of a
sign, a mantissa, an alphabetic character indicating the division between the mantissa and
exponent, and an exponent, constructed according to the following rules:

a) The sign may be ‘+’ or ‘-’. If the sign is omitted, ‘+’ shall be assumed.

b) The mantissa must be a string of no more than 16 decimal digits with a decimal point
   ‘.’ in the second position of the ASCII string, separating the integer portion of the
   mantissa from the fractional part of the mantissa.

c) The character used to denote exponentiation shall be ‘E’ or ‘e’. If the character
   indicating the exponent and the following exponent are omitted, an exponent value of
   zero shall be assumed (essentially yielding a fixed-point value).

d) The exponent must be an integer, and may have either a ‘+’ or ‘-’ sign (if the sign is
   omitted, then ‘+’ is assumed).

e) The maximum positive floating-point value is approximately 1.798E+308, with 16
   significant decimal digits precision. The minimum positive floating-point value is
   approximately 4.94E−324, with 16 significant decimal digits precision.

NOTE – These specifications for integer, fixed-point, and floating-point values conform to
the XML specifications for the data types four-byte integer ‘xsd:int’, ‘decimal’,
and ‘double’, respectively (see reference [5]). The specifications for floating-
point values conform to the IEEE 754 double precision type (see reference [6]).
Floating-point numbers in IEEE extended-single or IEEE extended-double
precision may be represented, but do require an ICD between participating
agencies because of their implementation specific attributes. The special values
‘NaN’, ‘-Inf’, ‘+Inf’, and ‘-0’ are not supported in the TDM.

4.3.6 Blanks shall not be permitted within numeric values and time values.

4.3.7 Text value fields may be constructed using mixed case; case shall not be significant.
All uppercase text values are preferred.

4.3.8 In value fields that are text, an underscore shall be equivalent to a single blank.
Individual blanks between non-blank characters shall be retained (shall be significant) but
multiple blanks shall be equivalent to a single blank.

4.3.9 In value fields that represent a timetag or epoch, one of the following two formats
shall be used:

   YYYY-MM-DDThh:mm:ss[.d→d][Z]

or

   YYYY-DDDThh:mm:ss[.d→d][Z]

where ‘YYYY’ is the year, ‘MM’ is the two-digit month, ‘DD’ is the two-digit day, ‘DDD’
is the three-digit day of year, ‘T’ is constant, ‘hh:mm:ss[.d→d]’ is the time in hours, minutes
seconds, and optional fractional seconds; ‘Z’ is an optional time code terminator (the only
permitted value is ‘Z’ for Zulu, i.e., UTC). All fields shall have leading zeros. (See
reference [3], ASCII Time Code A and B.)

4.3.10 There are four types of TDM values that represent a timetag or epoch, as shown in the
applicable tables. The time system for the CREATION_DATE, START_TIME, and
STOP_TIME shall be UTC. The time system for the timetags in the TDM Data Section shall
be determined by the TIME_SYSTEM metadata keyword.

4.3.11 For transmit and receive phase, the value shall be a string representing a real number
that can be any number of digits required to convey the necessary precision. The string must
not contain any alphabetic or special characters.

.. _units_in_the_tdm:

4.4 UNITS IN THE TDM
--------------------

Units are not explicitly displayed in the TDM. The units associated with values in the TDM
are as specified in table 3-5.

.. _comments_in_a_tdm:

4.5 COMMENTS IN A TDM
---------------------

4.5.1 Comments may be used to provide any pertinent information associated with the data
that is not covered via one of the keywords. This additional information is intended to aid in
consistency checks and elaboration where needed. Comments shall not be required for
successful processing of a TDM; that is, comment lines shall be optional.

NOTE – Given that TDMs may consist of large amounts of data, and are generally produced
via automation, using the COMMENT feature of the TDM may have limited
usefulness. On the other hand, a simple utility could be developed to search for
and extract all the comments in a TDM to make them easily reviewable. Existing
built-in utilities (e.g., UNIX ‘grep’) or ‘freeware’ utilities could also be used for
this purpose.

4.5.2 Comment lines, if used, shall only occur:

a) at the beginning of the TDM Header (i.e., between the CCSDS_TDM_VERS
   keyword and the CREATION_DATE keyword, as shown in table 3-2);

b) at the beginning of the TDM Metadata Section (i.e., between the META_START
   keyword and the TRACK_ID keyword, as shown in table 3-3);

c) at the beginning of the TDM Data Section (i.e., between the ‘DATA_START’
   keyword and the first Tracking Data Record).

4.5.3 All comment lines shall begin with the ‘COMMENT’ keyword followed by at least
   one space (note: may also be preceded by spaces). The ‘COMMENT’ keyword must appear
   on every comment line, not just the first comment line. After the keyword, the remainder of
   the line shall be the comment value. White space shall be retained (is significant) in comment
   values.

4.5.4 Conventions for particular comments in the TDM that may be required between any
   two participating agencies should be specified in the ICD.

4.5.5 Descriptions of any ancillary data that cannot be accommodated via keywords in the
   TDM may have to be specified via comments, and should be outlined in the ICD.
.. _tdm_xml:

5 TDM CONTENT/STRUCTURE IN XML
==============================

.. _tdm_xml_schema:

5.1 DISCUSSION—THE TDM/XML SCHEMA
---------------------------------

The TDM/XML schema is available on the SANA Web site. SANA is the registrar for the
protocol registries created under CCSDS.

The TDM XML schema explicitly defines the permitted data elements and values acceptable
for the XML version of the TDM message.

The location of the TDM/XML schema is:

– https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-2.0.0-tdm-2.0.xsd for
  messages with elements not qualified with respect to a namespace;

– https://sanaregistry.org/r/ndmxml_qualified/ndmxml-2.0.0-tdm-2.0.xsd for messages
  with elements qualified with respect to a namespace.

NOTE – Reference [9] subsection 4.3 has more information regarding messages with
elements qualified with respect to a namespace.

Where possible this schema uses simple types and complex types used by the constituent
schemas that make up NDMs (see reference [9]).

.. _tdm_xml_basic_structure:

5.2 TDM/XML BASIC STRUCTURE
---------------------------

5.2.1 Each TDM shall consist of a <header> and a <body>.

5.2.2 The TDM <body> shall consist of one or more <segment> constructs.

5.2.3 Each <segment> shall consist of a <metadata>/<data> pair, as shown in figure 5-1.

.. code-block:: xml

   <header>
   </header>
   <body>
    <segment>
    <metadata>
    </metadata>
    <data>
    </data>
    </segment>
    <segment>
    <metadata>
    </metadata>
    <data>
    </data>
    </segment>
   </body>

.. figure:: ../images/ccsds_books/TDM_XML_Basic_Structure.png
   :alt: TDM XML Basic Structure
   :align: center

   Figure 5-1: TDM XML Basic Structure

5.2.4 XML tags shall be uppercase and correspond with the KVN keywords in section 3 of
this document (uppercase with ‘_’ [the underscore character] as separators). The XML
logical tags related to message structure shall be in lowerCamelCase.

.. _constructing_tdm_xml:

5.3 CONSTRUCTING A TDM/XML INSTANCE
-----------------------------------

5.3.1 OVERVIEW
^^^^^^^^^^^^^^

This subsection provides more detailed instructions for the user on how to create an XML
message based on the ASCII-text KVN-formatted message described in section 3.

5.3.2 XML VERSION
^^^^^^^^^^^^^^^^^

The first line in the instantiation shall specify the XML version:

.. code-block:: xml

   <?xml version="1.0" encoding="UTF-8"?>

This line must appear on the first line of each instantiation, exactly as shown.

5.3.3 BEGINNING THE INSTANTIATION: ROOT DATA ELEMENT
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

5.3.3.1 A TDM instantiation shall be delimited with the <tdm></tdm> root element tags
using the standard attributes documented in reference [10].

5.3.3.2 The XML Schema Instance namespace attribute must appear in the root element tag
of all TDM/XML instantiations, exactly as shown:

   xmlns:xsi = "http://www.w3.org/2001/XMLSchema-instance"

5.3.3.3 For messages with elements qualified with respect to a namespace, the NDM/XML
namespace must next be coded, exactly as shown:

   xmlns:ndm="urn:ccsds:schema:ndmxml"

The value that follows the ‘xmlns:’ in the NDM/XML name space (‘ndm’ in this case) is a
prefix that must be used on every XML tag.

NOTE – This xmlns:ndm setting is only necessary for messages with elements qualified
with respect to a namespace, but it does not hurt anything for it to appear on any
NDM/XML instantiation.

5.3.3.4 If it is desired to validate an instantiation against the CCSDS Web-based schema,
the xsi:noNamespaceSchemaLocation attribute must be coded as a single string of non-blank
characters, with no line breaks, exactly as shown:

– xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/n
  dmxml-2.0.0-master-2.0.xsd" for messages with elements not qualified with respect
  to a namespace;

– xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_qualified/ndm
  xml-2.0.0-master-2.0.xsd" for messages with elements qualified with respect to a
  namespace.

NOTE – The length of the value associated with the xsi:noNamespaceSchemaLocation
attribute can cause the string to wrap to a new line; however, the string itself
contains no breaks.

5.3.3.5 For use in a local operations environment, the schema set may be downloaded from
the SANA Web site to a local server that meets local requirements for operations robustness.

5.3.3.6 If a local version is used, the value associated with the xsi:noNamespaceSchemaLocation
attribute must be changed to a URL that is accessible to the local server.

5.3.3.7 The final attributes of the <tdm> tag shall be ‘id’ and ‘version’.

5.3.3.7.1 The ‘id’ attribute shall be ‘id="CCSDS_TDM_VERS"’.

5.3.3.7.2 The ‘version’ attribute shall be ‘version="2.0"’.

NOTES

1 The following example root element tag for a TDM instantiation combines all the
directions in the preceding several subsections for messages with elements not
qualified with respect to a namespace:

.. code-block:: xml

   <?xml version="1.0" encoding="UTF-8"?>
   <tdm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xsi:noNamespaceSchemaLocation=
   "https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-2.0.0-master-2.0.xsd"
   id="CCSDS_TDM_VERS" version="2.0">

2 The following example root element tag for a TDM instantiation combines all the
directions in the preceding several subsections for messages with elements qualified
with respect to a namespace:

.. code-block:: xml

   <?xml version="1.0" encoding="UTF-8"?>
   <tdm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xmlns:ndm="urn:ccsds:schema:ndmxml"
   xsi:noNamespaceSchemaLocation=
   "https://sanaregistry.org/r/ndmxml_qualified/ndmxml-2.0.0-master-2.0.xsd"
   id="CCSDS_TDM_VERS" version="2.0">

5.3.4 THE TDM/XML HEADER SECTION
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

5.3.4.1 The TDM header shall have a standard header format, with tags <header> and
</header>.

5.3.4.2 Immediately following the <header> tag, the message may have any number of
<COMMENT></COMMENT> tag pairs.

5.3.4.3 The standard TDM header shall contain the following element tags:

a) <CREATION_DATE>;

b) <ORIGINATOR>.

NOTE – The rules for these keywords are specified in 3.2. The header would look like
this:

.. code-block:: xml

    <header>
    <COMMENT>Some comment string, which is not required.</COMMENT>
    <CREATION_DATE>2010-03-12T22:31:12.000</CREATION_DATE>
    <ORIGINATOR>NASA</ORIGINATOR>
    </header>

5.3.4.4 An optional <MESSAGE_ID> may be used in the TDM header after the
<ORIGINATOR> keyword.

5.3.5 THE TDM/XML BODY SECTION
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

5.3.5.1 After coding the <header>, the instantiation must include a <body></body> tag pair.

5.3.5.2 The TDM <body> shall consist of one or more <segment> constructs (see figure 5-1).

5.3.5.3 Each <segment> shall consist of a <metadata> section and a <data> section.

5.3.5.4 The keywords in the <metadata> and <data> sections shall be those specified in
table 3-3 and table 3-5, respectively.

5.3.5.5 Tags for TDM keywords shall be all uppercase.

5.3.5.6 TDM/XML keywords that do not correspond directly to a KVN keyword shall be in
‘lowerCamelCase’.

5.3.6 THE TDM/XML METADATA SECTION
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

5.3.6.1 Immediately following the <metadata> tag, the message may have any number of
<COMMENT></COMMENT> tag pairs.

5.3.6.2 Between the <metadata> and </metadata> tags, the keywords shall be those
specified in table 3-3.

5.3.7 THE TDM/XML DATA SECTION
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

5.3.7.1 Each data section shall follow the corresponding metadata section and shall be set
off by the <data></data> tag combination.

5.3.7.2 Immediately following the <data> tag, the message may have any number of
<COMMENT></COMMENT> tag pairs.

5.3.7.3 Between the <data> and </data> tags, the keywords shall be those specified in table 3-5.

5.3.8 SPECIAL TDM/XML TAGS
^^^^^^^^^^^^^^^^^^^^^^^^^^

NOTE – In addition to the TDM keywords specified in section 3, there is a special tag
associated with the TDM body as described in the next subsection.

5.3.8.1 The <observation> tag shall be used to encapsulate the keywords associated with
one of the tracking data types in the TDM.

5.3.8.2 The <observation> tag shall consist of two subcomponents:

a) the time tag (<EPOCH> tag); and

b) one specific data type (e.g., <RECEIVE_FREQ>).

NOTE – Thus a received frequency observation would appear in an NDM/XML TDM as
follows:

.. code-block:: xml

   <observation>
    <EPOCH>2008-200T12:34:56.789</EPOCH>
    <RECEIVE_FREQ>8415000000</RECEIVE_FREQ>
   </observation>

5.3.9 UNITS IN THE TDM/XML
^^^^^^^^^^^^^^^^^^^^^^^^^^

The units associated with values in the TDM/XML shall be the same units used in the KVN-
formatted TDM and are as specified in table 3-5.

5.4 DISCUSSION—TDM/XML EXAMPLE
------------------------------

Figure E-21 provides a sample of a TDM in XML format.
.. _annex_a:

ANNEX A
=======

IMPLEMENTATION CONFORMANCE STATEMENT (ICS)
==========================================

(NORMATIVE)

A1 INTRODUCTION
---------------

A1.1 OVERVIEW
^^^^^^^^^^^^^

This annex provides the Implementation Conformance Statement (ICS) Requirements List
(RL) for an implementation of Tracking Data Message (CCSDS 503.0-B-2). The ICS for an
implementation is generated by completing the RL in accordance with the instructions
below. An implementation shall satisfy the mandatory conformance requirements referenced
in the RL. (For further information on Implementation Conformance Statements, see
reference [F6].)

The RL in this annex is blank. An implementation’s completed RL is called the ICS. The ICS
states which capabilities and options have been implemented. The following can use the ICS:

– the implementer, as a checklist to reduce the risk of failure to conform to the standard
  through oversight;

– a supplier or potential acquirer of the implementation, as a detailed indication of the
  capabilities of the implementation, stated relative to the common basis for
  understanding provided by the standard ICS proforma;

– a user or potential user of the implementation, as a basis for initially checking the
  possibility of interworking with another implementation (it should be noted that,
  while interworking can never be guaranteed, failure to interwork can often be
  predicted from incompatible ICSes);

– a tester, as the basis for selecting appropriate tests against which to assess the claim
  for conformance of the implementation.

A1.2 ABBREVIATIONS AND CONVENTIONS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The RL consists of information in tabular form. The status of features is indicated using the
abbreviations and conventions described below.

Item Column
"""""""""""

The item column contains sequential numbers for items in the table.

Feature Column
""""""""""""""

The feature column contains a brief descriptive name for a feature. It implicitly means ‘Is
this feature supported by the implementation?’

NOTE – The features itemized in the RL are elements of a TDM. Therefore support for a
mandatory feature indicates that generated messages will include that feature,
and support for an optional feature indicates that generated messages can include
that feature.

Keyword Column
""""""""""""""

The keyword column contains, where applicable, the TDM keyword associated with the
feature.

Reference Column
""""""""""""""""

The reference column indicates the relevant subsection or table in Tracking Data Message
(CCSDS 503.0-B-2) (this document).

Status Column
"""""""""""""

The status column uses the following notations:

M mandatory.

O optional.

Support Column Symbols
""""""""""""""""""""""

The support column is to be used by the implementer to state whether a feature is supported
by entering Y, N, or N/A, indicating:

Y Yes, supported by the implementation.

N No, not supported by the implementation.

N/A Not applicable.

A1.3 INSTRUCTIONS FOR COMPLETING THE RL
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An implementer shows the extent of compliance to the Recommended Standard by
completing the RL; that is, the state of compliance with all mandatory requirements and the
options supported are shown. The resulting completed RL is called an ICS. The implementer
shall complete the RL by entering appropriate responses in the support or values-supported
column, using the notation described in A1.2. If a conditional requirement is inapplicable,
N/A should be used. If a mandatory requirement is not satisfied, exception information must
be supplied by entering a reference Xi, where i is a unique identifier, to an accompanying
rationale for the noncompliance.

A2 ICS PROFORMA FOR TRACKING DATA MESSAGE
-----------------------------------------

A2.1 GENERAL INFORMATION
^^^^^^^^^^^^^^^^^^^^^^^^

A2.1.1 Identification of ICS
""""""""""""""""""""""""""""

.. list-table::
   :widths: 50 50
   :header-rows: 0

   * - Date of Statement (DD/MM/YYYY)
     -
   * - ICS serial number
     -
   * - System Conformance statement cross-reference
     -

A2.1.2 Identification of Implementation Under Test (IUT)
""""""""""""""""""""""""""""""""""""""""""""""""""""""

.. list-table::
   :widths: 50 50
   :header-rows: 0

   * - Implementation name
     -
   * - Implementation version
     -
   * - Special Configuration
     -
   * - Other Information
     -

A2.1.3 Identification of Supplier
"""""""""""""""""""""""""""""""""

.. list-table::
   :widths: 50 50
   :header-rows: 0

   * - Supplier
     -
   * - Contact Point for Queries
     -
   * - Implementation Name(s) and Versions
     -
   * - Other information necessary for full identification, for example, name(s) and version(s) for machines and/or operating systems; System Name(s)
     -

A2.1.4 Document Version
"""""""""""""""""""""""

.. list-table::
   :widths: 50 50
   :header-rows: 0

   * - CCSDS 503.0-B-2 Document Version
     -
   * - Have any exceptions been required?
       (Note – A YES answer means that the implementation does not conform to the Recommended Standard. Non-supported mandatory capabilities are to be identified in the ICS, with an explanation of why the implementation is non-conforming.)
     - Yes \_\_\_\_\_ No\_\_\_\_\_

A2.1.5 Requirements List
^^^^^^^^^^^^^^^^^^^^^^^^

.. list-table:: Requirements List
   :widths: 5 25 25 15 10 20
   :header-rows: 1

   * - Seq #
     - Feature
     - Keyword
     - Reference (Blue Book)
     - Status
     - Support
   * - 1
     - TDM Header
     - N/A
     - Table 3-2
     - M
     -
   * - 2
     - TDM version
     - CCSDS_TDM_VERS
     - Table 3-2
     - M
     -
   * - 3
     - Comment
     - COMMENT
     - Table 3-2
     - O
     -
   * - 4
     - Message creation date/time
     - CREATION_DATE
     - Table 3-2
     - M
     -
   * - 5
     - Message originator
     - ORIGINATOR
     - Table 3-2
     - M
     -
   * - 6
     - Message ID
     - MESSAGE_ID
     - Table 3-2
     - O
     -
   * - 7
     - TDM Metadata
     - META_START
     - Table 3-3
     - M
     -
   * - 8
     - Comment
     - COMMENT
     - Table 3-3
     - O
     -
   * - 9
     - Track identifier
     - TRACK_ID
     - Table 3-3
     - O
     -
   * - 10
     - Specifies data types in data section
     - DATA_TYPES
     - Table 3-3
     - O
     -
   * - 11
     - Specifies time system relevant to timetags
     - TIME_SYSTEM
     - Table 3-3
     - M
     -
   * - 12
     - Start time of data
     - START_TIME
     - Table 3-3
     - O
     -
   * - 13
     - Stop time of data
     - STOP_TIME
     - Table 3-3
     - O
     -
   * - 14
     - Participants in the tracking session
     - PARTICIPANT_n
     - Table 3-3
     - M
     -
   * - 15
     - Mode of the tracking session
     - MODE
     - Table 3-3
     - O
     -
   * - 16
     - Signal path in the tracking session
     - PATH, PATH_1, PATH_2
     - Table 3-3
     - O
     -
   * - 17
     - Name of the ephemeris file used, if any.
     - EPHEMERIS_NAME_n
     - Table 3-3
     - O
     -
   * - 18
     - Frequency band of the transmitted data
     - TRANSMIT_BAND
     - Table 3-3
     - O
     -
   * - 19
     - Frequency band of the received data
     - RECEIVE_BAND
     - Table 3-3
     - O
     -
   * - 20
     - Numerator of the turnaround ratio
     - TURNAROUND_NUMERATOR
     - Table 3-3
     - O
     -
   * - 21
     - Denominator of the turnaround ratio
     - TURNAROUND_DENOMINATOR
     - Table 3-3
     - O
     -
   * - 22
     - Specifies whether data timetag is transmitted or received
     - TIMETAG_REF
     - Table 3-3
     - O
     -
   * - 23
     - Data compression rate
     - INTEGRATION_INTERVAL
     - Table 3-3
     - O
     -
   * - 24
     - Reference point of the timetag
     - INTEGRATION_REF
     - Table 3-3
     - O
     -
   * - 25
     - Specifies a base frequency to which frequency data is referenced.
     - FREQ_OFFSET
     - Table 3-3
     - O
     -
   * - 26
     - Specifies the ranging method
     - RANGE_MODE
     - Table 3-3
     - O
     -
   * - 27
     - Specifies the ranging modulus
     - RANGE_MODULUS
     - Table 3-3
     - O
     -
   * - 28
     - Specifies the units for ranging data
     - RANGE_UNITS
     - Table 3-3
     - O
     -
   * - 29
     - Specifies the angle type for angle data
     - ANGLE_TYPE
     - Table 3-3
     - O
     -
   * - 30
     - Specifies the reference frame for specific angle types
     - REFERENCE_FRAME
     - Table 3-3
     - O
     -
   * - 31
     - Specifies the interpolation method recommended for phase count data
     - INTERPOLATION
     - Table 3-3
     - O
     -
   * - 32
     - Specifies the degree of the interpolating polynomial for phase count data
     - INTERPOLATION_DEGREE
     - Table 3-3
     - O
     -
   * - 33
     - Specifies correction factors necessary to reconstruct a Doppler counter measurement
     - DOPPLER_COUNT_BIAS, DOPPLER_COUNT_SCALE, DOPPLER_COUNT_ROLLOVER
     - Table 3-3
     - O
     -
   * - 34
     - Specifies a fixed delay time applicable to transmitted data
     - TRANSMIT_DELAY_n
     - Table 3-3
     - O
     -
   * - 35
     - Specifies a fixed delay time applicable to received data
     - RECEIVE_DELAY_n
     - Table 3-3
     - O
     -
   * - 36
     - Indicates the data quality
     - DATA_QUALITY
     - Table 3-3
     - O
     -
   * - 37
     - Specifies a correction value to be added to each data point
     - CORRECTION_ANGLE_1, CORRECTION_ANGLE_2, CORRECTION_DOPPLER, CORRECTION_MAG, CORRECTION_RANGE, CORRECTION_RCS, CORRECTION_RECEIVE, CORRECTION_TRANSMIT, CORRECTION_ABERRATION_YEARLY, EARLY, CORRECTION_ABERRATION_DIURNAL
     - Table 3-3
     - O
     -
   * - 38
     - Specifies whether corrections have been applied, or have not
     - CORRECTIONS_APPLIED
     - Table 3-3
     - O
     -
   * - 39
     - End of TDM Metadata
     - META_STOP
     - Table 3-3
     - M
     -
   * - 40
     - TDM Data
     - DATA_START
     - Table 3-5
     - M
     -
   * - 41
     - Comment
     - COMMENT
     - Table 3-5
     - O
     -
   * - 42
     - Angle related data
     - ANGLE_1, ANGLE_2
     - Table 3-5
     - O
     -
   * - 43
     - Carrier signal related data
     - CARRIER_POWER, PC_N0
     - Table 3-5
     - O
     -
   * - 44
     - Clock related data
     - CLOCK_BIAS, CLOCK_DRIFT
     - Table 3-5
     - O
     -
   * - 45
     - Doppler data
     - DOPPLER_INSTANTANEOUS, DOPPLER_INTEGRATED, DOPPLER_COUNT
     - Table 3-5
     - O
     -
   * - 46
     - Media related data
     - STEC, TROPO_DRY, TROPO_WET
     - Table 3-5
     - O
     -
   * - 47
     - Meteorological data
     - PRESSURE, RHUMIDITY, TEMPERATURE
     - Table 3-5
     - O
     -
   * - 48
     - Optical/radar related data
     - MAG, RCS
     - Table 3-5
     - O
     -
   * - 49
     - Range related data
     - RANGE, PR_N0
     - Table 3-5
     - O
     -
   * - 50
     - Receive related data
     - RECEIVE_FREQ_n, RECEIVE_FREQ, RECEIVE_PHASE_CT
     - Table 3-5
     - O
     -
   * - 51
     - Transmit related data
     - TRANSMIT_FREQ_n, TRANSMIT_FREQ_RATE_n, TRANSMIT_PHASE_CT_n
     - Table 3-5
     - O
     -
   * - 52
     - VLBI related data
     - DOR, VLBI_DELAY
     - Table 3-5
     - O
     -
   * - 53
     - End of TDM Data
     - DATA_STOP
     - Table 3-5
     - M
     -

.. _annex_b:

ANNEX B
=======

VALUES FOR TIME_SYSTEM AND REFERENCE_FRAME
==========================================

(NORMATIVE)

B1 GENERAL
----------

Values for the TIME_SYSTEM and REFERENCE_FRAME keywords should come from the
SANA Registry. If exchange partners wish to use different settings, they should be documented
in the ICD.

B2 TIME_SYSTEM METADATA KEYWORD
-------------------------------

The value associated with this keyword must be selected from the SANA Time Systems
Registry (https://sanaregistry.org/r/time_systems). Customary values are shown as examples in
table 3-3.

B3 REFERENCE_FRAME KEYWORD
--------------------------

The value associated with this keyword must be selected from the SANA Celestial Body
Reference Frames Registry (https://sanaregistry.org/r/celestial_body_reference_frames.)
Customary values are shown as examples in table 3-3.

.. _annex_c:

ANNEX C
=======

SECURITY, SANA, AND PATENT CONSIDERATIONS
=========================================

(INFORMATIVE)

C1 SECURITY CONSIDERATIONS
--------------------------

C1.1 ANALYSIS OF SECURITY CONSIDERATIONS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

This annex subsection presents the results of an analysis of security considerations applied to
the technologies specified in this Recommended Standard.

C1.2 CONSEQUENCES OF NOT APPLYING SECURITY TO THE TECHNOLOGY
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The consequences of not applying security to the systems and networks on which this
Recommended Standard is implemented could include potential loss, corruption, and theft of
data. Because these messages are used in spacecraft orbit determination analyses, the
consequences of not applying security to the systems and networks on which this
Recommended Standard is implemented could include compromise or loss of the mission if
malicious tampering of a particularly severe nature occurs.

C1.3 POTENTIAL THREATS AND ATTACK SCENARIOS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Potential threats or attack scenarios include, but are not limited to, (a) unauthorized access to
the programs/processes that generate and interpret the messages, (b) unauthorized access to
the messages during transmission between exchange partners, and (c) modification of the
messages between partners. Protection from unauthorized access during transmission is
especially important if the mission utilizes open ground networks, such as the Internet, to
provide ground-station connectivity for the exchange of data formatted in compliance with
this Recommended Standard. It is strongly recommended that potential threats or attack
scenarios applicable to the systems and networks on which this Recommended Standard is
implemented be addressed by the management of those systems and networks.

C1.4 DATA PRIVACY
^^^^^^^^^^^^^^^^^

Privacy of data formatted in compliance with the specifications of this Recommended
Standard should be assured by the systems and networks on which this Recommended
Standard is implemented.

C1.5 DATA INTEGRITY
^^^^^^^^^^^^^^^^^^^

Integrity of data formatted in compliance with the specifications of this Recommended
Standard should be assured by the systems and networks on which this Recommended
Standard is implemented.

C1.6 AUTHENTICATION OF COMMUNICATING ENTITIES
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Authentication of communicating entities involved in the transport of data in compliance
with the specifications of this Recommended Standard should be provided by the systems
and networks on which this Recommended Standard is implemented.

C1.7 DATA TRANSFER BETWEEN COMMUNICATING ENTITIES
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The transfer of data formatted in compliance with this Recommended Standard between
communicating entities should be accomplished via secure mechanisms approved by the
Information Technology Security functionaries of exchange participants.

C1.8 CONTROL OF ACCESS TO RESOURCES
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Control of access to resources should be managed by the systems upon which originator
formatting and recipient processing are performed.

C1.9 AUDITING OF RESOURCE USAGE
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Auditing of resource usage should be handled by the management of systems and networks
on which this Recommended Standard is implemented.

C1.10 UNAUTHORIZED ACCESS
^^^^^^^^^^^^^^^^^^^^^^^^^

Unauthorized access to the programs/processes that generate and interpret the messages
should be prohibited in order to minimize potential threats and attack scenarios.

C1.11 DATA SECURITY IMPLEMENTATION SPECIFICS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Specific information-security interoperability provisions that may apply between agencies
and other independent users involved in an exchange of data formatted in compliance with
this Recommended Standard should be specified in an ICD.

C2 SANA CONSIDERATIONS
----------------------

The following TDM-related items will be registered with the SANA Operator. The registration
rule for new entries in the registry is the approval of new requests by the CCSDS Area or
Working Group responsible for the maintenance of the TDM at the time of the request. New
requests for this registry should be sent to the SANA (mailto:info@sanaregistry.org).

– the TDM XML schema;

– values for the TIME_SYSTEM keyword in https://sanaregistry.org/r/time_systems
  (reference [12]);

– values for the REFERENCE_FRAME keyword in
  https://sanaregistry.org/r/celestial_body_reference_frames (reference [13]); and

– values for the ORIGINATOR keyword in
  https://sanaregistry.org/r/organizations/organizations.html (reference [11]). The
  CCSDS Navigation Working Group has no purview over the contents of this registry.
  Suggestions should be sent to the SANA Operator at info@sanaregistry.org.

C3 PATENT CONSIDERATIONS
------------------------

The recommendations of this document have no patent issues.

.. _annex_d:

ANNEX D
=======

ITEMS FOR AN INTERFACE CONTROL DOCUMENT
=======================================

(INFORMATIVE)

In several places in this document there are references to items which should be specified in
an Interface Control Document (ICD) between agencies participating in an exchange of
tracking data, if they are applicable to the particular exchange. The ICD should be jointly
produced by both Agencies participating in a cross-support activity involving the collection,
analysis, and transfer of tracking data. This section compiles those items into a single
location.

The greater the amount of material specified via ICD, the lesser the utility/benefit of the
TDM (custom programming may be required to tailor software for each ICD). It is
suggested to avoid a large number of items specified via ICD, to ensure full utility/benefit of
the TDM.

For example, although turnaround ratios may not change frequently, having a TDM producer
include the turnaround keywords TURNAROUND_NUMERATOR and
TURNAROUND_DENOMINATOR in the TDM will increase the level of automation
possible in an exchange partner’s TDM reader.

From an implementation standpoint, it is probable that many of the items that need to be
negotiated via ICD will be introduced into the system that processes tracking data via one or
more configuration files that specify the settings of specific, related parameters that will be
used during the tracking session, for example, the value of the turnaround ratio to be used for
the tracking data. This may vary between exchange participants. Different versions of
programs could be used to prepare the tracking data where these parameters differ; however,
a more efficient design would be to have a single program that is configured based on
tracking-pass-specific information. It seems likely that there may be at least two
configuration files necessary, one which contains Agency-specific parameters that do not
change between tracking passes, and one which contains spacecraft/mission-specific
parameters that could change with every tracking pass.

Another thought on ICDs is that it might be feasible for participating agencies to have a
generic baseline ICD (‘standard service provider ICD’) that specifies mission/spacecraft-
independent entities on the interface, for example, those associated with the agency’s ground
antennas (axis offsets, station locations, side motions, reference frame, epoch, supported
frequency bands, etc.). Then smaller ICDs could be used for the mission/spacecraft-specific
arrangements.

The following table lists the items that should be covered in an ICD, along with where they
are discussed in the text:

.. list-table:: Items for an Interface Control Document
   :widths: 10 70 20
   :header-rows: 1

   * - Item
     - Description
     - Section
   * - 1.
     - Definition of accuracy requirements pertaining to any particular TDM.
     - 1.2.3
   * - 2.
     - Method of exchanging TDMs (e.g., post-processed SFTP, real-time stream, etc.).
     - 1.2.4, 3.1.7
   * - 3.
     - Whether the KVN or XML format of the TDM will be exchanged.
     - 2.2.3
   * - 4.
     - Frequency of exchange and special types of exchange.
     - 2.2.6
   * - 5.
     - TDM file naming conventions.
     - 3.1.6
   * - 6.
     - Specific TDM version number(s) that will be exchanged.
     - 3.2.5
   * - 7.
     - Antenna geometry, if not accommodated by built-in values of ‘ANGLE_TYPE’ keyword.
     - table 3-3
   * - 8.
     - The list of eligible names that is used for PARTICIPANT_n keywords.
     - table 3-3, 3.3.1.10
   * - 9.
     - Definitions of ‘RAW’, ‘VALIDATED’, and ‘DEGRADED’ as they apply to data quality for a particular exchange (DATA_QUALITY keyword).
     - table 3-3
   * - 10.
     - The range of frequencies associated with each value of the ‘TRANSMIT_BAND’ and ‘RECEIVE_BAND’ metadata keywords.
     - table 3-3
   * - 11.
     - If more than five participants are necessary, special arrangements are necessary.
     - 3.3.1.11, 3.3.2.4.4
   * - 12.
     - The methods used to extrapolate the measurements to other antennas when all the data in a TDM Segment is media related or weather related and the observable may be relative to a reference location within the tracking complex.
     - 3.3.2.7.2
   * - 13.
     - Complete description of the station locations and characteristics.
     - 3.4.13
   * - 14.
     - Whether TRANSMIT_DELAY and RECEIVE_DELAY are processed by the producer or the consumer of the tracking data.
     - 3.4.15.2
   * - 15.
     - Special sort orders that may be required by the producer or recipient.
     - 3.4.10, 3.5.4.1
   * - 16.
     - Spin correction arrangements (who will do the correction, the agency providing the tracking or the agency that operates the spacecraft).
     - 3.4.15.5
   * - 17.
     - Correction algorithms that are more complex than a simple scalar value.
     - 3.4.15.6
   * - 18.
     - Standard corrections that will (or will not) be applied to the data (e.g., tropospheric, meteorological, media, transponder, etc.), miscellaneous corrections.
     - 3.4.15.7
   * - 19.
     - Definition of the range unit, if it is not kilometers or seconds.
     - 3.5.2.7, table 3-3
   * - 20.
     - Equation for calculation of four-way Doppler shift, if applicable.
     - 3.5.2.8.5
   * - 21.
     - Transponder turnaround ratios necessary to calculate predicted downlink frequency and the Doppler measurement; also includes cases such as dual uplink where a ‘beacon’ or ‘pilot’ frequency is used (e.g., TDRS, DRTS, COMETS).
     - 3.5.2.8.4, 3.5.2.9, table 3-3
   * - 22.
     - Whether or not it is necessary to distinguish the separate Slant Total Electron Count contributions between ionospheric and interplanetary STEC.
     - 3.5.7.1
   * - 23.
     - Elevation mapping function for the tropospheric data.
     - 3.5.7.2, 3.5.7.3
   * - 24.
     - Recommended polynomial interpolations for tropospheric data.
     - 3.5.7.2, 3.5.7.3
   * - 25.
     - If non-standard floating-point numbers in extended-single or extended double precision are to be used, then discussion of implementation-specific attributes is required.
     - 4.3.5
   * - 26.
     - Information which must appear in comments for any given TDM exchange.
     - 4.5.4
   * - 27.
     - Description of any ancillary data not already included in the Tracking Data Record definition.
     - 4.5.5
   * - 28.
     - Interagency Information Technology (IT) security requirements in TDMs.
     - annex C
   * - 29.
     - Time systems not shown in annex B.
     - annex B
   * - 30.
     - Reference frames not shown in annex B.
     - annex B
   * - 31.
     - Whether the mean range rate for 2W and/or 3W Doppler is based on the one way light time or two-way light time.
     - 3.5.2.3
   * - 32.
     - Whether the RANGE observable for 2W and/or 3W range is based on the round trip light time, or half the round trip light time.
     - 3.5.2.7
   * - 33.
     - The usage and composition of a tracking data identifier specified by ‘TRACK_ID’ keyword.
     - table 3-3
.. _annex_e:

ANNEX E
=======

EXAMPLE TRACKING DATA MESSAGES
==============================

(INFORMATIVE)

.. code-block:: text
   :caption: Figure E-1: TDM Example: One-Way Data

   CCSDS_TDM_VERS = 2.0
   COMMENT TDM example created by yyyyy-nnnA Nav Team (NASA/JPL)
   COMMENT StarTrek 1-way data, Ka band down
   CREATION_DATE = 2005-160T20:15:00Z
   ORIGINATOR = NASA
   
   META_START
   COMMENT Data quality degraded by antenna pointing problem...
   COMMENT Slightly noisy data
   TIME_SYSTEM = UTC
   PARTICIPANT_1 = DSS-25
   PARTICIPANT_2 = yyyy-nnnA
   MODE = SEQUENTIAL
   PATH = 2,1
   INTEGRATION_INTERVAL = 1
   INTEGRATION_REF = MIDDLE
   FREQ_OFFSET = 0
   TRANSMIT_DELAY_1 = 0.000077
   RECEIVE_DELAY_1 = 0.000077
   DATA_QUALITY = DEGRADED
   META_STOP
   
   DATA_START
   COMMENT TRANSMIT_FREQ_2 is spacecraft reference downlink
   TRANSMIT_FREQ_2 = 2005-159T17:41:00 32023442781.733
   RECEIVE_FREQ_1 = 2005-159T17:41:00 32021034790.7265
   RECEIVE_FREQ_1 = 2005-159T17:41:01 32021034828.8432
   RECEIVE_FREQ_1 = 2005-159T17:41:02 32021034866.9449
   RECEIVE_FREQ_1 = 2005-159T17:41:03 32021034905.0327
   RECEIVE_FREQ_1 = 2005-159T17:41:04 32021034943.0946
   RECEIVE_FREQ_1 = 2005-159T17:41:05 32021034981.2049
   RECEIVE_FREQ_1 = 2005-159T17:41:06 32021035019.2778
   RECEIVE_FREQ_1 = 2005-159T17:41:07 32021035057.3773
   RECEIVE_FREQ_1 = 2005-159T17:41:08 32021035095.4377
   RECEIVE_FREQ_1 = 2005-159T17:41:09 32021035133.5604
   RECEIVE_FREQ_1 = 2005-159T17:41:10 32021035171.5861
   RECEIVE_FREQ_1 = 2005-159T17:41:11 32021035209.6653
   RECEIVE_FREQ_1 = 2005-159T17:41:12 32021035247.7804
   RECEIVE_FREQ_1 = 2005-159T17:41:13 32021035285.8715
   RECEIVE_FREQ_1 = 2005-159T17:41:14 32021035323.8187
   RECEIVE_FREQ_1 = 2005-159T17:41:15 32021035361.9571
   RECEIVE_FREQ_1 = 2005-159T17:41:16 32021035400.0304
   RECEIVE_FREQ_1 = 2005-159T17:41:17 32021035438.0126
   RECEIVE_FREQ_1 = 2005-159T17:41:18 32021035476.1241
   RECEIVE_FREQ_1 = 2005-159T17:41:19 32021035514.1714
   RECEIVE_FREQ_1 = 2005-159T17:41:20 32021035552.2263
   RECEIVE_FREQ_1 = 2005-159T17:41:21 32021035590.2671
   RECEIVE_FREQ_1 = 2005-159T17:41:22 32021035628.304
   RECEIVE_FREQ_1 = 2005-159T17:41:23 32021035666.3579
   RECEIVE_FREQ_1 = 2005-159T17:41:24 32021035704.3745
   RECEIVE_FREQ_1 = 2005-159T17:41:25 32021035742.4425
   RECEIVE_FREQ_1 = 2005-159T17:41:26 32021035780.4974
   RECEIVE_FREQ_1 = 2005-159T17:41:27 32021035818.5158
   RECEIVE_FREQ_1 = 2005-159T17:41:28 32021035856.5721
   RECEIVE_FREQ_1 = 2005-159T17:41:29 32021035894.5601
   DATA_STOP

.. code-block:: text
   :caption: Figure E-2: TDM Example: One-Way Data w/Frequency Offset

   CCSDS_TDM_VERS = 2.0
   COMMENT TDM example created by yyyyy-nnnA Nav Team (NASA/JPL)
   COMMENT StarTrek 1-way data, Ka band down
   CREATION_DATE = 2005-160T20:15:00
   ORIGINATOR = NASA
   
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2005-159T17:41:00
   STOP_TIME = 2005-159T17:41:40
   PARTICIPANT_1 = DSS-25
   PARTICIPANT_2 = yyyy-nnnA
   MODE = SEQUENTIAL
   PATH = 2,1
   INTEGRATION_INTERVAL = 1.0
   INTEGRATION_REF = MIDDLE
   FREQ_OFFSET = 32021035200.0
   TRANSMIT_DELAY_1 = 0.000077
   RECEIVE_DELAY_1 = 0.000077
   DATA_QUALITY = RAW
   META_STOP
   
   DATA_START
   TRANSMIT_FREQ_2 = 2005-159T17:41:00 32023442781.733
   RECEIVE_FREQ_1 = 2005-159T17:41:00 -409.2735
   RECEIVE_FREQ_1 = 2005-159T17:41:01 -371.1568
   RECEIVE_FREQ_1 = 2005-159T17:41:02 -333.0551
   RECEIVE_FREQ_1 = 2005-159T17:41:03 -294.9673
   RECEIVE_FREQ_1 = 2005-159T17:41:04 -256.9054
   RECEIVE_FREQ_1 = 2005-159T17:41:05 -218.7951
   RECEIVE_FREQ_1 = 2005-159T17:41:06 -180.7222
   RECEIVE_FREQ_1 = 2005-159T17:41:07 -142.6227
   RECEIVE_FREQ_1 = 2005-159T17:41:08 -104.5623
   RECEIVE_FREQ_1 = 2005-159T17:41:09 -66.4396
   RECEIVE_FREQ_1 = 2005-159T17:41:10 -28.4139
   RECEIVE_FREQ_1 = 2005-159T17:41:11 9.6653
   RECEIVE_FREQ_1 = 2005-159T17:41:12 47.7804
   RECEIVE_FREQ_1 = 2005-159T17:41:13 85.8715
   RECEIVE_FREQ_1 = 2005-159T17:41:14 123.8187
   RECEIVE_FREQ_1 = 2005-159T17:41:15 161.9571
   RECEIVE_FREQ_1 = 2005-159T17:41:16 200.0304
   RECEIVE_FREQ_1 = 2005-159T17:41:17 238.0126
   RECEIVE_FREQ_1 = 2005-159T17:41:18 276.1241
   RECEIVE_FREQ_1 = 2005-159T17:41:19 314.1714
   RECEIVE_FREQ_1 = 2005-159T17:41:20 352.2263
   RECEIVE_FREQ_1 = 2005-159T17:41:21 390.2671
   RECEIVE_FREQ_1 = 2005-159T17:41:22 428.3040
   RECEIVE_FREQ_1 = 2005-159T17:41:23 466.3579
   RECEIVE_FREQ_1 = 2005-159T17:41:24 504.3745
   RECEIVE_FREQ_1 = 2005-159T17:41:25 542.4425
   RECEIVE_FREQ_1 = 2005-159T17:41:26 580.4974
   RECEIVE_FREQ_1 = 2005-159T17:41:27 618.5158
   RECEIVE_FREQ_1 = 2005-159T17:41:28 656.5721
   RECEIVE_FREQ_1 = 2005-159T17:41:29 694.5601
   RECEIVE_FREQ_1 = 2005-159T17:41:30 732.5939
   RECEIVE_FREQ_1 = 2005-159T17:41:31 770.6275
   RECEIVE_FREQ_1 = 2005-159T17:41:32 808.6377
   RECEIVE_FREQ_1 = 2005-159T17:41:33 846.6657
   RECEIVE_FREQ_1 = 2005-159T17:41:34 884.6911
   RECEIVE_FREQ_1 = 2005-159T17:41:35 922.6890
   RECEIVE_FREQ_1 = 2005-159T17:41:36 960.7083
   RECEIVE_FREQ_1 = 2005-159T17:41:37 998.7493
   RECEIVE_FREQ_1 = 2005-159T17:41:38 1036.7388
   RECEIVE_FREQ_1 = 2005-159T17:41:39 1074.7529
   RECEIVE_FREQ_1 = 2005-159T17:41:40 1112.7732
   DATA_STOP

.. code-block:: text
   :caption: Figure E-3: TDM Example: Two-Way Frequency Data for Doppler Calculation

   CCSDS_TDM_VERS=2.0
   COMMENT TDM example created by yyyyy-nnnA Nav Team (NASA/JPL)
   CREATION_DATE=2005-184T20:15:00
   ORIGINATOR=NASA
   META_START
   TIME_SYSTEM=UTC
   START_TIME=2005-184T11:12:23
   STOP_TIME=2005-184T13:59:43.27
   PARTICIPANT_1=DSS-55
   PARTICIPANT_2=yyyy-nnnA
   MODE=SEQUENTIAL
   PATH=1,2,1
   INTEGRATION_INTERVAL=1.0
   INTEGRATION_REF=MIDDLE
   META_STOP
   DATA_START
   TRANSMIT_FREQ_1=2005-184T11:12:23 7175173383.615373
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:23 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:24 7175173384.017573
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:24 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:25 7175173384.419773
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:25 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:26 7175173384.821973
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:26 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:27 7175173385.224173
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:27 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:28 7175173385.626373
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:28 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:29 7175173386.028573
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:29 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:30 7175173386.430773
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:30 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:31 7175173386.832973
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:31 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:32 7175173387.235173
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:32 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:33 7175173387.637373
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:33 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:34 7175173388.039573
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:34 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:35 7175173388.441773
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:35 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:36 7175173388.843973
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:36 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:37 7175173389.246173
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:37 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:38 7175173389.648373
   TRANSMIT_FREQ_RATE_1=2005-184T11:12:38 0.40220
   TRANSMIT_FREQ_1=2005-184T11:12:39 7175173390.050573
   RECEIVE_FREQ_1=2005-184T13:59:27.27 8429753135.986102
   RECEIVE_FREQ_1=2005-184T13:59:28.27 8429749428.196568
   RECEIVE_FREQ_1=2005-184T13:59:29.27 8429749427.584727
   RECEIVE_FREQ_1=2005-184T13:59:30.27 8429749427.023103
   RECEIVE_FREQ_1=2005-184T13:59:31.27 8429749426.346252
   RECEIVE_FREQ_1=2005-184T13:59:32.27 8429749425.738658
   RECEIVE_FREQ_1=2005-184T13:59:33.27 8429749425.113143
   RECEIVE_FREQ_1=2005-184T13:59:34.27 8429749424.489933
   RECEIVE_FREQ_1=2005-184T13:59:35.27 8429749423.876996
   RECEIVE_FREQ_1=2005-184T13:59:36.27 8429749423.325228
   RECEIVE_FREQ_1=2005-184T13:59:37.27 8429749422.664049
   RECEIVE_FREQ_1=2005-184T13:59:38.27 8429749422.054996
   RECEIVE_FREQ_1=2005-184T13:59:39.27 8429749421.425801
   RECEIVE_FREQ_1=2005-184T13:59:40.27 8429749420.824186
   RECEIVE_FREQ_1=2005-184T13:59:41.27 8429749420.204178
   RECEIVE_FREQ_1=2005-184T13:59:42.27 8429749419.596043
   RECEIVE_FREQ_1=2005-184T13:59:43.27 8429749418.986191
   DATA_STOP

.. code-block:: text
   :caption: Figure E-4: TDM Example: Two-Way Ranging Data Only

   CCSDS_TDM_VERS = 2.0
   COMMENT TDM example created by yyyyy-nnnA Nav Team (NASA/JPL)
   CREATION_DATE = 2005-191T23:00:00
   ORIGINATOR = NASA
   META_START
   COMMENT Range correction applied is range calibration to DSS-24.
   COMMENT Estimated RTLT at begin of pass = 950 seconds
   COMMENT Antenna Z-height correction 0.0545 km applied to uplink signal
   COMMENT Antenna Z-height correction 0.0189 km applied to downlink signal
   TIME_SYSTEM = UTC
   PARTICIPANT_1 = DSS-24
   PARTICIPANT_2 = yyyy-nnnA
   MODE = SEQUENTIAL
   PATH = 1,2,1
   INTEGRATION_REF = START
   RANGE_MODE = COHERENT
   RANGE_MODULUS = 2.0e+26
   RANGE_UNITS = RU
   TRANSMIT_DELAY_1 = 7.7e-5
   RECEIVE_DELAY_1 = 7.7e-5
   CORRECTION_RANGE = 46.7741
   CORRECTIONS_APPLIED = YES
   META_STOP
   DATA_START
   TRANSMIT_FREQ_1 = 2005-191T00:31:51 7180064367.3536
   TRANSMIT_FREQ_RATE_1 = 2005-191T00:31:51 0.59299
   RANGE = 2005-191T00:31:51 39242998.5151986
   PR_N0 = 2005-191T00:31:51 28.52538
   TRANSMIT_FREQ_1 = 2005-191T00:34:48 7180064472.3146
   TRANSMIT_FREQ_RATE_1 = 2005-191T00:34:48 0.59305
   RANGE = 2005-191T00:34:48 61172265.3115234
   PR_N0 = 2005-191T00:34:48 28.39347
   TRANSMIT_FREQ_1 = 2005-191T00:37:45 7180064577.2756
   TRANSMIT_FREQ_RATE_1 = 2005-191T00:37:45 0.59299
   RANGE = 2005-191T00:37:45 15998108.8168328
   PR_N0 = 2005-191T00:37:45 28.16193
   TRANSMIT_FREQ_1 = 2005-191T00:40:42 7180064682.2366
   TRANSMIT_FREQ_RATE_1 = 2005-191T00:40:42 0.59299
   RANGE = 2005-191T00:40:42 37938284.4138008
   PR_N0 = 2005-191T00:40:42 29.44597
   TRANSMIT_FREQ_1 = 2005-191T00:43:39 7180064787.1976
   TRANSMIT_FREQ_RATE_1 = 2005-191T00:43:39 0.60774
   RANGE = 2005-191T00:43:39 59883968.0697146
   PR_N0 = 2005-191T00:43:39 27.44037
   TRANSMIT_FREQ_1 = 2005-191T00:46:36 7180064894.77345
   TRANSMIT_FREQ_RATE_1 = 2005-191T00:46:36 0.60989
   RANGE = 2005-191T00:46:36 14726355.3958799
   PR_N0 = 2005-191T00:46:36 27.30462
   TRANSMIT_FREQ_1 = 2005-191T00:49:33 7180065002.72044
   TRANSMIT_FREQ_RATE_1 = 2005-191T00:49:33 0.60989
   RANGE = 2005-191T00:49:33 36683224.3750253
   PR_N0 = 2005-191T00:49:33 28.32537
   TRANSMIT_FREQ_1 = 2005-191T00:52:30 7180065110.66743
   TRANSMIT_FREQ_RATE_1 = 2005-191T00:52:30 0.60983
   RANGE = 2005-191T00:52:30 58645699.4734682
   PR_N0 = 2005-191T00:52:30 29.06158
   TRANSMIT_FREQ_1 = 2005-191T00:55:27 7180065218.61442
   TRANSMIT_FREQ_RATE_1 = 2005-191T00:55:27 0.60989
   RANGE = 2005-191T00:55:27 13504948.3585422
   PR_N0 = 2005-191T00:55:27 27.29589
   TRANSMIT_FREQ_1 = 2005-191T00:58:24 7180065326.56141
   TRANSMIT_FREQ_RATE_1 = 2005-191T00:58:24 0.62085
   RANGE = 2005-191T00:58:24 35478729.4012973
   PR_N0 = 2005-191T00:58:24 30.48199
   TRANSMIT_FREQ_1 = 2005-191T01:01:21 7180065436.45167
   RANGE = 2005-191T01:01:21 57458219.0681689
   PR_N0 = 2005-191T01:01:21 27.15509
   DATA_STOP

.. code-block:: text
   :caption: Figure E-5: TDM Example: Three-Way Frequency Data

   CCSDS_TDM_VERS = 2.0
   COMMENT TDM example created by yyyyy-nnnA Nav Team (NASA/JPL)
   CREATION_DATE = 2005-184T20:15:00
   ORIGINATOR = NASA
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2005-184T11:12:23
   STOP_TIME = 2005-184T13:59:40.27
   PARTICIPANT_1 = DSS-55
   PARTICIPANT_2 = yyyy-nnnA
   PARTICIPANT_3 = DSS-15
   MODE = SEQUENTIAL
   PATH = 1,2,3
   INTEGRATION_INTERVAL = 1.0
   INTEGRATION_REF = MIDDLE
   META_STOP
   DATA_START
   TRANSMIT_FREQ_1 = 2005-184T11:12:23 7175173383.615373
   TRANSMIT_FREQ_RATE_1 = 2005-184T11:12:23 0.40220
   RECEIVE_FREQ_3 = 2005-184T13:59:27.27 8429753135.986102
   TRANSMIT_FREQ_1 = 2005-184T11:12:24 7175173384.017573
   TRANSMIT_FREQ_RATE_1 = 2005-184T11:12:24 0.40220
   RECEIVE_FREQ_3 = 2005-184T13:59:28.27 8429749428.196568
   TRANSMIT_FREQ_1 = 2005-184T11:12:25 7175173384.419773
   TRANSMIT_FREQ_RATE_1 = 2005-184T11:12:25 0.40220
   RECEIVE_FREQ_3 = 2005-184T13:59:29.27 8429749427.584727
   TRANSMIT_FREQ_1 = 2005-184T11:12:26 7175173384.821973
   TRANSMIT_FREQ_RATE_1 = 2005-184T11:12:26 0.40220
   RECEIVE_FREQ_3 = 2005-184T13:59:30.27 8429749427.023103
   TRANSMIT_FREQ_1 = 2005-184T11:12:27 7175173385.224173
   TRANSMIT_FREQ_RATE_1 = 2005-184T11:12:27 0.40220
   RECEIVE_FREQ_3 = 2005-184T13:59:31.27 8429749426.346252
   TRANSMIT_FREQ_1 = 2005-184T11:12:28 7175173385.626373
   TRANSMIT_FREQ_RATE_1 = 2005-184T11:12:28 0.40220
   RECEIVE_FREQ_3 = 2005-184T13:59:32.27 8429749425.738658
   TRANSMIT_FREQ_1 = 2005-184T11:12:29 7175173386.028573
   TRANSMIT_FREQ_RATE_1 = 2005-184T11:12:29 0.40220
   RECEIVE_FREQ_3 = 2005-184T13:59:33.27 8429749425.113143
   TRANSMIT_FREQ_1 = 2005-184T11:12:30 7175173386.430773
   TRANSMIT_FREQ_RATE_1 = 2005-184T11:12:30 0.40220
   RECEIVE_FREQ_3 = 2005-184T13:59:34.27 8429749424.489933
   TRANSMIT_FREQ_1 = 2005-184T11:12:31 7175173386.832973
   TRANSMIT_FREQ_RATE_1 = 2005-184T11:12:31 0.40220
   RECEIVE_FREQ_3 = 2005-184T13:59:35.27 8429749423.876996
   TRANSMIT_FREQ_1 = 2005-184T11:12:32 7175173387.235173
   TRANSMIT_FREQ_RATE_1 = 2005-184T11:12:32 0.40220
   RECEIVE_FREQ_3 = 2005-184T13:59:36.27 8429749423.325228
   TRANSMIT_FREQ_1 = 2005-184T11:12:33 7175173387.637373
   TRANSMIT_FREQ_RATE_1 = 2005-184T11:12:33 0.40220
   RECEIVE_FREQ_3 = 2005-184T13:59:37.27 8429749422.664049
   TRANSMIT_FREQ_1 = 2005-184T11:12:34 7175173388.039573
   TRANSMIT_FREQ_RATE_1 = 2005-184T11:12:34 0.40220
   RECEIVE_FREQ_3 = 2005-184T13:59:38.27 8429749422.054996
   TRANSMIT_FREQ_1 = 2005-184T11:12:35 7175173388.441773
   TRANSMIT_FREQ_RATE_1 = 2005-184T11:12:35 0.40220
   RECEIVE_FREQ_3 = 2005-184T13:59:39.27 8429749421.425801
   TRANSMIT_FREQ_1 = 2005-184T11:12:36 7175173388.843973
   RECEIVE_FREQ_3 = 2005-184T13:59:40.27 8429749420.824186
   DATA_STOP

.. code-block:: text
   :caption: Figure E-6: TDM Example: Four-Way Data

   CCSDS_TDM_VERS = 2.0
   COMMENT TDM example created by yyyyy-nnnA Nav Team (JAXA)
   CREATION_DATE = 1998-06-10T01:00:00
   ORIGINATOR = JAXA
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 1998-06-10T00:57:37
   STOP_TIME = 1998-06-10T00:57:44
   PARTICIPANT_1 = NORTH
   PARTICIPANT_2 = F07R07
   PARTICIPANT_3 = E7
   MODE = SEQUENTIAL
   PATH = 1,2,3,2,1
   INTEGRATION_INTERVAL = 1.0
   INTEGRATION_REF = MIDDLE
   RANGE_MODE = CONSTANT
   RANGE_MODULUS = 0
   RANGE_UNITS = km
   ANGLE_TYPE = AZEL
   META_STOP
   DATA_START
   RANGE = 1998-06-10T00:57:37 80452.7542
   ANGLE_1 = 1998-06-10T00:57:37 256.64002393
   ANGLE_2 = 1998-06-10T00:57:37 13.38100016
   TRANSMIT_FREQ_1 = 1998-06-10T00:57:37 2106395199.07917
   RECEIVE_FREQ = 1998-06-10T00:57:37 2287487999.0
   
   RANGE = 1998-06-10T00:57:38 80452.7368
   ANGLE_1 = 1998-06-10T00:57:38 256.64002393
   ANGLE_2 = 1998-06-10T00:57:38 13.38100016
   TRANSMIT_FREQ_1 = 1998-06-10T00:57:38 2106395199.07917
   RECEIVE_FREQ = 1998-06-10T00:57:38 2287487999.0
   
   RANGE = 1998-06-10T00:57:39 80452.7197
   ANGLE_1 = 1998-06-10T00:57:39 256.64002393
   ANGLE_2 = 1998-06-10T00:57:39 13.38100016
   TRANSMIT_FREQ_1 = 1998-06-10T00:57:39 2106395199.07917
   RECEIVE_FREQ = 1998-06-10T00:57:39 2287487999.0
   
   RANGE = 1998-06-10T00:57:40 80452.7025
   ANGLE_1 = 1998-06-10T00:57:40 256.64002393
   ANGLE_2 = 1998-06-10T00:57:40 13.38100016
   TRANSMIT_FREQ_1 = 1998-06-10T00:57:40 2106395199.07917
   RECEIVE_FREQ = 1998-06-10T00:57:40 2287487999.0
   
   RANGE = 1998-06-10T00:57:41 80452.6854
   ANGLE_1 = 1998-06-10T00:57:41 256.64002393
   ANGLE_2 = 1998-06-10T00:57:41 13.38100016
   TRANSMIT_FREQ_1 = 1998-06-10T00:57:41 2106395199.07917
   RECEIVE_FREQ = 1998-06-10T00:57:41 2287487999.0
   
   RANGE = 1998-06-10T00:57:42 80452.6680
   ANGLE_1 = 1998-06-10T00:57:42 256.64002393
   ANGLE_2 = 1998-06-10T00:57:42 13.38100016
   TRANSMIT_FREQ_1 = 1998-06-10T00:57:42 2106395199.07917
   RECEIVE_FREQ = 1998-06-10T00:57:42 2287487999.0
   
   RANGE = 1998-06-10T00:57:43 80452.6503
   ANGLE_1 = 1998-06-10T00:57:43 256.64002393
   ANGLE_2 = 1998-06-10T00:57:43 13.38100016
   TRANSMIT_FREQ_1 = 1998-06-10T00:57:43 2106395199.07917
   RECEIVE_FREQ = 1998-06-10T00:57:43 2287487999.0
   
   RANGE = 1998-06-10T00:57:44 80452.6331
   ANGLE_1 = 1998-06-10T00:57:44 256.64002393
   ANGLE_2 = 1998-06-10T00:57:44 13.38100016
   TRANSMIT_FREQ_1 = 1998-06-10T00:57:44 2106395199.07917
   RECEIVE_FREQ = 1998-06-10T00:57:44 2287487999.0
   DATA_STOP

.. code-block:: text
   :caption: Figure E-7: TDM Example: One S/C, X-up, S-down, X-down, Ka-down, Three Segments

   CCSDS_TDM_VERS = 2.0
   COMMENT TDM example created by yyyyy-nnnA Nav Team (NASA/JPL)
   COMMENT This example TDM describes a scenario such as might occur with a
   COMMENT spacecraft like Cassini, which has 3 transponders: X/S, X/X, X/Ka.
   COMMENT In this tracking session all 3 transponders were used.
   COMMENT This requires a TDM with 3 segments, because a single segment would
   COMMENT not be able to specify a ‘PATH’ statement that would describe the
   COMMENT S-down, X-down, and Ka-down signal paths.
   CREATION_DATE = 2006-347T22:51
   ORIGINATOR = NASA
   META_START
   TIME_SYSTEM = UTC
   PARTICIPANT_1 = DSS-25
   PARTICIPANT_2 = 1997-061A-X
   MODE = SEQUENTIAL
   PATH = 1,2,1
   TRANSMIT_BAND = X
   RECEIVE_BAND = X
   INTEGRATION_INTERVAL = 300.0
   INTEGRATION_REF = MIDDLE
   TRANSMIT_DELAY_1 = 0.000077
   RECEIVE_DELAY_1 = 0.000077
   META_STOP
   DATA_START
   TRANSMIT_FREQ_1 = 2006-347T03:50:34 7175802770.23
   RECEIVE_FREQ_1 = 2006-347T06:17:49 8430849716.68
   DATA_STOP
   META_START
   TIME_SYSTEM = UTC
   PARTICIPANT_1 = DSS-25
   PARTICIPANT_2 = 1997-061A-KA
   MODE = SEQUENTIAL
   PATH = 1,2,1
   TRANSMIT_BAND = X
   RECEIVE_BAND = KA
   INTEGRATION_INTERVAL = 300.0
   INTEGRATION_REF = MIDDLE
   TRANSMIT_DELAY_1 = 0.000077
   RECEIVE_DELAY_1 = 0.000077
   META_STOP
   DATA_START
   TRANSMIT_FREQ_1 = 2006-347T03:50:34 7175802770.23
   RECEIVE_FREQ_1 = 2006-347T06:17:49 32037228923.40
   DATA_STOP
   META_START
   TIME_SYSTEM = UTC
   PARTICIPANT_1 = DSS-25
   PARTICIPANT_2 = 1997-061A-S
   PARTICIPANT_3 = DSS-24
   MODE = SEQUENTIAL
   PATH = 1,2,3
   TRANSMIT_BAND = X
   RECEIVE_BAND = S
   INTEGRATION_INTERVAL = 300.0
   INTEGRATION_REF = MIDDLE
   TRANSMIT_DELAY_1 = 7.7e-5
   RECEIVE_DELAY_3 = 7.7e-5
   META_STOP
   DATA_START
   TRANSMIT_FREQ_1 = 2006-347T03:50:34 7175802770.23
   RECEIVE_FREQ_1 = 2006-347T06:17:49 2299322650.01
   DATA_STOP

.. code-block:: text
   :caption: Figure E-8: TDM Example: Angles, Range, Doppler Combined in Single TDM

   CCSDS_TDM_VERS = 2.0
   COMMENT GEOSCX_INP
   CREATION_DATE = 2007-08-30T12:01:44.749
   ORIGINATOR = DLR
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2007-08-29T07:00:02.000
   STOP_TIME = 2007-08-29T14:00:02.000
   PARTICIPANT_1 = HBSTK
   PARTICIPANT_2 = SAT
   MODE = SEQUENTIAL
   PATH = 1,2,1
   INTEGRATION_INTERVAL = 1.0
   INTEGRATION_REF = END
   ANGLE_TYPE = XSYE
   DATA_QUALITY = RAW
   META_STOP
   DATA_START
   DOPPLER_INTEGRATED = 2007-08-29T07:00:02.000 -1.498776048
   ANGLE_1 = 2007-08-29T07:00:02.000 67.01312389
   ANGLE_2 = 2007-08-29T07:00:02.000 18.28395556
   DOPPLER_INTEGRATED = 2007-08-29T08:00:02.000 -2.201305217
   ANGLE_1 = 2007-08-29T08:00:02.000 67.01982278
   ANGLE_2 = 2007-08-29T08:00:02.000 21.19609167
   DOPPLER_INTEGRATED = 2007-08-29T12:00:02.000 2.248620597
   ANGLE_1 = 2007-08-29T12:00:02.000 -84.79697583
   ANGLE_2 = 2007-08-29T12:00:02.000 4.11574444
   DOPPLER_INTEGRATED = 2007-08-29T13:00:02.000 1.547592295
   ANGLE_1 = 2007-08-29T13:00:02.000 -85.14762500
   ANGLE_2 = 2007-08-29T13:00:02.000 4.35471389
   DOPPLER_INTEGRATED = 2007-08-29T14:00:02.000 0.929545817
   ANGLE_1 = 2007-08-29T14:00:02.000 -89.35626083
   ANGLE_2 = 2007-08-29T14:00:02.000 2.78791667
   DATA_STOP
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2007-08-29T06:00:02.000
   STOP_TIME = 2007-08-29T12:00:02.000
   PARTICIPANT_1 = WHM1
   PARTICIPANT_2 = SAT
   MODE = SEQUENTIAL
   PATH = 1,2,1
   INTEGRATION_INTERVAL = 1.0
   INTEGRATION_REF = END
   RANGE_MODE = CONSTANT
   RANGE_MODULUS = 1.000000E+07
   ANGLE_TYPE = AZEL
   DATA_QUALITY = RAW
   META_STOP
   DATA_START
   RANGE = 2007-08-29T06:00:02.000 4.00165248953670E+04
   DOPPLER_INTEGRATED = 2007-08-29T06:00:02.000 -0.885640091
   ANGLE_1 = 2007-08-29T06:00:02.000 99.53204250
   ANGLE_2 = 2007-08-29T06:00:02.000 1.26724167
   RANGE = 2007-08-29T07:00:02.000 3.57238793591890E+04
   DOPPLER_INTEGRATED = 2007-08-29T07:00:02.000 -1.510223139
   ANGLE_1 = 2007-08-29T07:00:02.000 103.33061750
   ANGLE_2 = 2007-08-29T07:00:02.000 4.77875278
   RANGE = 2007-08-29T08:00:02.000 2.90270197047210E+04
   DOPPLER_INTEGRATED = 2007-08-29T08:00:02.000 -2.229907387
   ANGLE_1 = 2007-08-29T08:00:02.000 104.60635806
   ANGLE_2 = 2007-08-29T08:00:02.000 5.47492500
   RANGE = 2007-08-29T12:00:02.000 2.81439006334980E+04
   DOPPLER_INTEGRATED = 2007-08-29T12:00:02.000 2.222121620
   ANGLE_1 = 2007-08-29T12:00:02.000 240.89006194
   ANGLE_2 = 2007-08-29T12:00:02.000 6.71215556
   DATA_STOP

.. code-block:: text
   :caption: Figure E-9: TDM Example: Range Data with TIMETAG_REF=TRANSMIT

   CCSDS_TDM_VERS = 2.0
   COMMENT This TDM example contains range data timetagged at transmit time
   CREATION_DATE = 2005-09-17T23:59:59
   ORIGINATOR = JAXA
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2005-09-17T00:41:38.0000
   STOP_TIME = 2005-09-17T00:42:58.0000
   PARTICIPANT_1 = yyyy-nnnA
   PARTICIPANT_2 = USC1
   MODE = SEQUENTIAL
   PATH = 2,1,2
   TRANSMIT_BAND = S
   RECEIVE_BAND = S
   TIMETAG_REF = TRANSMIT
   INTEGRATION_REF = START
   RANGE_MODE = CONSTANT
   RANGE_MODULUS = 1.0E7
   RANGE_UNITS = km
   DATA_QUALITY = VALIDATED
   CORRECTION_RANGE = 0.0
   CORRECTIONS_APPLIED = YES
   META_STOP
   DATA_START
   RANGE = 2005-09-17T00:41:38.000000 3198.03679519614
   RANGE = 2005-09-17T00:41:40.000000 3199.82505720811
   RANGE = 2005-09-17T00:41:42.000000 3201.61631714467
   RANGE = 2005-09-17T00:41:44.000000 3203.40832656236
   RANGE = 2005-09-17T00:41:46.000000 3205.20108546120
   RANGE = 2005-09-17T00:41:48.000000 3206.99384436004
   RANGE = 2005-09-17T00:41:50.000000 3208.79110014575
   RANGE = 2005-09-17T00:41:52.000000 3210.58535800688
   RANGE = 2005-09-17T00:41:54.000000 3212.38336327374
   RANGE = 2005-09-17T00:41:56.000000 3214.18136854059
   RANGE = 2005-09-17T00:41:58.000000 3215.98012328859
   RANGE = 2005-09-17T00:42:00.000000 3217.78037699888
   RANGE = 2005-09-17T00:42:02.000000 3219.58287915260
   RANGE = 2005-09-17T00:42:04.000000 3221.38613078747
   RANGE = 2005-09-17T00:42:06.000000 3223.19013190349
   RANGE = 2005-09-17T00:42:08.000000 3224.99488250065
   RANGE = 2005-09-17T00:42:10.000000 3226.80113206010
   RANGE = 2005-09-17T00:42:12.000000 3228.60963006298
   RANGE = 2005-09-17T00:42:14.000000 3230.41587962244
   RANGE = 2005-09-17T00:42:16.000000 3232.22587658761
   RANGE = 2005-09-17T00:42:18.000000 3234.03662303393
   RANGE = 2005-09-17T00:42:20.000000 3235.84886844254
   RANGE = 2005-09-17T00:42:22.000000 3237.65961488886
   RANGE = 2005-09-17T00:42:24.000000 3239.47560770319
   RANGE = 2005-09-17T00:42:26.000000 3241.28860259295
   RANGE = 2005-09-17T00:42:28.000000 3243.10384592614
   RANGE = 2005-09-17T00:42:30.000000 3244.92133770276
   RANGE = 2005-09-17T00:42:32.000000 3246.73882947939
   RANGE = 2005-09-17T00:42:34.000000 3248.55856969945
   RANGE = 2005-09-17T00:42:36.000000 3250.37681095722
   RANGE = 2005-09-17T00:42:38.000000 3252.19879962071
   RANGE = 2005-09-17T00:42:40.000000 3254.02003880307
   RANGE = 2005-09-17T00:42:42.000000 3255.84352642885
   RANGE = 2005-09-17T00:42:44.000000 3257.66851301693
   RANGE = 2005-09-17T00:42:46.000000 3259.49125116157
   RANGE = 2005-09-17T00:42:48.000000 3261.31848619307
   RANGE = 2005-09-17T00:42:50.000000 3263.14572122459
   RANGE = 2005-09-17T00:42:52.000000 3264.97295625609
   RANGE = 2005-09-17T00:42:54.000000 3266.80169024990
   RANGE = 2005-09-17T00:42:56.000000 3268.63267268713
   RANGE = 2005-09-17T00:42:58.000000 3270.46440460551
   DATA_STOP

.. code-block:: text
   :caption: Figure E-10: TDM Example: Differenced Doppler Observable

   CCSDS_TDM_VERS = 2.0
   COMMENT This TDM example contains single differenced Doppler data.
   CREATION_DATE = 2006-354T01:38:00Z
   ORIGINATOR = NASA
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2003-07-08T04:45:25.0000
   STOP_TIME = 2003-07-08T04:48:25.0000
   PARTICIPANT_1 = yyyy-nnnA
   PARTICIPANT_2 = DSS-24
   PARTICIPANT_3 = DSS-25
   MODE = SINGLE_DIFF
   PATH_1 = 1,2
   PATH_2 = 1,3
   TRANSMIT_BAND = X
   RECEIVE_BAND = X
   INTEGRATION_INTERVAL = 10.0
   INTEGRATION_REF = MIDDLE
   RECEIVE_DELAY_2 = 0.00007732
   RECEIVE_DELAY_3 = 0.00007732
   DATA_QUALITY = VALIDATED
   META_STOP
   DATA_START
   COMMENT Transmit frequency is S/C beacon one OWLT prior to receive time
   TRANSMIT_FREQ_1 = 2003-07-08T04:10:0000 8.435360E+09
   RECEIVE_FREQ = 2003-07-08T04:45:25.0000 8.738750457763670E+00
   RECEIVE_FREQ = 2003-07-08T04:45:35.0000 8.320683479309080E+00
   RECEIVE_FREQ = 2003-07-08T04:45:45.0000 7.909399032592770E+00
   RECEIVE_FREQ = 2003-07-08T04:45:55.0000 7.490205764770500E+00
   RECEIVE_FREQ = 2003-07-08T04:46:05.0000 7.149572372436510E+00
   RECEIVE_FREQ = 2003-07-08T04:46:15.0000 6.808938980102530E+00
   RECEIVE_FREQ = 2003-07-08T04:46:25.0000 6.481011390686030E+00
   RECEIVE_FREQ = 2003-07-08T04:46:35.0000 6.167441368103020E+00
   RECEIVE_FREQ = 2003-07-08T04:46:45.0000 5.865190505981440E+00
   RECEIVE_FREQ = 2003-07-08T04:46:55.0000 5.590643882751460E+00
   RECEIVE_FREQ = 2003-07-08T04:47:05.0000 5.330531120300290E+00
   RECEIVE_FREQ = 2003-07-08T04:47:15.0000 5.083267211914060E+00
   RECEIVE_FREQ = 2003-07-08T04:47:25.0000 4.850607872009270E+00
   RECEIVE_FREQ = 2003-07-08T04:47:35.0000 4.643701979796000E+00
   RECEIVE_FREQ = 2003-07-08T04:47:45.0000 4.453802272725000E+00
   RECEIVE_FREQ = 2003-07-08T04:47:55.0000 4.281702585856000E+00
   RECEIVE_FREQ = 2003-07-08T04:48:05.0000 4.127402919189000E+00
   RECEIVE_FREQ = 2003-07-08T04:48:15.0000 3.990903272724000E+00
   RECEIVE_FREQ = 2003-07-08T04:48:25.0000 3.872203646461000E+00
   DATA_STOP

.. code-block:: text
   :caption: Figure E-11: TDM Example: Delta-DOR Observable

   CCSDS_TDM_VERS = 2.0
   COMMENT This TDM example contains Delta-DOR data.
   COMMENT Quasar CTD 20 also known as J023752.4+284808 (ICRF), 0234+285 (IERS)
   CREATION_DATE = 2005-178T21:45:00
   ORIGINATOR = NASA
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2004-136T15:42:00.0000
   STOP_TIME = 2004-136T16:02:00.0000
   PARTICIPANT_1 = VOYAGER1
   PARTICIPANT_2 = DSS-55
   PARTICIPANT_3 = DSS-25
   MODE = SINGLE_DIFF
   PATH_1 = 1,2
   PATH_2 = 1,3
   TRANSMIT_BAND = X
   RECEIVE_BAND = X
   TIMETAG_REF = RECEIVE
   RANGE_MODE = ONE_WAY
   RANGE_MODULUS = 1.674852710000000E+02
   RECEIVE_DELAY_3 = 0.000077
   DATA_QUALITY = VALIDATED
   META_STOP
   DATA_START
   COMMENT Timetag is time of signal arrival at PARTICIPANT_2.
   COMMENT Transmit frequency is spacecraft beacon a OWLT before receive time.
   DOR = 2004-136T15:42:00.0000 -4.911896106591159E-03
   DOR = 2004-136T16:02:00.0000 1.467382930436399E-02
   TRANSMIT_FREQ_1 = 2004-136T14:42:00.0000 8.415123456E+09
   DATA_STOP
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2004-136T15:52:00.0000
   STOP_TIME = 2004-136T15:52:00.0000
   PARTICIPANT_1 = CTD 20
   PARTICIPANT_2 = DSS-55
   PARTICIPANT_3 = DSS-25
   MODE = SINGLE_DIFF
   PATH_1 = 1,2
   PATH_2 = 1,3
   TRANSMIT_BAND = X
   RECEIVE_BAND = X
   TIMETAG_REF = RECEIVE
   RANGE_MODE = ONE_WAY
   RANGE_MODULUS = 1.674852710000000E+02
   RECEIVE_DELAY_3 = 0.000077
   DATA_QUALITY = VALIDATED
   META_STOP
   DATA_START
   COMMENT Timetag is time of signal arrival at PARTICIPANT_2.
   COMMENT Transmit frequency is reference for 2-station interferometer.
   VLBI_DELAY = 2004-136T15:52:00.0000 -1.911896106591159E-03
   TRANSMIT_FREQ_1 = 2004-136T15:42:00.0000 8.415123000E+09
   DATA_STOP
   META_START
   TIME_SYSTEM = UTC
   PARTICIPANT_1 = DSS-55
   PARTICIPANT_2 = DSS-25
   DATA_QUALITY = VALIDATED
   META_STOP
   DATA_START
   CLOCK_BIAS = 2004-136T15:41:00.0000 -4.59e-7
   DATA_STOP

.. code-block:: text
   :caption: Figure E-12: TDM Example: Angle Data Only

   CCSDS_TDM_VERS = 2.0
   COMMENT TDM example created by yyyyy-nnnA Nav Team (NASA/JPL)
   COMMENT StarTrek: one minute of launch angles from DSS-16
   CREATION_DATE = 2005-157T18:25:00
   ORIGINATOR = NASA
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2004-216T07:44:00
   STOP_TIME = 2004-216T07:45:00
   PARTICIPANT_1 = DSS-16
   PARTICIPANT_2 = yyyy-nnnA
   MODE = SEQUENTIAL
   PATH = 2,1
   ANGLE_TYPE = XSYE
   CORRECTION_ANGLE_1 = -0.09
   CORRECTION_ANGLE_2 = 0.18
   CORRECTIONS_APPLIED = NO
   META_STOP
   DATA_START
   ANGLE_1 = 2004-216T07:44:00 -23.62012
   ANGLE_2 = 2004-216T07:44:00 -73.11035
   ANGLE_1 = 2004-216T07:44:10 -23.04004
   ANGLE_2 = 2004-216T07:44:10 -72.74316
   ANGLE_1 = 2004-216T07:44:20 -22.78125
   ANGLE_2 = 2004-216T07:44:20 -72.53027
   ANGLE_1 = 2004-216T07:44:30 -22.59180
   ANGLE_2 = 2004-216T07:44:30 -72.37598
   ANGLE_1 = 2004-216T07:44:40 -22.40527
   ANGLE_2 = 2004-216T07:44:40 -72.23730
   ANGLE_1 = 2004-216T07:44:50 -22.23047
   ANGLE_2 = 2004-216T07:44:50 -72.08887
   ANGLE_1 = 2004-216T07:45:00 -22.08984
   ANGLE_2 = 2004-216T07:45:00 -71.93750
   DATA_STOP

.. code-block:: text
   :caption: Figure E-13: TDM Example: Media Data Only

   CCSDS_TDM_VERS = 2.0
   COMMENT TDM example created by NASA/JPL Navigation System Engineering
   CREATION_DATE = 2005-282T23:00:00
   ORIGINATOR = NASA
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2005-274T12:00:00
   STOP_TIME = 2005-280T12:00:00
   PARTICIPANT_1 = DSS-14
   DATA_QUALITY = VALIDATED
   META_STOP
   DATA_START
   COMMENT Elevation mapping function is Niell model
   TROPO_DRY = 2005-274T12:00:00 2.0526
   TROPO_DRY = 2005-275T12:00:00 2.0530
   TROPO_DRY = 2005-276T12:00:00 2.0533
   TROPO_DRY = 2005-277T12:00:00 2.0537
   TROPO_DRY = 2005-278T12:00:00 2.0540
   TROPO_DRY = 2005-279T12:00:00 2.0544
   TROPO_DRY = 2005-280T12:00:00 2.0547
   TROPO_WET = 2005-274T12:00:00 0.1139
   TROPO_WET = 2005-275T12:00:00 0.1126
   TROPO_WET = 2005-276T12:00:00 0.1113
   TROPO_WET = 2005-277T12:00:00 0.1099
   TROPO_WET = 2005-278T12:00:00 0.1086
   TROPO_WET = 2005-279T12:00:00 0.1074
   TROPO_WET = 2005-280T12:00:00 0.1061
   DATA_STOP
   META_START
   COMMENT Line of vertical ionospheric calibration for yyyy-nnnA
   COMMENT Time tags are end time of 15 minute measurement interval
   TIME_SYSTEM = UTC
   START_TIME = 2005-280T21:45:00
   STOP_TIME = 2005-281T00:00:00
   PARTICIPANT_1 = DSS-14
   PARTICIPANT_2 = yyyy-nnnA
   MODE = SEQUENTIAL
   PATH = 2,1
   DATA_QUALITY = VALIDATED
   META_STOP
   DATA_START
   STEC = 2005-280T21:45:00 23.1
   STEC = 2005-280T22:00:00 22.8
   STEC = 2005-280T22:15:00 23.2
   STEC = 2005-280T22:30:00 24.4
   STEC = 2005-280T22:45:00 23.6
   STEC = 2005-280T23:00:00 22.4
   STEC = 2005-280T23:15:00 22.6
   STEC = 2005-280T23:30:00 24.6
   STEC = 2005-280T23:45:00 24.0
   STEC = 2005-281T00:00:00 22.2
   DATA_STOP

.. code-block:: text
   :caption: Figure E-14: TDM Example: Meteorological Data Only

   CCSDS_TDM_VERS = 2.0
   COMMENT TDM example created by yyyyy-nnnA Nav Team (NASA/JPL)
   COMMENT JPL/DSN/Goldstone (DSS-10) weather for DOY 156, 2005
   CREATION_DATE = 2005-156T06:15:00
   ORIGINATOR = NASA
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2005-156T00:03:00
   STOP_TIME = 2005-156T06:03:00
   PARTICIPANT_1 = DSS-10
   DATA_QUALITY = VALIDATED
   META_STOP
   DATA_START
   TEMPERATURE = 2005-156T00:03:00 302.95
   PRESSURE = 2005-156T00:03:00 896.2
   RHUMIDITY = 2005-156T00:03:00 12.0
   TEMPERATURE = 2005-156T00:33:00 304.05
   PRESSURE = 2005-156T00:33:00 895.9
   RHUMIDITY = 2005-156T00:33:00 11.0
   TEMPERATURE = 2005-156T01:03:00 302.55
   PRESSURE = 2005-156T01:03:00 895.7
   RHUMIDITY = 2005-156T01:03:00 12.0
   TEMPERATURE = 2005-156T01:33:00 302.65
   PRESSURE = 2005-156T01:33:00 895.7
   RHUMIDITY = 2005-156T01:33:00 11.0
   TEMPERATURE = 2005-156T02:03:00 301.55
   PRESSURE = 2005-156T02:03:00 895.9
   RHUMIDITY = 2005-156T02:03:00 11.0
   TEMPERATURE = 2005-156T02:33:00 300.45
   PRESSURE = 2005-156T02:33:00 895.9
   RHUMIDITY = 2005-156T02:33:00 12.0
   TEMPERATURE = 2005-156T03:03:00 299.55
   PRESSURE = 2005-156T03:03:00 896.1
   RHUMIDITY = 2005-156T03:03:00 14.0
   TEMPERATURE = 2005-156T03:33:00 298.65
   PRESSURE = 2005-156T03:33:00 896.2
   RHUMIDITY = 2005-156T03:33:00 15.0
   TEMPERATURE = 2005-156T04:03:00 298.05
   PRESSURE = 2005-156T04:03:00 896.4
   RHUMIDITY = 2005-156T04:03:00 17.0
   TEMPERATURE = 2005-156T04:33:00 297.15
   PRESSURE = 2005-156T04:33:00 896.8
   RHUMIDITY = 2005-156T04:33:00 19.0
   TEMPERATURE = 2005-156T05:03:00 294.85
   PRESSURE = 2005-156T05:03:00 897.3
   RHUMIDITY = 2005-156T05:03:00 21.0
   TEMPERATURE = 2005-156T05:33:00 293.95
   PRESSURE = 2005-156T05:33:00 897.3
   RHUMIDITY = 2005-156T05:33:00 23.0
   TEMPERATURE = 2005-156T06:03:00 293.05
   PRESSURE = 2005-156T06:03:00 897.3
   RHUMIDITY = 2005-156T06:03:00 25.0
   DATA_STOP

.. code-block:: text
   :caption: Figure E-15: TDM Example: Clock Bias/Drift Only

   CCSDS_TDM_VERS = 2.0
   COMMENT TDM example created by yyyyy-nnnA Nav Team (NASA/JPL)
   COMMENT The following are clock offsets, in seconds between the
   COMMENT clocks at each DSN complex relative to UTC(NIST). The offset
   COMMENT is a mean of readings using several GPS space vehicles in
   COMMENT common view. Value is "station clock minus UTC”.
   CREATION_DATE = 2005-161T15:45:00
   ORIGINATOR = NASA
   META_START
   COMMENT Note: SPC10 switched back to Maser1 from Maser2 on 2005-142
   TIME_SYSTEM = UTC
   START_TIME = 2005-142T12:00:00
   STOP_TIME = 2005-145T12:00:00
   PARTICIPANT_1 = DSS-10
   PARTICIPANT_2 = UTC-NIST
   META_STOP
   DATA_START
   CLOCK_BIAS = 2005-142T12:00:00 9.56e-7
   CLOCK_DRIFT = 2005-142T12:00:00 6.944e-14
   CLOCK_BIAS = 2005-143T12:00:00 9.62e-7
   CLOCK_DRIFT = 2005-143T12:00:00 -2.083e-13
   CLOCK_BIAS = 2005-144T12:00:00 9.44e-7
   CLOCK_DRIFT = 2005-144T12:00:00 -2.778e-13
   CLOCK_BIAS = 2005-145T12:00:00 9.20e-7
   DATA_STOP
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2005-142T12:00:00
   STOP_TIME = 2005-145T12:00:00
   PARTICIPANT_1 = DSS-40
   PARTICIPANT_2 = UTC-NIST
   META_STOP
   DATA_START
   CLOCK_BIAS = 2005-142T12:00:00 -7.40e-7
   CLOCK_DRIFT = 2005-142T12:00:00 -3.125e-13
   CLOCK_BIAS = 2005-143T12:00:00 -7.67e-7
   CLOCK_DRIFT = 2005-143T12:00:00 -1.620e-13
   CLOCK_BIAS = 2005-144T12:00:00 -7.81e-7
   CLOCK_DRIFT = 2005-144T12:00:00 -4.745e-13
   CLOCK_BIAS = 2005-145T12:00:00 -8.22e-7
   DATA_STOP
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2005-142T12:00:00
   STOP_TIME = 2005-145T12:00:00
   PARTICIPANT_1 = DSS-60
   PARTICIPANT_2 = UTC-NIST
   META_STOP
   DATA_START
   CLOCK_BIAS = 2005-142T12:00:00 -1.782e-6
   CLOCK_DRIFT = 2005-142T12:00:00 1.736e-13
   CLOCK_BIAS = 2005-143T12:00:00 -1.767e-6
   CLOCK_DRIFT = 2005-143T12:00:00 1.157e-14
   CLOCK_BIAS = 2005-144T12:00:00 -1.766e-6
   CLOCK_DRIFT = 2005-144T12:00:00 8.102e-14
   CLOCK_BIAS = 2005-145T12:00:00 -1.759e-6
   DATA_STOP

.. code-block:: text
   :caption: Figure E-16: TDM Example: Ground Based Optical Tracking with Magnitude

   CCSDS_TDM_VERS = 2.0
   COMMENT All the angular data provided are free of any aberration effect.
   CREATION_DATE = 2012-10-30T20:00
   ORIGINATOR = ESA
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2012-10-29T17:46:39.02
   STOP_TIME = 2012-10-29T17:50:53.02
   PARTICIPANT_1 = TFRM
   PARTICIPANT_2 = TRACK NUMBER 001
   MODE = SEQUENTIAL
   PATH = 2,1
   ANGLE_TYPE = RADEC
   REFERENCE_FRAME = EME2000
   META_STOP
   DATA_START
   ANGLE_1 = 2012-10-29T17:46:39.02 332.2298750
   ANGLE_2 = 2012-10-29T17:46:39.02 -16.3028389
   MAG = 2012-10-29T17:46:39.02 12.1
   ANGLE_1 = 2012-10-29T17:48:46.02 332.7485833
   ANGLE_2 = 2012-10-29T17:48:46.02 -16.1876917
   MAG = 2012-10-29T17:48:46.02 12.3
   ANGLE_1 = 2012-10-29T17:50:53.02 333.2668750
   ANGLE_2 = 2012-10-29T17:50:53.02 -16.0716806
   MAG = 2012-10-29T17:50:53.02 12.3
   DATA_STOP
   META_START
   TIME_SYSTEM = UTC
   START_TIME = 2012-10-29T17:57:14.02
   STOP_TIME = 2012-10-29T18:01:28.02
   PARTICIPANT_1 = TFRM
   PARTICIPANT_2 = TRACK NUMBER 003
   MODE = SEQUENTIAL
   PATH = 2,1
   ANGLE_TYPE = RADEC
   REFERENCE_FRAME = EME2000
   META_STOP
   DATA_START
   ANGLE_1 = 2012-10-29T17:57:14.02 335.1698333
   ANGLE_2 = 2012-10-29T17:57:14.02 -17.7212861
   MAG = 2012-10-29T17:57:14.02 11.8
   ANGLE_1 = 2012-10-29T17:59:21.02 335.7062083
   ANGLE_2 = 2012-10-29T17:59:21.02 -17.6950278
   MAG = 2012-10-29T17:59:21.02 12.4
   ANGLE_1 = 2012-10-29T18:01:28.02 336.2425833
   ANGLE_2 = 2012-10-29T18:01:28.02 -17.6673694
   MAG = 2012-10-29T18:01:28.02 13.1
   DATA_STOP

.. code-block:: text
   :caption: Figure E-17: TDM Example: Ground Based Radar Tracking with RCS

   CCSDS_TDM_VERS = 2.0
   COMMENT Test file
   CREATION_DATE = 2011-05-12T00:00:00.000
   ORIGINATOR = ESA
   META_START
   COMMENT
   TIME_SYSTEM = UTC
   PARTICIPANT_1 = CAMRA
   PARTICIPANT_2 = CRYOSAT
   MODE = SEQUENTIAL
   PATH = 1,2,1
   EPHEMERIS_NAME = 3203_2013-11-09T23-02-30
   RANGE_UNITS = km
   ANGLE_TYPE = AZEL
   CORRECTION_RANGE = -1.48
   CORRECTIONS_APPLIED = NO
   META_STOP
   DATA_START
   RANGE = 2011-05-11T10:26:33.2613 2808.2696
   ANGLE_1 = 2011-05-11T10:26:33.2613 191.40208435
   ANGLE_2 = 2011-05-11T10:26:33.2613 25.44166756
   CARRIER_POWER = 2011-05-11T10:26:33.2613 -36.73723984
   RCS = 2011-05-11T10:26:33.2613 2.984
   RANGE = 2011-05-11T10:26:33.7008 2803.1731
   ANGLE_1 = 2011-05-11T10:26:33.7008 191.43959045
   ANGLE_2 = 2011-05-11T10:26:33.7008 25.51874924
   CARRIER_POWER = 2011-05-11T10:26:33.7008 -35.88296509
   RCS = 2011-05-11T10:26:33.7008 2.992
   RANGE = 2011-05-11T10:26:33.9686 2799.8754
   ANGLE_1 = 2011-05-11T10:26:33.9686 191.46458435
   ANGLE_2 = 2011-05-11T10:26:33.9686 25.56875038
   CARRIER_POWER = 2011-05-11T10:26:33.9686 -36.67897415
   RCS = 2011-05-11T10:26:33.7008 2.986
   DATA_STOP

.. code-block:: text
   :caption: Figure E-18: TDM Example: Two-Way Phase Data for Doppler Calculation

   CCSDS_TDM_VERS=2.0
   COMMENT TDM example created by yyyy-nnnA Nav Team (NASA/JPL)
   CREATION_DATE=2005-184T20:15:00
   ORIGINATOR=NASA
   MESSAGE_ID=DSN-2005-184-yyyynnnA-001
   META_START
   TIME_SYSTEM=UTC
   START_TIME=2005-184T11:12:23
   STOP_TIME=2005-184T11:12:32
   PARTICIPANT_1=DSS-55
   PARTICIPANT_2=yyyy-nnnA
   MODE=SEQUENTIAL
   PATH=1,2,1
   FREQ_OFFSET=0.0
   INTERPOLATION = HERMITE
   INTERPOLATION_DEGREE = 7
   META_STOP
   DATA_START
   TRANSMIT_PHASE_CT_1=2005-184T11:12:23 7175173383.615373
   TRANSMIT_PHASE_CT_1=2005-184T11:12:24 14350346766.632946
   TRANSMIT_PHASE_CT_1=2005-184T11:12:25 21525520150.052719
   TRANSMIT_PHASE_CT_1=2005-184T11:12:26 28700693531.874692
   TRANSMIT_PHASE_CT_1=2005-184T11:12:27 35875866917.098865
   TRANSMIT_PHASE_CT_1=2005-184T11:12:28 43051040300.725238
   TRANSMIT_PHASE_CT_1=2005-184T11:12:29 50226213683.753811
   TRANSMIT_PHASE_CT_1=2005-184T11:12:30 57401387067.184584
   TRANSMIT_PHASE_CT_1=2005-184T11:12:31 64576560451.017557
   TRANSMIT_PHASE_CT_1=2005-184T11:12:32 71751733834.252730
   DATA_STOP
   META_START
   TIME_SYSTEM=UTC
   START_TIME=2005-184T13:59:27.27
   STOP_TIME=2005-184T13:59:36.27
   PARTICIPANT_1=DSS-55
   PARTICIPANT_2=yyyy-nnnA
   MODE=SEQUENTIAL
   PATH=1,2,1
   FREQ_OFFSET=0.0
   INTERPOLATION = HERMITE
   INTERPOLATION_DEGREE = 7
   META_STOP
   DATA_START
   RECEIVE_PHASE_CT_1=2005-184T13:59:27.27 8429753135.986102
   RECEIVE_PHASE_CT_1=2005-184T13:59:28.27 16859502564.182670
   RECEIVE_PHASE_CT_1=2005-184T13:59:29.27 25289251991.767397
   RECEIVE_PHASE_CT_1=2005-184T13:59:30.27 33719001418.790500
   RECEIVE_PHASE_CT_1=2005-184T13:59:31.27 42148750841.136752
   RECEIVE_PHASE_CT_1=2005-184T13:59:32.27 50578500270.875410
   RECEIVE_PHASE_CT_1=2005-184T13:59:33.27 59008249695.988553
   RECEIVE_PHASE_CT_1=2005-184T13:59:34.27 67437999120.478486
   RECEIVE_PHASE_CT_1=2005-184T13:59:35.27 75867748544.355482
   RECEIVE_PHASE_CT_1=2005-184T13:59:36.27 84297497967.680710
   DATA_STOP

.. code-block:: text
   :caption: Figure E-19: TDM Example: Two-Way Range Data with Ranging Power to Spectral Density

   CCSDS_TDM_VERS = 2.0
   COMMENT CREATED BY TTC PGM V33.0.2
   CREATION_DATE = 2010-050T20:15:02.000
   ORIGINATOR = NASA/JPL/DSN
   META_START
   COMMENT SEQUENTIAL RANGE
   COMMENT RANGE IS ADJUSTED FOR CORRECTION_RANGE; MEASUREMENT MINUS CORRECTION_RANGE
   COMMENT CORRECTION_RANGE INCLUDES STATION DELAY, Z-HEIGHT CORRECTION, AND S/C DELAY
   COMMENT DOWNLINK CHANNEL NUMBER 4
   TIME_SYSTEM = UTC
   START_TIME = 2010-215T20:04:24.000
   STOP_TIME = 2010-215T20:53:24.000
   PARTICIPANT_1 = DSS-14
   PARTICIPANT_2 = CAS
   MODE = SEQUENTIAL
   PATH = 1,2,1
   TRANSMIT_BAND = X
   RECEIVE_BAND = X
   TURNAROUND_NUMERATOR = 880
   TURNAROUND_DENOMINATOR = 749
   TIMETAG_REF = RECEIVE
   INTEGRATION_REF = START
   RANGE_MODE = COHERENT
   RANGE_MODULUS = 262144
   RANGE_UNITS = RU
   TRANSMIT_DELAY_1 = 2.1E-07
   RECEIVE_DELAY_1 = 2.1E-07
   DATA_QUALITY = VALIDATED
   CORRECTION_RANGE = 4999.392714
   CORRECTIONS_APPLIED = YES
   META_STOP
   DATA_START
   RANGE = 2010-215T20:04:24.000 65249.6771931631
   PR_N0 = 2010-215T20:04:24.000 30.2351
   RANGE = 2010-215T20:11:24.000 52234.4753877508
   PR_N0 = 2010-215T20:11:24.000 32.7846
   RANGE = 2010-215T20:18:24.000 68142.6393474573
   PR_N0 = 2010-215T20:18:24.000 31.0379
   RANGE = 2010-215T20:25:24.000 113059.469322535
   PR_N0 = 2010-215T20:25:24.000 33.0883
   RANGE = 2010-215T20:32:24.000 187471.102944516
   PR_N0 = 2010-215T20:32:24.000 32.0965
   RANGE = 2010-215T20:39:24.000 29568.3320810896
   PR_N0 = 2010-215T20:39:24.000 33.7465
   RANGE = 2010-215T20:46:24.000 163212.340789491
   PR_N0 = 2010-215T20:46:24.000 31.0563
   RANGE = 2010-215T20:53:24.000 64457.0270879461
   PR_N0 = 2010-215T20:53:24.000 30.0224
   DATA_STOP

.. code-block:: text
   :caption: Figure E-20: TDM Example: Two-Way Received Frequency

   CCSDS_TDM_VERS = 2.0
   COMMENT CREATED BY JPL TTC PGM V33.0.2
   CREATION_DATE = 2010-050T20:15:02.000
   ORIGINATOR = NASA/JPL/DSN
   META_START
   COMMENT DOWNLINK CHANNEL NUMBER 4
   TIME_SYSTEM = UTC
   START_TIME = 2010-049T16:49:43.000
   STOP_TIME = 2010-049T17:04:43.000
   PARTICIPANT_1 = DSS-26
   PARTICIPANT_2 = CAS
   MODE = SEQUENTIAL
   PATH = 1,2,1
   TRANSMIT_BAND = X
   RECEIVE_BAND = X
   TURNAROUND_NUMERATOR = 880
   TURNAROUND_DENOMINATOR = 749
   TIMETAG_REF = RECEIVE
   INTEGRATION_INTERVAL = 60.0
   INTEGRATION_REF = MIDDLE
   FREQ_OFFSET = 8427221784.667
   RECEIVE_DELAY_1 = 0.00000556
   DATA_QUALITY = VALIDATED
   META_STOP
   DATA_START
   RECEIVE_FREQ_1 = 2010-049T16:49:43.000 60255.16982
   RECEIVE_FREQ_1 = 2010-049T16:50:43.000 60271.18801
   RECEIVE_FREQ_1 = 2010-049T16:51:43.000 60287.20579
   RECEIVE_FREQ_1 = 2010-049T16:52:43.000 60303.22356
   RECEIVE_FREQ_1 = 2010-049T16:53:43.000 60319.24150
   RECEIVE_FREQ_1 = 2010-049T16:54:43.000 60335.25922
   RECEIVE_FREQ_1 = 2010-049T16:55:43.000 60351.27720
   RECEIVE_FREQ_1 = 2010-049T16:56:43.000 60368.00758
   RECEIVE_FREQ_1 = 2010-049T16:57:43.000 60387.66759
   RECEIVE_FREQ_1 = 2010-049T16:58:43.000 60407.64419
   RECEIVE_FREQ_1 = 2010-049T16:59:43.000 60427.62086
   RECEIVE_FREQ_1 = 2010-049T17:00:43.000 60447.59751
   RECEIVE_FREQ_1 = 2010-049T17:01:43.000 60467.57420
   RECEIVE_FREQ_1 = 2010-049T17:02:43.000 60487.55087
   RECEIVE_FREQ_1 = 2010-049T17:03:43.000 60507.52741
   RECEIVE_FREQ_1 = 2010-049T17:04:43.000 60527.50426
   DATA_STOP

.. code-block:: xml
   :caption: Figure E-21: TDM Example: XML Format

   <?xml version="1.0" encoding="UTF-8"?>
   <tdm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xmlns:ndm="urn:ccsds:schema:ndmxml"
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-2.0.0-master-2.0.xsd"
    id="CCSDS_TDM_VERS" version="2.0">
    
    <header>
    <CREATION_DATE>2007-094T23:53:59.659</CREATION_DATE>
    <ORIGINATOR>NASA</ORIGINATOR>
    </header>
    <body>
    <segment>
    <metadata>
    <DATA_TYPES>TRANSMIT_FREQ_1, TRANSMIT_FREQ_RATE_1</DATA_TYPES>
    <TIME_SYSTEM>UTC</TIME_SYSTEM>
    <PARTICIPANT_1>‘DSS-25’</PARTICIPANT_1>
    <PARTICIPANT_2>MYSC</PARTICIPANT_2>
    <MODE>SEQUENTIAL</MODE>
    <PATH>1,2</PATH>
    <TRANSMIT_BAND>X</TRANSMIT_BAND>
    </metadata>
    <data>
    <observation>
    <EPOCH>2007-069T15:22:22.000</EPOCH>
    <TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>
    </observation>
    <observation>
    <EPOCH>2007-069T15:22:22.000</EPOCH>
    <TRANSMIT_FREQ_RATE_1>0.0</TRANSMIT_FREQ_RATE_1>
    </observation>
    <observation>
    <EPOCH>2007-069T15:23:30.000</EPOCH>
    <TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>
    </observation>
    <observation>
    <EPOCH>2007-069T15:23:30.000</EPOCH>
    <TRANSMIT_FREQ_RATE_1>0.0</TRANSMIT_FREQ_RATE_1>
    </observation>
    <observation>
    <EPOCH>2007-069T15:23:38.000</EPOCH>
    <TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>
    </observation>
    <observation>
    <EPOCH>2007-069T15:23:38.000</EPOCH>
    <TRANSMIT_FREQ_RATE_1>0.0</TRANSMIT_FREQ_RATE_1>
    </observation>
    <observation>
    <EPOCH>2007-069T15:34:36.000</EPOCH>
    <TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>
    </observation>
    <observation>
    <EPOCH>2007-069T15:34:36.000</EPOCH>
    <TRANSMIT_FREQ_RATE_1>0.0</TRANSMIT_FREQ_RATE_1>
    </observation>
    </data>
    </segment>
    </body>
   </tdm>

.. code-block:: text
   :caption: Figure E-22: TDM Example: Use of ‘TRACK_ID’

   CCSDS_TDM_VERS = 2.0
   CREATION_DATE = 2019-10-21T22:17:21
   ORIGINATOR = GSOC
   META_START
   TRACK_ID = S_191021_18593902_3
   TIME_SYSTEM = UTC
   START_TIME = 2019-10-21T18:59:38.869008
   STOP_TIME = 2019-10-21T19:00:39.023021
   PARTICIPANT_1 = SMARTNET-01-A-SUTH
   PARTICIPANT_2 = UNKNOWN
   MODE = SEQUENTIAL
   PATH = 2,1
   ANGLE_TYPE = RADEC
   REFERENCE_FRAME = EME2000
   CORRECTION_RECEIVE = -0.145
   CORRECTION_ABERRATION_YEARLY = 0.0056932
   CORRECTIONS_APPLIED = YES
   META_STOP
   DATA_START
   ANGLE_1 = 2019-10-21T18:59:38.869008 333.64830529
   ANGLE_2 = 2019-10-21T18:59:38.869008 5.23646136
   MAG = 2019-10-21T18:59:38.869008 10.66
   ANGLE_1 = 2019-10-21T19:00:24.405696 333.83841725
   ANGLE_2 = 2019-10-21T19:00:24.405696 5.23617947
   MAG = 2019-10-21T19:00:24.405696 10.77
   ANGLE_1 = 2019-10-21T19:00:39.023021 333.89958508
   ANGLE_2 = 2019-10-21T19:00:39.023021 5.23604417
   MAG = 2019-10-21T19:00:39.023021 10.80
   DATA_STOP

.. code-block:: xml
   :caption: Figure E-23: TDM Example: Use of Doppler Counts

   <?xml version="1.0" encoding="UTF-8"?>
   <tdm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-2.0.0-master-2.0.xsd"
    id="CCSDS_TDM_VERS" version="2.0">
    <header>
    <CREATION_DATE>2019-344T12:50:06.940</CREATION_DATE>
    <ORIGINATOR>GSFC</ORIGINATOR>
    </header>
    <body>
    <segment>
    <metadata>
    <TIME_SYSTEM>UTC</TIME_SYSTEM>
   <START_TIME>2019-081T14:39:02.0</START_TIME>
   <STOP_TIME>2019-081T14:39:07.0</STOP_TIME>
    <PARTICIPANT_1>STGT central antenna</PARTICIPANT_1>
   <PARTICIPANT_2>TDRS 10</PARTICIPANT_2>
   <PARTICIPANT_3>1874</PARTICIPANT_3>
   <PARTICIPANT_4>TDRS 10</PARTICIPANT_4>
   <PARTICIPANT_5>STGT central antenna</PARTICIPANT_5>
   <MODE>SEQUENTIAL</MODE>
   <PATH>1,2,3,4,5</PATH>
   <TRANSMIT_BAND>S</TRANSMIT_BAND>
   <RECEIVE_BAND>S</RECEIVE_BAND>
   <TURNAROUND_NUMERATOR>240</TURNAROUND_NUMERATOR>
   <TURNAROUND_DENOMINATOR>221</TURNAROUND_DENOMINATOR>
   <TIMETAG_REF>RECEIVE</TIMETAG_REF>
   <INTEGRATION_INTERVAL>1.0</INTEGRATION_INTERVAL>
   <INTEGRATION_REF>END</INTEGRATION_REF>
    <DOPPLER_COUNT_BIAS>2.4E8</DOPPLER_COUNT_BIAS>
    <DOPPLER_COUNT_SCALE>1000</DOPPLER_COUNT_SCALE>
   <DOPPLER_COUNT_ROLLOVER>NO</DOPPLER_COUNT_ROLLOVER>
   <TRANSMIT_DELAY_3>0.20967</TRANSMIT_DELAY_3>
    <DATA_QUALITY>RAW</DATA_QUALITY>
    </metadata>
    <data>
    <observation>
    <EPOCH>2019-081T14:39:02.0</EPOCH>
   <DOPPLER_COUNT>0</DOPPLER_COUNT>
    </observation>
    <observation>
    <EPOCH>2019-081T14:39:03.0</EPOCH>
   <DOPPLER_COUNT>0</DOPPLER_COUNT>
    </observation>
   <observation>
    <EPOCH>2019-081T14:39:04.0</EPOCH>
    <DOPPLER_COUNT>0</DOPPLER_COUNT>
    </observation>
   <observation>
    <EPOCH>2019-081T14:39:05.0</EPOCH>
   <DOPPLER_COUNT>0</DOPPLER_COUNT>
    </observation>
    <observation>
    <EPOCH>2019-081T14:39:06.0</EPOCH>
   <DOPPLER_COUNT>0</DOPPLER_COUNT>
    </observation>
   <observation>
    <EPOCH>2019-081T14:39:07.0</EPOCH>
    <DOPPLER_COUNT>0</DOPPLER_COUNT>
    </observation>
    </data>
    </segment>
    </body>
   </tdm>

The following are some additional scenarios that are not currently considered in the example
set, but could be included in later versions of the TDM:

a) spacecraft-to-spacecraft crosslinks;

b) ground-based transponder;

c) ‘DORIS’;

d) arrayed downlink;

e) orbital debris example;

f) combination of radiometric types with media or meteorological data.
.. _annex_f:

ANNEX F
=======

INFORMATIVE REFERENCES
======================

(INFORMATIVE)

NOTE – Normative references are provided in 1.5.

[F1] Standard Frequencies and Time Signals. Volume 7 in Recommendations and Reports
     of the CCIR: XVIIth Plenary Assembly. Geneva: CCIR, 1990.

[F2] Radio Metric and Orbit Data. Issue 1-S. Recommendation for Space Data System
     Standards (Historical), CCSDS 501.0-B-1-S. Washington, D.C.: CCSDS, (January
     1987) November 2003.

[F3] Catherine L. Thornton and James S. Border. Radiometric Tracking Techniques for
     Deep-Space Navigation. JPL Deep-Space Communications and Navigation Series.
     Joseph H. Yuen, Series Editor. Hoboken, N.J.: Wiley, 2003.

[F4] Theodore D. Moyer. Formulation for Observed and Computed Values of Deep Space
     Network Data Types for Navigation. JPL Deep-Space Communications and Navigation
     Series. Joseph H. Yuen, Series Editor. Hoboken, N.J.: Wiley, 2003.

[F5] Organization and Processes for the Consultative Committee for Space Data Systems.
     Issue 4. CCSDS Record (Yellow Book), CCSDS A02.1-Y-4. Washington, D.C.:
     CCSDS, April 2014.

[F6] CCSDS Implementation Conformance Statements. Issue 1. CCSDS Record (Yellow
     Book), CCSDS A20.1-Y-1. Washington, D.C.: CCSDS, April 2014.

[F7] Navigation Data—Definitions and Conventions. Issue 4. Report Concerning Space
     Data System Standards (Green Book), CCSDS 500.0-G-4. Washington, D.C.: CCSDS,
     November 2019.

.. _annex_g:

ANNEX G
=======

RATIONALE FOR TRACKING DATA MESSAGES
====================================

(INFORMATIVE)

G1 GENERAL
----------

This annex presents the rationale behind the design of the Tracking Data Message. It may
help the application engineer construct a suitable message. Corrections and/or additions to
these requirements may occur during future updates.

A specification of requirements agreed to by all parties is essential to focus design and to
ensure the product meets the needs of the Member Agencies. There are many ways of
organizing requirements, but the categorization of requirements is not as important as the
agreement to a sufficiently comprehensive set. In this section, the requirements are organized
into three categories:

**Primary Requirements** - These are the most elementary and necessary requirements. They
would exist no matter the context in which the CCSDS is operating, that is, regardless of pre-
existing conditions within the CCSDS or its Member Agencies.

**Heritage Requirements** - These are additional requirements that derive from pre-existing
Member Agency requirements, conditions, or needs. Ultimately these carry the same weight
as the Primary Requirements. This Recommended Standard reflects heritage requirements
pertaining to some of the technical participants’ home institutions collected during the
preparation of the Recommended Standard; it does not speculate on heritage requirements
that could arise from other Member Agencies.

**Desirable Characteristics** - These are not requirements, but they are felt to be important or
useful features of the Recommended Standard.

G2 PRIMARY REQUIREMENTS ACCEPTED FOR TRACKING DATA MESSAGES
------------------------------------------------------------

.. list-table:: Table G-1: Primary Requirements
   :widths: 10 40 40 10
   :header-rows: 1

   * - ID
     - Requirement
     - Rationale
     - Trace
   * - G-1-1
     - Data must be provided in digital form.
     - Facilitates computerized processing of TDMs.
     - 3.1.1
   * - G-1-2
     - The object being tracked must be clearly identified and unambiguous.
     - Ensures proper processing of the tracking data in orbit determination.
     - 3.3
   * - G-1-3
     - All primary resources used in the tracking session must be clearly identified and unambiguous.
     - Ensures proper processing of the tracking data in orbit determination.
     - 3.3
   * - G-1-4
     - Time measurements (time stamps, timetags, or epochs) must be provided in a commonly used, clearly specified system.
     - The CCSDS objective of promoting interoperability is not met if time measurements are produced in esoteric or proprietary time systems.
     - 3.3, annex B
   * - G-1-5
     - The time bounds of the tracking data must be unambiguously specified.
     - The accuracy of orbit determination is highly dependent on accurately knowing the time at which measurements are taken.
     - 3.3, 3.4
   * - G-1-6
     - Tracking Data Messages must have means of being uniquely identified and clearly annotated.
     - If discussions of tracking file content are necessary, parties can ensure they are speaking of the same data.
     - 3.2
   * - G-1-7
     - The Tracking Data Message format shall be independent of the equipment that was used to perform the tracking.
     - The producer of a Tracking Data Message has local-network knowledge that may not be available to the user of the data.
     - 3.4
   * - G-1-8
     - Every tracking instrument shall have a defined reference location that could be defined in the ODM format, possibly extended to define spacecraft body-fixed axis. This reference location should not depend on the observing geometry.
     - The accuracy of orbit determination is highly dependent on accurately knowing the location of the tracking instruments.
     - 3.4
   * - G-1-9
     - The timetag of the tracking data shall always be unambiguously specified with respect to the measurement point or instrument reference point.
     - The accuracy of orbit determination is highly dependent on accurately knowing the time at which measurements are taken.
     - 3.4
   * - G-1-10
     - The observable shall be corrected with the best estimate of all known tracking instrument calibrations, such as pass-specific path delay calibrations between the reference point and the tracking equipment, if applicable.
     - The producer of a Tracking Data Message has knowledge of his or her network that may not be available to the user of the data.
     - 3.4
   * - G-1-11
     - The observable shall be converted to an equipment-independent quantity; for example, frequencies shall be reported at the ‘sky level’ (i.e., actual transmitted/received frequencies).
     - The producer of a Tracking Data Message has knowledge of the details of the equipment in his or her network that may not be available to the user of the data.
     - 3.4
   * - G-1-12
     - The data transfer mechanism shall not place constraints on the tracking data content.
     - The tracking data measurements are taken prior to transfer from originator to user, so data content should not be affected.
     - 3.1.7
   * - G-1-13
     - The standard must provide for clear specification of units of measure.
     - Without clear specification of units of measure, mistakes can be made that involve the unit system in effect (e.g., Metric or Imperial) and/or orders of magnitude (e.g., meters or kilometers).
     - 4.4, table 3-5

.. list-table:: Table G-2: Heritage Requirements
   :widths: 10 50 40
   :header-rows: 1

   * - ID
     - Requirement
     - Rationale
   * - G-2-1
     - The standard shall be, or must include, an ASCII format.
     - ASCII character-based messages promote interoperability. ASCII messages are useful in transferring data between heterogeneous computing systems, because the ASCII character set is nearly universally used and is interpretable by all popular systems. In addition, direct human-readable dumps of text to displays, emails, documents or printers are possible without preprocessing.
   * - G-2-2
     - The standard shall not require software supplied by other agencies.
     - Provides the greatest flexibility to both the originator of a tracking data message and the consumer of the data.

.. list-table:: Table G-3: Desirable Characteristics
   :widths: 10 45 45
   :header-rows: 1

   * - ID
     - Requirement
     - Rationale
   * - G-3-1
     - The standard should apply to non-traditional objects, such as landers, rovers, balloons, spacecraft-spacecraft tracking data exchange, etc.
     - There are many different types of spacecraft that are tracked by space agencies. The broader the applicability of the standard, the more useful it will be.
   * - G-3-2
     - The standard should be extensible with no disruption to existing users/uses.
     - Space agencies and operators upgrade systems and processes on schedules that make sense for their organizations. In practice, some organizations will be early adopters but others will opt to wait until performance of a new version of the TDM has been proven in other operations facilities.
   * - G-3-3
     - Keywords, values, and terminology in the TDM should be the same as those in the other CCSDS standards, where applicable.
     - Helps to ensure similar ‘look and feel’ across the various CCSDS flight dynamics standards.
   * - G-3-4
     - The standard shall not preclude an XML implementation.
     - The CCSDS Management Council (CMC) has indicated that the Navigation Working Group must produce standards that can be represented in XML.
   * - G-3-5
     - Other corrections applied to the data, such as media corrections, should be agreed upon by the service-providing and the customer Agencies via an ICD.
     - The user of the data must know what types of corrections and calibrations have been applied to the data in order to process it correctly.

.. _annex_h:

ANNEX H
=======

ABBREVIATIONS AND ACRONYMS
==========================

(INFORMATIVE)

.. list-table::
   :widths: 20 80
   :header-rows: 0

   * - ADM
     - Attitude Data Message
   * - ASCII
     - American Standard Code for Information Interchange
   * - AU
     - Astronomical Unit
   * - AZEL
     - Azimuth-Elevation
   * - CCIR
     - International Coordinating Committee for Radio Frequencies
   * - CCSDS
     - Consultative Committee for Space Data Systems
   * - CMC
     - CCSDS Management Council
   * - Delta-DOR
     - Delta Differential One-Way Ranging
   * - DOR
     - Differential One-Way Ranging
   * - DORIS
     - Doppler Orbitography and Radiopositioning Integrated by Satellite
   * - GNSS
     - Global Navigation Satellite System
   * - GPS
     - Global Positioning System
   * - ICD
     - Interface Control Document
   * - ICRF
     - International Celestial Reference Frame
   * - ICS
     - Implementation Conformance Statement
   * - ID
     - Identifier
   * - IEEE
     - Institute of Electrical and Electronics Engineers
   * - IEC
     - International Electrotechnical Commission
   * - ISO
     - International Organization for Standardization
   * - K
     - Kelvin
   * - KVN
     - Keyword = Value Notation
   * - LIDAR
     - Light Detection and Ranging
   * - MOIMS
     - Mission Operations and Information Management Services
   * - N/A or n/a
     - Not Applicable / Not Available
   * - NDM
     - Navigation Data Message
   * - ODM
     - Orbit Data Message
   * - OEM
     - Orbit Ephemeris Message
   * - OPM
     - Orbit Parameter Message
   * - Pc/N0
     - Carrier Power to Noise Spectral Density ratio
   * - Pr/N0
     - Ranging Power to Noise Spectral Density ratio
   * - PRARE
     - Precise Range and Range Rate Equipment
   * - RADEC
     - Right Ascension-Declination
   * - RCS
     - Radar Cross Section
   * - RINEX
     - Receiver Independent Exchange
   * - RL
     - Requirements List
   * - RTLT
     - Round-Trip Light Time
   * - RU
     - Range Units
   * - SANA
     - Space Assigned Numbers Authority
   * - SCLK
     - Spacecraft Clock
   * - SFTP
     - Secure File Transfer Protocol
   * - SI
     - Système Interational (SI Units)
   * - SLR
     - Satellite Laser Ranging
   * - STEC
     - Slant Total Electron Count
   * - TDM
     - Tracking Data Message
   * - TDR
     - Tracking Data Record
   * - TEC
     - Total Electron Count
   * - TECU
     - Total Electron Count Units
   * - UTC
     - Coordinated Universal Time
   * - VLBI
     - Very Long Baseline Interferometry
   * - XEYN
     - X:East, Y:North
   * - XSYE
     - X:South, Y:East
   * - XML
     - eXtensible Markup Language

.. _annex_i:

ANNEX I
=======

TDM SUMMARY SHEET
=================

(INFORMATIVE)

The tables in the following pages of this annex show the association between data types and
metadata keywords. There are only a few required metadata keywords, but many more that
are applicable to one or more of the various data types. Additionally, there are some
keywords that are only applicable in certain restricted situations. Finally, there are some
metadata keywords that are completely optional. This summary may assist the user in
constructing a TDM that captures the data from a specific measurement session.

.. list-table:: TDM Summary Sheet - 1. MODE = SEQUENTIAL, a) constant uplink frequency
   :widths: 10 20 20 20 20 20
   :header-rows: 2

   * - 
     - Range Data
     - Doppler Data
     - 
     - 
     - 
   * - Data Keywords [unit]
     - RANGE [km, s, or RU]
     - DOPPLER_INSTANTANEOUS [km/s]
     - RECEIVE_FREQ_n [Hz]
       TRANSMIT_FREQ_n [Hz]
       RECEIVE_PHASE_CT_n
       TRANSMIT_PHASE_CT_n
     - DOPPLER_INTEGRATED [km/s]
     - DOPPLER_COUNT

   * - Required Metadata
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH
       - TIME_SYSTEM
       - RANGE_MODE
       - RANGE_MODULUS
       - RANGE_UNITS
       - INTEGRATION_REF
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH
       - TIME_SYSTEM
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH
       - TIME_SYSTEM
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH
       - TIME_SYSTEM
       - INTEGRATION_INTERVAL
       - INTEGRATION_REF
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH
       - TIME_SYSTEM
       - DOPPLER_COUNT_SCALE
       - DOPPLER_COUNT_BIAS
       - DOPPLER_COUNT_ROLLOVER

   * - Situationally Required Metadata
     - - TRANSMIT_DELAY_n
       - RECEIVE_DELAY_n
       - TURNAROUND_NUMERATOR
       - TURNAROUND_DENOMINATOR
       - DATA_QUALITY
       - CORRECTIONS_APPLIED
       - CORRECTION_RANGE
       - TIMETAG_REF
       - PR_NO
     - - TRANSMIT_FREQ_n *
       - RECEIVE_FREQ *
       - TRANSMIT_DELAY_n
       - RECEIVE_DELAY_n
       - TURNAROUND_NUMERATOR
       - TURNAROUND_DENOMINATOR
       - DATA_QUALITY
       - CORRECTIONS_APPLIED
       - CORRECTION_DOPPLER
       - TIMETAG_REF
       - CARRIER_POWER
       - PC_NO
     - - TRANSMIT_DELAY_n
       - RECEIVE_DELAY_n
       - TURNAROUND_NUMERATOR
       - TURNAROUND_DENOMINATOR
       - DATA_QUALITY
       - CORRECTIONS_APPLIED
       - CORRECTION_TRANSMIT
       - CORRECTION_RECEIVE
       - INTEGRATION_INTERVAL
       - INTEGRATION_REF
       - FREQ_OFFSET
       - CARRIER_POWER
       - PC_NO
     - - TRANSMIT_FREQ_n *
       - RECEIVE_FREQ *
       - TRANSMIT_DELAY_n
       - RECEIVE_DELAY_n
       - TURNAROUND_NUMERATOR
       - TURNAROUND_DENOMINATOR
       - DATA_QUALITY
       - CORRECTIONS_APPLIED
       - CORRECTION_DOPPLER
       - TIMETAG_REF
       - CARRIER_POWER
       - PC_NO
     - - TRANSMIT_DELAY_n
       - RECEIVE_DELAY_n
       - TURNAROUND_NUMERATOR
       - TURNAROUND_DENOMINATOR
       - DATA_QUALITY
       - CORRECTIONS_APPLIED
       - CORRECTION_DOPPLER
       - TIMETAG_REF

   * - Optional Metadata
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME
       - TRANSMIT_BAND
       - RECEIVE_BAND
       - INTEGRATION_INTERVAL
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME
       - TRANSMIT_BAND
       - RECEIVE_BAND
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME
       - TRANSMIT_BAND
       - RECEIVE_BAND
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME
       - TRANSMIT_BAND
       - RECEIVE_BAND
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME
       - TRANSMIT_BAND
       - RECEIVE_BAND
       - CARRIER_POWER
       - PC_NO

.. The TRANSMIT_FREQ_n and RECEIVE_FREQ keywords are TDM Data Section keywords that are recommended to be exchanged for this data type. (See 3.5.2.2 and 3.5.2.3.)

.. list-table:: TDM Summary Sheet - 1. MODE = SEQUENTIAL, b) changing uplink
   :widths: 10 30 30 30
   :header-rows: 2

   * - 
     - Range Data
     - Doppler Data
     - 
   * - Data Keywords [unit]
     - RANGE [km, s, or RU]
     - RECEIVE_FREQ_n [Hz]
       TRANSMIT_FREQ_n [Hz]
       TRANSMIT_FREQ_RATE_n [Hz/s]
       RECEIVE_PHASE_CT_n
       TRANSMIT_PHASE_CT_n
     - DOPPLER_COUNT

   * - Required Metadata
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH
       - TIME_SYSTEM
       - RANGE_MODE
       - RANGE_MODULUS
       - RANGE_UNITS
       - INTEGRATION_REF
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH
       - TIME_SYSTEM
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH
       - TIME_SYSTEM
       - DOPPLER_COUNT_SCALE
       - DOPPLER_COUNT_BIAS
       - DOPPLER_COUNT_ROLLOVER

   * - Situationally Required Metadata
     - - TRANSMIT_DELAY_n
       - RECEIVE_DELAY_n
       - TURNAROUND_NUMERATOR
       - TURNAROUND_DENOMINATOR
       - DATA_QUALITY
       - CORRECTIONS_APPLIED
       - CORRECTION_RANGE
       - TIMETAG_REF
       - PR_NO
     - - TRANSMIT_DELAY_n
       - RECEIVE_DELAY_n
       - TURNAROUND_NUMERATOR
       - TURNAROUND_DENOMINATOR
       - DATA_QUALITY
       - CORRECTIONS_APPLIED
       - CORRECTION_TRANSMIT
       - CORRECTION_RECEIVE
       - INTEGRATION_INTERVAL
       - INTEGRATION_REF
       - FREQ_OFFSET
       - INTERPOLATION
       - INTERPOLATION_DEGREE
       - CARRIER_POWER
       - PC_NO
     - - TRANSMIT_DELAY_n
       - RECEIVE_DELAY_n
       - TURNAROUND_NUMERATOR
       - TURNAROUND_DENOMINATOR
       - DATA_QUALITY
       - CORRECTIONS_APPLIED
       - CORRECTION_DOPPLER
       - TIMETAG_REF

   * - Optional Metadata
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME
       - TRANSMIT_BAND
       - RECEIVE_BAND
       - INTEGRATION_INTERVAL
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME
       - TRANSMIT_BAND
       - RECEIVE_BAND
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME
       - TRANSMIT_BAND
       - RECEIVE_BAND
       - CARRIER_POWER
       - PC_NO

.. list-table:: TDM Summary Sheet - 1. MODE = SEQUENTIAL, c) Frequency independent
   :widths: 10 30 30 30
   :header-rows: 2

   * - 
     - Angle Data
     - Media Related Data
     - Optical Data
   * - Data Keywords [unit]
     - ANGLE_1, ANGLE_2 [deg]
     - STEC [TECU]
     - MAG, RCS [m**2]

   * - Required Metadata
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH
       - TIME_SYSTEM
       - ANGLE_TYPE
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH
       - TIME_SYSTEM
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH
       - TIME_SYSTEM

   * - Situationally Required Metadata
     - - DATA_QUALITY
       - CORRECTIONS_APPLIED
       - CORRECTION_ANGLE_1
       - CORRECTION_ANGLE_2
       - REFERENCE_FRAME
     - DATA_QUALITY
     - - TRACK_ID
       - REFERENCE_FRAME
       - DATA_QUALITY
       - CORRECTIONS_APPLIED
       - CORRECTION_MAG
       - CORRECTION_RCS
       - CORRECTION_ABERRATION_YEARLY
       - CORRECTION_ABERRATION_DIURNAL

   * - Optional Metadata
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME
       - RECEIVE_BAND
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME
     - - COMMENT
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME

.. list-table:: TDM Summary Sheet - 2. MODE = SINGLE_DIFF
   :widths: 10 20 20 20 20
   :header-rows: 2

   * - 
     - Range Data
     - Doppler Data
     - VLBI Data
     - 
   * - Data Keywords [unit]
     - RANGE [km, s, or RU]
     - RECEIVE_FREQ_n [Hz]
       TRANSMIT_FREQ_n [Hz]
       TRANSMIT_FREQ_RATE_n [Hz/s]
       RECEIVE_PHASE_CT_n
       TRANSMIT_PHASE_CT_n
     - DOR [s]
     - VLBI_DELAY [s]

   * - Required Metadata
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH_1
       - PATH_2
       - TIME_SYSTEM
       - TRANSMIT_BAND
       - RECEIVE_BAND
       - RANGE_MODE
       - RANGE_MODULUS
       - RANGE_UNITS
       - INTEGRATION_REF
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH_1
       - PATH_2
       - TIME_SYSTEM
       - TRANSMIT_BAND
       - RECEIVE_BAND
       - FREQ_OFFSET
       - INTERPOLATION
       - INTERPOLATION_DEGREE
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH_1
       - PATH_2
       - TIME_SYSTEM
       - TRANSMIT_BAND
       - RECEIVE_BAND
       - RANGE_MODE
       - RANGE_MODULUS
       - TIMETAG_REF
     - - META_START
       - META_STOP
       - MODE
       - PARTICIPANT_n
       - PATH_1
       - PATH_2
       - TIME_SYSTEM
       - TRANSMIT_BAND
       - RECEIVE_BAND
       - RANGE_MODE
       - RANGE_MODULUS
       - TIMETAG_REF

   * - Situationally Required Metadata
     - - TRANSMIT_DELAY_n
       - RECEIVE_DELAY_n
       - TURNAROUND_NUMERATOR
       - TURNAROUND_DENOMINATOR
       - DATA_QUALITY
       - CORRECTIONS_APPLIED
       - CORRECTION_RANGE
       - TIMETAG_REF
       - PR_NO
     - - TRANSMIT_DELAY_n
       - RECEIVE_DELAY_n
       - TURNAROUND_NUMERATOR
       - TURNAROUND_DENOMINATOR
       - DATA_QUALITY
       - CORRECTIONS_APPLIED
       - CORRECTION_TRANSMIT
       - CORRECTION_RECEIVE
       - INTEGRATION_INTERVAL
       - INTEGRATION_REF
       - FREQ_OFFSET
       - CARRIER_POWER
       - PC_NO
     - - TRANSMIT_DELAY_n
       - RECEIVE_DELAY_n
       - DATA_QUALITY
     - - TRANSMIT_DELAY_n
       - RECEIVE_DELAY_n
       - DATA_QUALITY

   * - Optional Metadata
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME
       - INTEGRATION_INTERVAL
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME
       - RANGE_UNITS
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
       - EPHEMERIS_NAME
       - RANGE_UNITS

.. list-table:: TDM Summary Sheet - 3. MODE = Not applicable, not specified
   :widths: 10 30 30 30
   :header-rows: 2

   * - 
     - Time Data
     - Media Related Data
     - Meteorological Data
   * - Data Keywords [unit]
     - CLOCK_BIAS [s]
       CLOCK_DRIFT [s]
     - TROPO_DRY/TROPO_WET [m]
     - PRESSURE [hPa]
       RHUMIDITY [%]
       TEMPERATURE [K]

   * - Required Metadata
     - - META_START
       - META_STOP
       - PARTICIPANT_n
       - TIME_SYSTEM
     - - META_START
       - META_STOP
       - PARTICIPANT_n
       - TIME_SYSTEM
     - - META_START
       - META_STOP
       - PARTICIPANT_n
       - TIME_SYSTEM

   * - Situationally Required Metadata
     - DATA_QUALITY
     - DATA_QUALITY
     - DATA_QUALITY

   * - Optional Metadata
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
     - - COMMENT
       - TRACK_ID
       - DATA_TYPES
       - START_TIME
       - STOP_TIME
