Common Workflows
================

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

Converting notation and edition
-------------------------------

Use the same generic API for every recognized message family:

.. code-block:: python

   xml = ccsds_ndm.convert(kvn, "xml")
   ccsds_ndm.convert_file("input.opm", "output.xml", "xml")

The input notation is detected automatically.

OPM, OEM, and OMM support edition-correct ODM 2.0 and 3.0 output. Select an edition with
``version="2.0"`` or ``version="3.0"``. The 2.0 checks use the official `SANA NDM/XML schema
archive <https://sanaregistry.org/r/ndmxml_unqualified/>`_. OPM and OEM ODM 1.0 remain parse-only
because they do not have an audited schema-backed serializer; attempted relabeling is rejected.

Migrating the pre-0.0.9 API
---------------------------

Use ``convert`` and ``convert_file`` in place of the redundant OPM-only conversion functions.
Remove the ``validate`` argument from ``to_str`` and ``to_file``; output is always validated.
Change nested Python fields directly; the former editor/proxy API has been removed.

CCSDS units
-----------

The parser checks supplied units against each field's CCSDS unit enum. Required XML unit
attributes must be present, incompatible KVN units are rejected, and dimensionless fields reject
spurious units. Values are never silently reinterpreted under a different unit.
