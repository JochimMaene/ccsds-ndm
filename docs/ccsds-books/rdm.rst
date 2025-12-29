===
RDM
===

.. title:: RE-ENTRY DATA MESSAGE

.. rubric:: RECOMMENDED STANDARD

.. centered:: CCSDS 508.1-B-1

.. centered:: BLUE BOOK
.. centered:: November 2019

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
   | Issue:   | Recommended Standard, Issue 1  |
   +----------+--------------------------------+
   | Date:    | November 2019                  |
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

This document is a CCSDS Recommended Standard for a Re-entry Data Message (RDM)
and has been prepared by the CCSDS. The RDM described in this document is intended for
re-entry (prediction) information exchange between interested parties.

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
-  Centre National d’Etudes Spatiales (CNES)/France.
-  China National Space Administration (CNSA)/People’s Republic of China.
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
   :widths: 25 25 15 35
   :header-rows: 1

   * - Document
     - Title
     - Date
     - Status
   * - CCSDS 508.1-B-1
     - Re-entry Data Message, Recommended Standard, Issue 1
     - November 2019
     - Original issue
   * - CCSDS 508.1-B-1 Cor. 1
     - Technical Corrigendum 1
     - October 2021
     - Adds specifications and examples for messages with elements qualified with respect to a namespace.
.. _introduction_rdm:

1 INTRODUCTION
==============

.. _purpose_and_scope_rdm:

1.1 PURPOSE AND SCOPE
---------------------

The Re-entry Data Message (RDM) Recommended Standard (henceforth ‘the RDM’ or ‘this
standard’) specifies a standard message format to be used in the exchange of spacecraft re-
entry information between Space Situational Awareness (SSA) or Space Surveillance and
Tracking (SST)¹ data providers, satellite owners/operators, and other parties. This message
can be used to inform spacecraft owners/operators of predicted re-entries or warn civil
protection agencies about potential ground impacts.

The RDM will:

a) facilitate interoperability and enable consistent warning between data providers who
   supply re-entry prediction data and the entities that use it;
b) facilitate the automation of re-entry prediction processes; and
c) provide critical information to enable timely re-entry decisions (e.g., a change in the
   controlled re-entry strategy).

The document also includes informative descriptions of re-entry prediction methods and
data.

.. _applicability_rdm:

1.2 APPLICABILITY
-----------------

The RDM is applicable to Space Surveillance and Tracking activities, spacecraft operations,
and other ‘ground’ based activities (e.g., civil protection, civil and military aviation) where
re-entering space objects are concerns. This Recommended Standard contains the
specifications for an RDM designed for re-entry prediction information exchange (though it
can be used post-facto as well) between originators of re-entry data and recipients. Re-entry
data includes remaining orbital lifetime, start and end of the re-entry and impact windows,
impact location, and object physical properties. Further information about the types of data
exchanged can be found in section 3.

The RDM is suitable for both manual and automated interaction, and for machine-to-machine
interfaces because of the large amount of data present. The RDM is self-contained, but it can be
paired with other Navigation Data Messages to enhance its functionality. For example, an
RDM could be paired with several Orbit Parameter Messages (OPM─reference [1], section 3)
to exchange state vectors at critical epochs (last orbit determination, current, re-entry, etc.) or
with one Orbit Ephemeris Message (OEM─reference [1], section 5) to give the trajectory for
(most of) the re-entry. The presence of user defined keywords allows other information to be
exchanged after being specified in an Interface Control Document (ICD); ICDs are not
necessary for most RDM exchanges, but are expected in some cases (especially if ODMs are

.. note::
   ¹ The term SSA is more commonly used in the US, while SST is preferred in Europe. This document uses SST
   hereafter.

to be exchanged for object position). The CCSDS Navigation Working Group should be
informed of new optional keywords for possible inclusion in future revisions of the standard.

It is desirable that RDM originators maintain consistency with respect to the optional
keywords provided in their implementations; i.e., it is desirable that the composition of the
RDMs provided not change on a frequent basis.

This standard is applicable only to the message format and content, not to its transmission or
to the algorithms used to produce the data within. The method of transmitting the message
between exchange partners is beyond the scope of this document and could be specified in an
ICD. The methods used to predict re-entries and calculate the associated data (e.g.,
probability of ground impact and impact location) are also outside the scope of this standard.

.. _document_structure_rdm:

1.3 DOCUMENT STRUCTURE
----------------------

Section 2 provides a short overview of the CCSDS RDM.

Section 3 describes the structure and content of the ‘keyword = value’ (KVN) version of the
RDM.

Section 4 describes the structure and content of the XML version of the RDM.

Section 5 describes the data and syntax of RDM messages, both the KVN and XML versions.

Annex A shows the Implementation Conformance Statement (ICS), listing all the
requirements an RDM implementation should meet.

Annex B discusses security, SANA, and patent issues.

Annex C provides RDM examples.

Annex D lists the abbreviations and acronyms used in this document.

Annex E lists the requirements this standard was designed to meet.

Annex F presents a summary sheet that shows which optional metadata entries should be
present for the data blocks in the message.

Annex G describes the types of information exchanged in an RDM.

Annex H provides a list of informative references.

.. _nomenclature_rdm:

1.4 NOMENCLATURE
----------------

.. _normative_text_rdm:

1.4.1 NORMATIVE TEXT
^^^^^^^^^^^^^^^^^^^^

The following conventions apply for the normative specifications in this Recommended
Standard:

a) the words ‘shall’ and ‘must’ imply a binding and verifiable specification;
b) the word ‘should’ implies an optional, but desirable, specification;
c) the word ‘may’ implies an optional specification;
d) the words ‘is’, ‘are’, and ‘will’ imply statements of fact.

.. note::
   These conventions do not imply constraints on diction in text that is clearly
   informative in nature.

.. _informative_text_rdm:

1.4.2 INFORMATIVE TEXT
^^^^^^^^^^^^^^^^^^^^^^

In the normative sections of this document, informative text is set off from the normative
specifications either in notes or under one of the following subsection headings:

- Overview;
- Background;
- Rationale;
- Discussion.

.. _conventions_and_definitions_rdm:

1.5 CONVENTIONS AND DEFINITIONS
-------------------------------

The following conventions for unit notations apply throughout this standard. To the extent
possible, an effort has been made to use units that are part of the International System of
Units (SI—defined in reference [2]); units are either SI base units, SI derived units, or units
outside the SI that are accepted for use with the SI. The units used within this document are
as follows:

- km: kilometers;
- m: meters;
- d: days (86400 s);
- s: seconds of time;
- kg: kilograms;
- deg: degrees of arc;
- %: percent.

For compound units the following conventions are used:

- multiplication is denoted with a single asterisk, ‘*’ (e.g., ‘kg*s’);
- division is denoted with a single forward slash, ‘/’ (e.g., ‘km/s’);
- exponents are denoted with a double asterisk, ‘**’ (e.g., ‘km**2’).

The usual mathematical conventions for operation order (e.g., exponents before
multiplication) apply.

The following conventions apply when referring to Earth re-entry prediction simulations:

a) short-term re-entry prediction refers to simulations covering the last few days (up to
   a few weeks) before re-entry (for a circular orbit altitude below 200 km);
b) medium-term re-entry prediction refers to simulations covering the last few weeks up
   to 11 years (one solar cycle) before re-entry; for the purposes of this standard it is
   grouped with long-term prediction;
c) long-term re-entry prediction refers to all re-entry simulations covering more than
   one solar cycle (over 11 years); grouped with medium-term prediction for this
   standard.

The term ‘controlled re-entry’ is used in this document for a re-entry event where:

a) the re-entry epoch is controlled; and
b) the impact of fragments on the central body’s surface is confined to a predetermined
   area.

The term ‘ASCII’ is generally used in this document to refer to the text character set defined
in reference [3]. The term ‘n/a’ is used to mean ‘not applicable’ or ‘not available’.

.. _references_rdm:

1.6 REFERENCES
--------------

The following publications contain provisions which, through reference in this text,
constitute provisions of this document. At the time of publication, the editions indicated were
valid. All publications are subject to revision, and users of this standard are encouraged to
investigate the possibility of applying the most recent editions of the publications indicated
below. The CCSDS Secretariat maintains a register of currently valid CCSDS publications.

[1] *Orbit Data Messages*. Issue 2. Recommendation for Space Data System Standards
    (Blue Book), CCSDS 502.0-B-2. Washington, D.C.: CCSDS, November 2009.
[2] *The International System of Units (SI)*. 8th ed., 2006; updated in 2014. Sèvres, France:
    BIPM, 2006.
[3] *Information Technology—8-Bit Single-Byte Coded Graphic Character Sets—Part 1:
    Latin Alphabet No. 1*. International Standard, ISO/IEC 8859-1:1998. Geneva: ISO,
    1998.
[4] Henry S. Thompson, et al., eds. *XML Schema Part 1: Structures*. 2nd ed. W3C
    Recommendation. N.p.: W3C, October 2004.
[5] Paul V. Biron and Ashok Malhotra, eds. *XML Schema Part 2: Datatypes*. 2nd ed. W3C
    Recommendation. N.p.: W3C, October 2004.
[6] “Organizations.” Space Assigned Numbers Authority (SANA).
    https://sanaregistry.org/r/organizations/.
[7] “United Nations Register of Objects Launched into Outer Space.” UNOOSA.
    http://www.unoosa.org/oosa/en/spaceobjectregister/.
[8] “Conjunction Data Message CATALOG_NAME.” Space Assigned Numbers Authority
    (SANA). https://sanaregistry.org/r/cdm_catalog.
[9] “Orbit Centers.” Space Assigned Numbers Authority.
    https://sanaregistry.org/r/orbit_centers.
[10] “Time Systems.” Space Assigned Numbers Authority.
     https://sanaregistry.org/r/time_systems.
[11] “Celestial Body Reference Frames.” Space Assigned Numbers Authority.
     https://sanaregistry.org/r/celestial_body_reference_frames.
[12] “Orbit-Relative Reference Frames.” Space Assigned Numbers Authority.
     https://sanaregistry.org/r/orbit_relative_reference_frames.
[13] *XML Specification for Navigation Data Messages*. Issue 2. Recommendation for Space
     Data System Standards (Blue Book), CCSDS 505.0-B-2. Washington, D.C.: CCSDS,
     May 2021.
[14] *Time Code Formats*. Issue 4. Recommendation for Space Data System Standards (Blue
     Book), CCSDS 301.0-B-4. Washington, D.C.: CCSDS, November 2010.

