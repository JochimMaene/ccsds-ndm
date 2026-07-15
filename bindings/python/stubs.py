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
import ast
import functools
import inspect
import re
import subprocess
from collections import defaultdict
from pathlib import Path
from types import ModuleType
from typing import Any

INDENT = "    "
GENERATED_HEADER = """\
# Generated content DO NOT EDIT
from typing import Optional, Union
import numpy

"""

PUBLIC_API_PARAMETER_TYPES = {
    "from_file": {
        "format": "str",
        "path": "str",
        "strict": "bool",
    },
    "from_str": {
        "data": "str",
        "format": "str",
        "strict": "bool",
    },
    "to_file": {
        "format": "str",
        "path": "str",
        "validate": "bool",
        "version": "Optional[str]",
    },
    "to_kvn": {"version": "Optional[str]"},
    "to_str": {
        "format": "str",
        "validate": "bool",
        "version": "Optional[str]",
    },
    "to_xml": {"version": "Optional[str]"},
    "validate": {"strict": "bool"},
}


def _extract_pymethod_bodies(content: str) -> list[tuple[str, str]]:
    """Extract (class_name, impl_body) from #[pymethods] impl blocks."""
    impls: list[tuple[str, str]] = []
    impl_pattern = re.compile(r"#\[pymethods\]\s*impl\s+(\w+)\s*\{", re.MULTILINE)

    for match in impl_pattern.finditer(content):
        class_name = match.group(1)
        start = match.end()
        pos = start
        depth = 1
        while pos < len(content) and depth > 0:
            if content[pos] == "{":
                depth += 1
            elif content[pos] == "}":
                depth -= 1
            pos += 1
        impls.append((class_name, content[start : pos - 1]))
    return impls


def _collect_property_setters(binding_src_dir: Path) -> dict[str, set[str]]:
    """Collect writable properties from Rust #[setter] definitions."""
    setters_by_class: dict[str, set[str]] = defaultdict(set)
    setter_pattern = re.compile(
        r"#\[setter\]\s*\n(?:\s*#\[[^\]]*\]\s*\n)*\s*fn\s+(\w+)(?:<[^>]*>)?\s*\(",
        re.MULTILINE,
    )

    for file in binding_src_dir.glob("*.rs"):
        content = file.read_text()
        for class_name, body in _extract_pymethod_bodies(content):
            for setter in setter_pattern.finditer(body):
                func_name = setter.group(1)
                prop_name = func_name[4:] if func_name.startswith("set_") else func_name
                setters_by_class[class_name].add(prop_name)

    return dict(setters_by_class)


@functools.cache
def _property_setters() -> dict[str, set[str]]:
    """Cached writable property mapping derived from Rust binding sources."""
    root = Path(__file__).resolve().parent
    return _collect_property_setters(root / "src")


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
    match = re.search(rf":{tag}:\s*([^\n\r]+)", doc)
    if match:
        result = match.group(1).strip()
        return result or None
    return None


def _extract_numpy_returns_type(doc: str | None) -> str | None:
    """Extract return type from a NumPy-style ``Returns`` section."""
    if not doc:
        return None
    lines = doc.splitlines()
    for i, line in enumerate(lines):
        if line.strip() != "Returns":
            continue

        j = i + 1
        while j < len(lines) and not lines[j].strip():
            j += 1

        if j < len(lines):
            sep = lines[j].strip()
            if sep and set(sep) == {"-"}:
                j += 1

        while j < len(lines) and not lines[j].strip():
            j += 1

        if j < len(lines):
            type_line = lines[j].strip()
            if type_line:
                return type_line

    return None


