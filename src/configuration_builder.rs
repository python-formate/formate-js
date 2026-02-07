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

#[pymethods]
impl PyConfigurationBuilder {
	#[new]
	pub fn __init__() -> PyResult<Self> {
		Ok(PyConfigurationBuilder::new())
	}

	/// Set the global configuration.
	// fn global_config<'a>(
	// 	mut slf: PyRefMut<'a, Self>,
	// 	global_config: GlobalConfiguration,
	// ) -> PyResult<PyRefMut<'a, Self>> {
	// 	slf.0.global_config(value);
	// 	Ok(slf)
	// }

	#[pyo3(signature = () -> "ConfigurationBuilder")]
	/// Helper method to set the configuration to what’s used for Deno.
	fn deno<'a>(mut slf: PyRefMut<'a, Self>) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.deno();
		Ok(slf)
	}

	/// The width of a line the printer will try to stay under. Note that the printer may exceed this width in certain cases.
	///
	/// Default: 120
	#[pyo3(signature = (value: "int") -> "ConfigurationBuilder")]
	fn line_width<'a>(mut slf: PyRefMut<'a, Self>, value: u32) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.line_width(value);
		Ok(slf)
	}

	/// Whether to use tabs (true) or spaces (false).
	///
	/// Default: false
	#[pyo3(signature = (value: "bool") -> "ConfigurationBuilder")]
	fn use_tabs<'a>(mut slf: PyRefMut<'a, Self>, value: bool) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.use_tabs(value);
		Ok(slf)
	}

	/// The number of columns for an indent.
	///
	/// Default: 4
	#[pyo3(signature = (value: "int") -> "ConfigurationBuilder")]
	fn indent_width<'a>(mut slf: PyRefMut<'a, Self>, value: u8) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.indent_width(value);
		Ok(slf)
	}

	/// /// The kind of newline to use.
	///
	/// Default: NewLineKind::LineFeed
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn new_line_kind<'a>(mut slf: PyRefMut<'a, Self>, value: &str) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.new_line_kind(
			NewLineKind::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// The quote style to use.
	///
	/// Default: QuoteStyle::AlwaysDouble
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn quote_style<'a>(mut slf: PyRefMut<'a, Self>, value: &str) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.quote_style(
			QuoteStyle::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// The JSX quote style to use for string literals in JSX attributes.
	///
	/// Default: JsxQuoteStyle::PreferDouble
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn jsx_quote_style<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.jsx_quote_style(
			JsxQuoteStyle::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Whether to surround a JSX element or fragment with parentheses when it’s the top JSX node and it spans multiple lines.
	///
	/// Default: JsxMultiLineParens::Prefer
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn jsx_multi_line_parens<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.jsx_multi_line_parens(
			JsxMultiLineParens::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Forces newlines surrounding the content of JSX elements.
	///
	/// Default: false
	fn jsx_force_new_lines_surrounding_content<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.jsx_force_new_lines_surrounding_content(value);
		Ok(slf)
	}

	/// If the end angle bracket of a jsx opening element or self closing element should be on the same or next line when the attributes span multiple lines.
	///
	/// Default: nextLine
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn jsx_bracket_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.jsx_bracket_position(
			SameOrNextLinePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// If the end angle bracket of a jsx opening element should be on the same or next line when the attributes span multiple lines.
	///
	/// Default: nextLine
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn jsx_opening_element_bracket_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.jsx_opening_element_bracket_position(
			SameOrNextLinePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// If the end angle bracket of a jsx self closing element should be on the same or next line when the attributes span multiple lines.
	///
	/// Default: nextLine
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn jsx_self_closing_element_bracket_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.jsx_self_closing_element_bracket_position(
			SameOrNextLinePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Whether statements should end in a semi-colon.
	///
	/// Default: SemiColons::Prefer
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn semi_colons<'a>(mut slf: PyRefMut<'a, Self>, value: &str) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.semi_colons(
			SemiColons::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Set to prefer hanging indentation when exceeding the line width.
	///
	/// Default: false
	fn prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.prefer_hanging(value);
		Ok(slf)
	}

	/// Behaviour to use for quotes on property names.
	///
	/// Default: preserve
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn quote_props<'a>(mut slf: PyRefMut<'a, Self>, value: &str) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.quote_props(
			QuoteProps::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Where to place the opening brace.
	///
	/// Default: BracePosition::SameLineUnlessHanging
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum valuevalue '{}'", &value)),
		);
		Ok(slf)
	}

	/// Where to place the next control flow within a control flow statement.
	///
	/// Default: NextControlFlowPosition::NextLine
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn next_control_flow_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.next_control_flow_position(
			NextControlFlowPosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Where to place the operator for expressions that span multiple lines.
	///
	/// Default: OperatorPosition::NextLine
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn operator_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.operator_position(
			OperatorPosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Where to place the expression of a statement that could possibly be on one line (ex. if (true) console.log(5);).
	///
	/// Default: SameOrNextLinePosition::Maintain
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn single_body_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.single_body_position(
			SameOrNextLinePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Amount of indents to use for the whole file.
	///
	/// This should only be set by tools that need to indent all the code in the file.
	///
	/// Default: 0
	#[pyo3(signature = (value: "int") -> "ConfigurationBuilder")]
	fn file_indent_level<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: u16,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.file_indent_level(value);
		Ok(slf)
	}

	/// If trailing commas should be used.
	///
	/// Default: TrailingCommas::OnlyMultiLine
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn trailing_commas<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.trailing_commas(
			TrailingCommas::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// If braces should be used or not.
	///
	/// Default: UseBraces::WhenNotSingleLine
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn use_braces<'a>(mut slf: PyRefMut<'a, Self>, value: &str) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.use_braces(
			UseBraces::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// If code should revert back from being on multiple lines to being on a single line when able.
	///
	/// Default: false
	fn prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.prefer_single_line(value);
		Ok(slf)
	}

	/// Whether to surround bitwise and arithmetic operators in a binary expression with spaces.
	///
	/// 	true (default) - Ex. 1 + 2
	/// 	false - Ex. 1+2
	fn binary_expression_space_surrounding_bitwise_and_arithmetic_operator<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0
			.binary_expression_space_surrounding_bitwise_and_arithmetic_operator(value);
		Ok(slf)
	}

	/// Forces a space after the double slash in a comment line.
	///
	/// true (default) - Ex. //test -> // test false - Ex. //test -> //test
	fn comment_line_force_space_after_slashes<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.comment_line_force_space_after_slashes(value);
		Ok(slf)
	}

	/// Whether to add a space after the new keyword in a construct signature.
	///
	/// true - Ex. new (): MyClass; false (default) - Ex. new(): MyClass;
	fn construct_signature_space_after_new_keyword<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.construct_signature_space_after_new_keyword(value);
		Ok(slf)
	}

	/// Whether to add a space before the parentheses of a constructor.
	///
	/// true - Ex. constructor () false (false) - Ex. constructor()
	fn constructor_space_before_parentheses<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.constructor_space_before_parentheses(value);
		Ok(slf)
	}

	/// Whether to add a space after the new keyword in a constructor type.
	///
	/// true - Ex. type MyClassCtor = new () => MyClass; false (default) - Ex. type MyClassCtor = new() => MyClass;
	fn constructor_type_space_after_new_keyword<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.constructor_type_space_after_new_keyword(value);
		Ok(slf)
	}

	/// Whether to add a space after the while keyword in a do while statement.
	///
	/// true (true) - Ex. do {\n} while (condition); false - Ex. do {\n} while(condition);
	fn do_while_statement_space_after_while_keyword<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.do_while_statement_space_after_while_keyword(value);
		Ok(slf)
	}

	/// Whether to add spaces around named exports in an export declaration.
	///
	/// 	true (default) - Ex. export { SomeExport, OtherExport };
	/// 	false - Ex. export {SomeExport, OtherExport};
	fn export_declaration_space_surrounding_named_exports<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0
			.export_declaration_space_surrounding_named_exports(value);
		Ok(slf)
	}

	/// Whether to add a space after the for keyword in a “for” statement.
	///
	/// 	true (default) - Ex. for (let i = 0; i < 5; i++)
	/// 	false - Ex. for(let i = 0; i < 5; i++)
	fn for_statement_space_after_for_keyword<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_statement_space_after_for_keyword(value);
		Ok(slf)
	}

	/// Whether to add a space after the semi-colons in a “for” statement.
	///
	/// 	true (default) - Ex. for (let i = 0; i < 5; i++)
	/// 	false - Ex. for (let i = 0;i < 5;i++)
	fn for_statement_space_after_semi_colons<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_statement_space_after_semi_colons(value);
		Ok(slf)
	}

	/// Whether to add a space after the for keyword in a “for in” statement.
	///
	/// 	true (default) - Ex. for (const prop in obj)
	/// 	false - Ex. for(const prop in obj)
	fn for_in_statement_space_after_for_keyword<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_in_statement_space_after_for_keyword(value);
		Ok(slf)
	}

	/// Whether to add a space after the for keyword in a “for of” statement.
	///
	/// 	true (default) - Ex. for (const value of myArray)
	/// 	false - Ex. for(const value of myArray)
	fn for_of_statement_space_after_for_keyword<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_of_statement_space_after_for_keyword(value);
		Ok(slf)
	}

	/// Whether to add a space before the parentheses of a function declaration.
	///
	/// 	true - Ex. function myFunction ()
	/// 	false (default) - Ex. function myFunction()
	fn function_declaration_space_before_parentheses<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.function_declaration_space_before_parentheses(value);
		Ok(slf)
	}

	/// Whether to add a space before the parentheses of a function expression.
	///
	/// true - Ex. function<T> () false (default) - Ex. function<T> ()
	fn function_expression_space_before_parentheses<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.function_expression_space_before_parentheses(value);
		Ok(slf)
	}

	/// Whether to add a space after the function keyword of a function expression.
	///
	/// true - Ex. function <T>(). false (default) - Ex. function<T>()
	fn function_expression_space_after_function_keyword<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0
			.function_expression_space_after_function_keyword(value);
		Ok(slf)
	}

	/// Whether to add a space before the parentheses of a get accessor.
	///
	/// true - Ex. get myProp () false (false) - Ex. get myProp()
	fn get_accessor_space_before_parentheses<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.get_accessor_space_before_parentheses(value);
		Ok(slf)
	}

	/// Whether to add a space after the if keyword in an “if” statement.
	///
	/// true (default) - Ex. if (true) false - Ex. if(true)
	fn if_statement_space_after_if_keyword<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.if_statement_space_after_if_keyword(value);
		Ok(slf)
	}

	/// Whether to add spaces around named imports in an import declaration.
	///
	/// 	true (default) - Ex. import { SomeExport, OtherExport } from "my-module";
	/// 	false - Ex. import {SomeExport, OtherExport} from "my-module";
	fn import_declaration_space_surrounding_named_imports<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0
			.import_declaration_space_surrounding_named_imports(value);
		Ok(slf)
	}

	/// Whether to add a space surrounding the expression of a JSX container.
	///
	/// 	true - Ex. { myValue }
	/// 	false (default) - Ex. {myValue}
	fn jsx_expression_container_space_surrounding_expression<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0
			.jsx_expression_container_space_surrounding_expression(value);
		Ok(slf)
	}

	/// Whether to add a space before the slash in a self closing tag for a JSX element.
	///
	/// 	true (default) - Ex. <Test />
	/// 	false - Ex. <Test/>
	fn jsx_self_closing_element_space_before_slash<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.jsx_self_closing_element_space_before_slash(value);
		Ok(slf)
	}

	/// Whether to add a space surrounding the properties of an object expression.
	///
	/// 	true (default) - Ex. { key: value }
	/// 	false - Ex. {key: value}
	fn object_expression_space_surrounding_properties<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.object_expression_space_surrounding_properties(value);
		Ok(slf)
	}

	/// Whether to add a space surrounding the properties of an object pattern.
	///
	/// 	true (default) - Ex. { key: value } = obj
	/// 	false - Ex. {key: value} = obj
	fn object_pattern_space_surrounding_properties<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.object_pattern_space_surrounding_properties(value);
		Ok(slf)
	}

	/// Whether to add a space before the parentheses of a method.
	///
	/// true - Ex. myMethod () false - Ex. myMethod()
	///
	fn method_space_before_parentheses<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.method_space_before_parentheses(value);
		Ok(slf)
	}

	/// Whether to add a space before the parentheses of a set accessor.
	///
	/// true - Ex. set myProp (value: string) false (default) - Ex. set myProp(value: string)
	///
	fn set_accessor_space_before_parentheses<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.set_accessor_space_before_parentheses(value);
		Ok(slf)
	}

	/// Whether to add a space surrounding the properties of object-like nodes.
	///
	/// 	true (default) - Ex. { key: value }
	/// 	false - Ex. {key: value}
	fn space_surrounding_properties<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.space_surrounding_properties(value);
		Ok(slf)
	}

	/// Whether to add a space before the literal in a tagged template.
	///
	/// 	true (default) - Ex. html \``
	/// 	false - Ex. html\``
	fn tagged_template_space_before_literal<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.tagged_template_space_before_literal(value);
		Ok(slf)
	}

	/// Whether to add a space before the colon of a type annotation.
	///
	/// 	true - Ex. function myFunction() : string
	/// 	false (default) - Ex. function myFunction(): string
	fn type_annotation_space_before_colon<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.type_annotation_space_before_colon(value);
		Ok(slf)
	}

	/// Whether to add a space before the expression in a type assertion.
	///
	/// 	true (default) - Ex. <string> myValue
	/// 	false - Ex. <string>myValue
	fn type_assertion_space_before_expression<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.type_assertion_space_before_expression(value);
		Ok(slf)
	}

	/// Whether to add a space surrounding the properties of a type literal.
	///
	/// 	true (default) - Ex. value: { key: Type }
	/// 	false - Ex. value: {key: Type}
	fn type_literal_space_surrounding_properties<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.type_literal_space_surrounding_properties(value);
		Ok(slf)
	}

	/// Whether to add a space after the while keyword in a while statement.
	///
	/// 	true (default) - Ex. while (true)
	/// 	false - Ex. while(true)
	fn while_statement_space_after_while_keyword<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.while_statement_space_after_while_keyword(value);
		Ok(slf)
	}

	/// Whether to place spaces around enclosed expressions.
	///
	/// 	true - Ex. myFunction( true )
	/// 	false (default) - Ex. myFunction(true)
	fn space_around<'a>(mut slf: PyRefMut<'a, Self>, value: bool) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.space_around(value);
		Ok(slf)
	}

	/// Whether to use parentheses for arrow functions.
	///
	/// Default: UseParentheses::Maintain
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn arrow_function_use_parentheses<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.arrow_function_use_parentheses(
			UseParentheses::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Whether to force a line per expression when spanning multiple lines.
	///
	/// 	true - Formats with each part on a new line.
	/// 	false (default) - Maintains the line breaks as written by the programmer.
	fn binary_expression_line_per_expression<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.binary_expression_line_per_expression(value);
		Ok(slf)
	}

	/// Whether to force a line per expression when spanning multiple lines.
	///
	/// 	true - Formats with each part on a new line.
	/// 	false (default) - Maintains the line breaks as written by the programmer.
	fn conditional_expression_line_per_expression<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.conditional_expression_line_per_expression(value);
		Ok(slf)
	}

	/// Whether to force a line per expression when spanning multiple lines.
	///
	/// 	true - Formats with each part on a new line.
	/// 	false (default) - Maintains the line breaks as written by the programmer.
	fn member_expression_line_per_expression<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.member_expression_line_per_expression(value);
		Ok(slf)
	}

	/// The kind of separator to use in type literals.
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn type_literal_separator_kind<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.type_literal_separator_kind(
			SemiColonOrComma::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// The kind of separator to use in type literals when single line.
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn type_literal_separator_kind_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.type_literal_separator_kind_single_line(
			SemiColonOrComma::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// The kind of separator to use in type literals when multi-line.
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn type_literal_separator_kind_multi_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.type_literal_separator_kind_multi_line(
			SemiColonOrComma::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Alphabetically sorts the import declarations based on their module specifiers.
	///
	/// Default: Case insensitive
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn module_sort_import_declarations<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.module_sort_import_declarations(
			SortOrder::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Alphabetically sorts the export declarations based on their module specifiers.
	///
	/// Default: Case insensitive
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn module_sort_export_declarations<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.module_sort_export_declarations(
			SortOrder::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Alphabetically sorts the import declaration’s named imports.
	///
	/// Default: Case insensitive
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn import_declaration_sort_named_imports<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.import_declaration_sort_named_imports(
			SortOrder::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Sorts type-only named imports first, last, or none (no sorting).
	///
	/// Default: Last
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn import_declaration_sort_type_only_imports<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.import_declaration_sort_type_only_imports(
			NamedTypeImportsExportsOrder::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Alphabetically sorts the export declaration’s named exports.
	///
	/// Default: Case insensitive
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn export_declaration_sort_named_exports<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.export_declaration_sort_named_exports(
			SortOrder::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Sorts type-only named exports first, last, or none (no sorting).
	///
	/// Default: Last
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn export_declaration_sort_type_only_exports<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.export_declaration_sort_type_only_exports(
			NamedTypeImportsExportsOrder::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// The text to use for an ignore comment (ex. // dprint-ignore).
	///
	/// Default: "dprint-ignore"
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn ignore_node_comment_text<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.ignore_node_comment_text(value);
		Ok(slf)
	}

	/// The text to use for a file ignore comment (ex. // dprint-ignore-file).
	///
	/// Default: "dprint-ignore-file"
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn ignore_file_comment_text<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.ignore_file_comment_text(value);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn arrow_function_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.arrow_function_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn class_declaration_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.class_declaration_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn class_expression_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.class_expression_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn constructor_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.constructor_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn do_while_statement_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.do_while_statement_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn enum_declaration_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.enum_declaration_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn for_statement_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_statement_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn for_in_statement_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_in_statement_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn for_of_statement_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_of_statement_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn get_accessor_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.get_accessor_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn if_statement_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.if_statement_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn interface_declaration_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.interface_declaration_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn function_declaration_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.function_declaration_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn function_expression_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.function_expression_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn method_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.method_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn module_declaration_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.module_declaration_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn set_accessor_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.set_accessor_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn static_block_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.static_block_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn switch_case_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.switch_case_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn switch_statement_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.switch_statement_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn try_statement_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.try_statement_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn while_statement_brace_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.while_statement_brace_position(
			BracePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn arguments_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.arguments_prefer_hanging(
			PreferHanging::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn array_expression_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.array_expression_prefer_hanging(
			PreferHanging::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}
	fn array_pattern_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.array_pattern_prefer_hanging(value);
		Ok(slf)
	}
	fn do_while_statement_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.do_while_statement_prefer_hanging(value);
		Ok(slf)
	}
	fn export_declaration_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.export_declaration_prefer_hanging(value);
		Ok(slf)
	}
	fn extends_clause_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.extends_clause_prefer_hanging(value);
		Ok(slf)
	}
	fn for_in_statement_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_in_statement_prefer_hanging(value);
		Ok(slf)
	}
	fn for_of_statement_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_of_statement_prefer_hanging(value);
		Ok(slf)
	}
	fn for_statement_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_statement_prefer_hanging(value);
		Ok(slf)
	}
	fn if_statement_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.if_statement_prefer_hanging(value);
		Ok(slf)
	}
	fn implements_clause_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.implements_clause_prefer_hanging(value);
		Ok(slf)
	}
	fn import_declaration_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.import_declaration_prefer_hanging(value);
		Ok(slf)
	}
	fn jsx_attributes_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.jsx_attributes_prefer_hanging(value);
		Ok(slf)
	}
	fn object_expression_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.object_expression_prefer_hanging(value);
		Ok(slf)
	}
	fn object_pattern_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.object_pattern_prefer_hanging(value);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn parameters_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.parameters_prefer_hanging(
			PreferHanging::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}
	fn sequence_expression_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.sequence_expression_prefer_hanging(value);
		Ok(slf)
	}
	fn switch_statement_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.switch_statement_prefer_hanging(value);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn tuple_type_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.tuple_type_prefer_hanging(
			PreferHanging::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}
	fn type_literal_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.type_literal_prefer_hanging(value);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn type_parameters_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.type_parameters_prefer_hanging(
			PreferHanging::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}
	fn union_and_intersection_type_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.union_and_intersection_type_prefer_hanging(value);
		Ok(slf)
	}
	fn variable_statement_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.variable_statement_prefer_hanging(value);
		Ok(slf)
	}
	fn while_statement_prefer_hanging<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.while_statement_prefer_hanging(value);
		Ok(slf)
	}
	fn export_declaration_force_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.export_declaration_force_single_line(value);
		Ok(slf)
	}
	fn import_declaration_force_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.import_declaration_force_single_line(value);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn export_declaration_force_multi_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.export_declaration_force_multi_line(
			ForceMultiLine::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn import_declaration_force_multi_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.import_declaration_force_multi_line(
			ForceMultiLine::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn enum_declaration_member_spacing<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.enum_declaration_member_spacing(
			MemberSpacing::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn if_statement_next_control_flow_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.if_statement_next_control_flow_position(
			NextControlFlowPosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn try_statement_next_control_flow_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.try_statement_next_control_flow_position(
			NextControlFlowPosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn do_while_statement_next_control_flow_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.do_while_statement_next_control_flow_position(
			NextControlFlowPosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn binary_expression_operator_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.binary_expression_operator_position(
			OperatorPosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn conditional_expression_operator_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.conditional_expression_operator_position(
			OperatorPosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn conditional_type_operator_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.conditional_type_operator_position(
			OperatorPosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn if_statement_single_body_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.if_statement_single_body_position(
			SameOrNextLinePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn for_statement_single_body_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_statement_single_body_position(
			SameOrNextLinePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn for_in_statement_single_body_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_in_statement_single_body_position(
			SameOrNextLinePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn for_of_statement_single_body_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_of_statement_single_body_position(
			SameOrNextLinePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn while_statement_single_body_position<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.while_statement_single_body_position(
			SameOrNextLinePosition::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn arguments_trailing_commas<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.arguments_trailing_commas(
			TrailingCommas::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn parameters_trailing_commas<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.parameters_trailing_commas(
			TrailingCommas::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn array_expression_trailing_commas<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.array_expression_trailing_commas(
			TrailingCommas::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn array_pattern_trailing_commas<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.array_pattern_trailing_commas(
			TrailingCommas::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn enum_declaration_trailing_commas<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.enum_declaration_trailing_commas(
			TrailingCommas::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn export_declaration_trailing_commas<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.export_declaration_trailing_commas(
			TrailingCommas::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn import_declaration_trailing_commas<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.import_declaration_trailing_commas(
			TrailingCommas::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn object_expression_trailing_commas<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.object_expression_trailing_commas(
			TrailingCommas::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn object_pattern_trailing_commas<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.object_pattern_trailing_commas(
			TrailingCommas::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn tuple_type_trailing_commas<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.tuple_type_trailing_commas(
			TrailingCommas::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn type_literal_trailing_commas<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.type_literal_trailing_commas(
			TrailingCommas::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	/// Only applies when using commas on type literals.
	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn type_parameters_trailing_commas<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.type_parameters_trailing_commas(
			TrailingCommas::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn if_statement_use_braces<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.if_statement_use_braces(
			UseBraces::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn for_statement_use_braces<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_statement_use_braces(
			UseBraces::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn for_in_statement_use_braces<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_in_statement_use_braces(
			UseBraces::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn for_of_statement_use_braces<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_of_statement_use_braces(
			UseBraces::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}

	#[pyo3(signature = (value: "str") -> "ConfigurationBuilder")]
	fn while_statement_use_braces<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: &str,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.while_statement_use_braces(
			UseBraces::from_str(value)
				.unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value)),
		);
		Ok(slf)
	}
	fn array_expression_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.array_expression_prefer_single_line(value);
		Ok(slf)
	}
	fn array_pattern_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.array_pattern_prefer_single_line(value);
		Ok(slf)
	}
	fn arguments_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.arguments_prefer_single_line(value);
		Ok(slf)
	}
	fn binary_expression_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.binary_expression_prefer_single_line(value);
		Ok(slf)
	}
	fn computed_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.computed_prefer_single_line(value);
		Ok(slf)
	}
	fn conditional_expression_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.conditional_expression_prefer_single_line(value);
		Ok(slf)
	}
	fn conditional_type_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.conditional_type_prefer_single_line(value);
		Ok(slf)
	}
	fn decorators_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.decorators_prefer_single_line(value);
		Ok(slf)
	}
	fn export_declaration_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.export_declaration_prefer_single_line(value);
		Ok(slf)
	}
	fn for_statement_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_statement_prefer_single_line(value);
		Ok(slf)
	}
	fn import_declaration_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.import_declaration_prefer_single_line(value);
		Ok(slf)
	}
	fn jsx_attributes_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.jsx_attributes_prefer_single_line(value);
		Ok(slf)
	}
	fn jsx_element_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.jsx_element_prefer_single_line(value);
		Ok(slf)
	}
	fn mapped_type_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.mapped_type_prefer_single_line(value);
		Ok(slf)
	}
	fn member_expression_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.member_expression_prefer_single_line(value);
		Ok(slf)
	}
	fn object_expression_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.object_expression_prefer_single_line(value);
		Ok(slf)
	}
	fn object_pattern_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.object_pattern_prefer_single_line(value);
		Ok(slf)
	}
	fn parameters_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.parameters_prefer_single_line(value);
		Ok(slf)
	}
	fn parentheses_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.parentheses_prefer_single_line(value);
		Ok(slf)
	}
	fn tuple_type_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.tuple_type_prefer_single_line(value);
		Ok(slf)
	}
	fn type_literal_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.type_literal_prefer_single_line(value);
		Ok(slf)
	}
	fn type_parameters_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.type_parameters_prefer_single_line(value);
		Ok(slf)
	}
	fn union_and_intersection_type_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.union_and_intersection_type_prefer_single_line(value);
		Ok(slf)
	}
	fn variable_statement_prefer_single_line<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.variable_statement_prefer_single_line(value);
		Ok(slf)
	}
	fn arguments_space_around<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.arguments_space_around(value);
		Ok(slf)
	}
	fn array_expression_space_around<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.array_expression_space_around(value);
		Ok(slf)
	}
	fn array_pattern_space_around<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.array_pattern_space_around(value);
		Ok(slf)
	}
	fn catch_clause_space_around<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.catch_clause_space_around(value);
		Ok(slf)
	}
	fn do_while_statement_space_around<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.do_while_statement_space_around(value);
		Ok(slf)
	}
	fn for_in_statement_space_around<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_in_statement_space_around(value);
		Ok(slf)
	}
	fn for_of_statement_space_around<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_of_statement_space_around(value);
		Ok(slf)
	}
	fn for_statement_space_around<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.for_statement_space_around(value);
		Ok(slf)
	}
	fn if_statement_space_around<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.if_statement_space_around(value);
		Ok(slf)
	}
	fn parameters_space_around<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.parameters_space_around(value);
		Ok(slf)
	}
	fn paren_expression_space_around<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.paren_expression_space_around(value);
		Ok(slf)
	}
	fn switch_statement_space_around<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.switch_statement_space_around(value);
		Ok(slf)
	}
	fn tuple_type_space_around<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.tuple_type_space_around(value);
		Ok(slf)
	}

	fn while_statement_space_around<'a>(
		mut slf: PyRefMut<'a, Self>,
		value: bool,
	) -> PyResult<PyRefMut<'a, Self>> {
		slf.0.while_statement_space_around(value);
		Ok(slf)
	}
}
