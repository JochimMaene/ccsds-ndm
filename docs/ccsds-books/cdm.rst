.. title:: CONJUNCTION DATA MESSAGE

.. rubric:: RECOMMENDED STANDARD

.. centered:: CCSDS 508.0-B-1

.. centered:: BLUE BOOK
.. centered:: June 2013

.. centered::
   Note:
   This current
   issue includes
   all updates through
   Technical Corrigendum 2,
   dated October 2021


AUTHORITY
=========

.. table::
   :align: center

   +----------+--------------------------------+
   | Issue:   | Recommended Standard, Issue 1  |
   +----------+--------------------------------+
   | Date:    | June 2013                      |
   +----------+--------------------------------+
   | Location:| Washington, DC, USA            |
   +----------+--------------------------------+

This document has been approved for publication by the Management Council of the
Consultative Committee for Space Data Systems (CCSDS) and represents the consensus
technical agreement of the participating CCSDS Member Agencies. The procedure for
review and authorization of CCSDS documents is detailed in Organization and Processes for
the Consultative Committee for Space Data Systems (CCSDS A02.1-Y-3), and the record of
Agency participation in the authorization of this document can be obtained from the CCSDS
Secretariat at the address below.

This document is published and maintained by:

   CCSDS Secretariat
   Space Communications and Navigation Office, 7L70
   Space Operations Mission Directorate
   NASA Headquarters
   Washington, DC 20546-0001, USA


STATEMENT OF INTENT
===================

The Consultative Committee for Space Data Systems (CCSDS) is an organization officially
established by the management of its members. The Committee meets periodically to address
data systems problems that are common to all participants, and to formulate sound technical
solutions to these problems. Inasmuch as participation in the CCSDS is completely
voluntary, the results of Committee actions are termed Recommended Standards and are
not considered binding on any Agency.

This Recommended Standard is issued by, and represents the consensus of, the CCSDS
members. Endorsement of this Recommendation is entirely voluntary. Endorsement,
however, indicates the following understandings:

-  Whenever a member establishes a CCSDS-related standard, this standard will be in
   accord with the relevant Recommended Standard. Establishing such a standard
   does not preclude other provisions which a member may develop.
-  Whenever a member establishes a CCSDS-related standard, that member will
   provide other CCSDS members with the following information:

   -  The standard itself.
   -  The anticipated date of initial operational capability.
   -  The anticipated duration of operational service.

-  Specific service arrangements shall be made via memoranda of agreement. Neither
   this Recommended Standard nor any ensuing standard is a substitute for a
   memorandum of agreement.

No later than three years from its date of issuance, this Recommended Standard will be
reviewed by the CCSDS to determine whether it should: (1) remain in effect without change;
(2) be changed to reflect the impact of new technologies, new requirements, or new
directions; or (3) be retired or canceled.

In those instances when a new version of a Recommended Standard is issued, existing
CCSDS-related member standards and implementations are not negated or deemed to be
non-CCSDS compatible. It is the responsibility of each member to determine when such
standards or implementations are to be modified. Each member is, however, strongly
encouraged to direct planning for its new standards and implementations towards the later
version of the Recommended Standard.


FOREWORD
========

This document is a Recommended Standard for Conjunction Data Messages (CDMs) and has
been prepared by the CCSDS. The CDM described in this Recommended Standard is the
baseline concept for conjunction information interchange applications between interested
parties.

This Recommended Standard establishes a common framework and provides a common
basis for the format of conjunction information exchange between originators of conjunction
assessment data and satellite owner/operators. It allows implementing organizations within
each conjunction assessment originator to proceed coherently with the development of
compatible derived standards for the flight and ground systems that are within their
cognizance. Derived Agency standards can implement only a subset of the optional features
allowed by the Recommended Standard and can incorporate features not addressed by this
Recommended Standard.

Attention is drawn to the possibility that some of the elements of this document may be the
subject of patent rights. CCSDS shall not be held responsible for identifying any or all such
patent rights.

Through the process of normal evolution, it is expected that expansion, deletion, or
modification of this document may occur. This Recommended Standard is therefore subject
to CCSDS document management and change control procedures, which are defined in
Organization and Processes for the Consultative Committee for Space Data Systems
(CCSDS A02.1-Y-3). Current versions of CCSDS documents are maintained at the CCSDS
Web site:

   http://www.ccsds.org/

Questions relating to the contents or status of this document should be addressed to the
CCSDS Secretariat at the address indicated on page i.


At time of publication, the active Member and Observer Agencies of the CCSDS were:

Member Agencies
---------------

-  Agenzia Spaziale Italiana (ASI)/Italy.
-  Canadian Space Agency (CSA)/Canada.
-  Centre National d'Etudes Spatiales (CNES)/France.
-  China National Space Administration (CNSA)/People's Republic of China.
-  Deutsches Zentrum für Luft- und Raumfahrt e. V. (DLR)/Germany.
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
-  China Satellite Launch and Tracking Control General, Beijing Institute of Tracking
   and Telecommunications Technology (CLTC/BITTT)/China.
-  Chinese Academy of Sciences (CAS)/China.
-  Chinese Academy of Space Technology (CAST)/China.
-  Commonwealth Scientific and Industrial Research Organization (CSIRO)/Australia.
-  CSIR Satellite Applications Centre (CSIR)/Republic of South Africa.
-  Danish National Space Center (DNSC)/Denmark.
-  Departamento de Ciência e Tecnologia Aeroespacial (DCTA)/Brazil.
-  European Organization for the Exploitation of Meteorological Satellites
   (EUMETSAT)/Europe.
-  European Telecommunications Satellite Organization (EUTELSAT)/Europe.
-  Geo-Informatics and Space Technology Development Agency (GISTDA)/Thailand.
-  Hellenic National Space Committee (HNSC)/Greece.
-  Indian Space Research Organization (ISRO)/India.
-  Institute of Space Research (IKI)/Russian Federation.
-  KFKI Research Institute for Particle & Nuclear Physics (KFKI)/Hungary.
-  Korea Aerospace Research Institute (KARI)/Korea.
-  Ministry of Communications (MOC)/Israel.
-  National Institute of Information and Communications Technology (NICT)/Japan.
-  National Oceanic and Atmospheric Administration (NOAA)/USA.
-  National Space Agency of the Republic of Kazakhstan (NSARK)/Kazakhstan.
-  National Space Organization (NSPO)/Chinese Taipei.
- Naval Center for Space Technology (NCST)/USA.
-  Scientific and Technological Research Council of Turkey (TUBITAK)/Turkey.
-  Space and Upper Atmosphere Research Commission (SUPARCO)/Pakistan.
-  Swedish Space Corporation (SSC)/Sweden.
-  United States Geological Survey (USGS)/USA.

DOCUMENT CONTROL
================

.. list-table::
   :widths: 25 25 25 25
   :header-rows: 1

   * - Document
     - Title
     - Date
     - Status
   * - CCSDS 508.0-B-1
     - Conjunction Data Message, Recommended Standard, Issue 1
     - June 2013
     - Current issue
   * - CCSDS 508.0-B-1 EC 1
     - Editorial change 1
     - March 2014
     - Corrects broken hyperlinks; updates references to superseded documents; updates obsolete style elements.
   * - CCSDS 508.0-B-1 EC 2
     - Editorial change 2
     - February 2018
     - Addresses minor typographical and layout issues.
   * - CCSDS 508.0-B-1 Cor. 1
     - Technical Corrigendum 1
     - June 2018
     - Adds direction to use Space Assigned Numbers Authority (SANA) registry values.
   * - CCSDS 508.0-B-1 Cor. 2
     - Technical Corrigendum 2
     - October 2021
     - Adds specifications and examples for messages with elements qualified with respect to a namespace.


.. _introduction_cdm:

1 INTRODUCTION
==============

.. _introduction_purpose_and_scope_cdm:

1.1 PURPOSE AND SCOPE
---------------------

This Conjunction Data Message (CDM) Recommended Standard specifies a standard
message format for use in exchanging spacecraft conjunction information between
originators of Conjunction Assessments (CAs) and satellite owner/operators and other
authorized parties. Such exchanges are used to inform satellite owner/operators of
conjunctions between objects in space to enable consistent warning by different
organizations employing diverse CA techniques.

This Recommended Standard will:

a) facilitate interoperability and enable consistent warning between data originators who
   supply CA and the satellite owner/operators who use it;
b) facilitate automation for the CA processes; and
c) provide critical information to enable timely CA decisions.

This document includes requirements and criteria that the message format has been designed
to meet (see annex D). Also included are informative descriptions of conjunction information
pertinent to performing CA (see annex E).

.. _introduction_applicability_cdm:

1.2 APPLICABILITY
-----------------

This Recommended Standard is applicable to satellite operations in all environments in
which close approaches and collisions among satellites are concerns. It contains the
specification for a CDM designed for applications involving conjunction information
interchange between originators of CAs and recipients. Conjunction information includes
data types such as miss distance, probability of collision, Time of Closest Approach (TCA),
and closest approach relative position and velocity. Further information describing the
conjunction information contained in this message can be found in section 3 and annex E.

This message is suited for exchanges that involve manual or automated interaction. The
attributes of a CDM make it suitable for use in machine-to-machine interfaces because of the
large amount of data typically present. The CDM is self contained. However, additional
information could be specified in an Interface Control Document (ICD) written jointly by the
service originator and recipients.

It is desirable that CDM originators maintain consistency with respect to the optional
keywords provided in their implementations; i.e., it is desirable that the composition of the
CDMs provided not change on a frequent basis.

This Recommended Standard is applicable only to the message format and content, but not to
its transmission nor to the algorithms used to produce the data within. The method of
transmitting the message between exchange partners is beyond the scope of this document
and could be specified in an ICD.

The methods used to predict conjunctions and calculate the probability of collision, and the
definition of the conjunction assessment accuracy underlying a particular CDM, are also
outside the scope of this Recommended Standard (the interested reader can consult
references in annex F).

.. _introduction_document_structure_cdm:

1.3 DOCUMENT STRUCTURE
----------------------

Section 2 provides a brief overview of the CCSDS-recommended CDM.

Section 3 provides details about the structure and content of the CDM in ‘Keyword = Value
Notation' (KVN).

Section 4 provides details about the structure and content of the CDM in eXtensible Markup
Language (XML).

Section 5 addresses the CDM data in general.

Section 6 discusses the syntax considerations of the CDM.

Annex A contains an Implementation Conformance Statement (ICS) proforma that may be
used by implementers to compactly describe their implementations.

Annex B provides information on security, the Space Assigned Numbers Authority (SANA),
and patent-related information.

Annex C is a list of abbreviations and acronyms applicable to the CDM.

Annex D provides rationale and requirements for the CDM Recommended Standard.

Annex E provides a description of the CA information contained in the CDM.

Annex F provides informative references.

.. _introduction_conventions_and_definitions_cdm:

1.4 CONVENTIONS AND DEFINITIONS
-------------------------------

.. _introduction_notation_cdm:

1.4.1 NOTATION
^^^^^^^^^^^^^^

.. _introduction_unit_notations_cdm:

1.4.1.1 Unit Notations
""""""""""""""""""""""

The following conventions for unit notations apply throughout this Recommended Standard.
Insofar as possible, an effort has been made to use units that are part of the International
System of Units (SI); units are either SI base units, SI derived units, or units outside the SI
that are accepted for use with the SI (see reference [1]). The units used within this document
are as follows:

-  km: kilometers;
-  m: meters;
-  d: days, 86400 SI seconds;
-  s: SI seconds;
-  kg: kilograms;
-  W: watts;
-  %: percent.

.. _introduction_general_notation_cdm:

1.4.1.2 General
"""""""""""""""

The following notational conventions are used in this document:

a) multiplication of units is denoted with a single asterisk ‘*’ (e.g., ‘kg*s’);
b) exponents of units are denoted with a double asterisk ‘**’ (e.g., m\ :sup:`2` = m**2);
c) division of units is denoted with a single forward slash ‘/’ (e.g., m/s).

.. _introduction_nomenclature_cdm:

1.4.2 NOMENCLATURE
^^^^^^^^^^^^^^^^^^

.. _introduction_general_nomenclature_cdm:

1.4.2.1 General
"""""""""""""""

The CDM contains information about a conjunction between two space objects (hereafter
referred to as 'Object1' and 'Object2').

.. _introduction_normative_text_cdm:

1.4.2.2 Normative Text
""""""""""""""""""""""

The following conventions apply for the normative specifications in this Recommended
Standard:

a) the words 'shall' and 'must' imply a binding and verifiable specification;
b) the word 'should' implies an optional, but desirable, specification;
c) the word 'may' implies an optional specification;
d) the words 'is', 'are', and 'will' imply statements of fact.

.. NOTE::
   These conventions do not imply constraints on diction in text that is clearly
   informative in nature.

.. _introduction_informative_text_cdm:

1.4.2.3 Informative Text
""""""""""""""""""""""""

In the normative sections of this document (sections 3-6), informative text is set off from the
normative specifications either in notes or under one of the following subsection headings:

-  Overview;
-  Discussion.

.. _introduction_other_conventions_cdm:

1.4.3 OTHER CONVENTIONS
^^^^^^^^^^^^^^^^^^^^^^^

.. _introduction_terminology_cdm:

