import pytest
import fasttok


def test_sum_as_string():
    assert fasttok.sum_as_string(1, 1) == "2"
