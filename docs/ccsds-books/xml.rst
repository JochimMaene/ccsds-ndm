.. title:: XML SPECIFICATION FOR NAVIGATION DATA MESSAGES

.. rubric:: RECOMMENDED STANDARD

.. centered:: CCSDS 505.0-B-3

.. centered:: BLUE BOOK
.. centered:: May 2023

AUTHORITY
=========

.. table::
   :align: center

   +----------+--------------------------------+
   | Issue:   | Recommended Standard, Issue 3  |
   +----------+--------------------------------+
   | Date:    | May 2023                       |
   +----------+--------------------------------+
   | Location:| Washington, DC, USA            |
   +----------+--------------------------------+

This document has been approved for publication by the Management Council of the
Consultative Committee for Space Data Systems (CCSDS) and represents the consensus
technical agreement of the participating CCSDS Member Agencies. The procedure for
review and authorization of CCSDS documents is detailed in *Organization and Processes for
the Consultative Committee for Space Data Systems* (CCSDS A02.1-Y-4), and the record of
Agency participation in the authorization of this document can be obtained from the CCSDS
Secretariat at the email address below.

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
CCSDS-related member standards and implementations are not negated or deemed to be non-
CCSDS compatible. It is the responsibility of each member to determine when such
standards or implementations are to be modified. Each member is, however, strongly
encouraged to direct planning for its new standards and implementations towards the later
version of the Recommended Standard.

FOREWORD
========

This document is a technical Recommended Standard for an XML Specification for
Navigation Data Messages. This Recommended Standard has been developed via consensus
of the Navigation Working Group of the CCSDS Mission Operations and Information
Management Services (MOIMS) area. The XML schema set described in this Recommended
Standard represents the baseline concept for exchanging navigation data in XML format
between Agencies of the CCSDS.

This Recommended Standard establishes a common framework and provides a common basis
for the interchange of navigation data in XML format. It allows implementing organizations
within each Agency to proceed coherently with the development of compatible derived
standards for the flight and ground systems that are within their cognizance. Derived Agency
standards may implement only a subset of the optional features allowed by the
Recommended Standard and may incorporate features not addressed by this Recommended
Standard.

Attention is drawn to the possibility that some of the elements of this document may be the
subject of patent rights. CCSDS has processes for identifying patent issues and for securing
from the patent holder agreement that all licensing policies are reasonable and non-
discriminatory. However, CCSDS does not have a patent law staff, and CCSDS shall not be
held responsible for identifying any or all such patent rights.

Through the process of normal evolution, it is expected that expansion, deletion, or
modification of this document may occur. This Recommended Standard is therefore subject
to CCSDS document management and change control procedures, which are defined in the
*Organization and Processes for the Consultative Committee for Space Data Systems*
(CCSDS A02.1-Y-4). Current versions of CCSDS documents are maintained at the CCSDS
Web site:

http://www.ccsds.org/

Questions relating to the contents or status of this document should be sent to the CCSDS
Secretariat at the email address indicated on page i.

At time of publication, the active Member and Observer Agencies of the CCSDS were:

Member Agencies
---------------

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

Observer Agencies
-----------------

- Austrian Space Agency (ASA)/Austria.
- Belgian Science Policy Office (BELSPO)/Belgium.
- Central Research Institute of Machine Building (TsNIIMash)/Russian Federation.
- China Satellite Launch and Tracking Control General, Beijing Institute of Tracking and
  Telecommunications Technology (CLTC/BITTT)/China.
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
- Netherlands Space Office (NSO)/The Netherlands.
- Research Institute for Particle & Nuclear Physics (KFKI)/Hungary.
- Scientific and Technological Research Council of Turkey (TUBITAK)/Turkey.
- South African National Space Agency (SANSA)/Republic of South Africa.
- Space and Upper Atmosphere Research Commission (SUPARCO)/Pakistan.
- Swedish Space Corporation (SSC)/Sweden.
- Swiss Space Office (SSO)/Switzerland.
- United States Geological Survey (USGS)/USA.

DOCUMENT CONTROL
================

.. list-table::
   :widths: 25 25 15 35
   :header-rows: 1

   * - Document
     - Title
     - Date
     - Status
   * - CCSDS 505.0-B-1
     - XML Specification for Navigation Data Messages, Recommended Standard, Issue 1
     - December 2010
     - Original issue, superseded
   * - CCSDS 505.0-B-2
     - XML Specification for Navigation Data Messages, Recommended Standard, Issue 2
     - May 2021
     - Issue 2, superseded
   * - CCSDS 505.0-B-3
     - XML Specification for Navigation Data Messages, Recommended Standard, Issue 3
     - May 2023
     - Current issue: changes from the previous issue are summarized in annex J (note).
   * - CCSDS 505.0-B-3 EC 1
     - Editorial Change 1
     - February 2024
     - Corrects obsolete URL.
   * - CCSDS 505.0-B-3 EC 2
     - Editorial Change 2
     - October 2024
     - Replaces bitmap image with vector image in figure 4-2.

.. note:: Changes from the previous issue are too numerous to permit meaningful application of change bars.
.. _introduction_xml:

1 INTRODUCTION
==============

.. _purpose_xml:

1.1 PURPOSE
-----------

This Recommended Standard specifies a format for use in exchanging spacecraft navigation
data. Such exchanges are used for distributing navigation-related data between space agencies
and other space operators. The Recommended Standard specifies an integrated Extensible
Markup Language (XML) schema set that applies to Navigation Data Messages (NDMs)
defined in the CCSDS Recommended Standards developed by the CCSDS Navigation Working
Group (see references [4]–[8]). This XML schema set is suited to interagency exchanges of
any number of NDMs.

.. _scope_and_applicability_xml:

1.2 SCOPE AND APPLICABILITY
---------------------------

This Recommended Standard is applicable only to the schema content and layout, and to
instantiations of the schema, but not to the transmission of any instantiation of the schema.
The means of transmission of an XML-formatted NDM between exchange participants is
beyond the scope of this document; such arrangements require specification via other
arrangements, for example, in an Interface Control Document (ICD). Transmission of an
XML-formatted NDM could be based on a future CCSDS real-time data transfer service, a
file-based transfer protocol such as the Secure File Transfer Protocol (SFTP), streaming
media, email, or services provided via the World Wide Web and XML-compatible Web
browsers. The potential for compression/decompression of the message is an aspect of the
transmission that is not part of this specification. In general, it is a requirement that the
transmission mechanism not place constraints on the technical data content of an NDM.

As noted in the Purpose subsection above, this document applies to the NDMs defined in the
CCSDS Recommended Standards developed by the CCSDS Navigation Working Group.
Historically, the first few such Recommended Standards contained no XML representation.
Given the lack of XML representations in these early Recommended Standards, the first
version of this NDM/XML document contained information on how to create instantiations
of all the messages documented in the Orbit Data Messages (ODM), Attitude Data Messages
(ADM), and Tracking Data Message (TDM). Starting with Conjunction Data Message
(CDM) in 2013, the XML representation was directly included in the Recommended
Standard. XML representations have been added to other Recommended Standards as they
have been produced (the Re-Entry Data Message [RDM] in 2019, the TDM version 2 in
2020, and the ODM version 3 in 2023). As the early Navigation Working Group
Recommended Standards are being revised, the strategy is to remove the XML formatting
discussion from this NDM/XML document and migrate it into the revised documents; the
ADM is the last of these early Navigation Working Group early standards. (It should be
noted that the CCSDS Pointing Request Message [PRM] is also a standard created by the
CCSDS Navigation Working Group, but it is implemented using a set of XML templates
rather than as an XML message that can be validated via the XML schema language.)

The first version of this document only encompassed schemas and messages in which the
XML ‘elementFormDefault="unqualified"’ applied. This version of the Recommended
Standard encompasses schemas and messages in which the XML
‘elementFormDefault="unqualified"’ and ‘elementFormDefault="qualified"’ both apply. The
"qualified" schemas can be included/imported into XML schemas for other CCSDS
Recommended Standards that wish to leverage Navigation Working Group data structures.

.. _rationale_xml:

1.3 RATIONALE
-------------

This document responds to a requirement levied by the CCSDS to produce an XML format
for NDMs. Rather than revise several different CCSDS Recommended Standards, the
relevant XML format information was consolidated in Version 1 of this document. It includes
sets of requirements and criteria that the XML schema set has been designed to meet. The
rationale behind the design of the schema set is described in annex E in order to assist the
application engineer in constructing a suitable message.

.. _document_structure_xml:

1.4 STRUCTURE OF THIS DOCUMENT
------------------------------

Section 1 (this section) provides an introduction, scope, normative references, and the
description of the document structure.

Section 2 provides a very brief overview of the individual messages that constitute an NDM
(i.e., references [4]-[8]). It also provides a very brief overview of XML, and the justification
for an integrated NDM/XML schema set.

Section 3 provides an overview of the basic structure of the NDM/XML schema set. This
structure is external to the internal structure provided by the constituent messages.

Section 4 provides detailed discussion of the differences between the XML-formatted
messages and the Keyword Value Notation (KVN) text-formatted messages described in
references [4]–[8]. Instructions for how to construct instantiations of the ADM message
types and ‘combined instantiations’ are provided.

Annex A explains why this document does not contain an Implementation Conformance
Statement (ICS), a component of typical CCSDS Recommended Standards.

Annex B explains why this document does not provide in annex B the material that is
provided in annex B of other Navigation Working Group standards.

Annex C discusses information security, Space Assigned Numbers Authority (SANA), and
patent considerations.

Annex D is a list of abbreviations and acronyms applicable to the NDM/XML.

Annex E lists a set of requirements that were taken into consideration in the design of the
NDM/XML schema set.

Annex F provides some technical material and conventions relevant to the NDM/XML.

Annex G provides instructions on where to find the schema set referenced in this
Recommended Standard on the SANA website. Also provided for illustrative purposes are a
number of example instantiations of NDM/XML messages.

Annex H contains a list of informative references.

Annex I lists a number of items that should be covered in interagency ICDs prior to
exchanging NDMs in XML format on a regular basis.

Annex J lists the changes in this version of the Recommended Standard compared to the
previous version.

.. _conventions_and_definitions_xml:

1.5 CONVENTIONS AND DEFINITIONS
-------------------------------

.. _nomenclature_xml:

1.5.1 NOMENCLATURE
^^^^^^^^^^^^^^^^^^^^

The following conventions apply throughout this Recommended Standard:

a) the words ‘shall’ and ‘must’ imply a binding and verifiable specification;
b) the word ‘should’ implies an optional, but desirable, specification;
c) the word ‘may’ implies an optional specification;
d) the words ‘is’, ‘are’, and ‘will’ imply statements of fact.

.. _terms_xml:

1.5.2 TERMS
^^^^^^^^^^^

For the purposes of this document, the following definitions apply:

**CamelCase:** a style of capitalization in which the initial characters of concatenated words are
capitalized.

**lowerCamelCase:** a variant on CamelCase in which the first character of a character string
formed from concatenated words is lowercase. In the case of a character string consisting of
only a single word, only lowercase characters are used.

**ASCII:** a text character set defined in reference [H4].

.. _references_xml:

1.6 REFERENCES
--------------

The following documents contain provisions which, through reference in this text, constitute
provisions of this Recommended Standard. At the time of publication, the editions indicated
were valid. All documents are subject to revision, and users of this Recommended Standard
are encouraged to investigate the possibility of applying the most recent editions of the
documents indicated below. The CCSDS Secretariat maintains a register of currently valid
CCSDS Recommended Standards.

[1] Henry S. Thompson, et al., eds. “XML Schema Part 1: Structures.” W3C
    Recommendation. 2nd ed., 28 October 2004. The World Wide Web Consortium
    (W3C). https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/.
