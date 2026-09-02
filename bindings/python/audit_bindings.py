#!/usr/bin/env python3
"""
Python binding audit tool.

Validates that Rust core structs are properly exposed in Python bindings.
Reports missing field exposures, missing docstrings, and documentation gaps.
Designed for use as a pre-commit hook.

Usage:
    uv run python audit_bindings.py
"""

from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Literal

from binding_mappings import (
    get_read_only_reason,
    get_rust_path,
    get_rust_struct_name,
    is_python_helper_class,
    is_python_only,
    should_skip_rust_field,
)
from binding_source import (
    PythonClass,
    RustStruct,
    collect_rust_structs,
    parse_python_binding_file,
)

# ---------------------------------------------------------------------------
# Data Classes
# ---------------------------------------------------------------------------


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
        "missing_setter",  # Getter exists but setter is missing and no read-only reason
        "setter_without_getter",  # Setter exists without matching getter
    ]
    message: str


@dataclass
class AuditResult:
    """Result of the audit."""

    issues: list[AuditIssue] = field(default_factory=list)
    stats: dict[str, int] = field(default_factory=dict)


def collect_python_classes(binding_dir: Path) -> dict[str, PythonClass]:
    """Collect all Python classes from binding files."""
    all_classes: dict[str, PythonClass] = {}

    for rs_file in sorted(binding_dir.glob("*.rs"), key=lambda p: p.name):
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
        "readonly_documented": 0,
        "missing_setter": 0,
        "setter_without_getter": 0,
    }

    for class_name, py_class in python_classes.items():
        for py_field in py_class.getters:
            if is_python_only(class_name, py_field) or py_field in py_class.setters:
                continue
            if get_read_only_reason(class_name, py_field) is None:
                result.stats["missing_setter"] += 1
                result.issues.append(
                    AuditIssue(
                        struct_name=class_name,
                        field_name=py_field,
                        issue_type="missing_setter",
                        message=(
                            f"Python getter '{class_name}.{py_field}' has no setter and no "
                            "documented read-only rationale"
                        ),
                    )
                )
            else:
                result.stats["readonly_documented"] += 1

        for py_field in py_class.setters - py_class.getters.keys():
            if is_python_only(class_name, py_field):
                continue
            result.stats["setter_without_getter"] += 1
            result.issues.append(
                AuditIssue(
                    struct_name=class_name,
                    field_name=py_field,
                    issue_type="setter_without_getter",
                    message=f"Python setter '{class_name}.{py_field}' exists without matching getter",
                )
            )

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


def print_report(result: AuditResult) -> None:
    """Print a human-readable audit report."""
    print("\n=== Python Binding Audit Report ===\n")

    # Group issues by struct
    issues_by_struct: dict[str, list[AuditIssue]] = {}
    for issue in result.issues:
        issues_by_struct.setdefault(issue.struct_name, []).append(issue)

    if result.issues:
        for struct_name, issues in sorted(issues_by_struct.items()):
            print(f"{struct_name}:")
            for issue in issues:
                icon = {
                    "missing_exposure": "✗",
                    "missing_docstring": "⚠",
                    "missing_type_annotation": "⚠",
                    "missing_rust_docstring": "ℹ",
                    "struct_not_found": "✗",
                    "missing_setter": "✗",
                    "setter_without_getter": "✗",
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
    if "readonly_documented" in result.stats:
        print(
            f"  Read-only fields with rationale: {result.stats['readonly_documented']}"
        )
    if "missing_setter" in result.stats:
        print(f"  Getters missing setter rationale: {result.stats['missing_setter']}")
    if "setter_without_getter" in result.stats:
        print(f"  Setters without getter: {result.stats['setter_without_getter']}")

    if result.issues:
        print(f"\n❌ {len(result.issues)} issues found")
    else:
        print("\n✓ All bindings validated successfully")


def main() -> int:
    script_dir = Path(__file__).parent
    core_dir = (script_dir / "../../ccsds-ndm/src").resolve()
    binding_dir = script_dir / "src"

    print(f"Core directory: {core_dir}")
    print(f"Binding directory: {binding_dir}")
    print("\nParsing Rust core library...")
    rust_structs = collect_rust_structs(core_dir)
    print(f"Found {len(rust_structs)} structs")
    print("Parsing Python bindings...")
    python_classes = collect_python_classes(binding_dir)
    print(f"Found {len(python_classes)} classes")
    print("\nAuditing bindings...")
    result = audit_bindings(rust_structs, python_classes)
    print_report(result)
    return bool(result.issues)


if __name__ == "__main__":
    raise SystemExit(main())
