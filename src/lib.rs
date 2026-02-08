use pyo3::prelude::*;
mod configuration;
/// Formate plugin for reformatting JavaScript and TypeScript files with dprint.
mod configuration_builder;
mod format_text;
use crate::configuration::PyConfiguration;
use crate::configuration_builder::PyConfigurationBuilder;
use crate::format_text::{PyFormatTextOptions, format_text_py};

#[pymodule]
fn _formate_js(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_class::<PyConfigurationBuilder>().unwrap();
	m.add_class::<PyConfiguration>().unwrap();
	m.add_class::<PyFormatTextOptions>().unwrap();

	let format_text = wrap_pyfunction!(format_text_py, m)?;
	format_text.setattr("__module__", "formate_js")?;
	m.add_function(format_text).unwrap();

	Ok(())
}