[2] Paul V. Biron and Ashok Malhotra, eds. “Extensible Markup Language (XML) 1.0.”
    W3C Recommendation. 3rd ed., February 2004. https://www.w3.org/TR/2004/REC-
    xmlschema-2-20041028/.
[3] “Navigation Data Messages XML Schema.” Space Assigned Numbers Authority.
    https://sanaregistry.org/r/ndmxml.
[4] *Attitude Data Messages*. Issue 1. Recommendation for Space Data System Standards
    (Blue Book), CCSDS 504.0-B-1. Washington, D.C.: CCSDS, May 2008.
[5] *Orbit Data Messages*. Issue 3. Recommendation for Space Data System Standards
    (Blue Book), CCSDS 502.0-B-3. Washington, D.C.: CCSDS, April 2023.
[6] *Tracking Data Message*. Issue 2. Recommendation for Space Data System Standards
    (Blue Book), CCSDS 503.0-B-2. Washington, D.C.: CCSDS, June 2020.
[7] *Conjunction Data Message*. Issue 1. Recommendation for Space Data System
    Standards (Blue Book), CCSDS 508.0-B-1. Washington, D.C.: CCSDS, June 2013.
[8] *Re-entry Data Message*. Issue 1. Recommendation for (Blue Book), CCSDS 508.1-B-1.
    Washington, D.C.: CCSDS, November 2019.

.. note:: Informative references are provided in annex H.

.. _overview_xml:

2 OVERVIEW
==========

.. _navigation_data_messages_xml:

2.1 NAVIGATION DATA MESSAGES
----------------------------

This subsection provides a brief overview of the set of NDMs. There are five basic types of
NDM that are covered by the schemas described in this document: Attitude Data Messages
(reference [4]), Orbit Data Messages (reference [5]), Tracking Data Message (reference [6]),
Conjunction Data Message (reference [7]), and Re-entry Data Message (reference [8]). The
remainder of this document conveys the structure of the NDMs in an integrated XML schema set.

.. _attitude_data_messages_xml:

2.2 ATTITUDE DATA MESSAGES
--------------------------

Attitude Data Messages comprise two message types used to convey spacecraft attitude
information: the Attitude Parameter Message (APM) and Attitude Ephemeris Message
(AEM). The APM consists of an instantaneous attitude state and optional attitude maneuvers.
The AEM consists of a history/forecast of the attitude of the object; the history/forecast can
be interpolated to obtain the attitude of the spacecraft at times other than those specified in
the message. Instructions for creating an XML instantiation of the messages in the ADM are
specified in section 4 of this document. The APM and AEM are specified in reference [4].

.. _orbit_data_messages_xml:

2.3 ORBIT DATA MESSAGES
-----------------------

Orbit Data Messages comprise four message types used to convey trajectory information: the
Orbit Parameter Message (OPM), Orbit Mean Elements Message (OMM), Orbit Ephemeris
Message (OEM), and Orbit Comprehensive Message (OCM). The OPM consists of a single
state vector at a given time that can be propagated to generate the trajectory of the spacecraft;
specifications of maneuvers are optional. Like the OPM, the OMM also represents an orbit
state, but it is calculated on the basis of mean orbital elements instead of osculating elements
(there are other differences as well). The OEM represents a history/forecast of state vectors
that can be interpolated to obtain the state of the spacecraft at times other than those
explicitly specified in the message. The OCM aggregates and extends OPM, OMM, and
OEM content in a single comprehensive hybrid message and includes a great deal of
additional information about the spacecraft and its environment. Instructions for creating an
XML instantiation of the messages in the ODM version 3 are contained in the ODM document
itself. The OPM, OMM, OEM, and OCM are specified in reference [5].

.. _tracking_data_message_xml:

2.4 TRACKING DATA MESSAGE
-------------------------

The Tracking Data Message is a single message type for use in exchanging spacecraft tracking
data between space agencies. Such exchanges are used for distributing tracking data output from
interagency cross supports in which spacecraft missions managed by one agency are tracked
from a ground station managed by a second agency. Additionally, the ability to transfer tracking
data between space agencies facilitates the allocation of tracking sessions to alternate antenna
resources and increases the ability of space agencies to tolerate availability issues with their
primary antennas. The TDM supports commonly used ground-based radiometric data types,
spacecraft-to-spacecraft Doppler and range, and ancillary information needed to calculate the
measurement residuals. Instructions for creating an XML instantiation of the TDM version 2 are
contained in the TDM document itself. The TDM is specified in reference [6].

.. _conjunction_data_message_xml:

2.5 CONJUNCTION DATA MESSAGE
----------------------------

The Conjunction Data Message specifies a single message type for use in exchanging
spacecraft conjunction information between originators of conjunction assessments and
satellite owner/operators and other authorized parties. Such exchanges provide critical
information to satellite owner/operators to enable timely collision-avoidance decisions. The
CDM is applicable to satellite operations in all environments in which close approaches and
collisions among satellites are concerns. Instructions for creating an XML instantiation of the
CDM are contained in the CDM document itself. The CDM is specified in reference [7].

.. _re-entry_data_message_xml:

2.6 RE-ENTRY DATA MESSAGE
-------------------------

The Re-entry Data Message specifies a single message type for use in exchanging spacecraft re-
entry information between space situational-awareness data providers and recipients such as
satellite operators, civil protection authorities, and/or aviation authorities. The RDM contains
information about a single re-entry event, including identification of the re-entering object;
basic re-entry information such as remaining orbital lifetime; whether the re-entry is controlled
or not, and which celestial body the object is orbiting; and more complex re-entry information
such as re-entry and impact windows, impact location and probabilities, state vector, object
properties, the orbit determination process, and observations used to predict the re-entry. The
information is used by recipients to assess the re-entry risk and plan any needed mitigation
measures. The RDM is not limited to man-made objects re-entering the Earth’s atmosphere. It
could be used for any entry/impact event by specifying the appropriate center name, reference
frame, and object type. Instructions for creating an XML instantiation of the RDM are
contained in the RDM document itself. The RDM is specified in reference [8].
.. _structure_xml:

3 BASIC STRUCTURE OF THE NDM/XML SCHEMA SET
===========================================

.. _navigation_data_messages_and_schema_set_xml:

3.1 NAVIGATION DATA MESSAGES AND THE ASSOCIATED SCHEMA SET
------------------------------------------------------------

3.1.1 The basic element in the NDM/XML is an NDM. An NDM shall consist of at least
one of the messages documented in references [4]-[8].

3.1.2 The NDM/XML schema set shall consist of a schema for the most current Blue Book
issue of each individual message type (see references [4]–[8]), an ‘NDM combined
instantiation’ schema (see 4.11), a namespace schema, a master validator schema, and a
schema containing elements common to more than one Navigation Working Group (see
table 3-1).

3.1.3 The NDM/XML schema set shall be available on a CCSDS resource that is internet
accessible.

.. note::
   The NDM/XML schema set is currently available at:
   https://sanaregistry.org/r/ndmxml for links to the two schema subsets below:

   - https://sanaregistry.org/r/ndmxml_unqualified/[schemaName] for schemas
     with the attribute ‘elementFormDefault="unqualified"’;
   - https://sanaregistry.org/r/ndmxml_qualified/[schemaName] for schemas
     with the attribute ‘elementFormDefault="qualified"’.

3.1.4 For schemas directly associated with one of the NDM message types, the components
of [schemaName] shall be
``ndmxml-[ndmxmlVersionNumber]-[messageType]-[blueBookNumber].xsd``
where

- [ndmxmlVersionNumber] is formed via the string M.m.X, where ‘M’ is the Blue
  Book number of this document (i.e., the NDM/XML standard), ‘m’ is a minor
  revision number of the NDM/XML Blue Book (usually ‘0’), and ‘X’ is a schema
  revision indicator (‘0’ for the initial version, then {‘A’, ‘B’, ‘C’,...} for successive
  revisions (see the SANA Registry for the current NDM/XML version number);
- [messageType] is one of the individual message types described in references [4]–[8]
  (e.g., APM, CDM, OPM, RDM, TDM, etc.);
- [blueBookNumber] is the most current Blue Book version corresponding to the
  message (e.g., ‘1.0’ for the AEM and APM).

.. note::
   ¹ In this document, ‘Blue Book version’ is synonymous with ‘Blue Book issue’.

.. admonition:: NOTES

   1. There are several test files and example NDM/XML instantiations on the CCSDS
      Navigation Working Group Collaborative Work Environment (CWE) Web site
      https://cwe.ccsds.org/moims/docs/MOIMS-NAV/Test-Messages/XML.
   2. The following table illustrates the naming convention in the names of the NDM/XML
      schema set. The ‘Blue Book Supported’ column indicates the message and respective
      Blue Book to which the schema applies; an asterisk next to the schema name indicates
      that instructions for creating an instantiation are incorporated in this Blue Book.
   3. The naming convention for the auxiliary schemas not directly associated with an
      NDM (common, master, namespace) is similar to the convention for the message-
      related schemas, but not identical.
   4. As noted above, each schema is available with ‘elementFormDefault="qualified"’ and
      ‘elementFormDefault="unqualified"’.

.. list-table:: Table 3-1: The NDM/XML Schema Set
   :widths: 40 40 20
   :header-rows: 1

   * - Schema
     - Blue Book Supported
     - Root Tag
   * - ndmxml-3.0.x-aem-1.0.xsd *
     - ADM Attitude Ephemeris Message (reference [4])
     - <aem>
   * - ndmxml-3.0.x-apm-1.0.xsd *
     - ADM Attitude Parameter Message (reference [4])
     - <apm>
   * - ndmxml-3.0.x-cdm-1.0.xsd
     - CDM Conjunction Data Message (reference [7])
     - <cdm>
   * - ndmxml-3.0.x-common-3.n.xsd
     - Constructs used in more than one NDM schema (‘n’ is a sequence number 0, 1, 2, …). Supports all references listed in references [4]–[8].
     - N/A
   * - ndmxml-3.0.x-master-3.n.xsd
     - NDM/XML master schema, used to validate all instantiations, import NDM/XML namespace, and declare all schema set root elements. Supports all references listed in references [4]–[8].
     - N/A
   * - ndmxml-3.0.x-namespace-3.n.xsd
     - Includes each element of the NDM/XML schema set. Supports all references listed in references [4]–[8].
     - N/A
   * - ndmxml-3.0.x-ndm-3.0.xsd
     - NDM combined instantiation schema (see 4.11)
     - <ndm>
   * - ndmxml-3.0.x-ocm-3.0.xsd
     - ODM Orbit Comprehensive Message (reference [5])
     - <ocm>
   * - ndmxml-3.0.x-oem-3.0.xsd
     - ODM Orbit Ephemeris Message (reference [5])
     - <oem>
   * - ndmxml-3.0.x-omm-3.0.xsd
     - ODM Orbit Mean Elements Message (reference [5])
     - <omm>
   * - ndmxml-3.0.x-opm-3.0.xsd
     - ODM Orbit Parameter Message (reference [5])
     - <opm>
   * - ndmxml-3.0.x-rdm-1.0.xsd
     - RDM Re-entry Data Message (reference [8])
     - <rdm>
   * - ndmxml-3.0.x-tdm-2.0.xsd
     - TDM Tracking Data Message (reference [6])
     - <tdm>

.. _basic_structure_xml:

3.2 NDM/XML BASIC STRUCTURE
---------------------------

3.2.1 Each constituent NDM (see messages specified in references [4]–[8]) shall consist of
a <header> and a <body>.

3.2.2 The NDM body shall consist of one or more <segment> constructs, depending
upon the message type.

3.2.3 Each <segment> shall consist of a <metadata>/<data> pair.

.. note::
   The <body> and <segment> constructs are not explicitly specified in some of
   the constituent message documents (see references [4], [5], [6]); however, they
   are logically implied, and are necessary in order to enforce the strict ordering of
   metadata and data sections (see section 4).

.. _substructure_1_xml:

3.3 SUBSTRUCTURE 1: APM, OCM, OMM, OPM, RDM
---------------------------------------------

The body of several NDMs (e.g., APM, OCM, OMM, OPM, and RDM) shall consist of a
single segment, as shown in figure 3-1. Generally these NDMs describe a single state; the
OCM varies from this pattern.

.. note::
   In Substructure 1 the <segment> tag is not structurally necessary; however, it
   is present for symmetry with Substructure 2 in the ‘body’ of the message,
   enabling re-use of some schema data types.

.. figure:: /images/ndm_xml_substructure_1.png
   :align: center

   Figure 3-1: NDM/XML Substructure 1 (Single Segment)

.. _substructure_2_xml:

3.4 SUBSTRUCTURE 2: AEM, OEM, TDM, AND CDM
--------------------------------------------

3.4.1 The body of several NDMs (e.g., AEM, OEM, and TDM) shall consist of one or more
segments, as shown in figure 3-2. Generally, these messages describe multiple states or
tracking data types.

3.4.2 The CDM is a variant of Substructure 2. It contains exactly two segments, and includes
a unique ‘Relative Metadata Section’ prior to the first segment (see figure 3-3).

.. note::
   The alternation of associated metadata and data sections is the structural element
   that necessitates the notion of the segment.

.. figure:: /images/ndm_xml_substructure_2.png
   :align: center

   Figure 3-2: NDM/XML Substructure 2 (Possible Segment Multiplicity)

.. figure:: /images/variant_of_substructure_2_for_cdm.png
   :align: center

   Figure 3-3: Variant of Substructure 2 for CDM

.. _tags_xml:

3.5 NDM/XML TAGS
----------------

3.5.1 Within the structure and substructures described in 3.2 through 3.4, the individual
NDM/XML tags specific to the various message types shall be defined.

3.5.2 NDM/XML tag names shall be identical to the keywords in the reference documents
for the KVN representation, with exceptions as noted below and in section 4.

.. note::
   There are three exceptions for which there is not a strict correspondence between
   KVN keywords in a reference document and NDM/XML tags:

   a) the ‘CCSDS_xxx_VERS’ keyword that is present in each document;
   b) keywords associated with rotations in the ADM (see 4.8.2); and
   c) keywords associated with user-defined parameters in some of the documents
      (see 4.10).

   In the first two exceptions, the KVN keywords appear as XML attributes rather
   than as XML elements. In the last case, the KVN keywords appear as a
   combination of XML elements and attributes. The details of these special cases
   are described in section 4 of this document, which contains instructions for
   coding instantiations of the specific messages. The use of
   <userDefinedParameters> is defined in 4.10.

