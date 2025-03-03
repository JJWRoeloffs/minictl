mod model;
pub use model::{Model, ModelCreationError, State};

#[cfg(feature = "python")]
pub mod models_python;

#[cfg(feature = "python")]
pub(crate) mod python {
    use super::models_python::{PyModel, PyState};
    use pyo3::prelude::*;
    pub fn get_submodule(python: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
        let child_module = PyModule::new(python, "models")?;
        child_module.add_class::<PyModel>()?;
        child_module.add_class::<PyState>()?;
        Ok(child_module)
    }
}