.. _overview_rdm:

2 OVERVIEW
==========

.. _overview_overview_rdm:

2.1 OVERVIEW
------------

This section provides a high-level overview of the Re-entry Data Message, a message format
designed to help the exchange of re-entry data between originators of re-entry data,
spacecraft operators, and other interested entities. The originators of re-entry data can be SST
entities or even the spacecraft operators themselves (e.g., for a controlled re-entry).

.. _rdm_content_rdm:

2.2 RDM CONTENT
----------------

The RDM is an ASCII format, encoded either as plain text (usually referred to as KVN) or
XML (references [3], [4], and [5]). This standard describes both.

The RDM contains information about a single re-entry event:

- information about the message itself (creation date, originator, etc.);
- identification of the re-entering object (name, ID);
- basic re-entry information (mandatory): remaining orbital lifetime, whether the re-
  entry is controlled or not, and which celestial body the object is orbiting;
- more complex re-entry information (optional): re-entry and impact windows, impact
  location and probabilities, state vector, object properties, the orbit determination
  (OD) process, and observations used to predict the re-entry.

The information is used by satellite operators, civil protection, or aviation authorities to
assess the re-entry risk and plan any needed mitigation measures.

The RDM is not limited to man-made objects re-entering the Earth’s atmosphere. It could be
used for any entry/impact event by specifying the appropriate CENTER_NAME,
REF_FRAME, and OBJECT_TYPE values (e.g., a space probe landing on Venus).
.. _structure_and_content_kvn_rdm:

3 RE-ENTRY DATA MESSAGE STRUCTURE AND CONTENT (KVN)
===================================================

.. _overview_kvn_rdm:

3.1 OVERVIEW
------------

This section contains a description of the structure, content, and keywords allowed in a Re-
entry Data Message.

.. _general_kvn_rdm:

3.2 GENERAL
-----------

3.2.1 The RDM shall be plain text consisting of re-entry (prediction) data for a single re-
entry event.

3.2.2 The RDM in Keyword Value Notation (KVN) shall consist of digital data represented
as ASCII text lines. The RDM shall contain the following:

a) a header;
b) a metadata section;
c) a data section.

.. note::

   1. KVN messages contain one keyword per line (specified in 5.3.2.4).
   2. The standard order of keywords in the KVN representation is fixed by this standard,
      as listed in tables 3-1, 3-2, and 3-3 (specified in 5.3.2.10).

3.2.3 The RDM file naming scheme should be agreed to on a case-by-case basis between
the participating agencies, typically specified in an ICD. In general, the file name syntax and
length must not violate computer constraints for those computing environments in use by
Member Agencies for processing re-entry data.

3.2.4 The exchange method for RDMs should be determined on a case-by-case basis and
should be documented in an ICD.

.. _header_kvn_rdm:

3.3 RDM HEADER
--------------

The RDM header shall only consist of the KVN elements defined in table 3-1, which
specifies for each header item:

a) the keyword to be used;
b) a short description of the item;
c) examples of allowed values;
d) whether the item is mandatory (M) or optional (O).

.. _table_3-1_rdm:

.. list-table:: Table 3-1: RDM KVN Header
   :widths: 20 50 20 10
   :header-rows: 1

   * - Keyword
     - Description of values
     - Examples of values
     - M/O
   * - CCSDS_RDM_VERS
     - Format version in the form of ‘x.y’, where ‘y’
       is incremented for corrections and minor
       changes, and ‘x’ is incremented for major
       changes.
     - 1.0
     - M
   * - COMMENT
     - Comments (allowed in the RDM Header only
       immediately after the RDM version number).
     - COMMENT This is a
       comment
     - O
   * - CREATION_DATE
     - File creation date and time in UTC. The
       format is defined in 5.3.3.5.
     - 2001-11-06T11:17:33
       2002-204T15:56:23
     - M
   * - ORIGINATOR
     - Creating agency or entity; the value should
       be taken from the abbreviation column in the
       SANA organizations registry (reference [6]).
       The country of origin should also be provided
       where the originator is not a national space
       agency.
     - DLR
       ESA
     - M
   * - MESSAGE_ID
     - ID that uniquely identifies a message from a
       given originator. The format and content of
       the message identifier value are at the
       discretion of the originator.
     - 201113719185
       ESA20190101-3345
     - M

.. _metadata_kvn_rdm:

3.4 RDM METADATA
----------------

3.4.1 The RDM metadata shall only consist of the KVN elements defined in table 3-2,
which specifies for each metadata item:

a) the keyword;
b) a short description of the item;
c) normative values or examples of values (some examples wrap over multiple lines due
   to limited space, e.g., time strings; they would not wrap in an actual RDM);
d) whether the values are normative (N—all values allowed are in the list in the third
   column) or examples of values allowed (E);
e) whether the item is mandatory (M) or optional (O).

3.4.2 Mandatory items shall appear in every RDM metadata section. Items that are not
mandatory may or may not appear, at the discretion of the data producer, based on the
requirements of the data and its intended application (an RDM Summary Sheet that
illustrates the relationships between data types and metadata can be found in annex F).

.. include:: rdm_parts/table_3-2.rst

.. _data_kvn_rdm:

3.5 RDM DATA
------------

3.5.1 The RDM data section shall be formatted as logical blocks:

a) atmospheric re-entry;
b) ground impact data;
c) state vector (including epoch);
d) position/velocity covariance matrix (associated with the state vector and at the same
   epoch);
e) object physical parameters;
f) orbit determination parameters;
g) user defined parameters.

3.5.2 The logical blocks of the RDM data section shall consist of KVN lines, as defined in
table 3-3, which specifies:

a) the keyword to be used;
b) a description of the content;
c) the units to be used;
d) whether the item is mandatory (M) or optional (O).

.. include:: rdm_parts/table_3-3.rst

3.5.3 Table 3-3 contains seven logical blocks, each of which has a descriptive heading.
These descriptive headings shall not be included in an RDM, unless they appear in a properly
formatted COMMENT statement (KVN comments specified in 5.3.5, XML in 5.4.4).

3.5.4 COMMENT lines may be used at the beginning of each logical block of the data
section (comment placing is further specified in 5.2.5.2).

3.5.5 The ORBIT_LIFETIME keyword is mandatory and its value shall be used to convey
both:

a) whether it is short-term or medium and long-term re-entry prediction (defined in 1.5);
b) the remaining orbital lifetime.

3.5.6 For short-term re-entry predictions the NOMINAL_REENTRY_EPOCH,
REENTRY_WINDOW_START, and REENTRY_WINDOW_END keywords should be
used.

3.5.7 If the NOMINAL_REENTRY_EPOCH, REENTRY_WINDOW_START, and
REENTRY_WINDOW_END keywords are present, their values should be used in
computations, rather than those of the ORBIT_LIFETIME,
ORBIT_LIFETIME_WINDOW_START, and ORBIT_LIFETIME_WINDOW_END
keywords.

3.5.8 If the NOMINAL_REENTRY_EPOCH keyword is present, the ORBIT_LIFETIME
and NOMINAL_REENTRY_EPOCH keywords should resolve to the same value.

3.5.9 If both the ORBIT_LIFETIME_WINDOW_START and
REENTRY_WINDOW_START keywords are present, they should both resolve to the same
value. The same applies for ORBIT_LIFETIME_WINDOW_END and
REENTRY_WINDOW_END.

3.5.10 If a ground impact location is given, the IMPACT_REF_FRAME keyword shall be
mandatory and at least NOMINAL_IMPACT_LON and NOMINAL_IMPACT_LAT shall
be present. NOMINAL_IMPACT_ALT may be given as well.

3.5.11 Values for all longitude keywords shall be between -180.0 and 180.0, with positive
values for eastward longitudes and negative values for westward longitudes.

3.5.12 Values for all latitude keywords shall be between -90.0 and 90.0, with positive values
for northern latitudes and negative values for southern latitudes.

3.5.13 If one confidence ‘n’ interval is present then all the values associated with interval ‘n’
shall be present: IMPACT_n_CONFIDENCE, IMPACT_n_START_LON,
IMPACT_n_START_LAT, IMPACT_n_STOP_LON, IMPACT_n_STOP_LAT, and
IMPACT_n_CROSS_TRACK.

3.5.14 If only one confidence interval is present then the IMPACT_1_* keywords shall be
used.

3.5.15 If two confidence intervals are present then the IMPACT_1_* and IMPACT_2_*
keywords shall be used.

3.5.16 If more than one confidence interval is present, then IMPACT_1_CONFIDENCE
shall be smaller than IMPACT_2_CONFIDENCE, which in turn shall be smaller than
IMPACT_3_CONFIDENCE, if it is present.

3.5.17 If the REENTRY_DISINTEGRATION keyword in the metadata indicates that break-
up was simulated, then the ground impact location keywords shall refer to all potential
fragments related to the event; i.e., the NOMINAL_IMPACT_LAT and _LON will
correspond to the highest probability that any fragment will impact there, the confidence
intervals will apply to all fragments, etc.

3.5.18 The probability of impact should be within five percent at the following four points
for each confidence interval given: the start and end of the confidence interval in the along-
track direction, and the nominal impact location ± the cross-track confidence interval.

3.5.19 The state vector and covariance data are at the epoch specified by the EPOCH
keyword in the data section. There are no restrictions on which epoch this is supposed to be
(orbit determination epoch, message creation epoch, re-entry epoch, etc.). If the covariance
block is present then the state vector shall be present as well.

3.5.20 If the state vector block is present then all elements in the block shall be present, with
the exception of the comment line. No partial state vectors shall be present in an RDM. State
vector values shall be expressed in standard double precision as related in 5.2.3.2.

3.5.21 Values in the covariance matrix shall be expressed in the applicable reference frame
(COV_REF_FRAME keyword if used, or REF_FRAME keyword if not), and shall be
presented sequentially from upper left [1,1] to lower right [6,6], lower triangular form, row
by row, left to right. If the covariance block is present in the message, all covariance matrix
elements shall be present. Variance and covariance values shall be expressed in standard
double precision as related in 5.2.3.2.

3.5.22 Since re-entry prediction services are still in their infancy and some RDM originators
might need to provide information that is not foreseen by this standard, a section of User
Defined Parameters may be included at the end of the data section. In principle, User Defined
Parameters provide flexibility, but also introduce complexity, non-standardization, potential
ambiguity, and potential processing errors. Accordingly, if used, the keywords and their
meanings must be described in an ICD. User Defined Parameters, if included in an RDM,
should be used as sparingly as possible; their use is not encouraged.
.. _structure_and_content_xml_rdm:

