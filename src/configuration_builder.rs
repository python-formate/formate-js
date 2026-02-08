use crate::configuration::PyConfiguration;
use dprint_core::configuration::NewLineKind;
use dprint_plugin_typescript::configuration::ConfigurationBuilder;
use dprint_plugin_typescript::configuration::{
	BracePosition, ForceMultiLine, JsxMultiLineParens, JsxQuoteStyle, MemberSpacing,
	NamedTypeImportsExportsOrder, NextControlFlowPosition, OperatorPosition, PreferHanging,
	QuoteProps, QuoteStyle, SameOrNextLinePosition, SemiColonOrComma, SemiColons, SortOrder,
	TrailingCommas, UseBraces, UseParentheses,
};
use pyo3::prelude::*;
use std::str::FromStr;

// TODO: enum classes in stubs

#[pyclass(name = "ConfigurationBuilder", module = "formate_js")]
#[repr(transparent)]
// A wrapper around a [`ConfigurationBuilder`] that can be converted to and from python with `pyo3`.
/// TypeScript formatting configuration builder.
pub struct PyConfigurationBuilder(pub ConfigurationBuilder);

impl PyConfigurationBuilder {
	pub(crate) fn new() -> Self {
		PyConfigurationBuilder(ConfigurationBuilder::new())
	}
}

impl From<PyConfigurationBuilder> for ConfigurationBuilder {
	fn from(value: PyConfigurationBuilder) -> Self {
		value.0
	}
}

macro_rules! wrap_fn {
	($name:tt, $rust_type:ty, $py_type:literal) => {
		#[pymethods]
		impl PyConfigurationBuilder {
			#[pyo3(signature = (value: $py_type) -> "ConfigurationBuilder")]
			fn $name<'a>(
				mut slf: PyRefMut<'a, Self>,
				value: $rust_type,
			) -> PyResult<PyRefMut<'a, Self>> {
				slf.0.$name(value);
				Ok(slf)
			}
		}
	};

	($name:tt, $rust_type:ty, $py_type:literal, $doc:literal) => {
		#[pymethods]
		impl PyConfigurationBuilder {
			#[doc = $doc]
			#[pyo3(signature = (value: $py_type) -> "ConfigurationBuilder")]
			fn $name<'a>(
				mut slf: PyRefMut<'a, Self>,
				value: $rust_type,
			) -> PyResult<PyRefMut<'a, Self>> {
				slf.0.$name(value);
				Ok(slf)
			}
		}
	};
}

macro_rules! wrap_enum_fn {
	($name:tt, $rust_type:ty) => {
		#[pymethods]
		impl PyConfigurationBuilder {
			#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
			fn $name<'a>(mut slf: PyRefMut<'a, Self>, value: &str) -> PyResult<PyRefMut<'a, Self>> {
				slf.0.$name(
					<$rust_type>::from_str(value)
						.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
				);
				Ok(slf)
			}
		}
	};
	($name:tt, $rust_type:ty, $doc:literal) => {
		#[pymethods]
		impl PyConfigurationBuilder {
			#[doc = $doc]
			#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
			fn $name<'a>(mut slf: PyRefMut<'a, Self>, value: &str) -> PyResult<PyRefMut<'a, Self>> {
				slf.0.$name(
					<$rust_type>::from_str(value)
						.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
				);
				Ok(slf)
			}
		}
	};
}

#[pymethods]
impl PyConfigurationBuilder {
	#[new]
	pub fn __init__() -> PyResult<Self> {
		Ok(PyConfigurationBuilder::new())
	}

	// /// Set the global configuration.
	// fn global_config<'a>(
	// 	mut slf: PyRefMut<'a, Self>,
	// 	global_config: GlobalConfiguration,
	// ) -> PyResult<PyRefMut<'a, Self>> {
	// 	slf.0.global_config(value);
	// 	Ok(slf)
	// }

	/// Helper method to set the configuration to what's used for Deno.
	#[pyo3(signature = () -> "ConfigurationBuilder")]
	fn deno<'a>(mut slf: PyRefMut<'a, Self>) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.deno();
		Ok(slf)
	}

	fn build<'a>(slf: PyRefMut<'a, Self>) -> PyResult<PyConfiguration> {
		let configuration = slf.0.build();
		Ok(PyConfiguration::from(configuration))
	}
}

