# stdlib
from typing import TypedDict

# this package
from ._formate_js import *

__doc__ = _formate_js.__doc__
if hasattr(_formate_js, "__all__"):
	__all__ = _formate_js.__all__