4 RE-ENTRY DATA MESSAGE STRUCTURE & CONTENT (XML)
=================================================

.. _overview_xml_rdm:

4.1 OVERVIEW—THE RDM/XML SCHEMA
-------------------------------

This section applies only to the XML version.

The RDM/XML schema is available on the SANA Web site. SANA is the registrar for the
protocol registries created under CCSDS.

The RDM XML schema explicitly defines the permitted data elements and values acceptable
for the XML version of the RDM message. The location of the RDM/XML schema is:

- https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-2.0.0-rdm-1.0.xsd for
  messages with elements not qualified with respect to a namespace;
- https://sanaregistry.org/r/ndmxml_qualified/ndmxml-2.0.0-rdm-1.0.xsd for messages
  with elements qualified with respect to a namespace.

.. note::
   Reference [13] subsection 4.3 has more information regarding messages with
   elements qualified with respect to a namespace.

Where possible this schema uses simple types and complex types used by the constituent
schemas that make up Navigation Data Messages (specified in reference [13]).

.. _basic_structure_xml_rdm:

4.2 RDM/XML BASIC STRUCTURE
---------------------------

4.2.1 Each RDM shall consist of a <header> and a <body>.

4.2.2 The RDM <body> shall consist of a single <segment> construct.

4.2.3 The RDM <segment> shall consist of a <metadata>/<data> pair, as shown in
figure 4-1.

.. figure:: /images/rdm_xml_basic_structure.png
   :align: center

   Figure 4-1: RDM XML Basic Structure

.. _tags_xml_rdm:

4.3 RDM/XML TAGS
----------------

4.3.1 An RDM XML tag shall be all uppercase if it corresponds directly to a KVN keyword
from the header, metadata, or data sections.

4.3.2 There is an exception where there is not a strict correspondence between keywords in
the KVN and tags in the XML implementations, specifically, the ‘CCSDS_RDM_VERS’
keyword from the RDM header. The ‘CCSDS_RDM_VERS’ keyword and its value shall
appear as XML attributes rather than as an XML element.

4.3.3 RDM XML tags related to the XML message structure (i.e., that do not correspond
directly to a KVN keyword) shall be in ‘lowerCamelCase’ (e.g., <header>, <segment>,
<metadata>).

.. _constructing_instance_xml_rdm:

4.4 CONSTRUCTING AN RDM/XML INSTANCE
-------------------------------------

.. _overview_constructing_instance_xml_rdm:

4.4.1 OVERVIEW
^^^^^^^^^^^^^^

This subsection provides more detailed instructions for the user on how to create an XML
message based on the ASCII-text KVN-formatted message described in 3.3 through 3.5.

.. _version_xml_rdm:

4.4.2 XML VERSION
^^^^^^^^^^^^^^^^^

4.4.2.1 The first line in the instantiation shall specify the XML version:
``<?xml version="1.0" encoding="UTF-8"?>``

4.4.2.2 This line must appear on the first line of each instantiation, exactly as shown.

.. _beginning_instantiation_xml_rdm:

4.4.3 BEGINNING THE INSTANTIATION: ROOT DATA ELEMENT
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

4.4.3.1 An RDM instantiation shall be delimited with the <rdm></rdm> root element tags
using the standard attributes documented in reference [13], subsection 4.3.

4.4.3.2 The XML Schema Instance namespace attribute must appear in the root element tag
of all RDM/XML instantiations, exactly as shown:
``xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"``

4.4.3.3 For messages with elements qualified with respect to a namespace, the NDM/XML
namespace must next be coded, exactly as shown:
``xmlns:ndm="urn:ccsds:schema:ndmxml"``

The value that follows the ‘xmlns:’ in the NDM/XML name space (‘ndm’ in this case) is a
prefix that must be used on every XML tag.

.. note::
   This xmlns:ndm setting is only necessary for messages with elements qualified
   with respect to a namespace, but it does not hurt anything for it to appear on any
   NDM/XML instantiation.

4.4.3.4 If it is desired to validate an instantiation against the CCSDS Web-based schema,
the xsi:noNamespaceSchemaLocation attribute must be coded as a single string of non-blank
characters, with no line breaks, exactly as shown:

- xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-2.0.0-master-2.0.xsd" for messages with elements not qualified with respect to a namespace.
- xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_qualified/ndmxml-2.0.0-master-2.0.xsd" for messages with elements qualified with respect to a namespace.

.. note::
   The length of the value associated with the xsi:noNamespaceSchemaLocation
   attribute can cause the string to wrap to a new line; however, the string itself
   contains no breaks.

4.4.3.5 The final attributes of the <rdm> tag shall be ‘id’ and ‘version’.

4.4.3.6 The ‘id’ attribute shall be ‘id="CCSDS_RDM_VERS"’. The ‘version’ attribute
shall be ‘version="1.0"’.

.. admonition:: NOTES

   1. The following example root element tag for an RDM instantiation combines all the
      directions in the preceding several subsections for messages with elements not
      qualified with respect to a namespace:

      .. code-block:: xml

         <?xml version="1.0" encoding="UTF-8"?>
         <rdm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:noNamespaceSchemaLocation=
         "https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-2.0.0-master-2.0.xsd"
         id="CCSDS_RDM_VERS" version="1.0">

   2. The following example root element tag for an RDM instantiation combines all the
      directions in the preceding several subsections for messages with elements qualified
      with respect to a namespace:

      .. code-block:: xml

         <?xml version="1.0" encoding="UTF-8"?>
         <rdm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xmlns:ndm="urn:ccsds:schema:ndmxml"
         xsi:noNamespaceSchemaLocation=
         "https://sanaregistry.org/r/ndmxml_qualified/ndmxml-2.0.0-master-2.0.xsd"
         id="CCSDS_RDM_VERS" version="1.0">

.. _header_section_xml_rdm:

4.4.4 THE RDM/XML HEADER SECTION
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

4.4.4.1 The RDM header shall have a standard header format, with tags <header> and
</header>.

4.4.4.2 Immediately following the <header> tag the message may have any number of
<COMMENT></COMMENT> tag pairs.

4.4.4.3 The standard RDM header shall contain the following element tags:

a) <CREATION_DATE>;
b) <ORIGINATOR>;
c) <MESSAGE_ID>.

.. note::
   The rules for these keywords are specified in table 3-1. The header would look
   like this:

   .. code-block:: xml

      <header>
      <COMMENT>Some comment string.</COMMENT>
      <CREATION_DATE>2010-03-12T22:31:12.000</CREATION_DATE>
      <ORIGINATOR>AGENCYX</ORIGINATOR>
      <MESSAGE_ID>AGENCYX-1234</MESSAGE_ID>
      </header>

.. _body_section_xml_rdm:

4.4.5 THE RDM/XML BODY SECTION
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

4.4.5.1 After coding the <header>, the instantiation must include a <body></body> tag
pair.

4.4.5.2 Inside the <body></body> tag pair must appear one <segment></segment> tag
pair.

.. note::
   In essence, the segment tag in the RDM XML implementation is not strictly
   necessary; however, it is necessary for structural symmetry with the overall
   NDM/XML paradigm (specified in reference [13], subsection 3.2).

4.4.5.3 The <segment> must be made up of one <metadata></metadata> tag pair and one
<data></data> tag pair.

.. _metadata_section_xml_rdm:

4.4.6 THE RDM/XML METADATA SECTION
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

4.4.6.1 The metadata section shall be set off by the <metadata></metadata> tag
combination.

4.4.6.2 Between the <metadata> and </metadata> tags, the keywords shall be those
specified in table 3-2.

4.4.6.3 Immediately following the <metadata> tag, the message may have any number of
<COMMENT></COMMENT> tag pairs.

.. _data_section_xml_rdm:

4.4.7 THE RDM DATA SECTION
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

4.4.7.1 The data section shall follow the metadata section and shall be set off by the
<data></data> tag combination.

4.4.7.2 Between the <data> and </data> tags, the keywords shall be those specified in
table 3-3 and the tags specified in table 4-1 (as specified in 4.4.8).

4.4.7.3 Immediately following the <data> tag, the message may have any number of
<COMMENT></COMMENT> tag pairs.

4.4.7.4 Immediately following the special tags listed in table 4-1, the message may have
any number of <COMMENT></COMMENT> tag pairs.

.. _special_tags_xml_rdm:

4.4.8 SPECIAL RDM/XML TAGS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Special tags that are not necessary in the KVN implementation shall be used to encapsulate
the information in the XML implementation of the RDM. The special tags indicating data
section logical block divisions shall be those defined in table 4-1.

.. list-table:: Table 4-1: Special RDM/XML Tags
   :widths: 50 50
   :header-rows: 1

   * - Special Tag
     - Definition
   * - <atmosphericReentryParameters>
     - Delineates the logical block for atmospheric re-entry parameters.
   * - <groundImpactParameters>
     - Delineates the logical block for ground impact and burn-up data parameters, if they are used.
   * - <stateVector>
     - Delineates the state vector components in the coordinate system specified in the metadata, if it is used.
   * - <covarianceMatrix>
     - Delineates the 6x6 position/velocity covariance matrix, if it is used.
   * - <spacecraftParameters>
     - Delineates the logical block containing object physical parameters, if they are used.
   * - <odParameters>
     - Delineates the logical block containing orbit determination parameters, if they are used.
   * - <userDefinedParameters>
     - Delineates the logical block containing user defined parameters, if they are used. NOTE: The use of <userDefinedParameters> is defined in reference [13], subsection 4.16.

.. _units_xml_rdm:

4.4.9 UNITS IN THE RDM/XML
^^^^^^^^^^^^^^^^^^^^^^^^^^

The units in the RDM/XML shall be the same units used in the KVN-formatted RDM
described in 3.4 and 3.5. XML attributes shall be used to explicitly define the units or other
important information associated with the given data element (examples in annex C).

.. _local_operations_rdm:

4.5 LOCAL OPERATIONS
--------------------

4.5.1 For use in a local operations environment, the NDM/XML schema set (which
includes the RDM schema) may be downloaded from the SANA Web site to a local server
that meets local requirements for operations robustness.

4.5.2 If a local version is used, the value associated with the
xsi:noNamespaceSchemaLocation attribute must be changed to a URL that is accessible to
the local server.