wrap_fn!(
	line_width,
	u32,
	"int",
	"The width of a line the printer will try to stay under. Note that the printer may exceed this width in certain cases.\n\nDefault ```120``"
);

wrap_fn!(
	use_tabs,
	bool,
	"bool",
	"Whether to use tabs (true) or spaces (false).\n\nDefault: :py:obj:`False`"
);

wrap_fn!(
	indent_width,
	u8,
	"int",
	"The number of columns for an indent.\n\nDefault: ```4``"
);

wrap_enum_fn!(
	new_line_kind,
	NewLineKind,
	"The kind of newline to use.\n\nDefault: ```NewLineKind.LineFeed``"
);

wrap_enum_fn!(
	quote_style,
	QuoteStyle,
	"The quote style to use.\n\nDefault: ```QuoteStyle.AlwaysDouble``"
);

wrap_enum_fn!(
	jsx_quote_style,
	JsxQuoteStyle,
	"The JSX quote style to use for string literals in JSX attributes.\n\nDefault: ```JsxQuoteStyle.PreferDouble``"
);

wrap_enum_fn!(
	jsx_multi_line_parens,
	JsxMultiLineParens,
	"Whether to surround a JSX element or fragment with parentheses when it's the top JSX node and it spans multiple lines.\n\nDefault: ```JsxMultiLineParens.Prefer``"
);

wrap_fn!(
	jsx_force_new_lines_surrounding_content,
	bool,
	"bool",
	"Forces newlines surrounding the content of JSX elements.\n\nDefault: :py:obj:`False`"
);

wrap_enum_fn!(
	jsx_bracket_position,
	SameOrNextLinePosition,
	"If the end angle bracket of a jsx opening element or self closing element should be on the same or next line when the attributes span multiple lines.\n\nDefault: nextLine"
);

wrap_enum_fn!(
	jsx_opening_element_bracket_position,
	SameOrNextLinePosition,
	"If the end angle bracket of a jsx opening element should be on the same or next line when the attributes span multiple lines.\n\nDefault: nextLine"
);

wrap_enum_fn!(
	jsx_self_closing_element_bracket_position,
	SameOrNextLinePosition,
	"If the end angle bracket of a jsx self closing element should be on the same or next line when the attributes span multiple lines.\n\nDefault: nextLine"
);

wrap_enum_fn!(
	semi_colons,
	SemiColons,
	"Whether statements should end in a semi-colon.\n\nDefault: ```SemiColons.Prefer``"
);

wrap_fn!(
	prefer_hanging,
	bool,
	"bool",
	"Set to prefer hanging indentation when exceeding the line width.\n\nDefault: :py:obj:`False`"
);

wrap_enum_fn!(
	quote_props,
	QuoteProps,
	"Behaviour to use for quotes on property names.\n\nDefault: ```preserve``"
);

wrap_enum_fn!(
	brace_position,
	BracePosition,
	"Where to place the opening brace.\n\nDefault: ```BracePosition.SameLineUnlessHanging``"
);

wrap_enum_fn!(
	next_control_flow_position,
	NextControlFlowPosition,
	"Where to place the next control flow within a control flow statement.\n\nDefault: ```NextControlFlowPosition.NextLine``"
);

wrap_enum_fn!(
	operator_position,
	OperatorPosition,
	"Where to place the operator for expressions that span multiple lines.\n\nDefault: ```OperatorPosition.NextLine``"
);

wrap_enum_fn!(
	single_body_position,
	SameOrNextLinePosition,
	"Where to place the expression of a statement that could possibly be on one line (ex. ``if (true) console.log(5);`).\n\nDefault: ```SingleBodyPosition.Maintain``"
);

wrap_fn!(
	file_indent_level,
	u16,
	"int",
	"Amount of indents to use for the whole file.\n\nThis should only be set by tools that need to indent all the code in the file.\n\nDefault: ```0``"
);

wrap_enum_fn!(
	trailing_commas,
	TrailingCommas,
	"If trailing commas should be used.\n\nDefault: ```TrailingCommas.OnlyMultiLine``"
);

wrap_enum_fn!(
	use_braces,
	UseBraces,
	"If braces should be used or not.\n\nDefault: ```UseBraces.WhenNotSingleLine``"
);

wrap_fn!(
	prefer_single_line,
	bool,
	"bool",
	"If code should revert back from being on multiple lines to being on a single line when able.\n\nDefault: :py:obj:`False`"
);

wrap_fn!(
	binary_expression_space_surrounding_bitwise_and_arithmetic_operator,
	bool,
	"bool",
	"Whether to surround bitwise and arithmetic operators in a binary expression with spaces.\n\n* ``true`` (default) - Ex. ``1 + 2`\n* ``false`` - Ex. ``1+2`"
);

wrap_fn!(
	comment_line_force_space_after_slashes,
	bool,
	"bool",
	"Forces a space after the double slash in a comment line.\n\n* ``true`` (default) - Ex. ``//test`` -> ``// test`\n* ``false`` - Ex. ``//test`` -> ``//test`"
);

wrap_fn!(
	construct_signature_space_after_new_keyword,
	bool,
	"bool",
	"Whether to add a space after the ``new`` keyword in a construct signature.\n\n * ``true`` - Ex. ``new (): MyClass;`\n * ``false`` (default) - Ex. ``new(): MyClass;`"
);

wrap_fn!(
	constructor_space_before_parentheses,
	bool,
	"bool",
	"Whether to add a space before the parentheses of a constructor.\n\n * ``true`` - Ex. ``constructor ()`\n * ``false`` (false) - Ex. ``constructor()`"
);

wrap_fn!(
	constructor_type_space_after_new_keyword,
	bool,
	"bool",
	"Whether to add a space after the ``new`` keyword in a constructor type.\n\n * ``true`` - Ex. ``type MyClassCtor = new () => MyClass;`\n * ``false`` (default) - Ex. ``type MyClassCtor = new() => MyClass;`"
);

wrap_fn!(
	do_while_statement_space_after_while_keyword,
	bool,
	"bool",
	"Whether to add a space after the ``while`` keyword in a do while statement.\n\n * ``true`` (true) - Ex. ``do {\n} while (condition);`\n * ``false`` - Ex. ``do {\n} while(condition);`"
);

wrap_fn!(
	export_declaration_space_surrounding_named_exports,
	bool,
	"bool",
	"Whether to add spaces around named exports in an export declaration.\n\n * ``true`` (default) - Ex. ``export { SomeExport, OtherExport };`\n * ``false`` - Ex. ``export {SomeExport, OtherExport};`"
);

wrap_fn!(
	for_statement_space_after_for_keyword,
	bool,
	"bool",
	"Whether to add a space after the ``for`` keyword in a 'for' statement.\n\n * ``true`` (default) - Ex. ``for (let i = 0; i < 5; i++)`\n * ``false`` - Ex. ``for(let i = 0; i < 5; i++)`"
);

wrap_fn!(
	for_statement_space_after_semi_colons,
	bool,
	"bool",
	"Whether to add a space after the semi-colons in a 'for' statement.\n\n * ``true`` (default) - Ex. ``for (let i = 0; i < 5; i++)`\n * ``false`` - Ex. ``for (let i = 0;i < 5;i++)`"
);

wrap_fn!(
	for_in_statement_space_after_for_keyword,
	bool,
	"bool",
	"Whether to add a space after the ``for`` keyword in a 'for in' statement.\n\n * ``true`` (default) - Ex. ``for (const prop in obj)`\n * ``false`` - Ex. ``for(const prop in obj)`
	"
);

wrap_fn!(
	for_of_statement_space_after_for_keyword,
	bool,
	"bool",
	"Whether to add a space after the ``for`` keyword in a 'for of' statement.\n\n * ``true`` (default) - Ex. ``for (const value of myArray)`\n * ``false`` - Ex. ``for(const value of myArray)`"
);

wrap_fn!(
	function_declaration_space_before_parentheses,
	bool,
	"bool",
	"Whether to add a space before the parentheses of a function declaration.\n\n * ``true`` - Ex. ``function myFunction ()`\n * ``false`` (default) - Ex. ``function myFunction()`"
);

wrap_fn!(
	function_expression_space_before_parentheses,
	bool,
	"bool",
	"Whether to add a space before the parentheses of a function expression.\n\n * ``true`` - Ex. ``function<T> ()`\n * ``false`` (default) - Ex. ``function<T> ()`"
);

