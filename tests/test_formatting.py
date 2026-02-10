# stdlib
from typing import List, NamedTuple

# 3rd party
import pytest
from domdf_python_tools.paths import PathPlus

# this package
from formate_js import javascript_hook


def get_message_separator(file_name: PathPlus) -> str:
	if file_name.suffix == ".md":
		return "!!"
	return "=="


def get_spec_starts(file_name: PathPlus, lines: List[str]) -> List[int]:
	result = []
	message_separator = get_message_separator(file_name)

	if not lines[0].startswith(message_separator):
		raise ValueError(
				f"{file_name.parent.name}/{file_name.name}: All spec files should start with a message. (ex. {message_separator} Message {message_separator})"
				)

	for (i, line) in enumerate(lines):
		if line.startswith(message_separator):
			result.append(i)

	return result


class Spec(NamedTuple):
	file_name: str
	message: str
	file_text: str
	expected_text: str
	# is_only: bool
	# is_trace: bool
	# skip: bool
	# skip_format_twice: bool


def get_specs() -> List[Spec]:
	specs_dir = PathPlus(__file__).parent / "specs"
	for spec_file in specs_dir.iterchildren(match="**/*.txt"):
		lines = spec_file.read_lines()
		spec_starts = get_spec_starts(spec_file, lines)
		specs = []

		for i in range(len(spec_starts)):
			start_index = spec_starts[i]

			if len(spec_starts) == i + 1:
				end_index = len(lines)
			else:
				end_index = spec_starts[i + 1]

			message_line = lines[start_index]

			spec = parse_single_spec(spec_file, message_line, lines[(start_index + 1):end_index])
			specs.append(spec)

	return specs


def parse_single_spec(file_name: PathPlus, message_line: str, lines: List[str]) -> Spec:
	file_text = '\n'.join(lines)
	parts = file_text.split("[expect]")
	start_text = parts[0][:len(parts[0]) - len('\n')]  # remove last newline
	expected_text = parts[1][len('\n'):]  # remove first newline
	# lower_case_message_line = message_line.lower()
	message_separator = get_message_separator(file_name)
	# is_trace = "(trace)" in lower_case_message_line

	return Spec(
			file_name=file_name.name,
			message=message_line[len(message_separator):len(message_line) - len(message_separator)].strip(),
			file_text=start_text,
			expected_text=expected_text,
			# is_only = lower_case_message_line.contains("(only)") || is_trace,
			# is_trace=is_trace,
			# skip=lower_case_message_line.contains("(skip)"),
			# skip_format_twice=lower_case_message_line.contains("(skip-format-twice)"),
			)


specs = get_specs()


def param_for_spec(spec: Spec, **kwargs):
	return pytest.param(spec, id=spec.message, **kwargs)


@pytest.mark.parametrize("spec", [param_for_spec(s) for s in specs])
def test_specs(spec: Spec):
	assert javascript_hook(spec.file_text, formate_filename="code.ts", indent_width=4) == spec.expected_text
