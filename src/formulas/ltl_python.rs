// This file is basically a big switchboard matching over the formula types multiple times.
// ... in all cases recursively defining some simple formula.
// It might be possible to define these generically and make the file a lot shorter,
// but I doubt that would make the code easier to read or maintain.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

use std::fmt;
use std::rc::Rc;

use crate::formulas::{ltl_types::memoize_ltl, LTLVariable};

use super::{parse_ltl, LTLFormula};

#[pyclass(
    module = "minictl",
    name = "LTLFormula",
    get_all,
    frozen,
    eq,
    hash,
    str
)]
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct PyLTLFormula {
    pub name: String,
    pub arguments: Vec<PyLTLFormula>,
}

impl PyLTLFormula {
    #[inline(always)]
    fn new_bare(name: &str, arguments: Vec<PyLTLFormula>) -> Self {
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
                .map(|item| item.extract::<PyLTLFormula>())
                .collect::<PyResult<Vec<PyLTLFormula>>>()?;
            Ok(Self { name, arguments })
        } else {
            Err(PyValueError::new_err(
                "Expected {nr_args} arguments for {name}, found {nr_found}",
            ))
        }
    }
    pub(crate) fn from_rust(formula: &LTLFormula) -> Self {
        use LTLFormula as F;
        use PyLTLFormula as PF;
        match formula {
            F::Top => Self::new_bare("TOP", Vec::default()),
            F::Bot => Self::new_bare("BOT", Vec::default()),
            F::Neg(f) => Self::new_bare("Neg", vec![PF::from_rust(f)]),
            F::X(f) => Self::new_bare("X", vec![PF::from_rust(f)]),
            F::F(f) => Self::new_bare("F", vec![PF::from_rust(f)]),
            F::G(f) => Self::new_bare("G", vec![PF::from_rust(f)]),
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
            F::U(f1, f2) => Self::new_bare("U", vec![PF::from_rust(f1), PF::from_rust(f2)]),
            F::W(f1, f2) => Self::new_bare("W", vec![PF::from_rust(f1), PF::from_rust(f2)]),
            F::R(f1, f2) => Self::new_bare("R", vec![PF::from_rust(f1), PF::from_rust(f2)]),
            F::Atomic(variable) => Self {
                name: variable.inner.clone(),
                arguments: Vec::default(),
            },
        }
    }
    #[inline(always)]
    fn arg_to_rust(&self, index: usize) -> Option<Rc<LTLFormula>> {
        self.arguments.get(index)?.to_rust()
    }

    pub(crate) fn to_rust(&self) -> Option<Rc<LTLFormula>> {
        use LTLFormula as F;
        let ret = match self.name.as_str() {
            "TOP" => Rc::new(F::Top),
            "BOT" => Rc::new(F::Bot),
            "Neg" => Rc::new(F::Neg(self.arg_to_rust(0)?)),
            "X" => Rc::new(F::X(self.arg_to_rust(0)?)),
            "F" => Rc::new(F::F(self.arg_to_rust(0)?)),
            "G" => Rc::new(F::G(self.arg_to_rust(0)?)),
            "And" => Rc::new(F::And(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            "Or" => Rc::new(F::Or(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            "ImpliesR" => Rc::new(F::ImpliesR(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            "ImpliesL" => Rc::new(F::ImpliesL(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            "BiImplies" => Rc::new(F::BiImplies(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            "U" => Rc::new(F::U(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            "W" => Rc::new(F::W(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            "R" => Rc::new(F::R(self.arg_to_rust(0)?, self.arg_to_rust(1)?)),
            other => Rc::new(F::Atomic(LTLVariable::new(other.to_string()))),
        };
        Some(memoize_ltl(&ret))
    }
}

impl fmt::Display for PyLTLFormula {
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
            "X" => write!(f, "X({})", get_argstr(0)),
            "F" => write!(f, "F({})", get_argstr(0)),
            "G" => write!(f, "G({})", get_argstr(0)),
            "And" => write!(f, "({})∧({})", get_argstr(0), get_argstr(1)),
            "Or" => write!(f, "({})∧({})", get_argstr(0), get_argstr(1)),
            "ImpliesR" => write!(f, "({})→({})", get_argstr(0), get_argstr(1)),
            "ImpliesL" => write!(f, "({})←({})", get_argstr(0), get_argstr(1)),
            "BiImplies" => write!(f, "({})↔({})", get_argstr(0), get_argstr(1)),
            "U" => write!(f, "({})U({})", get_argstr(0), get_argstr(1)),
            "W" => write!(f, "({})W({})", get_argstr(0), get_argstr(1)),
            "R" => write!(f, "({})R({})", get_argstr(0), get_argstr(1)),
            _ => write!(f, "{}", self.name),
        }
    }
}

#[pymethods]
impl PyLTLFormula {
    #[new]
    #[pyo3(signature=(name, *py_args))]
    fn new(name: String, py_args: &Bound<'_, PyTuple>) -> PyResult<Self> {
        match name.as_str() {
            "TOP" => Self::new_with_pyargs(name, py_args, 0),
            "BOT" => Self::new_with_pyargs(name, py_args, 0),
            "Neg" => Self::new_with_pyargs(name, py_args, 1),
            "X" => Self::new_with_pyargs(name, py_args, 1),
            "F" => Self::new_with_pyargs(name, py_args, 1),
            "G" => Self::new_with_pyargs(name, py_args, 1),
            "And" => Self::new_with_pyargs(name, py_args, 2),
            "Or" => Self::new_with_pyargs(name, py_args, 2),
            "ImpliesR" => Self::new_with_pyargs(name, py_args, 2),
            "ImpliesL" => Self::new_with_pyargs(name, py_args, 2),
            "BiImplies" => Self::new_with_pyargs(name, py_args, 2),
            "U" => Self::new_with_pyargs(name, py_args, 2),
            "W" => Self::new_with_pyargs(name, py_args, 2),
            "R" => Self::new_with_pyargs(name, py_args, 2),
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
        let res = parse_ltl(&formula).map_err(|err| {
            PyValueError::new_err(format!("Cannot parse {} into formula: {}", formula, err))
        })?;
        Ok(Self::from_rust(&res))
    }
}
