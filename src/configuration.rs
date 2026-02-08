use dprint_core::configuration::NewLineKind;
use dprint_plugin_typescript::configuration::Configuration;
use dprint_plugin_typescript::configuration::{
	BracePosition, ForceMultiLine, JsxMultiLineParens, JsxQuoteStyle, MemberSpacing,
	NamedTypeImportsExportsOrder, NextControlFlowPosition, OperatorPosition, PreferHanging,
	QuoteProps, QuoteStyle, SameOrNextLinePosition, SemiColonOrComma, SemiColons, SortOrder,
	TrailingCommas, UseBraces, UseParentheses,
};
use pyo3::prelude::*;
use std::str::FromStr;

#[pyclass(name = "Configuration", module = "formate_js")]
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
// TODO: expose all the fields as properties?

#[pymethods]
impl PyConfiguration {
	#![allow(clippy::too_many_arguments)]
	#[pyo3(signature = (
		indent_width: "int" = 2,
		line_width: "int" = 120,
		use_tabs: "bool" = false,
		new_line_kind: "str" = "lf",
		quote_style: "str" = "alwaysDouble",
		quote_props: "str" = "preserve",
		semi_colons: "str" = "prefer",
		file_indent_level: "int" = 0,
		arrow_function_use_parentheses: "str" = "maintain",
		binary_expression_line_per_expression: "bool" = false,
		conditional_expression_line_per_expression: "bool" = true,
		jsx_quote_style: "str" = "preferDouble",
		jsx_multi_line_parens: "str" = "prefer",
		jsx_force_new_lines_surrounding_content: "bool" = false,
		jsx_opening_element_bracket_position: "str" = "nextLine",
		jsx_self_closing_element_bracket_position: "str" = "nextLine",
		member_expression_line_per_expression: "bool" = false,
		type_literal_separator_kind_single_line: "str" = "semiColon",
		type_literal_separator_kind_multi_line: "str" = "semiColon",
		module_sort_import_declarations: "str" = "caseInsensitive",
		module_sort_export_declarations: "str" = "caseInsensitive",
		import_declaration_sort_named_imports: "str" = "caseInsensitive",
		import_declaration_sort_type_only_imports: "str" = "none",
		export_declaration_sort_named_exports: "str" = "caseInsensitive",
		export_declaration_sort_type_only_exports: "str" = "none",
		ignore_node_comment_text: "str" = "dprint-ignore",
		ignore_file_comment_text: "str" = "dprint-ignore-file",
		arrow_function_brace_position: "str" = "sameLineUnlessHanging",
		class_declaration_brace_position: "str" = "sameLineUnlessHanging",
		class_expression_brace_position: "str" = "sameLineUnlessHanging",
		constructor_brace_position: "str" = "sameLineUnlessHanging",
		do_while_statement_brace_position: "str" = "sameLineUnlessHanging",
		enum_declaration_brace_position: "str" = "sameLineUnlessHanging",
		get_accessor_brace_position: "str" = "sameLineUnlessHanging",
		if_statement_brace_position: "str" = "sameLineUnlessHanging",
		interface_declaration_brace_position: "str" = "sameLineUnlessHanging",
		for_statement_brace_position: "str" = "sameLineUnlessHanging",
		for_in_statement_brace_position: "str" = "sameLineUnlessHanging",
		for_of_statement_brace_position: "str" = "sameLineUnlessHanging",
		function_declaration_brace_position: "str" = "sameLineUnlessHanging",
		function_expression_brace_position: "str" = "sameLineUnlessHanging",
		method_brace_position: "str" = "sameLineUnlessHanging",
		module_declaration_brace_position: "str" = "sameLineUnlessHanging",
		set_accessor_brace_position: "str" = "sameLineUnlessHanging",
		static_block_brace_position: "str" = "sameLineUnlessHanging",
		switch_case_brace_position: "str" = "sameLineUnlessHanging",
		switch_statement_brace_position: "str" = "sameLineUnlessHanging",
		try_statement_brace_position: "str" = "sameLineUnlessHanging",
		while_statement_brace_position: "str" = "sameLineUnlessHanging",
		arguments_prefer_hanging: "str" = "never",
		array_expression_prefer_hanging: "str" = "never",
		array_pattern_prefer_hanging: "bool" = false,
		do_while_statement_prefer_hanging: "bool" = false,
		export_declaration_prefer_hanging: "bool" = false,
		extends_clause_prefer_hanging: "bool" = false,
		for_statement_prefer_hanging: "bool" = false,
		for_in_statement_prefer_hanging: "bool" = false,
		for_of_statement_prefer_hanging: "bool" = false,
		if_statement_prefer_hanging: "bool" = false,
		implements_clause_prefer_hanging: "bool" = false,
		import_declaration_prefer_hanging: "bool" = false,
		jsx_attributes_prefer_hanging: "bool" = false,
		object_expression_prefer_hanging: "bool" = false,
		object_pattern_prefer_hanging: "bool" = false,
		parameters_prefer_hanging: "str" = "never",
		sequence_expression_prefer_hanging: "bool" = false,
		switch_statement_prefer_hanging: "bool" = false,
		tuple_type_prefer_hanging: "str" = "never",
		type_literal_prefer_hanging: "bool" = false,
		type_parameters_prefer_hanging: "str" = "never",
		union_and_intersection_type_prefer_hanging: "bool" = false,
		variable_statement_prefer_hanging: "bool" = false,
		while_statement_prefer_hanging: "bool" = false,
		enum_declaration_member_spacing: "str" = "maintain",
		if_statement_next_control_flow_position: "str" = "sameLine",
		try_statement_next_control_flow_position: "str" = "sameLine",
		do_while_statement_next_control_flow_position: "str" = "sameLine",
		binary_expression_operator_position: "str" = "nextLine",
		conditional_expression_operator_position: "str" = "nextLine",
		conditional_type_operator_position: "str" = "nextLine",
		if_statement_single_body_position: "str" = "maintain",
		for_statement_single_body_position: "str" = "maintain",
		for_in_statement_single_body_position: "str" = "maintain",
		for_of_statement_single_body_position: "str" = "maintain",
		while_statement_single_body_position: "str" = "maintain",
		arguments_trailing_commas: "str" = "onlyMultiLine",
		parameters_trailing_commas: "str" = "onlyMultiLine",
		array_expression_trailing_commas: "str" = "onlyMultiLine",
		array_pattern_trailing_commas: "str" = "onlyMultiLine",
		enum_declaration_trailing_commas: "str" = "onlyMultiLine",
		export_declaration_trailing_commas: "str" = "onlyMultiLine",
		import_declaration_trailing_commas: "str" = "onlyMultiLine",
		object_pattern_trailing_commas: "str" = "onlyMultiLine",
		object_expression_trailing_commas: "str" = "onlyMultiLine",
		tuple_type_trailing_commas: "str" = "onlyMultiLine",
		type_literal_trailing_commas: "str" = "onlyMultiLine",
		type_parameters_trailing_commas: "str" = "onlyMultiLine",
		if_statement_use_braces: "str" = "whenNotSingleLine",
		for_statement_use_braces: "str" = "whenNotSingleLine",
		for_of_statement_use_braces: "str" = "whenNotSingleLine",
		for_in_statement_use_braces: "str" = "whenNotSingleLine",
		while_statement_use_braces: "str" = "whenNotSingleLine",
		array_expression_prefer_single_line: "bool" = false,
		array_pattern_prefer_single_line: "bool" = false,
		arguments_prefer_single_line: "bool" = false,
		binary_expression_prefer_single_line: "bool" = false,
		computed_prefer_single_line: "bool" = false,
		conditional_expression_prefer_single_line: "bool" = false,
		conditional_type_prefer_single_line: "bool" = false,
		decorators_prefer_single_line: "bool" = false,
		export_declaration_prefer_single_line: "bool" = true,
		for_statement_prefer_single_line: "bool" = false,
		import_declaration_prefer_single_line: "bool" = true,
		jsx_attributes_prefer_single_line: "bool" = false,
		jsx_element_prefer_single_line: "bool" = false,
		mapped_type_prefer_single_line: "bool" = false,
		member_expression_prefer_single_line: "bool" = false,
		object_expression_prefer_single_line: "bool" = false,
		object_pattern_prefer_single_line: "bool" = false,
		parameters_prefer_single_line: "bool" = false,
		parentheses_prefer_single_line: "bool" = false,
		tuple_type_prefer_single_line: "bool" = false,
		type_literal_prefer_single_line: "bool" = false,
		type_parameters_prefer_single_line: "bool" = false,
		union_and_intersection_type_prefer_single_line: "bool" = false,
		variable_statement_prefer_single_line: "bool" = false,
		import_declaration_force_single_line: "bool" = false,
		export_declaration_force_single_line: "bool" = false,
		export_declaration_force_multi_line: "str" = "never",
		import_declaration_force_multi_line: "str" = "never",
		binary_expression_space_surrounding_bitwise_and_arithmetic_operator: "bool" = true,
		comment_line_force_space_after_slashes: "bool" = true,
		construct_signature_space_after_new_keyword: "bool" = false,
		constructor_space_before_parentheses: "bool" = false,
		constructor_type_space_after_new_keyword: "bool" = false,
		do_while_statement_space_after_while_keyword: "bool" = true,
		export_declaration_space_surrounding_named_exports: "bool" = true,
		for_statement_space_after_for_keyword: "bool" = true,
		for_statement_space_after_semi_colons: "bool" = true,
		for_in_statement_space_after_for_keyword: "bool" = true,
		for_of_statement_space_after_for_keyword: "bool" = true,
		function_declaration_space_before_parentheses: "bool" = false,
		function_expression_space_before_parentheses: "bool" = false,
		function_expression_space_after_function_keyword: "bool" = false,
		get_accessor_space_before_parentheses: "bool" = false,
		if_statement_space_after_if_keyword: "bool" = true,
		import_declaration_space_surrounding_named_imports: "bool" = true,
		jsx_expression_container_space_surrounding_expression: "bool" = false,
		jsx_self_closing_element_space_before_slash: "bool" = true,
		method_space_before_parentheses: "bool" = false,
		object_expression_space_surrounding_properties: "bool" = true,
		object_pattern_space_surrounding_properties: "bool" = true,
		set_accessor_space_before_parentheses: "bool" = false,
		space_surrounding_properties: "bool" = true,
		tagged_template_space_before_literal: "bool" = false,
		type_annotation_space_before_colon: "bool" = false,
		type_assertion_space_before_expression: "bool" = true,
		type_literal_space_surrounding_properties: "bool" = true,
		while_statement_space_after_while_keyword: "bool" = true,
		arguments_space_around: "bool" = false,
		array_expression_space_around: "bool" = false,
		array_pattern_space_around: "bool" = false,
		catch_clause_space_around: "bool" = false,
		do_while_statement_space_around: "bool" = false,
		for_in_statement_space_around: "bool" = false,
		for_of_statement_space_around: "bool" = false,
		for_statement_space_around: "bool" = false,
		if_statement_space_around: "bool" = false,
		parameters_space_around: "bool" = false,
		paren_expression_space_around: "bool" = false,
		switch_statement_space_around: "bool" = false,
		tuple_type_space_around: "bool" = false,
		while_statement_space_around: "bool" = false,
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
}
