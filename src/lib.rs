pub mod formulas;
pub mod models;
pub mod satisfies;

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Basic function used to test if everything is installed correctly
#[cfg(feature = "python")]
#[pyfunction]
fn hello_world() -> PyResult<String> {
    Ok(String::from("Hello World"))
}

// See the following issue as to why this is needed:
// * https://github.com/PyO3/pyo3/issues/759
#[cfg(feature = "python")]
fn add_submodule(parent: &Bound<'_, PyModule>, child: &Bound<'_, PyModule>) -> PyResult<()> {
    parent.add_submodule(child)?;
    parent
        .py()
        .import("sys")?
        .getattr("modules")?
        // parent.name()? doesn't work, as that would be `minictl.minicti.{}`
        // So, I instead only place this function here, with the understanding submodules only go
        // one layer deep.
        .set_item(format!("minictl.{}", child.name()?), child)?;
    Ok(())
}

#[cfg(feature = "python")]
#[pymodule]
fn minictl(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    add_submodule(m, &formulas::python::get_submodule(m.py())?)?;
    Ok(())
}