1.4.3.1 Terminology
"""""""""""""""""""""

In this document, the term ‘ASCII’is used generically to refer to the text character set
defined in reference [2]. The terms ‘N/A' and 'n/a' are defined to mean 'not available' or
'not applicable'.

.. _introduction_orthography_cdm:

1.4.3.2 Orthography
"""""""""""""""""""

The following terms define orthographic conventions for XML notation in this
Recommended Standard:

CamelCase
   A style of capitalization in which the initial characters of concatenated words
   are capitalized, as in CamelCase.

lowerCamelCase
   A variant of CamelCase in which the first character of a character string
   formed from concatenated words is lowercase, as in lowerCamelCase. In the case of a
   character string consisting of only a single word, only lowercase characters are used.

.. _introduction_references_cdm:

1.5 REFERENCES
--------------

The following publications contain provisions which, through reference in this text,
constitute provisions of this Recommended Standard. At the time of publication, the editions
indicated were valid. All publications are subject to revision, and users of this
Recommended Standard are encouraged to investigate the possibility of applying the most
recent editions of the publications indicated below. The CCSDS Secretariat maintains a
register of currently valid CCSDS publications.

[1] The International System of Units (SI). 8th ed. Sèvres, France: BIPM, 2006.
[2] Information Technology—8-Bit Single-Byte Coded Graphic Character Sets—Part 1:
    Latin Alphabet No. 1. International Standard, ISO/IEC 8859-1:1998. Geneva: ISO,
    1998.
[3] Henry S. Thompson, et al., eds. XML Schema Part 1: Structures. 2nd ed. W3C
    Recommendation. N.p.: W3C, October 2004.
[4] Paul V. Biron and Ashok Malhotra, eds. XML Schema Part 2: Datatypes. 2nd ed. W3C
    Recommendation. N.p.: W3C, October 2004.
[5] Time Code Formats. Issue 4. Recommendation for Space Data System Standards (Blue
    Book), CCSDS 301.0-B-4. Washington, D.C.: CCSDS, November 2010.
[6] XML Specification for Navigation Data Messages. Issue 2. Recommendation for Space
    Data System Standards (Blue Book), CCSDS 505.0-B-2. Washington, D.C.: CCSDS,
    May 2021.


.. _overview_cdm:

2 OVERVIEW
==========

.. _overview_general_cdm:

2.1 GENERAL
-----------

This section provides a high-level overview of the CCSDS-recommended CDM, a message
format designed to facilitate standardized exchange of conjunction information between
originators of CA data and satellite owner/operators.

.. _overview_basic_content_cdm:

2.2 CDM BASIC CONTENT
---------------------

The CDM is ASCII format encoded either in plain text or XML (see references [2], [3], and
[4]). This CDM document describes a KVN-formatted message as well as an XML-formatted
message (it is desirable that an ICD specify which of these formats will be exchanged).

The CDM contains information about a single conjunction between Object1 and Object2. It
contains

-  Object1/Object2 positions/velocities at TCA with respect to one of a small set of
   widely used reference frames (ITRF, GCRF—see reference [F11], EME2000);
-  Object1/Object2 covariances at TCA with respect to an object centered reference
   frame;
-  the relative position/velocity of Object2 with respect to an Object1 centered reference
   frame;
-  information relevant to how all the above data was determined.

This information is used by satellite owner/operators to evaluate the risk of a conjunction and
plan maneuvers if warranted by that agency/organization. Where possible, the CDM is
consistent with other CCSDS Navigation Data Messages (NDMs). Similar tables have been
used to describe header, metadata, and data information. Common keywords have been used
in order to minimize duplication and confusion (e.g., CREATION_DATE, ORIGINATOR,
OBJECT_NAME, INTERNATIONAL_DESIGNATOR, etc.).



.. _kvn_content_structure_cdm:

3 CDM CONTENT/STRUCTURE IN KVN
==============================

.. _kvn_general_cdm:

3.1 GENERAL
-----------

3.1.1 The CDM in KVN shall consist of digital data represented as ASCII text lines. The
lines constituting a CDM shall be represented as a combination of the following:

a) a header;
b) relative metadata/data (metadata/data describing relative relationships between
   Object1 and Object2);
c) metadata (data about how Object1 and Object2 data were created);
d) data (for both Object1 and Object2); and
e) optional comments (explanatory information).

.. NOTE::

   1. KVN messages contain one keyword per line (see 6.3.1.4).
   2. The order of keywords in the KVN representation is fixed by this Recommended
      Standard (see 6.3.1.9).

3.1.2 The CDM shall be plain text consisting of CA data for a single conjunction event. It
shall be easily readable by both humans and computers.

3.1.3 The method of exchanging CDMs shall be decided on a case-by-case basis by the
participating parties and should be documented in an ICD.

.. _kvn_header_cdm:

3.2 CDM HEADER
--------------

The CDM header shall consist of the KVN elements defined in table 3-1, which specifies for
each KVN header item:

a) the keyword to be used;
b) a short description of the item;
c) examples of allowed values; and
d) whether the item is obligatory or optional.

.. _table_3-1_cdm:

.. list-table:: Table 3-1: CDM KVN Header
   :widths: 20 50 15 15
   :header-rows: 1

   * - Keyword
     - Description
     - Example of Values
     - Obligatory
   * - CCSDS_CDM_VERS
     - Format version in the form of 'x.y', where 'y' is incremented for corrections
       and minor changes, and 'x' is incremented for major changes.
     - 1.0
       2.0
     - Yes
   * - COMMENT
     - (See 6.3.4 for formatting rules.)
     - COMMENT This is a
       comment
     - No
   * - CREATION_DATE
     - Message creation date/time in Coordinated Universal Time (UTC).
       (See 6.3.2.6 for formatting rules.)
     - 2010-03-12T22:31:12.000
       2010-071T22:31:12.000
     - Yes
   * - ORIGINATOR
     - Creating agency or owner/operator. Value should be the 'Abbreviation' value
       from the SANA 'Organizations' registry
       (https://sanaregistry.org/r/organizations) for an organization that has the Role of
       'Conjunction Data Message Originator'. (See 5.2.9 for formatting rules.)
     - JSPOC, ESA SST,
       CAESAR, JPL, SDC
     - Yes
   * - MESSAGE_FOR
     - Spacecraft name(s) for which the CDM is provided.
     - SPOT, ENVISAT,
       IRIDIUM, INTELSAT
     - No
   * - MESSAGE_ID
     - ID that uniquely identifies a message from a given originator. The format and
       content of the message identifier value are at the discretion of the originator.
       (See 5.2.9 for formatting rules.)
     - 201113719185
       ABC-12_34
     - Yes

.. _kvn_relative_metadata_data_cdm:

3.3 CDM RELATIVE METADATA/DATA
------------------------------

The CDM relative metadata/data shall consist of the KVN elements defined in table 3-2,
which specifies for each KVN relative metadata/data item:

a) the keyword to be used;
b) a short description of the item;
c) the units to be used if applicable; and
d) whether the item is obligatory or optional.

.. _table_3-2_cdm:

.. list-table:: Table 3-2: CDM KVN Relative Metadata/Data
   :widths: 25 55 10 10
   :header-rows: 1

   * - Keyword
     - Description
     - Units
     - Obligatory
   * - COMMENT
     - (See 6.3.4 for formatting rules.)
     - n/a
     - No
   * - TCA
     - The date and time in UTC of the closest approach. (See 6.3.2.6 for formatting rules.)
     - n/a
     - Yes
   * - MISS_DISTANCE
     - The norm of the relative position vector. It indicates how close the two objects are at
       TCA. Data type = double.
     - m
     - Yes
   * - RELATIVE_SPEED
     - The norm of the relative velocity vector. It indicates how fast the two objects are moving
       relative to each other at TCA. Data type = double.
     - m/s
     - No
   * - RELATIVE_POSITION_R
     - The R component of Object2's position relative to Object1's position in the Radial,
       Transverse, and Normal (RTN) coordinate frame. (See annex E for definition.) Data type
       = double.
     - m
     - No
   * - RELATIVE_POSITION_T
     - The T component of Object2's position relative to Object1's position in the RTN
       coordinate frame. (See annex E for definition.) Data type = double.
     - m
     - No
   * - RELATIVE_POSITION_N
     - The N component of Object2's position relative to Object1's position in the RTN
       coordinate frame. (See annex E for definition.) Data type = double.
     - m
     - No
   * - RELATIVE_VELOCITY_R
     - The R component of Object2's velocity relative to Object1's velocity in the RTN
       coordinate frame. (See annex E for definition.) Data type = double.
     - m/s
     - No
   * - RELATIVE_VELOCITY_T
     - The T component of Object2's velocity relative to Object1's velocity in the RTN
       coordinate frame. (See annex E for definition.) Data type = double.
     - m/s
     - No
   * - RELATIVE_VELOCITY_N
     - The N component of Object2's velocity relative to Object1's velocity in the RTN
       coordinate frame. (See annex E for definition.) Data type = double.
     - m/s
     - No
   * - START_SCREEN_PERIOD
     - The start time in UTC of the screening period for the conjunction assessment. (See 6.3.2.6
       for formatting rules.)
     - n/a
     - No
   * - STOP_SCREEN_PERIOD
     - The stop time in UTC of the screening period for the conjunction assessment. (See 6.3.2.6
       for formatting rules.)
     - n/a
     - No
   * - SCREEN_VOLUME_FRAME
     - Name of the Object1 centered reference frame in which the screening volume data are given.
       Available options are RTN and Transverse, Velocity, and Normal (TVN). (See annex E for
       definition.)
     - n/a
     - No
   * - SCREEN_VOLUME_SHAPE
     - Shape of the screening volume: ELLIPSOID or BOX.
     - n/a
     - No
   * - SCREEN_VOLUME_X
     - The R or T (depending on if RTN or TVN is selected) component size of the screening
       volume in the SCREEN_VOLUME_FRAME. Data type = double.
     - m
     - No
   * - SCREEN_VOLUME_Y
     - The T or V (depending on if RTN or TVN is selected) component size of the screening
       volume in the SCREEN_VOLUME_FRAME. Data type = double.
     - m
     - No
   * - SCREEN_VOLUME_Z
     - The N component size of the screening volume in the SCREEN_VOLUME_FRAME.
       Data type = double.
     - m
     - No
   * - SCREEN_ENTRY_TIME
     - The time in UTC when Object2 enters the screening volume. (See 6.3.2.6 for formatting
       rules.)
     - n/a
     - No
   * - SCREEN_EXIT_TIME
     - The time in UTC when Object2 exits the screening volume. (See 6.3.2.6 for formatting
       rules.)
     - n/a
     - No
   * - COLLISION_PROBABILITY
     - The probability (denoted 'p' where 0.0<=p<=1.0), that Object1 and Object2 will
       collide. Data type = double.
     - n/a
     - No
   * - COLLISION_PROBABILITY_METHOD
     - The method that was used to calculate the collision probability. (See annex E for
       definition.)
     - n/a
     - No



.. _kvn_metadata_cdm:

3.4 CDM METADATA
----------------

The CDM metadata shall consist of the KVN elements defined in table 3-3, which specifies
for each KVN metadata item:

a) the keyword to be used;
b) a short description of the item;
c) normative values or examples of allowed values;
d) whether the ‘Normative Values/Examples' column contains normative values (N) or
   examples of allowed values (E) for the item; and
e) whether the item is obligatory or optional.

.. NOTE::
   Table 3-3 and table 3-4 will be used to define both Object1 and Object2
   depending on the value of the keyword OBJECT which is specified in table 3-3.

.. _table_3-3_cdm:

.. list-table:: Table 3-3: CDM KVN Metadata
   :widths: 20 45 20 5 10
   :header-rows: 1

   * - Keyword
     - Description
     - Normative Values/
       Examples
     - N/E
     - Obligatory
   * - COMMENT
     - (See 6.3.4 for formatting rules.)
     - COMMENT This is a
       comment
     - E
     - No
   * - OBJECT
     - The object to which the
       metadata and data apply
       (Object1 or Object2).
     - OBJECT1
       OBJECT2
     - N
     - Yes
   * - OBJECT_DESIGNATOR
     - The satellite catalog
       designator for the object. (See
       5.2.9 for formatting rules.)
     - 12345
     - E
     - Yes
   * - CATALOG_NAME
     - The satellite catalog used for
       the object. Value should be
       taken from the SANA
       'Conjunction Data Message
       CATALOG_NAME' registry
       (https://sanaregistry.org/r/cdm
       _catalog). (See 5.2.9 for
       formatting rules.)
     - SATCAT
     - E
     - Yes
   * - OBJECT_NAME
     - Spacecraft name for the
       object.
     - SPOT, ENVISAT,
       IRIDIUM, INTELSAT
     - E
     - Yes
   * - INTERNATIONAL_DESIGNATOR
     - The full international
       designator for the object.
       Values shall have the format
       YYYY-NNNP{PP}, where:
       YYYY = year of launch;
       NNN = three-digit serial
       number of launch (with
       leading zeros);
       P{PP} = At least one capital
       letter for the identification of
       the part brought into space by
       the launch. In cases where the
       object has no international
       designator, the value
       UNKNOWN should be used.
       (See 5.2.9 for further
       formatting rules.)
     - 2002-021A
       2002-009A
       1997-020AA
       1998-037ABC
       2001-049PE
       UNKNOWN
     - E
     - Yes
   * - OBJECT_TYPE
     - The object type.
     - PAYLOAD
       ROCKET BODY
       DEBRIS
       UNKNOWN
       OTHER
     - N
     - No
   * - OPERATOR_CONTACT_POSITION
     - Contact position of the
       owner/operator of the object.
     - ORBITAL SAFETY
       ANALYST (OSA),
       NETWORK
       CONTROLLER
     - E
     - No
   * - OPERATOR_ORGANIZATION
     - Contact organization of the
       object.
     - EUMETSAT, ESA,
       INTELSAT, IRIDIUM
     - E
     - No
   * - OPERATOR_PHONE
     - Phone number of the contact
       position or organization for the
       object.
     - +49615130312
     - E
     - No
   * - OPERATOR_EMAIL
     - Email address of the contact
       position or organization of the
       object.
     - JOHN.DOE@
       SOMEWHERE.NET
     - E
     - No
   * - EPHEMERIS_NAME
     - Unique name of the external
       ephemeris file used for the
       object or NONE. This is used
       to indicate whether an external
       (i.e., Owner/Operator [O/O]
       provided) ephemeris file was
       used to calculate the CA. If
       'NONE' is specified, then the
       output of the most current
       Orbit Determination (OD) of
       the CDM originator was used
       in the CA.
     - EPHEMERIS
       SATELLITE A,
       NONE
     - E
     - Yes
   * - COVARIANCE_METHOD
     - Method used to calculate the
       covariance during the OD that
       produced the state vector, or
       whether an arbitrary, non-
       calculated default value was
       used. Caution should be used
       when using the default value
       for calculating collision
       probability.
     - CALCULATED
       DEFAULT
     - N
     - Yes
   * - MANEUVERABLE
     - The maneuver capacity of the
       object. (See 1.4.3.1 for
       definition of 'N/A'.)
     - YES
       NO
       N/A
     - N
     - Yes
   * - ORBIT_CENTER
     - The central body about which
       Object1 and Object2 orbit. If
       not specified, the center is
       assumed to be Earth.
     - EARTH
       SUN
       MOON
       MARS
     - E
     - No
   * - REF_FRAME
     - Name of the reference frame
       in which the state vector data
       are given. Value must be
       selected from the list of values
       to the right (see reference [F1])
       and be the same for both
       Object1 and Object2.
     - GCRF (see reference
       [F11])
       EME2000
       ITRF
     - N
     - Yes
   * - GRAVITY_MODEL
     - The gravity model used for the
       OD of the object. (See annex
       E under GRAVITY_MODEL for
       definition).
     - EGM-96: 36D 360
       WGS-84_GEOID: 24D
       240
       JGM-2: 41D 410
     - E
     - No
   * - ATMOSPHERIC_MODEL
     - The atmospheric density
       model used for the OD of the
       object. If 'NONE' is specified,
       then no atmospheric model
       was used.
     - JACCHIA 70
       MSIS
       JACCHIA 70 DCA
       NONE
     - E
     - No
   * - N_BODY_PERTURBATIONS
     - The N-body gravitational
       perturbations used for the OD
       of the object. If 'NONE' is
       specified, then no third-body
       gravitational perturbations
       were used.
     - MOON, SUN
       JUPITER
       NONE
     - E
     - No
   * - SOLAR_RAD_PRESSURE
     - Indication of whether solar
       radiation pressure
       perturbations were used for the
       OD of the object.
     - YES
       NO
     - N
     - No
   * - EARTH_TIDES
     - Indication of whether solid
       Earth and ocean tides were
       used for the OD of the object.
     - YES
       NO
     - N
     - No
   * - INTRACK_THRUST
     - Indication of whether in-track
       thrust modeling was used for
       the OD of the object.
     - YES
       NO
     - N
     - No



.. _kvn_data_cdm:

3.5 CDM DATA
------------

3.5.1 The CDM Data section shall be formed as logical blocks:

-  OD Parameters;
-  Additional Parameters;
-  State Vector; and
-  Covariance Matrix.

3.5.2 The logical blocks of the CDM Data section shall consist of KVN elements as
defined in table 3-4, which specifies for each data item:

a) the keyword to be used;
b) a short description of the item;
c) the units to be used if applicable; and
d) whether the item is obligatory or optional.

.. _table_3-4_cdm:

.. list-table:: Table 3-4: CDM KVN Data
   :widths: 25 65 5 5
   :header-rows: 1

   * - Keyword
     - Description
     - Units
     - Obligatory
   * - COMMENT
     - (See 6.3.4 for formatting rules.)
     - n/a
     - No
   * - **OD Parameters**
     -
     -
     -
   * - COMMENT
     - (See 6.3.4 for formatting rules.)
     - n/a
     - No
   * - TIME_LASTOB_START
     - The start of a time interval (UTC) that contains the time of the last accepted
       observation. (See 6.3.2.6 for formatting rules.) For an exact time, the time interval is
       of zero duration (i.e., same value as that of TIME_LASTOB_END).
     - n/a
     - No
   * - TIME_LASTOB_END
     - The end of a time interval (UTC) that contains the time of the last accepted
       observation. (See 6.3.2.6 for formatting rules.) For an exact time, the time interval is
       of zero duration (i.e., same value as that of TIME_LASTOB_START).
     - n/a
     - No
   * - RECOMMENDED_OD_SPAN
     - The recommended OD time span calculated for the object. (See annex E for definition.)
       Data type = double.
     - d
     - No
   * - ACTUAL_OD_SPAN
     - Based on the observations available and the RECOMMENDED_OD_SPAN, the actual
       time span used for the OD of the object. (See annex E for definition.) Data type =
       double.
     - d
     - No
   * - OBS_AVAILABLE
     - The number of observations available for the OD of the object. (See annex E for
       definition.) Data type = integer.
     - n/a
     - No
   * - OBS_USED
     - The number of observations accepted for the OD of the object. (See annex E for
       definition.) Data type = integer.
     - n/a
     - No
   * - TRACKS_AVAILABLE
     - The number of sensor tracks available for the OD of the object. (See annex E for
       definition.) Data type = integer.
     - n/a
     - No
   * - TRACKS_USED
     - The number of sensor tracks accepted for the OD of the object. (See annex E for
       definition.) Data type = integer.
     - n/a
     - No
   * - RESIDUALS_ACCEPTED
     - The percentage of residuals accepted in the OD of the object. Data type = double, range
       = 0.0 to 100.0.
     - %
     - No
   * - WEIGHTED_RMS
     - The weighted Root Mean Square (RMS) of the residuals from a batch least squares OD.
       (See annex E for definition.) Data type = double.
     - n/a
     - No
   * - **Additional Parameters**
     -
     -
     -
   * - COMMENT
     - (See 6.3.4 for formatting rules.)
     - n/a
     - No
   * - AREA_PC
     - The actual area of the object. (See annex E for definition.) Data type = double.
     - m**2
     - No
   * - AREA_DRG
     - The effective area of the object exposed to atmospheric drag. (See annex E for
       definition.) Data type = double.
     - m**2
     - No
   * - AREA_SRP
     - The effective area of the object exposed to solar radiation pressure. (See annex E for
       definition.) Data type = double.
     - m**2
     - No
   * - MASS
     - The mass of the object. Data type = double.
     - kg
     - No
   * - CD_AREA_OVER_MASS
     - The object's CD.A/m used to propagate the state vector and covariance to TCA. (See
       annex E for definition.) Data type = double.
     - m**2/kg
     - No
   * - CR_AREA_OVER_MASS
     - The object's C, •A/m used to propagate the state vector and covariance to TCA. (See
       annex E for definition.) Data type = double.
     - m**2/kg
     - No
   * - THRUST_ACCELERATION
     - The object's acceleration due to in-track thrust used to propagate the state vector and
       covariance to TCA. (See annex E for definition.) Data type = double.
     - m/s**2
     - No
   * - SEDR
     - The amount of energy being removed from the object's orbit by atmospheric drag. This
       value is an average calculated during the OD. (See annex E for definition.) Data type
       = double.
     - W/kg
     - No
   * - **State Vector (all values have data type=double)**
     -
     -
     -
   * - COMMENT
     - (See 6.3.4 for formatting rules.)
     - n/a
     - No
   * - X
     - Object Position Vector X component.
     - km
     - Yes
   * - Y
     - Object Position Vector Y component.
     - km
     - Yes
   * - Z
     - Object Position Vector Z component.
     - km
     - Yes
   * - X_DOT
     - Object Velocity Vector X component.
     - km/s
     - Yes
   * - Y_DOT
     - Object Velocity Vector Y component.
     - km/s
     - Yes
   * - Z_DOT
     - Object Velocity Vector Z component.
     - km/s
     - Yes
   * - **Covariance Matrix in the RTN Coordinate Frame (see annex E for definition)
       (Covariance Matrix 9x9 Lower Triangular Form. All parameters of the 6×6 position/velocity submatrix must be
       given. All data type=double.)**
     -
     -
     -
   * - COMMENT
     - (See 6.3.4 for formatting rules.)
     - n/a
     - No
   * - CR_R
     - Object covariance matrix [1,1].
     - m**2
     - Yes
   * - CT_R
     - Object covariance matrix [2,1].
     - m**2
     - Yes
   * - CT_T
     - Object covariance matrix [2,2].
     - m**2
     - Yes
   * - CN_R
     - Object covariance matrix [3,1].
     - m**2
     - Yes
   * - CN_T
     - Object covariance matrix [3,2].
     - m**2
     - Yes
   * - CN_N
     - Object covariance matrix [3,3].
     - m**2
     - Yes
   * - CRDOT_R
     - Object covariance matrix [4,1].
     - m**2/s
     - Yes
   * - CRDOT_T
     - Object covariance matrix [4,2].
     - m**2/s
     - Yes
   * - CRDOT_N
     - Object covariance matrix [4,3].
     - m**2/s
     - Yes
   * - CRDOT_RDOT
     - Object covariance matrix [4,4].
     - m**2/s**2
     - Yes
   * - CTDOT_R
     - Object covariance matrix [5,1].
     - m**2/s
     - Yes
   * - CTDOT_T
     - Object covariance matrix [5,2].
     - m**2/s
     - Yes
   * - CTDOT_N
     - Object covariance matrix [5,3].
     - m**2/s
     - Yes
   * - CTDOT_RDOT
     - Object covariance matrix [5,4].
     - m**2/s**2
     - Yes
   * - CTDOT_TDOT
     - Object covariance matrix [5,5].
     - m**2/s**2
     - Yes
   * - CNDOT_R
     - Object covariance matrix [6,1].
     - m**2/s
     - Yes
   * - CNDOT_T
     - Object covariance matrix [6,2].
     - m**2/s
     - Yes
   * - CNDOT_N
     - Object covariance matrix [6,3].
     - m**2/s
     - Yes
   * - CNDOT_RDOT
     - Object covariance matrix [6,4].
     - m**2/s**2
     - Yes
   * - CNDOT_TDOT
     - Object covariance matrix [6,5].
     - m**2/s**2
     - Yes
   * - CNDOT_NDOT
     - Object covariance matrix [6,6].
     - m**2/s**2
     - Yes
   * - CDRG_R
     - Object covariance matrix [7,1].
     - m**3/kg
     - No
   * - CDRG_T
     - Object covariance matrix [7,2].
     - m**3/kg
     - No
   * - CDRG_N
     - Object covariance matrix [7,3].
     - m**3/kg
     - No
   * - CDRG_RDOT
     - Object covariance matrix [7,4].
     - m**3/(kg*s)
     - No
   * - CDRG_TDOT
     - Object covariance matrix [7,5].
     - m**3/(kg*s)
     - No
   * - CDRG_NDOT
     - Object covariance matrix [7,6].
     - m**3/(kg*s)
     - No
   * - CDRG_DRG
     - Object covariance matrix [7,7].
     - m**4/kg**2
     - No
   * - CSRP_R
     - Object covariance matrix [8,1].
     - m**3/kg
     - No
   * - CSRP_T
     - Object covariance matrix [8,2].
     - m**3/kg
     - No
   * - CSRP_N
     - Object covariance matrix [8,3].
     - m**3/kg
     - No
   * - CSRP_RDOT
     - Object covariance matrix [8,4].
     - m**3/(kg*s)
     - No
   * - CSRP_TDOT
     - Object covariance matrix [8,5].
     - m**3/(kg*s)
     - No
   * - CSRP_NDOT
     - Object covariance matrix [8,6].
     - m**3/(kg*s)
     - No
   * - CSRP_DRG
     - Object covariance matrix [8,7].
     - m**4/kg**2
     - No
   * - CSRP_SRP
     - Object covariance matrix [8,8].
     - m**4/kg**2
     - No
   * - CTHR_R
     - Object covariance matrix [9,1].
     - m**2/s**2
     - No
   * - CTHR_T
     - Object covariance matrix [9,2].
     - m**2/s**2
     - No
   * - CTHR_N
     - Object covariance matrix [9,3].
     - m**2/s**2
     - No
   * - CTHR_RDOT
     - Object covariance matrix [9,4].
     - m**2/s**3
     - No
   * - CTHR_TDOT
     - Object covariance matrix [9,5].
     - m**2/s**3
     - No
   * - CTHR_NDOT
     - Object covariance matrix [9,6].
     - m**2/s**3
     - No
   * - CTHR_DRG
     - Object covariance matrix [9,7].
     - m**3/(kg*s**2)
     - No
   * - CTHR_SRP
     - Object covariance matrix [9,8].
     - m**3/(kg*s**2)
     - No
   * - CTHR_THR
     - Object covariance matrix [9,9].
     - m**2/s**4
     - No


.. _kvn_metadata_cdm:

3.4 CDM METADATA
----------------

The CDM metadata shall consist of the KVN elements defined in table 3-3, which specifies
for each KVN metadata item:

a) the keyword to be used;
b) a short description of the item;
c) normative values or examples of allowed values;
d) whether the ‘Normative Values/Examples' column contains normative values (N) or
   examples of allowed values (E) for the item; and
e) whether the item is obligatory or optional.

.. NOTE::
   Table 3-3 and table 3-4 will be used to define both Object1 and Object2
   depending on the value of the keyword OBJECT which is specified in table 3-3.

.. _table_3-3_cdm:

.. list-table:: Table 3-3: CDM KVN Metadata
   :widths: 20 45 20 5 10
   :header-rows: 1

   * - Keyword
     - Description
     - Normative Values/
       Examples
     - N/E
     - Obligatory
   * - COMMENT
     - (See 6.3.4 for formatting rules.)
     - COMMENT This is a
       comment
     - E
     - No
   * - OBJECT
     - The object to which the
       metadata and data apply
       (Object1 or Object2).
     - OBJECT1
       OBJECT2
     - N
     - Yes
   * - OBJECT_DESIGNATOR
     - The satellite catalog
       designator for the object. (See
       5.2.9 for formatting rules.)
     - 12345
     - E
     - Yes
   * - CATALOG_NAME
     - The satellite catalog used for
       the object. Value should be
       taken from the SANA
       'Conjunction Data Message
       CATALOG_NAME' registry
       (https://sanaregistry.org/r/cdm
       _catalog). (See 5.2.9 for
       formatting rules.)
     - SATCAT
     - E
     - Yes
   * - OBJECT_NAME
     - Spacecraft name for the
       object.
     - SPOT, ENVISAT,
       IRIDIUM, INTELSAT
     - E
     - Yes
   * - INTERNATIONAL_DESIGNATOR
     - The full international
       designator for the object.
       Values shall have the format
       YYYY-NNNP{PP}, where:
       YYYY = year of launch;
       NNN = three-digit serial
       number of launch (with
       leading zeros);
       P{PP} = At least one capital
       letter for the identification of
       the part brought into space by
       the launch. In cases where the
       object has no international
       designator, the value
       UNKNOWN should be used.
       (See 5.2.9 for further
       formatting rules.)
     - 2002-021A
       2002-009A
       1997-020AA
       1998-037ABC
       2001-049PE
       UNKNOWN
     - E
     - Yes
   * - OBJECT_TYPE
     - The object type.
     - PAYLOAD
       ROCKET BODY
       DEBRIS
       UNKNOWN
       OTHER
     - N
     - No
   * - OPERATOR_CONTACT_POSITION
     - Contact position of the
       owner/operator of the object.
     - ORBITAL SAFETY
       ANALYST (OSA),
       NETWORK
       CONTROLLER
     - E
     - No
   * - OPERATOR_ORGANIZATION
     - Contact organization of the
       object.
     - EUMETSAT, ESA,
       INTELSAT, IRIDIUM
     - E
     - No
   * - OPERATOR_PHONE
     - Phone number of the contact
       position or organization for the
       object.
     - +49615130312
     - E
     - No
   * - OPERATOR_EMAIL
     - Email address of the contact
       position or organization of the
       object.
     - JOHN.DOE@
       SOMEWHERE.NET
     - E
     - No
   * - EPHEMERIS_NAME
     - Unique name of the external
       ephemeris file used for the
       object or NONE. This is used
       to indicate whether an external
       (i.e., Owner/Operator [O/O]
       provided) ephemeris file was
       used to calculate the CA. If
       'NONE' is specified, then the
       output of the most current
       Orbit Determination (OD) of
       the CDM originator was used
       in the CA.
     - EPHEMERIS
       SATELLITE A,
       NONE
     - E
     - Yes
   * - COVARIANCE_METHOD
     - Method used to calculate the
       covariance during the OD that
       produced the state vector, or
       whether an arbitrary, non-
       calculated default value was
       used. Caution should be used
       when using the default value
       for calculating collision
       probability.
     - CALCULATED
       DEFAULT
     - N
     - Yes
   * - MANEUVERABLE
     - The maneuver capacity of the
       object. (See 1.4.3.1 for
       definition of 'N/A'.)
     - YES
       NO
       N/A
     - N
     - Yes
   * - ORBIT_CENTER
     - The central body about which
       Object1 and Object2 orbit. If
       not specified, the center is
       assumed to be Earth.
     - EARTH
       SUN
       MOON
       MARS
     - E
     - No
   * - REF_FRAME
     - Name of the reference frame
       in which the state vector data
       are given. Value must be
       selected from the list of values
       to the right (see reference [F1])
       and be the same for both
       Object1 and Object2.
     - GCRF (see reference
       [F11])
       EME2000
       ITRF
     - N
     - Yes
   * - GRAVITY_MODEL
     - The gravity model used for the
       OD of the object. (See annex
       E under GRAVITY_MODEL for
       definition).
     - EGM-96: 36D 360
       WGS-84_GEOID: 24D
       240
       JGM-2: 41D 410
     - E
     - No
   * - ATMOSPHERIC_MODEL
     - The atmospheric density
       model used for the OD of the
       object. If 'NONE' is specified,
       then no atmospheric model
       was used.
     - JACCHIA 70
       MSIS
       JACCHIA 70 DCA
       NONE
     - E
     - No
   * - N_BODY_PERTURBATIONS
     - The N-body gravitational
       perturbations used for the OD
       of the object. If 'NONE' is
       specified, then no third-body
       gravitational perturbations
       were used.
     - MOON, SUN
       JUPITER
       NONE
     - E
     - No
   * - SOLAR_RAD_PRESSURE
     - Indication of whether solar
       radiation pressure
       perturbations were used for the
       OD of the object.
     - YES
       NO
     - N
     - No
   * - EARTH_TIDES
     - Indication of whether solid
       Earth and ocean tides were
       used for the OD of the object.
     - YES
       NO
     - N
     - No
   * - INTRACK_THRUST
     - Indication of whether in-track
       thrust modeling was used for
       the OD of the object.
     - YES
       NO
     - N
     - No



.. _kvn_data_cdm:

3.5 CDM DATA
------------

3.5.1 The CDM Data section shall be formed as logical blocks:

-  OD Parameters;
-  Additional Parameters;
-  State Vector; and
-  Covariance Matrix.

3.5.2 The logical blocks of the CDM Data section shall consist of KVN elements as
defined in table 3-4, which specifies for each data item:

a) the keyword to be used;
b) a short description of the item;
c) the units to be used if applicable; and
d) whether the item is obligatory or optional.

.. _table_3-4_cdm:

.. list-table:: Table 3-4: CDM KVN Data
   :widths: 25 65 5 5
   :header-rows: 1

   * - Keyword
     - Description
     - Units
     - Obligatory
   * - COMMENT
     - (See 6.3.4 for formatting rules.)
     - n/a
     - No
   * - **OD Parameters**
     -
     -
     -
   * - COMMENT
     - (See 6.3.4 for formatting rules.)
     - n/a
     - No
   * - TIME_LASTOB_START
     - The start of a time interval (UTC) that contains the time of the last accepted
       observation. (See 6.3.2.6 for formatting rules.) For an exact time, the time interval is
       of zero duration (i.e., same value as that of TIME_LASTOB_END).
     - n/a
     - No
   * - TIME_LASTOB_END
     - The end of a time interval (UTC) that contains the time of the last accepted
       observation. (See 6.3.2.6 for formatting rules.) For an exact time, the time interval is
       of zero duration (i.e., same value as that of TIME_LASTOB_START).
     - n/a
     - No
   * - RECOMMENDED_OD_SPAN
     - The recommended OD time span calculated for the object. (See annex E for definition.)
       Data type = double.
     - d
     - No
   * - ACTUAL_OD_SPAN
     - Based on the observations available and the RECOMMENDED_OD_SPAN, the actual
       time span used for the OD of the object. (See annex E for definition.) Data type =
       double.
     - d
     - No
   * - OBS_AVAILABLE
     - The number of observations available for the OD of the object. (See annex E for
       definition.) Data type = integer.
     - n/a
     - No
   * - OBS_USED
     - The number of observations accepted for the OD of the object. (See annex E for
       definition.) Data type = integer.
     - n/a
     - No
   * - TRACKS_AVAILABLE
     - The number of sensor tracks available for the OD of the object. (See annex E for
       definition.) Data type = integer.
     - n/a
     - No
   * - TRACKS_USED
     - The number of sensor tracks accepted for the OD of the object. (See annex E for
       definition.) Data type = integer.
     - n/a
     - No
   * - RESIDUALS_ACCEPTED
     - The percentage of residuals accepted in the OD of the object. Data type = double, range
       = 0.0 to 100.0.
     - %
     - No
   * - WEIGHTED_RMS
     - The weighted Root Mean Square (RMS) of the residuals from a batch least squares OD.
       (See annex E for definition.) Data type = double.
     - n/a
     - No
   * - **Additional Parameters**
     -
     -
     -
   * - COMMENT
     - (See 6.3.4 for formatting rules.)
     - n/a
     - No
   * - AREA_PC
     - The actual area of the object. (See annex E for definition.) Data type = double.
     - m**2
     - No
   * - AREA_DRG
     - The effective area of the object exposed to atmospheric drag. (See annex E for
       definition.) Data type = double.
     - m**2
     - No
   * - AREA_SRP
     - The effective area of the object exposed to solar radiation pressure. (See annex E for
       definition.) Data type = double.
     - m**2
     - No
   * - MASS
     - The mass of the object. Data type = double.
     - kg
     - No
   * - CD_AREA_OVER_MASS
     - The object's CD.A/m used to propagate the state vector and covariance to TCA. (See
       annex E for definition.) Data type = double.
     - m**2/kg
     - No
   * - CR_AREA_OVER_MASS
     - The object's C, •A/m used to propagate the state vector and covariance to TCA. (See
       annex E for definition.) Data type = double.
     - m**2/kg
     - No
   * - THRUST_ACCELERATION
     - The object's acceleration due to in-track thrust used to propagate the state vector and
       covariance to TCA. (See annex E for definition.) Data type = double.
     - m/s**2
     - No
   * - SEDR
     - The amount of energy being removed from the object's orbit by atmospheric drag. This
       value is an average calculated during the OD. (See annex E for definition.) Data type
       = double.
     - W/kg
     - No
   * - **State Vector (all values have data type=double)**
     -
     -
     -
   * - COMMENT
     - (See 6.3.4 for formatting rules.)
     - n/a
     - No
   * - X
     - Object Position Vector X component.
     - km
     - Yes
   * - Y
     - Object Position Vector Y component.
     - km
     - Yes
   * - Z
     - Object Position Vector Z component.
     - km
     - Yes
   * - X_DOT
     - Object Velocity Vector X component.
     - km/s
     - Yes
   * - Y_DOT
     - Object Velocity Vector Y component.
     - km/s
     - Yes
   * - Z_DOT
     - Object Velocity Vector Z component.
     - km/s
     - Yes
   * - **Covariance Matrix in the RTN Coordinate Frame (see annex E for definition)
       (Covariance Matrix 9x9 Lower Triangular Form. All parameters of the 6×6 position/velocity submatrix must be
       given. All data type=double.)**
     -
     -
     -
   * - COMMENT
     - (See 6.3.4 for formatting rules.)
     - n/a
     - No
   * - CR_R
     - Object covariance matrix [1,1].
     - m**2
     - Yes
   * - CT_R
     - Object covariance matrix [2,1].
     - m**2
     - Yes
   * - CT_T
     - Object covariance matrix [2,2].
     - m**2
     - Yes
   * - CN_R
     - Object covariance matrix [3,1].
     - m**2
     - Yes
   * - CN_T
     - Object covariance matrix [3,2].
     - m**2
     - Yes
   * - CN_N
     - Object covariance matrix [3,3].
     - m**2
     - Yes
   * - CRDOT_R
     - Object covariance matrix [4,1].
     - m**2/s
     - Yes
   * - CRDOT_T
     - Object covariance matrix [4,2].
     - m**2/s
     - Yes
   * - CRDOT_N
     - Object covariance matrix [4,3].
     - m**2/s
     - Yes
   * - CRDOT_RDOT
     - Object covariance matrix [4,4].
     - m**2/s**2
     - Yes
   * - CTDOT_R
     - Object covariance matrix [5,1].
     - m**2/s
     - Yes
   * - CTDOT_T
     - Object covariance matrix [5,2].
     - m**2/s
     - Yes
   * - CTDOT_N
     - Object covariance matrix [5,3].
     - m**2/s
     - Yes
   * - CTDOT_RDOT
     - Object covariance matrix [5,4].
     - m**2/s**2
     - Yes
   * - CTDOT_TDOT
     - Object covariance matrix [5,5].
     - m**2/s**2
     - Yes
   * - CNDOT_R
     - Object covariance matrix [6,1].
     - m**2/s
     - Yes
   * - CNDOT_T
     - Object covariance matrix [6,2].
     - m**2/s
     - Yes
   * - CNDOT_N
     - Object covariance matrix [6,3].
     - m**2/s
     - Yes
   * - CNDOT_RDOT
     - Object covariance matrix [6,4].
     - m**2/s**2
     - Yes
   * - CNDOT_TDOT
     - Object covariance matrix [6,5].
     - m**2/s**2
     - Yes
   * - CNDOT_NDOT
     - Object covariance matrix [6,6].
     - m**2/s**2
     - Yes
   * - CDRG_R
     - Object covariance matrix [7,1].
     - m**3/kg
     - No
   * - CDRG_T
     - Object covariance matrix [7,2].
     - m**3/kg
     - No
   * - CDRG_N
     - Object covariance matrix [7,3].
     - m**3/kg
     - No
   * - CDRG_RDOT
     - Object covariance matrix [7,4].
     - m**3/(kg*s)
     - No
   * - CDRG_TDOT
     - Object covariance matrix [7,5].
     - m**3/(kg*s)
     - No
   * - CDRG_NDOT
     - Object covariance matrix [7,6].
     - m**3/(kg*s)
     - No
   * - CDRG_DRG
     - Object covariance matrix [7,7].
     - m**4/kg**2
     - No
   * - CSRP_R
     - Object covariance matrix [8,1].
     - m**3/kg
     - No
   * - CSRP_T
     - Object covariance matrix [8,2].
     - m**3/kg
     - No
   * - CSRP_N
     - Object covariance matrix [8,3].
     - m**3/kg
     - No
   * - CSRP_RDOT
     - Object covariance matrix [8,4].
     - m**3/(kg*s)
     - No
   * - CSRP_TDOT
     - Object covariance matrix [8,5].
     - m**3/(kg*s)
     - No
   * - CSRP_NDOT
     - Object covariance matrix [8,6].
     - m**3/(kg*s)
     - No
   * - CSRP_DRG
     - Object covariance matrix [8,7].
     - m**4/kg**2
     - No
   * - CSRP_SRP
     - Object covariance matrix [8,8].
     - m**4/kg**2
     - No
   * - CTHR_R
     - Object covariance matrix [9,1].
     - m**2/s**2
     - No
   * - CTHR_T
     - Object covariance matrix [9,2].
     - m**2/s**2
     - No
   * - CTHR_N
     - Object covariance matrix [9,3].
     - m**2/s**2
     - No
   * - CTHR_RDOT
     - Object covariance matrix [9,4].
     - m**2/s**3
     - No
   * - CTHR_TDOT
     - Object covariance matrix [9,5].
     - m**2/s**3
     - No
   * - CTHR_NDOT
     - Object covariance matrix [9,6].
     - m**2/s**3
     - No
   * - CTHR_DRG
     - Object covariance matrix [9,7].
     - m**3/(kg*s**2)
     - No
   * - CTHR_SRP
     - Object covariance matrix [9,8].
     - m**3/(kg*s**2)
     - No
   * - CTHR_THR
     - Object covariance matrix [9,9].
     - m**2/s**4
     - No


.. _xml_content_structure_cdm:

4 CDM CONTENT/STRUCTURE IN XML
==============================

.. _xml_schema_discussion_cdm:

4.1 DISCUSSION—THE CDM/XML SCHEMA
---------------------------------

The CDM/XML schema is available on the SANA Web site. SANA is the registrar for the
protocol registries created under CCSDS.

The CDM XML schema explicitly defines the permitted data elements and values acceptable
for the XML version of the CDM message.

The location of the CDM/XML schema is:

-  https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-2.0.0-cdm-1.0.xsd for
   messages with elements not qualified with respect to a namespace;
-  https://sanaregistry.org/r/ndmxml_qualified/ndmxml-2.0.0-cdm-1.0.xsd for messages
   with elements qualified with respect to a namespace.

.. NOTE::
   Reference [6] subsection 4.3 has more information regarding messages with
   elements qualified with respect to a namespace.

Where possible this schema uses simple types and complex types used by the constituent
schemas that make up NDMs (see reference [6]).

An Extensible Stylesheet Language Transformations (XSLT) converter is available on the SANA
Web site to transform an XML CDM to a KVN CDM if desired by the CDM recipient. The location
of the CDM/XML XSLT converter is:

-  https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-2.0.0-cdm-1.1.xsl for
   messages with elements not qualified with respect to a namespace;
-  https://sanaregistry.org/r/ndmxml_qualified/ndmxml-2.0.0-cdm-1.1.xsl for messages
   with elements qualified with respect to a namespace.

.. NOTE::
   Reference [6] subsection 4.3 has more information regarding messages with
   elements qualified with respect to a namespace.

.. _xml_basic_structure_cdm:

4.2 CDM/XML BASIC STRUCTURE
---------------------------

4.2.1 Each CDM shall consist of a <header> and a <body>.

4.2.2 The CDM body shall consist of one relative metadata/data and two segment constructs.

4.2.3 Each <segment> shall consist of a <metadata>/<data> pair, as shown in figure 4-1.

.. _figure_4-1_cdm:

.. figure:: /images/placeholder.png
   
   Figure 4-1: CDM XML Basic Structure

.. code-block::
   
   <header>
   </header>
   <body>
      <relativeMetadataData>
      </relativeMetadataData>
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

4.2.4 XML tags shall be uppercase and correspond with the KVN keywords in 3.2 through
3.5 (uppercase with ‘_’ [the underscore character] as separators). The XML logical tags
related to message structure shall be in lowerCamelCase.

.. _xml_constructing_instance_cdm:

4.3 CONSTRUCTING A CDM/XML INSTANCE
-----------------------------------

.. _xml_constructing_overview_cdm:

4.3.1 OVERVIEW
^^^^^^^^^^^^^^

This subsection provides more detailed instructions for the user on how to create an XML
message based on the ASCII-text KVN-formatted message described in 3.1 through 3.6 (see
reference [6]).

.. _xml_version_cdm:

4.3.2 XML VERSION
^^^^^^^^^^^^^^^^^

The first line in the instantiation shall specify the XML version:

.. code-block:: xml
   
   <?xml version="1.0" encoding="UTF-8"?>

This line must appear on the first line of each instantiation, exactly as shown.

.. _xml_root_element_cdm:

4.3.3 BEGINNING THE INSTANTIATION: ROOT DATA ELEMENT
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

4.3.3.1 A CDM instantiation shall be delimited with the <cdm></cdm> root element tags
using the standard attributes documented in reference [3].

4.3.3.2 The XML Schema Instance namespace attribute must appear in the root element tag
of all CDM/XML instantiations, exactly as shown:

.. code-block::
   
   xmlns:xsi = "http://www.w3.org/2001/XMLSchema-instance"

4.3.3.3 For messages with elements qualified with respect to a namespace, the NDM/XML
namespace must next be coded, exactly as shown:

.. code-block::
   
   xmlns:ndm="urn:ccsds:schema:ndmxml"

The value that follows the ‘xmlns:' in the NDM/XML name space (‘ndm' in this case) is a
prefix that must be used on every XML tag.

.. NOTE::
   This xmlns:ndm setting is only necessary for messages with elements qualified
   with respect to a namespace, but it does not hurt anything for it to appear on any
   NDM/XML instantiation.

4.3.3.4 If it is desired to validate an instantiation against the CCSDS Web-based schema,
the xsi:noNamespaceSchemaLocation attribute must be coded as a single string of non-blank
characters, with no line breaks, exactly as shown:

.. code-block::

   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/n
   dmxml-2.0.0-master-2.0.xsd" for messages with elements not qualified with respect
   to a namespace;
   
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_qualified/ndm
   xml-2.0.0-master-2.0.xsd" for messages with elements qualified with respect to a
   namespace.

.. NOTE::
   The length of the value associated with the xsi:noNamespaceSchemaLocation
   attribute can cause the string to wrap to a new line; however, the string itself
   contains no breaks.

4.3.3.5 For use in a local operations environment, the schema set may be downloaded from
the SANA Web site to a local server that meets local requirements for operations robustness.

4.3.3.6 If a local version is used, the value associated with the
xsi:noNamespaceSchemaLocation attribute must be changed to a URL that is accessible to
the local server.

4.3.3.7 The final attributes of the <cdm> tag shall be 'id' and 'version'.

4.3.3.8 The 'id' attribute shall be ‘id="CCSDS_CDM_VERS"'.

4.3.3.9 The 'version' attribute shall be 'version="1.0"'.

.. NOTE::

   1. The following example root element tag for a CDM instantiation combines all the
      directions in the preceding several subsections for messages with elements not
      qualified with respect to a namespace:
   
      .. code-block:: xml
      
         <?xml version="1.0" encoding="UTF-8"?>
         <cdm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:noNamespaceSchemaLocation=
         "https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-2.0.0-master-2.0.xs
         id="CCSDS_CDM_VERS" version="1.0">
   
   2. The following example root element tag for a CDM instantiation combines all the
      directions in the preceding several subsections for messages with elements qualified
      with respect to a namespace:
      
      .. code-block:: xml
      
         <?xml version="1.0" encoding="UTF-8"?>
         <cdm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xmlns:ndm="urn:ccsds:schema:ndmxml"
         xsi:noNamespaceSchemaLocation=
         "https://sanaregistry.org/r/ndmxml_qualified/ndmxml-2.0.0-master-2.0.xsd"
         id="CCSDS_CDM_VERS" version="1.0">

.. _xml_header_section_cdm:

4.3.4 THE CDM/XML HEADER SECTION
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

4.3.4.1 The CDM header shall have a standard header format, with tags <header> and </header>.

4.3.4.2 Immediately following the <header> tag, the message may have any number of
<COMMENT></COMMENT> tag pairs.

4.3.4.3 The standard CDM header shall contain the following element tags:

a) <CREATION_DATE>;
b) <ORIGINATOR>;
c) optional <MESSAGE_FOR>;
d) <MESSAGE_ID>.

.. NOTE::
   The rules for these keywords are specified in 3.2. The header would look like this:
   
   .. code-block:: xml
   
      <header>
         <COMMENT>Some comment string.</COMMENT>
         <CREATION_DATE>2010-03-12T22:31:12.000</CREATION_DATE>
         <ORIGINATOR>JSPOC</ORIGINATOR>
         <MESSAGE_FOR>SATELLITE A</MESSAGE_FOR>
         <MESSAGE_ID>201113719185</MESSAGE_ID>
      </header>

.. _xml_body_section_cdm:

4.3.5 THE CDM/XML BODY SECTION
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

4.3.5.1 After coding the <header>, the instantiation must include a <body></body> tag
pair.

4.3.5.2 Inside the <body></body> tag pair, there must appear one
<relativeMetadataData></relativeMetadataData> tag pair.

4.3.5.3 Following the <relativeMetadataData></relativeMetadataData> tag pair, there must
appear two <segment></segment> tag pairs, one for Object1 and one for Object2.

4.3.5.4 Each segment must be made up of one <metadata></metadata> tag pair and one
<data></data> tag pair.

.. _xml_relative_metadata_data_section_cdm:

4.3.6 THE CDM/XML RELATIVE METADATA/DATA SECTION
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

4.3.6.1 The relative metadata/data section shall be set off by the
<relativeMetadataData></relativeMetadataData> tag combination.

4.3.6.2 Immediately following the <relativeMetadataData> tag, the message may have any
number of <COMMENT></COMMENT> tag pairs.

4.3.6.3 Between the <relativeMetadataData> and </relativeMetadataData> tags, the
keywords shall be those specified in table 3-2.

.. _xml_metadata_section_cdm:

4.3.7 THE CDM/XML METADATA SECTION
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

4.3.7.1 All CDMs must have two metadata sections, one for Object1 and one for Object2.

4.3.7.2 The metadata section for Object1 shall follow the relative metadata/data section and
shall be set off by the <metadata></metadata> tag combination. The metadata section for
Object2 shall follow the Object1 data section and shall be set off by the
<metadata></metadata> tag combination.

4.3.7.3 Immediately following the <metadata> tag, the message may have any number of
<COMMENT></COMMENT> tag pairs.

4.3.7.4 Between the <metadata> and </metadata> tags for both Object1 and Object2, the
keywords shall be those specified in table 3-3. The value of the keyword OBJECT shall be
used to define whether the metadata defines Object1 or Object2.

.. _xml_data_section_cdm:

4.3.8 THE CDM DATA SECTION
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

4.3.8.1 All CDMs must have two data sections, one for Object1 and one for Object2.

4.3.8.2 Each data section shall follow the corresponding metadata section and shall be set
off by the <data></data> tag combination.

4.3.8.3 Immediately following the <data> tag, the message may have any number of
<COMMENT></COMMENT> tag pairs.

4.3.8.4 Between the <data> and </data> tags, the keywords shall be those specified in
table 3-4. The value of the keyword OBJECT, referenced in table 3-3, shall be used to define
whether the data defines Object1 or Object2.

.. _xml_special_tags_cdm:

4.3.9 SPECIAL CDM/XML TAGS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

4.3.9.1 The information content in the CDM shall be separated into constructs described in 3.5
as 'logical blocks'. Special tags in the CDM shall be used to encapsulate the information in
the logical blocks of the CDM. Immediately following the special tags for logical blocks, the
message may have any number of <COMMENT></COMMENT> tag pairs.

4.3.9.2 The special tags indicating logical block divisions shall be those defined in table 4-1.

.. _table_4-1_cdm:

.. list-table:: Table 4-1: Relation of KVN Logical Blocks to Special CDM/XML Tags
   :widths: 50 50
   :header-rows: 1

   * - CDM Logical Block
     - Associated CDM/XML Tag
   * - OD Parameters
     - <odParameters>
   * - Additional Parameters
     - <additionalParameters>
   * - State Vector
     - <stateVector>
   * - Covariance Matrix
     - <covarianceMatrix>

4.3.9.3 Another special tag that shall be used is defined in table 4-2.

.. _table_4-2_cdm:

.. list-table:: Table 4-2: Another Special CDM/XML Tag
   :widths: 30 70
   :header-rows: 1

   * - Special Tag
     - Definition
   * - <relativeStateVector>
     - Includes the relative state vector keywords:
       RELATIVE_POSITION_R, RELATIVE_POSITION_T,
       RELATIVE_POSITION_N, RELATIVE_VELOCITY_R,
       RELATIVE_VELOCITY_T, and
       RELATIVE_VELOCITY_N.

.. _xml_units_cdm:

4.3.10 UNITS IN THE CDM/XML
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The units in the CDM/XML shall be the same units used in the KVN-formatted CDM
described in 3.3 and 3.5. XML attributes shall be used to explicitly define the units or other
important information associated with the given data element (see 6.4.3 for examples).

.. _xml_example_discussion_cdm:

4.4 DISCUSSION—CDM/XML EXAMPLE
--------------------------------

The following is a sample of a CDM in XML format:

.. code-block:: xml
   
   <?xml version="1.0" encoding="UTF-8"?>
   <cdm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xmlns:ndm="urn:ccsds:schema:ndmxml"
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-2.0.0-master-2.0.xsd"
   id="CCSDS_CDM_VERS" version="1.0">
      <header>
         <COMMENT>Sample CDM - XML version</COMMENT>
         <CREATION_DATE>2010-03-12T22:31:12.000</CREATION_DATE>
         <ORIGINATOR>JSPOC</ORIGINATOR>
         <MESSAGE_FOR>SATELLITE A</MESSAGE_FOR>
         <MESSAGE_ID>201113719185</MESSAGE_ID>
      </header>
      <body>
         <relativeMetadataData>
            <COMMENT>Relative Metadata/Data</COMMENT>
            <TCA>2010-03-13T22:37:52.618</TCA>
            <MISS_DISTANCE units="m">715</MISS_DISTANCE>
            <RELATIVE_SPEED units="m/s">14762</RELATIVE_SPEED>
            <relativeStateVector>
               <RELATIVE_POSITION_R units="m">27.4</RELATIVE_POSITION_R>
               <RELATIVE_POSITION_T units="m">-70.2</RELATIVE_POSITION_T>
               <RELATIVE_POSITION_N units="m">711.8</RELATIVE_POSITION_N>
               <RELATIVE_VELOCITY_R units="m/s">-7.2</RELATIVE_VELOCITY_R>
               <RELATIVE_VELOCITY_T units="m/s">-14692.0</RELATIVE_VELOCITY_T>
               <RELATIVE_VELOCITY_N units="m/s">-1437.2</RELATIVE_VELOCITY_N>
            </relativeStateVector>
            <START_SCREEN_PERIOD>2010-03-12T18:29:32.212</START_SCREEN_PERIOD>
            <STOP_SCREEN_PERIOD>2010-03-15T18:29:32.212</STOP_SCREEN_PERIOD>
            <SCREEN_VOLUME_FRAME>RTN</SCREEN_VOLUME_FRAME>
            <SCREEN_VOLUME_SHAPE>ELLIPSOID</SCREEN_VOLUME_SHAPE>
            <SCREEN_VOLUME_X units="m">200</SCREEN_VOLUME_X>
            <SCREEN_VOLUME_Y units="m">1000</SCREEN_VOLUME_Y>
            <SCREEN_VOLUME_Z units="m">1000</SCREEN_VOLUME_Z>
            <SCREEN_ENTRY_TIME>2010-03-13T20:25:43.222</SCREEN_ENTRY_TIME>
            <SCREEN_EXIT_TIME>2010-03-13T23:44:29.324</SCREEN_EXIT_TIME>
            <COLLISION_PROBABILITY>4.835E-05</COLLISION_PROBABILITY>
            <COLLISION_PROBABILITY_METHOD>FOSTER-1992</COLLISION_PROBABILITY_METHOD>
         </relativeMetadataData>
         <segment>
            <metadata>
               <COMMENT>Object1 Metadata</COMMENT>
               <OBJECT>OBJECT1</OBJECT>
               <OBJECT_DESIGNATOR>12345</OBJECT_DESIGNATOR>
               <CATALOG_NAME>SATCAT</CATALOG_NAME>
               <OBJECT_NAME>SATELLITE A</OBJECT_NAME>
               <INTERNATIONAL_DESIGNATOR>1997-030E</INTERNATIONAL_DESIGNATOR>
               <OBJECT_TYPE>PAYLOAD</OBJECT_TYPE>
               <OPERATOR_CONTACT_POSITION>OSA</OPERATOR_CONTACT_POSITION>
               <OPERATOR_ORGANIZATION>EUMETSAT</OPERATOR_ORGANIZATION>
               <OPERATOR_PHONE>+49615130312</OPERATOR_PHONE>
               <OPERATOR_EMAIL>JOHN.DOE@SOMEWHERE.NET</OPERATOR_EMAIL>
               <EPHEMERIS_NAME>EPHEMERIS SATELLITE A</EPHEMERIS_NAME>
               <COVARIANCE_METHOD>CALCULATED</COVARIANCE_METHOD>
               <MANEUVERABLE>YES</MANEUVERABLE>
               <REF_FRAME>EME2000</REF_FRAME>
               <GRAVITY_MODEL>EGM-96: 36D 36O</GRAVITY_MODEL>
               <ATMOSPHERIC_MODEL>JACCHIA 70 DCA</ATMOSPHERIC_MODEL>
               <N_BODY_PERTURBATIONS>MOON,SUN</N_BODY_PERTURBATIONS>
               <SOLAR_RAD_PRESSURE>NO</SOLAR_RAD_PRESSURE>
               <EARTH_TIDES>NO</EARTH_TIDES>
               <INTRACK_THRUST>NO</INTRACK_THRUST>
            </metadata>
            <data>
               <COMMENT>Object1 Data</COMMENT>
               <odParameters>
                  <COMMENT>Object1 OD Parameters</COMMENT>
                  <TIME_LASTOB_START>2010-03-12T02:14:12.746</TIME_LASTOB_START>
                  <TIME_LASTOB_END>2010-03-12T02:14:12.746</TIME_LASTOB_END>
                  <RECOMMENDED_OD_SPAN units="d">7.88</RECOMMENDED_OD_SPAN>
                  <ACTUAL_OD_SPAN units="d">5.50</ACTUAL_OD_SPAN>
                  <OBS_AVAILABLE>592</OBS_AVAILABLE>
                  <OBS_USED>59</OBS_USED>
                  <TRACKS_AVAILABLE>123</TRACKS_AVAILABLE>
                  <TRACKS_USED>119</TRACKS_USED>
                  <RESIDUALS_ACCEPTED units="%">97.8</RESIDUALS_ACCEPTED>
                  <WEIGHTED_RMS>0.864</WEIGHTED_RMS>
               </odParameters>
               <additionalParameters>
                  <COMMENT>Object 1 Additional Parameters</COMMENT>
                  <AREA_PC units="m**2">5.2</AREA_PC>
                  <MASS units="kg">2516</MASS>
                  <CD_AREA_OVER_MASS units="m**2/kg">0.045663</CD_AREA_OVER_MASS>
                  <CR_AREA_OVER_MASS units="m**2/kg">0.000000</CR_AREA_OVER_MASS>
                  <THRUST_ACCELERATION units="m/s**2">0.0</THRUST_ACCELERATION>
                  <SEDR units="W/kg">4.54570E-05</SEDR>
               </additionalParameters>
               <stateVector>
                  <COMMENT>Object1 State Vector</COMMENT>
                  <X units="km">2570.097065</X>
                  <Y units="km">2244.654904</Y>
                  <Z units="km">6281.497978</Z>
                  <X_DOT units="km/s">4.418769571</X_DOT>
                  <Y_DOT units="km/s">4.833547743</Y_DOT>
                  <Z_DOT units="km/s">-3.526774282</Z_DOT>
               </stateVector>
               <covarianceMatrix>
                  <COMMENT>Object1 Covariance in the RTN Coordinate Frame </COMMENT>
                  <CR_R units="m**2">4.142E+01</CR_R>
                  <CT_R units="m**2">-8.579E+00</CT_R>
                  <CT_T units="m**2">2.533E+03</CT_T>
                  <CN_R units="m**2">-2.313E+01</CN_R>
                  <CN_T units="m**2">1.336E+01</CN_T>
                  <CN_N units="m**2">7.098E+01</CN_N>
                  <CRDOT_R units="m**2/s">2.520E-03</CRDOT_R>
                  <CRDOT_T units="m**2/s">-5.476E+00</CRDOT_T>
                  <CRDOT_N units="m**2/s">8.626E-04</CRDOT_N>
                  <CRDOT_RDOT units="m**2/s**2">5.744E-03</CRDOT_RDOT>
                  <CTDOT_R units="m**2/s">-1.006E-02</CTDOT_R>
                  <CTDOT_T units="m**2/s">4.041E-03</CTDOT_T>
                  <CTDOT_N units="m**2/s">-1.359E-03</CTDOT_N>
                  <CTDOT_RDOT units="m**2/s**2">-1.502E-05</CTDOT_RDOT>
                  <CTDOT_TDOT units="m**2/s**2">1.049E-05</CTDOT_TDOT>
                  <CNDOT_R units="m**2/s">1.053E-03</CNDOT_R>
                  <CNDOT_T units="m**2/s">-3.412E-03</CNDOT_T>
                  <CNDOT_N units="m**2/s">1.213E-02</CNDOT_N>
                  <CNDOT_RDOT units="m**2/s**2">-3.004E-06</CNDOT_RDOT>
                  <CNDOT_TDOT units="m**2/s**2">-1.091E-06</CNDOT_TDOT>
                  <CNDOT_NDOT units="m**2/s**2">5.529E-05</CNDOT_NDOT>
               </covarianceMatrix>
            </data>
         </segment>
         <segment>
            <metadata>
               <COMMENT>Object2 Metadata</COMMENT>
               <OBJECT>OBJECT2</OBJECT>
               <OBJECT_DESIGNATOR>30337</OBJECT_DESIGNATOR>
               <CATALOG_NAME>SATCAT</CATALOG_NAME>
               <OBJECT_NAME>FENGYUN 1C DEB</OBJECT_NAME>
               <INTERNATIONAL_DESIGNATOR>1999-025AA</INTERNATIONAL_DESIGNATOR>
               <OBJECT_TYPE>DEBRIS</OBJECT_TYPE>
               <EPHEMERIS_NAME>NONE</EPHEMERIS_NAME>
               <COVARIANCE_METHOD>CALCULATED</COVARIANCE_METHOD>
               <MANEUVERABLE>NO</MANEUVERABLE>
               <REF_FRAME>EME2000</REF_FRAME>
               <GRAVITY_MODEL>EGM-96: 36D 36O</GRAVITY_MODEL>
               <ATMOSPHERIC_MODEL>JACCHIA 70 DCA</ATMOSPHERIC_MODEL>
               <N_BODY_PERTURBATIONS>MOON,SUN</N_BODY_PERTURBATIONS>
               <SOLAR_RAD_PRESSURE>YES</SOLAR_RAD_PRESSURE>
               <EARTH_TIDES>NO</EARTH_TIDES>
               <INTRACK_THRUST>NO</INTRACK_THRUST>
            </metadata>
            <data>
               <COMMENT>Object2 Data</COMMENT>
               <odParameters>
                  <COMMENT>Object2 OD Parameters</COMMENT>
                  <TIME_LASTOB_START>2010-03-12T01:14:12.746</TIME_LASTOB_START>
                  <TIME_LASTOB_END>2010-03-12T03:14:12.746</TIME_LASTOB_END>
                  <RECOMMENDED_OD_SPAN units="d">2.63</RECOMMENDED_OD_SPAN>
                  <ACTUAL_OD_SPAN units="d">2.63</ACTUAL_OD_SPAN>
                  <OBS_AVAILABLE>59</OBS_AVAILABLE>
                  <OBS_USED>58</OBS_USED>
                  <TRACKS_AVAILABLE>15</TRACKS_AVAILABLE>
                  <TRACKS_USED>15</TRACKS_USED>
                  <RESIDUALS_ACCEPTED units="%">97.8</RESIDUALS_ACCEPTED>
                  <WEIGHTED_RMS>0.864</WEIGHTED_RMS>
               </odParameters>
               <additionalParameters>
                  <COMMENT>Object2 Additional Parameters</COMMENT>
                  <COMMENT>Apogee Altitude=768 km</COMMENT>
                  <COMMENT>Perigee Altitude=414 km</COMMENT>
                  <COMMENT>Inclination=98.8 deg</COMMENT>
                  <AREA_PC units="m**2">0.9</AREA_PC>
                  <CD_AREA_OVER_MASS units="m**2/kg">0.118668</CD_AREA_OVER_MASS>
                  <CR_AREA_OVER_MASS units="m**2/kg">0.075204</CR_AREA_OVER_MASS>
                  <THRUST_ACCELERATION units="m/s**2">0.0</THRUST_ACCELERATION>
                  <SEDR units="W/kg">5.40900E-03</SEDR>
               </additionalParameters>
               <stateVector>
                  <COMMENT>Object2 State Vector</COMMENT>
                  <X units="km">2569.540800</X>
                  <Y units="km">2245.093614</Y>
                  <Z units="km">6281.599946</Z>
                  <X_DOT units="km/s">-2.888612500</X_DOT>
                  <Y_DOT units="km/s">-6.007247516</Y_DOT>
                  <Z_DOT units="km/s">3.328770172</Z_DOT>
               </stateVector>
               <covarianceMatrix>
                  <COMMENT>Object2 Covariance in the RTN Coordinate Frame</COMMENT>
                  <CR_R units="m**2">1.337E+03</CR_R>
                  <CT_R units="m**2">-4.806E+04</CT_R>
                  <CT_T units="m**2">2.492E+06</CT_T>
                  <CN_R units="m**2">-3.298E+01</CN_R>
                  <CN_T units="m**2">-7.5888E+02</CN_T>
                  <CN_N units="m**2">7.105E+01</CN_N>
                  <CRDOT_R units="m**2/s">2.591E-03</CRDOT_R>
                  <CRDOT_T units="m**2/s">-4.152E-02</CRDOT_T>
                  <CRDOT_N units="m**2/s">-1.784E-06</CRDOT_N>
                  <CRDOT_RDOT units="m**2/s**2">6.886E-05</CRDOT_RDOT>
                  <CTDOT_R units="m**2/s">-1.016E-02</CTDOT_R>
                  <CTDOT_T units="m**2/s">-1.506E-04</CTDOT_T>
                  <CTDOT_N units="m**2/s">1.637E-03</CTDOT_N>
                  <CTDOT_RDOT units="m**2/s**2">-2.987E-06</CTDOT_RDOT>
                  <CTDOT_TDOT units="m**2/s**2">1.059E-05</CTDOT_TDOT>
                  <CNDOT_R units="m**2/s">4.400E-03</CNDOT_R>
                  <CNDOT_T units="m**2/s">8.482E-03</CNDOT_T>
                  <CNDOT_N units="m**2/s">8.633E-05</CNDOT_N>
                  <CNDOT_RDOT units="m**2/s**2">-1.903E-06</CNDOT_RDOT>
                  <CNDOT_TDOT units="m**2/s**2">-4.594E-06</CNDOT_TDOT>
                  <CNDOT_NDOT units="m**2/s**2">5.178E-05</CNDOT_NDOT>
               </covarianceMatrix>
            </data>
         </segment>
      </body>
   </cdm>
   
.. _general_data_cdm:

5 CDM DATA IN GENERAL
=====================

.. _general_overview_cdm:

5.1 OVERVIEW
------------

The following rules apply for both KVN- and XML-formatted CDMs.

.. _general_rules_cdm:

5.2 RULES THAT APPLY IN KVN AND XML
---------------------------------

5.2.1 Some keywords represent obligatory items and some are optional. KVN and XML
assignments representing optional items may be omitted.

5.2.2 The objects' state vectors and covariance shall be given 'at the time of closest
approach', i.e., at the time specified in the TCA keyword.

5.2.3 Table 3-4 is broken into four logical blocks, each of which has a descriptive heading.
These descriptive headings shall not be included in a CDM, unless they appear in a properly
formatted COMMENT statement for the KVN implementation and with values between the
<COMMENT> and </COMMENT> tags for the XML implementation.

5.2.4 For :math:`C_D \cdot A/m`, CD_AREA_OVER_MASS, a value of zero shall indicate no atmospheric
drag was taken into account in the orbit determination process.

5.2.5 For :math:`C_R \cdot A/m`, CR_AREA_OVER_MASS, a value of zero shall indicate no solar
radiation pressure was taken into account in the orbit determination process.

5.2.6 For acceleration due to in-track thrust, THRUST_ACCELERATION, a value of zero
shall indicate no in-track thrust acceleration was taken into account in the orbit determination
process.

5.2.7 Values in the covariance matrix shall be presented sequentially from upper left [1,1]
to lower right [9,9], lower triangular form, row by row, left to right. Variance and covariance
values shall be expressed in standard double precision as related in 6.3.2.3.

5.2.8 The covariance matrix shall be provided for the position and velocity terms, given in
the lower triangular form of a 6×6 matrix. If any of the diagonal terms are zero, the entire
row and column of the matrix related to that term should be discounted. Optional terms for
CD_AREA_OVER_MASS (denoted ‘DRG’), CR_AREA_OVER_MASS (denoted ‘SRP’),
and THRUST_ACCELERATION (denoted ‘THR’) may be added to the 6×6 matrix, in the
lower triangular form, to complete a 9×9 matrix. If any element in any of these rows (7, 8, or
9) is provided, then all of the elements for that row and all preceding rows shall be provided
(i.e., a subset of the terms for any of these rows is not allowed). (See annex E for definition.)

5.2.9 In the value fields for the keywords ORIGINATOR, MESSAGE_ID,
OBJECT_DESIGNATOR, CATALOG_NAME and INTERNATIONAL_DESIGNATOR,
values shall be given as alphanumeric text. The underscore ‘_’ and dash ‘-’ may also be used.

.. _syntax_cdm:

6 CDM SYNTAX
============

.. _syntax_overview_cdm:

6.1 OVERVIEW
------------

This section details the syntax requirements for the CDM using both KVN and XML
formats.

.. _common_syntax_cdm:

6.2 COMMON CDM SYNTAX
---------------------

.. _common_lines_cdm:

6.2.1 OVERVIEW
^^^^^^^^^^^^^^

This subsection details the syntax requirements that are common to both KVN and XML
formats.

.. _common_cdm_lines_cdm:

6.2.2 COMMON CDM LINES
^^^^^^^^^^^^^^^^^^^^^^^^

6.2.2.1 Each CDM line must not exceed 254 ASCII characters and spaces (excluding line
termination character[s]).

6.2.2.2 Only printable ASCII characters and blanks shall be used. Control characters (such
as TAB, etc.) shall not be used, with the exception of the line termination characters
specified below.

6.2.2.3 Blank lines may be used at any position within the file. Blank lines shall have no
assignable meaning, and may be ignored.

6.2.2.4 All lines shall be terminated by a single Carriage Return, a single Line Feed, a
Carriage Return/Line Feed pair, or a Line Feed/Carriage Return pair.

.. _cdm_values_in_kvn_cdm:

6.2.3 COMMON CDM VALUES
^^^^^^^^^^^^^^^^^^^^^^^^^

6.2.3.1 A nonempty, valid value must be specified for each obligatory keyword.

6.2.3.2 Non-integer numeric values may be expressed in either fixed-point or floating-point
notation.

6.2.3.3 Text value fields must be constructed using only all uppercase. An exception is
made for comment values (see 6.2.5 for formatting rules).

6.2.3.4 All time tags in the CDM shall be in UTC.

.. _cdm_units_in_kvn_cdm:

6.2.4 COMMON CDM UNITS
^^^^^^^^^^^^^^^^^^^^^^^^

6.2.4.1 If units are applicable, as specified in table 3-2 and/or table 3-4, they must be
displayed and must exactly match the units specified in each table (including case). (See
1.4.1.1 and 1.4.1.2 for units conventions and operations.)

6.2.4.2 The notation ‘[n/a]’ shall not appear in a CDM as a units designator.

.. NOTE::

   Some of the items in the applicable tables are dimensionless. For such items, the
   table shows a unit value of 'n/a', which in this case means that there is no
   applicable units designator for those items (e.g., for
   COLLISION_PROBABILITY, WEIGHTED_RMS).

.. _cdm_comments_in_kvn_cdm:

6.2.5 COMMON CDM COMMENTS
^^^^^^^^^^^^^^^^^^^^^^^^^^^

6.2.5.1 For the CDM, comment lines shall be optional.

6.2.5.2 Placement of comments shall be as specified in the tables in section 3 that describe
the CDM keywords. In places where comments are permitted any number of comments may
appear.

6.2.5.3 Comment text may be in any case desired by the user.

.. _the_cdm_in_kvn_cdm:

6.3 THE CDM IN KVN
------------------

.. _cdm_lines_in_kvn_cdm:

6.3.1 CDM LINES IN KVN
^^^^^^^^^^^^^^^^^^^^^^^^

6.3.1.1 Each CDM file shall consist of a set of CDM lines. Each CDM line shall be one of
the following:

-  Header line;
-  Relative Metadata/Data line;
-  Metadata line;
-  Data line; or
-  Blank line.

6.3.1.2 The first header line must be the first non-blank line in the file.

6.3.1.3 All header, relative metadata/data, metadata, and data lines shall use 'keyword =
value' notation. For this purpose, only those keywords shown in table 3-1, table 3-2,
table 3-3, and table 3-4 shall be used in a CDM.

6.3.1.4 Only a single ‘keyword = value’ assignment shall be made on a line.

6.3.1.5 Keywords must be uppercase and must not contain blanks.

6.3.1.6 Any white space immediately preceding or following the keyword shall not be
significant.

6.3.1.7 Any white space immediately preceding or following the ‘equals’ sign shall not be
significant.

6.3.1.8 Any white space immediately preceding the end of line shall not be significant.

6.3.1.9 The order of occurrence of obligatory and optional KVN assignments shall be fixed
as shown in the tables in section 3 that describe the CDM keywords.

.. _cdm_values_in_kvn_cdm:

6.3.2 CDM VALUES IN KVN
^^^^^^^^^^^^^^^^^^^^^^^^^

6.3.2.1 Integer values shall consist of a sequence of decimal digits with an optional leading
sign ('+' or '-'). If the sign is omitted, ‘+’ shall be assumed. Leading zeroes may be used.
The range of values that may be expressed as an integer is:

   :math:`-2,147,483,648 \le x \le +2,147,483,647` (i.e., :math:`-2^{31} \le x \le 2^{31}-1`).

.. NOTE::

   The commas in the range of values above are thousands separators and are used
   only for readability.

6.3.2.2 Non-integer numeric values expressed in fixed-point notation shall consist of a
sequence of decimal digits separated by a period as a decimal point indicator, with an
optional leading sign ('+' or '-'). If the sign is omitted, ‘+’ shall be assumed. Leading and
trailing zeroes may be used. At least one digit shall appear before and after a decimal point.
The number of digits shall be 16 or fewer.

6.3.2.3 Non-integer numeric values expressed in floating point notation shall consist of a
sign, a mantissa, an alphabetic character indicating the division between the mantissa and
exponent, and an exponent, constructed according to the following rules:

a) The sign may be '+' or '-'. If the sign is omitted, '+' shall be assumed.
b) The mantissa must be a string of no more than 16 decimal digits with a decimal point
   ('.') in the second position of the ASCII string, separating the integer portion of the
   mantissa from the fractional part of the mantissa.
c) The character used to denote exponentiation shall be ‘E’ or ‘e’. If the character
   indicating the exponent and the following exponent are omitted, an exponent value of
   zero shall be assumed (essentially yielding a fixed point value).
d) The exponent must be an integer, and may have either a ‘+’ or ‘-’ sign; if the sign is
   omitted, then ‘+’ shall be assumed.
e) The maximum positive floating point value is approximately 1.798E+308, with 16
   significant decimal digits precision. The minimum positive floating point value is
   approximately 4.94E-324, with 16 significant decimal digits precision.

6.3.2.4 Blanks shall not be used within numeric values.

6.3.2.5 In value fields that are text, an underscore shall be equivalent to a single blank.
Individual blanks shall be retained (shall be significant), but multiple contiguous blanks shall
be equivalent to a single blank.

6.3.2.6 In value fields that represent a time tag, times shall be given in one of the following
two formats:

   ``yyyy-mm-ddThh:mm:ss[.d→d][Z]``

or

   ``yyyy-dddThh:mm:ss[.d→d][Z]``

where 'yyyy' is the year, ‘mm’ is the two-digit month, ‘dd’ is the two-digit day of the month,
and 'ddd' is the three-digit day of the year, separated by hyphens; ‘T’ is a fixed separator
between the date and time portions of the string; and ‘hh:mm:ss[.d→d]’ is the time in hours,
minutes, seconds, and fractional seconds, separated by colons. As many ‘d’ characters to the
right of the period as required may be used to obtain the required precision, up to the
maximum allowed for a fixed-point number. Because all times in the CDM are UTC, the ‘Z’
indicator allowed by the CCSDS Time Code Formats Recommended Standard should be
omitted. All fields require leading zeros. (See reference [5], ASCII Time Code A or B.)

.. _cdm_units_in_kvn_cdm:

6.3.3 CDM UNITS IN KVN
^^^^^^^^^^^^^^^^^^^^^^^^

When units are displayed, then:

a) there must be at least one blank character between the value and the units;
b) the units must be enclosed within square brackets (e.g., '[km]').

.. _cdm_comments_in_kvn_cdm:

6.3.4 CDM COMMENTS IN KVN
^^^^^^^^^^^^^^^^^^^^^^^^^^^

All comment lines shall begin with the ‘COMMENT’ keyword followed by at least one
space. This keyword must appear on every comment line, not just the first such line. The
remainder of the line shall be the comment value. White space shall be retained (shall be
significant) in comment values.

.. _the_cdm_in_xml_cdm:

6.4 THE CDM IN XML
------------------

.. _cdm_lines_in_xml_cdm:

6.4.1 CDM LINES IN XML
^^^^^^^^^^^^^^^^^^^^^^^^

6.4.1.1 Each CDM file shall consist of a set of CDM lines. Each CDM line shall be one of
the following:

-  XML version line;
-  an XML-formatted line; or
-  a blank line.

6.4.1.2 The first line in the instantiation shall specify the XML version.

6.4.1.3 While specific formatting of an XML message is not critical, and white space and
line breaks are not significant, the message should be organized and formatted to facilitate
human comprehension.

.. _cdm_values_in_xml_cdm:

6.4.2 CDM VALUES IN XML
^^^^^^^^^^^^^^^^^^^^^^^^^

6.4.2.1 Integer values shall follow the conventions of the integer data type per reference [4].
Additional restrictions on the values permitted for any integer data element may also be
defined in the CDM XML Schema.

.. NOTE::

   Examples of such restrictions may include a defined range (e.g., 0 - 100, 1 - 10,
   etc.), a set of enumerated values (e.g., 0, 1, 2, 4, 8), a predefined specific
   variation such as positiveInteger, or a user-defined data type variation.

6.4.2.2 Non-integer numeric values shall follow the conventions of the double data type per
reference [4]. Additional restrictions on the allowable range or values permitted for any non-
integer numeric data element may also be defined in the CDM XML Schema.

.. NOTE::

   Examples of such restrictions may include a defined range (e.g., 0.0 - 100.0,
   etc.), or a user-defined data type variation.

6.4.2.3 Text value data shall follow the conventions of the string data type per reference [4].
Additional restrictions on the values permitted for any data element may also be defined in
the CDM XML Schema.

.. NOTE::

   Examples of such restrictions may include a set of enumerated values (e.g.,
   'YES'/'NO', or ‘RTN’/‘TVN’), or other user-defined data type variation.

6.4.2.4 In value fields that represent a time tag, values shall follow the conventions of the
ndm:epochType data type used in all CCSDS NDM/XML schemas. This data type supports
the options specified in 6.3.2.6.

.. _cdm_units_in_xml_cdm:

6.4.3 CDM UNITS IN XML
^^^^^^^^^^^^^^^^^^^^^^^^

CDM units shall be expressed as attributes in XML keyword tags in the form ‘units="unit-
notation"’, where unit-notation conforms to the convention stated in 1.4.1.1.

.. NOTE::

   Table 6-1 gives examples of XML keyword tags with specified units.

.. _table_6-1_cdm:

.. list-table:: Table 6-1: Example XML Keyword Tags with Specified Units
   :widths: 20 10 70
   :header-rows: 1

   * - Tag
     - Units
     - Example
   * - MISS_DISTANCE
     - m
     - <MISS_DISTANCE units="m">715</MISS_DISTANCE>
   * - RELATIVE_SPEED
     - m/s
     - <RELATIVE_SPEED units="m/s">14762</RELATIVE_SPEED>
   * - ACTUAL_OD_SPAN
     - d
     - <ACTUAL_OD_SPAN units="d">5.50</ACTUAL_OD_SPAN>


.. _cdm_comments_in_xml_cdm:

6.4.4 CDM COMMENTS IN XML
^^^^^^^^^^^^^^^^^^^^^^^^^^^

Comments must be displayed as values between the <COMMENT> and </COMMENT>
tags.

.. _annex_a_cdm:

ANNEX A
=======

IMPLEMENTATION CONFORMANCE STATEMENT PROFORMA
---------------------------------------------

(NORMATIVE)

.. _annex_a_introduction_cdm:

A1 INTRODUCTION
---------------

A1.1 OVERVIEW
^^^^^^^^^^^^^^

This annex provides the Implementation Conformance Statement (ICS) Requirements List
(RL) for an implementation of Conjunction Data Message (CCSDS 508.0). The ICS for an
implementation is generated by completing the RL in accordance with the instructions
below. An implementation shall satisfy the mandatory conformance requirements referenced
in the RL.

The RL in this annex is blank. An implementation's completed RL is called the ICS. The ICS
states which capabilities and options have been implemented. The following can use the ICS:

-  the implementer, as a checklist to reduce the risk of failure to conform to the standard
   through oversight;
-  a supplier or potential acquirer of the implementation, as a detailed indication of the
   capabilities of the implementation, stated relative to the common basis for
   understanding provided by the standard ICS proforma;
-  a user or potential user of the implementation, as a basis for initially checking the
   possibility of interworking with another implementation (it should be noted that,
   while interworking can never be guaranteed, failure to interwork can often be
   predicted from incompatible ICSes);
-  a tester, as the basis for selecting appropriate tests against which to assess the claim
   for conformance of the implementation.

.. _annex_a_abbreviations_cdm:

A1.2 ABBREVIATIONS AND CONVENTIONS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The RL consists of information in tabular form. The status of features is indicated using the
abbreviations and conventions described below.

Item Column
"""""""""""
The item column contains sequential numbers for items in the table.

