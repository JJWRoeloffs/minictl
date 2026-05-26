#![allow(unused)]

use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum GNBACreationError {
    #[error("State not mentionned in edge map: {0}")]
    StateNotMentionned(String),
    #[error("State mentionned in edges not in states: {0}")]
    UnusedEdgeList(String),
    #[error("Edge points to state {0}, but it does not exist")]
    DanglingEdge(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GNBATransition {
    pub from: String,
    pub to: String,
    pub vars: HashSet<String>,
}

impl GNBATransition {
    pub fn new(from: String, to: String, vars: HashSet<String>) -> Self {
        Self { from, to, vars }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct GNBA {
    states: Vec<String>,
    initial_states: Vec<String>,
    transitions: Vec<GNBATransition>,
    accepting_sets: Vec<HashSet<String>>,
    name_idx: HashMap<String, usize>,
    forewards: Vec<Vec<usize>>,
    backwards: Vec<Vec<usize>>,
}

impl GNBA {
    pub fn new(
        states: Vec<String>,
        initial_states: Vec<String>,
        transitions: Vec<GNBATransition>,
        accepting_sets: Vec<HashSet<String>>,
    ) -> Result<Self, GNBACreationError> {
        let name_idx = states
            .iter()
            .enumerate()
            .map(|(i, v)| (v.clone(), i))
            .collect::<HashMap<String, usize>>();

        let mut forewards: Vec<Vec<usize>> = vec![Vec::default(); states.len()];
        let mut backwards: Vec<Vec<usize>> = vec![Vec::default(); states.len()];

        for (i, item) in transitions.iter().enumerate() {
            forewards
                .get_mut(
                    *name_idx
                        .get(&item.from)
                        .ok_or(GNBACreationError::DanglingEdge(format!(
                            "Recieved Edge from {}, but no such state exists",
                            item.from
                        )))?,
                )
                .expect("The just created name_idx is valid")
                .push(i);
            backwards
                .get_mut(
                    *name_idx
                        .get(&item.to)
                        .ok_or(GNBACreationError::DanglingEdge(format!(
                            "Recieved Edge to {}, but no such state exists",
                            item.to
                        )))?,
                )
                .expect("The just created name_idx is valid")
                .push(i);
        }

        Ok(Self {
            states,
            initial_states,
            transitions,
            accepting_sets,
            name_idx,
            forewards,
            backwards,
        })
    }
    pub fn transition_from(&self, from: &str) -> Option<Vec<&GNBATransition>> {
        Some(
            self.forewards
                .get(*self.name_idx.get(from)?)
                .unwrap_or(&Vec::new())
                .iter()
                .map(|t| self.transitions.get(*t).unwrap())
                .collect(),
        )
    }
    pub fn transition_to(&self, to: &str) -> Option<Vec<&GNBATransition>> {
        Some(
            self.backwards
                .get(*self.name_idx.get(to)?)
                .unwrap_or(&Vec::new())
                .iter()
                .map(|t| self.transitions.get(*t).unwrap())
                .collect(),
        )
    }
}