3.5.3 An NDM/XML tag shall be all uppercase if it corresponds directly to a KVN
keyword in one of the reference documents.

3.5.4 References [4]-[8] shall define the contents of the specific KVN keyword related
NDM/XML tags.

3.5.5 NDM/XML keywords that do not correspond directly to a KVN keyword in one of
the special reference documents (references [4]–[8]) shall be in ‘lowerCamelCase’.

.. _text_values_xml:

3.6 NDM/XML TEXT VALUES
-----------------------

3.6.1 Text values in NDM/XML instantiations (i.e., the values between the element begin
and end tags and the values between opening and closing quotes for XML attributes) shall
consist of either all uppercase or all lowercase characters, with exceptions as noted in 3.6.2.

.. note::
   In some of the KVN format NDMs, it is stated that constructing text values using
   mixed case is permitted, and that case is not significant. However, this
   complicates checking for valid values in an XML schema. For example, if the
   word ‘cat’ is expected for a text value, but case is not significant, then the schema
   necessarily will allow the values ‘cat’, ‘Cat’, ‘cAt’, ‘caT’, ‘CAt’, ‘CaT’, ‘cAT’,
   and ‘CAT’. This is a 2ⁿ problem that is not feasible in schema coding for
   enumerations longer than a few characters. Thus in the NDM/XML schema set,
   regardless of whether or not mixed case is allowed in the underlying KVN
   standard, the requirement associated with this note is established.

3.6.2 An exception is made for values between the <COMMENT> and </COMMENT> tags,
which may be in any case desired by the user.
.. _constructing_instance_xml:

4 CONSTRUCTING AN NDM/XML INSTANCE
==================================

.. _overview_constructing_xml:

4.1 OVERVIEW
------------

This section provides more detailed instructions for the user on how to create an XML message
based on one of the ASCII-text KVN-formatted messages described in references [4]-[8]. In
particular, with the current exception of the Attitude Data Messages (reference [4]), the
instructions for creating XML formatted messages are described in references [5]-[8].

.. _version_xml:

4.2 XML VERSION
---------------

The first line of each instantiation shall specify the XML version, exactly as follows:
``<?xml version="1.0" encoding="UTF-8"?>``

.. _beginning_instantiation_xml:

4.3 BEGINNING THE INSTANTIATION: ROOT ELEMENT TAG
-------------------------------------------------

4.3.1 Each instantiation shall have a ‘root element tag’ that identifies the message type and
other information specific to the NDM/XML.

.. note:: ‘Other information’ includes things such as where to find the applicable schema,
   required attributes, etc.

4.3.2 The root element tag in an NDM/XML instantiation shall be one of those listed in the
‘Root Tag’ column of table 3-1.

4.3.3 The XML Schema Instance namespace attribute must appear in the root element tag
of all NDM/XML instantiations, exactly as shown:
``xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"``

.. note:: ‘https:’ is not a valid value in the above string, as it is the name of a namespace,
   not the name of an internet protocol.

4.3.4 The NDM/XML namespace must next be coded, exactly as shown:
``xmlns:ndm="urn:ccsds:schema:ndmxml"``

4.3.5 The value that follows the ‘xmlns:’ in the NDM/XML name space (‘ndm’ in this
case) is a prefix that must be used on every XML tag if it is desired to create an instantiation
in an environment that requires ‘elementFormDefault="qualified"’.

.. note:: The NDM/XML schemas for ‘elementFormDefault="qualified"’ and
   ‘elementFormDefault="unqualified"’ are identical with the exception of the
   value for the elementFormDefault parameter.

4.3.6 If it is desired to validate an instantiation against the CCSDS Web-based schema, one
of the options for the xsi:noNamespaceSchemaLocation attribute must be coded as a single
string of non-blank characters, with no line breaks:
``xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-3.0.X-master-3.0.xsd"`` (if ‘elementFormDefault="unqualified"’ is desired)
``xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_qualified/ndmxml-3.0.X-master-3.0.xsd"`` (if ‘elementFormDefault="qualified"’ is desired)

.. admonition:: NOTES

   1. The value associated with the xsi:noNamespaceSchemaLocation attribute shown in
      this document is too long to appear on a single line.
   2. In the schema name, the ‘X’ in ‘3.0.X’ is the most current revision of the NDM/XML
      schema set, which can be determined via the SANA Registry. For the initial schema
      set, X = 0 (i.e., 3.0.0 is the initial schema set).

4.3.7 For use in a local operations environment, the NDM/XML schema set may be
downloaded from the CCSDS Web site to a local server that meets local requirements for
operations robustness.

4.3.8 If a local version is used, the value associated with the xsi:noNamespaceSchemaLocation
attribute must be changed to a URL that is accessible to the local server.

4.3.9 There are two attributes that are required in the root element tag of an NDM/XML
single message instantiation, specifically, the CCSDS_xxx_VERS keyword that is also part
of the standard KVN header, and the Blue Book issue number.

4.3.10 The CCSDS_xxx_VERS keyword shall be supplied via the ‘id’ attribute of the root
element tag as noted in table 3-1. The value ‘xxx’ in the ‘id’ attribute must be in all capital letters.

4.3.11 The issue number of the Blue Book to which the schema applies shall be supplied via
the ‘version’ attribute.

.. note:: The following example root element tag for an OPM instantiation combines
   all the directions in the preceding several subsections for both ‘unqualified’
   and ‘qualified’ elementFormDefault:

   .. code-block:: xml

      <?xml version="1.0" encoding="UTF-8"?>
      <opm
      xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
      xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-3.0.0-master-3.0.xsd"
      id="CCSDS_OPM_VERS" version="3.0">

      <?xml version="1.0" encoding="UTF-8"?>
      <ndm:opm
      xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
      xmlns:ndm="urn:ccsds:schema:ndmxml"
      xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_qualified/ndmxml-3.0.0-master-3.0.xsd"
      id="CCSDS_OPM_VERS" version="3.0">

.. _header_section_xml:

4.4 THE STANDARD NDM/XML HEADER SECTION
---------------------------------------

4.4.1 The NDMs shall share a standard header format, with tags <header> and
</header>.

4.4.2 Immediately following the <header> tag, the message may have any number of
<COMMENT></COMMENT> tag pairs.

4.4.3 The standard NDM header shall contain the <CREATION_DATE> and the
<ORIGINATOR> tags.

.. note:: The rules for these keywords are specified in references [4]–[8]. An example
   <header> section is shown immediately below for both ‘unqualified’ and
   ‘qualified’ elementFormDefault. In some of the recent publications, an
   optional ‘MESSAGE_ID’ keyword has been included in the message header;
   details are in the specific reference.

   .. code-block:: xml

      <header>
      <COMMENT>This is the common NDM/XML header</COMMENT>
      <COMMENT>I can put as many comments here as I want,</COMMENT>
      <COMMENT>including none.</COMMENT>
      <CREATION_DATE>2004-281T17:26:06</CREATION_DATE>
      <ORIGINATOR>AGENCY-X</ORIGINATOR>
      </header>

      <ndm:header>
      <ndm:COMMENT>This is the common NDM/XML header</ndm:COMMENT>
      <ndm:COMMENT>I can put as many comments here as I want,</ndm:COMMENT>
      <ndm:COMMENT>including none.</ndm:COMMENT>
      <ndm:CREATION_DATE>2004-281T17:26:06</ndm:CREATION_DATE>
      <ndm:ORIGINATOR>AGENCY-X</ndm:ORIGINATOR>
      </ndm:header>

.. _body_section_xml:

4.5 THE NDM BODY SECTION
------------------------

4.5.1 After coding the <header>, the instantiation must include a <body></body> tag pair.

4.5.2 Inside the <body></body> tag pair must appear at least one
<segment></segment> tag pair.

4.5.3 Each segment must be made up of one or more <metadata></metadata> and
<data></data> tag pairs.

.. _metadata_section_xml:

4.6 THE NDM METADATA SECTION
----------------------------

4.6.1 All NDMs must have a metadata section.

4.6.2 The metadata section shall be set off by the <metadata></metadata> tag
combination.

4.6.3 Between the <metadata> and </metadata> tags, the keywords shall be the
same as those in the metadata sections in references [4]–[8].

.. _data_section_xml:

4.7 THE NDM DATA SECTION
------------------------

4.7.1 All NDMs must have a data section.

4.7.2 The data section shall follow the metadata section and shall be set off by the
<data></data> tag combination.

4.7.3 Between the <data> and </data> tags, the keywords shall be the same as those in
the data sections in the references [4]–[8], with exceptions as noted in the following
subsections that discuss creating instantiations of the specific messages.

.. _creating_aem_instantiation_xml:

4.8 CREATING AN AEM INSTANTIATION
---------------------------------

.. _general_aem_xml:

4.8.1 GENERAL
^^^^^^^^^^^^^

