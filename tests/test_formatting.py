# stdlib
from typing import Any, Dict, List, NamedTuple, Tuple, Union

# 3rd party
import pytest
import regex as re  # type: ignore[import-untyped]
from domdf_python_tools.paths import PathPlus

# this package
from formate_js import ConfigurationBuilder, javascript_hook


def get_spec_starts(spec_file: PathPlus, lines: List[str]) -> List[int]:
	result = []
	message_separator = "=="

	if not lines[0].startswith(message_separator):
		raise ValueError(
				f"{spec_file.as_posix()}: All spec files should start with a message. (ex. {message_separator} Message {message_separator})",
				)

	for (i, line) in enumerate(lines):
		if line.startswith(message_separator):
			result.append(i)

	return result


class Spec(NamedTuple):
	spec_file: str
	file_path: str
	message: str
	file_text: str
	expected_text: str
	# is_only: bool
	# is_trace: bool
	# skip: bool
	# skip_format_twice: bool
	config: str


def extract_config(file_text: str) -> Tuple[str, str]:
	if not file_text.startswith("~~"):
		return '', file_text

	last_index = file_text.find("~~\n")
	if last_index == -1:
		raise ValueError("Could not find final ~~\\n")

	config_text = file_text[len("~~"):last_index].replace('\n', '').strip()

	return (config_text, file_text[(last_index + len("~~\n")):])


def parse_file_path(file_text: str) -> Tuple[str, str]:
	if not file_text.startswith("--"):
		return ("code.ts", file_text)

	last_index = file_text.find("--\n")
	if last_index == -1:
		raise ValueError("Could not find final --")

	return file_text[len("--"):last_index].strip(), file_text[(last_index + len("--\n")):]


def get_specs() -> List[Spec]:
	specs_dir = PathPlus(__file__).parent / "specs"
	specs = []
	for spec_file in specs_dir.iterchildren(match="**/*.txt"):
		file_text = spec_file.read_text()
		spec_file = spec_file.relative_to(specs_dir)
		(file_path, file_text) = parse_file_path(file_text)
		config, file_text = extract_config(file_text)
		# TODO: action on config
		lines = file_text.split('\n')
		spec_starts = get_spec_starts(spec_file, lines)

		for i in range(len(spec_starts)):
			start_index = spec_starts[i]

			if len(spec_starts) == i + 1:
				end_index = len(lines)
			else:
				end_index = spec_starts[i + 1]

			message_line = lines[start_index]

			spec = parse_single_spec(
					spec_file,
					file_path,
					message_line,
					lines[(start_index + 1):end_index],
					config,
					)
			specs.append(spec)

	return specs


def parse_single_spec(
		spec_file: PathPlus,
		file_path: str,
		message_line: str,
		lines: List[str],
		config: str,
		) -> Spec:
	file_text = '\n'.join(lines)
	parts = file_text.split("[expect]")
	start_text = parts[0][:len(parts[0]) - len('\n')]  # remove last newline
	expected_text = parts[1][len('\n'):]  # remove first newline
	# lower_case_message_line = message_line.lower()
	message_separator = "=="
	# is_trace = "(trace)" in lower_case_message_line

	return Spec(
			spec_file=spec_file.as_posix(),
			file_path=file_path,
			message=message_line[len(message_separator):len(message_line) - len(message_separator)].strip(),
			file_text=start_text,
			expected_text=expected_text,
			# is_only = lower_case_message_line.contains("(only)") || is_trace,
			# is_trace=is_trace,
			# skip=lower_case_message_line.contains("(skip)"),
			# skip_format_twice=lower_case_message_line.contains("(skip-format-twice)"),
			config=config,
			)


_case_boundary_re = re.compile("(\\p{Ll})(\\p{Lu})")
_single_letters_re = re.compile("(\\p{Lu}|\\p{N})(\\p{Lu})(\\p{Ll})")


def to_snake_case(value: str) -> str:
	"""
	Convert the given string into ``snake_case``.

	:param value:
	"""

	# Matches VSCode behaviour
	case_boundary = _case_boundary_re.findall(value)
	single_letters = _single_letters_re.findall(value)
	if not case_boundary and not single_letters:
		return value.lower()
	value = _case_boundary_re.sub(r"\1_\2", value)
	value = _single_letters_re.sub(r"\1_\2\3", value)
	return value.lower()


specs = get_specs()


def parse_config(config: str) -> Dict[str, Any]:

	builder = ConfigurationBuilder()
	builder.indent_width(4)

	if not config:
		return builder.build().to_dict()

	options = config.split(", ")

	value: Union[str, int, bool]
	for option in options:
		key, value = option.split(": ")
		key = to_snake_case(key).replace('.', '_')

		if value == "true":
			value = True
		elif value == "false":
			value = False
		else:
			try:
				value = int(value)
			except ValueError:
				pass

		if key == "deno":
			builder.deno()
		else:
			getattr(builder, key)(value)

	return builder.build().to_dict()


def param_for_spec(spec: Spec, **kwargs):  # noqa: MAN002
	# if spec.config:
	# 	breakpoint()
	return pytest.param(spec, spec.file_path, id=f"{spec.spec_file}/{spec.message}", **kwargs)


@pytest.mark.parametrize("spec, file_path", [param_for_spec(s) for s in specs])
def test_specs(spec: Spec, file_path: str):
	assert javascript_hook(
			spec.file_text,
			formate_filename=file_path,
			**parse_config(spec.config),
			) == spec.expected_text
