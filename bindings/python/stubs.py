"""
Stub generator for ccsds_ndm Python bindings.

This script introspects the compiled PyO3 module and generates:
- __init__.pyi stub file with type hints and docstrings

The Rust docstrings use Python typing syntax (:type: Optional[str], List[X], etc.)
which is compatible with Python 3.9+ and used directly without transformation.

Usage:
    uv run python stubs.py           # Generate stubs
    uv run python stubs.py --check   # Check stubs are up to date
"""

import argparse
import inspect
import os
import re
import subprocess

INDENT = " " * 4
GENERATED_COMMENT = "# Generated content DO NOT EDIT\n"


def do_indent(text: str, indent: str):
    return text.replace("\n", f"\n{indent}")


def extract_bracketed_type(text: str, start_pos: int) -> str:
    """Extract a type expression with balanced brackets starting from position."""
    result = []
    depth = 0
    i = start_pos
    while i < len(text):
        char = text[i]
        if char == "[":
            depth += 1
            result.append(char)
        elif char == "]":
            result.append(char)
            depth -= 1
            if depth == 0:
                i += 1
                break
        elif char in " \n\t" and depth == 0:
            break
        else:
            result.append(char)
        i += 1
    return "".join(result)


def extract_type_from_doc(doc: str) -> str | None:
    """Extract type from :type: annotation in docstring.

    Returns the type string directly from Rust docs - they already use
    Python typing syntax (Optional[X], List[X], etc.) which is 3.9+ compatible.
    """
    if not doc:
        return None
    match = re.search(r":type:\s*", doc)
    if match:
        start = match.end()
        return extract_bracketed_type(doc, start) or None
    return None


def extract_rtype_from_doc(doc: str) -> str | None:
    """Extract return type from :rtype: annotation in docstring."""
    if not doc:
        return None
    match = re.search(r":rtype:\s*", doc)
    if match:
        start = match.end()
        return extract_bracketed_type(doc, start) or None
    return None


def clean_docstring(doc: str) -> str:
    """Remove :type: and :rtype: lines from docstring for cleaner output."""
    if not doc:
        return ""
    lines = doc.split("\n")
    cleaned = []
    for line in lines:
        stripped = line.strip()
        if stripped.startswith(":type:") or stripped.startswith(":rtype:"):
            continue
        cleaned.append(line)
    return "\n".join(cleaned).strip()


def function(obj, indent, text_signature=None, owner=None, return_type=None):
    name = obj.__name__

    if text_signature is None:
        text_signature = getattr(obj, "__text_signature__", None)
    if text_signature is None:
        text_signature = "()"
    else:
        text_signature = text_signature.replace("$self", "self").replace(" /,", "")

    if name in ("__getitem__", "__setitem__"):
        if name == "__getitem__":
            text_signature = "(self, key)"
        else:
            text_signature = "(self, key, value)"

    doc = obj.__doc__ or ""

    if return_type is None:
        return_type = extract_rtype_from_doc(doc)

    if return_type is None:
        return_type = extract_type_from_doc(doc)

    return_annotation = f" -> {return_type}" if return_type else ""

    string = ""
    string += f"{indent}def {name}{text_signature}{return_annotation}:\n"
    indent += INDENT

    cleaned_doc = clean_docstring(doc) if doc else ""

    string += f'{indent}"""\n'
    if cleaned_doc:
        string += f"{indent}{do_indent(cleaned_doc, indent)}\n"
    string += f'{indent}"""\n'
    string += f"{indent}...\n"
    string += "\n"
    return string


def member_sort(member):
    if inspect.isclass(member):
        value = 10 + len(inspect.getmro(member))
    else:
        value = 1
    return value


def fn_predicate(obj):
    always = {
        "__getitem__",
        "__setitem__",
        "__getstate__",
        "__setstate__",
        "__getnewargs__",
    }
    if inspect.ismethoddescriptor(obj) or inspect.isbuiltin(obj):
        name = obj.__name__
        if name in always:
            return True
        return obj.__text_signature__ and not obj.__name__.startswith("_")

    if inspect.isgetsetdescriptor(obj):
        return not obj.__name__.startswith("_")
    return False


def get_module_members(module):
    members = [
        member
        for name, member in inspect.getmembers(module)
        if not name.startswith("_") and not inspect.ismodule(member)
    ]
    members.sort(key=member_sort)
    return members


