#!/usr/bin/env python3
"""
Rust-to-Python docstring synchronization script.

This script extracts docstrings from Rust core library struct fields and
synchronizes them with matching Python binding getter/setter methods.

Usage:
    uv run python sync_docstrings.py                    # Sync all docstrings
    uv run python sync_docstrings.py --check            # Check if in sync
    uv run python sync_docstrings.py --dry-run          # Preview changes
    uv run python sync_docstrings.py --report           # Generate mismatch report
"""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

from binding_mappings import (
    PYTHON_ONLY_FIELDS,
    get_rust_path,
    get_rust_struct_name,
)

# ---------------------------------------------------------------------------
# Data Classes
# ---------------------------------------------------------------------------


@dataclass
class RustField:
    """A field in a Rust struct with its docstring."""

    name: str
    docstring: str
    rust_type: str = ""


@dataclass
class RustStruct:
    """A Rust struct with its fields and docstrings."""

    name: str
    fields: dict[str, RustField] = field(default_factory=dict)
    docstring: str = ""


@dataclass
class PythonGetter:
    """A Python getter method in a PyO3 binding."""

    name: str  # e.g., "get_object_name"
    field_name: str  # e.g., "object_name"
    docstring: str
    line_start: int  # Line number where docstring starts (or where to insert)
    line_end: int  # Line number where docstring ends
    has_docstring: bool


@dataclass
class PythonClass:
    """A Python class in a PyO3 binding."""

    name: str
    getters: dict[str, PythonGetter] = field(default_factory=dict)
    docstring: str = ""
    line_start: int = 0
    line_end: int = 0
    has_docstring: bool = False


@dataclass
class SyncResult:
    """Result of synchronization for a single getter or class."""

    status: str  # "updated", "added", "unchanged", "no_rust_doc", "not_found"
    struct_name: str
    field_name: str | None = None  # None for class docstrings
    old_doc: str = ""
    new_doc: str = ""


# ---------------------------------------------------------------------------
# Rust Parser
# ---------------------------------------------------------------------------


def parse_rust_docstring(lines: list[str], end_idx: int) -> str:
    """
    Extract docstring lines (///) above a field or struct definition.

    Args:
        lines: All lines in the file
        end_idx: Index of the line containing the field/struct definition

    Returns:
        Combined docstring text
    """
    doc_lines: list[str] = []
    idx = end_idx - 1

    while idx >= 0:
        line = lines[idx].strip()
        if line.startswith("///"):
            # Extract content after ///
            content = line[3:].strip() if len(line) > 3 else ""
            doc_lines.insert(0, content)
            idx -= 1
        elif line.startswith("#[") or line == "":
            # Skip attributes and empty lines
            idx -= 1
        else:
            break

    return "\n".join(doc_lines)


def parse_rust_struct(content: str, struct_match: re.Match) -> RustStruct | None:
    """Parse a single Rust struct and extract its fields with docstrings."""
    lines = content.split("\n")
    struct_name = struct_match.group(1)
    struct_start = content[: struct_match.start()].count("\n")

    # Get struct docstring
    struct_doc = parse_rust_docstring(lines, struct_start)

    rust_struct = RustStruct(name=struct_name, docstring=struct_doc)

    # The struct_match already includes the opening brace
    brace_start = struct_match.end() - 1
    if content[brace_start] != "{":
        # Find it if not included
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
    # Pattern matches: pub field_name: Type, with optional attributes
    field_pattern = re.compile(r"^\s*pub\s+(\w+)\s*:\s*([^,\n]+)", re.MULTILINE)

    for match in field_pattern.finditer(struct_body):
        field_name = match.group(1)
        field_type = match.group(2).strip().rstrip(",")

        # Calculate absolute line number using character offset
        field_char_in_content = struct_body_start_char + match.start()
        field_line = content[:field_char_in_content].count("\n")

        # Get docstring for this field
        docstring = parse_rust_docstring(lines, field_line)

        rust_struct.fields[field_name] = RustField(
            name=field_name, docstring=docstring, rust_type=field_type
        )

    return rust_struct