4.8.1.1 An AEM instantiation shall be delimited with the <aem></aem> root element tags
using the standard attributes documented in 4.3.

.. note:: Figures G-1 and G-2 in annex G provide example AEM instantiations.

4.8.1.2 The final attributes of the <aem> tag shall be ‘id’ and ‘version’.

4.8.1.3 The ‘id’ attribute shall be ‘id="CCSDS_AEM_VERS"’.

4.8.1.4 The ‘version’ attribute for the version of the AEM described in reference [4] shall
be ‘version="1.0"’.

4.8.1.5 The standard NDM header shall follow the <aem> tag.

4.8.1.6 The AEM <body> shall consist of one or more <segment> constructs (see
figure 3-2).

4.8.1.7 Each <segment> shall consist of a <metadata> section and a <data> section.

4.8.1.8 The keywords in the <metadata> and <data> sections shall be those specified
in reference [4].

.. note:: The rules for including any of the keyword tags in the instantiation are the same
   as those specified for the AEM in reference [4].

4.8.1.9 Tags for keywords specified in reference [4] shall be all uppercase as in reference [4].

.. _special_tags_aem_body_xml:

4.8.2 SPECIAL TAGS IN THE AEM BODY
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. note:: In addition to the AEM keywords specified in reference [4], there are several
   special tags associated with the AEM body as described in the next few
   subsections.

4.8.2.1 The <attitudeState> tag shall be used to encapsulate the keywords associated
with the structure of one of the attitude ephemeris data line types.

4.8.2.2 The NDM/XML tags used within the <attitudeState> structure shall be
drawn from table 4-1.

.. list-table:: Table 4-1: Special Tags Used in the AEM Body
   :widths: 50 50
   :header-rows: 1

   * - AEM ‘ATTITUDE_TYPE’ Metadata Value
     - Associated NDM/XML Tag in the <attitudeState>
   * - QUATERNION
     - <quaternionState>
   * - QUATERNION/DERIVATIVE
     - <quaternionDerivative>
   * - QUATERNION/RATE
     - <quaternionEulerRate>
   * - EULER_ANGLE
     - <eulerAngle>
   * - EULER_ANGLE/RATE
     - <eulerAngleRate>
   * - SPIN
     - <spin>
   * - SPIN/NUTATION
     - <spinNutation>

4.8.2.3 Between the begin tag and end tag (e.g., between <quaternionState> and
</quaternionState>), the user shall place the values required by the specific
ephemeris data line type as specified in reference [4].

4.8.2.4 In the XML representation of the AEM, the components of the
<attitudeState> ephemeris data line must be represented with keywords (i.e., a tag).

4.8.2.5 The <attitudeState> keywords shall be the same as those defined for the
same construct in the APM.

.. note:: In the KVN representations of the ephemeris data lines, keywords are not used.
   Rather, the components of the ephemeris data line appear in an order defined by
   the specific ephemeris data line type.

4.8.2.6 The <rotation*> constructs shall be used to encapsulate the keywords
associated with the structure of one of the rotation sequences.

.. note:: Some <attitudeState> entries include angles only, or rates only, or both
   angles and rates.

4.8.2.7 The NDM/XML tags used within the <rotation*> structure shall be drawn from
table 4-2.

.. list-table:: Table 4-2: AEM Rotation Tags
   :widths: 50 50
   :header-rows: 1

   * - <attitudeState> Tag
     - Associated Rotation Tag in the <attitudeState>
   * - <quaternionEulerRate>
     - <rotationRates>
   * - <eulerAngle>
     - <rotationAngles>
   * - <eulerAngleRate>
     - <rotationAngles> followed immediately by <rotationRates>

4.8.2.8 The <rotationAngles> and <rotationRates> elements shall be
composed of three tags: <rotation1>, <rotation2>, and <rotation3>.

.. note:: Depending on whether angles or rates are being described, these <rotationi>
   (i=1,2,3) keywords have different attributes.

4.8.2.9 For <rotationi> tags in the <rotationAngles> element, the attributes shall
be ‘angle=’ and ‘units="deg"’.

4.8.2.10 The ‘angle’ attribute must be coded on the <rotationi> tag.

4.8.2.11 The ‘units’ attribute may be coded on the <rotationi> tag.

4.8.2.12 The value associated with the ‘angle’ attribute must be chosen from the values
‘X_ANGLE’, ‘Y_ANGLE’, ‘Z_ANGLE’.

.. note:: ‘X_ANGLE’, ‘Y_ANGLE’, and ‘Z_ANGLE’ are keywords from the KVN AEM.

4.8.2.13 For <rotationi> tags in the <rotationRates> element, the attributes shall be
‘rate=’ and ‘units="deg/s"’.

4.8.2.14 The ‘rate’ attribute must be coded on the <rotationi> tag.

4.8.2.15 The ‘units’ attribute may be coded on the <rotationi> tag.

4.8.2.16 The value associated with the ‘rate’ attribute must be chosen from the values
‘X_RATE’, ‘Y_RATE’, ‘Z_RATE’.

.. note:: ‘X_RATE’, ‘Y_RATE’, and ‘Z_RATE’ are keywords from the KVN AEM.

4.8.3 DISCUSSION
^^^^^^^^^^^^^^^^

This non-normative subsection discusses and provides examples of the use of quaternion tags
in the AEM.

The XML representations of quaternions in the ADM constituent messages share a common
quaternion definition. However, there are some differences in those definitions in the
underlying KVN definitions of the APM and AEM. As in the KVN representation of the
quaternion, it is possible to code the tags for the individual components of the quaternion (Q1,
Q2, Q3, QC) in either of the standard orders (i.e., scalar component first or last). The following
examples are meant to illustrate the standard for representing quaternions in the AEM.

Here is an example AEM quaternion for a ‘QUATERNION’ ephemeris data line:

.. code-block:: xml

   <attitudeState>
     <quaternionState>
       <EPOCH>2004-100T00:00:00</EPOCH>
       <quaternion>
         <Q1>0.00005</Q1>
         <Q2>0.87543</Q2>
         <Q3>0.40949</Q3>
         <QC>0.25678</QC>
       </quaternion>
     </quaternionState>
   </attitudeState>

Here is an example AEM quaternion for a ‘QUATERNION/DERIVATIVE’ ephemeris data
line:

.. code-block:: xml

   <attitudeState>
     <quaternionDerivative>
       <EPOCH>2004-100T00:00:00</EPOCH>
       <quaternion>
         <Q1>0.00005</Q1>
         <Q2>0.87543</Q2>
         <Q3>0.40949</Q3>
         <QC>0.25678</QC>
       </quaternion>
       <quaternionRate>
         <Q1_DOT>0.002</Q1_DOT>
         <Q2_DOT>0.003</Q2_DOT>
         <Q3_DOT>0.004</Q3_DOT>
         <QC_DOT>0.001</QC_DOT>
       </quaternionRate>
     </quaternionDerivative>
   </attitudeState>

Here is an example AEM quaternion for a ‘QUATERNION/RATE’ ephemeris data line:

.. code-block:: xml

   <attitudeState>
     <quaternionEulerRate>
       <EPOCH>2004-100T00:00:00</EPOCH>
       <quaternion>
         <Q1>0.00005</Q1>
         <Q2>0.87543</Q2>
         <Q3>0.40949</Q3>
         <QC>0.25678</QC>
       </quaternion>
       <rotationRates>
         <rotation1 rate="X_RATE">1.0</rotation1>
         <rotation2 rate="Y_RATE">1.1</rotation2>
         <rotation3 rate="Z_RATE">1.2</rotation3>
       </rotationRates>
     </quaternionEulerRate>
   </attitudeState>

.. _creating_apm_instantiation_xml:

4.9 CREATING AN APM INSTANTIATION
---------------------------------

4.9.1 An APM instantiation shall be delimited by the <apm></apm> root element tags
using the standard attributes documented in 4.3.

.. note:: Figure G-3 in annex G provides an example APM instantiation.

4.9.2 The final attributes of the <apm> tag shall be ‘id’ and ‘version’.

4.9.3 The ‘id’ attribute shall be ‘id="CCSDS_APM_VERS"’.

4.9.4 The ‘version’ attribute for the version of the APM described in reference [4] shall be
‘version="1.0"’.

4.9.5 The standard NDM header shall follow the <apm> tag.

4.9.6 The APM <body> shall consist of a single <segment> (see figure 3-1).

4.9.7 The segment shall consist of a <metadata> section and a <data> section.

4.9.8 The keywords in the <metadata> and <data> sections shall be those specified in
reference [4].

.. note:: The rules for including any of the keyword tags in the instantiation are the same
   as those specified for the APM in reference [4].

4.9.9 Tags for keywords specified in reference [4] shall be all uppercase as in reference [4].

4.9.10 Several of the NDM/XML APM keywords may have a unit attribute, if desired by the
APM producer.

4.9.11 In all cases, the units shall match those defined in reference [4].

4.9.12 Table 4-3 specifies the keyword tags for which units may be specified:

.. list-table:: Table 4-3: APM Tags with Units
   :widths: 25 25 50
   :header-rows: 1

   * - Keyword
     - Units
     - Example
   * - Q1_DOT
     - 1/s
     - <Q1_DOT units="1/s">numeric-value</Q1_DOT>
   * - Q2_DOT
     - 1/s
     - <Q2_DOT units="1/s">numeric-value</Q2_DOT>
   * - Q3_DOT
     - 1/s
     - <Q3_DOT units="1/s">numeric-value</Q3_DOT>
   * - QC_DOT
     - 1/s
     - <QC_DOT units="1/s">numeric-value</QC_DOT>
   * - SPIN_ALPHA
     - deg
     - <SPIN_ALPHA units="deg">numeric-value</SPIN_ALPHA>
   * - SPIN_DELTA
     - deg
     - <SPIN_DELTA units="deg">numeric-value</SPIN_DELTA>
   * - SPIN_ANGLE
     - deg
     - <SPIN_ANGLE units="deg">numeric-value</SPIN_ANGLE>
   * - SPIN_ANGLE_VEL
     - deg/s
     - <SPIN_ANGLE_VEL units="deg/s">numeric-value</SPIN_ANGLE_VEL>
   * - NUTATION
     - deg
     - <NUTATION units="deg">numeric-value</NUTATION>
   * - NUTATION_PER
     - s
     - <NUTATION_PER units="s">numeric-value</NUTATION_PER>
   * - NUTATION_PHASE
     - deg
     - <NUTATION_PHASE units="deg">numeric-value</NUTATION_PHASE>
   * - I11
     - kg*m**2
     - <I11 units="kg*m**2">numeric-value</I11>
   * - I22
     - kg*m**2
     - <I22 units="kg*m**2">numeric-value</I22>
   * - I33
     - kg*m**2
     - <I33 units="kg*m**2">numeric-value</I33>
   * - I12
     - kg*m**2
     - <I12 units="kg*m**2">numeric-value</I12>
   * - I13
     - kg*m**2
     - <I13 units="kg*m**2">numeric-value</I13>
   * - I23
     - kg*m**2
     - <I23 units="kg*m**2">numeric-value</I23>
   * - MAN_DURATION
     - s
     - <MAN_DURATION units="s">numeric-value</MAN_DURATION>
   * - MAN_TOR_1
     - N*m
     - <MAN_TOR_1 units="N*m">numeric-value</MAN_TOR_1>
   * - MAN_TOR_2
     - N*m
     - <MAN_TOR_2 units="N*m">numeric-value</MAN_TOR_2>
   * - MAN_TOR_3
     - N*m
     - <MAN_TOR_3 units="N*m">numeric-value</MAN_TOR_3>