Feature Column
""""""""""""""
The feature column contains a brief descriptive name for a feature. It implicitly means 'Is
this feature supported by the implementation?'

.. NOTE::
   The features itemized in the RL are elements of a CDM. Therefore support for a
   mandatory feature indicates that generated messages will include that feature,
   and support for an optional feature indicates that generated messages can include
   that feature.

Keyword Column
""""""""""""""
The keyword column contains, where applicable, the CDM keyword associated with the
feature.

Reference Column
""""""""""""""""
The reference column indicates the relevant subsection or table in *Conjunction Data
Message* (CCSDS 508.0) (this document).

Status Column
"""""""""""""
The status column uses the following notations:

M
   mandatory.
O
   optional.

Support Column Symbols
""""""""""""""""""""""
The support column is to be used by the implementer to state whether a feature is supported
by entering Y, N, or N/A, indicating:

Y
   Yes, supported by the implementation.
N
   No, not supported by the implementation.
N/A
   Not applicable.

.. _annex_a_instructions_cdm:

A1.3 INSTRUCTIONS FOR COMPLETING THE RL
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An implementer shows the extent of compliance to the Recommended Standard by
completing the RL; that is, the state of compliance with all mandatory requirements and the
options supported are shown. The resulting completed RL is called an ICS. The implementer
shall complete the RL by entering appropriate responses in the support or values supported
column, using the notation described in A1.2. If a conditional requirement is inapplicable,
N/A should be used. If a mandatory requirement is not satisfied, exception information must
be supplied by entering a reference Xi, where i is a unique identifier, to an accompanying
rationale for the noncompliance.