.. _data_and_syntax_rdm:

5 RE-ENTRY DATA MESSAGE DATA AND SYNTAX
=======================================

.. _overview_data_and_syntax_rdm:

5.1 OVERVIEW
------------

This section details the syntax requirements for the RDM using both the KVN and XML
formats.

.. _common_syntax_rdm:

5.2 COMMON RDM SYNTAX
---------------------

.. _overview_common_syntax_rdm:

5.2.1 OVERVIEW
^^^^^^^^^^^^^^

This subsection details the syntax requirements that are common to both KVN and XML
formats.

.. _lines_rdm:

5.2.2 RDM LINES
^^^^^^^^^^^^^^^

5.2.2.1 Each RDM line must not exceed 254 ASCII characters and spaces (excluding line
termination character[s]).

5.2.2.2 Only printable ASCII characters and spaces shall be used. Control characters (such
as TAB, etc.) shall not be used, with the exception of the line termination characters
specified below.

5.2.2.3 Blank lines may be used at any position within the file. Blank lines shall have no
assignable meaning, and may be ignored.

5.2.2.4 All lines shall be terminated by a single Carriage Return or a single Line Feed, or a
Carriage Return/Line Feed pair, or a Line Feed/Carriage Return pair.

.. _values_rdm:

5.2.3 RDM VALUES
^^^^^^^^^^^^^^^^

5.2.3.1 A nonempty, valid value must be specified for each mandatory keyword.

5.2.3.2 Non-integer numeric values may be expressed in either fixed-point or floating-point
notation.

5.2.3.3 Text value fields must be constructed using only all uppercase (‘_’, ‘-’, space, and
digits are permitted as well). An exception is made for comment values (comments are
specified in 5.2.5).

5.2.3.4 Blanks shall not be permitted within numeric values and time strings.

.. _units_rdm:

5.2.4 RDM UNITS
^^^^^^^^^^^^^^^

5.2.4.1 If units are applicable, as specified in the tables in section 3, they must be displayed
and they must exactly match the units specified in each table (including case; units
conventions and operations are described in 1.5).

5.2.4.2 The notation ‘[n/a]’ shall not appear in an RDM as a units designator.

.. note::
   Some of the items in table 3-3 are dimensionless (e.g., DRAG_COEFF). For such
   items, the table shows a unit value of ‘n/a’, which in this case means that there is
   no applicable units designator for those items.

.. _comments_rdm:

5.2.5 RDM COMMENTS
^^^^^^^^^^^^^^^^^^

5.2.5.1 Comment lines shall be optional in the RDM.

5.2.5.2 Comments shall be placed as specified in the tables in section 3 describing the
keywords. In the places where comments are allowed any number of comments may appear.

5.2.5.3 Comment text may be in any case desired by the user.

.. _rdm_in_kvn:

5.3 THE RDM IN KVN
------------------

.. _overview_rdm_in_kvn:

5.3.1 OVERVIEW
^^^^^^^^^^^^^^

KVN instantiations of an RDM shall observe the syntax described in 5.3.

.. _lines_in_kvn_rdm:

5.3.2 RDM LINES IN KVN
^^^^^^^^^^^^^^^^^^^^^^

5.3.2.1 Each RDM file shall consist of a set of RDM lines. Each RDM line shall be one of
the following:

- Header line;
- Metadata line;
- Data line; or
- Blank line.

5.3.2.2 The first header line must be the first non-blank line in the file.

5.3.2.3 All header, metadata, and data lines shall use ‘keyword = value’ notation. For this
purpose, only those keywords shown in tables 3-1, 3-2, and 3-3, shall be used in an RDM.

5.3.2.4 Only a single ‘keyword = value’ assignment shall be made on a line.

5.3.2.5 Keywords must be uppercase and must not contain blanks.

5.3.2.6 Any white space immediately preceding or following the keyword shall not be
significant.

5.3.2.7 Any white space immediately preceding or following the ‘equals’ sign shall not be
significant.

5.3.2.8 Any white space immediately preceding or following the units shall not be
significant.

5.3.2.9 Any white space immediately preceding the end of line shall not be significant.

5.3.2.10 The order in which mandatory and optional KVN assignments appear shall be fixed
as shown in the tables that describe the RDM keywords in section 3.

.. _values_in_kvn_rdm:

5.3.3 RDM VALUES IN KVN
^^^^^^^^^^^^^^^^^^^^^^^^^

5.3.3.1 Integer values shall consist of a sequence of decimal digits with an optional leading
sign (‘+’ or ‘-’). If the sign is omitted, ‘+’ shall be assumed. Leading zeroes may be used.
The range of values that may be expressed as an integer is:
-2 147 483 648 ≤ x ≤ + 2 147 483 647 (i.e., -2³¹ ≤ x ≤ 2³¹ - 1)

5.3.3.2 Non-integer numeric values expressed in fixed-point notation shall consist of a
sequence of decimal digits separated by a period (‘.’) as a decimal point indicator, with an
optional leading sign (‘+’ or ‘-’). If the sign is omitted, ‘+’ shall be assumed. Leading and
trailing zeroes may be used. At least one digit shall appear before and after a decimal point.
The number of digits shall be 16 or fewer.

5.3.3.3 Non-integer numeric values expressed in floating point notation shall consist of a
sign, a mantissa, an alphabetic character indicating the division between the mantissa and
exponent, and an exponent, constructed according to the following rules:

a) The sign may be ‘+’ or ‘-’. If the sign is omitted, ‘+’ shall be assumed.
b) The mantissa must be a string of no more than 16 decimal digits with a decimal point
   (‘.’) in the second position of the ASCII string, separating the integer portion of the
   mantissa from the fractional part of the mantissa.
c) The character used to denote exponentiation shall be ‘E’ or ‘e’. If the character
   indicating the exponent and the following exponent are omitted, an exponent value of
   zero shall be assumed (yielding a fixed point value).
d) The exponent must be an integer, and may have either a ‘+’ or ‘-’ sign; if the sign is
   omitted, then ‘+’ shall be assumed.
e) The maximum positive floating point value is approximately 1.798·10³⁰⁸, with 16
   significant decimal digits precision. The minimum positive floating point value is
   approximately 4.94·10⁻³²⁴, with 16 significant decimal digits precision.

5.3.3.4 In value fields that are text, an underscore shall be equivalent to a single blank.
Individual blanks shall be retained (shall be significant), but multiple contiguous blanks shall
be equivalent to a single blank. This requirement does not apply to leading or trailing blanks,
as per 5.3.2.7, 5.3.2.8, and 5.3.2.9.

5.3.3.5 In value fields representing a time tag or epoch, one of the following two formats
shall be used:

``yyyy-mm-ddThh:mm:ss[.d→d][Z]``

or

``yyyy-dddThh:mm:ss[.d→d][Z]``

where ‘yyyy’ is the year, ‘mm’ is the two-digit month, ‘dd’ is the two-digit day of the month,
and ‘ddd’ is the three-digit day of the year, separated by hyphens; ‘T’ is a fixed separator
between the date and time portions of the string; and ‘hh:mm:ss[.d→d]’ is the time in hours,
minutes, seconds, and fractional seconds, separated by colons. As many ‘d’ characters to the
right of the period as required may be used to obtain the required precision, up to the
maximum allowed for a fixed-point number. All fields require leading zeros. The format is
specified in reference [14], subsection 3.5.

.. _units_in_kvn_rdm:

5.3.4 RDM UNITS IN KVN
^^^^^^^^^^^^^^^^^^^^^^

When units are displayed:

a) there must be at least one blank character between the value and the units;
b) units shall be in the correct case (e.g., lowercase for km, uppercase for K or W;
   uppercase units are not used in the RDM), as indicated in table 3-3; and
c) the units must be enclosed within square brackets (e.g., ‘[kg]’).

.. _comments_in_kvn_rdm:

5.3.5 RDM COMMENTS IN KVN
^^^^^^^^^^^^^^^^^^^^^^^^^^

All comment lines shall begin with the ‘COMMENT’ keyword followed by at least one
space. This keyword must appear on every comment line, not just the first such line. The
remainder of the line shall be the comment value. White space shall be retained (shall be
significant) in comment values.

.. _rdm_in_xml:

5.4 THE RDM IN XML
------------------

.. _overview_rdm_in_xml:

5.4.1 OVERVIEW
^^^^^^^^^^^^^^

XML instantiations of an RDM shall observe the syntax described in this subsection.

.. _lines_in_xml_rdm:

5.4.2 RDM LINES IN XML
^^^^^^^^^^^^^^^^^^^^^^

Each RDM file shall consist of a set of RDM lines. Each RDM line shall be one of the
following:

- XML version line;
- an XML-formatted line; or
- a blank line.

.. _values_in_xml_rdm:

5.4.3 RDM VALUES IN XML
^^^^^^^^^^^^^^^^^^^^^^^^^

5.4.3.1 Each mandatory XML tag must be present and contain a valid value.

5.4.3.2 Integer values shall follow the conventions of the integer data type per
reference [5], section 3.3.13. Additional restrictions on the allowable range of values
permitted for any integer data element may also be defined in the RDM XML Schema.

.. note::
   Examples of such restrictions may include a defined range (e.g., 0–100, 1–10), a
   set of enumerated values (e.g., 0, 1, 2, 4, 8), a pre-defined specific variation such
   as positiveInteger, or a user-defined data type variation.

5.4.3.3 Non-integer numeric values may be expressed in either fixed-point or floating-point
notation. Numeric values shall follow the conventions of the double data type per
reference [5], section 3.2.5. Additional restrictions on the allowable range of values
permitted for any numeric data element may also be defined in the RDM XML Schema.

.. note::
   Examples of such restrictions may include a defined range (e.g., 0.0–100.0), or a
   user-defined data type variation.

5.4.3.4 Text value data shall follow the conventions of the string data type per
reference [5], section 3.2.1. Additional restrictions on the allowable range or values
permitted for any data element may also be defined in the RDM XML Schema.

.. note::
   Examples of such restrictions may include a set of enumerated values (e.g.,
   ‘YES’/’NO’) or other user-defined data type variation.

5.4.3.5 Text values in RDM/XML instantiations (i.e., the values between the opening and
closing tags), shall consist of either all uppercase or all lowercase characters; an exception is
made for values between the <COMMENT> and </COMMENT> tags, which may be in any
case desired by the user. Otherwise, mixing of uppercase and lowercase characters is
prohibited.

