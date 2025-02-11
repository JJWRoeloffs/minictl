#![allow(unused)]

use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::sync::Arc;

use typed_arena::Arena;

use crate::formulas::{CTLFactory, CTLFormula, CTLVariable};
use crate::models::Model;

struct CTLSolverInner<'a> {
    map: HashMap<Arc<CTLFormula>, &'a HashSet<usize>>,
    arena: &'a Arena<HashSet<usize>>,
}
impl<'a> CTLSolverInner<'a> {
    fn map(&self) -> &HashMap<Arc<CTLFormula>, &'a HashSet<usize>> {
        &self.map
    }
    fn memoise_alloc(
        &mut self,
        formula: Arc<CTLFormula>,
        ret: HashSet<usize>,
    ) -> &'a HashSet<usize> {
        let ret_ref = self.arena.alloc(ret);
        self.map.insert(formula.clone(), ret_ref);
        ret_ref
    }
    fn memoise_ref(
        &mut self,
        formula: Arc<CTLFormula>,
        ret: &'a HashSet<usize>,
    ) -> &'a HashSet<usize> {
        self.map.insert(formula.clone(), ret);
        ret
    }
    fn sat_ex(&mut self, formula: Arc<CTLFormula>, model: &Model<CTLVariable>) -> HashSet<usize> {
        model.pre_e_idx(self.solve(formula, model))
    }
    fn sat_eu(
        &mut self,
        formula1: Arc<CTLFormula>,
        formula2: Arc<CTLFormula>,
        model: &Model<CTLVariable>,
    ) -> HashSet<usize> {
        // We are using Cow only to have some type generic over T and &T,
        // as our initial `solve()` returns a reference, but calls to pre_a return owned values.
        let mut set = Cow::Borrowed(self.solve(formula2, model));
        let base = self.solve(formula1, model);
        loop {
            let next = model
                .pre_e_idx(&set)
                .intersection(base)
                .copied()
                .collect::<HashSet<usize>>()
                .union(&set)
                .copied()
                .collect::<HashSet<usize>>();
            if next == *set {
                return match set {
                    Cow::Owned(owned) => owned,
                    Cow::Borrowed(_) => next,
                };
            }
            set = Cow::Owned(next);
        }
    }
    fn sat_af(&mut self, formula: Arc<CTLFormula>, model: &Model<CTLVariable>) -> HashSet<usize> {
        // We are using Cow only to have some type generic over T and &T,
        // as our initial `solve()` returns a reference, but calls to pre_a return owned values.
        let mut set = Cow::Borrowed(self.solve(formula, model));
        loop {
            let next: HashSet<usize> = model.pre_a_idx(&set).union(&set).copied().collect();
            if next == *set {
                return match set {
                    Cow::Owned(owned) => owned,
                    Cow::Borrowed(_) => next,
                };
            }
            set = Cow::Owned(next);
        }
    }
    // TODO: Make a macro to make the formula rewriting easier to read.
    fn solve(
        &mut self,
        formula: Arc<CTLFormula>,
        model: &Model<CTLVariable>,
    ) -> &'a HashSet<usize> {
        if let Some(ret) = self.map.get(&formula) {
            return ret;
        }
        use CTLFormula as F;
        match formula.as_ref() {
            F::Top => self.memoise_alloc(formula.clone(), model.all_idx()),
            F::Bot => self.memoise_alloc(formula.clone(), HashSet::new()),
            F::Atomic(var) => self.memoise_alloc(formula.clone(), model.all_containing_idx(var)),
            F::Neg(inner) => {
                let ret = model.all_except_idx(self.solve(inner.clone(), model));
                self.memoise_alloc(formula, ret)
            }
            F::And(inner1, inner2) => {
                let ret = self
                    .solve(inner1.clone(), model)
                    .intersection(self.solve(inner2.clone(), model))
                    .copied()
                    .collect();
                self.memoise_alloc(formula, ret)
            }
            F::Or(inner1, inner2) => {
                let ret = self
                    .solve(inner1.clone(), model)
                    .union(self.solve(inner2.clone(), model))
                    .copied()
                    .collect();
                self.memoise_alloc(formula, ret)
            }
            F::ImpliesR(inner1, inner2) => {
                let ret = self.solve(
                    Arc::new(F::Or(Arc::new(F::Neg(inner1.clone())), inner2.clone())),
                    model,
                );
                self.memoise_ref(formula, ret)
            }
            F::ImpliesL(inner1, inner2) => {
                let ret = self.solve(
                    Arc::new(F::Or(inner1.clone(), Arc::new(F::Neg(inner2.clone())))),
                    model,
                );
                self.memoise_ref(formula, ret)
            }
            F::BiImplies(inner1, inner2) => {
                let ret = self.solve(
                    Arc::new(F::And(
                        Arc::new(F::ImpliesR(inner1.clone(), inner2.clone())),
                        Arc::new(F::ImpliesR(inner2.clone(), inner1.clone())),
                    )),
                    model,
                );
                self.memoise_ref(formula, ret)
            }
            F::AX(inner) => {
                let ret = self.solve(
                    Arc::new(F::Neg(Arc::new(F::EX(Arc::new(F::Neg(inner.clone())))))),
                    model,
                );
                self.memoise_ref(formula, ret)
            }
            F::EX(inner) => {
                let ret = self.sat_ex(inner.clone(), model);
                self.memoise_alloc(formula, ret)
            }
            // yuk
            F::AU(inner1, inner2) => {
                let ret = self.solve(
                    Arc::new(F::Or(
                        Arc::new(F::Neg(Arc::new(F::EU(
                            Arc::new(F::Neg(inner2.clone())),
                            Arc::new(F::And(
                                Arc::new(F::Neg(inner1.clone())),
                                Arc::new(F::Neg(inner2.clone())),
                            )),
                        )))),
                        Arc::new(F::EG(inner2.clone())),
                    )),
                    model,
                );
                self.memoise_ref(formula, ret)
            }
            F::EU(inner1, inner2) => {
                let ret = self.sat_eu(inner1.clone(), inner2.clone(), model);
                self.memoise_alloc(formula, ret)
            }
            F::EF(inner) => {
                let ret = self.solve(Arc::new(F::EU(Arc::new(F::Top), inner.clone())), model);
                self.memoise_ref(formula, ret)
            }
            F::AF(inner) => {
                let ret = self.sat_af(inner.clone(), model);
                self.memoise_alloc(formula, ret)
            }
            F::EG(inner) => {
                let ret = self.solve(
                    Arc::new(F::Neg(Arc::new(F::AF(Arc::new(F::Neg(inner.clone())))))),
                    model,
                );
                self.memoise_ref(formula, ret)
            }
            F::AG(inner) => {
                let ret = self.solve(
                    Arc::new(F::Neg(Arc::new(F::EF(Arc::new(F::Neg(inner.clone())))))),
                    model,
                );
                self.memoise_ref(formula, ret)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CTLSolver {
    model: Model<CTLVariable>,
    formulas: CTLFactory,
    cache: HashMap<Arc<CTLFormula>, HashSet<usize>>,
}
impl CTLSolver {
    pub fn new(model: Model<CTLVariable>) -> Self {
        Self {
            model,
            formulas: CTLFactory::default(),
            cache: HashMap::new(),
        }
    }
    pub fn satisfies(&mut self, formula: Arc<CTLFormula>) -> HashSet<String> {
        let formula = self.formulas.create(formula);

        // We need to clone the current cache because we want to modify it
        // Otherwise we would have two mutable references out to self.
        let map: HashMap<Arc<CTLFormula>, &HashSet<usize>> =
            self.cache.iter().map(|(k, v)| (k.clone(), v)).collect();

        // We need to create an arena because we want to memoise
        // and memoising without arena would mean storing in the hashmap,
        // wich might get realocated and thus invalidating pointers.
        let arena: Arena<HashSet<usize>> = Arena::new();

        // Once this is in place, we can create the solver, solve,
        // and get the reference map out of it again,
        // which needs to be cloned because it won't live as long as self
        // We can perform this clone while transforming it from indexes to names.
        let mut solver = CTLSolverInner { map, arena: &arena };
        let ret = self.model.get_names(solver.solve(formula, &self.model));

        let cache_update: HashMap<Arc<CTLFormula>, HashSet<usize>> = solver
            .map()
            .iter()
            // We only need to take the value out of it wasn't in cache already
            .filter(|(k, v)| !self.cache.contains_key(k.as_ref()))
            // We could try to move from arena, but cloning shouldn't cost too much
            .map(|(k, &v)| (k.clone(), v.clone()))
            .collect();

        // Once everything is cloned and `solver` (with its reference to map) is dropped,
        // we can extend the self cache and return.
        self.cache.extend(cache_update);
        ret
    }
}
