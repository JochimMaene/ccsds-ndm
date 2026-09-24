Orbit Ephemeris Message (OEM)
=============================

.. currentmodule:: ccsds_ndm

Create an OEM from numerical data:

.. code-block:: python

   import numpy as np
   from ccsds_ndm import OdmHeader, Oem, OemData, OemMetadata, OemSegment

   epochs = ["2026-01-01T00:00:00", "2026-01-01T00:01:00"]
   # Columns: X, Y, Z in km; X_DOT, Y_DOT, Z_DOT in km/s.
   states = np.array([[7000., 0., 0., 0., 7.5, 0.],
                      [6985., 450., 0., -0.48, 7.48, 0.]])
   metadata = OemMetadata(
       object_name="EXAMPLE", object_id="2026-001A",
       center_name="EARTH", ref_frame="GCRF", time_system="UTC",
       start_time=epochs[0], stop_time=epochs[-1],
       interpolation="LINEAR", interpolation_degree=1,
   )
   data = OemData.from_numpy(epochs, states)
   message = Oem(
       OdmHeader("2026-01-01T00:02:00", "EXAMPLE"),
       [OemSegment(metadata, data)],
   )
   message.validate()
   message.to_file("example.oem", "kvn")
   Oem.from_file("example.oem").to_file("example.xml", "xml")

Generation also validates the selected notation. For example, XML permits individual
acceleration components and non-finite values that KVN cannot represent. NumPy arrays
are copies; assign an edited array back to ``data.state_vector_numpy`` to update records.
Epoch strings use the metadata's time system; the library does not interpolate or convert
frames or time scales. Parsing retains the whole history in memory.

.. autoclass:: Oem
   :members:
   :undoc-members:
.. autoclass:: OemSegment
   :members:
   :undoc-members:
.. autoclass:: OemMetadata
   :members:
   :undoc-members:
.. autoclass:: OemData
   :members:
   :undoc-members:
.. autoclass:: OemCovarianceMatrix
   :members:
   :undoc-members:
