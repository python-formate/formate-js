use pyo3::prelude::*;
/// Formate plugin for reformatting JavaScript and TypeScript files with dprint.
mod configuration_builder;
use crate::configuration_builder::PyConfigurationBuilder;

#[pymodule]
fn _formate_js(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_class::<PyConfigurationBuilder>().unwrap();

	Ok(())
}
