"""
Stub generator for ccsds_ndm Python bindings.

This script introspects the compiled PyO3 module and generates:
- __init__.pyi stub file with type hints and docstrings

Usage:
    uv run python stubs.py           # Generate stubs
    uv run python stubs.py --check   # Check stubs are up to date
"""

from __future__ import annotations

import argparse
import inspect
import re
import subprocess
from pathlib import Path
from types import ModuleType
from typing import Any

INDENT = "    "
GENERATED_HEADER = """\
# Generated content DO NOT EDIT
from typing import Optional
import numpy

"""


def _extract_bracketed_type(text: str, start_pos: int) -> str:
    """Extract a type expression with balanced brackets starting from position."""
    result: list[str] = []
    depth = 0
    for char in text[start_pos:]:
        if char == "[":
            depth += 1
        elif char == "]":
            depth -= 1
            if depth < 0:
                break
            result.append(char)
            if depth == 0:
                break
            continue
        elif char in " \n\t" and depth == 0:
            break
        result.append(char)
    return "".join(result)


def _extract_annotation(doc: str | None, tag: str) -> str | None:
    """Extract type from :type: or :rtype: annotation in docstring."""
    if not doc:
        return None
    match = re.search(rf":{tag}:\s*", doc)
    if match:
        result = _extract_bracketed_type(doc, match.end())
        return result or None
    return None


def _clean_docstring(doc: str | None) -> str:
    """Remove :type: and :rtype: lines from docstring for cleaner output."""
    if not doc:
        return ""
    lines = [
        line
        for line in doc.split("\n")
        if not line.strip().startswith((":type:", ":rtype:"))
    ]
    return "\n".join(lines).strip()


def _indent_text(text: str, indent: str) -> str:
    """Indent all lines after the first."""
    return text.replace("\n", f"\n{indent}")


def _format_docstring(doc: str | None, indent: str) -> str:
    """Format a docstring with proper indentation."""
    cleaned = _clean_docstring(doc)
    if not cleaned:
        return f'{indent}"""\n{indent}"""\n'
    return f'{indent}"""\n{indent}{_indent_text(cleaned, indent)}\n{indent}"""\n'


def _generate_function(obj: Any, indent: str) -> str:
    """Generate stub for a function or method."""
    name = obj.__name__
    sig = getattr(obj, "__text_signature__", None)

    if sig is None:
        sig = "()"
    else:
        sig = sig.replace("$self", "self").replace(" /,", "")

    # Special cases for dunder methods
    if name == "__getitem__":
        sig = "(self, key)"
    elif name == "__setitem__":
        sig = "(self, key, value)"

    doc = obj.__doc__ or ""
    return_type = _extract_annotation(doc, "rtype") or _extract_annotation(doc, "type")
    return_annotation = f" -> {return_type}" if return_type else ""

    inner_indent = indent + INDENT
    return (
        f"{indent}def {name}{sig}{return_annotation}:\n"
        f"{_format_docstring(doc, inner_indent)}"
        f"{inner_indent}...\n\n"
    )


def _generate_property(obj: Any, indent: str) -> str:
    """Generate stub for a property (getter and setter)."""
    name = obj.__name__
    doc = obj.__doc__ or ""
    prop_type = _extract_annotation(doc, "type")
    return_annotation = f" -> {prop_type}" if prop_type else ""
    inner_indent = indent + INDENT
    cleaned_doc = _clean_docstring(doc)

    lines = [
        f"{indent}@property",
        f"{indent}def {name}(self){return_annotation}:",
    ]
    if cleaned_doc:
        lines.append(
            f'{inner_indent}"""\n{inner_indent}{_indent_text(cleaned_doc, inner_indent)}\n{inner_indent}"""'
        )
    lines.extend([f"{inner_indent}...", ""])

    # Setter
    value_type = prop_type or "Any"
    lines.extend(
        [
            f"{indent}@{name}.setter",
            f"{indent}def {name}(self, value: {value_type}) -> None:",
            f"{inner_indent}...",
            "",
        ]
    )
    return "\n".join(lines) + "\n"


