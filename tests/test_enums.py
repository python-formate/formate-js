# 3rd party
import pytest

# this package
from formate_js import enums


@pytest.mark.parametrize("enum_name", enums.__all__)
def test_enums(enum_name: str):
	enum_obj = getattr(enums, enum_name)

	for value in enum_obj:
		# Check we can make it into a string
		assert str(value)
