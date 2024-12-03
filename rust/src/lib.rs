use pyo3::prelude::*;
use pyo3::types::{PyList, PyString};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn split_string(text: &str) -> PyResult<Vec<String>> {
    Ok(text.split_whitespace()
        .map(String::from)
        .collect())
}

/// A Python module implemented in Rust.
#[pymodule]
fn fasttok(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(split_string, m)?)?;
    Ok(())
}
