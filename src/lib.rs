pub mod formulas;
pub mod modelcheckers;
pub mod models;

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
    m.add_class::<formulas::ctl_python::PyCTLFormula>()?;
    m.add_class::<formulas::ltl_python::PyLTLFormula>()?;
    m.add_class::<models::models_python::PyState>()?;
    m.add_class::<models::models_python::PyModel>()?;
    m.add_class::<modelcheckers::ctl_checker_python::PyCTLChecker>()?;
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    Ok(())
}
