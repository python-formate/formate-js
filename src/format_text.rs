use crate::configuration::PyConfiguration;
use dprint_plugin_typescript::{FormatTextOptions, format_text};
use pyo3::{exceptions::PyValueError, prelude::*};
use std::path::PathBuf;

#[pyclass(name = "FormatTextOptions", module = "_formate_js")]
// #[repr(transparent)]
#[derive(Clone)]
// A wrapper around a [`FormatTextOptions`] that can be converted to and from python with `pyo3`.
/// Options for reformatting a file.
pub struct PyFormatTextOptions {
	pub path: PathBuf,
	pub extension: Option<String>,
	pub text: String,
	pub config: PyConfiguration,
	// TODO: pub external_formatter: Option<ExternalFormatter>,
}

// impl From<PyFormatTextOptions> for FormatTextOptions<'_> {
// 	fn from(value: PyFormatTextOptions) -> Self {
// 		FormatTextOptions{
// 			path: &value.path.clone(),
//             extension: match value.extension {
// 				Some(extension) => Some(&extension.clone()),
// 				None => None,
// 			},
//             text: value.text,
//             config: &value.config.0.clone(),
//             external_formatter: None,
// 		}
// 	}
// }

#[pymethods]
impl PyFormatTextOptions {
	#[new]
	pub fn __init__(
		path: PathBuf,
		extension: Option<String>,
		text: String,
		config: PyConfiguration,
		// TODO: external_formatter: Option<&ExternalFormatter>,
	) -> PyResult<Self> {
		Ok(PyFormatTextOptions {
			path,
			extension,
			text,
			config,
			// TODO: external_formatter,
		})
	}
}

#[pyfunction(name = "format_text")]
/// Formats a file.
///
/// Returns the file text or an error when it failed to parse.
pub fn format_text_py(options: PyFormatTextOptions) -> PyResult<Option<String>> {
	match format_text(FormatTextOptions {
		path: &options.path,
		extension: options.extension.as_deref(),
		text: options.text,
		config: &options.config.into(),
		external_formatter: None,
		// TODO: external_formatter: options.external_formatter,
	}) {
		Ok(formatted_text) => Ok(formatted_text),
		Err(error) => Err(PyValueError::new_err(error.to_string())),
	}
}