def parse_rust_file(filepath: Path) -> dict[str, RustStruct]:
    """Parse a Rust file and extract all structs with their field docstrings."""
    content = filepath.read_text()
    structs: dict[str, RustStruct] = {}

    # Pattern to find struct definitions
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

    # Find #[pyclass] structs and their docstrings
    pyclass_pattern = re.compile(
        r"((?:\s*///[^\n]*\n)*)\s*(#\[pyclass[^\]]*\]\s*(?:#\[[^\]]*\]\s*)*)pub\s+struct\s+(\w+)",
        re.MULTILINE,
    )

    for match in pyclass_pattern.finditer(content):
        docstring_block = match.group(1)
        class_name = match.group(3)

        # Parse existing docstring
        doc_lines = []
        for line in docstring_block.split("\n"):
            stripped = line.strip()
            if stripped.startswith("///"):
                slash_idx = line.find("///")
                if slash_idx != -1:
                    line_content = line[slash_idx + 3 :]
                    if line_content.startswith(" "):
                        line_content = line_content[1:]
                    doc_lines.append(line_content.rstrip())

        existing_docstring = "\n".join(doc_lines)
        class_line_abs = content[: match.start(2)].count("\n")

        # Find where docstring starts/ends
        docstring_start = class_line_abs
        if docstring_block.strip():
            docstring_start = class_line_abs - docstring_block.count("\n")

        classes[class_name] = PythonClass(
            name=class_name,
            docstring=existing_docstring,
            line_start=docstring_start,
            line_end=class_line_abs,
            has_docstring=bool(doc_lines),
        )

    # Find #[pymethods] impl blocks
    impl_pattern = re.compile(r"#\[pymethods\]\s*impl\s+(\w+)\s*\{", re.MULTILINE)

    for impl_match in impl_pattern.finditer(content):
        class_name = impl_match.group(1)
        if class_name not in classes:
            continue

        # Find the end of this impl block correctly by tracking brace depth
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
        impl_body_start_line = content[:impl_start].count("\n")

        # Find getters in this impl block
        # Pattern matches docstring + #[getter] + optional attributes + fn
        getter_pattern = re.compile(
            r"((?:\s*///[^\n]*\n)*)\s*#\[getter\]\s*\n(?:\s*#\[[^\]]*\]\s*\n)*\s*fn\s+(get_)?(\w+)\s*\(",
            re.MULTILINE,
        )

        for getter_match in getter_pattern.finditer(impl_body):
            docstring_block = getter_match.group(1)
            prefix = getter_match.group(2) or ""
            func_name_part = getter_match.group(3)

            # Full function name and field name
            full_func_name = f"{prefix}{func_name_part}"
            field_name = func_name_part  # Field name is without get_ prefix

            # Parse existing docstring
            doc_lines = []
            for line in docstring_block.split("\n"):
                stripped = line.strip()
                if stripped.startswith("///"):
                    slash_idx = line.find("///")
                    if slash_idx != -1:
                        line_content = line[slash_idx + 3 :]
                        if line_content.startswith(" "):
                            line_content = line_content[1:]
                        doc_lines.append(line_content.rstrip())

            existing_docstring = "\n".join(doc_lines)

            # Calculate line numbers
            getter_line_in_impl = impl_body[: getter_match.start()].count("\n")
            getter_line_abs = impl_body_start_line + getter_line_in_impl

            # Find where docstring starts/ends
            docstring_start = getter_line_abs
            if docstring_block.strip():
                docstring_start = getter_line_abs - docstring_block.count("\n")

            classes[class_name].getters[field_name] = PythonGetter(
                name=full_func_name,
                field_name=field_name,
                docstring=existing_docstring,
                line_start=docstring_start,
                line_end=getter_line_abs,
                has_docstring=bool(doc_lines),
            )

    return classes


def collect_python_classes(binding_dir: Path) -> dict[str, dict[str, PythonClass]]:
    """Collect all Python classes from binding files."""
    all_classes: dict[str, dict[str, PythonClass]] = {}

    for rs_file in binding_dir.glob("*.rs"):
        if rs_file.name in ("lib.rs", "mod.rs"):
            continue
        classes = parse_python_binding_file(rs_file)
        if classes:
            all_classes[rs_file.name] = classes

    return all_classes