4.9.13 SPECIAL TAGS IN THE APM BODY
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. note:: In addition to the APM keywords specified in reference [4], there are several
   special tags associated with the APM body as described in the next few
   subsections. The information content in the APM is separated into constructs
   described in reference [4] as ‘logical blocks’. Special tags in the APM are used
   to encapsulate the information in the logical blocks of the APM.

4.9.13.1 The NDM/XML tags used to delimit the logical blocks of the APM shall be drawn
from table 4-4.

.. list-table:: Table 4-4: Special Tags Used in the APM Body
   :widths: 50 50
   :header-rows: 1

   * - APM Logical Block
     - Associated NDM/XML APM Tag
   * - Quaternion
     - <quaternionState>, <quaternion>, <quaternionRate>
   * - Euler Elements / Three Axis Stabilized
     - <eulerElementsThree>
   * - Euler Elements / Spin Stabilized
     - <eulerElementsSpin>
   * - Spacecraft Parameters
     - <spacecraftParameters>
   * - Maneuver Parameters
     - <maneuverParameters>

4.9.13.2 Between the begin tag and end tag (e.g., between <spacecraftParameters>
and </spacecraftParameters>), the user shall place the keywords required by the
specific logical block as specified in reference [4].

4.9.14 DISCUSSION
^^^^^^^^^^^^^^^^

This non-normative subsection discusses and provides examples of the use of quaternion tags
in the APM.

The XML representations of quaternions in the ADM constituent messages share a common
quaternion definition. However, there are some differences in those definitions in the
underlying KVN definitions of the APM and AEM. As in the KVN representation of the
quaternion, it is possible to code the tags for the individual components of the quaternion
(Q1, Q2, Q3, QC) in either of the standard orders (i.e., scalar component first or last). The
following examples are meant to illustrate the standard for representing quaternions in the
APM.

Here is an example APM quaternion construct:

.. code-block:: xml

   <quaternionState>
     <EPOCH>2004-100T00:00:00Z</EPOCH>
     <Q_FRAME_A>ICRF</Q_FRAME_A>
     <Q_FRAME_B>ICRF</Q_FRAME_B>
     <Q_DIR>B2A</Q_DIR>
     <quaternion>
       <Q1>0.00005</Q1>
       <Q2>0.87543</Q2>
       <Q3>0.40949</Q3>
       <QC>0.25678</QC>
     </quaternion>
   </quaternionState>

Here is an example APM quaternion construct with the optional derivative:

.. code-block:: xml

   <quaternionState>
     <EPOCH>2004-100T00:00:00Z</EPOCH>
     <Q_FRAME_A>ICRF</Q_FRAME_A>
     <Q_FRAME_B>ICRF</Q_FRAME_B>
     <Q_DIR>A2B</Q_DIR>
     <quaternion>
       <Q1>0.00005</Q1>
       <Q2>0.87543</Q2>
       <Q3>0.40949</Q3>
       <QC>0.25678</QC>
     </quaternion>
     <quaternionRate>
       <Q1_DOT>0.002</Q1_DOT>
       <Q2_DOT>0.003</Q2_DOT>
       <Q3_DOT>0.004</Q3_DOT>
       <QC_DOT>0.001</QC_DOT>
     </quaternionRate>
   </quaternionState>

4.9.15 DISCUSSION
^^^^^^^^^^^^^^^^

This non-normative subsection discusses and provides examples of the use of rotation tags in
the APM.

The APM includes two rotation-related constructs that are used in conjunction with the
<eulerElementsThree> tag. The rotation combinations are complicated by the fact
that some rotation sequences are specified with more than one rotation about the same axis
(e.g., a ‘131’ rotation, in which the first rotation is about the x-axis, second about the z-axis,
and the final rotation again about the x-axis). The rotation constructs are used to encapsulate
the keywords associated with the structure of one of the rotation sequences. As in the KVN
APM, angles can be specified without rates, rates can be specified without angles, or both
angles and rates can be specified. The <rotationAngles> and <rotationRates>
elements are composed of three tags: <rotation1>, <rotation2>, and
<rotation3>. Depending on whether angles or rates are being described, these
<rotationi> (i=1,2,3) keywords have different attributes.

For example, the following shows rotation angles for a 321 rotation sequence:

.. code-block:: xml

   <rotationAngles>
     <rotation1 angle="Z_ANGLE">1.234</rotation1>
     <rotation2 angle="Y_ANGLE">5.678</rotation2>
     <rotation3 angle="X_ANGLE">9.1011</rotation3>
   </rotationAngles>

For example, the following shows rotation rates for a 321 rotation sequence:

.. code-block:: xml

   <rotationRates>
     <rotation1 rate="Z_RATE" units="deg/s">1.234</rotation1>
     <rotation2 rate="Y_RATE" units="deg/s">5.678</rotation2>
     <rotation3 rate="X_RATE" units="deg/s">9.1011</rotation3>
   </rotationRates>

.. _user_defined_parameters_xml:

4.10 USER DEFINED PARAMETERS
---------------------------

