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

# stdlib
from typing import Mapping, Optional

# 3rd party
from domdf_python_tools.paths import PathPlus
from domdf_python_tools.typing import PathLike
from formate.config import formats_filetypes, wants_filename, wants_global_config

# this package
from ._formate_js import Configuration, ConfigurationBuilder, FormatTextOptions, format_text

__all__ = ["javascript_hook", "Configuration", "ConfigurationBuilder"]


@formats_filetypes(".ts", ".js")
@wants_filename
@wants_global_config
def javascript_hook(
		source: str,
		formate_filename: PathLike,
		formate_global_config: Optional[Mapping] = None,
		**kwargs,
		) -> str:
	r"""
	Reformat JavaScript and TypeScript with dprint.

	:param source: The source to reformat.
	:param formate_filename: The name of the file being formatted.
	:param formate_global_config: The global configuration dictionary. Optional.
	:param \*\*kwargs:

	:returns: The reformatted source.
	"""

	if "line_width" not in kwargs and formate_global_config:
		if "line_length" in (formate_global_config or {}):
			kwargs["line_width"] = formate_global_config["line_length"]

	if "use_tabs" not in kwargs and formate_global_config:
		if "indent" in (formate_global_config or {}):
			indent_setting = formate_global_config["indent"]
			if indent_setting == '\t':
				kwargs["use_tabs"] = True
			else:
				kwargs["use_tabs"] = False
				if set(indent_setting) == {' '}:
					# All spaces
					kwargs.setdefault("indent_width", len(indent_setting))

	config = Configuration(**kwargs)

	filename_p = PathPlus(formate_filename)

	options = FormatTextOptions(filename_p.as_posix(), filename_p.suffix, source, config)

	result = format_text(options)

	if result is None:
		# No change
		return source

	return result
