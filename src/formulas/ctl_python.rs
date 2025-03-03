// This file is basically a big switchboard matching over the formula types multiple times.
// ... in all cases recursively defining some simple formula.
// It might be possible to define these generically and make the file a lot shorter,
// but I doubt that would make the code easier to read or maintain.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

use std::fmt;
use std::rc::Rc;

use crate::formulas::{ctl_types::memoize_ctl, CTLVariable};

use super::{parse_ctl, CTLFormula};

/// The python view into the CTLFormula.
/// This class is frozen. Objects, once created, cannot be modified.
///
/// In python, either create this litterally through the constructor,
/// like `CTLFormula("and", CTLFormula("p"), CTLFormula("q"))` or though the
/// .parse method like: CTLFormula.parse("p and q")
///
/// Implements `__str__`, `__eq__`, and `__hash__`.
#[pyclass(
    module = "minictl",
    name = "CTLFormula",
    get_all,
    frozen,
    eq,
    hash,
    str
)]
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct PyCTLFormula {
    pub name: String,
    pub arguments: Vec<PyCTLFormula>,
}

impl PyCTLFormula {
    #[inline(always)]
    fn new_bare(name: &str, arguments: Vec<PyCTLFormula>) -> Self {
        Self {
            name: name.to_owned(),
            arguments,
        }
    }
    fn new_with_pyargs(
        name: String,
        py_arguments: &Bound<'_, PyTuple>,
        nr_args: usize,
    ) -> PyResult<Self> {
        let nr_found = py_arguments.len();
        if nr_found == nr_args {
            let arguments = py_arguments
                .iter()
                .map(|item| item.extract::<PyCTLFormula>())
                .collect::<PyResult<Vec<PyCTLFormula>>>()?;
            Ok(Self { name, arguments })
        } else {
            Err(PyValueError::new_err(
                "Expected {nr_args} arguments for {name}, found {nr_found}",
            ))
        }
    }
    pub(crate) fn from_rust(formula: &CTLFormula) -> Self {
        use CTLFormula as F;
        use PyCTLFormula as PF;
        match formula {
            F::Top => Self::new_bare("TOP", Vec::default()),
            F::Bot => Self::new_bare("BOT", Vec::default()),
            F::Neg(f) => Self::new_bare("Neg", vec![PF::from_rust(f)]),
            F::EX(f) => Self::new_bare("EX", vec![PF::from_rust(f)]),
            F::AX(f) => Self::new_bare("AX", vec![PF::from_rust(f)]),
            F::EF(f) => Self::new_bare("EF", vec![PF::from_rust(f)]),
            F::AF(f) => Self::new_bare("AF", vec![PF::from_rust(f)]),
            F::EG(f) => Self::new_bare("EG", vec![PF::from_rust(f)]),
            F::AG(f) => Self::new_bare("AG", vec![PF::from_rust(f)]),
            F::And(f1, f2) => Self::new_bare("And", vec![PF::from_rust(f1), PF::from_rust(f2)]),
            F::Or(f1, f2) => Self::new_bare("Or", vec![PF::from_rust(f1), PF::from_rust(f2)]),
            F::ImpliesR(f1, f2) => {
                Self::new_bare("ImpliesR", vec![PF::from_rust(f1), PF::from_rust(f2)])
            }
            F::ImpliesL(f1, f2) => {
                Self::new_bare("ImpliesL", vec![PF::from_rust(f1), PF::from_rust(f2)])
            }
            F::BiImplies(f1, f2) => {
                Self::new_bare("BiImplies", vec![PF::from_rust(f1), PF::from_rust(f2)])
            }
            F::EU(f1, f2) => Self::new_bare("EU", vec![PF::from_rust(f1), PF::from_rust(f2)]),
            F::AU(f1, f2) => Self::new_bare("AU", vec![PF::from_rust(f1), PF::from_rust(f2)]),
            F::Atomic(variable) => Self {
                name: variable.inner.clone(),
                arguments: Vec::default(),
            },
        }
    }
    #[inline(always)]
    fn arg_to_rust(&self, index: usize) -> Option<Rc<CTLFormula>> {
        self.arguments.get(index)?.to_rust()
    }