# ---------------------------------------------------------------------------
# Docstring Transformation
# ---------------------------------------------------------------------------


def transform_rust_to_python_docstring(
    rust_doc: str,
    existing_doc: str | None = None,
    is_class: bool = False,
    known_fields: set[str] | None = None,
) -> str:
    """
    Transform a Rust docstring to Python format.

    Preserves the :type: annotation for fields.
    For classes, attempts to preserve existing Parameters/Returns sections.
    """
    if not rust_doc:
        return ""

    lines = rust_doc.split("\n")
    result_lines: list[str] = []

    # Process lines, removing **CCSDS Reference** but keeping other content
    for line in lines:
        # Skip CCSDS Reference lines for Python docs (too verbose)
        if "**CCSDS Reference**:" in line:
            continue
        # Skip empty **Examples**: headers without content
        if line.strip() == "**Examples**:":
            continue
        # Convert **Units**: X to more compact format
        if line.strip().startswith("**Units**:"):
            units = line.split("**Units**:")[-1].strip()
            if units:
                result_lines.append(f"Units: {units}")
            continue
        # Convert **Examples**: X to compact format
        if line.strip().startswith("**Examples**:"):
            examples = line.split("**Examples**:")[-1].strip()
            if examples:
                result_lines.append(f"Examples: {examples}")
            continue

        result_lines.append(line)

    # Clean up empty lines at start/end
    while result_lines and not result_lines[0].strip():
        result_lines.pop(0)
    while result_lines and not result_lines[-1].strip():
        result_lines.pop()

    # Join
    doc = "\n".join(result_lines)

    if not is_class:
        # Field specific: Add type hint at the end if provided
        type_hint = extract_type_hint(existing_doc) if existing_doc else None
        if type_hint:
            doc = doc.rstrip() + f"\n\n:type: {type_hint}"
    else:
        # Class specific: Preserve Parameters/Returns/Attributes sections from existing doc
        if existing_doc:
            sections_to_keep = []

            def extract_section(text, name, sep_len=10):
                # Look for section header like "Parameters\n----------"
                pattern = rf"({name}\n\s*-{{{sep_len},}})"
                parts = re.split(pattern, text)
                if len(parts) > 1:
                    # Content is in parts[2], but might be cut off by next section
                    content = parts[2]
                    # Find next section or end
                    next_sec = re.search(r"\n[A-Z][a-z]+\n\s*-+", content)
                    if next_sec:
                        content = content[: next_sec.start()]
                    return (parts[1] + content).rstrip()
                return None

            for section in ["Parameters", "Attributes", "Returns"]:
                extracted = extract_section(
                    existing_doc, section, 7 if section == "Returns" else 10
                )
                if extracted:
                    # Standardize indentation for the section content
                    lines = extracted.split("\n")
                    if len(lines) > 2:
                        formatted_lines = [lines[0], lines[1]]
                        for line in lines[2:]:
                            stripped = line.strip()
                            if not stripped:
                                formatted_lines.append("")
                                continue

                            # Determine indentation
                            indent = "    "  # Default to 4 spaces (description)

                            # Check if it is a parameter definition
                            if known_fields and section in ("Parameters", "Attributes"):
                                # Check if line starts with a known field name followed by space or colon
                                parts = stripped.split(":")
                                name = parts[0].strip()
                                if name in known_fields:
                                    indent = ""
                            elif section == "Returns":
                                # Heuristic for Returns: Type usually has no spaces
                                if " " not in stripped:
                                    indent = ""

                            formatted_lines.append(indent + stripped)

                        extracted = "\n".join(formatted_lines)
                    sections_to_keep.append(extracted)

            if sections_to_keep:
                doc = doc.rstrip() + "\n\n" + "\n\n".join(sections_to_keep).strip()

    return doc


def extract_type_hint(docstring: str) -> str | None:
    """Extract :type: annotation from existing docstring."""
    match = re.search(r":type:\s*(.+?)(?:\n|$)", docstring)
    if match:
        return match.group(1).strip()
    return None


# ---------------------------------------------------------------------------
# Synchronization Logic
# ---------------------------------------------------------------------------