5.4.3.6 In value fields that represent a time tag, values shall follow the conventions of the
ndm:epochType data type used in all CCSDS NDM/XML schemas (as specified in 5.3.3.5).

.. _comments_in_xml_rdm:

5.4.4 RDM COMMENTS IN XML
^^^^^^^^^^^^^^^^^^^^^^^^^^^

Comments are optional and must be displayed as values between the <COMMENT> and
</COMMENT> tags. Comments may be in any case desired by the user.
.. _annex_a_rdm:

ANNEX A
=======

IMPLEMENTATION CONFORMANCE STATEMENT PROFORMA
===============================================

(NORMATIVE)

.. _introduction_ics_rdm:

A1 INTRODUCTION
---------------

.. _overview_ics_rdm:

A1.1 OVERVIEW
^^^^^^^^^^^^^

This annex provides the Implementation Conformance Statement (ICS) Requirements List
(RL) for an implementation of a Re-entry Data Message. The ICS for an implementation is
generated by completing the RL in accordance with the instructions below. An
implementation claiming conformance must satisfy the mandatory requirements referenced
in the RL.

.. _abbreviations_and_conventions_ics_rdm:

A1.2 ABBREVIATIONS AND CONVENTIONS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The RL consists of information in tabular form. The status of features is indicated using the
abbreviations and conventions described below.

Item Column
"""""""""""
The item column contains sequential numbers for items in the table.

Feature Column
""""""""""""""
The feature column contains a brief descriptive name for a feature. It implicitly means ‘Is
this feature supported by the implementation?’.

Status Column
"""""""""""""
The status column uses the following notations:

- M: mandatory;
- O: optional;
- C: conditional;
- X: prohibited;
- I: out of scope;
- N/A: not applicable.

Support Column Symbols
""""""""""""""""""""""

The support column is to be used by the implementer to state whether a feature is supported
by entering Y, N, or N/A, indicating:

- Y: Yes, supported by the implementation.
- N: No, not supported by the implementation.
- N/A: Not applicable.

The support column should also be used, when appropriate, to enter values supported for a
given capability.

.. _instructions_for_completing_the_rl_rdm:

A1.3 INSTRUCTIONS FOR COMPLETING THE RL
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An implementer shows the extent of compliance to the RDM standard by completing the RL;
that is, the state of compliance with all mandatory requirements and the options supported
are shown. The resulting completed RL is called an ICS. The implementer shall complete the
RL by entering appropriate responses in the support or values supported column, using the
notation described in A1.2. If a conditional requirement is inapplicable, N/A should be used.
If a mandatory requirement is not satisfied, exception information must be supplied by
entering a reference Xi, where i is a unique identifier, to an accompanying rationale for the
noncompliance.

.. _ics_proforma_rdm:

A2 ICS PROFORMA FOR RDM
-----------------------

.. _general_information_ics_rdm:

A2.1 GENERAL INFORMATION
^^^^^^^^^^^^^^^^^^^^^^^^

.. _identification_of_ics_rdm:

A2.1.1 Identification of ICS
"""""""""""""""""""""""""""""

.. list-table::
   :widths: 50 50

   * - Date of Statement (DD/MM/YYYY)
     -
   * - ICS serial number
     -
   * - System Conformance statement cross-reference
     -

.. _identification_of_iut_rdm:

A2.1.2 Identification of Implementation Under Test
"""""""""""""""""""""""""""""""""""""""""""""""""""

.. list-table::
   :widths: 50 50

   * - Implementation Name
     -
   * - Implementation Version
     -
   * - Special Configuration
     -
   * - Other Information
     -

.. _identification_of_supplier_rdm:

A2.1.3 Identification of Supplier
"""""""""""""""""""""""""""""""""

.. list-table::
   :widths: 50 50

   * - Supplier
     -
   * - Contact Point for Queries
     -
   * - Implementation Name(s) and Versions
     -
   * - Other information necessary for full identification, e.g., name(s) and version(s) for machines and/or operating systems; System Name(s)
     -

.. _identification_of_specification_rdm:

A2.1.4 Identification of Specification
""""""""""""""""""""""""""""""""""""""

.. list-table::
   :widths: 50 50

   * - CCSDS 508.1-B-1
     -
   * - Have any exceptions been required?
     - Yes [ ] No [ ]
   * - NOTE – A YES answer means that the implementation does not conform to the Recommended Standard. Non-supported mandatory capabilities are to be identified in the ICS, with an explanation of why the implementation is non-conforming.
     -

.. _requirements_list_rdm:

A2.2 REQUIREMENTS LIST
^^^^^^^^^^^^^^^^^^^^^^

.. include:: rdm_parts/ics_requirements_table.rst

.. _annex_b_rdm:

ANNEX B
=======

SECURITY, SANA, AND PATENT CONSIDERATIONS
=========================================

(INFORMATIVE)

.. _security_considerations_rdm:

B1 SECURITY CONSIDERATIONS
--------------------------

This subsection presents the results of an analysis of security considerations applied to the
technologies specified in this Recommended Standard.

.. _security_concerns_rdm:

B1.1 SECURITY CONCERNS WITH RESPECT TO THE CCSDS DOCUMENT
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

B1.1.1 Data Privacy
""""""""""""""""""""

Privacy of data formatted in compliance with the specifications of this Recommended
Standard should be assured by the systems and networks on which this Recommended
Standard is implemented.

B1.1.2 Data Integrity
""""""""""""""""""""""

Integrity of data formatted in compliance with the specifications of this Recommended
Standard should be assured by the systems and networks on which this Recommended
Standard is implemented.

B1.1.3 Authentication of Communicating Entities
"""""""""""""""""""""""""""""""""""""""""""""""

Authentication of communicating entities involved in the transport of data which complies
with the specifications of this Recommended Standard should be provided by the systems
and networks on which this Recommended Standard is implemented.

B1.1.4 Control of Access to Resources
""""""""""""""""""""""""""""""""""""""

Control of access to resources should be managed by the systems upon which originator
formatting and recipient processing are performed.

B1.1.5 Auditing of Resource Usage
""""""""""""""""""""""""""""""""""

Auditing of resource usage should be handled by the management of systems and networks
on which this Recommended Standard is implemented.

.. _potential_threats_rdm:

B1.2 POTENTIAL THREATS AND ATTACK SCENARIOS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Potential threats or attack scenarios include, but are not limited to, (a) unauthorized access to
the programs/processes that generate and interpret the messages, (b) unauthorized access to
the messages during transmission between exchange partners, and (c) modification of the
messages between partners. Protection from unauthorized access during transmission is
especially important if the RDM provider and users utilize open ground networks, such as
the Internet, to exchange data formatted in compliance with this Recommended Standard. It
is strongly recommended that potential threats or attack scenarios applicable to the systems
and networks on which this Recommended Standard is implemented be addressed by the
management of those systems and networks.

.. _consequences_of_not_applying_security_rdm:

B1.3 CONSEQUENCES OF NOT APPLYING SECURITY TO THE TECHNOLOGY
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The consequences of not applying security to the systems and networks on which this
Recommended Standard is implemented could include potential loss, corruption, and theft of
data. The orbit data included in the RDM could allow an unintended recipient to track a re-
entering spacecraft, and the ground impact data could allow them to collect any debris in an
unauthorized fashion.

.. _sana_considerations_rdm:

B2 SANA CONSIDERATIONS
----------------------

The following RDM-related items have been be registered with the SANA Operator:

- The RDM XML schemas in Navigation Data Messages XML Schema.

The following RDM elements should be taken from the appropriate SANA registry:

- The RDM originator, object owner, and object operator should be taken from the
  SANA Organizations registry;
- The catalog name should be taken from the SANA CDM catalog name registry;
- The center name should be taken from the SANA Orbit Centers registry;
- The time system should be taken from the SANA Time Systems registry;
- The reference frame and impact reference frame should be taken from the SANA
  Celestial Body Reference Frames registry;
- The covariance reference frame should be taken from the SANA Orbit-relative
  Reference Frames registry.

The registration rule for new entries in any of the registries listed above is the approval of
new requests by the CCSDS Area or Working Group responsible for the maintenance of the
RDM or of the respective registry at the time of the request. New requests for these registries
should be sent to SANA (info@sanaregistry.org).

.. _patent_considerations_rdm:

B3 PATENT CONSIDERATIONS
------------------------

The recommendations of this Recommended Standard have no patent issues.

.. _annex_c_rdm:

ANNEX C
=======

RE-ENTRY DATA MESSAGE EXAMPLES
==============================

(INFORMATIVE)

Figures C-1 and C-2 show examples of RDM messages in KVN. The former only includes
mandatory keywords, while the latter uses optional keywords. Figure C-1 presents a very
basic RDM, the kind a Member Agency (or any other group) would make available to the
interested public (e.g., amateur astronomers wishing to schedule the observation of a re-
entry) for long-term re-entry prediction and containing only the remaining orbital lifetime.
Figure C-2 shows a more complex message, the kind of information two Member Agencies
might exchange with each other for short-term re-entry predictions and containing the state
vector, position/velocity covariance matrix.

.. code-block:: text

   CCSDS_RDM_VERS = 1.0
   CREATION_DATE = 2018-04-22T09:31:34.00
   ORIGINATOR = ESA
   MESSAGE_ID = ESA/20180422-001
   
   OBJECT_NAME = SPACEOBJECT
   INTERNATIONAL_DESIGNATOR = 2018-099B
   CONTROLLED_REENTRY = NO
   CENTER_NAME = EARTH
   TIME_SYSTEM = UTC
   EPOCH_TZERO = 2018-04-22T00:00:00.00
   
   ORBIT_LIFETIME = 23.0 [d]
   REENTRY_ALTITUDE = 150.0 [km]

**Figure C-1: Sample RDM in KVN Using Only Mandatory Keywords**