.. _annex_a_ics_proforma_cdm:

A2 ICS PROFORMA FOR CONJUNCTION DATA MESSAGE
---------------------------------------------

.. _annex_a_general_information_cdm:

A2.1 GENERAL INFORMATION
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. _annex_a_identification_ics_cdm:

A2.1.1 Identification of ICS
"""""""""""""""""""""""""""

.. list-table::
   :widths: 50 50
   :header-rows: 1

   * -
     -
   * - Date of Statement (DD/MM/YYYY)
     -
   * - ICS serial number
     -
   * - System Conformance statement
       cross-reference
     -

.. _annex_a_identification_iut_cdm:

A2.1.2 Identification of Implementation Under Test (IUT)
"""""""""""""""""""""""""""""""""""""""""""""""""""""""""

.. list-table::
   :widths: 50 50
   :header-rows: 1

   * -
     -
   * - Implementation name
     -
   * - Implementation version
     -
   * - Special Configuration
     -
   * - Other information
     -

.. _annex_a_identification_supplier_cdm:

A2.1.3 Identification of Supplier
"""""""""""""""""""""""""""""""""

.. list-table::
   :widths: 50 50
   :header-rows: 1

   * -
     -
   * - Supplier
     -
   * - Contact Point for Queries
     -
   * - Implementation Name(s) and Versions
     -
   * - Other information necessary for full
       identification, e.g., name(s) and version(s)
       for machines and/or operating systems;
     -
   * - System Name(s)
     -

.. _annex_a_document_version_cdm:

A2.1.4 Document Version
"""""""""""""""""""""""

