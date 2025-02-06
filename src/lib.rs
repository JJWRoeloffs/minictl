#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Basic function used to test if everything is installed correctly
#[cfg(feature = "python")]
#[pyfunction]
fn hello_world() -> PyResult<String> {
    Ok(String::from("Hello World"))
}

#[cfg(feature = "python")]
#[pymodule]
fn minictl(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    Ok(())
}
