# this package
from formate_js import javascript_hook


def test_line_length():
	example_code = "class Test {/* testing this out to go above 40 */}"
	formatted_40_output = """\
class Test {
  /* testing this out to go above 40 */
}
"""

	output = javascript_hook(example_code, "code.js", formate_global_config={"line_length": 40})
	assert output == formatted_40_output

	output = javascript_hook(example_code, "code.js", line_width=40)
	assert output == formatted_40_output

	output = javascript_hook(example_code, "code.js", line_width=40, formate_global_config={})
	assert output == formatted_40_output

	output = javascript_hook(example_code, "code.js", line_width=40, formate_global_config={"line_length": 80})
	assert output == formatted_40_output


def test_indent():
	example_code = "const test = testingThisOutt + testtt + testingThisOut + testingthi + outt;"
	formatted_tabs_output = """\
const test = testingThisOutt + testtt
	+ testingThisOut + testingthi
	+ outt;
"""

	output = javascript_hook(example_code, "code.js", line_width=40, formate_global_config={"indent": '\t'})
	assert output == formatted_tabs_output

	output = javascript_hook(example_code, "code.js", line_width=40, indent_width=4, use_tabs=True)
	assert output == formatted_tabs_output

	output = javascript_hook(
			example_code,
			"code.js",
			line_width=40,
			indent_width=4,
			use_tabs=True,
			formate_global_config={},
			)
	assert output == formatted_tabs_output

	output = javascript_hook(
			example_code,
			"code.js",
			line_width=40,
			indent_width=4,
			use_tabs=True,
			formate_global_config={"indent": "    "},
			)
	assert output == formatted_tabs_output

	formatted_spaces_output = """\
const test = testingThisOutt + testtt
      + testingThisOut + testingthi
      + outt;
"""

	output = javascript_hook(example_code, "code.js", line_width=40, formate_global_config={"indent": "      "})
	assert output == formatted_spaces_output

	output = javascript_hook(example_code, "code.js", line_width=40, indent_width=6)
	assert output == formatted_spaces_output

	output = javascript_hook(example_code, "code.js", line_width=40, indent_width=6, formate_global_config={})
	assert output == formatted_spaces_output

	output = javascript_hook(
			example_code,
			"code.js",
			line_width=40,
			indent_width=6,
			formate_global_config={"indent": '\t'},
			)
	assert output == formatted_spaces_output
