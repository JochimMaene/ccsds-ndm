Common Workflows
================

Constructing a message from caller data
---------------------------------------

.. code-block:: python

   import ccsds_ndm
   from ccsds_ndm import OdmHeader, Opm, OpmData, OpmMetadata, OpmSegment, StateVector

   header = OdmHeader("2023-01-01T00:00:00", "TEST")
   meta = OpmMetadata(
       object_name="SAT1",
       object_id="2023-001A",
       center_name="EARTH",
       ref_frame="GCRF",
       time_system="UTC",
   )
   state = StateVector(
       epoch="2023-01-01T00:00:00",
       x=7000.0, y=0.0, z=0.0, x_dot=0.0, y_dot=7.5, z_dot=0.0,
       comment=None,
   )
   opm = Opm(
       header=header,
       segment=OpmSegment(metadata=meta, data=OpmData(state_vector=state, comment=[])),
   )
   opm.to_file("output.opm", "kvn")

Generation validates the complete message before writing any caller-visible bytes.
For repeated-record shapes (OEM state vectors, AEM states), prefer the
documented ``from_numpy`` entry points at scale. ``just bench-python`` runs the
``OemData.from_numpy`` benchmark at 10,000 records; it is tracked by CodSpeed alongside the
Rust benchmarks.

Editing nested Python models
----------------------------

Nested model objects and record collections are live. Change them directly:

.. code-block:: python

   import ccsds_ndm

   message = ccsds_ndm.from_file("example.opm")
   message.segment.metadata.object_name = "UPDATED"
   message.to_file("updated.opm", "kvn")

Retained child references preserve their identity, and structural changes to repeated model fields
such as OEM segment, state-vector, and covariance lists affect the owning message. Generation always
validates the complete message. There is no commit step and no unchecked ``validate=False`` mode.

Lists of plain values are copies, not live views: ``comment`` returns a new ``list`` of strings each
time, so appending to it changes nothing. Assign the whole list instead:

.. code-block:: python

   data = message.segment.data
   data.comment = [*data.comment, "Reprocessed"]

NumPy properties such as ``state_vector_numpy`` are likewise copies; assign an array to write it back.

Model objects cannot be copied with :mod:`copy` or serialized with :mod:`pickle`. To duplicate a
message, generate and parse it again, for example ``Oem.from_str(message.to_str("xml"), "xml")``.
That round trip validates and normalizes, so it only duplicates a complete, valid message, and it
keeps only what the chosen notation represents.

Converting notation
-------------------

Use the same generic API for every recognized message family:

.. code-block:: python

   xml = ccsds_ndm.convert(kvn, "xml")
   ccsds_ndm.convert_file("input.opm", "output.xml", "xml")

The input notation is detected automatically.

OPM, OEM, and OMM support edition-correct ODM 2.0 and 3.0 output. The message's ``version``
attribute selects the edition, for example ``message.version = "2.0"`` before ``to_file``. The 2.0 checks use the official `SANA NDM/XML schema
archive <https://sanaregistry.org/r/ndmxml_unqualified/>`_. OEM 1.0 has a restricted KVN input-only compatibility reader; stored 1.0 messages cannot be
generated. OPM, APM, AEM, and TDM 1.0 input is rejected because their older syntax and requirements
are not implemented as separate editions. See the project support matrix for the precise scope.

CCSDS units
-----------

The parser checks supplied units against each field's CCSDS unit enum. Required XML unit
attributes must be present, incompatible KVN units are rejected, and dimensionless fields reject
spurious units. Values are never silently reinterpreted under a different unit.

Parsing and generation guarantees
---------------------------------

Callers that accept untrusted input should enforce size limits at their system boundary. XML
nesting retains a fixed internal safety limit. Parsing materializes complete messages; streaming
parsing is intentionally absent.

Generation always validates the complete message before writing any caller-visible bytes and
preserves the edition stored on the message. In Python, use ``to_str(format)`` for a string and
``to_file(path, format)`` for atomic file replacement. Rust exposes ``to_kvn``, ``to_xml``, and
the ``write_*_to`` streaming methods; streaming can leave partial output if the caller's sink
fails mid-write. ``convert_file`` also replaces its destination atomically. Ordinary numeric
values use double precision. KVN writers round them when necessary to the
16-significant-digit representation and refuse rounding that invalidates a model constraint.
TDM phase counts retain exact decimal strings instead, subject to KVN's line
limit. Optional fixed-unit annotations and some comment positions are normalized during
conversion, so a notation round trip need not produce an identical model. Other information
that cannot be represented causes conversion to fail. The support matrix and family records
describe the specific normalizations and refusals.

The shared XML parser checks supported message structure and field values without running an
XSD engine. Ignored processing instructions and unused namespace declarations receive limited
lexical checks; malformed names there can be accepted. Independently validate the source XML
when full XML well-formedness or independent schema validation is required. This limit and the common
preservation policy are recorded in the validation contract linked from the support matrix.

Errors expose stable ``code()``, ``field_path()``, and ``diagnostic()`` accessors; diagnostic
wording may improve before 1.0 while codes, enum meanings, and canonical paths are the machine
interface. Python NDM exceptions additionally expose ``operation``, ``notation``,
``message_kind``, source edition, and available source location/token fields.
Unavailable diagnostic fields are ``None``.

Migrating metadata construction
-------------------------------

Metadata constructors now require CCSDS-significant interpretation fields such as
``time_system``, reference frames, centers, and required interval epochs instead of silently
supplying defaults. Pass those values explicitly. ``interpolation_degree=0`` is now rejected;
omit it with ``None`` when no interpolation degree is present. CDM metadata likewise requires
explicit ``covariance_method`` and ``maneuverable`` values; it no longer assumes calculated
covariance or maneuver capability.
