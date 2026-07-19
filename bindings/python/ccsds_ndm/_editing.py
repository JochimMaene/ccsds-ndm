# SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""Small copy-on-write editor for the owned PyO3 model wrappers."""

from collections.abc import MutableSequence
from importlib import import_module
from typing import Any, Callable, Generic, TypeVar


T = TypeVar("T")
_native = import_module(f"{__package__}.ccsds_ndm")
_NATIVE_TYPES = frozenset(
    value for value in vars(_native).values() if isinstance(value, type)
)


def _unwrap(value: Any) -> Any:
    if isinstance(value, _EditProxy):
        return value._value
    if isinstance(value, _ListEditProxy):
        return value._values
    return value


def _is_model(value: Any) -> bool:
    return type(value) in _NATIVE_TYPES and not isinstance(value, BaseException)


class _EditProxy(Generic[T]):
    __slots__ = ("_value", "_commit")

    def __init__(self, value: T, commit: Callable[[T], None]) -> None:
        object.__setattr__(self, "_value", value)
        object.__setattr__(self, "_commit", commit)

    def __getattr__(self, name: str) -> Any:
        value = getattr(self._value, name)
        if isinstance(value, list):
            return _ListEditProxy(
                value,
                lambda updated: self._commit_attribute(name, updated),
            )
        if _is_model(value):
            return _EditProxy(
                value,
                lambda updated: self._commit_attribute(name, updated),
            )
        return value

    def __setattr__(self, name: str, value: Any) -> None:
        setattr(self._value, name, _unwrap(value))
        self._commit(self._value)

    def _commit_attribute(self, name: str, value: Any) -> None:
        setattr(self._value, name, _unwrap(value))
        self._commit(self._value)

    def __repr__(self) -> str:
        return f"edit({self._value!r})"


class _ListEditProxy(MutableSequence[Any]):
    __slots__ = ("_values", "_commit")

    def __init__(self, values: list[Any], commit: Callable[[list[Any]], None]) -> None:
        self._values = values
        self._commit = commit

    def __len__(self) -> int:
        return len(self._values)

    def __getitem__(self, index: Any) -> Any:
        value = self._values[index]
        if isinstance(index, slice):
            return value
        if _is_model(value):
            return _EditProxy(value, lambda updated: self._commit_item(index, updated))
        return value

    def __setitem__(self, index: Any, value: Any) -> None:
        if isinstance(index, slice):
            self._values[index] = [_unwrap(item) for item in value]
        else:
            self._values[index] = _unwrap(value)
        self._commit(self._values)

    def __delitem__(self, index: Any) -> None:
        del self._values[index]
        self._commit(self._values)

    def insert(self, index: int, value: Any) -> None:
        self._values.insert(index, _unwrap(value))
        self._commit(self._values)

    def _commit_item(self, index: int, value: Any) -> None:
        self._values[index] = _unwrap(value)
        self._commit(self._values)

    def __repr__(self) -> str:
        return repr(self._values)


def edit(message: T) -> _EditProxy[T]:
    """Return a live copy-on-write view of an NDM message.

    PyO3 model properties use owned values. The editor automatically writes every changed
    descendant back through its parents, including descendants held in lists.
    """

    if not _is_model(message):
        raise TypeError("edit() expects a ccsds_ndm model object")
    return _EditProxy(message, lambda _updated: None)
