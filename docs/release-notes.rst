Release notes
=============

Unreleased
----------

Book-aligned OEM and XML parsing (breaking change)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

OEM parsing and validation now take their rules from CCSDS 502.0-B-3 and the
NDM/XML books and schemas rather than project policy; the remaining limits are
listed in the OEM conformance inventory. Newly accepted:

- KVN blank lines anywhere, trailing blanks after ``META_START`` and the
  covariance markers, empty optional values, and an empty covariance section.
- Overlapping total spans without useable bounds, object names differing only
  in case, covariance matrices with equal epochs, and MET/MRT durations (including
  ``REF_FRAME_EPOCH``, now a time-system-dependent ``Epoch``) such as
  ``0000-000T00:10:00``.
- XML namespace declarations and schema-location hints on any element of a
  standalone message (ODM 8.12.7 still limits combined constituents to ``id``
  and ``version``), the qualified (``ndm:``-prefixed) element form in every
  family, and ``INF``/``-INF``/``NaN`` OEM state and covariance values in XML.
- Mixed calendar and day-of-year epochs with different fractional precision now
  compare correctly in OEM, AEM and OCM.

Newly rejected:

- XML documents in every family whose first line is not exactly
  ``<?xml version="1.0" encoding="UTF-8"?>``, or whose root does not declare
  ``xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"``.
- OEM KVN covariance rows of the wrong width,
  and mixed-case KVN spellings of the time systems and reference frames the book
  lists (for example ``Utc`` or ``Eme2000``). XML text follows ``xsd:string``,
  so such XML values are accepted and only their conversion to KVN fails;
  ICD-defined values keep their spelling in both notations.
- Time tags with more than 16 fractional-second digits, in every family except
  TDM (ODM 7.5.10 and the matching ADM, CDM and RDM rules).
- Empty or ``n/a`` XML values for optional numbers, epochs and enumerations in
  every family, which were silently treated as absent; the schema types do not
  allow them. Optional XML strings keep ``n/a`` and empty text literally. A
  malformed optional value can no longer silently drop its ``units`` attribute.
- XML special values spelled other than ``INF``, ``-INF`` or ``NaN`` (for
  example ``inf`` or ``+INF``), in every family.
- OPM ``nil`` and ``xsi:nil`` attributes, which the OPM schema does not allow.

OEM review follow-up:

- Shared XML parsing rejects literal ``]]>`` in text, empty prefixed namespace
  bindings, duplicate schema-location attributes hidden behind prefix aliases,
  and empty or reserved ``xml`` processing-instruction targets.
- OEM 3.0 and OEM 2.0 are now **Verified** in the support matrix. OEM 2.0 is
  reviewed against 502.0-B-2 with Corrigendum 1 and its schema; it differs
  from 3.0 in the header, and two looser readings (text case, ``-0``/special
  values) are documented. Every shipped OEM example is checked as 2.0. ODM 5.2.4.7 (enough
  records for the interpolation method) is recorded as a requirement the book
  leaves undefined rather than a conformance gap.
- OEM 1.0 is parse-only, like OPM 1.0: it is read with the 2.0 rules and
  cannot be written. XML is rejected for 1.0, a KVN-only format, and so is
  content that 502.0-B-1 does not have (``CLASSIFICATION``, ``MESSAGE_ID``,
  ``REF_FRAME_EPOCH``, accelerations, covariance). Some valid 1.0 files are
  still rejected; the OEM conformance page lists them.
- OEM covariance epochs are no longer required to fall inside the segment's
  ``START_TIME``/``STOP_TIME``: table 5-3 says they should, but the book's own
  example G-14 does not, so the shipped ``oem_g14.xml`` is now the example as
  published. State epochs are still bounded.
- OEM and AEM KVN numbers outside the double range (ODM 7.5.7e) are rejected
  instead of becoming infinity or zero.
- AEM KVN history numbers get the same reading as OEM: any number of digits
  and integer-form values of any size.
- OEM KVN reading accepts an unterminated last line and ephemeris and
  covariance numbers with more than 16 digits (such as the 17-digit shortest
  spelling of a double) or integers outside the 32-bit range. These break
  clear book rules but are common and lose nothing; writing still follows the
  book and rounds numbers to 16 significant digits.
- OEM KVN generation rejects an empty ``INTERPOLATION`` or ``COV_REF_FRAME``,
  which KVN would read back as absent; converting an XML ``<INTERPOLATION/>`` to KVN now fails
  instead of dropping it.
- Python: the NumPy inputs of OEM, AEM and CDM accept integer arrays and
  nested lists, converting them to float, instead of raising a confusing
  ``TypeError``.
