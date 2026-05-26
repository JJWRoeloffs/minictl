#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::convert::From;

use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;

use super::{GNBACreationError, GNBATransition, GNBA};

impl From<GNBACreationError> for PyErr {
    fn from(value: GNBACreationError) -> Self {
        PyValueError::new_err(value.to_string())
    }
}

/// The Python view into a GNBA Transition
/// This class is frozen. Objects, once created, cannot be modified.
///
/// You can create them with the GNBATransition("name_from", "name_to", {"var1", "var2"}) constructor,
/// providing the state names and a set of variables that are true in the transition.
#[pyclass(
    module = "minictl",
    name = "GNBATransition",
    get_all,
    frozen,
    eq,
    from_py_object
)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PyGNBATransition {
    pub from_state: String,
    pub to_state: String,
    pub variables: HashSet<String>,
}
impl PyGNBATransition {
    fn to_rust(&self) -> GNBATransition {
        GNBATransition::new(
            self.from_state.clone(),
            self.to_state.clone(),
            self.variables.clone(),
        )
    }
    fn from_rust(orig: &GNBATransition) -> Self {
        Self {
            from_state: orig.from.clone(),
            to_state: orig.to.clone(),
            variables: orig.vars.clone(),
        }
    }
}

#[pymethods]
impl PyGNBATransition {
    #[new]
    fn new(from_state: String, to_state: String, variables: HashSet<String>) -> Self {
        Self {
            from_state,
            to_state,
            variables,
        }
    }
    fn contains(&self, var: &str) -> bool {
        self.variables.contains(var)
    }
}

#[pyclass(module = "minictl", name = "GNBA", frozen, from_py_object)]
#[derive(Debug, Clone)]
pub struct PyGNBA {
    transitions: Vec<PyGNBATransition>,
    gnba: GNBA,
}

/// The python view into a Generalised Nondeterministic Büchi Automaton
/// This class is frozen. Objects, once created, cannot be modified.
/// This class does not expose any public fields. It can only be inspected through methods.
///
/// You can create them with the GNBA(["s1", "s2"], ["s1"], transitions, [{"s2"}]) constructor,
/// providing a list of states, a second list of initial states, the transitions (of type GNBATransition),
/// and the accepting sets that together represent the GNBA
/// This constructor throws a value error when the arguments do not lead to a valid automaton,
/// e.g. if edges point to unknown states.
impl PyGNBA {
    fn new_bare(
        states: Vec<String>,
        initial_states: Vec<String>,
        transitions: Vec<PyGNBATransition>,
        accepting_sets: Vec<HashSet<String>>,
    ) -> Result<Self, GNBACreationError> {
        let transitions_rs = transitions.iter().map(|t| t.to_rust()).collect();
        let gnba = GNBA::new(states, initial_states, transitions_rs, accepting_sets)?;
        Ok(Self { transitions, gnba })
    }
    fn to_rust(&self) -> GNBA {
        self.gnba.clone()
    }
}

#[pymethods]
impl PyGNBA {
    #[new]
    fn new(
        states: Vec<String>,
        initial_states: Vec<String>,
        transitions: Vec<PyGNBATransition>,
        accepting_sets: Vec<HashSet<String>>,
    ) -> PyResult<Self> {
        Self::new_bare(states, initial_states, transitions, accepting_sets).map_err(Into::into)
    }

    fn transition_to(&self, to_state: String) -> PyResult<Vec<PyGNBATransition>> {
        self.gnba
            .transition_to(&to_state)
            .map(|ts| ts.iter().map(|t| PyGNBATransition::from_rust(t)).collect())
            .ok_or(PyValueError::new_err("{state} is not a state".to_string()))
    }
    fn transition_from(&self, to_state: String) -> PyResult<Vec<PyGNBATransition>> {
        self.gnba
            .transition_from(&to_state)
            .map(|ts| ts.iter().map(|t| PyGNBATransition::from_rust(t)).collect())
            .ok_or(PyValueError::new_err("{state} is not a state".to_string()))
    }
}
