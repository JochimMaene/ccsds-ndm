Release notes
=============

Unreleased
----------

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