.. note::
   User-defined parameters have been added to some of the Navigation Data
   Messages (OPM, OMM, OCM (reference [5]); and RDM (reference [8]). As
   other Navigation Data Message standards are updated, it is likely that the ability
   to include user-defined parameters will be added to them. These parameters are
   situation specific and are not standardized. Accordingly, the use of user-defined
   parameters is not encouraged. Because these parameters are not known to the
   schema, there is only one very broad keyword offered in the NDM/XML:
   <USER_DEFINED>.

4.10.1 GENERAL
^^^^^^^^^^^^^^

4.10.1.1 User-defined parameters, if utilized, must be specified in ICDs between the
exchange participants.

4.10.1.2 User-defined parameters shall only appear in instantiations of the navigation data
messages which have defined them for the KVN format.

4.10.1.3 User-defined parameters shall appear in a logical block that is offset with the tag set
<userDefinedParameters></userDefinedParameters>.

4.10.1.4 Specific user-defined parameters in an NDM shall utilize the tag
<USER_DEFINED>.

4.10.1.5 Following the <userDefinedParameters> tag, any number and order of
<USER_DEFINED> tags may appear.

4.10.1.6 All information about the user-defined parameters shall be conveyed via one
attribute of the <USER_DEFINED> tag (the attribute ‘parameter’) and the
<USER_DEFINED> element value (which may include the applicable units).

4.10.1.7 In the NDM/XML, the variable-length value associated with the parameter attribute
shall be the string following ‘USER_DEFINED_’ in the associated KVN keyword.

4.10.1.8 The data type for the user-defined value shall be ‘xsd:string’, even if the actual
user-defined parameter has a numeric value.

4.10.2 DISCUSSION
^^^^^^^^^^^^^^^^^

For example, the following KVN parameters might appear in an OMM or OPM:

USER_DEFINED_ATMOSPHERE_MODEL = MSISE90
USER_DEFINED_C3 = 29.376 [km**2/s**2]
USER_DEFINED_EARTH_RADIUS = 6378.1 [km]
USER_DEFINED_3RD_BODY_PERTURBATION = JUPITER

These parameters would appear in an NDM/XML representation as:

.. code-block:: xml

   <userDefinedParameters>
     <USER_DEFINED parameter="ATMOSPHERE_MODEL">MSISE90</USER_DEFINED>
     <USER_DEFINED parameter="C3">29.376 [km**2/s**2]</USER_DEFINED>
     <USER_DEFINED parameter="EARTH_RADIUS">6378.1 [km]</USER_DEFINED>
     <USER_DEFINED parameter="3RD_BODY_PERTURBATION">JUPITER</USER_DEFINED>
   </userDefinedParameters>

.. _creating_combined_instantiation_xml:

4.11 CREATING AN NDM COMBINED INSTANTIATION
-------------------------------------------

4.11.1 OVERVIEW
^^^^^^^^^^^^^^^

4.11.2 It is possible to create an XML instance that incorporates any number of NDM
messages from references listed in references [4]–[8] in a logical suite called an ‘NDM
combined instantiation’. Such combined instantiations may be useful for some situations, for
example:

- a constellation of spacecraft in which ephemeris data for all of the spacecraft is
  combined in a single XML message;
- a spacecraft attitude that depends upon a particular orbital state (an APM and its
  associated OPM could be conveniently conveyed in a single NDM);
- an ephemeris message with the set of tracking data messages used in the orbit
  determination.

There are many other possible scenarios that may benefit from the combined instantiation
approach.

4.11.3 An NDM combined instantiation shall be delimited with the <ndm></ndm> root
element tags instead of one of the individual message tags.

4.11.4 The standard attributes documented in 4.3 shall be used with the <ndm> tag, with the
exception that neither ‘id’ nor ‘version’ attributes are associated with the <ndm> tag.

4.11.5 In the NDM combined instantiation, the only attributes that shall appear on the
constituent message tags are the ‘id’ and ‘version’ attributes, as described in the subsections
4.3.9 through 4.3.11.

4.11.6 Between the <ndm></ndm> tags, the desired messages described in table 3-1 may
be combined.

4.11.7 Any combination of constituent NDM message types may be used in an NDM
combined instantiation.

4.11.8 An NDM combined instantiation should consist of at least one constituent message
from the references listed in references [4]–[8].

4.11.9 DISCUSSION
^^^^^^^^^^^^^^^^^

Figures 4-1 through 4-3 illustrate the basic structure of an NDM combined instantiation.
Figure 4-1 has removed all detail to contrast the single message NDM with an NDM
combined instantiation. In figure 4-2, the basic structure of an NDM combined instantiation
is graphically illustrated. As shown in figure 4-3, in an NDM combined instantiation, the
individual message tags still have the ‘id’ and ‘version’ attributes, but the namespace
attributes and schema location attributes are associated with the <ndm> root element.

.. list-table:: Figure 4-1: Comparison of Single Message NDM with NDM Combined Instantiation
   :widths: 50 50
   :header-rows: 1

   * - Single Message NDM
     - NDM Combined Instantiation
   * - .. code-block:: xml

          <opm>
            <header>
            </header>
            <body>
            </body>
          </opm>
     - .. code-block:: xml

          <ndm>
            <opm>
              <header>
              </header>
              <body>
              </body>
            </opm>
            <apm>
              <header>
              </header>
              <body>
              </body>
            </apm>
          </ndm>

.. figure:: /images/ndm_combined_instantiation_basic_structure.png
   :align: center

   Figure 4-2: NDM Combined Instantiation Basic Structure

.. figure:: /images/ndm_combined_instantiation_attributes.png
   :align: center

   Figure 4-3: NDM Combined Instantiation Showing Use of Attributes
.. _annex_a_xml:

ANNEX A
=======

IMPLEMENTATION CONFORMANCE STATEMENT (ICS)
==========================================

(NORMATIVE)

.. note::
   This document does not contain an Implementation Conformance Specification
   (ICS), which is usually shown in annex A of Blue Books. This is because the
   material in this document simply reflects a reformatting of some older documents
   from KVN to XML. The constituent documents listed in 1.6 either now contain
   an ICS, or will contain an ICS annex upon publication of updated issues now in
   progress.

.. _annex_b_xml:

ANNEX B
=======

VALUES FOR SELECTED KEYWORDS
============================

(NORMATIVE)

.. note::
   This annex is not applicable to the Navigation Data Messages XML, though it
   does apply to other CCSDS Navigation Working Group Standards, which have a
   consistent ordering of annexes.

.. _annex_c_xml:

ANNEX C
=======

SECURITY, SANA, AND PATENT CONSIDERATIONS
=========================================

(INFORMATIVE)

.. _security_considerations_xml:

C1 SECURITY CONSIDERATIONS
--------------------------

.. _analysis_of_security_considerations_xml:

C1.1 ANALYSIS OF SECURITY CONSIDERATIONS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

This annex presents the results of an analysis of security considerations applied to the
technologies specified in this Recommended Standard.

.. _consequences_of_not_applying_security_xml:

C1.2 CONSEQUENCES OF NOT APPLYING SECURITY TO THE TECHNOLOGY
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The consequences of not applying security to the systems and networks on which this
Recommended Standard is implemented could include potential loss, corruption, and theft of
data. Because it is possible to utilize these messages in orbit determination, in preparing
pointing and frequency predicts used during spacecraft commanding, and in collision
avoidance studies, the consequences of not applying security to the systems and networks on
which this Recommended Standard is implemented could include compromise or loss of the
mission if malicious tampering of a particularly severe nature occurs.

.. _potential_threats_and_attack_scenarios_xml:

C1.3 POTENTIAL THREATS AND ATTACK SCENARIOS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Potential threats or attack scenarios include, but are not limited to, (a) unauthorized access to
the programs/processes that generate and interpret the messages, (b) unauthorized access to
the messages during transmission between exchange partners, and (c) modification of the
messages between partners. Protection from unauthorized access during transmission is
especially important if the mission utilizes open ground networks such as the Internet to
provide ground station connectivity for the exchange of data formatted in compliance with
this Recommended Standard. It is strongly recommended that potential threats or attack
scenarios applicable to the systems and networks on which this Recommended Standard is
implemented be addressed by the management of those systems and networks.

.. _data_privacy_xml:

C1.4 DATA PRIVACY
^^^^^^^^^^^^^^^^^

Privacy of data formatted in compliance with the specifications of this Recommended
Standard should be assured by the systems and networks on which this Recommended
Standard is implemented.

.. _data_integrity_xml:

C1.5 DATA INTEGRITY
^^^^^^^^^^^^^^^^^^^

Integrity of data formatted in compliance with the specifications of this Recommended
Standard should be assured by the systems and networks on which this Recommended
Standard is implemented.

.. _authentication_of_communicating_entities_xml:

C1.6 AUTHENTICATION OF COMMUNICATING ENTITIES
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Authentication of communicating entities involved in the transport of data that complies with
the specifications of this Recommended Standard should be provided by the systems and
networks on which this Recommended Standard is implemented.

.. _data_transfer_between_communicating_entities_xml:

C1.7 DATA TRANSFER BETWEEN COMMUNICATING ENTITIES
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The transfer of data formatted in compliance with this Recommended Standard between
communicating entities should be accomplished via secure mechanisms approved by the
Information Technology Security functionaries of exchange participants.

.. _control_of_access_to_resources_xml:

C1.8 CONTROL OF ACCESS TO RESOURCES
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Control of access to resources should be managed by the systems upon which originator
formatting and recipient processing are performed.

.. _auditing_of_resource_usage_xml:

C1.9 AUDITING OF RESOURCE USAGE
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Auditing of resource usage should be handled by the management of systems and networks
on which this Recommended Standard is implemented.

.. _unauthorized_access_xml:

C1.10 UNAUTHORIZED ACCESS
^^^^^^^^^^^^^^^^^^^^^^^^^

Unauthorized access to the programs/processes that generate and interpret the messages
should be prohibited in order to minimize potential threats and attack scenarios.

.. _data_security_implementation_specifics_xml:

C1.11 DATA SECURITY IMPLEMENTATION SPECIFICS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Specific information-security interoperability provisions that apply between agencies and
other independent users involved in an exchange of data formatted in compliance with this
Recommended Standard should be specified in an ICD.

.. _sana_considerations_xml:

C2 SANA CONSIDERATIONS
----------------------

The following NDM/XML related items are registered with the SANA Operator.

- The NDM/XML schemas (see reference [3]).

The values for certain fields in an XML instantiation are also registered with SANA. The
details as to these are incorporated in the ‘Security, SANA, and Patent Considerations’
annexes of references [4]-[8].

.. note:: This annex subsection is not present in older Navigation Working Group
   standards published prior to 2010.

The general policy for changes to the NDM/XML schemas is Expert Review by the Working
Group or Area responsible for the NDM/XML standard. Any NDM/XML schema changes in
the future will result in supersession of the older schema versions by the newer versions.
Older versions will be available for download at https://cwe.ccsds.org/moims/docs/MOIMS-
NAV/NDM-XML-Schema-Archive/.

The registration rule for new entries in the registry is the approval of new requests by the
CCSDS Area or Working Group responsible for the maintenance of the NDM/XML at the
time of the request. New requests for this registry should be sent to SANA
(mailto:info@sanaregistry.org).

.. _patent_considerations_xml:

C3 PATENT CONSIDERATIONS
------------------------

The recommendations of this document have no patent issues.

.. _annex_d_xml:

ANNEX D
=======

ABBREVIATIONS AND ACRONYMS
==========================

(INFORMATIVE)

.. list-table::
   :widths: 15 85

   * - ADM
     - Attitude Data Messages
   * - AEM
     - Attitude Ephemeris Message
   * - aem
     - Attitude Ephemeris Message tag
   * - APM
     - Attitude Parameter Message
   * - apm
     - Attitude Parameter Message tag
   * - ASCII
     - American Standard Code for Information Interchange
   * - CCSDS
     - Consultative Committee on Space Data Systems
   * - CDM
     - Conjunction Data Message
   * - cdm
     - Conjunction Data Message tag
   * - CMC
     - CCSDS Management Council
   * - CWE
     - Collaborative Working Environment
   * - DTD
     - Document Type Definition
   * - HTML
     - HyperText Markup Language
   * - ICD
     - Interface Control Document
   * - ICS
     - Implementation Conformance Statement
   * - ISO
     - International Organization for Standardization
   * - KVN
     - Keyword Value notation
   * - MOIMS
     - Mission Operations and Information Management Services
   * - NDM
     - Navigation Data Message
   * - ndm
     - Navigation Data Message tag
   * - NDM/XML
     - Navigation Data Messages XML Specification
   * - OCM
     - Orbit Comprehensive Message
   * - ocm
     - Orbit Comprehensive Message tag
   * - ODM
     - Orbit Data Messages
   * - OEM
     - Orbit Ephemeris Message
   * - oem
     - Orbit Ephemeris Message tag
   * - OMM
     - Orbit Mean Elements Message
   * - omm
     - Orbit Mean Elements Message tag
   * - OPM
     - Orbit Parameter Message
   * - opm
     - Orbit Parameter Message tag
   * - PVL
     - Parameter Value Language
   * - RDM
     - Re-entry Data Message
   * - rdm
     - Re-entry Data Message tag
   * - SANA
     - Space Assigned Numbers Authority
   * - SFTP
     - Secure File Transfer Protocol
   * - SIG
     - Special Interest Group
   * - TDM
     - Tracking Data Message
   * - tdm
     - Tracking Data Message tag
   * - URL
     - Uniform Resource Locator
   * - W3C
     - World Wide Web Consortium
   * - XML
     - Extensible Markup Language
   * - XSD
     - XML Schema Definition
   * - XTCE
     - XML Telemetry and Command Exchange
.. _annex_e_xml:

ANNEX E
=======

RATIONALE FOR XML-FORMAT NAVIGATION DATA MESSAGES
=================================================

(INFORMATIVE)

E1 GENERAL
----------

This annex presents the rationale behind the design of the NDM XML Specification. It is
intended to help the application engineer construct a suitable valid message. Corrections
and/or additions to these requirements during future updates are possible.

A specification of requirements agreed to by all parties is essential to focus design and to
ensure the product meets the needs of the Member Agencies. There are many ways of
organizing requirements, but the categorization of requirements is not as important as the
agreement to a sufficiently comprehensive set. In this annex, the requirements are organized
into three categories:

**Primary Requirements** are the most elementary and necessary requirements. They would
exist no matter the context in which the CCSDS is operating, that is, regardless of pre-
existing conditions within the CCSDS or its Member Agencies.

**Heritage Requirements** are additional requirements that derive from pre-existing Member
Agency requirements, conditions or needs. Ultimately these carry the same weight as the
Primary Requirements. This Recommended Standard reflects heritage requirements
pertaining to some of the technical participants’ home institutions collected during the
preparation of the Recommended Standard; it does not speculate on heritage requirements
that could arise from other Member Agencies.

**Desirable Characteristics** are not requirements, but they are felt to be important or useful
features of the Recommended Standard.

E2 PRIMARY REQUIREMENTS ACCEPTED FOR XML-FORMAT NAVIGATION DATA MESSAGES
-------------------------------------------------------------------------

.. list-table:: Table E-1: Primary Requirements
   :widths: 10 90
   :header-rows: 1

   * - ID
     - Requirement
   * - E-1-1
     - The NDM/XML data must be provided in digital form (computer file).
   * - E-1-2
     - The NDM/XML shall be represented by a valid XML format descriptor.
   * - E-1-3
     - The NDM/XML format descriptor shall be hosted on the SANA Registry, whence it can be shared by all agencies exchanging instantiations of the format descriptor.
   * - E-1-4
     - The NDM/XML shall allow for the representation of all the fields available in the Navigation Data Messages Recommended Standards.
   * - E-1-5
     - Files must be readily portable between and useable within ‘all’ computational environments in use by Member Agencies choosing to exchange NDMs via XML.
   * - E-1-6
     - Files must have means of being uniquely identified and clearly annotated. The file name alone is considered insufficient for this purpose.
   * - E-1-7
     - File name syntax and length must not violate computer constraints for those computing environments in use by Member Agencies.
   * - E-1-8
     - The NDM/XML shall use XML elements when there is substructure associated with information, for example, maneuver parameters.
   * - E-1-9
     - The NDM/XML shall use XML elements when there is data type checking associated with information, for example, maneuver parameters.

.. list-table:: Table E-2: Heritage Requirements
   :widths: 10 90
   :header-rows: 1

   * - ID
     - Requirement
   * - E-2-1
     - The standard shall be, or must include, an ASCII format.
   * - E-2-2
     - The standard shall not require software supplied by other agencies to process valid instantiations of the NDM/XML schema.

.. list-table:: Table E-3: Desirable Characteristics
   :widths: 10 90
   :header-rows: 1

   * - ID
     - Requirement
   * - E-3-1
     - The standard should be extensible with no disruption to existing users/uses.
   * - E-3-2
     - Keywords, values, and terminology in the NDM/XML should be the same as those in the Navigation Data Messages Recommended Standards, insofar as it is possible.
   * - E-3-3
     - Structures in the NDM/XML should be re-used across the different message types when practical.
   * - E-3-4
     - The NDM/XML should minimize the use of tags that do not correspond to keywords in the Navigation Data Messages Recommended Standards.
   * - E-3-5
     - Units may be specified in the NDM/XML instantiations. The standard should provide for clear specification of units of measure.
   * - E-3-6
     - The NDM/XML may use XML attributes when there is no substructure associated with information (e.g., units specifications).

.. note::
   ² These are the references listed in references [4]–[8].

.. _annex_f_xml:

ANNEX F
=======

TECHNICAL MATERIAL AND CONVENTIONS
==================================

(INFORMATIVE)

F1 EXTENSIBLE MARKUP LANGUAGE
----------------------------

F1.1 GENERAL
^^^^^^^^^^^^^

This annex describes very briefly the XML, generalities of the XML Schema Definition (XSD),
and the justification for using XML for NDMs. XML schema structures and data types are
specified in references [1] and [2].

F1.2 XML OVERVIEW
^^^^^^^^^^^^^^^^^^

F1.2.1 During the development of the first version of the ODM in the late 1990s/early
2000s, it was determined that the specified KVN format was limited and that it was not
necessarily well suited to cover all possible needs of the NDMs. XML can be a much better
form of specifying ASCII-based data. XML can also convey binary data using one of its
possible ASCII representations (e.g., base-64). This subsection presents a brief description
of the broad features of XML.

F1.2.2 XML is similar to the HyperText Markup Language (HTML) used for creating Web
pages, in that there are document tags (begin tags and end tags) that specify how to organize the
content. However, HTML has a fixed set of valid tags, while XML provides an extensible
framework that allows user-defined tag names that are structured according to the logic of the
particular application domain in which the document content exists. Additionally, XML documents
are required to be ‘well-formed’, whereas this restriction does not exist for HTML documents.
Discussion of the details of ‘well-formedness’ is beyond the scope of this document, but it is
essentially a set of rules that describe what constitutes a proper XML document. If the rules are not
followed, the document cannot be rendered correctly. HTML is less strict.

F1.2.3 Some of the advantages of using XML instead of standard text files for the
Navigation Data Messages application include:

- XML allows for the definition of the data message in a format that is readable both by
  humans and machines. The format is basically defined by a template called an XSD,
  or simply ‘schema’. This schema can then be referred to in the XML document, and it
  can be used to verify that the data structure and content are compliant with the
  schema. There are widely available programs to specify a schema, to assist with the
  processing of XML data, and to automatically verify that the data messages comply
  with the schema. Each participant in a data exchange can independently verify that
  the message is compliant. This can simplify the development and validation of the
  software used to write data in the proper format.

- XML defines standards for time formats and numerical values against which it is
  possible to validate the contents of an XML element.
- XML allows for the nesting of data so it is clear which metadata corresponds to which data.
- XML allows for the specification of default and alternative attributes, such as units.
- XML allows for required and optional elements and attributes.
- XML allows for range checking and specification of lists of allowed values.
- XML allows for sharing elements between different specifications.

F1.2.4 A few disadvantages of using XML for this application are:

- Tags are always duplicated, with the opening tag and the corresponding ending tag
  making files bigger (in some cases, it is possible that the byte count for tag information
  exceeds the byte count of the actual data associated with the tags). However, there are
  specific compressors for XML data (e.g., XMILL and XGRIND—references [H5] and
  [H6]) that are much more efficient than those used for non-XML-formatted ASCII data.
- Some values can be specified as either attributes or child elements, so there could be
  disagreement as to which method to use. This flexibility can also be seen as an
  advantage, depending upon the application and the implementation.
- There are not many Flight Dynamics specialists who are skilled in XML.
- There is not much Flight Dynamics software that can deal with data in XML format.

F1.3 JUSTIFICATION FOR USING XML SCHEMA
---------------------------------------

There are several ways in which XML files can be processed, for example: without
validation, with validation via Document Type Definition (DTD), with validation via
RELAX NG (reference [H7]), with validation via Schematron (reference [H8]), and with
validation via XML schema (references [1] and [2]). In the case of the CCSDS, the CCSDS
Management Council (CMC) has specified that the XML Schema method be used for XML
validation. The Navigation Working Group has therefore developed XML schema
implementations for its Recommended Standards, consistent with the directive of the CMC.³
These schema representations adopt the standard as approved by the World Wide Web
Consortium (W3C) (https://www.w3.org/).

.. note::
   ³ CCSDS Management Council Resolution MC-F02-09 directed Subpanel P1J (precursor to Navigation
   Working Group) to utilize PVL, or preferably XML schema language, in the CCSDS 502.0-R-2 Orbit Data
   Messages.

F1.4 JUSTIFICATION FOR INTEGRATED NDM/XML SCHEMA SET
----------------------------------------------------

There has been a movement towards the adoption of XML for space data systems data
interchange between agencies (e.g., the XML Telemetry and Command Exchange (XTCE)
developed by the Space Domain Task Force of the Object Management Group). Since there
are a number of separate NDM message types, some of which have considerable overlap in
structure and/or content, it is more efficient to structure the XML format for the set of NDMs
into an integrated set. This will help to ensure as much consistency and re-use as possible
between the message implementations and facilitates the coding of programs that will
produce the messages that will be exchanged.

The integrated NDM/XML schema set is stored in the CCSDS SANA repository
(reference [3]), accessible by all interested parties. Via such an arrangement, agencies
creating instantiations of an NDM/XML schema will be able to download the schema set
from the CCSDS site to an operations server in their own agencies. This will allow agencies
to control the reliability and operations aspects of providing the XML message types and will
ensure that all instantiations of an NDM/XML schema can be validated in a consistent
manner. Periodic updates of elements of the schema set could be necessary in order to retain
the correspondence to the KVN-formatted messages or to correct errors in an individual
schema, at which time agencies would download new copies of the schema set. An agency
that downloads a copy of the NDM/XML schema set to an operations server under its
management also has the option of introducing local modifications to the schema set, though
doing so could diminish its utility as an interagency exchange medium.

F2 SPECIAL CONSIDERATIONS
-------------------------

F2.1 COMMENTS IN NDM/XML INSTANTIATIONS
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Each of the KVN format NDMs provides a ‘COMMENT’ keyword that is used for a variety
of documentation purposes. In most cases the individual messages are consistent with
respect to the use of comments, and the placement is the same in the KVN and XML
versions. However, for historical reasons, in the original issue of the ODM Recommended
Standard, the allowed placement of comments was much freer than in subsequent
Recommended Standards of the Navigation Working Group. Allowing complete freedom in
the placement of comments in a KVN document is not problematic; however, an XML
schema supporting such free placement of comments has some difficulties. For example, it
could become impossible to convert between the XML and text versions of a message in a
way that comments can be uniquely associated with the proper data elements. Allowing
comments anywhere also makes a schema overly complex, lengthy, and error prone; obscures
the meaningful structure of the schema; and in some cases, makes it impossible for it to be
correctly interpreted by XML validators. For these reasons, the CCSDS Navigation Working
Group has restricted the placement of comments in all its subsequent standards.

F2.2 DISCUSSION OF ‘VALIDATION CHECKING’
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

There are some elements in the NDM Recommended Standards that have structure for which
checking could be performed, but is not done in the NDM XML schema set. Specifically,
time systems, object names, reference frames, and center names could be defined by an
enumerated list, and object IDs could be defined via a matching pattern. However, it has
been decided not to enforce these potential restrictions and to allow a generic string to be
used for the values associated with these concepts. In future versions of the NDM
Recommended Standards, there could be some validation checking imposed based on the
requirement to include ‘normative references’ that specifically enumerate the acceptable
values for some metadata keywords.

Because of this validation checking convention, the user of one of the messages will be
responsible for more validation code at the application level than would be necessary if strict
checking and validation were performed at the schema level (for example, if
<TIME_SYSTEM>UVC</TIME_SYSTEM> is coded, then user code will need to determine
that ‘UVC’ is not a valid value for the time system).

The design of the NDM/XML schema set is such that extension to cope with more restrictive
validation scenarios is easy to implement:

- restriction on generic values coded as character strings via pattern definition;
- value selection from an enumerated sequence;
- numerical ranges.

.. _annex_g_xml:

ANNEX G
=======

EXAMPLE NDM/XML SCHEMA INSTANTIATIONS
=====================================

(INFORMATIVE)

G1 GENERAL
----------

The schema sets associated with this standard are available via the CCSDS SANA repository:

- Overall Schema link: https://sanaregistry.org/r/ndmxml/
- Schemas with elementFormDefault="unqualified":
  https://sanaregistry.org/r/ndmxml_unqualified/
- Schemas with elementFormDefault="qualified":
  https://sanaregistry.org/r/ndmxml_qualified/

An assortment of valid instantiations of the NDM/XML Schema Set is available on the
CCSDS Web site’s CWE:
- https://cwe.ccsds.org/moims/docs/MOIMS-NAV/Test-Messages/XML

G2 SAMPLE NDM/XML AEM
---------------------

The following is a simple sample of an NDM/XML AEM:

.. code-block:: xml
   :linenos:

   <?xml version="1.0" encoding="UTF-8"?>
   <aem
   xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-3.0.0-master-3.0.xsd"
   id="CCSDS_AEM_VERS" version="1.0">
   <header>
   <COMMENT>This example corresponds to ADM Blue Book figure 4-2</COMMENT>
   <CREATION_DATE>2008-071T17:09:49</CREATION_DATE>
   <ORIGINATOR>GSFC</ORIGINATOR>
   </header>
   <body>
   <segment>
   <metadata>
   <COMMENT>This file was produced by M.R. Somebody, MSOO NAV, 2002 OCT 04.</COMMENT>
   <COMMENT>It is to be used for attitude reconstruction only. The relative accuracy of these</COMMENT>
   <COMMENT>attitudes is 0.1 degrees per axis.</COMMENT>
   <OBJECT_NAME>ST5-224</OBJECT_NAME>
   <OBJECT_ID>2006224</OBJECT_ID>
   <CENTER_NAME>EARTH</CENTER_NAME>
   <REF_FRAME_A>J2000</REF_FRAME_A>
   <REF_FRAME_B>SC_BODY_1</REF_FRAME_B>
   <ATTITUDE_DIR>A2B</ATTITUDE_DIR>
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
   </data>
   </segment>
   </body>
   </aem>

.. raw:: html

   <details>
   <summary><a>Sample NDM/XML AEM (continued)</a></summary>

.. code-block:: xml
   :linenos:

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
   <SPIN_ANGLE_VEL>-1.0996339e+002</SPIN_ANGLE_VEL>
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

.. raw:: html

   </details>

G3 SAMPLE NDM/XML AEM WITH ROTATION
------------------------------------

.. code-block:: xml
   :linenos:

   <?xml version="1.0" encoding="UTF-8"?>
   <aem
   xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-3.0.0-master-3.0.xsd"
   id="CCSDS_AEM_VERS" version="1.0">
   <header>
   <COMMENT>This example shows an AEM with a rotation</COMMENT>
   <CREATION_DATE>2008-071T17:09:49</CREATION_DATE>
   <ORIGINATOR>NASA</ORIGINATOR>
   </header>
   <body>
   <segment>
   <metadata>
   <COMMENT>The relative accuracy of these</COMMENT>
   <COMMENT>attitudes is 0.1 degrees per axis.</COMMENT>
   <OBJECT_NAME>FICTITIOUS</OBJECT_NAME>
   <OBJECT_ID>2020-224A</OBJECT_ID>
   <CENTER_NAME>EARTH</CENTER_NAME>
   <REF_FRAME_A>J2000</REF_FRAME_A>
   <REF_FRAME_B>SC_BODY_1</REF_FRAME_B>
   <ATTITUDE_DIR>A2B</ATTITUDE_DIR>
   <TIME_SYSTEM>UTC</TIME_SYSTEM>
   <START_TIME>2020-090T05:00:00.071</START_TIME>
   <STOP_TIME>2020-090T05:00:00.946</STOP_TIME>
   <ATTITUDE_TYPE>EULER_ANGLE/RATE</ATTITUDE_TYPE>
   </metadata>
   <data>
   <attitudeState>
   <eulerAngleRate>
   <EPOCH>2020-090T05:00:00.071</EPOCH>
   <rotationAngles>
   <rotation1 angle="X_ANGLE" units="deg">45</rotation1>
   <rotation2 angle="Y_ANGLE" units="deg">0.9</rotation2>
   <rotation3 angle="Z_ANGLE" units="deg">15</rotation3>
   </rotationAngles>
   <rotationRates>
   <rotation1 rate="X_RATE">4.5</rotation1>
   <rotation2 rate="Y_RATE">0.123</rotation2>
   <rotation3 rate="Z_RATE">15</rotation3>
   </rotationRates>
   </eulerAngleRate>
   </attitudeState>
   <attitudeState>
   <eulerAngleRate>
   <EPOCH>2020-090T05:00:00.946</EPOCH>
   <rotationAngles>
   <rotation1 angle="X_ANGLE" units="deg">50</rotation1>
   <rotation2 angle="Y_ANGLE" units="deg">1.9</rotation2>
   <rotation3 angle="Z_ANGLE" units="deg">1.5</rotation3>
   </rotationAngles>
   <rotationRates>
   <rotation1 rate="X_RATE">1.0</rotation1>
   <rotation2 rate="Y_RATE">0.123</rotation2>
   <rotation3 rate="Z_RATE">1.5</rotation3>
   </rotationRates>
   </eulerAngleRate>
   </attitudeState>
   </data>
   </segment>
   </body>
   </aem>

G4 SAMPLE NDM/XML APM
---------------------

The following is a simple sample of an NDM/XML APM:

.. code-block:: xml
   :linenos:

   <?xml version="1.0" encoding="UTF-8"?>
   <apm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_unqualified/ndmxml-3.0.0-master-3.0.xsd"
   id="CCSDS_APM_VERS" version="1.0">
   <header>
   <COMMENT>This example corresponds to ADM Blue Book figure 3-8</COMMENT>
   <CREATION_DATE>2004-02-14T19:23:57</CREATION_DATE>
   <ORIGINATOR>NASA</ORIGINATOR>
   </header>
   <body>
   <segment>
   <metadata>
   <OBJECT_NAME>MARS_SPIRIT</OBJECT_NAME>
   <OBJECT_ID>2004-003A</OBJECT_ID>
   <CENTER_NAME>EARTH</CENTER_NAME>
   <TIME_SYSTEM>UTC</TIME_SYSTEM>
   </metadata>
   <data>
   <COMMENT>GEOCENTRIC, CARTESIAN, EARTH_FIXED</COMMENT>
   <COMMENT>OBJECT_ID: 2004-003</COMMENT>
   <COMMENT>$ITIM = 2004 JAN 14 22:26:18.400000, original launch 14:36</COMMENT>
   <COMMENT>Generated by NASA</COMMENT>
   <COMMENT>Current attitude for orbit 20 and attitude maneuver</COMMENT>
   <COMMENT>planning data.</COMMENT>
   <COMMENT>Attitude state quaternion</COMMENT>
   <quaternionState>
   <EPOCH>2004-02-14T14:28:15.1172</EPOCH>
   <Q_FRAME_A>INSTRUMENT_1</Q_FRAME_A>
   <Q_FRAME_B>ITRF1997</Q_FRAME_B>
   <Q_DIR>A2B</Q_DIR>
   <quaternion>
   <Q1>0.03123</Q1>
   <Q2>0.78543</Q2>
   <Q3>0.39158</Q3>
   <QC>0.47832</QC>
   </quaternion>
   </quaternionState>
   <eulerElementsThree>
   <COMMENT>Attitude specified as Euler elements</COMMENT>
   <EULER_FRAME_A>INSTRUMENT_1</EULER_FRAME_A>
   <EULER_FRAME_B>ITRF1997</EULER_FRAME_B>
   <EULER_DIR>A2B</EULER_DIR>
   <EULER_ROT_SEQ>312</EULER_ROT_SEQ>
   <RATE_FRAME>EULER_FRAME_A</RATE_FRAME>
   <rotationAngles>
   <rotation1 angle="Z_ANGLE" units="deg">-53.3688</rotation1>
   <rotation2 angle="X_ANGLE" units="deg">139.7527</rotation2>
   <rotation3 angle="Y_ANGLE" units="deg">25.0658</rotation3>
   </rotationAngles>
   <rotationRates>
   <rotation1 rate="Z_RATE" units="deg/s">0.02156</rotation1>
   <rotation2 rate="X_RATE" units="deg/s">0.1045</rotation2>
   <rotation3 rate="Y_RATE" units="deg/s">0.03214</rotation3>
   </rotationRates>
   </eulerElementsThree>
   <spacecraftParameters>
   <COMMENT>Spacecraft Parameters</COMMENT>
   <I11 units="kg*m**2">6080.0</I11>
   <I22 units="kg*m**2">5245.5</I22>
   <I33 units="kg*m**2">8067.3</I33>
   <I12 units="kg*m**2">-135.9</I12>
   <I13 units="kg*m**2">89.3</I13>
   <I23 units="kg*m**2">-90.7</I23>
   </spacecraftParameters>
   <maneuverParameters>
   <COMMENT>Data follows for 1 planned maneuver.</COMMENT>
   <COMMENT>First attitude maneuver for: MARS_SPIRIT</COMMENT>
   <COMMENT>Impulsive, torque direction fixed in body frame</COMMENT>
   <MAN_EPOCH_START>2004-02-14T14:29:00.5098</MAN_EPOCH_START>
   <MAN_DURATION units="s">3</MAN_DURATION>
   <MAN_REF_FRAME>INSTRUMENT_1</MAN_REF_FRAME>
   <MAN_TOR_1 units="N*m">-1.25</MAN_TOR_1>
   <MAN_TOR_2 units="N*m">-0.5</MAN_TOR_2>
   <MAN_TOR_3 units="N*m">0.5</MAN_TOR_3>
   </maneuverParameters>
   </data>
   </segment>
   </body>
   </apm>

G5 SAMPLE QUALIFIED NDM/XML INSTANCE
------------------------------------

The following is a simple sample of a qualified NDM/XML instance:

.. code-block:: xml
   :linenos:

   <?xml version="1.0" encoding="UTF-8"?>
   <ndm:ndm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
   xmlns:ndm="urn:ccsds:schema:ndmxml"
   xsi:noNamespaceSchemaLocation="https://sanaregistry.org/r/ndmxml_qualified/ndmxml-3.0.0-master-3.0.xsd">
   <ndm:opm
   id="CCSDS_OPM_VERS" version="3.0">
   <ndm:header>
   <ndm:COMMENT>In this simple case, there are no Keplerian elements, no maneuver, no S/C
   parameters, no covariance matrix; this is essentially just the state vector</ndm:COMMENT>
   <ndm:CREATION_DATE>2009-05-18T13:06:00</ndm:CREATION_DATE>
   <ndm:ORIGINATOR>GSFC</ndm:ORIGINATOR>
   </ndm:header>
   <ndm:body>
   <ndm:segment>
   <ndm:metadata>
   <ndm:OBJECT_NAME>SOHO</ndm:OBJECT_NAME>
   <ndm:OBJECT_ID>2009-000A</ndm:OBJECT_ID>
   <ndm:CENTER_NAME>EARTH</ndm:CENTER_NAME>
   <ndm:REF_FRAME>EME2000</ndm:REF_FRAME>
   <ndm:TIME_SYSTEM>UTC</ndm:TIME_SYSTEM>
   </ndm:metadata>
   <ndm:data>
   <ndm:stateVector>
   <ndm:EPOCH>2009-04-28T00:00:00</ndm:EPOCH>
   <ndm:X>0.11480770338073E+07</ndm:X>
   <ndm:Y>0.50826618901580E+06</ndm:Y>
   <ndm:Z>0.32422917889939E+06</ndm:Z>
   <ndm:X_DOT>-0.29736064079430</ndm:X_DOT>
   <ndm:Y_DOT>0.39070228393147</ndm:Y_DOT>
   <ndm:Z_DOT>0.19156258887615</ndm:Z_DOT>
   </ndm:stateVector>
   </ndm:data>
   </ndm:segment>
   </ndm:body>
   </ndm:opm>
   <ndm:opm id="CCSDS_OPM_VERS" version="3.0">
   <ndm:header>
   <ndm:CREATION_DATE>2009-05-18T13:06:00</ndm:CREATION_DATE>
   <ndm:ORIGINATOR>GSFC</ndm:ORIGINATOR>
   </ndm:header>
   <ndm:body>
   <ndm:segment>
   <ndm:metadata>
   <ndm:OBJECT_NAME>SOHO</ndm:OBJECT_NAME>
   <ndm:OBJECT_ID>2009-000A</ndm:OBJECT_ID>
   <ndm:CENTER_NAME>EARTH</ndm:CENTER_NAME>
   <ndm:REF_FRAME>EME2000</ndm:REF_FRAME>
   <ndm:TIME_SYSTEM>UTC</ndm:TIME_SYSTEM>
   </ndm:metadata>
   <ndm:data>
   <ndm:stateVector>
   <ndm:EPOCH>2009-04-28T00:00:00</ndm:EPOCH>
   <ndm:X>0.11480770338073E+07</ndm:X>
   <ndm:Y>0.50826618901580E+06</ndm:Y>
   <ndm:Z>0.32422917889939E+06</ndm:Z>
   <ndm:X_DOT>-0.29736064079430</ndm:X_DOT>
   <ndm:Y_DOT>0.39070228393147</ndm:Y_DOT>
   <ndm:Z_DOT>0.19156258887615</ndm:Z_DOT>
   </ndm:stateVector>
   </ndm:data>
   </ndm:segment>
   </ndm:body>
   </ndm:opm>
   </ndm:ndm>

.. _annex_h_xml:

ANNEX H
=======

INFORMATIVE REFERENCES
======================

(INFORMATIVE)

[H1] *Navigation Data—Definitions and Conventions*. Issue 4. Report Concerning Space Data
     System Standards (Green Book), CCSDS 500.0-G-4. Washington, D.C.: CCSDS,
     November 2019.
[H2] Fran Martínez. “XML in CCSDS.” Presented at CCSDS Navigation Working Group
     meeting (May 2004, Montreal). https://cwe.ccsds.org/moims/docs/MOIMS-
     NAV/NDM%20XML%20Related%20Material/XML-in-CCSDS-Montreal-2004.ppt.
[H3] *Space Communication Cross Support—Service Management—Service Specification*.
     Issue 1-S. Recommendation for Space Data System Standards (Historical), CCSDS
     910.11-B-1-S. Washington, D.C.: CCSDS, (August 2009) June 2017.
[H4] *Information Technology—8-Bit Single-Byte Coded Graphic Character Sets—Part 1:
     Latin Alphabet No. 1*. International Standard. ISO/IEC 8859-1:1998. Geneva: ISO,
     1998.
[H5] “SourceForge.net: XMill.” SourceForge.net: Open Source Software.
     https://sourceforge.net/projects/xmill/.
[H6] “SourceForge.net: XGrind: A Query-Friendly XML Compressor.” SourceForge.net:
     Open Source Software. https://sourceforge.net/projects/xgrind/.
[H7] RELAX NG home page. https://relaxng.org/.
[H8] *Information Technology—Document Schema Definition Languages (DSDL)—Part 3:
     Rule-Based Validation—Schematron*. 3rd ed. International Standard, ISO/IEC 19757-
     3:2020. Geneva: ISO, 2020.

.. note:: Normative references appear in 1.6.

.. _annex_i_xml:

ANNEX I
=======

ITEMS FOR AN INTERFACE CONTROL DOCUMENT (ICD)
==============================================

(INFORMATIVE)

This annex lists a number of items that should be covered in interagency ICDs prior to
exchanging NDMs on a regular basis. There are some statements in the document
that refer to the desirability or necessity of such a document; this annex consolidates the
suggested ICD items in a single list:

- The means of transmission of an XML-formatted NDM between exchange
  participants (see 1.2);
- User-defined parameters, if utilized (see 4.10.1.1);
- Specific information-security interoperability provisions that apply between agencies
  and other independent users (see C1.11).

.. _annex_j_xml:

ANNEX J
=======

CHANGES IN NDM/XML VERSION 3
============================

(INFORMATIVE)

J1 Detailed material related to creating XML instantiations of the Orbit Data Messages
   (ODM) has been removed. This material is now described in the ODM version 3, reference [5].
J2 The document annexes have been rearranged relative to the previous version to
   conform to a guideline developed for all of the CCSDS Navigation Working Group
   documents.
