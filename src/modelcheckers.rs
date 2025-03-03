mod ctl_checker;
pub use ctl_checker::CTLChecker;

#[cfg(feature = "python")]
pub mod ctl_checker_python;

#[cfg(feature = "python")]
pub(crate) mod python {
    use super::ctl_checker_python::PyCTLChecker;
    use pyo3::prelude::*;
    pub fn get_submodule(python: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
        let child_module = PyModule::new(python, "modelcheckers")?;
        child_module.add_class::<PyCTLChecker>()?;
        Ok(child_module)
    }
}
