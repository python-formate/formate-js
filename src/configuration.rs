use dprint_core::configuration::NewLineKind;
use dprint_core::configuration::RECOMMENDED_GLOBAL_CONFIGURATION;
use dprint_plugin_typescript::configuration::Configuration;
use dprint_plugin_typescript::configuration::{
	BracePosition, ForceMultiLine, JsxMultiLineParens, JsxQuoteStyle, MemberSpacing,
	NamedTypeImportsExportsOrder, NextControlFlowPosition, OperatorPosition, PreferHanging,
	QuoteProps, QuoteStyle, SameOrNextLinePosition, SemiColonOrComma, SemiColons, SortOrder,
	TrailingCommas, UseBraces, UseParentheses,
};
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyIterator, PyList};
use std::str::FromStr;

#[pyclass(name = "Configuration", module = "formate_js", mapping)]
// #[repr(transparent)]
#[derive(Clone)]
/// Formatting configuration knobs.
pub struct PyConfiguration {
	#[pyo3(get, set)]
	pub indent_width: u8,
	#[pyo3(get, set)]
	pub line_width: u32,
	#[pyo3(get, set)]
	pub use_tabs: bool,
	#[pyo3(get, set)]
	pub new_line_kind: String,
	#[pyo3(get, set)]
	pub quote_style: String,
	#[pyo3(get, set)]
	pub quote_props: String,
	#[pyo3(get, set)]
	pub semi_colons: String,
	#[pyo3(get, set)]
	pub file_indent_level: u32,
	/* situational */
	#[pyo3(get, set)]
	pub arrow_function_use_parentheses: String,
	#[pyo3(get, set)]
	pub binary_expression_line_per_expression: bool,
	#[pyo3(get, set)]
	pub conditional_expression_line_per_expression: bool,
	#[pyo3(get, set)]
	pub jsx_quote_style: String,
	#[pyo3(get, set)]
	pub jsx_multi_line_parens: String,
	#[pyo3(get, set)]
	pub jsx_force_new_lines_surrounding_content: bool,
	#[pyo3(get, set)]
	pub jsx_opening_element_bracket_position: String,
	#[pyo3(get, set)]
	pub jsx_self_closing_element_bracket_position: String,
	#[pyo3(get, set)]
	pub member_expression_line_per_expression: bool,
	#[pyo3(get, set)]
	pub type_literal_separator_kind_single_line: String,
	#[pyo3(get, set)]
	pub type_literal_separator_kind_multi_line: String,
	/* sorting */
	#[pyo3(get, set)]
	pub module_sort_import_declarations: String,
	#[pyo3(get, set)]
	pub module_sort_export_declarations: String,
	#[pyo3(get, set)]
	pub import_declaration_sort_named_imports: String,
	#[pyo3(get, set)]
	pub import_declaration_sort_type_only_imports: String,
	#[pyo3(get, set)]
	pub export_declaration_sort_named_exports: String,
	#[pyo3(get, set)]
	pub export_declaration_sort_type_only_exports: String,
	/* ignore comments */
	#[pyo3(get, set)]
	pub ignore_node_comment_text: String,
	#[pyo3(get, set)]
	pub ignore_file_comment_text: String,
	/* brace position */
	#[pyo3(get, set)]
	pub arrow_function_brace_position: String,
	#[pyo3(get, set)]
	pub class_declaration_brace_position: String,
	#[pyo3(get, set)]
	pub class_expression_brace_position: String,
	#[pyo3(get, set)]
	pub constructor_brace_position: String,
	#[pyo3(get, set)]
	pub do_while_statement_brace_position: String,
	#[pyo3(get, set)]
	pub enum_declaration_brace_position: String,
	#[pyo3(get, set)]
	pub get_accessor_brace_position: String,
	#[pyo3(get, set)]
	pub if_statement_brace_position: String,
	#[pyo3(get, set)]
	pub interface_declaration_brace_position: String,
	#[pyo3(get, set)]
	pub for_statement_brace_position: String,
	#[pyo3(get, set)]
	pub for_in_statement_brace_position: String,
	#[pyo3(get, set)]
	pub for_of_statement_brace_position: String,
	#[pyo3(get, set)]
	pub function_declaration_brace_position: String,
	#[pyo3(get, set)]
	pub function_expression_brace_position: String,
	#[pyo3(get, set)]
	pub method_brace_position: String,
	#[pyo3(get, set)]
	pub module_declaration_brace_position: String,
	#[pyo3(get, set)]
	pub set_accessor_brace_position: String,
	#[pyo3(get, set)]
	pub static_block_brace_position: String,
	#[pyo3(get, set)]
	pub switch_case_brace_position: String,
	#[pyo3(get, set)]
	pub switch_statement_brace_position: String,
	#[pyo3(get, set)]
	pub try_statement_brace_position: String,
	#[pyo3(get, set)]
	pub while_statement_brace_position: String,
	/* prefer hanging */
	#[pyo3(get, set)]
	pub arguments_prefer_hanging: String,
	#[pyo3(get, set)]
	pub array_expression_prefer_hanging: String,
	#[pyo3(get, set)]
	pub array_pattern_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub do_while_statement_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub export_declaration_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub extends_clause_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub for_statement_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub for_in_statement_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub for_of_statement_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub if_statement_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub implements_clause_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub import_declaration_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub jsx_attributes_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub object_expression_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub object_pattern_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub parameters_prefer_hanging: String,
	#[pyo3(get, set)]
	pub sequence_expression_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub switch_statement_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub tuple_type_prefer_hanging: String,
	#[pyo3(get, set)]
	pub type_literal_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub type_parameters_prefer_hanging: String,
	#[pyo3(get, set)]
	pub union_and_intersection_type_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub variable_statement_prefer_hanging: bool,
	#[pyo3(get, set)]
	pub while_statement_prefer_hanging: bool,
	/* member spacing */
	#[pyo3(get, set)]
	pub enum_declaration_member_spacing: String,
	/* next control flow position */
	#[pyo3(get, set)]
	pub if_statement_next_control_flow_position: String,
	#[pyo3(get, set)]
	pub try_statement_next_control_flow_position: String,
	#[pyo3(get, set)]
	pub do_while_statement_next_control_flow_position: String,
	/* operator position */
	#[pyo3(get, set)]
	pub binary_expression_operator_position: String,
	#[pyo3(get, set)]
	pub conditional_expression_operator_position: String,
	#[pyo3(get, set)]
	pub conditional_type_operator_position: String,
	/* single body position */
	#[pyo3(get, set)]
	pub if_statement_single_body_position: String,
	#[pyo3(get, set)]
	pub for_statement_single_body_position: String,
	#[pyo3(get, set)]
	pub for_in_statement_single_body_position: String,
	#[pyo3(get, set)]
	pub for_of_statement_single_body_position: String,
	#[pyo3(get, set)]
	pub while_statement_single_body_position: String,
	/* trailing commas */
	#[pyo3(get, set)]
	pub arguments_trailing_commas: String,
	#[pyo3(get, set)]
	pub parameters_trailing_commas: String,
	#[pyo3(get, set)]
	pub array_expression_trailing_commas: String,
	#[pyo3(get, set)]
	pub array_pattern_trailing_commas: String,
	#[pyo3(get, set)]
	pub enum_declaration_trailing_commas: String,
	#[pyo3(get, set)]
	pub export_declaration_trailing_commas: String,
	#[pyo3(get, set)]
	pub import_declaration_trailing_commas: String,
	#[pyo3(get, set)]
	pub object_pattern_trailing_commas: String,
	#[pyo3(get, set)]
	pub object_expression_trailing_commas: String,
	#[pyo3(get, set)]
	pub tuple_type_trailing_commas: String,
	#[pyo3(get, set)]
	pub type_literal_trailing_commas: String,
	#[pyo3(get, set)]
	pub type_parameters_trailing_commas: String,
	/* use braces */
	#[pyo3(get, set)]
	pub if_statement_use_braces: String,
	#[pyo3(get, set)]
	pub for_statement_use_braces: String,
	#[pyo3(get, set)]
	pub for_of_statement_use_braces: String,
	#[pyo3(get, set)]
	pub for_in_statement_use_braces: String,
	#[pyo3(get, set)]
	pub while_statement_use_braces: String,
	/* prefer single line */
	#[pyo3(get, set)]
	pub array_expression_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub array_pattern_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub arguments_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub binary_expression_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub computed_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub conditional_expression_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub conditional_type_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub decorators_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub export_declaration_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub for_statement_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub import_declaration_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub jsx_attributes_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub jsx_element_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub mapped_type_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub member_expression_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub object_expression_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub object_pattern_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub parameters_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub parentheses_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub tuple_type_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub type_literal_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub type_parameters_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub union_and_intersection_type_prefer_single_line: bool,
	#[pyo3(get, set)]
	pub variable_statement_prefer_single_line: bool,
	/* force single line */
	#[pyo3(get, set)]
	pub import_declaration_force_single_line: bool,
	#[pyo3(get, set)]
	pub export_declaration_force_single_line: bool,
	/* force multi line specifiers */
	#[pyo3(get, set)]
	pub export_declaration_force_multi_line: String,
	#[pyo3(get, set)]
	pub import_declaration_force_multi_line: String,

	/* use space separator */
	#[pyo3(get, set)]
	pub binary_expression_space_surrounding_bitwise_and_arithmetic_operator: bool,
	#[pyo3(get, set)]
	pub comment_line_force_space_after_slashes: bool,
	#[pyo3(get, set)]
	pub construct_signature_space_after_new_keyword: bool,
	#[pyo3(get, set)]
	pub constructor_space_before_parentheses: bool,
	#[pyo3(get, set)]
	pub constructor_type_space_after_new_keyword: bool,
	#[pyo3(get, set)]
	pub do_while_statement_space_after_while_keyword: bool,
	#[pyo3(get, set)]
	pub export_declaration_space_surrounding_named_exports: bool,
	#[pyo3(get, set)]
	pub for_statement_space_after_for_keyword: bool,
	#[pyo3(get, set)]
	pub for_statement_space_after_semi_colons: bool,
	#[pyo3(get, set)]
	pub for_in_statement_space_after_for_keyword: bool,
	#[pyo3(get, set)]
	pub for_of_statement_space_after_for_keyword: bool,
	#[pyo3(get, set)]
	pub function_declaration_space_before_parentheses: bool,
	#[pyo3(get, set)]
	pub function_expression_space_before_parentheses: bool,
	#[pyo3(get, set)]
	pub function_expression_space_after_function_keyword: bool,
	#[pyo3(get, set)]
	pub get_accessor_space_before_parentheses: bool,
	#[pyo3(get, set)]
	pub if_statement_space_after_if_keyword: bool,
	#[pyo3(get, set)]
	pub import_declaration_space_surrounding_named_imports: bool,
	#[pyo3(get, set)]
	pub jsx_expression_container_space_surrounding_expression: bool,
	#[pyo3(get, set)]
	pub jsx_self_closing_element_space_before_slash: bool,
	#[pyo3(get, set)]
	pub method_space_before_parentheses: bool,
	#[pyo3(get, set)]
	pub object_expression_space_surrounding_properties: bool,
	#[pyo3(get, set)]
	pub object_pattern_space_surrounding_properties: bool,
	#[pyo3(get, set)]
	pub set_accessor_space_before_parentheses: bool,
	#[pyo3(get, set)]
	pub space_surrounding_properties: bool,
	#[pyo3(get, set)]
	pub tagged_template_space_before_literal: bool,
	#[pyo3(get, set)]
	pub type_annotation_space_before_colon: bool,
	#[pyo3(get, set)]
	pub type_assertion_space_before_expression: bool,
	#[pyo3(get, set)]
	pub type_literal_space_surrounding_properties: bool,
	#[pyo3(get, set)]
	pub while_statement_space_after_while_keyword: bool,
	#[pyo3(get, set)]
	pub arguments_space_around: bool,
	#[pyo3(get, set)]
	pub array_expression_space_around: bool,
	#[pyo3(get, set)]
	pub array_pattern_space_around: bool,
	#[pyo3(get, set)]
	pub catch_clause_space_around: bool,
	#[pyo3(get, set)]
	pub do_while_statement_space_around: bool,
	#[pyo3(get, set)]
	pub for_in_statement_space_around: bool,
	#[pyo3(get, set)]
	pub for_of_statement_space_around: bool,
	#[pyo3(get, set)]
	pub for_statement_space_around: bool,
	#[pyo3(get, set)]
	pub if_statement_space_around: bool,
	#[pyo3(get, set)]
	pub parameters_space_around: bool,
	#[pyo3(get, set)]
	pub paren_expression_space_around: bool,
	#[pyo3(get, set)]
	pub switch_statement_space_around: bool,
	#[pyo3(get, set)]
	pub tuple_type_space_around: bool,
	#[pyo3(get, set)]
	pub while_statement_space_around: bool,
}

fn lookup_enum<T: FromStr>(value: &str) -> T {
	T::from_str(value).unwrap_or_else(|_| panic!("Invalid enum value '{}'", &value))
}

