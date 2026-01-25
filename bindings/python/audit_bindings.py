#!/usr/bin/env python3
"""
Python binding audit tool.

Validates that Rust core structs are properly exposed in Python bindings.
Reports missing field exposures, missing docstrings, and documentation gaps.
Designed for use as a pre-commit hook.

Usage:
    uv run python audit_bindings.py              # Run audit, report issues
    uv run python audit_bindings.py --strict     # Exit 1 if any issues found
    uv run python audit_bindings.py --verbose    # Show all checked items
    uv run python audit_bindings.py --json       # Output as JSON for CI
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Literal

from binding_mappings import (
    get_rust_path,
    get_rust_struct_name,
    is_python_helper_class,
    is_python_only,
    should_skip_rust_field,
)

# ---------------------------------------------------------------------------
# Data Classes
# ---------------------------------------------------------------------------


@dataclass
class RustField:
    """A field in a Rust struct."""

    name: str
    rust_type: str
    has_docstring: bool


@dataclass
class RustStruct:
    """A Rust struct with its fields."""

    name: str
    fields: dict[str, RustField] = field(default_factory=dict)
    has_docstring: bool = False


@dataclass
class PythonGetter:
    """A Python getter in a PyO3 binding."""

    name: str  # e.g., "object_name" (without get_ prefix)
    has_docstring: bool
    has_type_annotation: bool  # Has :type: in docstring


@dataclass
class PythonClass:
    """A Python class in a PyO3 binding."""

    name: str
    getters: dict[str, PythonGetter] = field(default_factory=dict)
    has_docstring: bool = False


@dataclass
class AuditIssue:
    """An issue found during audit."""

    struct_name: str
    field_name: str | None
    issue_type: Literal[
        "missing_exposure",  # Rust field not exposed in Python
        "missing_docstring",  # Python getter lacks docstring
        "missing_type_annotation",  # Python getter lacks :type:
        "missing_rust_docstring",  # Rust field lacks docstring
        "struct_not_found",  # Python class has no matching Rust struct
    ]
    message: str


@dataclass
class AuditResult:
    """Result of the audit."""

    issues: list[AuditIssue] = field(default_factory=list)
    stats: dict[str, int] = field(default_factory=dict)


# ---------------------------------------------------------------------------
# Rust Parser
# ---------------------------------------------------------------------------


def parse_rust_struct(content: str, struct_match: re.Match) -> RustStruct | None:
    """Parse a single Rust struct and extract its fields."""
    struct_name = struct_match.group(1)
    struct_start = content[: struct_match.start()].count("\n")

    # Check if struct has docstring
    lines = content.split("\n")
    has_docstring = False
    idx = struct_start - 1
    while idx >= 0:
        line = lines[idx].strip()
        if line.startswith("///"):
            has_docstring = True
            break
        elif line.startswith("#[") or line == "":
            idx -= 1
        else:
            break

    rust_struct = RustStruct(name=struct_name, has_docstring=has_docstring)

    # Find struct body
    brace_start = struct_match.end() - 1
    if content[brace_start] != "{":
        brace_start = content.find("{", struct_match.end())
    if brace_start == -1:
        return rust_struct

    # Find matching closing brace
    depth = 1
    pos = brace_start + 1
    while pos < len(content) and depth > 0:
        if content[pos] == "{":
            depth += 1
        elif content[pos] == "}":
            depth -= 1
        pos += 1

    struct_body = content[brace_start + 1 : pos - 1]
    struct_body_start_char = brace_start + 1

    # Parse fields with pub keyword
    field_pattern = re.compile(r"^\s*pub\s+(\w+)\s*:\s*([^,\n]+)", re.MULTILINE)

    for match in field_pattern.finditer(struct_body):
        field_name = match.group(1)
        field_type = match.group(2).strip().rstrip(",")

        # Check if field has docstring
        field_char_in_content = struct_body_start_char + match.start()
        field_line = content[:field_char_in_content].count("\n")

        field_has_doc = False
        idx = field_line - 1
        while idx >= 0:
            line = lines[idx].strip()
            if line.startswith("///"):
                field_has_doc = True
                break
            elif line.startswith("#[") or line == "":
                idx -= 1
            else:
                break

        rust_struct.fields[field_name] = RustField(
            name=field_name, rust_type=field_type, has_docstring=field_has_doc
        )

    return rust_struct


def parse_rust_file(filepath: Path) -> dict[str, RustStruct]:
    """Parse a Rust file and extract all structs."""
    content = filepath.read_text()
    structs: dict[str, RustStruct] = {}

    struct_pattern = re.compile(
        r"pub\s+struct\s+(\w+)\s*(?:<[^>]*>)?\s*\{", re.MULTILINE
    )

    for match in struct_pattern.finditer(content):
        rust_struct = parse_rust_struct(content, match)
        if rust_struct:
            structs[rust_struct.name] = rust_struct

    return structs


def collect_rust_structs(core_dir: Path) -> dict[str, RustStruct]:
    """Collect all Rust structs from the core library."""
    all_structs: dict[str, RustStruct] = {}

    # Parse common.rs
    common_path = core_dir / "common.rs"
    if common_path.exists():
        all_structs.update(parse_rust_file(common_path))

    # Parse types.rs
    types_path = core_dir / "types.rs"
    if types_path.exists():
        all_structs.update(parse_rust_file(types_path))

    # Parse messages/*.rs
    messages_dir = core_dir / "messages"
    if messages_dir.exists():
        for rs_file in messages_dir.glob("*.rs"):
            if rs_file.name != "mod.rs":
                all_structs.update(parse_rust_file(rs_file))

    return all_structs


# ---------------------------------------------------------------------------
# Python Binding Parser
# ---------------------------------------------------------------------------


def parse_python_binding_file(filepath: Path) -> dict[str, PythonClass]:
    """Parse a Python binding Rust file and extract pyclass structs with getters."""
    content = filepath.read_text()
    classes: dict[str, PythonClass] = {}

    # Find #[pyclass] structs
    pyclass_pattern = re.compile(
        r"((?:\s*///[^\n]*\n)*)\s*(#\[pyclass[^\]]*\]\s*(?:#\[[^\]]*\]\s*)*)pub\s+struct\s+(\w+)",
        re.MULTILINE,
    )

    for match in pyclass_pattern.finditer(content):
        docstring_block = match.group(1)
        class_name = match.group(3)

        has_docstring = bool(docstring_block.strip())
        classes[class_name] = PythonClass(name=class_name, has_docstring=has_docstring)

    # Find #[pymethods] impl blocks
    impl_pattern = re.compile(r"#\[pymethods\]\s*impl\s+(\w+)\s*\{", re.MULTILINE)

    for impl_match in impl_pattern.finditer(content):
        class_name = impl_match.group(1)
        if class_name not in classes:
            continue

        # Find the end of this impl block
        impl_start = impl_match.end()
        depth = 1
        pos = impl_start
        while pos < len(content) and depth > 0:
            if content[pos] == "{":
                depth += 1
            elif content[pos] == "}":
                depth -= 1
            pos += 1

        impl_body = content[impl_start : pos - 1]

        # Find getters in this impl block
        getter_pattern = re.compile(
            r"((?:\s*///[^\n]*\n)*)\s*#\[getter\]\s*\n(?:\s*#\[[^\]]*\]\s*\n)*\s*fn\s+(get_)?(\w+)\s*\(",
            re.MULTILINE,
        )

        for getter_match in getter_pattern.finditer(impl_body):
            docstring_block = getter_match.group(1)
            func_name_part = getter_match.group(3)

            # Field name is the function name without get_ prefix
            field_name = func_name_part

            has_docstring = bool(docstring_block.strip())
            has_type_annotation = ":type:" in docstring_block

            classes[class_name].getters[field_name] = PythonGetter(
                name=field_name,
                has_docstring=has_docstring,
                has_type_annotation=has_type_annotation,
            )

    return classes


def collect_python_classes(binding_dir: Path) -> dict[str, PythonClass]:
    """Collect all Python classes from binding files."""
    all_classes: dict[str, PythonClass] = {}

    for rs_file in binding_dir.glob("*.rs"):
        if rs_file.name in ("lib.rs", "mod.rs"):
            continue
        classes = parse_python_binding_file(rs_file)
        all_classes.update(classes)

    return all_classes


# ---------------------------------------------------------------------------
# Audit Logic
# ---------------------------------------------------------------------------


def audit_bindings(
    rust_structs: dict[str, RustStruct],
    python_classes: dict[str, PythonClass],
) -> AuditResult:
    """Audit Python bindings against Rust structs."""
    result = AuditResult()
    result.stats = {
        "structs_checked": 0,
        "fields_exposed": 0,
        "fields_total": 0,
        "missing_exposure": 0,
        "missing_docstring": 0,
        "missing_type_annotation": 0,
        "python_only_fields": 0,
    }

    for class_name, py_class in python_classes.items():
        # Check if this is a Python-only helper class (no matching Rust struct)
        if is_python_helper_class(class_name):
            result.stats["python_only_fields"] += 1  # Count as OK
            continue

        # Check if the whole class should be skipped (found but intentionally ignored)
        if should_skip_rust_field(class_name, "*"):
            continue

        result.stats["structs_checked"] += 1

        # Get corresponding Rust struct
        rust_struct_name = get_rust_struct_name(class_name)
        rust_struct = rust_structs.get(rust_struct_name)

        if not rust_struct:
            result.issues.append(
                AuditIssue(
                    struct_name=class_name,
                    field_name=None,
                    issue_type="struct_not_found",
                    message=f"No Rust struct found for Python class '{class_name}'",
                )
            )
            continue

        # Check each Rust field is exposed in Python
        for field_name, rust_field in rust_struct.fields.items():
            if should_skip_rust_field(rust_struct_name, field_name):
                continue

            result.stats["fields_total"] += 1

            # Check if field has a corresponding Python getter
            # Apply field mapping to find the Python getter name
            python_field_name = None
            for py_field in py_class.getters.keys():
                if get_rust_path(class_name, py_field) == field_name:
                    python_field_name = py_field
                    break

            if python_field_name is None and field_name in py_class.getters:
                python_field_name = field_name

            if python_field_name is None:
                result.stats["missing_exposure"] += 1
                result.issues.append(
                    AuditIssue(
                        struct_name=class_name,
                        field_name=field_name,
                        issue_type="missing_exposure",
                        message=f"Rust field '{rust_struct_name}.{field_name}' not exposed in Python",
                    )
                )
                continue

            result.stats["fields_exposed"] += 1
            py_getter = py_class.getters[python_field_name]

            # Check docstring
            if not py_getter.has_docstring:
                result.stats["missing_docstring"] += 1
                result.issues.append(
                    AuditIssue(
                        struct_name=class_name,
                        field_name=python_field_name,
                        issue_type="missing_docstring",
                        message=f"Python getter '{class_name}.{python_field_name}' lacks docstring",
                    )
                )

            # Check :type: annotation
            if not py_getter.has_type_annotation:
                result.stats["missing_type_annotation"] += 1
                result.issues.append(
                    AuditIssue(
                        struct_name=class_name,
                        field_name=python_field_name,
                        issue_type="missing_type_annotation",
                        message=f"Python getter '{class_name}.{python_field_name}' lacks :type: annotation",
                    )
                )

        # Count Python-only fields
        for py_field in py_class.getters.keys():
            if is_python_only(class_name, py_field):
                result.stats["python_only_fields"] += 1

    return result


# ---------------------------------------------------------------------------
# Reporting
# ---------------------------------------------------------------------------


def print_report(result: AuditResult, verbose: bool = False) -> None:
    """Print a human-readable audit report."""
    print("\n=== Python Binding Audit Report ===\n")

    # Group issues by struct
    issues_by_struct: dict[str, list[AuditIssue]] = {}
    for issue in result.issues:
        issues_by_struct.setdefault(issue.struct_name, []).append(issue)

    if verbose or result.issues:
        for struct_name, issues in sorted(issues_by_struct.items()):
            print(f"{struct_name}:")
            for issue in issues:
                icon = {
                    "missing_exposure": "✗",
                    "missing_docstring": "⚠",
                    "missing_type_annotation": "⚠",
                    "missing_rust_docstring": "ℹ",
                    "struct_not_found": "✗",
                }[issue.issue_type]
                print(f"  {icon} {issue.message}")
            print()

    # Summary
    print("Summary:")
    print(f"  Structs checked: {result.stats['structs_checked']}")
    print(
        f"  Fields exposed: {result.stats['fields_exposed']}/{result.stats['fields_total']}"
    )
    print(f"  Missing exposure: {result.stats['missing_exposure']}")
    print(f"  Missing docstrings: {result.stats['missing_docstring']}")
    print(f"  Missing :type: annotations: {result.stats['missing_type_annotation']}")
    print(f"  Python-only fields: {result.stats['python_only_fields']} (OK)")

    if result.issues:
        print(f"\n❌ {len(result.issues)} issues found")
    else:
        print("\n✓ All bindings validated successfully")


def print_json(result: AuditResult) -> None:
    """Print audit result as JSON."""
    output = {
        "issues": [
            {
                "struct": issue.struct_name,
                "field": issue.field_name,
                "type": issue.issue_type,
                "message": issue.message,
            }
            for issue in result.issues
        ],
        "stats": result.stats,
    }
    print(json.dumps(output, indent=2))


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Audit Python bindings against Rust core structs"
    )
    parser.add_argument(
        "--core-dir",
        type=Path,
        default=Path("../../ccsds-ndm/src"),
        help="Path to ccsds-ndm/src directory",
    )
    parser.add_argument(
        "--binding-dir",
        type=Path,
        default=Path("src"),
        help="Path to bindings/python/src directory",
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="Exit with code 1 if any issues found (for CI)",
    )
    parser.add_argument(
        "--verbose",
        "-v",
        action="store_true",
        help="Show all checked items",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Output as JSON",
    )

    args = parser.parse_args()

    # Resolve paths
    script_dir = Path(__file__).parent
    core_dir = (script_dir / args.core_dir).resolve()
    binding_dir = (script_dir / args.binding_dir).resolve()

    if not core_dir.exists():
        print(f"Error: Core directory not found: {core_dir}", file=sys.stderr)
        return 1

    if not binding_dir.exists():
        print(f"Error: Binding directory not found: {binding_dir}", file=sys.stderr)
        return 1

    if not args.json:
        print(f"Core directory: {core_dir}")
        print(f"Binding directory: {binding_dir}")

    # Collect structs
    if not args.json:
        print("\nParsing Rust core library...")
    rust_structs = collect_rust_structs(core_dir)
    if not args.json:
        print(f"Found {len(rust_structs)} structs")

    if not args.json:
        print("Parsing Python bindings...")
    python_classes = collect_python_classes(binding_dir)
    if not args.json:
        print(f"Found {len(python_classes)} classes")

    # Run audit
    if not args.json:
        print("\nAuditing bindings...")
    result = audit_bindings(rust_structs, python_classes)

    # Output
    if args.json:
        print_json(result)
    else:
        print_report(result, verbose=args.verbose)

    # Exit code
    if args.strict and result.issues:
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
