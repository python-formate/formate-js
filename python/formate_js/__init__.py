#!/usr/bin/env python3
#
#  __init__.py
"""
Formate plugin for reformatting JavaScript and TypeScript files with dprint.
"""
#
#  Copyright © 2026 Dominic Davis-Foster <dominic@davis-foster.co.uk>
#
#  Permission is hereby granted, free of charge, to any person obtaining a copy
#  of this software and associated documentation files (the "Software"), to deal
#  in the Software without restriction, including without limitation the rights
#  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
#  copies of the Software, and to permit persons to whom the Software is
#  furnished to do so, subject to the following conditions:
#
#  The above copyright notice and this permission notice shall be included in all
#  copies or substantial portions of the Software.
#
#  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
#  EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
#  MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
#  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
#  DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
#  OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
#  OR OTHER DEALINGS IN THE SOFTWARE.
#

# 3rd party
from domdf_python_tools.paths import PathPlus
from domdf_python_tools.typing import PathLike
from formate.config import wants_filename

# this package
from ._formate_js import Configuration, ConfigurationBuilder, FormatTextOptions, format_text

__all__ = ["javascript_hook", "Configuration", "ConfigurationBuilder"]


@wants_filename
def javascript_hook(
		source: str,
		formate_filename: PathLike,
		**kwargs,
		) -> str:
	r"""
	Reformat JavaScript and TypeScript with dprint.

	:param source: The source to reformat.
	:param \*\*kwargs:

	:returns: The reformatted source.
	"""

	config = Configuration(**kwargs)

	filename_p = PathPlus(formate_filename)

	options = FormatTextOptions(filename_p.as_posix(), filename_p.suffix, source, config)

	result = format_text(options)

	if result is None:
		# No change
		return source

	return result
