# SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
#
# SPDX-License-Identifier: MPL-2.0

"""
Unit tests for UserDefined type in CCSDS NDM Python bindings.
"""

from ccsds_ndm import UserDefined


class TestUserDefined:
    def test_construction_empty(self):
        ud = UserDefined()
        assert ud.user_defined == {}
        assert ud.comment == []

    def test_construction_full(self):
        params = {"PARAM1": "VALUE1", "PARAM2": "VALUE2"}
        comment = ["Comment 1", "Comment 2"]
        ud = UserDefined(parameters=params, comment=comment)

        assert ud.user_defined == params
        assert ud.comment == comment

    def test_setters(self):
        ud = UserDefined()
        ud.user_defined = {"NEW": "VAL"}
        assert ud.user_defined["NEW"] == "VAL"

        ud.comment = ["New Comment"]
        assert ud.comment[0] == "New Comment"