impl From<PyConfiguration> for Configuration {
	fn from(value: PyConfiguration) -> Self {
		Configuration {
			indent_width: value.indent_width,
			line_width: value.line_width,
			use_tabs: value.use_tabs,
			new_line_kind: lookup_enum::<NewLineKind>(&value.new_line_kind),
			quote_style: lookup_enum::<QuoteStyle>(&value.quote_style),
			quote_props: lookup_enum::<QuoteProps>(&value.quote_props),
			semi_colons: lookup_enum::<SemiColons>(&value.semi_colons),
			file_indent_level: value.file_indent_level,
			arrow_function_use_parentheses: lookup_enum::<UseParentheses>(
				&value.arrow_function_use_parentheses,
			),
			binary_expression_line_per_expression: value.binary_expression_line_per_expression,
			conditional_expression_line_per_expression: value
				.conditional_expression_line_per_expression,
			jsx_quote_style: lookup_enum::<JsxQuoteStyle>(&value.jsx_quote_style),
			jsx_multi_line_parens: lookup_enum::<JsxMultiLineParens>(&value.jsx_multi_line_parens),
			jsx_force_new_lines_surrounding_content: value.jsx_force_new_lines_surrounding_content,
			jsx_opening_element_bracket_position: lookup_enum::<SameOrNextLinePosition>(
				&value.jsx_opening_element_bracket_position,
			),
			jsx_self_closing_element_bracket_position: lookup_enum::<SameOrNextLinePosition>(
				&value.jsx_self_closing_element_bracket_position,
			),
			member_expression_line_per_expression: value.member_expression_line_per_expression,
			type_literal_separator_kind_single_line: lookup_enum::<SemiColonOrComma>(
				&value.type_literal_separator_kind_single_line,
			),
			type_literal_separator_kind_multi_line: lookup_enum::<SemiColonOrComma>(
				&value.type_literal_separator_kind_multi_line,
			),
			module_sort_import_declarations: lookup_enum::<SortOrder>(
				&value.module_sort_import_declarations,
			),
			module_sort_export_declarations: lookup_enum::<SortOrder>(
				&value.module_sort_export_declarations,
			),
			import_declaration_sort_named_imports: lookup_enum::<SortOrder>(
				&value.import_declaration_sort_named_imports,
			),
			import_declaration_sort_type_only_imports: lookup_enum::<NamedTypeImportsExportsOrder>(
				&value.import_declaration_sort_type_only_imports,
			),
			export_declaration_sort_named_exports: lookup_enum::<SortOrder>(
				&value.export_declaration_sort_named_exports,
			),
			export_declaration_sort_type_only_exports: lookup_enum::<NamedTypeImportsExportsOrder>(
				&value.export_declaration_sort_type_only_exports,
			),
			ignore_node_comment_text: value.ignore_node_comment_text,
			ignore_file_comment_text: value.ignore_file_comment_text,
			arrow_function_brace_position: lookup_enum::<BracePosition>(
				&value.arrow_function_brace_position,
			),
			class_declaration_brace_position: lookup_enum::<BracePosition>(
				&value.class_declaration_brace_position,
			),
			class_expression_brace_position: lookup_enum::<BracePosition>(
				&value.class_expression_brace_position,
			),
			constructor_brace_position: lookup_enum::<BracePosition>(
				&value.constructor_brace_position,
			),
			do_while_statement_brace_position: lookup_enum::<BracePosition>(
				&value.do_while_statement_brace_position,
			),
			enum_declaration_brace_position: lookup_enum::<BracePosition>(
				&value.enum_declaration_brace_position,
			),
			get_accessor_brace_position: lookup_enum::<BracePosition>(
				&value.get_accessor_brace_position,
			),
			if_statement_brace_position: lookup_enum::<BracePosition>(
				&value.if_statement_brace_position,
			),
			interface_declaration_brace_position: lookup_enum::<BracePosition>(
				&value.interface_declaration_brace_position,
			),
			for_statement_brace_position: lookup_enum::<BracePosition>(
				&value.for_statement_brace_position,
			),
			for_in_statement_brace_position: lookup_enum::<BracePosition>(
				&value.for_in_statement_brace_position,
			),
			for_of_statement_brace_position: lookup_enum::<BracePosition>(
				&value.for_of_statement_brace_position,
			),
			function_declaration_brace_position: lookup_enum::<BracePosition>(
				&value.function_declaration_brace_position,
			),
			function_expression_brace_position: lookup_enum::<BracePosition>(
				&value.function_expression_brace_position,
			),
			method_brace_position: lookup_enum::<BracePosition>(&value.method_brace_position),
			module_declaration_brace_position: lookup_enum::<BracePosition>(
				&value.module_declaration_brace_position,
			),
			set_accessor_brace_position: lookup_enum::<BracePosition>(
				&value.set_accessor_brace_position,
			),
			static_block_brace_position: lookup_enum::<BracePosition>(
				&value.static_block_brace_position,
			),
			switch_case_brace_position: lookup_enum::<BracePosition>(
				&value.switch_case_brace_position,
			),
			switch_statement_brace_position: lookup_enum::<BracePosition>(
				&value.switch_statement_brace_position,
			),
			try_statement_brace_position: lookup_enum::<BracePosition>(
				&value.try_statement_brace_position,
			),
			while_statement_brace_position: lookup_enum::<BracePosition>(
				&value.while_statement_brace_position,
			),
			arguments_prefer_hanging: lookup_enum::<PreferHanging>(&value.arguments_prefer_hanging),
			array_expression_prefer_hanging: lookup_enum::<PreferHanging>(
				&value.array_expression_prefer_hanging,
			),
			array_pattern_prefer_hanging: value.array_pattern_prefer_hanging,
			do_while_statement_prefer_hanging: value.do_while_statement_prefer_hanging,
			export_declaration_prefer_hanging: value.export_declaration_prefer_hanging,
			extends_clause_prefer_hanging: value.extends_clause_prefer_hanging,
			for_statement_prefer_hanging: value.for_statement_prefer_hanging,
			for_in_statement_prefer_hanging: value.for_in_statement_prefer_hanging,
			for_of_statement_prefer_hanging: value.for_of_statement_prefer_hanging,
			if_statement_prefer_hanging: value.if_statement_prefer_hanging,
			implements_clause_prefer_hanging: value.implements_clause_prefer_hanging,
			import_declaration_prefer_hanging: value.import_declaration_prefer_hanging,
			jsx_attributes_prefer_hanging: value.jsx_attributes_prefer_hanging,
			object_expression_prefer_hanging: value.object_expression_prefer_hanging,
			object_pattern_prefer_hanging: value.object_pattern_prefer_hanging,
			parameters_prefer_hanging: lookup_enum::<PreferHanging>(
				&value.parameters_prefer_hanging,
			),
			sequence_expression_prefer_hanging: value.sequence_expression_prefer_hanging,
			switch_statement_prefer_hanging: value.switch_statement_prefer_hanging,
			tuple_type_prefer_hanging: lookup_enum::<PreferHanging>(
				&value.tuple_type_prefer_hanging,
			),
			type_literal_prefer_hanging: value.type_literal_prefer_hanging,
			type_parameters_prefer_hanging: lookup_enum::<PreferHanging>(
				&value.type_parameters_prefer_hanging,
			),
			union_and_intersection_type_prefer_hanging: value
				.union_and_intersection_type_prefer_hanging,
			variable_statement_prefer_hanging: value.variable_statement_prefer_hanging,
			while_statement_prefer_hanging: value.while_statement_prefer_hanging,
			enum_declaration_member_spacing: lookup_enum::<MemberSpacing>(
				&value.enum_declaration_member_spacing,
			),
			if_statement_next_control_flow_position: lookup_enum::<NextControlFlowPosition>(
				&value.if_statement_next_control_flow_position,
			),
			try_statement_next_control_flow_position: lookup_enum::<NextControlFlowPosition>(
				&value.try_statement_next_control_flow_position,
			),
			do_while_statement_next_control_flow_position: lookup_enum::<NextControlFlowPosition>(
				&value.do_while_statement_next_control_flow_position,
			),
			binary_expression_operator_position: lookup_enum::<OperatorPosition>(
				&value.binary_expression_operator_position,
			),
			conditional_expression_operator_position: lookup_enum::<OperatorPosition>(
				&value.conditional_expression_operator_position,
			),
			conditional_type_operator_position: lookup_enum::<OperatorPosition>(
				&value.conditional_type_operator_position,
			),
			if_statement_single_body_position: lookup_enum::<SameOrNextLinePosition>(
				&value.if_statement_single_body_position,
			),
			for_statement_single_body_position: lookup_enum::<SameOrNextLinePosition>(
				&value.for_statement_single_body_position,
			),
			for_in_statement_single_body_position: lookup_enum::<SameOrNextLinePosition>(
				&value.for_in_statement_single_body_position,
			),
			for_of_statement_single_body_position: lookup_enum::<SameOrNextLinePosition>(
				&value.for_of_statement_single_body_position,
			),
			while_statement_single_body_position: lookup_enum::<SameOrNextLinePosition>(
				&value.while_statement_single_body_position,
			),
			arguments_trailing_commas: lookup_enum::<TrailingCommas>(
				&value.arguments_trailing_commas,
			),
			parameters_trailing_commas: lookup_enum::<TrailingCommas>(
				&value.parameters_trailing_commas,
			),
			array_expression_trailing_commas: lookup_enum::<TrailingCommas>(
				&value.array_expression_trailing_commas,
			),
			array_pattern_trailing_commas: lookup_enum::<TrailingCommas>(
				&value.array_pattern_trailing_commas,
			),
			enum_declaration_trailing_commas: lookup_enum::<TrailingCommas>(
				&value.enum_declaration_trailing_commas,
			),
			export_declaration_trailing_commas: lookup_enum::<TrailingCommas>(
				&value.export_declaration_trailing_commas,
			),
			import_declaration_trailing_commas: lookup_enum::<TrailingCommas>(
				&value.import_declaration_trailing_commas,
			),
			object_pattern_trailing_commas: lookup_enum::<TrailingCommas>(
				&value.object_pattern_trailing_commas,
			),
			object_expression_trailing_commas: lookup_enum::<TrailingCommas>(
				&value.object_expression_trailing_commas,
			),
			tuple_type_trailing_commas: lookup_enum::<TrailingCommas>(
				&value.tuple_type_trailing_commas,
			),
			type_literal_trailing_commas: lookup_enum::<TrailingCommas>(
				&value.type_literal_trailing_commas,
			),
			type_parameters_trailing_commas: lookup_enum::<TrailingCommas>(
				&value.type_parameters_trailing_commas,
			),
			if_statement_use_braces: lookup_enum::<UseBraces>(&value.if_statement_use_braces),
			for_statement_use_braces: lookup_enum::<UseBraces>(&value.for_statement_use_braces),
			for_of_statement_use_braces: lookup_enum::<UseBraces>(
				&value.for_of_statement_use_braces,
			),
			for_in_statement_use_braces: lookup_enum::<UseBraces>(
				&value.for_in_statement_use_braces,
			),
			while_statement_use_braces: lookup_enum::<UseBraces>(&value.while_statement_use_braces),
			array_expression_prefer_single_line: value.array_expression_prefer_single_line,
			array_pattern_prefer_single_line: value.array_pattern_prefer_single_line,
			arguments_prefer_single_line: value.arguments_prefer_single_line,
			binary_expression_prefer_single_line: value.binary_expression_prefer_single_line,
			computed_prefer_single_line: value.computed_prefer_single_line,
			conditional_expression_prefer_single_line: value
				.conditional_expression_prefer_single_line,
			conditional_type_prefer_single_line: value.conditional_type_prefer_single_line,
			decorators_prefer_single_line: value.decorators_prefer_single_line,
			export_declaration_prefer_single_line: value.export_declaration_prefer_single_line,
			for_statement_prefer_single_line: value.for_statement_prefer_single_line,
			import_declaration_prefer_single_line: value.import_declaration_prefer_single_line,
			jsx_attributes_prefer_single_line: value.jsx_attributes_prefer_single_line,
			jsx_element_prefer_single_line: value.jsx_element_prefer_single_line,
			mapped_type_prefer_single_line: value.mapped_type_prefer_single_line,
			member_expression_prefer_single_line: value.member_expression_prefer_single_line,
			object_expression_prefer_single_line: value.object_expression_prefer_single_line,
			object_pattern_prefer_single_line: value.object_pattern_prefer_single_line,
			parameters_prefer_single_line: value.parameters_prefer_single_line,
			parentheses_prefer_single_line: value.parentheses_prefer_single_line,
			tuple_type_prefer_single_line: value.tuple_type_prefer_single_line,
			type_literal_prefer_single_line: value.type_literal_prefer_single_line,
			type_parameters_prefer_single_line: value.type_parameters_prefer_single_line,
			union_and_intersection_type_prefer_single_line: value
				.union_and_intersection_type_prefer_single_line,
			variable_statement_prefer_single_line: value.variable_statement_prefer_single_line,
			import_declaration_force_single_line: value.import_declaration_force_single_line,
			export_declaration_force_single_line: value.export_declaration_force_single_line,
			export_declaration_force_multi_line: lookup_enum::<ForceMultiLine>(
				&value.export_declaration_force_multi_line,
			),
			import_declaration_force_multi_line: lookup_enum::<ForceMultiLine>(
				&value.import_declaration_force_multi_line,
			),
			binary_expression_space_surrounding_bitwise_and_arithmetic_operator: value
				.binary_expression_space_surrounding_bitwise_and_arithmetic_operator,
			comment_line_force_space_after_slashes: value.comment_line_force_space_after_slashes,
			construct_signature_space_after_new_keyword: value
				.construct_signature_space_after_new_keyword,
			constructor_space_before_parentheses: value.constructor_space_before_parentheses,
			constructor_type_space_after_new_keyword: value
				.constructor_type_space_after_new_keyword,
			do_while_statement_space_after_while_keyword: value
				.do_while_statement_space_after_while_keyword,
			export_declaration_space_surrounding_named_exports: value
				.export_declaration_space_surrounding_named_exports,
			for_statement_space_after_for_keyword: value.for_statement_space_after_for_keyword,
			for_statement_space_after_semi_colons: value.for_statement_space_after_semi_colons,
			for_in_statement_space_after_for_keyword: value
				.for_in_statement_space_after_for_keyword,
			for_of_statement_space_after_for_keyword: value
				.for_of_statement_space_after_for_keyword,
			function_declaration_space_before_parentheses: value
				.function_declaration_space_before_parentheses,
			function_expression_space_before_parentheses: value
				.function_expression_space_before_parentheses,
			function_expression_space_after_function_keyword: value
				.function_expression_space_after_function_keyword,
			get_accessor_space_before_parentheses: value.get_accessor_space_before_parentheses,
			if_statement_space_after_if_keyword: value.if_statement_space_after_if_keyword,
			import_declaration_space_surrounding_named_imports: value
				.import_declaration_space_surrounding_named_imports,
			jsx_expression_container_space_surrounding_expression: value
				.jsx_expression_container_space_surrounding_expression,
			jsx_self_closing_element_space_before_slash: value
				.jsx_self_closing_element_space_before_slash,
			method_space_before_parentheses: value.method_space_before_parentheses,
			object_expression_space_surrounding_properties: value
				.object_expression_space_surrounding_properties,
			object_pattern_space_surrounding_properties: value
				.object_pattern_space_surrounding_properties,
			set_accessor_space_before_parentheses: value.set_accessor_space_before_parentheses,
			space_surrounding_properties: value.space_surrounding_properties,
			tagged_template_space_before_literal: value.tagged_template_space_before_literal,
			type_annotation_space_before_colon: value.type_annotation_space_before_colon,
			type_assertion_space_before_expression: value.type_assertion_space_before_expression,
			type_literal_space_surrounding_properties: value
				.type_literal_space_surrounding_properties,
			while_statement_space_after_while_keyword: value
				.while_statement_space_after_while_keyword,
			arguments_space_around: value.arguments_space_around,
			array_expression_space_around: value.array_expression_space_around,
			array_pattern_space_around: value.array_pattern_space_around,
			catch_clause_space_around: value.catch_clause_space_around,
			do_while_statement_space_around: value.do_while_statement_space_around,
			for_in_statement_space_around: value.for_in_statement_space_around,
			for_of_statement_space_around: value.for_of_statement_space_around,
			for_statement_space_around: value.for_statement_space_around,
			if_statement_space_around: value.if_statement_space_around,
			parameters_space_around: value.parameters_space_around,
			paren_expression_space_around: value.paren_expression_space_around,
			switch_statement_space_around: value.switch_statement_space_around,
			tuple_type_space_around: value.tuple_type_space_around,
			while_statement_space_around: value.while_statement_space_around,
		}
	}
}
impl From<Configuration> for PyConfiguration {
	fn from(value: Configuration) -> Self {
		PyConfiguration {
			indent_width: value.indent_width,
			line_width: value.line_width,
			use_tabs: value.use_tabs,
			new_line_kind: value.new_line_kind.to_string(),
			quote_style: value.quote_style.to_string(),
			quote_props: value.quote_props.to_string(),
			semi_colons: value.semi_colons.to_string(),
			file_indent_level: value.file_indent_level,
			arrow_function_use_parentheses: value.arrow_function_use_parentheses.to_string(),
			binary_expression_line_per_expression: value.binary_expression_line_per_expression,
			conditional_expression_line_per_expression: value
				.conditional_expression_line_per_expression,
			jsx_quote_style: value.jsx_quote_style.to_string(),
			jsx_multi_line_parens: value.jsx_multi_line_parens.to_string(),
			jsx_force_new_lines_surrounding_content: value.jsx_force_new_lines_surrounding_content,
			jsx_opening_element_bracket_position: value
				.jsx_opening_element_bracket_position
				.to_string(),
			jsx_self_closing_element_bracket_position: value
				.jsx_self_closing_element_bracket_position
				.to_string(),
			member_expression_line_per_expression: value.member_expression_line_per_expression,
			type_literal_separator_kind_single_line: value
				.type_literal_separator_kind_single_line
				.to_string(),
			type_literal_separator_kind_multi_line: value
				.type_literal_separator_kind_multi_line
				.to_string(),
			module_sort_import_declarations: value.module_sort_import_declarations.to_string(),
			module_sort_export_declarations: value.module_sort_export_declarations.to_string(),
			import_declaration_sort_named_imports: value
				.import_declaration_sort_named_imports
				.to_string(),
			import_declaration_sort_type_only_imports: value
				.import_declaration_sort_type_only_imports
				.to_string(),
			export_declaration_sort_named_exports: value
				.export_declaration_sort_named_exports
				.to_string(),
			export_declaration_sort_type_only_exports: value
				.export_declaration_sort_type_only_exports
				.to_string(),
			ignore_node_comment_text: value.ignore_node_comment_text,
			ignore_file_comment_text: value.ignore_file_comment_text,
			arrow_function_brace_position: value.arrow_function_brace_position.to_string(),
			class_declaration_brace_position: value.class_declaration_brace_position.to_string(),
			class_expression_brace_position: value.class_expression_brace_position.to_string(),
			constructor_brace_position: value.constructor_brace_position.to_string(),
			do_while_statement_brace_position: value.do_while_statement_brace_position.to_string(),
			enum_declaration_brace_position: value.enum_declaration_brace_position.to_string(),
			get_accessor_brace_position: value.get_accessor_brace_position.to_string(),
			if_statement_brace_position: value.if_statement_brace_position.to_string(),
			interface_declaration_brace_position: value
				.interface_declaration_brace_position
				.to_string(),
			for_statement_brace_position: value.for_statement_brace_position.to_string(),
			for_in_statement_brace_position: value.for_in_statement_brace_position.to_string(),
			for_of_statement_brace_position: value.for_of_statement_brace_position.to_string(),
			function_declaration_brace_position: value
				.function_declaration_brace_position
				.to_string(),
			function_expression_brace_position: value
				.function_expression_brace_position
				.to_string(),
			method_brace_position: value.method_brace_position.to_string(),
			module_declaration_brace_position: value.module_declaration_brace_position.to_string(),
			set_accessor_brace_position: value.set_accessor_brace_position.to_string(),
			static_block_brace_position: value.static_block_brace_position.to_string(),
			switch_case_brace_position: value.switch_case_brace_position.to_string(),
			switch_statement_brace_position: value.switch_statement_brace_position.to_string(),
			try_statement_brace_position: value.try_statement_brace_position.to_string(),
			while_statement_brace_position: value.while_statement_brace_position.to_string(),
			arguments_prefer_hanging: value.arguments_prefer_hanging.to_string(),
			array_expression_prefer_hanging: value.array_expression_prefer_hanging.to_string(),
			array_pattern_prefer_hanging: value.array_pattern_prefer_hanging,
			do_while_statement_prefer_hanging: value.do_while_statement_prefer_hanging,
			export_declaration_prefer_hanging: value.export_declaration_prefer_hanging,
			extends_clause_prefer_hanging: value.extends_clause_prefer_hanging,
			for_statement_prefer_hanging: value.for_statement_prefer_hanging,
			for_in_statement_prefer_hanging: value.for_in_statement_prefer_hanging,
			for_of_statement_prefer_hanging: value.for_of_statement_prefer_hanging,
			if_statement_prefer_hanging: value.if_statement_prefer_hanging,
			implements_clause_prefer_hanging: value.implements_clause_prefer_hanging,
			import_declaration_prefer_hanging: value.import_declaration_prefer_hanging,
			jsx_attributes_prefer_hanging: value.jsx_attributes_prefer_hanging,
			object_expression_prefer_hanging: value.object_expression_prefer_hanging,
			object_pattern_prefer_hanging: value.object_pattern_prefer_hanging,
			parameters_prefer_hanging: value.parameters_prefer_hanging.to_string(),
			sequence_expression_prefer_hanging: value.sequence_expression_prefer_hanging,
			switch_statement_prefer_hanging: value.switch_statement_prefer_hanging,
			tuple_type_prefer_hanging: value.tuple_type_prefer_hanging.to_string(),
			type_literal_prefer_hanging: value.type_literal_prefer_hanging,
			type_parameters_prefer_hanging: value.type_parameters_prefer_hanging.to_string(),
			union_and_intersection_type_prefer_hanging: value
				.union_and_intersection_type_prefer_hanging,
			variable_statement_prefer_hanging: value.variable_statement_prefer_hanging,
			while_statement_prefer_hanging: value.while_statement_prefer_hanging,
			enum_declaration_member_spacing: value.enum_declaration_member_spacing.to_string(),
			if_statement_next_control_flow_position: value
				.if_statement_next_control_flow_position
				.to_string(),
			try_statement_next_control_flow_position: value
				.try_statement_next_control_flow_position
				.to_string(),
			do_while_statement_next_control_flow_position: value
				.do_while_statement_next_control_flow_position
				.to_string(),
			binary_expression_operator_position: value
				.binary_expression_operator_position
				.to_string(),
			conditional_expression_operator_position: value
				.conditional_expression_operator_position
				.to_string(),
			conditional_type_operator_position: value
				.conditional_type_operator_position
				.to_string(),
			if_statement_single_body_position: value.if_statement_single_body_position.to_string(),
			for_statement_single_body_position: value
				.for_statement_single_body_position
				.to_string(),
			for_in_statement_single_body_position: value
				.for_in_statement_single_body_position
				.to_string(),
			for_of_statement_single_body_position: value
				.for_of_statement_single_body_position
				.to_string(),
			while_statement_single_body_position: value
				.while_statement_single_body_position
				.to_string(),
			arguments_trailing_commas: value.arguments_trailing_commas.to_string(),
			parameters_trailing_commas: value.parameters_trailing_commas.to_string(),
			array_expression_trailing_commas: value.array_expression_trailing_commas.to_string(),
			array_pattern_trailing_commas: value.array_pattern_trailing_commas.to_string(),
			enum_declaration_trailing_commas: value.enum_declaration_trailing_commas.to_string(),
			export_declaration_trailing_commas: value
				.export_declaration_trailing_commas
				.to_string(),
			import_declaration_trailing_commas: value
				.import_declaration_trailing_commas
				.to_string(),
			object_pattern_trailing_commas: value.object_pattern_trailing_commas.to_string(),
			object_expression_trailing_commas: value.object_expression_trailing_commas.to_string(),
			tuple_type_trailing_commas: value.tuple_type_trailing_commas.to_string(),
			type_literal_trailing_commas: value.type_literal_trailing_commas.to_string(),
			type_parameters_trailing_commas: value.type_parameters_trailing_commas.to_string(),
			if_statement_use_braces: value.if_statement_use_braces.to_string(),
			for_statement_use_braces: value.for_statement_use_braces.to_string(),
			for_of_statement_use_braces: value.for_of_statement_use_braces.to_string(),
			for_in_statement_use_braces: value.for_in_statement_use_braces.to_string(),
			while_statement_use_braces: value.while_statement_use_braces.to_string(),
			array_expression_prefer_single_line: value.array_expression_prefer_single_line,
			array_pattern_prefer_single_line: value.array_pattern_prefer_single_line,
			arguments_prefer_single_line: value.arguments_prefer_single_line,
			binary_expression_prefer_single_line: value.binary_expression_prefer_single_line,
			computed_prefer_single_line: value.computed_prefer_single_line,
			conditional_expression_prefer_single_line: value
				.conditional_expression_prefer_single_line,
			conditional_type_prefer_single_line: value.conditional_type_prefer_single_line,
			decorators_prefer_single_line: value.decorators_prefer_single_line,
			export_declaration_prefer_single_line: value.export_declaration_prefer_single_line,
			for_statement_prefer_single_line: value.for_statement_prefer_single_line,
			import_declaration_prefer_single_line: value.import_declaration_prefer_single_line,
			jsx_attributes_prefer_single_line: value.jsx_attributes_prefer_single_line,
			jsx_element_prefer_single_line: value.jsx_element_prefer_single_line,
			mapped_type_prefer_single_line: value.mapped_type_prefer_single_line,
			member_expression_prefer_single_line: value.member_expression_prefer_single_line,
			object_expression_prefer_single_line: value.object_expression_prefer_single_line,
			object_pattern_prefer_single_line: value.object_pattern_prefer_single_line,
			parameters_prefer_single_line: value.parameters_prefer_single_line,
			parentheses_prefer_single_line: value.parentheses_prefer_single_line,
			tuple_type_prefer_single_line: value.tuple_type_prefer_single_line,
			type_literal_prefer_single_line: value.type_literal_prefer_single_line,
			type_parameters_prefer_single_line: value.type_parameters_prefer_single_line,
			union_and_intersection_type_prefer_single_line: value
				.union_and_intersection_type_prefer_single_line,
			variable_statement_prefer_single_line: value.variable_statement_prefer_single_line,
			import_declaration_force_single_line: value.import_declaration_force_single_line,
			export_declaration_force_single_line: value.export_declaration_force_single_line,
			export_declaration_force_multi_line: value
				.export_declaration_force_multi_line
				.to_string(),
			import_declaration_force_multi_line: value
				.import_declaration_force_multi_line
				.to_string(),
			binary_expression_space_surrounding_bitwise_and_arithmetic_operator: value
				.binary_expression_space_surrounding_bitwise_and_arithmetic_operator,
			comment_line_force_space_after_slashes: value.comment_line_force_space_after_slashes,
			construct_signature_space_after_new_keyword: value
				.construct_signature_space_after_new_keyword,
			constructor_space_before_parentheses: value.constructor_space_before_parentheses,
			constructor_type_space_after_new_keyword: value
				.constructor_type_space_after_new_keyword,
			do_while_statement_space_after_while_keyword: value
				.do_while_statement_space_after_while_keyword,
			export_declaration_space_surrounding_named_exports: value
				.export_declaration_space_surrounding_named_exports,
			for_statement_space_after_for_keyword: value.for_statement_space_after_for_keyword,
			for_statement_space_after_semi_colons: value.for_statement_space_after_semi_colons,
			for_in_statement_space_after_for_keyword: value
				.for_in_statement_space_after_for_keyword,
			for_of_statement_space_after_for_keyword: value
				.for_of_statement_space_after_for_keyword,
			function_declaration_space_before_parentheses: value
				.function_declaration_space_before_parentheses,
			function_expression_space_before_parentheses: value
				.function_expression_space_before_parentheses,
			function_expression_space_after_function_keyword: value
				.function_expression_space_after_function_keyword,
			get_accessor_space_before_parentheses: value.get_accessor_space_before_parentheses,
			if_statement_space_after_if_keyword: value.if_statement_space_after_if_keyword,
			import_declaration_space_surrounding_named_imports: value
				.import_declaration_space_surrounding_named_imports,
			jsx_expression_container_space_surrounding_expression: value
				.jsx_expression_container_space_surrounding_expression,
			jsx_self_closing_element_space_before_slash: value
				.jsx_self_closing_element_space_before_slash,
			method_space_before_parentheses: value.method_space_before_parentheses,
			object_expression_space_surrounding_properties: value
				.object_expression_space_surrounding_properties,
			object_pattern_space_surrounding_properties: value
				.object_pattern_space_surrounding_properties,
			set_accessor_space_before_parentheses: value.set_accessor_space_before_parentheses,
			space_surrounding_properties: value.space_surrounding_properties,
			tagged_template_space_before_literal: value.tagged_template_space_before_literal,
			type_annotation_space_before_colon: value.type_annotation_space_before_colon,
			type_assertion_space_before_expression: value.type_assertion_space_before_expression,
			type_literal_space_surrounding_properties: value
				.type_literal_space_surrounding_properties,
			while_statement_space_after_while_keyword: value
				.while_statement_space_after_while_keyword,
			arguments_space_around: value.arguments_space_around,
			array_expression_space_around: value.array_expression_space_around,
			array_pattern_space_around: value.array_pattern_space_around,
			catch_clause_space_around: value.catch_clause_space_around,
			do_while_statement_space_around: value.do_while_statement_space_around,
			for_in_statement_space_around: value.for_in_statement_space_around,
			for_of_statement_space_around: value.for_of_statement_space_around,
			for_statement_space_around: value.for_statement_space_around,
			if_statement_space_around: value.if_statement_space_around,
			parameters_space_around: value.parameters_space_around,
			paren_expression_space_around: value.paren_expression_space_around,
			switch_statement_space_around: value.switch_statement_space_around,
			tuple_type_space_around: value.tuple_type_space_around,
			while_statement_space_around: value.while_statement_space_around,
		}
	}
}