def _clean_rust_type(type_str: str) -> str:
    """Extract inner type from Option<T>, Vec<T>, etc. to find struct name."""
    type_str = type_str.strip()
    if "<" in type_str and type_str.endswith(">"):
        struct_match = re.search(r"^\w+\s*<(.+)>$", type_str)
        if struct_match:
            return _clean_rust_type(struct_match.group(1))
    return type_str


def _resolve_rust_field(
    root_struct: RustStruct,
    field_path: str,
    all_structs: dict[str, RustStruct],
) -> RustField | None:
    """Resolve a dot-separated field path (e.g. 'body.segment') to a RustField."""
    current_struct = root_struct
    parts = field_path.split(".")

    for i, part in enumerate(parts):
        field = current_struct.fields.get(part)
        if not field:
            return None

        if i == len(parts) - 1:
            return field

        # Navigate to next struct
        type_name = _clean_rust_type(field.rust_type)

        # Determine strict name lookup (ignore crate prefixes if any)
        if "::" in type_name:
            type_name = type_name.split("::")[-1]

        next_struct = all_structs.get(type_name)
        if not next_struct:
            # Handle some common renames if necessary, or just fail
            return None

        current_struct = next_struct

    return None


def sync_docstrings(
    rust_structs: dict[str, RustStruct],
    python_classes: dict[str, dict[str, PythonClass]],
    binding_dir: Path,
    dry_run: bool = False,
) -> list[SyncResult]:
    """Synchronize docstrings from Rust to Python bindings."""
    results: list[SyncResult] = []

    for filename, classes in python_classes.items():
        filepath = binding_dir / filename
        content = filepath.read_text()
        modified = False
        changes: list[tuple[int, int, str, str, bool]] = []

        for class_name, py_class in classes.items():
            # Find matching Rust struct
            rust_struct_name = get_rust_struct_name(class_name)
            rust_struct = rust_structs.get(rust_struct_name)

            if not rust_struct:
                results.append(
                    SyncResult(
                        struct_name=class_name,
                        status="struct_not_found",
                    )
                )
                continue

            # 1. Sync class docstring
            if rust_struct.docstring:
                known_fields = set(rust_struct.fields.keys()) | set(
                    py_class.getters.keys()
                )
                new_class_doc = transform_rust_to_python_docstring(
                    rust_struct.docstring,
                    py_class.docstring,
                    is_class=True,
                    known_fields=known_fields,
                )
                if (
                    new_class_doc
                    and new_class_doc.strip() != py_class.docstring.strip()
                ):
                    results.append(
                        SyncResult(
                            struct_name=class_name,
                            status="updated" if py_class.has_docstring else "added",
                            old_doc=py_class.docstring,
                            new_doc=new_class_doc,
                        )
                    )
                    changes.append(
                        (
                            py_class.line_start,
                            py_class.line_end,
                            py_class.docstring,
                            new_class_doc,
                            True,  # is_class_sync
                        )
                    )
                    modified = True
                else:
                    results.append(
                        SyncResult(
                            struct_name=class_name,
                            status="unchanged",
                        )
                    )

            # 2. Sync field (getter) docstrings
            for field_name, getter in py_class.getters.items():
                # Skip Python-only fields
                if field_name in PYTHON_ONLY_FIELDS.get(class_name, []):
                    continue

                # Resolve Rust field path using mappings
                rust_field_path = get_rust_path(class_name, field_name)

                # Skip if Rust field is marked as skipped (though usually this means not exposed)
                # We check anyway if we can resolve it

                rust_field = _resolve_rust_field(
                    rust_struct, rust_field_path, rust_structs
                )

                if not rust_field:
                    results.append(
                        SyncResult(
                            struct_name=class_name,
                            field_name=field_name,
                            status="field_not_found",
                        )
                    )
                    continue

                if not rust_field.docstring:
                    results.append(
                        SyncResult(
                            struct_name=class_name,
                            field_name=field_name,
                            status="no_rust_doc",
                        )
                    )
                    continue

                # Transform Rust docstring to Python format
                new_docstring = transform_rust_to_python_docstring(
                    rust_field.docstring, getter.docstring, is_class=False
                )

                if not new_docstring:
                    continue

                # Compare with existing (normalize for comparison)
                existing_normalized = getter.docstring.strip()
                new_normalized = new_docstring.strip()

                # Check if everything but type hint is the same
                if existing_normalized == new_normalized:
                    results.append(
                        SyncResult(
                            struct_name=class_name,
                            field_name=field_name,
                            status="unchanged",
                        )
                    )
                    continue

                # Need to update
                results.append(
                    SyncResult(
                        struct_name=class_name,
                        field_name=field_name,
                        status="updated" if getter.has_docstring else "added",
                        old_doc=getter.docstring,
                        new_doc=new_docstring,
                    )
                )

                # Record change for later application
                changes.append(
                    (
                        getter.line_start,
                        getter.line_end,
                        getter.docstring,
                        new_docstring,
                        False,  # is_class_sync
                    )
                )
                modified = True

        # Apply changes to file
        if modified and not dry_run:
            new_content = apply_docstring_changes(content, changes)
            filepath.write_text(new_content)

    return results


