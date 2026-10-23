Core & Common Types
===================

.. currentmodule:: ccsds_ndm

.. autofunction:: from_str
.. autofunction:: from_file
.. autofunction:: convert
.. autofunction:: convert_file

.. autoclass:: CombinedNdm
   :members:
   :undoc-members:

.. autoclass:: UserDefined
   :members:
   :undoc-members:

.. autoclass:: OdmHeader
   :members:
   :undoc-members:
.. autoclass:: StateVector
   :members:
   :undoc-members:
.. autoclass:: StateVectorAcc
   :members:
   :undoc-members:

Enums
-----

.. autoclass:: YesNo
   :members:
   :undoc-members:
.. autoclass:: ControlledType
   :members:
   :undoc-members:

Exceptions
----------

Parse syntax and epoch errors inherit from ``ValueError``; file errors inherit from ``OSError``.
Validation and unsupported-message/notation errors inherit from ``NdmError``. Catching only
``NdmError`` therefore does not catch every library exception. Unsupported input/output editions
use ``NdmValidationError``; their ``parse.*`` or ``generation.*`` code distinguishes the operation.

Diagnostic attributes
~~~~~~~~~~~~~~~~~~~~~

Every library exception exposes these attributes, with ``None`` for unavailable context:

.. list-table::
   :header-rows: 1
   :widths: 25 75

   * - Attribute
     - Meaning
   * - ``code``
     - Stable category from the table below; human-readable wording can change.
   * - ``operation``
     - ``parse``, ``validate``, or ``generate`` when known.
   * - ``notation`` / ``message_kind``
     - Lowercase notation (``kvn`` / ``xml``) and family (such as ``opm``).
   * - ``source_edition``
     - Input edition, or the model's edition on generation failure.
   * - ``field_path``
     - Canonical model path when the containing validator supplies one.
   * - ``line`` / ``column``
     - One-based source location; XML failures can lack a location.
   * - ``byte_offset``
     - Zero-based byte position in the input.
   * - ``original_token``
     - Source excerpt of at most 128 characters.
   * - ``expected``
     - Expected parser token when available; excludes internal parser labels.

Stable codes
~~~~~~~~~~~~

Rust ``CcsdsNdmError::code()`` and Python ``error.code`` use the same categories, except the
Python typed-reader mismatch code, which describes that binding's class selection. Parse and
generation wrappers preserve the source error class and available field path. Pure model
validation has no wire notation or source token; Rust ``diagnostic()`` returns ``None`` there.

.. list-table::
   :header-rows: 1
   :widths: 35 35 30

   * - Code
     - Rust source variant
     - Python class
   * - ``parse.kvn.syntax``
     - ``Format::Kvn`` (or parse-wrapped ``InvalidFormat``)
     - ``NdmKvnParseError`` / ``NdmFormatError``
   * - ``parse.xml.syntax``
     - ``Format::Xml``, ``XmlDe``, ``XmlWithContext`` (or parse-wrapped ``InvalidFormat``)
     - ``NdmXmlError`` / ``NdmFormatError``
   * - ``parse.invalid_float`` / ``parse.invalid_integer`` / ``parse.invalid_enum``
     - ``Format::ParseFloat`` / ``ParseInt`` / ``Enum``
     - ``NdmFormatError``
   * - ``parse.invalid_epoch``
     - ``Epoch``
     - ``NdmEpochError``
   * - ``parse.unsupported_message_type``
     - ``UnsupportedMessage``
     - ``NdmUnsupportedMessageError``
   * - ``parse.unexpected_message_type``
     - Python typed-reader family check
     - ``NdmUnsupportedMessageError``
   * - ``parse.unsupported_input_version`` / ``generation.unsupported_output_version``
     - ``UnsupportedInputVersion`` / ``UnsupportedOutputVersion``
     - ``NdmValidationError``
   * - ``parse.unexpected_eof``
     - ``UnexpectedEof``
     - ``NdmFormatError``
   * - ``validation.missing_required_field`` / ``validation.invalid_choice``
     - ``Validation::MissingRequiredField`` / ``InvalidChoice``
     - ``NdmValidationError``
   * - ``validation.invalid_value`` / ``validation.out_of_range``
     - ``Validation::InvalidValue`` or ``EmptyValue`` / ``OutOfRange``
     - ``NdmValidationError``
   * - ``validation.conflict`` / ``validation.generic``
     - ``Validation::Conflict`` / ``Generic``
     - ``NdmValidationError``
   * - ``generation.xml.syntax``
     - ``Format::XmlSer``
     - ``NdmXmlError``
   * - ``format.invalid``
     - Unwrapped ``Format::InvalidFormat``
     - ``NdmFormatError``
   * - ``io.error``
     - ``Io``
     - ``NdmIoError`` (also has ``errno`` and ``filename`` for file entry points)
   * - ``unsupported.notation``
     - ``UnsupportedNotation``
     - ``NdmUnsupportedNotationError``
   * - ``resource.xml_depth_limit_exceeded`` / ``resource.limit_exceeded``
     - ``ResourceLimitExceeded``
     - ``NdmError``

.. autoexception:: NdmError
   :members:
   :show-inheritance:
.. autoexception:: NdmFormatError
   :members:
   :show-inheritance:
.. autoexception:: NdmKvnParseError
   :members:
   :show-inheritance:
.. autoexception:: NdmXmlError
   :members:
   :show-inheritance:
.. autoexception:: NdmValidationError
   :members:
   :show-inheritance:
.. autoexception:: NdmEpochError
   :members:
   :show-inheritance:
.. autoexception:: NdmIoError
   :members:
   :show-inheritance:
.. autoexception:: NdmUnsupportedMessageError
   :members:
   :show-inheritance:
.. autoexception:: NdmUnsupportedNotationError
   :members:
   :show-inheritance:

.. autoclass:: RelativeStateVector
   :members:
   :undoc-members:

.. autoclass:: SpacecraftParameters
   :members:
   :undoc-members:
