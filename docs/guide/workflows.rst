Common Workflows
================

Editing nested Python models
----------------------------

Nested model properties are owned snapshots. Use :func:`ccsds_ndm.edit` so changes are copied back
through their parents:

.. code-block:: python

   import ccsds_ndm

   message = ccsds_ndm.from_file("example.opm")
   ccsds_ndm.edit(message).segment.metadata.object_name = "UPDATED"
   message.to_file("updated.opm", "kvn")

Generation always validates the complete message. There is no unchecked ``validate=False`` mode.

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

Command line
------------

Install the executable and use it for shell pipelines or atomic file conversion:

.. code-block:: console

   $ cargo install ccsds-ndm
   $ ccsds-ndm validate example.opm
   $ ccsds-ndm convert --to xml example.opm -o example.xml
   $ ccsds-ndm convert --to xml --target-version 2.0 example.opm

``--to`` is required. Diagnostics go to stderr and document bytes go to stdout unless ``-o`` is
used. Exit codes distinguish invalid data, unsupported operations, resource limits, I/O, and
command usage.

Migrating the pre-0.0.9 API
---------------------------

Use ``convert`` and ``convert_file`` in place of the redundant OPM-only conversion functions.
Remove the ``validate`` argument from ``to_str`` and ``to_file``; output is always validated.
Use ``edit(message)`` for nested Python changes.

CCSDS units
-----------

The parser checks supplied units against each field's CCSDS unit enum. Required XML unit
attributes must be present, incompatible KVN units are rejected, and dimensionless fields reject
spurious units. Values are never silently reinterpreted under a different unit.
