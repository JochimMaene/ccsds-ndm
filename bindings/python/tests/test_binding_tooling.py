# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""Regression tests for the shared binding-maintenance tooling."""

from audit_bindings import ccsds_reference
from binding_source import parse_python_binding_file, parse_rust_file

STRUCTS = """\
/// A message.
///
/// **CCSDS Reference**: 508.0-B-1, Section 1.
pub struct Message {
    pub undocumented_first: String,
    /// The second field.
    pub documented_second: String,
}

/// Another message.
pub struct Other {
    /// The first field.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 2.
    pub documented_first: String,
}
"""


def test_struct_docstring_does_not_leak_onto_an_undocumented_first_field(tmp_path):
    path = tmp_path / "sample.rs"
    path.write_text(STRUCTS)
    structs = parse_rust_file(path)

    message = structs["Message"]
    assert "CCSDS Reference" in message.docstring
    assert message.fields["undocumented_first"].docstring == ""
    assert message.fields["documented_second"].docstring == "The second field."

    # A documented first field must still be read correctly.
    other = structs["Other"]
    assert "Section 2." in other.fields["documented_first"].docstring


def test_ccsds_reference_ignores_harmless_formatting_but_not_the_citation():
    core = "**CCSDS Reference**:  508.0-B-1,  Section 3.2."
    python = "CCSDS Reference: 508.0-B-1, Section 3.2"
    assert ccsds_reference(core) == ccsds_reference(python) == "508.0-B-1, Section 3.2"

    assert ccsds_reference(
        "CCSDS Reference: 999.9-Z-9, Section 42."
    ) != ccsds_reference(core)
    assert ccsds_reference("no citation here") is None
    assert ccsds_reference(None) is None


BINDING = """\
#[gen_stub_pyclass]
#[pyclass]
pub struct Example {
    inner: core::Example,
}

#[gen_stub_pymethods]
#[pymethods]
impl Example {
    /// The plain field.
    ///
    /// :type: str
    #[getter]
    fn get_plain(&self) -> String {
        self.inner.plain.clone()
    }

    /// The annotated field.
    ///
    /// CCSDS Reference: 502.0-B-3, Section 6.
    ///
    /// :type: list[Union[Oem, Cdm]]
    #[gen_stub(override_return_type(imports=("typing"), type_repr="list[typing.Union[Oem, Cdm]]"))]
    #[getter]
    fn get_annotated(&self, py: Python<'_>) -> Py<PyList> {
        self.annotated.clone_ref(py)
    }

    #[gen_stub(override_type(type_repr="builtins.str | os.PathLike[builtins.str]"))]
    #[setter]
    fn set_annotated(&mut self, value: Vec<Py<PyAny>>) -> PyResult<()> {
        Ok(())
    }
}
"""


def test_docstrings_survive_override_attributes_between_doc_and_getter(tmp_path):
    # `#[gen_stub(...)]` sits between the doc comment and `#[getter]`, and its arguments
    # nest brackets two deep. The parser must still pair the docstring with the getter,
    # or the audit reports every annotated field as undocumented.
    path = tmp_path / "binding.rs"
    path.write_text(BINDING)
    classes = parse_python_binding_file(path)

    example = classes["Example"]
    assert set(example.getters) == {"plain", "annotated"}
    assert example.getters["plain"].docstring.startswith("The plain field.")

    annotated = example.getters["annotated"]
    assert annotated.docstring.startswith("The annotated field.")
    assert "CCSDS Reference: 502.0-B-3, Section 6." in annotated.docstring
    assert annotated.has_type_annotation
    assert "annotated" in example.setters


NATIVE_ACCESSORS = """\
#[gen_stub_pyclass]
#[pyclass]
pub struct Example {
    /// The read-only identifier.
    ///
    /// :type: Optional[str]
    #[pyo3(get)]
    id: Option<String>,

    /// The nested child.
    ///
    /// CCSDS Reference: 502.0-B-3, Section 6.
    ///
    /// :type: Child
    #[pyo3(get, set)]
    child: Py<Child>,

    /// Write-only by design.
    ///
    /// :type: str
    #[pyo3(set)]
    write_only: String,

    /// Renamed, but not a property.
    ///
    /// :type: str
    #[pyo3(name = "renamed")]
    not_a_property: String,

    plain_field: String,
}

#[gen_stub_pymethods]
#[pymethods]
impl Example {
    /// Validating setter stays handwritten.
    #[setter]
    fn set_id(&mut self, value: String) -> PyResult<()> {
        Ok(())
    }
}
"""


def test_native_field_accessors_are_recognised_with_their_docs(tmp_path):
    # `#[pyo3(get, set)]` declares a property with no `#[getter]` method, and its
    # documentation lives on the field. The audit reports such fields as unexposed
    # and undocumented unless the parser reads them.
    path = tmp_path / "binding.rs"
    path.write_text(NATIVE_ACCESSORS)
    example = parse_python_binding_file(path)["Example"]

    assert set(example.getters) == {"id", "child"}
    assert "plain_field" not in example.getters, (
        "a field without #[pyo3(...)] is not a property"
    )

    # `get` and `set` are independent. Registering a getter for a `#[pyo3(set)]` field
    # would hide a setter-without-getter violation from the audit, and a `#[pyo3(name)]`
    # field declares no accessor at all.
    assert "write_only" not in example.getters
    assert "write_only" in example.setters
    assert "not_a_property" not in example.getters
    assert "not_a_property" not in example.setters

    assert example.getters["id"].docstring.startswith("The read-only identifier.")
    assert example.getters["id"].has_type_annotation
    assert "id" in example.setters, "the handwritten #[setter] still registers"

    child = example.getters["child"]
    assert "CCSDS Reference: 502.0-B-3, Section 6." in child.docstring
    assert "child" in example.setters, "#[pyo3(get, set)] declares a setter"