def _normalize_type_hint(raw: str) -> str:
    """Normalize docstring type text into a Python type hint."""
    hint = raw.strip().strip(".").replace("`", "")

    optional = False
    parts = _split_signature_items(hint)
    if len(parts) > 1 and any(p.strip().lower() == "optional" for p in parts[1:]):
        optional = True
        hint = parts[0].strip()

    lower = hint.lower()
    if lower in {"string"}:
        hint = "str"
    elif lower in {"integer"}:
        hint = "int"
    elif lower in {"boolean"}:
        hint = "bool"
    elif lower in {"array", "array-like", "array_like", "ndarray"}:
        hint = "numpy.ndarray"
    elif lower in {"any"}:
        hint = "object"

    list_match = re.match(r"^list of (.+)$", hint, flags=re.IGNORECASE)
    if list_match:
        inner = _normalize_type_hint(list_match.group(1).strip())
        hint = f"list[{inner}]"

    tuple_match = re.match(r"^tuple of (.+)$", hint, flags=re.IGNORECASE)
    if tuple_match:
        inner = _normalize_type_hint(tuple_match.group(1).strip())
        hint = f"tuple[{inner}]"

    dict_match = re.match(
        r"^dict(?:ionary)? of (.+) to (.+)$", hint, flags=re.IGNORECASE
    )
    if dict_match:
        key = _normalize_type_hint(dict_match.group(1).strip())
        value = _normalize_type_hint(dict_match.group(2).strip())
        hint = f"dict[{key}, {value}]"

    hint = hint.replace("array-like", "numpy.ndarray").replace(
        "array_like", "numpy.ndarray"
    )

    if (
        optional
        and "None" not in hint
        and not hint.startswith("Optional[")
        and "|" not in hint
    ):
        hint = f"Optional[{hint}]"

    try:
        ast.parse(f"def _f(x: {hint}):\n    pass\n")
    except SyntaxError:
        hint = "object"

    return hint


def _extract_numpy_parameter_types(doc: str | None) -> dict[str, str]:
    """Extract parameter type hints from a NumPy-style ``Parameters`` section."""
    if not doc:
        return {}

    lines = doc.splitlines()
    param_types: dict[str, str] = {}

    i = 0
    while i < len(lines):
        if lines[i].strip() != "Parameters":
            i += 1
            continue

        j = i + 1
        while j < len(lines) and not lines[j].strip():
            j += 1
        if j < len(lines):
            sep = lines[j].strip()
            if sep and set(sep) == {"-"}:
                j += 1

        while j < len(lines):
            line = lines[j]
            stripped = line.strip()
            if not stripped:
                j += 1
                continue

            # Next section header in NumPy docs, e.g. "Returns" + dashed underline.
            if ":" not in stripped and j + 1 < len(lines):
                next_line = lines[j + 1].strip()
                if next_line and set(next_line) == {"-"}:
                    break

            match = re.match(r"^([*A-Za-z_][A-Za-z0-9_,\s*]*)\s*:\s*(.+)$", stripped)
            if match:
                names_text, type_text = match.group(1), match.group(2)
                hint = _normalize_type_hint(type_text)
                for name in [n.strip() for n in names_text.split(",")]:
                    if name:
                        param_types[name.lstrip("*")] = hint

            j += 1

        i = j

    return param_types


def _split_signature_items(signature_inner: str) -> list[str]:
    """Split a function signature parameter list by commas with bracket awareness."""
    items: list[str] = []
    current: list[str] = []
    paren_depth = 0
    bracket_depth = 0
    brace_depth = 0
    quote: str | None = None
    escaped = False

    for char in signature_inner:
        if quote is not None:
            current.append(char)
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == quote:
                quote = None
            continue

        if char in {"'", '"'}:
            quote = char
            current.append(char)
            continue

        if char == "(":
            paren_depth += 1
        elif char == ")":
            paren_depth -= 1
        elif char == "[":
            bracket_depth += 1
        elif char == "]":
            bracket_depth -= 1
        elif char == "{":
            brace_depth += 1
        elif char == "}":
            brace_depth -= 1

        if char == "," and paren_depth == 0 and bracket_depth == 0 and brace_depth == 0:
            items.append("".join(current).strip())
            current = []
        else:
            current.append(char)

    tail = "".join(current).strip()
    if tail:
        items.append(tail)
    return items


