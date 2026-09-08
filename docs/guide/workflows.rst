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
       comments=None,
   )
   opm = Opm(
       header=header,
       segment=OpmSegment(metadata=meta, data=OpmData(state_vector=state, comment=[])),
   )
   opm.to_file("output.opm", "kvn")

Generation validates the complete message before writing any caller-visible bytes.
For repeated-record shapes (OEM state vectors, AEM states), prefer the
documented ``from_numpy`` entry points at scale; see the construction benchmark notes.

Editing nested Python models
----------------------------

Nested model properties and collections are live. Change them directly:

.. code-block:: python

   import ccsds_ndm

   message = ccsds_ndm.from_file("example.opm")
   message.segment.metadata.object_name = "UPDATED"
   message.to_file("updated.opm", "kvn")

Retained child references preserve their identity, and structural changes to repeated model fields
such as OEM state-vector lists affect the owning message. Generation always validates the complete
message. There is no commit step and no unchecked ``validate=False`` mode.

Converting notation
-------------------

Use the same generic API for every recognized message family:

.. code-block:: python

   xml = ccsds_ndm.convert(kvn, "xml")
   ccsds_ndm.convert_file("input.opm", "output.xml", "xml")

The input notation is detected automatically.

OPM, OEM, and OMM support edition-correct ODM 2.0 and 3.0 output. Select an edition with
``version="2.0"`` or ``version="3.0"``. The 2.0 checks use the official `SANA NDM/XML schema
archive <https://sanaregistry.org/r/ndmxml_unqualified/>`_. OPM and OEM ODM 1.0 remain parse-only
because they do not have an audited schema-backed serializer; attempted relabeling is rejected.

CCSDS units
-----------

The parser checks supplied units against each field's CCSDS unit enum. Required XML unit
attributes must be present, incompatible KVN units are rejected, and dimensionless fields reject
spurious units. Values are never silently reinterpreted under a different unit.

Parsing limits and generation guarantees
-----------------------------------------

Python's ``from_str`` and ``from_file`` accept ``max_input_bytes`` on every message and
``max_records`` on record-bearing messages. Rust additionally exposes ``from_str_with_options``
for an XML-depth policy. Input bytes are unlimited by default; XML depth defaults to 16 because
valid messages have a small fixed schema depth. Parsing remains bounded materialization;
streaming parsing is intentionally absent.

Generation always validates the complete message before writing any caller-visible bytes and
preserves the edition stored on the message. In Python, use ``to_str(format)`` for a string and
``to_file(path, format)`` for atomic file replacement. Rust exposes ``to_kvn``, ``to_xml``, and
the ``write_*_to`` streaming methods; streaming can leave partial output if the caller's sink
fails mid-write. ``convert_file`` also replaces its destination atomically. Finite XML values
are rounded when necessary to the 16-digit KVN representation required by CCSDS.

Errors expose stable ``code()``, ``field_path()``, and ``diagnostic()`` accessors; diagnostic
wording may improve before 1.0 while codes, enum meanings, and canonical paths are the machine
interface. Python NDM exceptions additionally expose ``operation``, ``notation``,
``message_kind``, source edition, and available source location/token fields.

Migrating metadata construction
-------------------------------

Metadata constructors now require CCSDS-significant interpretation fields such as
``time_system``, reference frames, centers, and required interval epochs instead of silently
supplying defaults. Pass those values explicitly. ``interpolation_degree=0`` is now rejected;
omit it with ``None`` when no interpolation degree is present.
