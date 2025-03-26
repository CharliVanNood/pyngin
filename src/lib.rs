use pyo3::prelude::*;

mod window_handler;

#[pyclass]
struct Window {
    name: String,
}

#[pymethods]
impl Window {
    #[new]
    fn new(name: Option<String>) -> Self {
        let name = name.unwrap_or_else(|| "PYNGIN".to_string());
        window_handler::create_window(&name);
        Window { name }
    }

    fn render(&self) {
        println!("Rendering window: {}", self.name);
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

#[pyfunction]
fn version() -> PyResult<String> {
    Ok("0.0.1".to_string())
}

#[pyfunction]
fn window(a: Option<String>) -> PyResult<Window> {
    let name = a.unwrap_or_else(|| "PYNGIN".to_string());
    let window = Window::new(Some(name));
    Ok(window)
}

#[pymodule]
fn pyngin(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(window, m)?)?;
    Ok(())
}