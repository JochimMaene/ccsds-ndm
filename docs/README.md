# Documentation

Start with the document for the question you need answered:

| Question | Document |
| --- | --- |
| How do I install and use the library? | [Quick tour](guide/quicktour.rst), [workflows](guide/workflows.rst), and [API reference](api/python.rst) |
| Which messages and editions are supported? | [Support matrix](support-matrix.md) |
| What changed, including breaking changes? | [Release notes](release-notes.rst) |
| Why was a CCSDS rule interpreted this way? | [Conformance index and family decisions](conformance/README.md) |
| Where are validation rules enforced? | [Validation contract](design/validation-contract.md) |
| Why do Python objects have these ownership semantics? | [Python object model](design/python-object-model.md) |
| Does a proposed feature belong in the project? | [Project scope](project-goal.md) |

The user guide and API reference are built with Sphinx:

```console
uv sync
uv run sphinx-build -E -W -b html docs docs/_build/html
```

The Markdown documents are maintained on GitHub and linked from the published guide. Keep support
claims in the matrix, shared policy in the validation contract, and each field-specific decision
in its family record. Package READMEs repeat the installation and short examples needed on PyPI
and crates.io; they link to those maintained records rather than maintaining separate support lists.

The [`ccsds-books/`](ccsds-books/) directory is raw extracted reference material, excluded from
Sphinx. Extraction may omit tables, annexes, or later corrigenda. Consult the publication and
schema baseline cited in the family record when resolving a requirement; an extraction is not a
replacement for that authority. Local source PDFs are in `data/`, and bundled schemas are in
`ccsds-ndm/data/xsd/`.