    pub(crate) fn to_rust(&self) -> Option<Rc<CTLFormula>> {
        use CTLFormula as F;
        let ret = match self.name.as_str() {
            "TOP" => Rc::new(F::Top),
            "BOT" => Rc::new(F::Bot),
            "Neg" => Rc::new(F::Neg(self.arg_to_rust(0)?)),
            "EX" => Rc::new(F::EX(self.arg_to_rust(0)?)),
            "AX" => Rc::new(F::AX(self.arg_to_rust(0)?)),
            "EF" => Rc::new(F::EF(self.arg_to_rust(0)?)),
            "AF" => Rc::new(F::AF(self.arg_to_rust(0)?)),
            "EG" => Rc::new(F::EG(self.arg_to_rust(0)?)),
            "AG" => Rc::new(F::AG(self.arg_to_rust(0)?)),
            "And" => Rc::new(F::And(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            "Or" => Rc::new(F::Or(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            "ImpliesR" => Rc::new(F::ImpliesR(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            "ImpliesL" => Rc::new(F::ImpliesL(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            "BiImplies" => Rc::new(F::BiImplies(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            "EU" => Rc::new(F::EU(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            "AU" => Rc::new(F::AU(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            other => Rc::new(F::Atomic(CTLVariable::new(other.to_string()))),
        };
        Some(memoize_ctl(&ret))
    }
}

impl fmt::Display for PyCTLFormula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Rust doesn't allow the definitions of helper formulas inside implementation blocks
        // So this formula is defined as a lambda within this function.
        // It should technically be unreachable, but you never know with python.
        let get_argstr = |index| {
            self.arguments
                .get(index)
                .map(|f| format!("{}", f))
                .unwrap_or("ERR: missing".to_string())
        };
        match self.name.as_str() {
            "TOP" => write!(f, "⊤"),
            "BOT" => write!(f, "⊥"),
            "Neg" => write!(f, "¬({})", get_argstr(0)),
            "EX" => write!(f, "EX({})", get_argstr(0)),
            "AX" => write!(f, "AX({})", get_argstr(0)),
            "EF" => write!(f, "EF({})", get_argstr(0)),
            "AF" => write!(f, "AF({})", get_argstr(0)),
            "EG" => write!(f, "EG({})", get_argstr(0)),
            "AG" => write!(f, "AG({})", get_argstr(0)),
            "And" => write!(f, "({})∧({})", get_argstr(0), get_argstr(1)),
            "Or" => write!(f, "({})∧({})", get_argstr(0), get_argstr(1)),
            "ImpliesR" => write!(f, "({})→({})", get_argstr(0), get_argstr(1)),
            "ImpliesL" => write!(f, "({})←({})", get_argstr(0), get_argstr(1)),
            "BiImplies" => write!(f, "({})↔({})", get_argstr(0), get_argstr(1)),
            "EU" => write!(f, "E[({})U({})]", get_argstr(0), get_argstr(1)),
            "AU" => write!(f, "A[({})U({})]", get_argstr(0), get_argstr(1)),
            _ => write!(f, "{}", self.name),
        }
    }
}

#[pymethods]
impl PyCTLFormula {
    #[new]
    #[pyo3(signature=(name, *py_args))]
    fn new(name: String, py_args: &Bound<'_, PyTuple>) -> PyResult<Self> {
        match name.as_str() {
            "TOP" => Self::new_with_pyargs(name, py_args, 0),
            "BOT" => Self::new_with_pyargs(name, py_args, 0),
            "Neg" => Self::new_with_pyargs(name, py_args, 1),
            "EX" => Self::new_with_pyargs(name, py_args, 1),
            "AX" => Self::new_with_pyargs(name, py_args, 1),
            "EF" => Self::new_with_pyargs(name, py_args, 1),
            "AF" => Self::new_with_pyargs(name, py_args, 1),
            "EG" => Self::new_with_pyargs(name, py_args, 1),
            "AG" => Self::new_with_pyargs(name, py_args, 1),
            "And" => Self::new_with_pyargs(name, py_args, 2),
            "Or" => Self::new_with_pyargs(name, py_args, 2),
            "ImpliesR" => Self::new_with_pyargs(name, py_args, 2),
            "ImpliesL" => Self::new_with_pyargs(name, py_args, 2),
            "BiImplies" => Self::new_with_pyargs(name, py_args, 2),
            "EU" => Self::new_with_pyargs(name, py_args, 2),
            "AU" => Self::new_with_pyargs(name, py_args, 2),
            _ if py_args.is_empty() => {
                if name
                    .chars()
                    .all(|c| c.is_alphanumeric() && !c.is_uppercase())
                {
                    Self::new_with_pyargs(name, py_args, 0)
                } else {
                    Err(PyValueError::new_err(
                        "{name} is not a valid formula name: not all letters are lowercase",
                    ))
                }
            }
            _ => Err(PyValueError::new_err(
                "{name} is not a valid formula name, or, if variable, arguments are nonempty",
            )),
        }
    }
    #[staticmethod]
    fn parse(formula: String) -> PyResult<Self> {
        let res = parse_ctl(&formula).map_err(|err| {
            PyValueError::new_err(format!("Cannot parse {} into formula: {}", formula, err))
        })?;
        Ok(Self::from_rust(&res))
    }
}
