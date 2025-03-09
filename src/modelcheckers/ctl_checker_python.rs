use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::CTLChecker;
use crate::formulas::ctl_python::PyCTLFormula;
use crate::formulas::{CTLFactory, CTLFormula};
use crate::models::models_python::PyModel;

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::types::{PyAny, PyTuple};

/// The Python view into the CTL Checker
/// Though this class is not frozen, you cannot modify it directly.
/// The object will update itself on calls of `check` by updating the cache.
/// This means subsequent calls of `check` will be increasingly faster.
///
/// In Python, you can create this class from a model with the
/// CTLChecker(model) constructor.
#[pyclass(module = "minictl", name = "CTLChecker")]
#[derive(Debug)]
pub struct PyCTLChecker {
    pymodel: PyModel,
    inner: CTLChecker,
    modifications: HashMap<String, Py<PyAny>>,
}

impl PyCTLChecker {
    fn has_modification(&self, formula: &CTLFormula) -> bool {
        use CTLFormula as F;
        match formula {
            F::EX(..) => self.modifications.contains_key("EX"),
            F::AX(..) => self.modifications.contains_key("AX"),
            F::EF(..) => self.modifications.contains_key("EF"),
            F::AF(..) => self.modifications.contains_key("AF"),
            F::EG(..) => self.modifications.contains_key("EG"),
            F::AG(..) => self.modifications.contains_key("AG"),
            F::EU(..) => self.modifications.contains_key("EU"),
            F::AU(..) => self.modifications.contains_key("AU"),
            _ => false,
        }
    }
    fn call_modification(
        &self,
        py: Python,
        which: &str,
        states: &[HashSet<String>],
    ) -> PyResult<HashSet<String>> {
        // Stupid hackfix: We're calling with model as a keyword argument and the states
        // as positional arguments, `into_pyobject` is hard to coerse into a `PyObject` withohut
        // any reference to the type it actually points to, meaning we'd haveto use `dyn`.
        let modelarg = [("model", self.pymodel.clone())].into_py_dict(py)?;
        let args = PyTuple::new(py, states)?;
        self.modifications
            .get(which)
            .ok_or(PyRuntimeError::new_err(format!(
                "Modifification {which} cannot be found. This is likely an internal error"
            )))?
            .call(py, args, Some(&modelarg))?
            .extract(py)
    }
    fn apply_modification(
        &mut self,
        py: Python,
        formula: Arc<CTLFormula>,
    ) -> PyResult<HashSet<String>> {
        use CTLFormula as F;
        match formula.as_ref() {
            F::EX(inner) => {
                let inner_res = self.inner.check(inner.clone());
                self.call_modification(py, "EX", &[inner_res])
            },
            F::AX(inner) => {
                let inner_res = self.inner.check(inner.clone());
                self.call_modification(py, "AX", &[inner_res])
            },
            F::EF(inner) => {
                let inner_res = self.inner.check(inner.clone());
                self.call_modification(py, "EF", &[inner_res])
            },
            F::AF(inner) => {
                let inner_res = self.inner.check(inner.clone());
                self.call_modification(py, "AF", &[inner_res])
            },
            F::EG(inner) => {
                let inner_res = self.inner.check(inner.clone());
                self.call_modification(py, "EG", &[inner_res])
            },
            F::AG(inner) => {
                let inner_res = self.inner.check(inner.clone());
                self.call_modification(py, "AG", &[inner_res])
            },
            F::EU(lhs, rhs) => {
                let lhs_res = self.inner.check(lhs.clone());
                let rhs_res = self.inner.check(rhs.clone());
                self.call_modification(py, "EU", &[lhs_res, rhs_res])
            },
            F::AU(lhs, rhs) => {
                let lhs_res = self.inner.check(lhs.clone());
                let rhs_res = self.inner.check(rhs.clone());
                self.call_modification(py, "AU", &[lhs_res, rhs_res])
            },
            _ => Err(PyRuntimeError::new_err("Called modification on something that cannot recieve one. This is likely an internal error."))
        }
    }
}

#[pymethods]
impl PyCTLChecker {
    #[new]
    fn new(model: PyModel) -> Self {
        Self {
            inner: CTLChecker::new(model.to_rust()),
            pymodel: model,
            modifications: HashMap::new(),
        }
    }
    fn check(&mut self, formula: PyCTLFormula) -> PyResult<HashSet<String>> {
        let rsformula = formula.to_rust().ok_or(PyValueError::new_err(
            "provided formula is not a valid CTL formula",
        ))?;
        Ok(self.inner.check(rsformula))
    }
    #[pyo3(signature = (formula, *, debug = false))]
    fn check_with_custom(
        &mut self,
        py: Python,
        formula: PyCTLFormula,
        debug: bool,
    ) -> PyResult<HashSet<String>> {
        let mut ctlfactory = CTLFactory::default();
        let rsformula = ctlfactory.create(formula.to_rust().ok_or(PyValueError::new_err(
            "provided formula is not a valid CTL formula",
        ))?);
        let mut formulas = ctlfactory
            .get_cache()
            .iter()
            .filter(|(k, _v)| self.has_modification(k))
            .map(|(_k, v)| v.clone())
            .collect::<Vec<Arc<CTLFormula>>>();
        formulas.sort_by_cached_key(|f| f.total_size());

        // Since the formulas are sorted by size, we know we get the lowest in the tree
        // first and only then the bigger ones that might have those lowers as dependencies.
        // After that, we can simply rely on the cache.
        for f in formulas.iter() {
            let res = self.apply_modification(py, f.clone())?;
            if debug {
                let expected = self.inner.check(f.clone());
                compare_sets(&res, &expected, f)?;
            }
            self.inner.update_cache(f.clone(), res);
        }

        // Since the inner's cache is updated with the custom algorithm,
        // we can just return inner.check() and expect it to be the modified values.
        Ok(self.inner.check(rsformula))
    }
    fn add_custom(&mut self, target: String, func: Py<PyAny>) -> PyResult<()> {
        match target.as_str() {
            "EX" | "AX" | "EF" | "AF" | "EG" | "AG" | "EU" | "AU" => {
                self.modifications.insert(target, func);
                Ok(())
            }
            _ => Err(PyValueError::new_err(format!(
                "{target} is not a valid modal operator"
            ))),
        }
    }
}

// It's nice to have good errors when working on algorithms.
fn compare_sets(
    res: &HashSet<String>,
    expected: &HashSet<String>,
    formula: &CTLFormula,
) -> PyResult<()> {
    let too_many: HashSet<_> = res.difference(expected).collect();
    let missing: HashSet<_> = expected.difference(res).collect();

    if too_many.is_empty() && missing.is_empty() {
        return Ok(());
    }

    let mut base = format!(
        "Error while applying modification on function: {}\n",
        &PyCTLFormula::from_rust(formula)
    );
    if !too_many.is_empty() {
        base.push_str(&format!(
            "Did not expect but recieved states: {:?}\n",
            too_many
        ));
    }
    if !missing.is_empty() {
        base.push_str(&format!(
            "Expected but didn't recieve states : {:?}\n",
            missing
        ));
    }
    Err(PyRuntimeError::new_err(base))
}
