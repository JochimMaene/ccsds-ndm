Release notes
=============

Unreleased
----------

Python metadata construction (breaking change)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Metadata constructors now require explicit interpretation fields instead of
silently supplying defaults:

* ``AemMetadata`` requires ``ref_frame_a``, ``ref_frame_b``, ``start_time``,
  ``stop_time``, and ``time_system``.
* ``OemMetadata`` requires ``center_name``, ``ref_frame``, and ``time_system``.

Calls omitting these arguments now raise ``TypeError``. Pass the values appropriate
to your message explicitly; keyword arguments make the intended fields clear.

``interpolation_degree=0`` is now rejected instead of being treated as absent.
Use ``None`` to omit the degree, or a positive integer when specifying it.
See :doc:`guide/workflows` for metadata construction guidance.