.. list-table::
   :widths: 50 25 25
   :header-rows: 1

   * - CCSDS 508.0 Document Version
     -
     -
   * - Have any exceptions been required?
     - Yes
     - No
   * - (Note: A YES answer means that the implementation
       does not conform to the Recommended Standard.
       Non-supported mandatory capabilities are to be
       identified in the ICS, with an explanation of why the
       implementation is non-conforming.
     -
     -

.. _annex_a_requirements_list_cdm:

A2.1.5 Requirements List
^^^^^^^^^^^^^^^^^^^^^^

.. list-table::
   :widths: 5 25 20 15 10 10
   :header-rows: 1

   * - Item
     - Feature
     - Keyword
     - Reference
     - Status
     - Support
   * - 1
     - CDM Header
     - N/A
     - Table 3-1
     - M
     -
   * - 2
     - CDM version
     - CCSDS_CDM_VERS
     - Table 3-1
     - M
     -
   * - 3
     - Comment
     - COMMENT
     - Table 3-1
     - O
     -
   * - 4
     - Message creation date/time
     - CREATION_DATE
     - Table 3-1
     - M
     -
   * - 5
     - Message originator
     - ORIGINATOR
     - Table 3-1
     - M
     -
   * - 6
     - Spacecraft name(s)
     - MESSAGE_FOR
     - Table 3-1
     - O
     -
   * - 7
     - Unique message identifier
     - MESSAGE_ID
     - Table 3-1
     - M
     -
   * - 8
     - CDM Relative Metadata and
       Relative Data
     - N/A
     - Table 3-2
     - M
     -
   * - 9
     - Comment
     - COMMENT
     - Table 3-2
     - O
     -
   * - 10
     - Time of closest approach
     - TCA
     - Table 3-2
     - M
     -
   * - 11
     - Miss distance at TCA
     - MISS_DISTANCE
     - Table 3-2
     - M
     -
   * - 12
     - Relative speed at TCA
     - RELATIVE_SPEED
     - Table 3-2
     - O
     -
   * - 13
     - Relative position of Object 2
       with respect to Object 1
     - RELATIVE_POSITION_R,
       RELATIVE_POSITION_T,
       RELATIVE_POSITION_N
     - Table 3-2
     - O
     -
   * - 14
     - Relative velocity of Object 2
       with respect to Object 1
     - RELATIVE_VELOCITY_R,
       RELATIVE_VELOCITY_T,
       RELATIVE_VELOCITY_N
     - Table 3-2
     - O
     -
   * - 15
     - Conjunction assessment
       screening period start/stop
       times
     - START_SCREEN_PERIOD,
       STOP_SCREEN_PERIOD
     - Table 3-2
     - O
     -
   * - 16
     - Object1 centered screening
       volume reference frame,
       shape, and dimensions
     - SCREEN_VOLUME_FRAME,
       SCREEN_VOLUME_SHAPE,
       SCREEN_VOLUME_X,
       SCREEN_VOLUME_Y,
       SCREEN_VOLUME_Z
     - Table 3-2
     - O
     -
   * - 17
     - Screening volume entry/exit
       times for Object2
     - SCREEN_ENTRY_TIME,
       SCREEN_EXIT_TIME
     - Table 3-2
     - O
     -
   * - 18
     - Probability that Object1 and
       Object2 will collide
     - COLLISION_PROBABILITY
     - Table 3-2
     - O
     -
   * - 19
     - Method that was used to
       calculate collision probability
     - COLLISION_PROBABILITY_METHOD
     - Table 3-2
     - O
     -
   * - 20
     - CDM Metadata
     - N/A
     - Table 3-3
     - M
     -
   * - 21
     - Comment
     - COMMENT
     - Table 3-3
     - O
     -
   * - 22
     - Specifies object (1 or 2) to
       which metadata/data apply
     - OBJECT
     - Table 3-3
     - M
     -
   * - 23
     - Satellite catalog designator
       for the object
     - OBJECT_DESIGNATOR
     - Table 3-3
     - M
     -
   * - 24
     - Satellite catalog used for the
       object
     - CATALOG_NAME
     - Table 3-3
     - M
     -
   * - 25
     - Spacecraft name for the
       object
     - OBJECT_NAME
     - Table 3-3
     - M
     -
   * - 26
     - Full international designator
       for the object
     - INTERNATIONAL_DESIGNATOR
     - Table 3-3
     - M
     -
   * - 27
     - Type of space object
     - OBJECT_TYPE
     - Table 3-3
     - O
     -
   * - 28
     - Contact information for the
       object's owner/operator
     - OPERATOR_CONTACT_POSITION,
       OPERATOR_ORGANIZATION,
       OPERATOR_PHONE,
       OPERATOR_EMAIL
     - Table 3-3
     - O
     -
   * - 29
     - Name of the external
       ephemeris file used, if any.
     - EPHEMERIS_NAME
     - Table 3-3
     - M
     -
   * - 30
     - Describes how covariance
       matrix was derived
     - COVARIANCE_METHOD
     - Table 3-3
     - M
     -
   * - 31
     - Object's maneuver capacity
     - MANEUVERABLE
     - Table 3-3
     - M
     -
   * - 32
     - Defines the central body
       about which Object1/2 orbit
     - ORBIT_CENTER
     - Table 3-3
     - O
     -
   * - 33
     - Name of reference frame in
       which state vector is given
     - REF_FRAME
     - Table 3-3
     - M
     -
   * - 34
     - Gravity model used for OD
     - GRAVITY_MODEL
     - Table 3-3
     - O
     -
   * - 35
     - Atmospheric density model
       used for OD of the object
     - ATMOSPHERIC_MODEL
     - Table 3-3
     - O
     -
   * - 36
     - N-body gravitational
       perturbations used for OD
     - N_BODY_PERTURBATIONS
     - Table 3-3
     - O
     -
   * - 37
     - Indicates if solar radiation
       pressure perturbations were
       used in OD (Y/N)
     - SOLAR_RAD_PRESSURE
     - Table 3-3
     - O
     -
   * - 38
     - Indicates if solid Earth and
       ocean tides were used in
       OD (Y/N)
     - EARTH_TIDES
     - Table 3-3
     - O
     -
   * - 39
     - Indicates if in-track thrust
       modeling was used in OD
       (Y/N)
     - INTRACK_THRUST
     - Table 3-3
     - O
     -
   * - 40
     - CDM Data
     - N/A
     - Table 3-4
     - M
     -
   * - 41
     - Comment
     - COMMENT
     - Table 3-4
     - O
     -
   * - 42
     - Orbit Determination
       Parameters
     - N/A
     - Table 3-4
     - O
     -
   * - 43
     - Comment
     - COMMENT
     - Table 3-4
     - O
     -
   * - 44
     - Interval containing last
       accepted observation
     - TIME_LASTOB_START,
       TIME_LASTOB_END
     - Table 3-4
     - O
     -
   * - 45
     - Recommended/actual
       OD time span for object
     - RECOMMENDED_OD_SPAN,
       ACTUAL_OD_SPAN
     - Table 3-4
     - O
     -
   * - 46
     - Number of observations
       available/accepted in OD
     - OBS_AVAILABLE,
       OBS_USED
     - Table 3-4
     - O
     -
   * - 47
     - Number of sensor tracks
       available/accepted in OD
     - TRACKS_AVAILABLE,
       TRACKS_USED
     - Table 3-4
     - O
     -
   * - 48
     - Percentage of residuals
       accepted in OD
     - RESIDUALS_ACCEPTED
     - Table 3-4
     - O
     -
   * - 49
     - Weighted RMS of the
       residuals from OD
     - WEIGHTED_RMS
     - Table 3-4
     - O
     -
   * - 50
     - Additional Modeling
       Parameters
     - N/A
     - Table 3-4
     - O
     -
   * - 51
     - Comment
     - COMMENT
     - Table 3-4
     - O
     -
   * - 52
     - Actual area of the object
     - AREA_PC
     - Table 3-4
     - O
     -
   * - 53
     - Effective area of object
       exposed to atmospheric
       drag
     - AREA_DRG
     - Table 3-4
     - O
     -
   * - 54
     - Effective area of object
       exposed to solar
       radiation pressure
     - AREA_SRP
     - Table 3-4
     - O
     -
   * - 55
     - Mass of the object
     - MASS
     - Table 3-4
     - O
     -
   * - 56
     - Object's :math:`C_D A/m` and
       :math:`C_R A/m` used to
       propagate state vector
       covariance to TCA
     - CD_AREA_OVER_MASS,
       CR_AREA_OVER_MASS
     - Table 3-4
     - O
     -
   * - 57
     - Object's acceleration
       due to in-track thrust
       used to propagate state
       vector/covariance to
       TCA
     - THRUST_ACCELERATION
     - Table 3-4
     - O
     -
   * - 58
     - Specific Energy
       Dissipation Rate (SEDR)
     - SEDR
     - Table 3-4
     - O
     -
   * - 59
     - State Vector
     - N/A
     - Table 3-4
     - M
     -
   * - 60
     - Comment
     - COMMENT
     - Table 3-4
     - O
     -
   * - 61
     - Object Position Vector
     - X, Y, Z
     - Table 3-4
     - M
     -
   * - 62
     - Object Velocity Vector
     - X_DOT, Y_DOT, Z_DOT
     - Table 3-4
     - M
     -
   * - 63
     - Covariance Matrix
     - N/A
     - Table 3-4
     - M
     -
   * - 64
     - Comment
     - COMMENT
     - Table 3-4
     - O
     -
   * - 65
     - Position/velocity 6x6
       covariance matrix
     - CR_R, CT_R, CT_T, CN_R, CN_T,
       CN_N, CRDOT_R, CRDOT_T,
       CRDOT_N, CRDOT_RDOT,
       CTDOT_R, CTDOT_T, CTDOT_N,
       CTDOT_RDOT, CTDOT_TDOT,
       CNDOT_R, CNDOT_T, CNDOT_N,
       CNDOT_RDOT, CNDOT_TDOT,
       CNDOT_NDOT
     - Table 3-4
     - M
     -
   * - 66
     - Covariance matrix row 7
       (Drag related)
     - CDRG_R, CDRG_T, CDRG_N,
       CDRG_RDOT, CDRG_TDOT,
       CDRG_NDOT, CDRG_DRG
     - Table 3-4
     - O
     -
   * - 67
     - Covariance matrix row 8
       (Solar Radiation
       Pressure related)
     - CSRP_R, CSRP_T, CSRP_N,
       CSRP_RDOT, CSRP_TDOT,
       CSRP_NDOT, CSRP_DRG,
       CSRP_SRP
     - Table 3-4
     - O
     -
   * - 68
     - Covariance matrix row 9
       (In-track Thrust related)
     - CTHR_R, CTHR_T, CTHR_N,
       CTHR_RDOT, CTHR_TDOT,
       CTHR_NDOT, CTHR_DRG,
       CTHR_SRP, CTHR_THR
     - Table 3-4
     - O
     -

.. _annex_b_cdm:

ANNEX B
=======

SECURITY, SANA, AND PATENT CONSIDERATIONS
-----------------------------------------

(INFORMATIVE)

.. _annex_b_security_cdm:

B1 SECURITY CONSIDERATIONS
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. _annex_b_analysis_security_cdm:

B1.1 ANALYSIS OF SECURITY CONSIDERATIONS
""""""""""""""""""""""""""""""""""""""""

This subsection presents the results of an analysis of security considerations applied to the
technologies specified in this Recommended Standard.

.. _annex_b_consequences_cdm:

B1.2 CONSEQUENCES OF NOT APPLYING SECURITY TO THE
     TECHNOLOGY
""""""""""""""""""""""""""""""""""""""""""""""""""""

The consequences of not applying security to the systems and networks on which this
Recommended Standard is implemented could include potential loss, corruption, and theft of
data. Because these messages are used in collision avoidance analyses and potential
maneuvers, the consequences of not applying security to the systems and networks on which
this Recommended Standard is implemented could include compromise or loss of the
mission if malicious tampering of a particularly severe nature occurs.

.. _annex_b_potential_threats_cdm:

B1.3 POTENTIAL THREATS AND ATTACK SCENARIOS
""""""""""""""""""""""""""""""""""""""""""""""

Potential threats or attack scenarios include, but are not limited to, (a) unauthorized access to
the programs/processes that generate and interpret the messages, and (b) unauthorized access
to the messages during transmission between exchange partners. Protection from
unauthorized access during transmission is especially important if the mission utilizes open
ground networks, such as the Internet, to provide ground-station connectivity for the
exchange of data formatted in compliance with this Recommended Standard. It is strongly
recommended that potential threats or attack scenarios applicable to the systems and
networks on which this Recommended Standard is implemented be addressed by the
management of those systems and networks.

.. _annex_b_data_privacy_cdm:

B1.4 DATA PRIVACY
"""""""""""""""""""""

Privacy of data formatted in compliance with the specifications of this Recommended
Standard should be assured by the systems and networks on which this Recommended
Standard is implemented.

.. _annex_b_data_integrity_cdm:

B1.5 DATA INTEGRITY
""""""""""""""""""""""

Integrity of data formatted in compliance with the specifications of this Recommended
Standard should be assured by the systems and networks on which this Recommended
Standard is implemented.

.. _annex_b_authentication_cdm:

B1.6 AUTHENTICATION OF COMMUNICATING ENTITIES
""""""""""""""""""""""""""""""""""""""""""""""""

Authentication of communicating entities involved in the transport of data which complies
with the specifications of this Recommended Standard should be provided by the systems
and networks on which this Recommended Standard is implemented.

.. _annex_b_data_transfer_cdm:

B1.7 DATA TRANSFER BETWEEN COMMUNICATING ENTITIES
"""""""""""""""""""""""""""""""""""""""""""""""""""""

The transfer of data formatted in compliance with this Recommended Standard between
communicating entities should be accomplished via secure mechanisms approved by the
Information Technology Security functionaries of exchange participants.

.. _annex_b_control_access_cdm:

B1.8 CONTROL OF ACCESS TO RESOURCES
"""""""""""""""""""""""""""""""""""""""

Control of access to resources should be managed by the systems upon which originator
formatting and recipient processing are performed.

.. _annex_b_auditing_cdm:

B1.9 AUDITING OF RESOURCE USAGE
"""""""""""""""""""""""""""""""""

Auditing of resource usage should be handled by the management of systems and networks
on which this Recommended Standard is implemented.

.. _annex_b_unauthorized_access_cdm:

B1.10 UNAUTHORIZED ACCESS
"""""""""""""""""""""""""""""

Unauthorized access to the programs/processes that generate and interpret the messages
should be prohibited in order to minimize potential threats and attack scenarios.

.. _annex_b_security_implementation_cdm:

B1.11 DATA SECURITY IMPLEMENTATION SPECIFICS
"""""""""""""""""""""""""""""""""""""""""""""

Specific information-security interoperability provisions that may apply between agencies
and other independent users involved in an exchange of data formatted in compliance with
this Recommended Standard could be specified in an ICD.

.. _annex_b_sana_considerations_cdm:

B2 SANA CONSIDERATIONS
^^^^^^^^^^^^^^^^^^^^^^

The following CDM-related items will be registered with the SANA Operator. The
registration rule for new entries in the registry is the approval of new requests by the CCSDS
Navigation Working Group chair. New requests for this registry should be sent to SANA
(mailto:info@sanaregistry.org).

-  The CDM XML schema;
-  A transform from the CDM XML to the CDM KVN version;
-  Values for the keywords ORIGINATOR and CATALOG_NAME; and
-  A list of options for the COLLISION_PROBABILITY_METHOD keyword.

.. _annex_b_patent_considerations_cdm:

B3 PATENT CONSIDERATIONS
^^^^^^^^^^^^^^^^^^^^^^^^

The recommendations of this document have no patent issues.

.. _annex_c_cdm:

ANNEX C
=======

ABBREVIATIONS AND ACRONYMS
--------------------------

(INFORMATIVE)

ACS
   attitude control system

ACM
   Attitude Comprehensive Message

ADCS
   attitude determination and control system

ADM
   Attitude Data Message

AEM
   Attitude Ephemeris Message

AOS
   acquisition of signal

APM
   Attitude Parameter Message

ASCII
   American Standard Code for Information Interchange

CA
   Conjunction Assessment

CARA
   Conjunction Assessment Risk Analysis

CCSDS
   Consultative Committee for Space Data Systems

CDM
   Conjunction Data Message

CNES
   Centre National d’Etudes Spatiales

CSpOC
   Combined Space Operations Center

CSTS
   Cross Support Transfer Services

DLR
   Deutsches Zentrum fur Luft-und Raumfahrt

DOR
   differential one-way ranging

DSN
   Deep Space Network

EFT-1
   Exploration Flight Test-1

ESA
   European Space Agency

ESOC
   European Space Operations Centre

FDM
   Fragmentation Data Message

FTP
   File Transfer Protocol

GN&C
   Guidance, Navigation, and Control

GNSS
   Global Navigation Satellite System

GSFC
   Goddard Space Flight Center

ICD
   interface control document

ID
   identification

IRU
   inertial reference unit

ISRO
   Indian Space Research Organization

ISS
   International Space Station

JAXA
   Japan Aerospace Exploration Agency

JPL
   Jet Propulsion Laboratory

JSC
   Johnson Space Center

KVN
   keyword value notation

LOS
   loss of signal

MMS
   Magnetospheric Multiscale Mission

NASA
   National Aeronautics and Space Administration

NDM
   Navigation Data Message

NEM
   Navigation Event Message

NORAD
   North American Aerospace Defense Command

OCM
   Orbit Comprehensive Message

ODM
   Orbit Data Message

OEM
   Orbit Ephemeris Message

OMM
   Orbit Mean-Elements Message

OPM
   Orbit Parameter Message

POC
   probability of collision

PRM
   Pointing Request Message

RDM
   Re-Entry Data Message

SANA
   Space Assigned Numbers Authority

SFTP
   Secure File Transfer Protocol

SI
   International System of Units

SSA
   space situational awareness

SST
   space surveillance and tracking

TCA
   time of closest approach

TDM
   Tracking Data Message

TDRSS
   Tracking and Data Relay Satellite System

TLE
   two-line elements

UT1
   Universal Time

UTC
   Coordinated Universal Time

WRMS
   weighted root-mean square

XML
   Extensible Markup Language

.. _annex_d_cdm:

ANNEX D
=======

RATIONALE AND REQUIREMENTS FOR
CONJUNCTION DATA MESSAGES
-----------------------------

(INFORMATIVE)

.. _annex_d_overview_cdm:

D1 OVERVIEW
-----------

This annex presents the rationale behind the design of the Conjunction Data Message.

A specification of requirements agreed to by all parties is essential to focus design and to
ensure the product meets the needs of the satellite owner/operators and other authorized
parties. There are many ways of organizing requirements, but the categorization of
requirements is not as important as the agreement on a sufficiently comprehensive set. In
this annex, the requirements are organized into two categories:

a) Primary Requirements, which are the most elementary and necessary requirements.
   They would exist no matter the context in which the CCSDS is operating, i.e.,
   regardless of pre-existing conditions within the CCSDS, satellite owner/operators, or
   other independent users.
b) Desirable Characteristics, which are not requirements, but are felt to be important or
   useful features of the Recommended Standard.

.. _annex_d_primary_requirements_cdm:

D2 PRIMARY REQUIREMENTS ACCEPTED BY THE CDM
-------------------------------------------

.. _table_d-1_cdm:

.. list-table:: Table D-1: Primary Requirements
   :widths: 10 30 30 30
   :header-rows: 1

   * - Reqt #
     - Requirement
     - Rationale
     - Trace
   * - CDM-P01
     - The CDM data shall be provided in
       digital form (computer file).
     - Facilitates computerized processing
       of CDMs.
     - 3.1.1, 3.1.2
   * - CDM-P02
     - The CDM shall be provided in data
       structures (e.g., files) that are readily
       ported between, and useable within,
       'all' computing environments in use
       by satellite owner/operators and
       other authorized parties.
     - The CCSDS objective of promoting
       interoperability is not met if messages
       are produced using esoteric or
       proprietary data structures.
     - 3.1.2
   * - CDM-P03
     - The CDM shall provide a
       mechanism by which messages may
       be uniquely identified and clearly
       annotated. The file name alone is
       considered insufficient for this
       purpose.
     - Facilitates discussion between a
       message recipient and the originator
       should it become necessary.
     - Table 3-1
   * - CDM-P04
     - The CDM shall clearly and
       unambiguously identify the two
       objects involved in a conjunction.
     - This information is fundamental to the
       owner/operators of the objects in the
       conjunction. Cited as required in ISO
       16158 (reference [F2]).
     - Table 3-3
   * - CDM-P05
     - The CDM shall provide the time of
       closest approach of the two objects
       involved in the conjunction.
     - This datum is required in order to
       determine remaining reaction time, to
       assess the risk of collision, and to
       assess potential preventive
       measures. Cited as required in ISO
       16158 (reference [F2]).
     - Table 3-2
   * - CDM-P06
     - The CDM shall provide time
       measurements (time stamps, or
       epochs) in commonly used, clearly
       specified systems.
     - The CCSDS objective of promoting
       interoperability is not met if time
       measurements are produced in
       esoteric or proprietary time systems.
     - 6.3.2.6, 6.4.2.4,
   * - CDM-P07
     - The CDM shall provide the states of
       the two objects involved in the
       conjunction at the time of closest
       approach.
     - The states at time of closest
       approach are required for calculation
       of collision probability in most
       methods. This information is useful
       to owner/operators who wish to
       perform an independent assessment
       of the conjunction and/or the
       probability of collision. Cited as
       desirable in ISO 16158
       (reference [F2]).
     - Table 3-4
   * - CDM-P08
     - The CDM shall provide the miss
       distance of the two objects involved
       in the conjunction at the time of
       closest approach.
     - This datum is required in order to
       assess the risk of collision and
       assess potential preventive
       measures. Cited as required in ISO
       16158 (reference [F2]).
     - Table 3-2
   * - CDM-P09
     - The CDM shall provide state vector
       information for both objects involved
       in the conjunction in a reference
       frame that is clearly identified and
       unambiguous.
     - Clearly understanding the frame of
       reference in which measurements are
       provided is fundamental to the
       analysis of most, if not all, physical
       processes. Cited as required in ISO
       16158 (reference [F2]).
     - Table 3-3
   * - CDM-P10
     - The CDM shall provide for clear
       specification of units of measure.
     - Without clear specification of units of
       measure, mistakes can be made that
       involve the unit system in effect (e.g.,
       Metric or Imperial) and/or orders of
       magnitude (e.g., meters or
       kilometers).
     - Table 3-4,
       4.3.10, 6.3.3,
       6.4.3, Table 3-2
   * - CDM-P11
     - The CDM shall provide a covariance
       matrix that includes at least 6x6
       position/velocity uncertainty
       information.
     - The determination of a satellite state
       is subject to measurement and
       process uncertainties at all phases of
       its development. Consideration of
       this uncertainty is a necessary part of
       conjunction analysis and risk
       assessment. The covariance matrix
       captures the requisite uncertainty.
       Cited as required in ISO 16158
       (reference [F2]).
     - Table 3-4
   * - CDM-P12
     - The CDM shall provide the most
       recently known operational status of
       the two objects.
     - This datum is required in order to
       assess the risk of collision and
       assess potential preventive
       measures. Cited as required in ISO
       16158 (reference [F2]).
     - Table 3-3
   * - CDM-P13
     - The CDM shall allow the possibility
       to exchange information regarding
       conjunctions of objects orbiting an
       arbitrary body or point in space.
     - While Earth is the most likely central
       body about which orbiting objects
       may collide, there are other orbit
       centers with more than one orbiting
       object (e.g., the Moon, Mars,
       Earth/Sun L1, Earth/Sun L2).
     - Table 3-3
   * - CDM-P14
     - The CDM shall provide data and/or
       metadata that will allow the recipient
       to calculate the probability of
       collision if it is not provided by the
       CDM originator.
     - Some CDM originators will not want
       to explicitly provide a probability of
       collision, but their customers may be
       interested in performing a calculation
       of their own based on data in the
       CDM. The probability of collision is
       cited as desirable in ISO 16158
       (reference [F2]).
     - Table 3-2,
       Table 3-3,
       Table 3-4
   * - CDM-P15
     - The CDM must not require of the
       receiving exchange partner the
       separate application of, or modeling
       of, spacecraft dynamics or
       gravitational force models, or
       integration or propagation.
     - The situation in which a CDM is
       provided may not allow time for
       checking/confirming a predicted
       conjunction by a recipient. Some
       owner/operators may not be able to
       perform the required computations.
     - Table 3-2,
       Table 3-3,
       Table 3-4
   * - CDM-P16
     - The CDM shall provide an indicator
       as to the ephemerides that were
       used in identifying the conjunction.
     - Informs the recipient as to whether
       the ephemeris used was
       owner/operator supplied or was
       created by the CDM originator.
     - Table 3-3

