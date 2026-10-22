"""Check documented diagnostic access on every exception, including inherited fields."""

from ccsds_ndm import (
    NdmEpochError,
    NdmError,
    NdmFormatError,
    NdmIoError,
    NdmKvnParseError,
    NdmUnsupportedMessageError,
    NdmUnsupportedNotationError,
    NdmValidationError,
    NdmXmlError,
)


def describe(
    error: NdmError
    | NdmFormatError
    | NdmKvnParseError
    | NdmXmlError
    | NdmValidationError
    | NdmEpochError
    | NdmIoError
    | NdmUnsupportedMessageError
    | NdmUnsupportedNotationError,
) -> str:
    text: tuple[str | None, ...] = (
        error.operation,
        error.notation,
        error.message_kind,
        error.source_edition,
        error.code,
        error.field_path,
        error.original_token,
        error.expected,
    )
    location: tuple[int | None, int | None, int | None] = (
        error.line,
        error.column,
        error.byte_offset,
    )
    return f"{text} at {location}"