wrap_fn!(
	function_expression_space_after_function_keyword,
	bool,
	"bool",
	"Whether to add a space after the function keyword of a function expression.\n\n * ``true`` - Ex. ``function <T>()`.\n * ``false`` (default) - Ex. ``function<T>()`"
);

wrap_fn!(
	get_accessor_space_before_parentheses,
	bool,
	"bool",
	"Whether to add a space before the parentheses of a get accessor.\n\n * ``true`` - Ex. ``get myProp ()`\n * ``false`` (false) - Ex. ``get myProp()`"
);

wrap_fn!(
	if_statement_space_after_if_keyword,
	bool,
	"bool",
	"Whether to add a space after the ``if`` keyword in an 'if' statement.\n\n * ``true`` (default) - Ex. ``if (true)`\n * ``false`` - Ex. ``if(true)`"
);

wrap_fn!(
	import_declaration_space_surrounding_named_imports,
	bool,
	"bool",
	"Whether to add spaces around named imports in an import declaration.\n\n * ``true`` (default) - Ex. ``import { SomeExport, OtherExport } from \"my-module\";`\n * ``false`` - Ex. ``import {SomeExport, OtherExport} from \"my-module\";`"
);

wrap_fn!(
	jsx_expression_container_space_surrounding_expression,
	bool,
	"bool",
	"Whether to add a space surrounding the expression of a JSX container.\n\n * ``true`` - Ex. ``{ myValue }`\n * ``false`` (default) - Ex. ``{myValue}`
	"
);

wrap_fn!(
	jsx_self_closing_element_space_before_slash,
	bool,
	"bool",
	"Whether to add a space before the slash in a self closing tag for a JSX element.\n\n * ``true`` (default) - Ex. ``<Test />`\n * ``false`` - Ex. ``<Test/>`"
);

wrap_fn!(
	object_expression_space_surrounding_properties,
	bool,
	"bool",
	"Whether to add a space surrounding the properties of an object expression.\n\n * ``true`` (default) - Ex. ``{ key: value }`\n * ``false`` - Ex. ``{key: value}`"
);

wrap_fn!(
	object_pattern_space_surrounding_properties,
	bool,
	"bool",
	"Whether to add a space surrounding the properties of an object pattern.\n\n * ``true`` (default) - Ex. ``{ key: value } = obj`\n * ``false`` - Ex. ``{key: value} = obj`"
);

wrap_fn!(
	method_space_before_parentheses,
	bool,
	"bool",
	"Whether to add a space before the parentheses of a method.\n\n * ``true`` - Ex. ``myMethod ()`\n * ``false`` - Ex. ``myMethod()`"
);

wrap_fn!(
	set_accessor_space_before_parentheses,
	bool,
	"bool",
	"Whether to add a space before the parentheses of a set accessor.\n\n * ``true`` - Ex. ``set myProp (value: string)`\n * ``false`` (default) - Ex. ``set myProp(value: string)`"
);

wrap_fn!(
	space_surrounding_properties,
	bool,
	"bool",
	"Whether to add a space surrounding the properties of object-like nodes.\n\n * ``true`` (default) - Ex. ``{ key: value }`\n * ``false`` - Ex. ``{key: value}`"
);

wrap_fn!(
	tagged_template_space_before_literal,
	bool,
	"bool",
	"Whether to add a space before the literal in a tagged template.\n\n* ``true`` (default) - Ex. ``html \\`<element />\\``\n* ``false`` - Ex. ``html\\`<element />\\``"
);

wrap_fn!(
	type_annotation_space_before_colon,
	bool,
	"bool",
	"Whether to add a space before the colon of a type annotation.\n\n * ``true`` - Ex. ``function myFunction() : string`\n * ``false`` (default) - Ex. ``function myFunction(): string`"
);

wrap_fn!(
	type_assertion_space_before_expression,
	bool,
	"bool",
	"Whether to add a space before the expression in a type assertion.\n\n * ``true`` (default) - Ex. ``<string> myValue`\n * ``false`` - Ex. ``<string>myValue`"
);

wrap_fn!(
	type_literal_space_surrounding_properties,
	bool,
	"bool",
	"Whether to add a space surrounding the properties of a type literal.\n\n * ``true`` (default) - Ex. ``value: { key: Type }`\n * ``false`` - Ex. ``value: {key: Type}`"
);

