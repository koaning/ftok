use pyo3::types::PyList;
use pyo3::prelude::*;


/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// Split a string using rust!
#[pyfunction]
fn split_string(text: &str) -> PyResult<Vec<String>> {
    Ok(text.split_whitespace()
        .map(String::from)
        .collect())
}


use pyo3::types::{PyListMethods};

/// Split each string in a Python list on whitespace
/// Returns a list of lists, where each inner list contains the split words
#[pyfunction]
fn split_strings(py: Python<'_>, strings: PyObject) -> PyResult<Vec<Vec<String>>> {
    let strings = strings.downcast_bound::<PyList>(py)?;
    let mut result = Vec::new();
    
    // Get list length and iterate through indices
    for i in 0..strings.len() {
        // Get item at index and convert to Rust String
        let rust_str: String = strings.get_item(i)?.extract()?;
        
        // Split on whitespace and collect into Vec<String>
        let split_words: Vec<String> = rust_str
            .split_whitespace()
            .map(String::from)
            .collect();
            
        result.push(split_words);
    }
    
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn fasttok(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(split_string, m)?)?;
    m.add_function(wrap_pyfunction!(split_strings, m)?)?;
    Ok(())
}