def apply_docstring_changes(
    content: str,
    changes: list[tuple[int, int, str, str, bool]],
) -> str:
    """Apply docstring changes to file content."""
    # Sort changes by line number in reverse order to apply from bottom to top
    changes_sorted = sorted(changes, key=lambda x: x[0], reverse=True)

    lines = content.split("\n")

    for start_line, end_line, old_doc, new_doc, is_class_sync in changes_sorted:
        # Find the target line (#[getter] or #[pyclass])
        target_line_idx = None
        target_marker = "#[pyclass" if is_class_sync else "#[getter]"

        for idx in range(end_line, min(end_line + 50, len(lines))):
            if target_marker in lines[idx]:
                target_line_idx = idx
                break

        if target_line_idx is None:
            # Fallback for classes that might have multiple attributes
            if is_class_sync:
                for idx in range(end_line, min(end_line + 15, len(lines))):
                    if "pub struct" in lines[idx]:
                        target_line_idx = idx
                        break
            if target_line_idx is None:
                continue

        # Find where existing docstring starts
        doc_start_idx = target_line_idx
        while doc_start_idx > 0:
            prev_line = lines[doc_start_idx - 1].strip()
            if prev_line.startswith("///") or prev_line == "":
                if prev_line.startswith("///"):
                    doc_start_idx -= 1
                else:
                    # Only stop at empty line if we haven't found any docs yet
                    has_docs_below = any(
                        lines[i].strip().startswith("///")
                        for i in range(doc_start_idx, target_line_idx)
                    )
                    if not has_docs_below:
                        doc_start_idx -= 1
                    else:
                        break
            else:
                break

        # Remove old docstring lines
        if doc_start_idx < target_line_idx:
            has_docs = any(
                lines[i].strip().startswith("///")
                for i in range(doc_start_idx, target_line_idx)
            )
            if has_docs:
                del lines[doc_start_idx:target_line_idx]
                target_line_idx = doc_start_idx

        # Get indentation from target line
        indent_match = re.match(r"^(\s*)", lines[target_line_idx])
        indent = indent_match.group(1) if indent_match else ""

        # Format new docstring
        new_doc_lines = [
            f"{indent}/// {line}" if line else f"{indent}///"
            for line in new_doc.split("\n")
        ]

        # Insert new docstring
        for i, doc_line in enumerate(new_doc_lines):
            lines.insert(target_line_idx + i, doc_line)

    return "\n".join(lines)


# ---------------------------------------------------------------------------
# Reporting
# ---------------------------------------------------------------------------


