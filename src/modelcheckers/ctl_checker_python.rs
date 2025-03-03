use std::collections::HashSet;

use crate::formulas::ctl_python::PyCTLFormula;
use crate::models::models_python::PyModel;

use super::CTLChecker;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// The Python view into the CTL Checker
/// Though this class is not frozen, you cannot modify it directly.
/// The object will update itself on calls of `check` by updating the cache.
/// This means subsequent calls of `check` will be increasingly faster.
///
/// In Python, you can create this class from a model with the
/// CTLChecker(model) constructor.
#[pyclass(module = "minictl", name = "CTLChecker")]
#[derive(Debug, Clone)]
pub struct PyCTLChecker {
    inner: CTLChecker,
}

#[pymethods]
impl PyCTLChecker {
    #[new]
    fn new(model: PyModel) -> Self {
        Self {
            inner: CTLChecker::new(model.into_rust()),
        }
    }
    fn check(&mut self, formula: PyCTLFormula) -> PyResult<HashSet<String>> {
        let rsformula = formula.to_rust().ok_or(PyValueError::new_err(
            "provided formula is not a valid CTL formula",
        ))?;
        Ok(self.inner.check(rsformula))
    }
}
