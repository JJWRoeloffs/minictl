use std::collections::{HashMap, HashSet};
use std::convert::From;

use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;

use super::{Model, ModelCreationError, State};

impl From<ModelCreationError> for PyErr {
    fn from(value: ModelCreationError) -> Self {
        PyValueError::new_err(value.to_string())
    }
}

#[pyclass(module = "minictl", get_all, name = "State")]
#[derive(Debug, Clone)]
pub struct PyState {
    pub name: String,
    pub variables: HashSet<String>,
}
impl PyState {
    fn to_rust(&self) -> State {
        State::new(self.name.clone(), self.variables.clone())
    }
}

#[pymethods]
impl PyState {
    #[new]
    fn new(name: String, variables: HashSet<String>) -> Self {
        Self { name, variables }
    }

    fn contains(&self, var: &str) -> bool {
        self.variables.contains(var)
    }
}

#[pyclass(module = "minictl", name = "Model")]
#[derive(Debug, Clone)]
pub struct PyModel {
    states: Vec<PyState>,
    names: HashSet<String>,
    model: Model,
}

impl PyModel {
    fn new_bare(
        states: Vec<PyState>,
        edges: HashMap<String, Vec<String>>,
    ) -> Result<Self, ModelCreationError> {
        let innerstates: Vec<State> = states.iter().map(PyState::to_rust).collect();
        let names = edges.keys().cloned().collect();
        let model = Model::new(innerstates, edges)?;
        Ok(Self {
            states,
            names,
            model,
        })
    }
    fn get_idx(&self, which: &str) -> PyResult<usize> {
        self.model.get_idx(which).ok_or(PyKeyError::new_err(format!(
            "{which} cannot be found in the model"
        )))
    }
}

#[pymethods]
impl PyModel {
    #[new]
    fn new(states: Vec<PyState>, edges: HashMap<String, Vec<String>>) -> PyResult<Self> {
        Self::new_bare(states, edges).map_err(Into::into)
    }
    fn get_state(&self, which: &str) -> PyResult<PyState> {
        Ok(self
            .states
            .get(self.get_idx(which)?)
            .expect("Internal indexes are valid")
            .clone())
    }
    fn get_states(&self) -> Vec<PyState> {
        self.states.clone()
    }
    fn all(&self) -> HashSet<String> {
        self.model.all()
    }
    fn all_containing(&self, var: &str) -> HashSet<String> {
        self.model.all_containing(var)
    }
    fn all_except(&self, names: HashSet<String>) -> PyResult<HashSet<String>> {
        if let Some(name) = names.iter().find(|n| !self.names.contains(n.as_str())) {
            return Err(PyKeyError::new_err(format!(
                "{name} cannot be found in model"
            )));
        }
        Ok(self.model.all_except(&names))
    }
    fn pre_e(&self, names: HashSet<String>) -> PyResult<HashSet<String>> {
        let indexes = names
            .iter()
            .map(|n| self.get_idx(n))
            .collect::<PyResult<HashSet<usize>>>()?;
        let res_indexes = self.model.pre_e_idx(&indexes);
        Ok(self.model.get_names(&res_indexes))
    }
    fn pre_a(&self, names: HashSet<String>) -> PyResult<HashSet<String>> {
        let indexes = names
            .iter()
            .map(|n| self.get_idx(n))
            .collect::<PyResult<HashSet<usize>>>()?;
        let res_indexes = self.model.pre_a_idx(&indexes);
        Ok(self.model.get_names(&res_indexes))
    }
}