.. code-block:: text

   CCSDS_RDM_VERS = 1.0
   CREATION_DATE = 2018-04-22T09:31:34.00
   ORIGINATOR = ESA
   MESSAGE_ID = ESA/20180422-001
   
   OBJECT_NAME = SPACEOBJECT
   INTERNATIONAL_DESIGNATOR = 2018-099B
   CATALOG_NAME = SATCAT
   OBJECT_DESIGNATOR = 81594
   OBJECT_TYPE = ROCKET BODY
   OBJECT_OWNER = ESA
   CONTROLLED_REENTRY = NO
   CENTER_NAME = EARTH
   TIME_SYSTEM = UTC
   EPOCH_TZERO = 2018-04-22T09:00:00.00
   REF_FRAME = EME2000
   GRAVITY_MODEL = EGM-96: 36D 360
   ATMOSPHERIC_MODEL = NRLMSISE-00
   N_BODY_PERTURBATIONS = MOON
   SOLAR_RAD_PRESSURE = NO
   EARTH_TIDES = ESR
   INTRACK_THRUST = NO
   REENTRY_DISINTEGRATION = MASS-LOSS + BREAK-UP
   PREVIOUS_MESSAGE_ID = ESA/20180421-007
   NEXT_MESSAGE_EPOCH = 2018-04-23T09:00:00
   
   COMMENT Short term re-entry prediction results
   ORBIT_LIFETIME = 5.5 [d]
   REENTRY_ALTITUDE = 80.0 [km]
   NOMINAL_REENTRY_EPOCH = 2018-04-27T19:45:33
   REENTRY_WINDOW_START = 2018-04-27T11:45:33
   REENTRY_WINDOW_END = 2018-04-27T22:12:56
   PROBABILITY_OF_IMPACT = 0.0
   PROBABILITY_OF_BURN_UP = 1.0
   
   COMMENT State vector at the last OD epoch
   EPOCH = 2018-04-22T09:30:12
   X = 4000.000000 [km]
   Y = 4000.000000 [km]
   Z = 4000.000000 [km]
   X_DOT = 7.000000 [km/s]
   Y_DOT = 7.000000 [km/s]
   Z_DOT = 7.000000 [km/s]
   
   COMMENT Position/velocity covariance matrix at last OD epoch
   COV_REF_FRAME = RTN
   CX_X = 0.10000 [km**2]
   CY_X = 0.10000 [km**2]
   CY_Y = 0.10000 [km**2]
   CZ_X = 0.10000 [km**2]
   CZ_Y = 0.10000 [km**2]
   CZ_Z = 0.10000 [km**2]
   CX_DOT_X = 0.02000 [km**2/s]
   CX_DOT_Y = 0.02000 [km**2/s]
   CX_DOT_Z = 0.02000 [km**2/s]
   CX_DOT_X_DOT = 0.00600 [km**2/s**2]
   CY_DOT_X = 0.02000 [km**2/s]
   CY_DOT_Y = 0.02000 [km**2/s]
   CY_DOT_Z = 0.02000 [km**2/s]
   CY_DOT_X_DOT = 0.00600 [km**2/s**2]
   CY_DOT_Y_DOT = 0.00600 [km**2/s**2]
   CZ_DOT_X = 0.02000 [km**2/s]
   CZ_DOT_Y = 0.02000 [km**2/s]
   CZ_DOT_Z = 0.02000 [km**2/s]
   CZ_DOT_X_DOT = 0.00400 [km**2/s**2]
   CZ_DOT_Y_DOT = 0.00400 [km**2/s**2]
   CZ_DOT_Z_DOT = 0.00400 [km**2/s**2]
   
   COMMENT Spacecraft parameters used in OD and re-entry prediction
   WET_MASS = 3582 [kg]
   DRAG_AREA = 23.3565 [m**2]
   DRAG_COEFF = 2.2634
   
   COMMENT OD parameters from batch orbit determination
   ACTUAL_OD_SPAN = 3.4554 [d]
   TRACKS_AVAILABLE = 18
   TRACKS_USED = 17

**Figure C-2: Sample RDM in KVN Using Optional Keywords**

.. include:: rdm_parts/figure_c-4.rst
.. _annex_d_rdm:

ANNEX D
=======

ABBREVIATIONS AND ACRONYMS
==========================

(INFORMATIVE)