wrap_fn!(
	while_statement_space_after_while_keyword,
	bool,
	"bool",
	"Whether to add a space after the ``while`` keyword in a while statement.\n\n * ``true`` (default) - Ex. ``while (true)`\n * ``false`` - Ex. ``while(true)`"
);

wrap_fn!(
	space_around,
	bool,
	"bool",
	"Whether to place spaces around enclosed expressions.\n\n* ``true`` - Ex. ``myFunction( true )`\n* ``false`` (default) - Ex. ``myFunction(true)`"
);

wrap_enum_fn!(
	arrow_function_use_parentheses,
	UseParentheses,
	"Whether to use parentheses for arrow functions.\n\nDefault: ```UseParentheses.Maintain``"
);
wrap_fn!(
	binary_expression_line_per_expression,
	bool,
	"bool",
	"Whether to force a line per expression when spanning multiple lines.\n\n * ``true`` - Formats with each part on a new line.\n * ``false`` (default) - Maintains the line breaks as written by the programmer."
);

wrap_fn!(
	conditional_expression_line_per_expression,
	bool,
	"bool",
	"Whether to force a line per expression when spanning multiple lines.\n\n * ``true`` - Formats with each part on a new line.\n * ``false`` (default) - Maintains the line breaks as written by the programmer."
);

wrap_fn!(
	member_expression_line_per_expression,
	bool,
	"bool",
	"Whether to force a line per expression when spanning multiple lines.\n\n * ``true`` - Formats with each part on a new line.\n * ``false`` (default) - Maintains the line breaks as written by the programmer."
);

wrap_enum_fn!(
	type_literal_separator_kind,
	SemiColonOrComma,
	"The kind of separator to use in type literals."
);

wrap_enum_fn!(
	type_literal_separator_kind_single_line,
	SemiColonOrComma,
	"The kind of separator to use in type literals when single line."
);

wrap_enum_fn!(
	type_literal_separator_kind_multi_line,
	SemiColonOrComma,
	"The kind of separator to use in type literals when multi-line."
);

wrap_enum_fn!(
	module_sort_import_declarations,
	SortOrder,
	"Alphabetically sorts the import declarations based on their module specifiers.\n\nDefault: Case insensitive"
);

wrap_enum_fn!(
	module_sort_export_declarations,
	SortOrder,
	"Alphabetically sorts the export declarations based on their module specifiers.\n\nDefault: Case insensitive"
);

wrap_enum_fn!(
	import_declaration_sort_named_imports,
	SortOrder,
	"Alphabetically sorts the import declaration's named imports.\n\nDefault: Case insensitive"
);

wrap_enum_fn!(
	import_declaration_sort_type_only_imports,
	NamedTypeImportsExportsOrder,
	"Sorts type-only named imports first, last, or none (no sorting).\n\nDefault: Last"
);

wrap_enum_fn!(
	export_declaration_sort_named_exports,
	SortOrder,
	"Alphabetically sorts the export declaration's named exports.\n\nDefault: Case insensitive"
);

wrap_enum_fn!(
	export_declaration_sort_type_only_exports,
	NamedTypeImportsExportsOrder,
	"Sorts type-only named exports first, last, or none (no sorting).\n\nDefault: Last"
);

wrap_fn!(
	ignore_node_comment_text,
	&str,
	"str",
	"The text to use for an ignore comment (ex. ``\"dprint-ignore\"``).\n\nDefault: \"dprint-ignore\""
);

wrap_fn!(
	ignore_file_comment_text,
	&str,
	"str",
	"The text to use for a file ignore comment (ex. ``\"dprint-ignore-file\"``).\n\nDefault: \"dprint-ignore-file\""
);