const SEMI_COLONS_DEFAULT: &str = "prefer";
const BRACE_POSITION_DEFAULT: &str = "sameLineUnlessHanging";
const NEXT_CONTROL_FLOW_POSITION_DEFAULT: &str = "sameLine";
const OPERATOR_POSITION_DEFAULT: &str = "nextLine";
const SINGLE_BODY_POSITION_DEFAULT: &str = "maintain";
const TRAILING_COMMAS_DEFAULT: &str = "onlyMultiLine";
const USE_BRACES_DEFAULT: &str = "whenNotSingleLine";
const PREFER_HANGING_DEFAULT: bool = false;
const PREFER_HANGING_GRANULAR_DEFAULT: &str = "never";
const PREFER_SINGLE_LINE_DEFAULT: bool = false;
const SPACE_SURROUNDING_PROPERTIES_DEFAULT: bool = true;
const TYPE_LITERAL_SEPARATOR_KIND_DEFAULT: &str = "semiColon";
const QUOTE_STYLE_DEFAULT: &str = "alwaysDouble";
const QUOTE_PROPS_DEFAULT: &str = "preserve";
const SPACE_AROUND_DEFAULT: bool = false;
const JSX_BRACKET_POSITION_DEFAULT: &str = "nextLine";

const NEW_LINE_KIND_DEFAULT: &str = "lf";
const FILE_INDENT_LEVEL_DEFAULT: u32 = 0;
const ARROW_FUNCTION_USE_PARENTHESES_DEFAULT: &str = "maintain";
const BINARY_EXPRESSION_LINE_PER_EXPRESSION_DEFAULT: bool = false;
const CONDITIONAL_EXPRESSION_LINE_PER_EXPRESSION_DEFAULT: bool = true;
const JSX_QUOTE_STYLE_DEFAULT: &str = "preferDouble";
const JSX_MULTI_LINE_PARENS_DEFAULT: &str = "prefer";
const JSX_FORCE_NEW_LINES_SURROUNDING_CONTENT_DEFAULT: bool = false;
const MEMBER_EXPRESSION_LINE_PER_EXPRESSION_DEFAULT: bool = false;
const MODULE_SORT_IMPORT_DECLARATIONS_DEFAULT: &str = "caseInsensitive";
const MODULE_SORT_EXPORT_DECLARATIONS_DEFAULT: &str = "caseInsensitive";
const IMPORT_DECLARATION_SORT_NAMED_IMPORTS_DEFAULT: &str = "caseInsensitive";
const IMPORT_DECLARATION_SORT_TYPE_ONLY_IMPORTS_DEFAULT: &str = "none";
const EXPORT_DECLARATION_SORT_NAMED_EXPORTS_DEFAULT: &str = "caseInsensitive";
const EXPORT_DECLARATION_SORT_TYPE_ONLY_EXPORTS_DEFAULT: &str = "none";
const IGNORE_NODE_COMMENT_TEXT_DEFAULT: &str = "dprint-ignore";
const IGNORE_FILE_COMMENT_TEXT_DEFAULT: &str = "dprint-ignore-file";
const ENUM_DECLARATION_MEMBER_SPACING_DEFAULT: &str = "maintain";
const IMPORT_DECLARATION_FORCE_SINGLE_LINE_DEFAULT: bool = false;
const EXPORT_DECLARATION_FORCE_SINGLE_LINE_DEFAULT: bool = false;
const EXPORT_DECLARATION_FORCE_MULTI_LINE_DEFAULT: &str = "never";
const IMPORT_DECLARATION_FORCE_MULTI_LINE_DEFAULT: &str = "never";
const BINARY_EXPRESSION_SPACE_SURROUNDING_BITWISE_AND_ARITHMETIC_OPERATOR_DEFAULT: bool = true;
const COMMENT_LINE_FORCE_SPACE_AFTER_SLASHES_DEFAULT: bool = true;
const CONSTRUCT_SIGNATURE_SPACE_AFTER_NEW_KEYWORD_DEFAULT: bool = false;
const CONSTRUCTOR_SPACE_BEFORE_PARENTHESES_DEFAULT: bool = false;
const CONSTRUCTOR_TYPE_SPACE_AFTER_NEW_KEYWORD_DEFAULT: bool = false;
const DO_WHILE_STATEMENT_SPACE_AFTER_WHILE_KEYWORD_DEFAULT: bool = true;
const EXPORT_DECLARATION_SPACE_SURROUNDING_NAMED_EXPORTS_DEFAULT: bool = true;
const FOR_STATEMENT_SPACE_AFTER_FOR_KEYWORD_DEFAULT: bool = true;
const FOR_STATEMENT_SPACE_AFTER_SEMI_COLONS_DEFAULT: bool = true;
const FOR_IN_STATEMENT_SPACE_AFTER_FOR_KEYWORD_DEFAULT: bool = true;
const FOR_OF_STATEMENT_SPACE_AFTER_FOR_KEYWORD_DEFAULT: bool = true;
const FUNCTION_DECLARATION_SPACE_BEFORE_PARENTHESES_DEFAULT: bool = false;
const FUNCTION_EXPRESSION_SPACE_BEFORE_PARENTHESES_DEFAULT: bool = false;
const FUNCTION_EXPRESSION_SPACE_AFTER_FUNCTION_KEYWORD_DEFAULT: bool = false;
const GET_ACCESSOR_SPACE_BEFORE_PARENTHESES_DEFAULT: bool = false;
const IF_STATEMENT_SPACE_AFTER_IF_KEYWORD_DEFAULT: bool = true;
const IMPORT_DECLARATION_SPACE_SURROUNDING_NAMED_IMPORTS_DEFAULT: bool = true;
const JSX_EXPRESSION_CONTAINER_SPACE_SURROUNDING_EXPRESSION_DEFAULT: bool = false;
const JSX_SELF_CLOSING_ELEMENT_SPACE_BEFORE_SLASH_DEFAULT: bool = true;
const SET_ACCESSOR_SPACE_BEFORE_PARENTHESES_DEFAULT: bool = false;
const TAGGED_TEMPLATE_SPACE_BEFORE_LITERAL_DEFAULT: bool = false;
const TYPE_ANNOTATION_SPACE_BEFORE_COLON_DEFAULT: bool = false;
const TYPE_ASSERTION_SPACE_BEFORE_EXPRESSION_DEFAULT: bool = true;
const WHILE_STATEMENT_SPACE_AFTER_WHILE_KEYWORD_DEFAULT: bool = true;

