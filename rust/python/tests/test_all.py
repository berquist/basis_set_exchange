import pytest
import bse


def test_sum_as_string():
    assert bse.sum_as_string(1, 1) == "2"