wrap_enum_fn!(arrow_function_brace_position, BracePosition);
wrap_enum_fn!(class_declaration_brace_position, BracePosition);
wrap_enum_fn!(class_expression_brace_position, BracePosition);
wrap_enum_fn!(constructor_brace_position, BracePosition);
wrap_enum_fn!(do_while_statement_brace_position, BracePosition);
wrap_enum_fn!(enum_declaration_brace_position, BracePosition);
wrap_enum_fn!(for_statement_brace_position, BracePosition);
wrap_enum_fn!(for_in_statement_brace_position, BracePosition);
wrap_enum_fn!(for_of_statement_brace_position, BracePosition);
wrap_enum_fn!(get_accessor_brace_position, BracePosition);
wrap_enum_fn!(if_statement_brace_position, BracePosition);
wrap_enum_fn!(interface_declaration_brace_position, BracePosition);
wrap_enum_fn!(function_declaration_brace_position, BracePosition);
wrap_enum_fn!(function_expression_brace_position, BracePosition);
wrap_enum_fn!(method_brace_position, BracePosition);
wrap_enum_fn!(module_declaration_brace_position, BracePosition);
wrap_enum_fn!(set_accessor_brace_position, BracePosition);
wrap_enum_fn!(static_block_brace_position, BracePosition);
wrap_enum_fn!(switch_case_brace_position, BracePosition);
wrap_enum_fn!(switch_statement_brace_position, BracePosition);
wrap_enum_fn!(try_statement_brace_position, BracePosition);
wrap_enum_fn!(while_statement_brace_position, BracePosition);
wrap_enum_fn!(arguments_prefer_hanging, PreferHanging);
wrap_enum_fn!(array_expression_prefer_hanging, PreferHanging);
wrap_fn!(array_pattern_prefer_hanging, bool, "bool");
wrap_fn!(do_while_statement_prefer_hanging, bool, "bool");
wrap_fn!(export_declaration_prefer_hanging, bool, "bool");
wrap_fn!(extends_clause_prefer_hanging, bool, "bool");
wrap_fn!(for_in_statement_prefer_hanging, bool, "bool");
wrap_fn!(for_of_statement_prefer_hanging, bool, "bool");
wrap_fn!(for_statement_prefer_hanging, bool, "bool");
wrap_fn!(if_statement_prefer_hanging, bool, "bool");
wrap_fn!(implements_clause_prefer_hanging, bool, "bool");
wrap_fn!(import_declaration_prefer_hanging, bool, "bool");
wrap_fn!(jsx_attributes_prefer_hanging, bool, "bool");
wrap_fn!(object_expression_prefer_hanging, bool, "bool");
wrap_fn!(object_pattern_prefer_hanging, bool, "bool");
wrap_enum_fn!(parameters_prefer_hanging, PreferHanging);
wrap_fn!(sequence_expression_prefer_hanging, bool, "bool");
wrap_fn!(switch_statement_prefer_hanging, bool, "bool");
wrap_enum_fn!(tuple_type_prefer_hanging, PreferHanging);
wrap_fn!(type_literal_prefer_hanging, bool, "bool");
wrap_enum_fn!(type_parameters_prefer_hanging, PreferHanging);
wrap_fn!(union_and_intersection_type_prefer_hanging, bool, "bool");
wrap_fn!(variable_statement_prefer_hanging, bool, "bool");
wrap_fn!(while_statement_prefer_hanging, bool, "bool");
wrap_fn!(export_declaration_force_single_line, bool, "bool");
wrap_fn!(import_declaration_force_single_line, bool, "bool");
wrap_enum_fn!(export_declaration_force_multi_line, ForceMultiLine);
wrap_enum_fn!(import_declaration_force_multi_line, ForceMultiLine);
wrap_enum_fn!(enum_declaration_member_spacing, MemberSpacing);

wrap_enum_fn!(
	if_statement_next_control_flow_position,
	NextControlFlowPosition
);

wrap_enum_fn!(
	try_statement_next_control_flow_position,
	NextControlFlowPosition
);

wrap_enum_fn!(
	do_while_statement_next_control_flow_position,
	NextControlFlowPosition
);

wrap_enum_fn!(binary_expression_operator_position, OperatorPosition);
wrap_enum_fn!(conditional_expression_operator_position, OperatorPosition);
wrap_enum_fn!(conditional_type_operator_position, OperatorPosition);
wrap_enum_fn!(if_statement_single_body_position, SameOrNextLinePosition);
wrap_enum_fn!(for_statement_single_body_position, SameOrNextLinePosition);

wrap_enum_fn!(
	for_in_statement_single_body_position,
	SameOrNextLinePosition
);

