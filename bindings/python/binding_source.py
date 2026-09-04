"""Minimal source inspection shared by the Python binding maintenance scripts."""

from __future__ import annotations

import re
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterator


@dataclass
class RustField:
    name: str
    rust_type: str
    docstring: str

    @property
    def has_docstring(self) -> bool:
        return bool(self.docstring)


@dataclass
class RustStruct:
    name: str
    fields: dict[str, RustField] = field(default_factory=dict)
    docstring: str = ""

    @property
    def has_docstring(self) -> bool:
        return bool(self.docstring)


@dataclass
class PythonGetter:
    name: str
    field_name: str
    docstring: str
    line_start: int
    line_end: int

    @property
    def has_docstring(self) -> bool:
        return bool(self.docstring)

    @property
    def has_type_annotation(self) -> bool:
        return ":type:" in self.docstring


@dataclass
class PythonClass:
    name: str
    getters: dict[str, PythonGetter] = field(default_factory=dict)
    setters: set[str] = field(default_factory=set)
    docstring: str = ""
    line_start: int = 0
    line_end: int = 0

    @property
    def has_docstring(self) -> bool:
        return bool(self.docstring)


def _docstring(block: str) -> str:
    return "\n".join(
        line.split("///", 1)[1].removeprefix(" ").rstrip()
        for line in block.splitlines()
        if "///" in line
    )


def _preceding_docstring(lines: list[str], end: int) -> str:
    doc = []
    for line in reversed(lines[:end]):
        stripped = line.strip()
        if stripped.startswith("///"):
            doc.append(stripped[3:].strip())
        elif stripped.startswith("#[") or not stripped:
            continue
        else:
            break
    return "\n".join(reversed(doc))


def braced_blocks(
    content: str, pattern: re.Pattern[str]
) -> Iterator[tuple[re.Match[str], str, int]]:
    """Yield a regex match, its brace-delimited body, and the body's first line."""
    for match in pattern.finditer(content):
        start = match.end()
        depth = 1
        pos = start
        while pos < len(content) and depth:
            depth += (content[pos] == "{") - (content[pos] == "}")
            pos += 1
        yield match, content[start : pos - 1], content[:start].count("\n")


def parse_rust_file(path: Path) -> dict[str, RustStruct]:
    content = path.read_text()
    lines = content.splitlines()
    structs = {}
    pattern = re.compile(r"pub\s+struct\s+(\w+)\s*(?:<[^>]*>)?\s*\{", re.MULTILINE)
    fields = re.compile(r"^\s*pub\s+(\w+)\s*:\s*([^,\n]+)", re.MULTILINE)

    for match, body, _ in braced_blocks(content, pattern):
        item = RustStruct(
            match.group(1),
            docstring=_preceding_docstring(lines, content[: match.start()].count("\n")),
        )
        body_start = match.end()
        for field_match in fields.finditer(body):
            line = content[: body_start + field_match.start()].count("\n")
            name = field_match.group(1)
            item.fields[name] = RustField(
                name,
                field_match.group(2).strip(),
                _preceding_docstring(lines, line),
            )
        structs[item.name] = item
    return structs


def collect_rust_structs(core_dir: Path) -> dict[str, RustStruct]:
    paths = [core_dir / "common.rs", core_dir / "types.rs"]
    paths.extend(sorted((core_dir / "messages").glob("*.rs")))
    structs = {}
    for path in paths:
        if path.exists() and path.name != "mod.rs":
            structs.update(parse_rust_file(path))
    return structs


def parse_python_binding_file(path: Path) -> dict[str, PythonClass]:
    content = path.read_text()
    classes = {}
    class_pattern = re.compile(
        r"((?:\s*///[^\n]*\n)*)\s*(#\[pyclass[^\]]*\]\s*(?:#\[[^\]]*\]\s*)*)pub\s+struct\s+(\w+)",
        re.MULTILINE,
    )
    for match in class_pattern.finditer(content):
        doc = _docstring(match.group(1))
        line_end = content[: match.start(2)].count("\n")
        classes[match.group(3)] = PythonClass(
            match.group(3),
            docstring=doc,
            line_start=line_end - match.group(1).count("\n") if doc else line_end,
            line_end=line_end,
        )

    impl_pattern = re.compile(r"#\[pymethods\]\s*impl\s+(\w+)\s*\{", re.MULTILINE)
    getter_pattern = re.compile(
        r"((?:\s*///[^\n]*\n)*)\s*#\[getter\]\s*\n(?:\s*#\[[^\]]*\]\s*\n)*\s*fn\s+(get_)?(\w+)\s*\(",
        re.MULTILINE,
    )
    setter_pattern = re.compile(
        r"#\[setter\]\s*\n(?:\s*#\[[^\]]*\]\s*\n)*\s*fn\s+(set_)?(\w+)\s*\(",
        re.MULTILINE,
    )
    for match, body, body_line in braced_blocks(content, impl_pattern):
        item = classes.get(match.group(1))
        if item is None:
            continue
        for getter in getter_pattern.finditer(body):
            doc = _docstring(getter.group(1))
            line_end = body_line + body[: getter.start()].count("\n")
            name = getter.group(3)
            item.getters[name] = PythonGetter(
                f"{getter.group(2) or ''}{name}",
                name,
                doc,
                line_end - getter.group(1).count("\n") if doc else line_end,
                line_end,
            )
        for setter in setter_pattern.finditer(body):
            item.setters.add(setter.group(2))
    return classes
