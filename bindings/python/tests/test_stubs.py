from stubs import _generate_class


def test_generator_omits_inherited_object_methods():
    class Example:
        pass

    assert "__getstate__" not in _generate_class(Example, "")
