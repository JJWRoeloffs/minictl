use crate::formulas::MLVariable;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct State<T: MLVariable> {
    name: String,
    vars: HashSet<T>,
}
impl<T: MLVariable> State<T> {
    pub fn new(name: String, vars: HashSet<T>) -> Self {
        Self { name, vars }
    }
    pub fn contains(&self, var: &T) -> bool {
        self.vars.contains(var)
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Model<T: MLVariable> {
    states: Vec<State<T>>,
    name_idx: HashMap<String, usize>,
    edges: HashMap<String, Vec<String>>,
    post_idx: Vec<Vec<usize>>,
    pre_idx: Vec<Vec<usize>>,
}
// Instead of strings, we will be dealing with usize indexes into the states vec
// when working inside the crate. This prevents us from the alternatives:
//     having to deal with the String names, and cloning them all over
//     having to deal with &str names, and a ton of really anoying lifetimes
//     having to deal with &State<T>, which cannot be hashed as it contains a HashSet
//
// In C++, I'd simply pass around raw pointers into the static states vec, but I guess
// relative pointers is the rust way in this case.
impl<T: MLVariable> Model<T> {
    pub fn new(states: Vec<State<T>>, edges: HashMap<String, Vec<String>>) -> Option<Self> {
        let name_idx = states
            .iter()
            .enumerate()
            .map(|(i, v)| (v.name(), i))
            .collect::<HashMap<String, usize>>();
        let post_idx = states
            .iter()
            .map(|s| {
                edges
                    .get(&s.name())?
                    .iter()
                    .map(|n| name_idx.get(n).copied())
                    .collect()
            })
            .collect::<Option<Vec<Vec<usize>>>>()?;
        let pre_idx = reverse_graph(&post_idx);
        Some(Self {
            states,
            name_idx,
            edges,
            post_idx,
            pre_idx,
        })
    }
    pub(crate) fn all_idx(&self) -> HashSet<usize> {
        self.states.iter().enumerate().map(|(i, _)| i).collect()
    }
    pub fn all(&self) -> HashSet<String> {
        self.states.iter().map(|s| s.name()).collect()
    }
    pub(crate) fn all_containing_idx(&self, var: &T) -> HashSet<usize> {
        self.states
            .iter()
            .enumerate()
            .filter(|(_i, s)| s.contains(var))
            .map(|(i, _s)| i)
            .collect()
    }
    pub fn all_containing(&self, var: &T) -> HashSet<String> {
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
        self.post_idx
            .iter()
            .enumerate()
            .filter(|(_i, v)| v.iter().all(|u| indexes.contains(u)))
            .map(|(i, _v)| i)
            .collect()
    }
    pub(crate) fn get_names(&self, indexes: &HashSet<usize>) -> HashSet<String> {
        // All indexes should still be valid, pointing into the vec, as we don't allow
        // public modification.
        debug_assert!(indexes.iter().all(|&i| self.states.get(i).is_some()));
        indexes
            .iter()
            .map(|&i| self.states.get(i).expect("Asserted earlier").name().clone())
            .collect()
    }
}

fn reverse_graph(backward_graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = backward_graph.len();
    let mut forward_graph = vec![Vec::new(); n];

    backward_graph
        .iter()
        .enumerate()
        .for_each(|(dest, sources)| {
            sources
                .iter()
                .for_each(|&src| forward_graph[src].push(dest));
        });

    forward_graph
}