#[pymethods]
impl PyConfiguration {
	#![allow(clippy::too_many_arguments)]
	#[pyo3(signature = (
		indent_width: "int" = RECOMMENDED_GLOBAL_CONFIGURATION.indent_width,
		line_width: "int" = RECOMMENDED_GLOBAL_CONFIGURATION.line_width,
		use_tabs: "bool" = RECOMMENDED_GLOBAL_CONFIGURATION.use_tabs,
		new_line_kind: "str" = NEW_LINE_KIND_DEFAULT,
		quote_style: "str" = QUOTE_STYLE_DEFAULT,
		quote_props: "str" = QUOTE_PROPS_DEFAULT,
		semi_colons: "str" = SEMI_COLONS_DEFAULT,
		file_indent_level: "int" = FILE_INDENT_LEVEL_DEFAULT,
		arrow_function_use_parentheses: "str" = ARROW_FUNCTION_USE_PARENTHESES_DEFAULT,
		binary_expression_line_per_expression: "bool" = BINARY_EXPRESSION_LINE_PER_EXPRESSION_DEFAULT,
		conditional_expression_line_per_expression: "bool" = CONDITIONAL_EXPRESSION_LINE_PER_EXPRESSION_DEFAULT,
		jsx_quote_style: "str" = JSX_QUOTE_STYLE_DEFAULT,
		jsx_multi_line_parens: "str" = JSX_MULTI_LINE_PARENS_DEFAULT,
		jsx_force_new_lines_surrounding_content: "bool" = JSX_FORCE_NEW_LINES_SURROUNDING_CONTENT_DEFAULT,
		jsx_opening_element_bracket_position: "str" = JSX_BRACKET_POSITION_DEFAULT,
		jsx_self_closing_element_bracket_position: "str" = JSX_BRACKET_POSITION_DEFAULT,
		member_expression_line_per_expression: "bool" = MEMBER_EXPRESSION_LINE_PER_EXPRESSION_DEFAULT,
		type_literal_separator_kind_single_line: "str" = TYPE_LITERAL_SEPARATOR_KIND_DEFAULT,
		type_literal_separator_kind_multi_line: "str" = TYPE_LITERAL_SEPARATOR_KIND_DEFAULT,
		module_sort_import_declarations: "str" = MODULE_SORT_IMPORT_DECLARATIONS_DEFAULT,
		module_sort_export_declarations: "str" = MODULE_SORT_EXPORT_DECLARATIONS_DEFAULT,
		import_declaration_sort_named_imports: "str" = IMPORT_DECLARATION_SORT_NAMED_IMPORTS_DEFAULT,
		import_declaration_sort_type_only_imports: "str" = IMPORT_DECLARATION_SORT_TYPE_ONLY_IMPORTS_DEFAULT,
		export_declaration_sort_named_exports: "str" = EXPORT_DECLARATION_SORT_NAMED_EXPORTS_DEFAULT,
		export_declaration_sort_type_only_exports: "str" = EXPORT_DECLARATION_SORT_TYPE_ONLY_EXPORTS_DEFAULT,
		ignore_node_comment_text: "str" = IGNORE_NODE_COMMENT_TEXT_DEFAULT,
		ignore_file_comment_text: "str" = IGNORE_FILE_COMMENT_TEXT_DEFAULT,
		arrow_function_brace_position: "str" = BRACE_POSITION_DEFAULT,
		class_declaration_brace_position: "str" = BRACE_POSITION_DEFAULT,
		class_expression_brace_position: "str" = BRACE_POSITION_DEFAULT,
		constructor_brace_position: "str" = BRACE_POSITION_DEFAULT,
		do_while_statement_brace_position: "str" = BRACE_POSITION_DEFAULT,
		enum_declaration_brace_position: "str" = BRACE_POSITION_DEFAULT,
		get_accessor_brace_position: "str" = BRACE_POSITION_DEFAULT,
		if_statement_brace_position: "str" = BRACE_POSITION_DEFAULT,
		interface_declaration_brace_position: "str" = BRACE_POSITION_DEFAULT,
		for_statement_brace_position: "str" = BRACE_POSITION_DEFAULT,
		for_in_statement_brace_position: "str" = BRACE_POSITION_DEFAULT,
		for_of_statement_brace_position: "str" = BRACE_POSITION_DEFAULT,
		function_declaration_brace_position: "str" = BRACE_POSITION_DEFAULT,
		function_expression_brace_position: "str" = BRACE_POSITION_DEFAULT,
		method_brace_position: "str" = BRACE_POSITION_DEFAULT,
		module_declaration_brace_position: "str" = BRACE_POSITION_DEFAULT,
		set_accessor_brace_position: "str" = BRACE_POSITION_DEFAULT,
		static_block_brace_position: "str" = BRACE_POSITION_DEFAULT,
		switch_case_brace_position: "str" = BRACE_POSITION_DEFAULT,
		switch_statement_brace_position: "str" = BRACE_POSITION_DEFAULT,
		try_statement_brace_position: "str" = BRACE_POSITION_DEFAULT,
		while_statement_brace_position: "str" = BRACE_POSITION_DEFAULT,
		arguments_prefer_hanging: "str" = PREFER_HANGING_GRANULAR_DEFAULT,
		array_expression_prefer_hanging: "str" = PREFER_HANGING_GRANULAR_DEFAULT,
		array_pattern_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		do_while_statement_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		export_declaration_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		extends_clause_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		for_statement_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		for_in_statement_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		for_of_statement_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		if_statement_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		implements_clause_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		import_declaration_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		jsx_attributes_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		object_expression_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		object_pattern_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		parameters_prefer_hanging: "str" = PREFER_HANGING_GRANULAR_DEFAULT,
		sequence_expression_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		switch_statement_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		tuple_type_prefer_hanging: "str" = PREFER_HANGING_GRANULAR_DEFAULT,
		type_literal_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		type_parameters_prefer_hanging: "str" = PREFER_HANGING_GRANULAR_DEFAULT,
		union_and_intersection_type_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		variable_statement_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		while_statement_prefer_hanging: "bool" = PREFER_HANGING_DEFAULT,
		enum_declaration_member_spacing: "str" = ENUM_DECLARATION_MEMBER_SPACING_DEFAULT,
		if_statement_next_control_flow_position: "str" = NEXT_CONTROL_FLOW_POSITION_DEFAULT,
		try_statement_next_control_flow_position: "str" = NEXT_CONTROL_FLOW_POSITION_DEFAULT,
		do_while_statement_next_control_flow_position: "str" = NEXT_CONTROL_FLOW_POSITION_DEFAULT,
		binary_expression_operator_position: "str" = OPERATOR_POSITION_DEFAULT,
		conditional_expression_operator_position: "str" = OPERATOR_POSITION_DEFAULT,
		conditional_type_operator_position: "str" = OPERATOR_POSITION_DEFAULT,
		if_statement_single_body_position: "str" = SINGLE_BODY_POSITION_DEFAULT,
		for_statement_single_body_position: "str" = SINGLE_BODY_POSITION_DEFAULT,
		for_in_statement_single_body_position: "str" = SINGLE_BODY_POSITION_DEFAULT,
		for_of_statement_single_body_position: "str" = SINGLE_BODY_POSITION_DEFAULT,
		while_statement_single_body_position: "str" = SINGLE_BODY_POSITION_DEFAULT,
		arguments_trailing_commas: "str" = TRAILING_COMMAS_DEFAULT,
		parameters_trailing_commas: "str" = TRAILING_COMMAS_DEFAULT,
		array_expression_trailing_commas: "str" = TRAILING_COMMAS_DEFAULT,
		array_pattern_trailing_commas: "str" = TRAILING_COMMAS_DEFAULT,
		enum_declaration_trailing_commas: "str" = TRAILING_COMMAS_DEFAULT,
		export_declaration_trailing_commas: "str" = TRAILING_COMMAS_DEFAULT,
		import_declaration_trailing_commas: "str" = TRAILING_COMMAS_DEFAULT,
		object_pattern_trailing_commas: "str" = TRAILING_COMMAS_DEFAULT,
		object_expression_trailing_commas: "str" = TRAILING_COMMAS_DEFAULT,
		tuple_type_trailing_commas: "str" = TRAILING_COMMAS_DEFAULT,
		type_literal_trailing_commas: "str" = TRAILING_COMMAS_DEFAULT,
		type_parameters_trailing_commas: "str" = TRAILING_COMMAS_DEFAULT,
		if_statement_use_braces: "str" = USE_BRACES_DEFAULT,
		for_statement_use_braces: "str" = USE_BRACES_DEFAULT,
		for_of_statement_use_braces: "str" = USE_BRACES_DEFAULT,
		for_in_statement_use_braces: "str" = USE_BRACES_DEFAULT,
		while_statement_use_braces: "str" = USE_BRACES_DEFAULT,
		array_expression_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		array_pattern_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		arguments_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		binary_expression_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		computed_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		conditional_expression_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		conditional_type_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		decorators_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		export_declaration_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		for_statement_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		import_declaration_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		jsx_attributes_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		jsx_element_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		mapped_type_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		member_expression_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		object_expression_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		object_pattern_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		parameters_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		parentheses_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		tuple_type_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		type_literal_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		type_parameters_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		union_and_intersection_type_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		variable_statement_prefer_single_line: "bool" = PREFER_SINGLE_LINE_DEFAULT,
		import_declaration_force_single_line: "bool" = IMPORT_DECLARATION_FORCE_SINGLE_LINE_DEFAULT,
		export_declaration_force_single_line: "bool" = EXPORT_DECLARATION_FORCE_SINGLE_LINE_DEFAULT,
		export_declaration_force_multi_line: "str" = EXPORT_DECLARATION_FORCE_MULTI_LINE_DEFAULT,
		import_declaration_force_multi_line: "str" = IMPORT_DECLARATION_FORCE_MULTI_LINE_DEFAULT,
		binary_expression_space_surrounding_bitwise_and_arithmetic_operator: "bool" = BINARY_EXPRESSION_SPACE_SURROUNDING_BITWISE_AND_ARITHMETIC_OPERATOR_DEFAULT,
		comment_line_force_space_after_slashes: "bool" = COMMENT_LINE_FORCE_SPACE_AFTER_SLASHES_DEFAULT,
		construct_signature_space_after_new_keyword: "bool" = CONSTRUCT_SIGNATURE_SPACE_AFTER_NEW_KEYWORD_DEFAULT,
		constructor_space_before_parentheses: "bool" = CONSTRUCTOR_SPACE_BEFORE_PARENTHESES_DEFAULT,
		constructor_type_space_after_new_keyword: "bool" = CONSTRUCTOR_TYPE_SPACE_AFTER_NEW_KEYWORD_DEFAULT,
		do_while_statement_space_after_while_keyword: "bool" = DO_WHILE_STATEMENT_SPACE_AFTER_WHILE_KEYWORD_DEFAULT,
		export_declaration_space_surrounding_named_exports: "bool" = EXPORT_DECLARATION_SPACE_SURROUNDING_NAMED_EXPORTS_DEFAULT,
		for_statement_space_after_for_keyword: "bool" = FOR_STATEMENT_SPACE_AFTER_FOR_KEYWORD_DEFAULT,
		for_statement_space_after_semi_colons: "bool" = FOR_STATEMENT_SPACE_AFTER_SEMI_COLONS_DEFAULT,
		for_in_statement_space_after_for_keyword: "bool" = FOR_IN_STATEMENT_SPACE_AFTER_FOR_KEYWORD_DEFAULT,
		for_of_statement_space_after_for_keyword: "bool" = FOR_OF_STATEMENT_SPACE_AFTER_FOR_KEYWORD_DEFAULT,
		function_declaration_space_before_parentheses: "bool" = FUNCTION_DECLARATION_SPACE_BEFORE_PARENTHESES_DEFAULT,
		function_expression_space_before_parentheses: "bool" = FUNCTION_EXPRESSION_SPACE_BEFORE_PARENTHESES_DEFAULT,
		function_expression_space_after_function_keyword: "bool" = FUNCTION_EXPRESSION_SPACE_AFTER_FUNCTION_KEYWORD_DEFAULT,
		get_accessor_space_before_parentheses: "bool" = GET_ACCESSOR_SPACE_BEFORE_PARENTHESES_DEFAULT,
		if_statement_space_after_if_keyword: "bool" = IF_STATEMENT_SPACE_AFTER_IF_KEYWORD_DEFAULT,
		import_declaration_space_surrounding_named_imports: "bool" = IMPORT_DECLARATION_SPACE_SURROUNDING_NAMED_IMPORTS_DEFAULT,
		jsx_expression_container_space_surrounding_expression: "bool" = JSX_EXPRESSION_CONTAINER_SPACE_SURROUNDING_EXPRESSION_DEFAULT,
		jsx_self_closing_element_space_before_slash: "bool" = JSX_SELF_CLOSING_ELEMENT_SPACE_BEFORE_SLASH_DEFAULT,
		method_space_before_parentheses: "bool" = SPACE_SURROUNDING_PROPERTIES_DEFAULT,
		object_expression_space_surrounding_properties: "bool" = SPACE_SURROUNDING_PROPERTIES_DEFAULT,
		object_pattern_space_surrounding_properties: "bool" = SPACE_SURROUNDING_PROPERTIES_DEFAULT,
		set_accessor_space_before_parentheses: "bool" = SET_ACCESSOR_SPACE_BEFORE_PARENTHESES_DEFAULT,
		space_surrounding_properties: "bool" = SPACE_SURROUNDING_PROPERTIES_DEFAULT,
		tagged_template_space_before_literal: "bool" = TAGGED_TEMPLATE_SPACE_BEFORE_LITERAL_DEFAULT,
		type_annotation_space_before_colon: "bool" = TYPE_ANNOTATION_SPACE_BEFORE_COLON_DEFAULT,
		type_assertion_space_before_expression: "bool" = TYPE_ASSERTION_SPACE_BEFORE_EXPRESSION_DEFAULT,
		type_literal_space_surrounding_properties: "bool" = SPACE_SURROUNDING_PROPERTIES_DEFAULT,
		while_statement_space_after_while_keyword: "bool" = WHILE_STATEMENT_SPACE_AFTER_WHILE_KEYWORD_DEFAULT,
		arguments_space_around: "bool" = SPACE_AROUND_DEFAULT,
		array_expression_space_around: "bool" = SPACE_AROUND_DEFAULT,
		array_pattern_space_around: "bool" = SPACE_AROUND_DEFAULT,
		catch_clause_space_around: "bool" = SPACE_AROUND_DEFAULT,
		do_while_statement_space_around: "bool" = SPACE_AROUND_DEFAULT,
		for_in_statement_space_around: "bool" = SPACE_AROUND_DEFAULT,
		for_of_statement_space_around: "bool" = SPACE_AROUND_DEFAULT,
		for_statement_space_around: "bool" = SPACE_AROUND_DEFAULT,
		if_statement_space_around: "bool" = SPACE_AROUND_DEFAULT,
		parameters_space_around: "bool" = SPACE_AROUND_DEFAULT,
		paren_expression_space_around: "bool" = SPACE_AROUND_DEFAULT,
		switch_statement_space_around: "bool" = SPACE_AROUND_DEFAULT,
		tuple_type_space_around: "bool" = SPACE_AROUND_DEFAULT,
		while_statement_space_around: "bool" = SPACE_AROUND_DEFAULT,
	))
	]
	#[new]
	pub fn __init__(
		indent_width: u8,
		line_width: u32,
		use_tabs: bool,
		new_line_kind: &str,
		quote_style: &str,
		quote_props: &str,
		semi_colons: &str,
		file_indent_level: u32,
		arrow_function_use_parentheses: &str,
		binary_expression_line_per_expression: bool,
		conditional_expression_line_per_expression: bool,
		jsx_quote_style: &str,
		jsx_multi_line_parens: &str,
		jsx_force_new_lines_surrounding_content: bool,
		jsx_opening_element_bracket_position: &str,
		jsx_self_closing_element_bracket_position: &str,
		member_expression_line_per_expression: bool,
		type_literal_separator_kind_single_line: &str,
		type_literal_separator_kind_multi_line: &str,
		module_sort_import_declarations: &str,
		module_sort_export_declarations: &str,
		import_declaration_sort_named_imports: &str,
		import_declaration_sort_type_only_imports: &str,
		export_declaration_sort_named_exports: &str,
		export_declaration_sort_type_only_exports: &str,
		ignore_node_comment_text: &str,
		ignore_file_comment_text: &str,
		arrow_function_brace_position: &str,
		class_declaration_brace_position: &str,
		class_expression_brace_position: &str,
		constructor_brace_position: &str,
		do_while_statement_brace_position: &str,
		enum_declaration_brace_position: &str,
		get_accessor_brace_position: &str,
		if_statement_brace_position: &str,
		interface_declaration_brace_position: &str,
		for_statement_brace_position: &str,
		for_in_statement_brace_position: &str,
		for_of_statement_brace_position: &str,
		function_declaration_brace_position: &str,
		function_expression_brace_position: &str,
		method_brace_position: &str,
		module_declaration_brace_position: &str,
		set_accessor_brace_position: &str,
		static_block_brace_position: &str,
		switch_case_brace_position: &str,
		switch_statement_brace_position: &str,
		try_statement_brace_position: &str,
		while_statement_brace_position: &str,
		arguments_prefer_hanging: &str,
		array_expression_prefer_hanging: &str,
		array_pattern_prefer_hanging: bool,
		do_while_statement_prefer_hanging: bool,
		export_declaration_prefer_hanging: bool,
		extends_clause_prefer_hanging: bool,
		for_statement_prefer_hanging: bool,
		for_in_statement_prefer_hanging: bool,
		for_of_statement_prefer_hanging: bool,
		if_statement_prefer_hanging: bool,
		implements_clause_prefer_hanging: bool,
		import_declaration_prefer_hanging: bool,
		jsx_attributes_prefer_hanging: bool,
		object_expression_prefer_hanging: bool,
		object_pattern_prefer_hanging: bool,
		parameters_prefer_hanging: &str,
		sequence_expression_prefer_hanging: bool,
		switch_statement_prefer_hanging: bool,
		tuple_type_prefer_hanging: &str,
		type_literal_prefer_hanging: bool,
		type_parameters_prefer_hanging: &str,
		union_and_intersection_type_prefer_hanging: bool,
		variable_statement_prefer_hanging: bool,
		while_statement_prefer_hanging: bool,
		enum_declaration_member_spacing: &str,
		if_statement_next_control_flow_position: &str,
		try_statement_next_control_flow_position: &str,
		do_while_statement_next_control_flow_position: &str,
		binary_expression_operator_position: &str,
		conditional_expression_operator_position: &str,
		conditional_type_operator_position: &str,
		if_statement_single_body_position: &str,
		for_statement_single_body_position: &str,
		for_in_statement_single_body_position: &str,
		for_of_statement_single_body_position: &str,
		while_statement_single_body_position: &str,
		arguments_trailing_commas: &str,
		parameters_trailing_commas: &str,
		array_expression_trailing_commas: &str,
		array_pattern_trailing_commas: &str,
		enum_declaration_trailing_commas: &str,
		export_declaration_trailing_commas: &str,
		import_declaration_trailing_commas: &str,
		object_pattern_trailing_commas: &str,
		object_expression_trailing_commas: &str,
		tuple_type_trailing_commas: &str,
		type_literal_trailing_commas: &str,
		type_parameters_trailing_commas: &str,
		if_statement_use_braces: &str,
		for_statement_use_braces: &str,
		for_of_statement_use_braces: &str,
		for_in_statement_use_braces: &str,
		while_statement_use_braces: &str,
		array_expression_prefer_single_line: bool,
		array_pattern_prefer_single_line: bool,
		arguments_prefer_single_line: bool,
		binary_expression_prefer_single_line: bool,
		computed_prefer_single_line: bool,
		conditional_expression_prefer_single_line: bool,
		conditional_type_prefer_single_line: bool,
		decorators_prefer_single_line: bool,
		export_declaration_prefer_single_line: bool,
		for_statement_prefer_single_line: bool,
		import_declaration_prefer_single_line: bool,
		jsx_attributes_prefer_single_line: bool,
		jsx_element_prefer_single_line: bool,
		mapped_type_prefer_single_line: bool,
		member_expression_prefer_single_line: bool,
		object_expression_prefer_single_line: bool,
		object_pattern_prefer_single_line: bool,
		parameters_prefer_single_line: bool,
		parentheses_prefer_single_line: bool,
		tuple_type_prefer_single_line: bool,
		type_literal_prefer_single_line: bool,
		type_parameters_prefer_single_line: bool,
		union_and_intersection_type_prefer_single_line: bool,
		variable_statement_prefer_single_line: bool,
		import_declaration_force_single_line: bool,
		export_declaration_force_single_line: bool,
		export_declaration_force_multi_line: &str,
		import_declaration_force_multi_line: &str,
		binary_expression_space_surrounding_bitwise_and_arithmetic_operator: bool,
		comment_line_force_space_after_slashes: bool,
		construct_signature_space_after_new_keyword: bool,
		constructor_space_before_parentheses: bool,
		constructor_type_space_after_new_keyword: bool,
		do_while_statement_space_after_while_keyword: bool,
		export_declaration_space_surrounding_named_exports: bool,
		for_statement_space_after_for_keyword: bool,
		for_statement_space_after_semi_colons: bool,
		for_in_statement_space_after_for_keyword: bool,
		for_of_statement_space_after_for_keyword: bool,
		function_declaration_space_before_parentheses: bool,
		function_expression_space_before_parentheses: bool,
		function_expression_space_after_function_keyword: bool,
		get_accessor_space_before_parentheses: bool,
		if_statement_space_after_if_keyword: bool,
		import_declaration_space_surrounding_named_imports: bool,
		jsx_expression_container_space_surrounding_expression: bool,
		jsx_self_closing_element_space_before_slash: bool,
		method_space_before_parentheses: bool,
		object_expression_space_surrounding_properties: bool,
		object_pattern_space_surrounding_properties: bool,
		set_accessor_space_before_parentheses: bool,
		space_surrounding_properties: bool,
		tagged_template_space_before_literal: bool,
		type_annotation_space_before_colon: bool,
		type_assertion_space_before_expression: bool,
		type_literal_space_surrounding_properties: bool,
		while_statement_space_after_while_keyword: bool,
		arguments_space_around: bool,
		array_expression_space_around: bool,
		array_pattern_space_around: bool,
		catch_clause_space_around: bool,
		do_while_statement_space_around: bool,
		for_in_statement_space_around: bool,
		for_of_statement_space_around: bool,
		for_statement_space_around: bool,
		if_statement_space_around: bool,
		parameters_space_around: bool,
		paren_expression_space_around: bool,
		switch_statement_space_around: bool,
		tuple_type_space_around: bool,
		while_statement_space_around: bool,
	) -> PyResult<Self> {
		Ok(PyConfiguration {
			indent_width,
			line_width,
			use_tabs,
			new_line_kind: new_line_kind.to_string(),
			quote_style: quote_style.to_string(),
			quote_props: quote_props.to_string(),
			semi_colons: semi_colons.to_string(),
			file_indent_level,
			arrow_function_use_parentheses: arrow_function_use_parentheses.to_string(),
			binary_expression_line_per_expression,
			conditional_expression_line_per_expression,
			jsx_quote_style: jsx_quote_style.to_string(),
			jsx_multi_line_parens: jsx_multi_line_parens.to_string(),
			jsx_force_new_lines_surrounding_content,
			jsx_opening_element_bracket_position: jsx_opening_element_bracket_position.to_string(),
			jsx_self_closing_element_bracket_position: jsx_self_closing_element_bracket_position
				.to_string(),
			member_expression_line_per_expression,
			type_literal_separator_kind_single_line: type_literal_separator_kind_single_line
				.to_string(),
			type_literal_separator_kind_multi_line: type_literal_separator_kind_multi_line
				.to_string(),
			module_sort_import_declarations: module_sort_import_declarations.to_string(),
			module_sort_export_declarations: module_sort_export_declarations.to_string(),
			import_declaration_sort_named_imports: import_declaration_sort_named_imports
				.to_string(),
			import_declaration_sort_type_only_imports: import_declaration_sort_type_only_imports
				.to_string(),
			export_declaration_sort_named_exports: export_declaration_sort_named_exports
				.to_string(),
			export_declaration_sort_type_only_exports: export_declaration_sort_type_only_exports
				.to_string(),
			ignore_node_comment_text: ignore_node_comment_text.to_string(),
			ignore_file_comment_text: ignore_file_comment_text.to_string(),
			arrow_function_brace_position: arrow_function_brace_position.to_string(),
			class_declaration_brace_position: class_declaration_brace_position.to_string(),
			class_expression_brace_position: class_expression_brace_position.to_string(),
			constructor_brace_position: constructor_brace_position.to_string(),
			do_while_statement_brace_position: do_while_statement_brace_position.to_string(),
			enum_declaration_brace_position: enum_declaration_brace_position.to_string(),
			get_accessor_brace_position: get_accessor_brace_position.to_string(),
			if_statement_brace_position: if_statement_brace_position.to_string(),
			interface_declaration_brace_position: interface_declaration_brace_position.to_string(),
			for_statement_brace_position: for_statement_brace_position.to_string(),
			for_in_statement_brace_position: for_in_statement_brace_position.to_string(),
			for_of_statement_brace_position: for_of_statement_brace_position.to_string(),
			function_declaration_brace_position: function_declaration_brace_position.to_string(),
			function_expression_brace_position: function_expression_brace_position.to_string(),
			method_brace_position: method_brace_position.to_string(),
			module_declaration_brace_position: module_declaration_brace_position.to_string(),
			set_accessor_brace_position: set_accessor_brace_position.to_string(),
			static_block_brace_position: static_block_brace_position.to_string(),
			switch_case_brace_position: switch_case_brace_position.to_string(),
			switch_statement_brace_position: switch_statement_brace_position.to_string(),
			try_statement_brace_position: try_statement_brace_position.to_string(),
			while_statement_brace_position: while_statement_brace_position.to_string(),
			arguments_prefer_hanging: arguments_prefer_hanging.to_string(),
			array_expression_prefer_hanging: array_expression_prefer_hanging.to_string(),
			array_pattern_prefer_hanging,
			do_while_statement_prefer_hanging,
			export_declaration_prefer_hanging,
			extends_clause_prefer_hanging,
			for_statement_prefer_hanging,
			for_in_statement_prefer_hanging,
			for_of_statement_prefer_hanging,
			if_statement_prefer_hanging,
			implements_clause_prefer_hanging,
			import_declaration_prefer_hanging,
			jsx_attributes_prefer_hanging,
			object_expression_prefer_hanging,
			object_pattern_prefer_hanging,
			parameters_prefer_hanging: parameters_prefer_hanging.to_string(),
			sequence_expression_prefer_hanging,
			switch_statement_prefer_hanging,
			tuple_type_prefer_hanging: tuple_type_prefer_hanging.to_string(),
			type_literal_prefer_hanging,
			type_parameters_prefer_hanging: type_parameters_prefer_hanging.to_string(),
			union_and_intersection_type_prefer_hanging,
			variable_statement_prefer_hanging,
			while_statement_prefer_hanging,
			enum_declaration_member_spacing: enum_declaration_member_spacing.to_string(),
			if_statement_next_control_flow_position: if_statement_next_control_flow_position
				.to_string(),
			try_statement_next_control_flow_position: try_statement_next_control_flow_position
				.to_string(),
			do_while_statement_next_control_flow_position:
				do_while_statement_next_control_flow_position.to_string(),
			binary_expression_operator_position: binary_expression_operator_position.to_string(),
			conditional_expression_operator_position: conditional_expression_operator_position
				.to_string(),
			conditional_type_operator_position: conditional_type_operator_position.to_string(),
			if_statement_single_body_position: if_statement_single_body_position.to_string(),
			for_statement_single_body_position: for_statement_single_body_position.to_string(),
			for_in_statement_single_body_position: for_in_statement_single_body_position
				.to_string(),
			for_of_statement_single_body_position: for_of_statement_single_body_position
				.to_string(),
			while_statement_single_body_position: while_statement_single_body_position.to_string(),
			arguments_trailing_commas: arguments_trailing_commas.to_string(),
			parameters_trailing_commas: parameters_trailing_commas.to_string(),
			array_expression_trailing_commas: array_expression_trailing_commas.to_string(),
			array_pattern_trailing_commas: array_pattern_trailing_commas.to_string(),
			enum_declaration_trailing_commas: enum_declaration_trailing_commas.to_string(),
			export_declaration_trailing_commas: export_declaration_trailing_commas.to_string(),
			import_declaration_trailing_commas: import_declaration_trailing_commas.to_string(),
			object_pattern_trailing_commas: object_pattern_trailing_commas.to_string(),
			object_expression_trailing_commas: object_expression_trailing_commas.to_string(),
			tuple_type_trailing_commas: tuple_type_trailing_commas.to_string(),
			type_literal_trailing_commas: type_literal_trailing_commas.to_string(),
			type_parameters_trailing_commas: type_parameters_trailing_commas.to_string(),
			if_statement_use_braces: if_statement_use_braces.to_string(),
			for_statement_use_braces: for_statement_use_braces.to_string(),
			for_of_statement_use_braces: for_of_statement_use_braces.to_string(),
			for_in_statement_use_braces: for_in_statement_use_braces.to_string(),
			while_statement_use_braces: while_statement_use_braces.to_string(),
			array_expression_prefer_single_line,
			array_pattern_prefer_single_line,
			arguments_prefer_single_line,
			binary_expression_prefer_single_line,
			computed_prefer_single_line,
			conditional_expression_prefer_single_line,
			conditional_type_prefer_single_line,
			decorators_prefer_single_line,
			export_declaration_prefer_single_line,
			for_statement_prefer_single_line,
			import_declaration_prefer_single_line,
			jsx_attributes_prefer_single_line,
			jsx_element_prefer_single_line,
			mapped_type_prefer_single_line,
			member_expression_prefer_single_line,
			object_expression_prefer_single_line,
			object_pattern_prefer_single_line,
			parameters_prefer_single_line,
			parentheses_prefer_single_line,
			tuple_type_prefer_single_line,
			type_literal_prefer_single_line,
			type_parameters_prefer_single_line,
			union_and_intersection_type_prefer_single_line,
			variable_statement_prefer_single_line,
			import_declaration_force_single_line,
			export_declaration_force_single_line,
			export_declaration_force_multi_line: export_declaration_force_multi_line.to_string(),
			import_declaration_force_multi_line: import_declaration_force_multi_line.to_string(),
			binary_expression_space_surrounding_bitwise_and_arithmetic_operator,
			comment_line_force_space_after_slashes,
			construct_signature_space_after_new_keyword,
			constructor_space_before_parentheses,
			constructor_type_space_after_new_keyword,
			do_while_statement_space_after_while_keyword,
			export_declaration_space_surrounding_named_exports,
			for_statement_space_after_for_keyword,
			for_statement_space_after_semi_colons,
			for_in_statement_space_after_for_keyword,
			for_of_statement_space_after_for_keyword,
			function_declaration_space_before_parentheses,
			function_expression_space_before_parentheses,
			function_expression_space_after_function_keyword,
			get_accessor_space_before_parentheses,
			if_statement_space_after_if_keyword,
			import_declaration_space_surrounding_named_imports,
			jsx_expression_container_space_surrounding_expression,
			jsx_self_closing_element_space_before_slash,
			method_space_before_parentheses,
			object_expression_space_surrounding_properties,
			object_pattern_space_surrounding_properties,
			set_accessor_space_before_parentheses,
			space_surrounding_properties,
			tagged_template_space_before_literal,
			type_annotation_space_before_colon,
			type_assertion_space_before_expression,
			type_literal_space_surrounding_properties,
			while_statement_space_after_while_keyword,
			arguments_space_around,
			array_expression_space_around,
			array_pattern_space_around,
			catch_clause_space_around,
			do_while_statement_space_around,
			for_in_statement_space_around,
			for_of_statement_space_around,
			for_statement_space_around,
			if_statement_space_around,
			parameters_space_around,
			paren_expression_space_around,
			switch_statement_space_around,
			tuple_type_space_around,
			while_statement_space_around,
		})
	}

	/// Returns a dictionary representation of the configuration.
	///
	/// :param changed: If :py:obj:`True` only values changed from their defaults are included.
	#[pyo3(signature = (changed: "bool" = true) -> "Dict[str, Any")]
	fn to_dict<'py>(&self, py: Python<'py>, changed: bool) -> PyResult<Bound<'py, PyDict>> {
		let as_dict = PyDict::new(py);
		macro_rules! dict_set_item {
			($key:literal, $value:expr, $default:expr) => {
				if (!changed || $value != $default) {
					as_dict
						.set_item($key, $value)
						.expect("Error setting dict item.");
				}
			};
		}

		dict_set_item!(
			"indent_width",
			self.indent_width,
			RECOMMENDED_GLOBAL_CONFIGURATION.indent_width
		);
		dict_set_item!(
			"line_width",
			self.line_width,
			RECOMMENDED_GLOBAL_CONFIGURATION.line_width
		);
		dict_set_item!(
			"use_tabs",
			self.use_tabs,
			RECOMMENDED_GLOBAL_CONFIGURATION.use_tabs
		);
		dict_set_item!("new_line_kind", &self.new_line_kind, &NEW_LINE_KIND_DEFAULT);
		dict_set_item!("quote_style", &self.quote_style, &QUOTE_STYLE_DEFAULT);
		dict_set_item!("quote_props", &self.quote_props, &QUOTE_PROPS_DEFAULT);
		dict_set_item!("semi_colons", &self.semi_colons, &SEMI_COLONS_DEFAULT);
		dict_set_item!(
			"file_indent_level",
			self.file_indent_level,
			FILE_INDENT_LEVEL_DEFAULT
		);
		dict_set_item!(
			"arrow_function_use_parentheses",
			&self.arrow_function_use_parentheses,
			&ARROW_FUNCTION_USE_PARENTHESES_DEFAULT
		);
		dict_set_item!(
			"binary_expression_line_per_expression",
			self.binary_expression_line_per_expression,
			BINARY_EXPRESSION_LINE_PER_EXPRESSION_DEFAULT
		);
		dict_set_item!(
			"conditional_expression_line_per_expression",
			self.conditional_expression_line_per_expression,
			CONDITIONAL_EXPRESSION_LINE_PER_EXPRESSION_DEFAULT
		);
		dict_set_item!(
			"jsx_quote_style",
			&self.jsx_quote_style,
			&JSX_QUOTE_STYLE_DEFAULT
		);
		dict_set_item!(
			"jsx_multi_line_parens",
			&self.jsx_multi_line_parens,
			&JSX_MULTI_LINE_PARENS_DEFAULT
		);
		dict_set_item!(
			"jsx_force_new_lines_surrounding_content",
			self.jsx_force_new_lines_surrounding_content,
			JSX_FORCE_NEW_LINES_SURROUNDING_CONTENT_DEFAULT
		);
		dict_set_item!(
			"jsx_opening_element_bracket_position",
			&self.jsx_opening_element_bracket_position,
			&JSX_BRACKET_POSITION_DEFAULT
		);
		dict_set_item!(
			"jsx_self_closing_element_bracket_position",
			&self.jsx_self_closing_element_bracket_position,
			&JSX_BRACKET_POSITION_DEFAULT
		);
		dict_set_item!(
			"member_expression_line_per_expression",
			self.member_expression_line_per_expression,
			MEMBER_EXPRESSION_LINE_PER_EXPRESSION_DEFAULT
		);
		dict_set_item!(
			"type_literal_separator_kind_single_line",
			&self.type_literal_separator_kind_single_line,
			&TYPE_LITERAL_SEPARATOR_KIND_DEFAULT
		);
		dict_set_item!(
			"type_literal_separator_kind_multi_line",
			&self.type_literal_separator_kind_multi_line,
			&TYPE_LITERAL_SEPARATOR_KIND_DEFAULT
		);
		dict_set_item!(
			"module_sort_import_declarations",
			&self.module_sort_import_declarations,
			&MODULE_SORT_IMPORT_DECLARATIONS_DEFAULT
		);
		dict_set_item!(
			"module_sort_export_declarations",
			&self.module_sort_export_declarations,
			&MODULE_SORT_EXPORT_DECLARATIONS_DEFAULT
		);
		dict_set_item!(
			"import_declaration_sort_named_imports",
			&self.import_declaration_sort_named_imports,
			&IMPORT_DECLARATION_SORT_NAMED_IMPORTS_DEFAULT
		);
		dict_set_item!(
			"import_declaration_sort_type_only_imports",
			&self.import_declaration_sort_type_only_imports,
			&IMPORT_DECLARATION_SORT_TYPE_ONLY_IMPORTS_DEFAULT
		);
		dict_set_item!(
			"export_declaration_sort_named_exports",
			&self.export_declaration_sort_named_exports,
			&EXPORT_DECLARATION_SORT_NAMED_EXPORTS_DEFAULT
		);
		dict_set_item!(
			"export_declaration_sort_type_only_exports",
			&self.export_declaration_sort_type_only_exports,
			&EXPORT_DECLARATION_SORT_TYPE_ONLY_EXPORTS_DEFAULT
		);
		dict_set_item!(
			"ignore_node_comment_text",
			&self.ignore_node_comment_text,
			&IGNORE_NODE_COMMENT_TEXT_DEFAULT
		);
		dict_set_item!(
			"ignore_file_comment_text",
			&self.ignore_file_comment_text,
			&IGNORE_FILE_COMMENT_TEXT_DEFAULT
		);
		dict_set_item!(
			"arrow_function_brace_position",
			&self.arrow_function_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"class_declaration_brace_position",
			&self.class_declaration_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"class_expression_brace_position",
			&self.class_expression_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"constructor_brace_position",
			&self.constructor_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"do_while_statement_brace_position",
			&self.do_while_statement_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"enum_declaration_brace_position",
			&self.enum_declaration_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"get_accessor_brace_position",
			&self.get_accessor_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"if_statement_brace_position",
			&self.if_statement_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"interface_declaration_brace_position",
			&self.interface_declaration_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"for_statement_brace_position",
			&self.for_statement_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"for_in_statement_brace_position",
			&self.for_in_statement_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"for_of_statement_brace_position",
			&self.for_of_statement_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"function_declaration_brace_position",
			&self.function_declaration_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"function_expression_brace_position",
			&self.function_expression_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"method_brace_position",
			&self.method_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"module_declaration_brace_position",
			&self.module_declaration_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"set_accessor_brace_position",
			&self.set_accessor_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"static_block_brace_position",
			&self.static_block_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"switch_case_brace_position",
			&self.switch_case_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"switch_statement_brace_position",
			&self.switch_statement_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"try_statement_brace_position",
			&self.try_statement_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"while_statement_brace_position",
			&self.while_statement_brace_position,
			&BRACE_POSITION_DEFAULT
		);
		dict_set_item!(
			"arguments_prefer_hanging",
			&self.arguments_prefer_hanging,
			&PREFER_HANGING_GRANULAR_DEFAULT
		);
		dict_set_item!(
			"array_expression_prefer_hanging",
			&self.array_expression_prefer_hanging,
			&PREFER_HANGING_GRANULAR_DEFAULT
		);
		dict_set_item!(
			"array_pattern_prefer_hanging",
			self.array_pattern_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"do_while_statement_prefer_hanging",
			self.do_while_statement_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"export_declaration_prefer_hanging",
			self.export_declaration_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"extends_clause_prefer_hanging",
			self.extends_clause_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"for_statement_prefer_hanging",
			self.for_statement_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"for_in_statement_prefer_hanging",
			self.for_in_statement_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"for_of_statement_prefer_hanging",
			self.for_of_statement_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"if_statement_prefer_hanging",
			self.if_statement_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"implements_clause_prefer_hanging",
			self.implements_clause_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"import_declaration_prefer_hanging",
			self.import_declaration_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"jsx_attributes_prefer_hanging",
			self.jsx_attributes_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"object_expression_prefer_hanging",
			self.object_expression_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"object_pattern_prefer_hanging",
			self.object_pattern_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"parameters_prefer_hanging",
			&self.parameters_prefer_hanging,
			&PREFER_HANGING_GRANULAR_DEFAULT
		);
		dict_set_item!(
			"sequence_expression_prefer_hanging",
			self.sequence_expression_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"switch_statement_prefer_hanging",
			self.switch_statement_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"tuple_type_prefer_hanging",
			&self.tuple_type_prefer_hanging,
			&PREFER_HANGING_GRANULAR_DEFAULT
		);
		dict_set_item!(
			"type_literal_prefer_hanging",
			self.type_literal_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"type_parameters_prefer_hanging",
			&self.type_parameters_prefer_hanging,
			&PREFER_HANGING_GRANULAR_DEFAULT
		);
		dict_set_item!(
			"union_and_intersection_type_prefer_hanging",
			self.union_and_intersection_type_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"variable_statement_prefer_hanging",
			self.variable_statement_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"while_statement_prefer_hanging",
			self.while_statement_prefer_hanging,
			PREFER_HANGING_DEFAULT
		);
		dict_set_item!(
			"enum_declaration_member_spacing",
			&self.enum_declaration_member_spacing,
			&ENUM_DECLARATION_MEMBER_SPACING_DEFAULT
		);
		dict_set_item!(
			"if_statement_next_control_flow_position",
			&self.if_statement_next_control_flow_position,
			&NEXT_CONTROL_FLOW_POSITION_DEFAULT
		);
		dict_set_item!(
			"try_statement_next_control_flow_position",
			&self.try_statement_next_control_flow_position,
			&NEXT_CONTROL_FLOW_POSITION_DEFAULT
		);
		dict_set_item!(
			"do_while_statement_next_control_flow_position",
			&self.do_while_statement_next_control_flow_position,
			&NEXT_CONTROL_FLOW_POSITION_DEFAULT
		);
		dict_set_item!(
			"binary_expression_operator_position",
			&self.binary_expression_operator_position,
			&OPERATOR_POSITION_DEFAULT
		);
		dict_set_item!(
			"conditional_expression_operator_position",
			&self.conditional_expression_operator_position,
			&OPERATOR_POSITION_DEFAULT
		);
		dict_set_item!(
			"conditional_type_operator_position",
			&self.conditional_type_operator_position,
			&OPERATOR_POSITION_DEFAULT
		);
		dict_set_item!(
			"if_statement_single_body_position",
			&self.if_statement_single_body_position,
			&SINGLE_BODY_POSITION_DEFAULT
		);
		dict_set_item!(
			"for_statement_single_body_position",
			&self.for_statement_single_body_position,
			&SINGLE_BODY_POSITION_DEFAULT
		);
		dict_set_item!(
			"for_in_statement_single_body_position",
			&self.for_in_statement_single_body_position,
			&SINGLE_BODY_POSITION_DEFAULT
		);
		dict_set_item!(
			"for_of_statement_single_body_position",
			&self.for_of_statement_single_body_position,
			&SINGLE_BODY_POSITION_DEFAULT
		);
		dict_set_item!(
			"while_statement_single_body_position",
			&self.while_statement_single_body_position,
			&SINGLE_BODY_POSITION_DEFAULT
		);
		dict_set_item!(
			"arguments_trailing_commas",
			&self.arguments_trailing_commas,
			&TRAILING_COMMAS_DEFAULT
		);
		dict_set_item!(
			"parameters_trailing_commas",
			&self.parameters_trailing_commas,
			&TRAILING_COMMAS_DEFAULT
		);
		dict_set_item!(
			"array_expression_trailing_commas",
			&self.array_expression_trailing_commas,
			&TRAILING_COMMAS_DEFAULT
		);
		dict_set_item!(
			"array_pattern_trailing_commas",
			&self.array_pattern_trailing_commas,
			&TRAILING_COMMAS_DEFAULT
		);
		dict_set_item!(
			"enum_declaration_trailing_commas",
			&self.enum_declaration_trailing_commas,
			&TRAILING_COMMAS_DEFAULT
		);
		dict_set_item!(
			"export_declaration_trailing_commas",
			&self.export_declaration_trailing_commas,
			&TRAILING_COMMAS_DEFAULT
		);
		dict_set_item!(
			"import_declaration_trailing_commas",
			&self.import_declaration_trailing_commas,
			&TRAILING_COMMAS_DEFAULT
		);
		dict_set_item!(
			"object_pattern_trailing_commas",
			&self.object_pattern_trailing_commas,
			&TRAILING_COMMAS_DEFAULT
		);
		dict_set_item!(
			"object_expression_trailing_commas",
			&self.object_expression_trailing_commas,
			&TRAILING_COMMAS_DEFAULT
		);
		dict_set_item!(
			"tuple_type_trailing_commas",
			&self.tuple_type_trailing_commas,
			&TRAILING_COMMAS_DEFAULT
		);
		dict_set_item!(
			"type_literal_trailing_commas",
			&self.type_literal_trailing_commas,
			&TRAILING_COMMAS_DEFAULT
		);
		dict_set_item!(
			"type_parameters_trailing_commas",
			&self.type_parameters_trailing_commas,
			&TRAILING_COMMAS_DEFAULT
		);
		dict_set_item!(
			"if_statement_use_braces",
			&self.if_statement_use_braces,
			&USE_BRACES_DEFAULT
		);
		dict_set_item!(
			"for_statement_use_braces",
			&self.for_statement_use_braces,
			&USE_BRACES_DEFAULT
		);
		dict_set_item!(
			"for_of_statement_use_braces",
			&self.for_of_statement_use_braces,
			&USE_BRACES_DEFAULT
		);
		dict_set_item!(
			"for_in_statement_use_braces",
			&self.for_in_statement_use_braces,
			&USE_BRACES_DEFAULT
		);
		dict_set_item!(
			"while_statement_use_braces",
			&self.while_statement_use_braces,
			&USE_BRACES_DEFAULT
		);
		dict_set_item!(
			"array_expression_prefer_single_line",
			self.array_expression_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"array_pattern_prefer_single_line",
			self.array_pattern_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"arguments_prefer_single_line",
			self.arguments_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"binary_expression_prefer_single_line",
			self.binary_expression_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"computed_prefer_single_line",
			self.computed_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"conditional_expression_prefer_single_line",
			self.conditional_expression_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"conditional_type_prefer_single_line",
			self.conditional_type_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"decorators_prefer_single_line",
			self.decorators_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"export_declaration_prefer_single_line",
			self.export_declaration_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"for_statement_prefer_single_line",
			self.for_statement_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"import_declaration_prefer_single_line",
			self.import_declaration_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"jsx_attributes_prefer_single_line",
			self.jsx_attributes_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"jsx_element_prefer_single_line",
			self.jsx_element_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"mapped_type_prefer_single_line",
			self.mapped_type_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"member_expression_prefer_single_line",
			self.member_expression_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"object_expression_prefer_single_line",
			self.object_expression_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"object_pattern_prefer_single_line",
			self.object_pattern_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"parameters_prefer_single_line",
			self.parameters_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"parentheses_prefer_single_line",
			self.parentheses_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"tuple_type_prefer_single_line",
			self.tuple_type_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"type_literal_prefer_single_line",
			self.type_literal_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"type_parameters_prefer_single_line",
			self.type_parameters_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"union_and_intersection_type_prefer_single_line",
			self.union_and_intersection_type_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"variable_statement_prefer_single_line",
			self.variable_statement_prefer_single_line,
			PREFER_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"import_declaration_force_single_line",
			self.import_declaration_force_single_line,
			IMPORT_DECLARATION_FORCE_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"export_declaration_force_single_line",
			self.export_declaration_force_single_line,
			EXPORT_DECLARATION_FORCE_SINGLE_LINE_DEFAULT
		);
		dict_set_item!(
			"export_declaration_force_multi_line",
			&self.export_declaration_force_multi_line,
			&EXPORT_DECLARATION_FORCE_MULTI_LINE_DEFAULT
		);
		dict_set_item!(
			"import_declaration_force_multi_line",
			&self.import_declaration_force_multi_line,
			&IMPORT_DECLARATION_FORCE_MULTI_LINE_DEFAULT
		);
		dict_set_item!(
			"binary_expression_space_surrounding_bitwise_and_arithmetic_operator",
			self.binary_expression_space_surrounding_bitwise_and_arithmetic_operator,
			BINARY_EXPRESSION_SPACE_SURROUNDING_BITWISE_AND_ARITHMETIC_OPERATOR_DEFAULT
		);
		dict_set_item!(
			"comment_line_force_space_after_slashes",
			self.comment_line_force_space_after_slashes,
			COMMENT_LINE_FORCE_SPACE_AFTER_SLASHES_DEFAULT
		);
		dict_set_item!(
			"construct_signature_space_after_new_keyword",
			self.construct_signature_space_after_new_keyword,
			CONSTRUCT_SIGNATURE_SPACE_AFTER_NEW_KEYWORD_DEFAULT
		);
		dict_set_item!(
			"constructor_space_before_parentheses",
			self.constructor_space_before_parentheses,
			CONSTRUCTOR_SPACE_BEFORE_PARENTHESES_DEFAULT
		);
		dict_set_item!(
			"constructor_type_space_after_new_keyword",
			self.constructor_type_space_after_new_keyword,
			CONSTRUCTOR_TYPE_SPACE_AFTER_NEW_KEYWORD_DEFAULT
		);
		dict_set_item!(
			"do_while_statement_space_after_while_keyword",
			self.do_while_statement_space_after_while_keyword,
			DO_WHILE_STATEMENT_SPACE_AFTER_WHILE_KEYWORD_DEFAULT
		);
		dict_set_item!(
			"export_declaration_space_surrounding_named_exports",
			self.export_declaration_space_surrounding_named_exports,
			EXPORT_DECLARATION_SPACE_SURROUNDING_NAMED_EXPORTS_DEFAULT
		);
		dict_set_item!(
			"for_statement_space_after_for_keyword",
			self.for_statement_space_after_for_keyword,
			FOR_STATEMENT_SPACE_AFTER_FOR_KEYWORD_DEFAULT
		);
		dict_set_item!(
			"for_statement_space_after_semi_colons",
			self.for_statement_space_after_semi_colons,
			FOR_STATEMENT_SPACE_AFTER_SEMI_COLONS_DEFAULT
		);
		dict_set_item!(
			"for_in_statement_space_after_for_keyword",
			self.for_in_statement_space_after_for_keyword,
			FOR_IN_STATEMENT_SPACE_AFTER_FOR_KEYWORD_DEFAULT
		);
		dict_set_item!(
			"for_of_statement_space_after_for_keyword",
			self.for_of_statement_space_after_for_keyword,
			FOR_OF_STATEMENT_SPACE_AFTER_FOR_KEYWORD_DEFAULT
		);
		dict_set_item!(
			"function_declaration_space_before_parentheses",
			self.function_declaration_space_before_parentheses,
			FUNCTION_DECLARATION_SPACE_BEFORE_PARENTHESES_DEFAULT
		);
		dict_set_item!(
			"function_expression_space_before_parentheses",
			self.function_expression_space_before_parentheses,
			FUNCTION_EXPRESSION_SPACE_BEFORE_PARENTHESES_DEFAULT
		);
		dict_set_item!(
			"function_expression_space_after_function_keyword",
			self.function_expression_space_after_function_keyword,
			FUNCTION_EXPRESSION_SPACE_AFTER_FUNCTION_KEYWORD_DEFAULT
		);
		dict_set_item!(
			"get_accessor_space_before_parentheses",
			self.get_accessor_space_before_parentheses,
			GET_ACCESSOR_SPACE_BEFORE_PARENTHESES_DEFAULT
		);
		dict_set_item!(
			"if_statement_space_after_if_keyword",
			self.if_statement_space_after_if_keyword,
			IF_STATEMENT_SPACE_AFTER_IF_KEYWORD_DEFAULT
		);
		dict_set_item!(
			"import_declaration_space_surrounding_named_imports",
			self.import_declaration_space_surrounding_named_imports,
			IMPORT_DECLARATION_SPACE_SURROUNDING_NAMED_IMPORTS_DEFAULT
		);
		dict_set_item!(
			"jsx_expression_container_space_surrounding_expression",
			self.jsx_expression_container_space_surrounding_expression,
			JSX_EXPRESSION_CONTAINER_SPACE_SURROUNDING_EXPRESSION_DEFAULT
		);
		dict_set_item!(
			"jsx_self_closing_element_space_before_slash",
			self.jsx_self_closing_element_space_before_slash,
			JSX_SELF_CLOSING_ELEMENT_SPACE_BEFORE_SLASH_DEFAULT
		);
		dict_set_item!(
			"method_space_before_parentheses",
			self.method_space_before_parentheses,
			SPACE_SURROUNDING_PROPERTIES_DEFAULT
		);
		dict_set_item!(
			"object_expression_space_surrounding_properties",
			self.object_expression_space_surrounding_properties,
			SPACE_SURROUNDING_PROPERTIES_DEFAULT
		);
		dict_set_item!(
			"object_pattern_space_surrounding_properties",
			self.object_pattern_space_surrounding_properties,
			SPACE_SURROUNDING_PROPERTIES_DEFAULT
		);
		dict_set_item!(
			"set_accessor_space_before_parentheses",
			self.set_accessor_space_before_parentheses,
			SET_ACCESSOR_SPACE_BEFORE_PARENTHESES_DEFAULT
		);
		dict_set_item!(
			"space_surrounding_properties",
			self.space_surrounding_properties,
			SPACE_SURROUNDING_PROPERTIES_DEFAULT
		);
		dict_set_item!(
			"tagged_template_space_before_literal",
			self.tagged_template_space_before_literal,
			TAGGED_TEMPLATE_SPACE_BEFORE_LITERAL_DEFAULT
		);
		dict_set_item!(
			"type_annotation_space_before_colon",
			self.type_annotation_space_before_colon,
			TYPE_ANNOTATION_SPACE_BEFORE_COLON_DEFAULT
		);
		dict_set_item!(
			"type_assertion_space_before_expression",
			self.type_assertion_space_before_expression,
			TYPE_ASSERTION_SPACE_BEFORE_EXPRESSION_DEFAULT
		);
		dict_set_item!(
			"type_literal_space_surrounding_properties",
			self.type_literal_space_surrounding_properties,
			SPACE_SURROUNDING_PROPERTIES_DEFAULT
		);
		dict_set_item!(
			"while_statement_space_after_while_keyword",
			self.while_statement_space_after_while_keyword,
			WHILE_STATEMENT_SPACE_AFTER_WHILE_KEYWORD_DEFAULT
		);
		dict_set_item!(
			"arguments_space_around",
			self.arguments_space_around,
			SPACE_AROUND_DEFAULT
		);
		dict_set_item!(
			"array_expression_space_around",
			self.array_expression_space_around,
			SPACE_AROUND_DEFAULT
		);
		dict_set_item!(
			"array_pattern_space_around",
			self.array_pattern_space_around,
			SPACE_AROUND_DEFAULT
		);
		dict_set_item!(
			"catch_clause_space_around",
			self.catch_clause_space_around,
			SPACE_AROUND_DEFAULT
		);
		dict_set_item!(
			"do_while_statement_space_around",
			self.do_while_statement_space_around,
			SPACE_AROUND_DEFAULT
		);
		dict_set_item!(
			"for_in_statement_space_around",
			self.for_in_statement_space_around,
			SPACE_AROUND_DEFAULT
		);
		dict_set_item!(
			"for_of_statement_space_around",
			self.for_of_statement_space_around,
			SPACE_AROUND_DEFAULT
		);
		dict_set_item!(
			"for_statement_space_around",
			self.for_statement_space_around,
			SPACE_AROUND_DEFAULT
		);
		dict_set_item!(
			"if_statement_space_around",
			self.if_statement_space_around,
			SPACE_AROUND_DEFAULT
		);
		dict_set_item!(
			"parameters_space_around",
			self.parameters_space_around,
			SPACE_AROUND_DEFAULT
		);
		dict_set_item!(
			"paren_expression_space_around",
			self.paren_expression_space_around,
			SPACE_AROUND_DEFAULT
		);
		dict_set_item!(
			"switch_statement_space_around",
			self.switch_statement_space_around,
			SPACE_AROUND_DEFAULT
		);
		dict_set_item!(
			"tuple_type_space_around",
			self.tuple_type_space_around,
			SPACE_AROUND_DEFAULT
		);
		dict_set_item!(
			"while_statement_space_around",
			self.while_statement_space_around,
			SPACE_AROUND_DEFAULT
		);
		Ok(as_dict)
	}

	fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
		self.to_dict(py, false)?.try_iter()
	}

	fn __getitem__<'py>(&self, py: Python<'py>, value: &str) -> PyResult<Bound<'py, PyAny>> {
		self.to_dict(py, false)?.as_any().get_item(value)
	}

	fn __contains__<'py>(&self, py: Python<'py>, key: &str) -> PyResult<bool> {
		self.to_dict(py, false)?.contains(key)
	}

	fn __len__<'py>(&self, py: Python<'py>) -> PyResult<usize> {
		Ok(self.to_dict(py, false)?.len())
	}

	fn keys<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
		Ok(self.to_dict(py, false)?.keys())
	}

	fn values<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
		Ok(self.to_dict(py, false)?.values())
	}

	fn items<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
		Ok(self.to_dict(py, false)?.items())
	}
}

impl<'py> IntoPyDict<'py> for PyConfiguration {
	fn into_py_dict(self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
		self.to_dict(py, false)
	}
}