def pyi_file(obj, indent="", owner=None):
    string = ""
    if inspect.ismodule(obj):
        string += GENERATED_COMMENT
        string += "from typing import Optional\n"
        string += "import numpy\n"
        string += "\n"
        members = get_module_members(obj)
        for member in members:
            string += pyi_file(member, indent)

    elif inspect.isclass(obj):
        indent += INDENT
        mro = inspect.getmro(obj)
        if len(mro) > 2:
            inherit = f"({mro[1].__name__})"
        else:
            inherit = ""
        string += f"class {obj.__name__}{inherit}:\n"

        body = ""
        if obj.__doc__:
            cleaned = clean_docstring(obj.__doc__)
            if cleaned:
                body += (
                    f'{indent}"""\n{indent}{do_indent(cleaned, indent)}\n{indent}"""\n'
                )

        fns = inspect.getmembers(obj, fn_predicate)

        # Init
        if obj.__text_signature__:
            init_sig = obj.__text_signature__
            init_sig = init_sig.replace("$self", "self").replace(" /,", "")
            body += f"{indent}def __init__{init_sig} -> None:\n"
            body += f"{indent + INDENT}...\n"
            body += "\n"

        for name, fn in fns:
            body += pyi_file(fn, indent=indent, owner=obj)

        if not body:
            body += f"{indent}...\n"

        string += body
        string += "\n\n"

    elif inspect.isbuiltin(obj):
        string += f"{indent}@staticmethod\n"
        string += function(obj, indent, owner=owner)

    elif inspect.ismethoddescriptor(obj):
        string += function(obj, indent, owner=owner)

    elif inspect.isgetsetdescriptor(obj):
        doc = obj.__doc__ or ""
        prop_type = extract_type_from_doc(doc)
        return_annotation = f" -> {prop_type}" if prop_type else ""

        cleaned_doc = clean_docstring(doc) if doc else ""

        # Property getter
        string += f"{indent}@property\n"
        string += f"{indent}def {obj.__name__}(self){return_annotation}:\n"
        inner_indent = indent + INDENT
        if cleaned_doc:
            string += f'{inner_indent}"""\n{inner_indent}{do_indent(cleaned_doc, inner_indent)}\n{inner_indent}"""\n'
        string += f"{inner_indent}...\n"
        string += "\n"

        # Property setter
        value_type = prop_type if prop_type else "Any"
        string += f"{indent}@{obj.__name__}.setter\n"
        string += f"{indent}def {obj.__name__}(self, value: {value_type}) -> None:\n"
        string += f"{inner_indent}...\n"
        string += "\n"
    else:
        raise Exception(f"Object {obj} is not supported")
    return string


def do_ruff(code, is_pyi: bool):
    command = ["ruff", "format", "--config", "pyproject.toml"]
    command.extend(["--stdin-filename", "test.pyi" if is_pyi else "test.py", "-"])
    process = subprocess.Popen(
        command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, stdin=subprocess.PIPE
    )
    stdout, stderr = process.communicate(input=code.encode("utf-8"))
    if stderr:
        print(f"Ruff warning: {stderr.decode('utf-8')}")
    return stdout.decode("utf-8") if stdout else code


def write(module, directory, origin, check=False):
    submodules = [
        (name, member)
        for name, member in inspect.getmembers(module)
        if inspect.ismodule(member)
    ]

    filename = os.path.join(directory, "__init__.pyi")
    pyi_content = pyi_file(module)

    try:
        pyi_content = do_ruff(pyi_content, is_pyi=True)
    except Exception as e:
        print(f"Ruff error: {e}")

    os.makedirs(directory, exist_ok=True)
    if check:
        with open(filename, "r") as f:
            data = f.read()
            assert data == pyi_content, (
                f"The content of {filename} seems outdated, please run `python stubs.py`"
            )
    else:
        with open(filename, "w") as f:
            f.write(pyi_content)
        print(f"Generated {filename}")

    for name, submodule in submodules:
        write(submodule, os.path.join(directory, name), f"{name}", check=check)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate Python stubs for ccsds_ndm")
    parser.add_argument(
        "--check", action="store_true", help="Check stubs are up to date"
    )

    args = parser.parse_args()
    import ccsds_ndm

    write(ccsds_ndm.ccsds_ndm, "ccsds_ndm/", "ccsds_ndm", check=args.check)  # type: ignore[attr-defined]
