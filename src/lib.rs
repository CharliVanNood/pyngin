use pyo3::prelude::*;

#[pyfunction]
fn version() -> PyResult<String> {
    Ok("0.0.1".to_string())
}

#[pyfunction]
fn window() -> PyResult<String> {
    Ok("unimplemented".to_string())
}

#[pymodule]
fn pyngin(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(window, m)?)?;
    Ok(())
}