def _annotate_signature(sig: str, param_types: dict[str, str]) -> str:
    """Apply parameter annotations to a ``(param, ...)`` signature string."""
    if not sig.startswith("(") or not sig.endswith(")") or not param_types:
        return sig

    inner = sig[1:-1]
    if not inner.strip():
        return sig

    items = _split_signature_items(inner)
    out: list[str] = []

    for item in items:
        token = item.strip()
        if token in {"", "/", "*"}:
            out.append(token)
            continue

        prefix = ""
        rest = token
        if rest.startswith("**"):
            prefix = "**"
            rest = rest[2:]
        elif rest.startswith("*"):
            prefix = "*"
            rest = rest[1:]

        if "=" in rest:
            left, default = rest.split("=", 1)
            default = "=" + default
        else:
            left, default = rest, ""
        left = left.strip()

        if ":" in left:
            name, annotation = left.split(":", 1)
            name = name.strip()
            annotation = annotation.strip()
        else:
            name = left
            annotation = ""

        if not annotation:
            inferred = param_types.get(name)
            if inferred:
                if (
                    default == "=None"
                    and not inferred.startswith("Optional[")
                    and "None" not in inferred
                ):
                    inferred = f"Optional[{inferred}]"
                left = f"{name}: {inferred}"
            else:
                left = name
        else:
            left = f"{name}: {annotation}"

        out.append(f"{prefix}{left}{default}")

    return f"({', '.join(out)})"


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


def _generate_function(obj: Any, indent: str, owner_class: str | None = None) -> str:
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
    elif name == "__setstate__":
        sig = "(self, state)"

    doc = obj.__doc__ or ""
    param_types = _extract_numpy_parameter_types(doc)
    param_types.update(PUBLIC_API_PARAMETER_TYPES.get(name, {}))
    sig = _annotate_signature(sig, param_types)

    return_type = _extract_annotation(doc, "rtype") or _extract_annotation(doc, "type")
    if not return_type:
        return_type = _extract_numpy_returns_type(doc)
    if not return_type:
        if name in {"to_file", "__setstate__"}:
            return_type = "None"
        elif name in {"to_kvn", "to_str", "to_xml"}:
            return_type = "str"
        elif name == "to_numpy":
            return_type = "numpy.ndarray"
        elif name == "__getstate__":
            return_type = "object"
        elif name == "validate":
            return_type = "Optional[list[str]]" if "strict" in sig else "None"
    if not return_type and owner_class and name.startswith("from_"):
        # PyO3 static factory methods usually expose no runtime return annotation.
        # Use class context so type checkers infer `Cdm.from_str(...) -> Cdm`, etc.
        return_type = owner_class
    return_annotation = f" -> {return_type}" if return_type else ""

    inner_indent = indent + INDENT
    return (
        f"{indent}def {name}{sig}{return_annotation}:\n"
        f"{_format_docstring(doc, inner_indent)}"
        f"{inner_indent}...\n\n"
    )


def _generate_property(obj: Any, indent: str, *, has_setter: bool) -> str:
    """Generate stub for a property."""
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

    if has_setter:
        value_type = prop_type or "object"
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
        if not init_sig.startswith("(self"):
            if init_sig == "()":
                init_sig = "(self)"
            else:
                init_sig = f"(self, {init_sig[1:]}"
        init_sig = _annotate_signature(
            init_sig, _extract_numpy_parameter_types(cls.__doc__)
        )
        lines.extend(
            [
                f"{inner_indent}def __init__{init_sig} -> None:",
                f"{inner_indent}{INDENT}...",
                "",
            ]
        )

    class_setters = _property_setters().get(cls.__name__, set())

    # Collect member stubs
    member_stubs: list[str] = []
    for member_name, member in inspect.getmembers(cls, _should_include_member):
        if inspect.isgetsetdescriptor(member):
            member_stubs.append(
                _generate_property(
                    member,
                    inner_indent,
                    has_setter=member_name in class_setters,
                )
            )
            continue
        member_stubs.append(
            _generate_stub(member, inner_indent, owner_class=cls.__name__)
        )

    # Add members or ellipsis if empty
    if member_stubs:
        lines.extend(member_stubs)
    elif not cls.__doc__ and not cls.__text_signature__:
        lines.append(f"{inner_indent}...")

    lines.append("\n")
    return "\n".join(lines)


def _generate_stub(obj: Any, indent: str = "", owner_class: str | None = None) -> str:
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
        fn = _generate_function(obj, indent, owner_class=owner_class)
        if owner_class:
            return f"{indent}@staticmethod\n{fn}"
        return fn

    if inspect.ismethoddescriptor(obj):
        return _generate_function(obj, indent, owner_class=owner_class)

    if inspect.isgetsetdescriptor(obj):
        # Property setter information is only available in class context.
        return _generate_property(obj, indent, has_setter=False)

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
