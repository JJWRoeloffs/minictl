pub mod parser;

#[cfg(feature = "python")]
mod pythonutils;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn minictl(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