- Python (breaking): ``OemData``, ``OemData.from_numpy``, ``OmmData``,
  ``CdmData``, ``CdmData.from_numpy``, ``CdmStateVector`` and
  ``StateVector`` take ``comment=`` instead of ``comments=``, matching their
  ``comment`` attribute and the other families. ``CombinedNdm`` keeps
  ``comments=``, the name of its attribute.
- An OEM KVN covariance section holding only comments is accepted; its
  comments join the data comments.
- An empty OEM KVN ``COV_REF_FRAME`` is absent, like other empty optional
  values.
- Mixed-case ``RSW``, ``RTN`` and ``TNW`` (ODM 3.2.4.11) are rejected in OEM
  KVN ``COV_REF_FRAME``.
- KVN syntax errors in every family carry their reason as the message instead
  of an empty message.
- Python: assigning an empty snapshot back to an empty ``OemData`` is a no-op
  instead of an error.
- Python: setting ``x_ddot``, ``y_ddot`` or ``z_ddot`` on a ``StateVectorAcc``
  keeps explicit XML units, like assigning ``state_vector_numpy``.
- Where the CCSDS books are unclear or conflict, parsing takes the permissive
  reading; OEM's remaining strict choices are listed in its conformance
  document.

Generated XML now declares ``xmlns:xsi`` and ``xmlns:ndm`` on the root and
writes special values as ``INF``, ``-INF`` and ``NaN``. KVN comments drop
trailing blanks, which the book makes insignificant.

Python docstrings now carry the CCSDS book reference of every field whose core
documentation has one.

Earlier OEM corrections in this release: covariance epochs must lie within the
total metadata span; KVN interpolation degrees accept a leading plus and must
fit a positive signed 32-bit integer; malformed optional OEM XML numbers and
epochs raise errors instead of being treated as absent; OEM optional strings and
shared ODM header strings retain literal content, including ``n/a``; shared XML
parsing rejects mixed container text, invalid character references and
malformed comments.

Python binding corrections (breaking change)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

``validate()`` on ``OemSegment``, ``OemMetadata``, ``OemData``, ``AemSegment``,
``AemMetadata``, ``AemData``, ``AcmSegment``, ``AcmMetadata`` and ``AcmData``
now raises ``NdmValidationError``, like message-level ``validate()``, instead of
``ValueError``. ``NdmValidationError`` is not a ``ValueError`` subclass.

``OemCovarianceMatrix(epoch, values, cov_ref_frame=None, comment=None)`` makes
the last two arguments optional. A wrong-typed element in a repeated field now
reports ``"<field>[<index>] must be <Type>"`` in every family.

``AemData.attitude_states_numpy`` assignment no longer partially writes when a
later element is invalid. The OEM NumPy setters update existing records in
place, keeping record identity, covariance frames and comments, and units, and
write nothing if any element is invalid. Covariance metadata supplied without
covariance epochs and values raises ``ValueError`` instead of being discarded.

Rust parsing controls (breaking change)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

``ParseOptions`` and the ``from_str_with_options``, ``from_file_with_options``,
``convert_with_options``, and ``convert_file_with_options`` functions have been
removed. Use ``from_str``/``from_file`` or their ``*_with_notation`` variants,
and use ``convert``/``convert_file`` for conversion. Applications that need
input-size or record-count limits should enforce them before parsing; XML depth
remains protected by the library's fixed safety limit.

Python metadata construction (breaking change)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Metadata constructors now require explicit interpretation fields instead of
silently supplying defaults:

* ``AemMetadata`` requires ``ref_frame_a``, ``ref_frame_b``, ``start_time``,
  ``stop_time``, and ``time_system``.
* ``OemMetadata`` requires ``center_name``, ``ref_frame``, and ``time_system``.
* ``OmmMetadata`` and ``OpmMetadata`` require ``center_name``, ``ref_frame``,
  and ``time_system``.
* ``RdmMetadata`` requires ``center_name`` and ``time_system``.
* ``AcmMetadata``, ``ApmMetadata``, ``OcmMetadata``, and ``TdmMetadata`` require
  ``time_system``.

Calls omitting these arguments now raise ``TypeError``. Pass the values appropriate
to your message explicitly; keyword arguments make the intended fields clear.

``interpolation_degree=0`` is now rejected instead of being treated as absent.
Use ``None`` to omit the degree, or a positive integer when specifying it.
See :doc:`guide/workflows` for metadata construction guidance.

``AemData`` and ``AemData.from_numpy`` now require an explicit
``attitude_type`` instead of inferring it from unambiguous row widths.
``OemData`` names its optional covariance collection ``covariance_matrices``;
the mutable model property remains ``covariance_matrix``.
