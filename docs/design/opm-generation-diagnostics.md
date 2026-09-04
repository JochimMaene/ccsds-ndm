# Minimal OPM Generation Diagnostic Context

Status: accepted for the OPM 3.0 completion work.

Generation errors retain the existing `CcsdsNdmError` and `ValidationError` categories. Public OPM
generation boundaries add one lightweight context wrapper only on failure. `CcsdsNdmError::diagnostic`
then exposes a borrowed view containing:

- the `generate` operation;
- KVN or XML output notation;
- OPM message kind;
- the message edition;
- the existing stable error code and optional model path;
- an optional normative requirement identifier;
- optional source location fields, which are `None` for public-model generation.

The context owns only the edition string. It is allocated only after generation has
already failed. Reading the view borrows the stored context and underlying error; it does not copy
diagnostic strings. Existing `code()` and `field_path()` accessors continue to work through the
wrapper.

This deliberately does not introduce a recovery framework or success-path context object. Parsing
extends the view when it has a
concrete need for bounded token and location data.

Before 1.0, stable compatibility covers diagnostic codes, the enum meanings exposed by the view,
and canonical model paths. Human-readable wording and the set of requirement identifiers may grow.