.. _table_d-2_cdm:

.. list-table:: Table D-2: Desirable Characteristics
   :widths: 5 30 30 30
   :header-rows: 1

   * - ID
     - Requirement
     - Rationale
     - Trace
   * - CDM-D01
     - The CDM should be extensible with
       no disruption to existing users/uses.
     - Space agencies and owner/operators
       upgrade systems and processes on
       schedules that make sense for their
       organizations. In practice, some
       organizations will be early adopters
       but others will opt to wait until
       performance of a new version of the
       CDM has been proven in other
       operations facilities.
     - Table 3-1
   * - CDM-D02
     - The CDM should be as consistent
       as reasonable with any related
       CCSDS Recommended Standards
       used for Earth-to-spacecraft or
       spacecraft-to-spacecraft
       applications.
     - Ideally, the set of Recommended
       Standards developed by a given
       CCSDS Working Group will be
       consistent.
     - 2.2
   * - CDM-D03
     - CDM originators should maintain
       consistency with respect to the
       optional keywords provided in their
       implementations; i.e., the
       composition of the CDMs provided
       should not change on a frequent
       basis.
     - Implementations that change on a
       frequent basis do not promote stable
       operations or interoperability.
     - 1.2
   * - CDM-D04
     - The CDM should allow the option for
       originators to provide a probability of
       collision of the two objects involved
       in the conjunction.
     - Some CDM originators will be
       interested in providing this datum.
       Cited as desirable by ISO 16158
       (reference [F2]).
     - Table 3-2
   * - CDM-D05
     - The CDM should provide information
       with which each object's spherical
       radius may be calculated.
     - The object radius is required for
       calculation of collision probability in
       most methods, which usually model
       objects as spheres given the lack of
       attitude information.
     - Table 3-4
   * - CDM-D06
     - The CDM should provide the
       threshold of close approach used by
       the originator in the screening.
     - This datum is desirable in order to
       assess the risk of collision and
       assess potential preventive
       measures. Cited as desirable by ISO
       16158 (reference [F2]).
     - Table 3-2
   * - CDM-D07
     - The CDM should provide the
       components of the relative position
       at the time of closest approach.
     - These data allow an owner/operator
       to quickly do a first-order qualitative
       assessment of the probability of
       collision immediately upon receipt of
       a CDM.
     - Table 3-2
   * - CDM-D08
     - The CDM should provide the relative
       velocity of the two objects in the
       conjunction at the time of closest
       approach.
     - This datum is desirable in order to
       assess the risk of collision and
       assess potential preventive
       measures. Cited as desirable by ISO
       16158 (reference [F2]).
     - Table 3-2
   * - CDM-D09
     - The CDM shall be provided using
       file name syntax and length that do
       not violate computer constraints for
       those computing environments in use
       by satellite owner/operators and
       other authorized parties.
     - The CCSDS objective of promoting
       interoperability is not met if messages
       are provided using nonstandard file-
       name syntax or length.
     - 3.1.2

