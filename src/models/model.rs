use std::collections::{HashMap, HashSet};
use thiserror::Error;

// Slightly more descriptive errors to mention in what way a state did not
// have any outgoing edges.
#[derive(Debug, PartialEq, Error)]
pub enum ModelCreationError {
    #[error("State not mentionned in edge map: {0}")]
    StateNotMentionned(String),
    #[error("State has empty edge list: {0}")]
    EmptyEdgeList(String),
    #[error("State mentionned in edges not in states: {0}")]
    UnusedEdgeList(String),
    #[error("Edge points to state {0}, but it does not exist")]
    DanglingEdge(String),
}

#[derive(Debug, Clone)]
pub struct State {
    name: String,
    vars: HashSet<String>,
}
impl State {
    pub fn new(name: String, vars: HashSet<String>) -> Self {
        Self { name, vars }
    }
    pub fn contains(&self, var: &str) -> bool {
        self.vars.contains(var)
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Model {
    states: Vec<State>,
    name_idx: HashMap<String, usize>,
    edges: HashMap<String, Vec<String>>,
    post_idx: Vec<Vec<usize>>,
    pre_idx: Vec<Vec<usize>>,
}

// Instead of strings, we will be dealing with usize indexes into the states vec
// when working inside the crate. This prevents us from the alternatives:
//     having to deal with the String names, and cloning them all over
//     having to deal with &str names, and a ton of really anoying lifetimes
//     having to deal with &State, which cannot be hashed as it contains a HashSet
//
// In C++, I'd simply pass around raw pointers into the static states vec, but I guess
// relative pointers is the rust way in this case.
impl Model {
    pub fn new(
        states: Vec<State>,
        edges: HashMap<String, Vec<String>>,
    ) -> Result<Self, ModelCreationError> {
        if let Some(state) = states.iter().find(|s| !edges.contains_key(&s.name)) {
            return Err(ModelCreationError::StateNotMentionned(state.name()));
        }
        if let Some((key, _v)) = edges.iter().find(|(_k, v)| v.is_empty()) {
            return Err(ModelCreationError::EmptyEdgeList(key.to_string()));
        }

        let name_idx = states
            .iter()
            .enumerate()
            .map(|(i, v)| (v.name(), i))
            .collect::<HashMap<String, usize>>();

        if let Some((key, _v)) = edges
            .iter()
            .find(|(k, _v)| !name_idx.contains_key(k.as_str()))
        {
            return Err(ModelCreationError::UnusedEdgeList(key.to_string()));
        }

        let post_idx = states
            .iter()
            .map(|s| {
                edges
                    .get(&s.name)
                    .ok_or(ModelCreationError::StateNotMentionned(s.name()))?
                    .iter()
                    .map(|n| {
                        name_idx
                            .get(n)
                            .ok_or(ModelCreationError::DanglingEdge(n.to_string()))
                            .copied()
                    })
                    .collect()
            })
            .collect::<Result<Vec<Vec<usize>>, ModelCreationError>>()?;
        let pre_idx = reverse_graph(&post_idx);
        Ok(Self {
            states,
            name_idx,
            edges,
            post_idx,
            pre_idx,
        })
    }
    pub(crate) fn get_idx(&self, name: &str) -> Option<usize> {
        self.name_idx.get(name).copied()
    }
    pub(crate) fn get_idxs(&self, names: &HashSet<String>) -> Option<HashSet<usize>> {
        names.iter().map(|n| self.get_idx(n)).collect()
    }
    pub fn get_state(&self, name: &str) -> Option<&State> {
        self.states.get(self.get_idx(name)?)
    }
    pub fn get_next(&self, name: &str) -> Option<Vec<String>> {
        self.edges.get(name).cloned()
    }
    pub(crate) fn all_idx(&self) -> HashSet<usize> {
        self.states.iter().enumerate().map(|(i, _)| i).collect()
    }
    pub fn all(&self) -> HashSet<String> {
        self.states.iter().map(|s| s.name()).collect()
    }
    pub(crate) fn all_containing_idx(&self, var: &str) -> HashSet<usize> {
        self.states
            .iter()
            .enumerate()
            .filter(|(_i, s)| s.contains(var))
            .map(|(i, _s)| i)
            .collect()
    }
    pub fn all_containing(&self, var: &str) -> HashSet<String> {
        self.states
            .iter()
            .filter(|s| s.contains(var))
            .map(|s| s.name())
            .collect()
    }
    pub(crate) fn all_except_idx(&self, which: &HashSet<usize>) -> HashSet<usize> {
        self.states
            .iter()
            .enumerate()
            .filter(|(i, _v)| !which.contains(i))
            .map(|(i, _v)| i)
            .collect()
    }
    pub fn all_except(&self, which: &HashSet<String>) -> HashSet<String> {
        self.states
            .iter()
            .map(|s| s.name())
            .filter(|s| !which.contains(s))
            .collect()
    }
    /// The set of states that can transition into the ones given.
    pub(crate) fn pre_e_idx(&self, indexes: &HashSet<usize>) -> HashSet<usize> {
        debug_assert!(indexes.iter().all(|&i| self.states.get(i).is_some()));
        indexes
            .iter()
            .flat_map(|&i| self.pre_idx.get(i).expect("All indexes are valid"))
            .copied()
            .collect()
    }
    /// The set of states for which all transitions mean transitioning into a state given.
    pub(crate) fn pre_a_idx(&self, indexes: &HashSet<usize>) -> HashSet<usize> {
        debug_assert!(indexes.iter().all(|&i| self.states.get(i).is_some()));
        indexes
            .iter()
            .flat_map(|&i| self.pre_idx.get(i).expect("All indexes are valid"))
            .filter(|&i| {
                self.post_idx
                    .get(*i)
                    .expect("All indexes are valid")
                    .iter()
                    .all(|u| indexes.contains(u))
            })
            .copied()
            .collect()
    }
    pub(crate) fn get_names(&self, indexes: &HashSet<usize>) -> HashSet<String> {
        // All indexes should still be valid, pointing into the vec, as we don't allow
        // public modification.
        debug_assert!(indexes.iter().all(|&i| self.states.get(i).is_some()));
        indexes
            .iter()
            .map(|&i| self.states.get(i).expect("All indexes are valid").name())
            .collect()
    }
}

fn reverse_graph(backward_graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut ret = vec![Vec::new(); backward_graph.len()];
    backward_graph
        .iter()
        .enumerate()
        .for_each(|(dest, sources)| {
            sources.iter().for_each(|&src| ret[src].push(dest));
        });
    ret
}
