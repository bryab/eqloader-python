import pytest
import blender_eqloader


def test_sum_as_string():
    assert blender_eqloader.sum_as_string(1, 1) == "2"