.. _annex_e_cdm:

ANNEX E
=======

CONJUNCTION INFORMATION DESCRIPTION
-----------------------------------

(INFORMATIVE)

.. _annex_e_relative_data_cdm:

E1 RELATIVE DATA
----------------

TCA (Time of Closest Approach): The date and time of the predicted conjunction. This
time tag is also the epoch of the relative state vector, Object1 and Object2 state vectors, as
well as the effective time of the covariance matrices for both Object1 and Object2.

COLLISION_PROBABILITY: The probability that Object1 and Object2 will collide.

COLLISION_PROBABILITY_METHOD: The method used to compute the value associated
with the COLLISION_PROBABILITY keyword. Example options are ‘FOSTER-1992' (see
reference [F4]), ‘CHAN-1997’ (see reference [F8]), ‘PATERA-2001' (see reference [F6]),
‘ALFANO-2005' (see reference [F7]), and ‘MCKINLEY-2006' (see reference [F9]). A list of
currently registered options is available on the SANA Registry at http://sanaregistry.org. (To
register a new option for this keyword, see annex B, subsection B2.)

MISS_DISTANCE: The miss distance is the norm of the relative position vector. It
indicates how close the two objects are at the time of the predicted encounter.

RELATIVE_SPEED: The relative speed is the norm of the relative velocity vector. It
indicates how fast the two objects are moving relative to each other at the time of the
predicted encounter.

RELATIVE_POSITION/RELATIVE_VELOCITY: Object2's position/velocity relative
to Object1's position/velocity, calculated by taking the difference of the position and velocity
vectors relative to the frame in which they are defined, with components expressed in the
Object1-centered RTN coordinate frame at the time of closest approach.

RTN Coordinate Frame: Object-centered coordinate system. The Object1-centered RTN
coordinate frame: R (Radial) is the unit vector in the radial direction pointed outward from the
center of the central body, T (Transverse) is the unit vector perpendicular to the R vector in the
direction of the spacecraft velocity, and N (Normal) is the unit vector normal to the satellite's
inertial orbit plane (in the direction of the satellite's angular momentum) that completes the
right-hand coordinate frame (see :ref:`figure E-1 <figure_e-1_cdm>`).

TVN Coordinate Frame: Object-centered coordinate system. The Object1-centered TVN
coordinate frame is defined as: V (Velocity) is the unit vector in the inertial velocity direction,
N (Normal) is the unit vector normal to the satellite's inertial orbit plane (in the direction of the
satellite's angular momentum), and T (Transverse) is the unit vector that completes the right-
hand coordinate frame (see :ref:`figure E-1 <figure_e-1_cdm>`).

Commonality Between RTN and TVN
"""""""""""""""""""""""""""""""""

The primary difference between the RTN and the TVN frames is that the RTN frame is
anchored on the unit radial vector R, and the TVN frame is anchored on the unit inertial
velocity vector V. The unit normal vector N is the same vector for both the RTN and TVN
frames. The unit transverse vector T completes the right-hand coordinate frame for both the
RTN and TVN frames, but is not in the same direction for both frames. The TVN frame can
be particularly useful for analyzing non-circular orbits where the user would like one
coordinate axis to align with the velocity direction of motion. The RTN and TVN frames are
the same when Object1 is at apoapsis, periapsis, or when its orbit is perfectly circular.

.. _figure_e-1_cdm:

.. figure:: /images/placeholder.png
   :alt: Figure E-1: Definition of the RTN and TVN Coordinate Frames
   :align: center

   Figure E-1: Definition of the RTN and TVN Coordinate Frames

SCREEN_VOLUME_SHAPE/SCREEN_VOLUME: Shape (ellipsoid or box) of the
screening volume used to screen the satellite catalog for possible conjunctors with Object1.
The screening volume is the component size of the screening volume shape (in the Object1
centered RTN or TVN reference frame).

.. _annex_e_orbit_determination_cdm:

E2 ORBIT DETERMINATION PARAMETERS
---------------------------------

Observation: Unique measurement of a satellite's location from a single sensor at a single
time (e.g., azimuth from a single sensor at a single time).

TIME_LASTOB_START and TIME_LASTOB_END: The start and end of a time
interval (UTC) that contains the time of the last accepted observation (see 6.3.2.6 for
formatting rules). For an exact time, the time interval is of zero duration (i.e.,
TIME_LASTOB_START = TIME_LASTOB_END).

RECOMMENDED_OD_SPAN: How many days of observations were recommended for
the OD of the object.

ACTUAL_OD_SPAN: The actual time span used for the OD of the object based on the
observations available and the RECOMMENDED_OD_SPAN.

OBS_AVAILABLE: The number of observations, for the recommended time span, that
were available for the OD.

OBS_USED: The number of observations, for the recommended time span, that were
accepted for the OD.

Sensor Track: A set of at least three observations for the same object, observed by the same
sensor, where each observation is within a specified number of minutes (which is dependent
on the orbit regime of the object) of the other observations in the track.

TRACKS_AVAILABLE: The number of sensor tracks, for the recommended time span,
that were available for the OD. This provides information about the independence of the
observational data used in the OD.

TRACKS_USED: The number of sensor tracks, for the recommended time span, that were
accepted for the OD. This provides information about the independence of the observational
data used in the OD.

WEIGHTED_RMS:

.. math::
   \text{Weighted RMS} = \sqrt{\frac{\sum_{i=1}^{N} w_i (y_i - \hat{y}_i)^2}{N}}

Where

:math:`y_i` is the observation measurement at the ith time;

:math:`\hat{y}_i` is the estimate of :math:`y_i`;

:math:`w_i = \frac{1}{\sigma_i^2}` is the weight associated with the measurement at the ith time; and

:math:`N` is the number of observations.

This is a value that can generally identify the quality of the most recent vector update, and is
used by the analyst in evaluating the OD process. A value of 1.00 is ideal.

.. _annex_e_model_parameters_cdm:

E3 MODEL PARAMETERS
-------------------

GRAVITY_MODEL: The geopotential model used in the state vector update. The degree
(D) and order (O) of the spherical harmonic coefficients applied should be given along with
the name of the model.

ATMOSPHERIC_MODEL: The atmospheric density model used in the state vector
update.

N_BODY_PERTURBATIONS: Which (if any) N-body gravitational perturbations were
included in the state vector update. The value is a comma-separated list of the body names.

SOLAR_RAD_PRESSURE: Whether perturbations due to solar radiation pressure were
included in the state vector update.

EARTH_TIDES: Whether perturbations due to solid Earth and ocean tides were included in
the state vector update.

.. _annex_e_additional_parameters_cdm:

E4 ADDITIONAL PARAMETERS
------------------------

AREA_PC: The actual area of the object (m**2). The area could be known by the
owner/operator of the satellite or defined by using a Radar Cross Section (RCS) as in the
case of debris. If the value of the area is unknown or not available, ‘0.0' may be displayed.
This parameter can be useful for calculating the collision probability.

AREA_DRG: The effective area of the object (m**2) exposed to atmospheric drag.

AREA_SRP: The effective area of the object (m**2) exposed to solar radiation pressure.

CD_AREA_OVER_MASS: The coefficient of the perturbation of the object due to
atmospheric drag (m**2/kg) used to propagate the state vector and covariance to TCA,
defined as :math:`C_D A/m`, where :math:`C_D` is the drag coefficient, :math:`A` is the effective area of the object
exposed to atmospheric drag, and :math:`m` is the mass of the object.

CR_AREA_OVER_MASS: The coefficient of the perturbation of the object due to solar
radiation pressure (m**2/kg) used to propagate the state vector and covariance to TCA,
defined as :math:`C_R \cdot A/m`, calculated using solar flux at 1 AU, where :math:`C_R` is the solar radiation
pressure coefficient, :math:`A` is the effective area of the object exposed to solar radiation pressure
and :math:`m` is the mass of the object.

THRUST_ACCELERATION: The object's acceleration due to in-track thrust (m/s**2)
used to propagate the state vector and covariance of the object to TCA.

SEDR (Specific Energy Dissipation Rate): The amount of energy (W/kg) being removed
from a satellite's orbit by atmospheric drag. It is a very useful metric for characterizing
satellites since it takes into account both the drag environment (atmospheric density) and the
'area to mass ratio' of the specific object. It does this by including drag acceleration in the
computation. Drag acceleration is proportional to atmospheric density and to satellite area to
mass.

SEDR is computed as follows:

Instantaneous SEDR at time t is given by

.. math::
   \text{SEDR}(t) = -\vec{A}_D \cdot \vec{V}

where,

:math:`\vec{A}_D` = drag acceleration vector (inertial)
:math:`\vec{V}` = velocity vector (inertial)

Average SEDR over the orbit determination interval is given by

.. math::
   \frac{1}{T} \int_0^T \text{SEDR}(t)dt

where, in order to correctly average over a complete orbital revolution, :math:`T` is an integer
multiple of the satellite period. This consideration is primarily for eccentric orbits. Aside
from this consideration, :math:`T` is the orbit determination interval.

.. _annex_e_covariance_matrix_cdm:

E5 COVARIANCE MATRIX
--------------------

The covariance matrix is obligatory for the position and velocity terms, given in the lower
triangular form of a 6×6 matrix. If any of the diagonal terms are zero, the entire row and
column of the matrix related to that term should be discounted. Optional terms for
CD_AREA_OVER_MASS (denoted ‘DRG’), CR_AREA_OVER_MASS (denoted ‘SRP’),
and THRUST_ACCELERATION (denoted ‘THR’) can be added to the 6×6 matrix, in the
lower triangular form, to complete a 9×9 matrix. If any element in any of these rows (7, 8, or
9) is provided, then all of the elements for that row and all preceding rows need to be
provided (i.e., a subset of the terms for any of these rows is not allowed). The purpose for
providing the 7, 8, and 9 terms is so that users, who have the originator's propagator model
available (along with the appropriate CD_AREA_OVER_MASS
and/or CR_AREA_OVER_MASS and/or THRUST_ACCELERATION terms), can correctly
propagate the 6×6 position and velocity covariance to another time point.

.. _annex_f_cdm:

ANNEX F
=======

INFORMATIVE REFERENCES
----------------------

(INFORMATIVE)

[F1] Navigation Data—Definitions and Conventions. Issue 3. Report Concerning Space
    Data System Standards (Green Book), CCSDS 500.0-G-3. Washington, D.C.: CCSDS,
    May 2010.
[F2] Space Systems—Avoiding Collisions with Orbiting Objects. International Standard,
    ISO/TR 16158:2013. Geneva: ISO, 2013.
[F3] Astrodynamics—Propagation Specifications, Technical Definitions, and Recommended
    Practices. ANSI/AIAA S-131-2010. Reston, Virginia: AIAA, 2010.
[F4] J. L. Foster and H. S. Estes. A Parametric Analysis of Orbital Debris Collision
    Probability and Maneuver Rate for Space Vehicles. NASA/JSC-25898. Houston,
    Texas: NASA Johnson Space Flight Center, August 1992.
[F5] Ken Chan. “Collision Probability Analyses for Earth Orbiting Satellites.” In *Space
    Cooperation into the 21st Century: 7th AAS/JRS/CSA Symposium, International Space
    Conference of Pacific-Basin Societies* (ISCOPS; formerly PISSTA) (July 15-18, 1997,
    Nagasaki, Japan). 1033–1050. Edited by Peter M. Bainum, et al.. *Advances in the
    Astronautical Sciences Series* 96. San Diego, California: Univelt, 1997.
[F6] Russell P. Patera. “General Method for Calculating Satellite Collision Probability.”
    *Journal of Guidance, Control, and Dynamics* 24, no. 4 (July–August 2001): 716-722.
[F7] Salvatore Alfano. “A Numerical Implementation of Spherical Object Collision
    Probability." *The Journal of the Astronautical Sciences* 53, no. 1 (January-March
    2005): 103-109.
[F8] Salvatore Alfano. “Review of Conjunction Probability Methods for Short-Term
    Encounters." In *Proceedings of the 17th AAS/AIAA Space Flight Mechanics Meeting*
    (January 28 - February 1, 2007, Sedona, Arizona). 719–747. Edited by Maruthi R.
    Akella, et al.. *Advances in the Astronautical Sciences Series* 127. San Diego,
    California: Univelt, 2007.
[F9] David McKinley. “Development of a Nonlinear Probability of Collision Tool for the
    Earth Observing System.” In *Proceedings of AIAA/AAS Astrodynamics Specialist
    Conference and Exhibit* (21 August 2006–24 August 2006, Keystone, Colorado).
    Reston, Virginia: AIAA, 2006.
[F10] K. Alfriend, et al. “Probability of Collision Error Analysis." *Space Debris* 1, no. 1
    (1999): 21-35.
[F11] *IERS Conventions (2010)*. Edited by Gérard Petit and Brian Luzum. IERS Technical
    Note No. 32. Frankfurt am Main, Germany: Bundesamt für Kartographie und
    Geodäsie, 2010.