wrap_enum_fn!(
	for_of_statement_single_body_position,
	SameOrNextLinePosition
);

wrap_enum_fn!(while_statement_single_body_position, SameOrNextLinePosition);
wrap_enum_fn!(arguments_trailing_commas, TrailingCommas);
wrap_enum_fn!(parameters_trailing_commas, TrailingCommas);
wrap_enum_fn!(array_expression_trailing_commas, TrailingCommas);
wrap_enum_fn!(array_pattern_trailing_commas, TrailingCommas);
wrap_enum_fn!(enum_declaration_trailing_commas, TrailingCommas);
wrap_enum_fn!(export_declaration_trailing_commas, TrailingCommas);
wrap_enum_fn!(import_declaration_trailing_commas, TrailingCommas);
wrap_enum_fn!(object_expression_trailing_commas, TrailingCommas);
wrap_enum_fn!(object_pattern_trailing_commas, TrailingCommas);
wrap_enum_fn!(tuple_type_trailing_commas, TrailingCommas);

wrap_enum_fn!(
	type_literal_trailing_commas,
	TrailingCommas,
	"Only applies when using commas on type literals."
);

wrap_enum_fn!(type_parameters_trailing_commas, TrailingCommas);
wrap_enum_fn!(if_statement_use_braces, UseBraces);
wrap_enum_fn!(for_statement_use_braces, UseBraces);
wrap_enum_fn!(for_in_statement_use_braces, UseBraces);
wrap_enum_fn!(for_of_statement_use_braces, UseBraces);
wrap_enum_fn!(while_statement_use_braces, UseBraces);
wrap_fn!(array_expression_prefer_single_line, bool, "bool");
wrap_fn!(array_pattern_prefer_single_line, bool, "bool");
wrap_fn!(arguments_prefer_single_line, bool, "bool");
wrap_fn!(binary_expression_prefer_single_line, bool, "bool");
wrap_fn!(computed_prefer_single_line, bool, "bool");
wrap_fn!(conditional_expression_prefer_single_line, bool, "bool");
wrap_fn!(conditional_type_prefer_single_line, bool, "bool");
wrap_fn!(decorators_prefer_single_line, bool, "bool");
wrap_fn!(export_declaration_prefer_single_line, bool, "bool");
wrap_fn!(for_statement_prefer_single_line, bool, "bool");
wrap_fn!(import_declaration_prefer_single_line, bool, "bool");
wrap_fn!(jsx_attributes_prefer_single_line, bool, "bool");
wrap_fn!(jsx_element_prefer_single_line, bool, "bool");
wrap_fn!(mapped_type_prefer_single_line, bool, "bool");
wrap_fn!(member_expression_prefer_single_line, bool, "bool");
wrap_fn!(object_expression_prefer_single_line, bool, "bool");
wrap_fn!(object_pattern_prefer_single_line, bool, "bool");
wrap_fn!(parameters_prefer_single_line, bool, "bool");
wrap_fn!(parentheses_prefer_single_line, bool, "bool");
wrap_fn!(tuple_type_prefer_single_line, bool, "bool");
wrap_fn!(type_literal_prefer_single_line, bool, "bool");
wrap_fn!(type_parameters_prefer_single_line, bool, "bool");
wrap_fn!(union_and_intersection_type_prefer_single_line, bool, "bool");
wrap_fn!(variable_statement_prefer_single_line, bool, "bool");
wrap_fn!(arguments_space_around, bool, "bool");
wrap_fn!(array_expression_space_around, bool, "bool");
wrap_fn!(array_pattern_space_around, bool, "bool");
wrap_fn!(catch_clause_space_around, bool, "bool");
wrap_fn!(do_while_statement_space_around, bool, "bool");
wrap_fn!(for_in_statement_space_around, bool, "bool");
wrap_fn!(for_of_statement_space_around, bool, "bool");
wrap_fn!(for_statement_space_around, bool, "bool");
wrap_fn!(if_statement_space_around, bool, "bool");
wrap_fn!(parameters_space_around, bool, "bool");
wrap_fn!(paren_expression_space_around, bool, "bool");
wrap_fn!(switch_statement_space_around, bool, "bool");
wrap_fn!(tuple_type_space_around, bool, "bool");
wrap_fn!(while_statement_space_around, bool, "bool");