.. list-table::
   :widths: 15 85
   :header-rows: 0

   * - ASCII
     - American Standard Code for Information Interchange
   * - CCSDS
     - Consultative Committee for Space Data Systems
   * - CDM
     - Conjunction Data Message
   * - COSPAR
     - Committee on Space Research
   * - DOF
     - degree of freedom
   * - IADC
     - Inter-Agency Debris Coordination Committee
   * - ICD
     - interface control document
   * - ICS
     - implementation conformance statement
   * - ID
     - identifier
   * - ISO
     - International Organization for Standardization
   * - KVN
     - keyword value notation
   * - NDM
     - Navigation Data Message
   * - OD
     - orbit determination
   * - ODM
     - Orbit Data Message
   * - OEM
     - Orbit Ephemeris Message
   * - OPM
     - Orbit Parameter Message
   * - PDF
     - probability distribution function
   * - RDM
     - Re-entry Data Message
   * - RL
     - requirements list
   * - RMS
     - root mean square
   * - SANA
     - Space Assigned Numbers Authority
   * - SI
     - International System of Units (Système international d'unités)
   * - SRP
     - solar radiation pressure
   * - SSA
     - space situational awareness
   * - SST
     - space surveillance and tracking
   * - UNOOSA
     - United Nations Office for Outer Space Affairs
   * - URL
     - uniform resource locator
   * - UTC
     - Coordinated Universal Time
   * - XML
     - Extensible Markup Language

.. _annex_e_rdm:

ANNEX E
=======

RATIONALE AND REQUIREMENTS FOR RE-ENTRY DATA MESSAGES
=====================================================

(INFORMATIVE)

This annex presents the rationale and requirements behind the design of the Re-entry Data
Message. Table E-1 shows the RDM requirements’:

- requirement identifier;
- requirement text;
- rationale behind the requirement;
- traceability to sections of this standard; and
- whether the requirement is mandatory (M) (shall) or optional (O), but desirable
  (should).

.. list-table:: Table E-1: RDM Requirements and Rationale
   :widths: 15 35 35 5 10
   :header-rows: 1

   * - #
     - Requirement
     - Rationale
     - Trace
     - M/O
   * - RDM-0010
     - The RDM shall be provided in digital form.
     - To facilitate automated interaction.
     - 3.2.2
     - M
   * - RDM-0020
     - The RDM shall be provided in platform-independent data structures.
     - The CCSDS objective of promoting interoperability is best met by avoiding proprietary data formats.
     - section 3
     - M
   * - RDM-0025
     - The RDM shall be readable by both humans and computers.
     - To facilitate both automatic interaction and analysis of the data by human operators.
     - table 3-1, table 3-2, table 3-3
     - M
   * - RDM-0030
     - The RDM shall provide a means of uniquely identifying each message.
     - The file name is not sufficient for these purposes and a unique ID facilitates exchange.
     - table 3-1
     - M
   * - RDM-0040
     - The RDM shall clearly identify the object predicted to (re-)enter.
     - Any ambiguity in the object reduces the usefulness of the message.
     - table 3-2
     - M
   * - RDM-0050
     - The RDM shall provide the remaining orbit lifetime of the object.
     - This information is needed to determine the timeliness of any needed mitigation measures.
     - table 3-3
     - M
   * - RDM-0055
     - The RDM shall specify whether the re-entry is controlled, uncontrolled, or unknown.
     - To determine if/what mitigation measures are needed. Unknown is a valid value as well.
     - table 3-2
     - M
   * - RDM-0060
     - The RDM shall provide time measurements in the accepted CCSDS timecode formats.
     - To fulfill the CCSDS’ objective of promoting interoperability and to make them easier to read and understand by humans.
     - 5.3.3.5
     - M
   * - RDM-0070
     - The RDM shall provide the units used for all the data.
     - To promote interoperability and eliminate ambiguity.
     - table 3-2, table 3-3
     - M
   * - RDM-0080
     - The RDM shall allow the exchange of (re-)entry information for objects orbiting an arbitrary body.
     - Allows the use of the standard for Moon/Mars/Jupiter atmospheric entry besides Earth, further promoting the use of this standard.
     - table 3-2
     - M
   * - RDM-0090
     - The information in the RDM shall be usable without the need to model any (re-)entry spacecraft dynamics by the receiving entity.
     - There are several situations in which this is useful, such as remaining orbit lifetime is too short for modeling or the receiving entity does not have those capabilities.
     - table 3-3
     - M
   * - RDM-0100
     - The RDM should be extensible, without disrupting existing uses.
     - Some users might need other parameters and they should be able to use them with user defined lines specified in an ICD.
     - table 3-3
     - O
   * - RDM-0110
     - The RDM should be consistent with the other CCSDS messages.
     - To facilitate use and improve readability.
     - table 3-1, table 3-2, table 3-3
     - O
   * - RDM-0120
     - The RDM shall allow for the object owner/operator to be specified.
     - Can help with mitigation measures.
     - table 3-2
     - M
   * - RDM-0130
     - The RDM should allow the exchange of one spacecraft position/velocity state vector and associated information.
     - Allows the message to be self-contained and not rely on ODMs (which might require an ICD).
     - table 3-3
     - O
   * - RDM-0140
     - The RDM should allow the exchange of covariance information for the position/velocity state vector.
     - To allow the uncertainty in the state vector to be exchanged as well.
     - table 3-3
     - O
   * - RDM-0150
     - The RDM should allow the modeling used to determine the data to be specified.
     - Necessary if the data is to be verified by the received agency.
     - table 3-2
     - O
   * - RDM-0160
     - The RDM shall contain information about the previous and next messages for the same spacecraft.
     - Because of modeling uncertainties it is expected that RDMs for the same spacecraft would be published regularly.
     - table 3-2
     - M
   * - RDM-0170
     - The RDM shall specify the predicted ground impact location and epoch for the object.
     - Important information if ground impact is predicted.
     - table 3-3
     - M
   * - RDM-0180
     - The RDM should contain the spacecraft parameters used in OD and re-entry prediction.
     - Allows verification and independent orbit propagation.
     - table 3-3
     - O
   * - RDM-0190
     - The RDM shall contain information about the observations used in OD.
     - Allows the receiving agency to evaluate the reliability of the data.
     - table 3-3
     - M

.. _annex_f_rdm:

ANNEX F
=======

RDM SUMMARY SHEET
=================

(INFORMATIVE)

The provision of optional keywords is at the discretion of the data provider. This annex is
intended to provide a helpful guide (table F-1) in associating optional metadata keywords
(from table 3-2) and data categories (from table 3-3). There are only a few required metadata
and data keywords, but many more that are applicable to one or more data categories. Some
data categories can also provide extra information for other categories. Additionally, there
are some keywords that are only applicable in certain restricted situations. Finally, there are
some metadata keywords that are completely optional. This summary may assist the user in
constructing an RDM that properly describes a specific re-entry event.

The terms ‘required’ and ‘recommended’ in table F-1 relate only to the context set by the
data categories and are not normative for the RDM.

.. list-table:: Table F-1: Relationship between RDM Data Categories and Optional Metadata Keywords
   :widths: 20 40 40
   :header-rows: 1

   * - Data category
     - Relevant metadata optional keywords
     - Relevant data blocks
   * - long-term atmospheric re-entry
     - required: none (only mandatory keywords)
       recommended: GRAVITY_MODEL, ATMOSPHERIC_MODEL, SOLAR_FLUX_PREDICTION, N_BODY_PERTURBATIONS
     - required: none (only mandatory keywords)
       recommended: object physical parameters
   * - short-term atmospheric re-entry
     - required: none (only mandatory keywords)
       recommended: GRAVITY_MODEL, ATMOSPHERIC_MODEL, SOLAR_FLUX_PREDICTION, N_BODY_PERTURBATIONS
     - required: none (only mandatory keywords)
       recommended: NOMINAL_REENTRY_EPOCH, REENTRY_WINDOW_START, REENTRY_WINDOW_END, object physical parameters
   * - ground impact data
     - required: REENTRY_DISINTEGRATION
       recommended: same as short-term atmospheric re-entry, and: IMPACT_UNCERTAINTY_METHOD
     - required: ground impact and burn-up data
       recommended: same as short-term atmospheric re-entry
   * - object state vector
     - required: REF_FRAME
     - required: object state vector
       recommended: object physical parameters
   * - position/velocity covariance matrix
     - required: none (only mandatory keywords)
     - required: position/velocity covariance matrix, object state vector
   * - object physical parameters
     - required: none (only mandatory keywords)
     - required: object physical parameters
   * - orbit determination parameters
     - required: GRAVITY_MODEL, ATMOSPHERIC_MODEL, SOLAR_FLUX_PREDICTION, N_BODY_PERTURBATIONS, SOLAR_RAD_PRESSURE, EARTH_TIDES, INTRACK_THRUST
     - required: object physical parameters

.. _annex_g_rdm:

ANNEX G
=======

RE-ENTRY INFORMATION DESCRIPTION
================================

(INFORMATIVE)

.. _re-entry_prediction_rdm:

G1 RE-ENTRY PREDICTION
----------------------

.. _overview_re-entry_prediction_rdm:

G1.1 OVERVIEW
^^^^^^^^^^^^^

As of April 2017, slightly more than 24000, or 56 percent of all catalogued objects, have
decayed and re-entered the atmosphere and several more re-enter every month. Re-entries
can pose risk to ground based infrastructure and population, either due to the large mass of
the spacecraft (e.g., Mir, Skylab, or the Salyut space stations), or due to hazardous materials
(e.g., Cosmos 954).

Predicting and simulating re-entries covers:

- long and medium-term re-entry predictions (several years to a few weeks before re-
  entry);
- short-term re-entry predictions (last days/weeks before re-entry; altitude below 200
  km);
- break-up and survival (out of scope of the RDM); and
- ground impact and on-ground risk (partially out of scope of the RDM, due to
  limitations in break-up and survival estimations).

The following subsections describe re-entry simulations and modeling for the Earth, but
similar considerations apply to other celestial bodies. General information on navigation data
can be found in reference [H1]. Re-entry simulation details can be found in chapter 9 of
[H2], which describes re-entry simulations and how they applied to the Mir re-entry. Details
on the European Space Agency’s contributions to IADC (Inter-Agency Debris Coordination
committee) re-entry campaigns can be found in references [H3] and [H4].

The estimation of the orbit lifetime is covered by an ISO standard, ISO 27852:2016
(reference [H5]). The standard focuses on long- and medium-term predictions and contains
useful information to determine which mathematical models (e.g., gravity and atmospheric
models) are appropriate in most situations. ISO is also working on a standard on atmospheric
modelling, but as of the writing of this document, it has not been completed.

.. _nomenclature_re-entry_prediction_rdm:

G1.2 NOMENCLATURE
^^^^^^^^^^^^^^^^^

The following terms will be used in equations in the following subsections:

.. list-table::
   :widths: 10 90

   * - a
     - semi-major axis
   * - A
     - area
   * - B
     - ballistic coefficient
   * - :math:`C_d`
     - drag coefficient
   * - :math:`C_p`
     - specific heat at constant pressure
   * - :math:`g_0`
     - sea-level gravitational acceleration
   * - :math:`h_m`
     - melting enthalpy
   * - m
     - mass
   * - Q
     - heat
   * - t
     - time
   * - T
     - temperature
   * - :math:`T_m`
     - melting temperature
   * - v
     - velocity
   * - V
     - volume
   * - :math:`v_a`
     - velocity relative to the atmosphere
   * - :math:`v_{impact}`
     - impact velocity
   * - :math:`ε`
     - radiation emission coefficient
   * - :math:`rho`
     - density
   * - :math:`rho_0`
     - Earth sea level atmospheric density
   * - :math:`σ`
     - Stefan-Boltzmann constant

.. _long_medium_term_re-entry_predictions_rdm:

G1.3 LONG- AND MEDIUM-TERM RE-ENTRY PREDICTIONS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

In the context of re-entry predictions, long- and medium-term means several years to a few
weeks of orbital lifetime remaining. The current typical approach for these simulations is to
use singly averaged perturbation equations integrated over a time step between 0.1 and 5
orbits. The perturbations accounted for by existing modeling software are: non-spherical
Earth gravity, dynamic Earth atmosphere, luni-solar gravity perturbations, and solar radiation
pressure combined with an oblate cylindrical Earth shadow. The end goal of these
simulations is to estimate the remaining orbital lifetime.
Long- and medium-term re-entry predictions require simulating the decrease in the orbit’s
semi-major axis. For nearly circular orbits the rate of change of the semi-major axis with
respect to time can be approximated by:

.. math:: \frac{da}{dt} = -\frac{a^2 \rho v_a}{B}

.. math:: B = \frac{m}{C_d A}

where :math:`v_a` is the velocity with respect to the atmosphere. While :math:`v_a` and the atmospheric density
can be estimated using Earth atmosphere models and orbit dynamics, the ballistic coefficient
cannot. Uncontrolled objects tumble, so their angle of attack, and hence aerodynamic cross-
section and drag coefficient are not constant. One approach to this issue is to retro-fit the
simulated semi-major axis time history over observed values over a period of time by
adjusting the ballistic coefficient.

Besides the ballistic coefficient, other sources of uncertainty in the results are: atmospheric
density modeling (both the models themselves and unexpected space weather events), attitude,
and the orbit state. A standard deviation of ±20 percent for the remaining orbital lifetime is a
good estimate for all the above in nominal conditions. More complex uncertainty modeling is
also possible, e.g., accounting for orbit covariance and ballistic coefficient fit residuals.

.. _short_term_re-entry_predictions_and_impact_rdm:

G1.4 SHORT-TERM RE-ENTRY PREDICTIONS AND IMPACT
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

When the orbit altitude drops below ~200 km, the assumptions used in long-term predictions
(perturbation effects are small, no cross-coupling effects) no longer hold and more accurate
modeling is needed. Numerical solutions to the perturbed equations of motion are used,
typically accounting for: Earth gravity, luni-solar attraction, solar radiation pressure with
umbral and penumbra transits, and atmospheric drag. Some success has been demonstrated
for controlled re-entry with 6DOF numerical integration of the satellite states and attitude.

Below ~120 km atmospheric drag becomes the dominating force, and properly accounting
for the change in drag coefficient between free-molecular flow, transitional, and continuum
conditions is critical. For example, the drag coefficient of the Mir space station decreased
from 2.21 in the free-molecular flow regime to 1.18 in the continuum subsonic regime.
Atmospheric drag steepens the flight path, making the orbit more eccentric. At some point
the re-entering object will achieve equilibrium free fall, leading to an impact velocity:

.. math:: v_{impact} = \sqrt{\frac{2mg_0}{C_d A \rho_0}}

Dispersion in the impact location of a re-entry survivor object is driven by uncertainties in
the area-to-mass ratio, aerodynamic coefficients (drag, lift, and side-force), air density, and
initial orbit and attitude state and it is given in the along- and cross-track directions. Monte
Carlo simulations are needed to determine a geographic probability distribution. The
probability density function in a given area can be computed as the fraction of impact in said
area over total number of simulations. The probability distribution function (PDF) in the
along-track direction is a distorted 2D Gaussian, further complicating matters.

Simpler approaches are sometimes used. Analytical formulae allow the computation of
dispersion due to aerodynamic forces, based on lift, drag, and side-force coefficients and the
ratio of lift and side-force, but they can be difficult to use and do not account for the main
sources of uncertainty. An alternative is to assume a standard Gaussian distribution, compute
the along-track dispersion by varying the aerodynamic drag term (ρ·B) and use an empirical
value for the cross-track dispersion (e.g., a 1-sigma value of 20 km).

.. _survival_and_break-up_during_re-entry_rdm:

G1.5 SURVIVAL AND BREAK-UP DURING RE-ENTRY
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

During re-entry aerodynamic braking and aerothermal heating can lead to:

- disintegration—the object breaks up into multiple fragments or loses external parts
  (e.g., solar panels, thermal shielding);
- mass loss—heating leads to melting and ablative mass loss; or
- total demise—the object burns up completely and there is no impact.

The heat flux due to aerothermal heating depends on the flow conditions (free-molecular,
transitional, continuum supersonic, and continuum subsonic), hence on object velocity and
atmospheric density. How to estimate this flux is beyond the scope of this informative annex.
The heat flux into the object leads to an increase in internal temperature, melting (once
temperature is high enough), and radiating heat away from the object. Assuming a uniform
composition leads to the following equations:

.. math:: \dot{Q} = \rho c_p V \dot{T} + \epsilon \sigma A T^4, T < T_m

.. math:: \dot{Q} = -\rho h_m \dot{V} + \epsilon \sigma A T^4, T = T_m

Until the object’s surface reaches the melting temperature, the heat flux in is equal to the heat
absorbed by increasing internal temperature and the heat radiated away. Once the surface
reaches the melting temperature, the heat flux is equal to the heat lost by mass ablation and
the heat radiated away. This leads to two ways in which an object can survive re-entry:
absorb the re-entry heat without reaching the melting temperature or radiate the heat away
efficiently. If the analysis is restricted to solid metallic spheres, very small objects (smaller
than ~1 cm), can do the latter, while very large objects (larger than ~ 10 cm) can do the
former. Objects in between (e.g., most nuts and bolts used in spacecraft assembly) will
typically, but not always, suffer total demise.

Simulating disintegration and mass loss for complex objects is a multi-disciplinary process,
since spacecraft have complex shapes and contain a multitude of materials. The SCARAB
tool, for example, can simulate disintegration and mass loss by dividing the spacecraft into
basic elements, each with consistent material properties and each element into multiple
voxels (volume elements). The simulation accounts for the forces acting on each
element/voxel (as in G1.4) and computes heat fluxes, temperatures, and normal and shear
stresses for a thermal and structural analysis. This determines break-ups (when stresses
exceed the ultimate tensile or shear stress in the structure) and ablative mass loss.

Exchanging the information needed to accurately replicate these types of simulations is
beyond the scope of the RDM. Furthermore, the tools needed to replicate the results are not
widely available, which restricts usefulness. Break-ups during re-entry tend to happen at
around 80 km altitude, when the combined aerodynamic and thermal loads are highest, but
without knowing the structure of the object nothing more can be said.

.. _atmospheric_re-entry_and_ground_impact_data_keywords_rdm:

G2 ATMOSPHERIC RE-ENTRY AND GROUND IMPACT DATA KEYWORDS
-----------------------------------------------------------

**ORBIT_LIFETIME:** The remaining time in orbit of the object, in days, from the
EPOCH_TZERO keyword in the metadata. This is intended to be a coarse estimation, useful
for long- and medium-term (more than a few days) re-entry prediction. The orbit lifetime is
considered to end when the altitude permanently drops below the REENTRY_ALTITUDE.
This keyword has two roles:

- Specify whether long/medium or short-term re-entry simulations were used for the
  object (e.g., a high value means a long-term simulation was performed); and
- Give an estimate of the remaining orbit lifetime, especially for long-term predictions,
  with appropriate accuracy.

**REENTRY_ALTITUDE:** The altitude at which the orbit lifetime ends and at which re-entry
is defined. Depending on the particular re-entry event, different values can be specified, e.g.,
150 km for ‘object captured by the Earth’s atmosphere’, 70 km for ‘object likely to break-up
at this altitude’, or simply the altitude limit at which the re-entry simulation stops.

**ORBIT_LIFETIME_WINDOW_START** and **ORBIT_LIFETIME_WINDOW_END:** The
uncertainty time window associated with the ORBIT_LIFETIME keyword and meant for long-
term re-entry predictions. ±20% from the orbit lifetime provides an acceptable empirical
estimate for nominal conditions, accounting for uncertainties in the ballistic coefficient,
atmospheric density, and orbit parameters.

Figure G-1 shows the type of information that can be generated from medium-term re-entry
prediction RDMs. It assumes a new RDM containing an updated orbital lifetime prediction
and uncertainty (fixed at 20 percent) is issued every couple of days. The predicted re-entry
date (orbit lifetime + epoch t_zero) is plotted against the date at which the simulation was
run (creation date), orbit lifetime window start and end (the dotted lines) are plotted as well.

.. figure:: /images/medium-term_re-entry_prediction.png
   :align: center

   Figure G-1: Example of Medium-Term Re-entry Prediction Data Use

**NOMINAL_REENTRY_EPOCH:** Predicted re-entry epoch, providing much greater
precision for short-term (last couple of days before re-entry, when altitude is below 200 km)
re-entry predictions.

**REENTRY_WINDOW_START** and **REENTRY_WINDOW_END:** Allow the RDM
originator to specify a re-entry time window, which does not have to be symmetric around
the nominal re-entry epoch. They can account for uncertainties in the modeling (atmospheric
density, drag coefficient, attitude influence) or for under/over performance of thrusting
maneuvers in a controlled re-entry.

**ORBIT_LIFETIME_CONFIDENCE_LEVEL:** The (estimated) confidence level of the re-
entering between ORBIT_LIFETIME_WINDOW_START and
ORBIT_LIFETIME_WINDOW_END or between REENTRY_WINDOW_START and
REENTRY_WINDOW_END.

**PROBABILITY_OF_IMPACT:** Probability of the object or any resulting fragments
impacting the Earth (land or sea).

**PROBABILITY_OF_BURN_UP:** Total demise probability for the object and/or any
resulting fragments.

**PROBABILITY_OF_BREAK_UP:** Probability of fragments being generated during re-
entering, both ‘shedding’ of parts (e.g., solar panels, heat shields) and fragmentation of the main
structure.

**PROBABILITY_OF_LAND_IMPACT:** Probability of the object or any resulting
fragments impacting solid land. This is the first step in assessing any casualty probability, in
conjunction with population density mapping.

**NOMINAL_IMPACT_EPOCH:** The nominal (predicted) epoch at which the object hits the
surface of the Earth (either solid land or ocean).

**IMPACT_WINDOW_START** and **IMPACT_WINDOW_END:** The same as
REENTRY_WINDOW_START and _END, but for the impact epoch.

**IMPACT_REF_FRAME, NOMINAL_IMPACT_LON, _LAT, _ALT:** Allow the
specification of the nominal (predicted) impact location in geodetic coordinates.

**IMPACT_n_CONFIDENCE, IMPACT_n_START_LON, IMPACT_n_START_LAT,**
**IMPACT_n_STOP_LON, IMPACT_n_STOP_LAT, IMPACT_n_CROSS_TRACK:**
Uncertainties in the ground impact location are expressed in the along-track and cross-track
directions, which means the (predicted) ground track needs to be known. Since the
probability distribution is skewed, the method chosen to deal with these two issues is to:

- give the coordinates corresponding to the maximum probability, with the
  NOMINAL_IMPACT_LON and _LAT keywords;
- define up to three confidence intervals;
- give the coordinates of the two points on the ground track corresponding to the start
  and end of each of the (up to) three confidence intervals.

As long as at least three points are provided the ground track can be estimated as one or more
segments of a great circle. The cross-track confidence intervals (in km) can be used to
determine the swaths of impact probability, which can then be used together with population
density mapping to determine the casualty probability.

The ground impact location and uncertainties are supposed to cover all eventual re-entry
fragments (fragments generated during re-entry, not before). For example, if the re-entry data
is obtained through Monte Carlo simulations, NOMINAL_IMPACT_LON, _LAT would
correspond to the bin with the most impacts, the first swath would cover
IMPACT_1_CONFIDENCE of the impacts, and so on. If analytical formulae are used for the
1-sigma confidence interval, IMPACT_1_CONFIDENCE should be 47 percent (for
rectangular, rather than elliptical areas), the IMPACT_1_START/STOP_LAT/LON values
can be determined from the formulae and the ground track, and
IMPACT_1_CROSS_TRACK from the formula used.

.. _orbit_determination_keywords_rdm:

G3 ORBIT DETERMINATION KEYWORDS
--------------------------------

To promote consistency and ease implementation, the orbit determination keywords are the
same as in the Conjunction Data Message (reference [H6]). These keywords are present to
give the consumer of the message an idea of the quality of the data provided and when the re-
entering object was last observed.

**Observation:** Unique measurement of a satellite’s location from a single sensor at a single
time (e.g., azimuth from a single sensor at a single time).

**TIME_LASTOB_START** and **TIME_LASTOB_END:** The start and end of a time interval
that contains the time of the last accepted observation. For an exact time, the time interval is
of zero duration (i.e., TIME_LASTOB_START = TIME_LASTOB_END).

**RECOMMENDED_OD_SPAN:** How many days of observations were recommended for
the OD of the object.

**ACTUAL_OD_SPAN:** The actual time span used for the OD of the object based on the
observations available and the RECOMMENDED_OD_SPAN.

**OBS_AVAILABLE:** The number of observations, for the recommended time span, that
were available for the OD.

**OBS_USED:** The number of observations, for the recommended time span, that were
accepted for the OD.

**Sensor Track:** A set of at least three observations for the same object, observed by the same
sensor, where each observation is within a specified number of minutes (which is dependent
on the orbit regime of the object) of the other observations in the track.

**TRACKS_AVAILABLE:** The number of sensor tracks, for the recommended time span, that
were available for the OD. This provides information about the independence of the
observational data used in the OD.

**TRACKS_USED:** The number of sensor tracks, for the recommended time span, that were
accepted for the OD. This provides information about the independence of the observational
data used in the OD.

**WEIGHTED_RMS:**

.. math:: \text{Weighted RMS} = \sqrt{\frac{\sum_{i=1}^{N} w_i (y_i - y_{i, \text{estimated}})^2}{N}}

Where:

- :math:`y_i` is the ith observation;
- :math:`y_{i, \text{estimated}}` is the estimated value (from the resulting orbit) of :math:`y_i`;
- :math:`w_i` is the weighting of the observation; and
- N is the total number of observations.

This is a value that can generally identify the quality of the most recent orbit update, and is
used by the analyst in evaluating the OD process.

.. _annex_h_rdm:

ANNEX H
=======

INFORMATIVE REFERENCES
======================

(INFORMATIVE)

[H1] *Navigation Data—Definitions and Conventions*. Issue 3. Report Concerning Space
     Data System Standards (Green Book), CCSDS 500.0-G-3. Washington, D.C.: CCSDS,
     May 2010.
[H2] Heiner Klinkrad. *Space Debris: Models and Risk Analysis*. Springer Praxis Books.
     Berlin, Heidelberg, New York: Springer, 2006.
[H3] Benjamin Bastida Virgili, et al. “GOCE Re-entry Campaign.” In *Proceedings of the 5th
     GOCE User Workshop (25–28 November 2014, UNESCO, Paris, France)*. Oakville,
     Ontario: ESA Communications, 2015.
[H4] Benjamin Bastida Virgili, et al. “Practicalities of Re-entry Predictions—The VEGA-01
     AVUM Case.” In *Proceedings of the 7th European Conference on Space Debris (18–
     21 April 2017, Darmstadt, Germany)*. Edited by T. Flohrer and F. Schmitz. Darmstadt,
     Germany: ESA Space Debris Office, 2017.
[H5] *Space Systems—Estimation of Orbit Lifetime*. 2nd ed. International Standard, ISO
     27852:2016. Geneva: ISO, 2016.
[H6] *Conjunction Data Message*. Issue 1. Recommendation for Space Data System
     Standards (Blue Book), CCSDS 508.0-B-1. Washington, D.C.: CCSDS, June 2013.