def print_report(
    results: list[SyncResult],
    rust_structs: dict[str, RustStruct],
    verbose: bool = False,
) -> None:
    """Print a summary report of synchronization results."""
    by_status: dict[str, list[SyncResult]] = {}
    for r in results:
        by_status.setdefault(r.status, []).append(r)

    print("\n=== Docstring Synchronization Report ===\n")

    status_labels = {
        "updated": "✓ Updated",
        "added": "✓ Added",
        "unchanged": "• Unchanged",
        "no_rust_doc": "⚠ No Rust docstring",
        "field_not_found": "⚠ Field not in Rust struct",
        "struct_not_found": "✗ Struct not found in Rust",
    }

    for status, label in status_labels.items():
        items = by_status.get(status, [])
        if items:
            print(f"{label}: {len(items)}")
            if verbose or status in ("field_not_found", "struct_not_found"):
                for r in items:
                    print(f"    - {r.struct_name}.{r.field_name}")

    # Show mismatches details
    struct_not_found = by_status.get("struct_not_found", [])
    if struct_not_found:
        missing_structs = sorted(set(r.struct_name for r in struct_not_found))
        print(f"\n--- Missing Rust structs ({len(missing_structs)}) ---")
        for struct_name in missing_structs:
            print(f"  Python: {struct_name}")
            # Suggest similar Rust structs
            similar = [
                n
                for n in rust_structs.keys()
                if struct_name.lower() in n.lower() or n.lower() in struct_name.lower()
            ]
            if similar:
                print(f"    Similar Rust: {', '.join(similar)}")

    field_not_found = by_status.get("field_not_found", [])
    if field_not_found:
        print("\n--- Missing Rust fields ---")
        by_struct: dict[str, list[str]] = {}
        for r in field_not_found:
            by_struct.setdefault(r.struct_name, []).append(r.field_name)
        for struct_name, fields in sorted(by_struct.items()):
            rust_struct = rust_structs.get(struct_name)
            rust_fields = list(rust_struct.fields.keys()) if rust_struct else []
            print(f"  {struct_name}:")
            print(f"    Python fields not in Rust: {', '.join(fields)}")
            if rust_fields:
                print(f"    Available Rust fields: {', '.join(rust_fields)}")

    total = len(results)
    synced = len(by_status.get("updated", [])) + len(by_status.get("added", []))
    print(f"\nTotal: {total}, Synced: {synced}")
    print(f"\nTotal: {total}, Synced: {synced}")


def print_dry_run(results: list[SyncResult]) -> None:
    """Print detailed changes for dry-run mode."""
    print("\n=== Dry Run - Changes to be made ===\n")

    changes = [r for r in results if r.status in ("updated", "added")]
    if not changes:
        print("No changes needed.")
        return

    for r in changes:
        print(f"{'─' * 60}")
        print(f"{r.struct_name}.{r.field_name} [{r.status}]")
        if r.old_doc:
            print(f"\n  OLD:\n    {r.old_doc.replace(chr(10), chr(10) + '    ')}")
        print(f"\n  NEW:\n    {r.new_doc.replace(chr(10), chr(10) + '    ')}")
    print(f"{'─' * 60}")
    print(f"\nTotal changes: {len(changes)}")


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Synchronize Rust docstrings to Python bindings"
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
        "--check",
        action="store_true",
        help="Check if docstrings are in sync (exit 1 if not)",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Show what would be changed without modifying files",
    )
    parser.add_argument(
        "--report",
        action="store_true",
        help="Generate detailed mismatch report",
    )
    parser.add_argument(
        "--verbose",
        "-v",
        action="store_true",
        help="Verbose output",
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

    print(f"Core directory: {core_dir}")
    print(f"Binding directory: {binding_dir}")

    # Collect Rust structs
    print("\nParsing Rust core library...")
    rust_structs = collect_rust_structs(core_dir)
    print(f"Found {len(rust_structs)} structs")

    # Collect Python classes
    print("Parsing Python bindings...")
    python_classes = collect_python_classes(binding_dir)
    total_classes = sum(len(c) for c in python_classes.values())
    print(f"Found {total_classes} classes in {len(python_classes)} files")

    # Synchronize
    print("\nSynchronizing docstrings...")
    results = sync_docstrings(
        rust_structs,
        python_classes,
        binding_dir,
        dry_run=args.dry_run or args.check,
    )

    # Output
    if args.dry_run:
        print_dry_run(results)
    elif args.report or args.verbose:
        print_report(results, rust_structs, verbose=args.verbose)
    else:
        print_report(results, rust_structs, verbose=False)

    # Check mode exit code
    if args.check:
        changes = [r for r in results if r.status in ("updated", "added")]
        if changes:
            print(f"\n{len(changes)} docstrings are out of sync.")
            return 1
        print("\nAll docstrings are in sync.")

    return 0


if __name__ == "__main__":
    sys.exit(main())
