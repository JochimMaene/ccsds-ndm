# Contributing

Install Rust, Python 3.10+, [uv](https://docs.astral.sh/uv/), and
[just](https://github.com/casey/just), then run:

```console
just setup
just dev
just check
```

`just --list` shows the focused format, lint, test, documentation, benchmark, and package commands.

## Changes

- Keep parsing, validation, and generation rules in the Rust core. Python should adapt inputs and
  expose the same behavior.
- Cite the applicable CCSDS publication edition and section or table beside non-obvious rules.
  The local schemas and extracted books are in `ccsds-ndm/data/xsd/` and `docs/ccsds-books/`.
  `ndmxml-4.0.0-*` is the current schema set and `ndmxml-2.0.0-*` holds SANA's ODM 2.0
  rebuilds. `ndmxml-1.0-*` is the complete original NDM/XML 1.0 set (505.0-B-1, labelled
  03/14/2010), archived from the only Internet Archive capture of sanaregistry.org/r/ndmxml/
  (2011-12-06) because SANA no longer hosts it. Only its TDM files are used by tests today.
- Add the smallest test that distinguishes the required behavior. Use official examples and XSD
  validation where they provide independent evidence.
- Apply the same [verification criteria](docs/conformance/README.md#verification-criteria) to every
  message family. Update its decision record with the rule, authority, limitation, and regression;
  shared dispatch tests alone do not establish conformance.
- Run `just stubs` after changing the Python API and `just audit` after changing exposed model
  fields. Commit the updated stub.
- Run `just check` before opening a pull request. Use `just verify` when changing packaging.

The project is pre-1.0, so APIs may change between releases. Explain breaking changes and their
replacement in the release notes.

## Test data

Keep valid fixtures in `ccsds-ndm/data/kvn/` or `ccsds-ndm/data/xml/`, and named negative examples
in `ccsds-ndm/data/invalid/`. Record their source, distinguish corrections from the published
example, and avoid sensitive operational data.