def _should_include_member(obj: Any) -> bool:
    """Determine if a member should be included in the stub."""
    always_include = {
        "__getitem__",
        "__setitem__",
        "__getstate__",
        "__setstate__",
        "__getnewargs__",
    }

    if inspect.ismethoddescriptor(obj) or inspect.isbuiltin(obj):
        if obj.__name__ in always_include:
            return True
        return bool(obj.__text_signature__) and not obj.__name__.startswith("_")

    if inspect.isgetsetdescriptor(obj):
        return not obj.__name__.startswith("_")

    return False


def _generate_class(cls: type, indent: str) -> str:
    """Generate stub for a class."""
    mro = inspect.getmro(cls)
    inherit = f"({mro[1].__name__})" if len(mro) > 2 else ""
    inner_indent = indent + INDENT

    lines = [f"{indent}class {cls.__name__}{inherit}:"]

    # Class docstring
    if cls.__doc__:
        cleaned = _clean_docstring(cls.__doc__)
        if cleaned:
            lines.append(
                f'{inner_indent}"""\n{inner_indent}{_indent_text(cleaned, inner_indent)}\n{inner_indent}"""'
            )

    # __init__ if it has a signature
    if cls.__text_signature__:
        init_sig = cls.__text_signature__.replace("$self", "self").replace(" /,", "")
        lines.extend(
            [
                f"{inner_indent}def __init__{init_sig} -> None:",
                f"{inner_indent}{INDENT}...",
                "",
            ]
        )

    # Collect member stubs
    member_stubs: list[str] = []
    for _, member in inspect.getmembers(cls, _should_include_member):
        member_stubs.append(_generate_stub(member, inner_indent))

    # Add members or ellipsis if empty
    if member_stubs:
        lines.extend(member_stubs)
    elif not cls.__doc__ and not cls.__text_signature__:
        lines.append(f"{inner_indent}...")

    lines.append("\n")
    return "\n".join(lines)


def _generate_stub(obj: Any, indent: str = "") -> str:
    """Generate stub for any supported object type."""
    if inspect.ismodule(obj):
        members = [
            m
            for name, m in inspect.getmembers(obj)
            if not name.startswith("_") and not inspect.ismodule(m)
        ]
        # Sort: non-classes first, then classes by inheritance depth
        members.sort(
            key=lambda m: (10 + len(inspect.getmro(m))) if inspect.isclass(m) else 1
        )
        return GENERATED_HEADER + "".join(_generate_stub(m, indent) for m in members)

    if inspect.isclass(obj):
        return _generate_class(obj, indent)

    if inspect.isbuiltin(obj):
        return f"{indent}@staticmethod\n{_generate_function(obj, indent)}"

    if inspect.ismethoddescriptor(obj):
        return _generate_function(obj, indent)

    if inspect.isgetsetdescriptor(obj):
        return _generate_property(obj, indent)

    raise ValueError(f"Unsupported object type: {type(obj).__name__}")


def _format_with_ruff(code: str) -> str:
    """Format code using ruff."""
    result = subprocess.run(
        [
            "ruff",
            "format",
            "--config",
            "pyproject.toml",
            "--stdin-filename",
            "stub.pyi",
            "-",
        ],
        input=code,
        capture_output=True,
        text=True,
    )
    if result.stderr:
        print(f"Ruff warning: {result.stderr}")
    return result.stdout or code


def write_stubs(module: ModuleType, directory: Path, *, check: bool = False) -> None:
    """Write stub file for a module and its submodules."""
    filename = directory / "__init__.pyi"
    content = _generate_stub(module)

    try:
        content = _format_with_ruff(content)
    except Exception as e:
        print(f"Ruff error: {e}")

    directory.mkdir(parents=True, exist_ok=True)

    if check:
        existing = filename.read_text()
        assert existing == content, (
            f"The content of {filename} seems outdated, please run `python stubs.py`"
        )
    else:
        filename.write_text(content)
        print(f"Generated {filename}")

    # Process submodules recursively
    for name, submodule in inspect.getmembers(module):
        if inspect.ismodule(submodule):
            write_stubs(submodule, directory / name, check=check)


def main() -> None:
    parser = argparse.ArgumentParser(description="Generate Python stubs for ccsds_ndm")
    parser.add_argument(
        "--check", action="store_true", help="Check stubs are up to date"
    )
    args = parser.parse_args()

    import ccsds_ndm

    write_stubs(ccsds_ndm.ccsds_ndm, Path("ccsds_ndm"), check=args.check)  # type: ignore[attr-defined]


if __name__ == "__main__":
    main()
