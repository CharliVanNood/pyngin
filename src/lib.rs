use pyo3::prelude::*;

mod window_handler;

#[pyfunction]
fn version() -> PyResult<String> {
    Ok("0.0.1".to_string())
}

#[pyfunction]
fn window(a: Option<String>) -> PyResult<String> {
    let name = a.unwrap_or_else(|| "PYNGIN".to_string());

    window_handler::create_window(&name);

    Ok(name)
}

#[pymodule]
fn pyngin(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(window, m)?)?;
    Ok(())
}