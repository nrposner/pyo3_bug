// src/lib.rs
use numpy::PyArray;
use pyo3::prelude::*;

/// A dummy function to force the numpy dependency to compile.
#[pyfunction]
fn dummy() -> usize {
    42
}

#[pymodule]
fn minimal_pyo3_bug(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(pyo3::wrap_pyfunction!(dummy, module)?)?;
    Ok(())
}
